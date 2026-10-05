/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/capabilities.g4
 *
 * GRAMMAR
 * -------
 * ResourceCapabilities
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-SCOPED CAPABILITY LEAF GRAMMAR
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * Pure ANTLR4 parser grammar.
 *
 * This file contains:
 *
 *     - no Rust;
 *     - no embedded target-language actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no resource allocation;
 *     - no scheduling;
 *     - no routing;
 *     - no runtime execution.
 *
 * The consuming Zamani implementation MUST use safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the specialized resource-layer owner for SOURCE-LEVEL
 * CAPABILITY INTENT.
 *
 * It does NOT own the universal capability identity model.
 *
 * Canonical capability identity/reference/version syntax belongs to:
 *
 *     grammar/core/capabilities.g4
 *
 * This file consumes that API and adds the resource-layer semantics needed to
 * express:
 *
 *     - capability requirements;
 *     - capability constraints;
 *     - capability preferences;
 *     - capability hints;
 *     - capability availability conditions;
 *     - capability properties;
 *     - capability relationships;
 *     - capability compositions;
 *     - resource-scoped capability assertions;
 *     - abstract target capability intent.
 *
 * The resulting syntax is target-independent.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     core/capabilities.g4       resources/capabilities.g4
 *          |                             |
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *              domain-neutral AST
 *                        |
 *                        v
 *                semantic analysis
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      capability      resource       target
 *      resolution      analysis       analysis
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *              canonical semantic model
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *     classical IR   quantum::ir   HDL/hardware
 *                        |
 *                        v
 *              optimization/lowering
 *                        |
 *               routing/scheduling
 *                        |
 *              resilience/QEC/ZQN
 *                        |
 *                        v
 *                       HAL
 *                        |
 *                        v
 *                target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     resourceCapabilityIntent
 *     resourceCapabilityAssertion
 *     resourceCapabilityRequirement
 *     resourceCapabilityRequirementBody
 *     resourceCapabilityConstraint
 *     resourceCapabilityConstraintBody
 *     resourceCapabilityPreference
 *     resourceCapabilityPreferenceBody
 *     resourceCapabilityHint
 *     resourceCapabilityHintBody
 *     resourceCapabilityAvailability
 *     resourceCapabilityRelationship
 *     resourceCapabilityComposition
 *     resourceCapabilityCompositionItem
 *     resourceCapabilityPropertyPredicate
 *     resourceCapabilityPropertyPath
 *     resourceCapabilityPropertySegment
 *     resourceCapabilityTargetRequirement
 *     resourceCapabilityPredicateOperator
 *     resourceCapabilityExpression
 *     resourceCapabilityReferenceList
 *     resourceCapabilityRequirementList
 *     resourceCapabilityItemList
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     capability identity
 *     capability reference syntax
 *     capability name syntax
 *     capability version syntax
 *     identifier syntax
 *     qualified-name syntax
 *     general expression syntax
 *     resource expression syntax
 *     universal resource declarations
 *     universal requirements
 *     universal constraints
 *     universal preferences
 *     universal hints
 *     policies
 *     effects
 *     contracts
 *     provenance
 *     hardware discovery
 *     physical device selection
 *     allocation
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT DEPENDENCIES
 * -------------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/capabilities.g4
 *     grammar/expressions/expressions.g4
 *
 * Imported parser grammars:
 *
 *     Capabilities
 *     Expressions
 *
 * The imported Capabilities grammar itself owns its dependency on names and
 * versioning. This file therefore does not recreate those rules.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * Primary public entry:
 *
 *     resourceCapabilityIntent
 *
 * Reusable public entries:
 *
 *     resourceCapabilityRequirement
 *     resourceCapabilityConstraint
 *     resourceCapabilityPreference
 *     resourceCapabilityHint
 *     resourceCapabilityAvailability
 *     resourceCapabilityRelationship
 *     resourceCapabilityComposition
 *     resourceCapabilityPropertyPredicate
 *     resourceCapabilityTargetRequirement
 *
 * ============================================================================
 * CONSUMERS
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/resources/resources.g4
 *
 * Other resource-domain consumers may use the reusable rules where appropriate.
 *
 * Domain grammars such as:
 *
 *     grammar/quantum/
 *     grammar/hardware/
 *     grammar/hdl/
 *     grammar/hybrid/
 *     grammar/classical/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/ai/
 *
 * MUST consume the capability semantic model rather than redefining capability
 * syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts from this file are transformed into domain-neutral semantic
 * nodes such as:
 *
 *     ResourceCapabilityAssertion
 *     ResourceCapabilityRequirement
 *     ResourceCapabilityConstraint
 *     ResourceCapabilityPreference
 *     ResourceCapabilityHint
 *     ResourceCapabilityAvailability
 *     ResourceCapabilityRelationship
 *     ResourceCapabilityComposition
 *     ResourceCapabilityPropertyPredicate
 *     ResourceCapabilityTargetRequirement
 *
 * The eventual AST/semantic representation MUST preserve, where applicable:
 *
 *     capability identity
 *     capability version requirement
 *     capability expression structure
 *     property path
 *     comparison operator
 *     expression/value
 *     relationship name
 *     resource context
 *     target context
 *     source span
 *     source ordering
 *
 * The representation MUST NOT contain:
 *
 *     physical device allocation
 *     scheduler state
 *     routing state
 *     calibration state
 *     physical qubit identity
 *     physical CPU identity
 *     physical GPU identity
 *     vendor-selected implementation
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not this grammar, determines:
 *
 *     whether a capability exists;
 *     whether a capability version is valid;
 *     whether a property exists;
 *     whether a property has the required type;
 *     whether a relationship is valid;
 *     whether a requirement is satisfiable;
 *     whether capabilities conflict;
 *     whether a target provides a capability;
 *     whether a preference can be honored;
 *     whether a hint is applicable;
 *     whether an availability condition is true;
 *     whether a capability can be realized using available resources.
 *
 * Syntax acceptance MUST NOT be treated as proof of target feasibility.
 *
 * ============================================================================
 * RESOURCE/CAPABILITY SEPARATION
 * ============================================================================
 *
 * CAPABILITY
 * ----------
 *
 * A capability describes an ability, facility, property, or semantic feature
 * that an environment may provide.
 *
 * REQUIREMENT
 * -----------
 *
 * A requirement states something that must be satisfied.
 *
 * CONSTRAINT
 * ----------
 *
 * A constraint states a mandatory condition on an otherwise valid realization.
 *
 * PREFERENCE
 * ----------
 *
 * A preference expresses non-mandatory optimization intent.
 *
 * HINT
 * ----
 *
 * A hint provides advisory information.
 *
 * RESOURCE
 * --------
 *
 * A resource describes a quantity or resource-bearing semantic object.
 *
 * TARGET
 * ------
 *
 * A target describes an abstract compilation/execution context.
 *
 * IMPLEMENTATION DECISION
 * -----------------------
 *
 * Selection of a physical device, processor, accelerator, QPU, memory bank,
 * topology, routing path, or scheduler placement belongs downstream.
 *
 * These concepts MUST remain distinct.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability identities are open-world.
 *
 * The grammar MUST NOT enumerate a finite set of:
 *
 *     CPU capabilities
 *     GPU capabilities
 *     FPGA capabilities
 *     ASIC capabilities
 *     accelerator capabilities
 *     QPU capabilities
 *     simulator capabilities
 *     AI capabilities
 *     networking capabilities
 *     vendor capabilities
 *     future capabilities
 *
 * Examples that remain ordinary capability identities:
 *
 *     compute::scalar
 *     compute::parallel
 *     compute::vector
 *     tensor::compute
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     hardware::reconfigurable_logic
 *     hdl::synthesis
 *     distributed::communication
 *     network::rdma
 *     security::trusted_execution
 *     future::architecture::new_feature
 *
 * Adding a new capability MUST NOT require changing this grammar.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Capability intent describes WHAT the program requires or prefers.
 *
 * It does not describe WHICH physical implementation must be selected.
 *
 * Therefore:
 *
 *     requires capability quantum::measurement;
 *
 * does NOT mean:
 *
 *     select a particular QPU;
 *
 *     select a particular physical qubit;
 *
 *     select a particular vendor;
 *
 *     select a particular topology;
 *
 *     select a particular device.
 *
 * The realization is determined downstream.
 *
 * ============================================================================
 * SCALE CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     capability count
 *     resource count
 *     device count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     memory capacity
 *     storage capacity
 *     thread count
 *     tensor rank
 *     network size
 *     topology size
 *     capability namespace depth
 *     property namespace depth
 *     capability-expression size
 *     composition size
 *
 * The grammar uses structural repetition:
 *
 *     *
 *     +
 *
 * rather than machine-specific constants.
 *
 * "Unbounded" here means no artificial language-level ceiling.
 * It does not claim that finite implementations possess infinite resources.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce or depend on:
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
 *     MAX_CAPABILITIES
 *     MAX_PROPERTIES
 *     MAX_RELATIONSHIPS
 *
 * It MUST NOT encode equivalent fixed capacities indirectly.
 *
 * Numeric values appearing in expressions remain program semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Source-level identifiers such as:
 *
 *     gpu0
 *     qpu0
 *     node0
 *     cpu0
 *
 * remain identifiers unless a separate target-specific language feature gives
 * them explicit target semantics.
 *
 * This grammar itself never assigns physical meaning to them.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Mentioning a capability has no effect by itself.
 *
 * For example:
 *
 *     capability network::communication;
 *
 * does not authorize network access.
 *
 * Effects are owned by grammar/effects/ and the semantic effect system.
 *
 * A capability may participate in effect validation, but capability syntax
 * itself does not grant authority.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain capability usage through:
 *
 *     permission
 *     prohibition
 *     requirement
 *     preference
 *     fallback
 *     adaptation
 *     execution
 *
 * Policy semantics belong to grammar/policies/ and downstream semantic
 * analysis.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Downstream provenance may preserve:
 *
 *     source capability
 *     resolved capability
 *     version
 *     provider
 *     evidence
 *     decision
 *     target realization
 *
 * This grammar creates no provenance records itself.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum capability names remain open-world.
 *
 * Examples:
 *
 *     requires capability quantum::measurement;
 *     requires capability quantum::dynamic_control;
 *     prefer capability quantum::low_noise;
 *
 * This file does not define quantum operations.
 *
 * The canonical path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * No quantum-specific capability IR is introduced here.
 *
 * ============================================================================
 * CLASSICAL / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability compute::parallel;
 *     requires capability compute::vector;
 *     requires capability tensor::compute;
 *     prefer capability accelerator::matrix;
 *
 * These are semantic capabilities.
 *
 * They do not select a processor or accelerator.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability hdl::synthesis;
 *     requires capability hardware::reconfigurable_logic;
 *     prefer capability hardware::parallel_compute;
 *
 * The grammar imposes no:
 *
 *     register width
 *     memory size
 *     device count
 *     pipeline count
 *     FPGA capacity
 *     ASIC capacity
 *     clock capacity
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability distributed::communication;
 *     requires capability distributed::replication;
 *     requires capability network::rdma;
 *
 * No node count or network-size limit is encoded.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability ai::training;
 *     requires capability ai::automatic_differentiation;
 *     requires capability tensor::compute;
 *     requires capability data::streaming;
 *
 * Application-specific model names remain identifiers and do not become
 * universal capability keywords.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file uses the canonical public lexer:
 *
 *     ZamaniLexer
 *
 * It MUST NOT define or shadow lexer tokens.
 *
 * Canonical resource/capability tokens consumed here include:
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
 *     SEMICOLON
 *     ASSIGN
 *     EQ
 *     NE
 *     LT
 *     LE
 *     GT
 *     GE
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     DOT
 *     DOUBLE_COLON
 *
 * General expression operators are consumed through the imported expression
 * grammar rather than redefined here.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Capabilities:
 *
 *     grammar/core/capabilities.g4
 *
 * provides:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityExpression
 *     capabilityReferenceList
 *     capabilityNameList
 *
 * Expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * provides:
 *
 *     expression
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 */

parser grammar ResourceCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Capabilities, Expressions;


/*
 * ============================================================================
 * 1. PRIMARY RESOURCE CAPABILITY ENTRY
 * ============================================================================
 *
 * This is the unique public entry point for the specialized capability leaf.
 *
 * The parent resource orchestrator may delegate its existing capability
 * production to this rule without creating a second capability grammar.
 *
 * ============================================================================
 */

resourceCapabilityIntent
    : resourceCapabilityAssertion
    | resourceCapabilityRequirement
    | resourceCapabilityConstraint
    | resourceCapabilityPreference
    | resourceCapabilityHint
    | resourceCapabilityAvailability
    | resourceCapabilityRelationship
    | resourceCapabilityComposition
    | resourceCapabilityTargetRequirement
    ;


/*
 * ============================================================================
 * 2. RESOURCE-SCOPED CAPABILITY ASSERTION
 * ============================================================================
 *
 * This preserves the existing resource-layer surface:
 *
 *     capability <capability-expression>;
 *
 * Examples:
 *
 *     capability compute::parallel;
 *
 *     capability quantum::measurement;
 *
 *     capability quantum::measurement and classical::control;
 *
 * This is source-level capability intent.
 *
 * It does not prove that the capability exists or is available.
 *
 * ============================================================================
 */

resourceCapabilityAssertion
    : CAPABILITY
      capabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. REQUIREMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     requires capability quantum::measurement;
 *
 *     requires capability quantum::measurement
 *         and classical::control;
 *
 *     requires resource capability accelerator::tensor_compute;
 *
 *     requires resource accelerator::tensor_compute;
 *
 * Capability expressions remain owned by core/capabilities.g4.
 *
 * ============================================================================
 */

resourceCapabilityRequirement
    : REQUIRES
      resourceCapabilityRequirementBody
      SEMICOLON
    ;


resourceCapabilityRequirementBody
    : CAPABILITY
      capabilityExpression
    | RESOURCE
      CAPABILITY
      capabilityExpression
    | RESOURCE
      capabilityExpression
    ;


/*
 * ============================================================================
 * 4. CONSTRAINT
 * ============================================================================
 *
 * A capability constraint is mandatory.
 *
 * Supported forms include:
 *
 *     constraint capability quantum::measurement;
 *
 *     constraint capability quantum::device
 *         property dynamic_control == true;
 *
 *     constraint capability quantum::device
 *         property fidelity >= required_fidelity;
 *
 * A generic resource expression may also be constrained:
 *
 *     constraint available_capability == required_capability;
 *
 * Resource quantities themselves remain owned by the resource constraint
 * grammar.
 *
 * ============================================================================
 */

resourceCapabilityConstraint
    : CONSTRAINT
      resourceCapabilityConstraintBody
      SEMICOLON
    ;


resourceCapabilityConstraintBody
    : CAPABILITY
      capabilityExpression
    | resourceCapabilityPropertyPredicateBody
    | resourceCapabilityExpression
    ;


resourceCapabilityPropertyPredicateBody
    : CAPABILITY
      capabilityReference
      PROPERTY
      resourceCapabilityPropertyPath
      resourceCapabilityPredicateOperator
      resourceCapabilityExpression
    ;


/*
 * ============================================================================
 * 5. PREFERENCE
 * ============================================================================
 *
 * Preferences are non-mandatory optimization intent.
 *
 * Examples:
 *
 *     prefer capability compute::vector;
 *
 *     prefer capability quantum::low_noise;
 *
 *     prefer capability quantum::device
 *         property fidelity >= preferred_fidelity;
 *
 * A preference MUST NOT become a requirement merely because the target cannot
 * honor it.
 *
 * ============================================================================
 */

resourceCapabilityPreference
    : PREFER
      resourceCapabilityPreferenceBody
      SEMICOLON
    ;


resourceCapabilityPreferenceBody
    : CAPABILITY
      capabilityExpression
    | resourceCapabilityPropertyPredicateBody
    | resourceCapabilityExpression
    ;


/*
 * ============================================================================
 * 6. HINT
 * ============================================================================
 *
 * Hints are advisory.
 *
 * Examples:
 *
 *     hint capability accelerator::matrix;
 *
 *     hint capability quantum::routing;
 *
 *     hint capability hardware::locality
 *         property domain == preferred_domain;
 *
 * The compiler may honor, transform, preserve, or ignore a hint according to
 * semantic policy.
 *
 * ============================================================================
 */

resourceCapabilityHint
    : HINT
      resourceCapabilityHintBody
      SEMICOLON
    ;


resourceCapabilityHintBody
    : CAPABILITY
      capabilityExpression
    | resourceCapabilityPropertyPredicateBody
    | resourceCapabilityExpression
    ;


/*
 * ============================================================================
 * 7. AVAILABILITY
 * ============================================================================
 *
 * Availability is a declarative semantic condition.
 *
 * Example:
 *
 *     capability quantum::measurement
 *         availability = execution_context.supports_measurement;
 *
 * The parser does not evaluate the condition.
 *
 * ============================================================================
 */

resourceCapabilityAvailability
    : CAPABILITY
      capabilityExpression
      AVAILABILITY
      ASSIGN
      resourceCapabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. CAPABILITY RELATIONSHIP
 * ============================================================================
 *
 * Relationship names remain open-world.
 *
 * Examples:
 *
 *     capability A implies capability B;
 *     capability A excludes capability B;
 *     capability A refines capability B;
 *     capability A requires capability B;
 *     capability A supersedes capability B;
 *
 * The relationship name is an identifier rather than a closed keyword list.
 *
 * Semantic analysis determines whether a relationship is recognized and what
 * it means.
 *
 * ============================================================================
 */

resourceCapabilityRelationship
    : CAPABILITY
      capabilityReference
      resourceCapabilityRelationshipName
      CAPABILITY
      capabilityReference
      SEMICOLON
    ;


resourceCapabilityRelationshipName
    : identifier
    ;


/*
 * ============================================================================
 * 9. CAPABILITY COMPOSITION
 * ============================================================================
 *
 * A capability composition provides a structured resource-scoped collection.
 *
 * Example:
 *
 *     capability {
 *         quantum::measurement;
 *         quantum::dynamic_control;
 *         property = value;
 *     };
 *
 * Composition cardinality is intentionally unbounded.
 *
 * Semantic analysis determines whether the composition means:
 *
 *     conjunction;
 *     disjunction;
 *     grouping;
 *     profile;
 *     another explicitly specified relation.
 *
 * ============================================================================
 */

resourceCapabilityComposition
    : CAPABILITY
      LBRACE
      resourceCapabilityCompositionItem*
      RBRACE
      SEMICOLON
    ;


resourceCapabilityCompositionItem
    : resourceCapabilityCompositionReference
    | resourceCapabilityCompositionProperty
    ;


resourceCapabilityCompositionReference
    : capabilityExpression
      SEMICOLON
    ;


resourceCapabilityCompositionProperty
    : resourceCapabilityPropertyPath
      ASSIGN
      resourceCapabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. TARGET-SCOPED CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * The target remains abstract.
 *
 * Example:
 *
 *     target capability quantum::measurement;
 *
 * This does NOT identify a physical target.
 *
 * It only states target capability intent.
 *
 * ============================================================================
 */

resourceCapabilityTargetRequirement
    : TARGET
      CAPABILITY
      capabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CAPABILITY PROPERTY PREDICATE
 * ============================================================================
 *
 * Property names are open-world.
 *
 * Examples:
 *
 *     capability quantum::device
 *         property fidelity >= required_fidelity;
 *
 *     capability hardware::accelerator
 *         property memory::coherence == required_coherence;
 *
 *     capability compute::vector
 *         property width >= required_width;
 *
 * The grammar does not enumerate standard or vendor properties.
 *
 * ============================================================================
 */

resourceCapabilityPropertyPredicate
    : resourceCapabilityPropertyPredicateBody
    ;


resourceCapabilityPropertyPath
    : resourceCapabilityPropertySegment
      (
          DOT resourceCapabilityPropertySegment
        | DOUBLE_COLON resourceCapabilityPropertySegment
      )*
    ;


resourceCapabilityPropertySegment
    : identifier
    ;


/*
 * ============================================================================
 * 12. PREDICATE OPERATORS
 * ============================================================================
 *
 * These are the canonical comparison tokens.
 *
 * Their semantic interpretation is determined by type/resource analysis.
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
 * 13. CAPABILITY EXPRESSION BRIDGE
 * ============================================================================
 *
 * Resource capability values that are specifically capability expressions
 * MUST use the canonical capability-expression grammar.
 *
 * This avoids creating a second capability boolean language.
 * ============================================================================
 */

resourceCapabilityExpression
    : expression
    ;


/*
 * ============================================================================
 * 14. CAPABILITY REFERENCE LIST
 * ============================================================================
 *
 * Reusable non-empty list.
 *
 * No finite cardinality is imposed.
 * ============================================================================
 */

resourceCapabilityReferenceList
    : capabilityReference
      (
          COMMA capabilityReference
      )*
    ;


/*
 * ============================================================================
 * 15. REQUIREMENT LIST
 * ============================================================================
 */

resourceCapabilityRequirementList
    : resourceCapabilityRequirement+
    ;


/*
 * ============================================================================
 * 16. CAPABILITY ITEM LIST
 * ============================================================================
 */

resourceCapabilityItemList
    : resourceCapabilityIntent+
    ;


/*
 * ============================================================================
 * 17. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The intended lowering is:
 *
 *     resourceCapabilityAssertion
 *             |
 *             v
 *     ResourceCapabilityAssertion
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
 *     resourceCapabilityTargetRequirement
 *             |
 *             v
 *     ResourceCapabilityTargetRequirement
 *
 * These nodes are domain-neutral.
 *
 * ============================================================================
 * 18. RESOURCE NEGOTIATION INTEGRATION
 * ============================================================================
 *
 * Capability intent participates in:
 *
 *     source intent
 *          |
 *          v
 *     capability resolution
 *          |
 *          v
 *     resource analysis
 *          |
 *          v
 *     target capability matching
 *          |
 *          v
 *     negotiation
 *          |
 *          v
 *     execution planning
 *          |
 *          v
 *     target realization
 *
 * Negotiation itself belongs to:
 *
 *     grammar/resources/negotiation.g4
 *
 * and downstream resource infrastructure.
 *
 * This file never performs negotiation.
 *
 * ============================================================================
 * 19. REQUIREMENT INTEGRATION
 * ============================================================================
 *
 * Resource requirements remain owned by the resource requirement subsystem.
 *
 * The resource orchestrator should delegate capability-shaped requirements to:
 *
 *     resourceCapabilityRequirement
 *
 * while retaining ordinary resource expressions for:
 *
 *     requires memory >= required_memory;
 *     requires qubits >= required_qubits;
 *     requires nodes >= required_nodes;
 *
 * This preserves:
 *
 *     resource requirement
 *
 * versus:
 *
 *     capability requirement.
 *
 * ============================================================================
 * 20. CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * Capability-shaped constraints should delegate to:
 *
 *     resourceCapabilityConstraint
 *
 * Ordinary resource constraints remain handled by the resource constraint
 * subsystem.
 *
 * ============================================================================
 * 21. PREFERENCE INTEGRATION
 * ============================================================================
 *
 * Capability-shaped preferences should delegate to:
 *
 *     resourceCapabilityPreference
 *
 * Ordinary resource preferences remain handled by the preference subsystem.
 *
 * ============================================================================
 * 22. HINT INTEGRATION
 * ============================================================================
 *
 * Capability-shaped hints should delegate to:
 *
 *     resourceCapabilityHint
 *
 * Ordinary resource hints remain handled by the hint subsystem.
 *
 * ============================================================================
 * 23. EFFECT INTEGRATION
 * ============================================================================
 *
 * A capability reference may participate in effect validation.
 *
 * Example:
 *
 *     capability network::communication;
 *
 * does not itself produce:
 *
 *     network
 *
 * effect.
 *
 * Effect analysis determines the effect of the operation that uses the
 * capability.
 *
 * ============================================================================
 * 24. SECURITY INTEGRATION
 * ============================================================================
 *
 * Capability syntax does not grant authority.
 *
 * Security analysis may require:
 *
 *     capability authorization;
 *     policy approval;
 *     sandbox compatibility;
 *     trust evidence;
 *     provenance.
 *
 * Such checks occur downstream.
 *
 * ============================================================================
 * 25. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * The eventual semantic model should be able to record:
 *
 *     source capability;
 *     source span;
 *     resolved identity;
 *     resolved version;
 *     provider;
 *     evidence;
 *     decision;
 *     target realization.
 *
 * This grammar remains responsible only for preserving source structure.
 *
 * ============================================================================
 * 26. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Capability names may express any future quantum facility without modifying
 * this grammar.
 *
 * Examples:
 *
 *     requires capability quantum::measurement;
 *     requires capability quantum::dynamic_control;
 *     requires capability quantum::mid_circuit_measurement;
 *     requires capability quantum::fault_tolerant_execution;
 *
 * No quantum gate enumeration belongs here.
 *
 * No physical qubit selection belongs here.
 *
 * No coupling map belongs here.
 *
 * No routing decision belongs here.
 *
 * No QEC decision belongs here.
 *
 * ============================================================================
 * 27. CLASSICAL / GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability compute::parallel;
 *     requires capability compute::vector;
 *     requires capability tensor::compute;
 *     requires capability hardware::reconfigurable_logic;
 *     requires capability accelerator::matrix;
 *
 * These remain abstract semantic capabilities.
 *
 * ============================================================================
 * 28. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability distributed::communication;
 *     requires capability distributed::replication;
 *     requires capability network::rdma;
 *
 * The grammar does not define a node count.
 *
 * ============================================================================
 * 29. HDL INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability hdl::synthesis;
 *     requires capability hdl::verification;
 *     requires capability hardware::programmable_logic;
 *
 * Physical synthesis and realization are downstream.
 *
 * ============================================================================
 * 30. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability ai::training;
 *     requires capability ai::inference;
 *     requires capability ai::automatic_differentiation;
 *     requires capability tensor::compute;
 *     requires capability data::streaming;
 *
 * No model, framework, vendor, or algorithm is hard-coded.
 *
 * ============================================================================
 * 31. METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Capability references may participate in compile-time and metaprogramming
 * validation.
 *
 * Reflection, generation, and compile-time execution remain owned by:
 *
 *     grammar/metaprogramming/
 *
 * This file does not execute metaprograms.
 *
 * ============================================================================
 * 32. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream;
 *     grammar version;
 *     lexer vocabulary;
 *     parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability;
 *     target selection;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     runtime state;
 *     scheduler state.
 *
 * ============================================================================
 * 33. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * PARSER DIAGNOSTICS
 * ------------------
 *
 *     missing capability expression;
 *     missing semicolon;
 *     malformed property predicate;
 *     malformed relationship;
 *     missing relationship operand;
 *     malformed availability condition;
 *     malformed composition;
 *     malformed target capability expression.
 *
 * SEMANTIC DIAGNOSTICS
 * --------------------
 *
 *     unknown capability;
 *     invalid capability version;
 *     invalid property;
 *     invalid property type;
 *     invalid relationship;
 *     conflicting capability requirements;
 *     unsatisfied capability requirement;
 *     unavailable capability.
 *
 * RESOURCE/TARGET DIAGNOSTICS
 * ---------------------------
 *
 *     insufficient resources;
 *     unsupported capability;
 *     unavailable target;
 *     infeasible realization.
 *
 * A target-feasibility failure MUST NOT be reported as a syntax error.
 *
 * ============================================================================
 * 34. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve source spans for:
 *
 *     capability keyword;
 *     capability reference;
 *     capability expression;
 *     version expression;
 *     property path;
 *     predicate operator;
 *     predicate value;
 *     relationship name;
 *     availability expression;
 *     target capability expression.
 *
 * This supports:
 *
 *     diagnostics;
 *     IDE/LSP;
 *     formatting;
 *     refactoring;
 *     provenance;
 *     compatibility tooling.
 *
 * ============================================================================
 * 35. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable capability identity syntax remains owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file does not rename or duplicate:
 *
 *     capabilityReference;
 *     capabilityName;
 *     capabilityVersionClause;
 *     capabilityExpression.
 *
 * Adding a new capability identity does not require a grammar change.
 *
 * Adding a new reserved language keyword follows the normal lexer compatibility
 * process and is outside this file.
 *
 * ============================================================================
 * 36. ANTLR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar.
 *
 * It MUST be generated using the canonical:
 *
 *     ZamaniLexer
 *
 * token vocabulary.
 *
 * The build system MUST make the imported parser grammars available through
 * ANTLR's grammar library path.
 *
 * The resulting generated parser remains one part of the canonical Zamani
 * frontend.
 *
 * ============================================================================
 * 37. RESOURCE ORCHESTRATOR INTEGRATION
 * ============================================================================
 *
 * grammar/resources/resources.g4 remains the owner of:
 *
 *     resources
 *     resourceItem
 *     resourceClause
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *
 * It MUST NOT duplicate the specialized capability syntax owned here.
 *
 * Required delegation is:
 *
 *     resourceCapability
 *         : resourceCapabilityIntent
 *         ;
 *
 * Capability-shaped universal clauses should delegate as follows:
 *
 *     resourceRequirement
 *         -> resourceCapabilityRequirement
 *
 *     resourceConstraint
 *         -> resourceCapabilityConstraint
 *
 *     resourcePreference
 *         -> resourceCapabilityPreference
 *
 *     resourceHint
 *         -> resourceCapabilityHint
 *
 * Ordinary resource expressions remain handled by their existing resource
 * grammars.
 *
 * This establishes one owner for each capability-specific syntax family.
 *
 * ============================================================================
 * 38. NO IMPORT CYCLE
 * ============================================================================
 *
 * This file MUST NOT import:
 *
 *     grammar/resources/resources.g4
 *
 * The dependency direction is:
 *
 *     resources/capabilities.g4
 *              |
 *              v
 *       resources.g4
 *
 * not:
 *
 *     resources.g4
 *              |
 *              v
 *       resources/capabilities.g4
 *              |
 *              v
 *       resources.g4
 *
 * The parent orchestrator consumes this leaf.
 *
 * ============================================================================
 * 39. NO SECOND IR
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     CapabilityIR
 *     ResourceCapabilityIR
 *     QuantumCapabilityIR
 *     HardwareCapabilityIR
 *
 * Capability information becomes semantic metadata and/or constraints in the
 * canonical semantic representation.
 *
 * Quantum-related capability information eventually participates in:
 *
 *     quantum::ir
 *
 * when the corresponding computation reaches the quantum semantic boundary.
 *
 * ============================================================================
 * 40. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Required scalability tests include:
 *
 *     many capability references;
 *     many capability requirements;
 *     many capability properties;
 *     deeply qualified capability names;
 *     deeply qualified property paths;
 *     large capability expressions;
 *     large capability compositions;
 *     many relationships;
 *     large symbolic expressions;
 *     mixed-domain capability requirements.
 *
 * Tests must verify that no grammar-level artificial machine ceiling exists.
 *
 * They MUST NOT require literal infinite input.
 *
 * ============================================================================
 * 41. POSITIVE CONFORMANCE CASES
 * ============================================================================
 *
 * Examples:
 *
 *     capability compute::parallel;
 *
 *     capability quantum::measurement;
 *
 *     requires capability quantum::measurement;
 *
 *     requires capability quantum::measurement
 *         and classical::control;
 *
 *     requires resource capability tensor::compute;
 *
 *     requires resource memory::coherent;
 *
 *     constraint capability quantum::measurement;
 *
 *     constraint capability quantum::device
 *         property fidelity >= required_fidelity;
 *
 *     prefer capability compute::vector;
 *
 *     prefer capability quantum::low_noise;
 *
 *     hint capability accelerator::matrix;
 *
 *     capability quantum::measurement
 *         availability = execution_context.supports_measurement;
 *
 *     capability quantum::A
 *         implies capability quantum::B;
 *
 *     capability quantum::A
 *         excludes capability quantum::B;
 *
 *     capability {
 *         quantum::measurement;
 *         quantum::dynamic_control;
 *         policy::mode = preferred_mode;
 *     };
 *
 *     target capability quantum::measurement;
 *
 * ============================================================================
 * 42. NEGATIVE CONFORMANCE CASES
 * ============================================================================
 *
 * Required failures include:
 *
 *     requires capability;
 *
 *     requires capability ;
 *
 *     constraint capability;
 *
 *     prefer capability;
 *
 *     hint capability;
 *
 *     capability quantum::measurement availability;
 *
 *     capability quantum::A implies;
 *
 *     capability quantum::A implies capability;
 *
 *     capability quantum::device property;
 *
 *     capability quantum::device property fidelity;
 *
 *     capability {
 *         quantum::measurement
 *     }
 *
 * where the terminating semicolon is required.
 *
 * ============================================================================
 * 43. BOUNDARY CONFORMANCE CASES
 * ============================================================================
 *
 * Verify:
 *
 *     requirement vs capability;
 *     requirement vs constraint;
 *     preference vs requirement;
 *     hint vs preference;
 *     capability vs resource;
 *     capability vs target;
 *     capability syntax vs physical allocation;
 *     capability syntax vs runtime discovery.
 *
 * In particular:
 *
 *     requires capability quantum::measurement;
 *
 * MUST NOT imply:
 *
 *     allocation of a physical quantum processor.
 *
 * ============================================================================
 * 44. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 * No universal machine-capacity constant exists.
 *
 * No physical device enumeration exists.
 *
 * No finite capability catalog exists.
 *
 * No finite vendor catalog exists.
 *
 * No fixed qubit count exists.
 *
 * No fixed processor count exists.
 *
 * No fixed accelerator count exists.
 *
 * No fixed topology exists.
 *
 * No fixed namespace depth exists.
 *
 * No fixed property count exists.
 *
 * No fixed relationship count exists.
 *
 * ============================================================================
 * 45. SAFETY AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 *     no Rust actions;
 *     no semantic predicates;
 *     no I/O;
 *     no network access;
 *     no hardware discovery;
 *     no resource allocation;
 *     no runtime execution;
 *     no unsafe implementation requirement.
 *
 * Generated Rust must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * ============================================================================
 * 46. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It has one parser grammar identity.
 *     [x] It consumes ZamaniLexer.
 *     [x] It imports canonical Capabilities.
 *     [x] It imports canonical Expressions.
 *     [x] It does not duplicate capability identity syntax.
 *     [x] It does not duplicate identifier syntax.
 *     [x] It does not duplicate general expressions.
 *     [x] It provides one specialized capability entry point.
 *     [x] It distinguishes capability assertion, requirement, constraint,
 *         preference, and hint.
 *     [x] It supports availability conditions.
 *     [x] It supports capability properties.
 *     [x] It supports open-world relationships.
 *     [x] It supports capability composition.
 *     [x] It supports abstract target capability intent.
 *     [x] It remains open-world.
 *     [x] It contains no physical resource selection.
 *     [x] It contains no machine-size ceiling.
 *     [x] It contains no second IR.
 *     [x] It preserves quantum::ir as the canonical quantum IR boundary.
 *     [x] It contains no Rust actions.
 *     [x] It requires no unsafe Rust.
 *     [x] It defines parser/semantic/IR boundaries.
 *     [x] It defines diagnostics.
 *     [x] It defines source-span requirements.
 *     [x] It defines compatibility behavior.
 *     [x] It defines scalability tests.
 *     [x] It defines downstream integration.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What capability-related intent is associated with a resource context?"
 *
 * It does NOT answer:
 *
 *     "Which physical machine should execute it?"
 *
 *     "Which physical device should be selected?"
 *
 *     "Which processor should execute it?"
 *
 *     "Which accelerator should execute it?"
 *
 *     "Which physical qubit should be used?"
 *
 *     "How should the computation be routed?"
 *
 *     "How should the computation be scheduled?"
 *
 *     "How should error correction be performed?"
 *
 *     "How should the HAL realize it?"
 *
 * Those questions remain downstream.
 *
 * ============================================================================
 */