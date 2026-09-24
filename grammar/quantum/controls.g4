/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/controls.g4
 *
 * Grammar:
 *     QuantumControls
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM CONTROL-MODIFIER GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical grammar owner for SOURCE-LEVEL QUANTUM
 * CONTROL-MODIFIER SYNTAX.
 *
 * It describes how a quantum operation may be structurally annotated with
 * control information without defining the operation itself.
 *
 * Examples of the intended semantic forms include:
 *
 *     control(q0)
 *
 *     control(q0, q1)
 *
 *     control(q0, q1) polarity(positive)
 *
 *     control(q0) state(|1>)
 *
 *     control(q0) state(|0>)
 *
 *     control(register)
 *
 *     control(register[index])
 *
 *     control(register[start .. end])
 *
 *     control(q0, q1) adjoint(...)
 *
 * The exact operation invocation remains owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * This file therefore MUST NOT define another operation invocation grammar.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A control is a semantic modifier over an operation.
 *
 * Conceptually:
 *
 *     operation
 *          |
 *          +---- control specification
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *
 * The control grammar does NOT determine:
 *
 *     - whether an operation supports control;
 *     - whether a target may also be a control;
 *     - how many controls an operation semantically accepts;
 *     - whether a control is quantum or classical;
 *     - whether control polarity is physically implementable;
 *     - whether a controlled operation has a native target instruction;
 *     - how a controlled operation is decomposed;
 *     - how controls are routed;
 *     - how controls are scheduled;
 *     - how controls are error-corrected;
 *     - how controls are mapped to physical qubits;
 *     - how controls are calibrated;
 *     - how a backend implements control.
 *
 * Those are downstream semantic/compiler/runtime responsibilities.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     quantumControlModifier
 *     quantumControlSpecification
 *     quantumControlList
 *     quantumControl
 *     quantumControlPolarity
 *     quantumControlState
 *     quantumControlStateExpression
 *     quantumControlModifierAttributes
 *     quantumControlModifierAttribute
 *
 * This file owns only the structural syntax necessary to represent
 * target-independent control intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - operation invocation;
 *     - operation names;
 *     - operation declarations;
 *     - operation parameters;
 *     - operation targets;
 *     - qubit declarations;
 *     - quantum register declarations;
 *     - quantum types;
 *     - quantum references;
 *     - general expressions;
 *     - classical control-flow;
 *     - dynamic-circuit statements;
 *     - measurement;
 *     - reset;
 *     - observables;
 *     - adjoint implementation;
 *     - inverse implementation;
 *     - powers;
 *     - decomposition;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - hardware topology;
 *     - calibration;
 *     - physical qubit allocation;
 *     - device selection;
 *     - resource allocation;
 *     - capability discovery;
 *     - canonical quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexer
 *       |
 *       v
 *     core names / expressions / quantum references
 *       |
 *       v
 *     QuantumControls
 *       |
 *       v
 *     quantum operations
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> decomposition
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC
 *       +--> ZQN
 *       +--> resilience
 *       +--> HAL
 *       |
 *       v
 *     target lowering
 *       |
 *       v
 *     runtime
 *
 * This grammar MUST NOT introduce a reverse dependency.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Control syntax is target-independent.
 *
 * It MUST NOT encode assumptions about:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     control count
 *     register width
 *     memory size
 *     device count
 *     network size
 *     topology
 *     physical qubit identifiers
 *     native gate sets
 *     pulse channels
 *     calibration
 *     scheduler capacity
 *
 * There is deliberately NO:
 *
 *     MAX_CONTROLS
 *     MAX_QUBITS
 *     MAX_CONTROL_REGISTER_SIZE
 *     MAX_TARGETS
 *     MAX_CONTROL_DEPTH
 *
 * or equivalent language-level ceiling.
 *
 * Repetition is represented using normal ANTLR repetition constructs.
 *
 * Actual feasibility is determined downstream from:
 *
 *     semantic requirements
 *     capability information
 *     resource availability
 *     compiler policy
 *     target constraints
 *     runtime resources
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * This file describes:
 *
 *     WHAT control relationship the programmer requested.
 *
 * It does not prescribe:
 *
 *     HOW the relationship is implemented.
 *
 * For example:
 *
 *     control(c0, c1)
 *
 * does not imply:
 *
 *     two physical control wires;
 *
 *     two physical qubits;
 *
 *     a particular native gate;
 *
 *     a particular decomposition;
 *
 *     a particular topology;
 *
 *     a particular control processor.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file consumes tokens from the canonical ZamaniLexer vocabulary.
 *
 * Known shared lexical concepts include:
 *
 *     CONTROL
 *     POSITIVE
 *     NEGATIVE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     LBRACK
 *     RBRACK
 *     LBRACE
 *     RBRACE
 *     COLON
 *
 * Where a control attribute name is not semantically required to be a
 * language keyword, it remains an IDENTIFIER rather than becoming a new
 * keyword.
 *
 * This prevents unnecessary growth of the reserved-word vocabulary.
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * NAME / EXPRESSION AUTHORITY
 * ============================================================================
 *
 * Names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * Expressions remain owned by the canonical expression grammar.
 *
 * Quantum reference syntax remains owned by the quantum-reference/type
 * boundary rather than being recreated here.
 *
 * Consequently this file MUST NOT define:
 *
 *     qualifiedName
 *     identifier
 *     expression
 *     quantumReference
 *     qubitReference
 *     quantumTarget
 *     rangeExpression
 *
 * as competing canonical productions.
 *
 * ============================================================================
 * DESIGN RULE
 * ============================================================================
 *
 * A control operand is syntactically an expression.
 *
 * Whether that expression denotes:
 *
 *     a qubit;
 *     a logical qubit;
 *     a register;
 *     a register element;
 *     a quantum selection;
 *     a symbolic resource;
 *     a future quantum resource type;
 *
 * is determined by semantic analysis.
 *
 * This is essential for extensibility.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar QuantumControls;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL IMPORTS
 * ============================================================================
 *
 * `Expressions` supplies the universal expression rule.
 *
 * `Names` supplies qualified-name syntax for extensible control attributes.
 *
 * No operation grammar is imported here.
 *
 * This is deliberate.
 *
 * Importing Operations here would create an ownership cycle because
 * Operations consumes control modifiers.
 *
 * Correct dependency:
 *
 *     Controls
 *         ^
 *         |
 *     Operations
 *
 * not:
 *
 *     Controls <--> Operations
 *
 * ============================================================================
 */

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * 1. CONTROL MODIFIER
 * ============================================================================
 *
 * This is the reusable control modifier consumed by the operation grammar.
 *
 * Conceptual source forms:
 *
 *     control(q)
 *
 *     control(q0, q1)
 *
 *     control(q0, q1, q2)
 *
 *     control(register)
 *
 *     control(register[index])
 *
 *     control(register[start .. end])
 *
 * There is no finite control count.
 *
 * ============================================================================
 */

quantumControlModifier
    : CONTROL quantumControlSpecification
    ;


/*
 * ============================================================================
 * 2. CONTROL SPECIFICATION
 * ============================================================================
 *
 * A control specification contains:
 *
 *     - one or more control operands;
 *     - optional control polarity;
 *     - optional control state;
 *     - optional extensible attributes.
 *
 * The grammar preserves all information without deciding its semantics.
 *
 * ============================================================================
 */

quantumControlSpecification
    : quantumControlList
      quantumControlModifierAttributes?
    ;


/*
 * ============================================================================
 * 3. CONTROL LIST
 * ============================================================================
 *
 * One or more controls are accepted.
 *
 * The `+` cardinality is intentionally unbounded by the language grammar.
 *
 * There is no:
 *
 *     MAX_CONTROLS
 *
 * ============================================================================
 */

quantumControlList
    : LPAREN
      quantumControl
      (
          COMMA
          quantumControl
      )*
      COMMA?
      RPAREN
    ;


/*
 * ============================================================================
 * 4. INDIVIDUAL CONTROL
 * ============================================================================
 *
 * A control operand is an ordinary Zamani expression.
 *
 * This is an intentional extensibility boundary.
 *
 * Semantic analysis determines whether the expression is a valid quantum
 * control operand.
 *
 * ============================================================================
 */

quantumControl
    : expression
    ;


/*
 * ============================================================================
 * 5. CONTROL POLARITY
 * ============================================================================
 *
 * Polarity describes the logical activation condition of a control.
 *
 * It does not describe physical implementation.
 *
 * Canonical semantic concepts:
 *
 *     positive
 *     negative
 *
 * The actual meaning is validated downstream.
 *
 * ============================================================================
 */

quantumControlPolarity
    : POSITIVE
    | NEGATIVE
    ;


/*
 * ============================================================================
 * 6. CONTROL STATE
 * ============================================================================
 *
 * Explicit state syntax is intentionally expression-based.
 *
 * This allows future state representations without requiring a new grammar
 * production for every quantum-state model.
 *
 * Conceptual examples:
 *
 *     state[|0⟩]
 *     state[|1⟩]
 *     state[expression]
 *
 * The state expression itself remains owned by the universal expression /
 * quantum literal machinery.
 *
 * ============================================================================
 */

quantumControlState
    : quantumControlStateExpression
    ;


quantumControlStateExpression
    : LBRACK
      expression
      RBRACK
    ;


/*
 * ============================================================================
 * 7. CONTROL MODIFIER ATTRIBUTES
 * ============================================================================
 *
 * This is the extensibility boundary for control metadata that is genuinely
 * source-level semantic intent.
 *
 * Example conceptual forms:
 *
 *     control(q) {
 *         polarity: positive
 *     }
 *
 *     control(q) {
 *         state: |1⟩
 *     }
 *
 * Attribute names remain open identifiers.
 *
 * This avoids hard-coding every future control concept into the lexer.
 *
 * IMPORTANT:
 *
 * The surrounding syntax must not be confused with generic Zamani attributes.
 * This rule is only for control-specific metadata when the quantum control
 * composition requires an attached block.
 *
 * ============================================================================
 */

quantumControlModifierAttributes
    : LBRACE
      quantumControlModifierAttribute*
      RBRACE
    ;


quantumControlModifierAttribute
    : qualifiedName
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. CONTROL POLARITY ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * The semantic layer may interpret:
 *
 *     polarity: positive
 *
 *     polarity: negative
 *
 * The grammar does not need a dedicated keyword for `polarity`.
 *
 * This keeps the lexical vocabulary stable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. CONTROL STATE ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * The semantic layer may interpret:
 *
 *     state: |0⟩
 *
 *     state: |1⟩
 *
 *     state: expression
 *
 * The quantum literal grammar remains the lexical owner of quantum literals.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. CONTROL VALUE EXPRESSION
 * ============================================================================
 *
 * A control can eventually be interpreted according to the semantic type of
 * its operand.
 *
 * The grammar therefore does not define separate syntactic categories for:
 *
 *     qubit
 *     logical qubit
 *     register
 *     register element
 *     symbolic quantum resource
 *
 * All remain expressions here.
 *
 * ============================================================================
 */

quantumControlValue
    : expression
    ;


/*
 * ============================================================================
 * 11. REUSABLE CONTROL LIST
 * ============================================================================
 *
 * Tooling and higher-level quantum grammar composition may use this rule when
 * they need the list without the surrounding `control` keyword.
 *
 * It intentionally has the same unbounded cardinality contract.
 *
 * ============================================================================
 */

quantumControlOperandList
    : quantumControl
      (
          COMMA
          quantumControl
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. CONTROL GROUP
 * ============================================================================
 *
 * A control group is the reusable parenthesized representation of controls.
 *
 * It is kept separate from the `CONTROL` keyword so that other grammar
 * compositions can consume the same structure without redefining it.
 *
 * ============================================================================
 */

quantumControlGroup
    : LPAREN
      quantumControlOperandList
      RPAREN
    ;


/*
 * ============================================================================
 * 13. CONTROL MODIFIER BODY
 * ============================================================================
 *
 * This is the canonical structural representation consumed by operation
 * composition.
 *
 * It is intentionally equivalent to:
 *
 *     CONTROL + control group + optional metadata
 *
 * No operation follows this rule.
 *
 * The caller owns the operation.
 *
 * ============================================================================
 */

quantumControlModifierBody
    : quantumControlGroup
      quantumControlModifierAttributes?
    ;


/*
 * ============================================================================
 * 14. COMPOSABLE CONTROL MODIFIER
 * ============================================================================
 *
 * Alias retained as an explicit composition point.
 *
 * This allows future operation-modifier grammars to consume a control without
 * knowing the implementation details of its operand list.
 *
 * ============================================================================
 */

quantumComposableControlModifier
    : quantumControlModifier
    ;


/*
 * ============================================================================
 * 15. CONTROL MODIFIER SEQUENCE
 * ============================================================================
 *
 * Multiple control modifiers can be represented without an artificial
 * nesting/count ceiling.
 *
 * Example conceptual structure:
 *
 *     control(c0)
 *     control(c1)
 *
 * Whether repeated control modifiers are semantically equivalent to one
 * aggregate control is NOT decided here.
 *
 * ============================================================================
 */

quantumControlModifierSequence
    : quantumControlModifier+
    ;


/*
 * ============================================================================
 * 16. CONTROL POLARITY EXPRESSION
 * ============================================================================
 *
 * This rule is deliberately separate from the operation modifier itself.
 *
 * It provides a stable semantic parse-tree node for tooling.
 *
 * ============================================================================
 */

quantumControlPolarityExpression
    : quantumControlPolarity
    ;


/*
 * ============================================================================
 * 17. CONTROL STATE EXPRESSION
 * ============================================================================
 *
 * The state expression is kept independent from the state implementation.
 *
 * ============================================================================
 */

quantumControlStateValue
    : quantumControlStateExpression
    ;


/*
 * ============================================================================
 * 18. NO OPERATION INVOCATION
 * ============================================================================
 *
 * This file intentionally does NOT contain a rule such as:
 *
 *     controlledOperation
 *         : CONTROL ... operationInvocation
 *         ;
 *
 * Operation invocation belongs to:
 *
 *     grammar/quantum/operations.g4
 *
 * This prevents controlled-operations.g4 / controls.g4 / operations.g4 from
 * becoming three competing owners of the same operation syntax.
 *
 * The operation grammar should consume:
 *
 *     quantumControlModifier
 *
 * as one of its modifiers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. NO QUANTUM REFERENCE REDEFINITION
 * ============================================================================
 *
 * This file intentionally does NOT define:
 *
 *     quantumReference
 *     quantumNameReference
 *     quantumIndexedReference
 *     quantumSliceReference
 *
 * Those concepts already belong to the quantum reference/expression/type
 * architecture.
 *
 * A control operand is simply:
 *
 *     expression
 *
 * Semantic analysis determines whether it is a valid quantum control.
 *
 * This removes the duplicate-reference problem present in the older
 * controlled-operations grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. NO CLASSICAL CONTROL
 * ============================================================================
 *
 * Classical conditions are deliberately NOT defined here.
 *
 * For example:
 *
 *     if (condition) operation(...)
 *
 * belongs to the dynamic/hybrid control-flow architecture.
 *
 * This file owns quantum control modifiers only.
 *
 * Relevant downstream owners include:
 *
 *     grammar/quantum/dynamic-circuits.g4
 *     grammar/quantum/mid-circuit-control.g4
 *     grammar/quantum/quantum-classical.g4
 *     grammar/hybrid/classical-quantum.g4
 *
 * Those files must consume/reuse the common expression/control foundations
 * rather than redefining quantum control operands.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. NO ADJOINT / INVERSE IMPLEMENTATION
 * ============================================================================
 *
 * `adjoint` and `inverse` are separate operation modifiers.
 *
 * They must not be implemented by this file.
 *
 * If the operation-modifier architecture allows:
 *
 *     control
 *     adjoint
 *     inverse
 *
 * composition, the operation grammar owns the modifier ordering/composition.
 *
 * This file contributes only:
 *
 *     quantumControlModifier
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis must determine:
 *
 *     1. whether each control expression resolves;
 *
 *     2. whether each control expression denotes a legal quantum resource;
 *
 *     3. whether the control resource has an appropriate quantum type;
 *
 *     4. whether control resources are distinct where required;
 *
 *     5. whether controls overlap operation targets illegally;
 *
 *     6. whether control polarity is supported;
 *
 *     7. whether control state is type-correct;
 *
 *     8. whether the operation accepts control;
 *
 *     9. whether the operation's semantic arity permits the controls;
 *
 *    10. whether control modifiers compose legally with other modifiers;
 *
 *    11. whether required capabilities exist;
 *
 *    12. whether required resources exist;
 *
 *    13. whether a target can realize the semantic operation;
 *
 *    14. whether decomposition is necessary.
 *
 * None of these checks belong to the parser.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. RESOURCE CONTRACT
 * ============================================================================
 *
 * A control may imply resource requirements.
 *
 * For example:
 *
 *     control(q0, q1)
 *
 * may produce a semantic requirement that the referenced quantum resources
 * remain available for the operation.
 *
 * The grammar MUST NOT determine whether the target has enough resources.
 *
 * Resource analysis belongs to:
 *
 *     grammar/resources/
 *
 * and the compiler semantic/resource layer.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. CAPABILITY CONTRACT
 * ============================================================================
 *
 * A controlled operation may require capabilities such as:
 *
 *     capability("quantum.control")
 *
 *     capability("quantum.multi_control")
 *
 *     capability("quantum.control.polarity")
 *
 * These are semantic capability identifiers, not grammar-level hardware
 * assumptions.
 *
 * The grammar must never convert them into fixed machine limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct dependency on Rust quantum IR.
 *
 * Conceptual lowering:
 *
 *     quantumControlModifier
 *             |
 *             v
 *     frontend AST control modifier
 *             |
 *             v
 *     semantic control intent
 *             |
 *             v
 *     quantum::ir
 *
 * The IR representation may preserve:
 *
 *     controls
 *     control polarity
 *     control-state intent
 *     source provenance
 *     semantic attributes
 *
 * according to the canonical quantum::ir contract.
 *
 * This grammar does not prescribe the Rust structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve enough structure to reconstruct:
 *
 *     control operand ordering;
 *     control count;
 *     control grouping;
 *     control metadata;
 *     source spans;
 *     source ordering.
 *
 * A representative semantic shape is:
 *
 *     ControlModifier {
 *         controls,
 *         attributes,
 *         source_span
 *     }
 *
 * This is a conceptual contract only.
 *
 * The authoritative Rust AST remains:
 *
 *     src/frontend/ast/
 *
 * This grammar must not introduce a competing Rust AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The parser/frontend must retain source spans for:
 *
 *     quantumControlModifier
 *     quantumControlSpecification
 *     quantumControlGroup
 *     quantumControl
 *     quantumControlModifierAttributes
 *     quantumControlModifierAttribute
 *
 * At minimum, diagnostics should be able to identify:
 *
 *     the `control` keyword;
 *     the complete control group;
 *     each control operand;
 *     each attached control attribute.
 *
 * This supports:
 *
 *     diagnostics
 *     IDE/LSP
 *     formatting
 *     refactoring
 *     source maps
 *     provenance
 *     compiler explanations
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level structural failures include:
 *
 *     control
 *
 *     control()
 *
 *     control(
 *
 *     control(q0,
 *
 *     control(q0 q1)
 *
 *     control(q0, )
 *
 *     control(q0) {
 *
 *     control(q0) {
 *         polarity
 *     }
 *
 * Semantic failures include:
 *
 *     unresolved control expression;
 *
 *     non-quantum control operand;
 *
 *     illegal control/target overlap;
 *
 *     unsupported control polarity;
 *
 *     unsupported control state;
 *
 *     unsupported operation control;
 *
 *     unavailable capability;
 *
 *     insufficient resources.
 *
 * The last categories MUST NOT be represented as parser errors.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no environment access;
 *     no hardware discovery;
 *     no randomness;
 *     no runtime invocation.
 *
 * Parsing therefore depends only on:
 *
 *     source tokens;
 *     canonical grammar composition;
 *     parser configuration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     I/O
 *     filesystem operations
 *     network operations
 *     hardware access
 *     process execution
 *     credential access
 *     backend invocation
 *
 * It contains no embedded target-language actions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. SCALABILITY
 * ============================================================================
 *
 * The grammar deliberately permits arbitrary source cardinality subject to
 * ANTLR/parser implementation resources.
 *
 * The language itself establishes no finite control ceiling.
 *
 * Conceptually valid structures include:
 *
 *     control(q0)
 *
 *     control(q0, q1)
 *
 *     control(q0, q1, q2, ... qN)
 *
 *     control(register)
 *
 *     control(register[index])
 *
 *     control(register[start .. end])
 *
 * The exact physical number of resources is a semantic/runtime concern.
 *
 * There is no:
 *
 *     smallControl
 *     mediumControl
 *     largeControl
 *
 * grammar split.
 *
 * There is no:
 *
 *     control2
 *     control4
 *     control8
 *     control32
 *
 * syntax.
 *
 * Cardinality remains data/structure rather than a machine-size class.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. POCO-REAF CONFORMANCE
 * ============================================================================
 *
 * The same source control syntax must remain usable when the target changes
 * from:
 *
 *     simulator
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     distributed accelerator
 *     future computational substrate
 *
 * provided the semantic requirements can be satisfied.
 *
 * The grammar cannot encode target-specific realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file as universal language constraints:
 *
 *     MAX_CONTROLS
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_REGISTER_WIDTH
 *     MAX_DEVICES
 *     MAX_CONTROL_DEPTH
 *     MAX_CONTROL_REGISTER_SIZE
 *
 * Also forbidden as universal control semantics:
 *
 *     qpu0
 *     qpu1
 *     physical_qubit_0
 *     physical_qubit_1
 *     device0
 *     device1
 *
 * Vendor-specific hardware identifiers belong downstream in explicitly
 * target-specific language/dialect layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. CONSTANTS ARE NOT THE PROBLEM
 * ============================================================================
 *
 * Program data remains legal.
 *
 * For example:
 *
 *     let n = 1000;
 *
 *     control(register[0 .. n]);
 *
 * is not a hard-coded language limitation.
 *
 * The distinction is:
 *
 *     source-program value
 *         !=
 *     grammar implementation ceiling
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. COMPATIBILITY
 * ============================================================================
 *
 * This new canonical file supersedes overlapping controlled-operation syntax
 * in:
 *
 *     grammar/quantum/controlled-operations.g4
 *
 * but does NOT require renaming that existing file immediately.
 *
 * Migration policy:
 *
 *     controlled-operations.g4
 *             |
 *             v
 *     compatibility/deprecated compatibility layer
 *             |
 *             v
 *     QuantumControls
 *
 * No new canonical control syntax should be added to the old duplicate owner.
 *
 * Existing source programs should continue to be accepted where their syntax
 * is already part of the supported Zamani language.
 *
 * Breaking syntax changes require the normal Zamani language-version and
 * compatibility process.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. INTEGRATION WITH operations.g4
 * ============================================================================
 *
 * REQUIRED CANONICAL INTEGRATION
 * ------------------------------
 *
 * `grammar/quantum/operations.g4` remains the owner of:
 *
 *     quantumOperationInvocation
 *     quantumOperationDesignator
 *     quantumOperationParameterClause
 *     quantumOperationTargetClause
 *     quantumOperationTargetList
 *
 * Operations should consume:
 *
 *     quantumControlModifier
 *
 * as a modifier rather than redefining control syntax.
 *
 * Conceptually:
 *
 *     quantumOperationInvocation
 *         : operationCallee
 *           parameterClause?
 *           targetClause
 *           quantumOperationModifier*
 *         ;
 *
 * The exact ordering must follow the already-established operation grammar
 * contract. This file deliberately does not dictate a competing ordering.
 *
 * The important invariant is:
 *
 *     Operations owns operation invocation.
 *
 *     Controls owns control modifier syntax.
 *
 * There is exactly one owner for each production.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. INTEGRATION WITH controlled-operations.g4
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/quantum/controlled-operations.g4
 *
 * currently defines:
 *
 *     controlledOperation
 *     controlledOperationPrefix
 *     controlPrefix
 *     controlSpec
 *     controlTarget
 *     controlTargetGroup
 *     controlTargetList
 *     quantumReference
 *     quantumNameReference
 *     quantumIndexedReference
 *     quantumSliceReference
 *
 * Those overlapping definitions must not remain competing canonical owners.
 *
 * Migration target:
 *
 *     controlled-operations.g4
 *             |
 *             +--> compatibility documentation
 *             |
 *             +--> migration aliases where genuinely necessary
 *             |
 *             v
 *     controls.g4
 *
 * In particular, the old file must stop owning a second:
 *
 *     quantumReference
 *
 * grammar.
 *
 * This file deliberately does not recreate those productions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. INTEGRATION WITH parameterized-operations.g4
 * ============================================================================
 *
 * Parameter syntax remains owned by:
 *
 *     grammar/quantum/parameterized-operations.g4
 *
 * A controlled parameterized operation therefore composes conceptually as:
 *
 *     control
 *       +
 *     operation
 *       +
 *     parameter bindings
 *       +
 *     targets
 *
 * No parameter grammar is duplicated here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. INTEGRATION WITH expressions/quantum.g4
 * ============================================================================
 *
 * Quantum expressions remain owned by:
 *
 *     grammar/expressions/quantum.g4
 *
 * Control operands are accepted structurally as universal expressions.
 *
 * The semantic layer determines whether an expression is a legal quantum
 * control.
 *
 * This allows future quantum resource forms without rewriting this file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. INTEGRATION WITH quantum types
 * ============================================================================
 *
 * This file does not define:
 *
 *     Qubit
 *     Qubit[N]
 *     LogicalQubit
 *     QuantumRegister
 *     QuantumState
 *
 * The universal type/quantum-type architecture remains responsible for those
 * constructs.
 *
 * Semantic analysis maps control expressions to their actual types.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. INTEGRATION WITH DYNAMIC CIRCUITS
 * ============================================================================
 *
 * `dynamic-circuits.g4` owns runtime-dependent control flow.
 *
 * This file does not define:
 *
 *     if
 *     while
 *     match
 *     branch
 *     runtime condition
 *
 * A quantum control modifier and a dynamic classical condition are distinct
 * semantic concepts and must remain distinct.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. INTEGRATION WITH HYBRID COMPUTATION
 * ============================================================================
 *
 * Hybrid classical/quantum control belongs to:
 *
 *     grammar/quantum/quantum-classical.g4
 *     grammar/hybrid/classical-quantum.g4
 *     dynamic-circuits.g4
 *
 * This file supplies the quantum-control modifier foundation only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. INTEGRATION WITH RESOURCES AND CAPABILITIES
 * ============================================================================
 *
 * Resource and capability requirements remain outside this grammar.
 *
 * A semantic control may produce requirements such as:
 *
 *     capability("quantum.control")
 *
 *     capability("quantum.multi_control")
 *
 *     capability("quantum.control.polarity")
 *
 *     capability("quantum.control.state")
 *
 * These are semantic facts, not parser decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. INTEGRATION WITH HARDWARE
 * ============================================================================
 *
 * This grammar never selects:
 *
 *     physical qubit
 *     physical control line
 *     QPU
 *     device
 *     topology
 *     coupling map
 *     native instruction
 *
 * Hardware realization belongs to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     compiler target lowering
 *     routing
 *     scheduling
 *     HAL
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. INTEGRATION WITH QEC / ZQN / RESILIENCE
 * ============================================================================
 *
 * Control syntax has no knowledge of:
 *
 *     error-correcting codes
 *     code distance
 *     syndrome extraction
 *     decoder choice
 *     noise model
 *     leakage
 *     fault model
 *     reliability state
 *
 * Those remain downstream.
 *
 * The semantic control operation may subsequently be transformed by:
 *
 *     QEC
 *     ZQN
 *     resilience
 *
 * without changing the source grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. NO UNSAFE RUST
 * ============================================================================
 *
 * This file contains no embedded Rust.
 *
 * Downstream Rust implementation requirements remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * Repository-level Rust enforcement should use:
 *
 *     #![forbid(unsafe_code)]
 *
 * in applicable Rust crates/modules.
 *
 * This grammar introduces no requirement for unsafe Rust.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. TEST CONTRACT
 * ============================================================================
 *
 * Required positive cases:
 *
 *     control(q0)
 *
 *     control(q0, q1)
 *
 *     control(q0, q1, q2)
 *
 *     control(register)
 *
 *     control(register[index])
 *
 *     control(register[start .. end])
 *
 *     control(symbolic_selection)
 *
 *     control(q0) { }
 *
 *     control(q0) {
 *         polarity: positive;
 *     }
 *
 *     control(q0) {
 *         polarity: negative;
 *     }
 *
 *     control(q0) {
 *         state: |1⟩;
 *     }
 *
 * Required scalable cases:
 *
 *     one control
 *     multiple controls
 *     symbolic control selection
 *     arbitrarily large generated control lists
 *
 * The exact benchmark sizes are implementation tests, not grammar limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Structurally invalid:
 *
 *     control
 *
 *     control()
 *
 *     control(
 *
 *     control(q0
 *
 *     control(q0 q1)
 *
 *     control(q0,)
 *
 *     control(,q0)
 *
 *     control(q0) {
 *
 *     control(q0) {
 *         polarity
 *     }
 *
 * Semantic invalidity must be tested separately:
 *
 *     control(non_quantum_value)
 *
 *     control(unresolved_name)
 *
 *     control(target_that_conflicts_with_operation)
 *
 *     control(unsupported_state)
 *
 *     control(unsupported_polarity)
 *
 * These must reach semantic validation rather than being rejected merely
 * because the parser does not know their types.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one control
 *     two controls
 *     many controls
 *     symbolic controls
 *     indexed controls
 *     sliced controls
 *     nested expressions
 *     qualified names
 *     empty attribute block
 *     multiple attributes
 *     trailing commas
 *
 * Also test interaction with:
 *
 *     parameterized operations
 *     adjoint
 *     inverse
 *     dynamic circuits
 *     hybrid classical/quantum programs
 *     measurement results
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical token streams must produce identical parse structures under the
 * same grammar/version configuration.
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     available QPUs
 *     compiler machine
 *     filesystem
 *     network
 *     environment variables
 *     random values
 *     wall-clock time
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is considered complete when:
 *
 * [x] It has a single clear ownership boundary.
 *
 * [x] It owns quantum control modifier syntax.
 *
 * [x] It does not own operation invocation.
 *
 * [x] It does not redefine quantum references.
 *
 * [x] It does not redefine general expressions.
 *
 * [x] It does not redefine quantum types.
 *
 * [x] It does not define classical dynamic control flow.
 *
 * [x] It does not define hardware realization.
 *
 * [x] It does not define QEC.
 *
 * [x] It does not define ZQN.
 *
 * [x] It does not define routing.
 *
 * [x] It does not define scheduling.
 *
 * [x] It does not define quantum::ir.
 *
 * [x] It contains no hardware-size constants.
 *
 * [x] It contains no embedded Rust actions.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It supports an unbounded source-level control list.
 *
 * [x] It preserves operand ordering.
 *
 * [x] It preserves source-level control metadata.
 *
 * [x] It is compatible with POCO-REAF.
 *
 * [x] Its downstream AST responsibility is explicit.
 *
 * [x] Its semantic responsibility is explicit.
 *
 * [x] Its IR responsibility is explicit.
 *
 * [x] Its compiler/runtime boundary is explicit.
 *
 * [x] Its test contract is explicit.
 *
 * [x] Its compatibility migration is explicit.
 *
 * [ ] `operations.g4` consumes `quantumControlModifier`.
 *
 * [ ] `controlled-operations.g4` is reduced to a compatibility/migration
 *     layer and no longer competes for canonical ownership.
 *
 * [ ] The composed parser imports this grammar exactly once.
 *
 * [ ] Generated ANTLR parser validation passes.
 *
 * [ ] Rust frontend AST conformance passes.
 *
 * [ ] Semantic control validation passes.
 *
 * [ ] quantum::ir lowering passes.
 *
 * [ ] Positive/negative/boundary/scalability tests pass.
 *
 * ============================================================================
 */