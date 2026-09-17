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
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * Canonical composition grammar for SOURCE-LEVEL RESOURCE INTENT.
 *
 * This file owns the universal syntactic composition of:
 *
 *     resource declarations
 *     resource references
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capabilities
 *     targets
 *     quantities
 *     capacities
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
 *     reservation intent
 *     acquisition intent
 *     release intent
 *     derivation
 *     resource groups
 *     resource contracts
 *     resource profiles
 *     resource properties
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This grammar describes PORTABLE PROGRAM INTENT.
 *
 * It does NOT:
 *
 *     - discover hardware;
 *     - allocate hardware;
 *     - select physical devices;
 *     - select physical qubits;
 *     - choose CPU cores;
 *     - choose GPUs;
 *     - choose FPGAs;
 *     - choose network nodes;
 *     - choose memory banks;
 *     - construct topology;
 *     - schedule operations;
 *     - route operations;
 *     - optimize programs;
 *     - calibrate devices;
 *     - execute programs;
 *     - implement QEC;
 *     - implement ZQN;
 *     - implement HAL;
 *     - construct classical IR;
 *     - construct quantum::ir;
 *     - construct HDL/hardware IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Resource syntax is intentionally open-world.
 *
 * A program describes:
 *
 *     WHAT it requires
 *     WHAT it permits
 *     WHAT it prefers
 *     WHAT capabilities it needs
 *     WHAT constraints must hold
 *     WHAT scaling behavior is intended
 *
 * It does not have to describe:
 *
 *     WHICH physical CPU
 *     WHICH physical GPU
 *     WHICH physical QPU
 *     WHICH physical qubit
 *     WHICH physical FPGA
 *     WHICH physical memory bank
 *     WHICH physical node
 *
 * Concrete realization is downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level resource limit is encoded here.
 *
 * In particular there is NO:
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
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_RESOURCE_COUNT
 *     MAX_RESOURCE_GROUPS
 *     MAX_PROPERTIES
 *     MAX_REQUIREMENTS
 *     MAX_CAPABILITIES
 *
 * Repetition uses `*`, `+`, and recursive structures.
 *
 * Practical limits belong to:
 *
 *     compiler resource policy
 *     implementation limits
 *     available memory
 *     target capabilities
 *     runtime policy
 *     deployment resources
 *
 * ============================================================================
 * SEMANTIC DISTINCTION
 * ============================================================================
 *
 * The following concepts MUST remain distinct:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     target
 *     capacity
 *     availability
 *
 * For example:
 *
 *     requires memory >= required_memory;
 *
 * is not equivalent to:
 *
 *     prefer memory >= required_memory;
 *
 * and neither is equivalent to:
 *
 *     target = memory;
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Resource quantities and predicates use the canonical Zamani expression
 * hierarchy through ResourceExpressions.
 *
 * This file MUST NOT redefine:
 *
 *     arithmetic
 *     logical operators
 *     comparisons
 *     unary operators
 *     calls
 *     indexing
 *     member access
 *     literals
 *     assignment
 *     expression precedence
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Name syntax comes from grammar/core/names.g4.
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *
 * ============================================================================
 * DOWNSTREAM PIPELINE
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     Zamani parser
 *        |
 *        v
 *     domain-neutral AST
 *        |
 *        v
 *     structural validation
 *        |
 *        v
 *     semantic resource model
 *        |
 *        +--> ResourceManager
 *        +--> capability resolution
 *        +--> target analysis
 *        +--> compilation context
 *        +--> scheduling
 *        +--> routing
 *        +--> optimization
 *        +--> runtime
 *        |
 *        +--> quantum semantics -> quantum::ir
 *        |
 *        v
 *     target realization
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
 * 1. PUBLIC RESOURCE COMPOSITION ENTRY
 * ============================================================================
 *
 * This is the resource-directory composition boundary.
 *
 * It may be consumed by:
 *
 *     Zamani.g4
 *     parser composition
 *     declaration grammar
 *     statement grammar
 *     domain grammars
 *
 * It does not itself require a resource section to contain any item.
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
 *     resource memory;
 *
 *     resource memory: memory;
 *
 *     resource compute: compute {
 *         quantity = workload_size;
 *         scalability = workload_size;
 *     };
 *
 * The resource name is symbolic.
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
 * The kind is a qualified symbolic name.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum::logical_qubit
 *     hardware::fpga
 *     network::bandwidth
 *     storage::capacity
 *
 * The grammar does not enumerate resource kinds.
 */
resourceKindClause
    : COLON qualifiedName
    ;


/*
 * ============================================================================
 * 5. RESOURCE SPECIFICATION
 * ============================================================================
 */

resourceSpecification
    : LBRACE
      resourceBodyItem*
      RBRACE
    ;


resourceBodyItem
    : resourceAttribute*
      resourceClause
    ;


/*
 * ============================================================================
 * 6. RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes remain generic.
 *
 * They are metadata, not executable resource operations.
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
 * 7. RESOURCE CLAUSE
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
 * 8. QUANTITY
 * ============================================================================
 *
 * Quantity is an expression.
 *
 * Examples:
 *
 *     quantity = input.size;
 *     quantity = workload_size * parallelism;
 *     quantity = required_qubits;
 *
 * No finite bound is encoded.
 */
resourceQuantityClause
    : QUANTITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. RESOURCE REFERENCE
 * ============================================================================
 *
 * A reference names an abstract resource.
 *
 * It does not select a physical resource.
 */
resourceReferenceClause
    : USE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. REQUIREMENTS
 * ============================================================================
 *
 * `requires` is already a canonical Zamani keyword.
 *
 * Requirement semantics are mandatory and are resolved downstream.
 */
resourceRequirement
    : REQUIRES
      resourceExpression
      SEMICOLON
    ;


resourceRequirementClause
    : resourceRequirement
    ;


/*
 * ============================================================================
 * 11. CONSTRAINTS
 * ============================================================================
 */

resourceConstraint
    : CONSTRAINT
      resourceExpression
      SEMICOLON
    ;


resourceConstraintClause
    : resourceConstraint
    ;


/*
 * ============================================================================
 * 12. PREFERENCES
 * ============================================================================
 *
 * A preference is advisory.
 *
 * It MUST NOT silently become a requirement.
 */
resourcePreference
    : PREFER
      resourceExpression
      SEMICOLON
    ;


resourcePreferenceClause
    : resourcePreference
    ;


/*
 * ============================================================================
 * 13. HINTS
 * ============================================================================
 *
 * A hint is advisory and may be ignored by an implementation.
 */
resourceHint
    : HINT
      resourceExpression
      SEMICOLON
    ;


resourceHintClause
    : resourceHint
    ;


/*
 * ============================================================================
 * 14. CAPABILITIES
 * ============================================================================
 *
 * Capability names are open-world qualified names.
 *
 * The parser does not determine whether a capability exists.
 */
resourceCapability
    : CAPABILITY
      resourceExpression
      SEMICOLON
    ;


resourceCapabilityClause
    : resourceCapability
    ;


/*
 * ============================================================================
 * 15. TARGET INTENT
 * ============================================================================
 *
 * Targets are abstract target classes or semantic target expressions.
 *
 * They do not identify physical devices.
 */
resourceTarget
    : TARGET
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


resourceTargetClause
    : resourceTarget
    ;


/*
 * ============================================================================
 * 16. CAPACITY
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
 * 17. AVAILABILITY
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
 * 18. PORTABILITY
 * ============================================================================
 *
 * Portability is semantic intent.
 */
resourcePortabilityClause
    : PORTABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. SCALABILITY
 * ============================================================================
 *
 * Scalability is expressed symbolically.
 *
 * Example:
 *
 *     scalability = input.size;
 */
resourceScalabilityClause
    : SCALABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. PERFORMANCE
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
 * 21. LATENCY
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
 * 22. THROUGHPUT
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
 * 23. BANDWIDTH
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
 * 24. ENERGY
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
 * 25. POWER
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
 * 26. RELIABILITY
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
 * 27. RESILIENCE
 * ============================================================================
 *
 * This expresses desired resilience characteristics.
 *
 * It does not implement the resilience subsystem.
 */
resourceResilienceClause
    : RESILIENCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. COST
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
 * 29. RESERVATION
 * ============================================================================
 *
 * Reservation syntax is declarative.
 *
 * Parsing MUST NOT reserve anything.
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
 * 30. ACQUISITION
 * ============================================================================
 *
 * Acquisition is an intent/declaration.
 *
 * Actual acquisition is downstream.
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
 * 31. RELEASE
 * ============================================================================
 *
 * Release syntax does not perform release during parsing.
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
 * 32. DERIVED RESOURCES
 * ============================================================================
 *
 * Derived quantities are symbolic expressions.
 *
 * Example:
 *
 *     derive required_memory = workload_size * element_size;
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
 * 33. RESOURCE GROUP
 * ============================================================================
 *
 * Groups are unbounded structurally.
 *
 * They do not imply physical co-location.
 */
resourceGroup
    : RESOURCE
      GROUP
      identifier
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


resourceGroupClause
    : resourceGroup
    ;


/*
 * ============================================================================
 * 34. RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract groups resource intent into a named semantic unit.
 */
resourceContract
    : RESOURCE
      CONTRACT
      identifier
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 35. RESOURCE PROFILE
 * ============================================================================
 *
 * A profile is a named portable collection of resource intent.
 */
resourceProfile
    : RESOURCE
      PROFILE
      identifier
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 36. RESOURCE PROPERTY
 * ============================================================================
 *
 * Open-world properties allow future resource domains without requiring a
 * grammar release for every new metric or capability.
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
 * 37. RESOURCE EXPRESSION
 * ============================================================================
 *
 * ResourceExpressions owns this composition boundary.
 *
 * It delegates to the canonical `expression` rule.
 */
resourceExpression
    : expression
    ;


resourceExpressionList
    : resourceExpression
      (COMMA resourceExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 38. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Every resource construct must lower conceptually to a domain-neutral
 * semantic resource model.
 *
 * Examples:
 *
 *     resource declaration
 *         -> ResourceDeclaration
 *
 *     requires ...
 *         -> ResourceRequirement
 *
 *     constraint ...
 *         -> ResourceConstraint
 *
 *     prefer ...
 *         -> ResourcePreference
 *
 *     hint ...
 *         -> ResourceHint
 *
 *     capability ...
 *         -> CapabilityRequirement
 *
 *     target = ...
 *         -> TargetIntent
 *
 *     quantity = ...
 *         -> ResourceQuantity
 *
 *     capacity = ...
 *         -> CapacityExpression
 *
 *     availability = ...
 *         -> AvailabilityExpression
 *
 *     derive ...
 *         -> ResourceDerivation
 *
 * The exact Rust semantic structures belong outside grammar/.
 *
 * ============================================================================
 * 39. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum resource syntax may express:
 *
 *     logical qubit requirements
 *     quantum memory requirements
 *     measurement capability
 *     error-correction capability
 *     reliability
 *     resilience
 *     quantum execution capabilities
 *
 * It MUST NOT encode:
 *
 *     physical qubit IDs
 *     coupling maps
 *     fixed QPU sizes
 *     native gate inventories
 *     calibration values
 *
 * Quantum meaning ultimately crosses:
 *
 *     quantum::ir
 *
 * The resource grammar does not create another quantum IR.
 *
 * ============================================================================
 * 40. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * The same resource syntax can describe:
 *
 *     scalar computation
 *     vector computation
 *     matrix computation
 *     tensor computation
 *     CPU computation
 *     accelerator computation
 *     memory
 *     storage
 *     parallelism
 *
 * without introducing machine-specific grammar.
 *
 * ============================================================================
 * 41. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware/HDL grammars may specialize the RESOURCE KIND and PROPERTY values,
 * but universal requirement/constraint/preference/capability semantics remain
 * owned here.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * 42. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed grammars may use resource expressions for:
 *
 *     node requirements
 *     communication requirements
 *     bandwidth
 *     latency
 *     replication
 *     storage
 *     service capacity
 *
 * No node count or topology limit is encoded.
 *
 * ============================================================================
 * 43. AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI/data grammars may express:
 *
 *     tensor resources
 *     accelerator capabilities
 *     memory bandwidth
 *     training resources
 *     inference resources
 *     data capacity
 *     throughput
 *
 * Framework names remain semantic/library identifiers rather than universal
 * resource keywords.
 *
 * ============================================================================
 * 44. RESOURCE MANAGER INTEGRATION
 * ============================================================================
 *
 * ResourceManager is downstream.
 *
 * It may consume:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     targets
 *     reservations
 *     acquisition intent
 *     release intent
 *     capacity
 *     availability
 *
 * Parsing itself MUST remain side-effect free.
 *
 * ============================================================================
 * 45. SCHEDULING / ROUTING / OPTIMIZATION
 * ============================================================================
 *
 * Scheduling consumes resource intent but owns scheduling.
 *
 * Routing consumes resource/capability/topology information but owns routing.
 *
 * Optimization may consume preferences and hints but owns transformations.
 *
 * A preference MUST NOT become a mandatory requirement merely because an
 * optimizer chooses to use it.
 *
 * ============================================================================
 * 46. QEC / ZQN / RESILIENCE
 * ============================================================================
 *
 * Resource syntax may express:
 *
 *     reliability
 *     resilience
 *     error-correction capability
 *     fault-tolerance requirements
 *
 * QEC owns correction mechanisms.
 *
 * ZQN owns fault/noise semantics.
 *
 * Resilience owns recovery/orchestration.
 *
 * This grammar only expresses intent.
 *
 * ============================================================================
 * 47. DETERMINISM / SECURITY
 * ============================================================================
 *
 * Parsing MUST NOT:
 *
 *     access hardware
 *     access the network
 *     access the filesystem
 *     allocate resources
 *     reserve resources
 *     execute programs
 *     invoke backends
 *     inspect runtime state
 *     inspect calibration state
 *
 * Identical source/token streams must produce identical parse structures.
 *
 * ============================================================================
 * 48. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_*
 *     fixed resource counts
 *     fixed qubit counts
 *     fixed CPU/core/thread counts
 *     fixed GPU/FPGA/QPU counts
 *     fixed memory capacity
 *     fixed node count
 *     physical device IDs
 *     physical addresses
 *     fixed topology dimensions
 *
 * Literal program values remain legal.
 *
 * Example:
 *
 *     let required_qubits = 1024;
 *
 * is program data.
 *
 * A grammar rule such as:
 *
 *     resourceCount : ... <= 1024;
 *
 * is prohibited.
 *
 * ============================================================================
 * 49. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser diagnostics own malformed syntax:
 *
 *     missing resource name
 *     malformed resource kind
 *     malformed expression
 *     missing assignment
 *     malformed group
 *     malformed contract
 *     malformed profile
 *     malformed property
 *
 * Semantic/resource diagnostics own:
 *
 *     unknown resource
 *     unavailable resource
 *     unsatisfied requirement
 *     violated constraint
 *     unsupported capability
 *     incompatible target
 *     impossible realization
 *
 * These must remain separate.
 *
 * ============================================================================
 * 50. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical ZamaniLexer vocabulary is consumed;
 *     [x] canonical Names grammar is reused;
 *     [x] canonical ResourceExpressions grammar is reused;
 *     [x] no expression grammar is duplicated;
 *     [x] resource semantics remain open-world;
 *     [x] requirements differ from preferences;
 *     [x] hints differ from requirements;
 *     [x] capabilities differ from targets;
 *     [x] target intent remains abstract;
 *     [x] resource quantities are expressions;
 *     [x] no physical resource is selected by parsing;
 *     [x] no machine-size limits exist;
 *     [x] no quantum IR is duplicated;
 *     [x] parser execution is side-effect free;
 *     [x] safe Rust remains the implementation requirement;
 *     [x] Rust 1.97 / 1.97.1 compatibility is preserved;
 *     [x] cross-domain integration is defined;
 *     [x] semantic/IR/runtime integration is defined.
 *
 * ============================================================================
 */