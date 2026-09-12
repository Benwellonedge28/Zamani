/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/types.g4
 *
 * Role:
 *     Canonical source-level TYPE SYNTAX for Zamani.
 *
 * This file owns:
 *     - type-expression syntax;
 *     - primitive type syntax;
 *     - named and qualified types;
 *     - generic type application;
 *     - tuple types;
 *     - array and slice syntax;
 *     - function types;
 *     - reference and pointer syntax;
 *     - optional syntax;
 *     - result syntax;
 *     - never and unit types;
 *     - quantum type syntax;
 *     - type-level value expressions;
 *     - dependent/value-parameterized type syntax;
 *     - linear/affine type qualifiers;
 *     - lifetime syntax;
 *     - type paths;
 *     - type argument lists.
 *
 * This file DOES NOT own:
 *     - lexical tokens;
 *     - identifier spelling;
 *     - name resolution;
 *     - type inference;
 *     - generic substitution;
 *     - trait/interface resolution;
 *     - ownership checking;
 *     - borrow checking;
 *     - resource allocation;
 *     - qubit allocation;
 *     - physical placement;
 *     - hardware selection;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - canonical quantum IR;
 *     - classical IR;
 *     - runtime representation;
 *     - ABI layout;
 *     - machine width;
 *     - backend selection.
 *
 * ============================================================================
 * ARCHITECTURE
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     ZamaniParser / Core parser
 *        |
 *        +---- Types.g4
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
 *        +---- classical semantic types
 *        +---- quantum semantic types
 *        +---- hardware/resource types
 *        +---- distributed types
 *        +---- future domain types
 *        |
 *        v
 *     canonical IR
 *        |
 *        +---- quantum::ir
 *        +---- classical IR
 *        +---- control/data IR
 *        +---- resource metadata
 *        +---- effect metadata
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     routing / scheduling / lowering
 *        |
 *        v
 *     target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Types describe PROGRAM SEMANTICS and COMPUTATIONAL INTENT.
 *
 * This grammar MUST NOT encode finite machine assumptions such as:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_REGISTER_COUNT
 *     MAX_GENERIC_ARITY
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_LENGTH
 *     MAX_TYPE_DEPTH
 *     MAX_FUNCTION_PARAMETERS
 *     MAX_TENSOR_RANK
 *     MAX_NODE_COUNT
 *     MAX_DEVICE_COUNT
 *
 * No such limits occur in this grammar.
 *
 * Any implementation/resource limit belongs to an explicit compiler,
 * semantic-analysis, resource-management, or runtime policy.
 *
 * ============================================================================
 * RUST IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This is an ANTLR grammar and contains no Rust implementation code.
 *
 * Any Zamani compiler/frontend generated around this grammar MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST:
 *
 *     #![deny(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * The grammar itself never requires Rust unsafe code.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * All tokens are supplied by the canonical Zamani lexer.
 *
 * This file MUST NOT declare lexer rules.
 *
 * In particular, do NOT create grammar-local uppercase rules such as:
 *
 *     OPTIONAL_TYPE_NAME
 *
 * merely to recognize a contextual type constructor.
 *
 * `Optional`, `Result`, `Vec`, `Map`, `Tensor`, user-defined types, and
 * future type constructors should be represented through the canonical
 * identifier/name token where the lexer does not explicitly reserve them.
 *
 * Reserved lexical type names such as:
 *
 *     int
 *     float
 *     bool
 *     str
 *     string
 *     char
 *     void
 *     Never
 *     Result
 *     Qubit
 *
 * are consumed using their canonical lexer tokens.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser preserves syntax.
 *
 * The semantic layer decides whether:
 *
 *     Foo
 *
 * means a user type, type parameter, alias, constructor, capability type,
 * quantum abstraction, hardware/resource abstraction, or another semantic
 * entity.
 *
 * The parser MUST NOT decide that.
 *
 * Likewise:
 *
 *     Qubit
 *
 * means a quantum type syntactically.
 *
 * It does NOT mean:
 *
 *     physical qubit 0
 *     a particular QPU
 *     a particular topology
 *     a particular number of qubits
 *     a particular gate set
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 */

parser grammar Types;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical source-level type expression.
 *
 * TypeExpression is intentionally recursive and has no finite arity or depth
 * encoded in the grammar.
 *
 * Examples:
 *
 *     int
 *     User
 *     quantum::State
 *     Vec<int>
 *     Matrix<float, Rows, Cols>
 *     (int, bool)
 *     [int]
 *     [int; N]
 *     fn(int, bool) -> string
 *     &int
 *     &mut T
 *     *T
 *     T?
 *     Result<T, E>
 *     Qubit
 *     Qubit?
 *     Tensor<float, N, M>
 */
typeExpression
    : typeQualifier*
      typeAtom
      typePostfix*
    ;


/* ============================================================================
 * 2. TYPE QUALIFIERS
 * ========================================================================== */

/**
 * Qualifiers express ownership/usage semantics syntactically.
 *
 * Their legality and interaction are semantic concerns.
 */
typeQualifier
    : LINEAR
    | AFFINE
    ;


/* ============================================================================
 * 3. TYPE POSTFIXES
 * ========================================================================== */

/**
 * Postfix type operators.
 *
 * Currently:
 *
 *     T?
 *
 * represents optionality.
 *
 * The semantic layer determines the canonical representation.
 */
typePostfix
    : QUESTION_MARK
    ;


/* ============================================================================
 * 4. TYPE ATOM
 * ========================================================================== */

typeAtom
    : primitiveType
    | unitType
    | neverType
    | quantumType
    | tupleType
    | arrayType
    | functionType
    | referenceType
    | pointerType
    | dependentType
    | genericOrNamedType
    | parenthesizedType
    ;


/* ============================================================================
 * 5. PRIMITIVE TYPES
 * ========================================================================== */

/**
 * Primitive lexical types currently reserved by Zamani.
 *
 * These names do not imply machine representation.
 *
 * For example:
 *
 *     int
 *
 * does not mean a particular CPU integer width.
 *
 * Width/layout/representation is selected downstream.
 */
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
 * ========================================================================== */

/**
 * Unit:
 *
 *     ()
 *
 * This is deliberately distinct from a one-element tuple.
 */
unitType
    : LPAREN RPAREN
    ;


/* ============================================================================
 * 7. NEVER TYPE
 * ========================================================================== */

/**
 * Uninhabited type:
 *
 *     Never
 */
neverType
    : NEVER
    ;


/* ============================================================================
 * 8. QUANTUM TYPES
 * ========================================================================== */

/**
 * Canonical primitive quantum type:
 *
 *     Qubit
 *
 * This represents a source-level quantum resource/type abstraction.
 *
 * It does NOT identify:
 *
 *     - a physical qubit;
 *     - a physical index;
 *     - a device;
 *     - a topology;
 *     - a vendor;
 *     - a fixed number of qubits.
 *
 * Logical/physical distinctions belong to semantic analysis and the
 * quantum/hardware layers.
 */
quantumType
    : QUBIT
    ;


/* ============================================================================
 * 9. NAMED AND GENERIC TYPES
 * ========================================================================== */

/**
 * A named type is either:
 *
 *     Name
 *
 * or:
 *
 *     namespace::Name
 *
 * or a generic application:
 *
 *     Name<T>
 *     namespace::Name<T>
 *
 * The parser deliberately does not distinguish standard-library constructors,
 * user types, type parameters, resource types, quantum types, hardware types,
 * or future domain types.
 */
genericOrNamedType
    : typePath genericArguments?
    ;


/**
 * Qualified source-level type name.
 *
 * There is no fixed namespace depth.
 *
 * Examples:
 *
 *     T
 *     User
 *     std::Vec
 *     std::collections::Map
 *     quantum::State
 *     hardware::Resource
 *     future::domain::Type
 */
typePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/**
 * Generic argument list.
 *
 * No maximum generic arity is encoded.
 *
 * A trailing comma is accepted for formatting stability.
 */
genericArguments
    : LESS_THAN
      typeArgumentList?
      GREATER_THAN
    ;


/**
 * Generic arguments are syntactic type/value arguments.
 *
 * Type expressions are intentionally the primary form.
 *
 * Type-level value expressions are represented explicitly through
 * `typeValueArgument`.
 *
 * Semantic analysis determines which arguments are legal for the selected
 * constructor.
 */
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
 * ========================================================================== */

/**
 * Type-level values support scalable compile-time cardinalities without
 * encoding fixed machine sizes.
 *
 * Examples:
 *
 *     [T; N]
 *     [T; Rows * Cols]
 *     Tensor<T, N, M>
 *
 * The parser does not evaluate these expressions.
 *
 * It preserves them structurally for semantic analysis.
 */
typeValueArgument
    : typeValueExpression
    ;


typeValueExpression
    : typeValueUnary*
      typeValuePrimary
      typeValueBinaryPart*
    ;


typeValueUnary
    : PLUS
    | MINUS
    ;


typeValueBinaryPart
    : typeValueOperator
      typeValueUnary*
      typeValuePrimary
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
    | qualifiedTypeValuePath
    | parenthesizedTypeValue
    ;


qualifiedTypeValuePath
    : identifier
      (DOUBLE_COLON identifier)+
    ;


parenthesizedTypeValue
    : LPAREN
      typeValueExpression
      RPAREN
    ;


/* ============================================================================
 * 11. TUPLE TYPES
 * ========================================================================== */

/**
 * Tuple:
 *
 *     (A, B)
 *     (A, B, C)
 *
 * Unit:
 *
 *     ()
 *
 * is handled separately.
 *
 * A single-element tuple is intentionally represented with a trailing comma:
 *
 *     (A,)
 */
tupleType
    : LPAREN
      typeExpression
      COMMA
      tupleTypeTail?
      RPAREN
    ;


tupleTypeTail
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/* ============================================================================
 * 12. ARRAY AND SLICE TYPES
 * ========================================================================== */

/**
 * Sized array:
 *
 *     [T; N]
 *
 * Unsized/slice form:
 *
 *     [T]
 *
 * This resolves the previous architectural problem where array and slice
 * productions competed for the exact same syntax.
 *
 * The semantic layer decides the canonical representation of `[T]`.
 */
arrayType
    : LBRACKET
      typeExpression
      (SEMI typeValueExpression)?
      RBRACKET
    ;


/* ============================================================================
 * 13. FUNCTION TYPES
 * ========================================================================== */

/**
 * Function type:
 *
 *     fn() -> T
 *     fn(A) -> B
 *     fn(A, B, C) -> D
 *
 * No fixed parameter count exists in the grammar.
 *
 * The function type is purely semantic. It does not select a calling
 * convention or ABI.
 */
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
 * 14. REFERENCE TYPES
 * ========================================================================== */

/**
 * Immutable reference:
 *
 *     &T
 *
 * Mutable reference:
 *
 *     &mut T
 *
 * Lifetime:
 *
 *     &'a T
 *
 * Lifetime semantics belong to semantic analysis.
 *
 * Reference syntax does not imply Rust implementation semantics.
 */
referenceType
    : AMPERSAND
      lifetimeAnnotation?
      MUT?
      typeExpression
    ;


lifetimeAnnotation
    : APOSTROPHE
      identifier
    ;


/* ============================================================================
 * 15. POINTER TYPES
 * ========================================================================== */

/**
 * Raw/source pointer forms:
 *
 *     *T
 *     *mut T
 *
 * Pointer width, address space, ABI, representation, and target legality are
 * downstream concerns.
 *
 * This grammar does not imply 32-bit, 64-bit, or any other machine width.
 */
pointerType
    : STAR
      MUT?
      typeExpression
    ;


/* ============================================================================
 * 16. DEPENDENT / VALUE-PARAMETERIZED TYPES
 * ========================================================================== */

/**
 * Bracketed value parameterization:
 *
 *     Matrix[T, Rows, Cols]
 *     Vector[T, N]
 *     Tensor[T, N, M, K]
 *
 * The values are not evaluated by the parser.
 *
 * This permits arbitrarily scalable semantic dimensions without putting a
 * fixed dimension ceiling into the grammar.
 */
dependentType
    : typePath
      LBRACKET
      typeValueExpression
      (COMMA typeValueExpression)*
      RBRACKET
    ;


/* ============================================================================
 * 17. PARENTHESIZED TYPES
 * ========================================================================== */

/**
 * Parenthesized grouping:
 *
 *     (T)
 *
 * This is useful for recursively nested types.
 */
parenthesizedType
    : LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 18. IDENTIFIER BRIDGE
 * ========================================================================== */

/**
 * The canonical lexer owns identifier spelling.
 *
 * This grammar does not duplicate:
 *
 *     Unicode policy
 *     normalization
 *     identifier length
 *     identifier character sets
 *
 * Those belong to the lexer/name system.
 */
identifier
    : IDENT
    ;


/* ============================================================================
 * 19. TYPE CONSTRUCTOR COMPATIBILITY
 * ========================================================================== */

/**
 * Common semantic constructors remain ordinary names unless explicitly
 * reserved by the lexer.
 *
 * Therefore:
 *
 *     Optional<T>
 *     Vec<T>
 *     Slice<T>
 *     Array<T, N>
 *     Map<K, V>
 *     Set<T>
 *     Result<T, E>
 *     Tensor<T, N, M>
 *     Matrix<T, R, C>
 *     Future<T>
 *     Stream<T>
 *     Resource<T>
 *     LogicalQubit<T>
 *     PhysicalQubit<T>
 *
 * are represented by genericOrNamedType.
 *
 * The semantic resolver decides their meaning.
 *
 * `Result` is additionally reserved by the current lexer and therefore can
 * appear through the generic type path only where the lexer exposes it as an
 * identifier-compatible token. If the canonical lexer keeps RESULT reserved,
 * the parser integration layer must provide the corresponding named-type
 * bridge rather than duplicating the token.
 */


/* ============================================================================
 * 20. TYPE-LEVEL PATHS
 * ========================================================================== */

/**
 * Explicit value paths are kept separate from type paths in the AST so that
 * semantic analysis can distinguish:
 *
 *     Type::Associated
 *
 * from:
 *
 *     dimensions::N
 *
 * without requiring the parser to resolve either.
 */
typeValuePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 21. ASSOCIATED / QUALIFIED TYPE FORMS
 * ========================================================================== */

/**
 * A qualified path can represent an associated or nested type:
 *
 *     Trait::Associated
 *     Module::Type
 *     Namespace::Nested::Type
 *
 * No semantic resolution occurs here.
 */
qualifiedType
    : typePath
    ;


/* ============================================================================
 * 22. TYPE ALIAS TARGET
 * ========================================================================== */

/**
 * Stable façade used by type-alias declarations.
 *
 * The owning declaration grammar decides whether this appears after:
 *
 *     type Name = ...
 *
 * This file only defines the type expression.
 */
typeAliasTarget
    : typeExpression
    ;


/* ============================================================================
 * 23. TYPE BOUNDS
 * ========================================================================== */

/**
 * Type bounds are syntax only.
 *
 * Examples:
 *
 *     T: Numeric
 *     T: quantum::State
 *     T: TraitA + TraitB
 *
 * Trait satisfaction and constraint solving belong to semantic analysis.
 */
typeBound
    : typeExpression
    ;


typeBoundList
    : typeBound
      (PLUS typeBound)*
    ;


/* ============================================================================
 * 24. TYPE PARAMETERS
 * ========================================================================== */

/**
 * Generic type parameter.
 *
 * Examples:
 *
 *     T
 *     T: Numeric
 *     T: A + B
 *
 * Default values and semantic constraints belong to the generic/declaration
 * layer, not to this grammar's type-expression semantics.
 */
typeParameter
    : identifier
      (COLON typeBoundList)?
    ;


/**
 * Generic parameter list.
 *
 * No finite generic arity is encoded.
 */
typeParameterList
    : LESS_THAN
      typeParameter
      (COMMA typeParameter)*
      COMMA?
      GREATER_THAN
    ;


/* ============================================================================
 * 25. TYPE ARGUMENT COMPATIBILITY HELPERS
 * ========================================================================== */

/**
 * Explicit type-only argument list.
 *
 * Useful to callers that know their context accepts only types.
 */
typeOnlyArgumentList
    : LESS_THAN
      typeExpression
      (COMMA typeExpression)*
      COMMA?
      GREATER_THAN
    ;


/**
 * Explicit value-only argument list.
 *
 * Useful for dependent/value constructors.
 */
typeValueArgumentList
    : LBRACKET
      typeValueExpression
      (COMMA typeValueExpression)*
      COMMA?
      RBRACKET
    ;


/* ============================================================================
 * 26. RESOURCE / DOMAIN-NEUTRAL TYPE FORMS
 * ========================================================================== */

/**
 * Resource-oriented types remain generic/name-based rather than being tied to
 * a finite hardware vocabulary.
 *
 * Examples:
 *
 *     Resource<Qubit>
 *     Resource<Device>
 *     Resource<Memory>
 *     Capability<Compute>
 *     Stream<T>
 *
 * No dedicated hardware grammar is created here.
 *
 * Hardware semantics belong to hardware/resource analysis.
 */
resourceType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 27. QUANTUM DOMAIN TYPE FORMS
 * ========================================================================== */

/**
 * Quantum domain types beyond the primitive `Qubit` are represented through
 * ordinary generic/named types.
 *
 * Examples:
 *
 *     Quantum<State>
 *     QuantumRegister<Qubit>
 *     Logical<Qubit>
 *     Observable<T>
 *     Circuit<...>
 *
 * This prevents the type grammar from becoming a second quantum IR.
 *
 * Canonical quantum semantic representation remains owned by the quantum
 * frontend/IR layer.
 */
quantumDomainType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 28. CLASSICAL / NUMERICAL DOMAIN TYPE FORMS
 * ========================================================================== */

/**
 * Classical numerical abstractions remain generic/name-based.
 *
 * Examples:
 *
 *     Vector<T, N>
 *     Matrix<T, R, C>
 *     Tensor<T, N, M, K>
 *
 * Dimensions remain symbolic/source-level values until semantic analysis.
 */
classicalDomainType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 29. HARDWARE DOMAIN TYPE FORMS
 * ========================================================================== */

/**
 * Hardware types remain target-independent.
 *
 * Examples:
 *
 *     hardware::Device
 *     hardware::Resource<T>
 *     hardware::Capability<C>
 *     accelerator::Engine
 *
 * The grammar never selects a particular physical device.
 */
hardwareDomainType
    : typePath
      genericArguments?
    ;


/* ============================================================================
 * 30. DISTRIBUTED DOMAIN TYPE FORMS
 * ========================================================================== */

/**
 * Distributed types remain abstract.
 *
 * Examples:
 *
 *     Node<T>
 *     Channel<T>
 *     Remote<T>
 *     Replicated<T>
 *     Partition<T>
 *
 * Placement and topology are semantic/resource concerns.
 */
distributedDomainType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 31. TYPE EXPRESSION COMPATIBILITY ENTRY
 * ========================================================================== */

/**
 * Compatibility façade for existing parser modules that refer to
 * `typeExpr`.
 *
 * New code should use `typeExpression`.
 *
 * Keeping this façade allows staged migration without requiring every
 * dependent grammar to be rewritten simultaneously.
 */
typeExpr
    : typeExpression
    ;


/* ============================================================================
 * 32. TYPE PATH COMPATIBILITY ENTRY
 * ========================================================================== */

/**
 * Compatibility façade for parser modules that historically used `namedType`.
 */
namedType
    : typePath
    ;


/* ============================================================================
 * 33. GENERIC TYPE COMPATIBILITY ENTRY
 * ========================================================================== */

genericType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 34. OPTIONAL TYPE COMPATIBILITY ENTRY
 * ========================================================================== */

/**
 * Explicit compatibility façade.
 *
 * This does NOT introduce a lexer token for Optional.
 *
 * `Optional<T>` is parsed through an ordinary type path:
 *
 *     Optional
 *
 * followed by generic arguments.
 *
 * The semantic layer identifies the constructor.
 */
optionalType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 35. RESULT TYPE COMPATIBILITY ENTRY
 * ========================================================================== */

/**
 * Explicit compatibility façade for Result<T, E>.
 *
 * The canonical semantic representation remains a generic type application.
 */
resultType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 36. SLICE TYPE COMPATIBILITY ENTRY
 * ========================================================================== */

/**
 * Canonical slice syntax is:
 *
 *     [T]
 *
 * and is intentionally represented by arrayType at the concrete syntax level.
 *
 * This façade exists for AST/tooling consumers that need to classify the
 * unsized form after parsing.
 *
 * Classification belongs to semantic analysis.
 */
sliceType
    : LBRACKET
      typeExpression
      RBRACKET
    ;


/* ============================================================================
 * 37. TYPE DECLARATION SUPPORT
 * ========================================================================== */

/**
 * A declaration owner may use this as its complete type target.
 */
declaredType
    : typeExpression
    ;


/* ============================================================================
 * 38. CAST / ASCRIPTION SUPPORT
 * ========================================================================== */

/**
 * A cast/ascription owner can use this stable rule.
 *
 * The semantic layer determines whether the operation is:
 *
 *     - coercion;
 *     - checked conversion;
 *     - reinterpretation;
 *     - quantum conversion;
 *     - resource conversion;
 *     - forbidden.
 */
castType
    : typeExpression
    ;


/* ============================================================================
 * 39. SOURCE TYPE CONTRACT
 * ========================================================================== */

/**
 * The following semantic categories are intentionally representable without
 * adding new grammar rules:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     future domains
 *
 * through:
 *
 *     named types
 *     qualified types
 *     generic types
 *     tuple types
 *     arrays/slices
 *     function types
 *     references/pointers
 *     dependent/value parameters
 *
 * This is a deliberate POCO-REAF property.
 */


/* ============================================================================
 * 40. NO HARDWARE LIMITS
 * ========================================================================== */

/**
 * No grammar production in this file may introduce:
 *
 *     Qubit0
 *     Qubit1
 *     CPU0
 *     GPU0
 *     device0
 *     node0
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Such concepts are target/resource data and belong outside source type
 * syntax.
 */


/* ============================================================================
 * 41. NO SEMANTIC SIDE EFFECTS
 * ========================================================================== */

/**
 * Parsing any type MUST NOT:
 *
 *     allocate memory;
 *     allocate hardware;
 *     allocate qubits;
 *     discover devices;
 *     contact the network;
 *     access the filesystem;
 *     query calibration;
 *     invoke QEC;
 *     invoke ZQN;
 *     invoke scheduling;
 *     invoke optimization;
 *     select a backend.
 *
 * The parser only constructs syntax.
 */


/* ============================================================================
 * 42. CANONICAL INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Expected parser integration:
 *
 *     functionDeclaration
 *         -> parameterList
 *         -> typeExpression
 *
 *     returnType
 *         -> typeExpression
 *
 *     structField
 *         -> typeExpression
 *
 *     typeAliasDeclaration
 *         -> typeExpression
 *
 *     generic parameter bounds
 *         -> typeBoundList
 *
 *     casts/ascriptions
 *         -> typeExpression
 *
 *     quantum declarations
 *         -> typeExpression where appropriate
 *
 *     hardware/resource declarations
 *         -> typeExpression where appropriate
 *
 * Types.g4 MUST NOT own those surrounding declaration contexts.
 */


/* ============================================================================
 * 43. AST CONTRACT
 * ========================================================================== */

/**
 * The frontend AST should preserve at least these structural categories:
 *
 *     Primitive
 *     Unit
 *     Never
 *     Named
 *     Qualified
 *     Generic
 *     Tuple
 *     Array
 *     Slice
 *     Function
 *     Reference
 *     Pointer
 *     Optional
 *     Result
 *     Quantum
 *     Dependent
 *     TypeValue
 *     Linear
 *     Affine
 *
 * The AST MUST NOT store target-specific interpretation merely because a
 * grammar token was encountered.
 *
 * Source spans must be preserved by the parser/frontend layer.
 */


/* ============================================================================
 * 44. SEMANTIC CONTRACT
 * ========================================================================== */

/**
 * Semantic analysis owns:
 *
 *     name lookup
 *     alias expansion
 *     type identity
 *     generic substitution
 *     constraint solving
 *     trait resolution
 *     ownership
 *     borrowing
 *     lifetime checking
 *     resource typing
 *     quantum typing
 *     hardware capability typing
 *     dimensional validation
 *     dependent-value validation
 *     representation selection
 *
 * This grammar does none of those things.
 */


/* ============================================================================
 * 45. QUANTUM IR CONTRACT
 * ========================================================================== */

/**
 * Quantum source types are lowered into the existing canonical quantum
 * semantic boundary.
 *
 * This grammar MUST NOT create:
 *
 *     Gate
 *     QuantumGate
 *     PhysicalQubit
 *     Qubit topology
 *     Schedule
 *     QEC code
 *     Noise model
 *
 * Those belong to the repository's corresponding subsystems.
 *
 * In particular:
 *
 *     grammar
 *         -> frontend semantic type
 *         -> quantum::ir
 *
 * rather than:
 *
 *     grammar
 *         -> private quantum representation.
 */


/* ============================================================================
 * 46. RESOURCE / HARDWARE CONTRACT
 * ========================================================================== */

/**
 * Resource and hardware types express requirements or semantic categories.
 *
 * They do not identify actual resources.
 *
 * Example:
 *
 *     Resource<Qubit>
 *
 * is not:
 *
 *     physical_qpu_7.qubit_31
 *
 * Resource discovery, capability checking, placement and allocation happen
 * downstream.
 */


/* ============================================================================
 * 47. SCALABILITY CONTRACT
 * ========================================================================== */

/**
 * The grammar intentionally uses recursive productions:
 *
 *     typeExpression
 *     typePath
 *     genericArguments
 *     tupleType
 *     functionType
 *     typeValueExpression
 *     dependentType
 *
 * rather than fixed enumerations.
 *
 * Therefore:
 *
 *     tiny programs
 *     large programs
 *     distributed programs
 *     large tensor programs
 *     large quantum programs
 *     heterogeneous programs
 *
 * are not syntactically constrained by an artificial hardware size.
 *
 * Actual parser/runtime stack limits, memory limits, token-stream limits, or
 * compiler resource budgets must be configured by compiler policy rather than
 * encoded here.
 */


/* ============================================================================
 * 48. DETERMINISM CONTRACT
 * ========================================================================== */

/**
 * This grammar:
 *
 *     performs no I/O;
 *     performs no runtime discovery;
 *     uses no mutable global state;
 *     performs no backend lookup;
 *     performs no nondeterministic action.
 *
 * Given the same token stream, parsing is deterministic under the configured
 * ANTLR prediction strategy.
 */


/* ============================================================================
 * 49. VERSIONING CONTRACT
 * ========================================================================== */

/**
 * Syntax-version evolution belongs to the grammar/specification layer.
 *
 * New type constructors should normally be introduced as:
 *
 *     identifiers
 *     qualified identifiers
 *     generic applications
 *
 * instead of reserving additional lexer keywords.
 *
 * A new reserved keyword is justified only when it changes parsing
 * unambiguously and is part of the language's stable lexical specification.
 */


/* ============================================================================
 * 50. FUTURE EXTENSION CONTRACT
 * ========================================================================== */

/**
 * Future type systems can be represented through existing extensible forms:
 *
 *     generic types
 *     named types
 *     qualified types
 *     type parameters
 *     dependent/value arguments
 *     function types
 *     references
 *     resource types
 *     domain-specific constructors
 *
 * Examples include:
 *
 *     capability types
 *     effect types
 *     session types
 *     temporal types
 *     distributed types
 *     accelerator types
 *     tensor types
 *     probabilistic types
 *     symbolic types
 *     cryptographic types
 *     security types
 *     proof types
 *     future computational domains
 *
 * The grammar therefore remains open rather than requiring one keyword per
 * future technology.
 */


/* ============================================================================
 * 51. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is complete only when:
 *
 * [ ] It compiles as an ANTLR parser grammar.
 * [ ] Its token vocabulary resolves entirely through ZamaniLexer.
 * [ ] It declares no lexer rules.
 * [ ] It contains no Rust code.
 * [ ] It contains no unsafe implementation requirement.
 * [ ] It contains no fixed machine/resource limit.
 * [ ] It contains no hardware-specific device IDs.
 * [ ] It contains no physical qubit indexing.
 * [ ] It contains no backend selection.
 * [ ] It contains no quantum IR duplication.
 * [ ] It supports primitive types.
 * [ ] It supports named types.
 * [ ] It supports qualified types.
 * [ ] It supports generic types.
 * [ ] It supports tuple types.
 * [ ] It supports arrays.
 * [ ] It supports slices.
 * [ ] It supports function types.
 * [ ] It supports references.
 * [ ] It supports pointers.
 * [ ] It supports optional types.
 * [ ] It supports result types.
 * [ ] It supports Never.
 * [ ] It supports Unit.
 * [ ] It supports Qubit.
 * [ ] It supports symbolic/dependent dimensions.
 * [ ] It supports linear/affine qualifiers.
 * [ ] It preserves type syntax for semantic analysis.
 * [ ] It integrates with the canonical frontend AST.
 * [ ] It integrates with the canonical semantic type system.
 * [ ] It does not force later hardware changes into this file.
 * [ ] Positive grammar tests exist.
 * [ ] Negative grammar tests exist.
 * [ ] Nested-type tests exist.
 * [ ] Generic scalability tests exist.
 * [ ] Quantum/classical cross-domain tests exist.
 * [ ] Round-trip tests exist where a canonical printer is available.
 */