/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/generic-types.g4
 *
 * Role:
 *     Canonical source-level GENERIC TYPE APPLICATION grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - generic type application syntax;
 *   - generic argument lists;
 *   - ordered generic arguments;
 *   - type arguments;
 *   - type-level value arguments;
 *   - generic argument separators;
 *   - trailing-comma syntax;
 *   - generic nesting;
 *   - generic applications over qualified type paths;
 *   - generic applications over type expressions where supported by the
 *     canonical type grammar;
 *   - syntax necessary to preserve generic source structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifier spelling;
 *   - namespace/name resolution;
 *   - generic declaration syntax;
 *   - generic parameter declarations;
 *   - trait/interface declarations;
 *   - constraint solving;
 *   - type inference;
 *   - generic substitution;
 *   - monomorphization;
 *   - specialization;
 *   - overload resolution;
 *   - ABI selection;
 *   - calling conventions;
 *   - machine representation;
 *   - memory layout;
 *   - resource allocation;
 *   - quantum allocation;
 *   - physical qubit selection;
 *   - hardware selection;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - canonical quantum IR;
 *   - classical IR;
 *   - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                    Zamani source
 *                         |
 *                         v
 *                  canonical lexer
 *                         |
 *                         v
 *                  Zamani parser
 *                         |
 *                         v
 *                  Types.g4
 *                         |
 *             +-----------+-----------+
 *             |                       |
 *             v                       v
 *       generic-types.g4       other type delegates
 *             |
 *             v
 *       frontend TypeExpr
 *             |
 *             v
 *       TypeExpr::Generic
 *             |
 *             v
 *       structural validation
 *             |
 *             v
 *       semantic type resolution
 *             |
 *       +-----+------+------+------------------+
 *       |            |      |                  |
 *       v            v      v                  v
 *    classical    quantum  hardware       distributed
 *      types       types    /resource         /future
 *                         types              types
 *             |
 *             v
 *        canonical IR
 *             |
 *             v
 *       optimization / routing / scheduling
 *             |
 *             v
 *        target realization
 *
 * ============================================================================
 * CANONICAL AST BOUNDARY
 * ============================================================================
 *
 * The frontend already defines GenericType as a façade over the canonical:
 *
 *     TypeExpr::Generic {
 *         base: Box<TypeExpr>,
 *         arguments: Vec<TypeExpr>
 *     }
 *
 * This grammar MUST preserve that semantic shape.
 *
 * It MUST NOT introduce a second AST-level generic representation.
 *
 * The grammar produces syntax.
 *
 * The AST owns representation.
 *
 * Semantic analysis owns:
 *
 *     - generic declaration lookup;
 *     - parameter binding;
 *     - substitution;
 *     - inference;
 *     - constraints;
 *     - specialization;
 *     - associated types;
 *     - resource interpretation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Generic syntax is one of the mechanisms enabling:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * Generic syntax MUST NOT encode:
 *
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *     MAX_TENSOR_RANK
 *     MAX_HARDWARE_WIDTH
 *
 * Generic arity is determined by source semantics.
 *
 * Any implementation/security/resource limit belongs to an explicit compiler
 * policy and MUST NOT become a language-level grammar restriction.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Generic arguments may eventually represent:
 *
 *     classical types
 *     quantum types
 *     logical resources
 *     hardware resources
 *     accelerator types
 *     HDL parameters
 *     distributed resources
 *     tensors
 *     data schemas
 *     cryptographic parameters
 *     networking abstractions
 *     future computational abstractions
 *
 * This grammar does not decide which interpretation is correct.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns:
 *
 *     IDENT
 *     INTEGER
 *     FLOAT
 *     LESS_THAN
 *     GREATER_THAN
 *     COMMA
 *
 * and all other lexical tokens.
 *
 * This file MUST NOT declare lexer rules.
 *
 * If the repository changes token names, the canonical lexer vocabulary and
 * this parser grammar must be updated together as an integration change.
 *
 * ============================================================================
 * TYPE-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * `Types` is the public type grammar/orchestrator.
 *
 * It is responsible for importing/composing this delegate and exposing the
 * canonical `typeExpression` entry point.
 *
 * This file deliberately does NOT redefine:
 *
 *     typeExpression
 *     typeAtom
 *     primitiveType
 *     tupleType
 *     functionType
 *     referenceType
 *     pointerType
 *     typePath
 *     identifier
 *
 * Those belong to their canonical owning grammar.
 *
 * IMPORTANT:
 *
 * The generic delegate therefore participates in the composed `Types` grammar.
 * It is not intended to become an independent second type system.
 *
 * ============================================================================
 */

parser grammar GenericTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. GENERIC TYPE APPLICATION
 * ========================================================================== */

/**
 * Canonical generic type application.
 *
 * Examples:
 *
 *     Vec<int>
 *     Option<T>
 *     Result<Value, Error>
 *     Map<Key, Value>
 *     quantum::Register<Qubit>
 *     hardware::Buffer<Element>
 *
 * The base type is supplied by the canonical type grammar.
 *
 * `typePath` MUST therefore resolve to the repository's authoritative type
 * path rule when this delegate is composed into `Types`.
 *
 * No fixed generic arity exists.
 */
genericType
    : typePath
      genericArguments
    ;


/* ============================================================================
 * 2. GENERIC ARGUMENT LIST
 * ========================================================================== */

/**
 * Generic argument delimiters.
 *
 * Examples:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *
 * An empty generic argument list:
 *
 *     <>
 *
 * is intentionally rejected.
 *
 * A generic application must contain at least one argument.
 *
 * A trailing comma is accepted:
 *
 *     <T,>
 *     <T, U,>
 *
 * This improves formatting stability without changing semantics.
 */
genericArguments
    : LESS_THAN
      genericArgumentList
      GREATER_THAN
    ;


/**
 * One or more generic arguments.
 *
 * There is no grammar-level maximum.
 *
 * The parser therefore scales with the source program rather than with an
 * arbitrary language-defined machine limit.
 */
genericArgumentList
    : genericArgument
      (COMMA genericArgument)*
      COMMA?
    ;


/* ============================================================================
 * 3. GENERIC ARGUMENT
 * ========================================================================== */

/**
 * A generic argument may be:
 *
 *     - a type expression;
 *     - a type-level value expression.
 *
 * Examples:
 *
 *     Vec<int>
 *     Matrix<float, Rows, Cols>
 *     Buffer<T, Capacity>
 *
 * Semantic analysis determines whether the selected generic declaration
 * actually accepts the supplied kind of argument.
 *
 * This grammar deliberately does not perform that semantic validation.
 */
genericArgument
    : genericTypeArgument
    | genericValueArgument
    ;


/* ============================================================================
 * 4. TYPE ARGUMENT
 * ========================================================================== */

/**
 * Type-valued generic argument.
 *
 * The actual `typeExpression` rule is supplied by the canonical Types
 * composition.
 *
 * Examples:
 *
 *     Vec<int>
 *     Option<T>
 *     Result<Value, Error>
 *     QuantumRegister<Qubit>
 *
 * Nested generic applications are naturally supported:
 *
 *     Vec<Option<Result<T, E>>>
 */
genericTypeArgument
    : typeExpression
    ;


/* ============================================================================
 * 5. TYPE-LEVEL VALUE ARGUMENT
 * ========================================================================== */

/**
 * Value-valued generic argument.
 *
 * Examples:
 *
 *     Vector<T, N>
 *     Matrix<T, Rows, Cols>
 *     Tensor<T, N, M, K>
 *     Array<T, Capacity>
 *
 * The grammar preserves the expression structurally.
 *
 * It does NOT evaluate:
 *
 *     N
 *     Rows * Cols
 *     2 * N
 *
 * Evaluation, validation, overflow handling, symbolic reasoning, and resource
 * interpretation belong to semantic analysis/compiler policy.
 */
genericValueArgument
    : typeValueExpression
    ;


/* ============================================================================
 * 6. TYPE-LEVEL VALUE EXPRESSION
 * ========================================================================== */

/**
 * Canonical grammar-level expression for a generic value argument.
 *
 * This is intentionally limited to compile-time/type-level expression syntax.
 *
 * It is NOT the general runtime expression grammar.
 *
 * This separation prevents generic-type syntax from depending on the entire
 * runtime expression grammar and therefore avoids a circular dependency:
 *
 *     types -> expressions -> types
 *
 * The semantic layer may later lower this representation into a canonical
 * compile-time/value-expression representation.
 */
typeValueExpression
    : typeValueTerm
      (
          typeValueAddOperator
          typeValueTerm
      )*
    ;


/* ============================================================================
 * 7. TYPE-LEVEL TERM
 * ========================================================================== */

/**
 * Multiplicative precedence level.
 *
 * Examples:
 *
 *     N * M
 *     N / M
 *     N % M
 *     2 * Rows * Cols
 */
typeValueTerm
    : typeValueUnary
      (
          typeValueMultiplyOperator
          typeValueUnary
      )*
    ;


/* ============================================================================
 * 8. TYPE-LEVEL UNARY
 * ========================================================================== */

/**
 * Unary compile-time value operators.
 *
 * Examples:
 *
 *     -N
 *     +N
 *
 * The semantic layer decides whether the resulting value is legal for a
 * particular generic parameter.
 */
typeValueUnary
    : typeValueUnaryOperator*
      typeValuePrimary
    ;


/**
 * Unary operators.
 */
typeValueUnaryOperator
    : PLUS
    | MINUS
    ;


/* ============================================================================
 * 9. TYPE-LEVEL ADDITIVE OPERATORS
 * ========================================================================== */

typeValueAddOperator
    : PLUS
    | MINUS
    ;


/* ============================================================================
 * 10. TYPE-LEVEL MULTIPLICATIVE OPERATORS
 * ========================================================================== */

typeValueMultiplyOperator
    : STAR
    | SLASH
    | MODULO
    ;


/* ============================================================================
 * 11. TYPE-LEVEL PRIMARY
 * ========================================================================== */

/**
 * Primary type-level values.
 *
 * Examples:
 *
 *     1024
 *     N
 *     Rows
 *     dimensions::Rows
 *     (Rows * Cols)
 *
 * Floating-point literals are lexically accepted because the language may
 * permit floating compile-time parameters, but semantic analysis MUST decide
 * whether a particular generic parameter accepts them.
 */
typeValuePrimary
    : INTEGER
    | FLOAT
    | identifier
    | qualifiedTypeValueName
    | parenthesizedTypeValue
    ;


/* ============================================================================
 * 12. QUALIFIED TYPE-LEVEL VALUE NAME
 * ========================================================================== */

/**
 * Qualified symbolic compile-time value.
 *
 * Examples:
 *
 *     dimensions::Rows
 *     matrix::Columns
 *     configuration::Capacity
 *
 * There is no fixed qualification depth.
 */
qualifiedTypeValueName
    : identifier
      (DOUBLE_COLON identifier)+
    ;


/* ============================================================================
 * 13. PARENTHESIZED TYPE-LEVEL VALUE
 * ========================================================================== */

/**
 * Parenthesized compile-time expression.
 *
 * Examples:
 *
 *     (N)
 *     (Rows * Cols)
 *     (2 * N + 1)
 */
parenthesizedTypeValue
    : LPAREN
      typeValueExpression
      RPAREN
    ;


/* ============================================================================
 * 14. GENERIC APPLICATION NESTING
 * ========================================================================== */

/**
 * Generic nesting is intentionally expressed through the canonical
 * `typeExpression` integration.
 *
 * Examples:
 *
 *     Vec<Option<T>>
 *
 *     Result<Vec<T>, Error>
 *
 *     Map<Key, Vec<Value>>
 *
 *     QuantumContainer<Logical<Qubit>>
 *
 * No explicit recursive depth limit is present.
 *
 * Resource/security limits, if required by a compiler deployment, belong to
 * the compiler's explicit validation policy.
 *
 * They must never be encoded as grammar constants.
 */


/* ============================================================================
 * 15. GENERIC CONSTRUCTOR OWNERSHIP
 * ========================================================================== */

/**
 * GenericTypes parses generic APPLICATIONS.
 *
 * Generic declaration syntax belongs elsewhere.
 *
 * For example, a declaration such as:
 *
 *     type Container<T> = ...
 *
 * must be owned by the declarations/generic-types or functions/generics
 * grammar responsible for declarations.
 *
 * This distinction is essential:
 *
 *     GenericTypes
 *          |
 *          +---- application
 *
 *     declarations/generics
 *          |
 *          +---- declaration
 *
 * Otherwise declaration and application grammar would become coupled and
 * difficult to evolve.
 */


/* ============================================================================
 * 16. GENERIC CONSTRAINT OWNERSHIP
 * ========================================================================== */

/**
 * Constraints are NOT parsed as part of the generic application itself.
 *
 * For example:
 *
 *     Container<T>
 *
 * is an application.
 *
 * Whether T satisfies:
 *
 *     Numeric
 *     Copy
 *     QuantumResource
 *     HardwareCompatible
 *     Send
 *     Sync
 *     Serializable
 *
 * is a semantic question.
 *
 * Constraint declaration syntax belongs to the appropriate generic/function/
 * type-constraint grammar.
 */


/* ============================================================================
 * 17. QUANTUM INTEGRATION
 * ========================================================================== */

/**
 * GenericTypes is intentionally quantum-neutral.
 *
 * These are valid architectural examples:
 *
 *     Register<Qubit>
 *     Register<LogicalQubit>
 *     QuantumState<T>
 *     Circuit<Operation>
 *
 * but this grammar does NOT determine:
 *
 *     physical qubit number
 *     QPU identity
 *     topology
 *     gate set
 *     calibration
 *     routing
 *     scheduling
 *     error model
 *     QEC implementation
 *
 * Those responsibilities belong to:
 *
 *     quantum/
 *     hardware/
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *
 * respectively.
 */


/* ============================================================================
 * 18. CLASSICAL / HDL / HARDWARE INTEGRATION
 * ========================================================================== */

/**
 * Generic types may parameterize:
 *
 *     classical data structures
 *     vectors
 *     matrices
 *     tensors
 *     accelerator abstractions
 *     HDL modules
 *     hardware interfaces
 *     memories
 *     pipelines
 *     distributed resources
 *
 * Examples:
 *
 *     Vector<T, N>
 *     Matrix<T, Rows, Cols>
 *     Tensor<T, Shape>
 *     Buffer<T, Capacity>
 *     Pipeline<Stage>
 *     HardwareModule<Config>
 *
 * The grammar remains independent of physical realization.
 */


/* ============================================================================
 * 19. RESOURCE-SCALABLE GENERICS
 * ========================================================================== */

/**
 * Generic parameters are semantic parameters, not machine declarations.
 *
 * For example:
 *
 *     Vector<Qubit, N>
 *
 * may express a resource-parametric abstraction.
 *
 * It does NOT mean:
 *
 *     allocate N physical qubits now
 *
 * and it does not identify a hardware topology.
 *
 * Resource allocation occurs only after semantic analysis and target/resource
 * negotiation.
 */


/* ============================================================================
 * 20. DETERMINISM
 * ========================================================================== */

/**
 * Generic argument order is source order.
 *
 * For:
 *
 *     Map<Key, Value>
 *
 * the parser MUST preserve:
 *
 *     Key
 *     Value
 *
 * in that order.
 *
 * No unordered grammar construct is permitted to determine generic argument
 * meaning.
 */


/* ============================================================================
 * 21. ERROR-RECOVERY CONTRACT
 * ========================================================================== */

/**
 * This grammar intentionally does not embed parser actions.
 *
 * Error recovery belongs to the ANTLR parser configuration/front-end layer.
 *
 * GenericTypes therefore remains:
 *
 *     deterministic
 *     target-neutral
 *     side-effect-free
 *     Rust-implementation-independent
 *     safe to generate
 *
 * No filesystem, network, hardware, runtime, or compiler service may be
 * invoked from this grammar.
 */


/* ============================================================================
 * 22. IMPLEMENTATION LIMITS
 * ========================================================================== */

/**
 * No source-level limits are encoded here.
 *
 * In particular:
 *
 *     genericArgument*
 *
 * is represented through ANTLR repetition and therefore does not establish a
 * language-defined maximum generic arity.
 *
 * Compiler deployments MAY impose explicit resource/security limits, such as
 * parser budgets or semantic-analysis budgets.
 *
 * Such policies MUST:
 *
 *     - be explicit;
 *     - be configurable;
 *     - be reported diagnostically;
 *     - not alter the language's semantic definition;
 *     - not be encoded as arbitrary grammar constants.
 */


/* ============================================================================
 * 23. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * Existing canonical generic syntax remains supported:
 *
 *     Name<T>
 *     Name<T, U>
 *     namespace::Name<T>
 *
 * The AST continues to use:
 *
 *     TypeExpr::Generic
 *
 * GenericType remains a façade/API over that canonical representation.
 *
 * No second AST node is introduced by this grammar.
 */


/* ============================================================================
 * 24. FORBIDDEN CONSTRUCTS
 * ========================================================================== */

/**
 * This grammar MUST NOT introduce:
 *
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_TYPE_PARAMETERS
 *     MAX_GENERIC_DEPTH
 *     MAX_RESOURCE_PARAMETERS
 *     MAX_QUANTUM_PARAMETERS
 *     MAX_TENSOR_RANK
 *
 * It MUST NOT introduce:
 *
 *     device IDs
 *     qubit IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     fixed memory capacities
 *     fixed topology
 *     fixed hardware counts
 *
 * It MUST NOT select:
 *
 *     ABI
 *     backend
 *     compiler target
 *     scheduling policy
 *     routing policy
 *     optimization policy
 *     runtime implementation
 */


/* ============================================================================
 * 25. COMPLETION CONTRACT
 * ========================================================================== */

/**
 * This file is complete when:
 *
 *   1. It is imported by the canonical Types grammar.
 *
 *   2. Generic application syntax exists only in its authoritative location.
 *
 *   3. `types.g4` no longer contains a competing generic application grammar.
 *
 *   4. `typeExpression` remains owned by the canonical type grammar.
 *
 *   5. Generic declarations remain owned by declaration/function grammars.
 *
 *   6. Generic constraints remain outside this application grammar.
 *
 *   7. Type/value argument order is preserved.
 *
 *   8. Nested generic applications parse without a fixed depth limit.
 *
 *   9. Type-level symbolic values can be preserved without machine limits.
 *
 *  10. No lexer rules are duplicated here.
 *
 *  11. No AST representation is duplicated here.
 *
 *  12. No quantum IR is introduced here.
 *
 *  13. No hardware assumptions exist here.
 *
 *  14. No runtime/compiler actions exist here.
 *
 *  15. Positive, negative, boundary, compatibility, determinism, and
 *      cross-domain tests exist.
 *
 *  16. The generated parser builds successfully under the repository's
 *      ANTLR/Rust toolchain.
 *
 *  17. The Rust frontend continues to target Rust 1.97/1.97.1 without
 *      requiring unsafe code.
 */