/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *   grammar/hardware/quantum-device.g4
 *
 * Grammar kind:
 *   ANTLR4 parser grammar
 *
 * Target:
 *   Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *   Action-free ANTLR grammar.
 *   No embedded Rust.
 *   No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * Defines the source-level syntax for declaring quantum-capable hardware
 * resources and quantum-device capability contracts.
 *
 * A quantum-device declaration describes WHAT a quantum-capable implementation
 * can provide or what a program requires from such an implementation.
 *
 * It does not identify, discover, calibrate, route, schedule, or control a
 * physical device.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - quantum-device declaration syntax;
 *   - QPU/device capability declarations;
 *   - logical-qubit capability declarations;
 *   - physical-qubit capability declarations;
 *   - quantum operation capability declarations;
 *   - gate-family capability declarations;
 *   - measurement capability declarations;
 *   - reset capability declarations;
 *   - dynamic-circuit capability declarations;
 *   - mid-circuit measurement capability declarations;
 *   - classical-control capability declarations;
 *   - quantum-device resource declarations;
 *   - quantum-device requirements;
 *   - quantum-device constraints;
 *   - quantum-device preferences;
 *   - quantum-device interfaces;
 *   - quantum-device parameters;
 *   - quantum-device properties;
 *   - quantum-device extensions.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This grammar does NOT own:
 *
 *   - lexer/token definitions;
 *   - general Zamani expressions;
 *   - canonical quantum AST;
 *   - quantum::ir;
 *   - quantum gate semantics;
 *   - quantum circuit semantics;
 *   - QEC algorithms;
 *   - QEC decoding;
 *   - ZQN/noise models;
 *   - calibration;
 *   - physical device discovery;
 *   - provider APIs;
 *   - device drivers;
 *   - physical topology discovery;
 *   - routing;
 *   - placement;
 *   - scheduling;
 *   - optimization;
 *   - pulse generation;
 *   - runtime execution;
 *   - resource accounting.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   ↓
 * ZamaniTokens
 *   ↓
 * ZamaniHardwareParser
 *   ↓
 * quantum-device syntax
 *   ↓
 * AST
 *   ↓
 * semantic analysis
 *   ↓
 * hardware capability/resource model
 *   ↓
 * quantum::ir where quantum computation is involved
 *   ↓
 * resource/capability resolution
 *   ↓
 * optimization
 *   ↓
 * routing
 *   ↓
 * scheduling
 *   ↓
 * hardware HAL
 *   ↓
 * runtime
 *
 * There must be no reverse dependency from this grammar to IR/runtime.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A quantum-device declaration describes requirements and capabilities rather
 * than a particular machine.
 *
 * Valid examples include concepts such as:
 *
 *   requires qubits >= algorithm_width;
 *   requires capability.dynamic_circuit;
 *   requires capability.mid_circuit_measurement;
 *   requires operation.entangling;
 *   prefer capability.fast_measurement;
 *
 * The grammar does NOT require:
 *
 *   - a fixed number of qubits;
 *   - fixed physical qubit IDs;
 *   - a fixed topology;
 *   - a fixed vendor;
 *   - a fixed backend;
 *   - a fixed calibration;
 *   - a fixed clock;
 *   - a fixed pulse duration;
 *   - a fixed device address.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately no grammar-level hardware limits.
 *
 * No:
 *
 *   MAX_QUBITS
 *   MAX_DEVICES
 *   MAX_OPERATIONS
 *   MAX_CHANNELS
 *   MAX_SHOTS
 *   MAX_CIRCUIT_DEPTH
 *
 * or equivalent restrictions exist here.
 *
 * Quantities are expressions.
 * Collections are structurally repeatable.
 * Physical capacity is resolved by the resource/capability subsystem.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical:
 *
 *   grammar/lexer/tokens.g4
 *
 * through:
 *
 *   ZamaniTokens
 *
 * No local lexer rules are permitted.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareQuantumDeviceParser;

options {
    tokenVocab = ZamaniTokens;
}


// ============================================================================
// 1. PUBLIC ENTRY POINT
// ============================================================================
//
// hardware.g4 delegates quantum-device declarations here.
//
// The parent hardware grammar decides where the declaration is legal.
// This grammar owns the contents of the declaration.
//
// ============================================================================

hardwareQuantumDeviceDecl
    : hardwareQuantumDeviceModifier*
      K_QUANTUM
      hardwareQuantumDeviceKind?
      IDENTIFIER
      hardwareQuantumDeviceGenerics?
      hardwareQuantumDeviceRequirementClause?
      hardwareQuantumDeviceCapabilityClause?
      LBRACE
          hardwareQuantumDeviceItem*
      RBRACE
    ;


// ============================================================================
// 2. DEVICE KIND
// ============================================================================
//
// The kind is intentionally extensible.
//
// Existing/core kinds can be represented using identifiers without requiring
// every future quantum technology to become a lexer keyword.
//
// Examples:
//
//   qpu
//   simulator
//   annealer
//   photonic
//   neutral_atom
//   ion_trap
//   superconducting
//   spin
//   topological
//
// The semantic layer determines whether a named kind is recognized.
//
// ============================================================================

hardwareQuantumDeviceKind
    : IDENTIFIER
    ;


// ============================================================================
// 3. MODIFIERS
// ============================================================================

hardwareQuantumDeviceModifier
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


// ============================================================================
// 4. GENERICS
// ============================================================================
//
// Generic quantum-device descriptions prevent source code from becoming tied
// to one machine size.
//
// Example:
//
//   quantum device Qpu<RequiredQubits, Precision> { ... }
//
// ============================================================================

hardwareQuantumDeviceGenerics
    : LT
      hardwareQuantumDeviceGenericParameter
      (
          COMMA
          hardwareQuantumDeviceGenericParameter
      )*
      GT
    ;

hardwareQuantumDeviceGenericParameter
    : IDENTIFIER
      (
          COLON
          hardwareQuantumDeviceGenericBound
      )?
      (
          ASSIGN
          hardwareQuantumDeviceExpression
      )?
    ;

hardwareQuantumDeviceGenericBound
    : hardwareQuantumDeviceQualifiedName
    | hardwareQuantumDeviceCapabilityReference
    ;


// ============================================================================
// 5. DEVICE BODY
// ============================================================================

hardwareQuantumDeviceItem
    : hardwareQuantumDeviceAnnotation*
      hardwareQuantumResourceDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumCapabilityDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumRequirementDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumConstraintDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumPreferenceDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumQubitDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumLogicalQubitDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumPhysicalQubitDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumOperationDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumGateSetDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumMeasurementDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumResetDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumDynamicCircuitDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumMidCircuitDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumClassicalControlDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumParameterizationDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumConnectivityDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumTimingDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumPrecisionDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumFidelityDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumErrorDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumMemoryDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumInterfaceDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumTargetDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumParameterDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumConstantDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumPropertyDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumCompositionDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumConnectionDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumExtensionDecl

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumAssertion

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumUsingDecl
    ;


// ============================================================================
// 6. ANNOTATIONS
// ============================================================================

hardwareQuantumDeviceAnnotation
    : AT
      hardwareQuantumDeviceQualifiedName
      (
          LPAREN
          hardwareQuantumDeviceAnnotationArguments?
          RPAREN
      )?
    ;

hardwareQuantumDeviceAnnotationArguments
    : hardwareQuantumDeviceAnnotationArgument
      (
          COMMA
          hardwareQuantumDeviceAnnotationArgument
      )*
    ;

hardwareQuantumDeviceAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareQuantumDeviceQualifiedName
    | hardwareQuantumDeviceExpression
    ;


// ============================================================================
// 7. QUBIT RESOURCE
// ============================================================================
//
// This is a capability/resource declaration, NOT physical allocation.
//
// Example:
//
//   qubit {
//       capacity = available_qubits;
//   }
//
// The expression may be compile-time, target-dependent, or runtime-derived.
// The semantic layer decides which.
//
// ============================================================================

hardwareQuantumQubitDecl
    : K_QUBIT
      IDENTIFIER?
      hardwareQuantumResourceBody
    ;


// ============================================================================
// 8. LOGICAL QUBITS
// ============================================================================

hardwareQuantumLogicalQubitDecl
    : K_LOGICAL
      K_QUBIT
      IDENTIFIER?
      hardwareQuantumCapabilityBody
    ;


// ============================================================================
// 9. PHYSICAL QUBIT CAPABILITIES
// ============================================================================
//
// This does NOT enumerate physical qubit IDs.
//
// It describes capabilities that a target may expose.
//
// ============================================================================

hardwareQuantumPhysicalQubitDecl
    : K_PHYSICAL
      K_QUBIT
      IDENTIFIER?
      hardwareQuantumCapabilityBody
    ;


// ============================================================================
// 10. QUANTUM OPERATIONS
// ============================================================================
//
// Operation names are symbolic.
//
// The grammar does not maintain a second gate/operation taxonomy.
// Canonical quantum semantics remain owned by quantum::ir.
//
// ============================================================================

hardwareQuantumOperationDecl
    : K_OPERATION
      IDENTIFIER?
      hardwareQuantumOperationBody
    ;

hardwareQuantumOperationBody
    : LBRACE
      hardwareQuantumOperationProperty*
      RBRACE
    ;

hardwareQuantumOperationProperty
    : K_SUPPORTED
      ASSIGN
      hardwareQuantumBooleanExpression
      SEMICOLON

    | K_ARITY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_DURATION
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_FIDELITY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_ERROR
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_PRECISION
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | hardwareQuantumDeviceKeyValue
      SEMICOLON
    ;


// ============================================================================
// 11. GATE SETS
// ============================================================================
//
// Gate names remain symbolic and extensible.
//
// ============================================================================

hardwareQuantumGateSetDecl
    : K_GATE
      IDENTIFIER?
      LBRACE
      hardwareQuantumGateSetItem*
      RBRACE
    ;

hardwareQuantumGateSetItem
    : hardwareQuantumDeviceQualifiedName
      (
          ASSIGN
          hardwareQuantumDeviceExpression
      )?
      SEMICOLON
    ;


// ============================================================================
// 12. MEASUREMENT
// ============================================================================

hardwareQuantumMeasurementDecl
    : K_MEASUREMENT
      IDENTIFIER?
      hardwareQuantumCapabilityBody
    ;


// ============================================================================
// 13. RESET
// ============================================================================

hardwareQuantumResetDecl
    : K_RESET
      IDENTIFIER?
      hardwareQuantumCapabilityBody
    ;


// ============================================================================
// 14. DYNAMIC CIRCUITS
// ============================================================================

hardwareQuantumDynamicCircuitDecl
    : K_DYNAMIC
      K_CIRCUIT
      IDENTIFIER?
      hardwareQuantumCapabilityBody
    ;


// ============================================================================
// 15. MID-CIRCUIT MEASUREMENT / CONTROL
// ============================================================================

hardwareQuantumMidCircuitDecl
    : K_MID
      K_CIRCUIT
      IDENTIFIER?
      hardwareQuantumCapabilityBody
    ;


// ============================================================================
// 16. CLASSICAL CONTROL
// ============================================================================
//
// Indicates support for classical feedback/control associated with quantum
// execution.
//
// This does not define the classical language or runtime.
//
// ============================================================================

hardwareQuantumClassicalControlDecl
    : K_CLASSICAL
      K_CONTROL
      IDENTIFIER?
      hardwareQuantumCapabilityBody
    ;


// ============================================================================
// 17. PARAMETERIZED OPERATIONS
// ============================================================================

hardwareQuantumParameterizationDecl
    : K_PARAMETERIZED
      K_OPERATION
      IDENTIFIER?
      hardwareQuantumParameterizationBody
    ;

hardwareQuantumParameterizationBody
    : LBRACE
      hardwareQuantumParameterizationItem*
      RBRACE
    ;

hardwareQuantumParameterizationItem
    : K_PARAMETER
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_RANGE
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_PRECISION
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      hardwareQuantumBooleanExpression
      SEMICOLON

    | hardwareQuantumDeviceKeyValue
      SEMICOLON
    ;


// ============================================================================
// 18. CONNECTIVITY
// ============================================================================
//
// Connectivity is an abstract capability/requirement.
//
// Physical graph construction belongs to topology/routing.
//
// ============================================================================

hardwareQuantumConnectivityDecl
    : K_CONNECTIVITY
      IDENTIFIER?
      hardwareQuantumAssignmentBody
    ;


// ============================================================================
// 19. TIMING
// ============================================================================
//
// No fixed clock rate, unit, or duration is imposed.
//
// ============================================================================

hardwareQuantumTimingDecl
    : K_TIMING
      IDENTIFIER?
      LBRACE
      hardwareQuantumTimingItem*
      RBRACE
    ;

hardwareQuantumTimingItem
    : K_LATENCY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_DURATION
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_PERIOD
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_FREQUENCY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_DEADLINE
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_SETUP
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_HOLD
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | hardwareQuantumDeviceKeyValue
      SEMICOLON
    ;


// ============================================================================
// 20. PRECISION
// ============================================================================

hardwareQuantumPrecisionDecl
    : K_PRECISION
      IDENTIFIER?
      hardwareQuantumAssignmentBody
    ;


// ============================================================================
// 21. FIDELITY
// ============================================================================
//
// No fixed threshold such as 0.95 is embedded in the grammar.
//
// ============================================================================

hardwareQuantumFidelityDecl
    : K_FIDELITY
      IDENTIFIER?
      hardwareQuantumAssignmentBody
    ;


// ============================================================================
// 22. ERROR PROPERTIES
// ============================================================================
//
// This describes requirements/properties only.
//
// QEC and ZQN remain downstream.
//
// ============================================================================

hardwareQuantumErrorDecl
    : K_ERROR
      IDENTIFIER?
      hardwareQuantumRequirementBody
    ;


// ============================================================================
// 23. QUANTUM MEMORY
// ============================================================================

hardwareQuantumMemoryDecl
    : K_MEMORY
      IDENTIFIER?
      hardwareQuantumResourceBody
    ;


// ============================================================================
// 24. INTERFACES
// ============================================================================

hardwareQuantumInterfaceDecl
    : K_INTERFACE
      IDENTIFIER
      hardwareQuantumInterfaceBody
    ;

hardwareQuantumInterfaceBody
    : LBRACE
      hardwareQuantumInterfaceItem*
      RBRACE
    ;

hardwareQuantumInterfaceItem
    : hardwareQuantumDevicePropertyDecl
    | hardwareQuantumCapabilityDecl
    | hardwareQuantumRequirementDecl
    | hardwareQuantumConstraintDecl
    | hardwareQuantumPreferenceDecl
    | hardwareQuantumParameterDecl
    | hardwareQuantumConnectionDecl
    ;


// ============================================================================
// 25. RESOURCES
// ============================================================================
//
// Resource quantities are expressions, not constants.
//
// ============================================================================

hardwareQuantumResourceDecl
    : K_RESOURCE
      IDENTIFIER?
      hardwareQuantumResourceBody
    ;

hardwareQuantumResourceBody
    : LBRACE
      hardwareQuantumResourceProperty*
      RBRACE
    ;

hardwareQuantumResourceProperty
    : K_CAPACITY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_QUANTITY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_SIZE
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_COUNT
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_WIDTH
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_DEPTH
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_BANDWIDTH
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_ENERGY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_RELIABILITY
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON

    | hardwareQuantumDeviceKeyValue
      SEMICOLON
    ;


// ============================================================================
// 26. CAPABILITIES
// ============================================================================

hardwareQuantumCapabilityDecl
    : K_CAPABILITY
      IDENTIFIER
      hardwareQuantumCapabilityBody?
      SEMICOLON?
    ;

hardwareQuantumCapabilityBody
    : LBRACE
      hardwareQuantumCapabilityItem*
      RBRACE
    ;

hardwareQuantumCapabilityItem
    : K_PROVIDES
      hardwareQuantumDeviceQualifiedName
      SEMICOLON

    | K_SUPPORTS
      hardwareQuantumDeviceQualifiedName
      SEMICOLON

    | K_VERSION
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_LEVEL
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_LIMIT
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_SUPPORTED
      ASSIGN
      hardwareQuantumBooleanExpression
      SEMICOLON

    | hardwareQuantumDeviceKeyValue
      SEMICOLON
    ;


// ============================================================================
// 27. REQUIREMENTS
// ============================================================================

hardwareQuantumRequirementDecl
    : K_REQUIRE
      IDENTIFIER?
      hardwareQuantumRequirementBody
    ;

hardwareQuantumRequirementBody
    : LBRACE
      hardwareQuantumRequirementItem*
      RBRACE
    ;

hardwareQuantumRequirementItem
    : K_CAPABILITY
      hardwareQuantumDeviceQualifiedName
      SEMICOLON

    | K_RESOURCE
      hardwareQuantumDeviceQualifiedName
      (
          hardwareQuantumComparisonOperator
          hardwareQuantumDeviceExpression
      )?
      SEMICOLON

    | hardwareQuantumDeviceQualifiedName
      hardwareQuantumComparisonOperator
      hardwareQuantumDeviceExpression
      SEMICOLON

    | hardwareQuantumDeviceKeyValue
      SEMICOLON
    ;


// ============================================================================
// 28. TOP-LEVEL REQUIREMENTS
// ============================================================================

hardwareQuantumDeviceRequirementClause
    : K_REQUIRES
      LBRACE
      hardwareQuantumRequirementItem*
      RBRACE
    ;


// ============================================================================
// 29. TOP-LEVEL CAPABILITIES
// ============================================================================

hardwareQuantumDeviceCapabilityClause
    : K_PROVIDES
      LBRACE
      hardwareQuantumProvidedCapability*
      RBRACE
    ;

hardwareQuantumProvidedCapability
    : hardwareQuantumDeviceQualifiedName
      (
          ASSIGN
          hardwareQuantumDeviceExpression
      )?
      SEMICOLON
    ;


// ============================================================================
// 30. CONSTRAINTS
// ============================================================================

hardwareQuantumConstraintDecl
    : K_CONSTRAINT
      IDENTIFIER?
      LBRACE
      hardwareQuantumConstraintItem*
      RBRACE
    ;

hardwareQuantumConstraintItem
    : hardwareQuantumDeviceExpression
      hardwareQuantumComparisonOperator
      hardwareQuantumDeviceExpression
      SEMICOLON

    | K_NOT
      hardwareQuantumDeviceExpression
      SEMICOLON

    | hardwareQuantumDeviceQualifiedName
      LPAREN
      hardwareQuantumArgumentList?
      RPAREN
      SEMICOLON
    ;


// ============================================================================
// 31. PREFERENCES
// ============================================================================

hardwareQuantumPreferenceDecl
    : K_PREFER
      LBRACE
      hardwareQuantumPreferenceItem*
      RBRACE
    ;

hardwareQuantumPreferenceItem
    : hardwareQuantumDeviceQualifiedName
      (
          ASSIGN
        | FAT_ARROW
      )
      hardwareQuantumDeviceExpression
      SEMICOLON
    ;


// ============================================================================
// 32. TARGET
// ============================================================================
//
// A target is a symbolic compilation target.
//
// This does not perform physical device selection.
//
// ============================================================================

hardwareQuantumTargetDecl
    : K_TARGET
      IDENTIFIER?
      hardwareQuantumAssignmentBody
    ;


// ============================================================================
// 33. PARAMETERS
// ============================================================================

hardwareQuantumParameterDecl
    : K_PARAMETER
      IDENTIFIER
      (
          COLON
          hardwareQuantumType
      )?
      (
          ASSIGN
          hardwareQuantumDeviceExpression
      )?
      SEMICOLON
    ;


// ============================================================================
// 34. CONSTANTS
// ============================================================================

hardwareQuantumConstantDecl
    : K_CONST
      IDENTIFIER
      COLON
      hardwareQuantumType
      ASSIGN
      hardwareQuantumDeviceExpression
      SEMICOLON
    ;


// ============================================================================
// 35. GENERIC PROPERTIES
// ============================================================================
//
// This is the forward-compatibility escape hatch.
//
// Future quantum technologies do not require immediate core grammar changes
// merely to express metadata.
//
// ============================================================================

hardwareQuantumDevicePropertyDecl
    : hardwareQuantumDeviceQualifiedName
      ASSIGN
      hardwareQuantumDeviceValue
      SEMICOLON
    ;

hardwareQuantumPropertyDecl
    : hardwareQuantumDeviceQualifiedName
      ASSIGN
      hardwareQuantumDeviceValue
      SEMICOLON
    ;


// ============================================================================
// 36. COMPOSITION
// ============================================================================

hardwareQuantumCompositionDecl
    : K_COMPOSE
      IDENTIFIER?
      LBRACE
      hardwareQuantumCompositionItem*
      RBRACE
    ;

hardwareQuantumCompositionItem
    : hardwareQuantumDeviceQualifiedName SEMICOLON
    | hardwareQuantumDevicePropertyDecl
    | hardwareQuantumCapabilityDecl
    | hardwareQuantumRequirementDecl
    ;


// ============================================================================
// 37. CONNECTION
// ============================================================================
//
// Abstract interface relationship only.
//
// ============================================================================

hardwareQuantumConnectionDecl
    : K_CONNECT
      hardwareQuantumDeviceQualifiedName
      THIN_ARROW
      hardwareQuantumDeviceQualifiedName
      SEMICOLON
    ;


// ============================================================================
// 38. EXTENSIONS
// ============================================================================

hardwareQuantumExtensionDecl
    : K_EXTEND
      hardwareQuantumDeviceQualifiedName
      LBRACE
      hardwareQuantumExtensionItem*
      RBRACE
    ;

hardwareQuantumExtensionItem
    : hardwareQuantumDevicePropertyDecl
    | hardwareQuantumCapabilityDecl
    | hardwareQuantumRequirementDecl
    | hardwareQuantumConstraintDecl
    | hardwareQuantumPreferenceDecl
    ;


// ============================================================================
// 39. ASSERTIONS
// ============================================================================

hardwareQuantumAssertion
    : K_ASSERT
      LPAREN
      hardwareQuantumDeviceExpression
      RPAREN
      SEMICOLON
    ;


// ============================================================================
// 40. USING
// ============================================================================

hardwareQuantumUsingDecl
    : K_USING
      hardwareQuantumDeviceQualifiedName
      SEMICOLON
    ;


// ============================================================================
// 41. ASSIGNMENT BODY
// ============================================================================

hardwareQuantumAssignmentBody
    : ASSIGN
      hardwareQuantumDeviceValue
      SEMICOLON

    | LBRACE
      hardwareQuantumPropertyItem*
      RBRACE
    ;

hardwareQuantumPropertyItem
    : hardwareQuantumDevicePropertyDecl
    | hardwareQuantumCapabilityDecl
    | hardwareQuantumRequirementDecl
    | hardwareQuantumConstraintDecl
    | hardwareQuantumPreferenceDecl
    ;


// ============================================================================
// 42. VALUES
// ============================================================================

hardwareQuantumDeviceValue
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | K_TRUE
    | K_FALSE
    | hardwareQuantumDeviceQualifiedName
    | hardwareQuantumDeviceExpression
    | hardwareQuantumList
    | hardwareQuantumMap
    ;


// ============================================================================
// 43. LISTS
// ============================================================================

hardwareQuantumList
    : LBRACKET
      (
          hardwareQuantumDeviceValue
          (
              COMMA
              hardwareQuantumDeviceValue
          )*
      )?
      RBRACKET
    ;


// ============================================================================
// 44. MAPS
// ============================================================================

hardwareQuantumMap
    : LBRACE
      (
          hardwareQuantumMapEntry
          (
              COMMA
              hardwareQuantumMapEntry
          )*
      )?
      RBRACE
    ;

hardwareQuantumMapEntry
    : hardwareQuantumDeviceQualifiedName
      COLON
      hardwareQuantumDeviceValue
    ;


// ============================================================================
// 45. NAMES
// ============================================================================

hardwareQuantumDeviceQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


// ============================================================================
// 46. TYPES
// ============================================================================

hardwareQuantumType
    : hardwareQuantumDeviceQualifiedName
    | K_INT
    | K_FLOAT
    | K_BOOL
    | K_STRING
    | K_STR
    ;


// ============================================================================
// 47. CAPABILITY REFERENCES
// ============================================================================

hardwareQuantumDeviceCapabilityReference
    : hardwareQuantumDeviceQualifiedName
    ;


// ============================================================================
// 48. COMPARISON
// ============================================================================

hardwareQuantumComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_EQUAL
    | GREATER_EQUAL
    | LT
    | GT
    ;


// ============================================================================
// 49. EXPRESSIONS
// ============================================================================
//
// These are intentionally symbolic.
//
// IMPORTANT INTEGRATION NOTE:
//
// When the common expression grammar becomes the authoritative shared
// expression delegate, this local expression family must be replaced by the
// shared rule rather than maintained as a second semantic expression system.
//
// Until that shared parser contract is finalized, this grammar keeps its
// expression syntax self-contained so the quantum-device grammar remains
// independently completable.
//
// ============================================================================

hardwareQuantumDeviceExpression
    : hardwareQuantumLogicalOrExpression
    ;

hardwareQuantumLogicalOrExpression
    : hardwareQuantumLogicalAndExpression
      (
          LOGICAL_OR
          hardwareQuantumLogicalAndExpression
      )*
    ;

hardwareQuantumLogicalAndExpression
    : hardwareQuantumEqualityExpression
      (
          LOGICAL_AND
          hardwareQuantumEqualityExpression
      )*
    ;

hardwareQuantumEqualityExpression
    : hardwareQuantumRelationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          hardwareQuantumRelationalExpression
      )*
    ;

hardwareQuantumRelationalExpression
    : hardwareQuantumAdditiveExpression
      (
          (
              LESS_THAN
            | LESS_EQUAL
            | GREATER_THAN
            | GREATER_EQUAL
          )
          hardwareQuantumAdditiveExpression
      )*
    ;

hardwareQuantumAdditiveExpression
    : hardwareQuantumMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          hardwareQuantumMultiplicativeExpression
      )*
    ;

hardwareQuantumMultiplicativeExpression
    : hardwareQuantumUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          hardwareQuantumUnaryExpression
      )*
    ;

hardwareQuantumUnaryExpression
    : (
          PLUS
        | MINUS
        | EXCLAMATION
      )
      hardwareQuantumUnaryExpression
    | hardwareQuantumPrimaryExpression
    ;

hardwareQuantumPrimaryExpression
    : IDENTIFIER
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | K_TRUE
    | K_FALSE
    | hardwareQuantumDeviceQualifiedName
    | LPAREN
      hardwareQuantumDeviceExpression
      RPAREN
    ;

hardwareQuantumBooleanExpression
    : hardwareQuantumDeviceExpression
    ;


// ============================================================================
// 50. ARGUMENTS
// ============================================================================

hardwareQuantumArgumentList
    : hardwareQuantumDeviceExpression
      (
          COMMA
          hardwareQuantumDeviceExpression
      )*
    ;


// ============================================================================
// 51. GENERIC KEY/VALUE
// ============================================================================

hardwareQuantumDeviceKeyValue
    : hardwareQuantumDeviceQualifiedName
      ASSIGN
      hardwareQuantumDeviceValue
    ;