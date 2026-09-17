/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/quantum.g4
 *
 * Grammar:
 *     Quantum
 *
 * Status:
 *     CANONICAL SOURCE-LEVEL QUANTUM TYPE GRAMMAR
 *
 * Purpose:
 *     Defines quantum-specific type syntax for the universal Zamani type
 *     system.
 *
 * This file is the canonical owner of quantum TYPE SYNTAX.
 *
 * It does NOT own:
 *
 *     - quantum operations
 *     - gates
 *     - circuits
 *     - measurement statements
 *     - reset
 *     - observables
 *     - QEC
 *     - ZQN
 *     - routing
 *     - scheduling
 *     - calibration
 *     - hardware discovery
 *     - physical allocation
 *     - backend selection
 *     - runtime representation
 *     - quantum::ir
 *
 * Those responsibilities belong to their existing repository boundaries.
 *
 * ============================================================================
 * ARCHITECTURE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       v
 *     parser
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *     universal Types         Quantum
 *                                 |
 *                                 v
 *                            TypeExpr
 *                                 |
 *                                 v
 *                         structural validation
 *                                 |
 *                                 v
 *                          semantic type model
 *                                 |
 *                                 v
 *                            quantum::ir
 *                                 |
 *              +------------------+------------------+
 *              |                  |                  |
 *              v                  v                  v
 *          optimization        routing           scheduling
 *              |                  |                  |
 *              +------------------+------------------+
 *                                 |
 *                                 v
 *                          QEC / ZQN / HAL
 *                                 |
 *                                 v
 *                         target realization
 *
 * IMPORTANT:
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT create or depend upon another quantum IR.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Quantum types are source-level computational intent.
 *
 * They MUST NOT encode implementation limits such as:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_REGISTER_SIZE
 *     MAX_STATE_SIZE
 *     MAX_CIRCUIT_WIDTH
 *     MAX_CIRCUIT_DEPTH
 *     MAX_QPU_COUNT
 *     MAX_DEVICE_COUNT
 *     MAX_TOPOLOGY_SIZE
 *
 * There is deliberately no finite grammar-level limit on:
 *
 *     - quantum-resource cardinality
 *     - generic arity
 *     - namespace depth
 *     - type nesting
 *     - symbolic expressions
 *     - quantum type parameters
 *
 * A source value such as:
 *
 *     qubit[1024]
 *
 * is ordinary program semantics.
 *
 * It MUST NOT be interpreted as a compiler-wide maximum.
 *
 * Likewise:
 *
 *     qubit[N]
 *     qubit[2 * N]
 *     qubit[algorithm_width]
 *
 * remain symbolic source expressions until semantic analysis.
 *
 * The grammar never converts these values to:
 *
 *     usize
 *     u32
 *     u64
 *
 * or any other implementation-specific representation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Grammar name:
 *
 *     ZamaniTokens
 *
 * No lexer rules are declared here.
 *
 * Relevant canonical tokens include:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_LOGICAL
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOUBLE_COLON
 *     LESS_THAN
 *     GREATER_THAN
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     PERCENT
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *     AMPERSAND
 *     PIPE
 *     CARET
 *
 * Token spelling belongs exclusively to the lexer.
 *
 * This grammar MUST NOT introduce aliases such as:
 *
 *     QUBIT
 *     QUANTUM
 *     INT
 *     FLOAT_TYPE
 *
 * merely to compensate for historical lexer vocabulary.
 *
 * The repository's current modular lexical direction uses the K_* vocabulary.
 *
 * ============================================================================
 * SOURCE COMPATIBILITY
 * ============================================================================
 *
 * Canonical lowercase quantum forms include:
 *
 *     qubit
 *     logical qubit
 *     qubit[N]
 *     logical qubit[N]
 *     quantum<T>
 *     quantum::State
 *     quantum::State<T>
 *
 * Historical/general type spellings such as:
 *
 *     Qubit
 *     QRegister<N>
 *     QState<T>
 *     QuantumRegister<N>
 *     LogicalQubit
 *     LogicalRegister<N>
 *     QuantumState<T>
 *
 * remain representable through the universal named/generic type grammar.
 *
 * They MUST NOT require an ever-growing list of special lexer keywords.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The frontend maps the resulting syntax into the existing canonical
 * `TypeExpr` representation.
 *
 * Expected semantic correspondence:
 *
 *     qubit
 *         -> source-level quantum TypeExpr
 *
 *     logical qubit
 *         -> source-level logical quantum TypeExpr
 *
 *     qubit[N]
 *         -> parameterized/collection quantum TypeExpr
 *
 *     logical qubit[N]
 *         -> parameterized/collection logical quantum TypeExpr
 *
 *     quantum<T>
 *         -> generic quantum TypeExpr
 *
 *     quantum::State
 *         -> TypeExpr::Identifier / qualified TypeExpr
 *
 *     quantum::State<T>
 *         -> TypeExpr::Generic
 *
 * The exact Rust enum fields remain owned by:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * The grammar MUST NOT invent:
 *
 *     QuantumTypeIR
 *     QuantumRegisterIR
 *     PhysicalQubitIR
 *     QuantumHardwareTypeIR
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser answers:
 *
 *     "What quantum type syntax did the programmer write?"
 *
 * Semantic analysis answers:
 *
 *     "What does that type mean in this program?"
 *
 * Therefore the grammar does not decide whether a type denotes:
 *
 *     - a simulator representation
 *     - a physical qubit
 *     - a logical qubit implementation
 *     - an encoded state
 *     - a tensor representation
 *     - a stabilizer representation
 *     - a vendor resource
 *     - a particular QPU
 *
 * Those decisions are downstream.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * `quantumType` is the public integration rule.
 *
 * The universal type grammar imports this grammar and delegates its quantum
 * branch to `quantumType`.
 *
 * ============================================================================
 */

parser grammar Quantum;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC QUANTUM TYPE ENTRY
 * ========================================================================== */

/**
 * Canonical quantum-specific type expression.
 *
 * Only syntactically quantum-specific forms belong here.
 *
 * Ordinary identifiers are deliberately NOT accepted directly by this rule.
 * That prevents this grammar from stealing every named type from the
 * universal type grammar.
 */
quantumType
    : quantumPrimitiveType
    | logicalQubitType
    | quantumCollectionType
    | quantumConstructorType
    | quantumQualifiedType
    ;


/* ============================================================================
 * 2. ABSTRACT QUBIT
 * ========================================================================== */

/**
 * Source-level quantum resource.
 *
 * Example:
 *
 *     qubit
 *
 * This does not identify:
 *
 *     - a physical qubit
 *     - a physical index
 *     - a device
 *     - a QPU
 *     - a topology
 *     - a vendor
 *     - a simulator representation
 */
quantumPrimitiveType
    : K_QUBIT
    ;


/* ============================================================================
 * 3. LOGICAL QUBIT
 * ========================================================================== */

/**
 * Logical quantum resource.
 *
 * Example:
 *
 *     logical qubit
 *
 * The grammar does not choose:
 *
 *     - QEC code
 *     - code distance
 *     - decoder
 *     - physical overhead
 *     - syndrome schedule
 *     - hardware topology
 */
logicalQubitType
    : K_LOGICAL K_QUBIT
    ;


/* ============================================================================
 * 4. SCALABLE QUANTUM COLLECTION
 * ========================================================================== */

/**
 * Quantum collection.
 *
 * Examples:
 *
 *     qubit[N]
 *     qubit[n + 1]
 *     qubit[2 * n]
 *     logical qubit[algorithm_width]
 *
 * The expression is preserved by the parser.
 */
quantumCollectionType
    : quantumCollectionElementType
      LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


/**
 * Quantum collection element.
 *
 * Only source-level quantum resource types belong here.
 */
quantumCollectionElementType
    : quantumPrimitiveType
    | logicalQubitType
    ;


/* ============================================================================
 * 5. GENERAL QUANTUM CONSTRUCTOR
 * ========================================================================== */

/**
 * Open quantum type constructor.
 *
 * Examples:
 *
 *     quantum<State>
 *     quantum<Observable>
 *     quantum<Register>
 *     quantum<Amplitude>
 *     quantum<CustomQuantumType>
 *
 * The grammar deliberately does not enumerate future quantum technologies.
 */
quantumConstructorType
    : K_QUANTUM
      LESS_THAN
      quantumTypeArgumentList
      GREATER_THAN
    ;


/**
 * Quantum constructor arguments.
 *
 * An argument can be:
 *
 *     - a quantum type
 *     - a symbolic type/value expression
 *     - a qualified source name
 *
 * Semantic analysis determines whether the combination is legal.
 */
quantumTypeArgumentList
    : quantumTypeArgument
      (COMMA quantumTypeArgument)*
      COMMA?
    ;


quantumTypeArgument
    : quantumTypeArgumentType
    | quantumTypeValueExpression
    ;


quantumTypeArgumentType
    : quantumType
    | quantumQualifiedTypeArgument
    ;


/**
 * Qualified or named type used as a quantum constructor argument.
 *
 * It intentionally begins with IDENTIFIER rather than K_QUANTUM so that
 * user-defined types remain open-world.
 */
quantumQualifiedTypeArgument
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 6. EXPLICIT QUANTUM NAMESPACE
 * ========================================================================== */

/**
 * Explicit quantum namespace.
 *
 * Examples:
 *
 *     quantum::State
 *     quantum::Observable
 *     quantum::Register
 *     quantum::future::State
 *
 * The parser performs no name resolution.
 */
quantumQualifiedType
    : K_QUANTUM
      DOUBLE_COLON
      quantumQualifiedTypeTail
    ;


/**
 * Arbitrarily deep qualified quantum type.
 *
 * No namespace-depth limit is imposed.
 */
quantumQualifiedTypeTail
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 7. SYMBOLIC QUANTUM CARDINALITY
 * ========================================================================== */

/**
 * Cardinality expression.
 *
 * Examples:
 *
 *     N
 *     n + 1
 *     2 * n
 *     width * factor
 *     algorithm::width
 *     (N + M) * factor
 *
 * This grammar does not:
 *
 *     - evaluate the expression
 *     - allocate resources
 *     - convert it to usize
 *     - select physical qubits
 *     - select a device
 *     - establish a maximum
 */
quantumCardinalityExpression
    : quantumCardinalityAdditiveExpression
    ;


/**
 * Addition/subtraction precedence level.
 */
quantumCardinalityAdditiveExpression
    : quantumCardinalityMultiplicativeExpression
      (
          PLUS quantumCardinalityMultiplicativeExpression
        | MINUS quantumCardinalityMultiplicativeExpression
      )*
    ;


/**
 * Multiplication/division/modulo precedence level.
 */
quantumCardinalityMultiplicativeExpression
    : quantumCardinalityShiftExpression
      (
          STAR quantumCardinalityShiftExpression
        | SLASH quantumCardinalityShiftExpression
        | PERCENT quantumCardinalityShiftExpression
      )*
    ;


/**
 * Shift precedence level.
 */
quantumCardinalityShiftExpression
    : quantumCardinalityBitwiseExpression
      (
          LEFT_SHIFT quantumCardinalityBitwiseExpression
        | RIGHT_SHIFT quantumCardinalityBitwiseExpression
      )*
    ;


/**
 * Bitwise precedence level.
 *
 * These operators are syntactically permitted for symbolic integer
 * cardinality expressions. Semantic validation determines whether they are
 * valid for a particular type/value context.
 */
quantumCardinalityBitwiseExpression
    : quantumCardinalityUnaryExpression
      (
          AMPERSAND quantumCardinalityUnaryExpression
        | PIPE quantumCardinalityUnaryExpression
        | CARET quantumCardinalityUnaryExpression
      )*
    ;


/**
 * Unary cardinality expression.
 */
quantumCardinalityUnaryExpression
    : PLUS quantumCardinalityUnaryExpression
    | MINUS quantumCardinalityUnaryExpression
    | quantumCardinalityPrimary
    ;


/**
 * Primary cardinality expression.
 *
 * Integer literals remain lexical values.
 *
 * Identifiers remain symbolic.
 */
quantumCardinalityPrimary
    : INTEGER_LITERAL
    | IDENTIFIER
    | quantumQualifiedCardinalityName
    | quantumCardinalityParenthesized
    ;


/**
 * Qualified symbolic cardinality.
 *
 * Examples:
 *
 *     config::width
 *     algorithm::qubits
 *     module::parameter::N
 */
quantumQualifiedCardinalityName
    : IDENTIFIER
      DOUBLE_COLON
      IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/**
 * Parenthesized cardinality.
 */
quantumCardinalityParenthesized
    : LPAREN
      quantumCardinalityExpression
      RPAREN
    ;


/* ============================================================================
 * 8. TYPE-LEVEL QUANTUM VALUES
 * ========================================================================== */

/**
 * Symbolic type-level argument.
 *
 * This is intentionally a restricted, deterministic expression grammar.
 *
 * It is NOT the general runtime expression grammar.
 *
 * This separation prevents type parsing from importing the entire expression
 * grammar and creating grammar dependency cycles.
 */
quantumTypeValueExpression
    : quantumTypeValueAdditiveExpression
    ;


quantumTypeValueAdditiveExpression
    : quantumTypeValueMultiplicativeExpression
      (
          PLUS quantumTypeValueMultiplicativeExpression
        | MINUS quantumTypeValueMultiplicativeExpression
      )*
    ;


quantumTypeValueMultiplicativeExpression
    : quantumTypeValuePrimary
      (
          STAR quantumTypeValuePrimary
        | SLASH quantumTypeValuePrimary
        | PERCENT quantumTypeValuePrimary
      )*
    ;


quantumTypeValuePrimary
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | IDENTIFIER
    | quantumTypeValueQualifiedName
    | quantumTypeValueParenthesized
    ;


quantumTypeValueQualifiedName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


quantumTypeValueParenthesized
    : LPAREN
      quantumTypeValueExpression
      RPAREN
    ;


/* ============================================================================
 * 9. QUANTUM TYPE APPLICATION
 * ========================================================================== */

/**
 * This rule exists for semantic/type-system integration.
 *
 * It is deliberately NOT a second generic-type grammar.
 *
 * General generic application remains owned by grammar/types/generic.g4 and
 * grammar/types/types.g4.
 *
 * This rule only describes the quantum-specific constructor spelling.
 */
quantumTypeApplication
    : quantumConstructorType
    ;


/* ============================================================================
 * 10. LEGACY SOURCE COMPATIBILITY
 * ========================================================================== */

/**
 * These historical forms are intentionally NOT lexer keywords:
 *
 *     Qubit
 *     QRegister
 *     QState
 *     QuantumRegister
 *     LogicalQubit
 *     LogicalRegister
 *     QuantumState
 *
 * They are represented through the universal named/generic type grammar:
 *
 *     Qubit
 *     QRegister<N>
 *     QState<T>
 *     QuantumRegister<N>
 *     LogicalQubit
 *     LogicalRegister<N>
 *     QuantumState<T>
 *
 * This rule is documentation-only in architectural intent; the actual
 * compatibility parsing is provided by the universal named/generic type
 * grammar.
 *
 * No parser alternatives are added here because adding IDENTIFIER as a
 * quantumType alternative would steal ordinary user-defined types.
 */


/* ============================================================================
 * 11. SOURCE-LEVEL QUANTUM TYPE GUARANTEES
 * ========================================================================== */

/**
 * The following constructs MUST remain representable by the overall type
 * system:
 *
 *     qubit
 *     logical qubit
 *     qubit[N]
 *     logical qubit[N]
 *     quantum<State>
 *     quantum<Observable>
 *     quantum<Register, N>
 *     quantum::State
 *     quantum::State<N>
 *
 * The following are deliberately NOT parser-level type constructs:
 *
 *     physical qubit 17
 *     qpu 0
 *     device 0
 *     topology 0
 *     coupling map
 *     calibration record
 *     vendor ABI
 *
 * Such information belongs to the hardware/resource/target layers.
 */


/* ============================================================================
 * 12. HARD-CODING PROHIBITION
 * ========================================================================== */

/**
 * There must be no grammar rules equivalent to:
 *
 *     qubitCount32
 *     qubitCount64
 *     register1024
 *     qpu0
 *     qpu1
 *     physicalQubit0
 *     physicalQubit1
 *
 * and no grammar constants representing machine capacity.
 *
 * The language's scalability boundary is semantic and resource-aware, not
 * parser-sized.
 */


/* ============================================================================
 * 13. DOWNSTREAM INTEGRATION
 * ========================================================================== */

/**
 * Required lowering:
 *
 *     quantumType
 *          |
 *          v
 *     frontend TypeExpr
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic quantum type
 *          |
 *          v
 *     canonical quantum::ir
 *
 * The grammar MUST NOT directly import:
 *
 *     src/quantum/ir
 *     QEC
 *     ZQN
 *     HAL
 *     routing
 *     scheduling
 *     calibration
 *     runtime
 *
 * Those dependencies would invert the compiler architecture.
 */


/* ============================================================================
 * 14. DIAGNOSTIC CONTRACT
 * ========================================================================== */

/**
 * The grammar should preserve enough parse structure and source spans for
 * semantic diagnostics to distinguish at least:
 *
 *     invalid cardinality
 *     invalid quantum type argument
 *     malformed qualified quantum type
 *     malformed quantum collection
 *     unexpected generic delimiter
 *
 * Semantic diagnostics must additionally distinguish:
 *
 *     unknown quantum type
 *     invalid type argument
 *     unsupported semantic combination
 *     unavailable resource
 *     unsupported target capability
 *
 * The parser must not convert these semantic failures into hardware-specific
 * parser errors.
 */


/* ============================================================================
 * 15. CONFORMANCE CONTRACT
 * ========================================================================== */

/**
 * Positive examples:
 *
 *     qubit
 *     logical qubit
 *     qubit[N]
 *     qubit[n + 1]
 *     qubit[2 * n]
 *     logical qubit[algorithm_width]
 *     quantum<State>
 *     quantum<State, N>
 *     quantum::State
 *     quantum::State<N>
 *     quantum::future::State
 *
 * Negative examples:
 *
 *     qubit[]
 *     qubit[N +]
 *     qubit[* N]
 *     logical
 *     quantum<>
 *     quantum<State,
 *     quantum::
 *
 * Scalability examples:
 *
 *     qubit[n]
 *     qubit[2 * n + offset]
 *     qubit[algorithm::required_width]
 *     logical qubit[workload::width * replication]
 *
 * The tests MUST NOT use a finite maximum as the definition of correctness.
 */


/* ============================================================================
 * 16. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is complete when:
 *
 *     [x] It has one public quantumType entry point.
 *     [x] It consumes only ZamaniTokens.
 *     [x] It defines no lexer rules.
 *     [x] It has no hardware limits.
 *     [x] It has no physical device assumptions.
 *     [x] It supports symbolic quantum cardinality.
 *     [x] It supports logical qubits.
 *     [x] It supports scalable quantum collections.
 *     [x] It supports generic quantum constructors.
 *     [x] It supports qualified quantum types.
 *     [x] It permits arbitrary namespace depth.
 *     [x] It does not steal ordinary identifiers from Types.
 *     [x] It does not duplicate quantum operations.
 *     [x] It does not duplicate quantum IR.
 *     [x] It preserves the quantum::ir boundary.
 *     [x] It remains independent of QEC/ZQN/HAL/routing/scheduling.
 *     [x] It is independent of Rust implementation details.
 *     [x] It requires no unsafe Rust.
 *
 * Integration completion additionally requires the repository changes listed
 * below this file.
 */