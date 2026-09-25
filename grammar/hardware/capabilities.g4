/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/capabilities.g4
 *
 * Grammar:
 *     ZamaniHardwareCapabilitiesParser
 *
 * Status:
 *     CANONICAL HARDWARE-CAPABILITY ADAPTER GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *     action-free
 *     predicate-free
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the HARDWARE-SPECIFIC USE of the canonical Zamani capability
 * model.
 *
 * It does NOT create a second capability language.
 *
 * Canonical capability identity, capability references, capability expressions,
 * and capability version syntax remain owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Canonical general expressions remain owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file therefore acts as the hardware-domain integration boundary:
 *
 *     hardware intent
 *          |
 *          v
 *     canonical capability model
 *          |
 *          v
 *     hardware semantic analysis
 *
 * ============================================================================
 * FUNDAMENTAL OWNERSHIP RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hardware capability contracts;
 *     - hardware capability requirements;
 *     - hardware capability preferences;
 *     - hardware capability constraints;
 *     - hardware capability hints;
 *     - hardware capability properties;
 *     - hardware capability extension;
 *     - hardware capability contract members;
 *     - hardware capability collections;
 *     - integration of canonical capability references into hardware syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - capability identity;
 *     - capability version semantics;
 *     - general expressions;
 *     - general types;
 *     - resources;
 *     - targets;
 *     - topology;
 *     - placement;
 *     - devices;
 *     - physical hardware discovery;
 *     - device allocation;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime capability probing;
 *     - backend selection.
 *
 * ============================================================================
 * CANONICAL DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     grammar/lexer
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     core/capabilities.g4
 *          |
 *          v
 *     hardware/capabilities.g4
 *          |
 *          v
 *     hardware.g4
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic capability model
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     resource analysis       target analysis
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              compiler planning
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *       optimize   routing   scheduling
 *                                |
 *                                v
 *                               HAL
 *                                |
 *                                v
 *                             runtime
 *
 * Quantum programs continue through:
 *
 *     semantic quantum model
 *             |
 *             v
 *         quantum::ir
 *
 * This file never creates another quantum IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Hardware capabilities describe WHAT an implementation must support.
 *
 * They do not prescribe:
 *
 *     - a physical CPU;
 *     - a physical GPU;
 *     - a physical FPGA;
 *     - a physical ASIC;
 *     - a physical QPU;
 *     - a physical qubit;
 *     - a device identifier;
 *     - a vendor;
 *     - a PCI address;
 *     - an IP address;
 *     - a machine name;
 *     - a topology;
 *     - a calibration record.
 *
 * Those are downstream realization concerns.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO universal capacity limits.
 *
 * It MUST NOT define:
 *
 *     MAX_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_ACCELERATORS
 *
 * Collections use ANTLR repetition:
 *
 *     *
 *     +
 *
 * Quantities and conditions use the canonical expression language.
 *
 * Practical limits belong to:
 *
 *     compiler resources;
 *     semantic/resource analysis;
 *     target capabilities;
 *     runtime resources;
 *     deployment policy;
 *     physical hardware.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * CAPABILITY
 *     What an implementation can do.
 *
 * RESOURCE
 *     Something that may be available, consumed, shared, reserved, or
 *     otherwise relevant to execution.
 *
 * REQUIREMENT
 *     A condition required for semantic validity.
 *
 * CONSTRAINT
 *     A mandatory condition on realization.
 *
 * PREFERENCE
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *     Advisory information.
 *
 * TARGET
 *     An abstract realization/execution context.
 *
 * This grammar does not collapse these concepts.
 *
 * ============================================================================
 * HARDWARE-CAPABILITY EXAMPLES
 * ============================================================================
 *
 * Valid conceptual forms include:
 *
 *     capability quantum::measurement;
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::dynamic_control;
 *
 *     prefer accelerator::tensor_compute;
 *
 *     constraint capability::deterministic_execution;
 *
 *     hint capability::vector_execution;
 *
 *     property native = true;
 *
 *     extends accelerator::compute;
 *
 * The exact capability identity is resolved by the canonical capability
 * grammar and semantic registry.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareCapabilitiesParser;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Import the canonical capability model.
 *
 * `Capabilities` is the grammar declared by:
 *
 *     grammar/core/capabilities.g4
 *
 * It owns:
 *
 *     capabilityDeclaration
 *     capabilityReference
 *     capabilityExpression
 *     capabilityVersionClause
 *     capabilityName
 *     capabilityAlias
 *     and related capability syntax.
 *
 * This file deliberately does not redefine any of those rules.
 *
 * The canonical general expression rule is also imported so that hardware
 * properties and predicates can use the same expression semantics as the
 * rest of Zamani.
 */
import Capabilities, Expressions;


/* ============================================================================
 * 1. PUBLIC HARDWARE-CAPABILITY ENTRY POINTS
 * ========================================================================== */

/*
 * A complete hardware capability declaration.
 *
 * Example:
 *
 *     capability quantum::dynamic_control;
 *
 *     capability accelerator::tensor_compute {
 *         requires compute::parallel;
 *     }
 *
 * The identity itself remains canonical.
 */
hardwareCapabilityDeclaration
    : capabilityDeclaration
    | hardwareCapabilityContractDeclaration
    ;


/*
 * A hardware-specific named capability contract.
 *
 * The leading `capability` keyword remains owned by the canonical capability
 * grammar. The contract body belongs to this file.
 */
hardwareCapabilityContractDeclaration
    : CAPABILITY
      capabilityName
      hardwareCapabilityContractBody
      SEMICOLON?
    ;


/*
 * A reusable anonymous contract.
 *
 * This is useful when hardware.g4, devices.g4, targets.g4, or accelerator
 * grammars need to attach a capability contract without declaring a new
 * capability identity.
 */
hardwareCapabilityContract
    : LBRACE
      hardwareCapabilityContractMember*
      RBRACE
    ;


/* ============================================================================
 * 2. HARDWARE CAPABILITY CONTRACT MEMBERS
 * ========================================================================== */

hardwareCapabilityContractMember
    : hardwareCapabilityRequirement
    | hardwareCapabilityConstraint
    | hardwareCapabilityPreference
    | hardwareCapabilityHint
    | hardwareCapabilityProperty
    | hardwareCapabilityExtension
    | hardwareCapabilityReferenceMember
    ;


/* ============================================================================
 * 3. REQUIREMENTS
 * ============================================================================
 *
 * Requirement syntax is mandatory semantic intent.
 *
 * It does not perform capability discovery.
 * It does not select a physical target.
 * ========================================================================== */

hardwareCapabilityRequirement
    : REQUIRES
      capabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 4. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict valid realization.
 *
 * They are not preferences.
 * ========================================================================== */

hardwareCapabilityConstraint
    : CONSTRAINT
      capabilityConstraintExpression
      SEMICOLON
    ;

hardwareCapabilityConstraintExpression
    : capabilityExpression
    | expression
    ;


/* ============================================================================
 * 5. PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory optimization intent.
 *
 * They MUST NOT be interpreted as semantic requirements merely because they
 * occur in source.
 * ========================================================================== */

hardwareCapabilityPreference
    : PREFER
      capabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 6. HINTS
 * ============================================================================
 *
 * Hints are weaker than preferences.
 *
 * A compiler may ignore a hint without invalidating the program.
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
 * 7. PROPERTIES
 * ============================================================================
 *
 * Properties are extensible metadata/semantic attributes.
 *
 * The property name remains an identifier rather than becoming a permanent
 * language keyword.
 *
 * Examples:
 *
 *     native = true;
 *     optional = false;
 *     emulated = true;
 *     experimental = true;
 *     priority = 10;
 *
 * The semantic layer decides which properties are valid.
 * ========================================================================== */

hardwareCapabilityProperty
    : PROPERTY
      hardwareCapabilityPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;

hardwareCapabilityPropertyName
    : identifier
    ;


/* ============================================================================
 * 8. EXTENSION
 * ============================================================================
 *
 * Capability extension is symbolic.
 *
 * It does not imply physical inheritance, implementation inheritance, or
 * object-oriented inheritance.
 * ========================================================================== */

hardwareCapabilityExtension
    : EXTENDS
      capabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 9. DIRECT CAPABILITY REFERENCE
 * ============================================================================
 *
 * This allows a contract body to contain a bare capability reference.
 *
 * Example:
 *
 *     {
 *         quantum::measurement;
 *         accelerator::tensor_compute;
 *     }
 *
 * Whether a bare reference means "requires", "provides", or another semantic
 * relationship must be determined by the enclosing semantic context.
 *
 * To avoid ambiguity, production hardware declarations should prefer explicit
 * requirement/preference/constraint forms.
 * ========================================================================== */

hardwareCapabilityReferenceMember
    : capabilityReference
      SEMICOLON
    ;


/* ============================================================================
 * 10. CAPABILITY CONTRACT BODY
 * ========================================================================== */

hardwareCapabilityContractBody
    : LBRACE
      hardwareCapabilityContractMember*
      RBRACE
    ;


/* ============================================================================
 * 11. CAPABILITY LISTS
 * ============================================================================
 *
 * These wrappers deliberately reuse canonical capability references.
 * ========================================================================== */

hardwareCapabilityReferenceList
    : capabilityReference
      (
          COMMA
          capabilityReference
      )*
    ;

hardwareCapabilityExpressionList
    : capabilityExpression
      (
          COMMA
          capabilityExpression
      )*
    ;


/* ============================================================================
 * 12. HARDWARE CAPABILITY REQUIREMENT LIST
 * ========================================================================== */

hardwareCapabilityRequirementList
    : hardwareCapabilityRequirement+
    ;


/* ============================================================================
 * 13. HARDWARE CAPABILITY CONTRACT LIST
 * ========================================================================== */

hardwareCapabilityContractList
    : hardwareCapabilityContract+
    ;


/* ============================================================================
 * 14. HARDWARE CAPABILITY COLLECTION
 * ============================================================================
 *
 * A collection is an abstract set of capability references.
 *
 * It does not allocate or discover hardware.
 * ========================================================================== */

hardwareCapabilityCollection
    : LBRACKET
      hardwareCapabilityReferenceList?
      RBRACKET
    ;


/* ============================================================================
 * 15. CAPABILITY PROPERTY PREDICATE
 * ============================================================================
 *
 * Property predicates use the canonical expression grammar.
 *
 * Example:
 *
 *     property supports_precision == required_precision;
 *
 * The grammar does not decide whether that property exists or what it means.
 * ========================================================================== */

hardwareCapabilityPropertyPredicate
    : hardwareCapabilityPropertyName
      hardwareCapabilityComparisonOperator
      expression
    ;

hardwareCapabilityComparisonOperator
    : EQ_EQ
    | NOT_EQ
    | LT
    | LE
    | GT
    | GE
    ;


/* ============================================================================
 * 16. CAPABILITY MATCHING INTENT
 * ============================================================================
 *
 * This is declarative matching intent only.
 *
 * It does not perform discovery.
 *
 * The semantic layer may later evaluate the expression against a capability
 * environment.
 * ========================================================================== */

hardwareCapabilityMatch
    : MATCH
      capabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 17. CAPABILITY AVAILABILITY INTENT
 * ============================================================================
 *
 * Availability is source-level information.
 *
 * The parser records the expression.
 *
 * Runtime/hardware probing remains outside this grammar.
 * ========================================================================== */

hardwareCapabilityAvailability
    : AVAILABILITY
      capabilityReference
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 18. CAPABILITY PROFILE
 * ============================================================================
 *
 * A profile is a reusable symbolic grouping of capabilities.
 *
 * It does not represent a physical device class.
 * ========================================================================== */

hardwareCapabilityProfile
    : PROFILE
      capabilityName
      hardwareCapabilityContract
    ;


/* ============================================================================
 * 19. CAPABILITY CONTRACT DECLARATION
 * ============================================================================
 *
 * `contract` is a language-level grouping construct.
 *
 * It is intentionally independent from:
 *
 *     deployment contracts;
 *     ABI contracts;
 *     legal contracts;
 *     runtime authorization;
 *     hardware procurement.
 *
 * Semantic context determines its exact use.
 * ========================================================================== */

hardwareCapabilityNamedContract
    : CONTRACT
      identifier
      hardwareCapabilityContract
    ;


/* ============================================================================
 * 20. HARDWARE-CAPABILITY TARGET ADAPTER
 * ============================================================================
 *
 * Targets may consume this rule.
 *
 * The target itself remains owned by:
 *
 *     grammar/hardware/targets.g4
 *
 * This rule only represents the capability portion of target intent.
 * ========================================================================== */

hardwareTargetCapabilityClause
    : REQUIRES
      capabilityExpression
      SEMICOLON
    | PREFER
      capabilityExpression
      SEMICOLON
    | CONSTRAINT
      capabilityConstraintExpression
      SEMICOLON
    ;


/* ============================================================================
 * 21. DEVICE-CAPABILITY ADAPTER
 * ============================================================================
 *
 * Devices may consume this contract without redefining capability syntax.
 *
 * Physical device discovery remains downstream.
 * ========================================================================== */

hardwareDeviceCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 22. ACCELERATOR-CAPABILITY ADAPTER
 * ========================================================================== */

hardwareAcceleratorCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 23. CPU/GPU/FPGA/ASIC/QPU CAPABILITY ADAPTER
 * ============================================================================
 *
 * These are deliberately generic.
 *
 * There is no separate closed capability enumeration for:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *
 * New hardware classes can use the same capability contract.
 * ========================================================================== */

hardwareComputeCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 24. HDL CAPABILITY ADAPTER
 * ============================================================================
 *
 * HDL may state capability requirements without moving physical realization
 * into the capability grammar.
 * ========================================================================== */

hardwareHdlCapabilityClause
    : REQUIRES
      capabilityExpression
      SEMICOLON
    | CONSTRAINT
      capabilityConstraintExpression
      SEMICOLON
    | PREFER
      capabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 25. QUANTUM CAPABILITY ADAPTER
 * ============================================================================
 *
 * Examples of semantic capability identities may include:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::reset
 *     quantum::logical_qubits
 *     quantum::error_correction
 *
 * These remain identifiers, not fixed parser alternatives.
 *
 * This grammar does not enumerate quantum operations or gates.
 * ========================================================================== */

hardwareQuantumCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 26. DISTRIBUTED CAPABILITY ADAPTER
 * ========================================================================== */

hardwareDistributedCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 27. AI / ACCELERATOR CAPABILITY ADAPTER
 * ========================================================================== */

hardwareAiCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 28. NETWORK CAPABILITY ADAPTER
 * ========================================================================== */

hardwareNetworkCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 29. SECURITY CAPABILITY ADAPTER
 * ========================================================================== */

hardwareSecurityCapabilityClause
    : hardwareCapabilityContractMember
    ;


/* ============================================================================
 * 30. GENERIC HARDWARE CAPABILITY EXPRESSION
 * ============================================================================
 *
 * This alias exists solely as an integration boundary.
 *
 * It delegates to the canonical capability expression.
 *
 * There is deliberately NO second expression hierarchy here.
 * ========================================================================== */

hardwareCapabilityExpression
    : capabilityExpression
    ;


/* ============================================================================
 * 31. GENERIC HARDWARE CAPABILITY REFERENCE
 * ========================================================================== */

hardwareCapabilityReference
    : capabilityReference
    ;


/* ============================================================================
 * 32. HARDWARE CAPABILITY NAME
 * ============================================================================
 *
 * This alias preserves a stable hardware-domain API without redefining
 * capability identity.
 * ========================================================================== */

hardwareCapabilityName
    : capabilityName
    ;


/* ============================================================================
 * 33. HARDWARE CAPABILITY VERSION
 * ============================================================================
 *
 * Version syntax remains owned by core/capabilities.g4.
 * ========================================================================== */

hardwareCapabilityVersionClause
    : capabilityVersionClause
    ;


/* ============================================================================
 * 34. HARDWARE CAPABILITY ALIAS
 * ========================================================================== */

hardwareCapabilityAlias
    : capabilityAlias
    ;


/* ============================================================================
 * 35. HARDWARE CAPABILITY REQUIREMENT REFERENCE
 * ========================================================================== */

hardwareCapabilityRequirementReference
    : capabilityRequirementReference
    ;


/* ============================================================================
 * 36. HARDWARE CAPABILITY PREFERENCE REFERENCE
 * ========================================================================== */

hardwareCapabilityPreferenceReference
    : capabilityPreferenceReference
    ;


/* ============================================================================
 * 37. HARDWARE CAPABILITY CONSTRAINT REFERENCE
 * ========================================================================== */

hardwareCapabilityConstraintReference
    : capabilityConstraintReference
    ;


/* ============================================================================
 * 38. HARDWARE CAPABILITY EFFECT REFERENCE
 * ========================================================================== */

hardwareCapabilityEffectReference
    : capabilityEffectReference
    ;


/* ============================================================================
 * 39. HARDWARE CAPABILITY TARGET REFERENCE
 * ========================================================================== */

hardwareCapabilityTargetReference
    : capabilityTargetReference
    ;


/* ============================================================================
 * 40. HARDWARE CAPABILITY FEATURE REFERENCE
 * ========================================================================== */

hardwareCapabilityFeatureReference
    : capabilityFeatureReference
    ;


/* ============================================================================
 * 41. INTEGRATION CONTRACT
 * ============================================================================
 *
 * HARDWARE.G4
 * -----------
 *
 * hardware.g4 MUST consume:
 *
 *     hardwareCapabilityDeclaration
 *     hardwareCapabilityReference
 *     hardwareCapabilityContract
 *
 * and MUST NOT redefine:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityExpression
 *     capabilityVersionClause
 *
 * TARGETS.G4
 * ----------
 *
 * targets.g4 may consume:
 *
 *     hardwareTargetCapabilityClause
 *     hardwareCapabilityContract
 *
 * It remains the sole owner of target declaration syntax.
 *
 * DEVICES.G4
 * ----------
 *
 * devices.g4 may consume:
 *
 *     hardwareDeviceCapabilityClause
 *     hardwareCapabilityContract
 *
 * It remains the owner of device syntax.
 *
 * RESOURCES
 * ---------
 *
 * Resource grammar remains responsible for:
 *
 *     resource identity;
 *     resource quantities;
 *     resource availability;
 *     resource accounting;
 *     resource requirements.
 *
 * A capability may semantically imply a resource requirement, but this
 * transformation is performed by semantic analysis, not the parser.
 *
 * TOPOLOGY
 * --------
 *
 * topology.g4 owns topology syntax.
 *
 * This grammar may express capabilities related to topology, but does not
 * define topology itself.
 *
 * PLACEMENT
 * ---------
 *
 * placement.g4 consumes resolved semantic capability information.
 *
 * It does not perform placement.
 *
 * QUANTUM
 * -------
 *
 * Quantum source capability intent may ultimately influence quantum semantic
 * validation.
 *
 * The canonical path remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic quantum model
 *       ->
 *     quantum::ir
 *
 * This file never creates or modifies quantum::ir.
 *
 * QEC
 * ---
 *
 * QEC may consume resolved capabilities.
 *
 * QEC implementation remains outside grammar.
 *
 * ZQN
 * ---
 *
 * ZQN may consume resolved noise/fault-related capabilities.
 *
 * ZQN remains the owner of noise/fault semantics.
 *
 * ROUTING
 * -------
 *
 * Routing may use capability information to determine legal realizations.
 *
 * Routing does not belong here.
 *
 * SCHEDULING
 * ----------
 *
 * Scheduling may use capability information.
 *
 * Scheduling does not belong here.
 *
 * HAL
 * ---
 *
 * HAL provides target/environment capability information.
 *
 * The parser never probes the HAL.
 *
 * RUNTIME
 * -------
 *
 * Runtime capability state is downstream data.
 *
 * It is not source syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * All hardware capability syntax must map to the existing domain-neutral
 * capability representation.
 *
 * Conceptual mapping:
 *
 *     hardwareCapabilityDeclaration
 *             |
 *             v
 *     CapabilityDeclaration / HardwareCapabilityContract
 *             |
 *             v
 *     semantic capability model
 *
 * The AST MUST preserve:
 *
 *     capability identity;
 *     version requirement;
 *     contract member order;
 *     property names;
 *     property expressions;
 *     source spans.
 *
 * The AST MUST NOT contain:
 *
 *     physical device IDs;
 *     physical qubit IDs;
 *     backend handles;
 *     runtime authorization tokens;
 *     calibration state;
 *     scheduler state;
 *     routing state.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability names;
 *     - resolving namespaces;
 *     - checking capability versions;
 *     - validating capability properties;
 *     - validating requirement satisfiability;
 *     - detecting contradictory requirements;
 *     - detecting incompatible constraints;
 *     - resolving capability inheritance;
 *     - resolving profiles;
 *     - deriving resource requirements when specified by capability metadata;
 *     - checking target applicability;
 *     - checking portability;
 *     - checking dialect ownership;
 *     - validating security/trust of capability evidence.
 *
 * None of these operations occur during parsing.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors:
 *
 *     requires;
 *     prefer;
 *     constraint;
 *     capability foo::;
 *
 * are parser errors.
 *
 * Semantic errors:
 *
 *     unknown capability;
 *     incompatible capability version;
 *     contradictory requirement;
 *     unavailable target capability;
 *
 * are semantic/resource/target errors.
 *
 * A resource shortage MUST NOT be reported as a syntax error.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is pure with respect to external state.
 *
 * It MUST NOT depend on:
 *
 *     hardware;
 *     network;
 *     filesystem;
 *     clock;
 *     randomness;
 *     environment variables;
 *     runtime state;
 *     target discovery.
 *
 * Identical source and identical grammar/token versions must produce
 * identical parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Capability syntax is untrusted input.
 *
 * This grammar performs:
 *
 *     no filesystem access;
 *     no network access;
 *     no process execution;
 *     no hardware probing;
 *     no credential access;
 *     no backend loading.
 *
 * Capability names and property expressions are data until semantic analysis.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No machine capacity is encoded here.
 *
 * Specifically absent:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * There is also no:
 *
 *     physical device enumeration;
 *     physical qubit enumeration;
 *     vendor enumeration;
 *     fixed topology;
 *     fixed accelerator catalogue;
 *     fixed quantum gate catalogue.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Therefore Rust 1.97 / Rust 1.97.1 integration occurs entirely in the
 * generated-parser/frontend crate.
 *
 * The Rust crate MUST:
 *
 *     - use Rust 2021;
 *     - compile without unsafe code;
 *     - preserve parser source spans;
 *     - map parse nodes to the existing AST;
 *     - preserve capability identity;
 *     - distinguish syntax errors from semantic errors.
 *
 * This .g4 file itself contains no Rust and therefore contains no unsafe code.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following classes must exist under:
 *
 *     grammar/tests/hardware/capabilities/
 *
 * POSITIVE:
 *
 *     capability quantum::measurement;
 *
 *     capability accelerator::tensor_compute {
 *         requires compute::parallel;
 *     }
 *
 *     capability future::domain::feature {
 *         prefer accelerator::tensor_compute;
 *     }
 *
 *     capability quantum::dynamic_control {
 *         constraint quantum::measurement;
 *         hint quantum::reset;
 *     }
 *
 *     capability accelerator::compute {
 *         property native = true;
 *     }
 *
 * NEGATIVE:
 *
 *     capability;
 *
 *     requires;
 *
 *     prefer;
 *
 *     constraint;
 *
 *     capability quantum::;
 *
 *     property = true;
 *
 * BOUNDARY:
 *
 *     one capability;
 *     many capability members;
 *     deeply qualified capability names;
 *     large symbolic expressions;
 *     large capability collections;
 *     nested contracts.
 *
 * SCALABILITY:
 *
 *     arbitrary number of capability members;
 *     arbitrary number of capability references;
 *     arbitrary qualified-name depth;
 *     arbitrary program-level capability declarations;
 *     symbolic resource requirements;
 *     changing target resource availability without changing source syntax.
 *
 * CROSS-DOMAIN:
 *
 *     classical capabilities;
 *     quantum capabilities;
 *     HDL capabilities;
 *     accelerator capabilities;
 *     distributed capabilities;
 *     AI capabilities;
 *     networking capabilities;
 *     security capabilities;
 *     future dialect capabilities.
 *
 * DETERMINISM:
 *
 *     identical source + grammar + lexer version
 *         ->
 *     identical parse structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Canonical lexer is used.
 * [x] No K_* pseudo-tokens are used.
 * [x] Canonical capability identity is reused.
 * [x] Canonical capability expressions are reused.
 * [x] General expressions are not reimplemented.
 * [x] No hardware-size limits exist.
 * [x] No physical hardware is selected.
 * [x] No vendor catalogue is embedded.
 * [x] No quantum gate catalogue is embedded.
 * [x] No quantum IR is duplicated.
 * [x] No QEC implementation is embedded.
 * [x] No ZQN implementation is embedded.
 * [x] No routing is embedded.
 * [x] No scheduling is embedded.
 * [x] No optimization is embedded.
 * [x] No runtime probing is embedded.
 * [x] No Rust actions exist.
 *
 * The repository integration items below must also be completed:
 *
 * [ ] canonical core/capabilities.g4 compiles against ZamaniLexer;
 * [ ] canonical Expressions grammar compiles against ZamaniLexer;
 * [ ] Hardware composition imports this grammar;
 * [ ] hardware.g4 removes duplicate capability productions;
 * [ ] targets.g4 consumes this capability contract rather than duplicating it;
 * [ ] resources/capabilities.g4 consumes canonical capabilityReference;
 * [ ] devices.g4 consumes the capability contract;
 * [ ] capability AST lowering uses the existing frontend AST;
 * [ ] semantic capability resolution is implemented;
 * [ ] positive/negative/boundary/scalability tests pass;
 * [ ] Rust 1.97/1.97.1 generated-parser integration passes;
 * [ ] repository-wide no-unsafe policy passes.
 *
 * ============================================================================
 */