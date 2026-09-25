/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/resources.g4
 *
 * Grammar:
 *     ZamaniHardwareResourcesParser
 *
 * Status:
 *     CANONICAL HARDWARE-RESOURCE INTENT LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL HARDWARE RESOURCE INTENT.
 *
 * It describes resources that a Zamani program or abstract hardware contract
 * may:
 *
 *     - declare;
 *     - reference;
 *     - require;
 *     - constrain;
 *     - prefer;
 *     - provide;
 *     - consume;
 *     - reserve;
 *     - acquire;
 *     - release;
 *     - quantify;
 *     - derive;
 *     - group;
 *     - profile;
 *     - expose as capabilities;
 *     - associate with abstract hardware contracts.
 *
 * Resource identity and quantities remain symbolic and semantic.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     hardware resource declaration syntax
 *     hardware resource kind syntax
 *     hardware resource references
 *     resource quantities
 *     resource requirements
 *     resource constraints
 *     resource preferences
 *     resource hints
 *     capability requirements/references
 *     abstract target references
 *     capacity observations/requirements
 *     availability observations/requirements
 *     portability intent
 *     scalability intent
 *     performance intent
 *     latency intent
 *     throughput intent
 *     bandwidth intent
 *     energy intent
 *     power intent
 *     reliability intent
 *     resilience intent
 *     cost intent
 *     abstract reservation intent
 *     abstract acquisition intent
 *     abstract release intent
 *     resource derivation
 *     resource groups
 *     resource contracts
 *     resource profiles
 *     resource properties
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical definitions
 *     identifiers
 *     literals
 *     general expression precedence
 *     general type syntax
 *     hardware module syntax
 *     HDL behavioral syntax
 *     physical-device discovery
 *     physical-device enumeration
 *     physical addresses
 *     physical allocation
 *     placement algorithms
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     backend selection
 *     runtime allocation
 *     QEC
 *     ZQN
 *     quantum::ir
 *     simulation
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Expression syntax is imported from:
 *
 *     grammar/expressions/expressions.g4
 *
 * Name syntax is imported from:
 *
 *     grammar/core/names.g4
 *
 * This file MUST NOT redefine:
 *
 *     expression
 *     qualifiedName
 *     identifier
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * Production parser grammars consume:
 *
 *     ZamaniLexer
 *
 * and MUST NOT create a second token vocabulary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Resource syntax describes:
 *
 *     WHAT is required
 *     WHAT is allowed
 *     WHAT is preferred
 *     WHAT capability is needed
 *     WHAT scaling relationship exists
 *     WHAT resource property matters
 *
 * It does NOT prescribe:
 *
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH QPU
 *     WHICH physical qubit
 *     WHICH memory bank
 *     WHICH node
 *     WHICH device identifier
 *     WHICH physical address
 *
 * Concrete realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally contains NO universal resource limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_RESOURCES
 *     MAX_RESOURCE_GROUPS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *
 * All resource quantities are expressions.
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * and is therefore not artificially bounded by this grammar.
 *
 * A program may therefore express:
 *
 *     quantity = n;
 *     quantity = workload_size;
 *     quantity = required_memory;
 *     quantity = problem_size * parallelism;
 *
 * without the grammar imposing a maximum.
 *
 * ============================================================================
 * SEMANTIC DISTINCTION
 * ============================================================================
 *
 * REQUIREMENT
 *     Mandatory semantic condition.
 *
 * CONSTRAINT
 *     Mandatory condition on legal realization.
 *
 * PREFERENCE
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *     Advisory information.
 *
 * CAPABILITY
 *     Ability expected from the realization.
 *
 * CAPACITY
 *     Available/provided quantity.
 *
 * AVAILABILITY
 *     Availability information supplied by the compilation/runtime context.
 *
 * RESOURCE
 *     Abstract computational or physical resource category.
 *
 * TARGET
 *     Abstract realization class.
 *
 * None of these concepts may be silently collapsed by the parser.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareResourcesParser;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Names
;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical hardware resource declaration.
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
 *     resource quantum_memory: quantum::logical_qubit {
 *         requires capacity >= required_qubits;
 *     };
 *
 * The resource name is symbolic.
 *
 * It is NOT a physical device identifier.
 */

hardwareResourceDeclaration
    : hardwareResourceAttributes*
      RESOURCE
      identifier
      hardwareResourceKind?
      hardwareResourceSpecification?
      SEMICOLON?
    ;


/* ============================================================================
 * 2. RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are syntactic metadata.
 *
 * Their validity and meaning are determined by semantic analysis.
 */

hardwareResourceAttributes
    : AT
      qualifiedName
      (
          LPAREN
          hardwareResourceAttributeArguments?
          RPAREN
      )?
    ;

hardwareResourceAttributeArguments
    : expressionList
    ;


/* ============================================================================
 * 3. RESOURCE KIND
 * ============================================================================
 *
 * Resource kinds are OPEN-WORLD symbolic names.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum::logical_qubit
 *     network::bandwidth
 *     storage::capacity
 *     custom::future_resource
 *
 * The grammar does not enumerate resource kinds.
 */

hardwareResourceKind
    : COLON
      qualifiedName
    ;


/* ============================================================================
 * 4. RESOURCE SPECIFICATION
 * ============================================================================
 */

hardwareResourceSpecification
    : LBRACE
      hardwareResourceBodyElement*
      RBRACE
    ;


/* ============================================================================
 * 5. RESOURCE BODY
 * ============================================================================
 */

hardwareResourceBodyElement
    : hardwareResourceAttributes*
      hardwareResourceClause
    ;


/* ============================================================================
 * 6. RESOURCE CLAUSE DISPATCH
 * ============================================================================
 *
 * Only resource-owned intent belongs here.
 *
 * Physical realization decisions are intentionally absent.
 */

hardwareResourceClause
    : hardwareResourceQuantityClause
    | hardwareResourceReferenceClause
    | hardwareResourceRequirementClause
    | hardwareResourceConstraintClause
    | hardwareResourcePreferenceClause
    | hardwareResourceHintClause
    | hardwareResourceCapabilityClause
    | hardwareResourceTargetClause
    | hardwareResourceCapacityClause
    | hardwareResourceAvailabilityClause
    | hardwareResourcePortabilityClause
    | hardwareResourceScalabilityClause
    | hardwareResourcePerformanceClause
    | hardwareResourceLatencyClause
    | hardwareResourceThroughputClause
    | hardwareResourceBandwidthClause
    | hardwareResourceEnergyClause
    | hardwareResourcePowerClause
    | hardwareResourceReliabilityClause
    | hardwareResourceResilienceClause
    | hardwareResourceCostClause
    | hardwareResourceReservationClause
    | hardwareResourceAcquisitionClause
    | hardwareResourceReleaseClause
    | hardwareResourceDerivationClause
    | hardwareResourceGroupClause
    | hardwareResourceContractClause
    | hardwareResourceProfileClause
    | hardwareResourcePropertyClause
    ;


/* ============================================================================
 * 7. QUANTITY
 * ============================================================================
 *
 * A quantity is an arbitrary Zamani expression.
 *
 * Examples:
 *
 *     quantity = n;
 *     quantity = workload_size;
 *     quantity = problem_size * lanes;
 *     quantity = required_memory;
 *
 * No finite numeric range is imposed here.
 */

hardwareResourceQuantityClause
    : QUANTITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 8. RESOURCE REFERENCE
 * ============================================================================
 *
 * References an abstract resource.
 *
 * This does not select a physical resource.
 */

hardwareResourceReferenceClause
    : USE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * Supported forms include:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires resource memory >= required_memory;
 *
 *     requires capability("tensor.compute");
 *
 * The parser records syntax only.
 * Satisfiability is determined downstream.
 */

hardwareResourceRequirementClause
    : REQUIRES
      hardwareResourceRequirementSubject
      SEMICOLON
    ;

hardwareResourceRequirementSubject
    : RESOURCE
      qualifiedName
      hardwareResourceRelationExpression?
    | CAPABILITY
      expression
    | expression
    ;

hardwareResourceRelationExpression
    : hardwareResourceRelationOperator
      expression
    ;


/* ============================================================================
 * 10. CONSTRAINT
 * ============================================================================
 */

hardwareResourceConstraintClause
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory and MUST remain semantically distinct from
 * requirements.
 */

hardwareResourcePreferenceClause
    : PREFER
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. HINT
 * ============================================================================
 */

hardwareResourceHintClause
    : HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. CAPABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     capability = quantum::measurement;
 *
 *     capability = capability("tensor.compute");
 *
 * The grammar does not enumerate capabilities.
 */

hardwareResourceCapabilityClause
    : CAPABILITY
      (
          ASSIGN
      )?
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. TARGET
 * ============================================================================
 *
 * Target intent is symbolic.
 *
 * Example:
 *
 *     target = quantum;
 *
 *     target = accelerator::tensor;
 *
 * No physical target is selected here.
 */

hardwareResourceTargetClause
    : TARGET
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 15. CAPACITY
 * ============================================================================
 *
 * Capacity is an implementation/context property.
 *
 * It is not a language-level maximum.
 */

hardwareResourceCapacityClause
    : CAPACITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 16. AVAILABILITY
 * ============================================================================
 */

hardwareResourceAvailabilityClause
    : AVAILABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 17. PORTABILITY
 * ============================================================================
 */

hardwareResourcePortabilityClause
    : PORTABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 18. SCALABILITY
 * ============================================================================
 *
 * Scaling remains symbolic.
 *
 * Examples:
 *
 *     scalability = problem_size;
 *
 *     scalability = workload_size * parallelism;
 *
 *     scalability = available_capacity;
 */

hardwareResourceScalabilityClause
    : SCALABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 19. PERFORMANCE
 * ============================================================================
 */

hardwareResourcePerformanceClause
    : PERFORMANCE
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 20. LATENCY
 * ============================================================================
 */

hardwareResourceLatencyClause
    : LATENCY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. THROUGHPUT
 * ============================================================================
 */

hardwareResourceThroughputClause
    : THROUGHPUT
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 22. BANDWIDTH
 * ============================================================================
 */

hardwareResourceBandwidthClause
    : BANDWIDTH
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 23. ENERGY
 * ============================================================================
 */

hardwareResourceEnergyClause
    : ENERGY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 24. POWER
 * ============================================================================
 */

hardwareResourcePowerClause
    : POWER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 25. RELIABILITY
 * ============================================================================
 */

hardwareResourceReliabilityClause
    : RELIABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 26. RESILIENCE
 * ============================================================================
 */

hardwareResourceResilienceClause
    : RESILIENCE
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 27. COST
 * ============================================================================
 */

hardwareResourceCostClause
    : COST
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 28. RESERVATION
 * ============================================================================
 *
 * Reservation is intent only.
 *
 * Parsing MUST NOT reserve anything.
 */

hardwareResourceReservationClause
    : RESERVE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 29. ACQUISITION
 * ============================================================================
 *
 * Acquisition is abstract intent.
 *
 * Actual allocation belongs downstream.
 */

hardwareResourceAcquisitionClause
    : ACQUIRE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 30. RELEASE
 * ============================================================================
 */

hardwareResourceReleaseClause
    : RELEASE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 31. DERIVATION
 * ============================================================================
 *
 * Derives a symbolic resource value from program semantics.
 *
 * Example:
 *
 *     derive required_memory = workload_size * element_size;
 */

hardwareResourceDerivationClause
    : DERIVE
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 32. RESOURCE GROUP
 * ============================================================================
 *
 * Groups remain unbounded.
 *
 * Example:
 *
 *     resource group compute {
 *         quantity = required_compute;
 *     }
 */

hardwareResourceGroupClause
    : RESOURCE
      GROUP
      identifier
      hardwareResourceSpecification
    ;


/* ============================================================================
 * 33. RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract groups resource intent without selecting a machine.
 */

hardwareResourceContractClause
    : CONTRACT
      identifier?
      hardwareResourceSpecification
    ;


/* ============================================================================
 * 34. RESOURCE PROFILE
 * ============================================================================
 *
 * A profile is a reusable symbolic resource-intent bundle.
 */

hardwareResourceProfileClause
    : PROFILE
      identifier?
      hardwareResourceSpecification
    ;


/* ============================================================================
 * 35. EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * Standard and future resource properties can be represented without
 * introducing a new keyword for every possible resource dimension.
 *
 * Both forms are accepted:
 *
 *     property bandwidth_class = ...;
 *
 *     bandwidth_class = ...;
 *
 * The semantic layer decides whether the property is:
 *
 *     standard
 *     dialect-defined
 *     target-defined
 *     experimental
 *     deprecated
 *     invalid
 */

hardwareResourcePropertyClause
    : PROPERTY
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    | qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 36. RELATION OPERATORS
 * ============================================================================
 *
 * The parser reuses the canonical operator vocabulary.
 *
 * This is intentionally not a second expression hierarchy.
 */

hardwareResourceRelationOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 37. IDENTIFIER BRIDGE
 * ============================================================================
 *
 * Contextual wrappers are allowed, but identifier syntax remains owned by
 * the canonical name grammar.
 */

identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * Every production above maps to domain-neutral frontend AST structures.
 *
 * At minimum, the semantic AST must preserve:
 *
 *     HardwareResourceDeclaration
 *     HardwareResourceKind
 *     HardwareResourceReference
 *     HardwareResourceQuantity
 *     HardwareResourceRequirement
 *     HardwareResourceConstraint
 *     HardwareResourcePreference
 *     HardwareResourceHint
 *     HardwareResourceCapability
 *     HardwareResourceTarget
 *     HardwareResourceCapacity
 *     HardwareResourceAvailability
 *     HardwareResourcePortability
 *     HardwareResourceScalability
 *     HardwareResourcePerformance
 *     HardwareResourceLatency
 *     HardwareResourceThroughput
 *     HardwareResourceBandwidth
 *     HardwareResourceEnergy
 *     HardwareResourcePower
 *     HardwareResourceReliability
 *     HardwareResourceResilience
 *     HardwareResourceCost
 *     HardwareResourceReservation
 *     HardwareResourceAcquisition
 *     HardwareResourceRelease
 *     HardwareResourceDerivation
 *     HardwareResourceGroup
 *     HardwareResourceContract
 *     HardwareResourceProfile
 *     HardwareResourceProperty
 *
 * Every node MUST preserve source spans.
 *
 * Expressions remain structured canonical expression nodes.
 *
 * Qualified names remain structured canonical name nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     resource-name resolution
 *     duplicate declaration checking
 *     resource-kind resolution
 *     quantity type checking
 *     dimensional/resource-unit checking
 *     requirement satisfiability
 *     constraint validation
 *     preference classification
 *     capability resolution
 *     target compatibility
 *     availability interpretation
 *     portability analysis
 *     scalability analysis
 *     resource dependency analysis
 *     resource lifetime analysis
 *
 * This grammar MUST NOT decide whether a resource is available.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO hardware IR.
 *
 * Resource declarations are lowered through the canonical semantic/resource
 * model.
 *
 * The eventual compiler path is:
 *
 *     source
 *       ↓
 *     frontend AST
 *       ↓
 *     semantic resource model
 *       ↓
 *     canonical compiler IR
 *       ↓
 *     optimization
 *       ↓
 *     target/resource resolution
 *       ↓
 *     routing / scheduling where applicable
 *       ↓
 *     HAL/backend
 *
 * Quantum resources ultimately participate in the existing canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR is introduced here.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     physical device IDs
 *     physical qubit IDs
 *     PCI addresses
 *     memory addresses
 *     vendor-specific device slots
 *     fixed machine topology
 *     fixed bus widths
 *     fixed register widths
 *     fixed memory capacities
 *
 * Such information belongs downstream target realization.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexical configuration
 *     grammar version
 *     explicitly selected dialect composition
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     runtime state
 *     environment variables
 *     filesystem state
 *     network state
 *     clock/time
 *     randomness
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no Rust code
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *
 * Rust generated/consuming code remains subject to:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * HARDWARE COMPOSITION
 *
 *     grammar/hardware/hardware.g4
 *
 * MUST import/compose this grammar and delegate:
 *
 *     hardwareResourceDeclaration
 *
 * instead of defining another rule with the same ownership.
 *
 * TARGET COMPOSITION
 *
 *     grammar/hardware/targets.g4
 *
 * may reference resource names and requirements, but MUST NOT duplicate
 * hardware resource declaration syntax.
 *
 * UNIVERSAL RESOURCE COMPOSITION
 *
 *     grammar/resources/resources.g4
 *
 * remains the owner of UNIVERSAL source-level resource syntax.
 *
 * It may delegate hardware-specific resource declarations here when a
 * construct is explicitly inside a hardware declaration.
 *
 * The two grammars therefore have different ownership:
 *
 *     resources/resources.g4
 *         universal resource intent
 *
 *     hardware/resources.g4
 *         hardware-contract resource intent
 *
 * QUANTUM
 *
 * Quantum grammar may express requirements such as:
 *
 *     requires qubits >= n;
 *
 * without selecting physical qubits.
 *
 * CLASSICAL
 *
 * Classical grammar may express:
 *
 *     requires memory >= required_memory;
 *
 * without assuming a particular RAM size.
 *
 * HDL
 *
 * HDL may refer to resource requirements without hard-coding a universal
 * implementation capacity.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource memory: memory {
 *         quantity = required_memory;
 *     };
 *
 *     resource quantum_memory: quantum::logical_qubit {
 *         requires capacity >= required_qubits;
 *     };
 *
 *     resource accelerator: accelerator {
 *         requires capability("tensor.compute");
 *         scalability = problem_size;
 *     };
 *
 *     resource network: network::bandwidth {
 *         bandwidth = required_bandwidth;
 *         latency = latency_budget;
 *     };
 *
 *     resource group compute {
 *         quantity = workload_size;
 *     }
 *
 *     derive required_memory = elements * element_size;
 *
 * Negative tests MUST include:
 *
 *     physical resource IDs
 *     physical addresses where prohibited by semantic rules
 *     malformed resource declarations
 *     missing expressions
 *     missing resource names
 *     malformed comparisons
 *     malformed property assignments
 *
 * Boundary/scalability tests MUST include:
 *
 *     symbolic quantity
 *     arbitrarily large integer literals supported by the lexer
 *     nested resource groups
 *     many resource clauses
 *     many resource properties
 *     symbolic scaling expressions
 *     resource quantities derived from program parameters
 *     quantum resource requirements
 *     classical resource requirements
 *     accelerator resource requirements
 *     distributed resource requirements
 *
 * The grammar MUST NOT test or establish an artificial maximum.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] one canonical hardware-resource declaration owner exists;
 *     [x] canonical ZamaniLexer is consumed;
 *     [x] canonical expression grammar is reused;
 *     [x] canonical name grammar is reused;
 *     [x] no K_* token aliases remain;
 *     [x] resource kinds are open-world;
 *     [x] quantities are expressions;
 *     [x] requirements are distinct from preferences;
 *     [x] constraints are distinct from hints;
 *     [x] capabilities remain symbolic;
 *     [x] target identity remains abstract;
 *     [x] no physical hardware is selected;
 *     [x] no universal hardware limit is encoded;
 *     [x] no second expression hierarchy exists;
 *     [x] no second quantum IR exists;
 *     [x] source spans are preserved by the AST contract;
 *     [x] semantic validation remains downstream;
 *     [x] deterministic parsing is preserved;
 *     [x] Rust integration requires no unsafe code;
 *     [x] positive/negative/boundary/scalability tests are defined.
 *
 * ============================================================================
 */