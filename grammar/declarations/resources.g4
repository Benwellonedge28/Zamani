/*
 * ============================================================================
 * Zamani — Universal Resource Declarations
 * File: grammar/declarations/resources.g4
 *
 * PURPOSE
 * -------
 * Defines target-independent resource declarations and resource contracts.
 *
 * This grammar describes:
 *   - resources
 *   - resource groups
 *   - resource contracts
 *   - resource profiles
 *   - requirements
 *   - hard constraints
 *   - preferences
 *   - hints
 *   - capabilities
 *   - targets
 *   - resource properties
 *   - quantities/capacities
 *   - availability
 *   - performance
 *   - latency/throughput/bandwidth
 *   - energy/power
 *   - reliability/resilience
 *   - cost
 *   - portability/scalability
 *   - reservation/acquisition/release intent
 *   - derived resource values
 *   - extensible resource attributes
 *
 * ARCHITECTURAL RULES
 * -------------------
 * 1. This file owns RESOURCE SYNTAX ONLY.
 * 2. It does not own resource semantics.
 * 3. It does not allocate resources.
 * 4. It does not identify physical devices.
 * 5. It does not encode hardware limits.
 * 6. It does not create a second IR.
 * 7. Resource expressions are delegated to ResourceExpressions.
 * 8. Names are delegated to the canonical Names grammar.
 * 9. The semantic layer determines whether a resource declaration is:
 *      - a requirement,
 *      - a constraint,
 *      - a preference,
 *      - a hint,
 *      - a capability requirement,
 *      - a target intent,
 *      - or runtime resource lifecycle intent.
 * 10. Physical realization belongs downstream to hardware/HAL/routing/
 *     scheduling/runtime infrastructure.
 *
 * POCO-REAF
 * ---------
 * Programs may express resource intent without specifying:
 *   - CPU count
 *   - GPU count
 *   - FPGA count
 *   - QPU count
 *   - qubit limit
 *   - node count
 *   - memory limit
 *   - thread limit
 *   - register width
 *   - tensor dimension limit
 *   - topology size
 *   - physical device ID
 *
 * Any quantity may be an expression. Therefore resource requirements may
 * depend on program semantics, target capabilities, configuration, negotiated
 * resources, or runtime information.
 *
 * SAFETY
 * ------
 * No embedded actions, predicates, semantic callbacks, or unsafe Rust code.
 *
 * RUST
 * ----
 * This grammar is consumed by the Zamani Rust toolchain and introduces no
 * Rust-specific implementation dependency. The Rust implementation remains
 * safe Rust; no unsafe code is required by this grammar.
 * ============================================================================
 */

parser grammar Resources;

/*
 * IMPORTANT:
 * The repository's lexer source is currently organized around
 * grammar/lexer/tokens.g4, whose lexer grammar is ZamaniTokens.
 *
 * Parser grammars must converge on that canonical vocabulary rather than
 * inventing legacy names such as RESOURCE, REQUIREMENT, EQUALS, etc.
 */
options {
    tokenVocab = ZamaniTokens;
}

import ResourceExpressions, Names;


// ============================================================================
// 1. RESOURCE DECLARATIONS
// ============================================================================

/*
 * Top-level or nested universal resource declaration.
 *
 * Examples:
 *
 *   resource compute {
 *       ...
 *   }
 *
 *   resource compute : ComputeResource {
 *       ...
 *   }
 *
 *   resource group cluster {
 *       ...
 *   }
 *
 *   resource contract execution {
 *       ...
 *   }
 *
 *   resource profile portable_compute {
 *       ...
 *   }
 *
 * The optional kind is semantic metadata represented by the declaration
 * structure; it does not impose a hardware model.
 */
resourceDeclaration
    : K_RESOURCE resourceDeclarationKind? resourceName
      resourceTypeClause?
      resourceBody?
      SEMICOLON?
    ;


/*
 * Reusable resource categories.
 *
 * GROUP:
 *   describes a logical collection of resources.
 *
 * CONTRACT:
 *   describes an externally consumable resource contract.
 *
 * PROFILE:
 *   describes a reusable target/resource profile.
 */
resourceDeclarationKind
    : K_GROUP
    | K_CONTRACT
    | K_PROFILE
    ;


/*
 * A collection-level resource block.
 *
 * This is deliberately separate from an individual resource declaration so
 * that a program can describe an unbounded resource set without introducing
 * fixed cardinality into the grammar.
 */
resourcesDeclaration
    : K_RESOURCES resourceBody
      SEMICOLON?
    ;


resourceBody
    : LBRACE resourceMember* RBRACE
    ;


resourceNestedDeclaration
    : K_RESOURCE resourceDeclarationKind? resourceName
      resourceTypeClause?
      resourceBody?
      SEMICOLON?
    ;


resourceName
    : identifier
    ;


resourceTypeClause
    : COLON qualifiedName
    ;


// ============================================================================
// 2. RESOURCE MEMBERS
// ============================================================================

resourceMember
    : resourceNestedDeclaration

    | resourceRequirementClause
    | resourceConstraintClause
    | resourcePreferenceClause
    | resourceHintClause
    | resourceCapabilityClause
    | resourceTargetClause

    | resourcePropertyClause
    | resourceQuantityClause
    | resourceCapacityClause
    | resourceAvailabilityClause

    | resourcePerformanceClause
    | resourceLatencyClause
    | resourceThroughputClause
    | resourceBandwidthClause

    | resourceEnergyClause
    | resourcePowerClause

    | resourceReliabilityClause
    | resourceResilienceClause

    | resourceCostClause
    | resourcePortabilityClause
    | resourceScalabilityClause

    | resourceReservationClause
    | resourceAcquisitionClause
    | resourceReleaseClause

    | resourceDerivationClause

    | resourceAttribute
    ;


// ============================================================================
// 3. SEMANTIC RESOURCE INTENT
// ============================================================================

/*
 * REQUIREMENT
 * -----------
 * A requirement describes something that must be satisfied for the program
 * or resource contract to be semantically valid.
 *
 * Both "require" and "requires" are supported because both forms already
 * exist in the repository's documented keyword vocabulary.
 */
resourceRequirementClause
    : K_REQUIRE resourceExpression SEMICOLON
    | K_REQUIRES resourceExpression SEMICOLON
    ;


/*
 * CONSTRAINT
 * ----------
 * A hard restriction.
 *
 * The semantic layer determines satisfiability and enforcement.
 */
resourceConstraintClause
    : K_CONSTRAINT resourceExpression SEMICOLON
    ;


/*
 * PREFERENCE
 * ----------
 * An optimization preference.
 *
 * A preference is NOT equivalent to a correctness requirement.
 */
resourcePreferenceClause
    : K_PREFER resourceExpression SEMICOLON
    | K_PREFERENCE resourceExpression SEMICOLON
    ;


/*
 * HINT
 * ----
 * A non-binding optimization/runtime suggestion.
 */
resourceHintClause
    : K_HINT resourceExpression SEMICOLON
    ;


/*
 * CAPABILITY
 * ----------
 * Expresses capability requirements or capability-related resource intent.
 *
 * Capability names remain open-world through resourceExpression/qualifiedName.
 * This prevents every future accelerator, QPU technology, network facility,
 * AI engine, or computational substrate from requiring a new grammar keyword.
 */
resourceCapabilityClause
    : K_CAPABILITY resourceExpression SEMICOLON
    ;


/*
 * TARGET
 * ------
 * Describes abstract target intent/profile.
 *
 * It does NOT mean a physical device identifier.
 */
resourceTargetClause
    : K_TARGET resourceExpression SEMICOLON
    ;


// ============================================================================
// 4. GENERIC RESOURCE PROPERTIES
// ============================================================================

/*
 * Extensible property:
 *
 *   property memory.capacity = required_memory;
 *   property vendor.feature = capability_expression;
 *
 * qualifiedName keeps the namespace open-ended.
 */
resourcePropertyClause
    : K_PROPERTY qualifiedName ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 5. QUANTITY AND CAPACITY
// ============================================================================

resourceQuantityClause
    : K_QUANTITY ASSIGN resourceExpression SEMICOLON
    ;


resourceCapacityClause
    : K_CAPACITY ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 6. AVAILABILITY
// ============================================================================

resourceAvailabilityClause
    : K_AVAILABILITY ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 7. PERFORMANCE
// ============================================================================

resourcePerformanceClause
    : K_PERFORMANCE ASSIGN resourceExpression SEMICOLON
    ;


resourceLatencyClause
    : K_LATENCY ASSIGN resourceExpression SEMICOLON
    ;


resourceThroughputClause
    : K_THROUGHPUT ASSIGN resourceExpression SEMICOLON
    ;


resourceBandwidthClause
    : K_BANDWIDTH ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 8. ENERGY AND POWER
// ============================================================================

resourceEnergyClause
    : K_ENERGY ASSIGN resourceExpression SEMICOLON
    ;


resourcePowerClause
    : K_POWER ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 9. RELIABILITY AND RESILIENCE
// ============================================================================

resourceReliabilityClause
    : K_RELIABILITY ASSIGN resourceExpression SEMICOLON
    ;


resourceResilienceClause
    : K_RESILIENCE ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 10. COST
// ============================================================================

resourceCostClause
    : K_COST ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 11. PORTABILITY AND SCALABILITY
// ============================================================================

resourcePortabilityClause
    : K_PORTABILITY ASSIGN resourceExpression SEMICOLON
    ;


resourceScalabilityClause
    : K_SCALABILITY ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 12. RESOURCE LIFECYCLE INTENT
// ============================================================================

/*
 * These are INTENTS.
 *
 * They do not directly acquire hardware from the parser.
 *
 * Runtime/resource-management layers decide how the intent is realized.
 */
resourceReservationClause
    : K_RESERVE resourceExpression SEMICOLON
    ;


resourceAcquisitionClause
    : K_ACQUIRE resourceExpression SEMICOLON
    ;


resourceReleaseClause
    : K_RELEASE resourceExpression SEMICOLON
    ;


// ============================================================================
// 13. DERIVED RESOURCES
// ============================================================================

/*
 * Allows a resource property/value to be derived from another expression.
 *
 * Example:
 *
 *   derive required_memory = tensor.elements * element_size;
 *
 * The grammar intentionally does not constrain the magnitude or number of
 * derived values.
 */
resourceDerivationClause
    : K_DERIVE identifier ASSIGN resourceExpression SEMICOLON
    ;


// ============================================================================
// 14. RESOURCE ATTRIBUTES
// ============================================================================

/*
 * Resource-specific metadata remains extensible.
 *
 * Example:
 *
 *   @portable
 *   @domain::quantum(...)
 *   @deployment::policy(...)
 *
 * Attribute semantics belong to semantic analysis/tooling.
 */
resourceAttribute
    : AT qualifiedName
      resourceAttributeArguments?
    ;


resourceAttributeArguments
    : LPAREN resourceArgumentList? RPAREN
    ;


resourceArgumentList
    : resourceExpression
      (COMMA resourceExpression)*
    ;


// ============================================================================
// 15. RESOURCE EXPRESSION BOUNDARY
// ============================================================================

/*
 * RESOURCE EXPRESSIONS ARE NOT DEFINED HERE.
 *
 * ResourceExpressions owns the reusable expression boundary.
 *
 * This prevents:
 *
 *   declarations/resources.g4
 *   resources/resource-expressions.g4
 *   expressions/*.g4
 *
 * from creating competing expression implementations.
 */
resourceExpression
    : expression
    ;


// ============================================================================
// 16. INTEGRATION CONTRACT
// ============================================================================

/*
 * AST CONTRACT
 * ------------
 *
 * resourceDeclaration
 *     -> ResourceDeclaration
 *
 * resourceDeclarationKind
 *     -> ResourceKind
 *
 * resourceRequirementClause
 *     -> ResourceIntent::Requirement
 *
 * resourceConstraintClause
 *     -> ResourceIntent::Constraint
 *
 * resourcePreferenceClause
 *     -> ResourceIntent::Preference
 *
 * resourceHintClause
 *     -> ResourceIntent::Hint
 *
 * resourceCapabilityClause
 *     -> ResourceIntent::Capability
 *
 * resourceTargetClause
 *     -> ResourceIntent::Target
 *
 * resourcePropertyClause
 *     -> ResourceProperty
 *
 * quantity/capacity/availability/performance/etc.
 *     -> ResourceProperty
 *
 * reserve/acquire/release
 *     -> ResourceLifecycleIntent
 *
 * resourceDerivationClause
 *     -> DerivedResourceValue
 *
 * resourceAttribute
 *     -> Attribute
 *
 *
 * SEMANTIC CONTRACT
 * -----------------
 *
 * The semantic layer MUST distinguish:
 *
 *   requirement
 *   constraint
 *   preference
 *   hint
 *   capability
 *   target
 *   implementation decision
 *
 * These concepts MUST NOT collapse into a single "resource requirement"
 * category.
 *
 *
 * IR CONTRACT
 * -----------
 *
 * This grammar does NOT create a resource IR.
 *
 * Resource declarations lower into the repository's existing semantic
 * resource/capability representation and subsequently into the appropriate
 * compiler/runtime resource structures.
 *
 * Quantum resource intent ultimately participates in the existing:
 *
 *   quantum::ir
 *       -> optimization
 *       -> routing
 *       -> scheduling
 *       -> QEC/resilience
 *       -> ZQN
 *       -> HAL
 *       -> target realization
 *
 * This grammar must never introduce a second quantum IR.
 *
 *
 * HARDWARE CONTRACT
 * -----------------
 *
 * This grammar may describe:
 *
 *   capability("quantum.measurement")
 *   memory >= required_memory
 *   latency <= allowed_latency
 *   bandwidth >= required_bandwidth
 *   reliability >= required_reliability
 *
 * It must NOT require:
 *
 *   physical_qubit(17)
 *   cpu(3)
 *   gpu(0)
 *   node(12)
 *
 * as the universal representation of resource intent.
 *
 *
 * SCALABILITY CONTRACT
 * --------------------
 *
 * There are no grammar-level cardinality limits.
 *
 * No MAX_* constants are encoded by this grammar.
 *
 * Resource quantities may be:
 *
 *   literals
 *   variables
 *   symbolic expressions
 *   generic expressions
 *   capability-derived values
 *   configuration-derived values
 *   negotiated values
 *   runtime-derived values
 *
 *
 * DIAGNOSTICS CONTRACT
 * --------------------
 *
 * Parser diagnostics originate from the parser infrastructure and retain
 * source locations.
 *
 * Semantic diagnostics such as:
 *
 *   unsatisfied requirement
 *   contradictory constraint
 *   unavailable capability
 *   invalid resource type
 *   incompatible target
 *   impossible portability contract
 *
 * belong to semantic/resource analysis, not this grammar.
 *
 *
 * DETERMINISM CONTRACT
 * --------------------
 *
 * The grammar contains no semantic predicates or target-dependent actions.
 * Equivalent token streams must produce equivalent parse structures.
 *
 *
 * SECURITY CONTRACT
 * -----------------
 *
 * Resource declarations do not grant authority by themselves.
 *
 * Capability, acquisition, release, deployment, identity, authorization,
 * secrets, and security policy remain separate semantic/runtime concerns.
 *
 *
 * COMPLETION CRITERIA
 * -------------------
 *
 * This file is complete when:
 *
 * [x] Resource declaration syntax is defined.
 * [x] Resource groups are defined.
 * [x] Resource contracts are defined.
 * [x] Resource profiles are defined.
 * [x] Requirements are distinct from constraints.
 * [x] Preferences are distinct from requirements.
 * [x] Hints are non-binding.
 * [x] Capabilities are open-world.
 * [x] Targets remain abstract.
 * [x] Generic properties are extensible.
 * [x] Quantities are expression-valued.
 * [x] Capacities are expression-valued.
 * [x] Availability is expression-valued.
 * [x] Performance metrics are expression-valued.
 * [x] Energy/power are expression-valued.
 * [x] Reliability/resilience are expression-valued.
 * [x] Cost is expression-valued.
 * [x] Portability/scalability are expression-valued.
 * [x] Lifecycle intent is represented.
 * [x] Derived resources are represented.
 * [x] Attributes are extensible.
 * [x] No hardware cardinality is hard-coded.
 * [x] No physical device IDs are required.
 * [x] No second quantum IR is introduced.
 * [x] Expressions are delegated to the shared expression architecture.
 * [x] Names are delegated to the shared name architecture.
 * [x] No Rust action code exists.
 * [x] No unsafe implementation is required.
 *
 * Downstream completion additionally requires AST, semantic, resource-manager,
 * compiler, runtime, and conformance tests to implement the contracts above.
 * ============================================================================
 */