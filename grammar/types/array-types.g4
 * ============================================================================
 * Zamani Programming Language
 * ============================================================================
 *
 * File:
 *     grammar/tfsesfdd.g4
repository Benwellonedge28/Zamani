/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/types.g4
 *
 * Status:
 *     Canonical modular SOURCE-TYPE composition grammar.
 *
 * Purpose:
 *     Defines the complete source-level entry point and common composition
 *     rules for Zamani type syntax.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *   - the canonical type-expression entry point;
 *   - composition of primitive, named, generic, composite, function,
 *     reference, pointer, optional, result, quantum, classical, hardware,
 *     resource, capability, dependent and future-domain type syntax;
 *   - common type-path syntax;
 *   - common generic/type argument syntax;
 *   - source-level type-value expressions;
 *   - source-level lifetime syntax;
 *   - source-level type qualifiers;
 *   - type grouping;
 *   - syntax-level type composition invariants.
 *
 * This file does NOT own:
 *
 *   - lexical token definitions;
 *   - Unicode identifier rules;
 *   - keyword registration;
 *   - lexical precedence;
 *   - name resolution;
 *   - type inference;
 *   - type unification;
 *   - generic substitution;
 *   - trait/interface resolution;
 *   - ownership checking;
 *   - borrow checking;
 *   - lifetime checking;
 *   - resource allocation;
 *   - resource discovery;
 *   - hardware discovery;
 *   - physical qubit allocation;
 *   - logical-to-physical mapping;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - compiler backend selection;
 *   - runtime representation;
 *   - ABI layout.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     typeExpression
 *          |
 *          v
 *     domain-neutral frontend AST TypeExpr
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir          HDL/resource
 *       semantics          semantics            semantics
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    canonical semantic IR
 *                              |
 *                    optimization/lowering
 *                              |
 *                 routing/scheduling/resilience
 *                              |
 *                         ZQN / HAL
 *                              |
 *                      target realization
 *
 * `TypeExpr` remains a source-level AST concept.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar MUST NOT establish implementation limits.
 *
 * In particular it MUST NOT encode:
 *
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_CORES
 *   MAX_THREADS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_REGISTER_WIDTH
 *   MAX_REGISTER_COUNT
 *   MAX_TENSOR_RANK
 *   MAX_TENSOR_DIMENSION
 *   MAX_ARRAY_LENGTH
 *   MAX_GENERIC_ARITY
 *   MAX_TUPLE_ARITY
 *   MAX_FUNCTION_PARAMETER_COUNT
 *   MAX_TYPE_DEPTH
 *   MAX_RESOURCE_COUNT
 *   MAX_DEVICE_COUNT
 *
 * Recursive grammar constructs are intentionally unbounded by language
 * semantics. Implementations may impose resource-safety policies outside
 * the language grammar.
 *
 * A program-level value such as:
 *
 *     1024
 *
 * is valid program data.
 *
 * A compiler-wide rule such as:
 *
 *     arrays may contain at most 1024 elements
 *
 * is NOT part of this grammar.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * Type syntax describes portable PROGRAM MEANING.
 *
 * It must not select:
 *
 *   - a CPU;
 *   - a GPU;
 *   - an FPGA;
 *   - a QPU;
 *   - a physical qubit;
 *   - a memory bank;
 *   - a network node;
 *   - a fixed accelerator;
 *   - a vendor ABI.
 *
 * Target-specific realization occurs downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must lower type syntax into the repository's canonical frontend
 * TypeExpr vocabulary.
 *
 * The grammar must NOT introduce a second type AST.
 *
 * Expected mappings include:
 *
 *     namedType       -> TypeExpr::Identifier
 *     genericType     -> TypeExpr::Generic
 *     tupleType       -> TypeExpr::Tuple
 *     arrayType       -> TypeExpr::Array
 *     sliceType       -> TypeExpr::Slice
 *     functionType    -> TypeExpr::Function
 *     referenceType   -> TypeExpr::Reference
 *     pointerType     -> TypeExpr::Pointer
 *     optionalType    -> TypeExpr::Optional
 *     resultType      -> TypeExpr::Result
 *     neverType       -> TypeExpr::Never
 *     unitType        -> TypeExpr::Unit
 *     quantumType     -> TypeExpr::Quantum
 *     linearType      -> TypeExpr::Linear
 *     affineType      -> TypeExpr::Affine
 *
 * Exact Rust enum/field names remain owned by:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * The grammar does not invent alternative semantic representations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax answers:
 *
 *     "What type expression did the programmer write?"
 *
 * Semantic analysis answers:
 *
 *     "What does that type mean in this program?"
 *
 * Therefore:
 *
 *     Foo
 *
 * is syntactically a named type.
 *
 * Semantic analysis decides whether Foo is:
 *
 *     - a user type;
 *     - a generic parameter;
 *     - a type alias;
 *     - a capability type;
 *     - a resource type;
 *     - a quantum abstraction;
 *     - a hardware abstraction;
 *     - another domain-defined type.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum types are SOURCE abstractions.
 *
 * They must not encode:
 *
 *     physical qubit IDs
 *     physical topology
 *     vendor devices
 *     coupling maps
 *     calibration
 *     gate decomposition
 *     routing
 *     scheduling
 *
 * Examples:
 *
 *     Qubit
 *     Qubit[n]
 *     LogicalQubit
 *     QRegister[n]
 *     QuantumState<T>
 *
 * express source-level intent.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * HARDWARE / RESOURCE CONTRACT
 * ============================================================================
 *
 * Types may describe semantic resource abstractions:
 *
 *     Resource<T>
 *     Capability<C>
 *     Memory<T, N>
 *     Accelerator<A>
 *     QuantumResource<N>
 *
 * but the grammar MUST NOT decide whether a concrete machine has the
 * requested resources.
 *
 * Resource requirements, capabilities, preferences and implementation
 * decisions remain distinct concepts.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * This parser grammar consumes the canonical lexical vocabulary.
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical lexer must expose equivalent concepts for:
 *
 *     identifiers
 *     integer literals
 *     floating literals
 *     strings
 *     punctuation
 *     operators
 *     keywords
 *
 * Existing Rust lexer token concepts include Identifier, Integer, Float,
 * QuestionMark, DoubleColon, LessThan, GreaterThan, ThinArrow, Ampersand,
 * Star, Comma, Colon, Brackets and related operators.
 *
 * The eventual ANTLR lexer vocabulary must map those concepts consistently.
 *
 * No grammar-local token aliases are introduced here.
 *
 * ============================================================================
 * INTEGRATION WITH THE ROOT GRAMMAR
 * ============================================================================
 *
 * The root grammar MUST delegate its typeExpression rule to this contract.
 *
 * It MUST NOT independently define another competing:
 *
 *     typeExpression
 *     primitiveType
 *     genericType
 *     arrayType
 *     sliceType
 *     quantumType
 *     tensorType
 *     optionalType
 *     resultType
 *
 * implementation.
 *
 * Domain-specific type files may refine individual categories, but this file
 * owns their composition and public entry point.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING TYPE GRAMMARS
 * ============================================================================
 *
 * Existing specialized files remain domain/type-category implementation
 * modules. They must ultimately expose compatible parser rules rather than
 * redefining `typeExpression`.
 *
 * Examples include:
 *
 *     primitive-types.g4
 *     array-types.g4
 *     classical-types.g4
 *     composite-types.g4
 *     function-types.g4
 *     generic-types.g4
 *     hardware-types.g4
 *     map-types.g4
 *     option-types.g4
 *     quantum-types.g4
 *     algebraic-types.g4
 *
 * They are subordinate to this composition contract.
 *
 * ============================================================================
 */

parser grammar Types;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical source-level type expression.
 *
 * All declarations, parameters, return types, generic bounds, casts,
 * type-ascriptions and domain-specific type positions ultimately consume this
 * rule.
 */
typeExpression
    : typeQualifier* typePrimary typePostfix*
    ;


/**
 * A type appearing where a type is syntactically expected.
 *
 * The ordering is deliberate:
 *
 *   1. qualifiers;
 *   2. primary type;
 *   3. postfix constructors.
 *
 * This prevents optionality and reference syntax from becoming competing
 * top-level type systems.
 */
typePrimary
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
    | resourceType
    | capabilityType
    | dependentType
    | parenthesizedType
    ;


/* ============================================================================
 * 2. TYPE QUALIFIERS
 * ========================================================================== */

/**
 * Source-level linear/affine qualification.
 *
 * These are syntactic markers only.
 *
 * Semantic legality is determined by the ownership/resource/type system.
 */
typeQualifier
    : LINEAR
    | AFFINE
    ;


/* ============================================================================
 * 3. TYPE POSTFIXES
 * ========================================================================== */

/**
 * Optional type.
 *
 * Canonical source spelling:
 *
 *     T?
 *
 * Optionality is represented as a postfix rather than as a competing prefix
 * grammar.
 *
 * Examples:
 *
 *     int?
 *     Foo?
 *     Qubit?
 *     Vec<T>?
 */
typePostfix
    : QUESTION_MARK
    ;


/* ============================================================================
 * 4. PRIMITIVE TYPES
 * ========================================================================== */

/**
 * Primitive semantic categories.
 *
 * Representation width is NOT encoded by generic `int` or `float`.
 *
 * Explicit-width types may be provided by the primitive-type module where
 * those widths are part of source semantics rather than implementation
 * defaults.
 */
primitiveType
    : BOOL
    | CHAR
    | INT
    | UINT
    | FLOAT_TYPE
    | F16
    | F32
    | F64
    | F128
    | BYTE
    | STR_TYPE
    | STRING_TYPE
    | VOID
    ;


/**
 * Unit type.
 *
 * Canonical spelling:
 *
 *     ()
 */
unitType
    : LPAREN RPAREN
    ;


/**
 * Never / uninhabited type.
 *
 * The canonical spelling is represented by the language keyword/token
 * selected by the lexer contract.
 */
neverType
    : NEVER
    ;


/* ============================================================================
 * 5. NAMES AND TYPE PATHS
 * ========================================================================== */

/**
 * A named source-level type.
 *
 * Examples:
 *
 *     T
 *     User
 *     Self
 *     quantum::State
 *     std::collections::Map
 */
namedType
    : typePath
    ;


/**
 * Qualified type path.
 *
 * There is intentionally no finite namespace-depth limit.
 */
typePath
    : typePathSegment (DOUBLE_COLON typePathSegment)*
    ;


/**
 * A path component.
 *
 * The lexical layer owns identifier spelling.
 */
typePathSegment
    : IDENTIFIER
    ;


/* ============================================================================
 * 6. GENERIC TYPES
 * ========================================================================== */

/**
 * Generic type application.
 *
 * Examples:
 *
 *     Vec<int>
 *     Map<str, int>
 *     Tensor<float, N, M>
 *     Resource<Qubit>
 */
genericType
    : typePath typeArguments
    ;


/**
 * Generic argument list.
 *
 * There is no language-level maximum argument count.
 */
typeArguments
    : LESS_THAN genericArgumentList? GREATER_THAN
    ;


/**
 * Ordered generic arguments.
 *
 * A trailing comma is accepted.
 */
genericArgumentList
    : genericArgument (COMMA genericArgument)* COMMA?
    ;


/**
 * Generic arguments are either:
 *
 *     - a type;
 *     - a source-level type value.
 *
 * Semantic analysis determines whether the selected generic constructor
 * accepts the supplied argument category.
 */
genericArgument
    : typeExpression
    | typeValueExpression
    ;


/* ============================================================================
 * 7. TYPE-LEVEL VALUES
 * ========================================================================== */

/**
 * Type-level value expressions are syntax only.
 *
 * The parser does not evaluate them.
 *
 * Examples:
 *
 *     N
 *     1024
 *     Rows * Cols
 *     2 * N
 *     size + offset
 *
 * These values can remain symbolic until semantic analysis and later
 * compilation stages.
 */
typeValueExpression
    : typeValueUnary* typeValuePrimary typeValueBinaryPart*
    ;


/**
 * Unary operators permitted in type-level value expressions.
 */
typeValueUnary
    : PLUS
    | MINUS
    ;


/**
 * Binary type-value expression component.
 */
typeValueBinaryPart
    : typeValueOperator typeValueUnary* typeValuePrimary
    ;


/**
 * Operators usable in source-level type-value expressions.
 *
 * Semantic validation determines which operators are legal for a particular
 * dependent/type-value context.
 */
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


/**
 * Primary type-level values.
 *
 * Integer and floating literals are lexical values, not machine limits.
 */
typeValuePrimary
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | IDENTIFIER
    | qualifiedTypeValuePath
    | parenthesizedTypeValue
    ;


/**
 * Qualified symbolic type-level value.
 */
qualifiedTypeValuePath
    : IDENTIFIER (DOUBLE_COLON IDENTIFIER)+
    ;


/**
 * Parenthesized type-level value.
 */
parenthesizedTypeValue
    : LPAREN typeValueExpression RPAREN
    ;


/* ============================================================================
 * 8. DEPENDENT / VALUE-PARAMETERIZED TYPES
 * ========================================================================== */

/**
 * A value-parameterized type.
 *
 * Examples:
 *
 *     Matrix<T, Rows, Cols>
 *     Vector<T, N>
 *     Tensor<T, N, M, K>
 *
 * This is intentionally generic rather than tied to a particular mathematical
 * or hardware domain.
 */
dependentType
    : typePath LBRACKET dependentArgumentList RBRACKET
    ;


/**
 * Ordered dependent arguments.
 */
dependentArgumentList
    : dependentArgument (COMMA dependentArgument)* COMMA?
    ;


/**
 * Dependent arguments may be types or type-level values.
 */
dependentArgument
    : typeExpression
    | typeValueExpression
    ;


/* ============================================================================
 * 9. TUPLE TYPES
 * ========================================================================== */

/**
 * Tuple types.
 *
 * Unit:
 *
 *     ()
 *
 * One-element tuple:
 *
 *     (T,)
 *
 * Multi-element tuple:
 *
 *     (T, U, V)
 *
 * Tuple arity is not bounded by the grammar.
 */
tupleType
    : LPAREN tupleTypeElements RPAREN
    ;


tupleTypeElements
    : typeExpression COMMA tupleTypeTail?
    ;


tupleTypeTail
    : typeExpression (COMMA typeExpression)* COMMA?
    ;


/* ============================================================================
 * 10. ARRAY TYPES
 * ========================================================================== */

/**
 * Fixed-size array type.
 *
 * Canonical form:
 *
 *     [T; N]
 *
 * `N` is a source-level type value and may remain symbolic.
 */
arrayType
    : LBRACKET typeExpression SEMI typeValueExpression RBRACKET
    ;


/**
 * Dynamically sized / slice type.
 *
 * Canonical form:
 *
 *     [T]
 *
 * This rule is intentionally distinct from `arrayType`, eliminating the
 * previous competition between sized and unsized arrays.
 */
sliceType
    : LBRACKET typeExpression RBRACKET
    ;


/* ============================================================================
 * 11. FUNCTION TYPES
 * ========================================================================== */

/**
 * Function type.
 *
 * Examples:
 *
 *     fn() -> int
 *     fn(int) -> bool
 *     fn(A, B) -> C
 *
 * Parameter count is unbounded by grammar.
 */
functionType
    : FN LPAREN functionParameterTypes? RPAREN functionReturnType?
    ;


functionParameterTypes
    : typeExpression (COMMA typeExpression)* COMMA?
    ;


functionReturnType
    : THIN_ARROW typeExpression
    ;


/* ============================================================================
 * 12. REFERENCE TYPES
 * ========================================================================== */

/**
 * Immutable or mutable reference.
 *
 * Examples:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * The grammar records syntax.
 *
 * Borrow/lifetime legality belongs to semantic analysis.
 */
referenceType
    : AMPERSAND lifetimeAnnotation? MUT? typeExpression
    ;


/**
 * Source lifetime annotation.
 *
 * The apostrophe is syntax and the identifier is supplied by the canonical
 * identifier vocabulary.
 */
lifetimeAnnotation
    : APOSTROPHE IDENTIFIER
    ;


/* ============================================================================
 * 13. POINTER TYPES
 * ========================================================================== */

/**
 * Source-level pointer type.
 *
 * Examples:
 *
 *     *T
 *     *mut T
 *
 * Pointer width and address-space representation are downstream concerns.
 */
pointerType
    : STAR MUT? typeExpression
    ;


/* ============================================================================
 * 14. RESULT TYPES
 * ========================================================================== */

/**
 * Generic result type.
 *
 * Canonical form:
 *
 *     Result<T, E>
 *
 * The parser does not special-case error implementations.
 */
resultType
    : RESULT LESS_THAN typeExpression COMMA typeExpression GREATER_THAN
    ;


/* ============================================================================
 * 15. QUANTUM TYPES
 * ========================================================================== */

/**
 * Canonical quantum source types.
 *
 * These are semantic abstractions, not physical allocations.
 */
quantumType
    : qubitType
    | logicalQubitType
    | quantumRegisterType
    | quantumStateType
    | quantumTypeApplication
    ;


qubitType
    : QUBIT
    ;


logicalQubitType
    : LOGICAL_QUBIT
    ;


quantumRegisterType
    : QREGISTER
      (LESS_THAN typeValueExpression GREATER_THAN)?
    ;


quantumStateType
    : QSTATE
      (LESS_THAN typeExpression GREATER_THAN)?
    ;


/**
 * Generic quantum type application.
 *
 * Examples:
 *
 *     Quantum<T>
 *     QuantumRegister<T>
 *     QuantumState<T>
 *
 * The semantic layer decides the meaning.
 */
quantumTypeApplication
    : QUANTUM
      LESS_THAN
      genericArgumentList
      GREATER_THAN
    ;


/* ============================================================================
 * 16. RESOURCE TYPES
 * ========================================================================== */

/**
 * Target-independent resource type.
 *
 * Examples:
 *
 *     Resource<T>
 *     Resource<Memory>
 *     Resource<Qubit>
 */
resourceType
    : RESOURCE
      LESS_THAN
      typeExpression
      GREATER_THAN
    ;


/**
 * Target-independent capability type.
 *
 * Example:
 *
 *     Capability<quantum::measurement>
 */
capabilityType
    : CAPABILITY
      LESS_THAN
      typePath
      GREATER_THAN
    ;


/* ============================================================================
 * 17. PARENTHESIZED TYPES
 * ========================================================================== */

/**
 * Explicit grouping.
 *
 * Parenthesized types are syntactic grouping and do not create a semantic
 * wrapper.
 */
parenthesizedType
    : LPAREN typeExpression RPAREN
    ;


/* ============================================================================
 * 18. TYPE QUALIFIER COMPOSITION INVARIANTS
 * ========================================================================== */

/**
 * The following are semantic invariants, documented here so every downstream
 * implementation can be completed without returning to this grammar:
 *
 *   - `linear T` and `affine T` are source qualifiers;
 *   - qualifiers do not allocate resources;
 *   - qualifiers do not select hardware;
 *   - qualifiers do not imply a particular ABI;
 *   - qualifiers do not imply a runtime representation;
 *   - duplicate/incompatible qualifiers are semantic errors;
 *   - qualifier legality is checked after parsing.
 *
 * The grammar intentionally accepts recursive combinations so that malformed
 * source can be represented and diagnosed by structural/semantic validation
 * rather than causing parser-specific hidden restrictions.
 */


/* ============================================================================
 * 19. SOURCE-LEVEL TYPE CONTRACT
 * ========================================================================== */

/**
 * A valid type grammar implementation MUST preserve:
 *
 *   1. source ordering;
 *   2. source nesting;
 *   3. generic argument ordering;
 *   4. tuple element ordering;
 *   5. type-value expression structure;
 *   6. lifetime spelling;
 *   7. qualified-name structure;
 *   8. source spans through the parser/AST integration layer.
 *
 * It MUST NOT resolve:
 *
 *   - aliases;
 *   - traits;
 *   - interfaces;
 *   - generic substitutions;
 *   - resource capabilities;
 *   - physical resources;
 *   - hardware devices;
 *   - quantum topology;
 *   - implementation widths.
 */


/* ============================================================================
 * 20. DIAGNOSTIC CONTRACT
 * ========================================================================== */

/**
 * Diagnostics are owned by the parser/diagnostic layer.
 *
 * This grammar requires the parser integration to report at least:
 *
 *   - malformed generic argument lists;
 *   - missing closing delimiters;
 *   - malformed tuple types;
 *   - malformed array/slice types;
 *   - malformed function types;
 *   - malformed references;
 *   - malformed lifetimes;
 *   - malformed dependent values;
 *   - malformed Result types;
 *   - malformed quantum type applications.
 *
 * The grammar itself does not embed user-facing diagnostic strings.
 */


/* ============================================================================
 * 21. SECURITY CONTRACT
 * ========================================================================== */

/**
 * This grammar:
 *
 *   - performs no I/O;
 *   - executes no source expressions;
 *   - evaluates no type-level expressions;
 *   - performs no allocation based on source values;
 *   - accesses no hardware;
 *   - invokes no external programs;
 *   - requires no unsafe code.
 *
 * Resource limits for hostile input belong to parser/compilation policy.
 *
 * They MUST NOT silently become language-level semantic limits.
 */


/* ============================================================================
 * 22. DETERMINISM CONTRACT
 * ========================================================================== */

/**
 * Parsing of a fixed token sequence must be deterministic.
 *
 * Ordered constructs use ordered grammar repetition.
 *
 * No semantic map/set ordering is introduced by this grammar.
 *
 * Any AST serialization must preserve source ordering.
 */


/* ============================================================================
 * 23. SCALABILITY CONTRACT
 * ========================================================================== */

/**
 * The following are intentionally recursive/unbounded at the language level:
 *
 *     type nesting
 *     generic arguments
 *     tuple elements
 *     function parameters
 *     namespace depth
 *     dependent dimensions
 *     symbolic resource quantities
 *     quantum register cardinality
 *
 * Therefore:
 *
 *     Vec<Vec<Vec<T>>>
 *
 * and:
 *
 *     Tensor<T, N1, N2, N3, ...>
 *
 * remain language-valid independently of current machine size.
 *
 * Actual implementation/resource exhaustion is handled by explicit compiler
 * policy rather than grammar constants.
 */


/* ============================================================================
 * 24. COMPILER INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Compilation flow:
 *
 *     Types.g4
 *        |
 *        v
 *     parser
 *        |
 *        v
 *     frontend TypeExpr
 *        |
 *        v
 *     structural validation
 *        |
 *        v
 *     semantic type resolution
 *        |
 *        +-------------------------+
 *        |                         |
 *        v                         v
 *     classical                 quantum
 *     semantic                  semantic
 *        |                         |
 *        |                         v
 *        |                    quantum::ir
 *        |                         |
 *        +------------+------------+
 *                     |
 *                     v
 *                 canonical IR
 *                     |
 *                     v
 *            optimization/lowering
 *                     |
 *                     v
 *          target-independent intent
 *                     |
 *                     v
 *            target realization
 *
 * No type grammar construct may bypass semantic validation.
 */


/* ============================================================================
 * 25. RUNTIME INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Runtime representation is not defined by this grammar.
 *
 * For example:
 *
 *     int
 *
 * does not require:
 *
 *     i32
 *     i64
 *     machine-word
 *
 * The compiler/runtime chooses a representation consistent with the language
 * semantic contract and target capabilities.
 *
 * Likewise:
 *
 *     Qubit
 *
 * does not select a physical qubit.
 */


/* ============================================================================
 * 26. HDL / HARDWARE INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Hardware-related types remain target-independent.
 *
 * Examples:
 *
 *     Memory<T, N>
 *     Resource<Accelerator>
 *     Capability<hardware::vector>
 *
 * describe requirements or abstractions.
 *
 * They do not encode:
 *
 *     device 0
 *     core 7
 *     GPU 3
 *     physical_qubit 17
 *     fixed memory bank
 *
 * unless such target-specific information is explicitly represented later by
 * the hardware/resource semantic layers.
 */


/* ============================================================================
 * 27. QUANTUM IR INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Quantum type syntax must lower through semantic analysis to the repository's
 * canonical quantum representation.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumTypeIR
 *     CircuitIR
 *     GateIR
 *     PhysicalQubitIR
 *
 * as alternative canonical representations.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * Physical realization is downstream:
 *
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 */


/* ============================================================================
 * 28. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * The grammar must preserve source compatibility for established canonical
 * forms unless a language-version migration explicitly changes them.
 *
 * In particular, the following conceptual forms remain distinct:
 *
 *     T
 *     T?
 *     [T]
 *     [T; N]
 *     (T, U)
 *     fn(T) -> U
 *     &T
 *     &mut T
 *     *T
 *     Result<T, E>
 *     Qubit
 *     QRegister<N>
 *
 * No specialized domain grammar may silently redefine one of these forms.
 */


/* ============================================================================
 * 29. TEST CONTRACT
 * ========================================================================== */

/**
 * Conformance tests for this grammar must include at least:
 *
 * POSITIVE:
 *
 *     int
 *     uint
 *     float
 *     bool
 *     str
 *     User
 *     module::User
 *     Vec<int>
 *     Map<str, int>
 *     (int, bool)
 *     (int,)
 *     ()
 *     [int]
 *     [int; N]
 *     fn(int) -> bool
 *     &T
 *     &mut T
 *     &'a T
 *     *T
 *     Result<T, E>
 *     Qubit
 *     QRegister<N>
 *     Quantum<T>
 *     Resource<T>
 *     Capability<domain::feature>
 *     Matrix<T, Rows, Cols>
 *     Tensor<T, N, M, K>
 *
 * NEGATIVE:
 *
 *     missing generic closing delimiter
 *     missing array closing delimiter
 *     malformed tuple
 *     malformed function parameter list
 *     malformed lifetime
 *     malformed Result arity
 *     malformed dependent arguments
 *
 * BOUNDARY:
 *
 *     deeply nested types
 *     large generic argument lists
 *     large tuple types
 *     many function parameters
 *     deeply qualified names
 *     symbolic dimensions
 *
 * SCALABILITY:
 *
 *     arbitrary symbolic N
 *     arbitrary symbolic M
 *     arbitrary resource quantities
 *     arbitrary quantum cardinality expressions
 *
 * The tests must never establish an artificial maximum.
 */


/* ============================================================================
 * 30. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * Forbidden in this grammar:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TUPLE_ARITY
 *     MAX_GENERIC_ARITY
 *     MAX_TYPE_DEPTH
 *
 * Numeric literals appearing in source-level type-value expressions are not
 * implementation limits.
 */


/* ============================================================================
 * 31. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is complete when:
 *
 *   [x] There is exactly one public typeExpression composition contract.
 *   [x] Named types are target-independent.
 *   [x] Generic types are unbounded by grammar.
 *   [x] Tuple types are unbounded by grammar.
 *   [x] Arrays support symbolic cardinalities.
 *   [x] Slices are syntactically distinct from arrays.
 *   [x] Function types have no fixed parameter limit.
 *   [x] References support lifetimes and mutability.
 *   [x] Pointer representation remains downstream.
 *   [x] Optionality has one canonical syntax.
 *   [x] Result has one canonical syntax.
 *   [x] Quantum types remain source abstractions.
 *   [x] Resource/capability types remain target-independent.
 *   [x] Type-level values remain symbolic.
 *   [x] No hardware limits are encoded.
 *   [x] No physical quantum mapping is encoded.
 *   [x] No second quantum IR is introduced.
 *   [x] The canonical frontend TypeExpr remains the AST boundary.
 *   [x] Semantic resolution remains downstream.
 *   [x] Compiler/runtime representation remains downstream.
 *   [x] Diagnostics remain parser/diagnostic-layer responsibilities.
 *   [x] No unsafe implementation is required.
 *
 * Rust implementation layers consuming this grammar remain subject to:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     edition 2021
 *     safe Rust only
 *     no unsafe
 */