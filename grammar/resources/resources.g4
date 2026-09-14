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
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no embedded unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL UNIVERSAL RESOURCE-INTENT SYNTAX of Zamani.
 *
 * Resource intent is deliberately independent of:
 *
 *     - physical machines;
 *     - hardware inventories;
 *     - device identifiers;
 *     - topology;
 *     - scheduling algorithms;
 *     - routing algorithms;
 *     - optimization algorithms;
 *     - runtime allocation;
 *     - backend implementation;
 *     - simulation;
 *     - calibration;
 *     - QEC algorithms;
 *     - ZQN noise models;
 *     - canonical quantum IR.
 *
 * The grammar expresses semantic intent such as:
 *
 *     resource
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     target
 *     capacity
 *     availability
 *     portability
 *     scalability
 *     performance
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     cost
 *     reservation
 *     acquisition
 *     release
 *     derivation
 *     grouping
 *     properties
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani's resource model participates in:
 *
 *     Program_Once
 *          |
 *          v
 *     Stable source semantics
 *          |
 *          v
 *     Compile_Once
 *          |
 *          v
 *     Abstract executable/semantic representation
 *          |
 *          v
 *     Target realization
 *          |
 *          v
 *     Available capabilities/resources
 *          |
 *          v
 *     Run_Everywhere_Anywhere_Forever
 *
 * "Forever" means that the semantic model remains versioned and extensible.
 *
 * It does NOT claim that one physical binary can execute on every future
 * architecture without target-specific lowering.
 *
 * ============================================================================
 * SCALABILITY RULE
 * ============================================================================
 *
 * THIS FILE CONTAINS NO FIXED MACHINE LIMITS.
 *
 * It MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_CPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *     MAX_DEVICES
 *     MAX_SHOTS
 *     device IDs
 *     physical addresses
 *     fixed topology dimensions
 *
 * Quantities are expressions.
 *
 * Example:
 *
 *     requires resource memory >= workload_memory;
 *
 * The quantity is evaluated by semantic analysis / compilation / runtime
 * context. The grammar itself imposes no finite resource ceiling.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - resource declarations;
 *     - resource references;
 *     - resource kinds;
 *     - resource quantities;
 *     - resource requirements;
 *     - resource constraints;
 *     - resource preferences;
 *     - resource hints;
 *     - capability intent;
 *     - target intent;
 *     - capacity expressions;
 *     - availability expressions;
 *     - portability intent;
 *     - scalability intent;
 *     - performance intent;
 *     - latency intent;
 *     - throughput intent;
 *     - bandwidth intent;
 *     - energy intent;
 *     - power intent;
 *     - reliability intent;
 *     - resilience intent;
 *     - cost intent;
 *     - reservation intent;
 *     - acquisition intent;
 *     - release intent;
 *     - resource derivation;
 *     - resource grouping;
 *     - resource contracts;
 *     - resource profiles;
 *     - extensible resource properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - numeric literal syntax;
 *     - string literal syntax;
 *     - general expression precedence;
 *     - general types;
 *     - module resolution;
 *     - hardware discovery;
 *     - physical allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - canonical classical IR;
 *     - canonical quantum::ir;
 *     - runtime implementation.
 *
 * ============================================================================
 * INTEGRATION PRINCIPLE
 * ============================================================================
 *
 * Domain grammars may specialize this grammar:
 *
 *     quantum/quantum-resources.g4
 *     hardware/resources.g4
 *     hybrid/hybrid-resources.g4
 *     distributed/...
 *     ai/...
 *
 * They MUST NOT redefine universal resource semantics.
 *
 * They may introduce domain-specific:
 *
 *     resource kinds;
 *     properties;
 *     capability names;
 *     target classes;
 *     domain-specific intent.
 *
 * Those forms must normalize downstream into the canonical resource semantic
 * model.
 *
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This parser grammar expects the canonical Zamani lexer vocabulary to expose
 * the resource keywords used below. Those lexer tokens belong in the
 * repository's canonical lexer, not in this file.
 *
 * ============================================================================
 */

parser grammar ZamaniResourcesParser;

options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A resource section may contain any number of resource declarations or
 * resource-intent statements.
 *
 * No fixed cardinality is imposed.
 * ============================================================================
 */

resources
    : resourceItem*
    ;


/*
 * ============================================================================
 * 2. RESOURCE ITEM
 * ============================================================================
 */

resourceItem
    : resourceDeclaration
    | resourceContract
    | resourceProfile
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
    ;


/*
 * ============================================================================
 * 3. RESOURCE DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     resource memory;
 *
 *     resource memory: memory {
 *         quantity = workload_memory;
 *     };
 *
 *     resource compute {
 *         quantity = required_parallelism;
 *     };
 *
 * A resource name is symbolic.
 *
 * It is NOT automatically:
 *
 *     - a device identifier;
 *     - an address;
 *     - a physical core;
 *     - a physical qubit;
 *     - a backend identifier.
 * ============================================================================
 */

resourceDeclaration
    : resourceAttributes?
      RESOURCE
      resourceName
      resourceType?
      resourceSpecification?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. RESOURCE NAME
 * ============================================================================
 */

resourceName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 5. RESOURCE TYPE
 * ============================================================================
 *
 * Resource kinds remain open-ended.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum.logical_qubit
 *     quantum.logical_qubit.resource
 *     hardware.fpga
 *     network.bandwidth
 *     storage.capacity
 *
 * The grammar does not enumerate every future resource kind.
 * ============================================================================
 */

resourceType
    : qualifiedResourceName
    ;


qualifiedResourceName
    : resourceName
      (SCOPE_SEPARATOR resourceName)*
    ;


/*
 * ============================================================================
 * 6. RESOURCE SPECIFICATION
 * ============================================================================
 */

resourceSpecification
    : LBRACE
      resourceBodyItem*
      RBRACE
    ;


resourceBodyItem
    : resourceAttributes?
      resourceClause
    ;


/*
 * ============================================================================
 * 7. RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are syntactic metadata attached to resource constructs.
 *
 * They do not themselves define runtime behavior.
 * ============================================================================
 */

resourceAttributes
    : resourceAttribute+
    ;


resourceAttribute
    : ATTRIBUTE_START
      resourceAttributeName
      resourceAttributeArguments?
      ATTRIBUTE_END
    ;


resourceAttributeName
    : qualifiedResourceName
    ;


resourceAttributeArguments
    : LPAREN
      resourceArgumentList?
      RPAREN
    ;


resourceArgumentList
    : resourceArgument
      (COMMA resourceArgument)*
    ;


resourceArgument
    : resourceExpression
    ;


/*
 * ============================================================================
 * 8. RESOURCE CLAUSES
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
 * 9. QUANTITY
 * ============================================================================
 *
 * Quantity is an expression.
 *
 * This is fundamental to scalability.
 *
 * Valid conceptual forms include:
 *
 *     quantity = problem_size;
 *     quantity = workload_size * parallelism;
 *     quantity = input.count;
 *     quantity = available_capacity;
 *
 * No maximum value is imposed here.
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
 * 10. RESOURCE REFERENCE
 * ============================================================================
 *
 * References an abstract resource without selecting a physical realization.
 * ============================================================================
 */

resourceReferenceClause
    : RESOURCE
      qualifiedResourceName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * If no realization can satisfy it, compilation/execution must not silently
 * reinterpret it as a preference.
 * ============================================================================
 */

resourceRequirement
    : REQUIRES
      resourceRequirementBody
      SEMICOLON
    ;


resourceRequirementClause
    : REQUIREMENT
      resourceRequirementBody
      SEMICOLON
    ;


resourceRequirementBody
    : RESOURCE
      resourceExpression
    | resourceExpression
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT
 * ============================================================================
 *
 * A constraint must be respected by the selected realization.
 * ============================================================================
 */

resourceConstraint
    : CONSTRAINT
      resourceConstraintBody
      SEMICOLON
    ;


resourceConstraintClause
    : CONSTRAINT
      resourceConstraintBody
      SEMICOLON
    ;


resourceConstraintBody
    : RESOURCE
      resourceExpression
    | resourceExpression
    ;


/*
 * ============================================================================
 * 13. PREFERENCE
 * ============================================================================
 *
 * A preference is advisory.
 *
 * It MUST NOT silently become a requirement.
 * ============================================================================
 */

resourcePreference
    : PREFERENCE
      resourcePreferenceBody
      SEMICOLON
    ;


resourcePreferenceClause
    : PREFERENCE
      resourcePreferenceBody
      SEMICOLON
    ;


resourcePreferenceBody
    : RESOURCE
      resourceExpression
    | resourceExpression
    ;


/*
 * ============================================================================
 * 14. HINT
 * ============================================================================
 *
 * A hint is advisory information that an implementation may ignore.
 * ============================================================================
 */

resourceHint
    : HINT
      resourceHintBody
      SEMICOLON
    ;


resourceHintClause
    : HINT
      resourceHintBody
      SEMICOLON
    ;


resourceHintBody
    : RESOURCE
      resourceExpression
    | resourceExpression
    ;


/*
 * ============================================================================
 * 15. CAPABILITY
 * ============================================================================
 *
 * A capability represents a property expected from or associated with the
 * execution environment.
 *
 * Capability discovery is downstream.
 * ============================================================================
 */

resourceCapability
    : CAPABILITY
      resourceCapabilityBody
      SEMICOLON
    ;


resourceCapabilityClause
    : CAPABILITY
      resourceCapabilityBody
      SEMICOLON
    ;


resourceCapabilityBody
    : RESOURCE
      resourceExpression
    | resourceExpression
    ;


/*
 * ============================================================================
 * 16. TARGET
 * ============================================================================
 *
 * Targets are abstract target classes.
 *
 * Examples:
 *
 *     target = cpu;
 *     target = quantum;
 *     target = accelerator;
 *     target = heterogeneous;
 *
 * A target does not imply a particular device.
 * ============================================================================
 */

resourceTarget
    : TARGET
      ASSIGN
      qualifiedResourceName
      SEMICOLON
    ;


resourceTargetClause
    : TARGET
      ASSIGN
      qualifiedResourceName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. CAPACITY
 * ============================================================================
 *
 * Capacity is contextual information.
 *
 * It does not define a source-language maximum.
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
 * 18. AVAILABILITY
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
 * 19. PORTABILITY
 * ============================================================================
 *
 * Portability is an intent, not a device choice.
 * ============================================================================
 */

resourcePortabilityClause
    : PORTABLE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. SCALABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     scalability = input.size;
 *     scalability = workload;
 *     scalability = workload * parallelism;
 *
 * The grammar itself imposes no finite scale.
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
 * 21. PERFORMANCE
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
 * 22. LATENCY
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
 * 23. THROUGHPUT
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
 * 24. BANDWIDTH
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
 * 25. ENERGY
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
 * 26. POWER
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
 * 27. RELIABILITY
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
 * 28. RESILIENCE
 * ============================================================================
 *
 * This is resource-level resilience intent.
 *
 * It does NOT define the resilience subsystem or recovery algorithms.
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
 * 29. COST
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
 * 30. RESERVATION
 * ============================================================================
 *
 * Reservation is intent only.
 *
 * Actual reservation is a downstream execution/resource-management operation.
 * ============================================================================
 */

resourceReservation
    : RESERVE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


resourceReservationClause
    : RESERVE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. ACQUISITION
 * ============================================================================
 */

resourceAcquisition
    : ACQUIRE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


resourceAcquisitionClause
    : ACQUIRE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 32. RELEASE
 * ============================================================================
 */

resourceRelease
    : RELEASE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


resourceReleaseClause
    : RELEASE
      RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. RESOURCE DERIVATION
 * ============================================================================
 *
 * A derived resource expression is symbolic.
 *
 * Example:
 *
 *     derive required_memory = workload_size * element_size;
 *
 * No evaluation occurs in the parser.
 * ============================================================================
 */

resourceDerivation
    : DERIVE
      resourceName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


resourceDerivationClause
    : DERIVE
      resourceName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. RESOURCE GROUP
 * ============================================================================
 *
 * Groups have arbitrary cardinality.
 *
 * No fixed resource count is encoded.
 * ============================================================================
 */

resourceGroupClause
    : RESOURCE
      GROUP
      resourceName
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 35. RESOURCE CONTRACT
 * ============================================================================
 *
 * Contracts collect resource intent.
 *
 * A contract does not allocate resources.
 * ============================================================================
 */

resourceContract
    : RESOURCE
      CONTRACT
      resourceName
      resourceContractSpecification
    ;


resourceContractSpecification
    : LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 36. RESOURCE PROFILE
 * ============================================================================
 *
 * A profile is a named collection of resource intent.
 *
 * Profiles remain abstract and portable.
 * ============================================================================
 */

resourceProfile
    : RESOURCE
      PROFILE
      resourceName
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 37. RESOURCE PROPERTY
 * ============================================================================
 *
 * Open properties allow future domains to extend resource descriptions
 * without requiring this universal grammar to enumerate every future
 * computing technology.
 *
 * Semantic analysis determines whether the property is:
 *
 *     standard
 *     dialect-defined
 *     target-defined
 *     experimental
 *     unsupported
 *     invalid
 * ============================================================================
 */

resourcePropertyClause
    : PROPERTY
      resourcePropertyName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


resourcePropertyName
    : qualifiedResourceName
    ;


/*
 * ============================================================================
 * 38. RESOURCE EXPRESSION
 * ============================================================================
 *
 * Resource expressions MUST reuse the canonical Zamani expression grammar.
 *
 * This file deliberately does not redefine arithmetic, comparison, logical,
 * indexing, member access, function calls, literals, or operator precedence.
 *
 * `expression` is therefore the integration boundary with the canonical
 * expression grammar owned elsewhere in grammar/.
 *
 * If the repository splits expression parsing into a dedicated parser grammar,
 * this rule is replaced by the imported canonical expression rule.
 * ============================================================================
 */

resourceExpression
    : expression
    ;


/*
 * ============================================================================
 * 39. SEMANTIC COMPARISON
 * ============================================================================
 *
 * The following semantic relationships are intentionally represented by the
 * canonical expression grammar rather than duplicated here:
 *
 *     <
 *     <=
 *     ==
 *     !=
 *     >=
 *     >
 *
 * Therefore constructs such as:
 *
 *     requires resource memory >= workload_memory;
 *
 * are parsed through:
 *
 *     resourceRequirement
 *         |
 *         +-- resourceExpression
 *                 |
 *                 +-- canonical expression grammar
 *
 * This avoids creating a second expression language.
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. ABSTRACT RESOURCE QUALIFICATION
 * ============================================================================
 *
 * Qualified names are symbolic:
 *
 *     quantum.logical_qubit
 *     hardware.fpga
 *     network.bandwidth
 *     accelerator.tensor
 *
 * They do not imply:
 *
 *     device identity
 *     topology
 *     physical address
 *     hardware inventory
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/quantum/quantum-resources.g4 may specialize resource kinds such as:
 *
 *     quantum.logical_qubit
 *     quantum.physical_qubit
 *     quantum.memory
 *     quantum.measurement
 *     quantum.error_correction
 *
 * It MUST NOT redefine:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     target
 *     quantity
 *
 * Quantum semantics ultimately lower toward the repository's canonical
 * quantum::ir boundary.
 *
 * This grammar does not create quantum IR.
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/hardware/resources.g4 may specialize:
 *
 *     cpu
 *     gpu
 *     fpga
 *     asic
 *     accelerator
 *     memory
 *     interconnect
 *
 * Hardware discovery, calibration, topology discovery, and device selection
 * remain downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. HYBRID INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/hybrid/hybrid-resources.g4 may compose:
 *
 *     classical
 *     quantum
 *     accelerator
 *     hardware
 *     distributed
 *
 * resource intent.
 *
 * It MUST normalize into the same universal resource semantic model.
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. DISTRIBUTED INTEGRATION CONTRACT
 * ============================================================================
 *
 * Distributed grammars may introduce abstract:
 *
 *     node
 *     service
 *     communication
 *     replication
 *     placement
 *
 * resource kinds.
 *
 * This grammar does not prescribe the number of nodes or topology.
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. AI / ACCELERATOR INTEGRATION CONTRACT
 * ============================================================================
 *
 * AI grammars may specialize:
 *
 *     tensor resources
 *     accelerator capabilities
 *     memory bandwidth
 *     model execution resources
 *     training/inference resources
 *
 * No accelerator count is fixed here.
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. HDL INTEGRATION CONTRACT
 * ============================================================================
 *
 * HDL grammars may specialize:
 *
 *     logic resources
 *     register resources
 *     memory resources
 *     pipeline resources
 *     clock resources
 *     hardware-interface resources
 *
 * Hardware semantics remain separate from physical implementation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. SCHEDULING INTEGRATION CONTRACT
 * ============================================================================
 *
 * Scheduling consumes resource intent.
 *
 * This grammar does NOT:
 *
 *     schedule operations;
 *     assign times;
 *     create dependency DAGs;
 *     perform resource allocation.
 *
 * Scheduler policy belongs to scheduling/.
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. ROUTING INTEGRATION CONTRACT
 * ============================================================================
 *
 * Routing may consume:
 *
 *     target intent
 *     resource requirements
 *     capability requirements
 *     topology-related semantic constraints
 *
 * Routing itself remains outside this grammar.
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. OPTIMIZATION INTEGRATION CONTRACT
 * ============================================================================
 *
 * Optimization may consume:
 *
 *     preferences
 *     hints
 *     performance objectives
 *     energy objectives
 *     cost objectives
 *
 * A preference MUST NOT silently become a requirement.
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. HARDWARE HAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware HAL supplies concrete capabilities and observations.
 *
 * This grammar does not discover hardware.
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. RESOURCE MANAGEMENT INTEGRATION CONTRACT
 * ============================================================================
 *
 * Resource management evaluates:
 *
 *     requirements
 *     constraints
 *     reservations
 *     acquisitions
 *     releases
 *     capacity
 *     availability
 *
 * It owns actual resource accounting/allocation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. RUNTIME INTEGRATION CONTRACT
 * ============================================================================
 *
 * Runtime may consume normalized resource intent.
 *
 * Runtime owns:
 *
 *     allocation
 *     dispatch
 *     availability
 *     reservations
 *     execution state
 *
 * Parser execution MUST remain side-effect free.
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. QEC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Resource syntax may express abstract requirements for:
 *
 *     error correction resources
 *     logical-qubit resources
 *     reliability
 *     resilience
 *
 * QEC owns the actual error-correction algorithms and state.
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. ZQN INTEGRATION CONTRACT
 * ============================================================================
 *
 * Resource syntax may express:
 *
 *     reliability
 *     resilience
 *     fault-tolerance requirements
 *
 * ZQN owns:
 *
 *     noise
 *     faults
 *     fault classification
 *     correlations
 *     leakage
 *     loss
 *     erasure
 *
 * This grammar does not define noise semantics.
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. RESILIENCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Resource intent may be consumed by the resilience subsystem when deciding
 * whether an execution environment remains acceptable.
 *
 * This grammar does not implement:
 *
 *     retry
 *     rollback
 *     remapping
 *     rerouting
 *     rescheduling
 *     recompilation
 *     backend switching
 *     quarantine
 *     recovery
 *
 * Those belong to resilience/.
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. SEMANTIC NORMALIZATION CONTRACT
 * ============================================================================
 *
 * Every valid resource construct should normalize into a semantic model
 * conceptually equivalent to:
 *
 *     ResourceId
 *     ResourceKind
 *     ResourceExpression
 *     Requirement
 *     Constraint
 *     Preference
 *     Hint
 *     CapabilityRequirement
 *     TargetIntent
 *     CapacityExpression
 *     AvailabilityExpression
 *     PortabilityIntent
 *     ScalabilityIntent
 *     PerformanceIntent
 *     LatencyIntent
 *     ThroughputIntent
 *     BandwidthIntent
 *     EnergyIntent
 *     PowerIntent
 *     ReliabilityIntent
 *     ResilienceIntent
 *     CostIntent
 *     ReservationIntent
 *     AcquisitionIntent
 *     ReleaseIntent
 *     ResourceDerivation
 *     ResourceGroup
 *     ResourceContract
 *     ResourceProfile
 *     ResourceProperty
 *
 * These are semantic concepts.
 *
 * They are NOT Rust structures defined by this grammar.
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     MUST NOT inspect hardware.
 *     MUST NOT query runtime state.
 *     MUST NOT access the network.
 *     MUST NOT access the filesystem.
 *     MUST NOT allocate resources.
 *     MUST NOT invoke a backend.
 *     MUST NOT execute arbitrary code.
 *
 * The same token stream must produce the same parse result.
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. SECURITY
 * ============================================================================
 *
 * Resource declarations are declarative.
 *
 * Parsing MUST NOT cause:
 *
 *     hardware access;
 *     network access;
 *     filesystem discovery;
 *     process execution;
 *     resource reservation;
 *     resource allocation.
 *
 * All such operations require explicitly authorized downstream components.
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. SCALABILITY
 * ============================================================================
 *
 * This grammar places no source-level limits on:
 *
 *     number of resources;
 *     number of resource clauses;
 *     number of resource groups;
 *     number of resource properties;
 *     expression size;
 *     quantum resources;
 *     classical resources;
 *     accelerator resources;
 *     distributed resources.
 *
 * Actual limits are imposed only by:
 *
 *     compiler implementation;
 *     available memory;
 *     execution environment;
 *     target capabilities;
 *     runtime policy;
 *     explicit program semantics.
 *
 * Those limits MUST NOT be represented as arbitrary grammar constants.
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_*
 *     fixed qubit counts
 *     fixed core counts
 *     fixed thread counts
 *     fixed GPU counts
 *     fixed FPGA counts
 *     fixed node counts
 *     fixed device IDs
 *     physical addresses
 *     topology dimensions
 *     fixed memory capacities
 *
 * Resource quantities MUST remain expressions.
 * ============================================================================
 */


/*
 * ============================================================================
 * 61. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     malformed resource declaration;
 *     malformed resource clause;
 *     malformed requirement;
 *     malformed constraint;
 *     malformed preference;
 *     malformed hint;
 *     malformed capability;
 *     malformed target;
 *     malformed quantity;
 *     malformed property;
 *     malformed group;
 *     malformed contract;
 *     malformed profile.
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown resource;
 *     unsupported resource;
 *     unavailable resource;
 *     unsatisfied requirement;
 *     violated constraint;
 *     unsupported capability;
 *     incompatible target;
 *     impossible realization.
 *
 * These categories MUST NOT be conflated.
 * ============================================================================
 */


/*
 * ============================================================================
 * 62. COMPATIBILITY
 * ============================================================================
 *
 * New resource kinds should normally be introduced as qualified symbolic
 * names rather than by modifying this universal grammar.
 *
 * Example:
 *
 *     future.accelerator
 *     quantum.next_generation_resource
 *     photonic.processor
 *     neuromorphic.core
 *
 * This allows future domains to extend Zamani without repeatedly changing
 * the universal resource syntax.
 *
 * Language-version changes remain governed by the grammar versioning and
 * compatibility policy.
 * ============================================================================
 */


/*
 * ============================================================================
 * 63. ROUND-TRIP REQUIREMENT
 * ============================================================================
 *
 * Where the repository provides a canonical Zamani formatter:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     formatter
 *       |
 *       v
 *     parser
 *
 * resource semantics must remain equivalent.
 *
 * Formatting differences must not alter resource intent.
 * ============================================================================
 */


/*
 * ============================================================================
 * 64. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     resource declarations
 *     resource references
 *     symbolic quantities
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capabilities
 *     targets
 *     capacity
 *     availability
 *     portability
 *     scalability
 *     performance
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     cost
 *     reservations
 *     acquisition
 *     release
 *     derivations
 *     groups
 *     contracts
 *     profiles
 *     properties
 *
 * Negative:
 *
 *     missing resource name
 *     malformed resource type
 *     malformed expression
 *     missing assignment
 *     malformed group
 *     malformed contract
 *     malformed profile
 *     malformed property
 *     malformed qualified name
 *
 * Boundary:
 *
 *     zero resources
 *     one resource
 *     many resources
 *     deeply nested groups
 *     deeply nested expressions
 *     very large symbolic quantities
 *
 * Cross-domain:
 *
 *     classical + resources
 *     quantum + resources
 *     hybrid + resources
 *     HDL + resources
 *     hardware + resources
 *     distributed + resources
 *     AI + resources
 *     networking + resources
 *
 * Scalability:
 *
 *     no artificial machine limit may be inferred from this grammar.
 *
 * Determinism:
 *
 *     identical token streams produce identical parse trees.
 * ============================================================================
 */


/*
 * ============================================================================
 * 65. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] The canonical Zamani lexer exports every token referenced here.
 *
 * [ ] The canonical expression rule is available through the parser
 *     integration boundary.
 *
 * [ ] No duplicate expression grammar exists here.
 *
 * [ ] No duplicate type grammar exists here.
 *
 * [ ] No physical machine limit exists here.
 *
 * [ ] No device identity is implicitly selected here.
 *
 * [ ] Requirement != constraint != preference != hint.
 *
 * [ ] Capability != target.
 *
 * [ ] Capacity != availability.
 *
 * [ ] Reservation != acquisition != release.
 *
 * [ ] Parser semantics remain side-effect free.
 *
 * [ ] Quantum resource syntax can specialize this model without duplicating
 *     universal resource semantics.
 *
 * [ ] Hardware resource syntax can specialize this model without duplicating
 *     universal resource semantics.
 *
 * [ ] Hybrid resource syntax can compose this model.
 *
 * [ ] Resource intent can normalize into the repository's canonical semantic
 *     representation.
 *
 * [ ] Resource intent can be consumed by compilation.
 *
 * [ ] Resource intent can be consumed by scheduling.
 *
 * [ ] Resource intent can be consumed by routing.
 *
 * [ ] Resource intent can be consumed by hardware HAL.
 *
 * [ ] Resource intent can be consumed by resource management.
 *
 * [ ] Resource intent can be consumed by runtime.
 *
 * [ ] Resource intent can be associated with quantum::ir without creating a
 *     second quantum IR.
 *
 * [ ] QEC remains the owner of QEC algorithms.
 *
 * [ ] ZQN remains the owner of noise/fault semantics.
 *
 * [ ] Resilience remains the owner of recovery decisions.
 *
 * [ ] No filesystem, network, hardware, or runtime access occurs while
 *     parsing.
 *
 * [ ] Rust 1.97 / 1.97.1 integration remains safe Rust.
 *
 * [ ] No unsafe Rust is required by Zamani's grammar integration.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Round-trip tests exist where formatter infrastructure exists.
 *
 * ============================================================================
 */