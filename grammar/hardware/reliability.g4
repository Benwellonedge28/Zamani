/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/reliability.g4
 *
 * Grammar:
 *     ZamaniHardwareReliabilityParser
 *
 * Status:
 *     CANONICAL HARDWARE RELIABILITY / RESILIENCE INTENT LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only
 *
 * Safety:
 *     Action-free ANTLR parser grammar.
 *     No embedded Rust.
 *     No target-language actions.
 *     No semantic predicates.
 *     No I/O.
 *     No hardware discovery.
 *     No runtime execution.
 *     No unsafe implementation requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL, TARGET-INDEPENDENT hardware reliability
 * intent.
 *
 * It describes properties and contracts such as:
 *
 *     reliability requirements
 *     reliability constraints
 *     reliability preferences
 *     reliability hints
 *     availability
 *     durability
 *     dependability
 *     fault tolerance
 *     failure probability
 *     failure rate
 *     recovery objectives
 *     redundancy
 *     survivability
 *     service continuity
 *     failure domains
 *     resilience relationships
 *     resilience states
 *     resilience outcomes
 *     recovery intent
 *     reliability scaling
 *     reliability capabilities
 *     reliability resources
 *     reliability profiles
 *     reliability properties
 *     reliability assertions
 *
 * The grammar describes WHAT reliability/resilience properties are required
 * or preferred.
 *
 * It does NOT decide HOW those properties are physically achieved.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> reliability analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> resilience analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware semantics
 *          +--> distributed semantics
 *          |
 *          v
 *     optimization
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime / deployment
 *
 * This grammar remains ABOVE target realization.
 *
 * ============================================================================
 * 3. SINGLE OWNER
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     hardware reliability declarations
 *     hardware reliability contracts
 *     reliability-specific requirements
 *     reliability-specific constraints
 *     reliability-specific preferences
 *     reliability-specific hints
 *     reliability properties
 *     reliability profiles
 *     reliability groups
 *     reliability failure-domain intent
 *     reliability redundancy intent
 *     reliability recovery intent
 *     resilience-state intent
 *     resilience-outcome intent
 *     reliability scaling intent
 *     reliability capability references
 *     reliability resource references
 *     reliability assertions
 *     reliability relationships
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer rules
 *     identifiers
 *     literals
 *     general expressions
 *     general types
 *     universal resource declarations
 *     universal capability declarations
 *     hardware targets
 *     physical device IDs
 *     physical addresses
 *     topology realization
 *     routing
 *     scheduling
 *     calibration
 *     thermal modeling
 *     power modeling
 *     QEC algorithms
 *     ZQN implementation
 *     quantum::ir
 *     runtime recovery
 *     device discovery
 *     backend selection
 *     HAL implementation
 *
 * ============================================================================
 * 4. RELATIONSHIP WITH EXISTING RESOURCE GRAMMARS
 * ============================================================================
 *
 * The repository already contains lower-level reliability syntax, including:
 *
 *     grammar/resources/reliability.g4
 *     grammar/declarations/resources.g4
 *     grammar/hardware/memory.g4
 *     grammar/hardware/cpu.g4
 *
 * Those files remain responsible for reliability information attached to
 * their respective resource/declaration contexts.
 *
 * THIS FILE DOES NOT duplicate those productions.
 *
 * Instead:
 *
 *     resource reliability
 *             |
 *             v
 *     generic resource semantics
 *
 *     hardware reliability
 *             |
 *             v
 *     hardware reliability contract
 *
 * Both converge in semantic analysis.
 *
 * This separation is intentional.
 *
 * ============================================================================
 * 5. RELATIONSHIP WITH RESILIENCE
 * ============================================================================
 *
 * Reliability and resilience are related but distinct.
 *
 * Reliability generally describes the probability/ability of a system,
 * resource, service, or computation to satisfy a required behavior over
 * a defined context or interval.
 *
 * Resilience describes the ability of the system to tolerate, recover from,
 * adapt to, or continue operation through adverse conditions.
 *
 * Therefore:
 *
 *     reliability
 *         !=
 *     resilience
 *
 * This grammar permits both concepts while preserving their semantic
 * distinction.
 *
 * The semantic/runtime resilience subsystem remains responsible for actual:
 *
 *     detection
 *     diagnosis
 *     recovery
 *     retry
 *     repair
 *     quarantine
 *     failover
 *     QEC
 *     fault handling
 *
 * ============================================================================
 * 6. POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Reliability intent must remain portable across:
 *
 *     atom-scale systems
 *     embedded systems
 *     microcontrollers
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC
 *     clusters
 *     distributed systems
 *     cloud systems
 *     edge systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * A reliability contract describes semantic requirements.
 *
 * It does NOT select:
 *
 *     a specific CPU
 *     a specific GPU
 *     a specific FPGA
 *     a specific ASIC
 *     a specific QPU
 *     a physical qubit
 *     a physical node
 *     a device address
 *     a vendor implementation
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * 7. NO ARTIFICIAL SCALABILITY LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_RELIABILITY_CONTRACTS
 *     MAX_FAILURE_DOMAINS
 *     MAX_REDUNDANCY
 *     MAX_RECOVERY_STEPS
 *     MAX_FAILURES
 *     MAX_RETRIES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_THREADS
 *     MAX_RESOURCES
 *     MAX_WORKERS
 *     MAX_REPLICAS
 *     MAX_STATES
 *     MAX_OUTCOMES
 *
 * Nor may equivalent finite limits be encoded through bounded alternatives.
 *
 * Collections therefore use:
 *
 *     *
 *     +
 *
 * and quantities use expressions.
 *
 * "Infinity" means:
 *
 *     unbounded by the language architecture.
 *
 * It does NOT claim physically infinite resources.
 *
 * Actual scale is determined by:
 *
 *     source semantics
 *     available resources
 *     target capabilities
 *     compiler resources
 *     runtime resources
 *     deployment policy
 *     physical reality
 *
 * ============================================================================
 * 8. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * REQUIREMENT
 *     Mandatory semantic condition.
 *
 * CONSTRAINT
 *     Mandatory condition restricting valid realization.
 *
 * PREFERENCE
 *     Advisory optimization objective.
 *
 * HINT
 *     Optional optimization information.
 *
 * These categories MUST remain distinguishable in the parse tree.
 *
 * A backend MUST NOT silently convert:
 *
 *     requirement -> preference
 *
 * or:
 *
 *     constraint -> hint
 *
 * because a target cannot satisfy the stronger form.
 *
 * ============================================================================
 * 9. LEXICAL CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical Zamani token vocabulary.
 *
 * It does NOT define lexer rules.
 *
 * The hardware grammar family currently uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * This file follows that existing hardware integration contract.
 *
 * Reliability-specific vocabulary uses existing canonical tokens such as:
 *
 *     K_RELIABILITY
 *     K_RESILIENCE
 *     K_REQUIRES
 *     K_CONSTRAINT
 *     K_PREFER
 *     K_HINT
 *     K_CAPABILITY
 *     K_RESOURCE
 *     K_TARGET
 *     K_PROFILE
 *     K_CONTRACT
 *     K_GROUP
 *     K_ASSERT
 *
 * Property names remain extensible wherever structural syntax permits.
 *
 * This avoids turning every future reliability concept into a permanent
 * language keyword.
 *
 * ============================================================================
 * 10. SHARED HARDWARE PARSER CONTRACT
 * ============================================================================
 *
 * When composed by grammar/hardware/hardware.g4, this grammar consumes the
 * shared hardware parser rules:
 *
 *     hardwareExpression
 *     hardwareQualifiedName
 *     hardwareRelationOperator
 *     hardwareLogicalOperator
 *
 * Those rules remain owned by the hardware composition grammar.
 *
 * This file MUST NOT redefine them.
 *
 * The eventual migration to the universal expression grammar can therefore
 * replace the hardware expression bridge without changing the reliability
 * contract.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareReliabilityParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 11. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Examples:
 *
 *     reliability compute {
 *         availability >= required_availability;
 *     }
 *
 *     reliability contract compute {
 *         requires availability >= required_availability;
 *         constraint failure_rate <= allowed_failure_rate;
 *     }
 *
 *     reliability profile resilient {
 *         redundancy >= required_redundancy;
 *     }
 *
 * The declaration is symbolic.
 *
 * It does not identify a physical device.
 */

hardwareReliabilityDeclaration
    : K_RELIABILITY
      hardwareReliabilityDeclarationModifier*
      hardwareReliabilityDeclarationKind?
      IDENTIFIER?
      hardwareReliabilityTargetClause?
      hardwareReliabilityBody
      SEMICOLON?
    ;


/* ============================================================================
 * 12. DECLARATION KINDS
 * ============================================================================
 *
 * These are intentionally limited to existing structural vocabulary.
 *
 * Future reliability concepts should normally be represented as properties
 * or dialect extensions rather than permanent root keywords.
 */

hardwareReliabilityDeclarationKind
    : K_CONTRACT
    | K_PROFILE
    ;


/* ============================================================================
 * 13. MODIFIERS
 * ============================================================================
 *
 * Reliability modifiers describe source-level declaration intent only.
 *
 * They do not identify a physical implementation.
 */

hardwareReliabilityDeclarationModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/* ============================================================================
 * 14. TARGET
 * ============================================================================
 *
 * The target is an abstract semantic subject.
 *
 * Example:
 *
 *     reliability compute for hardware::accelerator {
 *         ...
 *     }
 *
 * Actual device selection remains downstream.
 */

hardwareReliabilityTargetClause
    : K_TARGET
      hardwareQualifiedName
    ;


/* ============================================================================
 * 15. BODY
 * ============================================================================
 */

hardwareReliabilityBody
    : LBRACE
      hardwareReliabilityItem*
      RBRACE
    ;


/* ============================================================================
 * 16. BODY DISPATCH
 * ============================================================================
 *
 * Every reliability construct has one owner.
 *
 * No reliability production is duplicated inside hardware.g4.
 */

hardwareReliabilityItem
    : hardwareReliabilityRequirement
    | hardwareReliabilityConstraint
    | hardwareReliabilityPreference
    | hardwareReliabilityHint
    | hardwareReliabilityProperty
    | hardwareReliabilityCapability
    | hardwareReliabilityResource
    | hardwareReliabilityTarget
    | hardwareReliabilityProfile
    | hardwareReliabilityGroup
    | hardwareReliabilityFailureDomain
    | hardwareReliabilityRedundancy
    | hardwareReliabilityRecovery
    | hardwareReliabilityResilience
    | hardwareReliabilityRelation
    | hardwareReliabilityScaling
    | hardwareReliabilityAssertion
    ;


/* ============================================================================
 * 17. REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     requires availability >= required_availability;
 *     requires failure_rate <= allowed_failure_rate;
 *     requires capability::fault_tolerance;
 *
 * The grammar records intent.
 *
 * Semantic analysis determines:
 *
 *     dimensional validity
 *     probabilistic validity
 *     resource validity
 *     capability validity
 *     target compatibility
 *     satisfiability
 */

hardwareReliabilityRequirement
    : K_REQUIRES
      hardwareReliabilityCondition
      SEMICOLON
    ;


/* ============================================================================
 * 18. CONSTRAINT
 * ============================================================================
 */

hardwareReliabilityConstraint
    : K_CONSTRAINT
      hardwareReliabilityCondition
      SEMICOLON
    ;


/* ============================================================================
 * 19. PREFERENCE
 * ============================================================================
 */

hardwareReliabilityPreference
    : K_PREFER
      hardwareReliabilityCondition
      SEMICOLON
    ;


/* ============================================================================
 * 20. HINT
 * ============================================================================
 */

hardwareReliabilityHint
    : K_HINT
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 21. RELIABILITY CONDITION
 * ============================================================================
 *
 * A condition may be:
 *
 *     property >= expression
 *     property <= expression
 *     expression
 *
 * Logical composition uses the existing hardware logical operator contract.
 *
 * The grammar does not evaluate the condition.
 */

hardwareReliabilityCondition
    : hardwareReliabilityPredicate
      (
          hardwareLogicalOperator
          hardwareReliabilityPredicate
      )*
    ;


hardwareReliabilityPredicate
    : hardwareExpression
      hardwareRelationOperator
      hardwareExpression
    | hardwareExpression
    | LPAREN
      hardwareReliabilityCondition
      RPAREN
    ;


/* ============================================================================
 * 22. GENERIC PROPERTY
 * ============================================================================
 *
 * This is the principal extensibility mechanism.
 *
 * Examples:
 *
 *     availability >= required_availability;
 *     durability >= required_durability;
 *     failure_probability <= acceptable_probability;
 *     failure_rate <= acceptable_rate;
 *     mtbf >= required_mtbf;
 *     mttr <= recovery_budget;
 *     confidence >= required_confidence;
 *     redundancy >= required_redundancy;
 *     survivability >= required_survivability;
 *     resilience_state = Healthy;
 *     resilience_outcome = ACCEPT;
 *
 * Property names remain open-world.
 *
 * The semantic layer determines whether a property is recognized and what
 * dimension/type/domain it requires.
 */

hardwareReliabilityProperty
    : hardwareReliabilityPropertyName
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON
    ;


hardwareReliabilityPropertyName
    : IDENTIFIER
    | K_RELIABILITY
    | K_RESILIENCE
    | K_AVAILABILITY
    | K_CAPACITY
    | K_PERFORMANCE
    | K_LATENCY
    | K_THROUGHPUT
    | K_BANDWIDTH
    | K_ENERGY
    | K_POWER
    | K_COST
    | K_SCALABILITY
    | K_PORTABILITY
    | K_CAPABILITY
    | K_RESOURCE
    | K_TARGET
    | K_PROFILE
    | K_CONTRACT
    | K_GROUP
    | K_PROPERTY
    ;


/* ============================================================================
 * 23. CAPABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     capability = fault_tolerance;
 *     capability = quantum::fault_tolerance;
 *     capability = resilience::recovery;
 *
 * Capability discovery remains downstream.
 */

hardwareReliabilityCapability
    : K_CAPABILITY
      (
          ASSIGN
      )?
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 24. RESOURCE
 * ============================================================================
 *
 * A resource reference is symbolic.
 *
 * It does not allocate or reserve physical hardware.
 */

hardwareReliabilityResource
    : K_RESOURCE
      hardwareQualifiedName
      (
          hardwareRelationOperator
          hardwareExpression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 25. TARGET REFERENCE
 * ============================================================================
 *
 * This is a semantic target expression.
 *
 * It is not a physical device selection.
 */

hardwareReliabilityTarget
    : K_TARGET
      (
          ASSIGN
      )?
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 26. PROFILE
 * ============================================================================
 *
 * A profile groups reliability properties without defining a closed world of
 * reliability technologies.
 */

hardwareReliabilityProfile
    : K_PROFILE
      IDENTIFIER
      hardwareReliabilityProfileBody
    ;


hardwareReliabilityProfileBody
    : LBRACE
      hardwareReliabilityProfileItem*
      RBRACE
    ;


hardwareReliabilityProfileItem
    : hardwareReliabilityProperty
    | hardwareReliabilityRequirement
    | hardwareReliabilityConstraint
    | hardwareReliabilityPreference
    | hardwareReliabilityHint
    | hardwareReliabilityCapability
    | hardwareReliabilityResource
    | hardwareReliabilityResilience
    ;


/* ============================================================================
 * 27. GROUP
 * ============================================================================
 *
 * A logical reliability group can represent:
 *
 *     service
 *     component set
 *     resource set
 *     execution region
 *     failure domain
 *     deployment unit
 *
 * It does not imply a physical topology.
 */

hardwareReliabilityGroup
    : K_GROUP
      IDENTIFIER
      hardwareReliabilityGroupBody
    ;


hardwareReliabilityGroupBody
    : LBRACE
      hardwareReliabilityGroupItem*
      RBRACE
    ;


hardwareReliabilityGroupItem
    : hardwareReliabilityProperty
    | hardwareReliabilityRequirement
    | hardwareReliabilityConstraint
    | hardwareReliabilityPreference
    | hardwareReliabilityHint
    | hardwareReliabilityFailureDomain
    | hardwareReliabilityRedundancy
    | hardwareReliabilityRecovery
    | hardwareReliabilityResilience
    ;


/* ============================================================================
 * 28. FAILURE DOMAIN
 * ============================================================================
 *
 * Failure domains are LOGICAL.
 *
 * They may later be mapped to:
 *
 *     process
 *     task
 *     service
 *     resource
 *     module
 *     rack
 *     node
 *     device
 *     QPU
 *     QEC region
 *
 * but this grammar does not make any of those mappings mandatory.
 *
 * The name is intentionally symbolic.
 */

hardwareReliabilityFailureDomain
    : IDENTIFIER
      hardwareReliabilityFailureDomainOperator
      hardwareQualifiedName
      SEMICOLON
    ;


hardwareReliabilityFailureDomainOperator
    : ASSIGN
    ;


/* ============================================================================
 * 29. REDUNDANCY
 * ============================================================================
 *
 * Redundancy is semantic intent.
 *
 * It does not require replication onto a fixed number of physical devices.
 */

hardwareReliabilityRedundancy
    : IDENTIFIER
      hardwareReliabilityRedundancyOperator
      hardwareExpression
      SEMICOLON
    ;


hardwareReliabilityRedundancyOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 30. RECOVERY
 * ============================================================================
 *
 * Recovery properties are intentionally represented through an extensible
 * named-property form.
 *
 * Examples:
 *
 *     recovery_time <= recovery_budget;
 *     recovery_probability >= required_probability;
 *     recovery_overhead <= overhead_budget;
 *
 * Actual recovery implementation belongs to the resilience/runtime layers.
 */

hardwareReliabilityRecovery
    : IDENTIFIER
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 31. RESILIENCE
 * ============================================================================
 *
 * Resilience is represented as a semantic contract.
 *
 * Example:
 *
 *     resilience = required_resilience;
 *
 * or:
 *
 *     resilience >= required_resilience;
 *
 * This grammar does not implement recovery.
 */

hardwareReliabilityResilience
    : K_RESILIENCE
      (
          hardwareRelationOperator
          hardwareExpression
        | ASSIGN
          hardwareExpression
        | hardwareReliabilityResilienceBody
      )
      SEMICOLON?
    ;


hardwareReliabilityResilienceBody
    : LBRACE
      hardwareReliabilityResilienceItem*
      RBRACE
    ;


hardwareReliabilityResilienceItem
    : hardwareReliabilityProperty
    | hardwareReliabilityRequirement
    | hardwareReliabilityConstraint
    | hardwareReliabilityPreference
    | hardwareReliabilityHint
    | hardwareReliabilityCapability
    | hardwareReliabilityResource
    ;


/* ============================================================================
 * 32. RESILIENCE STATES AND OUTCOMES
 * ============================================================================
 *
 * Standard semantic vocabulary established by the resilience architecture:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * Standard resilience outcomes:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These are intentionally NOT lexer-level closed enums here.
 *
 * They are represented as values of extensible properties such as:
 *
 *     resilience_state = Healthy;
 *     resilience_outcome = ACCEPT;
 *
 * This allows future states/outcomes without changing the universal lexer or
 * creating a parser-level hard-coded closed world.
 *
 * Semantic analysis is responsible for validating standard vocabulary and
 * dialect-defined extensions.
 */


/* ============================================================================
 * 33. RELATIONSHIP
 * ============================================================================
 *
 * A relationship expresses a semantic dependency between reliability
 * properties, resources, or logical domains.
 *
 * Example:
 *
 *     relationship {
 *         availability = service::availability;
 *         redundancy = compute::redundancy;
 *     }
 *
 * The relationship is declarative.
 */

hardwareReliabilityRelation
    : IDENTIFIER
      hardwareReliabilityRelationBody
    ;


hardwareReliabilityRelationBody
    : LBRACE
      hardwareReliabilityRelationItem*
      RBRACE
    ;


hardwareReliabilityRelationItem
    : hardwareReliabilityProperty
    | hardwareReliabilityRequirement
    | hardwareReliabilityConstraint
    | hardwareReliabilityPreference
    | hardwareReliabilityHint
    | hardwareReliabilityCapability
    | hardwareReliabilityResource
    ;


/* ============================================================================
 * 34. SCALING
 * ============================================================================
 *
 * Reliability may depend on:
 *
 *     workload size
 *     resource scale
 *     parallelism
 *     replication
 *     execution duration
 *     environmental conditions
 *
 * The grammar accepts arbitrary expressions.
 *
 * No finite number of scaling points is encoded.
 *
 * Example:
 *
 *     scaling = workload_size;
 *     reliability_target >= required_reliability(workload_size);
 */

hardwareReliabilityScaling
    : K_SCALABILITY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 35. ASSERTION
 * ============================================================================
 *
 * Assertions are source-level contracts.
 *
 * They do not perform runtime recovery.
 */

hardwareReliabilityAssertion
    : K_ASSERT
      LPAREN
      hardwareReliabilityCondition
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 36. STANDARD RELIABILITY PROPERTY VOCABULARY
 * ============================================================================
 *
 * The following names are examples of semantic properties.
 *
 * They intentionally remain identifiers rather than new lexer keywords:
 *
 *     availability
 *     durability
 *     dependability
 *     reliability
 *     failure_probability
 *     failure_rate
 *     fault_tolerance
 *     survivability
 *     recoverability
 *     redundancy
 *     replication
 *     continuity
 *     confidence
 *     mtbf
 *     mttf
 *     mttr
 *     recovery_time
 *     recovery_probability
 *     recovery_overhead
 *     service_level
 *     service_continuity
 *     failure_domain
 *     correlated_failure
 *     common_mode_failure
 *     resilience_state
 *     resilience_outcome
 *
 * The semantic specification decides which properties are normative.
 *
 * This grammar remains open to future reliability models.
 */


/* ============================================================================
 * 37. QUANTITY / UNIT BOUNDARY
 * ============================================================================
 *
 * This grammar does NOT define:
 *
 *     probability literals
 *     duration literals
 *     frequency literals
 *     energy literals
 *     power literals
 *     percentage literals
 *     arbitrary-precision numeric literals
 *
 * Those belong to the canonical lexical/expression/type system.
 *
 * Examples that may be represented through the canonical expression system
 * include:
 *
 *     availability >= required_availability
 *     failure_probability <= 1e-9
 *     mtbf >= required_mtbf
 *     mttr <= 20ms
 *     recovery_time <= recovery_budget
 *
 * Dimensional and probabilistic correctness belongs to semantic analysis.
 *
 * ============================================================================
 * 38. TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following MUST remain invalid as implicit semantics:
 *
 *     physical_device(0)
 *     physical_qubit(17)
 *     cpu_core(7)
 *     gpu(3)
 *     node(42)
 *     memory_address(...)
 *
 * unless a separate explicitly target-specific/deployment language owns such
 * syntax.
 *
 * A reliability contract must remain meaningful when the implementation
 * changes target.
 *
 * ============================================================================
 * 39. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Reliability intent associated with quantum computation follows:
 *
 *     Zamani source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic reliability/resource analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     resilience
 *          |
 *          +--> QEC
 *          +--> ZQN
 *          +--> routing
 *          +--> scheduling
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     QPU realization
 *
 * This grammar MUST NOT define:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     QPUAllocation
 *     QEC algorithm
 *     syndrome decoder
 *     physical topology
 *     calibration data
 *
 * Those remain downstream concerns.
 *
 * ============================================================================
 * 40. CLASSICAL / GPU / FPGA / ASIC INTEGRATION
 * ============================================================================
 *
 * The same reliability contract may apply to:
 *
 *     CPU computation
 *     GPU computation
 *     FPGA implementation
 *     ASIC implementation
 *     accelerator computation
 *     distributed computation
 *     hybrid computation
 *
 * Target-specific reliability realization is performed after semantic
 * validation.
 *
 * The source contract remains portable.
 *
 * ============================================================================
 * 41. POWER INTEGRATION
 * ============================================================================
 *
 * Power remains owned by:
 *
 *     grammar/hardware/power.g4
 *
 * This file may reference power as a reliability property:
 *
 *     power <= power_budget;
 *
 * but it MUST NOT duplicate power grammar.
 *
 * Semantic analysis may correlate:
 *
 *     reliability
 *     power
 *     thermal behavior
 *     timing
 *     resource availability
 *
 * without collapsing those domains into one grammar.
 *
 * ============================================================================
 * 42. TIMING INTEGRATION
 * ============================================================================
 *
 * Timing remains owned by:
 *
 *     grammar/hardware/timing.g4
 *
 * Reliability may reference timing expressions such as:
 *
 *     recovery_time <= recovery_budget;
 *
 * but timing declaration syntax remains outside this file.
 *
 * ============================================================================
 * 43. THERMAL INTEGRATION
 * ============================================================================
 *
 * Thermal semantics remain downstream.
 *
 * Reliability may contain symbolic properties such as:
 *
 *     thermal_reliability >= required_value;
 *
 * but this file does not define:
 *
 *     temperature equations
 *     thermal simulation
 *     cooling implementation
 *     heat-sink selection
 *     physical thermal topology
 *
 * ============================================================================
 * 44. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements remain composable with reliability requirements.
 *
 * Example:
 *
 *     reliability compute {
 *         requires resource::compute >= required_compute;
 *         requires availability >= required_availability;
 *     }
 *
 * The reliability grammar does not allocate the resource.
 *
 * Resource resolution remains downstream.
 *
 * ============================================================================
 * 45. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability references are open-world.
 *
 * Examples:
 *
 *     capability = fault_tolerance;
 *     capability = resilience::recovery;
 *     capability = quantum::error_correction;
 *     capability = distributed::failover;
 *
 * No fixed capability list is encoded.
 *
 * ============================================================================
 * 46. FAILURE MODEL BOUNDARY
 * ============================================================================
 *
 * This grammar may describe failure-related intent:
 *
 *     failure_probability
 *     failure_rate
 *     correlated_failure
 *     common_mode_failure
 *     failure_domain
 *
 * It does NOT define a particular probabilistic or physical fault model.
 *
 * Fault-model semantics belong to:
 *
 *     semantic analysis
 *     resilience
 *     ZQN
 *     QEC
 *     hardware capability models
 *     runtime
 *
 * ============================================================================
 * 47. RECOVERY BOUNDARY
 * ============================================================================
 *
 * This grammar can state recovery objectives.
 *
 * It does not implement:
 *
 *     retry loops
 *     rollback
 *     checkpoint restore
 *     migration
 *     failover
 *     device replacement
 *     repair
 *     quarantine
 *
 * Those are downstream runtime/resilience responsibilities.
 *
 * ============================================================================
 * 48. DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text
 *     selected grammar version
 *     canonical token stream
 *     explicit parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     current hardware
 *     available resources
 *     runtime state
 *     device health
 *     network state
 *     randomness
 *     wall-clock time
 *     environment variables
 *     filesystem state
 *
 * ============================================================================
 * 49. AST CONTRACT
 * ============================================================================
 *
 * The parser output maps into the domain-neutral AST.
 *
 * Conceptually:
 *
 *     hardwareReliabilityDeclaration
 *             |
 *             v
 *     Declaration
 *             |
 *             v
 *     HardwareReliabilityContract
 *
 * The AST MUST preserve:
 *
 *     declaration name
 *     declaration kind
 *     modifiers
 *     target
 *     body items
 *     source spans
 *
 * Body item categories MUST remain distinguishable:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     property
 *     capability
 *     resource
 *     target
 *     profile
 *     group
 *     failure domain
 *     redundancy
 *     recovery
 *     resilience
 *     relationship
 *     scaling
 *     assertion
 *
 * No semantic information may be discarded by the parser.
 *
 * ============================================================================
 * 50. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     property validity
 *     unit validity
 *     dimensional correctness
 *     probability validity
 *     reliability model validity
 *     capability resolution
 *     resource resolution
 *     target compatibility
 *     failure-domain semantics
 *     redundancy semantics
 *     recovery semantics
 *     resilience semantics
 *     state validation
 *     outcome validation
 *     correlation semantics
 *     scaling semantics
 *     contradiction detection
 *     satisfiability
 *     diagnostics
 *
 * For example:
 *
 *     availability >= 0.999999
 *
 * is syntactically valid here.
 *
 * Whether that is a valid availability quantity and whether a target can
 * satisfy it is semantic analysis.
 *
 * ============================================================================
 * 51. IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT define an IR.
 *
 * Reliability information is lowered from:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware semantics
 *          +--> distributed semantics
 *
 * There MUST NOT be:
 *
 *     ReliabilityIR
 *     HardwareReliabilityIR
 *     QuantumReliabilityIR
 *
 * created by this grammar.
 *
 * Reliability is cross-cutting semantic information consumed by the
 * appropriate canonical downstream representations.
 *
 * ============================================================================
 * 52. COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler consumers may include:
 *
 *     resource analysis
 *     target capability negotiation
 *     optimization
 *     replication
 *     fault-domain analysis
 *     placement
 *     routing
 *     scheduling
 *     resilience planning
 *     QEC planning
 *     ZQN analysis
 *     deployment planning
 *
 * The grammar itself performs none of these operations.
 *
 * ============================================================================
 * 53. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may consume the semantic contract to determine:
 *
 *     health
 *     availability
 *     degradation
 *     recovery
 *     failover
 *     retry
 *     quarantine
 *     retirement
 *
 * Runtime observations MUST NOT alter source parsing.
 *
 * ============================================================================
 * 54. RESILIENCE STATE CONTRACT
 * ============================================================================
 *
 * The canonical resilience state vocabulary is:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * These remain semantic values.
 *
 * The parser does not hard-code them as a finite machine.
 *
 * This allows future states without requiring a grammar fork.
 *
 * ============================================================================
 * 55. RESILIENCE OUTCOME CONTRACT
 * ============================================================================
 *
 * The canonical outcome vocabulary is:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These are semantic outcomes.
 *
 * The parser preserves them as source values when used through properties such
 * as:
 *
 *     resilience_outcome = ACCEPT;
 *
 * Runtime/resilience analysis determines their actual meaning in context.
 *
 * ============================================================================
 * 56. SECURITY
 * ============================================================================
 *
 * This grammar MUST NOT provide a way to bypass:
 *
 *     capability checking
 *     resource checking
 *     type checking
 *     ownership
 *     effects
 *     security policy
 *     semantic validation
 *     IR validation
 *
 * Attributes and symbolic properties are not authority to bypass semantic
 * validation.
 *
 * ============================================================================
 * 57. PERFORMANCE
 * ============================================================================
 *
 * The grammar must scale with source size without introducing artificial
 * reliability-domain cardinality limits.
 *
 * Avoid:
 *
 *     fixed-size alternatives
 *     recursive physical topology enumeration
 *     provider-specific branches
 *     duplicated expression grammars
 *     semantic predicates
 *     target-dependent parser decisions
 *
 * Large reliability contracts should remain collections of independently
 * parseable semantic items.
 *
 * ============================================================================
 * 58. COMPATIBILITY
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not rename:
 *
 *     grammar/hardware/hardware.g4
 *     grammar/hardware/power.g4
 *     grammar/resources/reliability.g4
 *     grammar/declarations/resources.g4
 *
 * Existing resource-level reliability syntax remains valid.
 *
 * New hardware-level reliability syntax is introduced through:
 *
 *     hardwareReliabilityDeclaration
 *
 * Existing lower-level resource reliability clauses MUST NOT be removed
 * merely because this file exists.
 *
 * ============================================================================
 * 59. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no maximum qubit count
 *     no maximum CPU count
 *     no maximum GPU count
 *     no maximum FPGA count
 *     no maximum node count
 *     no maximum device count
 *     no maximum memory
 *     no maximum thread count
 *     no maximum redundancy
 *     no maximum failure-domain count
 *     no maximum recovery count
 *     no maximum state count
 *     no maximum outcome count
 *
 * Numeric values remain program semantics.
 *
 * Example:
 *
 *     availability >= 0.999999999;
 *
 * is a program-level reliability requirement.
 *
 * It is NOT a Zamani-wide reliability ceiling.
 *
 * ============================================================================
 * 60. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] one hardware reliability grammar owner exists;
 *     [x] resource reliability remains separately owned;
 *     [x] reliability and resilience remain distinct;
 *     [x] requirements are distinct from constraints;
 *     [x] preferences are distinct from hints;
 *     [x] properties are open-world;
 *     [x] capabilities are open-world;
 *     [x] resources are symbolic;
 *     [x] targets are abstract;
 *     [x] failure domains are logical;
 *     [x] redundancy is symbolic;
 *     [x] recovery is intent only;
 *     [x] resilience states remain extensible;
 *     [x] resilience outcomes remain extensible;
 *     [x] no physical device selection is encoded;
 *     [x] no physical topology is encoded;
 *     [x] no QEC implementation is encoded;
 *     [x] no ZQN implementation is encoded;
 *     [x] no second IR is created;
 *     [x] quantum uses the canonical quantum::ir boundary;
 *     [x] expressions remain owned by the canonical expression system;
 *     [x] source spans can be preserved;
 *     [x] semantic validation is downstream;
 *     [x] Rust implementation remains safe;
 *     [x] Rust 1.97 / 1.97.1 remains supported;
 *     [x] no artificial hardware limits are encoded;
 *     [x] POCO-REAF is preserved.
 *
 * Remaining completion work is repository integration and conformance tests,
 * not further expansion of this leaf grammar.
 *
 * ============================================================================
 */