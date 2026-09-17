/*
 * ============================================================================
 * Zamani Universal Programming Language
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
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the SINGLE CANONICAL OWNER of quantum-specific SOURCE TYPE
 * SYNTAX in Zamani.
 *
 * It is intentionally independent of:
 *
 *     - physical hardware;
 *     - QPU selection;
 *     - device identifiers;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC implementation;
 *     - ZQN;
 *     - HAL;
 *     - runtime representation;
 *     - vendor APIs;
 *     - quantum::ir.
 *
 * The universal type composition layer remains owned by:
 *
 *     grammar/types/types.g4
 *
 * Generic type application remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * This grammar supplies only the quantum-specific type forms which those
 * universal grammars cannot express without knowing that a construct is
 * quantum-specific.
 *
 * ============================================================================
 * CANONICAL SOURCE FORMS
 * ============================================================================
 *
 *     qubit
 *     logical qubit
 *
 *     qubit[N]
 *     logical qubit[N]
 *
 *     qubit[n + 1]
 *     qubit[2 * n]
 *     qubit[algorithm_width]
 *     qubit[(N + M) * factor]
 *
 *     quantum<T>
 *     quantum<T, P>
 *
 *     quantum::Type
 *     quantum::state::Type
 *     quantum::future::namespace::Type
 *
 * Generic application of a qualified quantum type is composed by the
 * universal generic grammar:
 *
 *     quantum::Type<T>
 *     quantum::state::StateVector<T>
 *
 * This prevents this file from becoming a second generic-type grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - abstract `qubit` type syntax;
 *     - `logical qubit` type syntax;
 *     - scalable quantum collection syntax;
 *     - symbolic quantum cardinality syntax;
 *     - the source-level `quantum<...>` constructor;
 *     - explicit `quantum::...` type qualification;
 *     - quantum-specific parser boundaries;
 *     - quantum type syntax compatibility.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - universal generic type application;
 *     - ordinary named types;
 *     - primitive classical types;
 *     - tuples;
 *     - arrays;
 *     - references;
 *     - pointers;
 *     - function types;
 *     - resource types;
 *     - capability types;
 *     - hardware types;
 *     - quantum operations;
 *     - gates;
 *     - circuits;
 *     - measurement statements;
 *     - reset;
 *     - dynamic control;
 *     - QEC;
 *     - ZQN;
 *     - noise semantics;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware allocation;
 *     - physical qubit IDs;
 *     - topology;
 *     - backend selection;
 *     - runtime representation;
 *     - ABI representation;
 *     - canonical IR.
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
 *     canonical parser
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     universal Types                  Quantum
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                     frontend AST
 *                       TypeExpr
 *                          |
 *                          v
 *                  structural validation
 *                          |
 *                          v
 *                  semantic type analysis
 *                          |
 *             +------------+-------------+
 *             |                          |
 *             v                          v
 *      resource/capability          quantum semantics
 *             |                          |
 *             +------------+-------------+
 *                          |
 *                          v
 *                    canonical IR
 *                          |
 *                          v
 *                     quantum::ir
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        optimization   routing     scheduling
 *                                      |
 *                                      v
 *                              QEC / ZQN / HAL
 *                                      |
 *                                      v
 *                              target realization
 *
 * IMPORTANT:
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT create a second quantum IR.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The grammar expresses portable computational intent.
 *
 * It MUST NOT establish implementation limits such as:
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
 *     MAX_GATE_COUNT
 *
 * It also MUST NOT encode:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     qpu0
 *     gpu0
 *     device0
 *     vendor topology
 *     fixed connectivity
 *     vendor gate inventories.
 *
 * There is no language-level finite limit on:
 *
 *     - cardinality values;
 *     - namespace depth;
 *     - type nesting;
 *     - generic arity;
 *     - symbolic expression size.
 *
 * Actual parser/compiler resource limits, if required for operational safety,
 * are implementation configuration and MUST NOT become language semantics.
 *
 * ============================================================================
 * CARDINALITY SEMANTICS
 * ============================================================================
 *
 *     qubit[N]
 *
 * means that the source program contains a quantum resource requirement
 * parameterized by N.
 *
 * It does NOT mean:
 *
 *     physical qubits 0 through N-1
 *
 * and it does not select a machine.
 *
 * Examples:
 *
 *     qubit[1]
 *     qubit[1024]
 *     qubit[N]
 *     qubit[2 * N]
 *     qubit[algorithm_width]
 *     qubit[(N + M) * factor]
 *
 * are all source-level forms.
 *
 * Semantic analysis determines:
 *
 *     - whether the expression is valid;
 *     - whether it is statically known;
 *     - whether it is symbolic;
 *     - whether it is runtime-dependent;
 *     - what resource requirement it creates;
 *     - whether a target can satisfy it.
 *
 * This grammar never converts the value into:
 *
 *     usize
 *     u32
 *     u64
 *
 * or any other Rust implementation type.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar is parser-only.
 *
 * Its token vocabulary is:
 *
 *     ZamaniTokens
 *
 * supplied by:
 *
 *     grammar/lexer/tokens.g4
 *
 * Relevant canonical parser-facing token names are expected to include:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_LOGICAL
 *
 *     IDENTIFIER
 *     INTEGER_LITERAL
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
 * This grammar MUST NOT declare lexer rules.
 *
 * Token spelling belongs exclusively to grammar/lexer/.
 *
 * ============================================================================
 * LEXER MIGRATION REQUIREMENT
 * ============================================================================
 *
 * The repository currently contains an inconsistency between the modular
 * parser-facing K_* vocabulary and the older keyword vocabulary.
 *
 * The canonical lexer layer currently contains spellings such as:
 *
 *     QUANTUM
 *     QUBIT
 *     LOGICAL
 *
 * while the modular parser architecture uses:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_LOGICAL
 *
 * This grammar intentionally does NOT introduce local aliases.
 *
 * REQUIRED LEXER WORK:
 *
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * must be reconciled so that exactly ONE canonical parser-facing token
 * identity exists for each language-level keyword.
 *
 * The intended parser-facing vocabulary is:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_LOGICAL
 *
 * Existing historical token names may be retained temporarily in the Rust
 * compatibility adapter, but they must not become a second parser grammar
 * vocabulary.
 *
 * Do NOT add:
 *
 *     QUBIT | K_QUBIT
 *     QUANTUM | K_QUANTUM
 *     LOGICAL | K_LOGICAL
 *
 * as competing parser alternatives.
 *
 * The lexer is the sole lexical authority.
 *
 * ============================================================================
 * SOURCE COMPATIBILITY
 * ============================================================================
 *
 * Canonical lowercase source forms:
 *
 *     qubit
 *     logical qubit
 *     quantum<T>
 *     quantum::Type
 *
 * Historical names such as:
 *
 *     Qubit
 *     QRegister
 *     QState
 *     QuantumRegister
 *     LogicalQubit
 *     QuantumState
 *
 * MUST remain ordinary named types unless an explicit compatibility
 * specification says otherwise.
 *
 * They must not cause permanent expansion of the reserved keyword set.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser structure only.
 *
 * It maps into the existing domain-neutral frontend TypeExpr architecture.
 *
 * Conceptual mappings:
 *
 *     qubit
 *         -> existing quantum TypeExpr representation
 *
 *     logical qubit
 *         -> existing logical quantum TypeExpr representation
 *
 *     qubit[N]
 *         -> existing parameterized/collection quantum TypeExpr
 *
 *     logical qubit[N]
 *         -> existing parameterized/collection logical quantum TypeExpr
 *
 *     quantum<T>
 *         -> existing generic/quantum TypeExpr representation
 *
 *     quantum::Type
 *         -> existing qualified/named TypeExpr
 *
 * Generic:
 *
 *     quantum::Type<T>
 *
 * is composed by the universal generic type grammar and therefore lowers to
 * the existing generic TypeExpr representation.
 *
 * The grammar MUST NOT introduce:
 *
 *     QuantumTypeIR
 *     QuantumTypeNode
 *     QuantumRegisterIR
 *     PhysicalQubitIR
 *     QuantumHardwareTypeIR
 *
 * merely to represent syntax.
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
 * The parser therefore does NOT decide:
 *
 *     - physical implementation;
 *     - logical encoding;
 *     - QEC code;
 *     - code distance;
 *     - decoder;
 *     - simulator representation;
 *     - vendor;
 *     - QPU;
 *     - topology;
 *     - calibration;
 *     - routing;
 *     - scheduling.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This grammar has no dependency on quantum::ir.
 *
 * The required pipeline is:
 *
 *     parser
 *       |
 *       v
 *     TypeExpr
 *       |
 *       v
 *     semantic quantum type
 *       |
 *       v
 *     quantum::ir
 *
 * QEC, ZQN, routing, scheduling, optimization and HAL remain downstream
 * consumers.
 *
 * ============================================================================
 * GENERIC-TYPE CONTRACT
 * ============================================================================
 *
 * Universal generic application is owned by:
 *
 *     grammar/types/generic.g4
 *
 * Therefore this grammar MUST NOT duplicate:
 *
 *     genericType
 *     genericTypeArguments
 *     genericArgumentList
 *
 * Examples:
 *
 *     quantum::State<T>
 *     quantum::Register<Qubit>
 *     quantum::State<SomeType>
 *
 * are composed as:
 *
 *     quantum-qualified type
 *          +
 *     universal generic application
 *
 * This avoids a second generic type system.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     - no actions;
 *     - no predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware queries;
 *     - no runtime queries;
 *     - no randomness.
 *
 * The same token stream under the same grammar version must produce the same
 * parse structure.
 *
 * ============================================================================
 */


/* ============================================================================
 * GRAMMAR DECLARATION
 * ========================================================================== */

parser grammar Quantum;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical quantum-specific type syntax.
 *
 * Ordinary identifiers are intentionally excluded.
 *
 * This prevents Quantum from stealing every named type from the universal
 * Types grammar.
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
 * Abstract quantum resource.
 *
 *     qubit
 *
 * This does not select a physical qubit.
 */
quantumPrimitiveType
    : K_QUBIT
    ;


/* ============================================================================
 * 3. LOGICAL QUBIT
 * ========================================================================== */

/**
 * Logical-level quantum resource.
 *
 *     logical qubit
 *
 * This does not select:
 *
 *     - a QEC code;
 *     - code distance;
 *     - decoder;
 *     - physical overhead;
 *     - topology.
 */
logicalQubitType
    : K_LOGICAL
      K_QUBIT
    ;


/* ============================================================================
 * 4. SCALABLE QUANTUM COLLECTION
 * ========================================================================== */

/**
 * Parameterized quantum resource collection.
 *
 *     qubit[N]
 *     logical qubit[N]
 *
 * The cardinality remains a source expression.
 */
quantumCollectionType
    : quantumCollectionElementType
      LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


/**
 * Collection element must itself be a source-level quantum resource type.
 */
quantumCollectionElementType
    : quantumPrimitiveType
    | logicalQubitType
    ;


/* ============================================================================
 * 5. QUANTUM TYPE CONSTRUCTOR
 * ========================================================================== */

/**
 * Open quantum constructor.
 *
 * Examples:
 *
 *     quantum<T>
 *     quantum<T, P>
 *
 * Arguments are quantum/type arguments only.
 *
 * General generic applications remain owned by Generic.
 */
quantumConstructorType
    : K_QUANTUM
      LESS_THAN
      quantumConstructorArgumentList
      GREATER_THAN
    ;


/**
 * One or more quantum constructor arguments.
 *
 * A trailing comma is supported consistently with the repository's generic
 * type grammar.
 *
 *     quantum<T,>
 *     quantum<T, P,>
 *
 * is therefore syntactically valid.
 *
 * Semantic validation determines whether the argument list is meaningful.
 */
quantumConstructorArgumentList
    : quantumConstructorArgument
      (COMMA quantumConstructorArgument)*
      COMMA?
    ;


/**
 * Quantum constructor arguments.
 *
 * This grammar intentionally accepts:
 *
 *     quantum types;
 *     qualified/named type references;
 *     symbolic integer type-level values.
 *
 * The distinction between a named type and a symbolic semantic name is made
 * during semantic type analysis, not by introducing another AST.
 */
quantumConstructorArgument
    : quantumType
    | quantumConstructorNamedType
    | quantumConstructorValue
    ;


/**
 * Named/qualified type argument.
 *
 * Examples:
 *
 *     quantum<State>
 *     quantum<Observable>
 *     quantum<custom::State>
 *
 * Generic application of such a type is handled by the universal generic
 * grammar when it occurs as a complete type expression.
 */
quantumConstructorNamedType
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/**
 * Symbolic integer type-level value.
 *
 * This remains deliberately restricted.
 *
 * Floating-point values are not accepted as quantum cardinality/type values.
 * TypeExpr currently represents generic arguments as types, while quantum
 * cardinalities have their own semantic representation.
 *
 * A future dependent type/value model may extend this boundary centrally.
 */
quantumConstructorValue
    : quantumCardinalityExpression
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
 *     quantum::state::StateVector
 *     quantum::future::namespace::Type
 *
 * Name resolution is semantic.
 */
quantumQualifiedType
    : K_QUANTUM
      DOUBLE_COLON
      quantumQualifiedName
    ;


/**
 * Arbitrarily deep qualified quantum name.
 *
 * There is no grammar-level namespace-depth limit.
 */
quantumQualifiedName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 7. QUANTUM CARDINALITY EXPRESSIONS
 * ========================================================================== */

/**
 * Quantum cardinality expression.
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
 * Operator precedence is explicitly represented so that the parse tree
 * preserves the intended structure.
 */
quantumCardinalityExpression
    : quantumCardinalityBitwiseOrExpression
    ;


/* ---------------------------------------------------------------------------
 * Bitwise OR
 * ------------------------------------------------------------------------- */

quantumCardinalityBitwiseOrExpression
    : quantumCardinalityBitwiseXorExpression
      (
          PIPE
          quantumCardinalityBitwiseXorExpression
      )*
    ;


/* ---------------------------------------------------------------------------
 * Bitwise XOR
 * ------------------------------------------------------------------------- */

quantumCardinalityBitwiseXorExpression
    : quantumCardinalityBitwiseAndExpression
      (
          CARET
          quantumCardinalityBitwiseAndExpression
      )*
    ;


/* ---------------------------------------------------------------------------
 * Bitwise AND
 * ------------------------------------------------------------------------- */

quantumCardinalityBitwiseAndExpression
    : quantumCardinalityShiftExpression
      (
          AMPERSAND
          quantumCardinalityShiftExpression
      )*
    ;


/* ---------------------------------------------------------------------------
 * Shift
 * ------------------------------------------------------------------------- */

quantumCardinalityShiftExpression
    : quantumCardinalityAdditiveExpression
      (
          LEFT_SHIFT
          quantumCardinalityAdditiveExpression
        | RIGHT_SHIFT
          quantumCardinalityAdditiveExpression
      )*
    ;


/* ---------------------------------------------------------------------------
 * Addition / subtraction
 * ------------------------------------------------------------------------- */

quantumCardinalityAdditiveExpression
    : quantumCardinalityMultiplicativeExpression
      (
          PLUS
          quantumCardinalityMultiplicativeExpression
        | MINUS
          quantumCardinalityMultiplicativeExpression
      )*
    ;


/* ---------------------------------------------------------------------------
 * Multiplication / division / modulo
 * ------------------------------------------------------------------------- */

quantumCardinalityMultiplicativeExpression
    : quantumCardinalityUnaryExpression
      (
          STAR
          quantumCardinalityUnaryExpression
        | SLASH
          quantumCardinalityUnaryExpression
        | PERCENT
          quantumCardinalityUnaryExpression
      )*
    ;


/* ---------------------------------------------------------------------------
 * Unary operators
 * ------------------------------------------------------------------------- */

quantumCardinalityUnaryExpression
    : PLUS
      quantumCardinalityUnaryExpression
    | MINUS
      quantumCardinalityUnaryExpression
    | quantumCardinalityPrimary
    ;


/* ---------------------------------------------------------------------------
 * Primary
 * ------------------------------------------------------------------------- */

quantumCardinalityPrimary
    : INTEGER_LITERAL
    | quantumQualifiedCardinalityName
    | IDENTIFIER
    | quantumCardinalityParenthesized
    ;


/**
 * Qualified symbolic cardinality.
 *
 * Examples:
 *
 *     configuration::width
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
 * 8. EXPLICIT COMPATIBILITY BOUNDARY
 * ========================================================================== */

/**
 * Compatibility spelling for tooling that wants a named rule for a quantum
 * resource cardinality.
 *
 * It delegates directly to the canonical expression.
 */
quantumExtent
    : LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


/* ============================================================================
 * 9. SEMANTIC CATEGORY BOUNDARY
 * ========================================================================== */

/**
 * These concepts intentionally remain qualified names rather than dedicated
 * keyword inventories:
 *
 *     quantum::state::StateVector
 *     quantum::state::DensityMatrix
 *     quantum::observable::Observable
 *     quantum::measurement::Result
 *     quantum::qec::LogicalQubit
 *     quantum::qec::Syndrome
 *     quantum::noise::Channel
 *     quantum::noise::Model
 *     quantum::resource::Register
 *     quantum::photonic::Mode
 *     quantum::bosonic::Mode
 *     quantum::topological::Qubit
 *
 * No rules are created for these individual semantic categories.
 *
 * This keeps the language open to future quantum technologies without
 * repeatedly changing the core grammar or lexer.
 */


/* ============================================================================
 * 10. PHYSICAL / LOGICAL SEPARATION
 * ========================================================================== */

/**
 * `logical qubit` is intentionally a source-level abstraction.
 *
 * A physical realization such as:
 *
 *     physical qubit 17
 *
 * is NOT a quantum type production.
 *
 * Physical mapping belongs to:
 *
 *     hardware/
 *     resources/
 *     compile/
 *     routing/
 *     scheduling/
 *     HAL
 *
 * A physical-oriented quantum type, if required by the language specification,
 * should normally be represented through an open qualified semantic type such
 * as:
 *
 *     quantum::physical::Qubit
 *
 * rather than introducing a hard-coded physical-ID grammar.
 */


/* ============================================================================
 * 11. NO FIXED GATE SET
 * ========================================================================== */

/**
 * This grammar deliberately does NOT contain:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * Gate and operation syntax belongs to the quantum operation grammar.
 *
 * Gate availability is a semantic capability question.
 */


/* ============================================================================
 * 12. NO QEC IMPLEMENTATION
 * ========================================================================== */

/**
 * This grammar does NOT encode:
 *
 *     - QEC code;
 *     - code distance;
 *     - decoder;
 *     - syndrome extraction;
 *     - physical-to-logical ratio;
 *     - correction schedule;
 *     - fault threshold.
 *
 * A logical qubit expresses logical-level intent.
 *
 * QEC selection remains downstream.
 */


/* ============================================================================
 * 13. NO ZQN IMPLEMENTATION
 * ========================================================================== */

/**
 * This grammar does NOT implement:
 *
 *     - noise models;
 *     - fault classification;
 *     - leakage;
 *     - erasure;
 *     - correlated noise;
 *     - calibration;
 *     - reliability thresholds.
 *
 * Qualified semantic types may refer to ZQN concepts, but ZQN owns their
 * meaning.
 */


/* ============================================================================
 * 14. NO HARDWARE OWNERSHIP
 * ========================================================================== */

/**
 * The grammar never:
 *
 *     - queries hardware;
 *     - selects a device;
 *     - allocates a QPU;
 *     - assigns physical qubit IDs;
 *     - inspects topology;
 *     - selects a simulator;
 *     - selects a vendor backend.
 */


/* ============================================================================
 * 15. RESOURCE INTEGRATION
 * ========================================================================== */

/**
 * Example:
 *
 *     qubit[N]
 *
 * may produce a semantic resource requirement equivalent to:
 *
 *     quantum-resource-cardinality = N
 *
 * after semantic analysis.
 *
 * The grammar itself does not allocate N resources.
 */


/* ============================================================================
 * 16. COMPILER INTEGRATION
 * ========================================================================== */

/**
 * Compiler responsibilities after parsing include:
 *
 *     - name resolution;
 *     - type resolution;
 *     - cardinality validation;
 *     - resource analysis;
 *     - capability checking;
 *     - target feasibility;
 *     - specialization;
 *     - optimization;
 *     - quantum lowering.
 *
 * None of those operations belong in this grammar.
 */


/* ============================================================================
 * 17. RUNTIME INTEGRATION
 * ========================================================================== */

/**
 * A source-level `qubit` may ultimately be represented as:
 *
 *     - an abstract runtime handle;
 *     - a simulator resource;
 *     - a logical resource;
 *     - a provider resource;
 *     - a distributed resource;
 *     - another target-specific representation.
 *
 * The grammar does not choose among these representations.
 */


/* ============================================================================
 * 18. TOOLING / SOURCE-SPAN CONTRACT
 * ========================================================================== */

/**
 * Parser/AST tooling must preserve source spans for:
 *
 *     quantumType
 *     quantumPrimitiveType
 *     logicalQubitType
 *     quantumCollectionType
 *     quantumConstructorType
 *     quantumQualifiedType
 *     quantumCardinalityExpression
 *
 * This supports:
 *
 *     - diagnostics;
 *     - IDE navigation;
 *     - formatting;
 *     - refactoring;
 *     - provenance;
 *     - source maps.
 */


/* ============================================================================
 * 19. ERROR / NEGATIVE CONTRACT
 * ========================================================================== */

/**
 * These must be rejected structurally:
 *
 *     logical
 *     qubit[
 *     qubit[]
 *     qubit[N
 *     qubit[N +]
 *     qubit[*]
 *     quantum<
 *     quantum<>
 *     quantum<T
 *     quantum:::
 *     quantum::
 *
 * The semantic layer must additionally reject invalid meanings such as:
 *
 *     negative cardinality;
 *     zero cardinality where the type system forbids it;
 *     non-integral cardinality;
 *     unknown quantum type;
 *     invalid quantum generic arguments.
 *
 * Those semantic constraints must NOT become parser hard limits.
 */


/* ============================================================================
 * 20. BOUNDARY / SCALABILITY CONTRACT
 * ========================================================================== */

/**
 * Positive boundary examples:
 *
 *     qubit[1]
 *     qubit[1024]
 *     qubit[N]
 *     qubit[2 * N]
 *     qubit[(N + M) * factor]
 *     logical qubit[N]
 *
 * Namespace examples:
 *
 *     quantum::State
 *     quantum::state::State
 *     quantum::future::deep::namespace::Type
 *
 * Constructor examples:
 *
 *     quantum<Qubit>
 *     quantum<LogicalQubit>
 *     quantum<State>
 *     quantum<T, P>
 *
 * No source-level upper bound is encoded.
 */


/* ============================================================================
 * 21. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * The old:
 *
 *     grammar/types/quantum-types.g4
 *
 * MUST NOT remain a second authoritative implementation.
 *
 * Migration:
 *
 *     QuantumTypes consumers
 *          |
 *          v
 *     Quantum.quantumType
 *
 * After all references are migrated, `quantum-types.g4` should be removed.
 *
 * This file retains the canonical filename:
 *
 *     grammar/types/quantum.g4
 *
 * No unnecessary rename is required.
 */


/* ============================================================================
 * 22. INTEGRATION WITH TYPES
 * ========================================================================== */

/**
 * grammar/types/types.g4 remains the universal type composition owner.
 *
 * It must expose one canonical type-expression entry point and delegate the
 * quantum-specific branch to:
 *
 *     quantumType
 *
 * It must NOT copy these rules into types.g4.
 *
 * It must also compose Generic independently so that:
 *
 *     quantum::Type<T>
 *
 * is parsed as:
 *
 *     quantumQualifiedType
 *          +
 *     genericType
 *
 * rather than by duplicating generic syntax here.
 */


/* ============================================================================
 * 23. INTEGRATION WITH GENERIC
 * ========================================================================== */

/**
 * grammar/types/generic.g4 already owns:
 *
 *     genericType
 *     genericTypeArguments
 *     genericArgumentList
 *
 * Therefore this grammar intentionally does not redefine those rules.
 *
 * The combined type grammar must make the following possible:
 *
 *     quantum::Type<T>
 *     quantum::State<StateVector>
 *     quantum::Register<Qubit>
 *     quantum::Register<LogicalQubit>
 *
 * through the canonical universal generic grammar.
 */


/* ============================================================================
 * 24. INTEGRATION WITH FRONTEND AST
 * ========================================================================== */

/**
 * The frontend must map this grammar into the existing TypeExpr hierarchy.
 *
 * Required rule-to-AST mapping must be documented and tested before the
 * grammar is marked complete.
 *
 * No grammar rule may require creation of a second quantum-specific AST.
 */


/* ============================================================================
 * 25. INTEGRATION WITH QUANTUM IR
 * ========================================================================== */

/**
 * No quantum::ir types are referenced here.
 *
 * Required downstream path:
 *
 *     Quantum parser rule
 *          |
 *          v
 *     TypeExpr
 *          |
 *          v
 *     semantic quantum type
 *          |
 *          v
 *     quantum::ir
 *
 * quantum::ir remains the single canonical quantum semantic boundary.
 */


/* ============================================================================
 * 26. RUST CONTRACT
 * ========================================================================== */

/**
 * This grammar contains no Rust implementation code.
 *
 * Generated/integrating Rust must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The Rust implementation must remain safe Rust.
 *
 * The repository-level Rust contract must enforce:
 *
 *     #![deny(unsafe_code)]
 *
 * No `unsafe` code is required by this grammar.
 */


/* ============================================================================
 * 27. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * This grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_REGISTER_SIZE
 *     MAX_STATE_SIZE
 *     MAX_QPU_COUNT
 *     MAX_DEVICE_COUNT
 *     MAX_TOPOLOGY_SIZE
 *     MAX_GATE_COUNT
 *     fixed namespace depth
 *     fixed generic arity
 *     fixed cardinality
 *
 * Numeric literals are ordinary source values.
 *
 * A value such as:
 *
 *     qubit[1024]
 *
 * is program semantics, not a language limit.
 */


/* ============================================================================
 * 28. SECURITY / DETERMINISM AUDIT
 * ========================================================================== */

/**
 * No:
 *
 *     - embedded actions;
 *     - semantic predicates;
 *     - filesystem access;
 *     - network access;
 *     - environment access;
 *     - hardware access;
 *     - randomness;
 *     - backend queries.
 *
 * The grammar is declarative and deterministic.
 */


/* ============================================================================
 * 29. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is complete when ALL of the following are true:
 *
 * [ ] grammar name is Quantum;
 *
 * [ ] filename remains grammar/types/quantum.g4;
 *
 * [ ] tokenVocab is ZamaniTokens;
 *
 * [ ] no lexer rules are declared;
 *
 * [ ] `qubit` parses;
 *
 * [ ] `logical qubit` parses;
 *
 * [ ] `qubit[N]` parses;
 *
 * [ ] `logical qubit[N]` parses;
 *
 * [ ] symbolic cardinalities parse;
 *
 * [ ] cardinality operator precedence is preserved;
 *
 * [ ] arbitrary qualified quantum names parse;
 *
 * [ ] `quantum<T>` parses;
 *
 * [ ] `quantum<T, P>` parses;
 *
 * [ ] nested quantum constructors are supported where syntactically valid;
 *
 * [ ] generic application remains owned by Generic;
 *
 * [ ] `quantum::Type<T>` is composed through the universal generic grammar;
 *
 * [ ] no fixed gate set exists here;
 *
 * [ ] no physical qubit IDs exist here;
 *
 * [ ] no device IDs exist here;
 *
 * [ ] no vendor topology exists here;
 *
 * [ ] no QEC implementation exists here;
 *
 * [ ] no ZQN implementation exists here;
 *
 * [ ] no routing exists here;
 *
 * [ ] no scheduling exists here;
 *
 * [ ] no HAL dependency exists here;
 *
 * [ ] no quantum::ir dependency exists here;
 *
 * [ ] no artificial resource limits exist;
 *
 * [ ] frontend TypeExpr mapping is documented;
 *
 * [ ] quantum::ir mapping is documented downstream;
 *
 * [ ] positive tests exist;
 *
 * [ ] negative tests exist;
 *
 * [ ] boundary tests exist;
 *
 * [ ] scalability tests exist;
 *
 * [ ] determinism tests exist;
 *
 * [ ] compatibility tests exist;
 *
 * [ ] lexer token migration is complete;
 *
 * [ ] all consumers of QuantumTypes have migrated;
 *
 * [ ] grammar/types/quantum-types.g4 can then be deleted;
 *
 * [ ] the grammar compiles under the repository's ANTLR build;
 *
 * [ ] generated Rust remains compatible with Rust 1.97/1.97.1;
 *
 * [ ] no unsafe Rust is introduced.
 */