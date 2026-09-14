/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-classical.g4
 *
 * Role:
 *     Quantum/classical interoperability grammar boundary.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-SYNTAX BOUNDARY between:
 *
 *     classical computation
 *
 * and:
 *
 *     quantum computation.
 *
 * It exists to make hybrid quantum-classical programs first-class Zamani
 * programs without creating a second expression language, second type system,
 * second quantum IR, second classical IR, or hardware-specific programming
 * model.
 *
 * The fundamental model is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     classical semantics          quantum semantics
 *          |                              |
 *          v                              v
 *     classical IR                 quantum::ir
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                   optimization
 *                          |
 *                          v
 *                     routing
 *                          |
 *                          v
 *                     scheduling
 *                          |
 *                          v
 *                         ZQN
 *                          |
 *                          v
 *                         QEC
 *                          |
 *                          v
 *                      resilience
 *                          |
 *                          v
 *                    hardware HAL
 *                          |
 *                          v
 *                    target/runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantum/classical boundary syntax;
 *     - classical-to-quantum parameter boundaries;
 *     - quantum-to-classical measurement boundaries;
 *     - measurement-result references;
 *     - classical conditions over quantum results;
 *     - dynamic quantum control syntax;
 *     - hybrid computation regions;
 *     - quantum invocation from classical code;
 *     - classical invocation around quantum code;
 *     - hybrid value-flow syntax;
 *     - explicit quantum/classical conversion boundaries;
 *     - hybrid result binding syntax;
 *     - hybrid capability/requirement composition;
 *     - hybrid effect-boundary syntax;
 *     - source-level synchronization between domains.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - literals;
 *     - general expressions;
 *     - expression precedence;
 *     - types;
 *     - generic types;
 *     - function declarations;
 *     - module declarations;
 *     - general statements;
 *     - quantum gate definitions;
 *     - quantum operations;
 *     - quantum registers;
 *     - quantum states;
 *     - quantum measurement semantics;
 *     - QEC algorithms;
 *     - ZQN noise models;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - calibration;
 *     - topology;
 *     - physical allocation;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - backend selection.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Quantum/classical interoperability is a SOURCE-LEVEL SEMANTIC RELATIONSHIP.
 *
 * It is NOT a hardware interface.
 *
 * Therefore this file MUST NOT contain assumptions about:
 *
 *     - number of qubits;
 *     - number of classical bits;
 *     - number of CPU cores;
 *     - number of threads;
 *     - number of GPUs;
 *     * number of accelerators;
 *     - machine memory;
 *     - register width;
 *     - SIMD width;
 *     - QPU size;
 *     - device topology;
 *     - physical qubit identifiers;
 *     - vendor;
 *     - backend;
 *     - native gate set;
 *     - pulse duration;
 *     - hardware timing;
 *     - network topology.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The hybrid source program describes:
 *
 *     computation
 *     data flow
 *     control flow
 *     quantum intent
 *     classical intent
 *     semantic requirements
 *     capabilities
 *     constraints
 *     effects
 *
 * It does NOT prescribe how the hybrid computation is physically realized.
 *
 * Therefore the following are valid semantic possibilities:
 *
 *     classical CPU + QPU
 *     classical CPU + simulator
 *     classical CPU + accelerator
 *     GPU + QPU
 *     distributed classical + distributed quantum
 *     embedded controller + quantum device
 *     future heterogeneous architecture
 *
 * without changing the source-level hybrid semantics.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level constants such as:
 *
 *     MAX_QUBITS
 *     MAX_CLASSICAL_BITS
 *     MAX_PARAMETERS
 *     MAX_MEASUREMENTS
 *     MAX_RESULTS
 *     MAX_BRANCHES
 *     MAX_DEVICES
 *     MAX_ITERATIONS
 *     MAX_NODES
 *     MAX_THREADS
 *
 * Repetition is represented structurally.
 *
 * Any implementation limit belongs to:
 *
 *     parser resource policy
 *     semantic resource policy
 *     compiler resource policy
 *     runtime resource policy
 *     hardware capability
 *     deployment configuration
 *
 * and MUST NOT become source-language semantics.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether a classical value is valid for a quantum parameter;
 *     - whether a measurement result is usable as a classical value;
 *     - whether a condition is valid for dynamic quantum control;
 *     - whether a conversion is lossless;
 *     - whether a conversion is permitted;
 *     - whether a value is compile-time or runtime;
 *     - whether a value depends on measurement;
 *     - whether an operation is deterministic;
 *     - whether an operation has quantum effects;
 *     - whether an operation has classical effects;
 *     - whether a hybrid boundary is supported by a target;
 *     - whether synchronization is required;
 *     - whether the resulting computation can be lowered.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file NEVER creates an IR.
 *
 * Hybrid syntax is lowered by the frontend/semantic pipeline into canonical
 * semantic representations.
 *
 * Quantum portions MUST eventually use:
 *
 *     quantum::ir
 *
 * Classical portions MUST use the repository's canonical classical semantic
 * representation.
 *
 * This file MUST NOT define:
 *
 *     QuantumGate
 *     QuantumInstruction
 *     ClassicalInstruction
 *     QubitId
 *     PhysicalQubitId
 *     ClassicalBitId
 *     QuantumRegister
 *     HardwareDevice
 *
 * or equivalent IR/data structures.
 *
 * ============================================================================
 * DYNAMIC-CIRCUIT CONTRACT
 * ============================================================================
 *
 * Dynamic quantum computation is explicitly supported.
 *
 * A classical expression may depend on:
 *
 *     - ordinary classical data;
 *     - function results;
 *     - prior measurement results;
 *     - compile-time values;
 *     - runtime values;
 *     - future supported classical domains.
 *
 * The grammar does not determine when such a value becomes available.
 *
 * Scheduling and runtime systems determine:
 *
 *     dependency;
 *     synchronization;
 *     latency;
 *     placement;
 *     execution order;
 *     target realization.
 *
 * ============================================================================
 * MEASUREMENT CONTRACT
 * ============================================================================
 *
 * Measurement is a semantic boundary:
 *
 *     quantum value/state
 *             |
 *             v
 *        measurement
 *             |
 *             v
 *     classical observable/result
 *
 * The grammar represents the source relationship.
 *
 * It does not prescribe:
 *
 *     bit width;
 *     encoding;
 *     register layout;
 *     hardware readout mechanism;
 *     readout latency;
 *     physical measurement implementation.
 *
 * ============================================================================
 * PARAMETER CONTRACT
 * ============================================================================
 *
 * Quantum operation parameters may originate from:
 *
 *     literals;
 *     constants;
 *     variables;
 *     function results;
 *     classical expressions;
 *     measurement-derived values;
 *     compile-time expressions;
 *     runtime expressions.
 *
 * The grammar does not determine whether a parameter is:
 *
 *     compile-time;
 *     runtime;
 *     host-side;
 *     device-side;
 *     symbolic;
 *     differentiable.
 *
 * Those are semantic/compiler properties.
 *
 * ============================================================================
 * NO HARDWARE COUPLING
 * ============================================================================
 *
 * This file MUST remain independent of:
 *
 *     hardware/
 *     scheduling/
 *     optimization/
 *     ZQN
 *     QEC
 *     resilience
 *
 * at the grammar dependency level.
 *
 * Those subsystems consume semantic representations produced downstream.
 *
 * ============================================================================
 */

parser grammar QuantumClassical;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. HYBRID DOMAIN ROOT
 * ============================================================================
 *
 * Stable entry point for quantum/classical interoperability.
 *
 * A hybrid construct can represent:
 *
 *     a hybrid region;
 *     a quantum invocation;
 *     a classical-to-quantum binding;
 *     a quantum-to-classical binding;
 *     a dynamic control region;
 *     a conversion;
 *     a synchronization boundary;
 *     a hybrid requirement/capability.
 *
 * ============================================================================
 */

quantumClassicalConstruct
    : hybridComputationRegion
    | quantumInvocationFromClassical
    | classicalBindingFromQuantum
    | classicalControlOfQuantum
    | quantumParameterBinding
    | hybridValueBinding
    | quantumClassicalConversion
    | quantumClassicalSynchronization
    | hybridRequirement
    | hybridCapability
    ;


/* ============================================================================
 * 2. HYBRID COMPUTATION REGION
 * ============================================================================
 *
 * A hybrid region contains canonical Zamani statements.
 *
 * The grammar intentionally does not duplicate statement syntax.
 *
 * Quantum-specific statements are accepted through the quantum grammar
 * integration boundary.
 *
 * ============================================================================
 */

hybridComputationRegion
    : K_HYBRID
      LBRACE
      hybridStatement*
      RBRACE
    ;


hybridStatement
    : quantumClassicalBoundaryStatement
    | statement
    ;


quantumClassicalBoundaryStatement
    : quantumInvocationFromClassical
    | classicalBindingFromQuantum
    | classicalControlOfQuantum
    | quantumParameterBinding
    | hybridValueBinding
    | quantumClassicalConversion
    | quantumClassicalSynchronization
    ;


/* ============================================================================
 * 3. CLASSICAL-TO-QUANTUM INVOCATION
 * ============================================================================
 *
 * Classical code may invoke a quantum operation/circuit.
 *
 * Example:
 *
 *     quantum_result = quantum_call(circuit, theta);
 *
 * The actual callable semantics remain owned by the general expression and
 * function systems.
 *
 * This rule represents the explicit hybrid boundary.
 *
 * ============================================================================
 */

quantumInvocationFromClassical
    : K_QUANTUM
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      quantumInvocationTarget?
      SEMI
    ;


quantumInvocationTarget
    : K_RETURNING
      expression
    ;


/* ============================================================================
 * 4. QUANTUM-TO-CLASSICAL BINDING
 * ============================================================================
 *
 * A quantum result can become a classical binding.
 *
 * Example:
 *
 *     let result = measure(q);
 *
 * The semantic layer determines the resulting type.
 *
 * ============================================================================
 */

classicalBindingFromQuantum
    : quantumResultBinding
    ;


quantumResultBinding
    : quantumResultSource
      quantumResultBindingOperator
      expression
      SEMI
    ;


quantumResultBindingOperator
    : ASSIGN
    ;


quantumResultSource
    : K_MEASURE
      LPAREN
      quantumMeasurementInput
      RPAREN
    | K_OBSERVE
      LPAREN
      expression
      RPAREN
    | K_READ
      LPAREN
      expression
      RPAREN
    ;


quantumMeasurementInput
    : quantumTargetList
    | expression
    ;


/* ============================================================================
 * 5. MEASUREMENT RESULT REFERENCE
 * ============================================================================
 *
 * A measurement result may be referenced by subsequent classical computation.
 *
 * The identifier resolution belongs to semantic analysis.
 *
 * ============================================================================
 */

measurementResultReference
    : K_MEASUREMENT
      LPAREN
      expression
      RPAREN
    | K_RESULT
      LPAREN
      expression
      RPAREN
    | IDENTIFIER
    ;


/* ============================================================================
 * 6. CLASSICAL CONTROL OF QUANTUM COMPUTATION
 * ============================================================================
 *
 * Classical values may control whether quantum computation occurs.
 *
 * This is the central dynamic-circuit boundary.
 *
 * Example:
 *
 *     if result {
 *         quantum_call(...);
 *     }
 *
 * The general condition semantics remain owned by the expression/type/effect
 * systems.
 *
 * ============================================================================
 */

classicalControlOfQuantum
    : K_IF
      classicalControlExpression
      quantumControlBlock
      quantumClassicalElseClause?
    ;


classicalControlExpression
    : expression
    ;


quantumControlBlock
    : block
    ;


quantumClassicalElseClause
    : K_ELSE
      quantumControlBlock
    ;


/* ============================================================================
 * 7. CLASSICAL CONTROLLED QUANTUM OPERATION
 * ============================================================================
 *
 * Explicit operation-level dynamic control.
 *
 * Example:
 *
 *     when result apply X to q;
 *
 * The condition remains a canonical expression.
 *
 * ============================================================================
 */

classicalControlledQuantumOperation
    : K_WHEN
      classicalControlExpression
      K_APPLY
      quantumOperation
      quantumOperationTargetClause
      SEMI
    ;


/* ============================================================================
 * 8. CLASSICAL CONDITION FROM MEASUREMENT
 * ============================================================================
 *
 * Explicit syntax for conditions derived from measurement.
 *
 * This is syntactic only; semantic analysis validates that the referenced
 * value is actually measurement-derived or otherwise legally usable.
 *
 * ============================================================================
 */

measurementControlledQuantumOperation
    : K_IF
      measurementResultReference
      K_APPLY
      quantumOperation
      quantumOperationTargetClause
      SEMI
    ;


/* ============================================================================
 * 9. CLASSICAL PARAMETER BINDING
 * ============================================================================
 *
 * A classical expression supplies a quantum operation parameter.
 *
 * Example:
 *
 *     apply RY(theta) to q;
 *
 * The expression grammar owns theta and arithmetic syntax.
 *
 * ============================================================================
 */

quantumParameterBinding
    : quantumOperationReference
      LPAREN
      quantumParameterArgumentList?
      RPAREN
      quantumOperationTargetClause
      SEMI
    ;


quantumParameterArgumentList
    : quantumParameterArgument
      (COMMA quantumParameterArgument)*
    ;


quantumParameterArgument
    : expression
    ;


/* ============================================================================
 * 10. HYBRID VALUE BINDING
 * ============================================================================
 *
 * Explicit source-level value flow between classical and quantum semantic
 * domains.
 *
 * ============================================================================
 */

hybridValueBinding
    : hybridValueDeclaration
    | hybridValueAssignment
    ;


hybridValueDeclaration
    : K_LET
      IDENTIFIER
      hybridTypeAnnotation?
      ASSIGN
      hybridValueExpression
      SEMI
    ;


hybridValueAssignment
    : IDENTIFIER
      ASSIGN
      hybridValueExpression
      SEMI
    ;


hybridTypeAnnotation
    : COLON
      typeExpression
    ;


hybridValueExpression
    : expression
    | quantumResultExpression
    ;


quantumResultExpression
    : quantumResultSource
    ;


/* ============================================================================
 * 11. EXPLICIT QUANTUM RESULT
 * ============================================================================
 *
 * Provides an expression-level boundary for quantum results.
 *
 * This does not define the result's representation.
 *
 * ============================================================================
 */

quantumResult
    : quantumMeasurementResult
    | quantumObservationResult
    | quantumExecutionResult
    ;


quantumMeasurementResult
    : K_MEASURE
      LPAREN
      quantumMeasurementInput
      RPAREN
    ;


quantumObservationResult
    : K_OBSERVE
      LPAREN
      expression
      RPAREN
    ;


quantumExecutionResult
    : K_RESULT
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 12. QUANTUM-CLASSICAL CONVERSION
 * ============================================================================
 *
 * Explicit conversion is available when a semantic boundary requires it.
 *
 * The conversion itself does not decide:
 *
 *     width;
 *     encoding;
 *     precision;
 *     physical representation;
 *     machine representation.
 *
 * ============================================================================
 */

quantumClassicalConversion
    : K_CONVERT
      LPAREN
      quantumClassicalConversionSource
      K_TO
      typeExpression
      RPAREN
    ;


quantumClassicalConversionSource
    : expression
    | quantumResult
    ;


/* ============================================================================
 * 13. CLASSICAL VALUE INTO QUANTUM VALUE
 * ============================================================================
 *
 * A classical value can initialize or parameterize quantum semantics.
 *
 * This rule does not prescribe physical initialization.
 *
 * ============================================================================
 */

classicalToQuantumValue
    : K_FROM
      expression
    ;


quantumValueFromClassical
    : quantumValueConstructor
      LPAREN
      classicalValueArgumentList?
      RPAREN
    ;


quantumValueConstructor
    : qualifiedName
    ;


classicalValueArgumentList
    : expression
      (COMMA expression)*
    ;


/* ============================================================================
 * 14. QUANTUM VALUE INTO CLASSICAL VALUE
 * ============================================================================
 *
 * Explicit semantic boundary for values exposed to classical computation.
 *
 * ============================================================================
 */

quantumToClassicalValue
    : K_TO
      K_CLASSICAL
      LPAREN
      quantumResult
      RPAREN
    ;


/* ============================================================================
 * 15. HYBRID EXPRESSION
 * ============================================================================
 *
 * A hybrid expression is still a normal Zamani expression syntactically.
 *
 * Domain membership is established semantically.
 *
 * ============================================================================
 */

hybridExpression
    : expression
    ;


hybridCondition
    : expression
    ;


hybridArgument
    : expression
    ;


/* ============================================================================
 * 16. QUANTUM OPERATION WITH CLASSICAL CONDITION
 * ============================================================================
 *
 * Explicit operation-level condition.
 *
 * ============================================================================
 */

conditionalQuantumOperation
    : quantumConditionClause
      quantumOperationStatement
    ;


quantumConditionClause
    : K_WHEN
      expression
    ;


/* ============================================================================
 * 17. QUANTUM OPERATION WITH CLASSICAL PARAMETERS
 * ============================================================================
 *
 * This is intentionally an integration boundary rather than a duplicate
 * operation grammar.
 *
 * ============================================================================
 */

parameterizedQuantumOperation
    : quantumOperationExpression
      quantumClassicalParameterClause?
      quantumOperationTargetClause
      SEMI
    ;


quantumClassicalParameterClause
    : LPAREN
      quantumParameterArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 18. CLASSICAL LOOP AROUND QUANTUM COMPUTATION
 * ============================================================================
 *
 * Classical iteration can surround quantum execution.
 *
 * The loop itself remains owned by statements/loops.g4.
 *
 * This rule only establishes a hybrid semantic boundary.
 *
 * ============================================================================
 */

classicalLoopWithQuantumBody
    : K_FOR
      IDENTIFIER
      K_IN
      expression
      quantumClassicalLoopBody
    | K_WHILE
      expression
      quantumClassicalLoopBody
    ;


quantumClassicalLoopBody
    : block
    ;


/* ============================================================================
 * 19. CLASSICAL REDUCTION OVER QUANTUM RESULTS
 * ============================================================================
 *
 * Reduction syntax remains semantic and does not prescribe a fixed number of
 * iterations, measurements, qubits, or results.
 *
 * ============================================================================
 */

quantumResultReduction
    : K_REDUCE
      IDENTIFIER
      K_IN
      quantumResultExpression
      K_WITH
      expression
      SEMI
    ;


/* ============================================================================
 * 20. CLASSICAL FUNCTION AROUND QUANTUM COMPUTATION
 * ============================================================================
 *
 * General function declaration syntax remains owned by functions/.
 *
 * This boundary represents invocation/composition only.
 *
 * ============================================================================
 */

hybridFunctionCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 21. CLASSICAL COMPUTATION FROM QUANTUM RESULT
 * ============================================================================
 *
 * Any canonical expression may consume a quantum-derived classical value.
 *
 * ============================================================================
 */

classicalComputationFromQuantum
    : expression
    ;


/* ============================================================================
 * 22. QUANTUM COMPUTATION FROM CLASSICAL RESULT
 * ============================================================================
 *
 * Any canonical expression may provide a value consumed by quantum semantics.
 *
 * ============================================================================
 */

quantumComputationFromClassical
    : expression
    ;


/* ============================================================================
 * 23. HYBRID SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization is a semantic dependency boundary.
 *
 * It does NOT specify:
 *
 *     hardware barriers;
 *     pulse barriers;
 *     fixed delays;
 *     CPU fences;
 *     network barriers;
 *     scheduler implementation.
 *
 * ============================================================================
 */

quantumClassicalSynchronization
    : K_SYNCHRONIZE
      hybridSynchronizationTarget?
      SEMI
    ;


hybridSynchronizationTarget
    : expression
    ;


/* ============================================================================
 * 24. EXPLICIT WAIT FOR QUANTUM RESULT
 * ============================================================================
 *
 * This is a semantic wait/dependency boundary.
 *
 * Runtime/scheduling layers determine its realization.
 *
 * ============================================================================
 */

waitForQuantumResult
    : K_AWAIT
      quantumResultExpression
      SEMI
    ;


/* ============================================================================
 * 25. ASYNCHRONOUS QUANTUM INVOCATION
 * ============================================================================
 *
 * No execution resource or concurrency model is hard-coded.
 *
 * ============================================================================
 */

asyncQuantumInvocation
    : K_ASYNC
      K_QUANTUM
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMI
    ;


/* ============================================================================
 * 26. HYBRID FUTURE / HANDLE
 * ============================================================================
 *
 * A handle represents a source-level future result.
 *
 * The runtime determines the concrete representation.
 *
 * ============================================================================
 */

quantumFutureExpression
    : K_FUTURE
      LPAREN
      quantumInvocationExpression
      RPAREN
    ;


quantumInvocationExpression
    : K_QUANTUM
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 27. HYBRID RESULT COLLECTION
 * ============================================================================
 *
 * Result collection remains abstract.
 *
 * No fixed result count is encoded.
 *
 * ============================================================================
 */

quantumResultCollection
    : K_COLLECT
      LPAREN
      quantumResultExpression
      RPAREN
    ;


/* ============================================================================
 * 28. HYBRID CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Capability expressions describe semantic requirements, not devices.
 *
 * Example conceptually:
 *
 *     requires hybrid;
 *     requires quantum;
 *     requires dynamic_control;
 *
 * The semantic capability system determines legality.
 *
 * ============================================================================
 */

hybridRequirement
    : K_REQUIRES
      hybridRequirementExpression
      SEMI
    ;


hybridRequirementExpression
    : expression
    ;


/* ============================================================================
 * 29. HYBRID CAPABILITY DECLARATION
 * ============================================================================
 */

hybridCapability
    : K_CAPABILITY
      hybridCapabilityExpression
      SEMI
    ;


hybridCapabilityExpression
    : expression
    ;


/* ============================================================================
 * 30. HYBRID RESOURCE INTENT
 * ============================================================================
 *
 * Resource requirements remain abstract.
 *
 * The source may express intent without identifying hardware.
 *
 * ============================================================================
 */

hybridResourceRequirement
    : K_REQUIRES
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 31. HYBRID CONSTRAINT
 * ============================================================================
 *
 * Constraints are semantic constraints.
 *
 * They are not machine constants.
 *
 * ============================================================================
 */

hybridConstraint
    : K_CONSTRAINT
      expression
      SEMI
    ;


/* ============================================================================
 * 32. HYBRID PREFERENCE
 * ============================================================================
 *
 * Preferences are non-mandatory implementation guidance.
 *
 * A preference MUST NOT silently become a semantic requirement.
 *
 * ============================================================================
 */

hybridPreference
    : K_PREFER
      expression
      SEMI
    ;


/* ============================================================================
 * 33. HYBRID HINT
 * ============================================================================
 *
 * Hints do not alter program semantics.
 *
 * ============================================================================
 */

hybridHint
    : K_HINT
      expression
      SEMI
    ;


/* ============================================================================
 * 34. CLASSICAL FEEDBACK TO QUANTUM COMPUTATION
 * ============================================================================
 *
 * Feedback is a first-class hybrid semantic relationship.
 *
 * ============================================================================
 */

quantumClassicalFeedback
    : K_FEEDBACK
      expression
      K_TO
      quantumFeedbackTarget
      SEMI
    ;


quantumFeedbackTarget
    : quantumOperationReference
    | qualifiedName
    ;


/* ============================================================================
 * 35. ITERATIVE HYBRID COMPUTATION
 * ============================================================================
 *
 * Supports algorithms where classical computation repeatedly updates quantum
 * parameters.
 *
 * The iteration count is an expression rather than a grammar-level constant.
 *
 * ============================================================================
 */

hybridIteration
    : K_ITERATE
      expression
      K_WITH
      hybridIterationBody
    ;


hybridIterationBody
    : block
    ;


/* ============================================================================
 * 36. VARIATIONAL / PARAMETER UPDATE BOUNDARY
 * ============================================================================
 *
 * The grammar does not implement an optimization algorithm.
 *
 * Optimization belongs to optimization/ and downstream compiler layers.
 *
 * ============================================================================
 */

quantumParameterUpdate
    : K_UPDATE
      expression
      K_FROM
      expression
      SEMI
    ;


/* ============================================================================
 * 37. MEASUREMENT-DRIVEN CONTROL REGION
 * ============================================================================
 *
 * Explicit dynamic quantum control region.
 *
 * ============================================================================
 */

measurementDrivenQuantumRegion
    : K_ON
      measurementResultReference
      quantumControlBlock
    ;


/* ============================================================================
 * 38. CLASSICAL POST-PROCESSING REGION
 * ============================================================================
 *
 * Quantum execution may produce classical results which are then processed by
 * ordinary Zamani computation.
 *
 * ============================================================================
 */

quantumPostProcessingRegion
    : K_POSTPROCESS
      quantumResultExpression
      K_WITH
      block
    ;


/* ============================================================================
 * 39. HYBRID PIPELINE
 * ============================================================================
 *
 * A pipeline is an ordered semantic composition.
 *
 * No machine pipeline is implied.
 *
 * ============================================================================
 */

hybridPipeline
    : K_PIPELINE
      LBRACE
      hybridPipelineStage*
      RBRACE
    ;


hybridPipelineStage
    : expression
      SEMI
    ;


/* ============================================================================
 * 40. HYBRID STAGE TRANSITION
 * ============================================================================
 *
 * Explicit source-level transition between classical and quantum stages.
 *
 * ============================================================================
 */

hybridStageTransition
    : K_CLASSICAL
      K_TO
      K_QUANTUM
      SEMI
    | K_QUANTUM
      K_TO
      K_CLASSICAL
      SEMI
    ;


/* ============================================================================
 * 41. DOMAIN ANNOTATION
 * ============================================================================
 *
 * Domain annotations are metadata.
 *
 * They MUST NOT directly choose hardware.
 *
 * ============================================================================
 */

hybridDomainAnnotation
    : AT
      IDENTIFIER
      LPAREN
      hybridDomainAnnotationValue?
      RPAREN
    ;


hybridDomainAnnotationValue
    : expression
    ;


/* ============================================================================
 * 42. HYBRID EFFECT BOUNDARY
 * ============================================================================
 *
 * Effects are owned by effects/.
 *
 * This grammar only provides a syntactic boundary for hybrid code that is
 * explicitly annotated with effects.
 *
 * ============================================================================
 */

hybridEffectBoundary
    : K_WITH
      K_EFFECTS
      LBRACE
      hybridEffectName
      (COMMA hybridEffectName)*
      RBRACE
    ;


hybridEffectName
    : qualifiedName
    ;


/* ============================================================================
 * 43. CLASSICAL-QUANTUM TYPE BOUNDARY
 * ============================================================================
 *
 * Type syntax remains owned by types/.
 *
 * This rule exists only for hybrid declarations that require an explicit
 * source-level type.
 *
 * ============================================================================
 */

hybridType
    : typeExpression
    ;


/* ============================================================================
 * 44. HYBRID GENERIC BOUNDARY
 * ============================================================================
 *
 * Generic syntax remains owned by the generic/type/function grammar.
 *
 * ============================================================================
 */

hybridGenericBinding
    : genericParameters
    ;


/* ============================================================================
 * 45. QUANTUM VALUE PARAMETER
 * ============================================================================
 *
 * A quantum parameter can be a classical expression.
 *
 * ============================================================================
 */

quantumClassicalParameter
    : expression
    ;


/* ============================================================================
 * 46. CLASSICAL CONDITION WITH QUANTUM RESULT
 * ============================================================================
 *
 * This provides an explicit syntactic boundary while leaving legality to
 * semantic analysis.
 *
 * ============================================================================
 */

quantumResultCondition
    : measurementResultReference
    | quantumResultExpression
    ;


/* ============================================================================
 * 47. QUANTUM OPERATION CONDITION
 * ============================================================================
 */

quantumConditionalExecution
    : quantumResultCondition
      K_THEN
      quantumControlBlock
    ;


/* ============================================================================
 * 48. HYBRID ASSERTION
 * ============================================================================
 *
 * Assertions are semantic checks.
 *
 * ============================================================================
 */

hybridAssertion
    : K_ASSERT
      expression
      SEMI
    ;


/* ============================================================================
 * 49. HYBRID ERROR HANDLING
 * ============================================================================
 *
 * Runtime failure/recovery policy belongs downstream.
 *
 * This rule only permits ordinary language-level handling around hybrid code.
 *
 * ============================================================================
 */

hybridTryRegion
    : K_TRY
      block
      hybridCatchClause*
      hybridFinallyClause?
    ;


hybridCatchClause
    : K_CATCH
      LPAREN
      IDENTIFIER
      hybridTypeAnnotation?
      RPAREN
      block
    ;


hybridFinallyClause
    : K_FINALLY
      block
    ;


/* ============================================================================
 * 50. HYBRID RETURN
 * ============================================================================
 *
 * A hybrid computation may return a classical or quantum-derived result.
 *
 * The function/type system determines legality.
 *
 * ============================================================================
 */

hybridReturn
    : K_RETURN
      hybridReturnExpression?
      SEMI
    ;


hybridReturnExpression
    : expression
    | quantumResultExpression
    ;


/* ============================================================================
 * 51. HYBRID DOMAIN-NEUTRAL VALUE
 * ============================================================================
 *
 * Stable extension point for future computational domains.
 *
 * A future domain must not require modification of existing quantum/classical
 * value semantics merely because it interoperates with them.
 *
 * ============================================================================
 */

hybridDomainValue
    : expression
    ;


/* ============================================================================
 * 52. EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future hybrid constructs can be introduced through ordinary Zamani
 * expressions/statements without embedding vendor-specific syntax here.
 *
 * ============================================================================
 */

hybridExtensionStatement
    : expression
      SEMI
    ;