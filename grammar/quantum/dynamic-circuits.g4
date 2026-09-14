/*
 * ============================================================================
 * Zamani Quantum Dynamic-Circuit Grammar
 * File:
 *     grammar/quantum/dynamic-circuits.g4
 *
 * PURPOSE
 * -------
 * Defines source-level syntax for dynamic quantum programs whose later
 * computation can depend on values produced during execution.
 *
 * Dynamic behavior may include:
 *
 *   - mid-circuit measurement
 *   - classical conditions controlling quantum operations
 *   - runtime branches
 *   - runtime loops
 *   - condition-dependent reset
 *   - condition-dependent quantum operations
 *   - classical computation between quantum operations
 *   - runtime-generated control values
 *
 *
 * ARCHITECTURAL OWNER
 * -------------------
 * This file owns ONLY the syntax specific to dynamic quantum control flow.
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 * It does not own:
 *
 *   - Qubit identifiers
 *   - Logical qubits
 *   - Physical qubits
 *   - Quantum registers
 *   - Classical registers
 *   - Measurement semantics
 *   - Observable semantics
 *   - Gate semantics
 *   - Quantum IR
 *   - Classical IR
 *   - QEC algorithms
 *   - ZQN/noise models
 *   - Routing
 *   - Scheduling
 *   - Hardware discovery
 *   - Hardware topology
 *   - Resource allocation
 *   - Runtime implementation
 *   - Backend-specific control protocols
 *
 *
 * CANONICAL SEMANTIC BOUNDARY
 * ----------------------------
 * Parsed dynamic-circuit syntax is lowered through semantic analysis into
 * the repository's canonical intermediate representations.
 *
 * Quantum semantics ultimately cross the established:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar must NEVER introduce a second quantum operation model.
 *
 *
 * DEPENDENCY DIRECTION
 * --------------------
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +----------------------+
 *       |                      |
 *       v                      v
 *   classical IR          quantum::ir
 *       |                      |
 *       +----------+-----------+
 *                  |
 *                  v
 *          optimization
 *                  |
 *                  v
 *              routing
 *                  |
 *                  v
 *             scheduling
 *                  |
 *                  v
 *          ZQN / hardware
 *                  |
 *                  v
 *               runtime
 *
 *
 * SCALABILITY
 * -----------
 * No machine capacity is encoded here.
 *
 * There is intentionally no:
 *
 *     MAX_QUBITS
 *     MAX_CLASSICAL_BITS
 *     MAX_BRANCHES
 *     MAX_LOOP_ITERATIONS
 *     MAX_MEASUREMENTS
 *     MAX_NESTING
 *     MAX_DEVICES
 *     MAX_SHOTS
 *
 * Program cardinality is limited only by the applicable language,
 * compiler, resource, runtime, and backend constraints.
 *
 * Those constraints MUST NOT become grammar constants.
 *
 *
 * POCO-REAF
 * ---------
 * Dynamic-circuit syntax describes portable computation.
 *
 * It does not prescribe:
 *
 *     which machine
 *     which device
 *     which topology
 *     which timing grid
 *     which instruction set
 *     which control processor
 *     which reset implementation
 *     which measurement implementation
 *
 * Those decisions belong downstream.
 *
 *
 * RUST
 * ----
 * This grammar contains no embedded Rust actions or predicates.
 *
 * Generated parser/frontend code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must use safe Rust only.
 *
 * No unsafe Rust is introduced or required by this grammar.
 *
 *
 * OWNERSHIP RULE
 * --------------
 * The rules defined here are the sole grammar owners of dynamic-circuit
 * control-flow syntax.
 *
 * Existing owners remain responsible for the constructs they already own:
 *
 *     measurement.g4
 *         -> measurement syntax
 *
 *     reset.g4
 *         -> reset syntax
 *
 *     gates.g4
 *         -> gate syntax
 *
 *     operations.g4
 *         -> operation aggregation
 *
 *     quantum-classical.g4
 *         -> general quantum/classical interoperability
 *
 *     mid-circuit-control.g4
 *         -> if that file is retained as a narrower compatibility/dialect
 *            layer, it must reference these canonical rules rather than
 *            redefine them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * Dynamic quantum statement
 * ============================================================================
 *
 * A dynamic quantum statement is a runtime-dependent statement whose
 * execution or control flow can depend on values available during execution.
 *
 * The actual quantum operation is delegated to the canonical quantum
 * operation grammar.
 *
 * The actual classical statement is delegated to the canonical statement
 * grammar.
 *
 * ============================================================================
 */

quantumDynamicCircuitStatement
    : quantumDynamicConditionalStatement
    | quantumDynamicLoopStatement
    | quantumDynamicBlock
    ;


/*
 * ============================================================================
 * Dynamic block
 * ============================================================================
 *
 * A dynamic block groups runtime-dependent computation.
 *
 * The body is intentionally represented through the canonical block rule
 * rather than defining another block grammar here.
 *
 * ============================================================================
 */

quantumDynamicBlock
    : '{' quantumDynamicStatementList? '}'
    ;


/*
 * ============================================================================
 * Dynamic statement list
 * ============================================================================
 *
 * There is no fixed number of statements.
 * ============================================================================
 */

quantumDynamicStatementList
    : quantumDynamicStatement
    | quantumDynamicStatementList quantumDynamicStatement
    ;


/*
 * ============================================================================
 * Dynamic conditional
 * ============================================================================
 *
 * Canonical form:
 *
 *     if <condition> {
 *         ...
 *     }
 *
 * Optional else branches are supported.
 *
 * The condition is evaluated according to the language's canonical expression
 * and semantic model.
 *
 * It may depend on:
 *
 *     - classical values
 *     - measurement results
 *     - computed runtime values
 *     - values produced by previous dynamic operations
 *
 * It must NOT directly encode a physical backend property.
 *
 * ============================================================================
 */

quantumDynamicConditionalStatement
    : 'if' expression quantumDynamicBlock
      ('else' quantumDynamicElseBranch)?
    ;


/*
 * ============================================================================
 * Else branch
 * ============================================================================
 *
 * A nested conditional is allowed without imposing a fixed nesting depth.
 * ============================================================================
 */

quantumDynamicElseBranch
    : quantumDynamicBlock
    | quantumDynamicConditionalStatement
    ;


/*
 * ============================================================================
 * Dynamic loop
 * ============================================================================
 *
 * Runtime-controlled repetition.
 *
 * The loop condition belongs to the canonical expression language.
 *
 * The grammar intentionally does not specify a fixed iteration count.
 *
 * Examples of possible semantic forms include:
 *
 *     while condition {
 *         ...
 *     }
 *
 *     while measurement_result == 0 {
 *         ...
 *     }
 *
 *     while not done {
 *         ...
 *     }
 *
 * Whether a loop is statically bounded, dynamically bounded, or potentially
 * unbounded is a semantic/compiler/runtime property.
 *
 * ============================================================================
 */

quantumDynamicLoopStatement
    : 'while' expression quantumDynamicBlock
    ;


/*
 * ============================================================================
 * Dynamic condition
 * ============================================================================
 *
 * This rule provides a stable semantic boundary for consumers that need to
 * identify a runtime condition without redefining expression syntax.
 *
 * ============================================================================
 */

quantumDynamicCondition
    : expression
    ;


/*
 * ============================================================================
 * Runtime-controlled quantum operation
 * ============================================================================
 *
 * This rule provides the dynamic-control wrapper around an existing quantum
 * operation.
 *
 * The actual operation remains owned by the canonical quantum operation
 * grammar.
 *
 * This prevents dynamic-circuits.g4 from defining duplicate gate/reset/
 * measurement syntax.
 *
 * ============================================================================
 */

quantumConditionedQuantumOperation
    : 'if' quantumDynamicCondition quantumOperation
    ;


/*
 * ============================================================================
 * Runtime-controlled statement
 * ============================================================================
 *
 * Allows a runtime condition to control a canonical language statement.
 *
 * This is intentionally expressed through the existing statement grammar.
 *
 * ============================================================================
 */

quantumConditionedStatement
    : 'if' quantumDynamicCondition statement
    ;


/*
 * ============================================================================
 * Dynamic measurement-dependent operation
 * ============================================================================
 *
 * This rule makes the semantic relationship explicit:
 *
 *     runtime condition
 *          |
 *          v
 *     quantum operation
 *
 * The measurement itself remains owned by measurement.g4.
 *
 * ============================================================================
 */

quantumMeasurementConditionedOperation
    : 'if' quantumDynamicCondition quantumOperation
    ;


/*
 * ============================================================================
 * Dynamic circuit declaration
 * ============================================================================
 *
 * Declares a named dynamic circuit body without embedding hardware-specific
 * limits.
 *
 * Parameters, if present, are handled through the canonical function/
 * parameter grammar.
 *
 * ============================================================================
 */

quantumDynamicCircuitDeclaration
    : 'dynamic' 'circuit' identifier
      quantumDynamicCircuitParameterClause?
      quantumDynamicBlock
    ;


/*
 * ============================================================================
 * Dynamic circuit parameters
 * ============================================================================
 *
 * Parameters are intentionally expressed through existing identifiers and
 * type/expression machinery.
 *
 * ============================================================================
 */

quantumDynamicCircuitParameterClause
    : '(' quantumDynamicCircuitParameterList? ')'
    ;


quantumDynamicCircuitParameterList
    : quantumDynamicCircuitParameter
    | quantumDynamicCircuitParameterList
      ','
      quantumDynamicCircuitParameter
    ;


quantumDynamicCircuitParameter
    : identifier
    ;


/*
 * ============================================================================
 * Dynamic circuit invocation
 * ============================================================================
 *
 * Invocation delegates argument syntax to the canonical expression/call
 * system where the aggregate grammar provides it.
 *
 * ============================================================================
 */

quantumDynamicCircuitInvocation
    : identifier '(' argumentList? ')'
    ;