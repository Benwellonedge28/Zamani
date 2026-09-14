/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *   grammar/hardware/hardware.g4
 *
 * Grammar kind:
 *   ANTLR4 parser grammar
 *
 * Rust target:
 *   Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *   This grammar contains no embedded target-language actions and therefore
 *   introduces no unsafe Rust. Generated Rust must additionally be compiled
 *   under the repository-wide unsafe-code prohibition.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - machine-independent hardware declarations;
 *   - hardware modules;
 *   - ports;
 *   - interfaces;
 *   - signals;
 *   - nets and wires;
 *   - hardware parameters;
 *   - hardware generics;
 *   - clocks;
 *   - timing contracts;
 *   - combinational regions;
 *   - sequential regions;
 *   - hardware processes;
 *   - state-machine declarations;
 *   - memories;
 *   - pipelines;
 *   - hardware resources;
 *   - hardware capabilities;
 *   - hardware requirements;
 *   - hardware constraints;
 *   - hardware preferences;
 *   - placement intent;
 *   - target-independent hardware descriptions;
 *   - accelerator declarations;
 *   - CPU/GPU/FPGA/ASIC/quantum-device capability declarations;
 *   - hardware composition;
 *   - hardware instantiation;
 *   - hardware connections;
 *   - implementation-independent topology descriptions;
 *   - hardware annotations;
 *   - hardware-level compile/deployment intent.
 *
 * THIS FILE DOES NOT OWN
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - numeric literal recognition;
 *   - strings;
 *   - comments;
 *   - the canonical Zamani AST;
 *   - semantic type checking;
 *   - hardware discovery;
 *   - hardware calibration;
 *   - physical device enumeration;
 *   - physical topology discovery;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN/noise semantics;
 *   - runtime dispatch;
 *   - device drivers;
 *   - backend-specific resource limits.
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
 * hardware.g4
 *   |
 *   v
 * syntax/AST
 *   |
 *   v
 * semantic analysis
 *   |
 *   +----------------------+
 *   |                      |
 *   v                      v
 * hardware model       quantum::ir
 *   |                      |
 *   +----------+-----------+
 *              |
 *              v
 *      capability/resource
 *          resolution
 *              |
 *              v
 *      optimization/routing
 *              |
 *              v
 *          scheduling
 *              |
 *              v
 *       hardware HAL/runtime
 *
 * The grammar never directly selects a physical machine.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A hardware declaration expresses:
 *
 *   - what the program requires;
 *   - what an implementation provides;
 *   - what constraints must hold;
 *   - what capabilities are preferred;
 *   - what implementation choices are permitted.
 *
 * It does NOT inherently express:
 *
 *   - a fixed number of devices;
 *   - a fixed number of cores;
 *   - a fixed number of qubits;
 *   - a fixed amount of memory;
 *   - a fixed topology;
 *   - a fixed device ID;
 *   - a fixed hardware address;
 *   - a fixed clock frequency;
 *   - a fixed physical location;
 *   - a fixed vendor.
 *
 * Those properties belong to target descriptions, capabilities, runtime
 * discovery, deployment configuration, or backend-specific implementations.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No MAX_* hardware quantity is defined here.
 *
 * Repetition is represented structurally:
 *
 *   item*
 *   item+
 *
 * rather than by fixed-size grammar constructs.
 *
 * Quantities are expressions rather than parser-level constants so that the
 * semantic/resource layer can determine representability from the compilation
 * and execution context.
 *
 * ============================================================================
 * INTEGRATION
 * ============================================================================
 *
 * Required lexical vocabulary:
 *
 *   grammar/lexer/tokens.g4
 *
 * The canonical lexical layer is responsible for:
 *
 *   K_INPUT
 *   K_OUTPUT
 *   K_INOUT
 *   K_INTERFACE
 *   K_SIGNAL
 *   K_WIRE
 *   K_NET
 *   K_LOGIC
 *   K_BIT
 *   and the remaining hardware/resource vocabulary.
 *
 * Shared parser concepts are intentionally referenced through stable token
 * vocabulary and small local grammar abstractions rather than duplicated
 * semantic types.
 *
 * The root parser should expose this file's:
 *
 *   hardwareDeclaration
 *
 * as a declaration alternative.
 *
 * ============================================================================
 * IMPORTANT
 * ============================================================================
 *
 * This grammar deliberately permits symbolic expressions in dimensions,
 * widths, capacities, timing values, resource quantities, and constraints.
 *
 * For example:
 *
 *   width = data_width;
 *   lanes = available_lanes;
 *   memory = required_memory;
 *   capacity >= workload_size;
 *
 * are syntactically valid.
 *
 * The semantic layer decides whether such expressions are constant,
 * runtime-dependent, target-dependent, or otherwise resolvable.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareParser;

options {
    tokenVocab = ZamaniTokens;
}


// ============================================================================
// 1. PUBLIC ENTRY POINT
// ============================================================================

/**
 * Public entry point consumed by the root Zamani parser.
 *
 * The root parser is responsible for deciding where a hardware declaration
 * is legal in a compilation unit.
 */
hardwareDeclaration
    : hardwareAnnotation*
      hardwareVisibility?
      hardwareModifier*
      K_HARDWARE
      IDENTIFIER
      hardwareGenericParameters?
      hardwareRequirementClause?
      hardwareCapabilityClause?
      LBRACE hardwareItem* RBRACE
    ;


// ============================================================================
// 2. HARDWARE DECLARATION MODIFIERS
// ============================================================================

hardwareVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;

hardwareModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


// ============================================================================
// 3. HARDWARE ANNOTATIONS
// ============================================================================
//
// Hardware annotations are syntactic metadata only.
// Their semantics are interpreted outside the grammar.
//
// Example:
//
//   @portable
//   @synthesizable
//   @simulation
//
// Vendor/device-specific annotations remain legal through qualified names
// rather than being hard-coded into this grammar.
//

hardwareAnnotation
    : AT IDENTIFIER
      (LPAREN hardwareAnnotationArguments? RPAREN)?
    ;

hardwareAnnotationArguments
    : hardwareAnnotationArgument
      (COMMA hardwareAnnotationArgument)*
    ;

hardwareAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareQualifiedName
    | hardwareExpression
    ;


// ============================================================================
// 4. GENERICS
// ============================================================================
//
// Hardware generics allow source-level hardware descriptions to remain
// independent of a particular resource size.
//
// Example:
//
//   hardware Accelerator<Width, Lanes, Memory> { ... }
//
// Width/Lanes/Memory are semantic parameters. They are not parser limits.
//

hardwareGenericParameters
    : LT hardwareGenericParameter
      (COMMA hardwareGenericParameter)*
      GT
    ;

hardwareGenericParameter
    : IDENTIFIER
      (COLON hardwareGenericBound)?
      (ASSIGN hardwareExpression)?
    ;

hardwareGenericBound
    : hardwareQualifiedName
    | hardwareCapabilityReference
    ;


// ============================================================================
// 5. HARDWARE BODY
// ============================================================================

hardwareItem
    : hardwareAnnotation* hardwareModuleDecl
    | hardwareAnnotation* hardwarePortDecl
    | hardwareAnnotation* hardwareInterfaceDecl
    | hardwareAnnotation* hardwareSignalDecl
    | hardwareAnnotation* hardwareWireDecl
    | hardwareAnnotation* hardwareNetDecl
    | hardwareAnnotation* hardwareParameterDecl
    | hardwareAnnotation* hardwareConstantDecl
    | hardwareAnnotation* hardwareClockDecl
    | hardwareAnnotation* hardwareTimingDecl
    | hardwareAnnotation* hardwareCombinationalDecl
    | hardwareAnnotation* hardwareSequentialDecl
    | hardwareAnnotation* hardwareProcessDecl
    | hardwareAnnotation* hardwareStateMachineDecl
    | hardwareAnnotation* hardwareMemoryDecl
    | hardwareAnnotation* hardwarePipelineDecl
    | hardwareAnnotation* hardwareResourceDecl
    | hardwareAnnotation* hardwareCapabilityDecl
    | hardwareAnnotation* hardwareRequirementDecl
    | hardwareAnnotation* hardwareConstraintDecl
    | hardwareAnnotation* hardwarePreferenceDecl
    | hardwareAnnotation* hardwarePlacementDecl
    | hardwareAnnotation* hardwareTargetDecl
    | hardwareAnnotation* hardwareAcceleratorDecl
    | hardwareAnnotation* hardwareCpuDecl
    | hardwareAnnotation* hardwareGpuDecl
    | hardwareAnnotation* hardwareFpgaDecl
    | hardwareAnnotation* hardwareAsicDecl
    | hardwareAnnotation* hardwareQuantumDeviceDecl
    | hardwareAnnotation* hardwareInstanceDecl
    | hardwareAnnotation* hardwareConnectionDecl
    | hardwareAnnotation* hardwareGenerateDecl
    | hardwareAnnotation* hardwareFunctionDecl
    | hardwareAnnotation* hardwareAssertion
    | hardwareAnnotation* hardwareUsingDecl
    ;


// ============================================================================
// 6. MODULE COMPOSITION
// ============================================================================

hardwareModuleDecl
    : K_MODULE
      IDENTIFIER
      hardwareGenericParameters?
      hardwareRequirementClause?
      hardwareCapabilityClause?
      LBRACE hardwareItem* RBRACE
    ;


// ============================================================================
// 7. PORTS
// ============================================================================

hardwarePortDecl
    : (K_INPUT | K_OUTPUT | K_INOUT)
      IDENTIFIER
      hardwarePortType
      hardwarePortAttributes?
      SEMICOLON
    ;

hardwarePortType
    : COLON hardwareType
    | hardwareType
    ;

hardwarePortAttributes
    : LBRACE hardwarePortAttribute*
      RBRACE
    ;

hardwarePortAttribute
    : K_WIDTH ASSIGN hardwareExpression SEMICOLON
    | K_DIRECTION ASSIGN hardwareDirection SEMICOLON
    | K_PROTOCOL ASSIGN hardwareQualifiedName SEMICOLON
    | K_CLOCK ASSIGN hardwareQualifiedName SEMICOLON
    | K_RESET ASSIGN hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;

hardwareDirection
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;


// ============================================================================
// 8. INTERFACES
// ============================================================================

hardwareInterfaceDecl
    : K_INTERFACE
      IDENTIFIER
      hardwareGenericParameters?
      hardwareInterfaceExtends?
      LBRACE hardwareInterfaceMember* RBRACE
    ;

hardwareInterfaceExtends
    : K_EXTENDS
      hardwareQualifiedName
      (COMMA hardwareQualifiedName)*
    ;

hardwareInterfaceMember
    : hardwareAnnotation* hardwarePortDecl
    | hardwareAnnotation* hardwareSignalDecl
    | hardwareAnnotation* hardwareClockDecl
    | hardwareAnnotation* hardwareFunctionDecl
    | hardwareAnnotation* hardwareParameterDecl
    | hardwareAnnotation* hardwareRequirementClause
    | hardwareAnnotation* hardwareCapabilityClause
    ;


// ============================================================================
// 9. SIGNALS
// ============================================================================

hardwareSignalDecl
    : K_SIGNAL
      IDENTIFIER
      hardwareSignalType?
      hardwareWidthClause?
      hardwareInitializer?
      SEMICOLON
    ;

hardwareSignalType
    : COLON hardwareType
    ;

hardwareWidthClause
    : LBRACKET hardwareDimension RBRACKET
    ;

hardwareDimension
    : hardwareExpression
    ;


// ============================================================================
// 10. WIRES
// ============================================================================

hardwareWireDecl
    : K_WIRE
      IDENTIFIER
      hardwareType?
      hardwareWidthClause?
      hardwareInitializer?
      SEMICOLON
    ;


// ============================================================================
// 11. NETS
// ============================================================================

hardwareNetDecl
    : K_NET
      IDENTIFIER
      hardwareNetKind?
      hardwareType?
      hardwareWidthClause?
      hardwareInitializer?
      SEMICOLON
    ;

hardwareNetKind
    : K_LOGIC
    | K_BIT
    | IDENTIFIER
    ;


// ============================================================================
// 12. PARAMETERS
// ============================================================================

hardwareParameterDecl
    : K_PARAMETER
      IDENTIFIER
      (COLON hardwareType)?
      (ASSIGN hardwareExpression)?
      SEMICOLON
    ;

hardwareConstantDecl
    : K_CONST
      IDENTIFIER
      COLON hardwareType
      ASSIGN hardwareExpression
      SEMICOLON
    ;


// ============================================================================
// 13. CLOCKS
// ============================================================================
//
// Frequency is expressed symbolically or numerically.
//
// The grammar never assumes MHz/GHz or any particular clock domain count.
//

hardwareClockDecl
    : K_CLOCK
      IDENTIFIER
      hardwareClockSpec?
      SEMICOLON
    ;

hardwareClockSpec
    : COLON hardwareType
    | hardwareClockPropertyBlock
    ;

hardwareClockPropertyBlock
    : LBRACE
      hardwareClockProperty*
      RBRACE
    ;

hardwareClockProperty
    : K_FREQUENCY ASSIGN hardwareExpression SEMICOLON
    | K_PERIOD ASSIGN hardwareExpression SEMICOLON
    | K_PHASE ASSIGN hardwareExpression SEMICOLON
    | K_DUTY ASSIGN hardwareExpression SEMICOLON
    | K_SOURCE ASSIGN hardwareQualifiedName SEMICOLON
    | K_DOMAIN ASSIGN hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 14. TIMING
// ============================================================================

hardwareTimingDecl
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
    : K_PERIOD ASSIGN hardwareExpression SEMICOLON
    | K_FREQUENCY ASSIGN hardwareExpression SEMICOLON
    | K_LATENCY ASSIGN hardwareExpression SEMICOLON
    | K_DELAY ASSIGN hardwareExpression SEMICOLON
    | K_SETUP ASSIGN hardwareExpression SEMICOLON
    | K_HOLD ASSIGN hardwareExpression SEMICOLON
    | K_DEADLINE ASSIGN hardwareExpression SEMICOLON
    | K_PHASE ASSIGN hardwareExpression SEMICOLON
    | hardwareTimingRelation SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;

hardwareTimingRelation
    : hardwareQualifiedName
      K_MEETS
      hardwareQualifiedName
    ;


// ============================================================================
// 15. COMBINATIONAL LOGIC
// ============================================================================

hardwareCombinationalDecl
    : K_COMBINATIONAL
      IDENTIFIER?
      LBRACE
      hardwareLogicStatement*
      RBRACE
    ;


// ============================================================================
// 16. SEQUENTIAL LOGIC
// ============================================================================

hardwareSequentialDecl
    : K_SEQUENTIAL
      IDENTIFIER?
      hardwareClockBinding?
      LBRACE
      hardwareLogicStatement*
      RBRACE
    ;

hardwareClockBinding
    : K_CLOCK
      hardwareQualifiedName
    ;


// ============================================================================
// 17. HARDWARE PROCESSES
// ============================================================================

hardwareProcessDecl
    : K_PROCESS
      IDENTIFIER?
      hardwareSensitivityList?
      LBRACE
      hardwareLogicStatement*
      RBRACE
    ;

hardwareSensitivityList
    : LPAREN
      hardwareSensitivity
      (COMMA hardwareSensitivity)*
      RPAREN
    ;

hardwareSensitivity
    : hardwareQualifiedName
    | K_ANY
    | K_POSEDGE hardwareQualifiedName
    | K_NEGEDGE hardwareQualifiedName
    ;


// ============================================================================
// 18. HARDWARE LOGIC STATEMENTS
// ============================================================================

hardwareLogicStatement
    : hardwareAssignment
    | hardwareConditional
    | hardwareCase
    | hardwareLoop
    | hardwareProcessCall
    | hardwareAssertion
    | hardwareInstanceDecl
    | hardwareConnectionDecl
    | hardwareExpression SEMICOLON
    | LBRACE hardwareLogicStatement* RBRACE
    ;

hardwareAssignment
    : hardwareLValue ASSIGN hardwareExpression SEMICOLON
    ;

hardwareConditional
    : K_IF
      LPAREN hardwareExpression RPAREN
      hardwareLogicStatement
      (K_ELSE hardwareLogicStatement)?
    ;

hardwareCase
    : K_MATCH
      hardwareExpression
      LBRACE
      hardwareCaseArm+
      RBRACE
    ;

hardwareCaseArm
    : hardwareExpression FAT_ARROW hardwareLogicStatement
    | K_DEFAULT FAT_ARROW hardwareLogicStatement
    ;

hardwareLoop
    : K_FOR
      IDENTIFIER
      K_IN
      hardwareExpression
      hardwareLogicStatement
    ;

hardwareProcessCall
    : hardwareQualifiedName
      LPAREN
      hardwareArgumentList?
      RPAREN
      SEMICOLON
    ;

hardwareLValue
    : hardwareQualifiedName
    | hardwareQualifiedName LBRACKET hardwareExpression RBRACKET
    | hardwareQualifiedName DOT IDENTIFIER
    ;


// ============================================================================
// 19. STATE MACHINES
// ============================================================================

hardwareStateMachineDecl
    : K_STATE_MACHINE
      IDENTIFIER
      hardwareStateMachineBody
    ;

hardwareStateMachineBody
    : LBRACE
      hardwareStateMachineItem*
      RBRACE
    ;

hardwareStateMachineItem
    : K_STATE
      IDENTIFIER
      (COMMA IDENTIFIER)*
      SEMICOLON
    | K_INITIAL
      IDENTIFIER
      SEMICOLON
    | K_TRANSITION
      hardwareStateTransition
      SEMICOLON
    | K_OUTPUT
      hardwareExpression
      SEMICOLON
    ;

hardwareStateTransition
    : IDENTIFIER
      (K_ON hardwareExpression)?
      (K_IF hardwareExpression)?
      FAT_ARROW
      IDENTIFIER
      (K_DO hardwareLogicStatement)?
    ;


// ============================================================================
// 20. MEMORIES
// ============================================================================
//
// Memory dimensions are expressions.
//
// This deliberately permits:
//
//   memory words;
//   parameterized memory;
//   inferred memory;
//   externally supplied memory;
//   implementation-selected memory.
//

hardwareMemoryDecl
    : K_MEMORY
      IDENTIFIER
      hardwareMemorySpec
      SEMICOLON?
    ;

hardwareMemorySpec
    : COLON hardwareType
      hardwareMemoryDimensions?
      hardwareMemoryProperties?
    | hardwareMemoryDimensions
      hardwareMemoryProperties?
    ;

hardwareMemoryDimensions
    : LBRACKET
      hardwareDimensionList
      RBRACKET
    ;

hardwareDimensionList
    : hardwareExpression
      (COMMA hardwareExpression)*
    ;

hardwareMemoryProperties
    : LBRACE
      hardwareMemoryProperty*
      RBRACE
    ;

hardwareMemoryProperty
    : K_DEPTH ASSIGN hardwareExpression SEMICOLON
    | K_WIDTH ASSIGN hardwareExpression SEMICOLON
    | K_READ_PORTS ASSIGN hardwareExpression SEMICOLON
    | K_WRITE_PORTS ASSIGN hardwareExpression SEMICOLON
    | K_LATENCY ASSIGN hardwareExpression SEMICOLON
    | K_ALIGNMENT ASSIGN hardwareExpression SEMICOLON
    | K_BANKS ASSIGN hardwareExpression SEMICOLON
    | K_LAYOUT ASSIGN hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 21. PIPELINES
// ============================================================================

hardwarePipelineDecl
    : K_PIPELINE
      IDENTIFIER
      hardwarePipelineBody
    ;

hardwarePipelineBody
    : LBRACE
      hardwarePipelineItem*
      RBRACE
    ;

hardwarePipelineItem
    : K_STAGE
      IDENTIFIER
      (hardwarePipelineLatency)?
      LBRACE
      hardwareLogicStatement*
      RBRACE
    | K_DEPTH ASSIGN hardwareExpression SEMICOLON
    | K_THROUGHPUT ASSIGN hardwareExpression SEMICOLON
    | K_LATENCY ASSIGN hardwareExpression SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;

hardwarePipelineLatency
    : COLON hardwareExpression
    ;


// ============================================================================
// 22. RESOURCES
// ============================================================================
//
// Resource declarations describe requirements or logical resources.
// They do not allocate or discover physical resources.
//

hardwareResourceDecl
    : K_RESOURCE
      IDENTIFIER
      hardwareResourceKind?
      hardwareResourceProperties?
      SEMICOLON?
    ;

hardwareResourceKind
    : hardwareQualifiedName
    ;

hardwareResourceProperties
    : LBRACE
      hardwareResourceProperty*
      RBRACE
    ;

hardwareResourceProperty
    : K_CAPACITY ASSIGN hardwareExpression SEMICOLON
    | K_QUANTITY ASSIGN hardwareExpression SEMICOLON
    | K_SIZE ASSIGN hardwareExpression SEMICOLON
    | K_WIDTH ASSIGN hardwareExpression SEMICOLON
    | K_COUNT ASSIGN hardwareExpression SEMICOLON
    | K_LATENCY ASSIGN hardwareExpression SEMICOLON
    | K_BANDWIDTH ASSIGN hardwareExpression SEMICOLON
    | K_ENERGY ASSIGN hardwareExpression SEMICOLON
    | K_RELIABILITY ASSIGN hardwareExpression SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 23. CAPABILITIES
// ============================================================================

hardwareCapabilityDecl
    : K_CAPABILITY
      IDENTIFIER
      hardwareCapabilityBody?
      SEMICOLON?
    ;

hardwareCapabilityBody
    : LBRACE
      hardwareCapabilityItem*
      RBRACE
    ;

hardwareCapabilityItem
    : K_PROVIDES
      hardwareCapabilityReference
      SEMICOLON
    | K_SUPPORTS
      hardwareCapabilityReference
      SEMICOLON
    | K_VERSION
      hardwareExpression
      SEMICOLON
    | K_LIMIT
      hardwareExpression
      SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;

hardwareCapabilityReference
    : hardwareQualifiedName
    ;


// ============================================================================
// 24. REQUIREMENTS
// ============================================================================
//
// Requirements express what an implementation must provide.
//
// They are not device selection.
//

hardwareRequirementClause
    : K_REQUIRES
      LBRACE
      hardwareRequirement*
      RBRACE
    ;

hardwareRequirement
    : hardwareRequirementExpression SEMICOLON
    ;

hardwareRequirementExpression
    : hardwareCapabilityRequirement
    | hardwareResourceRequirement
    | hardwarePropertyRequirement
    | hardwareConstraintExpression
    ;

hardwareCapabilityRequirement
    : K_CAPABILITY
      hardwareCapabilityReference
    ;

hardwareResourceRequirement
    : K_RESOURCE
      hardwareQualifiedName
      (hardwareComparisonOperator hardwareExpression)?
    ;

hardwarePropertyRequirement
    : hardwareQualifiedName
      hardwareComparisonOperator
      hardwareExpression
    ;


// ============================================================================
// 25. CAPABILITY BLOCK
// ============================================================================

hardwareCapabilityClause
    : K_PROVIDES
      LBRACE
      hardwareProvidedCapability*
      RBRACE
    ;

hardwareProvidedCapability
    : hardwareCapabilityReference
      (ASSIGN hardwareExpression)?
      SEMICOLON
    ;


// ============================================================================
// 26. CONSTRAINTS
// ============================================================================

hardwareConstraintDecl
    : K_CONSTRAINT
      IDENTIFIER?
      LBRACE
      hardwareConstraintExpression*
      RBRACE
    ;

hardwareConstraintExpression
    : hardwareExpression
      hardwareComparisonOperator
      hardwareExpression
      SEMICOLON
    | K_NOT
      hardwareConstraintExpression
    | LPAREN
      hardwareConstraintExpression
      RPAREN
    | hardwareQualifiedName
      LPAREN
      hardwareArgumentList?
      RPAREN
      SEMICOLON
    ;

hardwareComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_EQUAL
    | GREATER_EQUAL
    | LT
    | GT
    ;


// ============================================================================
// 27. PREFERENCES
// ============================================================================
//
// Preferences are deliberately weaker than requirements.
// The compiler may satisfy them when possible without making them semantic
// correctness conditions.
//

hardwarePreferenceDecl
    : K_PREFER
      LBRACE
      hardwarePreference*
      RBRACE
    ;

hardwarePreference
    : hardwareQualifiedName
      (ASSIGN | FAT_ARROW)
      hardwareExpression
      SEMICOLON
    ;


// ============================================================================
// 28. PLACEMENT
// ============================================================================
//
// Placement expresses intent, not final physical routing.
//
// The routing subsystem owns physical placement.
//

hardwarePlacementDecl
    : K_PLACE
      IDENTIFIER?
      hardwarePlacementBody
    ;

hardwarePlacementBody
    : LBRACE
      hardwarePlacementItem*
      RBRACE
    ;

hardwarePlacementItem
    : K_NEAR hardwareQualifiedName SEMICOLON
    | K_WITH hardwareQualifiedName SEMICOLON
    | K_AVOID hardwareQualifiedName SEMICOLON
    | K_GROUP hardwareQualifiedName SEMICOLON
    | K_REGION hardwareQualifiedName SEMICOLON
    | K_REQUIRE hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 29. TARGET DESCRIPTIONS
// ============================================================================
//
// A target is a semantic target profile, not necessarily a physical device.
//
// The actual target resolution is owned by the compilation/backend layers.
//

hardwareTargetDecl
    : K_TARGET
      IDENTIFIER
      hardwareTargetBody?
      SEMICOLON?
    ;

hardwareTargetBody
    : LBRACE
      hardwareTargetItem*
      RBRACE
    ;

hardwareTargetItem
    : K_ARCHITECTURE ASSIGN hardwareQualifiedName SEMICOLON
    | K_ABI ASSIGN hardwareQualifiedName SEMICOLON
    | K_FORMAT ASSIGN hardwareQualifiedName SEMICOLON
    | K_CAPABILITY hardwareCapabilityReference SEMICOLON
    | K_REQUIRES hardwareRequirementExpression SEMICOLON
    | K_PREFER hardwareQualifiedName ASSIGN hardwareExpression SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 30. GENERIC ACCELERATOR
// ============================================================================
//
// Accelerator is intentionally generic.
// CUDA, ROCm, FPGA, QPU, TPU, NPU, DSP, etc. can be represented through
// capabilities/dialects without forcing the core grammar to enumerate every
// future accelerator.
//

hardwareAcceleratorDecl
    : K_ACCELERATOR
      IDENTIFIER
      hardwareAcceleratorKind?
      hardwareAcceleratorBody?
    ;

hardwareAcceleratorKind
    : COLON hardwareQualifiedName
    ;

hardwareAcceleratorBody
    : LBRACE
      hardwareAcceleratorItem*
      RBRACE
    ;

hardwareAcceleratorItem
    : K_CAPABILITY hardwareCapabilityReference SEMICOLON
    | K_RESOURCE hardwareQualifiedName SEMICOLON
    | K_INTERFACE hardwareQualifiedName SEMICOLON
    | K_PIPELINE hardwareQualifiedName SEMICOLON
    | K_MEMORY hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 31. CPU
// ============================================================================
//
// This is a semantic capability declaration.
// It does not impose a core/thread/register maximum.
//

hardwareCpuDecl
    : K_CPU
      IDENTIFIER
      hardwareComputeBody?
    ;


// ============================================================================
// 32. GPU
// ============================================================================
//
// No CUDA-specific machine dimensions are embedded here.
//

hardwareGpuDecl
    : K_GPU
      IDENTIFIER
      hardwareComputeBody?
    ;


// ============================================================================
// 33. FPGA
// ============================================================================

hardwareFpgaDecl
    : K_FPGA
      IDENTIFIER
      hardwareProgrammableBody?
    ;


// ============================================================================
// 34. ASIC
// ============================================================================

hardwareAsicDecl
    : K_ASIC
      IDENTIFIER
      hardwareProgrammableBody?
    ;


// ============================================================================
// 35. QUANTUM DEVICE
// ============================================================================
//
// Quantum hardware syntax belongs here only insofar as it describes a
// hardware capability/resource boundary.
//
// It does NOT replace quantum::ir.
//
// It does NOT define QEC algorithms.
//
// It does NOT define ZQN noise semantics.
//
// It does NOT define routing/scheduling.
//

hardwareQuantumDeviceDecl
    : K_QUANTUM
      K_DEVICE
      IDENTIFIER
      hardwareQuantumDeviceBody?
    ;

hardwareQuantumDeviceBody
    : LBRACE
      hardwareQuantumDeviceItem*
      RBRACE
    ;

hardwareQuantumDeviceItem
    : K_CAPABILITY hardwareCapabilityReference SEMICOLON
    | K_RESOURCE hardwareQualifiedName SEMICOLON
    | K_QUBIT hardwareExpression SEMICOLON
    | K_CONNECTIVITY hardwareQualifiedName SEMICOLON
    | K_GATE hardwareQualifiedName SEMICOLON
    | K_MEASUREMENT hardwareQualifiedName SEMICOLON
    | K_RESET hardwareQualifiedName SEMICOLON
    | K_TIMING hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 36. GENERIC COMPUTE BODY
// ============================================================================

hardwareComputeBody
    : LBRACE
      hardwareComputeItem*
      RBRACE
    ;

hardwareComputeItem
    : K_CAPABILITY hardwareCapabilityReference SEMICOLON
    | K_RESOURCE hardwareQualifiedName SEMICOLON
    | K_MEMORY hardwareQualifiedName SEMICOLON
    | K_PIPELINE hardwareQualifiedName SEMICOLON
    | K_INTERFACE hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 37. PROGRAMMABLE HARDWARE BODY
// ============================================================================

hardwareProgrammableBody
    : LBRACE
      hardwareProgrammableItem*
      RBRACE
    ;

hardwareProgrammableItem
    : K_CAPABILITY hardwareCapabilityReference SEMICOLON
    | K_RESOURCE hardwareQualifiedName SEMICOLON
    | K_MEMORY hardwareQualifiedName SEMICOLON
    | K_INTERFACE hardwareQualifiedName SEMICOLON
    | K_CLOCK hardwareQualifiedName SEMICOLON
    | K_TIMING hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 38. HARDWARE INSTANCES
// ============================================================================
//
// Instantiation names a logical hardware component.
// Physical realization remains outside this grammar.
//

hardwareInstanceDecl
    : K_INSTANCE
      IDENTIFIER
      COLON
      hardwareQualifiedName
      hardwareGenericArguments?
      hardwareInstanceArguments?
      SEMICOLON
    ;

hardwareGenericArguments
    : LT
      hardwareExpression
      (COMMA hardwareExpression)*
      GT
    ;

hardwareInstanceArguments
    : LPAREN
      hardwareArgumentList?
      RPAREN
    ;


// ============================================================================
// 39. CONNECTIONS
// ============================================================================

hardwareConnectionDecl
    : K_CONNECT
      hardwareEndpoint
      (K_TO | ARROW)
      hardwareEndpoint
      hardwareConnectionProperties?
      SEMICOLON
    ;

hardwareEndpoint
    : hardwareQualifiedName
    | hardwareQualifiedName LBRACKET hardwareExpression RBRACKET
    | hardwareQualifiedName DOT IDENTIFIER
    ;

hardwareConnectionProperties
    : LBRACE
      hardwareConnectionProperty*
      RBRACE
    ;

hardwareConnectionProperty
    : K_WIDTH ASSIGN hardwareExpression SEMICOLON
    | K_LATENCY ASSIGN hardwareExpression SEMICOLON
    | K_BANDWIDTH ASSIGN hardwareExpression SEMICOLON
    | K_PROTOCOL ASSIGN hardwareQualifiedName SEMICOLON
    | K_CLOCK ASSIGN hardwareQualifiedName SEMICOLON
    | hardwareKeyValue SEMICOLON
    ;


// ============================================================================
// 40. GENERATION
// ============================================================================
//
// Generation is semantic metaprogramming for hardware structures.
// It must not encode a fixed machine size.
//

hardwareGenerateDecl
    : K_GENERATE
      IDENTIFIER?
      K_FOR
      IDENTIFIER
      K_IN
      hardwareExpression
      hardwareGenerateBody
    ;

hardwareGenerateBody
    : LBRACE
      hardwareItem*
      RBRACE
    ;


// ============================================================================
// 41. HARDWARE FUNCTIONS
// ============================================================================
//
// These are hardware-local behavioral helpers. They do not replace the
// universal function grammar.
//

hardwareFunctionDecl
    : K_FN
      IDENTIFIER
      LPAREN
      hardwareParameterList?
      RPAREN
      (ARROW hardwareType)?
      hardwareFunctionBody
    ;

hardwareFunctionBody
    : LBRACE
      hardwareLogicStatement*
      RBRACE
    ;

hardwareParameterList
    : hardwareParameter
      (COMMA hardwareParameter)*
    ;

hardwareParameter
    : IDENTIFIER
      COLON
      hardwareType
      (ASSIGN hardwareExpression)?
    ;


// ============================================================================
// 42. ASSERTIONS
// ============================================================================

hardwareAssertion
    : K_ASSERT
      LPAREN
      hardwareExpression
      RPAREN
      SEMICOLON
    ;


// ============================================================================
// 43. USING
// ============================================================================

hardwareUsingDecl
    : K_USING
      hardwareQualifiedName
      (K_AS IDENTIFIER)?
      SEMICOLON
    ;


// ============================================================================
// 44. TYPES
// ============================================================================
//
// Hardware types are structural/symbolic at grammar level.
//
// Width is separate from type identity so that semantic analysis can decide
// whether a width is compile-time, runtime, inferred, symbolic, or target
// dependent.
//

hardwareType
    : hardwareQualifiedName
    | hardwareTypeParameter
    | hardwareBitType
    | hardwareArrayType
    | hardwareVectorType
    | hardwareStreamType
    | hardwareInterfaceType
    ;

hardwareTypeParameter
    : IDENTIFIER
    ;

hardwareBitType
    : K_BIT
      hardwareWidthClause?
    ;

hardwareArrayType
    : LBRACKET
      hardwareExpression
      RBRACKET
      hardwareType
    ;

hardwareVectorType
    : K_VECTOR
      LT
      hardwareType
      COMMA
      hardwareExpression
      GT
    ;

hardwareStreamType
    : K_STREAM
      LT
      hardwareType
      GT
    ;

hardwareInterfaceType
    : K_INTERFACE
      LT
      hardwareQualifiedName
      GT
    ;


// ============================================================================
// 45. INITIALIZERS
// ============================================================================

hardwareInitializer
    : ASSIGN
      hardwareExpression
    ;


// ============================================================================
// 46. EXPRESSIONS
// ============================================================================
//
// The grammar intentionally accepts symbolic expressions rather than
// constraining values to host-language integer ranges.
//
// Semantic analysis owns:
//   - constant folding;
//   - dimensional validation;
//   - overflow checks;
//   - target representability;
//   - resource accounting.
//

hardwareExpression
    : hardwareConditionalExpression
    ;

hardwareConditionalExpression
    : hardwareLogicalOrExpression
      (QUESTION hardwareExpression COLON hardwareExpression)?
    ;

hardwareLogicalOrExpression
    : hardwareLogicalAndExpression
      (LOGICAL_OR hardwareLogicalAndExpression)*
    ;

hardwareLogicalAndExpression
    : hardwareBitwiseOrExpression
      (LOGICAL_AND hardwareBitwiseOrExpression)*
    ;

hardwareBitwiseOrExpression
    : hardwareBitwiseXorExpression
      (PIPE hardwareBitwiseXorExpression)*
    ;

hardwareBitwiseXorExpression
    : hardwareBitwiseAndExpression
      (CARET hardwareBitwiseAndExpression)*
    ;

hardwareBitwiseAndExpression
    : hardwareEqualityExpression
      (AMPERSAND hardwareEqualityExpression)*
    ;

hardwareEqualityExpression
    : hardwareRelationalExpression
      ((EQUAL_EQUAL | NOT_EQUAL) hardwareRelationalExpression)*
    ;

hardwareRelationalExpression
    : hardwareShiftExpression
      ((LT | GT | LESS_EQUAL | GREATER_EQUAL)
       hardwareShiftExpression)*
    ;

hardwareShiftExpression
    : hardwareAdditiveExpression
      ((LEFT_SHIFT | RIGHT_SHIFT)
       hardwareAdditiveExpression)*
    ;

hardwareAdditiveExpression
    : hardwareMultiplicativeExpression
      ((PLUS | MINUS)
       hardwareMultiplicativeExpression)*
    ;

hardwareMultiplicativeExpression
    : hardwareUnaryExpression
      ((STAR | SLASH | PERCENT)
       hardwareUnaryExpression)*
    ;

hardwareUnaryExpression
    : (PLUS | MINUS | EXCLAMATION | TILDE)
      hardwareUnaryExpression
    | hardwarePrimaryExpression
    ;

hardwarePrimaryExpression
    : hardwareLiteral
    | hardwareQualifiedName
    | hardwareCallExpression
    | hardwareIndexExpression
    | hardwareMemberExpression
    | LPAREN hardwareExpression RPAREN
    ;

hardwareCallExpression
    : hardwareQualifiedName
      LPAREN
      hardwareArgumentList?
      RPAREN
    ;

hardwareIndexExpression
    : hardwareQualifiedName
      LBRACKET
      hardwareExpression
      RBRACKET
    ;

hardwareMemberExpression
    : hardwareQualifiedName
      DOT
      IDENTIFIER
    ;


// ============================================================================
// 47. LITERALS
// ============================================================================

hardwareLiteral
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | TRUE
    | FALSE
    ;


// ============================================================================
// 48. ARGUMENTS
// ============================================================================

hardwareArgumentList
    : hardwareExpression
      (COMMA hardwareExpression)*
    ;


// ============================================================================
// 49. QUALIFIED NAMES
// ============================================================================
//
// Qualified names allow open-ended future hardware namespaces:
//
//   cpu.vector
//   gpu.tensor
//   vendor.family.feature
//   quantum.readout
//
// No vendor namespace is hard-coded.
//

hardwareQualifiedName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


// ============================================================================
// 50. KEY/VALUE EXTENSION POINT
// ============================================================================
//
// This provides controlled syntactic extensibility without modifying the
// core grammar for every new hardware property.
//

hardwareKeyValue
    : hardwareQualifiedName
      ASSIGN
      hardwareExpression
    ;


// ============================================================================
// 51. SEMANTIC INTEGRATION NOTES
// ============================================================================
//
// The following concepts are intentionally represented only syntactically:
//
// Resource
//   -> resources/ model
//
// Capability
//   -> capability analysis / hardware HAL
//
// Target
//   -> compiler target selection
//
// Placement
//   -> routing / placement subsystem
//
// Timing
//   -> scheduling subsystem
//
// Quantum device
//   -> hardware capability model + quantum::ir boundary
//
// Noise/fault information
//   -> ZQN
//
// Error correction
//   -> QEC
//
// Optimization
//   -> optimization subsystem
//
// Runtime selection
//   -> execution/runtime subsystem
//
// This prevents this grammar from becoming a second implementation of those
// systems.
//
// ============================================================================
// 52. SCALABILITY CONTRACT
// ============================================================================
//
// The following are deliberately NOT grammar constants:
//
//   MAX_DEVICES
//   MAX_CORES
//   MAX_THREADS
//   MAX_MEMORY
//   MAX_LANES
//   MAX_PORTS
//   MAX_SIGNALS
//   MAX_QUBITS
//   MAX_QUBITS_PER_DEVICE
//   MAX_NODES
//   MAX_PIPELINE_STAGES
//   MAX_MEMORY_BANKS
//   MAX_ACCELERATORS
//
// Any limits required for a particular compilation or execution environment
// must be supplied through the appropriate resource/capability/limits layer.
//
// ============================================================================
// 53. SEMANTIC OWNERSHIP CONTRACT
// ============================================================================
//
// Hardware grammar
//     = syntax
//
// Hardware semantic model
//     = meaning
//
// Hardware capability model
//     = what target can provide
//
// Hardware resource model
//     = what target can allocate
//
// Hardware HAL
//     = how the target is controlled
//
// Routing
//     = where operations/resources are placed
//
// Scheduling
//     = when operations/resources execute
//
// Optimization
//     = how implementation is improved
//
// Runtime
//     = how execution is dispatched
//
// Quantum IR
//     = canonical quantum semantics
//
// QEC
//     = quantum error detection/correction
//
// ZQN
//     = quantum noise/fault semantics
//
// ============================================================================
// 54. DETERMINISM CONTRACT
// ============================================================================
//
// This grammar contains:
//   - no semantic predicates;
//   - no target-language actions;
//   - no filesystem access;
//   - no network access;
//   - no runtime discovery;
//   - no randomness.
//
// Therefore parsing is deterministic for a fixed token stream and grammar
// version.
//
// ============================================================================
// 55. SECURITY CONTRACT
// ============================================================================
//
// This grammar never executes hardware operations.
//
// A source file can describe:
//
//   devices
//   resources
//   capabilities
//   connections
//   targets
//
// without granting authority to access them.
//
// Authorization, capability possession, sandboxing, credentials, and runtime
// policy belong to later layers.
//
// ============================================================================
// 56. COMPATIBILITY CONTRACT
// ============================================================================
//
// Adding a new hardware property should preferably use:
//
//   hardwareKeyValue
//
// or a new qualified capability/property rather than forcing a breaking
// change to an existing declaration.
//
// Removing or changing a keyword requires language-version compatibility
// handling in:
//
//   grammar/specification/
//   grammar/compatibility/
//   grammar/tests/compatibility/
//
// ============================================================================
// 57. TEST CONTRACT
// ============================================================================
//
// Required positive tests:
//
//   hardware module
//   generic hardware
//   ports
//   interfaces
//   signals
//   wires
//   nets
//   clocks
//   timing
//   combinational logic
//   sequential logic
//   processes
//   state machines
//   memories
//   pipelines
//   resources
//   capabilities
//   requirements
//   constraints
//   preferences
//   placement
//   targets
//   CPU
//   GPU
//   FPGA
//   ASIC
//   quantum device
//   accelerators
//   instances
//   connections
//   generated hardware
//
// Required negative tests:
//
//   malformed ports
//   malformed widths
//   malformed dimensions
//   malformed timing
//   malformed constraints
//   malformed connections
//   malformed generic parameters
//   missing declarations
//   missing separators
//
// Required scalability tests:
//
//   symbolic width
//   symbolic memory depth
//   symbolic resource quantity
//   symbolic accelerator count
//   symbolic quantum capacity
//   generated hardware with large ranges
//
// Required cross-domain tests:
//
//   classical + hardware
//   quantum + hardware
//   HDL + hardware
//   quantum + HDL + hardware
//   distributed + hardware
//   AI + accelerator + hardware
//
// ============================================================================
// 58. COMPLETION CRITERIA
// ============================================================================
//
// This file is complete when:
//
//   [ ] ANTLR parser grammar generation succeeds.
//   [ ] ZamaniTokens supplies every referenced token.
//   [ ] No lexer rules are duplicated here.
//   [ ] No machine-size limits exist.
//   [ ] No physical device is hard-coded.
//   [ ] No hardware discovery occurs during parsing.
//   [ ] No runtime action occurs during parsing.
//   [ ] No unsafe Rust is introduced.
//   [ ] Hardware declarations have stable AST mapping.
//   [ ] Semantic analysis can distinguish requirement/capability/
//       constraint/preference/target/placement.
//   [ ] Hardware semantics remain independent of physical machine size.
//   [ ] Quantum hardware remains separate from quantum::ir.
//   [ ] QEC remains outside this grammar.
//   [ ] ZQN remains outside this grammar.
//   [ ] Routing remains outside this grammar.
//   [ ] Scheduling remains outside this grammar.
//   [ ] Optimization remains outside this grammar.
//   [ ] Runtime dispatch remains outside this grammar.
//   [ ] Positive tests pass.
//   [ ] Negative tests pass.
//   [ ] Boundary tests pass.
//   [ ] Cross-domain tests pass.
//   [ ] Compatibility tests pass.
//   [ ] Determinism tests pass.
//   [ ] Repository-wide grammar generation passes.
//
// ============================================================================