/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/hardware.g4
 *
 * Grammar:
 *     ZamaniHardwareParser
 *
 * Purpose:
 *     Defines the machine-independent hardware contract layer of Zamani.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Action-free ANTLR grammar.
 *     No embedded Rust.
 *     No embedded target-language code.
 *     No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - hardware declarations;
 *   - hardware composition;
 *   - abstract hardware modules;
 *   - hardware interfaces;
 *   - ports and connections at the hardware-contract level;
 *   - hardware resources;
 *   - hardware capabilities;
 *   - hardware requirements;
 *   - hardware constraints;
 *   - hardware preferences;
 *   - hardware targets;
 *   - target-independent placement intent;
 *   - target-independent topology requirements;
 *   - accelerator contracts;
 *   - CPU/GPU/FPGA/ASIC contracts;
 *   - QPU delegation;
 *   - hardware instances;
 *   - hardware mappings;
 *   - resource-independent dimensions;
 *   - hardware properties;
 *   - hardware extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical recognition;
 *   - identifiers;
 *   - numeric literals;
 *   - general expression semantics;
 *   - canonical AST implementation;
 *   - semantic type checking;
 *   - HDL behavioral semantics;
 *   - synthesis;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - hardware discovery;
 *   - calibration;
 *   - physical device enumeration;
 *   - physical device IDs;
 *   - physical addresses;
 *   - QPU gate semantics;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - runtime dispatch;
 *   - device drivers.
 *
 * ============================================================================
 * CRITICAL BOUNDARY
 * ============================================================================
 *
 * hardware.g4 describes WHAT hardware capability or relationship is required.
 *
 * It does not decide WHICH physical machine supplies it.
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
 * syntax AST
 *   |
 *   v
 * semantic analysis
 *   |
 *   +--> type/capability/resource checking
 *   |
 *   +--> hardware semantic model
 *   |
 *   +--> quantum semantic model
 *   |
 *   v
 * canonical compiler representations
 *   |
 *   +--> optimization
 *   +--> routing
 *   +--> scheduling
 *   +--> hardware HAL
 *   +--> runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The grammar must permit:
 *
 *     hardware
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     resource
 *     target
 *     topology requirement
 *     placement intent
 *
 * without requiring:
 *
 *     fixed device count
 *     fixed core count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed qubit count
 *     fixed memory size
 *     fixed topology
 *     fixed clock frequency
 *     fixed device ID
 *     fixed physical address
 *     fixed vendor
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level maximums.
 *
 * The grammar does not define:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_PORTS
 *     MAX_CONNECTIONS
 *     MAX_RESOURCES
 *
 * All quantities are expressions.
 *
 * Actual representability is determined by:
 *
 *     semantic analysis
 *     resource resolution
 *     target capabilities
 *     compiler policy
 *     runtime availability
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULES
 * ============================================================================
 *
 * 1. ZamaniTokens is the only lexer.
 *
 * 2. General expressions come from the canonical expression grammar.
 *
 * 3. General types come from the canonical type grammar.
 *
 * 4. HDL behavioral syntax belongs to grammar/hdl.
 *
 * 5. QPU syntax belongs to grammar/hardware/qpu.g4.
 *
 * 6. hardware.g4 delegates QPU declarations rather than redefining them.
 *
 * 7. This grammar never creates quantum::ir.
 *
 * 8. This grammar never performs target discovery.
 *
 * 9. This grammar never performs scheduling/routing/optimization.
 *
 * 10. Vendor-specific extensions use qualified names and properties rather
 *     than forcing permanent core keywords.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule is consumed by the canonical Zamani parser.
 *
 * The canonical root parser decides where a hardware declaration is legal.
 */

hardwareDeclaration
    : hardwareAttributes*
      hardwareVisibility?
      hardwareModifier*
      K_HARDWARE
      IDENTIFIER
      hardwareGenericParameters?
      hardwareContractClauses?
      hardwareBody
    ;


/* ============================================================================
 * 2. DECLARATION MODIFIERS
 * ============================================================================
 */

hardwareVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;

hardwareModifier
    : K_STATIC
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/* ============================================================================
 * 3. ATTRIBUTES
 * ============================================================================
 *
 * Attribute names remain identifiers.
 *
 * This avoids making every future vendor, technology, synthesis flow or
 * deployment feature a permanent Zamani keyword.
 */

hardwareAttributes
    : AT
      hardwareQualifiedName
      (
          LPAREN
          hardwareArgumentList?
          RPAREN
      )?
    ;

hardwareArgumentList
    : hardwareArgument
      (
          COMMA
          hardwareArgument
      )*
      COMMA?
    ;

hardwareArgument
    : hardwareExpression
    | hardwareQualifiedName
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | CHAR_LITERAL
    ;


/* ============================================================================
 * 4. GENERICS
 * ============================================================================
 *
 * Hardware generics describe semantic parameters.
 *
 * Example:
 *
 *     hardware Accelerator<Width, Lanes, Capacity> { ... }
 *
 * No fixed physical size is implied.
 */

hardwareGenericParameters
    : LT
      hardwareGenericParameter
      (
          COMMA
          hardwareGenericParameter
      )*
      COMMA?
      GT
    ;

hardwareGenericParameter
    : IDENTIFIER
      (
          COLON
          hardwareGenericBound
      )?
      (
          ASSIGN
          hardwareExpression
      )?
    ;

hardwareGenericBound
    : hardwareQualifiedName
    | hardwareCapabilityReference
    ;


/* ============================================================================
 * 5. HARDWARE CONTRACT
 * ============================================================================
 */

hardwareContractClauses
    : hardwareContractClause+
    ;

hardwareContractClause
    : hardwareRequiresClause
    | hardwareProvidesClause
    | hardwareConstraintClause
    | hardwarePreferenceClause
    ;


/* ============================================================================
 * 6. HARDWARE BODY
 * ============================================================================
 */

hardwareBody
    : LBRACE
      hardwareItem*
      RBRACE
    ;

hardwareItem
    : hardwareAttributes*
      (
          hardwareModuleDeclaration
        | hardwareInterfaceDeclaration
        | hardwarePortDeclaration
        | hardwareResourceDeclaration
        | hardwareCapabilityDeclaration
        | hardwareRequirementDeclaration
        | hardwareConstraintDeclaration
        | hardwarePreferenceDeclaration
        | hardwareTargetDeclaration
        | hardwareTopologyDeclaration
        | hardwarePlacementDeclaration
        | hardwareMappingDeclaration
        | hardwareInstanceDeclaration
        | hardwareConnectionDeclaration
        | hardwareClockContract
        | hardwareTimingContract
        | hardwareMemoryContract
        | hardwareAcceleratorDeclaration
        | hardwareCpuDeclaration
        | hardwareGpuDeclaration
        | hardwareFpgaDeclaration
        | hardwareAsicDeclaration
        | hardwareQpuDeclaration
        | hardwarePropertyDeclaration
        | hardwareGenerateDeclaration
        | hardwareUsingDeclaration
        | hardwareAssertion
      )
    ;


/* ============================================================================
 * 7. ABSTRACT HARDWARE MODULE
 * ============================================================================
 *
 * This is NOT the same thing as HDL behavioral module syntax.
 *
 * HDL behavioral modules remain owned by grammar/hdl.
 *
 * This rule describes a reusable hardware capability/composition boundary.
 */

hardwareModuleDeclaration
    : K_MODULE
      IDENTIFIER
      hardwareGenericParameters?
      hardwareContractClauses?
      hardwareBody
    ;


/* ============================================================================
 * 8. HARDWARE INTERFACES
 * ============================================================================
 */

hardwareInterfaceDeclaration
    : K_INTERFACE
      IDENTIFIER
      hardwareGenericParameters?
      hardwareInterfaceInheritance?
      LBRACE
      hardwareInterfaceMember*
      RBRACE
    ;

hardwareInterfaceInheritance
    : K_EXTENDS
      hardwareQualifiedName
      (
          COMMA
          hardwareQualifiedName
      )*
    ;

hardwareInterfaceMember
    : hardwareAttributes*
      (
          hardwarePortDeclaration
        | hardwareResourceDeclaration
        | hardwareCapabilityDeclaration
        | hardwareRequirementDeclaration
        | hardwarePropertyDeclaration
      )
    ;


/* ============================================================================
 * 9. PORTS
 * ============================================================================
 *
 * Ports describe an interface.
 *
 * Physical pins are deliberately NOT described here.
 */

hardwarePortDeclaration
    : hardwarePortDirection
      IDENTIFIER
      hardwareTypeReference?
      hardwareDimensionList?
      hardwarePortPropertyBlock?
      SEMICOLON
    ;

hardwarePortDirection
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;


/* ============================================================================
 * 10. RESOURCES
 * ============================================================================
 *
 * Resource quantities are expressions.
 *
 * Examples:
 *
 *     resource cores >= required_cores;
 *     resource memory >= workload_memory;
 *     resource lanes = available_lanes;
 *
 * No physical limit is encoded.
 */

hardwareResourceDeclaration
    : K_RESOURCE
      IDENTIFIER
      hardwareResourceType?
      hardwareResourceQuantity?
      hardwareResourcePropertyBlock?
      SEMICOLON
    ;

hardwareResourceType
    : COLON
      hardwareTypeReference
    ;

hardwareResourceQuantity
    : hardwareRelationOperator
      hardwareExpression
    ;

hardwareResourcePropertyBlock
    : LBRACE
      hardwarePropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 11. CAPABILITIES
 * ============================================================================
 */

hardwareCapabilityDeclaration
    : K_CAPABILITY
      hardwareCapabilityReference
      hardwareCapabilityValue?
      SEMICOLON
    ;

hardwareCapabilityReference
    : hardwareQualifiedName
    ;

hardwareCapabilityValue
    : ASSIGN
      hardwareExpression
    ;


/* ============================================================================
 * 12. REQUIREMENTS
 * ============================================================================
 */

hardwareRequirementDeclaration
    : K_REQUIRES
      hardwareRequirementExpression
      SEMICOLON
    ;

hardwareRequiresClause
    : K_REQUIRES
      hardwareRequirementExpression
      SEMICOLON
    ;

hardwareRequirementExpression
    : hardwareRequirementOperand
      (
          hardwareLogicalOperator
          hardwareRequirementOperand
      )*
    ;

hardwareRequirementOperand
    : hardwareQualifiedName
    | hardwareCapabilityReference
    | hardwareResourceReference
    | hardwareComparison
    | LPAREN
      hardwareRequirementExpression
      RPAREN
    ;


/* ============================================================================
 * 13. PROVIDES
 * ============================================================================
 */

hardwareProvidesClause
    : K_PROVIDES
      hardwareProvidesExpression
      SEMICOLON
    ;

hardwareProvidesExpression
    : hardwareCapabilityReference
    | hardwareResourceReference
    | hardwareQualifiedName
    ;


/* ============================================================================
 * 14. CONSTRAINTS
 * ============================================================================
 *
 * Constraints are not preferences.
 *
 * A constraint is a semantic restriction that must hold.
 */

hardwareConstraintDeclaration
    : K_CONSTRAINT
      hardwareConstraintExpression
      SEMICOLON
    ;

hardwareConstraintClause
    : K_CONSTRAINT
      hardwareConstraintExpression
      SEMICOLON
    ;

hardwareConstraintExpression
    : hardwareComparison
    | hardwareQualifiedName
    | LPAREN
      hardwareConstraintExpression
      RPAREN
    ;


/* ============================================================================
 * 15. PREFERENCES
 * ============================================================================
 *
 * Preferences guide implementation but do not change semantic correctness.
 */

hardwarePreferenceDeclaration
    : K_PREFER
      hardwarePreferenceExpression
      SEMICOLON
    ;

hardwarePreferenceClause
    : K_PREFER
      hardwarePreferenceExpression
      SEMICOLON
    ;

hardwarePreferenceExpression
    : hardwareComparison
    | hardwareQualifiedName
    | hardwareExpression
    ;


/* ============================================================================
 * 16. TARGETS
 * ============================================================================
 *
 * A target declaration describes a class of execution target.
 *
 * It must not imply that a particular physical device exists.
 */

hardwareTargetDeclaration
    : K_TARGET
      IDENTIFIER
      hardwareGenericParameters?
      hardwareTargetPropertyBlock?
      SEMICOLON
    ;

hardwareTargetPropertyBlock
    : LBRACE
      hardwarePropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 17. TOPOLOGY
 * ============================================================================
 *
 * This describes topology requirements/capabilities.
 *
 * It does NOT discover or allocate physical topology.
 */

hardwareTopologyDeclaration
    : K_TOPOLOGY
      IDENTIFIER?
      hardwareTopologyBody
    ;

hardwareTopologyBody
    : LBRACE
      hardwareTopologyItem*
      RBRACE
    ;

hardwareTopologyItem
    : K_MODEL
      ASSIGN
      hardwareQualifiedName
      SEMICOLON

    | K_CONNECTIVITY
      ASSIGN
      hardwareExpression
      SEMICOLON

    | K_DEGREE
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_DISTANCE
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 18. PLACEMENT
 * ============================================================================
 *
 * Placement is INTENT only.
 *
 * Actual placement belongs to the routing/placement subsystem.
 */

hardwarePlacementDeclaration
    : K_PLACEMENT
      IDENTIFIER?
      hardwarePlacementBody
    ;

hardwarePlacementBody
    : LBRACE
      hardwarePlacementItem*
      RBRACE
    ;

hardwarePlacementItem
    : K_NEAR
      ASSIGN
      hardwareQualifiedName
      SEMICOLON

    | K_GROUP
      ASSIGN
      hardwareQualifiedName
      SEMICOLON

    | K_AFFINITY
      ASSIGN
      hardwareExpression
      SEMICOLON

    | K_AVOID
      ASSIGN
      hardwareExpression
      SEMICOLON

    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 19. MAPPING
 * ============================================================================
 *
 * Mapping expresses a semantic relationship between abstract resources.
 *
 * Physical allocation is downstream.
 */

hardwareMappingDeclaration
    : K_MAP
      hardwareQualifiedName
      K_TO
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 20. INSTANCES
 * ============================================================================
 */

hardwareInstanceDeclaration
    : K_INSTANCE
      hardwareQualifiedName
      IDENTIFIER
      hardwareGenericArguments?
      hardwareInstancePropertyBlock?
      SEMICOLON
    ;

hardwareGenericArguments
    : LT
      hardwareExpression
      (
          COMMA
          hardwareExpression
      )*
      COMMA?
      GT
    ;

hardwareInstancePropertyBlock
    : LBRACE
      hardwarePropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 21. CONNECTIONS
 * ============================================================================
 *
 * Connections are logical connections.
 *
 * Physical routing remains outside the grammar.
 */

hardwareConnectionDeclaration
    : K_CONNECT
      hardwareEndpoint
      K_TO
      hardwareEndpoint
      hardwareConnectionPropertyBlock?
      SEMICOLON
    ;

hardwareEndpoint
    : hardwareQualifiedName
      hardwareEndpointSelector?
    ;

hardwareEndpointSelector
    : LBRACKET
      hardwareExpression
      RBRACKET
    ;

hardwareConnectionPropertyBlock
    : LBRACE
      hardwarePropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 22. CLOCK CONTRACTS
 * ============================================================================
 *
 * Clock declarations describe semantic timing relationships.
 *
 * They do not force a physical oscillator or fixed frequency.
 */

hardwareClockContract
    : K_CLOCK
      IDENTIFIER
      hardwareClockPropertyBlock?
      SEMICOLON
    ;

hardwareClockPropertyBlock
    : LBRACE
      hardwareClockProperty*
      RBRACE
    ;

hardwareClockProperty
    : K_FREQUENCY
      ASSIGN
      hardwareExpression
      SEMICOLON

    | K_PERIOD
      ASSIGN
      hardwareExpression
      SEMICOLON

    | K_PHASE
      ASSIGN
      hardwareExpression
      SEMICOLON

    | K_DUTY
      ASSIGN
      hardwareExpression
      SEMICOLON

    | K_DOMAIN
      ASSIGN
      hardwareQualifiedName
      SEMICOLON

    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 23. TIMING CONTRACTS
 * ============================================================================
 */

hardwareTimingContract
    : K_TIMING
      IDENTIFIER?
      hardwareTimingBody
    ;

hardwareTimingBody
    : LBRACE
      hardwareTimingItem*
      RBRACE
    ;

hardwareTimingItem
    : K_LATENCY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_THROUGHPUT
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_FREQUENCY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_PERIOD
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_DEADLINE
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | hardwareComparison
      SEMICOLON

    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 24. MEMORY CONTRACT
 * ============================================================================
 *
 * Memory capacity is a semantic requirement/capability.
 *
 * No fixed memory size is encoded.
 */

hardwareMemoryContract
    : K_MEMORY
      IDENTIFIER?
      hardwareMemoryBody
    ;

hardwareMemoryBody
    : LBRACE
      hardwareMemoryItem*
      RBRACE
    ;

hardwareMemoryItem
    : K_CAPACITY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_BANDWIDTH
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_LATENCY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_ADDRESS_SPACE
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 25. ACCELERATORS
 * ============================================================================
 */

hardwareAcceleratorDeclaration
    : K_ACCELERATOR
      IDENTIFIER
      hardwareGenericParameters?
      hardwareContractClauses?
      hardwareAcceleratorBody?
      SEMICOLON?
    ;

hardwareAcceleratorBody
    : LBRACE
      hardwareItem*
      RBRACE
    ;


/* ============================================================================
 * 26. CPU CONTRACT
 * ============================================================================
 *
 * This does not define a particular CPU architecture.
 *
 * Architecture-specific information belongs in target descriptions/dialects.
 */

hardwareCpuDeclaration
    : K_CPU
      IDENTIFIER?
      hardwareProcessorBody
    ;

hardwareProcessorBody
    : LBRACE
      hardwareProcessorItem*
      RBRACE
    ;

hardwareProcessorItem
    : K_CORES
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_THREADS
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_VECTOR_WIDTH
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_MEMORY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_ARCHITECTURE
      ASSIGN
      hardwareQualifiedName
      SEMICOLON

    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 27. GPU CONTRACT
 * ============================================================================
 */

hardwareGpuDeclaration
    : K_GPU
      IDENTIFIER?
      hardwareProcessorBody
    ;


/* ============================================================================
 * 28. FPGA CONTRACT
 * ============================================================================
 */

hardwareFpgaDeclaration
    : K_FPGA
      IDENTIFIER?
      hardwareFpgaBody
    ;

hardwareFpgaBody
    : LBRACE
      hardwareFpgaItem*
      RBRACE
    ;

hardwareFpgaItem
    : K_LOGIC_RESOURCES
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_MEMORY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_DSP
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | K_BANDWIDTH
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON

    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 29. ASIC CONTRACT
 * ============================================================================
 */

hardwareAsicDeclaration
    : K_ASIC
      IDENTIFIER?
      hardwareAsicBody
    ;

hardwareAsicBody
    : LBRACE
      hardwarePropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 30. QPU DELEGATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * qpu.g4 owns QPU syntax.
 *
 * hardware.g4 MUST NOT duplicate QPU declarations.
 */

hardwareQpuDeclaration
    : qpuDeclaration
    ;


/* ============================================================================
 * 31. PROPERTY DECLARATIONS
 * ============================================================================
 *
 * Generic properties provide forward compatibility.
 *
 * This is the primary extension mechanism for technology-specific properties
 * that do not yet deserve a permanent core keyword.
 */

hardwarePropertyDeclaration
    : K_PROPERTY
      hardwareQualifiedName
      ASSIGN
      hardwareExpression
      SEMICOLON
    ;

hardwarePropertyStatement
    : hardwareQualifiedName
      ASSIGN
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 32. GENERATION
 * ============================================================================
 *
 * Generate is structural source generation intent.
 *
 * It does not impose a fixed generated cardinality.
 */

hardwareGenerateDeclaration
    : K_GENERATE
      IDENTIFIER?
      hardwareGenerateBody
    ;

hardwareGenerateBody
    : LBRACE
      hardwareGenerateItem*
      RBRACE
    ;

hardwareGenerateItem
    : K_FOR
      IDENTIFIER
      K_IN
      hardwareExpression
      hardwareGenerateBody
    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 33. USING / EXTENSION REFERENCES
 * ============================================================================
 */

hardwareUsingDeclaration
    : K_USING
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 34. ASSERTIONS
 * ============================================================================
 */

hardwareAssertion
    : K_ASSERT
      LPAREN
      hardwareExpression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 35. TYPE REFERENCES
 * ============================================================================
 *
 * Hardware types are references into the canonical type system.
 *
 * This file does not redefine the universal Zamani type system.
 */

hardwareTypeReference
    : hardwareQualifiedName
    | hardwareBuiltinType
    ;

hardwareBuiltinType
    : K_BOOL
    | K_INT
    | K_UINT
    | K_FLOAT
    | K_BIT
    | K_LOGIC
    ;


/* ============================================================================
 * 36. RESOURCE REFERENCES
 * ============================================================================
 */

hardwareResourceReference
    : K_RESOURCE
      hardwareQualifiedName
    ;

hardwareQualifiedResourceReference
    : hardwareQualifiedName
    ;


/* ============================================================================
 * 37. COMPARISONS
 * ============================================================================
 */

hardwareComparison
    : hardwareExpression
      hardwareRelationOperator
      hardwareExpression
    ;

hardwareRelationOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;

hardwareLogicalOperator
    : LOGICAL_AND
    | LOGICAL_OR
    | K_AND
    | K_OR
    ;


/* ============================================================================
 * 38. EXPRESSIONS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This is intentionally a bounded bridge to the canonical expression layer.
 *
 * The production integration should replace this rule with the shared
 * expression nonterminal exported by grammar/expressions.
 *
 * hardware.g4 does not become a second expression language.
 */

hardwareExpression
    : hardwarePrimaryExpression
      hardwareExpressionSuffix*
    ;

hardwareExpressionSuffix
    : DOT
      IDENTIFIER
    | LBRACKET
      hardwareExpression
      RBRACKET
    | LPAREN
      hardwareArgumentList?
      RPAREN
    ;

hardwarePrimaryExpression
    : IDENTIFIER
    | hardwareQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | K_TRUE
    | K_FALSE
    | LPAREN
      hardwareExpression
      RPAREN
    ;


/* ============================================================================
 * 39. QUALIFIED NAMES
 * ============================================================================
 */

hardwareQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 40. DIMENSIONS
 * ============================================================================
 *
 * Dimensions are expressions, never fixed constants.
 */

hardwareDimensionList
    : LBRACKET
      hardwareExpression
      RBRACKET
      (
          LBRACKET
          hardwareExpression
          RBRACKET
      )*
    ;