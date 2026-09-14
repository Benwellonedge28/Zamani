/*
 * ============================================================================
 * Zamani Universal Quantum Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum.g4
 *
 * Role:
 *     Canonical reusable parser fragment for quantum source syntax.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantum source syntax;
 *   - quantum declarations;
 *   - quantum regions;
 *   - abstract qubit/register declarations;
 *   - quantum operation syntax;
 *   - operation modifiers;
 *   - controlled operations;
 *   - parameterized operations;
 *   - measurements;
 *   - reset;
 *   - barriers;
 *   - synchronization;
 *   - abstract quantum resource lifetime syntax;
 *   - quantum/classical control syntax;
 *   - logical/physical intent annotations;
 *   - observables;
 *   - entanglement intent;
 *   - quantum error/noise intent syntax;
 *   - quantum capability/requirement syntax;
 *   - quantum program composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - identifiers;
 *   - numeric/string literals;
 *   - AST implementation;
 *   - type checking;
 *   - quantum semantics;
 *   - quantum IR;
 *   - QEC algorithms;
 *   - QEC decoders;
 *   - ZQN noise models;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware discovery;
 *   - calibration;
 *   - topology;
 *   - backend selection;
 *   - simulator implementation;
 *   - runtime dispatch;
 *   - physical qubit allocation;
 *   - machine-size limits.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          +---- this quantum grammar
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     name/type/effect/capability/resource analysis
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +---- quantum::ir
 *          |
 *          +---- optimization
 *          |
 *          +---- routing
 *          |
 *          +---- scheduling
 *          |
 *          +---- ZQN
 *          |
 *          +---- QEC
 *          |
 *          +---- resilience
 *          |
 *          +---- hardware HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     execution
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Quantum source describes computational intent.
 *
 * It MUST NOT encode assumptions about:
 *
 *   - number of qubits;
 *   - number of quantum registers;
 *   - register width;
 *   - number of classical bits;
 *   - processor size;
 *   - QPU size;
 *   - topology;
 *   - connectivity;
 *   - vendor;
 *   - device identifier;
 *   - calibration;
 *   - pulse duration;
 *   - gate duration;
 *   - native gate set;
 *   - physical qubit identifier;
 *   - simulator size;
 *   - backend.
 *
 * Resource availability is resolved later.
 *
 * Therefore:
 *
 *     program semantics
 *          !=
 *     machine realization
 *
 * This is necessary for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This file is intentionally NOT:
 *
 *     grammar ZamaniQuantum;
 *
 * and does NOT declare:
 *
 *     lexer grammar ...
 *
 * It is imported into the canonical parser.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. TOP-LEVEL QUANTUM DECLARATION
 * ========================================================================== */

/*
 * Explicit quantum declaration.
 *
 * Supported forms include:
 *
 *     quantum {
 *         ...
 *     }
 *
 *     quantum circuit Bell {
 *         ...
 *     }
 *
 *     quantum circuit Algorithm<T> (...) where ... {
 *         ...
 *     }
 *
 * No physical resource count is encoded.
 */
quantumDeclaration
    : K_QUANTUM quantumDeclarationBody
    ;


quantumDeclarationBody
    : K_CIRCUIT identifier
      genericParameters?
      quantumParameterList?
      whereClause?
      blockExpression

    | blockExpression
    ;


/* ============================================================================
 * 2. QUANTUM PARAMETERS
 * ========================================================================== */

/*
 * Quantum parameters are source-level parameters.
 *
 * Their actual semantic type is resolved by the type system.
 *
 * Examples:
 *
 *     quantum circuit QFT(register: QubitRegister) { ... }
 *
 *     quantum circuit Rotation(theta: Angle) { ... }
 */
quantumParameterList
    : LPAREN parameterList? RPAREN
    ;


/* ============================================================================
 * 3. QUANTUM BLOCK
 * ========================================================================== */

quantumBlock
    : LBRACE quantumBlockElement* RBRACE
    ;


quantumBlockElement
    : attributes*
      quantumElement
    ;


quantumElement
    : quantumDeclarationStatement
    | quantumOperationStatement
    | quantumMeasurementStatement
    | quantumResetStatement
    | quantumBarrierStatement
    | quantumSynchronizationStatement
    | quantumAllocationStatement
    | quantumReleaseStatement
    | quantumObservationStatement
    | quantumEntanglementStatement
    | quantumConditionalStatement
    | quantumNoiseStatement
    | quantumRequirementStatement
    | quantumCapabilityStatement
    | quantumStatement
    | statement
    ;


/* ============================================================================
 * 4. GENERIC QUANTUM STATEMENT
 * ========================================================================== */

/*
 * This rule deliberately provides an extension boundary.
 *
 * Future quantum constructs should preferably be introduced through semantic
 * extensions rather than modifying the core grammar for every new operation.
 */
quantumStatement
    : expression SEMI
    ;


/* ============================================================================
 * 5. LOCAL QUANTUM DECLARATIONS
 * ========================================================================== */

quantumDeclarationStatement
    : quantumQubitDeclaration
    | quantumRegisterDeclaration
    | quantumStateDeclaration
    | quantumOperationDeclaration
    | quantumCircuitLocalDeclaration
    ;


/* ----------------------------------------------------------------------------
 * 5.1 Qubit declaration
 * ------------------------------------------------------------------------- */

/*
 * Examples:
 *
 *     qubit q;
 *     qubit q = |0>;
 *
 * The declaration does not specify physical hardware.
 */
quantumQubitDeclaration
    : K_QUBIT identifier
      quantumInitializer?
      SEMI
    ;


quantumInitializer
    : ASSIGN quantumStateExpression
    ;


/* ----------------------------------------------------------------------------
 * 5.2 Quantum register declaration
 * ------------------------------------------------------------------------- */

/*
 * A register may have:
 *
 *     - compile-time symbolic extent;
 *     - runtime extent;
 *     - an inferred extent;
 *     - an externally supplied extent.
 *
 * The grammar does not restrict the extent to a fixed integer.
 */
quantumRegisterDeclaration
    : K_QUBIT identifier
      LBRACK quantumExtentExpression RBRACK
      quantumInitializer?
      SEMI
    ;


quantumExtentExpression
    : expression
    ;


/* ----------------------------------------------------------------------------
 * 5.3 Quantum state declaration
 * ------------------------------------------------------------------------- */

quantumStateDeclaration
    : K_QUANTUM identifier
      COLON quantumStateType
      (ASSIGN quantumStateExpression)?
      SEMI
    ;


quantumStateType
    : identifier
    | qualifiedName
    ;


/* ----------------------------------------------------------------------------
 * 5.4 Operation declaration
 * ------------------------------------------------------------------------- */

/*
 * User-defined operations are semantic objects.
 *
 * No predefined gate catalogue is embedded here.
 */
quantumOperationDeclaration
    : quantumOperationDeclarationHeader
      blockExpression
    ;


quantumOperationDeclarationHeader
    : identifier
      genericParameters?
      quantumParameterList?
      whereClause?
    ;


/* ----------------------------------------------------------------------------
 * 5.5 Local circuit declaration
 * ------------------------------------------------------------------------- */

quantumCircuitLocalDeclaration
    : K_CIRCUIT identifier
      genericParameters?
      quantumParameterList?
      whereClause?
      blockExpression
    ;


/* ============================================================================
 * 6. QUANTUM OPERATIONS
 * ========================================================================== */

/*
 * Canonical source form:
 *
 *     apply H to q;
 *
 *     apply U(theta, phi) to q;
 *
 *     apply CNOT to q0, q1;
 *
 *     apply controlled(X) from control to target;
 *
 * Operation identity remains semantic.
 */
quantumOperationStatement
    : K_APPLY
      quantumOperation
      quantumOperationTargetClause
      SEMI
    ;


quantumOperation
    : quantumOperationExpression
    ;


quantumOperationExpression
    : quantumOperationReference
    | quantumControlledOperation
    | quantumAdjointOperation
    | quantumInverseOperation
    | quantumPowerOperation
    | quantumCompositeOperation
    | quantumParameterizedOperation
    ;


quantumOperationReference
    : qualifiedName
    ;


quantumParameterizedOperation
    : quantumOperationReference
      LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 7. OPERATION TARGETS
 * ========================================================================== */

quantumOperationTargetClause
    : K_TO quantumTargetList
    | K_FROM quantumTargetList
      K_TO quantumTargetList
    ;


quantumTargetList
    : quantumTarget
      (COMMA quantumTarget)*
    ;


quantumTarget
    : expression
    ;


/* ============================================================================
 * 8. CONTROLLED OPERATIONS
 * ========================================================================== */

/*
 * Control arity is deliberately unbounded by grammar.
 *
 * Example:
 *
 *     controlled(X)
 *
 *     controlled(U(theta))
 *
 *     controlled(controlled(U(theta)))
 *
 * Resource and semantic validation determine whether the target can realize it.
 */
quantumControlledOperation
    : K_CONTROLLED
      LPAREN quantumOperationExpression RPAREN
    ;


/* ============================================================================
 * 9. ADJOINT OPERATIONS
 * ========================================================================== */

quantumAdjointOperation
    : K_ADJOINT
      LPAREN quantumOperationExpression RPAREN
    ;


quantumInverseOperation
    : K_INVERSE
      LPAREN quantumOperationExpression RPAREN
    ;


/* ============================================================================
 * 10. POWER / REPETITION
 * ========================================================================== */

/*
 * Example:
 *
 *     power(U(theta), repetitions);
 *
 * Repetition is an expression, not a fixed grammar-level integer.
 */
quantumPowerOperation
    : K_POWER
      LPAREN
      quantumOperationExpression
      COMMA
      expression
      RPAREN
    ;


/* ============================================================================
 * 11. COMPOSITION
 * ========================================================================== */

quantumCompositeOperation
    : K_COMPOSE
      LPAREN
      quantumOperationExpression
      (COMMA quantumOperationExpression)*
      RPAREN
    ;


/* ============================================================================
 * 12. MEASUREMENT
 * ========================================================================== */

/*
 * Supported forms:
 *
 *     measure q;
 *
 *     measure q -> result;
 *
 *     measure q -> result using basis;
 *
 * The result representation is semantic.
 */
quantumMeasurementStatement
    : K_MEASURE
      quantumTargetList
      quantumMeasurementDestination?
      SEMI
    ;


quantumMeasurementDestination
    : THIN_ARROW expression
    ;


/* ============================================================================
 * 13. RESET
 * ========================================================================== */

quantumResetStatement
    : K_RESET
      quantumTargetList
      SEMI
    ;


/* ============================================================================
 * 14. BARRIER
 * ========================================================================== */

/*
 * Barrier means a semantic ordering constraint.
 *
 * It does NOT mean:
 *
 *     insert hardware pulse
 *
 * or:
 *
 *     wait fixed duration
 */
quantumBarrierStatement
    : K_BARRIER
      quantumTargetList?
      SEMI
    ;


/* ============================================================================
 * 15. SYNCHRONIZATION
 * ========================================================================== */

/*
 * Synchronization expresses an ordering dependency.
 *
 * Scheduling decides its concrete realization.
 */
quantumSynchronizationStatement
    : K_SYNCHRONIZE
      quantumTargetList?
      SEMI
    ;


/* ============================================================================
 * 16. ABSTRACT RESOURCE ALLOCATION
 * ========================================================================== */

/*
 * Allocation is source-level resource intent.
 *
 * It does NOT allocate a physical qubit.
 */
quantumAllocationStatement
    : K_ALLOCATE
      identifier
      quantumAllocationType?
      SEMI
    ;


quantumAllocationType
    : COLON typeExpression
    ;


/* ============================================================================
 * 17. ABSTRACT RESOURCE RELEASE
 * ========================================================================== */

quantumReleaseStatement
    : K_RELEASE
      expression
      SEMI
    ;


/* ============================================================================
 * 18. OBSERVABLES
 * ========================================================================== */

quantumObservationStatement
    : K_OBSERVE
      expression
      quantumObservationTarget?
      SEMI
    ;


quantumObservationTarget
    : K_ON quantumTargetList
    ;


/* ============================================================================
 * 19. ENTANGLEMENT
 * ========================================================================== */

/*
 * Entanglement is expressed as semantic intent.
 *
 * The implementation may use:
 *
 *     native gates;
 *     decomposition;
 *     teleportation;
 *     measurement-based computation;
 *     photonic mechanisms;
 *     another supported realization.
 */
quantumEntanglementStatement
    : K_ENTANGLE
      quantumTargetList
      SEMI
    ;


/* ============================================================================
 * 20. QUANTUM / CLASSICAL CONTROL
 * ========================================================================== */

/*
 * A classical expression may determine whether quantum work executes.
 *
 * Whether that expression is:
 *
 *     compile-time;
 *     classical runtime;
 *     measurement-derived;
 *     distributed;
 *     dynamic-circuit control
 *
 * is semantic analysis.
 */
quantumConditionalStatement
    : K_IF
      expression
      quantumBlock
      (K_ELSE quantumBlock)?
    ;


/* ============================================================================
 * 21. QUANTUM NOISE INTENT
 * ========================================================================== */

/*
 * This syntax describes an intent to reason about or expose noise.
 *
 * It does NOT define a noise model.
 *
 * ZQN remains the owner of:
 *
 *     - fault classes;
 *     - correlated faults;
 *     - leakage;
 *     - loss;
 *     - erasure;
 *     - calibration effects;
 *     - noise channels;
 *     - fault semantics.
 */
quantumNoiseStatement
    : K_NOISE
      expression
      SEMI
    ;


/* ============================================================================
 * 22. QUANTUM REQUIREMENTS
 * ========================================================================== */

/*
 * Resource requirements belong to the resource/capability layer.
 *
 * Examples:
 *
 *     requires quantum;
 *     requires logical;
 *     requires capability;
 *
 * This grammar only preserves their syntax.
 */
quantumRequirementStatement
    : K_REQUIRES
      expression
      SEMI
    ;


quantumCapabilityStatement
    : K_CAPABILITY
      expression
      SEMI
    ;


/* ============================================================================
 * 23. LOGICAL QUANTUM INTENT
 * ========================================================================== */

/*
 * Logical-vs-physical distinction is important.
 *
 * The source can request logical semantics without naming a physical qubit.
 */
quantumLogicalDeclaration
    : K_LOGICAL
      identifier
      (COLON typeExpression)?
      SEMI
    ;


/* ============================================================================
 * 24. QUANTUM CODE / ERROR-CORRECTION INTENT
 * ========================================================================== */

/*
 * This is intentionally only a source-level declaration/reference.
 *
 * The grammar does NOT implement QEC.
 */
quantumCodeDeclaration
    : K_CODE
      identifier
      genericParameters?
      quantumParameterList?
      blockExpression
    ;


quantumParityExpression
    : K_PARITY
      LPAREN
      quantumTargetList
      RPAREN
    ;


/* ============================================================================
 * 25. FIDELITY / QUALITY INTENT
 * ========================================================================== */

/*
 * No threshold is hard-coded.
 *
 * For example, this grammar must never encode:
 *
 *     fidelity > 0.95
 *
 * A policy/analysis subsystem owns thresholds.
 */
quantumFidelityExpression
    : K_FIDELITY
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 26. QUANTUM SURFACE / TOPOLOGY INTENT
 * ========================================================================== */

/*
 * Surface syntax may identify a semantic family such as a code/surface model,
 * but MUST NOT encode physical topology.
 */
quantumSurfaceExpression
    : K_SURFACE
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 27. QUANTUM STATE EXPRESSIONS
 * ========================================================================== */

quantumStateExpression
    : quantumBasisState
    | expression
    ;


quantumBasisState
    : PIPE quantumBasisSymbol KET_CLOSE
    ;


quantumBasisSymbol
    : ZERO
    | ONE
    | PLUS
    | MINUS
    ;


/* ============================================================================
 * 28. QUANTUM REGISTER INDEXING
 * ========================================================================== */

/*
 * Register indexing is delegated to the normal expression grammar.
 *
 * This rule exists as an explicit semantic integration point.
 */
quantumIndexedTarget
    : quantumTarget
      LBRACK
      expression
      RBRACK
    ;


/* ============================================================================
 * 29. QUANTUM RANGE TARGETS
 * ========================================================================== */

/*
 * Supports symbolic/range-based selection.
 *
 * Examples:
 *
 *     q[start .. end]
 *
 * Actual range semantics belong to the expression/type system.
 */
quantumRangeTarget
    : quantumTarget
      LBRACK
      expression
      RANGE
      expression
      RBRACK
    ;


/* ============================================================================
 * 30. QUANTUM TARGET GROUPS
 * ========================================================================== */

quantumTargetGroup
    : LPAREN
      quantumTargetList
      RPAREN
    ;


/* ============================================================================
 * 31. QUANTUM OPERATION MODIFIER TREE
 * ========================================================================== */

/*
 * Recursive structure allows future semantic operation modifiers without
 * introducing a separate grammar production for every possible composition.
 */
quantumOperationModifierExpression
    : quantumOperationExpression
    | K_CONTROLLED
      LPAREN quantumOperationModifierExpression RPAREN
    | K_ADJOINT
      LPAREN quantumOperationModifierExpression RPAREN
    | K_INVERSE
      LPAREN quantumOperationModifierExpression RPAREN
    | K_POWER
      LPAREN
      quantumOperationModifierExpression
      COMMA expression
      RPAREN
    ;


/* ============================================================================
 * 32. QUANTUM EXECUTION REGION
 * ========================================================================== */

/*
 * Execution strategy is deliberately separated from machine identity.
 *
 * This does not select a vendor or device.
 */
quantumExecutionRegion
    : K_EXECUTE
      quantumBlock
    ;


/* ============================================================================
 * 33. SIMULATION REGION
 * ========================================================================== */

/*
 * Simulation is a possible execution strategy.
 *
 * It does not make simulation the canonical meaning of the program.
 */
quantumSimulationRegion
    : K_SIMULATE
      quantumBlock
    ;


/* ============================================================================
 * 34. SYNTHESIS INTENT
 * ========================================================================== */

/*
 * Synthesis is a compilation request.
 *
 * The actual synthesis algorithm belongs to the compiler/optimization layers.
 */
quantumSynthesisRegion
    : K_SYNTHESIZE
      quantumBlock
    ;


/* ============================================================================
 * 35. DEPLOYMENT INTENT
 * ========================================================================== */

/*
 * Deployment does not contain a physical device ID.
 *
 * Target selection belongs to deployment/resource/runtime configuration.
 */
quantumDeploymentRegion
    : K_DEPLOY
      quantumBlock
    ;


/* ============================================================================
 * 36. QUANTUM REQUIREMENT GROUP
 * ========================================================================== */

quantumRequirementGroup
    : K_REQUIRES
      LBRACE
      quantumRequirementItem*
      RBRACE
    ;


quantumRequirementItem
    : expression
      SEMI
    ;


/* ============================================================================
 * 37. QUANTUM CAPABILITY GROUP
 * ========================================================================== */

quantumCapabilityGroup
    : K_CAPABILITY
      LBRACE
      quantumCapabilityItem*
      RBRACE
    ;


quantumCapabilityItem
    : expression
      SEMI
    ;


/* ============================================================================
 * 38. TARGET-INDEPENDENT HARDWARE INTENT
 * ========================================================================== */

/*
 * A program can express what it needs without specifying what machine supplies
 * it.
 *
 * This is central to POCO-REAF.
 */
quantumTargetRequirement
    : K_TARGET
      expression
      SEMI
    ;


/* ============================================================================
 * 39. LINEAR / AFFINE QUANTUM RESOURCE INTENT
 * ========================================================================== */

/*
 * Quantum values frequently require non-copying semantics.
 *
 * The grammar only records the modifier. The type/effect system enforces the
 * actual rules.
 */
quantumResourceModifier
    : K_LINEAR
    | K_AFFINE
    ;


/* ============================================================================
 * 40. QUANTUM ANNOTATIONS
 * ========================================================================== */

/*
 * Quantum annotations remain generic attributes.
 *
 * This avoids creating a closed annotation vocabulary.
 *
 * Example:
 *
 *     @logical
 *     @resource(...)
 *     @capability(...)
 *     @architecture_independent
 */
quantumAnnotation
    : AT identifier
      (LPAREN argumentList? RPAREN)?
    ;


/* ============================================================================
 * 41. QUANTUM DOMAIN TYPE
 * ========================================================================== */

/*
 * Qubit is the only primitive quantum type made syntactically special.
 *
 * Register widths, state representations and future quantum types remain
 * semantic/type-system concerns.
 */
quantumDomainType
    : K_QUBIT
    | identifier
    | qualifiedName
    ;


/* ============================================================================
 * 42. QUANTUM RESOURCE EXPRESSION
 * ========================================================================== */

/*
 * Resource expressions remain symbolic.
 *
 * They may eventually represent:
 *
 *     qubits;
 *     logical qubits;
 *     measurement capacity;
 *     coherence requirements;
 *     communication requirements;
 *     memory;
 *     accelerator capability;
 *     error-correction requirements.
 *
 * No physical ceiling is encoded.
 */
quantumResourceExpression
    : identifier
    | qualifiedName
    | expression
    ;


/* ============================================================================
 * 43. QUANTUM PROGRAM COMPOSITION
 * ========================================================================== */

quantumProgram
    : quantumElement*
    ;


/* ============================================================================
 * 44. QUANTUM OPERATION ARGUMENTS
 * ========================================================================== */

quantumOperationArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 45. QUANTUM NAMED OPERATION
 * ========================================================================== */

/*
 * Operation names are identifiers.
 *
 * Consequently all of these can be supported without changing this grammar:
 *
 *     H
 *     X
 *     CNOT
 *     SWAP
 *     T
 *     U
 *     RZZ
 *     custom_gate
 *     vendor_extension
 *     future_operation
 *
 * Their availability and semantics are resolved later.
 */
quantumNamedOperation
    : qualifiedName
    ;


/* ============================================================================
 * 46. QUANTUM PARAMETER EXPRESSION
 * ========================================================================== */

quantumParameterExpression
    : expression
    ;


/* ============================================================================
 * 47. QUANTUM CLASSICAL BOUNDARY
 * ========================================================================== */

/*
 * Quantum/classical interaction is represented through normal expressions.
 *
 * The semantic layer determines whether a value:
 *
 *     - is classical;
 *     - is quantum;
 *     - is measurement-derived;
 *     - is compile-time;
 *     - is runtime;
 *     - crosses an execution boundary.
 */
quantumClassicalExpression
    : expression
    ;


/* ============================================================================
 * 48. QUANTUM MEASUREMENT CONDITION
 * ========================================================================== */

quantumMeasurementCondition
    : K_IF
      expression
    ;


/* ============================================================================
 * 49. DYNAMIC CIRCUIT REGION
 * ========================================================================== */

quantumDynamicRegion
    : K_CIRCUIT
      identifier
      quantumParameterList?
      blockExpression
    ;


/* ============================================================================
 * 50. SEMANTIC QUANTUM REGION
 * ========================================================================== */

/*
 * This is the preferred generic extension boundary.
 */
quantumSemanticRegion
    : K_QUANTUM
      identifier
      blockExpression
    ;


/* ============================================================================
 * 51. QUANTUM TERMINATION
 * ========================================================================== */

/*
 * Quantum statements use explicit semicolons.
 *
 * This prevents ambiguity between adjacent operations and ordinary language
 * statements.
 */
quantumTerminator
    : SEMI
    ;


/* ============================================================================
 * 52. INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * Required canonical parser integration:
 *
 * declaration
 *     ...
 *     | quantumDeclaration
 *     ...
 *     ;
 *
 * The parser must import this file rather than duplicating these rules.
 *
 * ============================================================================
 *
 * AST INTEGRATION
 *
 * The frontend AST must preserve:
 *
 *     - source span;
 *     - operation name;
 *     - operation arguments;
 *     - target expressions;
 *     - control expressions;
 *     - measurement destination;
 *     - resource intent;
 *     - attributes;
 *     - generic parameters;
 *     - semantic annotations.
 *
 * It must NOT directly encode:
 *
 *     - physical topology;
 *     - backend-specific gate decomposition;
 *     - scheduling;
 *     - routing;
 *     - calibration;
 *     - QEC implementation.
 *
 * ============================================================================
 *
 * QUANTUM IR INTEGRATION
 *
 * The AST lowers into the canonical `quantum::ir`.
 *
 * `quantum::ir` remains the semantic boundary.
 *
 * This grammar MUST NOT define a second:
 *
 *     QuantumGate
 *     QubitId
 *     PhysicalQubitId
 *     QuantumCircuit
 *     Schedule
 *
 * representation.
 *
 * ============================================================================
 *
 * QEC INTEGRATION
 *
 * Quantum code declarations and parity expressions are source-level intent.
 *
 * QEC owns:
 *
 *     - syndrome extraction;
 *     - decoding;
 *     - correction;
 *     - logical error handling;
 *     - code-specific algorithms.
 *
 * ============================================================================
 *
 * ZQN INTEGRATION
 *
 * `quantumNoiseStatement` records source intent only.
 *
 * ZQN owns:
 *
 *     - noise/fault classification;
 *     - fault channels;
 *     - correlated faults;
 *     - leakage;
 *     - loss;
 *     - erasure;
 *     - calibration-related fault semantics.
 *
 * ============================================================================
 *
 * SCHEDULING INTEGRATION
 *
 * Barrier and synchronization constructs are dependency intent.
 *
 * Scheduling owns:
 *
 *     - operation ordering;
 *     - timing;
 *     - resource conflicts;
 *     - alignment;
 *     - duration;
 *     - dynamical decoupling;
 *     - schedule construction.
 *
 * ============================================================================
 *
 * OPTIMIZATION INTEGRATION
 *
 * Operations are semantic operations.
 *
 * Optimization may:
 *
 *     - cancel;
 *     - commute;
 *     - decompose;
 *     - synthesize;
 *     - reduce;
 *     - transform.
 *
 * Such transformations must preserve source semantics.
 *
 * ============================================================================
 *
 * ROUTING INTEGRATION
 *
 * Targets are logical/source targets.
 *
 * Routing decides physical realization.
 *
 * This grammar must never encode physical adjacency.
 *
 * ============================================================================
 *
 * HARDWARE INTEGRATION
 *
 * Hardware capability discovery happens after parsing.
 *
 * This grammar does not select:
 *
 *     IBM
 *     Google
 *     IonQ
 *     Rigetti
 *     a simulator
 *     a CPU
 *     a GPU
 *     a topology
 *     a physical qubit.
 *
 * ============================================================================
 *
 * RESILIENCE INTEGRATION
 *
 * Resilience may use the semantic program and downstream execution state to
 * decide:
 *
 *     retry;
 *     reroute;
 *     reschedule;
 *     recompile;
 *     change QEC;
 *     mitigate;
 *     switch backend;
 *     quarantine;
 *     recover;
 *     abort.
 *
 * None of those policies belongs to this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 53. SCALABILITY INVARIANTS
 * ========================================================================== */

/*
 * This grammar deliberately contains:
 *
 *     zero fixed qubit limits;
 *     zero fixed register limits;
 *     zero fixed gate counts;
 *     zero fixed operation counts;
 *     zero fixed circuit depth limits;
 *     zero fixed topology sizes;
 *     zero backend identifiers;
 *     zero physical-qubit identifiers.
 *
 * Repetition is expressed using parser recursion / lists.
 *
 * Resource limits belong to resource analysis and runtime admission.
 */


/* ============================================================================
 * 54. DETERMINISM INVARIANTS
 * ========================================================================== */

/*
 * The grammar must:
 *
 *     - avoid ambiguous alternatives where possible;
 *     - avoid semantic predicates;
 *     - avoid target-specific parser actions;
 *     - avoid lexer state assumptions;
 *     - preserve source ordering;
 *     - preserve nested operation structure.
 *
 * Semantic ambiguity must be reported by semantic analysis rather than resolved
 * differently depending on backend.
 */


/* ============================================================================
 * 55. SECURITY INVARIANTS
 * ========================================================================== */

/*
 * Parsing quantum syntax must never:
 *
 *     - access the filesystem;
 *     - access a network;
 *     - contact a QPU;
 *     - query hardware;
 *     - execute an operation;
 *     - allocate physical resources;
 *     - execute arbitrary code.
 *
 * Source syntax is data.
 *
 * Execution occurs only after compilation and explicit runtime authorization.
 */


/* ============================================================================
 * 56. COMPLETION CONTRACT
 * ========================================================================== */

/*
 * This file is complete when:
 *
 * [ ] every referenced token exists in the canonical lexer;
 * [ ] every referenced parser rule exists in the canonical parser;
 * [ ] this file is imported exactly once;
 * [ ] no duplicate quantum declaration grammar remains authoritative;
 * [ ] every accepted construct has an AST destination;
 * [ ] AST lowering reaches canonical quantum::ir;
 * [ ] no hardware limit exists here;
 * [ ] no fixed gate catalogue exists here;
 * [ ] no physical topology exists here;
 * [ ] no QEC algorithm exists here;
 * [ ] no ZQN model exists here;
 * [ ] no scheduler implementation exists here;
 * [ ] no optimizer implementation exists here;
 * [ ] positive parser tests exist;
 * [ ] negative parser tests exist;
 * [ ] cross-domain tests exist;
 * [ ] scalability tests exist;
 * [ ] deterministic parsing tests exist;
 * [ ] round-trip tests exist where a canonical printer exists.
 *
 * ============================================================================
 */