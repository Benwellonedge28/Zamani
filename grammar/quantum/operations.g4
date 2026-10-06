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
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical SOURCE-SYNTAX owner for invoking quantum
 * operations.
 *
 * It defines the language-independent structure of a quantum operation
 * invocation without defining the finite universe of quantum operations.
 *
 * Canonical source forms include:
 *
 *     apply operation(q);
 *
 *     apply operation()(q);
 *
 *     apply operation(parameter)(q);
 *
 *     apply operation(p0, p1)(q0, q1);
 *
 *     apply namespace::operation(q);
 *
 *     apply namespace::operation(parameter)(q);
 *
 *     apply operation::<T>(parameter)(q);
 *
 *     apply control(operation)(control, target);
 *
 *     apply adjoint(operation)(target);
 *
 *     apply inverse(operation)(target);
 *
 *     apply control(adjoint(operation))(control, target);
 *
 *     apply inverse(control(operation))(control, target);
 *
 * Operation identity is OPEN-WORLD.
 *
 * The grammar deliberately does NOT enumerate:
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
 *     U
 *     vendor operations
 *     simulator operations
 *     future operations
 *     user-defined operations
 *
 * Those remain ordinary source-level names.
 *
 * Their meaning is resolved by semantic analysis.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     quantumOperationStatement
 *     quantumOperation
 *     quantumOperationApplication
 *     quantumOperationSpecifier
 *     quantumOperationDesignator
 *     quantumOperationReference
 *     quantumOperationInvocation
 *     quantumOperationTargetClause
 *     quantumOperationTargetList
 *     quantumOperationTarget
 *     quantumOperationTargets
 *     quantumOperationModifier
 *     quantumOperationModifierOperand
 *     quantumControlledOperation
 *     quantumAdjointOperation
 *     quantumInverseOperation
 *     quantumExtendedOperationInvocation
 *     quantumOperationExpressionReference
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical tokens
 *     identifiers
 *     qualified-name syntax
 *     generic argument syntax
 *     general expressions
 *     general types
 *     quantum parameter declarations
 *     quantum parameter argument syntax
 *     qubit declarations
 *     register declarations
 *     logical qubits
 *     physical qubits
 *     quantum states
 *     measurements
 *     reset
 *     observables
 *     barriers
 *     noise
 *     error correction
 *     dynamic-circuit control
 *     classical feed-forward
 *     gate definitions
 *     gate bodies
 *     gate matrices
 *     resource allocation
 *     capability discovery
 *     hardware selection
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     resilience
 *     ZQN
 *     HAL
 *     runtime execution
 *     quantum::ir implementation
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Quantum operation invocation is owned here.
 *
 * Parameter syntax is owned by:
 *
 *     grammar/quantum/parameters.g4
 *
 * Therefore this file MUST consume:
 *
 *     QuantumParameters
 *
 * rather than redefine:
 *
 *     quantumOperationArgumentList
 *     quantumOperationArguments
 *     quantumOperationArgument
 *     quantumOperationNamedArgument
 *     quantumOperationPositionalArgument
 *     quantumOperationPackArgument
 *     quantumParameterExpression
 *
 * This prevents parameter syntax from being implemented twice.
 *
 * Operation declaration syntax belongs to the appropriate declaration/gate
 * grammar.
 *
 * Controlled-operation extensions belong to:
 *
 *     grammar/quantum/controlled-operations.g4
 *
 * but the fundamental operation modifier boundary remains available here so
 * operation invocation can remain composable.
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
 *          v
 *     QuantumOperations
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
 *          +--> generic resolution
 *          +--> parameter binding
 *          +--> type checking
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          |
 *          v
 *     canonical quantum semantic representation
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
 *     execution
 *
 * This grammar MUST NOT bypass the AST or semantic model.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A quantum operation expresses PORTABLE COMPUTATIONAL INTENT.
 *
 * It does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     physical qubit
 *     physical register
 *     device identifier
 *     coupling map
 *     topology
 *     calibration
 *     pulse
 *     scheduler
 *     router
 *     vendor instruction
 *
 * Those are resolved downstream.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * The operation namespace is intentionally open.
 *
 * Any valid operation name may be represented structurally:
 *
 *     operation
 *     namespace::operation
 *     vendor::operation
 *     library::operation
 *     future::operation
 *     user::defined::operation
 *
 * The parser does not need to change when a new quantum operation is added.
 *
 * New operations are introduced through:
 *
 *     semantic registration
 *     declarations
 *     libraries
 *     dialects
 *     imported modules
 *     capability providers
 *     future language extensions
 *
 * They are NOT added as lexer keywords.
 *
 * ============================================================================
 * NO GATE CATALOG
 * ============================================================================
 *
 * This grammar intentionally contains no rules such as:
 *
 *     quantumGate
 *         : H
 *         | X
 *         | Y
 *         | Z
 *         | CNOT
 *         | ...
 *
 * Such a grammar would create an artificial language ceiling and would force
 * every future operation to modify the universal parser.
 *
 * Operation identity is data.
 *
 * Operation semantics are semantic information.
 *
 * Operation realization is downstream information.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no fixed maximum for:
 *
 *     operations
 *     parameters
 *     targets
 *     controls
 *     namespace depth
 *     generic arguments
 *     modifier nesting
 *     circuit size
 *     program size
 *     quantum registers
 *     qubits
 *     devices
 *     nodes
 *     processors
 *     memory
 *     accelerators
 *
 * There are deliberately no constants such as:
 *
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_PARAMETERS
 *     MAX_OPERATIONS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * Actual feasibility is determined downstream from available resources,
 * capabilities, policies and execution context.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: LANGUAGE SCALE VS RESOURCE SCALE
 * ============================================================================
 *
 * "Tiny to infinity" means the language grammar does not artificially cap
 * computational scale.
 *
 * It does NOT claim that a physical machine has infinite resources.
 *
 * Therefore:
 *
 *     parsing
 *
 * is independent from:
 *
 *     resource feasibility.
 *
 * For example:
 *
 *     apply operation(q0, q1, q2, ...)(...);
 *
 * may be syntactically representable regardless of eventual machine size.
 *
 * Whether the program can actually execute is determined by:
 *
 *     semantic analysis
 *     resource analysis
 *     capability negotiation
 *     compilation
 *     routing
 *     scheduling
 *     runtime
 *     HAL
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer boundary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This grammar MUST NOT declare lexer rules.
 *
 * Quantum operation names such as:
 *
 *     H
 *     X
 *     CNOT
 *     RX
 *     custom_operation
 *
 * MUST remain identifiers.
 *
 * Language-level quantum vocabulary such as:
 *
 *     apply
 *     control
 *     adjoint
 *     inverse
 *
 * is supplied by the canonical lexer.
 *
 * ============================================================================
 * PARSER DEPENDENCIES
 * ============================================================================
 *
 * Names:
 *
 *     qualifiedName
 *     identifier
 *
 * are owned by the canonical Names grammar.
 *
 * Expressions:
 *
 *     expression
 *
 * are owned by the canonical Expressions grammar.
 *
 * Generic operation arguments:
 *
 *     genericArgumentSuffix
 *
 * are consumed from the canonical expression/type system.
 *
 * Quantum parameter syntax is owned by:
 *
 *     QuantumParameters
 *
 * ============================================================================
 * GENERIC OPERATION SUPPORT
 * ============================================================================
 *
 * The operation grammar must not create a second generic type/argument system.
 *
 * Generic operation identity is represented by the existing generic suffix:
 *
 *     operation::<T>
 *
 * or the canonical equivalent accepted by the universal expression grammar.
 *
 * Semantic analysis determines:
 *
 *     generic parameter count
 *     generic parameter kinds
 *     type compatibility
 *     specialization
 *     inference
 *
 * ============================================================================
 * PARAMETER / TARGET SEPARATION
 * ============================================================================
 *
 * Quantum operation parameters and quantum operation targets are structurally
 * distinct.
 *
 * Examples:
 *
 *     apply H(q);
 *
 *     apply RX(theta)(q);
 *
 *     apply U(theta, phi, lambda)(q0, q1);
 *
 *     apply operation(parameter)(target);
 *
 * The operation parameter clause is supplied by QuantumParameters.
 *
 * The operation target clause is owned here.
 *
 * This distinction is important because:
 *
 *     parameters
 *
 * describe values controlling operation behavior, while:
 *
 *     targets
 *
 * identify values/resources on which the operation acts.
 *
 * Semantic analysis determines their actual types and roles.
 *
 * ============================================================================
 * EMPTY PARAMETER CLAUSE
 * ============================================================================
 *
 * The grammar permits:
 *
 *     apply operation()(q);
 *
 * even where an operation ultimately has no parameters.
 *
 * Whether an empty parameter clause is semantically meaningful for a particular
 * operation is a semantic question.
 *
 * This allows syntax to remain generic and open-world.
 *
 * ============================================================================
 * TARGET MODEL
 * ============================================================================
 *
 * Targets are ordinary expressions.
 *
 * This deliberately permits source forms such as:
 *
 *     q
 *     q[0]
 *     register
 *     register[i]
 *     selection
 *     expression
 *
 * without embedding a second target-reference language here.
 *
 * Semantic analysis determines whether the expression is a valid quantum
 * operand and determines:
 *
 *     operand type
 *     operand role
 *     arity
 *     aliasing
 *     overlap
 *     ordering
 *     dimensionality
 *     mutability/ownership rules
 *     logical/physical meaning
 *
 * ============================================================================
 * CONTROL / ADJOINT / INVERSE
 * ============================================================================
 *
 * These are operation modifiers.
 *
 * Canonical structural forms:
 *
 *     control(operation)
 *     adjoint(operation)
 *     inverse(operation)
 *
 * Modifiers may be nested:
 *
 *     control(adjoint(operation))
 *
 *     inverse(control(operation))
 *
 *     control(inverse(adjoint(operation)))
 *
 * No finite nesting depth is imposed.
 *
 * Whether a modifier is mathematically or semantically valid for a particular
 * operation is determined by semantic analysis.
 *
 * ============================================================================
 * CONTROL OPERANDS
 * ============================================================================
 *
 * The base operation grammar deliberately does not impose a fixed control
 * count.
 *
 * Example:
 *
 *     apply control(operation)(c, q);
 *
 *     apply control(operation)(c0, c1, q);
 *
 *     apply control(operation)(controls, targets);
 *
 * Semantic analysis determines:
 *
 *     control roles
 *     target roles
 *     operand partition
 *     control polarity
 *     operation signature
 *     type compatibility
 *
 * More specialized control syntax belongs to:
 *
 *     controlled-operations.g4
 *
 * and:
 *
 *     controls.g4
 *
 * ============================================================================
 * OPERATION SPECIFIER
 * ============================================================================
 *
 * `quantumOperationSpecifier` is the stable boundary between:
 *
 *     WHAT operation is being invoked
 *
 * and:
 *
 *     HOW it is being applied.
 *
 * It may identify:
 *
 *     an ordinary operation
 *     a qualified operation
 *     a generic operation
 *     a modified operation
 *
 * The specifier does not contain target operands.
 *
 * It does not perform name resolution.
 *
 * It does not select a backend.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar must preserve enough structure for the domain-neutral AST to
 * represent:
 *
 *     operation designator
 *     namespace/path
 *     generic arguments
 *     parameter arguments
 *     target expressions
 *     modifier nesting
 *     source ordering
 *     source spans
 *
 * Conceptual semantic shape:
 *
 *     QuantumOperation {
 *         specifier,
 *         parameters,
 *         targets,
 *         source_span
 *     }
 *
 * The actual Rust AST remains owned by the repository frontend.
 *
 * This grammar MUST NOT define Rust AST structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     operation existence
 *     operation visibility
 *     operation overload resolution
 *     generic argument validity
 *     parameter binding
 *     target validity
 *     operand roles
 *     arity
 *     type compatibility
 *     modifier validity
 *     effect requirements
 *     capability requirements
 *     resource requirements
 *     contracts
 *     policies
 *     provenance
 *     target support
 *     decomposition requirements
 *
 * None of these belong in parser syntax.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A quantum operation may eventually carry semantic effects such as:
 *
 *     quantum
 *     measurement
 *     randomness
 *     native
 *     foreign
 *     simulation
 *     distributed
 *
 * This grammar does not hard-code those effects.
 *
 * Effects are determined from the resolved operation semantics.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * An operation may require capabilities such as:
 *
 *     quantum.operation
 *     quantum.dynamic_control
 *     quantum.measurement
 *     quantum.parameterized_operation
 *     quantum.custom_operation
 *
 * Capability names remain open-world.
 *
 * This grammar does not test capability availability.
 *
 * Capability negotiation belongs downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Operation syntax does not allocate resources.
 *
 * Resource requirements may be inferred from semantic operation definitions
 * and operation operands.
 *
 * The resource layer may determine requirements involving:
 *
 *     qubits
 *     logical qubits
 *     quantum memory
 *     execution time
 *     measurement capacity
 *     communication
 *     accelerator capacity
 *     resilience
 *     other future resources
 *
 * No resource quantity is hard-coded here.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Operations may participate in universal:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     policy
 *
 * systems.
 *
 * Those systems remain outside this grammar.
 *
 * An operation invocation can be associated with such information by the
 * surrounding semantic/attribute/contract layers.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The operation source structure must remain traceable through the frontend.
 *
 * Provenance may later record:
 *
 *     source span
 *     operation identity
 *     parameter bindings
 *     target expressions
 *     semantic resolution
 *     transformations
 *     decomposition
 *     routing
 *     scheduling
 *     optimization
 *     lowering
 *
 * This grammar itself records no runtime provenance.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * The parser does NOT produce `quantum::ir`.
 *
 * The required path is:
 *
 *     source
 *       |
 *       v
 *     parse tree
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum operation
 *       |
 *       v
 *     canonical quantum::ir
 *
 * There must be exactly one canonical quantum IR boundary.
 *
 * This grammar must not define:
 *
 *     QuantumOperationIR
 *     QuantumGateIR
 *     QuantumCircuitIR
 *     PhysicalQuantumOperation
 *     VendorQuantumOperation
 *     HardwareQuantumOperation
 *
 * as competing IRs.
 *
 * ============================================================================
 * TARGET LOWERING CONTRACT
 * ============================================================================
 *
 * Target-specific realization occurs after semantic quantum representation.
 *
 * The operation may eventually be transformed through:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *
 * The source operation does not prescribe any of these transformations.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no runtime calls
 *     no randomness
 *
 * Parsing depends only on:
 *
 *     source text
 *     selected grammar
 *     canonical lexer vocabulary
 *     imported parser grammars
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * Syntax errors belong here.
 *
 * Examples of structurally invalid forms:
 *
 *     apply;
 *     apply operation;
 *     apply operation(;
 *     apply operation);
 *     apply control;
 *     apply control(;
 *     apply adjoint;
 *     apply inverse;
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     apply nonexistent_operation(q);
 *
 *     apply operation(invalid_parameter)(q);
 *
 *     apply operation(invalid_target);
 *
 *     apply inverse(non_invertible_operation)(q);
 *
 * These are parsed structurally and rejected during semantic analysis.
 *
 * Resource failure is also not a syntax failure.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid forms must remain representable:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(control, target);
 *     apply RX(theta)(q);
 *     apply U(theta, phi, lambda)(q0, q1);
 *     apply library::operation(q);
 *     apply vendor::operation(parameter)(q);
 *     apply control(X)(control, target);
 *     apply adjoint(U)(q);
 *     apply inverse(U)(q);
 *
 * The grammar does not reserve those operation names.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4 / canonical Names grammar
 *     grammar/expressions/*
 *     grammar/quantum/parameters.g4
 *
 * THIS FILE:
 *
 *     grammar/quantum/operations.g4
 *
 * DOWNSTREAM:
 *
 *     grammar/quantum/quantum.g4
 *     frontend parser
 *     domain-neutral AST
 *     structural validation
 *     semantic quantum operation resolution
 *     capability analysis
 *     resource analysis
 *     effect analysis
 *     contract/policy analysis
 *     provenance
 *     canonical quantum::ir
 *
 * RELATED OWNERS:
 *
 *     parameters.g4
 *         parameter syntax
 *
 *     controlled-operations.g4
 *         advanced control syntax
 *
 *     controls.g4
 *         control semantics/syntax extensions
 *
 *     adjoints.g4
 *         adjoint-specific syntax/extensions
 *
 *     gates.g4
 *         operation/gate declarations where applicable
 *
 *     circuits.g4
 *         circuit structure
 *
 *     measurement.g4
 *         measurement
 *
 *     reset.g4
 *         reset
 *
 *     quantum-capabilities.g4
 *         quantum capability declarations/references
 *
 *     quantum-resources.g4
 *         quantum resource intent
 *
 *     resource-requirements.g4
 *         resource requirement integration
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] all operation invocations have one canonical entry point;
 *     [ ] operation names remain open-world identifiers;
 *     [ ] no finite gate catalogue exists here;
 *     [ ] parameter syntax is delegated to QuantumParameters;
 *     [ ] target syntax is owned here;
 *     [ ] generic operation identity is supported;
 *     [ ] modifiers are recursively representable;
 *     [ ] no fixed operation/target/control/parameter limit exists;
 *     [ ] no hardware-specific syntax exists;
 *     [ ] no physical qubit syntax exists;
 *     [ ] no routing/scheduling syntax exists;
 *     [ ] no QEC/ZQN/HAL logic exists;
 *     [ ] no second quantum IR exists;
 *     [ ] parser remains deterministic;
 *     [ ] parser remains free of embedded Rust;
 *     [ ] generated Rust remains safe;
 *     [ ] Rust 1.97+ remains supported;
 *     [ ] Quantum root can import this grammar;
 *     [ ] semantic layer can map the result to canonical quantum::ir;
 *     [ ] positive tests cover generic operations;
 *     [ ] negative tests cover structural failures;
 *     [ ] boundary tests cover modifiers and parameter/target separation;
 *     [ ] scalability tests contain no artificial size assumptions.
 *
 * ============================================================================
 */

parser grammar QuantumOperations;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    QuantumParameters
;


/*
 * ============================================================================
 * 1. CANONICAL OPERATION STATEMENT
 * ============================================================================
 *
 * The statement owns the terminating semicolon.
 *
 * The operation itself remains reusable without the statement terminator.
 */

quantumOperationStatement
    : quantumOperationApplication SEMICOLON
    ;


/*
 * ============================================================================
 * 2. CANONICAL QUANTUM OPERATION
 * ============================================================================
 *
 * This is the principal reusable operation-invocation rule.
 *
 * Conceptually:
 *
 *     operation specifier
 *     + optional parameter arguments
 *     + target operands
 *
 * Examples:
 *
 *     H(q)
 *
 *     RX(theta)(q)
 *
 *     operation(a, b)(q0, q1)
 *
 *     control(X)(c, q)
 */

quantumOperation
    : quantumOperationSpecifier
      quantumOperationParameterArguments?
      quantumOperationTargetClause
    ;


/*
 * ============================================================================
 * 3. OPERATION APPLICATION
 * ============================================================================
 *
 * Canonical source-level statement:
 *
 *     apply operation(...);
 *
 * `apply` is lexical language vocabulary.
 *
 * It does not imply a particular hardware execution mechanism.
 */

quantumOperationApplication
    : APPLY quantumOperation
    ;


/*
 * ============================================================================
 * 4. OPERATION SPECIFIER
 * ============================================================================
 *
 * The operation specifier identifies what operation is being invoked.
 *
 * It intentionally excludes target operands.
 *
 * It may be:
 *
 *     an ordinary operation designator
 *     a recursively modified operation
 *
 * The semantic layer resolves the resulting operation identity.
 */

quantumOperationSpecifier
    : quantumOperationDesignator
    | quantumOperationModifier
    ;


/*
 * ============================================================================
 * 5. OPERATION DESIGNATOR
 * ============================================================================
 *
 * Operation names are canonical qualified names.
 *
 * Examples:
 *
 *     H
 *     custom_operation
 *     quantum::operation
 *     library::quantum::operation
 *     vendor::operation
 *
 * Generic operation specialization may follow through the canonical generic
 * argument suffix.
 *
 * No operation catalog is encoded.
 */

quantumOperationDesignator
    : qualifiedName
      genericArgumentSuffix?
    ;


/*
 * ============================================================================
 * 6. PARAMETER ARGUMENTS
 * ============================================================================
 *
 * Parameter syntax is owned by QuantumParameters.
 *
 * This wrapper exists so that operations.g4 owns the integration boundary
 * without duplicating the parameter grammar.
 *
 * Examples:
 *
 *     ()
 *     (theta)
 *     (theta, phi)
 *     (theta = value)
 *     (theta, phi = value)
 *     (...parameters)
 */

quantumOperationParameterArguments
    : quantumOperationArgumentList
    ;


/*
 * ============================================================================
 * 7. TARGET CLAUSE
 * ============================================================================
 *
 * Targets are structurally distinct from parameter arguments.
 *
 * Examples:
 *
 *     (q)
 *     (q0, q1)
 *     (register)
 *     (register[i], register[j])
 *
 * Semantic analysis decides whether the expressions are legal quantum
 * operands.
 */

quantumOperationTargetClause
    : LPAREN
      quantumOperationTargetList?
      RPAREN
    ;


/*
 * ============================================================================
 * 8. TARGET LIST
 * ============================================================================
 *
 * No fixed target cardinality exists.
 *
 * A semantic operation signature determines the legal target structure.
 */

quantumOperationTargetList
    : quantumOperationTarget
      (
          COMMA
          quantumOperationTarget
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. SINGLE TARGET
 * ============================================================================
 *
 * The target is a canonical Zamani expression.
 *
 * This allows target selection to evolve independently from the operation
 * grammar.
 */

quantumOperationTarget
    : expression
    ;


/*
 * ============================================================================
 * 10. REUSABLE TARGET LIST
 * ============================================================================
 *
 * Named reusable boundary for quantum grammar consumers.
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
 * Core modifiers:
 *
 *     control(operation)
 *     adjoint(operation)
 *     inverse(operation)
 *
 * Recursive composition is intentional.
 *
 * Examples:
 *
 *     control(X)
 *     adjoint(U)
 *     inverse(U)
 *     control(adjoint(U))
 *     inverse(control(U))
 *     control(inverse(adjoint(U)))
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
 *     operation designator
 *     recursively modified operation
 *
 * A parameter argument list may be attached to a directly named operation.
 *
 * Examples:
 *
 *     control(X)
 *
 *     control(RX(theta))
 *
 *     adjoint(U(theta))
 *
 *     inverse(operation(parameter))
 *
 * Nested modifiers remain recursively composable.
 */

quantumOperationModifierOperand
    : quantumOperationDesignator
      quantumOperationParameterArguments?

    | quantumOperationModifier
    ;


/*
 * ============================================================================
 * 13. OPERATION REFERENCE
 * ============================================================================
 *
 * Reusable operation identity without target operands.
 */

quantumOperationReference
    : quantumOperationDesignator
    ;


/*
 * ============================================================================
 * 14. CONTROLLED OPERATION REFERENCE
 * ============================================================================
 *
 * This exposes the control modifier as a reusable semantic boundary.
 *
 * Detailed control operand semantics remain downstream.
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
 * Reusable alias for consumers that require an explicitly named operation
 * invocation boundary.
 */

quantumExtendedOperationInvocation
    : quantumOperation
    ;


/*
 * ============================================================================
 * 18. OPERATION EXPRESSION REFERENCE
 * ============================================================================
 *
 * This is intentionally a reference to the operation identity rather than a
 * second expression hierarchy.
 *
 * General expression-level quantum semantics remain owned by the expression
 * subsystem.
 */

quantumOperationExpressionReference
    : quantumOperationReference
    ;


/*
 * ============================================================================
 * 19. OPERATION DESIGNATOR LIST
 * ============================================================================
 *
 * Reusable open-world list.
 *
 * No finite number of operations is encoded.
 */

quantumOperationDesignatorList
    : quantumOperationDesignator
      (
          COMMA
          quantumOperationDesignator
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. OPERATION SPECIFIER REFERENCE
 * ============================================================================
 *
 * Reusable boundary for grammars that need an operation identity or modifier
 * without invoking it.
 */

quantumOperationSpecifierReference
    : quantumOperationSpecifier
    ;


/*
 * ============================================================================
 * 21. SCALABILITY / CARDINALITY CONTRACT
 * ============================================================================
 *
 * The following are intentionally open-ended:
 *
 *     operation count
 *     parameter count
 *     target count
 *     control count
 *     modifier depth
 *     namespace depth
 *     generic argument count
 *
 * Examples:
 *
 *     apply operation(q);
 *
 *     apply operation(p0, p1)(q0, q1);
 *
 *     apply control(operation)(c0, c1, target);
 *
 *     apply control(adjoint(inverse(operation)))(...);
 *
 * Actual resource limits are external to this grammar.
 *
 * ============================================================================
 * 22. SEMANTIC NON-RESPONSIBILITIES
 * ============================================================================
 *
 * The parser MUST NOT determine:
 *
 *     whether an operation exists;
 *     whether an operation is callable;
 *     whether an operation is unitary;
 *     whether an operation has an adjoint;
 *     whether an operation has an inverse;
 *     whether a target is a qubit;
 *     whether a target is logical;
 *     whether a target is physical;
 *     whether an operation is supported by a backend;
 *     whether a capability exists;
 *     whether resources are sufficient;
 *     whether routing is required;
 *     whether decomposition is required;
 *     whether QEC is required;
 *     whether a pulse implementation exists;
 *     whether calibration exists.
 *
 * These belong to downstream semantic/compiler/runtime layers.
 *
 * ============================================================================
 * 23. SEMANTIC OPERATION SHAPE
 * ============================================================================
 *
 * The frontend should conceptually be able to derive:
 *
 *     operation
 *       ├── specifier
 *       │    ├── designator
 *       │    └── modifiers
 *       ├── parameters
 *       └── targets
 *
 * This is a semantic shape, not a Rust structure defined here.
 *
 * ============================================================================
 * 24. QUANTUM::IR BOUNDARY
 * ============================================================================
 *
 * A successfully resolved operation follows:
 *
 *     QuantumOperations
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic operation
 *          |
 *          v
 *     quantum::ir
 *
 * The grammar MUST NOT construct or reference:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumGate
 *     QuantumOperationIR
 *     PhysicalGate
 *     VendorInstruction
 *
 * The parser remains independent of Rust quantum IR implementation details.
 *
 * ============================================================================
 * 25. TARGET-INDEPENDENCE
 * ============================================================================
 *
 * Nothing in this file identifies:
 *
 *     a CPU
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     a QPU
 *     a simulator
 *     a vendor
 *     a physical device
 *     a physical qubit
 *     a topology
 *     a coupling map
 *     a pulse channel
 *     a calibration
 *
 * The same source operation can therefore participate in target-independent
 * compilation.
 *
 * ============================================================================
 * 26. DETERMINISTIC PARSING
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no embedded Rust
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime calls
 *     no randomness
 *
 * Parsing is therefore determined by source text and the selected grammar/
 * lexical configuration.
 *
 * ============================================================================
 * 27. SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar generates no Rust source itself beyond normal ANTLR-generated
 * parser artifacts.
 *
 * The Rust frontend consuming the parser must remain:
 *
 *     Rust 2021
 *     Rust 1.97+
 *     safe Rust
 *     no unsafe
 *
 * This file contains no target-language actions that could require unsafe
 * Rust.
 *
 * ============================================================================
 * 28. INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     Names
 *     Expressions
 *     QuantumParameters
 *
 * DIRECT COMPOSITION:
 *
 *     grammar/quantum/quantum.g4
 *
 * RELATED QUANTUM OWNERS:
 *
 *     parameters.g4
 *         parameter declarations and argument syntax
 *
 *     parameterized-operations.g4
 *         compatibility/reference material for parameterized operation
 *         constructs; it must not become a second operation-invocation
 *         authority
 *
 *     controlled-operations.g4
 *         extended control syntax
 *
 *     controls.g4
 *         control-specific syntax/semantics
 *
 *     adjoints.g4
 *         adjoint-specific syntax/semantics
 *
 *     gates.g4
 *         operation/gate declaration syntax
 *
 *     circuits.g4
 *         circuit structure
 *
 *     measurement.g4
 *         measurement syntax
 *
 *     reset.g4
 *         reset syntax
 *
 *     quantum-capabilities.g4
 *         capability contracts
 *
 *     quantum-resources.g4
 *         resource contracts
 *
 *     resource-requirements.g4
 *         resource requirement integration
 *
 * DOWNSTREAM:
 *
 *     domain-neutral AST
 *     structural validation
 *     semantic analysis
 *     type checking
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract/policy analysis
 *     provenance
 *     canonical quantum::ir
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * 29. ROOT-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * Quantum root:
 *
 *     grammar/quantum/quantum.g4
 *
 * imports:
 *
 *     QuantumOperations
 *
 * Therefore this grammar's public entry points are available to Quantum.
 *
 * The universal Zamani root must ultimately dispatch quantum operation
 * statements through the canonical quantum declaration/block composition.
 *
 * No second operation dispatcher should be introduced elsewhere.
 *
 * ============================================================================
 * 30. TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests must include:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(c, q);
 *     apply RX(theta)(q);
 *     apply U(theta, phi, lambda)(q0, q1);
 *     apply operation(q);
 *     apply namespace::operation(q);
 *     apply operation::<T>(theta)(q);
 *     apply operation()(q);
 *     apply control(operation)(c, q);
 *     apply control(operation)(c0, c1, q);
 *     apply adjoint(operation)(q);
 *     apply inverse(operation)(q);
 *     apply control(adjoint(operation))(c, q);
 *     apply inverse(control(operation))(c, q);
 *
 * Open-world tests must include operation names that are not part of any
 * built-in catalogue.
 *
 * Parameter/target separation tests must verify:
 *
 *     apply operation(theta)(q);
 *
 * is structurally different from:
 *
 *     apply operation(q);
 *
 * and:
 *
 *     apply operation(theta, phi)(q0, q1);
 *
 * Negative syntax tests must include:
 *
 *     apply;
 *     apply operation;
 *     apply operation(;
 *     apply operation);
 *     apply control;
 *     apply control(;
 *     apply adjoint;
 *     apply inverse;
 *
 * Semantic-negative tests must remain outside this grammar and verify:
 *
 *     unknown operation
 *     invalid parameter
 *     invalid target
 *     invalid modifier
 *     insufficient capability
 *     insufficient resource
 *
 * ============================================================================
 * 31. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must NOT use artificial repository constants such as:
 *
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_PARAMETERS
 *
 * Instead, scalability tests should construct operation structures whose
 * cardinality is determined by the test/resource environment.
 *
 * The grammar remains valid as cardinality increases until an external
 * implementation/resource constraint is reached.
 *
 * ============================================================================
 * 32. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain:
 *
 *     no finite operation catalogue
 *     no hardware catalogue
 *     no vendor catalogue
 *     no physical topology
 *     no machine-size constants
 *     no fixed qubit count
 *     no fixed control count
 *     no fixed parameter count
 *     no fixed target count
 *     no fixed circuit depth
 *     no backend selection
 *     no runtime discovery
 *     no hardware probing
 *     no embedded Rust
 *     no unsafe code
 *
 * ============================================================================
 * 33. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. QuantumOperations is the only canonical operation-invocation
 *        parser grammar.
 *
 *     2. `quantumOperation` is the canonical reusable operation rule.
 *
 *     3. `quantumOperationSpecifier` is the canonical operation-identity
 *        boundary.
 *
 *     4. Parameter syntax comes exclusively from QuantumParameters.
 *
 *     5. Target syntax comes exclusively from this file.
 *
 *     6. Operation names remain open-world identifiers.
 *
 *     7. No built-in gate list exists.
 *
 *     8. Generic operation identity remains extensible.
 *
 *     9. Recursive modifiers remain supported.
 *
 *     10. No finite hardware/resource limits are encoded.
 *
 *     11. The grammar remains independent of physical qubits and devices.
 *
 *     12. The grammar remains independent of routing and scheduling.
 *
 *     13. The grammar remains independent of QEC, ZQN and HAL.
 *
 *     14. The frontend can map the structure into the domain-neutral AST.
 *
 *     15. Semantic analysis can resolve it into the canonical quantum model.
 *
 *     16. The semantic model can lower to the existing quantum::ir.
 *
 *     17. Quantum root can import it without another operation authority.
 *
 *     18. Generated Rust remains compatible with Rust 1.97+ and safe Rust.
 *
 *     19. Positive, negative, boundary and scalability tests pass.
 *
 *     20. No subsequent modification is required merely because another
 *         downstream implementation adds a new quantum operation.
 *
 * ============================================================================
 */