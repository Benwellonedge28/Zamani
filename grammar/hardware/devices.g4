/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/devices.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded target-language actions.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for logical hardware-device
 * declarations and device-related intent.
 *
 * A device declaration describes what a computational device IS from the
 * program's semantic perspective and what capabilities/resources/requirements
 * are associated with that logical device.
 *
 * It does NOT enumerate physical devices.
 *
 * Examples of things this grammar can describe:
 *
 *     - CPU-class devices
 *     - GPU-class devices
 *     - FPGA-class devices
 *     - ASIC-class devices
 *     - quantum-device-class devices
 *     - accelerator-class devices
 *     - memory devices
 *     - storage devices
 *     - communication devices
 *     - heterogeneous devices
 *     - user-defined device classes
 *     - logical device groups
 *     - device interfaces
 *     - device capabilities
 *     - device resources
 *     - device requirements
 *     - device constraints
 *     - device preferences
 *     - device relationships
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - numeric literals;
 *     - physical-device discovery;
 *     - hardware enumeration;
 *     - calibration;
 *     - topology discovery;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - device drivers;
 *     - runtime dispatch;
 *     - resource allocation;
 *     - physical addresses;
 *     - physical device IDs;
 *     - vendor-specific machine definitions;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN/noise semantics;
 *     - canonical AST implementation;
 *     - semantic type checking.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Device declarations describe SEMANTIC DEVICE REQUIREMENTS AND CAPABILITIES.
 *
 * They must never require a source program to be rewritten merely because
 * execution moves between:
 *
 *     - different CPUs;
 *     - different CPU counts;
 *     - different GPUs;
 *     - different GPU counts;
 *     - different FPGAs;
 *     - different ASICs;
 *     - different quantum processors;
 *     - different quantum simulators;
 *     - different accelerators;
 *     - different clusters;
 *     - different clouds;
 *     - different future architectures.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains:
 *
 *     NO MAX_DEVICES
 *     NO MAX_CORES
 *     NO MAX_THREADS
 *     NO MAX_QUBITS
 *     NO MAX_MEMORY
 *     NO MAX_ACCELERATORS
 *     NO MAX_PORTS
 *     NO MAX_CAPABILITIES
 *     NO MAX_RESOURCES
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Quantities are represented by expressions.
 *
 * Physical availability is resolved downstream by:
 *
 *     semantic analysis
 *         ->
 *     capability/resource resolution
 *         ->
 *     hardware abstraction
 *         ->
 *     routing/scheduling
 *         ->
 *     runtime/discovery.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * lexer/tokens.g4
 *   |
 *   v
 * devices.g4
 *   |
 *   v
 * syntax tree / AST
 *   |
 *   v
 * semantic analysis
 *   |
 *   +-------------------------------+
 *   |                               |
 *   v                               v
 * logical device model          resource model
 *   |                               |
 *   +---------------+---------------+
 *                   |
 *                   v
 *          capability resolution
 *                   |
 *                   v
 *        hardware abstraction layer
 *                   |
 *          +--------+---------+
 *          |                  |
 *          v                  v
 *       routing          scheduling
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *              runtime
 *
 * ============================================================================
 * IMPORTANT QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum devices may be declared here as hardware capabilities.
 *
 * This file MUST NOT create a quantum IR.
 *
 * For example:
 *
 *     device QuantumProcessor {
 *         capability quantum;
 *         capability dynamic_circuit;
 *     }
 *
 * is a hardware declaration.
 *
 * The resulting quantum computation is lowered through the canonical
 * quantum::ir boundary elsewhere in the repository.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareDevicesParser;

options {
    tokenVocab = ZamaniTokens;
}


// ============================================================================
// 1. PUBLIC ENTRY POINT
// ============================================================================
//
// This is the integration point consumed by the hardware parser/root parser.
//
// A root parser should expose:
//
//     deviceDeclaration
//
// through its hardware declaration alternatives.
//
// ============================================================================

deviceDeclaration
    : deviceAnnotation*
      deviceVisibility?
      deviceModifier*
      K_DEVICE
      IDENTIFIER
      deviceGenericParameters?
      deviceClassClause?
      deviceRequirementClause?
      deviceCapabilityClause?
      deviceResourceClause?
      deviceConstraintClause?
      devicePreferenceClause?
      LBRACE deviceMember* RBRACE
    ;


// ============================================================================
// 2. VISIBILITY
// ============================================================================

deviceVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


// ============================================================================
// 3. MODIFIERS
// ============================================================================
//
// Modifiers describe source-level properties only.
//
// They do not imply a physical implementation.
//

deviceModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


// ============================================================================
// 4. ANNOTATIONS
// ============================================================================
//
// Device annotations are extensibility metadata.
//
// Vendor-specific annotations are represented through identifiers rather than
// being hard-coded into the core language.
//

deviceAnnotation
    : AT IDENTIFIER
      (LPAREN deviceAnnotationArguments? RPAREN)?
    ;

deviceAnnotationArguments
    : deviceAnnotationArgument
      (COMMA deviceAnnotationArgument)*
    ;

deviceAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | deviceQualifiedName
    | deviceExpression
    ;


// ============================================================================
// 5. GENERIC DEVICE PARAMETERS
// ============================================================================
//
// Generic parameters are essential for POCO-REAF.
//
// Example:
//
//     device Accelerator<Width, Lanes, Capacity> { ... }
//
// No concrete machine size is encoded in the grammar.
//

deviceGenericParameters
    : LT deviceGenericParameter
      (COMMA deviceGenericParameter)*
      GT
    ;

deviceGenericParameter
    : IDENTIFIER
      deviceGenericBound?
      deviceGenericDefault?
    ;

deviceGenericBound
    : COLON deviceQualifiedName
    ;

deviceGenericDefault
    : ASSIGN deviceExpression
    ;


// ============================================================================
// 6. DEVICE CLASS
// ============================================================================
//
// Device class is semantic classification, not physical enumeration.
//
// Examples:
//
//     device CPUDevice class cpu
//     device GPUDevice class accelerator
//     device QuantumDevice class quantum
//
// A future implementation may introduce dedicated tokens for well-known
// classes. The grammar therefore also permits qualified identifiers.
//

deviceClassClause
    : K_CLASS deviceQualifiedName
    ;


// ============================================================================
// 7. DEVICE MEMBERS
// ============================================================================

deviceMember
    : deviceAnnotation* deviceIdentityMember
    | deviceAnnotation* deviceInterfaceMember
    | deviceAnnotation* deviceCapabilityMember
    | deviceAnnotation* deviceRequirementMember
    | deviceAnnotation* deviceResourceMember
    | deviceAnnotation* deviceConstraintMember
    | deviceAnnotation* devicePreferenceMember
    | deviceAnnotation* deviceTargetMember
    | deviceAnnotation* devicePropertyMember
    | deviceAnnotation* deviceParameterMember
    | deviceAnnotation* deviceGenericMember
    | deviceAnnotation* deviceRelationshipMember
    | deviceAnnotation* deviceCompositionMember
    | deviceAnnotation* deviceImplementationMember
    ;


// ============================================================================
// 8. DEVICE IDENTITY
// ============================================================================
//
// This is a LOGICAL source-level identity.
//
// It must not be confused with:
//
//     - serial numbers;
//     - physical addresses;
//     - runtime handles;
//     - provider backend IDs.
//
// Those belong to hardware discovery/runtime layers.
//

deviceIdentityMember
    : K_ID ASSIGN deviceExpression SEMICOLON
    | K_NAME ASSIGN STRING_LITERAL SEMICOLON
    | K_VERSION ASSIGN deviceExpression SEMICOLON
    ;


// ============================================================================
// 9. INTERFACES
// ============================================================================
//
// Interfaces describe what the device exposes to programs.
//
// They do not select a physical implementation.
//

deviceInterfaceMember
    : K_INTERFACE deviceQualifiedName
      deviceInterfaceArguments?
      SEMICOLON
    ;

deviceInterfaceArguments
    : LPAREN deviceArgumentList? RPAREN
    ;


// ============================================================================
// 10. CAPABILITIES
// ============================================================================
//
// Capabilities describe what a device can provide.
//
// Capability != requirement.
// Capability != preference.
// Capability != target.
//
// This distinction is critical for POCO-REAF.
//

deviceCapabilityClause
    : K_CAPABILITY
      LBRACE
      deviceCapabilityEntry*
      RBRACE
    ;

deviceCapabilityMember
    : K_CAPABILITY
      deviceCapabilityReference
      deviceCapabilityArguments?
      SEMICOLON
    ;

deviceCapabilityEntry
    : deviceCapabilityReference
      deviceCapabilityArguments?
      SEMICOLON
    ;

deviceCapabilityReference
    : deviceQualifiedName
    ;

deviceCapabilityArguments
    : LPAREN deviceArgumentList? RPAREN
    ;


// ============================================================================
// 11. REQUIREMENTS
// ============================================================================
//
// Requirements express what an implementation MUST provide.
//
// They do not select a specific physical device.
//

deviceRequirementClause
    : K_REQUIRES
      LBRACE
      deviceRequirementEntry*
      RBRACE
    ;

deviceRequirementMember
    : K_REQUIRES
      deviceRequirementExpression
      SEMICOLON
    ;

deviceRequirementEntry
    : deviceRequirementExpression
      SEMICOLON
    ;

deviceRequirementExpression
    : deviceCapabilityReference
    | deviceResourceReference
    | deviceComparisonExpression
    | deviceLogicalExpression
    ;


// ============================================================================
// 12. RESOURCES
// ============================================================================
//
// Resources describe abstract resource requirements or provisions.
//
// Examples:
//
//     resource compute = available_compute;
//
//     resource memory >= required_memory;
//
//     resource quantum_capacity >= requested_capacity;
//
// The grammar imposes no physical resource ceiling.
//

deviceResourceClause
    : K_RESOURCE
      LBRACE
      deviceResourceEntry*
      RBRACE
    ;

deviceResourceMember
    : K_RESOURCE
      deviceResourceReference
      deviceResourceValue?
      SEMICOLON
    ;

deviceResourceEntry
    : deviceResourceReference
      deviceResourceValue?
      SEMICOLON
    ;

deviceResourceReference
    : deviceQualifiedName
    ;

deviceResourceValue
    : ASSIGN deviceExpression
    | deviceComparisonOperator deviceExpression
    ;


// ============================================================================
// 13. CONSTRAINTS
// ============================================================================
//
// Constraints are stronger than preferences.
//
// They constrain valid implementations but do not necessarily prescribe
// exactly how the hardware is realized.
//

deviceConstraintClause
    : K_CONSTRAINT
      LBRACE
      deviceConstraintEntry*
      RBRACE
    ;

deviceConstraintMember
    : K_CONSTRAINT
      deviceConstraintExpression
      SEMICOLON
    ;

deviceConstraintEntry
    : deviceConstraintExpression
      SEMICOLON
    ;

deviceConstraintExpression
    : deviceComparisonExpression
    | deviceLogicalExpression
    | deviceQualifiedName
    ;


// ============================================================================
// 14. PREFERENCES
// ============================================================================
//
// Preferences are optimization hints, NOT semantic requirements.
//
// A compiler/runtime may ignore a preference when necessary.
//

devicePreferenceClause
    : K_PREFER
      LBRACE
      devicePreferenceEntry*
      RBRACE
    ;

devicePreferenceMember
    : K_PREFER
      devicePreferenceExpression
      SEMICOLON
    ;

devicePreferenceEntry
    : devicePreferenceExpression
      SEMICOLON
    ;

devicePreferenceExpression
    : deviceQualifiedName
    | deviceComparisonExpression
    | deviceKeyValue
    ;


// ============================================================================
// 15. TARGET INTENT
// ============================================================================
//
// A target declaration is semantic compilation intent.
//
// It MUST NOT require a physical device identifier.
//
// Physical target resolution belongs downstream.
//

deviceTargetMember
    : K_TARGET
      deviceTargetExpression
      SEMICOLON
    ;

deviceTargetExpression
    : deviceQualifiedName
    | deviceTargetSet
    | deviceTargetPredicate
    ;

deviceTargetSet
    : LBRACE
      deviceQualifiedName
      (COMMA deviceQualifiedName)*
      RBRACE
    ;

deviceTargetPredicate
    : LPAREN
      deviceExpression
      RPAREN
    ;


// ============================================================================
// 16. PROPERTIES
// ============================================================================
//
// Properties are descriptive attributes.
//
// They do not automatically become runtime behavior.
//

devicePropertyMember
    : IDENTIFIER
      ASSIGN
      deviceExpression
      SEMICOLON
    ;


// ============================================================================
// 17. PARAMETERS
// ============================================================================
//
// Parameters are source-level symbolic values.
//
// They can represent dimensions, capacities, timing quantities, etc.
//
// No parser-level maximum is imposed.
//

deviceParameterMember
    : K_PARAMETER
      IDENTIFIER
      deviceTypeAnnotation?
      deviceParameterDefault?
      SEMICOLON
    ;

deviceParameterDefault
    : ASSIGN deviceExpression
    ;

deviceGenericMember
    : K_TYPE
      IDENTIFIER
      (ASSIGN deviceTypeExpression)?
      SEMICOLON
    ;


// ============================================================================
// 18. DEVICE RELATIONSHIPS
// ============================================================================
//
// Relationships express logical relationships between devices.
//
// They do not discover physical topology.
//

deviceRelationshipMember
    : deviceRelationshipKeyword
      deviceQualifiedName
      SEMICOLON
    ;

deviceRelationshipKeyword
    : K_PARENT
    | K_CHILD
    | K_COMPOSES
    | K_CONTAINS
    | K_DEPENDS
    | K_REQUIRES
    | K_PROVIDES
    ;


// ============================================================================
// 19. DEVICE COMPOSITION
// ============================================================================
//
// Composition describes a logical device made from other logical devices.
//
// Quantity is an expression and therefore remains scalable.
//

deviceCompositionMember
    : K_COMPOSE
      deviceQualifiedName
      deviceCompositionQuantity?
      deviceAliasClause?
      SEMICOLON
    ;

deviceCompositionQuantity
    : LBRACKET
      deviceExpression
      RBRACKET
    ;

deviceAliasClause
    : K_AS IDENTIFIER
    ;


// ============================================================================
// 20. IMPLEMENTATION INTENT
// ============================================================================
//
// Implementation intent is deliberately abstract.
//
// A semantic compiler may lower this into FPGA synthesis, CPU code,
// GPU kernels, ASIC generation, quantum execution, etc.
//

deviceImplementationMember
    : K_IMPLEMENT
      deviceQualifiedName
      deviceImplementationArguments?
      SEMICOLON
    ;

deviceImplementationArguments
    : LPAREN
      deviceArgumentList?
      RPAREN
    ;


// ============================================================================
// 21. DEVICE ARGUMENTS
// ============================================================================

deviceArgumentList
    : deviceArgument
      (COMMA deviceArgument)*
    ;

deviceArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | deviceQualifiedName
    | deviceExpression
    ;


// ============================================================================
// 22. QUALIFIED NAMES
// ============================================================================
//
// Qualified names provide extensibility:
//
//     vendor.domain.Device
//     quantum.processor
//     accelerator.tensor
//
// The grammar does not encode vendor/device names.
//

deviceQualifiedName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


// ============================================================================
// 23. TYPES
// ============================================================================
//
// Device types remain abstract.
//
// Physical representation belongs to target lowering.
//

deviceTypeAnnotation
    : COLON deviceTypeExpression
    ;

deviceTypeExpression
    : deviceQualifiedName
    | deviceGenericType
    ;

deviceGenericType
    : deviceQualifiedName
      LT
      deviceTypeArgument
      (COMMA deviceTypeArgument)*
      GT
    ;

deviceTypeArgument
    : deviceTypeExpression
    | deviceExpression
    ;


// ============================================================================
// 24. EXPRESSIONS
// ============================================================================
//
// Device expressions intentionally remain generic enough to represent:
//
//     dimensions
//     capacities
//     resource quantities
//     timing
//     symbolic constraints
//     target predicates
//     capability arguments
//
// The semantic layer determines their actual types and evaluation mode.
//

deviceExpression
    : deviceLogicalExpression
    ;

deviceLogicalExpression
    : deviceComparisonExpression
      (
          LOGICAL_AND deviceComparisonExpression
        | LOGICAL_OR deviceComparisonExpression
      )*
    ;

deviceComparisonExpression
    : deviceAdditiveExpression
      (
          deviceComparisonOperator
          deviceAdditiveExpression
      )?
    ;

deviceComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;

deviceAdditiveExpression
    : deviceMultiplicativeExpression
      (
          PLUS deviceMultiplicativeExpression
        | MINUS deviceMultiplicativeExpression
      )*
    ;

deviceMultiplicativeExpression
    : deviceUnaryExpression
      (
          STAR deviceUnaryExpression
        | SLASH deviceUnaryExpression
        | PERCENT deviceUnaryExpression
      )*
    ;

deviceUnaryExpression
    : MINUS deviceUnaryExpression
    | PLUS deviceUnaryExpression
    | K_NOT deviceUnaryExpression
    | devicePrimaryExpression
    ;

devicePrimaryExpression
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | K_TRUE
    | K_FALSE
    | IDENTIFIER
    | deviceQualifiedName
    | deviceCallExpression
    | deviceIndexExpression
    | deviceMemberAccessExpression
    | LPAREN deviceExpression RPAREN
    ;


// ============================================================================
// 25. CALLS
// ============================================================================

deviceCallExpression
    : deviceQualifiedName
      LPAREN
      deviceArgumentList?
      RPAREN
    ;


// ============================================================================
// 26. INDEXING
// ============================================================================
//
// Indexing remains symbolic.
//
// No fixed index range is encoded.
//

deviceIndexExpression
    : deviceQualifiedName
      LBRACKET
      deviceExpression
      RBRACKET
    ;


// ============================================================================
// 27. MEMBER ACCESS
// ============================================================================

deviceMemberAccessExpression
    : deviceQualifiedName
      DOT
      IDENTIFIER
    ;


// ============================================================================
// 28. KEY/VALUE EXTENSION POINT
// ============================================================================
//
// This allows future capabilities/resources to be introduced without
// changing the core grammar every time a new implementation-specific
// property appears.
//

deviceKeyValue
    : IDENTIFIER
      ASSIGN
      deviceExpression
    ;


// ============================================================================
// 29. DEVICE DECLARATION ALIASES / REFERENCES
// ============================================================================
//
// A reference is intentionally distinct from a physical runtime handle.
//

deviceReference
    : deviceQualifiedName
    ;


// ============================================================================
// 30. RESERVED SEMANTIC DEVICE CATEGORIES
// ============================================================================
//
// These are intentionally represented as qualified identifiers rather than
// fixed hardware counts or fixed device names.
//
// Semantic analysis may recognize standard categories such as:
//
//     cpu
//     gpu
//     fpga
//     asic
//     quantum
//     accelerator
//     memory
//     storage
//     network
//     controller
//
// New categories do not require grammar changes.
//

deviceCategory
    : deviceQualifiedName
    ;


// ============================================================================
// 31. RESOURCE-CAPABILITY RELATION
// ============================================================================
//
// A capability can be associated with a resource without coupling the grammar
// to one particular hardware architecture.
//

deviceCapabilityResourceRelation
    : deviceCapabilityReference
      deviceResourceReference
      SEMICOLON
    ;


// ============================================================================
// 32. DEVICE PREDICATES
// ============================================================================
//
// Predicates are semantic expressions.
//
// They are deliberately not tied to a physical discovery API.
//

devicePredicate
    : deviceExpression
    ;


// ============================================================================
// 33. EXTENSIBLE DEVICE ATTRIBUTE
// ============================================================================

deviceAttribute
    : IDENTIFIER
      (
          ASSIGN deviceExpression
        | LPAREN deviceArgumentList? RPAREN
      )
    ;


// ============================================================================
// 34. INTEGRATION CONTRACT
// ============================================================================
/*
 *
 * REQUIRED UPSTREAM:
 *
 *     grammar/lexer/tokens.g4
 *
 * The lexer must provide the stable hardware/resource vocabulary used by
 * this parser.
 *
 * At minimum, the hardware lexical contract must provide:
 *
 *     K_DEVICE
 *     K_CAPABILITY
 *     K_REQUIRES
 *     K_RESOURCE
 *     K_CONSTRAINT
 *     K_PREFER
 *     K_TARGET
 *     K_PARAMETER
 *     K_CLASS
 *     K_COMPOSE
 *     K_IMPLEMENT
 *     K_PARENT
 *     K_CHILD
 *     K_DEPENDS
 *     K_PROVIDES
 *     K_CONTAINS
 *     K_ID
 *     K_NAME
 *     K_VERSION
 *
 * together with the common punctuation/operator/literal tokens used above.
 *
 * IMPORTANT:
 *
 * The lexical layer owns those tokens.
 * This parser must never redefine them.
 *
 *
 * REQUIRED ROOT-PARSER INTEGRATION:
 *
 * The root Zamani parser must expose:
 *
 *     deviceDeclaration
 *
 * as one of its hardware/source declaration alternatives.
 *
 * Conceptually:
 *
 *     declaration
 *         : ...
 *         | hardwareDeclaration
 *         | deviceDeclaration
 *         | ...
 *         ;
 *
 * The exact root integration belongs to the root parser and is deliberately
 * NOT duplicated here.
 *
 *
 * HARDWARE.G4 INTEGRATION:
 *
 * hardware.g4 owns:
 *
 *     hardware modules
 *     hardware ports
 *     signals
 *     wires
 *     nets
 *     clocks
 *     timing
 *     processes
 *     state machines
 *     pipelines
 *     hardware implementation structure
 *
 * devices.g4 owns:
 *
 *     logical devices
 *     device classification
 *     device capability declarations
 *     device requirements
 *     device resources
 *     device constraints
 *     device preferences
 *     logical composition
 *     logical device relationships
 *
 * Neither file should redefine the other's semantic model.
 *
 *
 * RESOURCE INTEGRATION:
 *
 * Device resource expressions are lowered into the repository's canonical
 * resource model.
 *
 * This grammar does not allocate resources.
 *
 *
 * HARDWARE HAL INTEGRATION:
 *
 * A device declaration may eventually resolve to a Hardware HAL capability
 * or target description.
 *
 * This grammar does not discover or select the physical device.
 *
 *
 * QUANTUM INTEGRATION:
 *
 * Quantum device declarations describe capabilities only.
 *
 * Quantum computation itself is lowered through quantum::ir.
 *
 * This grammar does not define quantum operations or a second quantum IR.
 *
 *
 * QEC / ZQN:
 *
 * No QEC algorithm is defined here.
 *
 * No noise model is defined here.
 *
 * Device noise/fault semantics belong to ZQN.
 *
 * Error correction belongs to QEC.
 *
 *
 * ROUTING:
 *
 * Device topology requirements may be expressed as semantic requirements,
 * but physical topology resolution belongs to routing/hardware layers.
 *
 *
 * SCHEDULING:
 *
 * Timing/latency/resource constraints may be expressed symbolically.
 *
 * Scheduling decides actual execution order and timing.
 *
 *
 * OPTIMIZATION:
 *
 * Preferences are optimization inputs, not optimization decisions.
 *
 *
 * RUNTIME:
 *
 * Runtime may resolve logical device requirements against discovered
 * capabilities.
 *
 * Runtime handles and physical identifiers must never become required syntax
 * of a portable device declaration.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * Therefore:
 *
 *     - no unsafe Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment access;
 *     - no device probing;
 *     - no runtime side effects.
 *
 * Generated Rust must remain compatible with the repository's Rust 1.97 /
 * Rust 1.97.1 policy and its prohibition on unsafe code.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_DEVICE
 *     MAX_DEVICES
 *     MAX_RESOURCE
 *     MAX_RESOURCES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *
 * Physical quantities are expressions.
 *
 * Collections are unbounded by the grammar itself.
 *
 * Actual limits are imposed only by:
 *
 *     - available resources;
 *     - semantic validation;
 *     - compilation limits;
 *     - runtime capabilities;
 *     - explicit user constraints.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic for a fixed token stream.
 *
 * Semantic resolution must not be performed in this grammar.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] ANTLR accepts the grammar.
 *
 * [ ] The declared token vocabulary exists in ZamaniTokens.
 *
 * [ ] No embedded actions exist.
 *
 * [ ] No machine-size constants exist.
 *
 * [ ] Device identity remains distinct from physical device identity.
 *
 * [ ] Capability, requirement, resource, constraint and preference remain
 *     distinct syntactic concepts.
 *
 * [ ] Device declarations can describe arbitrary logical device classes.
 *
 * [ ] Generic symbolic quantities are supported.
 *
 * [ ] CPU/GPU/FPGA/ASIC/quantum/accelerator concepts do not impose fixed
 *     machine sizes.
 *
 * [ ] Physical discovery is outside this grammar.
 *
 * [ ] Hardware HAL integration is downstream.
 *
 * [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 * [ ] QEC and ZQN remain outside this grammar.
 *
 * [ ] Routing remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] Optimization remains downstream.
 *
 * [ ] Runtime dispatch remains downstream.
 *
 * [ ] Positive parser tests exist.
 *
 * [ ] Negative parser tests exist.
 *
 * [ ] Boundary/scalability tests exist.
 *
 * [ ] Cross-domain hardware/quantum/classical tests exist.
 *
 * [ ] Deterministic parsing tests exist.
 *
 * ============================================================================
 */