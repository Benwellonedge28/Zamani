/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/resources.g4
 *
 * GRAMMAR
 * -------
 * parser grammar Resources
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE SUBSYSTEM ORCHESTRATOR
 *
 * PURPOSE
 * -------
 * This file is the composition root of grammar/resources/.
 *
 * It does NOT implement every resource concept itself.
 *
 * Instead it:
 *
 *   1. provides the single resource-domain entry point;
 *   2. dispatches resource declarations and resource statements;
 *   3. composes the independent resource grammars;
 *   4. provides universal resource declaration syntax;
 *   5. provides resource-body composition;
 *   6. provides target-independent lifecycle intent;
 *   7. provides open-world resource properties;
 *   8. preserves the distinction between:
 *
 *        requirement
 *        constraint
 *        capability
 *        budget
 *        preference
 *        hint
 *        negotiation
 *        scalability
 *        target intent
 *        lifecycle intent
 *
 * The specialized grammars remain the owners of their respective payloads.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar
 * Rust 2021
 * Rust 1.97 / Rust 1.97.1
 *
 * Safety requirements:
 *
 *   - no embedded Rust;
 *   - no semantic predicates;
 *   - no parser actions;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime execution;
 *   - no unsafe implementation requirement.
 *
 * The generated parser is consumed by the safe-Rust Zamani frontend.
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
 *                         ZamaniParser
 *                              |
 *                              v
 *                    +---------------------+
 *                    |      Resources      |
 *                    |   THIS ORCHESTRATOR |
 *                    +---------------------+
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *    declarations        requirements        constraints
 *          |                   |                   |
 *          v                   v                   v
 *    capabilities          budgets           preferences
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                   +----------+----------+
 *                   |          |          |
 *                   v          v          v
 *                 hints   negotiation scalability
 *                   |          |          |
 *                   +----------+----------+
 *                              |
 *                              v
 *                     domain-neutral AST
 *                              |
 *                              v
 *                     structural validation
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *          resources       capabilities     policies
 *              |               |               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *         classical       quantum::ir     HDL/hardware
 *                              |
 *                              v
 *                    optimization/lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                       resilience / QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file owns:
 *
 *   - resource composition;
 *   - resource dispatch;
 *   - resource declaration;
 *   - resource specification;
 *   - resource-body dispatch;
 *   - universal resource lifecycle syntax;
 *   - universal open-world resource properties;
 *   - resource-domain integration boundaries.
 *
 * This file does NOT own:
 *
 *   - lexical tokens;
 *   - identifier spelling;
 *   - general expressions;
 *   - capability identity;
 *   - requirement semantics;
 *   - constraint semantics;
 *   - budget semantics;
 *   - preference semantics;
 *   - hint semantics;
 *   - negotiation semantics;
 *   - scalability semantics;
 *   - policy semantics;
 *   - effect semantics;
 *   - contracts;
 *   - provenance;
 *   - hardware discovery;
 *   - physical allocation;
 *   - placement;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - quantum decomposition;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime resource management.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Direct grammar dependencies:
 *
 *   ResourceExpressions
 *   ResourceCapabilities
 *   ResourceRequirements
 *   ResourceConstraints
 *   ResourceBudgets
 *   ResourcePreferences
 *   ResourceHints
 *   ResourceNegotiation
 *   ResourceScalability
 *   Names
 *
 * Dependency direction:
 *
 *   Names
 *      |
 *      v
 *   Expressions
 *      |
 *      v
 *   ResourceExpressions
 *      |
 *      +------------------------------+
 *      |                              |
 *      v                              v
 *   resource leaf grammars       Resources
 *                                     |
 *                                     v
 *                              ZamaniParser
 *
 * No resource leaf grammar may import Resources.
 *
 * This prevents:
 *
 *   Resources -> Child -> Resources
 *
 * import cycles.
 *
 * ============================================================================
 * IMPORTANT ANTLR COMPOSITION RULE
 * ============================================================================
 *
 * All imported resource grammars MUST expose unique rule names.
 *
 * The following duplicate rule names that previously existed in the resource
 * tree MUST be normalized before this file is generated:
 *
 *   optionalResourceExpressionList
 *   resourceCapabilityExpression
 *   resourceComparisonOperator
 *   resourcePreferenceValue
 *   resourceHintValue
 *   resourceCapabilityConstraint
 *
 * Canonical ownership:
 *
 *   resource-expressions.g4
 *       optionalResourceExpressionList
 *       resourceCapabilityExpression
 *       resourceComparisonOperator
 *
 *   preferences.g4
 *       resourcePreferenceValue
 *
 *   hints.g4
 *       resourceHintValue
 *
 *   constraints.g4
 *       resourceCapabilityConstraint
 *
 * No duplicate rule may be resolved accidentally through import ordering.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar Resources;

options {
    tokenVocab = ZamaniLexer;
}

import
    ResourceExpressions,
    ResourceCapabilities,
    ResourceRequirements,
    ResourceConstraints,
    ResourceBudgets,
    ResourcePreferences,
    ResourceHints,
    ResourceNegotiation,
    ResourceScalability,
    Names
;


/*
 * ============================================================================
 * 1. RESOURCE SUBSYSTEM ENTRY POINT
 * ============================================================================
 *
 * This is the only complete resource-domain parser entry point.
 *
 * Cardinality is intentionally unbounded.
 *
 * ============================================================================
 */

resources
    : resourceItem*
    ;


/*
 * ============================================================================
 * 2. RESOURCE ITEM DISPATCH
 * ============================================================================
 *
 * Every source-level resource construct enters through this rule.
 *
 * Specialized resource grammars provide the payloads.
 *
 * This rule provides composition, not specialized semantics.
 *
 * ============================================================================
 */

resourceItem
    : resourceDeclaration
    | resourceStatement
    ;


/*
 * ============================================================================
 * 3. RESOURCE STATEMENT DISPATCH
 * ============================================================================
 *
 * The resource statement layer deliberately distinguishes mandatory intent,
 * advisory intent, negotiation, budgeting and lifecycle operations.
 *
 * ============================================================================
 */

resourceStatement
    : resourceRequirement
    | resourceConstraintStatement
    | resourceCapabilityStatement
    | resourcePreferenceStatement
    | resourceHintStatement
    | resourceBudgetStatement
    | resourceNegotiationStatement
    | resourceScalabilityStatement
    | resourceTarget
    | resourceReservation
    | resourceAcquisition
    | resourceRelease
    | resourceDerivation
    | resourceGroup
    | resourceContract
    | resourceProfile
    ;


/*
 * ============================================================================
 * 4. RESOURCE DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource accelerator: hardware::accelerator;
 *
 *     resource qpu: quantum::qpu;
 *
 *     resource workload {
 *         capacity = required_capacity;
 *         latency = latency_budget;
 *     };
 *
 * A declaration introduces a logical resource concept.
 *
 * It does NOT allocate hardware.
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
 * 5. RESOURCE KIND
 * ============================================================================
 *
 * Resource kinds are OPEN-WORLD semantic names.
 *
 * They are not a finite hardware enumeration.
 *
 * ============================================================================
 */

resourceKindClause
    : COLON resourceNamePath
    ;


resourceNamePath
    : resourceNameSegment
      (DOUBLE_COLON resourceNameSegment)*
    ;


resourceNameSegment
    : identifier
    | QUANTUM
    | NANO
    | MEMORY
    | CAPABILITY
    | TARGET
    | RESOURCE
    | RESOURCES
    ;


/*
 * ============================================================================
 * 6. RESOURCE SPECIFICATION
 * ============================================================================
 *
 * A specification contains an unbounded sequence of resource-body items.
 *
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
 * Attributes are metadata.
 *
 * They do not allocate resources.
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
      optionalResourceExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 8. RESOURCE CLAUSE DISPATCH
 * ============================================================================
 *
 * This is the internal composition boundary for resource declarations.
 *
 * Resource clauses may represent:
 *
 *   - quantities;
 *   - references;
 *   - mandatory requirements;
 *   - constraints;
 *   - capabilities;
 *   - preferences;
 *   - hints;
 *   - target intent;
 *   - capacity;
 *   - availability;
 *   - portability;
 *   - scalability;
 *   - performance;
 *   - latency;
 *   - throughput;
 *   - bandwidth;
 *   - energy;
 *   - power;
 *   - reliability;
 *   - resilience;
 *   - cost;
 *   - reservation;
 *   - acquisition;
 *   - release;
 *   - derivation;
 *   - groups;
 *   - contracts;
 *   - profiles;
 *   - open-world properties.
 *
 * ============================================================================
 */

resourceClause
    : resourceQuantityClause
    | resourceReferenceClause
    | resourceRequirementClause
    | resourceConstraintClause
    | resourceCapabilityClause
    | resourcePreferenceClause
    | resourceHintClause
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
    | resourceContractClause
    | resourceProfileClause
    | resourcePropertyClause
    ;


/*
 * ============================================================================
 * 9. RESOURCE QUANTITY
 * ============================================================================
 *
 * The value is an expression.
 *
 * It may be:
 *
 *   literal;
 *   symbolic;
 *   computed;
 *   dependent;
 *   input-derived;
 *   runtime-derived;
 *   target-derived.
 *
 * No evaluation occurs in the grammar.
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
 * 10. RESOURCE REFERENCE
 * ============================================================================
 */

resourceReferenceClause
    : REFERENCE
      ASSIGN
      resourceSelector
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. REQUIREMENT COMPOSITION
 * ============================================================================
 *
 * The complete requirement statement belongs to requirements.g4.
 *
 * This wrapper exists only so resource declarations can consume it without
 * taking ownership of its semantics.
 *
 * ============================================================================
 */

resourceRequirementClause
    : resourceRequirement
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT COMPOSITION
 * ============================================================================
 *
 * constraints.g4 owns the actual constraint payload.
 *
 * This orchestrator owns only the source-level statement boundary.
 *
 * ============================================================================
 */

resourceConstraintStatement
    : CONSTRAINT
      resourceConstraintExpression
      SEMICOLON
    ;


resourceConstraintClause
    : resourceConstraintStatement
    ;


/*
 * ============================================================================
 * 13. CAPABILITY COMPOSITION
 * ============================================================================
 *
 * The capability leaf grammar owns capability identity and capability
 * expressions.
 *
 * The orchestrator exposes only the resource-domain statement boundary.
 *
 * ============================================================================
 */

resourceCapabilityStatement
    : resourceCapabilityAssertion
    ;


resourceCapabilityClause
    : resourceCapabilityStatement
    ;


/*
 * ============================================================================
 * 14. PREFERENCE COMPOSITION
 * ============================================================================
 *
 * Preference payloads are owned by preferences.g4.
 *
 * This wrapper deliberately permits both:
 *
 *     prefer { ... };
 *
 * and individual preference clauses.
 *
 * ============================================================================
 */

resourcePreferenceStatement
    : PREFER resourcePreferenceSpecification SEMICOLON?
    | PREFER resourcePreferenceAssignment
    | PREFER resourcePreferenceObjective
    | PREFER resourcePreferenceOrdering
    | PREFER resourcePreferenceCondition
    | PREFER resourcePreferenceProperty
    | PREFER resourcePreferenceGroup
    ;


resourcePreferenceClause
    : resourcePreferenceStatement
    ;


/*
 * ============================================================================
 * 15. HINT COMPOSITION
 * ============================================================================
 *
 * Hints remain advisory.
 *
 * The hint grammar owns payload structure.
 *
 * ============================================================================
 */

resourceHintStatement
    : HINT resourceHintSpecification
    | HINT resourceHintPropertyAssignment
    | HINT resourceHintValueClause
    | HINT resourceHintGroup
    ;


resourceHintClause
    : resourceHintStatement
    ;


/*
 * ============================================================================
 * 16. BUDGET COMPOSITION
 * ============================================================================
 *
 * budgets.g4 owns the complete budget declaration.
 *
 * ============================================================================
 */

resourceBudgetStatement
    : resourceBudgetDeclaration
    ;


resourceBudgetClause
    : resourceBudgetStatement
    ;


/*
 * ============================================================================
 * 17. NEGOTIATION COMPOSITION
 * ============================================================================
 *
 * negotiation.g4 owns negotiation payloads.
 *
 * ============================================================================
 */

resourceNegotiationStatement
    : NEGOTIATE
      resourceNegotiationSpecification
      SEMICOLON?
    ;


resourceNegotiationClause
    : resourceNegotiationStatement
    ;


/*
 * ============================================================================
 * 18. SCALABILITY COMPOSITION
 * ============================================================================
 *
 * scalability.g4 owns scalability payload syntax.
 *
 * The payload already contains the SCALABILITY introducer for its direct
 * declaration forms, so this orchestrator does not add another keyword.
 *
 * ============================================================================
 */

resourceScalabilityStatement
    : resourceScalabilityDeclaration
    | resourceScalabilityNamedDeclaration
    | resourceScalabilityRelationship
    ;


resourceScalabilityClause
    : resourceScalabilityStatement
    ;


/*
 * ============================================================================
 * 19. TARGET INTENT
 * ============================================================================
 *
 * A target is an abstract execution-domain intent.
 *
 * It is NOT a physical device selector.
 *
 * Valid semantic examples include:
 *
 *     target = execution_domain;
 *     target quantum::execution;
 *     target hardware::accelerator;
 *
 * The semantic layer determines whether the referenced target exists and is
 * compatible.
 *
 * ============================================================================
 */

resourceTarget
    : TARGET
      resourceTargetExpression
      SEMICOLON
    ;


resourceTargetExpression
    : resourceTargetSymbol
    | resourceExpression
    ;


resourceTargetSymbol
    : identifier
    | qualifiedName
    ;


resourceTargetClause
    : resourceTarget
    ;


/*
 * ============================================================================
 * 20. RESOURCE CAPACITY
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
 * 21. RESOURCE AVAILABILITY
 * ============================================================================
 *
 * This describes semantic availability information.
 *
 * The grammar does not inspect the environment.
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
 * 22. RESOURCE PORTABILITY
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
 * 23. RESOURCE SCALABILITY
 * ============================================================================
 *
 * The simple resource-body form remains supported:
 *
 *     scalability = scaling_model;
 *
 * The richer top-level scalability model belongs to scalability.g4.
 *
 * ============================================================================
 */

resourceScalabilityBodyClause
    : SCALABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. PERFORMANCE
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
 * 25. LATENCY
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
 * 26. THROUGHPUT
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
 * 27. BANDWIDTH
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
 * 28. ENERGY
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
 * 29. POWER
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
 * 30. RELIABILITY
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
 * 31. RESILIENCE
 * ============================================================================
 *
 * Resilience states/outcomes remain semantic vocabulary.
 *
 * This grammar does not execute recovery.
 *
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
 * 32. COST
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
 * 33. RESERVATION
 * ============================================================================
 *
 * Declarative reservation intent.
 *
 * Actual reservation is downstream.
 *
 * ============================================================================
 */

resourceReservation
    : RESERVE
      resourceExpression
      SEMICOLON
    ;


resourceReservationClause
    : RESERVATION
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. ACQUISITION
 * ============================================================================
 *
 * Declarative acquisition intent.
 *
 * Actual acquisition belongs to semantic/runtime systems.
 *
 * ============================================================================
 */

resourceAcquisition
    : ACQUIRE
      resourceExpression
      SEMICOLON
    ;


resourceAcquisitionClause
    : ACQUISITION
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 35. RELEASE
 * ============================================================================
 */

resourceRelease
    : RELEASE
      resourceExpression
      SEMICOLON
    ;


resourceReleaseClause
    : RELEASE_KW
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 36. DERIVATION
 * ============================================================================
 *
 * Resource derivation describes a semantic relationship between resource
 * quantities or properties.
 *
 * It does not perform allocation.
 *
 * ============================================================================
 */

resourceDerivation
    : DERIVE
      resourceExpression
      SEMICOLON
    ;


resourceDerivationClause
    : DERIVE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 37. RESOURCE GROUP
 * ============================================================================
 *
 * Groups are logical resource groupings.
 *
 * They do not represent a fixed number of devices.
 *
 * ============================================================================
 */

resourceGroup
    : RESOURCE_GROUP
      resourceGroupClauseBody
      SEMICOLON
    ;


resourceGroupClause
    : RESOURCE_GROUP
      resourceGroupClauseBody
      SEMICOLON
    ;


resourceGroupClauseBody
    : resourceExpression
    ;


/*
 * ============================================================================
 * 38. RESOURCE CONTRACT
 * ============================================================================
 *
 * This is a resource-domain structural boundary.
 *
 * Universal contract semantics remain owned by validation/specification.
 *
 * ============================================================================
 */

resourceContract
    : CONTRACT
      resourceContractClauseBody
      SEMICOLON
    ;


resourceContractClause
    : CONTRACT
      resourceContractClauseBody
      SEMICOLON
    ;


resourceContractClauseBody
    : resourceExpression
    ;


/*
 * ============================================================================
 * 39. RESOURCE PROFILE
 * ============================================================================
 *
 * Profiles are logical resource descriptions.
 *
 * They do not represent physical machine selection.
 *
 * ============================================================================
 */

resourceProfile
    : PROFILE
      resourceProfileClauseBody
      SEMICOLON
    ;


resourceProfileClause
    : PROFILE
      resourceProfileClauseBody
      SEMICOLON
    ;


resourceProfileClauseBody
    : resourceExpression
    ;


/*
 * ============================================================================
 * 40. OPEN-WORLD RESOURCE PROPERTY
 * ============================================================================
 *
 * This is one of the most important scalability mechanisms.
 *
 * A new resource dimension does NOT require a new universal keyword.
 *
 * Examples:
 *
 *     thermal::margin = required_margin;
 *
 *     quantum::fidelity = required_fidelity;
 *
 *     accelerator::occupancy = desired_occupancy;
 *
 *     tensor::memory = tensor_memory;
 *
 *     network::topology = required_topology;
 *
 *     future::resource::property = symbolic_value;
 *
 * ============================================================================
 */

resourcePropertyClause
    : qualifiedName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 41. RESOURCE PROPERTY PATH
 * ============================================================================
 *
 * Qualified names remain the canonical name authority.
 *
 * This rule is intentionally an alias boundary for consumers that need a
 * resource-specific property rule.
 *
 * ============================================================================
 */

resourcePropertyPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 42. RESOURCE EXPRESSION STATEMENT
 * ============================================================================
 *
 * Resource expressions may be exposed directly to the resource domain.
 *
 * The root Zamani parser already has the universal expression architecture.
 *
 * This wrapper is intentionally separate from resourceExpression itself.
 *
 * ============================================================================
 */

resourceExpressionStatement
    : resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 43. RESOURCE SEMANTIC SEPARATION
 * ============================================================================
 *
 * The parser preserves the following distinctions:
 *
 *   requirement != constraint
 *   constraint != capability
 *   capability != preference
 *   preference != hint
 *   budget != requirement
 *   budget != allocation
 *   negotiation != allocation
 *   scalability != hardware size limit
 *   target intent != physical target selection
 *   resource declaration != resource allocation
 *
 * Semantic analysis may relate these concepts, but the grammar does not
 * collapse them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Resource intent follows:
 *
 *     source
 *       |
 *       v
 *     resource intent
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     capability analysis
 *       |
 *       v
 *     resource negotiation
 *       |
 *       v
 *     execution planning
 *       |
 *       v
 *     target realization
 *
 * The source program describes what it needs.
 *
 * It does not silently select:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     ASIC 0
 *     QPU 0
 *     physical qubit 0
 *     memory bank 0
 *     network node 0
 *
 * A target that cannot satisfy a mandatory requirement must produce a defined
 * semantic/resource diagnostic or use an explicitly declared valid fallback.
 *
 * It must not silently alter program meaning.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * This grammar does not enumerate:
 *
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     accelerators
 *     memory banks
 *     network links
 *     tensor ranks
 *     register widths
 *     topology sizes
 *
 * New resource dimensions are represented through:
 *
 *     identifiers
 *     qualified names
 *     resource expressions
 *     capabilities
 *     requirements
 *     constraints
 *     policies
 *
 * Consequently, adding a future computational technology does not require
 * changing this orchestrator merely because the technology introduces a new
 * resource concept.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no language-level maximum on:
 *
 *     resource declarations
 *     resource statements
 *     resource clauses
 *     resource properties
 *     requirements
 *     constraints
 *     capabilities
 *     budgets
 *     preferences
 *     hints
 *     negotiation clauses
 *     scalability clauses
 *     resource groups
 *     expression depth
 *     qualified-name depth
 *     resource domains
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * "Unbounded" means:
 *
 *     no artificial language-defined ceiling.
 *
 * It does not claim that:
 *
 *     physical hardware
 *     memory
 *     compiler resources
 *     runtime resources
 *     operating systems
 *     networks
 *
 * are physically infinite.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing resource syntax has no runtime effect.
 *
 * This grammar does not:
 *
 *     allocate;
 *     release;
 *     reserve;
 *     discover;
 *     inspect hardware;
 *     contact networks;
 *     perform measurements;
 *     execute foreign code;
 *     perform native execution.
 *
 * If a resource-related semantic operation later produces an effect, that
 * effect belongs to the canonical effect/semantic system.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability identity is owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Resource capability intent is owned by:
 *
 *     grammar/resources/capabilities.g4
 *
 * This orchestrator only composes those authorities.
 *
 * The grammar never assumes that mentioning a capability proves availability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. TYPE CONTRACT
 * ============================================================================
 *
 * Resource values are expressions.
 *
 * Therefore:
 *
 *     type checking
 *     unit checking
 *     dimensional checking
 *     numeric compatibility
 *     symbolic dependency checking
 *
 * belong to the canonical type/semantic layers.
 *
 * This file does not establish:
 *
 *     fixed integer widths;
 *     fixed floating-point widths;
 *     fixed qubit-count types;
 *     fixed device-count types;
 *     fixed memory-size types;
 *     fixed tensor ranks.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Resource syntax may be governed by:
 *
 *     contracts
 *     assumptions
 *     guarantees
 *     policies
 *     authorization
 *     security
 *     adaptation
 *
 * Those semantics remain outside this file.
 *
 * A resource negotiation or preference cannot weaken a mandatory contract or
 * security prohibition.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Downstream AST construction MUST preserve source provenance for:
 *
 *     resource declaration;
 *     resource kind;
 *     resource property;
 *     requirement;
 *     constraint;
 *     capability;
 *     budget;
 *     preference;
 *     hint;
 *     negotiation;
 *     scalability;
 *     target;
 *     lifecycle operation.
 *
 * Later semantic resource decisions MAY additionally record:
 *
 *     selected realization;
 *     rejected realization;
 *     capability evidence;
 *     policy decision;
 *     fallback;
 *     reason;
 *     transformation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The domain-neutral AST layer owns semantic node construction.
 *
 * Recommended semantic AST categories:
 *
 *     ResourceDeclaration
 *     ResourceRequirement
 *     ResourceConstraint
 *     ResourceCapabilityIntent
 *     ResourceBudget
 *     ResourcePreference
 *     ResourceHint
 *     ResourceNegotiation
 *     ResourceScalability
 *     ResourceTarget
 *     ResourceReservation
 *     ResourceAcquisition
 *     ResourceRelease
 *     ResourceDerivation
 *     ResourceGroup
 *     ResourceContract
 *     ResourceProfile
 *     ResourceProperty
 *
 * AST nodes must preserve:
 *
 *     source span;
 *     source ordering;
 *     expression structure;
 *     qualified names;
 *     attributes;
 *     semantic category.
 *
 * Physical resource handles MUST NOT appear in the domain-neutral AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Resource semantics are consumed by the canonical semantic model.
 *
 * Resource information may subsequently contribute to:
 *
 *     classical IR;
 *     quantum::ir metadata;
 *     HDL/hardware intent;
 *     distributed execution plans;
 *     accelerator planning;
 *     scheduling;
 *     routing;
 *     resilience;
 *     deployment.
 *
 * There is deliberately no separate universal "resource IR" created here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum resource information follows:
 *
 *     resource syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition/routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience/QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * This grammar never assigns physical qubits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Resource information may describe semantic intent for:
 *
 *     timing;
 *     power;
 *     energy;
 *     memory;
 *     bandwidth;
 *     throughput;
 *     reliability;
 *     resilience;
 *     accelerator requirements;
 *     hardware/software partitioning.
 *
 * It does not select:
 *
 *     a physical FPGA;
 *     a physical ASIC;
 *     a physical register;
 *     a physical wire;
 *     a physical memory bank;
 *     a physical device.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. BACKEND CONTRACT
 * ============================================================================
 *
 * Backend systems consume semantic resource information after parsing and
 * validation.
 *
 * They may use it for:
 *
 *     target matching;
 *     specialization;
 *     placement;
 *     routing;
 *     scheduling;
 *     parallelization;
 *     vectorization;
 *     accelerator selection;
 *     quantum realization;
 *     simulation;
 *     deployment.
 *
 * None of these decisions occur in this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics MUST identify structural failures such as:
 *
 *     missing resource name;
 *     malformed resource kind;
 *     malformed resource block;
 *     missing assignment value;
 *     missing semicolon;
 *     malformed requirement;
 *     malformed constraint;
 *     malformed capability statement;
 *     malformed preference;
 *     malformed hint;
 *     malformed budget;
 *     malformed negotiation;
 *     malformed scalability declaration.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unknown resource;
 *     unknown capability;
 *     unavailable capability;
 *     invalid unit;
 *     incompatible dimensions;
 *     contradictory requirements;
 *     contradictory constraints;
 *     unsatisfiable resource set;
 *     policy conflict;
 *     security violation;
 *     impossible realization;
 *     unsupported target.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing resource intent must remain representable where its semantics are
 * still supported.
 *
 * In particular, the orchestrator preserves:
 *
 *     resource <name>;
 *
 *     resource <name>: <kind>;
 *
 *     resource <name> { ... };
 *
 *     requires <expression>;
 *
 *     constraint <expression>;
 *
 *     capability <expression>;
 *
 *     prefer ...;
 *
 *     hint ...;
 *
 *     budget ...;
 *
 *     target ...;
 *
 *     reserve ...;
 *
 *     acquire ...;
 *
 *     release ...;
 *
 *     derive ...;
 *
 * and open-world property assignments.
 *
 * Incompatible changes require:
 *
 *     language-version documentation;
 *     migration guidance;
 *     compatibility tests.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Identical source text and identical lexer/parser configuration must produce
 * structurally equivalent parse trees.
 *
 * This grammar contains no:
 *
 *     actions;
 *     semantic predicates;
 *     I/O;
 *     environment inspection;
 *     hardware inspection;
 *     randomness.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. HARD-CODING AUDIT
 * ============================================================================
 *
 * THIS FILE CONTAINS:
 *
 *     NO maximum resource count;
 *     NO maximum device count;
 *     NO maximum CPU count;
 *     NO maximum GPU count;
 *     NO maximum FPGA count;
 *     NO maximum QPU count;
 *     NO maximum qubit count;
 *     NO maximum node count;
 *     NO maximum memory capacity;
 *     NO maximum thread count;
 *     NO maximum tensor rank;
 *     NO maximum register width;
 *     NO maximum network size;
 *     NO physical device enumeration;
 *     NO vendor device enumeration;
 *     NO physical topology enumeration.
 *
 * Numeric values remain ordinary program expressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 61. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource accelerator: hardware::accelerator;
 *
 *     resource qpu: quantum::qpu;
 *
 *     requires memory >= required_memory;
 *
 *     requires qubits >= required_qubits;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     constraint latency <= latency_budget;
 *
 *     capability quantum::measurement;
 *
 *     prefer latency <= latency_goal;
 *
 *     hint quantum::routing = routing_goal;
 *
 *     budget memory = required_memory;
 *
 *     negotiate {
 *         capability = capability("tensor.compute");
 *     };
 *
 *     scalability work = problem_size;
 *
 *     target quantum::execution;
 *
 *     reserve reservation_capacity;
 *
 *     acquire execution_capacity;
 *
 *     release execution_capacity;
 *
 *     derive required_memory;
 *
 *     resource workload {
 *         capacity = workload_capacity;
 *         latency = latency_budget;
 *         quantum::logical_qubits = logical_qubits;
 *         future::resource::metric = symbolic_value;
 *     };
 *
 * NEGATIVE
 * --------
 *
 *     resource;
 *
 *     resource : memory;
 *
 *     resource compute {
 *
 *     requires;
 *
 *     constraint;
 *
 *     capability;
 *
 *     budget;
 *
 *     target;
 *
 *     reserve;
 *
 *     acquire;
 *
 *     release;
 *
 *     derive;
 *
 * BOUNDARY
 * --------
 *
 *     deeply qualified resource names;
 *     symbolic resource values;
 *     computed resource values;
 *     nested resource groups;
 *     quantum resource requirements;
 *     classical resource requirements;
 *     accelerator requirements;
 *     distributed resource requirements;
 *     networking requirements;
 *     HDL resource intent;
 *     AI/tensor resource intent;
 *     future resource dimensions.
 *
 * SCALABILITY
 * ----------
 *
 * Tests must demonstrate that the grammar has no source-level finite ceiling
 * on:
 *
 *     resource items;
 *     resource clauses;
 *     qualified-name depth;
 *     symbolic resource dimensions;
 *     requirement collections;
 *     capability collections;
 *     negotiation clauses.
 *
 * DETERMINISM
 * -----------
 *
 * Identical input/configuration must yield structurally equivalent parser
 * output.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 62. INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/capabilities.g4
 *     grammar/expressions/*
 *     grammar/resources/resource-expressions.g4
 *
 * THIS FILE
 * ---------
 *
 *     grammar/resources/resources.g4
 *
 * DOWNSTREAM RESOURCE COMPONENTS
 * ------------------------------
 *
 *     grammar/resources/capabilities.g4
 *     grammar/resources/requirements.g4
 *     grammar/resources/constraints.g4
 *     grammar/resources/budgets.g4
 *     grammar/resources/preferences.g4
 *     grammar/resources/hints.g4
 *     grammar/resources/negotiation.g4
 *     grammar/resources/scalability.g4
 *
 * DOWNSTREAM LANGUAGE SYSTEMS
 * ---------------------------
 *
 *     grammar/validation/
 *     grammar/effects/
 *     grammar/security/
 *     grammar/policies/
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/ai/
 *     grammar/interoperability/
 *
 * PARSER COMPOSITION ROOT
 * -----------------------
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The root parser imports:
 *
 *     Resources
 *
 * and consumes:
 *
 *     resourceElement
 *
 * Therefore this file MUST retain:
 *
 *     resources
 *     resourceItem
 *     resourceDeclaration
 *     resourceStatement
 *
 * as stable public integration rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 63. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *   1. It is the only resource-directory composition root.
 *
 *   2. ZamaniParser imports Resources successfully.
 *
 *   3. `resourceElement` can consume:
 *
 *        resourceDeclaration
 *        resourceStatement
 *        resourceExpression
 *
 *      through the existing root-parser integration.
 *
 *   4. All resource leaf grammars are imported.
 *
 *   5. No leaf grammar imports this file.
 *
 *   6. No duplicate imported rule names remain.
 *
 *   7. Requirements remain requirements.
 *
 *   8. Constraints remain constraints.
 *
 *   9. Capabilities remain capabilities.
 *
 *  10. Budgets remain budgets.
 *
 *  11. Preferences remain advisory.
 *
 *  12. Hints remain advisory.
 *
 *  13. Negotiation remains declarative.
 *
 *  14. Scalability remains semantic and open-world.
 *
 *  15. Resource declarations remain target-independent.
 *
 *  16. Physical allocation remains outside grammar/.
 *
 *  17. Quantum information reaches quantum::ir only downstream.
 *
 *  18. HDL/hardware information remains target-independent until lowering.
 *
 *  19. No universal hardware-size constant exists.
 *
 *  20. No parser action or embedded Rust exists.
 *
 *  21. Generated Rust remains compatible with Rust 1.97 / 1.97.1.
 *
 *  22. The consuming implementation remains safe Rust.
 *
 *  23. Positive, negative, boundary, scalability, compatibility and
 *      determinism tests pass.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */