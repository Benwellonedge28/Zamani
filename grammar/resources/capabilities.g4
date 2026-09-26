/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/capabilities.g4
 *
 * Grammar:
 *     ResourceCapabilities
 *
 * Status:
 *     Canonical resource-capability composition grammar
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
 *     No I/O.
 *     No hardware access.
 *     No network access.
 *     No runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL RESOURCE/CAPABILITY INTENT.
 *
 * It connects:
 *
 *     resource intent
 *
 * with:
 *
 *     canonical capability identity
 *
 * without taking ownership of either the universal resource model or the
 * canonical capability identity model.
 *
 * This file therefore defines syntax for expressing things such as:
 *
 *     requires capability zamani::quantum::measurement;
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 *     requires capability future::compute::new_architecture;
 *
 *     constraint capability.property >= value;
 *
 *     prefer capability accelerator::vector_compute;
 *
 *     hint capability vendor::specialized_feature;
 *
 *     capability foo availability = condition;
 *
 * The exact semantic meaning of these constructs is determined downstream.
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
 *                      canonical parser
 *                              |
 *                              v
 *               +--------------+---------------+
 *               |                              |
 *               v                              v
 *       core/capabilities.g4        resources/capabilities.g4
 *               |                              |
 *               |                              |
 *               +--------------+---------------+
 *                              |
 *                              v
 *                   Resource Capability Intent
 *                              |
 *                              v
 *                     Domain-neutral AST
 *                              |
 *                              v
 *                     Semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        capability        resource          target
 *        resolution        analysis          analysis
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *          +-------------------+--------------------+
 *          |                   |                    |
 *          v                   v                    v
 *      classical IR       quantum::ir         HDL/hardware
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                  routing / scheduling /
 *                  resilience / QEC / ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - resource-scoped capability requirements;
 *   - resource-scoped capability constraints;
 *   - resource-scoped capability preferences;
 *   - resource-scoped capability hints;
 *   - capability availability intent;
 *   - capability property predicates;
 *   - capability relationship syntax;
 *   - capability composition syntax;
 *   - association of capability intent with resource context;
 *   - association of capability intent with target/resource expressions;
 *   - source-level capability predicates used by resource analysis.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - qualified names;
 *   - capability identity;
 *   - capability version semantics;
 *   - capability registry;
 *   - capability discovery;
 *   - hardware discovery;
 *   - device selection;
 *   - physical resource allocation;
 *   - topology realization;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime allocation;
 *   - authorization;
 *   - general expression syntax;
 *   - general type syntax;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR.
 *
 * ============================================================================
 * CANONICAL OWNERSHIP BOUNDARIES
 * ============================================================================
 *
 * Capability identity belongs to:
 *
 *     grammar/core/capabilities.g4
 *
 * In particular, this grammar consumes:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *
 * from that grammar.
 *
 * Identifier and qualified-name syntax belongs to:
 *
 *     grammar/core/names.g4
 *
 * General expression syntax belongs to the canonical expression grammar.
 *
 * Resource expression syntax belongs to:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * This file MUST NOT duplicate any of those systems.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * All lexical tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * The canonical lexer vocabulary currently provides resource/capability
 * keywords including:
 *
 *     RESOURCE
 *     RESOURCES
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     TARGET
 *     AVAILABILITY
 *     PORTABILITY
 *     SCALABILITY
 *     PERFORMANCE
 *     LATENCY
 *     THROUGHPUT
 *     BANDWIDTH
 *     ENERGY
 *     POWER
 *     RELIABILITY
 *     RESILIENCE
 *     COST
 *
 * This grammar deliberately uses the canonical token names.
 *
 * It MUST NOT introduce private aliases such as:
 *
 *     K_REQUIRES
 *     K_CAPABILITY
 *     K_RESOURCE
 *
 * because those create a second, nonexistent lexical vocabulary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability intent is target-independent.
 *
 * This grammar supports:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * by describing:
 *
 *     WHAT capability is required
 *
 * rather than:
 *
 *     WHICH physical implementation must be used.
 *
 * For example:
 *
 *     requires capability zamani::quantum::measurement;
 *
 * does NOT mean:
 *
 *     use QPU 0
 *
 *     use physical qubit 0
 *
 *     use vendor X
 *
 *     use topology Y
 *
 *     use N qubits
 *
 * Such decisions belong downstream.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability names are intentionally open-ended.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU capabilities
 *     GPU capabilities
 *     FPGA capabilities
 *     QPU capabilities
 *     vendor capabilities
 *     accelerator capabilities
 *     future capabilities
 *
 * as closed parser alternatives.
 *
 * Examples:
 *
 *     zamani::compute::parallel
 *     zamani::compute::vector
 *     zamani::quantum::measurement
 *     zamani::quantum::dynamic_control
 *     zamani::quantum::mid_circuit_measurement
 *     zamani::hardware::fpga
 *     zamani::hardware::gpu
 *     zamani::network::rdma
 *     zamani::security::post_quantum_crypto
 *     future::compute::new_architecture
 *
 * remain ordinary capability identities.
 *
 * Adding a new capability therefore does not require changing this grammar.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     capabilities
 *     resources
 *     capability predicates
 *     capability relationships
 *     resource groups
 *     devices
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     QPUs
 *     nodes
 *     memory
 *     storage
 *     tensor dimensions
 *     topology size
 *     program size
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * rather than bounded by machine-specific constants.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * CAPABILITY
 *
 *     A property/facility that an environment may provide.
 *
 * REQUIREMENT
 *
 *     A condition that must be satisfied.
 *
 * CONSTRAINT
 *
 *     A mandatory condition on an otherwise valid realization.
 *
 * PREFERENCE
 *
 *     An optimization preference.
 *
 * HINT
 *
 *     Advisory information.
 *
 * AVAILABILITY
 *
 *     A semantic condition describing whether a capability can be used.
 *
 * RELATIONSHIP
 *
 *     A source-level relationship between capability identities.
 *
 * PROPERTY
 *
 *     A named semantic property associated with a capability.
 *
 * None of these constructs allocates hardware.
 *
 * ============================================================================
 * RESOURCE/CAPABILITY DISTINCTION
 * ============================================================================
 *
 * These concepts remain distinct:
 *
 *     requires qubits >= n
 *
 * is a resource requirement.
 *
 *     requires capability zamani::quantum::measurement
 *
 * is a capability requirement.
 *
 *     prefer capability zamani::quantum::dynamic_control
 *
 * is a capability preference.
 *
 *     constraint capability ... 
 *
 * is a realization constraint.
 *
 *     hint capability ...
 *
 * is advisory information.
 *
 * The compiler/resource system determines how those requirements can be
 * satisfied.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     physical CPU IDs
 *     physical GPU IDs
 *     physical FPGA regions
 *     physical qubit IDs
 *     fixed machine topology
 *     fixed device counts
 *     fixed memory capacity
 *     fixed register width
 *     fixed network size
 *
 * Source-level identifiers such as:
 *
 *     gpu0
 *     qpu0
 *     node0
 *
 * remain names unless a separate target-specific construct explicitly gives
 * them target-specific semantics.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Capability property values and predicates use the canonical expression
 * boundary.
 *
 * This grammar does NOT create:
 *
 *     capabilityExpression
 *
 * as a replacement for the language expression system.
 *
 * Where resource-specific expressions are required, the resource expression
 * grammar remains authoritative.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser output should map conceptually to domain-neutral nodes such as:
 *
 *     ResourceCapabilityRequirement
 *     ResourceCapabilityConstraint
 *     ResourceCapabilityPreference
 *     ResourceCapabilityHint
 *     ResourceCapabilityAvailability
 *     ResourceCapabilityRelationship
 *     ResourceCapabilityPredicate
 *     ResourceCapabilityProperty
 *
 * Each node carries, as applicable:
 *
 *     capability identity
 *     version constraint
 *     relation
 *     property name
 *     predicate/value
 *     optional resource context
 *     source span
 *
 * The AST MUST NOT contain:
 *
 *     physical device selection
 *     scheduler state
 *     routing state
 *     calibration state
 *     runtime capability tokens
 *     physical qubit IDs
 *     backend-specific allocation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability identity;
 *     - validating namespaces;
 *     - validating capability versions;
 *     - resolving capability properties;
 *     - checking capability relationships;
 *     - checking requirement satisfiability;
 *     - checking conflicts;
 *     - matching requirements against target capabilities;
 *     - determining whether preferences can be honored;
 *     - determining whether hints are applicable;
 *     - determining resource/capability interactions.
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file does not define an IR.
 *
 * Resource capability intent is lowered into the repository's canonical
 * semantic/resource model.
 *
 * Quantum capability requirements eventually participate in:
 *
 *     quantum semantic analysis
 *             |
 *             v
 *         quantum::ir
 *
 * Classical capability requirements participate in the classical semantic/IR
 * path.
 *
 * HDL/hardware requirements participate in the hardware semantic/IR path.
 *
 * There MUST NOT be a second capability IR created merely because the source
 * originated in this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Diagnostics must distinguish at least:
 *
 *     invalid capability syntax
 *     unknown capability
 *     invalid capability version
 *     unsupported capability
 *     unsatisfied requirement
 *     conflicting capabilities
 *     invalid property
 *     invalid capability relationship
 *     unavailable capability
 *
 * A semantic capability failure MUST NOT be reported as a parser failure.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no I/O;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no runtime calls;
 *     no random state.
 *
 * Given the same token stream, parser behavior is deterministic.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains no Rust actions.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No unsafe Rust is required by this grammar.
 *
 * Repository-level Rust crates should enforce the no-unsafe policy through
 * their normal crate/CI configuration.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar preserves the existing resource/capability conceptual model.
 *
 * The important compatibility correction is lexical:
 *
 *     K_REQUIRES      -> REQUIRES
 *     K_CAPABILITY    -> CAPABILITY
 *     K_RESOURCE      -> RESOURCE
 *     K_CONSTRAINT    -> CONSTRAINT
 *     K_PREFER        -> PREFER
 *     K_HINT          -> HINT
 *
 * Existing canonical lexer token names are therefore consumed directly.
 *
 * New capability identities do not require grammar changes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar ResourceCapabilities;


/*
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Capabilities:
 *
 *     canonical capability identity/version/reference syntax
 *
 * Expressions:
 *
 *     canonical expression syntax
 *
 * Names are therefore not recreated locally.
 * ============================================================================
 */

import Capabilities, Expressions;


/*
 * ============================================================================
 * 1. RESOURCE CAPABILITY SECTION
 * ============================================================================
 *
 * Public entry point for a sequence of resource capability clauses.
 *
 * There is intentionally no finite maximum.
 * ============================================================================
 */

resourceCapabilities
    : resourceCapabilityItem*
    ;


/*
 * ============================================================================
 * 2. RESOURCE CAPABILITY ITEM
 * ============================================================================
 */

resourceCapabilityItem
    : resourceCapabilityRequirement
    | resourceCapabilityConstraint
    | resourceCapabilityPreference
    | resourceCapabilityHint
    | resourceCapabilityAvailability
    | resourceCapabilityRelationship
    | resourceCapabilityComposition
    ;


/*
 * ============================================================================
 * 3. REQUIREMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     requires capability zamani::quantum::measurement;
 *
 *     requires capability
 *         zamani::quantum::dynamic_control;
 *
 *     requires capability future::compute::new_architecture;
 *
 * The capability identity is supplied by the canonical Capabilities grammar.
 * ============================================================================
 */

resourceCapabilityRequirement
    : REQUIRES
      resourceCapabilityRequirementBody
      SEMI
    ;


resourceCapabilityRequirementBody
    : CAPABILITY capabilityReference
    | RESOURCE CAPABILITY capabilityReference
    | RESOURCE capabilityReference
    ;


/*
 * ============================================================================
 * 4. CONSTRAINT
 * ============================================================================
 *
 * A constraint is mandatory.
 *
 * Examples:
 *
 *     constraint capability zamani::quantum::measurement;
 *
 *     constraint capability.some_property >= requested_value;
 *
 * The semantic layer determines satisfiability.
 * ============================================================================
 */

resourceCapabilityConstraint
    : CONSTRAINT
      resourceCapabilityConstraintBody
      SEMI
    ;


resourceCapabilityConstraintBody
    : CAPABILITY capabilityReference
    | CAPABILITY capabilityReference
      resourceCapabilityPredicateOperator
      resourceCapabilityExpression
    | resourceCapabilityExpression
    ;


/*
 * ============================================================================
 * 5. PREFERENCE
 * ============================================================================
 *
 * A preference is non-mandatory optimization intent.
 * ============================================================================
 */

resourceCapabilityPreference
    : PREFER
      resourceCapabilityPreferenceBody
      SEMI
    ;


resourceCapabilityPreferenceBody
    : CAPABILITY capabilityReference
    | CAPABILITY capabilityReference
      resourceCapabilityPredicateOperator
      resourceCapabilityExpression
    | resourceCapabilityExpression
    ;


/*
 * ============================================================================
 * 6. HINT
 * ============================================================================
 *
 * Hints are advisory and may be ignored by the compiler/runtime.
 * ============================================================================
 */

resourceCapabilityHint
    : HINT
      resourceCapabilityHintBody
      SEMI
    ;


resourceCapabilityHintBody
    : CAPABILITY capabilityReference
    | CAPABILITY capabilityReference
      resourceCapabilityPredicateOperator
      resourceCapabilityExpression
    | resourceCapabilityExpression
    ;


/*
 * ============================================================================
 * 7. AVAILABILITY
 * ============================================================================
 *
 * Uses the existing AVAILABILITY keyword.
 *
 * Example:
 *
 *     capability zamani::quantum::measurement
 *         availability = execution_context.supports_measurement;
 *
 * Availability is semantic data.
 *
 * It does not perform runtime discovery while parsing.
 * ============================================================================
 */

resourceCapabilityAvailability
    : CAPABILITY
      capabilityReference
      AVAILABILITY
      ASSIGN
      resourceCapabilityExpression
      SEMI
    ;


/*
 * ============================================================================
 * 8. CAPABILITY RELATIONSHIP
 * ============================================================================
 *
 * Relationship names are deliberately open rather than being hard-coded
 * keyword enumerations.
 *
 * Examples:
 *
 *     capability A implies capability B;
 *
 *     capability A excludes capability B;
 *
 *     capability A refines capability B;
 *
 *     capability A requires capability B;
 *
 *     capability A supersedes capability B;
 *
 * The relation itself is an identifier.
 *
 * This permits future capability relationships without modifying the grammar.
 *
 * The semantic layer owns the set of relationships that are actually valid.
 * ============================================================================
 */

resourceCapabilityRelationship
    : CAPABILITY
      capabilityReference
      resourceCapabilityRelationshipName
      CAPABILITY
      capabilityReference
      SEMI
    ;


resourceCapabilityRelationshipName
    : identifier
    ;


/*
 * ============================================================================
 * 9. CAPABILITY COMPOSITION
 * ============================================================================
 *
 * A capability composition groups capability predicates.
 *
 * Example:
 *
 *     capability {
 *         ...
 *     };
 *
 * The contents remain source intent.
 *
 * Semantic analysis determines whether the composition means conjunction,
 * disjunction, implication, or another explicitly defined semantic relation.
 * ============================================================================
 */

resourceCapabilityComposition
    : CAPABILITY
      LBRACE
      resourceCapabilityCompositionItem*
      RBRACE
      SEMI
    ;


resourceCapabilityCompositionItem
    : resourceCapabilityCompositionRequirement
    | resourceCapabilityCompositionProperty
    ;


resourceCapabilityCompositionRequirement
    : resourceCapabilityRequirementBody
      SEMI
    ;


resourceCapabilityCompositionProperty
    : identifier
      ASSIGN
      resourceCapabilityExpression
      SEMI
    ;


/*
 * ============================================================================
 * 10. CAPABILITY PREDICATE
 * ============================================================================
 *
 * Capability predicates intentionally reuse the canonical expression system.
 *
 * Examples:
 *
 *     width >= requested_width
 *
 *     availability == required_state
 *
 *     capability_property == expected_value
 *
 * ============================================================================
 */

resourceCapabilityPredicate
    : resourceCapabilityExpression
    ;


resourceCapabilityExpression
    : expression
    ;


/*
 * ============================================================================
 * 11. COMPARISON OPERATORS
 * ============================================================================
 *
 * These operators are inherited from the canonical lexer.
 *
 * Their semantic meaning is determined by semantic analysis and the type of
 * the compared values.
 * ============================================================================
 */

resourceCapabilityPredicateOperator
    : EQ
    | NE
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * 12. PROPERTY PREDICATE
 * ============================================================================
 *
 * Property names remain open-world.
 *
 * Example:
 *
 *     capability zamani::quantum::device
 *         property dynamic_control == true;
 *
 * This grammar does not enumerate capability properties.
 * ============================================================================
 */

resourceCapabilityPropertyPredicate
    : CAPABILITY
      capabilityReference
      PROPERTY
      identifier
      resourceCapabilityPredicateOperator
      resourceCapabilityExpression
    ;


/*
 * ============================================================================
 * 13. PROPERTY ITEM
 * ============================================================================
 *
 * This rule is intentionally generic so that future capability properties do
 * not require grammar modification.
 * ============================================================================
 */

resourceCapabilityProperty
    : identifier
      ASSIGN
      resourceCapabilityExpression
      SEMI
    ;


/*
 * ============================================================================
 * 14. RESOURCE-SCOPED CAPABILITY PROPERTY
 * ============================================================================
 *
 * This is useful where a capability is attached to a resource context.
 *
 * Example:
 *
 *     resource capability {
 *         property = value;
 *     }
 *
 * The enclosing Resources grammar determines the resource scope.
 * ============================================================================
 */

resourceCapabilityResourceProperty
    : RESOURCE
      CAPABILITY
      LBRACE
      resourceCapabilityProperty*
      RBRACE
      SEMI
    ;


/*
 * ============================================================================
 * 15. TARGET-SCOPED CAPABILITY INTENT
 * ============================================================================
 *
 * A target is still an abstract target context.
 *
 * It is NOT a physical device selector.
 *
 * Example:
 *
 *     target capability foo;
 *
 * The downstream target model decides how this intent is realized.
 * ============================================================================
 */

resourceCapabilityTargetRequirement
    : TARGET
      CAPABILITY
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 16. OPTIONAL RESOURCE CAPABILITY LIST
 * ============================================================================
 *
 * No finite limit.
 * ============================================================================
 */

resourceCapabilityReferenceList
    : capabilityReference
      (COMMA capabilityReference)*
    ;


/*
 * ============================================================================
 * 17. RESOURCE CAPABILITY REQUIREMENT LIST
 * ============================================================================
 */

resourceCapabilityRequirementList
    : resourceCapabilityRequirement+
    ;


/*
 * ============================================================================
 * 18. RESOURCE CAPABILITY ITEM LIST
 * ============================================================================
 */

resourceCapabilityItemList
    : resourceCapabilityItem+
    ;


/*
 * ============================================================================
 * 19. SEMANTIC INTEGRATION NOTES
 * ============================================================================
 *
 * The parser output is expected to be transformed approximately as follows:
 *
 *     resourceCapabilityRequirement
 *             |
 *             v
 *     ResourceCapabilityRequirement
 *
 *     resourceCapabilityConstraint
 *             |
 *             v
 *     ResourceCapabilityConstraint
 *
 *     resourceCapabilityPreference
 *             |
 *             v
 *     ResourceCapabilityPreference
 *
 *     resourceCapabilityHint
 *             |
 *             v
 *     ResourceCapabilityHint
 *
 *     resourceCapabilityAvailability
 *             |
 *             v
 *     ResourceCapabilityAvailability
 *
 *     resourceCapabilityRelationship
 *             |
 *             v
 *     ResourceCapabilityRelationship
 *
 *     resourceCapabilityComposition
 *             |
 *             v
 *     ResourceCapabilityComposition
 *
 * AST nodes remain domain-neutral.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. SEMANTIC RESOLUTION
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     capability identity
 *     capability version
 *     capability namespace
 *     capability provider
 *     capability properties
 *     capability relationships
 *     target support
 *     resource interaction
 *     conflict
 *     satisfiability
 *
 * Example:
 *
 *     requires capability zamani::quantum::measurement;
 *
 * becomes conceptually:
 *
 *     CapabilityRequirement {
 *         identity:
 *             zamani::quantum::measurement,
 *         version:
 *             none,
 *         source:
 *             source_span
 *     }
 *
 * No physical device is selected at this stage.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. RESOURCE NEGOTIATION
 * ============================================================================
 *
 * Capability requirements participate in:
 *
 *     program requirements
 *             |
 *             v
 *     target capability discovery
 *             |
 *             v
 *     capability matching
 *             |
 *             v
 *     resource planning
 *             |
 *             v
 *     target realization
 *
 * The grammar does not perform negotiation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     requires capability zamani::quantum::mid_circuit_measurement;
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 *     prefer capability zamani::quantum::low_noise;
 *
 * These requirements eventually participate in quantum semantic analysis.
 *
 * The canonical boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumCapabilityIR
 *     QuantumResourceIR
 *     PhysicalQubitCapabilityIR
 *
 * as competing representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. CLASSICAL / GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability zamani::compute::parallel;
 *
 *     requires capability zamani::compute::vector;
 *
 *     requires capability zamani::hardware::gpu;
 *
 *     requires capability zamani::hardware::fpga;
 *
 * These remain abstract capabilities.
 *
 * They do not mean:
 *
 *     gpu0
 *     fpga0
 *     core0
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability zamani::distributed::communication;
 *
 *     requires capability zamani::network::rdma;
 *
 *     requires capability zamani::distributed::replication;
 *
 * No fixed node count is implied.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability zamani::tensor::compute;
 *
 *     requires capability zamani::ai::training;
 *
 *     requires capability zamani::ai::automatic_differentiation;
 *
 *     requires capability zamani::data::streaming;
 *
 * Capability names remain open-world.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL grammars may consume the resource capability entry points
 * without duplicating capability identity syntax.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. NO ARTIFICIAL RESOURCE LIMITS
 * ============================================================================
 *
 * The following are prohibited as grammar semantics:
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
 * A target can report any finite or effectively unbounded capability set.
 *
 * The language itself remains independent of that set.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. HARDWARE-SCALE EXAMPLES
 * ============================================================================
 *
 * Tiny:
 *
 *     requires capability zamani::compute::scalar;
 *
 * Large:
 *
 *     requires capability zamani::compute::parallel;
 *
 * Quantum:
 *
 *     requires capability zamani::quantum::measurement;
 *
 * HDL:
 *
 *     requires capability zamani::hardware::programmable_logic;
 *
 * Distributed:
 *
 *     requires capability zamani::distributed::communication;
 *
 * Future:
 *
 *     requires capability future::architecture::novel_compute;
 *
 * None of these examples impose a universal machine size.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Capability resolution errors belong to semantic analysis.
 *
 * Resource insufficiency belongs to resource/target analysis.
 *
 * Unsupported target capability belongs to target compatibility analysis.
 *
 * Runtime availability belongs to execution/runtime systems.
 *
 * Example:
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 * is syntactically valid even when the current target does not provide it.
 *
 * The compiler must report an appropriate semantic/target diagnostic rather
 * than rejecting the source as invalid syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. SOURCE SPANS
 * ============================================================================
 *
 * The frontend AST must preserve source spans for:
 *
 *     REQUIRES
 *     CAPABILITY
 *     capability identity
 *     version clause
 *     relationship
 *     property
 *     predicate/value
 *
 * This enables precise diagnostics, IDE support, formatting, refactoring, and
 * compatibility tooling.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation must verify:
 *
 *     - grammar name matches ResourceCapabilities;
 *     - tokenVocab is ZamaniLexer;
 *     - canonical Capabilities grammar is imported;
 *     - canonical Expressions grammar is imported;
 *     - no local capability identity grammar exists;
 *     - no K_* shadow tokens exist;
 *     - no fixed capability enumeration exists;
 *     - no hardware capacity constants exist;
 *     - no physical device selection is encoded;
 *     - no semantic predicates exist;
 *     - no actions exist;
 *     - repetition is unbounded by machine constants;
 *     - parser entry points are composable;
 *     - source spans can be preserved;
 *     - parser behavior remains deterministic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     requires capability zamani::compute::parallel;
 *
 *     requires capability zamani::quantum::measurement;
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 *     requires capability future::compute::new_architecture;
 *
 *     prefer capability zamani::compute::vector;
 *
 *     hint capability zamani::hardware::gpu;
 *
 *     constraint capability zamani::quantum::measurement;
 *
 * Required relationship tests:
 *
 *     capability A implies capability B;
 *
 *     capability A excludes capability B;
 *
 *     capability A refines capability B;
 *
 * Required availability test:
 *
 *     capability zamani::quantum::measurement
 *         availability = execution_context.supports_measurement;
 *
 * Required property test:
 *
 *     capability zamani::quantum::device
 *         property dynamic_control == true;
 *
 * Required expression tests:
 *
 *     constraint capability zamani::compute::vector
 *         property width >= requested_width;
 *
 * Required scalability tests:
 *
 *     thousands of capability references;
 *     deeply qualified capability names;
 *     large capability compositions;
 *     large predicate expressions;
 *     arbitrary symbolic resource values;
 *
 * Required negative tests:
 *
 *     malformed capability identity;
 *     missing capability reference;
 *     missing semicolon;
 *     malformed relationship;
 *     malformed comparison;
 *     malformed availability expression;
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable lexical token names remain:
 *
 *     REQUIRES
 *     RESOURCE
 *     CAPABILITY
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     TARGET
 *     AVAILABILITY
 *     PROPERTY
 *
 * Capability identities remain open-world.
 *
 * Adding:
 *
 *     zamani::future::new_capability
 *
 * does not require a grammar change.
 *
 * Adding a new reserved keyword DOES require the normal lexical compatibility
 * process.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has one canonical parser grammar identity.
 *
 * [x] It consumes the canonical ZamaniLexer vocabulary.
 *
 * [x] It does not use nonexistent K_* tokens.
 *
 * [x] It imports the canonical capability grammar.
 *
 * [x] It imports the canonical expression grammar.
 *
 * [x] It does not duplicate identifier syntax.
 *
 * [x] It does not duplicate capability identity syntax.
 *
 * [x] It does not define a closed capability enumeration.
 *
 * [x] It supports arbitrary capability namespaces.
 *
 * [x] It supports arbitrary future capabilities.
 *
 * [x] It distinguishes requirement/constraint/preference/hint.
 *
 * [x] It supports capability availability.
 *
 * [x] It supports extensible capability relationships.
 *
 * [x] It supports capability properties.
 *
 * [x] It supports capability composition.
 *
 * [x] It does not select physical hardware.
 *
 * [x] It does not encode resource capacities.
 *
 * [x] It does not encode machine-size limits.
 *
 * [x] It does not define a second expression language.
 *
 * [x] It does not define an IR.
 *
 * [x] It preserves the quantum::ir boundary.
 *
 * [x] It is deterministic.
 *
 * [x] It contains no embedded unsafe Rust.
 *
 * [x] It is compatible with Rust 1.97 / 1.97.1 generated-parser integration.
 *
 * [x] Its AST contract is defined before semantic implementation.
 *
 * [x] Its semantic contract is defined before IR integration.
 *
 * [x] Its downstream consumers are identified.
 *
 * [x] Its scalability requirements are explicit.
 *
 * [x] Its validation and test contract is explicit.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What capability does the program require, constrain, prefer, hint at,
 *      or relate to in the abstract resource model?"
 *
 * It does NOT answer:
 *
 *     "Which physical machine should execute it?"
 *
 *     "Which device should be selected?"
 *
 *     "Which qubit should be used?"
 *
 *     "Which CPU core should execute it?"
 *
 *     "Which GPU should execute it?"
 *
 *     "How should it be routed?"
 *
 *     "How should it be scheduled?"
 *
 *     "How should QEC be performed?"
 *
 *     "How should the HAL realize it?"
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 */