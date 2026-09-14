/*
 * ============================================================================
 * Zamani Programming Language
 * grammar/quantum/gates.g4
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Source syntax for logical quantum gate applications.
 *
 * ARCHITECTURAL BOUNDARY
 * ----------------------
 * This grammar defines SYNTAX only.
 *
 * It does NOT define:
 *
 *   - QubitId
 *   - PhysicalQubitId
 *   - quantum IR
 *   - hardware topology
 *   - hardware calibration
 *   - scheduling
 *   - routing
 *   - optimization
 *   - QEC algorithms
 *   - ZQN/noise semantics
 *   - backend implementation
 *   - machine size
 *   - maximum qubit count
 *   - maximum register count
 *   - maximum gate count
 *
 * Logical gate syntax is lowered by the frontend/semantic layer into the
 * canonical quantum::ir representation.
 *
 * CANONICAL DOWNSTREAM BOUNDARY
 * -----------------------------
 *
 *   Zamani source
 *        |
 *        v
 *   ANTLR lexer/parser
 *        |
 *        v
 *   frontend AST
 *        |
 *        v
 *   semantic validation
 *        |
 *        v
 *   quantum::ir
 *        |
 *        +--> optimization
 *        +--> routing/mapping
 *        +--> scheduling
 *        +--> QEC
 *        +--> ZQN/fault semantics
 *        +--> hardware/backend
 *        +--> runtime
 *
 * This file must never introduce a competing QuantumGate representation.
 *
 * SCALABILITY
 * -----------
 * There are deliberately NO fixed machine-size limits in this grammar.
 *
 * In particular, this grammar never imposes:
 *
 *   MAX_QUBITS
 *   MAX_QUBITS = 32
 *   MAX_QUBITS = 64
 *   MAX_QUBITS = 1024
 *   q[0]
 *   q[1]
 *   fixed register sizes
 *   fixed hardware topology
 *   fixed physical device identifiers
 *
 * Qubit cardinality is determined by source declarations, semantic analysis,
 * resource constraints, target capabilities, and execution resources.
 *
 * STANDARD VS EXTENSIBLE OPERATIONS
 * ---------------------------------
 * Standard gates are represented by the stable gate-name vocabulary below.
 *
 * This vocabulary is intentionally finite.
 *
 * It does NOT mean Zamani can only express these gates.
 *
 * Custom/vendor/future operations must enter through the extensible quantum
 * operation/dialect mechanism rather than requiring this grammar to change
 * every time a new architecture appears.
 *
 * RUST IMPLEMENTATION CONTRACT
 * ----------------------------
 * The generated parser is consumed by the Rust frontend.
 *
 * Repository target:
 *
 *   Rust 1.97 / 1.97.1
 *   Rust 2021
 *   no unsafe
 *
 * The grammar itself contains no Rust unsafe code.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GATE APPLICATION
 * ============================================================================
 *
 * Generic gate application.
 *
 * Examples:
 *
 *   x(q);
 *   h(q);
 *   cx(control, target);
 *   rx(theta, q);
 *   u3(theta, phi, lambda, q);
 *
 * Parameters precede logical operands so that parameter expressions remain
 * unambiguous and can be lowered independently of physical placement.
 *
 * The semantic layer determines whether a named standard gate has the correct
 * parameter and operand arity.
 */
gateApplication
    : standardGateName
      gateParameterList?
      gateOperandList
      gateModifierList?
      gateAttributeList?
      ';'
    ;


/*
 * ============================================================================
 * CONTROLLED / ADJOINT / POWER / INVERSE FORMS
 * ============================================================================
 *
 * These constructs express semantic transformations of a logical operation.
 *
 * They do NOT imply any particular physical decomposition.
 *
 * For example:
 *
 *   controlled(x)(a, b);
 *
 * does not mean that the target backend must use a particular native
 * controlled-X instruction.
 *
 * The lowering layer determines the implementation.
 */
gateModifierList
    : gateModifier+
    ;

gateModifier
    : controlModifier
    | adjointModifier
    | inverseModifier
    | powerModifier
    ;

controlModifier
    : 'controlled'
    | 'control'
    ;

adjointModifier
    : 'adjoint'
    | 'dagger'
    ;

inverseModifier
    : 'inverse'
    ;

powerModifier
    : 'pow' '(' expression ')'
    ;


/*
 * ============================================================================
 * STANDARD GATE NAMES
 * ============================================================================
 *
 * These names correspond to the repository's canonical standard-gate
 * vocabulary where available.
 *
 * GateKind remains a semantic concern of the quantum IR.
 *
 * The grammar only recognizes source spellings.
 */
standardGateName
    : singleQubitFixedGate
    | singleQubitParameterizedGate
    | twoQubitFixedGate
    | twoQubitParameterizedGate
    | threeQubitGate
    ;


/*
 * ============================================================================
 * SINGLE-QUBIT FIXED GATES
 * ============================================================================
 */
singleQubitFixedGate
    : 'i'
    | 'id'
    | 'identity'

    | 'x'
    | 'X'
    | 'pauli_x'

    | 'y'
    | 'Y'
    | 'pauli_y'

    | 'z'
    | 'Z'
    | 'pauli_z'

    | 'h'
    | 'H'
    | 'hadamard'

    | 's'
    | 'S'

    | 'sdg'
    | 'Sdg'
    | 's_dagger'

    | 't'
    | 'T'

    | 'tdg'
    | 'Tdg'
    | 't_dagger'

    | 'v'
    | 'V'

    | 'vdg'
    | 'Vdg'
    | 'v_dagger'
    ;


/*
 * ============================================================================
 * SINGLE-QUBIT PARAMETERIZED GATES
 * ============================================================================
 */
singleQubitParameterizedGate
    : 'rx'
    | 'RX'

    | 'ry'
    | 'RY'

    | 'rz'
    | 'RZ'

    | 'phase'
    | 'Phase'

    | 'u1'
    | 'U1'

    | 'u2'
    | 'U2'

    | 'u3'
    | 'U3'
    ;


/*
 * ============================================================================
 * TWO-QUBIT FIXED GATES
 * ============================================================================
 */
twoQubitFixedGate
    : 'cx'
    | 'CX'
    | 'cnot'
    | 'CNOT'

    | 'cy'
    | 'CY'

    | 'cz'
    | 'CZ'

    | 'ch'
    | 'CH'

    | 'swap'
    | 'SWAP'

    | 'iswap'
    | 'ISWAP'

    | 'ecr'
    | 'ECR'
    ;


/*
 * ============================================================================
 * TWO-QUBIT PARAMETERIZED GATES
 * ============================================================================
 */
twoQubitParameterizedGate
    : 'crx'
    | 'CRX'

    | 'cry'
    | 'CRY'

    | 'crz'
    | 'CRZ'
    ;


/*
 * ============================================================================
 * THREE-QUBIT GATES
 * ============================================================================
 */
threeQubitGate
    : 'ccx'
    | 'CCX'
    | 'toffoli'
    | 'Toffoli'

    | 'cswap'
    | 'CSWAP'
    | 'fredkin'
    | 'Fredkin'
    ;


/*
 * ============================================================================
 * GATE PARAMETERS
 * ============================================================================
 *
 * Parameters are expressions, not machine constants.
 *
 * Valid examples:
 *
 *   pi / 2
 *   theta
 *   angle
 *   theta + phi
 *   f(x)
 *
 * The expression grammar owns expression semantics.
 */
gateParameterList
    : '(' gateParameter (',' gateParameter)* ')'
    ;

gateParameter
    : expression
    ;


/*
 * ============================================================================
 * LOGICAL GATE OPERANDS
 * ============================================================================
 *
 * Operand syntax intentionally references the existing quantum qubit grammar
 * instead of defining another QubitId or physical-qubit representation.
 *
 * The exact qubit-reference rule is expected to be supplied by
 * grammar/quantum/qubits.g4.
 */
gateOperandList
    : '(' gateOperand (',' gateOperand)* ')'
    ;

gateOperand
    : qubitReference
    | quantumRegisterSlice
    | quantumRegisterElement
    ;


/*
 * ============================================================================
 * BROADCAST / REGISTER OPERATIONS
 * ============================================================================
 *
 * Register-wide operations are semantic sugar.
 *
 * They do not impose a maximum register size.
 *
 * Examples:
 *
 *   h(register);
 *   x(register[range]);
 *
 * The frontend must expand or represent these operations without changing
 * their semantic meaning.
 */
registerGateApplication
    : standardGateName
      gateParameterList?
      registerGateOperand
      gateModifierList?
      gateAttributeList?
      ';'
    ;

registerGateOperand
    : quantumRegisterReference
    | quantumRegisterSlice
    ;


/*
 * ============================================================================
 * MEASUREMENT
 * ============================================================================
 *
 * Measurement is intentionally separated from unitary gate syntax.
 *
 * The canonical semantic representation belongs to quantum::ir::measurement.
 *
 * This grammar supports:
 *
 *   measure(q) -> c;
 *   measure q -> c;
 *   measure(q);
 *
 * depending on the surrounding language conventions.
 *
 * The semantic layer must reject ambiguous or incomplete forms.
 */
measurementStatement
    : 'measure'
      measurementSource
      measurementTarget?
      gateAttributeList?
      ';'
    ;

measurementSource
    : '(' quantumMeasurementOperandList ')'
    | gateOperand
    | quantumRegisterReference
    | quantumRegisterSlice
    ;

quantumMeasurementOperandList
    : gateOperand (',' gateOperand)*
    ;

measurementTarget
    : '->' classicalMeasurementTarget
    | 'into' classicalMeasurementTarget
    ;

classicalMeasurementTarget
    : classicalBitReference
    | classicalRegisterReference
    | classicalRegisterSlice
    ;


/*
 * ============================================================================
 * RESET
 * ============================================================================
 *
 * Reset is a semantic quantum operation, not a hardware pulse.
 *
 * It therefore belongs in source syntax but is interpreted downstream by
 * quantum::ir and the backend.
 */
resetStatement
    : 'reset'
      resetOperand
      gateAttributeList?
      ';'
    ;

resetOperand
    : gateOperand
    ;


/*
 * ============================================================================
 * BARRIER
 * ============================================================================
 *
 * Barrier is a semantic synchronization boundary.
 *
 * It does NOT define:
 *
 *   - a hardware synchronization primitive;
 *   - a pulse;
 *   - a clock cycle;
 *   - a fixed duration;
 *   - a topology.
 *
 * Scheduling determines the eventual implementation.
 */
barrierStatement
    : 'barrier'
      barrierOperandList
      gateAttributeList?
      ';'
    ;

barrierOperandList
    : gateOperand (',' gateOperand)*
    ;


/*
 * ============================================================================
 * GENERIC QUANTUM OPERATION
 * ============================================================================
 *
 * Standard gates are not the entire quantum language.
 *
 * This rule provides the syntactic bridge to the extensible quantum
 * operation/dialect system.
 *
 * A custom operation must be resolved semantically through a registered
 * dialect/operation namespace.
 *
 * The grammar does not attempt to enumerate all future operations.
 */
quantumOperationApplication
    : quantumOperationName
      gateParameterList?
      gateOperandList
      gateModifierList?
      gateAttributeList?
      ';'
    ;

quantumOperationName
    : qualifiedQuantumOperationName
    ;

qualifiedQuantumOperationName
    : quantumOperationIdentifier
      ('::' quantumOperationIdentifier)*
    ;

quantumOperationIdentifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * GATE ATTRIBUTES
 * ============================================================================
 *
 * Attributes annotate source intent.
 *
 * They must not silently become hardware requirements.
 *
 * Examples:
 *
 *   @logical
 *   @fault_tolerant
 *   @approximate
 *   @commutative
 *
 * Hardware-specific attributes must be rejected here unless explicitly
 * declared through the target/capability system.
 */
gateAttributeList
    : gateAttribute+
    ;

gateAttribute
    : '@' IDENTIFIER
      ('(' gateAttributeArguments? ')')?
    ;

gateAttributeArguments
    : gateAttributeArgument (',' gateAttributeArgument)*
    ;

gateAttributeArgument
    : expression
    | IDENTIFIER
    | STRING
    ;


/*
 * ============================================================================
 * GATE DECLARATIONS
 * ============================================================================
 *
 * User-defined logical gates.
 *
 * A user-defined gate describes reusable semantic behavior.
 *
 * It does not bind the gate to:
 *
 *   - physical qubits;
 *   - hardware topology;
 *   - a vendor;
 *   - a pulse;
 *   - a backend;
 *   - a calibration.
 *
 * Example:
 *
 *   gate bell(a, b) {
 *       h(a);
 *       cx(a, b);
 *   }
 */
gateDeclaration
    : gateVisibilityModifier?
      'gate'
      IDENTIFIER
      gateFormalParameterList?
      gateFormalOperandList
      gateAttributeList?
      gateBlock
    ;

gateVisibilityModifier
    : 'pub'
    | 'private'
    | 'internal'
    ;

gateFormalParameterList
    : '(' gateFormalParameter (',' gateFormalParameter)* ')'
    ;

gateFormalParameter
    : IDENTIFIER
      (':' typeExpr)?
    ;

gateFormalOperandList
    : '(' gateFormalOperand (',' gateFormalOperand)* ')'
    ;

gateFormalOperand
    : IDENTIFIER
      (':' quantumOperandType)?
    ;

quantumOperandType
    : 'qubit'
    | 'qubit_ref'
    | 'logical_qubit'
    | 'qubit_register'
    ;

gateBlock
    : '{'
      quantumGateStatement*
      '}'
    ;


/*
 * ============================================================================
 * QUANTUM GATE STATEMENT
 * ============================================================================
 *
 * This is the compositional entry point for a logical quantum gate body.
 */
quantumGateStatement
    : gateApplication
    | registerGateApplication
    | measurementStatement
    | resetStatement
    | barrierStatement
    | quantumOperationApplication
    | quantumGateControlStatement
    ;


/*
 * ============================================================================
 * CONDITIONAL QUANTUM GATES
 * ============================================================================
 *
 * Dynamic circuits are expressed in terms of classical conditions.
 *
 * The grammar does not specify how the backend realizes the condition.
 */
quantumGateControlStatement
    : 'if'
      '(' expression ')'
      quantumControlledBody
    ;

quantumControlledBody
    : gateApplication
    | registerGateApplication
    | measurementStatement
    | resetStatement
    | barrierStatement
    | quantumOperationApplication
    ;


/*
 * ============================================================================
 * INVOCATION FORM
 * ============================================================================
 *
 * Explicit invocation form is useful for user-defined and dialect operations.
 *
 * Example:
 *
 *   apply bell(a, b);
 */
gateInvocation
    : 'apply'
      quantumOperationName
      gateParameterList?
      gateOperandList
      gateAttributeList?
      ';'
    ;


/*
 * ============================================================================
 * REFERENCE CONTRACTS
 * ============================================================================
 *
 * These names are intentionally references to rules owned elsewhere.
 *
 * They MUST NOT be redefined in this file.
 *
 * Expected owners:
 *
 *   qubitReference
 *       grammar/quantum/qubits.g4
 *
 *   quantumRegisterReference
 *   quantumRegisterSlice
 *   quantumRegisterElement
 *       grammar/quantum/qubits.g4
 *
 *   classicalBitReference
 *   classicalRegisterReference
 *   classicalRegisterSlice
 *       classical/types/register grammar
 *
 *   expression
 *       grammar/expressions/expressions.g4
 *
 *   typeExpr
 *       grammar/types/types.g4
 *
 *   IDENTIFIER
 *   STRING
 *       lexer grammar
 *
 * These names are deliberately documented rather than duplicated here.
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser accepts syntactically valid gate forms.
 *
 * Semantic analysis MUST subsequently validate:
 *
 *   1. gate existence;
 *   2. standard-gate identity;
 *   3. parameter count;
 *   4. operand count;
 *   5. operand types;
 *   6. duplicate logical operands where prohibited;
 *   7. classical measurement destination;
 *   8. measurement compatibility;
 *   9. register broadcast semantics;
 *  10. modifier legality;
 *  11. user-defined gate signatures;
 *  12. dialect operation resolution;
 *  13. effect/capability requirements;
 *  14. resource requirements;
 *  15. target compatibility.
 *
 * This grammar MUST NOT attempt to perform those semantic checks.
 */


/*
 * ============================================================================
 * HARDWARE-INDEPENDENCE CONTRACT
 * ============================================================================
 *
 * Forbidden source-level assumptions in this grammar:
 *
 *   physical_qubit
 *   device_0
 *   device_1
 *   qpu_0
 *   topology_*
 *   fixed coupling maps
 *   fixed channel numbers
 *   fixed pulse IDs
 *   fixed calibration IDs
 *   fixed clock periods
 *   fixed qubit counts
 *   fixed processor counts
 *
 * Such concepts belong to target/hardware/resource descriptions.
 */


/*
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New standard gate:
 *
 *   1. Add source spelling here.
 *   2. Add corresponding canonical GateKind only if it genuinely belongs to
 *      the stable standard gate vocabulary.
 *   3. Add semantic arity/parameter rules in quantum::ir.
 *   4. Add AST/lowering support.
 *   5. Add parser and semantic tests.
 *   6. Add documentation and compatibility entry.
 *
 * New vendor/future/custom operation:
 *
 *   MUST NOT automatically require modifying this file.
 *
 * It should use quantumOperationApplication and the dialect/operation
 * registration mechanism.
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing preserves:
 *
 *   - gate order;
 *   - operand order;
 *   - parameter expression order;
 *   - source locations;
 *   - attributes.
 *
 * This grammar never sorts operands.
 *
 * For example:
 *
 *   cx(a, b)
 *
 * is not equivalent at the syntax level to:
 *
 *   cx(b, a)
 *
 * because control/target ordering can be semantically significant.
 */


/*
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *   - filesystem access;
 *   - network access;
 *   - device discovery;
 *   - backend invocation;
 *   - command execution;
 *   - dynamic code execution.
 *
 * Parsing untrusted source must therefore remain a pure frontend operation.
 */


/*
 * ============================================================================
 * END OF gates.g4
 * ============================================================================
 */