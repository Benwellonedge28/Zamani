/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/asic.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Rust target:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     native code, or unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX specific to ASIC hardware
 * descriptions.
 *
 * An ASIC declaration describes an application-specific integrated circuit
 * as a logical hardware implementation target.
 *
 * It describes:
 *
 *     - ASIC identity;
 *     - ASIC composition;
 *     - technology intent;
 *     - logical cells/components;
 *     - interfaces and ports;
 *     - signals and nets;
 *     - clocks and timing intent;
 *     - memories;
 *     - pipelines;
 *     - combinational/sequential regions;
 *     - processes;
 *     - instances;
 *     - connections;
 *     - implementation requirements;
 *     - capabilities;
 *     - resources;
 *     - constraints;
 *     - preferences;
 *     - placement intent;
 *     - target intent;
 *     - implementation properties;
 *     - synthesis/implementation metadata.
 *
 * This grammar deliberately does NOT encode a particular ASIC technology,
 * foundry, process node, cell library, die size, transistor count, pin count,
 * routing capacity, clock count, memory capacity, or physical device.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *     ASIC-specific declaration syntax.
 *
 * THIS FILE DOES NOT OWN
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - numeric literal spelling;
 *     - string literal spelling;
 *     - canonical expressions;
 *     - generic hardware declarations;
 *     - physical ASIC discovery;
 *     - foundry selection;
 *     - process-node selection;
 *     - standard-cell characterization;
 *     - physical design;
 *     - placement algorithms;
 *     - routing algorithms;
 *     - synthesis algorithms;
 *     - timing analysis;
 *     - power analysis;
 *     - IR construction;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - scheduling;
 *     - hardware HAL;
 *     - runtime dispatch;
 *     - machine-specific limits.
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
 * Hardware/ASIC parser
 *   |
 *   v
 * Frontend AST
 *   |
 *   v
 * Semantic analysis
 *   |
 *   +--------------------+
 *   |                    |
 *   v                    v
 * Hardware semantic   Classical/quantum/
 * representation      HDL semantic models
 *   |                    |
 *   +----------+---------+
 *              |
 *              v
 *       capability/resource
 *          resolution
 *              |
 *              v
 *       synthesis / optimization
 *              |
 *              v
 *       placement / routing
 *              |
 *              v
 *          scheduling
 *              |
 *              v
 *        hardware HAL
 *              |
 *              v
 *           runtime
 *
 * This grammar never directly constructs or selects a physical ASIC.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * An ASIC declaration expresses semantic hardware intent.
 *
 * It MUST NOT require the source program to be rewritten merely because the
 * implementation changes from:
 *
 *     tiny ASIC
 *     large ASIC
 *     SoC
 *     heterogeneous SoC
 *     accelerator ASIC
 *     quantum control ASIC
 *     AI accelerator ASIC
 *     future hardware
 *
 * Physical implementation information belongs to:
 *
 *     target descriptions
 *     capability descriptions
 *     resource descriptions
 *     compilation context
 *     technology libraries
 *     deployment configuration
 *     hardware discovery
 *     backend configuration
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No fixed machine quantity is encoded here.
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_CELLS
 *     MAX_PINS
 *     MAX_PORTS
 *     MAX_NETS
 *     MAX_MODULES
 *     MAX_CORES
 *     MAX_MEMORY
 *     MAX_TRANSISTORS
 *     MAX_CLOCKS
 *     MAX_PIPELINES
 *     MAX_INSTANCES
 *     MAX_LANES
 *     MAX_WIDTH
 *     MAX_DEPTH
 *
 * Repetition is represented structurally using ANTLR repetition and
 * recursion.
 *
 * Numeric quantities are expressions.
 *
 * The semantic/resource layers determine whether a requested quantity can
 * be realized by a selected target.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary is supplied by:
 *
 *     grammar/lexer/tokens.g4
 *
 * This grammar therefore defines NO lexer rules.
 *
 * Important existing tokens used by this grammar include:
 *
 *     K_ASIC
 *     K_MODULE
 *     K_INTERFACE
 *     K_INPUT
 *     K_OUTPUT
 *     K_INOUT
 *     K_SIGNAL
 *     K_WIRE
 *     K_NET
 *     K_LOGIC
 *     K_BIT
 *     K_CONST
 *     K_FN
 *     K_IF
 *     K_ELSE
 *     K_FOR
 *     K_IN
 *     K_MATCH
 *     K_ASSERT
 *     K_RESOURCE
 *     K_CAPABILITY
 *     K_REQUIRES
 *     K_PROVIDES
 *     K_CONSTRAINT
 *     K_PREFER
 *     K_PLACE
 *     K_TARGET
 *     K_MEMORY
 *     K_PIPELINE
 *     K_CLOCK
 *     K_TIMING
 *     K_GENERATE
 *     K_INSTANCE
 *     K_CONNECT
 *     K_TO
 *     K_WIDTH
 *     K_LATENCY
 *     K_BANDWIDTH
 *     K_FREQUENCY
 *     K_PERIOD
 *     K_PHASE
 *     K_DELAY
 *     K_SETUP
 *     K_HOLD
 *     K_DEADLINE
 *
 * Where the lexer does not have a dedicated ASIC-specific keyword, this
 * grammar intentionally uses identifier-based property names rather than
 * adding vendor/process-node keywords.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Hardware quantities must remain expressions rather than lexer-level
 * constants.
 *
 * This permits:
 *
 *     width = data_width;
 *     memory = required_memory;
 *     lanes = available_lanes;
 *     frequency = target_frequency;
 *     latency <= latency_budget;
 *
 * The ASIC grammar provides a deliberately small local expression envelope
 * so this file does not become coupled to an incompatible expression grammar.
 *
 * The canonical language expression layer remains responsible for general
 * source expressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar HardwareAsic;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC ASIC DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     asic Name {
 *         ...
 *     }
 *
 * Optional generic parameters allow scalable source descriptions:
 *
 *     asic Accelerator<Width, Lanes> {
 *         ...
 *     }
 *
 * Generic parameters are semantic parameters, not physical machine limits.
 */

hardwareAsicDecl
    : hardwareAsicAnnotation*
      hardwareAsicVisibility?
      hardwareAsicModifier*
      K_ASIC
      IDENTIFIER
      hardwareAsicGenericParameters?
      hardwareAsicRequirementClause?
      hardwareAsicCapabilityClause?
      hardwareAsicBody?
    ;


/*
 * ============================================================================
 * 2. VISIBILITY
 * ============================================================================
 */

hardwareAsicVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


hardwareAsicModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/*
 * ============================================================================
 * 3. ANNOTATIONS
 * ============================================================================
 *
 * Annotation semantics are handled outside the parser.
 *
 * Vendor-specific annotations remain identifier-based rather than becoming
 * permanent language keywords.
 */

hardwareAsicAnnotation
    : AT
      IDENTIFIER
      (
          LPAREN
          hardwareAsicAnnotationArguments?
          RPAREN
      )?
    ;


hardwareAsicAnnotationArguments
    : hardwareAsicAnnotationArgument
      (
          COMMA
          hardwareAsicAnnotationArgument
      )*
    ;


hardwareAsicAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareAsicQualifiedName
    | hardwareAsicExpression
    ;


/*
 * ============================================================================
 * 4. GENERIC PARAMETERS
 * ============================================================================
 *
 * ASIC source must be able to describe scalable designs without embedding
 * fixed implementation sizes.
 */

hardwareAsicGenericParameters
    : LT
      hardwareAsicGenericParameter
      (
          COMMA
          hardwareAsicGenericParameter
      )*
      GT
    ;


hardwareAsicGenericParameter
    : IDENTIFIER
      (
          COLON
          hardwareAsicGenericBound
      )?
      (
          ASSIGN
          hardwareAsicExpression
      )?
    ;


hardwareAsicGenericBound
    : hardwareAsicQualifiedName
    | hardwareAsicCapabilityReference
    ;


/*
 * ============================================================================
 * 5. ASIC BODY
 * ============================================================================
 *
 * ASIC-specific declarations are kept separate from generic hardware
 * declarations so ownership remains explicit.
 */

hardwareAsicBody
    : LBRACE
      hardwareAsicItem*
      RBRACE
    ;


hardwareAsicItem
    : hardwareAsicAnnotation* hardwareAsicTechnologyDecl
    | hardwareAsicAnnotation* hardwareAsicModuleDecl
    | hardwareAsicAnnotation* hardwareAsicInterfaceDecl
    | hardwareAsicAnnotation* hardwareAsicPortDecl
    | hardwareAsicAnnotation* hardwareAsicSignalDecl
    | hardwareAsicAnnotation* hardwareAsicWireDecl
    | hardwareAsicAnnotation* hardwareAsicNetDecl
    | hardwareAsicAnnotation* hardwareAsicParameterDecl
    | hardwareAsicAnnotation* hardwareAsicConstantDecl
    | hardwareAsicAnnotation* hardwareAsicClockDecl
    | hardwareAsicAnnotation* hardwareAsicTimingDecl
    | hardwareAsicAnnotation* hardwareAsicCombinationalDecl
    | hardwareAsicAnnotation* hardwareAsicSequentialDecl
    | hardwareAsicAnnotation* hardwareAsicProcessDecl
    | hardwareAsicAnnotation* hardwareAsicMemoryDecl
    | hardwareAsicAnnotation* hardwareAsicPipelineDecl
    | hardwareAsicAnnotation* hardwareAsicCellDecl
    | hardwareAsicAnnotation* hardwareAsicMacroDecl
    | hardwareAsicAnnotation* hardwareAsicInstanceDecl
    | hardwareAsicAnnotation* hardwareAsicConnectionDecl
    | hardwareAsicAnnotation* hardwareAsicGenerateDecl
    | hardwareAsicAnnotation* hardwareAsicResourceDecl
    | hardwareAsicAnnotation* hardwareAsicCapabilityDecl
    | hardwareAsicAnnotation* hardwareAsicRequirementDecl
    | hardwareAsicAnnotation* hardwareAsicConstraintDecl
    | hardwareAsicAnnotation* hardwareAsicPreferenceDecl
    | hardwareAsicAnnotation* hardwareAsicPlacementDecl
    | hardwareAsicAnnotation* hardwareAsicTargetDecl
    | hardwareAsicAnnotation* hardwareAsicPropertyDecl
    | hardwareAsicAnnotation* hardwareAsicAssertion
    ;


/*
 * ============================================================================
 * 6. TECHNOLOGY INTENT
 * ============================================================================
 *
 * Technology is deliberately represented as a symbolic semantic value.
 *
 * This permits:
 *
 *     technology = target.technology;
 *     technology = implementation.technology;
 *
 * without making "7nm", "5nm", "3nm", a foundry name, or any particular
 * process node part of the language grammar.
 */

hardwareAsicTechnologyDecl
    : IDENTIFIER
      ASSIGN
      hardwareAsicExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. MODULES
 * ============================================================================
 */

hardwareAsicModuleDecl
    : K_MODULE
      IDENTIFIER
      hardwareAsicGenericParameters?
      hardwareAsicRequirementClause?
      hardwareAsicCapabilityClause?
      LBRACE
      hardwareAsicItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. INTERFACES
 * ============================================================================
 */

hardwareAsicInterfaceDecl
    : K_INTERFACE
      IDENTIFIER
      hardwareAsicGenericParameters?
      hardwareAsicInterfaceExtends?
      LBRACE
      hardwareAsicInterfaceMember*
      RBRACE
    ;


hardwareAsicInterfaceExtends
    : K_EXTENDS
      hardwareAsicQualifiedName
      (
          COMMA
          hardwareAsicQualifiedName
      )*
    ;


hardwareAsicInterfaceMember
    : hardwareAsicPortDecl
    | hardwareAsicSignalDecl
    | hardwareAsicClockDecl
    | hardwareAsicParameterDecl
    ;


/*
 * ============================================================================
 * 9. PORTS
 * ============================================================================
 */

hardwareAsicPortDecl
    : hardwareAsicPortDirection
      IDENTIFIER
      hardwareAsicPortType?
      hardwareAsicWidthClause?
      hardwareAsicPropertyBlock?
      SEMICOLON
    ;


hardwareAsicPortDirection
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;


hardwareAsicPortType
    : COLON
      hardwareAsicType
    ;


/*
 * ============================================================================
 * 10. SIGNALS
 * ============================================================================
 */

hardwareAsicSignalDecl
    : K_SIGNAL
      IDENTIFIER
      hardwareAsicType?
      hardwareAsicWidthClause?
      hardwareAsicInitializer?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. WIRES
 * ============================================================================
 */

hardwareAsicWireDecl
    : K_WIRE
      IDENTIFIER
      hardwareAsicType?
      hardwareAsicWidthClause?
      hardwareAsicInitializer?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. NETS
 * ============================================================================
 */

hardwareAsicNetDecl
    : K_NET
      IDENTIFIER
      hardwareAsicNetType?
      hardwareAsicWidthClause?
      SEMICOLON
    ;


hardwareAsicNetType
    : K_LOGIC
    | K_BIT
    | hardwareAsicQualifiedName
    ;


/*
 * ============================================================================
 * 13. PARAMETERS / CONSTANTS
 * ============================================================================
 */

hardwareAsicParameterDecl
    : K_PARAMETER
      IDENTIFIER
      (
          COLON
          hardwareAsicType
      )?
      (
          ASSIGN
          hardwareAsicExpression
      )?
      SEMICOLON
    ;


hardwareAsicConstantDecl
    : K_CONST
      IDENTIFIER
      COLON
      hardwareAsicType
      ASSIGN
      hardwareAsicExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. CLOCKS
 * ============================================================================
 *
 * No number of clock domains is fixed.
 *
 * Frequency and period are semantic quantities.
 */

hardwareAsicClockDecl
    : K_CLOCK
      IDENTIFIER
      hardwareAsicPropertyBlock?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. TIMING
 * ============================================================================
 */

hardwareAsicTimingDecl
    : K_TIMING
      IDENTIFIER?
      LBRACE
      hardwareAsicTimingItem*
      RBRACE
    ;


hardwareAsicTimingItem
    : K_FREQUENCY
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_PERIOD
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_DELAY
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_SETUP
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_HOLD
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_DEADLINE
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_PHASE
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | hardwareAsicPropertyStatement
    ;


/*
 * ============================================================================
 * 16. COMBINATIONAL LOGIC
 * ============================================================================
 */

hardwareAsicCombinationalDecl
    : IDENTIFIER
      LBRACE
      hardwareAsicLogicStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * 17. SEQUENTIAL LOGIC
 * ============================================================================
 */

hardwareAsicSequentialDecl
    : IDENTIFIER
      hardwareAsicClockBinding?
      LBRACE
      hardwareAsicLogicStatement*
      RBRACE
    ;


hardwareAsicClockBinding
    : K_CLOCK
      hardwareAsicQualifiedName
    ;


/*
 * ============================================================================
 * 18. PROCESSES
 * ============================================================================
 */

hardwareAsicProcessDecl
    : K_PROCESS
      IDENTIFIER?
      hardwareAsicSensitivityList?
      LBRACE
      hardwareAsicLogicStatement*
      RBRACE
    ;


hardwareAsicSensitivityList
    : LPAREN
      hardwareAsicSensitivity
      (
          COMMA
          hardwareAsicSensitivity
      )*
      RPAREN
    ;


hardwareAsicSensitivity
    : hardwareAsicQualifiedName
    | K_ANY
    | K_POSEDGE hardwareAsicQualifiedName
    | K_NEGEDGE hardwareAsicQualifiedName
    ;


/*
 * ============================================================================
 * 19. ASIC STANDARD-CELL / LOGICAL-CELL DECLARATIONS
 * ============================================================================
 *
 * Cell identity remains symbolic.
 *
 * A particular cell library is selected by semantic/target resolution rather
 * than by hard-coded grammar vocabulary.
 */

hardwareAsicCellDecl
    : IDENTIFIER
      IDENTIFIER
      hardwareAsicCellBody?
      SEMICOLON?
    ;


hardwareAsicCellBody
    : LBRACE
      hardwareAsicCellItem*
      RBRACE
    ;


hardwareAsicCellItem
    : hardwareAsicPortDecl
    | hardwareAsicPropertyStatement
    | hardwareAsicTimingItem
    ;


/*
 * ============================================================================
 * 20. MACROS / HARDWARE BLOCKS
 * ============================================================================
 *
 * A macro is a reusable logical implementation unit.
 *
 * It does not imply a physical fixed-size macro.
 */

hardwareAsicMacroDecl
    : IDENTIFIER
      IDENTIFIER
      hardwareAsicGenericParameters?
      LBRACE
      hardwareAsicItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 21. MEMORY
 * ============================================================================
 */

hardwareAsicMemoryDecl
    : K_MEMORY
      IDENTIFIER
      hardwareAsicMemorySpec?
      SEMICOLON
    ;


hardwareAsicMemorySpec
    : LBRACE
      hardwareAsicMemoryItem*
      RBRACE
    ;


hardwareAsicMemoryItem
    : K_WIDTH
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_DEPTH
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | hardwareAsicPropertyStatement
    ;


/*
 * ============================================================================
 * 22. PIPELINES
 * ============================================================================
 *
 * Stage count and width are expressions rather than constants.
 */

hardwareAsicPipelineDecl
    : K_PIPELINE
      IDENTIFIER
      hardwareAsicPipelineBody
    ;


hardwareAsicPipelineBody
    : LBRACE
      hardwareAsicPipelineItem*
      RBRACE
    ;


hardwareAsicPipelineItem
    : IDENTIFIER
      ASSIGN
      hardwareAsicExpression
      SEMICOLON

    | hardwareAsicPropertyStatement
    ;


/*
 * ============================================================================
 * 23. INSTANCES
 * ============================================================================
 */

hardwareAsicInstanceDecl
    : K_INSTANCE
      IDENTIFIER
      COLON
      hardwareAsicQualifiedName
      hardwareAsicGenericArguments?
      hardwareAsicInstanceArguments?
      SEMICOLON
    ;


hardwareAsicGenericArguments
    : LT
      hardwareAsicExpression
      (
          COMMA
          hardwareAsicExpression
      )*
      GT
    ;


hardwareAsicInstanceArguments
    : LPAREN
      hardwareAsicArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 24. CONNECTIONS
 * ============================================================================
 */

hardwareAsicConnectionDecl
    : K_CONNECT
      hardwareAsicEndpoint
      K_TO
      hardwareAsicEndpoint
      hardwareAsicPropertyBlock?
      SEMICOLON
    ;


hardwareAsicEndpoint
    : hardwareAsicQualifiedName
    | hardwareAsicQualifiedName
      LBRACKET
      hardwareAsicExpression
      RBRACKET
    | hardwareAsicQualifiedName
      DOT
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 25. GENERATION
 * ============================================================================
 *
 * Generation allows an arbitrary number of logical structures to be produced
 * from parameters or expressions.
 *
 * No fixed expansion count is encoded by this grammar.
 */

hardwareAsicGenerateDecl
    : K_GENERATE
      IDENTIFIER?
      K_FOR
      IDENTIFIER
      K_IN
      hardwareAsicExpression
      LBRACE
      hardwareAsicItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 26. RESOURCES
 * ============================================================================
 */

hardwareAsicResourceDecl
    : K_RESOURCE
      hardwareAsicQualifiedName
      hardwareAsicResourceBody?
      SEMICOLON?
    ;


hardwareAsicResourceBody
    : LBRACE
      hardwareAsicResourceItem*
      RBRACE
    ;


hardwareAsicResourceItem
    : hardwareAsicPropertyStatement
    | hardwareAsicRequirementStatement
    | hardwareAsicCapabilityStatement
    ;


/*
 * ============================================================================
 * 27. CAPABILITIES
 * ============================================================================
 */

hardwareAsicCapabilityDecl
    : K_CAPABILITY
      hardwareAsicCapabilityReference
      (
          ASSIGN
          hardwareAsicExpression
      )?
      SEMICOLON
    ;


hardwareAsicCapabilityClause
    : K_PROVIDES
      LBRACE
      hardwareAsicCapabilityStatement*
      RBRACE
    ;


hardwareAsicCapabilityStatement
    : hardwareAsicCapabilityReference
      (
          ASSIGN
          hardwareAsicExpression
      )?
      SEMICOLON
    ;


hardwareAsicCapabilityReference
    : hardwareAsicQualifiedName
    ;


/*
 * ============================================================================
 * 28. REQUIREMENTS
 * ============================================================================
 */

hardwareAsicRequirementDecl
    : K_REQUIRES
      hardwareAsicRequirementExpression
      SEMICOLON
    ;


hardwareAsicRequirementClause
    : K_REQUIRES
      LBRACE
      hardwareAsicRequirementExpression*
      RBRACE
    ;


hardwareAsicRequirementExpression
    : hardwareAsicExpression
      (
          hardwareAsicComparisonOperator
          hardwareAsicExpression
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 29. CONSTRAINTS
 * ============================================================================
 */

hardwareAsicConstraintDecl
    : K_CONSTRAINT
      IDENTIFIER?
      LBRACE
      hardwareAsicConstraintExpression*
      RBRACE
    ;


hardwareAsicConstraintExpression
    : hardwareAsicExpression
      hardwareAsicComparisonOperator
      hardwareAsicExpression
      SEMICOLON

    | K_NOT
      hardwareAsicConstraintExpression

    | LPAREN
      hardwareAsicConstraintExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 30. PREFERENCES
 * ============================================================================
 *
 * Preferences are not requirements.
 */

hardwareAsicPreferenceDecl
    : K_PREFER
      LBRACE
      hardwareAsicPreference*
      RBRACE
    ;


hardwareAsicPreference
    : hardwareAsicQualifiedName
      (
          ASSIGN
        | FAT_ARROW
      )
      hardwareAsicExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. PLACEMENT INTENT
 * ============================================================================
 *
 * Placement is intent only.
 *
 * Actual physical placement remains owned by placement/routing infrastructure.
 */

hardwareAsicPlacementDecl
    : K_PLACE
      IDENTIFIER?
      LBRACE
      hardwareAsicPlacementItem*
      RBRACE
    ;


hardwareAsicPlacementItem
    : hardwareAsicQualifiedName
      hardwareAsicPlacementOperator
      hardwareAsicQualifiedName
      SEMICOLON

    | hardwareAsicPropertyStatement
    ;


hardwareAsicPlacementOperator
    : K_NEAR
    | K_WITH
    | K_AVOID
    | K_GROUP
    | K_REGION
    | K_REQUIRE
    ;


/*
 * ============================================================================
 * 32. TARGET
 * ============================================================================
 *
 * Target describes semantic target intent.
 *
 * It does not mean that a physical ASIC has been selected.
 */

hardwareAsicTargetDecl
    : K_TARGET
      IDENTIFIER
      hardwareAsicTargetBody?
      SEMICOLON?
    ;


hardwareAsicTargetBody
    : LBRACE
      hardwareAsicTargetItem*
      RBRACE
    ;


hardwareAsicTargetItem
    : hardwareAsicPropertyStatement
    | hardwareAsicCapabilityStatement
    | hardwareAsicRequirementExpression
    ;


/*
 * ============================================================================
 * 33. PROPERTY BLOCKS
 * ============================================================================
 *
 * Property names are identifiers.
 *
 * This is intentional.
 *
 * It allows future ASIC concepts without permanently adding every technology,
 * foundry, process, packaging, interconnect, memory, power, or implementation
 * term to the Zamani lexer.
 */

hardwareAsicPropertyBlock
    : LBRACE
      hardwareAsicPropertyStatement*
      RBRACE
    ;


hardwareAsicPropertyDecl
    : IDENTIFIER
      ASSIGN
      hardwareAsicExpression
      SEMICOLON
    ;


hardwareAsicPropertyStatement
    : hardwareAsicQualifiedName
      (
          ASSIGN
        | FAT_ARROW
      )
      hardwareAsicExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. LOGIC STATEMENTS
 * ============================================================================
 */

hardwareAsicLogicStatement
    : hardwareAsicAssignment
    | hardwareAsicConditional
    | hardwareAsicInstanceDecl
    | hardwareAsicConnectionDecl
    | hardwareAsicAssertion
    | hardwareAsicPropertyStatement
    | LBRACE
      hardwareAsicLogicStatement*
      RBRACE
    ;


hardwareAsicAssignment
    : hardwareAsicLValue
      ASSIGN
      hardwareAsicExpression
      SEMICOLON
    ;


hardwareAsicConditional
    : K_IF
      LPAREN
      hardwareAsicExpression
      RPAREN
      hardwareAsicLogicStatement
      (
          K_ELSE
          hardwareAsicLogicStatement
      )?
    ;


hardwareAsicAssertion
    : K_ASSERT
      LPAREN
      hardwareAsicExpression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 35. TYPES
 * ============================================================================
 */

hardwareAsicType
    : hardwareAsicQualifiedName
    | K_BIT
      hardwareAsicWidthClause?
    | K_LOGIC
      hardwareAsicWidthClause?
    | hardwareAsicArrayType
    ;


hardwareAsicArrayType
    : LBRACKET
      hardwareAsicExpression
      RBRACKET
      hardwareAsicType
    ;


hardwareAsicWidthClause
    : LBRACKET
      hardwareAsicExpression
      RBRACKET
    ;


/*
 * ============================================================================
 * 36. INITIALIZERS
 * ============================================================================
 */

hardwareAsicInitializer
    : ASSIGN
      hardwareAsicExpression
    ;


/*
 * ============================================================================
 * 37. QUALIFIED NAMES
 * ============================================================================
 *
 * Arbitrary qualification depth is allowed.
 *
 * Examples:
 *
 *     technology::library
 *     target::technology
 *     memory::bank
 *     implementation::constraint::timing
 *
 * No namespace-depth limit is imposed.
 */

hardwareAsicQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 38. ARGUMENTS
 * ============================================================================
 */

hardwareAsicArgumentList
    : hardwareAsicExpression
      (
          COMMA
          hardwareAsicExpression
      )*
    ;


/*
 * ============================================================================
 * 39. LVALUES
 * ============================================================================
 */

hardwareAsicLValue
    : hardwareAsicQualifiedName
    | hardwareAsicQualifiedName
      LBRACKET
      hardwareAsicExpression
      RBRACKET
    | hardwareAsicQualifiedName
      DOT
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 40. COMPARISON OPERATORS
 * ============================================================================
 */

hardwareAsicComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_EQUAL
    | GREATER_EQUAL
    | LT
    | GT
    ;


/*
 * ============================================================================
 * 41. EXPRESSIONS
 * ============================================================================
 *
 * This expression envelope exists only to keep this specialized grammar
 * independent from unrelated expression grammar implementation details.
 *
 * It supports symbolic hardware quantities and ordinary arithmetic/logic
 * needed for hardware declarations.
 *
 * Semantic analysis remains responsible for:
 *
 *     - type checking;
 *     - units;
 *     - dimensional analysis;
 *     - overflow;
 *     - constant evaluation;
 *     - resource evaluation;
 *     - target representability;
 *     - legality of dynamic values.
 *
 * The grammar imposes no maximum value or machine capacity.
 */

hardwareAsicExpression
    : hardwareAsicConditionalExpression
    ;


hardwareAsicConditionalExpression
    : hardwareAsicLogicalOrExpression
      (
          QUESTION
          hardwareAsicExpression
          COLON
          hardwareAsicExpression
      )?
    ;


hardwareAsicLogicalOrExpression
    : hardwareAsicLogicalAndExpression
      (
          LOGICAL_OR
          hardwareAsicLogicalAndExpression
      )*
    ;


hardwareAsicLogicalAndExpression
    : hardwareAsicBitwiseOrExpression
      (
          LOGICAL_AND
          hardwareAsicBitwiseOrExpression
      )*
    ;


hardwareAsicBitwiseOrExpression
    : hardwareAsicBitwiseXorExpression
      (
          PIPE
          hardwareAsicBitwiseXorExpression
      )*
    ;


hardwareAsicBitwiseXorExpression
    : hardwareAsicBitwiseAndExpression
      (
          CARET
          hardwareAsicBitwiseAndExpression
      )*
    ;


hardwareAsicBitwiseAndExpression
    : hardwareAsicEqualityExpression
      (
          AMPERSAND
          hardwareAsicEqualityExpression
      )*
    ;


hardwareAsicEqualityExpression
    : hardwareAsicRelationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          hardwareAsicRelationalExpression
      )*
    ;


hardwareAsicRelationalExpression
    : hardwareAsicAdditiveExpression
      (
          (
              LT
            | GT
            | LESS_EQUAL
            | GREATER_EQUAL
          )
          hardwareAsicAdditiveExpression
      )*
    ;


hardwareAsicAdditiveExpression
    : hardwareAsicMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          hardwareAsicMultiplicativeExpression
      )*
    ;


hardwareAsicMultiplicativeExpression
    : hardwareAsicUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          hardwareAsicUnaryExpression
      )*
    ;


hardwareAsicUnaryExpression
    : (
          PLUS
        | MINUS
        | TILDE
        | EXCLAMATION
      )*
      hardwareAsicPostfixExpression
    ;


hardwareAsicPostfixExpression
    : hardwareAsicPrimaryExpression
      (
          LBRACKET
          hardwareAsicExpression
          RBRACKET
        | DOT
          IDENTIFIER
      )*
    ;


hardwareAsicPrimaryExpression
    : IDENTIFIER
    | hardwareAsicQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | K_TRUE
    | K_FALSE
    | K_NULL
    | K_NIL
    | LPAREN
      hardwareAsicExpression
      RPAREN
    ;


/*
 * ============================================================================
 * END OF FILE
 * ============================================================================
 *
 * Completion invariant:
 *
 *     This grammar defines ASIC SOURCE SYNTAX only.
 *
 *     It does not define:
 *
 *         ASIC implementation algorithms
 *         ASIC physical design
 *         foundry data
 *         process technology data
 *         standard-cell libraries
 *         timing models
 *         power models
 *         placement algorithms
 *         routing algorithms
 *         hardware discovery
 *         target selection
 *         resource limits
 *         runtime behavior
 *         quantum IR
 *         QEC
 *         ZQN
 *
 * Those remain downstream semantic/backend responsibilities.
 *
 * ============================================================================
 */