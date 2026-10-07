/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/hardware.g4
 *
 * GRAMMAR
 * -------
 * Hardware
 *
 * STATUS
 * ------
 * CANONICAL HARDWARE-DOMAIN COMPOSITION ROOT
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION ROOT for grammar/hardware/.
 *
 * It is intentionally an ORCHESTRATOR, not a second implementation of the
 * hardware language.
 *
 * Its responsibilities are limited to:
 *
 *   1. composing the independently owned hardware parser grammars;
 *   2. exposing one stable Hardware grammar boundary to ZamaniParser;
 *   3. dispatching hardware declarations to their owning leaf grammar;
 *   4. dispatching hardware statements where a hardware leaf owns them;
 *   5. providing the hardware-domain expression boundary;
 *   6. providing stable cross-domain composition points;
 *   7. ensuring that new hardware technologies can be added without changing
 *      the universal language's machine-capacity model.
 *
 * It MUST NOT become a monolithic hardware grammar.
 *
 * ============================================================================
 * LANGUAGE / IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar
 * Rust 2021
 * Rust 1.97 or later
 * Safe Rust only
 * No unsafe Rust
 *
 * This grammar contains no:
 *
 *   - embedded Rust actions;
 *   - semantic predicates;
 *   - filesystem access;
 *   - network access;
 *   - environment inspection;
 *   - hardware discovery;
 *   - allocation;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - calibration execution;
 *   - runtime execution;
 *   - target selection;
 *   - physical-device access.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The hardware grammar participates in the canonical frontend as follows:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Hardware
 *          |
 *          +--------------------------------------------------+
 *          |                                                  |
 *          v                                                  v
 *     hardware leaf grammars                         canonical expressions
 *          |                                                  |
 *          +-------------------------+------------------------+
 *                                    |
 *                                    v
 *                           domain-neutral AST
 *                                    |
 *                                    v
 *                           structural validation
 *                                    |
 *                                    v
 *                            semantic analysis
 *                                    |
 *                  +-----------------+------------------+
 *                  |                 |                  |
 *                  v                 v                  v
 *              resources        capabilities        constraints
 *                  |                 |                  |
 *                  +-----------------+------------------+
 *                                    |
 *                                    v
 *                         target-independent intent
 *                                    |
 *                                    v
 *                         canonical semantic model
 *                                    |
 *                  +-----------------+------------------+
 *                  |                 |                  |
 *                  v                 v                  v
 *              classical        quantum::ir          HDL
 *                  |                 |                  |
 *                  +-----------------+------------------+
 *                                    |
 *                                    v
 *                              optimization
 *                                    |
 *                       +------------+------------+
 *                       |            |            |
 *                       v            v            v
 *                    routing     scheduling   resilience
 *                                                 |
 *                                                 v
 *                                                ZQN
 *                                                 |
 *                                                 v
 *                                                HAL
 *                                                 |
 *                                                 v
 *                                         target realization
 *
 * ============================================================================
 * COMPOSITION PRINCIPLE
 * ============================================================================
 *
 * hardware.g4 owns COMPOSITION.
 *
 * Leaf grammars own their feature syntax.
 *
 * Therefore:
 *
 *     hardware.g4
 *         |
 *         +--> resources.g4
 *         +--> capabilities.g4
 *         +--> constraints.g4
 *         +--> targets.g4
 *         +--> devices.g4
 *         +--> topology.g4
 *         +--> placement.g4
 *         +--> accelerators.g4
 *         +--> memory.g4
 *         +--> interconnect.g4
 *         +--> qpu.g4
 *         +--> cpu.g4
 *         +--> fpga.g4
 *         +--> asic.g4
 *         +--> compute.g4
 *         +--> calibration.g4
 *         +--> deployment.g4
 *         +--> performance.g4
 *         +--> power.g4
 *         +--> reliability.g4
 *         +--> thermal.g4
 *         +--> negotiation.g4
 *         +--> quantum-device.g4
 *
 * Each leaf remains independently testable.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * This file MUST NOT duplicate rules owned by a leaf grammar.
 *
 * In particular, this file does not define:
 *
 *     hardwareResourceDeclaration
 *     hardwareCapabilityDeclaration
 *     hardwareConstraintDeclaration
 *     hardwareTargetDeclaration
 *     deviceDeclaration
 *     topologyDeclaration
 *     placementDeclaration
 *     hardwareAcceleratorDeclaration
 *     hardwareMemoryContract
 *     hardwareInterconnectDeclaration
 *     qpuDeclaration
 *     cpuDeclaration
 *     hardwareFpgaDecl
 *     hardwareAsicDecl
 *     computeDeclaration
 *     hardwareCalibrationDeclaration
 *     hardwareDeploymentDeclaration
 *     hardwarePerformanceDeclaration
 *     hardwarePowerDeclaration
 *     hardwareReliabilityDeclaration
 *     hardwareThermalDeclaration
 *     hardwareNegotiationDeclaration
 *     hardwareQuantumDeviceDeclaration
 *
 * Those rules belong to their respective leaf grammars.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Hardware grammar files consume its vocabulary.
 *
 * This file MUST NOT introduce:
 *
 *     hardware-specific lexer rules
 *     duplicate keyword aliases
 *     duplicate punctuation
 *     duplicate literals
 *     vendor-specific lexical tokens
 *
 * Hardware technology names should remain semantic identifiers unless the
 * canonical lexical specification explicitly reserves a keyword.
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * Hardware grammar does NOT define another expression language.
 *
 * Expressions are owned by:
 *
 *     grammar/expressions/
 *
 * Consequently all:
 *
 *     quantities
 *     dimensions
 *     capacities
 *     timing values
 *     performance values
 *     power values
 *     thermal values
 *     reliability values
 *     predicates
 *     constraints
 *     capability arguments
 *     resource requirements
 *     placement predicates
 *     topology predicates
 *
 * ultimately consume the canonical expression model through the leaf grammar
 * that owns the corresponding construct.
 *
 * ============================================================================
 * TYPE AUTHORITY
 * ============================================================================
 *
 * Hardware grammar does not define a second type system.
 *
 * Types remain owned by:
 *
 *     grammar/types/
 *
 * Hardware-specific semantic interpretation occurs after parsing.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * RESOURCE
 *     A computational or physical resource that may be available, consumed,
 *     reserved, shared, or otherwise relevant to realization.
 *
 * CAPABILITY
 *     Something a realization can provide.
 *
 * REQUIREMENT
 *     Something required for semantic validity.
 *
 * CONSTRAINT
 *     A mandatory condition restricting legal realization.
 *
 * PREFERENCE
 *     Non-mandatory guidance for realization/optimization.
 *
 * HINT
 *     Advisory information.
 *
 * TARGET
 *     An abstract realization class or execution context.
 *
 * DEVICE
 *     A logical device or device class.
 *
 * TOPOLOGY
 *     A logical relationship model between computational entities.
 *
 * PLACEMENT
 *     Target-independent realization intent.
 *
 * NEGOTIATION
 *     A declarative description of acceptable realization alternatives.
 *
 * This composition root merely exposes these concepts through their owning
 * grammars.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Hardware syntax expresses PORTABLE INTENT.
 *
 * It must not require a source program to hard-code:
 *
 *     physical CPU IDs
 *     physical core IDs
 *     physical thread IDs
 *     physical GPU IDs
 *     physical FPGA coordinates
 *     physical ASIC identifiers
 *     physical QPU identifiers
 *     physical qubit IDs
 *     PCI addresses
 *     machine hostnames
 *     physical memory addresses
 *     serial numbers
 *     vendor-specific device instances
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This composition grammar imposes NO universal physical capacity.
 *
 * It MUST NOT define language-level constants such as:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_PORTS
 *     MAX_CONNECTIONS
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TOPOLOGY_SIZE
 *
 * The grammar itself also imposes no maximum number of declarations.
 *
 * Repetition remains expressed through ordinary ANTLR repetition constructs
 * in the owning leaf grammars.
 *
 * Quantities and dimensions remain semantic expressions.
 *
 * Therefore the same language architecture can describe:
 *
 *     microscopic computational substrates
 *     embedded systems
 *     single processors
 *     multicore systems
 *     many-processor systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     workstations
 *     servers
 *     HPC systems
 *     clusters
 *     distributed systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * Subject only to:
 *
 *     source semantics
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *     explicit program requirements
 *     explicit program constraints
 *
 * ============================================================================
 * OPEN-WORLD HARDWARE MODEL
 * ============================================================================
 *
 * Hardware categories are OPEN semantic categories.
 *
 * The universal grammar must not require an update merely because a new
 * hardware technology appears.
 *
 * Examples of semantic categories may include:
 *
 *     cpu
 *     gpu
 *     accelerator
 *     fpga
 *     asic
 *     quantum
 *     memory
 *     storage
 *     network
 *     controller
 *     optical
 *     neuromorphic
 *     molecular
 *     biological
 *     analog
 *     reconfigurable
 *     heterogeneous
 *
 * These are semantic categories, not a closed enumeration in this file.
 *
 * New technologies should normally be introduced through:
 *
 *     identifiers
 *     capabilities
 *     resources
 *     targets
 *     dialects
 *     semantic registrations
 *     leaf grammars where genuinely new syntax is required
 *
 * ============================================================================
 * ANTLR IMPORT AUTHORITY
 * ============================================================================
 *
 * Every grammar named in this import list MUST be a valid ANTLR4 parser
 * grammar.
 *
 * Import names are grammar names, not filesystem paths.
 *
 * The source tree path remains:
 *
 *     grammar/hardware/
 *
 * ============================================================================
 */

parser grammar Hardware;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,

    ZamaniHardwareResourcesParser,
    ZamaniHardwareCapabilitiesParser,
    ZamaniHardwareConstraintsParser,
    ZamaniHardwareTargetsParser,
    ZamaniHardwareDevicesParser,
    ZamaniHardwareTopologyParser,
    ZamaniHardwarePlacementParser,
    ZamaniHardwareAcceleratorParser,
    ZamaniHardwareMemoryParser,
    ZamaniHardwareInterconnectParser,

    ZamaniHardwareQpuParser,
    ZamaniHardwareCpuParser,
    HardwareFpga,
    HardwareAsic,

    ZamaniHardwareComputeParser,
    ZamaniHardwareCalibrationParser,
    HardwareDeployment,
    HardwarePerformance,
    ZamaniHardwarePowerParser,
    ZamaniHardwareReliabilityParser,
    ZamaniHardwareThermalParser,
    ZamaniHardwareNegotiationParser,
    ZamaniHardwareQuantumDeviceParser
;


/* ============================================================================
 * 1. CANONICAL HARDWARE DECLARATION DISPATCH
 * ============================================================================
 *
 * This is the principal public hardware declaration boundary.
 *
 * Each alternative is owned by a leaf grammar.
 *
 * The composition root does not reinterpret or transform the leaf rule.
 *
 * ========================================================================== */

hardwareDeclaration
    : hardwareResourceDeclaration
    | hardwareCapabilityDeclaration
    | hardwareConstraintDeclaration
    | hardwareTargetDeclaration
    | deviceDeclaration
    | topologyDeclaration
    | placementDeclaration
    | hardwareAcceleratorDeclaration
    | hardwareMemoryContract
    | hardwareInterconnectDeclaration

    | qpuDeclaration
    | cpuDeclaration
    | hardwareFpgaDecl
    | hardwareAsicDecl

    | computeDeclaration
    | hardwareCalibrationDeclaration
    | hardwareDeploymentDeclaration
    | hardwarePerformanceDeclaration
    | hardwarePowerDeclaration
    | hardwareReliabilityDeclaration
    | hardwareThermalDeclaration
    | hardwareNegotiationDeclaration
    | hardwareQuantumDeviceDeclaration
    ;


/* ============================================================================
 * 2. HARDWARE STATEMENT DISPATCH
 * ============================================================================
 *
 * Hardware statements remain intentionally small.
 *
 * Executable behavior belongs to the domain that owns that behavior.
 *
 * HDL behavior remains in grammar/hdl/.
 * Quantum behavior remains in grammar/quantum/.
 * Classical behavior remains in grammar/classical/.
 * Distributed behavior remains in grammar/distributed/.
 *
 * This composition root does not create a second executable hardware language.
 *
 * ========================================================================== */

hardwareStatement
    : hardwareAssertionStatement
    ;


/* ============================================================================
 * 3. UNIVERSAL HARDWARE ASSERTION
 * ============================================================================
 *
 * Assertions are properties of hardware intent.
 *
 * The assertion expression itself remains owned by the canonical expression
 * grammar.
 *
 * ========================================================================== */

hardwareAssertionStatement
    : ASSERT
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 4. HARDWARE EXPRESSION BOUNDARY
 * ============================================================================
 *
 * The hardware domain re-exports the canonical expression boundary without
 * changing expression precedence or semantics.
 *
 * ========================================================================== */

hardwareExpression
    : expression
    ;


/* ============================================================================
 * 5. HARDWARE DOMAIN DISPATCH
 * ============================================================================
 *
 * This is the stable public boundary consumed by the universal parser.
 *
 * The universal parser therefore knows only:
 *
 *     Hardware
 *
 * and does not need to know every file below grammar/hardware/.
 *
 * ========================================================================== */

hardwareDomain
    : hardwareDeclaration
    | hardwareStatement
    | hardwareExpression
    ;


/* ============================================================================
 * 6. HARDWARE CONTRACT
 * ============================================================================
 *
 * A hardware contract is an ordered sequence of hardware declarations.
 *
 * The repetition is intentionally unbounded at the grammar level.
 *
 * Physical feasibility is not determined by parsing.
 *
 * ========================================================================== */

hardwareContract
    : hardwareDeclaration+
    ;


/* ============================================================================
 * 7. HARDWARE DECLARATION LIST
 * ============================================================================
 *
 * This rule is a convenience boundary for visitors, validators and tooling.
 *
 * It does not impose a size limit.
 *
 * ========================================================================== */

hardwareDeclarationList
    : hardwareDeclaration*
    ;


/* ============================================================================
 * 8. HARDWARE STATEMENT LIST
 * ============================================================================
 */

hardwareStatementList
    : hardwareStatement*
    ;


/* ============================================================================
 * 9. HARDWARE ELEMENT
 * ============================================================================
 *
 * A hardware element is any construct exposed by the hardware-domain
 * composition root.
 *
 * ========================================================================== */

hardwareElement
    : hardwareDeclaration
    | hardwareStatement
    ;


/* ============================================================================
 * 10. HARDWARE CONTRACT ELEMENT
 * ============================================================================
 *
 * This rule intentionally excludes arbitrary expressions.
 *
 * Expressions are embedded through the owning leaf grammar and through the
 * hardwareExpression boundary when a consumer explicitly requires an
 * expression.
 *
 * ========================================================================== */

hardwareContractElement
    : hardwareDeclaration
    | hardwareStatement
    ;


/* ============================================================================
 * 11. HARDWARE REQUIREMENT BOUNDARY
 * ============================================================================
 *
 * Requirements themselves remain owned by the appropriate leaf grammar.
 *
 * This rule exists only as an integration boundary for consumers that need
 * to accept a canonical expression in a hardware requirement context.
 *
 * It does not redefine the syntax of `requires`.
 *
 * ========================================================================== */

hardwareRequirementExpression
    : expression
    ;


/* ============================================================================
 * 12. HARDWARE CONSTRAINT EXPRESSION
 * ============================================================================
 */

hardwareConstraintExpression
    : expression
    ;


/* ============================================================================
 * 13. HARDWARE PREFERENCE EXPRESSION
 * ============================================================================
 */

hardwarePreferenceExpression
    : expression
    ;


/* ============================================================================
 * 14. HARDWARE HINT EXPRESSION
 * ============================================================================
 */

hardwareHintExpression
    : expression
    ;


/* ============================================================================
 * 15. HARDWARE VALUE
 * ============================================================================
 *
 * A hardware value is simply a canonical Zamani expression.
 *
 * This rule provides a semantic naming boundary without creating a second
 * expression language.
 *
 * ========================================================================== */

hardwareValue
    : expression
    ;


/* ============================================================================
 * 16. HARDWARE ATTRIBUTE VALUE
 * ============================================================================
 */

hardwareAttributeValue
    : expression
    ;


/* ============================================================================
 * 17. HARDWARE DIMENSION
 * ============================================================================
 *
 * Dimensions are expressions.
 *
 * No rank or dimensionality ceiling is encoded here.
 *
 * ========================================================================== */

hardwareDimension
    : expression
    ;


/* ============================================================================
 * 18. HARDWARE DIMENSION LIST
 * ============================================================================
 *
 * The number of dimensions is not limited by the language architecture.
 *
 * ========================================================================== */

hardwareDimensionList
    : LBRACKET
      expression
      RBRACKET
      (
          LBRACKET
          expression
          RBRACKET
      )*
    ;


/* ============================================================================
 * 19. HARDWARE EXPRESSION LIST
 * ============================================================================
 *
 * The list is unbounded at the grammar level.
 *
 * ========================================================================== */

hardwareExpressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 20. HARDWARE NAME BOUNDARY
 * ============================================================================
 *
 * Hardware names must remain ordinary Zamani identifiers.
 *
 * Physical identity is resolved downstream.
 *
 * ========================================================================== */

hardwareNameReference
    : IDENTIFIER
    ;


/* ============================================================================
 * 21. HARDWARE QUALIFIED REFERENCE
 * ============================================================================
 *
 * The canonical expression/member-access system remains responsible for
 * qualified semantic references.
 *
 * This rule deliberately does not create a second qualified-name grammar.
 *
 * ========================================================================== */

hardwareQualifiedReference
    : expression
    ;


/* ============================================================================
 * 22. CROSS-DOMAIN INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Hardware intent can accompany other Zamani domains.
 *
 * Examples:
 *
 *     classical computation + hardware requirements
 *     quantum computation + QPU requirements
 *     hybrid computation + accelerator requirements
 *     HDL + hardware implementation intent
 *     AI computation + tensor accelerator requirements
 *     distributed computation + topology requirements
 *     networking + interconnect requirements
 *
 * Hardware does not own those domains.
 *
 * It provides the realization-contract boundary they consume.
 *
 * ========================================================================== */

hardwareCrossDomainElement
    : hardwareDeclaration
    | hardwareStatement
    ;


/* ============================================================================
 * 23. QUANTUM HARDWARE BOUNDARY
 * ============================================================================
 *
 * Quantum hardware descriptions enter through the dedicated hardware leaf
 * grammars:
 *
 *     qpu.g4
 *     quantum-device.g4
 *
 * Quantum PROGRAM semantics do not belong here.
 *
 * Quantum computation follows:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     hardware capability/resource analysis
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * hardware.g4 does not create or modify quantum::ir.
 *
 * ========================================================================== */

hardwareQuantumDeclaration
    : qpuDeclaration
    | hardwareQuantumDeviceDeclaration
    ;


/* ============================================================================
 * 24. CLASSICAL HARDWARE BOUNDARY
 * ============================================================================
 *
 * CPU and generic compute declarations describe target-independent hardware
 * intent.
 *
 * They do not determine actual CPU/core/thread allocation.
 *
 * ========================================================================== */

hardwareClassicalDeclaration
    : cpuDeclaration
    | computeDeclaration
    ;


/* ============================================================================
 * 25. RECONFIGURABLE / FIXED HARDWARE BOUNDARY
 * ============================================================================
 */

hardwareImplementationDeclaration
    : hardwareFpgaDecl
    | hardwareAsicDecl
    | hardwareAcceleratorDeclaration
    ;


/* ============================================================================
 * 26. SYSTEM INFRASTRUCTURE BOUNDARY
 * ============================================================================
 *
 * These constructs collectively describe the non-compute aspects of a target:
 *
 *     resources
 *     memory
 *     interconnect
 *     topology
 *     placement
 *     power
 *     thermal
 *     timing
 *     reliability
 *     performance
 *
 * Actual realization remains downstream.
 *
 * ========================================================================== */

hardwareInfrastructureDeclaration
    : hardwareResourceDeclaration
    | hardwareMemoryContract
    | hardwareInterconnectDeclaration
    | topologyDeclaration
    | placementDeclaration
    | hardwarePowerDeclaration
    | hardwareThermalDeclaration
    | hardwareReliabilityDeclaration
    | hardwarePerformanceDeclaration
    ;


/* ============================================================================
 * 27. TARGET / REALIZATION BOUNDARY
 * ============================================================================
 *
 * Targets and negotiation remain declarations of intent.
 *
 * They do not perform target discovery or allocation.
 *
 * ========================================================================== */

hardwareRealizationDeclaration
    : hardwareTargetDeclaration
    | hardwareNegotiationDeclaration
    | hardwareDeploymentDeclaration
    ;


/* ============================================================================
 * 28. QUALITY / VALIDATION BOUNDARY
 * ============================================================================
 *
 * These declarations express conditions or evidence about hardware
 * realization. They remain declarative.
 *
 * ========================================================================== */

hardwareQualityDeclaration
    : hardwareConstraintDeclaration
    | hardwareCalibrationDeclaration
    | hardwareReliabilityDeclaration
    | hardwarePerformanceDeclaration
    | hardwarePowerDeclaration
    | hardwareThermalDeclaration
    ;


/* ============================================================================
 * 29. HARDWARE DOMAIN CONTRACT
 * ============================================================================
 *
 * This is the strongest aggregate boundary exposed by the hardware grammar.
 *
 * It deliberately accepts only constructs that are genuinely hardware-domain
 * declarations/statements.
 *
 * It does not accept arbitrary source elements and therefore cannot become a
 * competing program root.
 *
 * ========================================================================== */

hardwareProgram
    : hardwareElement*
    ;


/* ============================================================================
 * 30. AST CONTRACT
 * ============================================================================
 *
 * hardware.g4 does not define AST structures.
 *
 * Every leaf declaration must enter the repository's domain-neutral frontend
 * AST.
 *
 * The AST must preserve, where applicable:
 *
 *     declaration kind
 *     source span
 *     name
 *     qualified name
 *     parameters
 *     attributes
 *     requirements
 *     resources
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     target intent
 *     topology intent
 *     placement intent
 *     provenance metadata
 *
 * Physical hardware identity must NOT be required for source portability.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for:
 *
 *     name resolution
 *     type checking
 *     requirement validation
 *     capability validation
 *     resource validation
 *     constraint validation
 *     target compatibility
 *     topology validation
 *     placement validation
 *     negotiation analysis
 *     portability analysis
 *     conflict detection
 *     policy validation
 *     effect validation
 *     provenance handling
 *
 * Parsing must never determine whether a requested resource physically exists.
 *
 * Example:
 *
 *     requires qubits >= required_qubits;
 *
 * may be syntactically valid even when the selected execution context cannot
 * satisfy the requirement.
 *
 * That is a resource/capability semantic result, not a parser error.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * hardware.g4 introduces no runtime effects.
 *
 * Hardware-related effects such as:
 *
 *     hardware access
 *     native access
 *     device I/O
 *     measurement
 *     networking
 *     foreign calls
 *
 * are semantic properties handled downstream.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are declarative.
 *
 * A hardware declaration may describe capabilities such as:
 *
 *     compute
 *     tensor.compute
 *     quantum.measurement
 *     quantum.dynamic_circuit
 *     memory
 *     interconnect
 *     acceleration
 *     reconfiguration
 *     secure.execution
 *     simulation
 *
 * without hard-coding a closed list in this composition root.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Hardware resource quantities are symbolic expressions.
 *
 * This supports:
 *
 *     requires memory >= required_memory;
 *     requires qubits >= required_qubits;
 *     requires capability("tensor.compute");
 *     requires topology(required_topology);
 *
 * without establishing a universal physical ceiling.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Hardware grammar does not implement policy.
 *
 * Policy is evaluated by the appropriate semantic/security/resource/execution
 * layers.
 *
 * A hardware declaration may therefore be affected by:
 *
 *     resource policies
 *     execution policies
 *     security policies
 *     deployment policies
 *     adaptation policies
 *     fallback policies
 *
 * without embedding those implementations in ANTLR.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Hardware declarations must remain traceable to their source spans.
 *
 * Downstream provenance may record:
 *
 *     source declaration
 *     semantic interpretation
 *     resource decision
 *     capability decision
 *     target realization
 *     optimization decision
 *     routing decision
 *     scheduling decision
 *     resilience decision
 *
 * hardware.g4 itself records none of these runtime decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * hardware.g4 defines NO independent hardware IR.
 *
 * It MUST NOT introduce:
 *
 *     HardwareIR
 *     HardwareResourceIR
 *     HardwareCapabilityIR
 *     HardwareDeviceIR
 *     HardwareTopologyIR
 *     HardwareTargetIR
 *     HardwareQuantumIR
 *
 * Hardware syntax becomes part of the repository's canonical semantic model.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * HDL behavior continues through:
 *
 *     grammar/hdl/
 *
 * Classical computation continues through:
 *
 *     grammar/classical/
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * The hardware grammar never chooses:
 *
 *     LLVM
 *     QIR
 *     SPIR-V
 *     CUDA
 *     OpenCL
 *     Verilog
 *     VHDL
 *     SystemVerilog
 *     vendor-specific FPGA flows
 *     vendor-specific ASIC flows
 *     vendor-specific QPU flows
 *
 * Those are downstream lowering/backend concerns.
 *
 * ============================================================================
 * ROUTING CONTRACT
 * ============================================================================
 *
 * Topology and placement are SOURCE-LEVEL INTENT.
 *
 * Physical routing is downstream.
 *
 * hardware.g4 therefore does not:
 *
 *     route
 *     allocate
 *     map physical qubits
 *     assign physical cores
 *     assign physical GPU units
 *     assign FPGA regions
 *     assign ASIC cells
 *     select network paths
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * Timing requirements and performance requirements are declarative.
 *
 * Actual scheduling is downstream.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Reliability declarations describe intent.
 *
 * Runtime resilience remains downstream.
 *
 * This includes integration with the repository's resilience states and
 * execution outcomes where applicable.
 *
 * ============================================================================
 * CALIBRATION CONTRACT
 * ============================================================================
 *
 * Calibration grammar expresses calibration requirements/contracts.
 *
 * It does not:
 *
 *     discover calibration
 *     execute calibration
 *     select physical calibration records
 *     mutate hardware
 *
 * ============================================================================
 * DEPLOYMENT CONTRACT
 * ============================================================================
 *
 * hardware/deployment.g4 describes hardware-specific deployment intent.
 *
 * General deployment semantics remain owned by:
 *
 *     grammar/execution/
 *
 * No deployment occurs during parsing.
 *
 * ============================================================================
 * LEGACY CONSTRAINT FILE
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/hardware/constraints.g4
 *     grammar/hardware/hardware-constraints.g4
 *
 * `constraints.g4` is the canonical owner.
 *
 * `hardware-constraints.g4` MUST NOT become a competing implementation.
 *
 * It must be treated as:
 *
 *     compatibility facade
 *     migration/deprecation surface
 *
 * until all repository references are migrated.
 *
 * hardware.g4 therefore imports ONLY:
 *
 *     ZamaniHardwareConstraintsParser
 *
 * from constraints.g4.
 *
 * ============================================================================
 * GPU INTEGRATION
 * ============================================================================
 *
 * IMPORTANT REPOSITORY INTEGRATION REQUIREMENT:
 *
 * The current:
 *
 *     grammar/hardware/gpu.g4
 *
 * is not presently declared as an ANTLR parser grammar.
 *
 * It therefore cannot legally be imported by this parser grammar in its
 * current state.
 *
 * hardware.g4 MUST NOT fake an import or duplicate the GPU grammar.
 *
 * Required repository correction:
 *
 *     gpu.g4
 *         ->
 *     valid ANTLR4 parser grammar
 *
 * with a stable grammar name and a canonical exported GPU declaration rule.
 *
 * Once corrected, the GPU parser grammar belongs in this composition root's
 * import set and GPU declaration dispatch.
 *
 * Until then, GPU-specific syntax must not be duplicated here.
 *
 * This is an integration prerequisite, not a reason to make hardware.g4
 * monolithic.
 *
 * ============================================================================
 * TIMING INTEGRATION
 * ============================================================================
 *
 * The current:
 *
 *     grammar/hardware/timing.g4
 *
 * also lacks a parser-grammar declaration.
 *
 * Therefore it cannot legally be imported as a parser grammar yet.
 *
 * Required repository correction:
 *
 *     timing.g4
 *         ->
 *     valid ANTLR4 parser grammar
 *
 * with a stable grammar name and canonical exported timing declaration rule.
 *
 * hardware.g4 must then compose it without duplicating timing rules.
 *
 * ============================================================================
 * ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * grammar/antlr/ZamaniParser.g4 already imports:
 *
 *     Hardware
 *
 * This is the correct architecture.
 *
 * ZamaniParser must continue to see only the Hardware composition boundary
 * rather than every hardware leaf grammar.
 *
 * Conceptually:
 *
 *     sourceElement
 *          |
 *          +--> domainElement
 *                    |
 *                    +--> hardwareElement
 *                              |
 *                              +--> Hardware
 *                                       |
 *                                       +--> hardwareDeclaration
 *
 * No direct leaf imports should be added to ZamaniParser merely to support a
 * new hardware technology.
 *
 * ============================================================================
 * BUILD INTEGRATION
 * ============================================================================
 *
 * The ANTLR build configuration must place the entire hardware grammar
 * directory on the grammar source path.
 *
 * The generation order is logically:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     hardware leaf parser grammars
 *          |
 *          v
 *     Hardware
 *          |
 *          v
 *     ZamaniParser
 *
 * No grammar action may depend on Rust implementation details.
 *
 * ============================================================================
 * TEST INTEGRATION
 * ============================================================================
 *
 * The hardware test suite must exercise this composition root through:
 *
 *     grammar/tests/
 *
 * and the hardware-specific test hierarchy.
 *
 * Required categories:
 *
 *     lexical
 *     parser
 *     AST
 *     semantic
 *     resource
 *     capability
 *     target
 *     topology
 *     placement
 *     quantum
 *     classical
 *     accelerator
 *     HDL/co-design
 *     distributed
 *     portability
 *     scalability
 *     compatibility
 *     negative
 *     deterministic parsing
 *
 * ============================================================================
 * REQUIRED POSITIVE COVERAGE
 * ============================================================================
 *
 * At minimum, the repository must parse hardware programs covering:
 *
 *     generic compute
 *     CPU
 *     GPU
 *     accelerator
 *     FPGA
 *     ASIC
 *     QPU
 *     quantum device
 *     memory
 *     interconnect
 *     topology
 *     placement
 *     resource requirements
 *     capabilities
 *     constraints
 *     target intent
 *     negotiation
 *     deployment intent
 *     performance
 *     power
 *     thermal
 *     reliability
 *     calibration
 *
 * ============================================================================
 * REQUIRED NEGATIVE COVERAGE
 * ============================================================================
 *
 * Tests must reject or semantically diagnose:
 *
 *     malformed hardware declarations
 *     malformed resource declarations
 *     malformed capability declarations
 *     malformed constraints
 *     invalid target syntax
 *     invalid topology syntax
 *     invalid placement syntax
 *     malformed specialized declarations
 *     malformed contracts
 *
 * Physical infeasibility must NOT automatically be represented as a parser
 * error.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests must verify that the grammar architecture does not introduce limits
 * based on:
 *
 *     number of devices
 *     number of processors
 *     number of accelerators
 *     number of QPUs
 *     number of qubits
 *     number of nodes
 *     amount of memory
 *     number of topology participants
 *     number of dimensions
 *     number of declarations
 *
 * The tests should use generated/parameterized inputs rather than hard-coded
 * language ceilings.
 *
 * ============================================================================
 * DETERMINISM TEST
 * ============================================================================
 *
 * For a fixed token stream:
 *
 *     parse(source)
 *
 * must have deterministic syntactic interpretation.
 *
 * No hardware discovery or runtime state may influence parsing.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Normative hardware semantics are defined by the appropriate specification
 * files.
 *
 * This grammar must remain consistent with:
 *
 *     grammar/spec/hardware.md
 *     grammar/spec/resources.md
 *     grammar/spec/portability.md
 *     grammar/compatibility/
 *     grammar/grammar.md
 *     grammar/Zamani-Grammar.md
 *
 * Historical or proposed syntax in Zamani-Grammar.md does not automatically
 * become legal syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * hardware.g4 is DONE when:
 *
 *   [ ] it is the only hardware composition root;
 *   [ ] all valid hardware leaf parser grammars are composed here;
 *   [ ] ZamaniParser imports Hardware rather than individual hardware leaves;
 *   [ ] no leaf syntax is duplicated here;
 *   [ ] no second expression language exists here;
 *   [ ] no second type system exists here;
 *   [ ] no hardware IR is defined here;
 *   [ ] quantum::ir remains the canonical quantum IR boundary;
 *   [ ] no routing exists here;
 *   [ ] no scheduling exists here;
 *   [ ] no physical allocation exists here;
 *   [ ] no physical device discovery exists here;
 *   [ ] no universal capacity constants exist here;
 *   [ ] no vendor-specific physical assumptions exist here;
 *   [ ] GPU integration is completed through a valid parser grammar;
 *   [ ] timing integration is completed through a valid parser grammar;
 *   [ ] legacy hardware-constraints ownership is resolved;
 *   [ ] all imported rule names compile;
 *   [ ] all hardware parser tests pass;
 *   [ ] cross-domain tests pass;
 *   [ ] scalability tests pass;
 *   [ ] deterministic parsing tests pass;
 *   [ ] generated Rust remains compatible with Rust 1.97+;
 *   [ ] no unsafe Rust is required.
 *
 * ============================================================================
 */

parser grammar Hardware;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,

    ZamaniHardwareResourcesParser,
    ZamaniHardwareCapabilitiesParser,
    ZamaniHardwareConstraintsParser,
    ZamaniHardwareTargetsParser,
    ZamaniHardwareDevicesParser,
    ZamaniHardwareTopologyParser,
    ZamaniHardwarePlacementParser,
    ZamaniHardwareAcceleratorParser,
    ZamaniHardwareMemoryParser,
    ZamaniHardwareInterconnectParser,

    ZamaniHardwareQpuParser,
    ZamaniHardwareCpuParser,
    HardwareFpga,
    HardwareAsic,

    ZamaniHardwareComputeParser,
    ZamaniHardwareCalibrationParser,
    HardwareDeployment,
    HardwarePerformance,
    ZamaniHardwarePowerParser,
    ZamaniHardwareReliabilityParser,
    ZamaniHardwareThermalParser,
    ZamaniHardwareNegotiationParser,
    ZamaniHardwareQuantumDeviceParser
;


/* ============================================================================
 * PUBLIC HARDWARE DECLARATION DISPATCH
 * ========================================================================== */

hardwareDeclaration
    : hardwareResourceDeclaration
    | hardwareCapabilityDeclaration
    | hardwareConstraintDeclaration
    | hardwareTargetDeclaration
    | deviceDeclaration
    | topologyDeclaration
    | placementDeclaration
    | hardwareAcceleratorDeclaration
    | hardwareMemoryContract
    | hardwareInterconnectDeclaration
    | qpuDeclaration
    | cpuDeclaration
    | hardwareFpgaDecl
    | hardwareAsicDecl
    | computeDeclaration
    | hardwareCalibrationDeclaration
    | hardwareDeploymentDeclaration
    | hardwarePerformanceDeclaration
    | hardwarePowerDeclaration
    | hardwareReliabilityDeclaration
    | hardwareThermalDeclaration
    | hardwareNegotiationDeclaration
    | hardwareQuantumDeviceDeclaration
    ;


/* ============================================================================
 * PUBLIC HARDWARE STATEMENT DISPATCH
 * ========================================================================== */

hardwareStatement
    : hardwareAssertionStatement
    ;

hardwareAssertionStatement
    : ASSERT
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * PUBLIC HARDWARE EXPRESSION BOUNDARY
 * ========================================================================== */

hardwareExpression
    : expression
    ;


/* ============================================================================
 * PUBLIC HARDWARE DOMAIN BOUNDARY
 *
 * This is the only hardware entry point that the universal parser needs.
 * ========================================================================== */

hardwareDomain
    : hardwareDeclaration
    | hardwareStatement
    | hardwareExpression
    ;


/* ============================================================================
 * AGGREGATE CONTRACT BOUNDARIES
 * ========================================================================== */

hardwareContract
    : hardwareDeclaration+
    ;

hardwareDeclarationList
    : hardwareDeclaration*
    ;

hardwareStatementList
    : hardwareStatement*
    ;

hardwareElement
    : hardwareDeclaration
    | hardwareStatement
    ;

hardwareContractElement
    : hardwareDeclaration
    | hardwareStatement
    ;

hardwareProgram
    : hardwareElement*
    ;


/* ============================================================================
 * CANONICAL EXPRESSION BRIDGES
 *
 * These are aliases only. They do not create another expression language.
 * ========================================================================== */

hardwareRequirementExpression
    : expression
    ;

hardwareConstraintExpression
    : expression
    ;

hardwarePreferenceExpression
    : expression
    ;

hardwareHintExpression
    : expression
    ;

hardwareValue
    : expression
    ;

hardwareAttributeValue
    : expression
    ;

hardwareDimension
    : expression
    ;

hardwareExpressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;

hardwareDimensionList
    : LBRACKET
      expression
      RBRACKET
      (
          LBRACKET
          expression
          RBRACKET
      )*
    ;


/* ============================================================================
 * NAME / REFERENCE BRIDGES
 * ========================================================================== */

hardwareNameReference
    : IDENTIFIER
    ;

hardwareQualifiedReference
    : expression
    ;


/* ============================================================================
 * CROSS-DOMAIN HARDWARE BOUNDARIES
 * ========================================================================== */

hardwareCrossDomainElement
    : hardwareDeclaration
    | hardwareStatement
    ;

hardwareQuantumDeclaration
    : qpuDeclaration
    | hardwareQuantumDeviceDeclaration
    ;

hardwareClassicalDeclaration
    : cpuDeclaration
    | computeDeclaration
    ;

hardwareImplementationDeclaration
    : hardwareFpgaDecl
    | hardwareAsicDecl
    | hardwareAcceleratorDeclaration
    ;

hardwareInfrastructureDeclaration
    : hardwareResourceDeclaration
    | hardwareMemoryContract
    | hardwareInterconnectDeclaration
    | topologyDeclaration
    | placementDeclaration
    | hardwarePowerDeclaration
    | hardwareThermalDeclaration
    | hardwareReliabilityDeclaration
    | hardwarePerformanceDeclaration
    ;

hardwareRealizationDeclaration
    : hardwareTargetDeclaration
    | hardwareNegotiationDeclaration
    | hardwareDeploymentDeclaration
    ;

hardwareQualityDeclaration
    : hardwareConstraintDeclaration
    | hardwareCalibrationDeclaration
    | hardwareReliabilityDeclaration
    | hardwarePerformanceDeclaration
    | hardwarePowerDeclaration
    | hardwareThermalDeclaration
    ;