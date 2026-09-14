/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/classical-quantum.g4
 *
 * Role:
 *     Canonical source-syntax boundary between classical and quantum
 *     computation.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     safe Rust only
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This grammar component defines only the syntax required to express
 * interactions between:
 *
 *     classical computation
 *
 * and:
 *
 *     quantum computation.
 *
 * It deliberately does NOT define:
 *
 *     - a second expression language;
 *     - a second type system;
 *     - a second classical IR;
 *     - a second quantum IR;
 *     - hardware allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - device discovery.
 *
 * The architecture is:
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
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     classical semantics       quantum semantics
 *          |                          |
 *          v                          v
 *     canonical classical       quantum::ir
 *          semantic model              |
 *          |                          |
 *          +------------+-------------+
 *                       |
 *                       v
 *                  optimization
 *                       |
 *                       v
 *                    routing
 *                       |
 *                       v
 *                  scheduling
 *                       |
 *                       v
 *                      ZQN
 *                       |
 *                       v
 *                      QEC
 *                       |
 *                       v
 *                  resilience
 *                       |
 *                       v
 *                  hardware HAL
 *                       |
 *                       v
 *                    runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - classical/quantum boundary syntax;
 *     - hybrid computation regions;
 *     - classical invocation of quantum computations;
 *     - quantum invocation from classical computation;
 *     - classical values supplied to quantum operations;
 *     - quantum results exposed to classical computation;
 *     - measurement-result value flow;
 *     - classical control over quantum execution;
 *     - quantum execution producing classical results;
 *     - explicit hybrid conversion syntax;
 *     - explicit hybrid synchronization syntax;
 *     - hybrid requirements;
 *     - hybrid capabilities;
 *     - hybrid value bindings;
 *     - source-level domain crossing.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - literals;
 *     - expression precedence;
 *     - general expressions;
 *     - general statements;
 *     - types;
 *     - generic types;
 *     - functions;
 *     - modules;
 *     - quantum gates;
 *     - quantum registers;
 *     - quantum states;
 *     - quantum operation definitions;
 *     - measurement implementation;
 *     - QEC algorithms;
 *     - ZQN semantics;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware topology;
 *     - device identifiers;
 *     - physical allocation;
 *     - runtime behavior;
 *     - classical IR;
 *     - quantum::ir.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar expresses semantic relationships, not machine realization.
 *
 * A hybrid program must remain portable across:
 *
 *     CPU + QPU
 *     CPU + simulator
 *     GPU + QPU
 *     accelerator + QPU
 *     embedded controller + quantum processor
 *     distributed classical + distributed quantum
 *     future heterogeneous machines
 *
 * without requiring source changes merely because the available machine
 * changes.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file intentionally contains NO fixed limits.
 *
 * It does NOT define:
 *
 *     MAX_QUBITS
 *     MAX_BITS
 *     MAX_PARAMETERS
 *     MAX_RESULTS
 *     MAX_BRANCHES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_ITERATIONS
 *     MAX_OPERATIONS
 *
 * Any implementation/resource limit belongs downstream to the appropriate
 * parser, semantic-analysis, compiler, scheduler, runtime, or hardware
 * capability policy.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Syntax does not decide:
 *
 *     - whether a value is classical or quantum;
 *     - whether a conversion is valid;
 *     - whether a measurement result is available;
 *     - whether a condition is measurement-dependent;
 *     - whether an operation is compile-time or runtime;
 *     - whether synchronization is necessary;
 *     - whether a target supports a hybrid operation;
 *     - whether a parameter is host-side or device-side;
 *     - whether a value is differentiable;
 *     - whether execution is deterministic.
 *
 * Semantic analysis owns those decisions.
 *
 * ============================================================================
 * IR BOUNDARY
 * ============================================================================
 *
 * This file NEVER creates or defines an IR.
 *
 * Quantum constructs eventually lower to:
 *
 *     quantum::ir
 *
 * Classical constructs eventually lower to the repository's canonical
 * classical semantic representation.
 *
 * Hybrid relationships are represented in the frontend semantic model and
 * subsequently lowered into the appropriate canonical representations.
 *
 * ============================================================================
 */

parser grammar ClassicalQuantum;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC HYBRID CONSTRUCT
 * ============================================================================
 *
 * This is the stable entry point imported by the canonical Zamani parser.
 *
 * The canonical parser may use:
 *
 *     import ClassicalQuantum;
 *
 * and expose:
 *
 *     classicalQuantumConstruct
 *
 * as part of its statement/declaration/expression integration layer.
 *
 * ============================================================================
 */

classicalQuantumConstruct
    : hybridRegion
    | quantumCall
    | classicalQuantumCall
    | quantumResultBinding
    | quantumValueBinding
    | quantumControlledStatement
    | quantumParameterBinding
    | hybridConversion
    | hybridSynchronization
    | hybridRequirement
    | hybridCapability
    ;


/* ============================================================================
 * 2. HYBRID REGION
 * ============================================================================
 *
 * A hybrid region explicitly permits both classical and quantum operations
 * within one source-level semantic region.
 *
 * It does not introduce a new statement language.
 *
 * ============================================================================
 */

hybridRegion
    : K_HYBRID
      LBRACE
      hybridRegionItem*
      RBRACE
    ;


hybridRegionItem
    : hybridBoundaryStatement
    | statement
    ;


/*
 * The boundary statement alternatives are deliberately explicit so that the
 * frontend can distinguish a domain-crossing construct from an ordinary
 * classical statement without changing the general statement grammar.
 */
hybridBoundaryStatement
    : quantumCall
    | classicalQuantumCall
    | quantumResultBinding
    | quantumValueBinding
    | quantumControlledStatement
    | quantumParameterBinding
    | hybridConversion
    | hybridSynchronization
    ;


/* ============================================================================
 * 3. CLASSICAL -> QUANTUM INVOCATION
 * ============================================================================
 *
 * Classical computation may invoke a quantum callable.
 *
 * Example:
 *
 *     quantum call circuit(theta, input);
 *
 * The callable's declaration and type remain owned elsewhere.
 *
 * ============================================================================
 */

quantumCall
    : K_QUANTUM
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      quantumCallResultClause?
      SEMI
    ;


quantumCallResultClause
    : K_RETURNING
      expression
    ;


/* ============================================================================
 * 4. QUANTUM -> CLASSICAL INVOCATION
 * ============================================================================
 *
 * A quantum computation may explicitly expose a classical result.
 *
 * The syntax describes the boundary only.
 *
 * ============================================================================
 */

classicalQuantumCall
    : K_CLASSICAL
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMI
    ;


/* ============================================================================
 * 5. QUANTUM RESULT BINDING
 * ============================================================================
 *
 * A quantum result can be bound to a classical source-level name.
 *
 * Example:
 *
 *     let result = measure(q);
 *
 * The semantic/type layer determines the actual result type.
 *
 * ============================================================================
 */

quantumResultBinding
    : K_LET
      IDENTIFIER
      typeAnnotation?
      ASSIGN
      quantumResultExpression
      SEMI
    ;


quantumResultExpression
    : measurementExpression
    | observationExpression
    | quantumReadExpression
    | quantumCallExpression
    ;


measurementExpression
    : K_MEASURE
      LPAREN
      quantumMeasurementSource
      RPAREN
    ;


observationExpression
    : K_OBSERVE
      LPAREN
      expression
      RPAREN
    ;


quantumReadExpression
    : K_READ
      LPAREN
      expression
      RPAREN
    ;


quantumCallExpression
    : K_QUANTUM
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


quantumMeasurementSource
    : quantumTargetList
    | expression
    ;


/* ============================================================================
 * 6. QUANTUM VALUE BINDING
 * ============================================================================
 *
 * Provides explicit source-level binding for a value that crosses the
 * classical/quantum boundary.
 *
 * This rule does not determine the underlying representation.
 *
 * ============================================================================
 */

quantumValueBinding
    : quantumValueDeclaration
    | quantumValueAssignment
    ;


quantumValueDeclaration
    : K_LET
      IDENTIFIER
      typeAnnotation?
      ASSIGN
      quantumValueExpression
      SEMI
    ;


quantumValueAssignment
    : IDENTIFIER
      ASSIGN
      quantumValueExpression
      SEMI
    ;


quantumValueExpression
    : expression
    | quantumResultExpression
    ;


/* ============================================================================
 * 7. MEASUREMENT RESULT REFERENCE
 * ============================================================================
 *
 * Measurement results become ordinary semantic values after the measurement
 * boundary. Their exact type and lifetime are determined semantically.
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
    ;


/* ============================================================================
 * 8. CLASSICAL CONTROL OF QUANTUM EXECUTION
 * ============================================================================
 *
 * Classical control may determine whether quantum computation executes.
 *
 * Example:
 *
 *     if result {
 *         quantum call circuit();
 *     }
 *
 * The condition itself remains an ordinary Zamani expression.
 *
 * ============================================================================
 */

quantumControlledStatement
    : K_IF
      expression
      block
      quantumElseClause?
    ;


quantumElseClause
    : K_ELSE
      block
    ;


/* ============================================================================
 * 9. EXPLICIT MEASUREMENT-CONTROLLED OPERATION
 * ============================================================================
 *
 * This form makes the dynamic-circuit boundary explicit.
 *
 * The grammar does not require the condition to be physically implemented
 * through a particular control mechanism.
 *
 * ============================================================================
 */

measurementControlledQuantumOperation
    : K_IF
      measurementResultReference
      K_APPLY
      quantumOperationReference
      quantumOperationArguments?
      quantumOperationTargetClause
      SEMI
    ;


/* ============================================================================
 * 10. QUANTUM PARAMETER BINDING
 * ============================================================================
 *
 * Quantum operation parameters may be arbitrary classical expressions.
 *
 * Example:
 *
 *     apply rotate(theta + delta) to q;
 *
 * The expression grammar owns:
 *
 *     theta + delta
 *
 * The quantum grammar owns the quantum operation boundary.
 *
 * ============================================================================
 */

quantumParameterBinding
    : K_APPLY
      quantumOperationReference
      quantumOperationArguments?
      quantumOperationTargetClause
      SEMI
    ;


quantumOperationArguments
    : LPAREN
      quantumParameterArgumentList?
      RPAREN
    ;


quantumParameterArgumentList
    : quantumParameterArgument
      (COMMA quantumParameterArgument)*
    ;


quantumParameterArgument
    : expression
    ;


quantumOperationTargetClause
    : K_TO
      quantumTargetList
    ;


quantumOperationReference
    : qualifiedName
    ;


quantumTargetList
    : quantumTarget
      (COMMA quantumTarget)*
    ;


quantumTarget
    : expression
    ;


/* ============================================================================
 * 11. EXPLICIT HYBRID CONVERSION
 * ============================================================================
 *
 * Explicit conversion syntax allows the source program to state that a value
 * crosses a semantic domain boundary.
 *
 * Example:
 *
 *     convert(value) to quantum;
 *     convert(value) to classical;
 *
 * The compiler decides whether the conversion is legal.
 *
 * ============================================================================
 */

hybridConversion
    : K_CONVERT
      LPAREN
      expression
      RPAREN
      K_TO
      hybridDomain
      SEMI
    ;


hybridDomain
    : K_CLASSICAL
    | K_QUANTUM
    ;


/* ============================================================================
 * 12. HYBRID SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization expresses a semantic boundary without encoding a hardware
 * timing model.
 *
 * It does NOT specify:
 *
 *     clocks;
 *     pulse duration;
 *     latency;
 *     queue time;
 *     device synchronization primitives.
 *
 * Those belong downstream.
 *
 * ============================================================================
 */

hybridSynchronization
    : K_SYNCHRONIZE
      synchronizationScope?
      SEMI
    ;


synchronizationScope
    : LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 13. HYBRID REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe semantic/resource capabilities needed by a program.
 *
 * They do not select a physical device.
 *
 * Example:
 *
 *     requires quantum;
 *     requires quantum + classical;
 *
 * ============================================================================
 */

hybridRequirement
    : K_REQUIRES
      hybridRequirementExpression
      SEMI
    ;


hybridRequirementExpression
    : hybridRequirementTerm
      (PLUS hybridRequirementTerm)*
    ;


hybridRequirementTerm
    : qualifiedName
    | IDENTIFIER
    ;


/* ============================================================================
 * 14. HYBRID CAPABILITIES
 * ============================================================================
 *
 * Capability declarations describe what an execution context may provide.
 *
 * They do not perform hardware discovery.
 *
 * ============================================================================
 */

hybridCapability
    : K_CAPABILITY
      qualifiedName
      hybridCapabilityValue?
      SEMI
    ;


hybridCapabilityValue
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 15. HYBRID DOMAIN ANNOTATION
 * ============================================================================
 *
 * Allows source-level annotation of an expression without introducing a new
 * type system.
 *
 * ============================================================================
 */

hybridDomainAnnotation
    : AT
      hybridDomain
    ;


/* ============================================================================
 * 16. CLASSICAL VALUE SUPPLIED TO QUANTUM
 * ============================================================================
 *
 * This rule provides a named integration point for semantic analysis.
 *
 * It intentionally delegates the actual expression to the canonical
 * expression grammar.
 *
 * ============================================================================
 */

classicalToQuantumValue
    : expression
    ;


/* ============================================================================
 * 17. QUANTUM VALUE EXPOSED TO CLASSICAL
 * ============================================================================
 *
 * This is intentionally a syntactic wrapper around an existing quantum result
 * expression.
 *
 * ============================================================================
 */

quantumToClassicalValue
    : quantumResultExpression
    ;


/* ============================================================================
 * 18. HYBRID VALUE
 * ============================================================================
 *
 * Generic source-level hybrid value boundary.
 *
 * ============================================================================
 */

hybridValue
    : classicalToQuantumValue
    | quantumToClassicalValue
    ;


/* ============================================================================
 * 19. HYBRID ARGUMENT
 * ============================================================================
 *
 * Hybrid arguments remain ordinary expressions. This rule exists as an
 * integration point for semantic analysis and future domain-specific
 * diagnostics.
 *
 * ============================================================================
 */

hybridArgument
    : expression
    ;


/* ============================================================================
 * 20. HYBRID RETURN VALUE
 * ============================================================================
 *
 * Quantum results may be returned through normal language return semantics.
 *
 * The grammar does not create a second return system.
 *
 * ============================================================================
 */

hybridReturnValue
    : K_RETURN
      hybridValue
      SEMI
    ;


/* ============================================================================
 * 21. DOMAIN-EXPLICIT QUANTUM CALL
 * ============================================================================
 *
 * Optional explicit form for semantic tooling.
 *
 * ============================================================================
 */

domainQualifiedQuantumCall
    : K_QUANTUM
      COLONCOLON
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMI
    ;


/* ============================================================================
 * 22. DOMAIN-EXPLICIT CLASSICAL CALL
 * ============================================================================
 *
 * Optional explicit form for semantic tooling.
 *
 * ============================================================================
 */

domainQualifiedClassicalCall
    : K_CLASSICAL
      COLONCOLON
      K_CALL
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMI
    ;


/* ============================================================================
 * 23. HYBRID CALL
 * ============================================================================
 *
 * Generic boundary allowing the semantic layer to determine the actual
 * domain relationship.
 *
 * ============================================================================
 */

hybridCall
    : domainQualifiedQuantumCall
    | domainQualifiedClassicalCall
    | quantumCall
    | classicalQuantumCall
    ;


/* ============================================================================
 * 24. HYBRID BLOCK
 * ============================================================================
 *
 * Provides a stable semantic boundary for future frontend/AST construction.
 * The block itself remains the canonical Zamani block.
 *
 * ============================================================================
 */

hybridBlock
    : LBRACE
      hybridRegionItem*
      RBRACE
    ;


/* ============================================================================
 * 25. HYBRID EXPRESSION
 * ============================================================================
 *
 * This rule does not introduce new operators.
 *
 * ============================================================================
 */

hybridExpression
    : expression
    | quantumResultExpression
    ;


/* ============================================================================
 * 26. HYBRID CONDITION
 * ============================================================================
 *
 * Conditions remain ordinary Zamani expressions.
 *
 * Semantic analysis determines whether the expression is permitted to control
 * quantum execution.
 *
 * ============================================================================
 */

hybridCondition
    : expression
    ;


/* ============================================================================
 * 27. HYBRID TARGET
 * ============================================================================
 *
 * A semantic target expression is intentionally opaque at grammar level.
 *
 * It may eventually describe:
 *
 *     a quantum circuit;
 *     a logical operation;
 *     a callable;
 *     a symbolic computation;
 *     a future execution abstraction.
 *
 * ============================================================================
 */

hybridTarget
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 28. HYBRID DECLARATION
 * ============================================================================
 *
 * General integration point for frontend semantic analysis.
 *
 * ============================================================================
 */

hybridDeclaration
    : K_LET
      IDENTIFIER
      typeAnnotation?
      ASSIGN
      hybridExpression
      SEMI
    ;


/* ============================================================================
 * 29. HYBRID ASSIGNMENT
 * ============================================================================
 */

hybridAssignment
    : IDENTIFIER
      ASSIGN
      hybridExpression
      SEMI
    ;


/* ============================================================================
 * 30. HYBRID STATEMENT
 * ============================================================================
 *
 * Canonical integration point used by hybridRegion.
 *
 * ============================================================================
 */

hybridStatement
    : hybridBoundaryStatement
    | hybridDeclaration
    | hybridAssignment
    | hybridReturnValue
    | statement
    ;


/* ============================================================================
 * 31. SEMANTIC DOMAIN PAIR
 * ============================================================================
 *
 * Syntax-only representation of the two participating domains.
 *
 * ============================================================================
 */

hybridDomainPair
    : K_CLASSICAL
      K_AND
      K_QUANTUM
    | K_QUANTUM
      K_AND
      K_CLASSICAL
    ;


/* ============================================================================
 * 32. EXPLICIT HYBRID DECLARATION
 * ============================================================================
 */

hybridDeclarationWithDomain
    : K_HYBRID
      IDENTIFIER
      hybridDomainPair?
      ASSIGN
      hybridExpression
      SEMI
    ;


/* ============================================================================
 * 33. HYBRID REQUIREMENT WITH VALUE
 * ============================================================================
 *
 * Resource quantities, dimensions, capacities, and limits remain expressions.
 *
 * No fixed numerical grammar-level restriction is introduced.
 *
 * ============================================================================
 */

hybridRequirementWithValue
    : K_REQUIRES
      qualifiedName
      comparisonOperator
      expression
      SEMI
    ;


/* ============================================================================
 * 34. HYBRID PREFERENCE
 * ============================================================================
 *
 * Preferences are not requirements.
 *
 * The compiler/runtime may choose another implementation if the preference
 * cannot be satisfied while preserving program semantics.
 *
 * ============================================================================
 */

hybridPreference
    : K_PREFER
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 35. HYBRID CONSTRAINT
 * ============================================================================
 *
 * Constraints describe conditions that must remain true for a selected
 * realization.
 *
 * They do not describe a specific machine.
 *
 * ============================================================================
 */

hybridConstraint
    : K_CONSTRAIN
      expression
      SEMI
    ;


/* ============================================================================
 * 36. HYBRID HINT
 * ============================================================================
 *
 * Hints are non-binding implementation guidance.
 *
 * ============================================================================
 */

hybridHint
    : K_HINT
      qualifiedName
      hybridHintValue?
      SEMI
    ;


hybridHintValue
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 37. COMPLETE HYBRID ITEM
 * ============================================================================
 *
 * This rule is the broad integration surface used by frontend composition.
 *
 * ============================================================================
 */

hybridItem
    : hybridRegion
    | hybridStatement
    | hybridCall
    | hybridConversion
    | hybridSynchronization
    | hybridRequirement
    | hybridRequirementWithValue
    | hybridCapability
    | hybridPreference
    | hybridConstraint
    | hybridHint
    ;


/* ============================================================================
 * 38. INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar component assumes the canonical parser provides or imports:
 *
 *     statement
 *     block
 *     expression
 *     argumentList
 *     qualifiedName
 *     typeAnnotation
 *     typeExpression
 *     quantumOperation
 *     quantumOperationReference
 *     quantumOperationTargetClause
 *     quantumTargetList
 *     comparisonOperator
 *
 * The exact definitions remain owned by their respective grammar components.
 *
 * This file MUST NOT redefine them.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. NON-OWNERSHIP OF IR
 * ============================================================================
 *
 * No rule in this grammar creates:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumCircuit
 *     ClassicalInstruction
 *     HardwareDevice
 *     Schedule
 *     ResourceAllocation
 *
 * These belong downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. NON-OWNERSHIP OF HARDWARE
 * ============================================================================
 *
 * There are deliberately no grammar rules for:
 *
 *     device = ...
 *     qpu = ...
 *     gpu_count = ...
 *     cpu_count = ...
 *     qubit_count = ...
 *     topology = ...
 *
 * unless such constructs are separately defined by the hardware/resource
 * language and semantically interpreted downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. NON-OWNERSHIP OF SCHEDULING
 * ============================================================================
 *
 * No source construct here specifies:
 *
 *     physical start time;
 *     duration;
 *     pulse;
 *     hardware clock;
 *     queue position;
 *     resource reservation.
 *
 * Scheduling owns those concerns.
 *
 * ============================================================================
 */


/* ============================================================================
 * 42. DYNAMIC-CIRCUIT CONTRACT
 * ============================================================================
 *
 * A classical condition may depend on:
 *
 *     - ordinary classical values;
 *     - function results;
 *     - prior measurement results;
 *     - runtime data;
 *     - compile-time values.
 *
 * The grammar records the relationship.
 *
 * The semantic/compiler/scheduling/runtime pipeline determines:
 *
 *     dependency;
 *     availability;
 *     synchronization;
 *     placement;
 *     lowering;
 *     execution.
 *
 * ============================================================================
 */


/* ============================================================================
 * 43. MEASUREMENT CONTRACT
 * ============================================================================
 *
 * Measurement is a semantic boundary:
 *
 *     quantum computation
 *             |
 *             v
 *        measurement
 *             |
 *             v
 *     classical observable
 *
 * This grammar does not prescribe:
 *
 *     bit width;
 *     encoding;
 *     readout hardware;
 *     measurement latency;
 *     register layout.
 *
 * ============================================================================
 */


/* ============================================================================
 * 44. PARAMETER CONTRACT
 * ============================================================================
 *
 * Quantum parameters may originate from:
 *
 *     literals
 *     constants
 *     variables
 *     expressions
 *     function results
 *     measurement results
 *     compile-time computation
 *     runtime computation
 *
 * No distinction is hard-coded here between host/device/compile/runtime
 * values. That distinction belongs to semantic analysis and lowering.
 *
 * ============================================================================
 */


/* ============================================================================
 * 45. POCO-REAF CONTRACT
 * ============================================================================
 *
 * This component preserves:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * by making the source-level relationship between classical and quantum
 * computation independent of:
 *
 *     machine size;
 *     topology;
 *     device identity;
 *     vendor;
 *     backend;
 *     hardware generation;
 *     execution location.
 *
 * ============================================================================
 */


/* ============================================================================
 * 46. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing of the same token sequence must produce the same parse structure.
 *
 * Runtime nondeterminism is not a grammar concern.
 *
 * ============================================================================
 */


/* ============================================================================
 * 47. RESOURCE SAFETY
 * ============================================================================
 *
 * The grammar itself contains no unbounded host-side computation.
 *
 * Parser resource limits, if required for denial-of-service protection, must
 * be supplied through parser configuration rather than encoded as language
 * semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 48. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid classical syntax remains owned by the canonical classical
 * grammar.
 *
 * Existing valid quantum syntax remains owned by the quantum grammar.
 *
 * This file adds only the missing interoperability boundary.
 *
 * Migration from:
 *
 *     grammar/quantum/quantum-classical.g4
 *
 * must preserve valid source semantics. The old file must not remain a
 * competing canonical definition.
 *
 * ============================================================================
 */


/* ============================================================================
 * 49. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     classical -> quantum call
 *     quantum -> classical result
 *     measurement -> classical binding
 *     classical condition -> quantum operation
 *     classical parameter -> quantum operation
 *     quantum result -> classical expression
 *     explicit conversion
 *     synchronization
 *     hybrid region
 *     hybrid requirements
 *     hybrid capabilities
 *
 * Required negative tests:
 *
 *     malformed quantum call
 *     missing target
 *     malformed conversion
 *     malformed synchronization
 *     invalid hybrid binding syntax
 *     invalid argument structure
 *
 * Required scalability tests:
 *
 *     no fixed qubit count
 *     no fixed classical-bit count
 *     no fixed device count
 *     no fixed parameter count
 *     no fixed operation count
 *     no fixed branch count
 *
 * Required cross-domain tests:
 *
 *     classical + quantum
 *     classical + quantum + distributed
 *     classical + quantum + HDL
 *     classical + quantum + hardware
 *     classical + quantum + AI
 *     classical + quantum + HDL + hardware
 *
 * ============================================================================
 */


/* ============================================================================
 * 50. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     numeric machine limit;
 *     qubit limit;
 *     device limit;
 *     topology;
 *     hardware address;
 *     vendor identifier;
 *     backend identifier;
 *     fixed register width;
 *     fixed accelerator count.
 *
 * Any future addition violating this rule requires an explicit language
 * specification decision and must not be introduced as an implementation
 * convenience.
 *
 * ============================================================================
 */


/* ============================================================================
 * 51. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     1. It is imported by the canonical Zamani parser.
 *
 *     2. It introduces no duplicate lexer ownership.
 *
 *     3. It introduces no duplicate expression ownership.
 *
 *     4. It introduces no duplicate type ownership.
 *
 *     5. It introduces no duplicate quantum operation ownership.
 *
 *     6. It introduces no IR.
 *
 *     7. It introduces no hardware allocation.
 *
 *     8. It introduces no scheduling semantics.
 *
 *     9. It introduces no QEC implementation.
 *
 *    10. It introduces no ZQN implementation.
 *
 *    11. It has positive, negative, boundary, scalability and cross-domain
 *        tests.
 *
 *    12. The same source semantics remain valid across different target
 *        architectures.
 *
 *    13. No grammar-level machine-size limitation exists.
 *
 *    14. The frontend can lower quantum portions to quantum::ir.
 *
 *    15. Classical portions continue through the canonical classical
 *        semantic pipeline.
 *
 *    16. The component does not create a circular dependency on IR, runtime,
 *        hardware, scheduling, or optimization.
 *
 * ============================================================================
 */