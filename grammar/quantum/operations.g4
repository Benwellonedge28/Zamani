/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/operations.g4
 *
 * Grammar:
 *     QuantumOperations
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM OPERATION INVOCATION GRAMMAR
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
 * This file is the canonical parser owner for SOURCE-LEVEL QUANTUM OPERATION
 * APPLICATION SYNTAX.
 *
 * It defines the structural syntax for:
 *
 *     apply operation(targets);
 *     apply operation(parameters)(targets);
 *     apply namespace::operation(targets);
 *     apply namespace::operation(parameters)(targets);
 *     apply control(operation)(controls, targets);
 *     apply adjoint(operation)(targets);
 *     apply inverse(operation)(targets);
 *     apply control(adjoint(operation))(controls, targets);
 *
 * Operation names remain OPEN-ENDED source names.
 *
 * The grammar deliberately does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     vendor operations
 *     simulator operations
 *     future operations
 *
 * Those names are ordinary source-level names whose meaning is resolved by
 * semantic analysis.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantumOperationStatement
 *     - quantumOperationApplication
 *     - quantumOperationInvocation
 *     - quantumOperationDesignator
 *     - quantumOperationParameterClause
 *     - quantumOperationTargetClause
 *     - quantumOperationTargetList
 *     - quantumOperationModifier
 *     - quantumOperationModifierOperand
 *     - quantumOperationReference
 *     - quantumOperationTargets
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - general types;
 *     - qubit declarations;
 *     - register declarations;
 *     - logical-qubit declarations;
 *     - physical-qubit declarations;
 *     - quantum-state expressions;
 *     - measurements;
 *     - reset;
 *     - observables;
 *     - dynamic-circuit control;
 *     - quantum/classical control-flow;
 *     - gate definitions;
 *     - gate matrices;
 *     - operation implementation;
 *     - resource allocation;
 *     - capability discovery;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - calibration;
 *     - hardware topology;
 *     - physical qubit assignment;
 *     - HAL;
 *     - runtime execution;
 *     - canonical quantum::ir.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          +--> QuantumOperations
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> quantum semantic validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime
 *
 * This grammar MUST NOT bypass the AST/semantic/IR boundaries.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Zamani operations describe PORTABLE COMPUTATIONAL INTENT.
 *
 * They do not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     physical register
 *     memory bank
 *     device ID
 *     topology
 *     coupling map
 *     pulse
 *     calibration
 *     scheduler
 *     router
 *     vendor instruction
 *
 * This grammar therefore contains NO language-level maximum for:
 *
 *     operations
 *     parameters
 *     targets
 *     controls
 *     nesting
 *     circuit depth
 *     qubits
 *     registers
 *     devices
 *     nodes
 *     memory
 *     threads
 *     accelerators
 *
 * There are intentionally no:
 *
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_PARAMETERS
 *     MAX_OPERATIONS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *
 * or equivalent constants.
 *
 * Repetition and cardinality are represented with ordinary ANTLR repetition
 * constructs such as `*` and `+`.
 *
 * Actual resource availability is resolved after parsing.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * whose lexical composition is:
 *
 *     grammar/lexer/tokens.g4
 *
 * Existing canonical tokens consumed here include:
 *
 *     APPLY
 *     CONTROL
 *     ADJOINT
 *     INVERSE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * Names are supplied by the canonical Names grammar.
 *
 * Expressions are supplied by the canonical Expressions grammar.
 *
 * Generic type syntax is supplied through the canonical expression/type
 * integration rather than by defining another generic grammar here.
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * IMPORTANT DESIGN DECISION
 * ============================================================================
 *
 * Operation names are NOT lexer keywords.
 *
 * Therefore all of the following can remain ordinary source-level names:
 *
 *     H
 *     X
 *     CNOT
 *     SWAP
 *     RX
 *     custom_gate
 *     vendor::operation
 *     library::operation
 *     future::operation
 *
 * The semantic layer decides whether a name denotes:
 *
 *     a primitive operation;
 *     a library operation;
 *     a user-defined operation;
 *     a dialect operation;
 *     a capability-backed operation;
 *     an intrinsic;
 *     an imported operation;
 *     or an unresolved name.
 *
 * ============================================================================
 * PARAMETER / TARGET SEPARATION
 * ============================================================================
 *
 * The canonical invocation forms are:
 *
 *     apply H(q);
 *
 *     apply RX(theta)(q);
 *
 *     apply U(theta, phi, lambda)(q0, q1);
 *
 * The first parenthesized clause after the operation designator is optional
 * operation-parameter syntax.
 *
 * The final parenthesized clause is the operation-target syntax.
 *
 * This permits the parser to distinguish:
 *
 *     apply H(q);
 *
 * from:
 *
 *     apply RX(theta)(q);
 *
 * without knowing what H, RX, theta, or q mean.
 *
 * ============================================================================
 * MODIFIERS
 * ============================================================================
 *
 * Modifiers are structural operation transformations.
 *
 * Supported core forms:
 *
 *     control(operation)
 *     adjoint(operation)
 *     inverse(operation)
 *
 * Modifiers are recursively composable:
 *
 *     control(adjoint(U))
 *
 *     inverse(control(U))
 *
 *     control(inverse(adjoint(U)))
 *
 * There is no grammar-imposed modifier nesting limit.
 *
 * Semantic analysis determines whether a particular composition is valid.
 *
 * ============================================================================
 * CONTROL OPERANDS
 * ============================================================================
 *
 * Controls and operation targets are represented as ordinary expressions.
 *
 * Example:
 *
 *     apply control(X)(control, target);
 *
 *     apply control(X)(c0, c1, target);
 *
 *     apply control(operation)(control_register, target_register);
 *
 * The grammar deliberately does not decide how many operands are controls.
 *
 * The semantic operation signature determines:
 *
 *     control operands;
 *     target operands;
 *     operand roles;
 *     operand types;
 *     arity;
 *     dimensional compatibility.
 *
 * ============================================================================
 * GENERIC OPERATION REFERENCES
 * ============================================================================
 *
 * Generic operation references use the existing universal generic invocation
 * syntax supplied by the expression grammar.
 *
 * For example:
 *
 *     operation::<T>(q)
 *
 * The operation grammar does not create a second generic-argument grammar.
 *
 * ============================================================================
 * SEMANTIC RESPONSIBILITY
 * ============================================================================
 *
 * Parsing establishes only structural validity.
 *
 * Semantic analysis must determine:
 *
 *     whether the operation exists;
 *     whether the operation is callable;
 *     whether its parameters are valid;
 *     whether its targets have compatible quantum types;
 *     whether control operands are valid;
 *     whether adjoint is defined;
 *     whether inverse is defined;
 *     whether the operation has the required capabilities;
 *     whether the required resources exist;
 *     whether the operation is supported by a target;
 *     whether decomposition is necessary.
 *
 * None of these decisions belong in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * A successfully validated operation maps conceptually to:
 *
 *     generic operation intent
 *             |
 *             v
 *     semantic quantum operation
 *             |
 *             v
 *     quantum::ir
 *
 * The grammar MUST NOT create:
 *
 *     QuantumGate
 *     PhysicalGate
 *     VendorGate
 *     HardwareOperation
 *
 * as a closed grammar enumeration.
 *
 * The canonical IR remains the existing:
 *
 *     quantum::ir
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough source structure for the frontend AST to
 * represent:
 *
 *     operation name;
 *     namespace/path;
 *     generic arguments;
 *     operation parameters;
 *     operation targets;
 *     modifier nesting;
 *     source spans;
 *     argument ordering;
 *     target ordering.
 *
 * A representative semantic shape is:
 *
 *     Operation {
 *         designator,
 *         generic_arguments,
 *         parameters,
 *         targets,
 *         modifiers,
 *         source_span
 *     }
 *
 * The exact Rust AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must not introduce a competing AST type.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * The grammar should reject structurally incomplete forms such as:
 *
 *     apply;
 *     apply H;
 *     apply H(;
 *     apply H);
 *     apply control;
 *     apply control(X;
 *     apply adjoint;
 *
 * Semantic analysis, rather than parsing, should reject constructs such as:
 *
 *     apply unknown_operation(q);
 *
 * when `unknown_operation` cannot be resolved.
 *
 * Likewise, resource failures must not be turned into syntax failures.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no runtime calls;
 *     no hardware discovery;
 *     no randomness.
 *
 * Parse results therefore depend only on:
 *
 *     source text;
 *     canonical token vocabulary;
 *     canonical imported grammar definitions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar QuantumOperations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Names:
 *     qualifiedName
 *     identifier
 *
 * Expressions:
 *     expression
 *     argumentList
 *     genericArgumentSuffix
 *
 * Types:
 *     generic type arguments remain part of the universal type/expression
 *     integration and are not duplicated here.
 *
 * Attributes are intentionally NOT imported here.
 *
 * Attribute attachment belongs to the surrounding quantum composition grammar.
 *
 * ============================================================================
 */

import
    Names,
    Expressions,
    Types
;


/*
 * ============================================================================
 * 1. CANONICAL QUANTUM OPERATION STATEMENT
 * ============================================================================
 *
 * Examples:
 *
 *     apply H(q);
 *
 *     apply X(q);
 *
 *     apply CNOT(control, target);
 *
 *     apply RX(theta)(q);
 *
 *     apply library::operation(q);
 *
 *     apply vendor::operation(parameter)(q);
 *
 *     apply control(X)(control, target);
 *
 *     apply adjoint(U)(q);
 *
 *     apply inverse(U)(q);
 */

quantumOperationStatement
    : quantumOperationApplication SEMICOLON
    ;


/*
 * ============================================================================
 * 2. OPERATION APPLICATION
 * ============================================================================
 *
 * The semicolon is intentionally owned by quantumOperationStatement.
 *
 * This rule can therefore be reused by quantum block/domain composition.
 */

quantumOperationApplication
    : APPLY quantumOperationInvocation
    ;


/*
 * ============================================================================
 * 3. OPERATION INVOCATION
 * ============================================================================
 *
 * An operation consists of:
 *
 *     operation designator
 *     optional parameter clause
 *     target clause
 *
 * Examples:
 *
 *     H(q)
 *
 *     RX(theta)(q)
 *
 *     U(theta, phi, lambda)(q0, q1)
 *
 *     control(X)(c, q)
 *
 *     control(RX(theta))(c, q)
 */

quantumOperationInvocation
    : quantumOperationCallee
      quantumOperationParameterClause?
      quantumOperationTargetClause
    ;


/*
 * ============================================================================
 * 4. OPERATION CALLEE
 * ============================================================================
 *
 * A callee is either:
 *
 *     ordinary operation designator
 *
 * or:
 *
 *     recursively modified operation.
 *
 * This prevents a second closed gate grammar from being necessary.
 */

quantumOperationCallee
    : quantumOperationDesignator
    | quantumOperationModifier
    ;


/*
 * ============================================================================
 * 5. OPERATION DESIGNATOR
 * ============================================================================
 *
 * The operation name is a canonical qualified source name.
 *
 * Examples:
 *
 *     H
 *     custom_gate
 *     quantum::operation
 *     library::quantum::operation
 *
 * Generic operation invocation is supported by the existing expression
 * generic-argument suffix.
 *
 * Example:
 *
 *     operation::<T>
 *
 * No vendor or hardware operation list is encoded here.
 */

quantumOperationDesignator
    : qualifiedName
      genericArgumentSuffix?
    ;


/*
 * ============================================================================
 * 6. OPERATION PARAMETER CLAUSE
 * ============================================================================
 *
 * Examples:
 *
 *     RX(theta)
 *
 *     U(theta, phi, lambda)
 *
 *     operation(parameter_expression)
 *
 * Parameters are ordinary Zamani expressions.
 *
 * Parameter cardinality is not bounded by this grammar.
 */

quantumOperationParameterClause
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * 7. OPERATION TARGET CLAUSE
 * ============================================================================
 *
 * Examples:
 *
 *     (q)
 *
 *     (q0, q1)
 *
 *     (register)
 *
 *     (register[i], register[j])
 *
 *     (selection)
 *
 * Targets are ordinary expressions.
 *
 * Semantic analysis determines whether an expression is a legal quantum
 * operand.
 */

quantumOperationTargetClause
    : LPAREN quantumOperationTargetList? RPAREN
    ;


/*
 * ============================================================================
 * 8. OPERATION TARGET LIST
 * ============================================================================
 *
 * There is deliberately no finite target count.
 *
 * The semantic layer determines:
 *
 *     target roles;
 *     target types;
 *     operation arity;
 *     aliasing rules;
 *     overlap rules;
 *     dimensional compatibility.
 */

quantumOperationTargetList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. OPERATION TARGET
 * ============================================================================
 *
 * Named wrapper for downstream grammar consumers.
 *
 * This does not create a quantum-specific target-reference grammar.
 */

quantumOperationTarget
    : expression
    ;


/*
 * ============================================================================
 * 10. REUSABLE TARGET LIST
 * ============================================================================
 *
 * This rule exists for consumers that need an explicitly named target-list
 * production without reimplementing the list syntax.
 */

quantumOperationTargets
    : quantumOperationTarget
      (
          COMMA
          quantumOperationTarget
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. OPERATION MODIFIER
 * ============================================================================
 *
 * Core modifier forms:
 *
 *     control(operation)
 *     adjoint(operation)
 *     inverse(operation)
 *
 * Modifiers are recursively nestable.
 *
 * No finite nesting depth is encoded.
 */

quantumOperationModifier
    : CONTROL
      LPAREN
      quantumOperationModifierOperand
      RPAREN

    | ADJOINT
      LPAREN
      quantumOperationModifierOperand
      RPAREN

    | INVERSE
      LPAREN
      quantumOperationModifierOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 12. MODIFIER OPERAND
 * ============================================================================
 *
 * A modifier may wrap:
 *
 *     an operation designator;
 *     another modifier;
 *
 * An optional parameter clause is allowed on a directly named operation.
 *
 * Therefore these forms are structurally valid:
 *
 *     control(X)
 *     control(RX(theta))
 *     adjoint(U)
 *     inverse(U)
 *     control(adjoint(U))
 *     inverse(control(RX(theta)))
 */

quantumOperationModifierOperand
    : quantumOperationDesignator
      quantumOperationParameterClause?

    | quantumOperationModifier
    ;


/*
 * ============================================================================
 * 13. OPERATION REFERENCE
 * ============================================================================
 *
 * This rule exposes the reusable operation identity independently from an
 * invocation.
 */

quantumOperationReference
    : quantumOperationDesignator
    ;


/*
 * ============================================================================
 * 14. CONTROLLED OPERATION REFERENCE
 * ============================================================================
 *
 * This represents the modifier portion only.
 *
 * Operand interpretation remains semantic.
 */

quantumControlledOperation
    : CONTROL
      LPAREN
      quantumOperationModifierOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 15. ADJOINT OPERATION REFERENCE
 * ============================================================================
 */

quantumAdjointOperation
    : ADJOINT
      LPAREN
      quantumOperationModifierOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 16. INVERSE OPERATION REFERENCE
 * ============================================================================
 */

quantumInverseOperation
    : INVERSE
      LPAREN
      quantumOperationModifierOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 17. EXTENDED INVOCATION
 * ============================================================================
 *
 * Canonical reusable alias for downstream quantum composition.
 */

quantumExtendedOperationInvocation
    : quantumOperationInvocation
    ;


/*
 * ============================================================================
 * 18. OPERATION EXPRESSION REFERENCE
 * ============================================================================
 *
 * This is intentionally an operation-reference abstraction rather than a
 * second universal expression hierarchy.
 *
 * Expression-level quantum semantics belong to:
 *
 *     grammar/expressions/quantum.g4
 *
 * when an operation must participate directly in a general expression.
 */

quantumOperationExpressionReference
    : quantumOperationReference
    ;


/*
 * ============================================================================
 * 19. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The following constructs are intentionally unbounded by language grammar:
 *
 *     operation count
 *     target count
 *     parameter count
 *     control count
 *     modifier nesting
 *     namespace depth
 *     generic argument count
 *
 * Examples of structurally valid scalable forms include:
 *
 *     apply operation(q);
 *
 *     apply operation(q0, q1, q2, q3, ...);
 *
 *     apply operation(p0, p1, p2, ...)(q0, q1, q2, ...);
 *
 *     apply control(operation)(c0, c1, c2, ..., target);
 *
 *     apply control(adjoint(inverse(operation)))(...);
 *
 * The actual resource feasibility of these programs is deliberately deferred
 * to semantic/resource/capability analysis and later compilation stages.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal limits:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * No such limits are represented in this grammar.
 *
 * Also forbidden:
 *
 *     physical qubit numbers;
 *     vendor gate inventories;
 *     native gate assumptions;
 *     fixed topology;
 *     fixed register width;
 *     fixed device identifiers;
 *     backend-specific scheduling;
 *     calibration data.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. SEMANTIC RESOURCE SEPARATION
 * ============================================================================
 *
 * The following concepts remain downstream:
 *
 * REQUIREMENT
 *     e.g. requires qubits >= n
 *
 * CAPABILITY
 *     e.g. requires capability("quantum.mid_circuit_measurement")
 *
 * PREFERENCE
 *     e.g. prefer accelerator("quantum")
 *
 * IMPLEMENTATION DECISION
 *     e.g. physical placement / routing / target mapping
 *
 * `operations.g4` must not collapse these categories into operation syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * The complete lowering chain remains:
 *
 *     quantumOperationStatement
 *             |
 *             v
 *     frontend AST operation
 *             |
 *             v
 *     semantic quantum operation
 *             |
 *             v
 *     quantum::ir
 *             |
 *             +--> optimization
 *             +--> decomposition
 *             +--> routing
 *             +--> scheduling
 *             +--> QEC
 *             +--> resilience
 *             +--> ZQN
 *             +--> HAL
 *
 * No operation grammar production may instantiate or encode a physical
 * operation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics should identify structural failures with source
 * spans.
 *
 * Examples:
 *
 *     missing operation name
 *     missing parameter close delimiter
 *     missing target close delimiter
 *     missing target expression
 *     missing modifier operand
 *     malformed modifier nesting
 *     malformed qualified operation name
 *
 * Semantic diagnostics handle:
 *
 *     unknown operation;
 *     invalid operation parameters;
 *     invalid target type;
 *     invalid control operand;
 *     unsupported adjoint;
 *     unsupported inverse;
 *     insufficient capability;
 *     insufficient resources;
 *     unsupported target realization.
 *
 * Resource/capability failures MUST NOT be reported as parser syntax errors.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     filesystem operations;
 *     network operations;
 *     environment inspection;
 *     target probing;
 *     runtime execution;
 *     embedded Rust actions;
 *     semantic predicates;
 *     dynamic code execution.
 *
 * Any untrusted-input resource limits required by the parser implementation
 * are implementation-level protections and MUST NOT become language semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Operation syntax is intentionally regular and compositional.
 *
 * Unbounded source cardinality is represented with ordinary parser repetition
 * rather than manually enumerated alternatives.
 *
 * Implementations should preserve parser progress on malformed input.
 *
 * No recursive rule here is required merely to count resources.
 *
 * Modifier recursion is structural and naturally bounded by the actual source
 * nesting depth rather than an artificial language constant.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. POSITIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * These must parse:
 *
 *     apply H(q);
 *
 *     apply X(q);
 *
 *     apply CNOT(q0, q1);
 *
 *     apply RX(theta)(q);
 *
 *     apply U(theta, phi, lambda)(q0, q1);
 *
 *     apply custom_gate(q);
 *
 *     apply vendor::operation(q);
 *
 *     apply library::operation(parameter)(q);
 *
 *     apply operation::<T>(q);
 *
 *     apply control(X)(control, target);
 *
 *     apply control(X)(c0, c1, target);
 *
 *     apply adjoint(U)(q);
 *
 *     apply inverse(U)(q);
 *
 *     apply control(adjoint(U))(control, target);
 *
 *     apply inverse(control(RX(theta)))(control, target);
 *
 *     apply operation(q0, q1, q2, q3, q4);
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. NEGATIVE CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * These must NOT parse as complete operation statements:
 *
 *     apply;
 *
 *     apply H
 *
 *     apply H(
 *
 *     apply H);
 *
 *     apply control;
 *
 *     apply control(
 *
 *     apply control(X;
 *
 *     apply adjoint;
 *
 *     apply inverse;
 *
 *     apply H(q
 *
 *     apply H(q);
 *     <missing closing source delimiter>
 *
 * The final case is intentionally structural; semantic validity of H(q)
 * belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. SEMANTICALLY INVALID BUT SYNTACTICALLY STRUCTURED EXAMPLES
 * ============================================================================
 *
 * These may parse structurally and must be rejected later when invalid:
 *
 *     apply operation_that_does_not_exist(q);
 *
 *     apply operation(incompatible_parameter)(q);
 *
 *     apply operation(invalid_target);
 *
 *     apply adjoint(non_unitary_operation)(q);
 *
 *     apply inverse(non_invertible_operation)(q);
 *
 *     apply control(operation)(invalid_control, target);
 *
 * The parser must not attempt to resolve these meanings.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. BOUNDARY / SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     one target;
 *     multiple targets;
 *     one parameter;
 *     many parameters;
 *     one control;
 *     many controls;
 *     nested modifiers;
 *     qualified names;
 *     generic operation references;
 *     indexed targets;
 *     sliced/register-view targets;
 *     expression-derived targets;
 *     empty target clauses where the semantic operation permits them;
 *     very deep but valid modifier nesting;
 *     very long operation argument lists;
 *     very long target lists.
 *
 * No test may define a universal maximum.
 *
 * The purpose of scalability tests is to demonstrate that grammar cardinality
 * is not artificially capped.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * This grammar can be used from:
 *
 *     quantum.g4
 *     hybrid quantum/classical composition
 *     circuit grammar
 *     dynamic quantum constructs
 *     interoperability adapters
 *     dialect composition
 *
 * It must NOT directly depend on:
 *
 *     hardware.g4
 *     resources.g4
 *     execution.g4
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * Those dependencies flow downstream from semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical source forms remain structurally supported:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(q0, q1);
 *
 * Generic operation names remain extensible.
 *
 * This grammar intentionally does not require changes to the lexical spelling
 * of operation names.
 *
 * Any change to:
 *
 *     apply
 *     control
 *     adjoint
 *     inverse
 *     parentheses
 *     comma
 *     semicolon
 *
 * must be coordinated through the canonical lexer/specification authority.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. COMPLETION CRITERIA
 * ============================================================================
 *
 * operations.g4 is complete when:
 *
 * [x] It has one canonical parser grammar declaration.
 * [x] It consumes the canonical ZamaniLexer vocabulary.
 * [x] It owns one canonical quantum operation statement.
 * [x] Operation names remain open-ended.
 * [x] Qualified names are delegated to Names.
 * [x] Expressions are delegated to Expressions.
 * [x] Generic syntax is delegated to existing generic grammar.
 * [x] Parameters are structurally distinct from targets.
 * [x] Target lists are unbounded by grammar.
 * [x] Modifier nesting is unbounded by grammar.
 * [x] Control count is unbounded by grammar.
 * [x] No fixed quantum gate enumeration exists.
 * [x] No physical qubit mapping exists.
 * [x] No hardware topology exists.
 * [x] No machine-size limits exist.
 * [x] No resource limits exist.
 * [x] No QEC implementation exists.
 * [x] No ZQN implementation exists.
 * [x] No routing exists.
 * [x] No scheduling exists.
 * [x] No calibration exists.
 * [x] No second quantum IR exists.
 * [x] No embedded Rust exists.
 * [x] No unsafe Rust is required.
 *
 * Repository-level completion additionally requires:
 *
 * [ ] QuantumOperations is imported by the canonical Quantum composition
 *     grammar.
 *
 * [ ] quantumOperationElement delegates only to
 *     quantumOperationStatement.
 *
 * [ ] No competing grammar defines another canonical
 *     quantumOperationStatement.
 *
 * [ ] controlled-operations.g4 no longer competes for ordinary operation
 *     invocation ownership.
 *
 * [ ] parameterized-operations.g4 no longer competes for ordinary parameter
 *     invocation ownership.
 *
 * [ ] gates.g4 does not enumerate a closed application gate set.
 *
 * [ ] Frontend AST has a generic operation representation.
 *
 * [ ] Semantic analysis resolves operation names and operand roles.
 *
 * [ ] Lowering reaches canonical quantum::ir.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Round-trip tests exist where the formatter/printer supports them.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */