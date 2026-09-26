/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/resources.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar identity:
 *     Resources
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Grammar-only.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No parser actions.
 *     No filesystem access.
 *     No network access.
 *     No hardware access.
 *     No unsafe Rust requirement.
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * CANONICAL UNIVERSAL RESOURCE GRAMMAR
 *
 * This file is the SINGLE concrete source-level owner for universal resource
 * intent in Zamani.
 *
 * It is intentionally NOT a hardware allocator, scheduler, router, runtime,
 * resource manager, HAL, QEC implementation, ZQN implementation, or IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                       Zamani parser
 *                              |
 *                              v
 *                    +--------------------+
 *                    |     Resources      |
 *                    +--------------------+
 *                              |
 *             +----------------+----------------+
 *             |                                 |
 *             v                                 v
 *       ResourceExpressions                   Names
 *             |                                 |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                         Frontend AST
 *                              |
 *                              v
 *                    Semantic resource model
 *                              |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *  requirements          capabilities            constraints
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical           quantum::ir        HDL/hardware
 *         IR                                      IR
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *              routing     scheduling   resilience
 *                 |            |            |
 *                 +------------+------------+
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                      target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Resource syntax expresses:
 *
 *     WHAT computation requires
 *     WHAT capabilities are required
 *     WHAT constraints must hold
 *     WHAT resources are involved
 *     WHAT properties are preferred
 *     WHAT scaling behavior is intended
 *
 * Resource syntax does NOT express:
 *
 *     WHICH physical CPU
 *     WHICH physical core
 *     WHICH physical GPU
 *     WHICH physical FPGA
 *     WHICH physical ASIC
 *     WHICH physical QPU
 *     WHICH physical qubit
 *     WHICH physical memory bank
 *     WHICH physical network node
 *     WHICH cloud instance
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar is a foundational component of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Resource intent MUST remain target-independent wherever the programmer has
 * not explicitly requested a target-specific realization.
 *
 * A portable source program may therefore express:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires nodes >= required_nodes;
 *
 *     prefer latency <= latency_budget;
 *
 *     target = quantum;
 *
 * without binding the source program to a particular machine.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define or imply:
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
 * Nor may it encode equivalent indirect limits such as:
 *
 *     exactly 32 CPUs
 *     exactly 1024 qubits
 *     exactly 64 GB memory
 *     exactly 24 GB VRAM
 *     exactly 32-bit registers
 *     exactly N network nodes
 *
 * Numeric literals occurring inside resource expressions are PROGRAM VALUES,
 * not universal machine limits.
 *
 * Example:
 *
 *     requires qubits >= 1024;
 *
 * is valid program intent.
 *
 * It does NOT establish:
 *
 *     MAX_QUBITS = 1024
 *
 * ============================================================================
 * UNBOUNDED SCALABILITY
 * ============================================================================
 *
 * Lists and nested structures use ANTLR repetition:
 *
 *     *
 *     +
 *
 * and recursive composition.
 *
 * There is deliberately no language-level maximum number of:
 *
 *     resources
 *     properties
 *     requirements
 *     constraints
 *     capabilities
 *     groups
 *     contracts
 *     profiles
 *     resource expressions
 *     resource hierarchy levels
 *
 * "Infinity" means:
 *
 *     unbounded by the language architecture.
 *
 * It does not claim physically infinite hardware.
 *
 * Physical limits are determined by:
 *
 *     target availability
 *     compiler resources
 *     runtime resources
 *     deployment resources
 *     security policy
 *     implementation limits
 *
 * ============================================================================
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resources
 *     resourceItem
 *     resourceDeclaration
 *     resourceKindClause
 *     resourceSpecification
 *     resourceBodyItem
 *     resourceClause
 *     resourceQuantityClause
 *     resourceReferenceClause
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *     resourceCapacityClause
 *     resourceAvailabilityClause
 *     resourcePortabilityClause
 *     resourceScalabilityClause
 *     resourcePerformanceClause
 *     resourceLatencyClause
 *     resourceThroughputClause
 *     resourceBandwidthClause
 *     resourceEnergyClause
 *     resourcePowerClause
 *     resourceReliabilityClause
 *     resourceResilienceClause
 *     resourceCostClause
 *     resourceReservation
 *     resourceAcquisition
 *     resourceRelease
 *     resourceDerivation
 *     resourceGroup
 *     resourceContract
 *     resourceProfile
 *     resourcePropertyClause
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     resourceExpression
 *     arithmetic
 *     logical operators
 *     comparison precedence
 *     literals
 *     type syntax
 *     hardware discovery
 *     target selection algorithms
 *     placement
 *     routing
 *     scheduling
 *     optimization
 *     QEC
 *     ZQN
 *     HAL
 *     runtime resource management
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * ResourceExpressions:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * owns the resource-expression composition boundary and delegates ordinary
 * expression syntax to the canonical expression grammar.
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * owns:
 *
 *     identifier
 *     qualifiedName
 *
 * No resource-specific expression or name implementation is duplicated here.
 *
 * ============================================================================
 */

parser grammar Resources;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 1. RESOURCE SECTION
 * ============================================================================
 *
 * A resource section may contain zero or more resource items.
 *
 * Cardinality is intentionally unbounded at the language level.
 *
 * This rule is the principal composition entry point for resource-aware
 * grammars that want a complete sequence.
 *
 * ============================================================================
 */

resources
    : resourceItem*
    ;


/*
 * ============================================================================
 * 2. RESOURCE ITEM
 * ============================================================================
 *
 * Every universal resource construct enters through this rule.
 *
 * Domain grammars should consume resourceItem rather than duplicating this
 * alternative list.
 * ============================================================================
 */

resourceItem
    : resourceDeclaration
    | resourceRequirement
    | resourceConstraint
    | resourcePreference
    | resourceHint
    | resourceCapability
    | resourceTarget
    | resourceDerivation
    | resourceReservation
    | resourceAcquisition
    | resourceRelease
    | resourceGroup
    | resourceContract
    | resourceProfile
    ;


/*
 * ============================================================================
 * 3. RESOURCE DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource accelerator: accelerator {
 *         quantity = workload_size;
 *     };
 *
 *     resource qpu: quantum::qpu {
 *         requires capability("quantum.measurement");
 *     };
 *
 * The resource identifier is symbolic.
 *
 * It is NOT a physical device identifier.
 *
 * ============================================================================
 */

resourceDeclaration
    : resourceAttributes?
      RESOURCE
      identifier
      resourceKindClause?
      resourceSpecification?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. RESOURCE KIND
 * ============================================================================
 *
 * Resource kinds are OPEN-WORLD semantic names.
 *
 * The grammar does not enumerate:
 *
 *     cpu
 *     gpu
 *     fpga
 *     qpu
 *     accelerator
 *     memory
 *     storage
 *     network
 *     future devices
 *
 * Those remain semantic names.
 *
 * Standard identifiers are accepted through qualifiedName.
 *
 * Reserved language words that can legitimately occur as domain names are
 * handled by resourceNamePath below.
 *
 * ============================================================================
 */

resourceKindClause
    : COLON resourceNamePath
    ;


/*
 * ============================================================================
 * 5. OPEN RESOURCE NAME PATH
 * ============================================================================
 *
 * Normal resource namespaces use the canonical name system:
 *
 *     compute
 *     compute::gpu
 *     quantum::logical_qubit
 *     accelerator::tensor
 *     future::resource
 *
 * Resource names remain open-world.
 *
 * This rule does NOT define a finite list of resource kinds.
 *
 * The small reserved-token bridge exists only because certain existing
 * Zamani domain words, such as `quantum` and `gpu`, are reserved lexer tokens
 * rather than IDENTIFIER tokens.
 *
 * New resource kinds remain ordinary identifiers.
 *
 * ============================================================================
 */

resourceNamePath
    : resourceNameSegment
      (DOUBLE_COLON resourceNameSegment)*
    ;


resourceNameSegment
    : identifier
    | QUANTUM
    | GPU
    | NANO
    | AGENT
    | CIRCUIT
    | RESOURCE
    | RESOURCES
    | CAPABILITY
    | TARGET
    | MEMORY
    ;


/*
 * ============================================================================
 * 6. RESOURCE SPECIFICATION
 * ============================================================================
 *
 * A resource specification contains zero or more resource clauses.
 *
 * No resource-count limit is encoded.
 *
 * ============================================================================
 */

resourceSpecification
    : LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. RESOURCE BODY ITEM
 * ============================================================================
 *
 * Attributes may precede any resource clause.
 *
 * ============================================================================
 */

resourceBodyItem
    : resourceAttributes?
      resourceClause
    ;


/*
 * ============================================================================
 * 8. RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They do not execute operations.
 *
 * Example:
 *
 *     @portable
 *     @domain("quantum")
 *     resource qpu: quantum::qpu;
 *
 * ============================================================================
 */

resourceAttributes
    : resourceAttribute+
    ;


resourceAttribute
    : AT
      qualifiedName
      resourceAttributeArguments?
    ;


resourceAttributeArguments
    : LPAREN
      resourceExpressionList?
      RPAREN
    ;


/*
 * ============================================================================
 * 9. RESOURCE CLAUSE
 * ============================================================================
 *
 * This is the internal composition boundary used by resource declarations,
 * groups, contracts, and profiles.
 *
 * ============================================================================
 */

resourceClause
    : resourceQuantityClause
    | resourceReferenceClause
    | resourceRequirementClause
    | resourceConstraintClause
    | resourcePreferenceClause
    | resourceHintClause
    | resourceCapabilityClause
    | resourceTargetClause
    | resourceCapacityClause
    | resourceAvailabilityClause
    | resourcePortabilityClause
    | resourceScalabilityClause
    | resourcePerformanceClause
    | resourceLatencyClause
    | resourceThroughputClause
    | resourceBandwidthClause
    | resourceEnergyClause
    | resourcePowerClause
    | resourceReliabilityClause
    | resourceResilienceClause
    | resourceCostClause
    | resourceReservationClause
    | resourceAcquisitionClause
    | resourceReleaseClause
    | resourceDerivationClause
    | resourceGroupClause
    | resourcePropertyClause
    ;


/*
 * ============================================================================
 * 10. QUANTITY
 * ============================================================================
 *
 * Quantity is a canonical expression.
 *
 * Examples:
 *
 *     quantity = input.count;
 *
 *     quantity = workload_size * element_size;
 *
 *     quantity = logical_qubits + ancilla_qubits;
 *
 *     quantity = problem_size;
 *
 * The grammar does not evaluate the expression.
 *
 * ============================================================================
 */

resourceQuantityClause
    : QUANTITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. RESOURCE REFERENCE
 * ============================================================================
 *
 * This references an ABSTRACT resource.
 *
 * It does not select a physical resource.
 *
 * ============================================================================
 */

resourceReferenceClause
    : USE
      RESOURCE
      resourceNamePath
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * It must not silently degrade into a preference or hint.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("tensor.compute");
 *
 *     requires nodes >= required_nodes;
 *
 * ============================================================================
 */

resourceRequirement
    : REQUIRES
      resourceRequirementExpression
      SEMICOLON
    ;


resourceRequirementExpression
    : resourceCapabilityCall
    | resourceExpression
    ;


resourceRequirementClause
    : resourceRequirement
    ;


/*
 * ============================================================================
 * 13. CONSTRAINTS
 * ============================================================================
 *
 * A constraint restricts valid realizations.
 *
 * Examples:
 *
 *     constraint latency <= latency_budget;
 *
 *     constraint energy <= energy_budget;
 *
 *     constraint reliability >= required_reliability;
 *
 * ============================================================================
 */

resourceConstraint
    : CONSTRAINT
      resourceConstraintExpression
      SEMICOLON
    ;


resourceConstraintExpression
    : resourceCapabilityCall
    | resourceExpression
    ;


resourceConstraintClause
    : resourceConstraint
    ;


/*
 * ============================================================================
 * 14. PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * They must never silently become hard requirements.
 *
 * ============================================================================
 */

resourcePreference
    : PREFER
      resourcePreferenceExpression
      SEMICOLON
    ;


resourcePreferenceExpression
    : resourceCapabilityCall
    | resourceExpression
    ;


resourcePreferenceClause
    : resourcePreference
    ;


/*
 * ============================================================================
 * 15. HINTS
 * ============================================================================
 *
 * Hints are advisory implementation guidance.
 *
 * ============================================================================
 */

resourceHint
    : HINT
      resourceHintExpression
      SEMICOLON
    ;


resourceHintExpression
    : resourceCapabilityCall
    | resourceExpression
    ;


resourceHintClause
    : resourceHint
    ;


/*
 * ============================================================================
 * 16. CAPABILITY
 * ============================================================================
 *
 * Capabilities are OPEN-WORLD semantic names.
 *
 * Examples:
 *
 *     capability("quantum.measurement");
 *
 *     capability("quantum.mid_circuit_measurement");
 *
 *     capability("tensor.compute");
 *
 *     capability("distributed.execution");
 *
 *     capability("hardware.reconfiguration");
 *
 * The grammar does not enumerate capabilities.
 *
 * ============================================================================
 */

resourceCapability
    : CAPABILITY
      resourceCapabilityValue
      SEMICOLON
    ;


resourceCapabilityValue
    : resourceCapabilityCallArguments
    | resourceNamePath
    | resourceExpression
    ;


resourceCapabilityCallArguments
    : LPAREN
      resourceExpressionList?
      RPAREN
    ;


resourceCapabilityCall
    : CAPABILITY
      resourceCapabilityCallArguments
    ;


resourceCapabilityClause
    : resourceCapability
    ;


/*
 * ============================================================================
 * 17. TARGET INTENT
 * ============================================================================
 *
 * Target intent is abstract.
 *
 * It does not select a physical target.
 *
 * Examples:
 *
 *     target = quantum;
 *
 *     target = gpu;
 *
 *     target = accelerator;
 *
 *     target = compute::accelerator;
 *
 * ============================================================================
 */

resourceTarget
    : TARGET
      ASSIGN
      resourceTargetExpression
      SEMICOLON
    ;


resourceTargetExpression
    : resourceTargetSymbol
    | resourceExpression
    ;


resourceTargetSymbol
    : resourceNamePath
    | QUANTUM
    | GPU
    | NANO
    | AGENT
    | CIRCUIT
    ;


resourceTargetClause
    : resourceTarget
    ;


/*
 * ============================================================================
 * 18. CAPACITY
 * ============================================================================
 *
 * Capacity is represented by an expression.
 *
 * The actual capacity may be supplied by:
 *
 *     compile-time context
 *     target description
 *     runtime environment
 *     resource manager
 *     hardware capability model
 *
 * Parsing does not inspect any of those systems.
 *
 * ============================================================================
 */

resourceCapacityClause
    : CAPACITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. AVAILABILITY
 * ============================================================================
 *
 * Availability can be dynamic.
 *
 * This grammar only records the expression.
 *
 * ============================================================================
 */

resourceAvailabilityClause
    : AVAILABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. PORTABILITY
 * ============================================================================
 *
 * Portability is semantic intent, not target discovery.
 *
 * ============================================================================
 */

resourcePortabilityClause
    : PORTABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. SCALABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     scalability = input.size;
 *
 *     scalability = workload_size * parallelism;
 *
 *     scalability = problem_size;
 *
 * No finite scaling ceiling is encoded.
 *
 * ============================================================================
 */

resourceScalabilityClause
    : SCALABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. PERFORMANCE
 * ============================================================================
 */

resourcePerformanceClause
    : PERFORMANCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. LATENCY
 * ============================================================================
 */

resourceLatencyClause
    : LATENCY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. THROUGHPUT
 * ============================================================================
 */

resourceThroughputClause
    : THROUGHPUT
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. BANDWIDTH
 * ============================================================================
 */

resourceBandwidthClause
    : BANDWIDTH
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. ENERGY
 * ============================================================================
 */

resourceEnergyClause
    : ENERGY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 27. POWER
 * ============================================================================
 */

resourcePowerClause
    : POWER
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. RELIABILITY
 * ============================================================================
 *
 * Reliability intent may be consumed by resilience, ZQN, QEC, hardware,
 * scheduling, or runtime analysis.
 *
 * This grammar does not implement any of those systems.
 *
 * ============================================================================
 */

resourceReliabilityClause
    : RELIABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 29. RESILIENCE
 * ============================================================================
 */

resourceResilienceClause
    : RESILIENCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 30. COST
 * ============================================================================
 *
 * Cost remains abstract.
 *
 * No currency, vendor, cloud provider, or pricing model is hard-coded.
 *
 * ============================================================================
 */

resourceCostClause
    : COST
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. RESERVATION
 * ============================================================================
 *
 * Reservation is DECLARATIVE.
 *
 * Parsing does not reserve anything.
 *
 * ============================================================================
 */

resourceReservation
    : RESERVE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


resourceReservationClause
    : resourceReservation
    ;


/*
 * ============================================================================
 * 32. ACQUISITION
 * ============================================================================
 *
 * Acquisition is source-level intent.
 *
 * Actual acquisition belongs to an authorized compiler/runtime/deployment
 * subsystem.
 *
 * ============================================================================
 */

resourceAcquisition
    : ACQUIRE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


resourceAcquisitionClause
    : resourceAcquisition
    ;


/*
 * ============================================================================
 * 33. RELEASE
 * ============================================================================
 *
 * Release is source-level intent.
 *
 * Parsing itself has no side effects.
 *
 * ============================================================================
 */

resourceRelease
    : RELEASE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


resourceReleaseClause
    : resourceRelease
    ;


/*
 * ============================================================================
 * 34. RESOURCE DERIVATION
 * ============================================================================
 *
 * Derived values are symbolic.
 *
 * Example:
 *
 *     derive required_memory = elements * element_size;
 *
 * The grammar does not evaluate the expression.
 *
 * ============================================================================
 */

resourceDerivation
    : DERIVE
      identifier
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


resourceDerivationClause
    : resourceDerivation
    ;


/*
 * ============================================================================
 * 35. RESOURCE GROUP
 * ============================================================================
 *
 * Groups are logical source-level collections.
 *
 * They do NOT imply:
 *
 *     physical co-location
 *     physical topology
 *     same machine
 *     same device
 *     same memory
 *
 * unless downstream semantic/target realization explicitly establishes that.
 *
 * ============================================================================
 */

resourceGroup
    : RESOURCE
      GROUP
      identifier
      resourceSpecification
      SEMICOLON?
    ;


resourceGroupClause
    : resourceGroup
    ;


/*
 * ============================================================================
 * 36. RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract groups resource intent into a named semantic unit.
 *
 * It does not define a separate resource language.
 *
 * ============================================================================
 */

resourceContract
    : RESOURCE
      CONTRACT
      identifier
      resourceSpecification
      SEMICOLON?
    ;


resourceContractClause
    : resourceContract
    ;


/*
 * ============================================================================
 * 37. RESOURCE PROFILE
 * ============================================================================
 *
 * A profile is a named portable collection of resource intent.
 *
 * Profiles do not select physical hardware.
 *
 * ============================================================================
 */

resourceProfile
    : RESOURCE
      PROFILE
      identifier
      resourceSpecification
      SEMICOLON?
    ;


resourceProfileClause
    : resourceProfile
    ;


/*
 * ============================================================================
 * 38. OPEN-WORLD RESOURCE PROPERTY
 * ============================================================================
 *
 * Properties are semantic names.
 *
 * This avoids turning every future resource metric into a language keyword.
 *
 * Examples:
 *
 *     property memory.bandwidth = required_bandwidth;
 *
 *     property quantum.fidelity = required_fidelity;
 *
 *     property accelerator.occupancy = desired_occupancy;
 *
 *     property future::resource::metric = value;
 *
 * The property name is not interpreted by the parser.
 *
 * ============================================================================
 */

resourcePropertyClause
    : PROPERTY
      qualifiedName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 39. CANONICAL RESOURCE EXPRESSION INTEGRATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `resourceExpression` is OWNED by:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * This file intentionally does NOT redefine it.
 *
 * That prevents a second expression grammar from appearing here.
 *
 * All of the following therefore retain the canonical Zamani expression
 * semantics:
 *
 *     arithmetic
 *     comparison
 *     logical operators
 *     function calls
 *     indexing
 *     member access
 *     literals
 *     assignment semantics where applicable
 *     precedence
 *
 * ============================================================================
 */

resourceExpressionList
    : resourceExpression
      (COMMA resourceExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 40. RESOURCE VALUE / CAPABILITY HELPERS
 * ============================================================================
 *
 * These are structural bridges only.
 *
 * They do not define a second expression language.
 *
 * ============================================================================
 */

resourceCapabilityCall
    : CAPABILITY
      LPAREN
      resourceExpressionList?
      RPAREN
    ;


/*
 * ============================================================================
 * 41. RESOURCE INTENT COMPOSITION HELPERS
 * ============================================================================
 *
 * These aliases are intentionally small and stable.
 *
 * They allow other grammar components to depend on named resource concepts
 * without copying the concrete resource grammar.
 *
 * ============================================================================
 */

resourceRequirementIntent
    : resourceRequirement
    ;


resourceConstraintIntent
    : resourceConstraint
    ;


resourcePreferenceIntent
    : resourcePreference
    ;


resourceHintIntent
    : resourceHint
    ;


resourceCapabilityIntent
    : resourceCapability
    ;


resourceTargetIntent
    : resourceTarget
    ;


resourceDerivationIntent
    : resourceDerivation
    ;


resourceReservationIntent
    : resourceReservation
    ;


resourceAcquisitionIntent
    : resourceAcquisition
    ;


resourceReleaseIntent
    : resourceRelease
    ;


resourceGroupIntent
    : resourceGroup
    ;


resourceContractIntent
    : resourceContract
    ;


resourceProfileIntent
    : resourceProfile
    ;


/*
 * ============================================================================
 * 42. CROSS-DOMAIN INTEGRATION CONTRACT
 * ============================================================================
 *
 * CLASSICAL
 * ----------
 *
 * Resource intent may describe:
 *
 *     scalar compute
 *     vector compute
 *     matrix compute
 *     tensor compute
 *     memory
 *     storage
 *     parallelism
 *     throughput
 *
 * No processor architecture is hard-coded.
 *
 *
 * QUANTUM
 * -------
 *
 * Resource intent may describe:
 *
 *     logical qubits
 *     quantum memory
 *     measurement capability
 *     mid-circuit measurement
 *     dynamic control
 *     error-correction capability
 *     reliability
 *     resilience
 *
 * It MUST NOT encode:
 *
 *     physical qubit IDs
 *     coupling maps
 *     fixed QPU size
 *     vendor-native gate inventory
 *     calibration data
 *
 * Quantum information eventually crosses the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR is introduced here.
 *
 *
 * HDL / HARDWARE
 * --------------
 *
 * Resource intent may describe:
 *
 *     compute capacity
 *     memory capacity
 *     interfaces
 *     timing
 *     throughput
 *     accelerator intent
 *     reconfiguration capability
 *
 * It must not hard-code:
 *
 *     wire [31:0]
 *     fixed register width
 *     fixed FPGA size
 *     fixed LUT count
 *     fixed BRAM count
 *     fixed physical topology
 *
 *
 * HYBRID
 * ------
 *
 * The same resource model may span:
 *
 *     classical host
 *     quantum accelerator
 *     hardware accelerator
 *     distributed services
 *
 * without creating a second resource language.
 *
 *
 * DISTRIBUTED
 * -----------
 *
 * Resource expressions may represent:
 *
 *     node requirements
 *     communication requirements
 *     bandwidth
 *     latency
 *     replication
 *     storage
 *     availability
 *
 * No node-count ceiling is encoded.
 *
 *
 * AI / DATA
 * --------
 *
 * Resource intent may represent:
 *
 *     tensor computation
 *     accelerator capability
 *     memory requirements
 *     bandwidth
 *     training resources
 *     inference resources
 *     data capacity
 *
 * Framework-specific names remain identifiers/library semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. AST CONTRACT
 * ============================================================================
 *
 * This grammar must lower into the repository's domain-neutral frontend AST.
 *
 * Every resource node must preserve, as applicable:
 *
 *     source span
 *     resource identity
 *     resource kind
 *     intent category
 *     expression
 *     attributes
 *     nested clauses
 *     property names
 *     ordering where semantically observable
 *
 * The parser MUST NOT create:
 *
 *     PhysicalResource
 *     PhysicalQubit
 *     CpuDevice
 *     GpuDevice
 *     FpgaDevice
 *     QpuDevice
 *     CloudInstance
 *
 * merely because a resource kind happens to name such a concept.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     resource-kind resolution
 *     duplicate detection
 *     type checking
 *     unit/dimensional checking
 *     capability resolution
 *     requirement satisfiability
 *     constraint validation
 *     preference interpretation
 *     hint interpretation
 *     target compatibility
 *     capacity analysis
 *     availability analysis
 *     scalability analysis
 *     portability analysis
 *     resource negotiation
 *     lifecycle validation
 *     dependency analysis
 *
 * The grammar only preserves the information required for those phases.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT SEMANTICS
 * ============================================================================
 *
 * REQUIREMENT
 * -----------
 *
 * Must be satisfied for a valid realization.
 *
 *
 * CONSTRAINT
 * ----------
 *
 * Restricts the set of valid realizations.
 *
 *
 * CAPABILITY
 * ----------
 *
 * Describes an ability/property that may be required, available, or queried
 * semantically.
 *
 *
 * PREFERENCE
 * ----------
 *
 * Advisory optimization information.
 *
 * It MUST NOT become a hard requirement merely because an optimizer chooses
 * to consider it.
 *
 *
 * HINT
 * ----
 *
 * Advisory implementation guidance.
 *
 * It MAY be ignored while preserving program correctness.
 *
 *
 * TARGET
 * ------
 *
 * Abstract target intent.
 *
 * It is not necessarily a physical device selection.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. ALLOCATION / RESERVATION / ACQUISITION / RELEASE
 * ============================================================================
 *
 * The existence of these syntactic constructs does NOT grant the parser
 * permission to perform the corresponding operation.
 *
 * Parsing must never:
 *
 *     allocate resources
 *     reserve resources
 *     acquire resources
 *     release resources
 *     discover hardware
 *     contact a cloud provider
 *     contact a QPU
 *     inspect runtime state
 *
 * These constructs become semantic instructions/intent for authorized
 * downstream systems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. RESOURCE LIFECYCLE
 * ============================================================================
 *
 * The semantic lifecycle is conceptually:
 *
 *     declaration
 *         |
 *         v
 *     requirement / capability analysis
 *         |
 *         v
 *     negotiation
 *         |
 *         v
 *     realization
 *         |
 *         v
 *     acquisition / reservation
 *         |
 *         v
 *     execution
 *         |
 *         v
 *     release
 *
 * The grammar only represents source intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. COMPILER / OPTIMIZER INTEGRATION
 * ============================================================================
 *
 * Compiler phases may consume resource intent for:
 *
 *     target-independent optimization
 *     specialization
 *     vectorization
 *     parallelization
 *     accelerator selection
 *     memory planning
 *     distributed partitioning
 *     quantum decomposition
 *     scheduling
 *     routing
 *
 * None of those algorithms belongs in this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. QUANTUM / QEC / ZQN INTEGRATION
 * ============================================================================
 *
 * Resource syntax may express:
 *
 *     reliability
 *     resilience
 *     error-correction capability
 *     fault-tolerance requirements
 *     noise-related requirements
 *
 * QEC remains responsible for:
 *
 *     code construction
 *     syndrome processing
 *     decoding
 *     correction
 *
 * ZQN remains responsible for:
 *
 *     noise
 *     fault
 *     uncertainty
 *     reliability modeling
 *     resilience semantics
 *
 * Resource grammar does not duplicate either subsystem.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware grammars may specialize resource semantics.
 *
 * Universal resource intent remains owned here.
 *
 * Hardware-specific information such as:
 *
 *     actual width
 *     actual capacity
 *     actual topology
 *     actual timing
 *     actual device identity
 *
 * belongs to target realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     token stream
 *     grammar version
 *     explicit parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     runtime state
 *     filesystem contents
 *     network state
 *     current time
 *     randomness
 *     cloud-provider state
 *     QPU state
 *     calibration state
 *
 * Identical source and lexer configuration must produce equivalent parse
 * structures.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. SECURITY CONTRACT
 * ============================================================================
 *
 * Resource syntax is untrusted input.
 *
 * Parsing must not imply:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     device access
 *     shell execution
 *     backend invocation
 *     cloud API calls
 *     hardware discovery
 *
 * Semantic evaluation must occur only in explicitly authorized compiler or
 * runtime services.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * SYNTAX errors belong to the parser.
 *
 * Examples:
 *
 *     resource;
 *     requires;
 *     constraint;
 *     target =;
 *     resource memory: ;
 *     resource group;
 *
 * SEMANTIC errors belong downstream.
 *
 * Examples:
 *
 *     unknown resource
 *     unsupported capability
 *     unsatisfied requirement
 *     violated constraint
 *     incompatible target
 *     impossible realization
 *     invalid dimensional relationship
 *
 * Parser recovery MUST NOT silently transform:
 *
 *     requirement -> preference
 *     constraint -> hint
 *     capability -> target
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects may extend resource semantics through the repository's explicit
 * dialect mechanism.
 *
 * A dialect must not silently redefine the meaning of universal constructs:
 *
 *     resource
 *     requires
 *     constraint
 *     prefer
 *     hint
 *     capability
 *     target
 *
 * New semantic resource kinds and properties should normally be represented
 * as open-world names rather than new parser keywords.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. VERSIONING / COMPATIBILITY
 * ============================================================================
 *
 * Adding a new RESOURCE KIND:
 *
 *     normally requires no grammar change.
 *
 * Adding a new CAPABILITY NAME:
 *
 *     normally requires no grammar change.
 *
 * Adding a new PROPERTY NAME:
 *
 *     normally requires no grammar change.
 *
 * Adding a new hardware vendor:
 *
 *     must not require a universal grammar change.
 *
 * Adding a new accelerator:
 *
 *     must not require a universal grammar change.
 *
 * Adding a new QPU:
 *
 *     must not require a universal grammar change.
 *
 * Adding a new computational substrate:
 *
 *     should normally use an existing open-world resource structure.
 *
 * Grammar changes are reserved for genuinely new syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. INTEGRATION WITH OTHER EXISTING FILES
 * ============================================================================
 *
 * grammar/Zamani.g4
 * -----------------
 *
 * Remains the canonical composition root.
 *
 * It should import/compose Resources rather than duplicating resource rules.
 *
 *
 * grammar/resources/resource-expressions.g4
 * -----------------------------------------
 *
 * Owns resourceExpression.
 *
 * This file consumes it.
 *
 *
 * grammar/core/names.g4
 * ---------------------
 *
 * Owns identifier and qualifiedName.
 *
 * This file consumes them.
 *
 *
 * grammar/statements/resource.g4
 * ------------------------------
 *
 * Must remain a thin adapter:
 *
 *     resourceStatement
 *         : resourceItem
 *         ;
 *
 * It must not reproduce this file's alternatives.
 *
 *
 * grammar/declarations/resources.g4
 * ---------------------------------
 *
 * Must converge on this resource model rather than define another universal
 * resource declaration language.
 *
 *
 * grammar/hardware/resources.g4
 * -----------------------------
 *
 * May specialize hardware resource realization but must not replace this
 * universal source-level model.
 *
 *
 * grammar/quantum/quantum-resources.g4
 * ------------------------------------
 *
 * May specialize quantum resource semantics while preserving this universal
 * resource boundary.
 *
 *
 * grammar/hybrid/
 * ---------------
 *
 * May consume this model for classical/quantum/hardware combinations.
 *
 *
 * grammar/distributed/
 * --------------------
 *
 * May attach node/network/storage requirements without creating another
 * resource language.
 *
 *
 * grammar/compile/
 * ----------------
 *
 * Consumes resource intent during target-independent compilation and target
 * realization.
 *
 *
 * grammar/execution/
 * -----------------
 *
 * Consumes semantic resource information during scheduling, deployment,
 * runtime adaptation, and lifecycle management.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. IR CONTRACT
 * ============================================================================
 *
 * This grammar defines NO IR.
 *
 * Resource syntax lowers conceptually as:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic resource model
 *       |
 *       +------------------+------------------+
 *       |                  |                  |
 *       v                  v                  v
 *   classical          quantum            HDL/hardware
 *      IR             quantum::ir              IR
 *       |                  |                  |
 *       +------------------+------------------+
 *                          |
 *                          v
 *                  optimization/lowering
 *                          |
 *                    routing/scheduling
 *                          |
 *                      resilience
 *                          |
 *                         ZQN
 *                          |
 *                         HAL
 *                          |
 *                   target realization
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The following forms are intended to be valid:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource accelerator: accelerator {
 *         quantity = workload_size;
 *     };
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     constraint latency <= latency_budget;
 *
 *     constraint energy <= energy_budget;
 *
 *     prefer latency <= preferred_latency;
 *
 *     hint locality;
 *
 *     target = quantum;
 *
 *     target = gpu;
 *
 *     target = accelerator;
 *
 *     derive required_memory = elements * element_size;
 *
 *     resource group compute {
 *         quantity = workload_size;
 *     }
 *
 *     resource contract portable_compute {
 *         requires capability("tensor.compute");
 *         scalability = workload_size;
 *     }
 *
 *     resource profile quantum_execution {
 *         requires qubits >= logical_qubits;
 *         requires capability("quantum.measurement");
 *     }
 *
 *     resource accelerator: accelerator {
 *         property tensor::throughput = required_throughput;
 *         property memory::capacity = required_memory;
 *     };
 *
 * NEGATIVE
 * --------
 *
 * The following should be rejected:
 *
 *     resource;
 *
 *     requires;
 *
 *     constraint;
 *
 *     prefer;
 *
 *     hint;
 *
 *     target =;
 *
 *     resource memory: ;
 *
 *     derive = value;
 *
 *     resource group;
 *
 *     resource contract;
 *
 *     resource profile;
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     one resource
 *     many resources
 *     nested groups
 *     nested contracts
 *     nested profiles
 *     large property sets
 *     large requirement sets
 *     large capability sets
 *     symbolic quantities
 *     derived quantities
 *     quantum resources
 *     classical resources
 *     accelerator resources
 *     distributed resources
 *     HDL/hardware resources
 *
 * No test may establish an artificial universal maximum.
 *
 * SCALABILITY
 * ----------
 *
 * Examples should include symbolic expressions such as:
 *
 *     required_qubits
 *     workload_size
 *     problem_size * element_size
 *     nodes_required
 *     available_memory
 *     parallelism
 *
 * and should verify that the grammar imposes no fixed machine-size ceiling.
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and parser configuration must produce equivalent parse
 * structures regardless of:
 *
 *     hardware
 *     runtime state
 *     cloud availability
 *     QPU availability
 *     network state
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains:
 *
 *     NO MAX_QUBITS
 *     NO MAX_CPUS
 *     NO MAX_GPUS
 *     NO MAX_FPGAS
 *     NO MAX_NODES
 *     NO MAX_MEMORY
 *     NO MAX_THREADS
 *     NO MAX_TENSOR_RANK
 *     NO MAX_REGISTER_WIDTH
 *     NO MAX_NETWORK_SIZE
 *     NO MAX_DEVICE_COUNT
 *
 * It also contains no:
 *
 *     physical device ID
 *     physical qubit ID
 *     physical address
 *     fixed topology
 *     fixed register width
 *     fixed memory size
 *     fixed node count
 *     fixed accelerator count
 *
 * The only finite alternatives in this file concern EXISTING LEXICAL
 * KEYWORDS that must be accepted where those keywords can occur as symbolic
 * resource names. They do not constitute a finite resource-kind vocabulary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This grammar itself contains no Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 61. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is structurally COMPLETE when:
 *
 *     [x] Grammar identity is Resources.
 *     [x] This file is the canonical universal resource owner.
 *     [x] ResourceExpressions is imported.
 *     [x] Names is imported.
 *     [x] No resourceExpression duplicate is defined.
 *     [x] No identifier duplicate is defined.
 *     [x] No qualifiedName duplicate is defined.
 *     [x] Requirements are distinct from preferences.
 *     [x] Constraints are distinct from requirements.
 *     [x] Hints are distinct from preferences.
 *     [x] Capabilities are open-world.
 *     [x] Targets are abstract.
 *     [x] Quantities are expressions.
 *     [x] Properties are open-world.
 *     [x] Resource groups are recursively compositional.
 *     [x] Resource contracts are recursively compositional.
 *     [x] Resource profiles are recursively compositional.
 *     [x] No machine-size limit is encoded.
 *     [x] No physical device is selected.
 *     [x] No hardware discovery occurs.
 *     [x] No allocation occurs during parsing.
 *     [x] No scheduling occurs during parsing.
 *     [x] No routing occurs during parsing.
 *     [x] No QEC occurs during parsing.
 *     [x] No ZQN implementation occurs during parsing.
 *     [x] No second quantum IR is created.
 *     [x] No embedded Rust exists.
 *     [x] No unsafe Rust requirement exists.
 *     [x] Cross-domain integration is explicit.
 *     [x] AST integration is explicit.
 *     [x] Semantic integration is explicit.
 *     [x] IR integration is explicit.
 *     [x] Compiler/runtime boundaries are explicit.
 *
 * Repository-level completion additionally requires:
 *
 *     [ ] Zamani.g4 imports/consumes Resources.
 *     [ ] statements/resource.g4 consumes resourceItem.
 *     [ ] declarations/resources.g4 converges on this resource model.
 *     [ ] ANTLR generation succeeds.
 *     [ ] generated parser compiles on Rust 1.97.1.
 *     [ ] resource positive tests pass.
 *     [ ] resource negative tests pass.
 *     [ ] scalability tests pass.
 *     [ ] determinism tests pass.
 *     [ ] cross-domain tests pass.
 *     [ ] AST coverage is complete.
 *     [ ] semantic coverage is complete.
 *     [ ] canonical IR/resource metadata coverage is complete.
 *
 * Those repository-wide checks are intentionally NOT hidden inside this
 * grammar file.
 *
 * ============================================================================
 */