/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/types/quantum.g4
 *
 * GRAMMAR
 * -------
 * Quantum
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION SOURCE-LEVEL QUANTUM TYPE GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE OWNER of quantum-specific source-level type syntax.
 *
 * It is composed by the universal Zamani type system and provides the
 * quantum-specific type forms that cannot be expressed by ordinary named,
 * generic, tuple, array, reference, or function types alone.
 *
 * This file describes SOURCE SEMANTICS ONLY.
 *
 * It does not describe:
 *
 *   - physical hardware;
 *   - QPU selection;
 *   - simulator selection;
 *   - physical qubit allocation;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - pulse implementation;
 *   - QEC implementation;
 *   - decoder implementation;
 *   - ZQN implementation;
 *   - HAL implementation;
 *   - runtime representation;
 *   - vendor APIs;
 *   - canonical quantum IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *                    +---------+---------+
 *                    |                   |
 *                    v                   v
 *              universal Types       Quantum
 *                    |                   |
 *                    +---------+---------+
 *                              |
 *                              v
 *                     domain-neutral AST
 *                              |
 *                              v
 *                     semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *          type system     resources       capabilities
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                     canonical semantic IR
 *                              |
 *                              v
 *                         quantum::ir
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        optimization       routing        scheduling
 *                              |
 *                              v
 *                         QEC / ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                      target realization
 *
 * ============================================================================
 * SINGLE-OWNER INVARIANT
 * ============================================================================
 *
 * This file owns:
 *
 *   - `qubit`;
 *   - `logical qubit`;
 *   - quantum collection type syntax;
 *   - symbolic quantum cardinality syntax;
 *   - open quantum type constructors;
 *   - qualified quantum type names;
 *   - quantum-specific type wrappers;
 *   - quantum type syntax extension points.
 *
 * It does NOT own:
 *
 *   - ordinary generic application;
 *   - ordinary named types;
 *   - arrays;
 *   - tuples;
 *   - references;
 *   - pointers;
 *   - function types;
 *   - resource declarations;
 *   - capability declarations;
 *   - qubit declarations;
 *   - quantum operations;
 *   - gates;
 *   - circuits;
 *   - measurement;
 *   - reset;
 *   - dynamic control;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - hardware realization.
 *
 * There MUST NOT be another authoritative `quantumType` implementation.
 *
 * In particular:
 *
 *   grammar/quantum/quantum-types.g4
 *
 * must not remain an independently authoritative implementation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Quantum types are target-independent.
 *
 * The same source type must be capable of being lowered to:
 *
 *   - a simulator representation;
 *   - a logical quantum representation;
 *   - a physical quantum representation;
 *   - a distributed representation;
 *   - a heterogeneous representation;
 *   - a future quantum architecture.
 *
 * The source type never selects the physical realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal hardware capacity.
 *
 * It contains no:
 *
 *   MAX_QUBITS
 *   MAX_LOGICAL_QUBITS
 *   MAX_PHYSICAL_QUBITS
 *   MAX_REGISTER_SIZE
 *   MAX_STATE_SIZE
 *   MAX_CIRCUIT_WIDTH
 *   MAX_CIRCUIT_DEPTH
 *   MAX_QPU_COUNT
 *   MAX_DEVICE_COUNT
 *   MAX_GATE_COUNT
 *   MAX_TYPE_ARGUMENTS
 *   MAX_NAMESPACE_DEPTH
 *   MAX_CARDINALITY
 *
 * There is no grammar-level finite limit on:
 *
 *   - number of quantum resources;
 *   - type nesting;
 *   - namespace depth;
 *   - generic arity;
 *   - symbolic cardinality expression size.
 *
 * Actual resource limitations are discovered downstream.
 *
 * A source expression such as:
 *
 *     qubit[1024]
 *
 * is program data.
 *
 * It is NOT a language limit.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * These are different concepts:
 *
 *     qubit[N]
 *         source-level type/resource cardinality
 *
 *     requires qubits >= N
 *         resource requirement
 *
 *     requires capability("quantum.measurement")
 *         capability requirement
 *
 *     prefer accelerator("quantum")
 *         preference
 *
 *     physical mapping
 *         target realization
 *
 * This grammar owns only the first category.
 *
 * Resource/capability requirements belong to their owning resource grammars.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file is parser-only.
 *
 * It MUST NOT declare lexer rules.
 *
 * It consumes the canonical parser-facing vocabulary supplied by the lexer
 * composition layer.
 *
 * Expected token categories include:
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
 * The lexer is responsible for assigning these token identities.
 *
 * This grammar MUST NOT create compatibility alternatives such as:
 *
 *     QUBIT | K_QUBIT
 *
 * or:
 *
 *     QUANTUM | K_QUANTUM
 *
 * ============================================================================
 * GENERIC TYPE OWNERSHIP
 * ============================================================================
 *
 * Generic application remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * This file MUST NOT define a second generic type system.
 *
 * Therefore:
 *
 *     quantum::State<T>
 *
 * must ultimately be composed from:
 *
 *     quantum-qualified type
 *
 * plus:
 *
 *     universal generic application.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax structure.
 *
 * It MUST lower into the existing domain-neutral frontend TypeExpr model.
 *
 * Conceptual mappings:
 *
 *     qubit
 *         -> quantum semantic TypeExpr
 *
 *     logical qubit
 *         -> logical quantum semantic TypeExpr
 *
 *     qubit[N]
 *         -> quantum collection/cardinality TypeExpr
 *
 *     logical qubit[N]
 *         -> logical quantum collection/cardinality TypeExpr
 *
 *     quantum<T>
 *         -> quantum generic/constructor TypeExpr
 *
 *     quantum::State
 *         -> qualified/named TypeExpr
 *
 *     quantum::State<T>
 *         -> qualified generic TypeExpr
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumTypeIR
 *     QuantumTypeNode
 *     QuantumRegisterIR
 *     PhysicalQubitIR
 *     QuantumHardwareTypeIR
 *
 * merely for syntax.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *   - whether a quantum type exists;
 *   - whether a type argument is valid;
 *   - whether a cardinality is integral;
 *   - whether a cardinality is positive where required;
 *   - whether a cardinality is compile-time known;
 *   - whether a cardinality is symbolic;
 *   - whether a cardinality depends on runtime values;
 *   - whether a type is linear/affine where required;
 *   - whether resource requirements can be satisfied;
 *   - whether capabilities exist;
 *   - whether the type can be lowered to the selected target.
 *
 * None of these decisions belong in this grammar.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This file has NO dependency on `quantum::ir`.
 *
 * The required direction is:
 *
 *     source
 *       |
 *       v
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
 * There must be no reverse dependency:
 *
 *     grammar -> quantum::ir
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions.
 *
 * Repository integration is required to support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Generated/integrating Rust MUST use safe Rust only.
 *
 * No `unsafe` implementation is required or permitted.
 *
 * Repository-level enforcement belongs to Rust source crates/modules, for
 * example:
 *
 *     #![forbid(unsafe_code)]
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
 * Quantum-specific type syntax.
 *
 * Ordinary identifiers are deliberately NOT accepted here.
 *
 * That prevents the quantum grammar from stealing every named type from the
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
 * Abstract physical-realization-independent quantum resource.
 *
 *     qubit
 *
 * The type does not identify:
 *
 *     - a physical qubit;
 *     - a QPU;
 *     - a topology;
 *     - a simulator;
 *     - a vendor.
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
 *     logical qubit
 *
 * This does not choose:
 *
 *     - QEC code;
 *     - code distance;
 *     - decoder;
 *     - physical overhead;
 *     - physical topology.
 */
logicalQubitType
    : K_LOGICAL
      K_QUBIT
    ;


/* ============================================================================
 * 4. QUANTUM COLLECTION
 * ========================================================================== */

/**
 * Parameterized collection of quantum resources.
 *
 * Examples:
 *
 *     qubit[N]
 *     qubit[n + 1]
 *     logical qubit[N]
 *     logical qubit[algorithm::width]
 *
 * The cardinality is an expression.
 *
 * It is never interpreted as a machine capacity.
 */
quantumCollectionType
    : quantumCollectionElementType
      LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


quantumCollectionElementType
    : quantumPrimitiveType
    | logicalQubitType
    ;


/* ============================================================================
 * 5. OPEN QUANTUM CONSTRUCTOR
 * ========================================================================== */

/**
 * Open semantic quantum constructor.
 *
 * Examples:
 *
 *     quantum<State>
 *     quantum<Observable>
 *     quantum<Register>
 *     quantum<State, N>
 *
 * The constructor is intentionally open.
 *
 * New quantum abstractions do not require adding another keyword merely to
 * become representable by source syntax.
 */
quantumConstructorType
    : K_QUANTUM
      LESS_THAN
      quantumConstructorArgumentList
      GREATER_THAN
    ;


quantumConstructorArgumentList
    : quantumConstructorArgument
      (COMMA quantumConstructorArgument)*
      COMMA?
    ;


quantumConstructorArgument
    : quantumConstructorTypeArgument
    | quantumConstructorValueArgument
    ;


/**
 * Type-valued constructor argument.
 *
 * The first branch permits the quantum-specific forms owned by this file.
 *
 * Named/qualified types are also accepted as open semantic type references.
 */
quantumConstructorTypeArgument
    : quantumType
    | quantumQualifiedTypeName
    | IDENTIFIER
    ;


/**
 * Value-level type argument.
 *
 * Values are restricted to the source-level cardinality expression language
 * below. Floating-point values are intentionally excluded.
 */
quantumConstructorValueArgument
    : quantumCardinalityExpression
    ;


/* ============================================================================
 * 6. QUALIFIED QUANTUM TYPE
 * ========================================================================== */

/**
 * Explicit quantum namespace.
 *
 * Examples:
 *
 *     quantum::State
 *     quantum::state::StateVector
 *     quantum::observable::Observable
 *     quantum::future::deep::Type
 *
 * Namespace resolution is semantic.
 */
quantumQualifiedType
    : K_QUANTUM
      DOUBLE_COLON
      quantumQualifiedTypeName
    ;


quantumQualifiedTypeName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 7. QUANTUM CARDINALITY
 * ========================================================================== */

/**
 * Source-level cardinality expression.
 *
 * The expression is deliberately independent from Rust integer widths.
 *
 * The grammar does not convert it to:
 *
 *     usize
 *     u32
 *     u64
 *
 * or any other implementation representation.
 */
quantumCardinalityExpression
    : quantumCardinalityBitwiseOrExpression
    ;


/* ============================================================================
 * 8. BITWISE OR
 * ========================================================================== */

quantumCardinalityBitwiseOrExpression
    : quantumCardinalityBitwiseXorExpression
      (
          PIPE quantumCardinalityBitwiseXorExpression
      )*
    ;


/* ============================================================================
 * 9. BITWISE XOR
 * ========================================================================== */

quantumCardinalityBitwiseXorExpression
    : quantumCardinalityBitwiseAndExpression
      (
          CARET quantumCardinalityBitwiseAndExpression
      )*
    ;


/* ============================================================================
 * 10. BITWISE AND
 * ========================================================================== */

quantumCardinalityBitwiseAndExpression
    : quantumCardinalityShiftExpression
      (
          AMPERSAND quantumCardinalityShiftExpression
      )*
    ;


/* ============================================================================
 * 11. SHIFTS
 * ========================================================================== */

quantumCardinalityShiftExpression
    : quantumCardinalityAdditiveExpression
      (
          LEFT_SHIFT quantumCardinalityAdditiveExpression
        | RIGHT_SHIFT quantumCardinalityAdditiveExpression
      )*
    ;


/* ============================================================================
 * 12. ADDITION / SUBTRACTION
 * ========================================================================== */

quantumCardinalityAdditiveExpression
    : quantumCardinalityMultiplicativeExpression
      (
          PLUS quantumCardinalityMultiplicativeExpression
        | MINUS quantumCardinalityMultiplicativeExpression
      )*
    ;


/* ============================================================================
 * 13. MULTIPLICATION / DIVISION / MODULO
 * ========================================================================== */

quantumCardinalityMultiplicativeExpression
    : quantumCardinalityUnaryExpression
      (
          STAR quantumCardinalityUnaryExpression
        | SLASH quantumCardinalityUnaryExpression
        | PERCENT quantumCardinalityUnaryExpression
      )*
    ;


/* ============================================================================
 * 14. UNARY
 * ========================================================================== */

quantumCardinalityUnaryExpression
    : PLUS quantumCardinalityUnaryExpression
    | MINUS quantumCardinalityUnaryExpression
    | quantumCardinalityPrimary
    ;


/* ============================================================================
 * 15. CARDINALITY PRIMARY
 * ========================================================================== */

/**
 * Primary cardinality values.
 *
 * IDENTIFIER permits symbolic values.
 *
 * Qualified identifiers permit values such as:
 *
 *     algorithm::qubits
 *     configuration::width
 *     module::parameter::N
 */
quantumCardinalityPrimary
    : INTEGER_LITERAL
    | IDENTIFIER
    | quantumQualifiedCardinalityName
    | quantumCardinalityParenthesized
    ;


quantumQualifiedCardinalityName
    : IDENTIFIER
      DOUBLE_COLON IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


quantumCardinalityParenthesized
    : LPAREN
      quantumCardinalityExpression
      RPAREN
    ;


/* ============================================================================
 * 16. EXPLICIT EXTENT BOUNDARY
 * ========================================================================== */

/**
 * Reusable extent syntax.
 *
 * Examples:
 *
 *     [N]
 *     [2 * N]
 *     [(N + M) * factor]
 *
 * This is a syntactic helper only.
 */
quantumExtent
    : LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


/* ============================================================================
 * 17. QUALIFIED SEMANTIC TYPES
 * ========================================================================== */

/**
 * The following are deliberately represented as names instead of hard-coded
 * productions:
 *
 *     quantum::state::StateVector
 *     quantum::state::DensityMatrix
 *     quantum::observable::Observable
 *     quantum::measurement::Result
 *     quantum::noise::Channel
 *     quantum::qec::Syndrome
 *     quantum::resource::Register
 *     quantum::photonic::Mode
 *     quantum::bosonic::Mode
 *     quantum::topological::Qubit
 *
 * This allows future quantum technologies to be introduced without changing
 * the core grammar solely because a new semantic type was invented.
 */


/* ============================================================================
 * 18. TYPE-LEVEL PHYSICAL / LOGICAL SEPARATION
 * ========================================================================== */

/**
 * `logical qubit` is an abstraction.
 *
 * A physical resource type, if needed, must be introduced through the
 * target-independent hardware/resource type system rather than through a
 * hard-coded physical-ID syntax here.
 *
 * Examples of semantic names that may be resolved downstream:
 *
 *     quantum::physical::Qubit
 *     quantum::logical::Qubit
 *
 * The grammar does not assign physical identity.
 */


/* ============================================================================
 * 19. LINEAR / AFFINE COMPOSITION
 * ========================================================================== */

/**
 * Quantum resources commonly participate in linear or affine ownership
 * semantics.
 *
 * The universal type system owns the `linear` and `affine` constructors.
 *
 * This file deliberately does not create duplicate constructors.
 *
 * Consequently forms such as:
 *
 *     Linear<qubit>
 *     Affine<qubit>
 *
 * belong to the universal type composition path.
 *
 * Semantic analysis determines whether a particular quantum resource is
 * required to satisfy linear/affine constraints.
 */


/* ============================================================================
 * 20. NO GATE INVENTORY
 * ========================================================================== */

/**
 * No operation is a type-level keyword here.
 *
 * In particular this grammar does NOT enumerate:
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
 * Gate/operation names belong to the operation grammar and semantic operation
 * registry.
 */


/* ============================================================================
 * 21. NO PHYSICAL RESOURCE ASSIGNMENT
 * ========================================================================== */

/**
 * This grammar does not contain productions for:
 *
 *     physical qubit 0
 *     qpu0
 *     device0
 *     vendor0
 *
 * A physical realization is downstream.
 */


/* ============================================================================
 * 22. NO QEC IMPLEMENTATION
 * ========================================================================== */

/**
 * `logical qubit` does not imply any particular:
 *
 *     - error-correcting code;
 *     - code distance;
 *     - decoder;
 *     - physical overhead;
 *     - syndrome extraction schedule.
 *
 * QEC is a downstream semantic/compiler responsibility.
 */


/* ============================================================================
 * 23. NO ZQN IMPLEMENTATION
 * ========================================================================== */

/**
 * This grammar does not implement:
 *
 *     - noise;
 *     - faults;
 *     - leakage;
 *     - erasure;
 *     - correlated noise;
 *     - reliability thresholds;
 *     - calibration.
 *
 * These remain downstream ZQN/resilience/HAL responsibilities.
 */


/* ============================================================================
 * 24. DETERMINISM
 * ========================================================================== */

/**
 * Parsing depends only on:
 *
 *     - source token stream;
 *     - selected grammar version;
 *     - parser configuration explicitly supplied by the compiler.
 *
 * It MUST NOT depend on:
 *
 *     - hardware;
 *     - target availability;
 *     - network state;
 *     - filesystem state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime state.
 */


/* ============================================================================
 * 25. SOURCE-SPAN CONTRACT
 * ========================================================================== */

/**
 * The frontend AST builder must preserve source spans for at least:
 *
 *     quantumType
 *     quantumPrimitiveType
 *     logicalQubitType
 *     quantumCollectionType
 *     quantumConstructorType
 *     quantumQualifiedType
 *     quantumCardinalityExpression
 *     quantumExtent
 *
 * This supports:
 *
 *     - diagnostics;
 *     - LSP;
 *     - IDE navigation;
 *     - formatting;
 *     - refactoring;
 *     - provenance;
 *     - source maps.
 */


/* ============================================================================
 * 26. ERROR CONTRACT
 * ========================================================================== */

/**
 * Structurally invalid examples include:
 *
 *     logical
 *     qubit[
 *     qubit[]
 *     qubit[N
 *     qubit[N +]
 *     qubit[*]
 *     logical qubit[
 *     quantum<
 *     quantum<>
 *     quantum<T
 *     quantum::
 *     quantum:::
 *
 * The semantic layer, rather than this grammar, determines whether:
 *
 *     - a cardinality is negative;
 *     - zero is permitted;
 *     - an expression is integral;
 *     - a symbol is defined;
 *     - a type argument is valid;
 *     - a resource is available.
 */


/* ============================================================================
 * 27. POSITIVE CONFORMANCE CONTRACT
 * ========================================================================== */

/**
 * The following forms MUST be representable:
 *
 * Primitive:
 *
 *     qubit
 *
 * Logical:
 *
 *     logical qubit
 *
 * Static cardinality:
 *
 *     qubit[1]
 *     qubit[1024]
 *
 * Symbolic cardinality:
 *
 *     qubit[N]
 *     qubit[n + 1]
 *     qubit[2 * n]
 *     qubit[width]
 *     qubit[algorithm::width]
 *     qubit[(N + M) * factor]
 *
 * Logical collections:
 *
 *     logical qubit[N]
 *
 * Constructors:
 *
 *     quantum<State>
 *     quantum<Observable>
 *     quantum<Register>
 *     quantum<State, N>
 *
 * Qualified names:
 *
 *     quantum::State
 *     quantum::state::State
 *     quantum::future::deep::Type
 */


/* ============================================================================
 * 28. SCALABILITY CONFORMANCE CONTRACT
 * ========================================================================== */

/**
 * The following are architectural requirements:
 *
 *     qubit[1]
 *     qubit[N]
 *     qubit[2 * N]
 *     qubit[(N + M) * factor]
 *
 * must all use the same source grammar.
 *
 * There MUST NOT be separate grammar paths such as:
 *
 *     smallQubitRegister
 *     mediumQubitRegister
 *     largeQubitRegister
 *
 * and there MUST NOT be source syntax such as:
 *
 *     qubit64
 *     qubit128
 *     qubit1024
 *
 * to represent machine-size classes.
 *
 * Cardinality is data, not grammar structure.
 */


/* ============================================================================
 * 29. RESOURCE AVAILABILITY BOUNDARY
 * ========================================================================== */

/**
 * This grammar does not answer:
 *
 *     "Can this machine provide N qubits?"
 *
 * That question belongs to:
 *
 *     semantic resource analysis
 *             |
 *             v
 *     capability analysis
 *             |
 *             v
 *     target discovery
 *             |
 *             v
 *     compilation / scheduling / routing
 *
 * Example:
 *
 *     qubit[N]
 *
 * may create a semantic resource requirement.
 *
 * The grammar itself creates no allocation.
 */


/* ============================================================================
 * 30. GENERIC INTEGRATION
 * ========================================================================== */

/**
 * The universal generic grammar remains the owner of generic application.
 *
 * Therefore:
 *
 *     quantum::State<T>
 *
 * MUST NOT require a duplicate quantum-specific generic grammar.
 *
 * Integration direction:
 *
 *     Types
 *       |
 *       +--> quantumType
 *       |
 *       +--> genericType
 *       |
 *       +--> ordinaryType
 *
 * The universal type layer decides how these branches compose.
 */


/* ============================================================================
 * 31. FRONTEND AST INTEGRATION
 * ========================================================================== */

/**
 * Existing repository frontend type representation:
 *
 *     TypeExpr
 *
 * remains authoritative.
 *
 * This grammar MUST NOT require a new quantum-specific AST merely to represent
 * syntax.
 *
 * Required semantic lowering is conceptually:
 *
 *     quantumType
 *          |
 *          v
 *     TypeExpr
 *          |
 *          v
 *     semantic quantum type
 *
 * Existing TypeExpr currently contains a `Quantum(...)` semantic constructor.
 *
 * If future quantum cardinality information cannot be represented losslessly
 * by the existing TypeExpr, the AST/type contract must be expanded at the
 * universal type boundary rather than creating a parser-only QuantumTypeNode.
 *
 * Such an AST change is a separate implementation contract and is NOT hidden
 * inside this grammar.
 */


/* ============================================================================
 * 32. QUANTUM IR INTEGRATION
 * ========================================================================== */

/**
 * This grammar does not import or reference Rust quantum IR types.
 *
 * Canonical path:
 *
 *     source
 *       |
 *       v
 *     TypeExpr
 *       |
 *       v
 *     semantic type/resource model
 *       |
 *       v
 *     quantum::ir
 *
 * The canonical quantum IR remains the repository's existing:
 *
 *     quantum::ir
 *
 * There is no second quantum type IR introduced here.
 */


/* ============================================================================
 * 33. QUANTUM DOMAIN INTEGRATION
 * ========================================================================== */

/**
 * Other quantum grammar files own:
 *
 *     grammar/quantum/qubits.g4
 *     grammar/quantum/quantum-registers.g4
 *     grammar/quantum/quantum-states.g4
 *     grammar/quantum/operations.g4
 *     grammar/quantum/measurement.g4
 *     grammar/quantum/error-correction.g4
 *     grammar/quantum/quantum-resources.g4
 *     grammar/quantum/quantum-capabilities.g4
 *
 * Their relationship to this file is:
 *
 *     TYPE
 *       |
 *       +---- qubit
 *       +---- logical qubit
 *       +---- quantum collection
 *
 *     RESOURCE DECLARATION
 *       |
 *       +---- qubit variable/register declarations
 *
 *     OPERATION
 *       |
 *       +---- operation over typed quantum operands
 *
 *     RESOURCE ANALYSIS
 *       |
 *       +---- cardinality/capability requirements
 *
 * No file may redefine the meaning of `quantumType`.
 */


/* ============================================================================
 * 34. HARDWARE INTEGRATION
 * ========================================================================== */

/**
 * Hardware-specific realization belongs to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *
 * This file therefore cannot contain:
 *
 *     device IDs
 *     QPU IDs
 *     physical qubit numbers
 *     coupling maps
 *     topology
 *     calibration values
 *     native gate sets
 */


/* ============================================================================
 * 35. COMPILER INTEGRATION
 * ========================================================================== */

/**
 * After parsing, the compiler is responsible for:
 *
 *     1. name resolution;
 *     2. generic resolution;
 *     3. type checking;
 *     4. cardinality analysis;
 *     5. resource analysis;
 *     6. capability analysis;
 *     7. effect/ownership analysis;
 *     8. target feasibility;
 *     9. specialization;
 *    10. canonical semantic lowering;
 *    11. quantum::ir construction;
 *    12. optimization;
 *    13. routing;
 *    14. scheduling;
 *    15. resilience/QEC/ZQN processing;
 *    16. target lowering.
 *
 * None of these operations are performed by the grammar.
 */


/* ============================================================================
 * 36. COMPATIBILITY
 * ========================================================================== */

/**
 * Existing source forms remain stable:
 *
 *     qubit
 *     logical qubit
 *     qubit[N]
 *     logical qubit[N]
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
 * remain ordinary identifiers unless a separately versioned compatibility
 * specification explicitly gives them another meaning.
 *
 * They MUST NOT become permanent lexer keywords merely for compatibility.
 */


/* ============================================================================
 * 37. DUPLICATE-GRAMMAR MIGRATION
 * ========================================================================== */

/**
 * CURRENT REPOSITORY STATE
 * ------------------------
 *
 * The repository currently contains:
 *
 *     grammar/quantum/quantum-types.g4
 *
 * and:
 *
 *     grammar/types/quantum.g4
 *
 * Both historically define quantum type syntax.
 *
 * This is not acceptable as a production architecture.
 *
 * CANONICAL OWNER
 * ---------------
 *
 * This file:
 *
 *     grammar/types/quantum.g4
 *
 * is the canonical owner because quantum types are part of the universal
 * `Types` composition boundary.
 *
 * MIGRATION
 * ---------
 *
 *     grammar/quantum/quantum-types.g4
 *                 |
 *                 v
 *          deprecated compatibility
 *                 |
 *                 v
 *             removal
 *
 * No new feature may be added to the old duplicate grammar.
 *
 * `QuantumTypes` must not be imported by the production parser after migration.
 *
 * The canonical parser path is:
 *
 *     ZamaniParser
 *         |
 *         v
 *       Types
 *         |
 *         v
 *       Quantum
 */


/* ============================================================================
 * 38. ROOT INTEGRATION
 * ========================================================================== */

/**
 * grammar/Zamani.g4
 *
 * remains the root composition grammar.
 *
 * It must NOT import this file directly.
 *
 * Correct composition:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *         Types
 *          |
 *          +--> Quantum
 *
 * This preserves the single root grammar architecture.
 */


/* ============================================================================
 * 39. LEXER INTEGRATION
 * ========================================================================== */

/**
 * The lexer composition hierarchy remains responsible for:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_LOGICAL
 *     INTEGER_LITERAL
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     brackets
 *     generic delimiters
 *     arithmetic operators
 *     bitwise operators
 *
 * This grammar must never compensate for lexical inconsistencies by adding
 * local token definitions.
 */


/* ============================================================================
 * 40. NO UNSAFE RUST
 * ========================================================================== */

/**
 * This file contains no Rust code.
 *
 * The downstream Rust implementation must remain:
 *
 *     Rust 1.97 / 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * Repository enforcement belongs to the Rust crate.
 *
 * No quantum type feature defined here requires `unsafe`.
 */


/* ============================================================================
 * 41. SECURITY
 * ========================================================================== */

/**
 * This grammar:
 *
 *     - performs no I/O;
 *     - accesses no filesystem;
 *     - accesses no network;
 *     - reads no environment variables;
 *     - accesses no credentials;
 *     - discovers no hardware;
 *     - executes no programs;
 *     - invokes no backend;
 *     - contains no embedded target-language actions.
 *
 * It is a declarative parsing boundary.
 */


/* ============================================================================
 * 42. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is DONE only when:
 *
 * [x] It is the sole canonical quantum source-type owner.
 *
 * [x] `qubit` is supported.
 *
 * [x] `logical qubit` is supported.
 *
 * [x] `qubit[N]` is supported.
 *
 * [x] `logical qubit[N]` is supported.
 *
 * [x] Symbolic cardinalities are supported.
 *
 * [x] Qualified symbolic cardinalities are supported.
 *
 * [x] Cardinality precedence is explicit.
 *
 * [x] Open `quantum<T>` constructors are supported.
 *
 * [x] Qualified quantum types are supported.
 *
 * [x] Arbitrary namespace depth is supported.
 *
 * [x] Generic ownership remains universal.
 *
 * [x] No fixed gate inventory exists.
 *
 * [x] No physical qubit IDs exist.
 *
 * [x] No hardware limits exist.
 *
 * [x] No QEC implementation exists.
 *
 * [x] No ZQN implementation exists.
 *
 * [x] No routing exists.
 *
 * [x] No scheduling exists.
 *
 * [x] No HAL dependency exists.
 *
 * [x] No quantum::ir dependency exists.
 *
 * [x] No Rust actions exist.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Source spans are part of the downstream contract.
 *
 * [x] Resource availability is explicitly downstream.
 *
 * [x] Type semantics are explicitly downstream.
 *
 * [x] The universal TypeExpr remains authoritative.
 *
 * [ ] The duplicate `grammar/quantum/quantum-types.g4` has been deprecated and
 *     removed from production composition.
 *
 * [ ] `grammar/types/types.g4` imports/composes this grammar exactly once.
 *
 * [ ] `grammar/types/generic.g4` remains the sole generic-application owner.
 *
 * [ ] Lexer token identity has been reconciled.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] ANTLR generation passes.
 *
 * [ ] Rust frontend conformance passes.
 *
 * [ ] Semantic TypeExpr conformance passes.
 *
 * [ ] End-to-end quantum::ir lowering passes.
 */