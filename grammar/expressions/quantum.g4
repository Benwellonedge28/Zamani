/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/quantum.g4
 *
 * Status:
 *     Canonical quantum-expression component grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL QUANTUM EXPRESSIONS.
 *
 * It does NOT own:
 *
 *     - the complete Zamani expression hierarchy;
 *     - the complete quantum statement grammar;
 *     - quantum declarations;
 *     - quantum type declarations;
 *     - quantum hardware descriptions;
 *     - physical qubit identifiers;
 *     - hardware topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC implementation;
 *     - ZQN implementation;
 *     - HAL implementation;
 *     - backend selection;
 *     - target-specific gate sets;
 *     - quantum::ir.
 *
 * Canonical pipeline:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     expression parser
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> decomposition
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC / resilience
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target realization
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT create a second quantum IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani must support:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore quantum expressions describe:
 *
 *     WHAT operation is requested
 *     WHAT operands participate
 *     WHAT parameters are supplied
 *     WHAT quantum values are produced
 *
 * They do NOT prescribe:
 *
 *     WHICH QPU
 *     WHICH physical qubit
 *     WHICH topology
 *     WHICH native gate set
 *     WHICH calibration
 *     WHICH pulse implementation
 *     WHICH scheduler
 *     WHICH routing algorithm
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No universal finite limit is encoded here for:
 *
 *     qubits
 *     registers
 *     operands
 *     parameters
 *     controls
 *     targets
 *     circuit depth
 *     operation count
 *     expression depth
 *     register cardinality
 *     tensor dimensions
 *     distributed participants
 *     logical qubits
 *     timelines
 *     devices
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * "Infinity" means that the language introduces no artificial finite semantic
 * ceiling. Actual resource limits belong to compiler/runtime/resource policy
 * and available execution resources.
 *
 * ============================================================================
 * ARCHITECTURAL SEPARATION
 * ============================================================================
 *
 * This file distinguishes:
 *
 *     1. Quantum operation identity
 *     2. Quantum operation parameters
 *     3. Quantum operands
 *     4. Controls
 *     5. Targets
 *     6. Measurements
 *     7. State/value expressions
 *     8. Quantum composition
 *
 * It intentionally does NOT distinguish a closed set such as:
 *
 *     X
 *     Y
 *     Z
 *     H
 *     CNOT
 *     SWAP
 *     ...
 *
 * Those names remain identifiers and are resolved semantically.
 *
 * ============================================================================
 * GENERIC QUANTUM OPERATION MODEL
 * ============================================================================
 *
 * Conceptually:
 *
 *     operation-name
 *     operation-namespace
 *     operation-parameters
 *     operation-operands
 *     operation-modifiers
 *
 * become a generic frontend operation representation.
 *
 * They may eventually lower to:
 *
 *     quantum::ir
 *
 * without introducing a grammar-level QuantumGate enum.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every rule in this file must map to the existing domain-neutral frontend
 * AST contract.
 *
 * Quantum-specific source information that must survive parsing includes:
 *
 *     - operation name;
 *     - namespace/qualification;
 *     - source span;
 *     - argument order;
 *     - parameter order;
 *     - operand order;
 *     - control/target roles;
 *     - measurement destination;
 *     - modifiers;
 *     - syntactic nesting.
 *
 * The AST must remain generic/extensible.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *     - whether an operation exists;
 *     - whether an operation is quantum;
 *     - whether operands have quantum types;
 *     - whether controls are valid;
 *     - whether target cardinality is valid;
 *     - whether parameters have valid types;
 *     - whether measurement is legal;
 *     - whether reset is legal;
 *     - whether an operation requires capabilities;
 *     - whether an operation is compatible with the selected execution model.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * The grammar MUST NOT decide:
 *
 *     physical qubit count;
 *     physical device ID;
 *     physical topology;
 *     number of available QPUs;
 *     maximum circuit depth;
 *     native gate set;
 *     available calibration.
 *
 * Those are resolved downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - lexical vocabulary;
 *     - grammar version.
 *
 * Parsing must not depend on:
 *
 *     - hardware discovery;
 *     - runtime state;
 *     - device state;
 *     - randomness;
 *     - current time;
 *     - network state;
 *     - scheduler state.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Integration target:
 *
 *     grammar/expressions/expressions.g4
 *
 * The existing public expression hierarchy remains authoritative.
 *
 * This file contributes:
 *
 *     quantumExpression
 *
 * and its private/supporting rules.
 *
 * It MUST NOT define:
 *
 *     expression
 *     assignmentExpression
 *     postfixExpression
 *     primaryExpression
 *
 * because those belong to the canonical expression hierarchy.
 *
 * Existing grammar/expressions/expressions.g4 currently contains a quantum
 * expression branch. That branch should be migrated to this component and
 * removed from the competing composition surface once this file is integrated.
 *
 * Likewise, the quantum-specific expression productions in
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * must eventually delegate to the same canonical component rather than
 * remaining an independent quantum expression implementation.
 *
 * ============================================================================
 * STATEMENT BOUNDARY
 * ============================================================================
 *
 * Statements such as:
 *
 *     apply ...
 *     measure ...
 *     reset ...
 *
 * may also exist in grammar/quantum/ and in the parser's statement layer.
 *
 * This file owns expression-valued forms only.
 *
 * Statement-level quantum syntax must not be duplicated here.
 *
 * ============================================================================
 * NO HARD-CODED GATE INVENTORY
 * ============================================================================
 *
 * NEVER add:
 *
 *     quantumGate
 *         : X
 *         | H
 *         | CNOT
 *         | ...
 *
 * The operation name is intentionally open.
 *
 * This permits:
 *
 *     H
 *     X
 *     CNOT
 *     vendor::operation
 *     logical::operation
 *     custom_operation
 *     future_operation
 *
 * without changing the grammar merely because a new operation is introduced.
 *
 * ============================================================================
 */

parser grammar QuantumExpressions;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC QUANTUM EXPRESSION ENTRY
 * ========================================================================== */

/**
 * Quantum expression.
 *
 * This is the public integration point consumed by the canonical expression
 * grammar.
 *
 * Supported expression-valued quantum forms include:
 *
 *     quantum { ... }
 *     apply ...
 *     measure ...
 *     reset ...
 *     barrier ...
 *     entangle ...
 *     controlled ...
 *     adjoint ...
 *     inverse ...
 *     observe ...
 *     quantum operation invocation
 */
quantumExpression
    : quantumBlockExpression
    | quantumApplyExpression
    | quantumMeasureExpression
    | quantumResetExpression
    | quantumBarrierExpression
    | quantumEntangleExpression
    | quantumControlledExpression
    | quantumAdjointExpression
    | quantumInverseExpression
    | quantumObserveExpression
    | quantumOperationExpression
    ;


/* ============================================================================
 * 2. QUANTUM BLOCK
 * ========================================================================== */

/**
 * Quantum block.
 *
 * A quantum block creates a syntactic quantum-computation region.
 *
 * Example:
 *
 *     quantum {
 *         apply H to q;
 *         apply CNOT to q[0], q[1];
 *     }
 *
 * The grammar does not assign physical meaning to the block.
 *
 * Block contents are delegated to the existing block/statement/expression
 * composition during integration.
 */
quantumBlockExpression
    : K_QUANTUM LBRACE quantumBlockElement* RBRACE
    ;

quantumBlockElement
    : quantumBlockStatement
    | quantumBlockExpression
    ;

quantumBlockStatement
    : quantumApplyExpression
    | quantumMeasureExpression
    | quantumResetExpression
    | quantumBarrierExpression
    | quantumEntangleExpression
    | quantumControlledExpression
    | quantumAdjointExpression
    | quantumInverseExpression
    | quantumObserveExpression
    | quantumOperationExpression
    ;


/* ============================================================================
 * 3. APPLY
 * ========================================================================== */

/**
 * Generic quantum operation application.
 *
 * Examples:
 *
 *     apply H to q;
 *     apply CNOT to control, target;
 *     apply U(theta, phi, lambda) to q;
 *     apply vendor::operation(a, b) to q0, q1;
 *
 * Operation identity remains an open qualified name.
 */
quantumApplyExpression
    : K_APPLY quantumOperationSpecification quantumTargetClause
    ;


/* ============================================================================
 * 4. OPERATION SPECIFICATION
 * ========================================================================== */

/**
 * Operation specification.
 *
 * This deliberately accepts a generic operation name rather than a fixed
 * quantum-gate catalogue.
 */
quantumOperationSpecification
    : quantumOperationName
      quantumOperationArguments?
      quantumOperationModifier*
    ;


/**
 * Qualified operation name.
 *
 * Examples:
 *
 *     H
 *     CNOT
 *     vendor::operation
 *     logical::rotation
 *     custom::future_operation
 *
 * The semantic layer determines whether the name denotes a valid quantum
 * operation.
 */
quantumOperationName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/**
 * Operation parameters.
 *
 * Parameters are syntactic expressions and therefore remain target-independent.
 *
 * Examples:
 *
 *     U(theta, phi, lambda)
 *     rotation(angle)
 *     custom(a + b)
 */
quantumOperationArguments
    : LPAREN quantumArgumentList? RPAREN
    ;

quantumArgumentList
    : quantumArgument (COMMA quantumArgument)* COMMA?
    ;

quantumArgument
    : quantumArgumentAtom
    ;

quantumArgumentAtom
    : quantumLiteralValue
    | quantumIdentifierReference
    | quantumQualifiedReference
    | quantumParenthesizedArgument
    ;


/**
 * Parenthesized quantum argument.
 *
 * Arithmetic and general expression composition remain owned by the canonical
 * expression grammar. This rule provides the quantum component's syntactic
 * grouping boundary without introducing another expression hierarchy.
 */
quantumParenthesizedArgument
    : LPAREN quantumArgumentAtom RPAREN
    ;


/* ============================================================================
 * 5. OPERATION MODIFIERS
 * ========================================================================== */

/**
 * Operation modifiers are syntactic qualifiers.
 *
 * They do not select a backend implementation.
 */
quantumOperationModifier
    : quantumControlModifier
    | quantumAdjointModifier
    | quantumInverseModifier
    ;


/**
 * Controlled operation modifier.
 *
 * Example:
 *
 *     controlled H
 *     controlled(2) H
 *
 * The optional cardinality is a program-level value, not a grammar limit.
 */
quantumControlModifier
    : K_CONTROL
      quantumControlSpecification?
    | K_CONTROLLED
      quantumControlSpecification?
    ;

quantumControlSpecification
    : LPAREN quantumArgumentList? RPAREN
    ;


/**
 * Adjoint modifier.
 */
quantumAdjointModifier
    : K_ADJOINT
    ;


/**
 * Inverse modifier.
 */
quantumInverseModifier
    : K_INVERSE
    ;


/* ============================================================================
 * 6. TARGET CLAUSE
 * ========================================================================== */

/**
 * Quantum target specification.
 *
 * Targets remain expressions because quantum targets may be:
 *
 *     registers
 *     slices
 *     indexed qubits
 *     symbolic references
 *     computed views
 *     logical resources
 *
 * Physical placement is not represented here.
 */
quantumTargetClause
    : K_TARGET quantumTargetList
    | K_TO quantumTargetList
    ;

quantumTargetList
    : quantumTarget (COMMA quantumTarget)*
    ;

quantumTarget
    : quantumTargetAtom
    ;

quantumTargetAtom
    : quantumIdentifierReference
    | quantumQualifiedReference
    | quantumTargetIndex
    | quantumTargetSlice
    | quantumParenthesizedTarget
    ;

quantumTargetIndex
    : quantumIdentifierReference
      LBRACKET quantumIndexExpression RBRACKET
    ;

quantumIndexExpression
    : quantumIndexAtom
    ;

quantumIndexAtom
    : quantumIdentifierReference
    | quantumLiteralValue
    | quantumQualifiedReference
    ;

quantumTargetSlice
    : quantumIdentifierReference
      LBRACKET quantumSliceExpression RBRACKET
    ;

quantumSliceExpression
    : quantumSliceBound?
      DOT_DOT
      quantumSliceBound?
    | quantumSliceBound?
      DOT_DOT_EQ
      quantumSliceBound?
    ;

quantumSliceBound
    : quantumIdentifierReference
    | quantumLiteralValue
    | quantumQualifiedReference
    ;

quantumParenthesizedTarget
    : LPAREN quantumTarget RPAREN
    ;


/* ============================================================================
 * 7. MEASUREMENT
 * ========================================================================== */

/**
 * Expression-valued measurement.
 *
 * Examples:
 *
 *     measure q
 *     measure q[0]
 *
 * The result type and measurement semantics belong to semantic analysis.
 */
quantumMeasureExpression
    : K_MEASURE quantumMeasurementOperand
    ;

quantumMeasurementOperand
    : quantumTargetList
    ;


/* ============================================================================
 * 8. RESET
 * ========================================================================== */

/**
 * Expression-valued reset.
 *
 * Reset semantics are downstream concerns.
 */
quantumResetExpression
    : K_RESET quantumTargetList
    ;


/* ============================================================================
 * 9. BARRIER
 * ========================================================================== */

/**
 * Barrier expression.
 *
 * A barrier is a semantic operation boundary.
 *
 * The grammar does not determine how a backend realizes it.
 */
quantumBarrierExpression
    : K_BARRIER quantumTargetList?
    ;


/* ============================================================================
 * 10. ENTANGLEMENT
 * ========================================================================== */

/**
 * Generic entanglement expression.
 *
 * The grammar does not prescribe:
 *
 *     - a particular entangling gate;
 *     - topology;
 *     - physical interaction;
 *     - native gate decomposition.
 */
quantumEntangleExpression
    : K_ENTANGLE quantumTargetList
    ;


/* ============================================================================
 * 11. CONTROLLED EXPRESSIONS
 * ========================================================================== */

/**
 * Explicit controlled operation.
 *
 * Examples:
 *
 *     controlled H to control, target
 *     controlled(2) operation to controls, targets
 */
quantumControlledExpression
    : K_CONTROLLED
      quantumOperationSpecification
      quantumTargetClause
    ;


/* ============================================================================
 * 12. ADJOINT
 * ========================================================================== */

/**
 * Adjoint expression.
 *
 * The semantic layer determines whether the referenced operation has an
 * adjoint and how it is represented in quantum::ir.
 */
quantumAdjointExpression
    : K_ADJOINT quantumOperationReference
    ;

quantumOperationReference
    : quantumOperationSpecification
    ;


/* ============================================================================
 * 13. INVERSE
 * ========================================================================== */

/**
 * Inverse expression.
 */
quantumInverseExpression
    : K_INVERSE quantumOperationReference
    ;


/* ============================================================================
 * 14. OBSERVATION
 * ========================================================================== */

/**
 * Observable/observation expression.
 *
 * Examples:
 *
 *     observe observable_name
 *     observe observable_name on q
 *
 * The physical measurement strategy is not grammar-level information.
 */
quantumObserveExpression
    : K_OBSERVE quantumObservableSpecification
    ;

quantumObservableSpecification
    : quantumObservableName
      quantumObservableTargets?
    ;

quantumObservableName
    : quantumOperationName
    ;

quantumObservableTargets
    : K_ON quantumTargetList
    ;


/* ============================================================================
 * 15. GENERIC OPERATION EXPRESSION
 * ========================================================================== */

/**
 * Generic quantum operation expression.
 *
 * This is intentionally extensible.
 *
 * It permits semantic operation resolution without requiring every operation
 * to become a new grammar keyword.
 *
 * Example:
 *
 *     custom_operation(q)
 *     vendor::operation(theta, phi)
 *
 * A bare identifier is NOT automatically considered quantum by the semantic
 * layer; semantic resolution determines its domain.
 */
quantumOperationExpression
    : quantumOperationSpecification
      quantumTargetClause?
    ;


/* ============================================================================
 * 16. QUANTUM VALUES
 * ========================================================================== */

/**
 * Quantum source literals.
 *
 * The lexical layer owns the exact token representation.
 *
 * Examples include:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 */
quantumLiteralValue
    : QUANTUM_LITERAL
    ;


/**
 * Unqualified quantum identifier reference.
 */
quantumIdentifierReference
    : identifier
    ;


/**
 * Qualified quantum identifier reference.
 */
quantumQualifiedReference
    : identifier
      DOUBLE_COLON
      identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 17. LEXICAL BRIDGE
 * ========================================================================== */

/**
 * `identifier` is intentionally kept as a local integration rule.
 *
 * The canonical identifier implementation may be supplied by the composed
 * core/name grammar.
 *
 * When the expression composition is finalized, this rule must be replaced
 * by/delegated to the repository's canonical identifier rule rather than
 * creating a second identifier representation.
 *
 * This local bridge exists only so the quantum component has an explicit,
 * documented integration boundary.
 */
identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 18. INTEGRATION NOTES
 * ============================================================================
 *
 * The following migration must be performed at the composition layer:
 *
 *     grammar/expressions/expressions.g4
 *
 * Its existing quantumExpression production should delegate to:
 *
 *     quantumExpression
 *
 * from this component.
 *
 * Existing quantum syntax in:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * should likewise be consolidated so that only one quantum-expression contract
 * remains authoritative.
 *
 * The following files remain separate concerns:
 *
 *     grammar/quantum/operations.g4
 *     grammar/quantum/measurement.g4
 *     grammar/quantum/states.g4
 *     grammar/quantum/types.g4
 *     grammar/quantum/quantum-types.g4
 *     grammar/quantum/quantum-capabilities.g4
 *     grammar/quantum/quantum-states.g4
 *     grammar/effects/quantum.g4
 *     grammar/hardware/quantum-device.g4
 *
 * This file MUST NOT duplicate their declarations.
 *
 * Those files provide domain contracts consumed by semantic analysis or other
 * grammar compositions.
 *
 * ============================================================================
 * 19. AST INTEGRATION
 * ============================================================================
 *
 * Recommended mappings:
 *
 *     quantumOperationExpression
 *         -> generic Operation expression
 *
 *     quantumApplyExpression
 *         -> Operation application
 *
 *     quantumMeasureExpression
 *         -> Measurement expression
 *
 *     quantumResetExpression
 *         -> Reset operation expression
 *
 *     quantumBarrierExpression
 *         -> Barrier operation expression
 *
 *     quantumEntangleExpression
 *         -> Generic quantum operation / semantic operation
 *
 *     quantumControlledExpression
 *         -> Operation + control metadata
 *
 *     quantumAdjointExpression
 *         -> Operation + adjoint modifier
 *
 *     quantumInverseExpression
 *         -> Operation + inverse modifier
 *
 *     quantumObserveExpression
 *         -> Observable expression
 *
 * No frontend AST node should encode a fixed exhaustive gate enumeration.
 *
 * ============================================================================
 * 20. QUANTUM::IR INTEGRATION
 * ============================================================================
 *
 * Lowering path:
 *
 *     source
 *       |
 *       v
 *     quantumExpression
 *       |
 *       v
 *     generic frontend AST operation
 *       |
 *       v
 *     semantic quantum operation
 *       |
 *       v
 *     quantum::ir
 *
 * This grammar must never construct:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     RoutedCircuit
 *     ScheduledCircuit
 *     QECProgram
 *     ZQNProgram
 *
 * Those are downstream concepts.
 *
 * ============================================================================
 * 21. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_CONTROLS
 *     MAX_TARGETS
 *     MAX_PARAMETERS
 *     MAX_REGISTER_SIZE
 *     MAX_CIRCUIT_DEPTH
 *     MAX_OPERATION_COUNT
 *     MAX_DEVICES
 *     MAX_QPU_COUNT
 *     MAX_TOPOLOGY_SIZE
 *
 * Repetition operators such as `*`, `+`, and `?` are language structure and
 * do not constitute hardware limits.
 *
 * ============================================================================
 * 22. NEGATIVE REQUIREMENTS
 * ============================================================================
 *
 * This grammar must reject at semantic/structural validation time, or prevent
 * from becoming valid universal syntax:
 *
 *     fixed physical qubit identifiers;
 *     fixed backend gate inventories;
 *     hardware vendor instructions;
 *     physical topology assumptions;
 *     scheduler directives masquerading as quantum expressions;
 *     calibration data masquerading as syntax;
 *     QEC implementation details;
 *     ZQN implementation details;
 *     fixed resource limits.
 *
 * ============================================================================
 * 23. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] quantumExpression is the sole quantum-expression component entry.
 *     [ ] No second public `expression` rule exists here.
 *     [ ] No fixed QuantumGate enumeration exists.
 *     [ ] Operation names remain extensible.
 *     [ ] Operation parameters are structurally extensible.
 *     [ ] Target lists have no artificial cardinality limit.
 *     [ ] Control expressions have no artificial cardinality limit.
 *     [ ] Measurement is expression-capable.
 *     [ ] Reset is expression-capable.
 *     [ ] Barrier is expression-capable.
 *     [ ] Adjoint is expression-capable.
 *     [ ] Inverse is expression-capable.
 *     [ ] Observable expressions are represented.
 *     [ ] Quantum blocks are represented.
 *     [ ] Physical hardware is not encoded.
 *     [ ] quantum::ir remains the semantic boundary.
 *     [ ] Existing AST contracts are identified.
 *     [ ] Existing quantum grammar files are not duplicated.
 *     [ ] Existing expression composition delegates here.
 *     [ ] ZamaniParser quantum expressions are consolidated.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Compatibility tests exist.
 *     [ ] No unsafe Rust requirement exists.
 *     [ ] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * ============================================================================
 */