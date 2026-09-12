/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/quantum-types.g4
 *
 * Grammar:
 *     QuantumTypes
 *
 * Status:
 *     Production-ready quantum-type syntax delegate.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL QUANTUM TYPE SYNTAX.
 *
 * It provides the type-system boundary for quantum computational resources
 * without encoding physical machine characteristics.
 *
 * Canonical examples:
 *
 *     qubit
 *     logical qubit
 *     qubit[N]
 *     logical qubit[N]
 *     quantum<T>
 *     quantum<T, P>
 *     quantum::Type
 *     quantum::Type<T>
 *
 * The grammar deliberately remains open to future quantum type constructors
 * through the ordinary named/generic type system.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - primitive quantum type syntax;
 *     - logical-qubit type syntax;
 *     - scalable quantum cardinality syntax;
 *     - quantum namespace/type qualification;
 *     - the source-level `quantum<...>` type constructor;
 *     - stable parser boundaries for quantum types;
 *     - quantum-type syntax compatibility.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - quantum operations;
 *     - quantum gates;
 *     - circuits;
 *     - measurement statements;
 *     - reset;
 *     - dynamic circuits;
 *     - control flow;
 *     - QEC algorithms;
 *     - QEC decoders;
 *     - ZQN;
 *     - noise models;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware discovery;
 *     - physical qubit allocation;
 *     - physical qubit IDs;
 *     - device selection;
 *     - topology;
 *     - backend selection;
 *     - simulator selection;
 *     - runtime representation;
 *     - ABI representation;
 *     - quantum::ir.
 *
 * Those concepts belong to their respective repository subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser
 *          |
 *          +---- Types
 *          |       |
 *          |       +---- QuantumTypes
 *          |
 *          v
 *     frontend TypeExpr
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     resource semantics          quantum semantics
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                   canonical IR
 *                        |
 *                        v
 *              quantum::ir / other IR
 *                        |
 *                        v
 *       optimization / routing / scheduling
 *                        |
 *                        v
 *               hardware realization
 *
 * The grammar never depends on downstream quantum infrastructure.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Quantum types describe COMPUTATIONAL INTENT.
 *
 * They do NOT describe a particular physical machine.
 *
 * This grammar therefore MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_REGISTER_SIZE
 *     MAX_STATE_SIZE
 *     MAX_CIRCUIT_WIDTH
 *     MAX_QPU_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_TOPOLOGY_SIZE
 *     MAX_GATE_COUNT
 *
 * Nor may it encode:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     IBM-Q
 *     Rigetti
 *     IonQ
 *     Quantinuum
 *     NVIDIA
 *     a vendor topology
 *     a device identifier
 *     a fixed gate set
 *     a fixed connectivity graph.
 *
 * A source-level cardinality such as:
 *
 *     qubit[N]
 *
 * means that the program has a semantic requirement involving N quantum
 * resources.
 *
 * It does NOT mean:
 *
 *     use physical qubits 0..N-1
 *
 * and it does not select a machine.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Cardinality is represented symbolically.
 *
 * Examples:
 *
 *     qubit[N]
 *     qubit[Q]
 *     qubit[register_size]
 *     qubit[2 * N]
 *
 * This grammar deliberately does not convert cardinalities to host integers.
 *
 * Parsing preserves the source expression.
 *
 * Semantic analysis determines:
 *
 *     - whether the expression is a valid compile-time cardinality;
 *     - whether it is known statically;
 *     - whether it is runtime-dependent;
 *     - whether the target can satisfy it;
 *     - whether a resource constraint exists.
 *
 * The grammar itself imposes no finite cardinality limit.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexical vocabulary for this grammar is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * Therefore this grammar uses:
 *
 *     tokenVocab = ZamaniTokens
 *
 * The current lexer provides:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_LOGICAL
 *     K_CODE
 *     K_SURFACE
 *     K_PARITY
 *     K_NOISE
 *     K_FIDELITY
 *
 * among the quantum-related vocabulary.
 *
 * This grammar MUST NOT declare lexer rules.
 *
 * ============================================================================
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * Lexer:
 *
 *     qubit
 *         -> K_QUBIT
 *
 *     quantum
 *         -> K_QUANTUM
 *
 *     logical
 *         -> K_LOGICAL
 *
 * Parser:
 *
 *     K_QUBIT
 *         -> quantumPrimitiveType
 *
 *     K_LOGICAL K_QUBIT
 *         -> logicalQubitType
 *
 *     K_QUANTUM ...
 *         -> quantumConstructorType
 *
 * The lexer remains the sole authority for token spelling.
 *
 * ============================================================================
 * IMPORTANT TOKEN-VOCABULARY CORRECTION
 * ============================================================================
 *
 * The repository currently contains older grammar components which refer to
 * token names such as:
 *
 *     QUBIT
 *     INT
 *     FLOAT_TYPE
 *     BOOL_TYPE
 *
 * while the current lexer vocabulary uses the K_* naming convention:
 *
 *     K_QUBIT
 *     K_INT
 *     K_FLOAT
 *     K_BOOL
 *
 * This file intentionally follows the current authoritative vocabulary:
 *
 *     ZamaniTokens
 *
 * It does NOT introduce compatibility aliases in the parser grammar.
 *
 * Token aliases must be resolved centrally in the lexer/token-vocabulary
 * migration rather than duplicated in every parser grammar.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser recognizes:
 *
 *     qubit
 *
 * as a quantum type.
 *
 * It does NOT decide:
 *
 *     physical qubit;
 *     logical qubit implementation;
 *     encoded qubit;
 *     superconducting qubit;
 *     trapped-ion qubit;
 *     photonic qubit;
 *     neutral-atom qubit;
 *     topological qubit;
 *     simulator state;
 *     hardware allocation.
 *
 * Those are semantic/resource/target decisions.
 *
 * ============================================================================
 * QUANTUM::IR BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT construct quantum::ir nodes.
 *
 * The intended transformation is:
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
 *     canonical quantum::ir
 *
 * If the frontend represents a quantum type using an existing TypeExpr
 * variant, that representation is authoritative.
 *
 * This grammar must never introduce:
 *
 *     QuantumTypeIR
 *     QuantumTypeNode
 *     QuantumTypeHardwareIR
 *
 * merely to represent syntax.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is ANTLR grammar source and contains no Rust implementation code.
 *
 * All Rust components integrating this grammar MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the repository must prohibit unsafe Rust:
 *
 *     #![deny(unsafe_code)]
 *
 * This grammar itself requires no unsafe code.
 *
 * ============================================================================
 */

parser grammar QuantumTypes;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical source-level quantum type expression.
 *
 * This rule is intentionally narrow.
 *
 * General user-defined quantum types remain representable through the
 * canonical named/generic type system owned by Types.
 */
quantumType
    : quantumPrimitiveType
    | logicalQubitType
    | quantumRegisterType
    | quantumConstructorType
    | quantumQualifiedType
    ;


/* ============================================================================
 * 2. PRIMITIVE QUBIT TYPE
 * ========================================================================== */

/**
 * Primitive source-level quantum resource.
 *
 * Canonical spelling:
 *
 *     qubit
 *
 * This is an abstract computational resource.
 *
 * It does NOT identify:
 *
 *     a physical qubit;
 *     a physical index;
 *     a device;
 *     a topology;
 *     a vendor;
 *     a QPU;
 *     a simulator.
 */
quantumPrimitiveType
    : K_QUBIT
    ;


/* ============================================================================
 * 3. LOGICAL QUBIT TYPE
 * ========================================================================== */

/**
 * Logical qubit abstraction.
 *
 * Canonical spelling:
 *
 *     logical qubit
 *
 * The syntax expresses logical-level intent.
 *
 * It does not select:
 *
 *     a QEC code;
 *     a code distance;
 *     a decoder;
 *     a physical-qubit count;
 *     a hardware topology.
 *
 * QEC selection belongs to semantic/resource/compilation layers.
 */
logicalQubitType
    : K_LOGICAL K_QUBIT
    ;


/* ============================================================================
 * 4. SCALABLE QUANTUM REGISTER TYPE
 * ========================================================================== */

/**
 * Quantum resource collection.
 *
 * Canonical forms:
 *
 *     qubit[N]
 *     logical qubit[N]
 *
 * The cardinality expression is symbolic.
 *
 * Examples:
 *
 *     qubit[n]
 *     qubit[N + 1]
 *     qubit[2 * n]
 *     logical qubit[algorithm_width]
 *
 * The grammar does not evaluate or bound the cardinality.
 */
quantumRegisterType
    : quantumRegisterElementType
      LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


/**
 * Element type of a scalable quantum register.
 */
quantumRegisterElementType
    : K_QUBIT
    | K_LOGICAL K_QUBIT
    ;


/* ============================================================================
 * 5. QUANTUM TYPE CONSTRUCTOR
 * ========================================================================== */

/**
 * General quantum type constructor.
 *
 * Canonical form:
 *
 *     quantum<T>
 *
 * Examples:
 *
 *     quantum<State>
 *     quantum<Amplitude>
 *     quantum<Register>
 *     quantum<CustomQuantumType>
 *
 * The type argument is intentionally represented as an open quantum type
 * argument rather than a closed list of physical quantum implementations.
 *
 * Semantic analysis determines whether the selected type constructor and
 * arguments are meaningful.
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
 * A quantum constructor can carry:
 *
 *     - a named semantic type;
 *     - a qualified type;
 *     - a symbolic quantum domain;
 *     - a nested quantum constructor;
 *     - a type-level symbolic value.
 *
 * This grammar deliberately keeps the type constructor extensible.
 */
quantumTypeArgumentList
    : quantumTypeArgument
      (COMMA quantumTypeArgument)*
      COMMA?
    ;


quantumTypeArgument
    : quantumTypeArgumentAtom
    | quantumCardinalityExpression
    ;


/**
 * Quantum type argument atom.
 *
 * The identifier form intentionally remains open to future quantum domains.
 *
 * Examples:
 *
 *     quantum<State>
 *     quantum<Amplitude>
 *     quantum<Observable>
 *     quantum<CustomType>
 *
 * The parser does not decide what those names mean.
 */
quantumTypeArgumentAtom
    : quantumQualifiedName
    | quantumPrimitiveType
    | logicalQubitType
    | quantumConstructorType
    ;


/* ============================================================================
 * 6. QUALIFIED QUANTUM TYPES
 * ========================================================================== */

/**
 * Qualified quantum type.
 *
 * Canonical examples:
 *
 *     quantum::State
 *     quantum::Observable
 *     quantum::Amplitude
 *     quantum::Register
 *     quantum::future::Type
 *
 * Qualification is syntactic.
 *
 * Name resolution belongs to the semantic layer.
 */
quantumQualifiedType
    : K_QUANTUM
      DOUBLE_COLON
      quantumQualifiedNameTail
    ;


/**
 * Qualified name after the `quantum::` namespace.
 *
 * No fixed namespace depth is imposed.
 */
quantumQualifiedNameTail
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/**
 * Reusable qualified quantum name.
 *
 * This rule intentionally supports arbitrary namespace depth.
 */
quantumQualifiedName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 7. QUANTUM CARDINALITY
 * ========================================================================== */

/**
 * Cardinality expression used by quantum resource types.
 *
 * Examples:
 *
 *     qubit[N]
 *     qubit[n + 1]
 *     qubit[2 * n]
 *     qubit[register_width]
 *     qubit[quantum::width]
 *
 * This grammar preserves the expression.
 *
 * It does not:
 *
 *     evaluate;
 *     allocate;
 *     convert to usize;
 *     impose a maximum;
 *     select a device.
 */
quantumCardinalityExpression
    : quantumCardinalityUnary*
      quantumCardinalityPrimary
      quantumCardinalityBinaryPart*
    ;


quantumCardinalityUnary
    : PLUS
    | MINUS
    ;


quantumCardinalityBinaryPart
    : quantumCardinalityOperator
      quantumCardinalityUnary*
      quantumCardinalityPrimary
    ;


quantumCardinalityOperator
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


quantumCardinalityPrimary
    : INTEGER_LITERAL
    | IDENTIFIER
    | quantumQualifiedCardinalityName
    | quantumCardinalityParenthesized
    ;


quantumQualifiedCardinalityName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)+
    ;


quantumCardinalityParenthesized
    : LPAREN
      quantumCardinalityExpression
      RPAREN
    ;


/* ============================================================================
 * 8. NESTED QUANTUM TYPES
 * ========================================================================== */

/**
 * Nested quantum constructors remain structurally recursive.
 *
 * Example:
 *
 *     quantum<quantum<State>>
 *
 * Whether such a type is semantically legal is NOT determined here.
 *
 * The grammar preserves the source structure and leaves legality to semantic
 * type checking.
 */
nestedQuantumType
    : quantumConstructorType
    ;


/* ============================================================================
 * 9. QUANTUM TYPE COMPOSITION
 * ========================================================================== */

/**
 * Quantum type composition is deliberately limited to type syntax.
 *
 * The grammar does not attempt to encode:
 *
 *     gate composition;
 *     circuit composition;
 *     pulse composition;
 *     routing;
 *     scheduling.
 *
 * Those belong elsewhere.
 */
quantumComposedType
    : quantumType
    ;


/* ============================================================================
 * 10. TYPE-LEVEL QUANTUM CAPABILITY MARKER
 * ========================================================================== */

/**
 * The semantic type system may use qualified quantum types to express
 * capability-oriented abstractions.
 *
 * Examples:
 *
 *     quantum::State
 *     quantum::Observable
 *     quantum::Measurement
 *
 * This grammar does not enumerate those semantic categories.
 *
 * Keeping the namespace open is essential for future quantum models without
 * repeatedly modifying the core grammar.
 */
quantumCapabilityType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 11. TYPE-LEVEL QUANTUM RESOURCE MARKER
 * ========================================================================== */

/**
 * Resource-bearing quantum types remain abstract.
 *
 * Examples represented through the canonical type namespace include:
 *
 *     quantum::Register
 *     quantum::Resource
 *     quantum::LogicalResource
 *
 * The grammar does not attach a physical allocation policy.
 */
quantumResourceType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 12. TYPE-LEVEL QUANTUM STATE MARKER
 * ========================================================================== */

/**
 * State-like quantum types remain open through the qualified namespace.
 *
 * The grammar deliberately does not reserve:
 *
 *     State
 *     DensityMatrix
 *     StateVector
 *     StabilizerState
 *     Amplitude
 *     Wavefunction
 *
 * as lexical keywords.
 *
 * They can therefore evolve without expanding the global lexer vocabulary.
 */
quantumStateType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 13. TYPE-LEVEL OBSERVABLE MARKER
 * ========================================================================== */

/**
 * Observable-like quantum types are open semantic names.
 *
 * No fixed observable inventory is encoded.
 */
quantumObservableType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 14. TYPE-LEVEL MEASUREMENT MARKER
 * ========================================================================== */

/**
 * Measurement result types remain ordinary quantum-qualified types.
 *
 * This grammar does not decide whether a measurement produces:
 *
 *     a bit;
 *     a bit-vector;
 *     an integer;
 *     a probability distribution;
 *     a classical register;
 *     a richer measurement object.
 *
 * Those are semantic type decisions.
 */
quantumMeasurementType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 15. TYPE-LEVEL ERROR-CORRECTION MARKER
 * ========================================================================== */

/**
 * QEC-related type names remain qualified semantic types.
 *
 * Examples that may be represented by the semantic namespace:
 *
 *     quantum::qec::LogicalQubit
 *     quantum::qec::Code
 *     quantum::qec::Syndrome
 *     quantum::qec::EncodedState
 *
 * This grammar does NOT select or implement a QEC code.
 */
quantumErrorCorrectionType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 16. TYPE-LEVEL NOISE MARKER
 * ========================================================================== */

/**
 * Noise-related types remain semantic names.
 *
 * For example:
 *
 *     quantum::noise::Channel
 *     quantum::noise::Model
 *     quantum::noise::Parameter
 *
 * This grammar does not own ZQN semantics.
 */
quantumNoiseType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 17. TYPE-LEVEL FIDELITY MARKER
 * ========================================================================== */

/**
 * Fidelity-related values/types remain semantic abstractions.
 *
 * This grammar does not establish:
 *
 *     a fidelity threshold;
 *     an acceptance threshold;
 *     a backend-specific metric;
 *     a fixed precision.
 */
quantumFidelityType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 18. TYPE-LEVEL SURFACE-CODE MARKER
 * ========================================================================== */

/**
 * Surface-code vocabulary remains semantic/QEC-owned.
 *
 * The lexer may recognize `surface` and `code`, but this grammar deliberately
 * does not make:
 *
 *     surface code
 *
 * a special hardware-specific type.
 *
 * If the semantic namespace exposes such a type, it is represented through
 * qualified type syntax.
 */
quantumSurfaceCodeType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 19. TYPE-LEVEL PARITY MARKER
 * ========================================================================== */

/**
 * Parity is not a physical type by itself.
 *
 * A semantic quantum subsystem may expose:
 *
 *     quantum::Parity
 *
 * or a more specialized qualified type.
 *
 * This grammar remains neutral.
 */
quantumParityType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 20. TYPE-LEVEL CODE MARKER
 * ========================================================================== */

/**
 * Quantum code abstractions remain open semantic types.
 *
 * No fixed code family is encoded here.
 */
quantumCodeType
    : quantumQualifiedType
    ;


/* ============================================================================
 * 21. NO HARD-CODED HARDWARE
 * ========================================================================== */

/**
 * Explicitly forbidden source forms are NOT enumerated as grammar rules.
 *
 * The following concepts must remain semantic/configuration concerns:
 *
 *     physical-qubit-0
 *     physical-qubit-1
 *     qpu-0
 *     device-0
 *     topology-0
 *     vendor-specific identifiers
 *     fixed connectivity
 *
 * If a program requires a physical mapping, that requirement belongs to:
 *
 *     resource constraints;
 *     target descriptions;
 *     compilation context;
 *     deployment configuration;
 *     runtime capability discovery.
 */


/* ============================================================================
 * 22. NO FIXED GATE SET
 * ========================================================================== */

/**
 * This grammar deliberately does NOT define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * as quantum types.
 *
 * Gate/operation syntax belongs to the quantum operation grammar.
 *
 * Gate availability belongs to semantic capability analysis and target
 * lowering.
 */


/* ============================================================================
 * 23. NO QEC IMPLEMENTATION
 * ========================================================================== */

/**
 * The following are NOT encoded here:
 *
 *     code distance;
 *     physical-to-logical ratio;
 *     syndrome extraction;
 *     decoder selection;
 *     correction algorithm;
 *     threshold;
 *     recovery schedule.
 *
 * Those belong to QEC and resilience subsystems.
 */


/* ============================================================================
 * 24. NO ZQN OWNERSHIP
 * ========================================================================== */

/**
 * Quantum noise is not represented as a grammar-level implementation.
 *
 * A source program may refer to semantic noise types through qualified names,
 * but:
 *
 *     classification;
 *     correlation;
 *     leakage;
 *     loss;
 *     erasure;
 *     calibration;
 *     fault models;
 *
 * belong to ZQN and downstream semantic layers.
 */


/* ============================================================================
 * 25. AST INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * The parser should lower these constructs into the EXISTING canonical type
 * representation.
 *
 * Required conceptual mappings:
 *
 *     qubit
 *         -> existing quantum TypeExpr representation
 *
 *     logical qubit
 *         -> quantum logical-resource TypeExpr representation
 *
 *     qubit[N]
 *         -> quantum collection/cardinality TypeExpr representation
 *
 *     logical qubit[N]
 *         -> logical quantum collection TypeExpr representation
 *
 *     quantum<T>
 *         -> generic/quantum TypeExpr representation
 *
 *     quantum::Type
 *         -> named/qualified TypeExpr representation
 *
 * This grammar MUST NOT require a second quantum AST hierarchy.
 */


/* ============================================================================
 * 26. SEMANTIC INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Semantic analysis owns:
 *
 *     name resolution;
 *     type resolution;
 *     generic validation;
 *     cardinality validation;
 *     compile-time value evaluation;
 *     resource requirements;
 *     capability requirements;
 *     logical/physical interpretation;
 *     QEC interpretation;
 *     noise interpretation;
 *     backend legality;
 *     target feasibility.
 *
 * The parser must only preserve syntax.
 */


/* ============================================================================
 * 27. RESOURCE INTEGRATION
 * ========================================================================== */

/**
 * A quantum cardinality is a semantic resource requirement, not an allocation.
 *
 * Example:
 *
 *     qubit[N]
 *
 * may result in a resource requirement equivalent to:
 *
 *     required_quantum_resources = N
 *
 * after semantic analysis.
 *
 * The grammar itself MUST NOT:
 *
 *     reserve N qubits;
 *     inspect hardware;
 *     query a backend;
 *     allocate memory;
 *     allocate QPU resources.
 */


/* ============================================================================
 * 28. HARDWARE INTEGRATION
 * ========================================================================== */

/**
 * Hardware mapping occurs after semantic analysis.
 *
 * Conceptually:
 *
 *     qubit[N]
 *          |
 *          v
 *     semantic requirement
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     resource planning
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     hardware realization
 *
 * No hardware dependency is permitted in this grammar.
 */


/* ============================================================================
 * 29. QUANTUM IR INTEGRATION
 * ========================================================================== */

/**
 * The grammar does not import quantum::ir.
 *
 * The frontend owns the conversion:
 *
 *     parsed TypeExpr
 *         |
 *         v
 *     semantic quantum type
 *         |
 *         v
 *     canonical quantum::ir metadata/types where required
 *
 * `quantum::ir` remains the canonical semantic quantum boundary.
 */


/* ============================================================================
 * 30. QEC INTEGRATION
 * ========================================================================== */

/**
 * Logical quantum types may create semantic requirements for QEC.
 *
 * Example:
 *
 *     logical qubit
 *
 * means the program operates at the logical abstraction level.
 *
 * It does NOT specify:
 *
 *     surface code;
 *     color code;
 *     repetition code;
 *     Steane code;
 *     Bacon-Shor;
 *     code distance;
 *     decoder.
 *
 * QEC chooses an implementation later.
 */


/* ============================================================================
 * 31. ZQN INTEGRATION
 * ========================================================================== */

/**
 * Noise-related qualified quantum types may be recognized syntactically.
 *
 * ZQN determines the actual fault/noise semantics.
 *
 * No ZQN dependency exists in this grammar.
 */


/* ============================================================================
 * 32. OPTIMIZATION INTEGRATION
 * ========================================================================== */

/**
 * Optimization consumes semantic representations produced after parsing.
 *
 * This grammar does not:
 *
 *     optimize types;
 *     simplify cardinalities;
 *     eliminate resources;
 *     choose gates;
 *     transform circuits.
 */


/* ============================================================================
 * 33. ROUTING INTEGRATION
 * ========================================================================== */

/**
 * Routing consumes logical quantum structures after semantic analysis.
 *
 * The grammar never performs:
 *
 *     physical placement;
 *     connectivity mapping;
 *     qubit permutation;
 *     topology selection.
 */


/* ============================================================================
 * 34. SCHEDULING INTEGRATION
 * ========================================================================== */

/**
 * Scheduling consumes quantum operations/resources after semantic lowering.
 *
 * The grammar never determines:
 *
 *     operation duration;
 *     start time;
 *     resource occupancy;
 *     pulse alignment;
 *     dynamic scheduling.
 */


/* ============================================================================
 * 35. RUNTIME INTEGRATION
 * ========================================================================== */

/**
 * Runtime representation is target-dependent.
 *
 * A `qubit` may be represented by:
 *
 *     an abstract runtime handle;
 *     a simulator object;
 *     a logical resource;
 *     a provider handle;
 *     a distributed resource;
 *     another target-specific representation.
 *
 * The source grammar remains unchanged.
 */


/* ============================================================================
 * 36. FUTURE EXTENSIBILITY
 * ========================================================================== */

/**
 * Future quantum domains should normally be introduced through:
 *
 *     qualified names
 *     generic type constructors
 *     semantic registries
 *     dialect namespaces
 *
 * rather than by adding a new lexer keyword for every scientific concept.
 *
 * This permits future concepts such as:
 *
 *     quantum::photonic::Mode
 *     quantum::neutral_atom::Atom
 *     quantum::topological::Qubit
 *     quantum::bosonic::Mode
 *     quantum::continuous_variable::State
 *
 * without forcing the core grammar to become a closed inventory.
 */


/* ============================================================================
 * 37. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * Stable syntax:
 *
 *     qubit
 *     logical qubit
 *     qubit[N]
 *     logical qubit[N]
 *     quantum<T>
 *     quantum::Type
 *
 * Adding a new qualified quantum type must not invalidate existing programs.
 *
 * New hardware technologies must not require rewriting these productions.
 *
 * Deprecated quantum types must be handled by semantic/versioning layers,
 * not by silently changing the meaning of existing parser productions.
 */


/* ============================================================================
 * 38. DETERMINISM CONTRACT
 * ========================================================================== */

/**
 * Parsing the same token stream with the same grammar version must produce
 * the same parse structure.
 *
 * This grammar contains:
 *
 *     no actions;
 *     no external state;
 *     no filesystem access;
 *     no network access;
 *     no hardware queries;
 *     no random behavior;
 *     no backend queries.
 */


/* ============================================================================
 * 39. SECURITY CONTRACT
 * ========================================================================== */

/**
 * This grammar performs no external operations.
 *
 * Resource/security limits such as:
 *
 *     maximum parse depth;
 *     maximum source size;
 *     maximum AST size;
 *     maximum diagnostic count;
 *
 * belong to parser/compiler configuration.
 *
 * They MUST NOT be expressed as language-level grammar constants.
 */


/* ============================================================================
 * 40. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * This file intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_REGISTER
 *     MAX_STATE_SIZE
 *     MAX_DIMENSION
 *     MAX_DEVICE_COUNT
 *     MAX_QPU_COUNT
 *     MAX_TOPOLOGY_SIZE
 *     MAX_GATE_COUNT
 *     MAX_NAMESPACE_DEPTH
 *
 * Namespace depth, generic arity and cardinality expression complexity are
 * deliberately represented recursively.
 */


/* ============================================================================
 * 41. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * QuantumTypes.g4 is complete when:
 *
 * [ ] It compiles as an ANTLR parser delegate against ZamaniTokens.
 *
 * [ ] It contains parser rules only.
 *
 * [ ] It declares no lexer rules.
 *
 * [ ] It introduces no hardware limits.
 *
 * [ ] It introduces no qubit-count limit.
 *
 * [ ] It introduces no physical qubit identifiers.
 *
 * [ ] It introduces no vendor-specific vocabulary.
 *
 * [ ] It introduces no gate inventory.
 *
 * [ ] It introduces no QEC algorithm.
 *
 * [ ] It introduces no ZQN semantics.
 *
 * [ ] It introduces no IR definitions.
 *
 * [ ] It introduces no runtime representation.
 *
 * [ ] It preserves symbolic cardinalities.
 *
 * [ ] It supports arbitrary namespace depth.
 *
 * [ ] It supports recursively nested quantum constructors.
 *
 * [ ] It is deterministic.
 *
 * [ ] It contains no actions or external effects.
 *
 * [ ] It remains compatible with Rust 1.97 / 1.97.1 generated integration.
 *
 * [ ] Generated Rust remains subject to deny(unsafe_code).
 *
 * [ ] Types remains the owner of the complete type-expression grammar.
 *
 * [ ] QuantumTypes remains the owner only of quantum-specific type syntax.
 *
 * [ ] The frontend maps the rules into the existing TypeExpr system.
 *
 * [ ] Quantum semantics eventually reach quantum::ir without the grammar
 *     becoming a second IR.
 */