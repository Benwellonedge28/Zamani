/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/capabilities.g4
 *
 * GRAMMAR
 * -------
 * ZamaniHardwareCapabilitiesParser
 *
 * STATUS
 * ------
 * CANONICAL HARDWARE-CAPABILITY ADAPTER
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ANTLR
 * -----
 * ANTLR4 parser grammar
 * Action-free
 * Predicate-free
 * Runtime-independent
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the hardware-domain adapter for Zamani's canonical capability
 * model.
 *
 * It allows hardware declarations to express:
 *
 *     capability intent
 *     capability requirements
 *     capability constraints
 *     capability preferences
 *     capability hints
 *     capability properties
 *     capability extension
 *     capability composition
 *
 * The canonical capability identity and capability-expression syntax remain
 * owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * General expression syntax remains owned by:
 *
 *     grammar/expressions/
 *
 * Resource quantities and resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * Hardware target syntax remains owned by:
 *
 *     grammar/hardware/targets.g4
 *
 * Hardware resource syntax remains owned by:
 *
 *     grammar/hardware/resources.g4
 *
 * This file therefore does NOT create another capability language.
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * Hardware-domain composition of canonical capabilities:
 *
 *     hardwareCapabilityDeclaration
 *     hardwareCapabilityContractDeclaration
 *     hardwareCapabilityContract
 *     hardwareCapabilityContractMember
 *
 * Hardware capability intent:
 *
 *     hardwareCapabilityAssertion
 *     hardwareCapabilityRequirement
 *     hardwareCapabilityConstraint
 *     hardwareCapabilityPreference
 *     hardwareCapabilityHint
 *     hardwareCapabilityProperty
 *     hardwareCapabilityExtension
 *     hardwareCapabilityComposition
 *
 * Hardware-domain adapter rules:
 *
 *     hardwareTargetCapabilityClause
 *     hardwareDeviceCapabilityClause
 *     hardwareAcceleratorCapabilityClause
 *     hardwareComputeCapabilityClause
 *     hardwareHdlCapabilityClause
 *     hardwareQuantumCapabilityClause
 *     hardwareDistributedCapabilityClause
 *     hardwareAiCapabilityClause
 *     hardwareNetworkCapabilityClause
 *     hardwareSecurityCapabilityClause
 *
 *
 * DOES NOT OWN
 * -----------
 *
 * This file does NOT own:
 *
 *     lexical tokens
 *     identifiers
 *     qualified-name syntax
 *     capability identity
 *     capability version semantics
 *     general expressions
 *     general types
 *     resource quantities
 *     resource declarations
 *     resource requirements
 *     generic constraints
 *     policies
 *     effects
 *     targets
 *     devices
 *     topology
 *     placement
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     QEC
 *     ZQN
 *     quantum operations
 *     quantum gates
 *     quantum::ir
 *     hardware discovery
 *     physical device allocation
 *     backend selection
 *     runtime execution
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/core/capabilities.g4
 *     grammar/expressions/expressions.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 *
 * IMPORTED AUTHORITIES
 * --------------------
 *
 * Capabilities owns:
 *
 *     capabilityDeclaration
 *     capabilityReference
 *     capabilityName
 *     capabilityExpression
 *     capabilityReferenceList
 *     capabilityVersionClause
 *     capabilityAttributeList
 *
 * Expressions owns:
 *
 *     expression
 *
 *
 * EXPORTS
 * -------
 *
 * Primary:
 *
 *     hardwareCapabilityDeclaration
 *     hardwareCapabilityContractDeclaration
 *     hardwareCapabilityContract
 *
 * Hardware integration:
 *
 *     hardwareTargetCapabilityClause
 *     hardwareDeviceCapabilityClause
 *     hardwareAcceleratorCapabilityClause
 *     hardwareComputeCapabilityClause
 *     hardwareHdlCapabilityClause
 *     hardwareQuantumCapabilityClause
 *     hardwareDistributedCapabilityClause
 *     hardwareAiCapabilityClause
 *     hardwareNetworkCapabilityClause
 *     hardwareSecurityCapabilityClause
 *
 *
 * CONSUMED BY
 * -----------
 *
 *     grammar/hardware/hardware.g4
 *     grammar/hardware/targets.g4
 *     grammar/hardware/devices.g4
 *     grammar/hardware/accelerators.g4
 *     grammar/hardware/cpu.g4
 *     grammar/hardware/gpu.g4
 *     grammar/hardware/fpga.g4
 *     grammar/hardware/asic.g4
 *     grammar/hardware/qpu.g4
 *     grammar/hardware/memory.g4
 *     grammar/hardware/interconnect.g4
 *     grammar/hardware/topology.g4
 *     grammar/hardware/placement.g4
 *
 * Domain-specific hardware grammars may consume the adapter rules but must
 * not redefine capability identity or capability expressions.
 *
 *
 * AST_OWNER
 * ---------
 *
 * Existing domain-neutral frontend AST / capability AST.
 *
 * This grammar introduces no Rust AST structures.
 *
 * The AST must preserve:
 *
 *     capability identity
 *     capability version
 *     capability expression structure
 *     contract member ordering
 *     property names
 *     property expressions
 *     source spans
 *
 * It must not contain:
 *
 *     physical device handles
 *     physical device IDs
 *     PCI addresses
 *     driver handles
 *     runtime authorization tokens
 *     calibration state
 *     scheduler state
 *     routing state
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Hardware capability semantic analysis.
 *
 * Semantic analysis is responsible for:
 *
 *     capability resolution
 *     namespace resolution
 *     version compatibility
 *     property validation
 *     requirement satisfaction
 *     constraint validation
 *     preference evaluation
 *     capability inheritance/extension
 *     capability composition
 *     resource implications
 *     target compatibility
 *     portability
 *     policy interaction
 *     security/trust validation
 *
 * Parsing does none of these operations.
 *
 *
 * IR_OWNER
 * --------
 *
 * This file owns NO independent IR.
 *
 * Capability information becomes semantic metadata and/or constraints in the
 * canonical compiler representation.
 *
 * Quantum-related capability information may influence the semantic path that
 * eventually reaches:
 *
 *     quantum::ir
 *
 * This grammar never creates another quantum IR.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/hardware/capabilities/
 *     grammar/tests/hardware/
 *     grammar/tests/semantic/
 *     grammar/tests/resources/
 *     grammar/tests/quantum/
 *     grammar/tests/hdl/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/hardware.md
 *     grammar/spec/resources.md
 *     grammar/spec/portability.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * The correct dependency direction is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical capability grammar
 *       |
 *       v
 *     hardware capability adapter
 *       |
 *       v
 *     hardware composition
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic capability model
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *     resource analysis   target analysis
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *        compilation planning
 *                 |
 *       +---------+---------+
 *       |         |         |
 *       v         v         v
 *   optimize   routing   scheduling
 *                           |
 *                           v
 *                       resilience
 *                           |
 *                           v
 *                          ZQN
 *                           |
 *                           v
 *                          HAL
 *                           |
 *                           v
 *                        target
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Hardware capability syntax describes semantic requirements and properties,
 * not a particular machine.
 *
 * Valid capability identities may include:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     tensor::compute
 *     accelerator::matrix
 *     hardware::reconfigurable_logic
 *     hardware::parallel_compute
 *     hdl::synthesis
 *     distributed::collectives
 *     networking::high_bandwidth
 *     security::trusted_execution
 *     future::architecture::capability
 *
 * None of these names selects a physical implementation.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Capability names are intentionally open-ended.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU capabilities
 *     GPU capabilities
 *     FPGA capabilities
 *     ASIC capabilities
 *     QPU capabilities
 *     accelerator models
 *     vendor capabilities
 *     simulator capabilities
 *     future hardware classes
 *
 * A new capability normally requires semantic registry/specification work,
 * not a modification to this grammar.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO language-level hardware capacity ceilings here.
 *
 * NEVER introduce:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_PORTS
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_CAPABILITIES
 *     MAX_CAPABILITY_MEMBERS
 *
 * Collections use:
 *
 *     *
 *     +
 *
 * Numeric quantities remain expressions and are not interpreted as universal
 * implementation limits.
 *
 * Actual limits belong to:
 *
 *     parser resources
 *     compiler resources
 *     target resources
 *     runtime resources
 *     deployment policy
 *     physical hardware
 *
 *
 * ============================================================================
 * CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * CAPABILITY
 *     What an implementation can do.
 *
 * RESOURCE
 *     Something available, consumed, reserved, shared, or otherwise relevant
 *     to realization.
 *
 * REQUIREMENT
 *     A condition that must be satisfied.
 *
 * CONSTRAINT
 *     A mandatory realization restriction.
 *
 * PREFERENCE
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *     Advisory information.
 *
 * TARGET
 *     An abstract compilation/execution context.
 *
 * This grammar does not merge those concepts.
 *
 *
 * ============================================================================
 * DECLARATION MODEL
 * ============================================================================
 *
 * A hardware capability declaration may use the canonical capability
 * declaration:
 *
 *     capability hardware::parallel_compute;
 *
 * or a hardware capability contract:
 *
 *     capability hardware::parallel_compute {
 *         requires compute::parallel;
 *         prefer accelerator::matrix;
 *         property native = true;
 *     }
 *
 * The first form is the canonical capability declaration.
 *
 * The second form is a hardware-domain capability contract that associates
 * additional hardware realization intent with the canonical capability
 * identity.
 *
 * The semantic layer determines whether the declaration is legal for the
 * current declaration context.
 *
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareCapabilitiesParser;

options {
    tokenVocab = ZamaniLexer;
}

import Capabilities, Expressions;


/* ============================================================================
 * 1. PUBLIC HARDWARE CAPABILITY ENTRY POINT
 * ============================================================================
 *
 * Hardware.g4 consumes this rule.
 *
 * The rule intentionally keeps the canonical capability declaration available
 * while adding the hardware-specific contract form.
 *
 * ========================================================================== */

hardwareCapabilityDeclaration
    : capabilityDeclaration
    | hardwareCapabilityContractDeclaration
    ;


/* ============================================================================
 * 2. HARDWARE CAPABILITY CONTRACT DECLARATION
 * ============================================================================
 *
 * Example:
 *
 *     capability hardware::parallel_compute {
 *         requires compute::parallel;
 *         prefer accelerator::matrix;
 *         property native = true;
 *     }
 *
 * Capability identity remains canonical.
 *
 * The body is the hardware-specific extension.
 *
 * ========================================================================== */

hardwareCapabilityContractDeclaration
    : CAPABILITY
      capabilityName
      capabilityVersionClause?
      capabilityAttributeList?
      hardwareCapabilityContract
      SEMICOLON?
    ;


/* ============================================================================
 * 3. HARDWARE CAPABILITY CONTRACT
 * ============================================================================
 *
 * Contract cardinality is intentionally unbounded.
 *
 * Empty contracts are syntactically legal; semantic validation may reject
 * them if a particular declaration context requires at least one member.
 *
 * ========================================================================== */

hardwareCapabilityContract
    : LBRACE
      hardwareCapabilityContractMember*
      RBRACE
    ;


/* ============================================================================
 * 4. CONTRACT MEMBERS
 * ============================================================================
 *
 * Each member has one semantic category.
 *
 * No member performs hardware discovery.
 *
 * ========================================================================== */

hardwareCapabilityContractMember
    : hardwareCapabilityAssertion
    | hardwareCapabilityRequirement
    | hardwareCapabilityConstraint
    | hardwareCapabilityPreference
    | hardwareCapabilityHint
    | hardwareCapabilityProperty
    | hardwareCapabilityExtension
    | hardwareCapabilityComposition
    ;


/* ============================================================================
 * 5. CAPABILITY ASSERTION
 * ============================================================================
 *
 * Explicitly associates another capability expression with this hardware
 * capability contract.
 *
 * Example:
 *
 *     capability hardware::quantum_control {
 *         capability quantum::measurement;
 *     }
 *
 * This is declarative intent.
 *
 * It does not prove availability.
 *
 * ========================================================================== */

hardwareCapabilityAssertion
    : CAPABILITY
      capabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 6. REQUIREMENT
 * ============================================================================
 *
 * Hardware capability requirements use the canonical capability expression.
 *
 * Examples:
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::measurement
 *              and quantum::dynamic_control;
 *
 *     requires accelerator::tensor_compute;
 *
 * Resource quantities such as memory, qubits, nodes, or storage are NOT
 * parsed by this rule. Those belong to the resource requirement subsystem.
 *
 * ========================================================================== */

hardwareCapabilityRequirement
    : REQUIRES
      capabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 7. CONSTRAINT
 * ============================================================================
 *
 * A hardware capability constraint is a mandatory condition.
 *
 * The payload is a canonical expression so that future capability metadata
 * and resource-related predicates can be represented without creating a
 * second expression grammar.
 *
 * Examples:
 *
 *     constraint capability_available;
 *
 *     constraint hardware_mode == required_mode;
 *
 * The semantic layer determines whether the expression is a valid capability
 * constraint.
 *
 * ========================================================================== */

hardwareCapabilityConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 8. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory optimization intent.
 *
 * They MUST NOT become requirements merely because a target cannot satisfy
 * them.
 *
 * Examples:
 *
 *     prefer accelerator::tensor_compute;
 *
 *     prefer quantum::low_noise;
 *
 * ========================================================================== */

hardwareCapabilityPreference
    : PREFER
      capabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 9. HINT
 * ============================================================================
 *
 * Hints are weaker than preferences.
 *
 * The compiler may ignore them without making an otherwise valid program
 * invalid.
 *
 * ========================================================================== */

hardwareCapabilityHint
    : HINT
      (
          capabilityExpression
        | expression
      )
      SEMICOLON
    ;


/* ============================================================================
 * 10. PROPERTY
 * ============================================================================
 *
 * Hardware capability properties are open-world.
 *
 * The property name is an identifier rather than a closed keyword catalogue.
 *
 * Examples:
 *
 *     property native = true;
 *
 *     property emulated = false;
 *
 *     property precision = required_precision;
 *
 *     property execution_model = execution_model;
 *
 * Property semantics are resolved downstream.
 *
 * ========================================================================== */

hardwareCapabilityProperty
    : PROPERTY
      hardwareCapabilityPropertyPath
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. PROPERTY PATH
 * ============================================================================
 *
 * Property paths remain open-ended.
 *
 * Both:
 *
 *     property precision
 *
 * and:
 *
 *     property execution.precision
 *
 * can be represented without reserving additional keywords.
 *
 * ========================================================================== */

hardwareCapabilityPropertyPath
    : identifier
      (
          DOT
          identifier
      )*
    ;


/* ============================================================================
 * 12. EXTENSION
 * ============================================================================
 *
 * Extension is symbolic capability composition.
 *
 * It does not imply:
 *
 *     object inheritance
 *     implementation inheritance
 *     physical inheritance
 *     device inheritance
 *
 * Semantic analysis determines the relationship.
 *
 * Example:
 *
 *     extends accelerator::compute, tensor::compute;
 *
 * ========================================================================== */

hardwareCapabilityExtension
    : EXTENDS
      capabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 13. CAPABILITY COMPOSITION
 * ============================================================================
 *
 * This is an explicit structured grouping of capability intent.
 *
 * Example:
 *
 *     capability hardware::accelerated_compute {
 *         {
 *             accelerator::tensor_compute;
 *             quantum::measurement;
 *         }
 *     }
 *
 * The semantic model decides whether the composition represents conjunction,
 * grouping, profile membership, refinement, or another explicitly specified
 * relationship.
 *
 * No physical target is selected here.
 *
 * ========================================================================== */

hardwareCapabilityComposition
    : LBRACE
      hardwareCapabilityCompositionMember*
      RBRACE
    ;


hardwareCapabilityCompositionMember
    : capabilityExpression
      SEMICOLON
    | hardwareCapabilityProperty
    ;


/* ============================================================================
 * 14. CAPABILITY COLLECTION
 * ============================================================================
 *
 * A collection is an unbounded source-level list of canonical capability
 * references.
 *
 * It does not allocate resources.
 *
 * ========================================================================== */

hardwareCapabilityCollection
    : LBRACKET
      hardwareCapabilityReferenceList?
      RBRACKET
    ;


hardwareCapabilityReferenceList
    : capabilityReference
      (
          COMMA
          capabilityReference
      )*
    ;


/* ============================================================================
 * 15. HARDWARE TARGET CAPABILITY ADAPTER
 * ============================================================================
 *
 * Targets remain owned by targets.g4.
 *
 * This rule exposes only capability-related target intent.
 *
 * ========================================================================== */

hardwareTargetCapabilityClause
    : REQUIRES
      capabilityExpression
      SEMICOLON
    | PREFER
      capabilityExpression
      SEMICOLON
    | HINT
      (
          capabilityExpression
        | expression
      )
      SEMICOLON
    ;


/* ============================================================================
 * 16. HARDWARE DEVICE CAPABILITY ADAPTER
 * ============================================================================
 *
 * devices.g4 owns device declaration syntax.
 *
 * This adapter lets a device declaration consume canonical capability intent.
 *
 * ========================================================================== */

hardwareDeviceCapabilityClause
    : hardwareCapabilityAssertion
    | hardwareCapabilityRequirement
    | hardwareCapabilityConstraint
    | hardwareCapabilityPreference
    | hardwareCapabilityHint
    | hardwareCapabilityProperty
    ;


/* ============================================================================
 * 17. ACCELERATOR CAPABILITY ADAPTER
 * ============================================================================
 *
 * Accelerator grammars own accelerator syntax.
 *
 * This rule exposes capability contracts without defining accelerator
 * identity, topology, or implementation.
 *
 * ========================================================================== */

hardwareAcceleratorCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 18. GENERIC COMPUTE CAPABILITY ADAPTER
 * ============================================================================
 *
 * This is intentionally generic.
 *
 * It applies equally to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     future compute substrate
 *
 * No closed hardware-class enumeration is created.
 *
 * ========================================================================== */

hardwareComputeCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 19. HDL CAPABILITY ADAPTER
 * ============================================================================
 *
 * HDL owns hardware-description syntax.
 *
 * This adapter exposes capability intent only.
 *
 * ========================================================================== */

hardwareHdlCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 20. QUANTUM CAPABILITY ADAPTER
 * ============================================================================
 *
 * Quantum capability names remain open-world.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::reset
 *     quantum::logical_qubits
 *     quantum::fault_tolerant_execution
 *
 * Quantum operations are NOT enumerated here.
 *
 * ========================================================================== */

hardwareQuantumCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 21. DISTRIBUTED CAPABILITY ADAPTER
 * ============================================================================
 *
 * Distributed semantics remain owned by the distributed subsystem.
 *
 * Examples may include:
 *
 *     distributed::communication
 *     distributed::collectives
 *     distributed::fault_tolerance
 *
 * Capability identities remain open-world.
 *
 * ========================================================================== */

hardwareDistributedCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 22. AI CAPABILITY ADAPTER
 * ============================================================================
 *
 * AI capability names remain semantic identifiers.
 *
 * No model, framework, algorithm, or application catalogue is embedded here.
 *
 * ========================================================================== */

hardwareAiCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 23. NETWORK CAPABILITY ADAPTER
 * ============================================================================
 *
 * Networking capability semantics remain outside this grammar.
 *
 * ========================================================================== */

hardwareNetworkCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 24. SECURITY CAPABILITY ADAPTER
 * ============================================================================
 *
 * Security authorization is NOT implied by mentioning a capability.
 *
 * Authorization belongs to security/policy analysis.
 *
 * ========================================================================== */

hardwareSecurityCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 25. CANONICAL CAPABILITY ALIASES
 * ============================================================================
 *
 * These aliases exist only where hardware consumers need an explicitly named
 * adapter boundary.
 *
 * They do NOT redefine canonical capability semantics.
 *
 * ========================================================================== */

hardwareCapabilityReference
    : capabilityReference
    ;


hardwareCapabilityName
    : capabilityName
    ;


hardwareCapabilityExpression
    : capabilityExpression
    ;


hardwareCapabilityVersionClause
    : capabilityVersionClause
    ;


hardwareCapabilityAttributeList
    : capabilityAttributeList
    ;


/* ============================================================================
 * 26. HARDWARE CAPABILITY REQUIREMENT LIST
 * ============================================================================
 *
 * Unbounded list.
 *
 * ========================================================================== */

hardwareCapabilityRequirementList
    : hardwareCapabilityRequirement+
    ;


/* ============================================================================
 * 27. HARDWARE CAPABILITY CONTRACT LIST
 * ============================================================================
 *
 * Unbounded list.
 *
 * ========================================================================== */

hardwareCapabilityContractList
    : hardwareCapabilityContract+
    ;


/* ============================================================================
 * 28. HARDWARE CAPABILITY MEMBER LIST
 * ============================================================================
 *
 * Unbounded list.
 *
 * ========================================================================== */

hardwareCapabilityMemberList
    : hardwareCapabilityContractMember+
    ;


/* ============================================================================
 * 29. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * SOURCE
 * ------
 *
 *     Zamani source
 *         |
 *         v
 *     canonical lexer
 *         |
 *         v
 *     canonical parser
 *         |
 *         v
 *     hardwareCapabilityDeclaration
 *
 *
 * AST
 * ---
 *
 * Every capability construct must lower to the existing domain-neutral
 * capability representation.
 *
 * Conceptually:
 *
 *     hardwareCapabilityAssertion
 *         -> CapabilityAssertion
 *
 *     hardwareCapabilityRequirement
 *         -> CapabilityRequirement
 *
 *     hardwareCapabilityConstraint
 *         -> CapabilityConstraint
 *
 *     hardwareCapabilityPreference
 *         -> CapabilityPreference
 *
 *     hardwareCapabilityHint
 *         -> CapabilityHint
 *
 *     hardwareCapabilityProperty
 *         -> CapabilityProperty
 *
 *     hardwareCapabilityExtension
 *         -> CapabilityExtension
 *
 *     hardwareCapabilityComposition
 *         -> CapabilityComposition
 *
 *
 * SEMANTIC
 * --------
 *
 * Semantic analysis must:
 *
 *     resolve capability identity;
 *     validate namespaces;
 *     validate versions;
 *     validate properties;
 *     distinguish requirement/constraint/preference/hint;
 *     resolve extension relationships;
 *     resolve composition;
 *     derive resource implications when specified by semantic metadata;
 *     evaluate target compatibility;
 *     preserve source provenance.
 *
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * This file does NOT parse resource quantities.
 *
 * Therefore:
 *
 *     requires qubits >= required_qubits;
 *     requires memory >= required_memory;
 *     requires nodes >= required_nodes;
 *
 * remain owned by:
 *
 *     grammar/resources/
 *
 * Hardware capability syntax may express:
 *
 *     requires quantum::measurement;
 *     requires tensor::compute;
 *
 * The semantic layer may combine:
 *
 *     capability requirements
 *     resource requirements
 *
 * during target feasibility analysis.
 *
 * Capability metadata may imply resource requirements, but that implication is
 * semantic metadata, not parser behavior.
 *
 *
 * ============================================================================
 * TARGET INTEGRATION
 * ============================================================================
 *
 * targets.g4 owns:
 *
 *     target declaration
 *     target parameters
 *     target compatibility
 *     target-specific intent
 *
 * This file supplies:
 *
 *     hardwareTargetCapabilityClause
 *
 * It does NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     physical device
 *
 *
 * ============================================================================
 * DEVICE INTEGRATION
 * ============================================================================
 *
 * devices.g4 owns device-class syntax.
 *
 * This file supplies:
 *
 *     hardwareDeviceCapabilityClause
 *
 * A source declaration may therefore describe what a logical device class
 * supports without naming a physical device.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum capabilities remain ordinary open-world capability identities.
 *
 * This file does not define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * and does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     coupling maps
 *     calibration
 *     routing
 *     scheduling
 *
 * The quantum path remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL capability intent may describe:
 *
 *     synthesis
 *     programmable logic
 *     timing facilities
 *     simulation
 *     verification
 *     reconfiguration
 *     hardware acceleration
 *
 * The actual HDL representation remains owned by grammar/hdl/.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Capability intent may describe:
 *
 *     distributed communication
 *     collectives
 *     fault tolerance
 *     remote execution
 *     accelerator sharing
 *
 * The number of nodes is never encoded here.
 *
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Capability references are not permissions.
 *
 * For example:
 *
 *     requires security::trusted_execution;
 *
 * does not authorize execution.
 *
 * Authorization is resolved through:
 *
 *     security
 *     policies
 *     capabilities
 *     effects
 *     runtime authorization
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Mentioning a capability does not itself produce an effect.
 *
 * For example:
 *
 *     capability network::communication;
 *
 * does not grant network access.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Capability requirements may be consumed by policy analysis.
 *
 * Policy analysis determines whether a capability:
 *
 *     is permitted;
 *     is prohibited;
 *     requires authorization;
 *     requires additional evidence;
 *     may be used for adaptation;
 *     may be used for execution.
 *
 * This grammar does not evaluate policy.
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Capability source locations must remain traceable.
 *
 * Downstream provenance may record:
 *
 *     source capability
 *     resolved capability
 *     version
 *     provider
 *     evidence
 *     semantic decision
 *     target realization
 *
 * This grammar only preserves source structure.
 *
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime capability state is NOT source syntax.
 *
 * A runtime may report:
 *
 *     available
 *     unavailable
 *     degraded
 *     recovering
 *     quarantined
 *     retired
 *
 * according to the existing resilience architecture.
 *
 * Such runtime state must never alter the meaning of this grammar.
 *
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * PARSER ERRORS
 * ------------
 *
 *     capability;
 *     requires;
 *     prefer;
 *     constraint;
 *     property = true;
 *     extends;
 *
 * are syntax errors.
 *
 *
 * SEMANTIC ERRORS
 * --------------
 *
 *     unknown capability
 *     incompatible version
 *     invalid property
 *     contradictory capability requirement
 *     invalid extension
 *
 * are semantic errors.
 *
 *
 * RESOURCE ERRORS
 * ---------------
 *
 * Insufficient memory, qubits, compute, storage, nodes, or other resources
 * are resource-resolution errors.
 *
 *
 * TARGET ERRORS
 * -------------
 *
 * A target that cannot realize a capability is a target/capability-resolution
 * failure.
 *
 *
 * RUNTIME ERRORS
 * -------------
 *
 * Runtime loss or degradation of a capability belongs to runtime/resilience.
 *
 * These categories MUST NOT be collapsed into parser errors.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     network
 *     filesystem
 *     clock
 *     randomness
 *     environment variables
 *     runtime state
 *     target availability
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     performs no filesystem access;
 *     performs no network access;
 *     performs no hardware discovery;
 *     performs no process execution;
 *     accesses no credentials;
 *     loads no drivers;
 *     invokes no backend;
 *     allocates no physical resource.
 *
 * It is safe to use against untrusted source input.
 *
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * The Rust frontend consuming generated ANTLR output must:
 *
 *     use Rust 2021;
 *     support Rust 1.97 or later;
 *     remain safe Rust;
 *     preserve source spans;
 *     distinguish syntax and semantic diagnostics;
 *     preserve canonical capability identity;
 *     preserve capability-expression structure.
 *
 * This file introduces no unsafe Rust requirement.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It contains no:
 *
 *     physical device catalogue;
 *     vendor catalogue;
 *     physical address syntax;
 *     fixed topology;
 *     fixed accelerator catalogue;
 *     fixed quantum-gate catalogue;
 *     physical qubit enumeration.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must parse:
 *
 *     capability hardware::parallel_compute;
 *
 *     capability hardware::parallel_compute {
 *         requires compute::parallel;
 *     }
 *
 *     capability accelerator::tensor_compute {
 *         requires tensor::compute;
 *         prefer accelerator::matrix;
 *         hint hardware::vector_execution;
 *         property native = true;
 *     }
 *
 *     capability quantum::control {
 *         requires quantum::measurement
 *             and quantum::dynamic_control;
 *     }
 *
 *     capability hardware::reconfigurable_logic {
 *         constraint hardware_mode == required_mode;
 *         property precision = required_precision;
 *     }
 *
 *     capability future::compute::feature {
 *         extends accelerator::compute;
 *     }
 *
 *     capability hardware::composed {
 *         {
 *             hardware::parallel_compute;
 *             accelerator::tensor_compute;
 *         }
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must fail syntactically:
 *
 *     capability;
 *
 *     capability hardware::;
 *
 *     requires;
 *
 *     prefer;
 *
 *     constraint;
 *
 *     hint;
 *
 *     property;
 *
 *     property = true;
 *
 *     extends;
 *
 *     capability hardware::x {
 *         requires;
 *     }
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one capability;
 *     many capabilities;
 *     many contract members;
 *     deeply qualified capability names;
 *     deeply qualified property paths;
 *     nested capability expressions;
 *     nested composition;
 *     mixed quantum/classical capabilities;
 *     mixed hardware/accelerator capabilities;
 *     future capability namespaces;
 *     empty contracts;
 *     repeated capability members.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must support arbitrary source-level quantities subject only to
 * actual parser/compiler resources.
 *
 * Test:
 *
 *     arbitrarily many capability declarations;
 *     arbitrarily many contract members;
 *     arbitrarily many capability references;
 *     arbitrarily deep qualified names within implementation limits;
 *     arbitrarily large capability expressions within implementation limits;
 *     arbitrarily large capability compositions within implementation limits.
 *
 * The grammar itself imposes no hardware capacity ceiling.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Test capability integration with:
 *
 *     classical computing
 *     quantum computing
 *     HDL
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerators
 *     QPU
 *     simulation
 *     distributed computing
 *     networking
 *     AI
 *     security
 *     resource negotiation
 *     execution
 *     policies
 *
 * New domain names must be representable through open-world capability
 * identities without changing this grammar.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This rewrite preserves the public hardware capability entry point:
 *
 *     hardwareCapabilityDeclaration
 *
 * and the principal hardware capability adapters.
 *
 * The following obsolete/redundant concepts are intentionally removed:
 *
 *     hardwareCapabilityFeatureReference
 *     hardwareCapabilityMatch
 *     hardwareCapabilityAvailability
 *     hardwareCapabilityProfile
 *     hardwareCapabilityNamedContract
 *     hardwareCapabilityComparisonOperator
 *
 * Their responsibilities belong elsewhere:
 *
 *     matching        -> semantic capability resolution
 *     availability    -> resource/target/runtime state
 *     profiles        -> semantic/target profile systems
 *     generic contracts -> contract subsystem
 *     comparisons     -> canonical expression grammar
 *
 * This prevents this file from becoming a second semantic system.
 *
 *
 * ============================================================================
 * HARDWARE.G4 INTEGRATION
 * ============================================================================
 *
 * grammar/hardware/hardware.g4 already imports:
 *
 *     ZamaniHardwareCapabilitiesParser
 *
 * and dispatches:
 *
 *     hardwareCapabilityDeclaration
 *
 * Therefore the hardware composition root consumes this file through one
 * stable entry point.
 *
 * hardware.g4 MUST NOT duplicate any of the rules defined here.
 *
 *
 * ============================================================================
 * TARGETS.G4 INTEGRATION
 * ============================================================================
 *
 * targets.g4 should consume:
 *
 *     hardwareTargetCapabilityClause
 *
 * rather than redefining:
 *
 *     capabilityReference
 *     capabilityExpression
 *     capability requirement syntax
 *
 *
 * ============================================================================
 * DEVICES.G4 INTEGRATION
 * ============================================================================
 *
 * devices.g4 should consume:
 *
 *     hardwareDeviceCapabilityClause
 *
 * and remain the owner of:
 *
 *     deviceDeclaration
 *     device parameters
 *     device relationships
 *     device-class semantics
 *
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * resources/capabilities.g4 remains the owner of resource-scoped capability
 * intent.
 *
 * hardware/capabilities.g4 does not duplicate:
 *
 *     resourceCapabilityIntent
 *     resourceCapabilityRequirement
 *     resourceCapabilityConstraint
 *     resourceCapabilityAvailability
 *     resourceCapabilityRelationship
 *
 * Hardware resource declarations should consume whichever resource capability
 * form is appropriate rather than creating another resource capability model.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * quantum/ grammars consume canonical capability references.
 *
 * No quantum capability is hard-coded into this file.
 *
 * Therefore future quantum capabilities can be added through the semantic
 * registry without changing this grammar.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It uses the canonical ZamaniLexer token vocabulary.
 * [x] It imports the canonical capability grammar.
 * [x] It imports the canonical expression grammar.
 * [x] It does not redefine capability identity.
 * [x] It does not redefine capability expressions.
 * [x] It does not redefine version syntax.
 * [x] It does not create a second resource grammar.
 * [x] It does not create a second policy grammar.
 * [x] It does not create a second effect grammar.
 * [x] It does not create a second target grammar.
 * [x] It does not create a second topology grammar.
 * [x] It does not create a second placement grammar.
 * [x] It does not create a second quantum IR.
 * [x] It does not enumerate quantum operations.
 * [x] It does not enumerate hardware vendors.
 * [x] It does not enumerate physical devices.
 * [x] It does not contain hardware capacity constants.
 * [x] It does not perform hardware discovery.
 * [x] It does not perform runtime execution.
 * [x] It contains no Rust actions.
 * [x] It requires no unsafe Rust.
 * [x] It has explicit AST ownership.
 * [x] It has explicit semantic ownership.
 * [x] It has explicit IR ownership.
 * [x] It has explicit downstream integration.
 * [x] It has positive-test requirements.
 * [x] It has negative-test requirements.
 * [x] It has boundary-test requirements.
 * [x] It has scalability-test requirements.
 *
 * Repository integration must additionally verify:
 *
 * [ ] ANTLR generation succeeds for this grammar.
 * [ ] ZamaniHardwareCapabilitiesParser is importable by Hardware.
 * [ ] All referenced lexer tokens exist in ZamaniLexer.
 * [ ] Existing hardware declarations delegate here without duplicate rules.
 * [ ] AST lowering consumes the existing capability AST.
 * [ ] Semantic lowering consumes the existing capability model.
 * [ ] Resource analysis consumes capability requirements.
 * [ ] Target analysis consumes capability constraints/preferences.
 * [ ] Quantum capability metadata can influence semantic quantum validation.
 * [ ] quantum::ir remains the only quantum IR.
 * [ ] Positive tests pass.
 * [ ] Negative tests pass.
 * [ ] Boundary tests pass.
 * [ ] Scalability tests pass.
 * [ ] Cross-domain tests pass.
 * [ ] Rust 1.97+ frontend integration passes.
 * [ ] Repository-wide safe-Rust validation passes.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Hardware capability grammar defines INTENT.
 *
 * It does not define REALIZATION.
 *
 * Therefore:
 *
 *     hardware capability
 *             |
 *             v
 *     semantic capability
 *             |
 *             v
 *     resource analysis
 *             |
 *             v
 *     target negotiation
 *             |
 *             v
 *     optimization
 *             |
 *             v
 *     placement / routing / scheduling
 *             |
 *             v
 *     resilience
 *             |
 *             v
 *     ZQN
 *             |
 *             v
 *     HAL
 *             |
 *             v
 *     actual target
 *
 * The source program remains target-independent.
 *
 * The implementation may therefore scale from a tiny target to arbitrarily
 * large computational systems, limited only by actual semantic feasibility,
 * available resources, target capabilities, and implementation resources.
 *
 * No artificial hardware ceiling belongs in this grammar.
 *
 * ============================================================================
 */