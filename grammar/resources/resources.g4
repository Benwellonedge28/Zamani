/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/resources.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the UNIVERSAL, DOMAIN-INDEPENDENT SOURCE-LEVEL RESOURCE
 * INTENT MODEL for Zamani.
 *
 * It is the common grammar boundary for:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerators
 *     embedded systems
 *     distributed systems
 *     networking
 *     storage
 *     memory
 *     AI/ML
 *     HPC
 *     future computing domains
 *
 * The grammar describes WHAT a program requires, permits, prefers, hints,
 * targets, derives, or observes about computational resources.
 *
 * It does NOT describe HOW those resources are physically realized.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     program semantics             resource intent
 *                                          |
 *                  +-----------------------+-----------------------+
 *                  |           |           |           |           |
 *                  v           v           v           v           v
 *             requirement  constraint  preference   hint      capability
 *                  |           |           |           |           |
 *                  +-----------+-----------+-----------+-----------+
 *                                          |
 *                                          v
 *                              resource semantic model
 *                                          |
 *                                          v
 *                              compilation context
 *                                          |
 *                         +----------------+----------------+
 *                         |                |                |
 *                         v                v                v
 *                     hardware         scheduler         runtime
 *                       HAL             routing       resource manager
 *                         |                |                |
 *                         +----------------+----------------+
 *                                          |
 *                                          v
 *                                    execution
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - universal resource declarations;
 *   - abstract resource references;
 *   - resource quantities;
 *   - requirements;
 *   - constraints;
 *   - preferences;
 *   - hints;
 *   - capability requirements/references;
 *   - abstract targets;
 *   - resource capacity expressions;
 *   - availability expressions;
 *   - portability intent;
 *   - scalability intent;
 *   - performance intent;
 *   - latency intent;
 *   - throughput intent;
 *   - bandwidth intent;
 *   - energy intent;
 *   - power intent;
 *   - reliability intent;
 *   - resilience intent;
 *   - cost intent;
 *   - resource groups;
 *   - resource relationships;
 *   - resource derivations;
 *   - resource contracts;
 *   - resource scopes;
 *   - abstract reservation intent;
 *   - abstract acquisition intent;
 *   - abstract release intent;
 *   - resource policies;
 *   - extensible resource properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - literals;
 *   - general expressions;
 *   - general types;
 *   - classical IR;
 *   - quantum::ir;
 *   - quantum gates;
 *   - QEC algorithms;
 *   - ZQN noise models;
 *   - physical qubit identifiers;
 *   - physical device identifiers;
 *   - hardware discovery;
 *   - calibration;
 *   - routing;
 *   - scheduling algorithms;
 *   - optimization algorithms;
 *   - runtime allocation implementation;
 *   - deployment implementation;
 *   - simulator implementation;
 *   - backend implementation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Resource syntax MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A source program describes semantic resource intent.
 *
 * It must not silently encode:
 *
 *     a machine size;
 *     a device number;
 *     a physical address;
 *     a topology;
 *     a processor count;
 *     a qubit count;
 *     a memory limit;
 *     a fixed accelerator count.
 *
 * Resource quantities therefore use expressions.
 *
 * For example:
 *
 *     requires resource memory >= workload_size;
 *
 * means:
 *
 *     the execution must have enough abstract memory for workload_size.
 *
 * It does NOT mean:
 *
 *     allocate a fixed amount of memory;
 *     use a particular machine;
 *     use a particular memory address.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally contains no fixed machine limits.
 *
 * There is no:
 *
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *     MAX_SHOTS
 *
 * Repetition is represented by ANTLR's:
 *
 *     *
 *     +
 *
 * operators.
 *
 * Quantities are expressions rather than grammar constants.
 *
 * Therefore resource requirements may scale with:
 *
 *     input size
 *     problem size
 *     program parameters
 *     workload
 *     available capacity
 *     execution context
 *     compilation context
 *
 * Actual feasibility is decided downstream.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * RESOURCE
 *     Abstract computational resource.
 *
 * REQUIREMENT
 *     Mandatory semantic condition.
 *
 * CONSTRAINT
 *     Condition that every valid realization must respect.
 *
 * PREFERENCE
 *     Advisory optimization objective.
 *
 * HINT
 *     Advisory information that may be ignored.
 *
 * CAPABILITY
 *     Property provided by an execution environment.
 *
 * TARGET
 *     Abstract execution/compilation category.
 *
 * CAPACITY
 *     Amount of resource available from a context.
 *
 * AVAILABILITY
 *     Current or prospective resource availability.
 *
 * RESERVATION
 *     Intent to obtain resources.
 *
 * ACQUISITION
 *     Intent to acquire resources.
 *
 * RELEASE
 *     Intent to release resources.
 *
 * NONE of these concepts are interchangeable.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION CONTRACT
 * ============================================================================
 *
 * Domain-specific resource grammars such as:
 *
 *     grammar/quantum/quantum-resources.g4
 *     grammar/hardware/resources.g4
 *     grammar/hybrid/hybrid-resources.g4
 *
 * MUST NOT redefine the universal semantics represented here when the same
 * concept already exists here.
 *
 * They may:
 *
 *     - specialize domain syntax;
 *     - add domain-specific resource properties;
 *     - introduce domain-specific resource kinds;
 *     - attach resource intent to domain constructs.
 *
 * Semantic analysis MUST normalize those forms into the canonical resource
 * semantic representation.
 *
 * This file MUST NOT become a second IR.
 *
 * ============================================================================
 */

parser grammar ZamaniResourcesParser;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A resource section contains zero or more resource declarations, contracts,
 * requirements, constraints, preferences, hints, or groups.
 *
 * This rule is intentionally self-contained so the resource grammar can be
 * imported by a higher-level parser.
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
    | resourceRequirement
    | resourceConstraint
    | resourcePreference
    | resourceHint
    | resourceCapability
    | resourceTarget
    | resourceGroup
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
 * Example:
 *
 *     resource memory {
 *         quantity = workload_memory;
 *     }
 *
 * The identifier is symbolic.
 *
 * It is NOT:
 *
 *     a device identifier;
 *     a memory address;
 *     a physical core;
 *     a physical qubit.
 * ============================================================================
 */

resourceDeclaration
    : resourceAttribute*
      K_RESOURCE
      resourceName
      resourceType?
      resourceSpecification?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 4. RESOURCE NAME
 * ============================================================================
 *
 * Resource names remain symbolic and are resolved semantically.
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
 * Resource types are open through qualified names.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum.logical_qubit
 *     hardware.fpga
 *     network.bandwidth
 *
 * No finite resource catalogue is embedded in this grammar.
 * ============================================================================
 */

resourceType
    : qualifiedResourceName
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


/*
 * ============================================================================
 * 7. RESOURCE BODY
 * ============================================================================
 */

resourceBodyItem
    : resourceAttribute*
      resourceClause
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
    | resourceGroupClause
    | resourceDerivationClause
    | resourcePropertyClause
    ;


/*
 * ============================================================================
 * 9. RESOURCE QUANTITY
 * ============================================================================
 *
 * Quantities are expressions.
 *
 * Examples:
 *
 *     quantity = n;
 *     quantity = problem_size;
 *     quantity = workload_size * lanes;
 *     quantity = available_capacity;
 *
 * There is deliberately no numeric limit here.
 * ============================================================================
 */

resourceQuantityClause
    : K_QUANTITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. RESOURCE REFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     resource memory;
 *
 * or:
 *
 *     resource quantum.logical_qubits;
 * ============================================================================
 */

resourceReferenceClause
    : K_RESOURCE
      qualifiedResourceName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Examples:
 *
 *     requires resource memory >= required_memory;
 *     requires resource quantum.logical_qubits >= logical_qubits;
 *
 * The grammar does not determine feasibility.
 * ============================================================================
 */

resourceRequirementClause
    : K_REQUIRES
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT
 * ============================================================================
 */

resourceConstraintClause
    : K_CONSTRAINT
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. PREFERENCE
 * ============================================================================
 *
 * Preferences MUST remain distinguishable from requirements.
 * ============================================================================
 */

resourcePreferenceClause
    : K_PREFERENCE
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. HINT
 * ============================================================================
 *
 * Hints are advisory and may be ignored by a valid implementation.
 * ============================================================================
 */

resourceHintClause
    : K_HINT
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. CAPABILITY
 * ============================================================================
 *
 * Capability expressions describe required or referenced capabilities.
 *
 * Discovery is outside the grammar.
 * ============================================================================
 */

resourceCapabilityClause
    : K_CAPABILITY
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. TARGET
 * ============================================================================
 *
 * Target is an abstract semantic category.
 *
 * Examples:
 *
 *     target = cpu;
 *     target = accelerator;
 *     target = quantum;
 *     target = heterogeneous;
 *
 * A concrete backend/device is selected downstream.
 * ============================================================================
 */

resourceTargetClause
    : K_TARGET
      ASSIGN
      qualifiedResourceName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. CAPACITY
 * ============================================================================
 *
 * Capacity describes an implementation/context-provided quantity.
 *
 * It does not impose a source-level maximum.
 * ============================================================================
 */

resourceCapacityClause
    : K_CAPACITY
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
    : K_AVAILABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. PORTABILITY
 * ============================================================================
 *
 * Portability is semantic intent.
 *
 * It does not select a particular backend.
 * ============================================================================
 */

resourcePortabilityClause
    : K_PORTABLE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. SCALABILITY
 * ============================================================================
 *
 * Scaling is represented by an expression.
 *
 * Examples:
 *
 *     scalability = problem_size;
 *     scalability = problem_size * lanes;
 *     scalability = symbolic_scaling;
 *
 * The grammar does not impose a finite scalability model.
 * ============================================================================
 */

resourceScalabilityClause
    : K_SCALABILITY
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
    : K_PERFORMANCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. LATENCY
 * ============================================================================
 *
 * Timing realization belongs to scheduling/runtime.
 * ============================================================================
 */

resourceLatencyClause
    : K_LATENCY
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
    : K_THROUGHPUT
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
    : K_BANDWIDTH
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
    : K_ENERGY
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
    : K_POWER
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
    : K_RELIABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. RESILIENCE
 * ============================================================================
 *
 * Resilience expresses an abstract execution property.
 *
 * Resilience implementation remains owned by the resilience subsystem.
 * ============================================================================
 */

resourceResilienceClause
    : K_RESILIENCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 29. COST
 * ============================================================================
 *
 * Cost is abstract intent.
 *
 * The grammar does not prescribe a currency, provider, pricing model, or
 * deployment platform.
 * ============================================================================
 */

resourceCostClause
    : K_COST
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
 * Actual reservation belongs to resource management/deployment/runtime.
 * ============================================================================
 */

resourceReservationClause
    : K_RESERVE
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. ACQUISITION
 * ============================================================================
 */

resourceAcquisitionClause
    : K_ACQUIRE
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 32. RELEASE
 * ============================================================================
 */

resourceReleaseClause
    : K_RELEASE
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. RESOURCE GROUP
 * ============================================================================
 *
 * Groups have arbitrary cardinality.
 * ============================================================================
 */

resourceGroupClause
    : K_RESOURCE
      K_GROUP
      resourceName
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 34. RESOURCE DERIVATION
 * ============================================================================
 *
 * Example:
 *
 *     resource required_memory = workload_size * element_size;
 *
 * This is symbolic until semantic evaluation.
 * ============================================================================
 */

resourceDerivationClause
    : K_RESOURCE
      resourceName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 35. RESOURCE PROPERTY
 * ============================================================================
 *
 * Open property syntax is essential for long-term extensibility.
 *
 * A future domain must not necessarily modify this grammar merely because
 * it introduces a new resource property.
 *
 * Semantic analysis determines whether a property is:
 *
 *     standard
 *     dialect-defined
 *     target-defined
 *     experimental
 *     invalid
 * ============================================================================
 */

resourcePropertyClause
    : resourcePropertyName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


resourcePropertyName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 36. RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract groups resource semantics.
 *
 * Example:
 *
 *     resource contract scalable_execution {
 *         requires resource compute >= workload;
 *         scalability = workload;
 *         portability = true;
 *     }
 *
 * A contract describes intent.
 *
 * It does not itself allocate resources.
 * ============================================================================
 */

resourceContract
    : K_RESOURCE
      K_CONTRACT
      resourceName
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 37. DIRECT REQUIREMENT
 * ============================================================================
 *
 * Convenience form for higher-level grammars.
 * ============================================================================
 */

resourceRequirement
    : K_REQUIRES
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 38. DIRECT CONSTRAINT
 * ============================================================================
 */

resourceConstraint
    : K_CONSTRAINT
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 39. DIRECT PREFERENCE
 * ============================================================================
 */

resourcePreference
    : K_PREFERENCE
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 40. DIRECT HINT
 * ============================================================================
 */

resourceHint
    : K_HINT
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 41. DIRECT CAPABILITY
 * ============================================================================
 */

resourceCapability
    : K_CAPABILITY
      K_RESOURCE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 42. RESOURCE TARGET
 * ============================================================================
 */

resourceTarget
    : K_TARGET
      ASSIGN
      qualifiedResourceName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 43. RESOURCE EXPRESSION
 * ============================================================================
 *
 * CRITICAL INTEGRATION RULE:
 *
 * This grammar MUST NOT define a second expression language.
 *
 * `resourceExpression` is a semantic boundary around the repository's
 * canonical expression grammar.
 *
 * During integration, this rule MUST be mapped to the canonical expression
 * rule supplied by the parser architecture.
 *
 * If the canonical rule is named differently in the final parser, this rule
 * must be updated exactly once as part of parser integration rather than
 * creating another expression implementation.
 * ============================================================================
 */

resourceExpression
    : expression
    ;


/*
 * ============================================================================
 * 44. QUALIFIED RESOURCE NAME
 * ============================================================================
 *
 * Resource namespaces remain open.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     quantum.logical_qubits
 *     hardware.fpga.resources
 *     network.bandwidth
 * ============================================================================
 */

qualifiedResourceName
    : resourceName
      (
          DOT resourceName
      )*
    ;


/*
 * ============================================================================
 * 45. RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are syntactic metadata.
 *
 * Semantic validation belongs outside the grammar.
 * ============================================================================
 */

resourceAttribute
    : AT resourceAttributeName
      (
          LPAREN resourceAttributeArguments? RPAREN
      )?
    ;


resourceAttributeName
    : IDENTIFIER
    ;


resourceAttributeArguments
    : resourceAttributeArgument
      (
          COMMA resourceAttributeArgument
      )*
    ;


resourceAttributeArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | qualifiedResourceName
    | resourceExpression
    ;


/*
 * ============================================================================
 * 46. RESOURCE RELATIONSHIPS
 * ============================================================================
 *
 * Relationships express semantic associations without defining physical
 * topology.
 * ============================================================================
 */

resourceRelationship
    : K_RESOURCE
      resourceName
      relationshipOperator
      resourceName
      SEMICOLON
    ;


relationshipOperator
    : K_REQUIRES
    | K_DEPENDS
    | K_SUPPORTS
    | K_PROVIDES
    | K_ASSOCIATED
    ;


/*
 * ============================================================================
 * 47. RESOURCE POLICY
 * ============================================================================
 *
 * Policy is abstract semantic intent.
 *
 * The actual policy engine is outside the grammar.
 * ============================================================================
 */

resourcePolicy
    : K_POLICY
      resourceName
      LBRACE
      resourcePolicyItem*
      RBRACE
    ;


resourcePolicyItem
    : resourceRequirement
    | resourceConstraint
    | resourcePreference
    | resourceHint
    | resourcePropertyClause
    ;


/*
 * ============================================================================
 * 48. RESOURCE PROFILE
 * ============================================================================
 *
 * A profile groups reusable resource intent.
 * ============================================================================
 */

resourceProfile
    : K_RESOURCE
      K_PROFILE
      resourceName
      LBRACE
      resourceBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 49. RESOURCE IMPORT
 * ============================================================================
 *
 * The module system remains the owner of general imports.
 *
 * This rule is intentionally absent from the universal resource grammar.
 *
 * Resource definitions should therefore use the repository's canonical module
 * and import grammar instead of creating a second module system.
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. RESOURCE VALIDATION BOUNDARY
 * ============================================================================
 *
 * Parsing MUST NOT determine:
 *
 *     whether a resource exists;
 *     whether a resource is available;
 *     whether a quantity is sufficient;
 *     whether a machine can satisfy a requirement;
 *     whether a quantum device has enough qubits;
 *     whether a topology supports an operation;
 *     whether a schedule is feasible;
 *     whether a backend exists;
 *     whether calibration is sufficient;
 *     whether QEC can satisfy an error budget;
 *     whether ZQN permits an execution;
 *     whether runtime allocation succeeds.
 *
 * These belong to semantic analysis and downstream systems.
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. SEMANTIC NORMALIZATION CONTRACT
 * ============================================================================
 *
 * All resource syntax must normalize into a common semantic resource model.
 *
 * Conceptual normalized categories:
 *
 *     ResourceId
 *     ResourceKind
 *     QuantityExpression
 *     Requirement
 *     Constraint
 *     Preference
 *     Hint
 *     CapabilityRequirement
 *     TargetIntent
 *     CapacityObservation
 *     AvailabilityObservation
 *     PortabilityIntent
 *     ScalabilityIntent
 *     PerformanceIntent
 *     ReservationIntent
 *     AcquisitionIntent
 *     ReleaseIntent
 *
 * These are semantic concepts, NOT grammar-defined Rust structures.
 *
 * The grammar must remain independent from Rust implementation details.
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. DOMAIN INTEGRATION CONTRACT
 * ============================================================================
 *
 * QUANTUM
 *
 * grammar/quantum/quantum-resources.g4 may specialize:
 *
 *     logical qubits
 *     physical qubits
 *     quantum memory
 *     measurement capability
 *     error-correction resources
 *
 * but must not redefine universal requirement/constraint/preference semantics.
 *
 *
 * HARDWARE
 *
 * grammar/hardware/resources.g4 may specialize:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     memory
 *     accelerators
 *
 * but must not redefine the universal resource model.
 *
 *
 * HYBRID
 *
 * grammar/hybrid/hybrid-resources.g4 may combine resource intents from
 * multiple computational domains.
 *
 *
 * SCHEDULING
 *
 * Scheduling consumes resource requirements and constraints.
 *
 * This grammar does not implement scheduling.
 *
 *
 * ROUTING
 *
 * Routing consumes target/resource/capability information.
 *
 * This grammar does not implement routing.
 *
 *
 * OPTIMIZATION
 *
 * Optimization may consume preferences and hints.
 *
 * A preference MUST NOT silently become a requirement.
 *
 *
 * HARDWARE HAL
 *
 * Hardware capability information is supplied downstream.
 *
 * This grammar does not discover hardware.
 *
 *
 * RUNTIME
 *
 * Runtime may evaluate availability, reservation, acquisition, and release.
 *
 *
 * QUANTUM::IR
 *
 * Resource intent may be attached to or associated with canonical quantum
 * semantics during lowering.
 *
 * This grammar MUST NOT create another quantum IR.
 *
 *
 * QEC
 *
 * QEC owns error-correction algorithms and their semantic machinery.
 *
 * This grammar may express abstract resource intent required by a program,
 * but it does not implement QEC.
 *
 *
 * ZQN
 *
 * ZQN owns noise/fault semantics.
 *
 * Resource requirements involving reliability or resilience may be consumed
 * by downstream semantic layers, but resource grammar does not define noise
 * models.
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. HARDWARE-INDEPENDENCE CONTRACT
 * ============================================================================
 *
 * The following must NEVER be encoded as grammar-level constants:
 *
 *     number of CPUs
 *     number of cores
 *     number of threads
 *     number of GPUs
 *     number of FPGAs
 *     number of ASICs
 *     number of qubits
 *     number of nodes
 *     memory size
 *     network size
 *     accelerator count
 *     device IDs
 *     physical addresses
 *     topology dimensions
 *
 * They must appear, where needed, as:
 *
 *     symbolic expressions;
 *     semantic requirements;
 *     constraints;
 *     capabilities;
 *     target properties;
 *     runtime observations;
 *     compilation context.
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. POCO-REAF EXAMPLES
 * ============================================================================
 *
 * These are examples only and are not parser actions.
 *
 * --------------------------------------------------------------------------
 *
 *     resource workload {
 *         quantity = problem_size;
 *         scalability = problem_size;
 *         portability = true;
 *     }
 *
 * --------------------------------------------------------------------------
 *
 *     requires resource memory >= required_memory;
 *
 * --------------------------------------------------------------------------
 *
 *     requires resource compute >= workload_parallelism;
 *
 * --------------------------------------------------------------------------
 *
 *     requires resource quantum.logical_qubits >= logical_qubits;
 *
 * --------------------------------------------------------------------------
 *
 *     constraint resource latency <= latency_budget;
 *
 * --------------------------------------------------------------------------
 *
 *     preference resource energy <= preferred_energy;
 *
 * --------------------------------------------------------------------------
 *
 *     hint resource accelerator.preferred = true;
 *
 * --------------------------------------------------------------------------
 *
 *     target = heterogeneous;
 *
 * --------------------------------------------------------------------------
 *
 * None of these expressions chooses a physical device.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. INVALID SCALABILITY PATTERNS
 * ============================================================================
 *
 * The following concepts MUST NOT be introduced into this grammar:
 *
 *     max_qubits = 64;
 *     max_cores = 128;
 *     max_gpus = 8;
 *     device_id = 7;
 *     physical_qubit = 3;
 *     memory_address = 0x...;
 *
 * Such information belongs to hardware descriptions, target configurations,
 * capabilities, or runtime state when it is genuinely required.
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar must be deterministic for the same token stream.
 *
 * The grammar must not:
 *
 *     inspect hardware;
 *     query runtime state;
 *     perform network access;
 *     perform filesystem discovery;
 *     invoke a backend;
 *     perform resource allocation.
 *
 * Those are downstream responsibilities.
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. SECURITY
 * ============================================================================
 *
 * Resource syntax is declarative.
 *
 * Parsing MUST NOT:
 *
 *     allocate resources;
 *     reserve hardware;
 *     access devices;
 *     access networks;
 *     execute programs;
 *     execute arbitrary code;
 *     invoke external commands.
 *
 * All such operations belong to explicitly authorized downstream systems.
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *     malformed resource declaration;
 *     malformed requirement;
 *     malformed constraint;
 *     malformed preference;
 *     malformed hint;
 *     malformed capability;
 *     malformed target;
 *     malformed quantity;
 *     malformed resource property;
 *     malformed resource group;
 *     malformed resource contract.
 *
 * Semantic diagnostics must separately identify:
 *
 *     unknown resource;
 *     unavailable resource;
 *     unsatisfied requirement;
 *     violated constraint;
 *     unsupported capability;
 *     incompatible target;
 *     impossible deployment.
 *
 * Parser and semantic errors MUST NOT be conflated.
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must cover:
 *
 *     resource declarations;
 *     resource types;
 *     qualified resource names;
 *     symbolic quantities;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     capabilities;
 *     targets;
 *     capacity;
 *     availability;
 *     portability;
 *     scalability;
 *     performance;
 *     latency;
 *     throughput;
 *     bandwidth;
 *     energy;
 *     power;
 *     reliability;
 *     resilience;
 *     cost;
 *     reservation;
 *     acquisition;
 *     release;
 *     groups;
 *     contracts;
 *     profiles;
 *     derivations;
 *     extensible properties.
 *
 * Negative tests must cover:
 *
 *     missing resource name;
 *     malformed quantity;
 *     malformed expression;
 *     malformed assignment;
 *     missing semicolon where required;
 *     malformed qualified name;
 *     malformed group;
 *     malformed contract;
 *     malformed attribute;
 *     invalid token placement.
 *
 * Scalability tests must verify that syntax does not impose fixed limits.
 *
 * Cross-domain tests must include:
 *
 *     classical + resources;
 *     quantum + resources;
 *     hardware + resources;
 *     hybrid + resources;
 *     distributed + resources;
 *     AI + resources;
 *     networking + resources;
 *     HDL + resources.
 *
 * Integration tests must verify that domain-specific resource grammars can
 * normalize into the common resource semantic model without duplication.
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It is accepted by the repository's ANTLR grammar build.
 *
 * [ ] Its token vocabulary matches the canonical lexer/token architecture.
 *
 * [ ] `expression` resolves to the repository's canonical expression grammar.
 *
 * [ ] `qualifiedResourceName` does not duplicate the general module/name
 *     system beyond the minimum parser boundary required here.
 *
 * [ ] No physical machine size is hard-coded.
 *
 * [ ] No quantum count is hard-coded.
 *
 * [ ] No CPU/GPU/FPGA/device count is hard-coded.
 *
 * [ ] No topology is hard-coded.
 *
 * [ ] No runtime allocation occurs during parsing.
 *
 * [ ] No hardware discovery occurs during parsing.
 *
 * [ ] No network or filesystem access occurs during parsing.
 *
 * [ ] Resource requirement, constraint, preference, hint, and capability
 *     semantics remain distinct.
 *
 * [ ] Quantum resource grammar can specialize this grammar without semantic
 *     duplication.
 *
 * [ ] Hardware resource grammar can specialize this grammar without semantic
 *     duplication.
 *
 * [ ] Hybrid resource grammar can compose this grammar.
 *
 * [ ] Resource intent can be lowered into the repository's canonical semantic
 *     representation.
 *
 * [ ] Resource intent can be consumed by compilation, scheduling, routing,
 *     hardware HAL, resource management, and runtime layers.
 *
 * [ ] The grammar itself remains independent of concrete hardware.
 *
 * [ ] Rust integration remains compatible with Rust 1.97/1.97.1 and Rust 2021.
 *
 * [ ] Generated Rust integration requires no unsafe Rust written by Zamani.
 *
 * [ ] Positive, negative, boundary, scalability, determinism, and cross-domain
 *     tests exist.
 *
 * ============================================================================
 */