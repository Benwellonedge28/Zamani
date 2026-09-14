/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/fpga.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target integration:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific code.
 *     - No filesystem access.
 *     - No network access.
 *     - No device access.
 *     - No code execution.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL FPGA-SPECIFIC SYNTAX.
 *
 * It describes programmable-logic intent without encoding the physical
 * characteristics of one FPGA device.
 *
 * An FPGA declaration may describe:
 *
 *     - programmable fabric;
 *     - logical FPGA regions;
 *     - programmable compute units;
 *     - interfaces;
 *     - ports;
 *     - clocks;
 *     - memories;
 *     - pipelines;
 *     - programmable processes;
 *     - configuration intent;
 *     - accelerator regions;
 *     - resource requirements;
 *     - capability requirements;
 *     - timing intent;
 *     - implementation constraints;
 *     - implementation preferences;
 *     - implementation hints;
 *     - target intent;
 *     - parameterized FPGA designs;
 *     - reusable FPGA components.
 *
 * It MUST NOT encode a fixed FPGA architecture.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - FPGA declarations;
 *     - FPGA generic parameters;
 *     - FPGA logical regions;
 *     - FPGA fabric declarations;
 *     - FPGA interfaces;
 *     - FPGA ports;
 *     - FPGA clocks;
 *     - FPGA memories;
 *     - FPGA pipelines;
 *     - FPGA processes;
 *     - FPGA compute/accelerator regions;
 *     - FPGA configuration intent;
 *     - FPGA resource requirements;
 *     - FPGA capability requirements;
 *     - FPGA constraints;
 *     - FPGA preferences;
 *     - FPGA hints;
 *     - FPGA target intent;
 *     - FPGA implementation-neutral properties;
 *     - FPGA composition;
 *     - FPGA instances;
 *     - FPGA-qualified references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - general expression syntax;
 *     - general type syntax;
 *     - general statements;
 *     - generic HDL syntax;
 *     - physical FPGA discovery;
 *     - device enumeration;
 *     - vendor databases;
 *     - package databases;
 *     - pin databases;
 *     - routing algorithms;
 *     - placement algorithms;
 *     - scheduling;
 *     - synthesis;
 *     - bitstream generation;
 *     - timing closure;
 *     - optimization;
 *     - calibration;
 *     - runtime dispatch;
 *     - device drivers;
 *     - canonical IR construction.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * FPGA syntax expresses:
 *
 *     computation
 *     structure
 *     interfaces
 *     capabilities
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     portability
 *
 * It does NOT silently select:
 *
 *     a vendor;
 *     a board;
 *     a package;
 *     a device;
 *     a device identifier;
 *     a fixed fabric size;
 *     a fixed number of logic blocks;
 *     a fixed number of DSP blocks;
 *     a fixed number of memories;
 *     a fixed number of I/O pins;
 *     a fixed routing topology;
 *     a fixed clock-resource count.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No fixed machine limits are encoded here.
 *
 * In particular, this grammar contains no:
 *
 *     MAX_FPGAS
 *     MAX_LUTS
 *     MAX_CLBS
 *     MAX_DSP
 *     MAX_BRAM
 *     MAX_URAM
 *     MAX_IO
 *     MAX_PINS
 *     MAX_CLOCKS
 *     MAX_REGISTERS
 *     MAX_LANES
 *     MAX_PIPELINES
 *     MAX_REGIONS
 *     MAX_PORTS
 *     MAX_MEMORIES
 *     MAX_INSTANCES
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * Dimensions, widths, counts, frequencies, capacities and other quantities
 * are expressions and are therefore resolved semantically.
 *
 * "Infinity" here means that this grammar introduces no arbitrary finite
 * hardware-scale ceiling. Real compilation remains constrained only by:
 *
 *     - program semantics;
 *     - declared requirements;
 *     - target capabilities;
 *     - available resources;
 *     - implementation policy;
 *     - compiler/runtime limits.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     HardwareFpga parser
 *          |
 *          v
 *     syntax AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     HDL semantic model       resource/capability model
 *          |                          |
 *          +-------------+------------+
 *                        |
 *                        v
 *                canonical semantics
 *                        |
 *                        v
 *             optimization / lowering
 *                        |
 *                        v
 *                 scheduling
 *                        |
 *                        v
 *                  placement
 *                        |
 *                        v
 *                    routing
 *                        |
 *                        v
 *                   hardware HAL
 *                        |
 *                        v
 *                     runtime
 *
 * This grammar never constructs IR directly.
 *
 * ============================================================================
 */

parser grammar HardwareFpga;

options {
    tokenVocab = ZamaniTokens;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * hardware.g4 MUST delegate FPGA declarations to this rule.
 *
 * Existing hardware.g4 ownership:
 *
 *     hardwareFpgaDecl
 *         : K_FPGA IDENTIFIER hardwareProgrammableBody?
 *         ;
 *
 * must be replaced by an integration reference to this grammar's
 * hardwareFpgaDecl rule.
 */
hardwareFpgaDecl
    : hardwareFpgaAttribute*
      K_FPGA
      IDENTIFIER
      hardwareFpgaGenericParameters?
      hardwareFpgaKindClause?
      hardwareFpgaRequirementSection*
      hardwareFpgaCapabilitySection*
      hardwareFpgaPropertySection?
      hardwareFpgaBody?
    ;


/*
 * ============================================================================
 * 2. ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They do not automatically become:
 *
 *     target selectors
 *     device selectors
 *     synthesis commands
 *     filesystem paths
 *     driver commands
 */
hardwareFpgaAttribute
    : AT
      IDENTIFIER
      (
          LPAREN
          hardwareFpgaAttributeArguments?
          RPAREN
      )?
    ;


hardwareFpgaAttributeArguments
    : hardwareFpgaAttributeArgument
      (
          COMMA
          hardwareFpgaAttributeArgument
      )*
    ;


hardwareFpgaAttributeArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareFpgaQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 3. GENERIC PARAMETERS
 * ============================================================================
 *
 * FPGA designs are parameterized instead of being hard-coded for one size.
 *
 * Examples:
 *
 *     fpga VectorFabric<WIDTH>
 *
 *     fpga MatrixFabric<
 *         ROWS,
 *         COLUMNS,
 *         ELEMENT_TYPE
 *     >
 *
 * Parameter values remain source-level symbolic values.
 */
hardwareFpgaGenericParameters
    : LT
      hardwareFpgaGenericParameter
      (
          COMMA
          hardwareFpgaGenericParameter
      )*
      GT
    ;


hardwareFpgaGenericParameter
    : IDENTIFIER
      (
          COLON
          hardwareFpgaQualifiedName
      )?
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * 4. FPGA KIND
 * ============================================================================
 *
 * The FPGA kind remains semantically open.
 *
 * The grammar recognizes the FPGA construct itself, but does not enumerate
 * vendors or device families.
 */
hardwareFpgaKindClause
    : COLON
      hardwareFpgaQualifiedName
    ;


/*
 * ============================================================================
 * 5. TOP-LEVEL FPGA BODY
 * ============================================================================
 *
 * An FPGA may contain arbitrarily many logical components.
 */
hardwareFpgaBody
    : LBRACE
      hardwareFpgaBodyItem*
      RBRACE
    ;


hardwareFpgaBodyItem
    : hardwareFpgaAttribute*
      (
          hardwareFpgaFabricDecl
        | hardwareFpgaInterfaceDecl
        | hardwareFpgaPortDecl
        | hardwareFpgaClockDecl
        | hardwareFpgaMemoryDecl
        | hardwareFpgaPipelineDecl
        | hardwareFpgaProcessDecl
        | hardwareFpgaRegionDecl
        | hardwareFpgaComputeDecl
        | hardwareFpgaAcceleratorDecl
        | hardwareFpgaInstanceDecl
        | hardwareFpgaResourceDecl
        | hardwareFpgaRequirementDecl
        | hardwareFpgaCapabilityDecl
        | hardwareFpgaConstraintDecl
        | hardwareFpgaPreferenceDecl
        | hardwareFpgaHintDecl
        | hardwareFpgaTargetDecl
        | hardwareFpgaConfigurationDecl
        | hardwareFpgaPropertyDecl
      )
    ;


/*
 * ============================================================================
 * 6. PROGRAMMABLE FABRIC
 * ============================================================================
 *
 * The fabric is logical.
 *
 * This does NOT mean:
 *
 *     number of LUTs
 *     number of CLBs
 *     number of slices
 *     number of routing switches
 *
 * Those values belong to target capability data.
 */
hardwareFpgaFabricDecl
    : IDENTIFIER
      LBRACE
      hardwareFpgaFabricItem*
      RBRACE
    ;


hardwareFpgaFabricItem
    : hardwareFpgaPropertyDecl
    | hardwareFpgaRegionDecl
    | hardwareFpgaComputeDecl
    | hardwareFpgaMemoryDecl
    | hardwareFpgaPipelineDecl
    ;


/*
 * ============================================================================
 * 7. INTERFACES
 * ============================================================================
 *
 * Interfaces describe logical communication contracts.
 *
 * They do not imply a particular physical bus or pinout.
 */
hardwareFpgaInterfaceDecl
    : K_INTERFACE
      IDENTIFIER
      hardwareFpgaGenericParameters?
      hardwareFpgaExtendsClause?
      LBRACE
      hardwareFpgaInterfaceItem*
      RBRACE
    ;


hardwareFpgaExtendsClause
    : K_EXTENDS
      hardwareFpgaQualifiedName
      (
          COMMA
          hardwareFpgaQualifiedName
      )*
    ;


hardwareFpgaInterfaceItem
    : hardwareFpgaPortDecl
    | hardwareFpgaOperationDecl
    | hardwareFpgaPropertyDecl
    ;


/*
 * ============================================================================
 * 8. PORTS
 * ============================================================================
 *
 * Widths and dimensions are expressions.
 *
 * There is no fixed:
 *
 *     1-bit
 *     8-bit
 *     32-bit
 *     64-bit
 *     128-bit
 *
 * restriction.
 */
hardwareFpgaPortDecl
    : hardwareFpgaPortDirection
      IDENTIFIER
      hardwareFpgaPortType?
      hardwareFpgaPortShape*
      hardwareFpgaPortProperties?
      SEMICOLON
    ;


hardwareFpgaPortDirection
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;


hardwareFpgaPortType
    : COLON
      hardwareFpgaQualifiedName
    ;


hardwareFpgaPortShape
    : LBRACKET
      expression
      RBRACKET
    ;


hardwareFpgaPortProperties
    : LBRACE
      hardwareFpgaProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 9. OPERATIONS
 * ============================================================================
 *
 * Operations remain open-ended.
 *
 * No fixed accelerator/function list is embedded in the FPGA grammar.
 */
hardwareFpgaOperationDecl
    : K_FN
      IDENTIFIER
      hardwareFpgaGenericParameters?
      LPAREN
      hardwareFpgaParameterList?
      RPAREN
      hardwareFpgaReturnType?
      hardwareFpgaOperationBody?
    ;


hardwareFpgaParameterList
    : hardwareFpgaParameter
      (
          COMMA
          hardwareFpgaParameter
      )*
      COMMA?
    ;


hardwareFpgaParameter
    : IDENTIFIER
      (
          COLON
          hardwareFpgaQualifiedName
      )?
      (
          ASSIGN
          expression
      )?
    ;


hardwareFpgaReturnType
    : THIN_ARROW
      hardwareFpgaQualifiedName
    ;


hardwareFpgaOperationBody
    : LBRACE
      hardwareFpgaExecutionItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 10. CLOCKS
 * ============================================================================
 *
 * Clock declarations describe logical timing intent.
 *
 * A frequency is an expression.
 *
 * The grammar does not assume:
 *
 *     one clock
 *     two clocks
 *     a fixed frequency
 *     a fixed clock tree
 *     a fixed PLL count.
 */
hardwareFpgaClockDecl
    : K_CLOCK
      IDENTIFIER
      hardwareFpgaClockSpecification?
      SEMICOLON
    ;


hardwareFpgaClockSpecification
    : LBRACE
      hardwareFpgaClockProperty*
      RBRACE
    ;


hardwareFpgaClockProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. MEMORY
 * ============================================================================
 *
 * Memory is logical.
 *
 * Capacity, width, depth and banking are expressions.
 *
 * Physical BRAM/URAM/register-file mapping belongs downstream.
 */
hardwareFpgaMemoryDecl
    : K_MEMORY
      IDENTIFIER
      hardwareFpgaMemoryType?
      hardwareFpgaMemorySpecification?
      SEMICOLON
    ;


hardwareFpgaMemoryType
    : COLON
      hardwareFpgaQualifiedName
    ;


hardwareFpgaMemorySpecification
    : LBRACE
      hardwareFpgaMemoryProperty*
      RBRACE
    ;


hardwareFpgaMemoryProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. PIPELINES
 * ============================================================================
 *
 * Pipeline depth, initiation interval, throughput and latency are semantic
 * or implementation intent.
 *
 * They are not tied to a particular FPGA architecture.
 */
hardwareFpgaPipelineDecl
    : K_PIPELINE
      IDENTIFIER
      hardwareFpgaPipelineSpecification?
      hardwareFpgaOperationBody?
    ;


hardwareFpgaPipelineSpecification
    : LBRACE
      hardwareFpgaPipelineProperty*
      RBRACE
    ;


hardwareFpgaPipelineProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. PROCESSES
 * ============================================================================
 *
 * Processes describe programmable hardware behavior.
 */
hardwareFpgaProcessDecl
    : K_PROCESS
      IDENTIFIER
      hardwareFpgaSensitivityClause?
      hardwareFpgaOperationBody?
    ;


hardwareFpgaSensitivityClause
    : K_ON
      hardwareFpgaQualifiedName
      (
          COMMA
          hardwareFpgaQualifiedName
      )*
    ;


/*
 * ============================================================================
 * 14. LOGICAL REGIONS
 * ============================================================================
 *
 * Regions are logical decomposition units.
 *
 * They are NOT physical FPGA regions unless semantic analysis explicitly maps
 * them to a target-specific resource.
 */
hardwareFpgaRegionDecl
    : K_REGION
      IDENTIFIER
      hardwareFpgaRegionSpecification?
      hardwareFpgaBody?
    ;


hardwareFpgaRegionSpecification
    : LBRACKET
      hardwareFpgaRegionProperty+
      RBRACKET
    ;


hardwareFpgaRegionProperty
    : hardwareFpgaPropertyName
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * 15. COMPUTE REGIONS
 * ============================================================================
 *
 * A compute region describes programmable computation.
 *
 * It may ultimately lower to:
 *
 *     LUT logic
 *     DSP logic
 *     vector logic
 *     custom logic
 *     memory-coupled logic
 *     accelerator logic
 *
 * but this grammar does not select any of those physical implementations.
 */
hardwareFpgaComputeDecl
    : K_COMPUTE
      IDENTIFIER
      hardwareFpgaGenericParameters?
      hardwareFpgaComputeSpecification?
      hardwareFpgaOperationBody?
    ;


hardwareFpgaComputeSpecification
    : LBRACE
      hardwareFpgaComputeProperty*
      RBRACE
    ;


hardwareFpgaComputeProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. ACCELERATOR REGIONS
 * ============================================================================
 *
 * Accelerator composition remains symbolic.
 *
 * FPGA-specific accelerator realization belongs to the hardware/accelerator
 * semantic layer, not this grammar.
 */
hardwareFpgaAcceleratorDecl
    : K_ACCELERATOR
      IDENTIFIER
      hardwareFpgaGenericParameters?
      hardwareFpgaAcceleratorSpecification?
      hardwareFpgaOperationBody?
    ;


hardwareFpgaAcceleratorSpecification
    : LBRACE
      hardwareFpgaAcceleratorProperty*
      RBRACE
    ;


hardwareFpgaAcceleratorProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. INSTANCE
 * ============================================================================
 *
 * Instances are logical instances.
 *
 * The number of instances is unbounded by grammar.
 */
hardwareFpgaInstanceDecl
    : K_INSTANCE
      IDENTIFIER
      K_OF
      hardwareFpgaQualifiedName
      hardwareFpgaGenericArguments?
      hardwareFpgaInstanceBinding?
      SEMICOLON
    ;


hardwareFpgaGenericArguments
    : LT
      hardwareFpgaGenericArgument
      (
          COMMA
          hardwareFpgaGenericArgument
      )*
      GT
    ;


hardwareFpgaGenericArgument
    : expression
    ;


hardwareFpgaInstanceBinding
    : WITH
      LBRACE
      hardwareFpgaBinding*
      RBRACE
    ;


hardwareFpgaBinding
    : IDENTIFIER
      ASSIGN
      expression
      COMMA?
    ;


/*
 * ============================================================================
 * 18. RESOURCE INTENT
 * ============================================================================
 *
 * These are semantic declarations.
 *
 * They do not enumerate physical FPGA resources.
 */
hardwareFpgaResourceDecl
    : K_RESOURCE
      hardwareFpgaQualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


hardwareFpgaRequirementDecl
    : K_REQUIRES
      hardwareFpgaRequirementExpression
      SEMICOLON
    ;


hardwareFpgaRequirementExpression
    : expression
    ;


hardwareFpgaCapabilityDecl
    : K_CAPABILITY
      hardwareFpgaQualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


hardwareFpgaConstraintDecl
    : K_CONSTRAINT
      hardwareFpgaConstraintExpression
      SEMICOLON
    ;


hardwareFpgaConstraintExpression
    : expression
    ;


hardwareFpgaPreferenceDecl
    : K_PREFERENCE
      hardwareFpgaPreferenceExpression
      SEMICOLON
    ;


hardwareFpgaPreferenceExpression
    : expression
    ;


hardwareFpgaHintDecl
    : K_HINT
      hardwareFpgaHintExpression
      SEMICOLON
    ;


hardwareFpgaHintExpression
    : expression
    ;


/*
 * ============================================================================
 * 19. TARGET INTENT
 * ============================================================================
 *
 * A target clause expresses target intent.
 *
 * It is NOT a device-discovery mechanism.
 *
 * For POCO-REAF, portable programs should normally use capability/resource
 * expressions instead of binding themselves to a particular device.
 */
hardwareFpgaTargetDecl
    : K_TARGET
      hardwareFpgaTargetExpression
      SEMICOLON
    ;


hardwareFpgaTargetExpression
    : hardwareFpgaQualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 20. CONFIGURATION
 * ============================================================================
 *
 * Configuration describes source-level implementation intent.
 *
 * It is not bitstream data and does not execute a programming operation.
 */
hardwareFpgaConfigurationDecl
    : K_CONFIGURATION
      IDENTIFIER?
      LBRACE
      hardwareFpgaConfigurationItem*
      RBRACE
    ;


hardwareFpgaConfigurationItem
    : hardwareFpgaConfigurationProperty
    | hardwareFpgaConfigurationBinding
    ;


hardwareFpgaConfigurationProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


hardwareFpgaConfigurationBinding
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. GENERIC PROPERTY
 * ============================================================================
 *
 * Properties provide forward-compatible extension without requiring every
 * future FPGA feature to become a lexer keyword.
 */
hardwareFpgaPropertySection
    : LBRACE
      hardwareFpgaProperty*
      RBRACE
    ;


hardwareFpgaPropertyDecl
    : hardwareFpgaPropertyName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


hardwareFpgaProperty
    : hardwareFpgaPropertyName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


hardwareFpgaPropertyName
    : IDENTIFIER
    | hardwareFpgaQualifiedName
    ;


/*
 * ============================================================================
 * 22. EXECUTION ITEMS
 * ============================================================================
 *
 * This grammar deliberately does not duplicate the general Zamani statement
 * grammar.
 *
 * FPGA operation bodies contain expressions and qualified operation references
 * until the canonical statement grammar is integrated at the frontend layer.
 */
hardwareFpgaExecutionItem
    : hardwareFpgaExecutionCall
    | hardwareFpgaExecutionAssignment
    | hardwareFpgaExecutionProperty
    ;


hardwareFpgaExecutionCall
    : hardwareFpgaQualifiedName
      LPAREN
      hardwareFpgaArgumentList?
      RPAREN
      SEMICOLON
    ;


hardwareFpgaArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


hardwareFpgaExecutionAssignment
    : hardwareFpgaQualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


hardwareFpgaExecutionProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. QUALIFIED NAMES
 * ============================================================================
 *
 * Qualified names are logical names.
 *
 * They must never be interpreted by the parser as filesystem paths or network
 * addresses.
 */
hardwareFpgaQualifiedName
    : IDENTIFIER
      (
          DOT
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 24. EXECUTION ITEM ALIAS
 * ============================================================================
 *
 * Named rule for downstream semantic tooling.
 */
hardwareFpgaExecution
    : hardwareFpgaExecutionItem*
    ;


/*
 * ============================================================================
 * 25. PARAMETERIZED PROPERTY
 * ============================================================================
 *
 * Allows a property value to contain arbitrary canonical expressions.
 */
hardwareFpgaParameterizedProperty
    : hardwareFpgaPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. RESOURCE/CAPABILITY CONTRACT
 * ============================================================================
 *
 * This rule provides a stable syntax-level integration point for tooling.
 */
hardwareFpgaResourceContract
    : hardwareFpgaResourceDecl
    | hardwareFpgaRequirementDecl
    | hardwareFpgaCapabilityDecl
    | hardwareFpgaConstraintDecl
    | hardwareFpgaPreferenceDecl
    | hardwareFpgaHintDecl
    | hardwareFpgaTargetDecl
    ;


/*
 * ============================================================================
 * 27. FPGA COMPONENT
 * ============================================================================
 *
 * A component is any reusable FPGA-level declaration.
 */
hardwareFpgaComponent
    : hardwareFpgaFabricDecl
    | hardwareFpgaInterfaceDecl
    | hardwareFpgaPortDecl
    | hardwareFpgaClockDecl
    | hardwareFpgaMemoryDecl
    | hardwareFpgaPipelineDecl
    | hardwareFpgaProcessDecl
    | hardwareFpgaRegionDecl
    | hardwareFpgaComputeDecl
    | hardwareFpgaAcceleratorDecl
    | hardwareFpgaInstanceDecl
    ;


/*
 * ============================================================================
 * 28. FPGA DECLARATION CONTRACT
 * ============================================================================
 *
 * Stable integration rule for hardware.g4.
 */
hardwareFpgaDeclaration
    : hardwareFpgaDecl
    ;


/*
 * ============================================================================
 * 29. FPGA RESOURCE BLOCK
 * ============================================================================
 *
 * A grouped resource block remains logical and target-neutral.
 */
hardwareFpgaResourceBlock
    : K_RESOURCE
      LBRACE
      hardwareFpgaResourceDecl*
      RBRACE
    ;


/*
 * ============================================================================
 * 30. FPGA CAPABILITY BLOCK
 * ============================================================================
 */
hardwareFpgaCapabilityBlock
    : K_CAPABILITY
      LBRACE
      hardwareFpgaCapabilityDecl*
      RBRACE
    ;


/*
 * ============================================================================
 * 31. FPGA CONSTRAINT BLOCK
 * ============================================================================
 */
hardwareFpgaConstraintBlock
    : K_CONSTRAINT
      LBRACE
      hardwareFpgaConstraintDecl*
      RBRACE
    ;


/*
 * ============================================================================
 * 32. FPGA PREFERENCE BLOCK
 * ============================================================================
 */
hardwareFpgaPreferenceBlock
    : K_PREFERENCE
      LBRACE
      hardwareFpgaPreferenceDecl*
      RBRACE
    ;


/*
 * ============================================================================
 * 33. FPGA HINT BLOCK
 * ============================================================================
 */
hardwareFpgaHintBlock
    : K_HINT
      LBRACE
      hardwareFpgaHintDecl*
      RBRACE
    ;


/*
 * ============================================================================
 * 34. FPGA TARGET BLOCK
 * ============================================================================
 */
hardwareFpgaTargetBlock
    : K_TARGET
      LBRACE
      hardwareFpgaTargetDecl*
      RBRACE
    ;


/*
 * ============================================================================
 * 35. HARDWARE-NEUTRAL QUANTITY
 * ============================================================================
 *
 * Quantities are expressions rather than fixed literals.
 *
 * This permits:
 *
 *     parameter-derived widths;
 *     symbolic dimensions;
 *     compile-time expressions;
 *     resource-dependent expressions;
 *     target capability expressions.
 */
hardwareFpgaQuantity
    : expression
    ;


/*
 * ============================================================================
 * 36. WIDTH
 * ============================================================================
 */
hardwareFpgaWidth
    : hardwareFpgaQuantity
    ;


/*
 * ============================================================================
 * 37. DEPTH
 * ============================================================================
 */
hardwareFpgaDepth
    : hardwareFpgaQuantity
    ;


/*
 * ============================================================================
 * 38. DIMENSION
 * ============================================================================
 */
hardwareFpgaDimension
    : hardwareFpgaQuantity
    ;


/*
 * ============================================================================
 * 39. FREQUENCY
 * ============================================================================
 */
hardwareFpgaFrequency
    : hardwareFpgaQuantity
    ;


/*
 * ============================================================================
 * 40. LATENCY
 * ============================================================================
 */
hardwareFpgaLatency
    : hardwareFpgaQuantity
    ;


/*
 * ============================================================================
 * 41. THROUGHPUT
 * ============================================================================
 */
hardwareFpgaThroughput
    : hardwareFpgaQuantity
    ;