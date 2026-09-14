/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *   grammar/hardware/qpu.g4
 *
 * Grammar kind:
 *   ANTLR4 parser grammar
 *
 * Rust target:
 *   Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *   Action-free grammar.
 *   No embedded Rust.
 *   No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL syntax for describing a Quantum
 * Processing Unit (QPU) as a machine-independent hardware capability contract.
 *
 * A QPU declaration may describe:
 *
 *   - quantum-processing capabilities;
 *   - supported quantum execution models;
 *   - logical resource requirements;
 *   - qubit resource requirements/capabilities;
 *   - operation/gate capabilities;
 *   - measurement capabilities;
 *   - reset capabilities;
 *   - dynamic-circuit capabilities;
 *   - classical-control capabilities;
 *   - parameterization capabilities;
 *   - precision requirements;
 *   - error/fidelity requirements;
 *   - timing/latency requirements;
 *   - connectivity requirements;
 *   - memory/interface capabilities;
 *   - execution/resource constraints;
 *   - preferences;
 *   - target-independent metadata;
 *   - extensible vendor/dialect properties.
 *
 * ============================================================================
 * THIS FILE OWNS
 * ============================================================================
 *
 *   - QPU declaration syntax;
 *   - QPU capability contracts;
 *   - QPU resource contracts;
 *   - QPU operation capability declarations;
 *   - QPU measurement/reset declarations;
 *   - QPU dynamic-execution declarations;
 *   - QPU logical/physical capability annotations;
 *   - QPU requirements;
 *   - QPU constraints;
 *   - QPU preferences;
 *   - QPU target references;
 *   - QPU implementation-independent properties;
 *   - QPU composition and relationships;
 *   - QPU extension properties.
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - numeric literal recognition;
 *   - strings;
 *   - canonical AST representation;
 *   - quantum::ir;
 *   - quantum gate semantic definitions;
 *   - QEC algorithms;
 *   - ZQN/noise models;
 *   - calibration;
 *   - physical device discovery;
 *   - physical device IDs;
 *   - physical qubit numbering;
 *   - topology discovery;
 *   - routing;
 *   - placement;
 *   - scheduling;
 *   - optimization;
 *   - pulse generation;
 *   - backend drivers;
 *   - runtime dispatch;
 *   - resource accounting;
 *   - provider-specific execution APIs.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniTokens
 *   |
 *   v
 * ZamaniHardwareParser
 *   |
 *   v
 * QPU syntax AST
 *   |
 *   v
 * semantic analysis
 *   |
 *   +--------------------------+
 *   |                          |
 *   v                          v
 * hardware capability model   quantum semantic model
 *   |                          |
 *   +------------+-------------+
 *                |
 *                v
 *       resource/capability
 *           resolution
 *                |
 *                v
 *     routing / optimization
 *                |
 *                v
 *           scheduling
 *                |
 *                v
 *        hardware HAL/runtime
 *
 * The grammar never directly selects a physical QPU.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * The grammar describes syntax only.
 *
 * Quantum program semantics continue to flow through the canonical:
 *
 *   quantum::ir
 *
 * QPU declarations must not create a second quantum operation/gate/qubit IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A source program may say:
 *
 *   qpu QuantumBackend {
 *       requires qubits >= required_qubits;
 *       requires capability.dynamic_circuit;
 *   }
 *
 * without saying:
 *
 *   - use device X;
 *   - use physical qubit 0;
 *   - use physical qubit 1;
 *   - use exactly N physical qubits;
 *   - use topology X;
 *   - use a particular vendor;
 *   - use a fixed calibration;
 *   - use a fixed clock;
 *   - use a fixed pulse duration.
 *
 * Those decisions belong to target resolution and runtime.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally no:
 *
 *   MAX_QUBITS
 *   MAX_LOGICAL_QUBITS
 *   MAX_PHYSICAL_QUBITS
 *   MAX_GATES
 *   MAX_CIRCUIT_DEPTH
 *   MAX_CONNECTIONS
 *   MAX_SHOTS
 *   MAX_CHANNELS
 *   MAX_DEVICES
 *
 * or equivalent grammar-level limits.
 *
 * Quantities are expressions.
 * Collections are structurally unbounded by this grammar.
 * Physical capacity is resolved by the resource/capability system.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Stable language concepts may receive dedicated tokens.
 *
 * Provider-, vendor-, technology-, experiment-, or future-specific concepts
 * should normally use qualified identifiers/properties rather than forcing a
 * new core grammar keyword.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareQpuParser;

options {
    tokenVocab = ZamaniTokens;
}


// ============================================================================
// 1. PUBLIC ENTRY POINT
// ============================================================================
//
// hardware.g4 delegates QPU declarations here.
//
// Conceptually:
//
//   hardwareItem
//       -> qpuDeclaration
//
// A QPU declaration is a hardware capability contract, not a device
// discovery statement.
//

qpuDeclaration
    : qpuAnnotation*
      qpuVisibility?
      qpuModifier*
      K_QPU
      IDENTIFIER
      qpuGenericParameters?
      qpuRequirementClause?
      qpuCapabilityClause?
      LBRACE
          qpuItem*
      RBRACE
    ;


// ============================================================================
// 2. VISIBILITY / MODIFIERS
// ============================================================================

qpuVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;

qpuModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


// ============================================================================
// 3. ANNOTATIONS
// ============================================================================
//
// Annotation names remain identifiers so the grammar does not need a
// permanent list of vendor/provider-specific annotations.
//

qpuAnnotation
    : AT
      IDENTIFIER
      (
          LPAREN
          qpuAnnotationArguments?
          RPAREN
      )?
    ;

qpuAnnotationArguments
    : qpuAnnotationArgument
      (
          COMMA
          qpuAnnotationArgument
      )*
    ;

qpuAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | qpuQualifiedName
    | qpuExpression
    ;


// ============================================================================
// 4. GENERIC PARAMETERS
// ============================================================================
//
// Generic QPUs permit reusable declarations:
//
//   qpu GenericQpu<QubitCapacity, Precision> { ... }
//
// The parameters are semantic parameters, not parser-level limits.
//

qpuGenericParameters
    : LT
      qpuGenericParameter
      (
          COMMA
          qpuGenericParameter
      )*
      GT
    ;

qpuGenericParameter
    : IDENTIFIER
      (
          COLON
          qpuGenericBound
      )?
      (
          ASSIGN
          qpuExpression
      )?
    ;

qpuGenericBound
    : qpuQualifiedName
    | qpuCapabilityReference
    ;


// ============================================================================
// 5. QPU BODY
// ============================================================================

qpuItem
    : qpuAnnotation* qpuQuantumModelDecl
    | qpuAnnotation* qpuQubitResourceDecl
    | qpuAnnotation* qpuLogicalQubitDecl
    | qpuAnnotation* qpuPhysicalQubitCapabilityDecl
    | qpuAnnotation* qpuOperationCapabilityDecl
    | qpuAnnotation* qpuGateSetDecl
    | qpuAnnotation* qpuMeasurementCapabilityDecl
    | qpuAnnotation* qpuResetCapabilityDecl
    | qpuAnnotation* qpuDynamicCircuitDecl
    | qpuAnnotation* qpuMidCircuitDecl
    | qpuAnnotation* qpuClassicalControlDecl
    | qpuAnnotation* qpuParameterizationDecl
    | qpuAnnotation* qpuConnectivityDecl
    | qpuAnnotation* qpuTimingDecl
    | qpuAnnotation* qpuPrecisionDecl
    | qpuAnnotation* qpuFidelityDecl
    | qpuAnnotation* qpuErrorRequirementDecl
    | qpuAnnotation* qpuMemoryDecl
    | qpuAnnotation* qpuInterfaceDecl
    | qpuAnnotation* qpuResourceDecl
    | qpuAnnotation* qpuCapabilityDecl
    | qpuAnnotation* qpuRequirementDecl
    | qpuAnnotation* qpuConstraintDecl
    | qpuAnnotation* qpuPreferenceDecl
    | qpuAnnotation* qpuTargetDecl
    | qpuAnnotation* qpuParameterDecl
    | qpuAnnotation* qpuConstantDecl
    | qpuAnnotation* qpuPropertyDecl
    | qpuAnnotation* qpuCompositionDecl
    | qpuAnnotation* qpuConnectionDecl
    | qpuAnnotation* qpuExtensionDecl
    | qpuAnnotation* qpuAssertion
    | qpuAnnotation* qpuUsingDecl
    ;


// ============================================================================
// 6. QUANTUM EXECUTION MODEL
// ============================================================================
//
// Examples of semantic values:
//
//   quantum_model = gate_model
//   quantum_model = annealing
//   quantum_model = measurement_based
//   quantum_model = hybrid
//
// New models remain expressible through qualified names.
//

qpuQuantumModelDecl
    : K_QUANTUM
      IDENTIFIER?
      qpuAssignmentBody
    ;


// ============================================================================
// 7. QUBIT RESOURCE CONTRACT
// ============================================================================
//
// IMPORTANT:
//
// This does NOT allocate physical qubits.
//
// Examples:
//
//   qubits >= workload_qubits;
//   qubits = required_qubits;
//   capacity >= algorithm_width;
//
// The semantic layer determines whether a target can satisfy the contract.
//

qpuQubitResourceDecl
    : K_QUBIT
      IDENTIFIER?
      qpuResourceBody
    ;

qpuLogicalQubitDecl
    : K_LOGICAL
      K_QUBIT
      IDENTIFIER?
      qpuResourceBody
    ;

qpuPhysicalQubitCapabilityDecl
    : K_PHYSICAL
      K_QUBIT
      IDENTIFIER?
      qpuCapabilityBody
    ;


// ============================================================================
// 8. OPERATION CAPABILITIES
// ============================================================================
//
// Operation names are identifiers/qualified names.
//
// This deliberately avoids embedding a permanent gate list into hardware.g4.
//
// Quantum semantic definitions remain owned by the quantum grammar/IR.
//

qpuOperationCapabilityDecl
    : K_OPERATION
      IDENTIFIER?
      qpuOperationBody
    ;

qpuOperationBody
    : LBRACE
      qpuOperationProperty*
      RBRACE
    ;

qpuOperationProperty
    : K_SUPPORTED
      ASSIGN
      qpuBooleanExpression
      SEMICOLON

    | K_COUNT
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_DURATION
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_FIDELITY
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_ERROR
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_PRECISION
      ASSIGN
      qpuExpression
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 9. GATE SETS
// ============================================================================
//
// Gate names are qualified names.
//
// The grammar does not impose a fixed gate count.
//

qpuGateSetDecl
    : K_GATE
      IDENTIFIER?
      qpuGateSetBody
    ;

qpuGateSetBody
    : LBRACE
      qpuGateSetItem*
      RBRACE
    ;

qpuGateSetItem
    : qpuQualifiedName
      (
          ASSIGN
          qpuExpression
      )?
      SEMICOLON
    ;


// ============================================================================
// 10. MEASUREMENT
// ============================================================================

qpuMeasurementCapabilityDecl
    : K_MEASUREMENT
      IDENTIFIER?
      qpuCapabilityBody
    ;


// ============================================================================
// 11. RESET
// ============================================================================

qpuResetCapabilityDecl
    : K_RESET
      IDENTIFIER?
      qpuCapabilityBody
    ;


// ============================================================================
// 12. DYNAMIC CIRCUITS
// ============================================================================
//
// Dynamic execution is a capability contract.
//
// Scheduling and runtime semantics remain downstream.
//

qpuDynamicCircuitDecl
    : K_DYNAMIC
      K_CIRCUIT
      IDENTIFIER?
      qpuCapabilityBody
    ;

qpuMidCircuitDecl
    : K_MID
      K_CIRCUIT
      IDENTIFIER?
      qpuCapabilityBody
    ;


// ============================================================================
// 13. CLASSICAL CONTROL
// ============================================================================
//
// Describes whether the QPU interface permits classical feedback/control.
// It does not define the classical runtime itself.
//

qpuClassicalControlDecl
    : K_CLASSICAL
      K_CONTROL
      IDENTIFIER?
      qpuCapabilityBody
    ;


// ============================================================================
// 14. PARAMETERIZED OPERATIONS
// ============================================================================

qpuParameterizationDecl
    : K_PARAMETERIZED
      K_OPERATION
      IDENTIFIER?
      qpuParameterizationBody
    ;

qpuParameterizationBody
    : LBRACE
      qpuParameterizationProperty*
      RBRACE
    ;

qpuParameterizationProperty
    : K_PARAMETER
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_PRECISION
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_RANGE
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      qpuBooleanExpression
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 15. CONNECTIVITY
// ============================================================================
//
// Connectivity expresses a requirement/capability class.
//
// It must NOT encode a physical topology directly.
//
// Examples:
//
//   connectivity = all_to_all;
//   connectivity >= required_connectivity;
//   connectivity = symbolic_topology;
//
// Detailed topology realization belongs to hardware/topology/placement/routing.
//

qpuConnectivityDecl
    : K_CONNECTIVITY
      IDENTIFIER?
      qpuAssignmentBody
    ;


// ============================================================================
// 16. TIMING
// ============================================================================
//
// Timing values are expressions.
//
// No fixed unit, frequency, pulse duration, or clock is assumed by the parser.
//

qpuTimingDecl
    : K_TIMING
      IDENTIFIER?
      qpuTimingBody
    ;

qpuTimingBody
    : LBRACE
      qpuTimingProperty*
      RBRACE
    ;

qpuTimingProperty
    : K_LATENCY
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_DURATION
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_PERIOD
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_FREQUENCY
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_DEADLINE
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_SETUP
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_HOLD
      ASSIGN
      qpuExpression
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 17. PRECISION
// ============================================================================

qpuPrecisionDecl
    : K_PRECISION
      IDENTIFIER?
      qpuAssignmentBody
    ;


// ============================================================================
// 18. FIDELITY
// ============================================================================
//
// Fidelity is a property/requirement, not a hard-coded threshold.
//
// The grammar deliberately permits:
//
//   fidelity >= required_fidelity;
//
// rather than:
//
//   fidelity >= 0.95
//
// A numerical threshold, when used, is source semantics and not a grammar
// limitation.
//

qpuFidelityDecl
    : K_FIDELITY
      IDENTIFIER?
      qpuAssignmentBody
    ;


// ============================================================================
// 19. ERROR REQUIREMENTS
// ============================================================================
//
// This does not implement QEC or ZQN.
//
// It expresses source-level requirements that downstream resilience/QEC/noise
// systems may consume.
//

qpuErrorRequirementDecl
    : K_ERROR
      IDENTIFIER?
      qpuRequirementBody
    ;


// ============================================================================
// 20. MEMORY / CLASSICAL INTERFACE
// ============================================================================
//
// Quantum memory semantics remain separate from physical memory realization.
//

qpuMemoryDecl
    : K_MEMORY
      IDENTIFIER?
      qpuResourceBody
    ;

qpuInterfaceDecl
    : K_INTERFACE
      IDENTIFIER
      qpuInterfaceBody
    ;

qpuInterfaceBody
    : LBRACE
      qpuInterfaceItem*
      RBRACE
    ;

qpuInterfaceItem
    : qpuPropertyDecl
    | qpuCapabilityDecl
    | qpuRequirementDecl
    | qpuConstraintDecl
    | qpuPreferenceDecl
    | qpuParameterDecl
    | qpuConnectionDecl
    ;


// ============================================================================
// 21. GENERIC RESOURCE CONTRACT
// ============================================================================

qpuResourceDecl
    : K_RESOURCE
      IDENTIFIER?
      qpuResourceBody
    ;

qpuResourceBody
    : LBRACE
      qpuResourceProperty*
      RBRACE
    ;

qpuResourceProperty
    : K_COUNT
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_CAPACITY
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_MINIMUM
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_MAXIMUM
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_WIDTH
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_DEPTH
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_THROUGHPUT
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      qpuExpression
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 22. CAPABILITIES
// ============================================================================

qpuCapabilityDecl
    : K_CAPABILITY
      IDENTIFIER?
      qpuCapabilityBody
    ;

qpuCapabilityBody
    : LBRACE
      qpuCapabilityProperty*
      RBRACE
    ;

qpuCapabilityProperty
    : K_SUPPORTED
      ASSIGN
      qpuBooleanExpression
      SEMICOLON

    | K_VERSION
      ASSIGN
      qpuValue
      SEMICOLON

    | K_LEVEL
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_PRECISION
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_FIDELITY
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      qpuExpression
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 23. REQUIREMENTS
// ============================================================================

qpuRequirementDecl
    : K_REQUIRE
      IDENTIFIER?
      qpuRequirementBody
    ;

qpuRequirementBody
    : LBRACE
      qpuRequirementProperty*
      RBRACE
    ;

qpuRequirementProperty
    : K_MINIMUM
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_MAXIMUM
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_AT_LEAST
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_AT_MOST
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      qpuBooleanExpression
      SEMICOLON

    | qpuComparison
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 24. TOP-LEVEL REQUIREMENT / CAPABILITY CLAUSES
// ============================================================================

qpuRequirementClause
    : K_REQUIRES
      qpuRequirementExpression
    ;

qpuCapabilityClause
    : K_PROVIDES
      qpuCapabilityExpression
    ;

qpuRequirementExpression
    : qpuExpression
    ;

qpuCapabilityExpression
    : qpuExpression
    ;


// ============================================================================
// 25. CONSTRAINTS
// ============================================================================

qpuConstraintDecl
    : K_CONSTRAINT
      IDENTIFIER?
      qpuConstraintBody
    ;

qpuConstraintBody
    : LBRACE
      qpuConstraintProperty*
      RBRACE
    ;

qpuConstraintProperty
    : qpuComparison
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      qpuBooleanExpression
      SEMICOLON

    | K_FORBIDDEN
      ASSIGN
      qpuBooleanExpression
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 26. PREFERENCES
// ============================================================================

qpuPreferenceDecl
    : K_PREFERENCE
      IDENTIFIER?
      qpuPreferenceBody
    ;

qpuPreferenceBody
    : LBRACE
      qpuPreferenceProperty*
      RBRACE
    ;

qpuPreferenceProperty
    : K_PRIORITY
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_WEIGHT
      ASSIGN
      qpuExpression
      SEMICOLON

    | K_PREFERRED
      ASSIGN
      qpuExpression
      SEMICOLON

    | qpuKeyValue
      SEMICOLON
    ;


// ============================================================================
// 27. TARGET REFERENCE
// ============================================================================
//
// A target reference is symbolic.
//
// It does not enumerate or discover a physical device.
//

qpuTargetDecl
    : K_TARGET
      IDENTIFIER?
      qpuAssignmentBody
    ;


// ============================================================================
// 28. PARAMETERS / CONSTANTS
// ============================================================================

qpuParameterDecl
    : K_PARAMETER
      IDENTIFIER
      (
          COLON
          qpuType
      )?
      (
          ASSIGN
          qpuExpression
      )?
      SEMICOLON
    ;

qpuConstantDecl
    : K_CONST
      IDENTIFIER
      COLON
      qpuType
      ASSIGN
      qpuExpression
      SEMICOLON
    ;


// ============================================================================
// 29. GENERIC PROPERTY
// ============================================================================
//
// Generic properties provide forward compatibility without making every new
// quantum technology a core language keyword.
//

qpuPropertyDecl
    : IDENTIFIER
      ASSIGN
      qpuValue
      SEMICOLON
    ;


// ============================================================================
// 30. COMPOSITION
// ============================================================================

qpuCompositionDecl
    : K_COMPOSE
      IDENTIFIER?
      qpuCompositionBody
    ;

qpuCompositionBody
    : LBRACE
      qpuCompositionItem*
      RBRACE
    ;

qpuCompositionItem
    : qpuQualifiedName
    | qpuPropertyDecl
    | qpuCapabilityDecl
    | qpuRequirementDecl
    ;


// ============================================================================
// 31. CONNECTIONS
// ============================================================================
//
// This is an abstract interface relationship.
//
// It does NOT perform physical routing.
//

qpuConnectionDecl
    : K_CONNECT
      qpuQualifiedName
      THIN_ARROW
      qpuQualifiedName
      SEMICOLON
    ;


// ============================================================================
// 32. EXTENSIONS
// ============================================================================
//
// Future/vendor-specific declarations remain structurally representable.
//

qpuExtensionDecl
    : K_EXTEND
      qpuQualifiedName
      qpuExtensionBody
    ;

qpuExtensionBody
    : LBRACE
      qpuExtensionItem*
      RBRACE
    ;

qpuExtensionItem
    : qpuPropertyDecl
    | qpuCapabilityDecl
    | qpuRequirementDecl
    | qpuConstraintDecl
    | qpuPreferenceDecl
    ;


// ============================================================================
// 33. ASSERTIONS
// ============================================================================

qpuAssertion
    : K_ASSERT
      LPAREN
      qpuExpression
      RPAREN
      SEMICOLON
    ;


// ============================================================================
// 34. USING
// ============================================================================

qpuUsingDecl
    : K_USING
      qpuQualifiedName
      SEMICOLON
    ;


// ============================================================================
// 35. ASSIGNMENT BODY
// ============================================================================

qpuAssignmentBody
    : ASSIGN
      qpuValue
      SEMICOLON

    | LBRACE
      qpuPropertyItem*
      RBRACE
    ;

qpuPropertyItem
    : qpuPropertyDecl
    | qpuCapabilityDecl
    | qpuRequirementDecl
    | qpuConstraintDecl
    | qpuPreferenceDecl
    ;


// ============================================================================
// 36. VALUE MODEL
// ============================================================================
//
// This rule intentionally does not impose hardware-size limits.
//

qpuValue
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | qpuBooleanLiteral
    | qpuQualifiedName
    | qpuExpression
    | qpuList
    | qpuMap
    ;

qpuBooleanLiteral
    : K_TRUE
    | K_FALSE
    ;


// ============================================================================
// 37. LIST VALUES
// ============================================================================

qpuList
    : LBRACKET
      (
          qpuValue
          (
              COMMA
              qpuValue
          )*
      )?
      RBRACKET
    ;


// ============================================================================
// 38. MAP VALUES
// ============================================================================

qpuMap
    : LBRACE
      (
          qpuMapEntry
          (
              COMMA
              qpuMapEntry
          )*
      )?
      RBRACE
    ;

qpuMapEntry
    : qpuQualifiedName
      COLON
      qpuValue
    ;


// ============================================================================
// 39. QUALIFIED NAMES
// ============================================================================

qpuQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


// ============================================================================
// 40. TYPES
// ============================================================================
//
// Hardware/QPU type semantics are resolved by the common type system.
// This grammar does not create a parallel type system.
//

qpuType
    : qpuQualifiedName
    | K_INT
    | K_FLOAT
    | K_BOOL
    | K_STRING
    | K_STR
    ;


// ============================================================================
// 41. EXPRESSIONS
// ============================================================================
//
// Expressions are intentionally symbolic and unbounded.
//
// The semantic/type layer remains responsible for meaning and representability.
//

qpuExpression
    : qpuLogicalOrExpression
    ;

qpuLogicalOrExpression
    : qpuLogicalAndExpression
      (
          LOGICAL_OR
          qpuLogicalAndExpression
      )*
    ;

qpuLogicalAndExpression
    : qpuEqualityExpression
      (
          LOGICAL_AND
          qpuEqualityExpression
      )*
    ;

qpuEqualityExpression
    : qpuRelationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          qpuRelationalExpression
      )*
    ;

qpuRelationalExpression
    : qpuAdditiveExpression
      (
          (
              LESS_THAN
            | LESS_EQUAL
            | GREATER_THAN
            | GREATER_EQUAL
          )
          qpuAdditiveExpression
      )*
    ;

qpuAdditiveExpression
    : qpuMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          qpuMultiplicativeExpression
      )*
    ;

qpuMultiplicativeExpression
    : qpuUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          qpuUnaryExpression
      )*
    ;

qpuUnaryExpression
    : (
          PLUS
        | MINUS
        | EXCLAMATION
      )
      qpuUnaryExpression
    | qpuPrimaryExpression
    ;

qpuPrimaryExpression
    : qpuValueAtom
    | LPAREN qpuExpression RPAREN
    ;

qpuValueAtom
    : IDENTIFIER
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | K_TRUE
    | K_FALSE
    ;


// ============================================================================
// 42. BOOLEAN EXPRESSIONS
// ============================================================================

qpuBooleanExpression
    : qpuExpression
    ;


// ============================================================================
// 43. COMPARISONS
// ============================================================================

qpuComparison
    : qpuExpression
      (
          EQUAL_EQUAL
        | NOT_EQUAL
        | LESS_THAN
        | LESS_EQUAL
        | GREATER_THAN
        | GREATER_EQUAL
      )
      qpuExpression
    ;


// ============================================================================
// 44. RESOURCE / CAPABILITY REFERENCES
// ============================================================================

qpuCapabilityReference
    : qpuQualifiedName
    ;


// ============================================================================
// 45. GENERIC KEY/VALUE PROPERTY
// ============================================================================

qpuKeyValue
    : qpuQualifiedName
      ASSIGN
      qpuValue
    ;