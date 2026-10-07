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
 *     CANONICAL HARDWARE RELIABILITY INTENT LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the SOURCE-LEVEL, TARGET-INDEPENDENT syntax for
 * expressing hardware reliability and resilience intent.
 *
 * It allows a Zamani program to describe requirements concerning:
 *
 *     reliability
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
 *     resilience
 *     degradation
 *     recovery
 *     reliability scaling
 *     reliability capabilities
 *     reliability resources
 *     reliability profiles
 *     reliability assertions
 *     evidence
 *     policy references
 *
 * The grammar describes WHAT the program requires or prefers.
 *
 * It does NOT describe HOW a target physically realizes that intent.
 *
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *     hardwareReliabilityDeclaration
 *     hardware reliability declaration modifiers
 *     hardware reliability declaration kinds
 *     hardware reliability bodies
 *     reliability requirements
 *     reliability constraints
 *     reliability preferences
 *     reliability hints
 *     reliability properties
 *     reliability capability references
 *     reliability resource references
 *     reliability target references
 *     reliability profiles
 *     reliability groups
 *     reliability resilience blocks
 *     reliability assertions
 *     reliability evidence
 *     reliability policy references
 *
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     identifiers
 *     qualified-name syntax
 *     literal syntax
 *     expression precedence
 *     universal resource declarations
 *     universal capability declarations
 *     generic contracts
 *     generic policies
 *     hardware allocation
 *     device discovery
 *     physical topology
 *     routing
 *     scheduling
 *     placement algorithms
 *     calibration
 *     thermal implementation
 *     power implementation
 *     runtime recovery
 *     QEC implementation
 *     ZQN implementation
 *     quantum::ir
 *     HAL implementation
 *     backend selection
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/expressions.g4
 *
 * The canonical expression grammar is imported directly.
 *
 * This is deliberate.
 *
 * The leaf grammar must not depend on an accidental transitive import from
 * hardware.g4 merely to obtain `expression`.
 *
 *
 * EXPORTS:
 *
 *     hardwareReliabilityDeclaration
 *     hardwareReliabilityBody
 *     hardwareReliabilityItem
 *     hardwareReliabilityRequirement
 *     hardwareReliabilityConstraint
 *     hardwareReliabilityPreference
 *     hardwareReliabilityHint
 *     hardwareReliabilityProperty
 *     hardwareReliabilityCapability
 *     hardwareReliabilityResource
 *     hardwareReliabilityTarget
 *     hardwareReliabilityProfile
 *     hardwareReliabilityGroup
 *     hardwareReliabilityResilience
 *     hardwareReliabilityAssertion
 *     hardwareReliabilityEvidence
 *     hardwareReliabilityPolicy
 *
 *
 * CONSUMED BY:
 *
 *     grammar/hardware/hardware.g4
 *
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST subsystem
 *
 *
 * SEMANTIC_OWNER:
 *
 *     hardware reliability semantic subsystem
 *     resilience semantic subsystem
 *     resource/capability semantic subsystems
 *
 *
 * TYPE_OWNER:
 *
 *     canonical Zamani type system
 *
 *
 * EFFECT_OWNER:
 *
 *     canonical effect subsystem
 *
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/
 *     resource semantic subsystem
 *
 *
 * CAPABILITY_OWNER:
 *
 *     grammar/core/capabilities.g4
 *     hardware capability semantic subsystem
 *
 *
 * CONTRACT_OWNER:
 *
 *     grammar/validation/
 *
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *     grammar/security/
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     domain-specific downstream IRs
 *
 * Quantum reliability information ultimately participates in:
 *
 *     quantum::ir
 *
 * followed by:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/hardware/
 *     grammar/tests/resources/
 *     grammar/tests/reliability/
 *     grammar/tests/quantum/
 *     grammar/tests/hybrid/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/negative/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/reliability.md
 *     grammar/spec/resources.md
 *     grammar/spec/policies.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * Reliability is CROSS-CUTTING semantic information.
 *
 * It is not a separate intermediate representation.
 *
 * Therefore this grammar MUST NOT create:
 *
 *     ReliabilityIR
 *     HardwareReliabilityIR
 *     QuantumReliabilityIR
 *     ResilienceIR
 *
 * Reliability information remains attached to the domain-neutral semantic
 * model and is consumed by the appropriate downstream subsystem.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A reliability declaration must remain meaningful when realization changes
 * between:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     edge systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * The source program expresses reliability intent.
 *
 * The compiler/runtime determines whether that intent can be realized.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO language-level maximum for:
 *
 *     reliability declarations
 *     reliability properties
 *     profiles
 *     groups
 *     nested groups
 *     nested resilience blocks
 *     failure domains
 *     replicas
 *     recovery objectives
 *     resources
 *     capabilities
 *     devices
 *     nodes
 *     processors
 *     accelerators
 *     quantum resources
 *     qubits
 *
 * The grammar MUST NOT introduce any universal capacity constants.
 *
 * In particular, this file contains no:
 *
 *     MAX_RELIABILITY_CONTRACTS
 *     MAX_FAILURE_DOMAINS
 *     MAX_REDUNDANCY
 *     MAX_RECOVERY_STEPS
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
 *     MAX_REPLICAS
 *     MAX_STATES
 *     MAX_OUTCOMES
 *
 * Repetition is represented through normal ANTLR repetition operators.
 *
 * Physical scale is determined by:
 *
 *     available resources
 *     target capabilities
 *     compiler resources
 *     runtime resources
 *     deployment policy
 *     physical feasibility
 *
 * The language itself imposes no artificial hardware ceiling.
 *
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * REQUIREMENT
 *     A mandatory semantic condition.
 *
 * CONSTRAINT
 *     A mandatory restriction on valid realization.
 *
 * PREFERENCE
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *     Advisory information.
 *
 * These categories MUST remain structurally distinguishable.
 *
 * A backend MUST NOT silently weaken:
 *
 *     requirement -> preference
 *
 * or:
 *
 *     constraint -> hint
 *
 * because a particular realization cannot satisfy the stronger form.
 *
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT select:
 *
 *     physical CPU
 *     physical GPU
 *     physical FPGA
 *     physical ASIC
 *     physical QPU
 *     physical qubit
 *     physical node
 *     physical memory bank
 *     PCI address
 *     serial number
 *     machine hostname
 *     vendor-specific device instance
 *
 * A symbolic target expression is allowed.
 *
 * Physical realization remains downstream.
 *
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * All values and conditions use the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This file does NOT create:
 *
 *     hardwareExpression
 *     reliabilityExpression
 *     reliabilityLogicalExpression
 *     reliabilityArithmeticExpression
 *
 * as competing expression systems.
 *
 * This is important for production maintainability.
 *
 * A future expression-system improvement therefore propagates naturally to
 * reliability syntax without requiring this file to duplicate that change.
 *
 *
 * ============================================================================
 * LEXICAL INTEGRATION
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Its vocabulary is assembled from:
 *
 *     grammar/lexer/
 *
 * This file therefore uses the actual canonical token names:
 *
 *     RELIABILITY
 *     RESILIENCE
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     RESOURCE
 *     TARGET
 *     PROFILE
 *     GROUP
 *     ASSERT
 *     EVIDENCE
 *     POLICY
 *     PROPERTY
 *     SCALABILITY
 *     AVAILABILITY
 *     CAPACITY
 *     PERFORMANCE
 *     LATENCY
 *     THROUGHPUT
 *     BANDWIDTH
 *     ENERGY
 *     POWER
 *     COST
 *
 * No K_* aliases are invented here.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     canonical token stream
 *     grammar version
 *     explicit parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     device health
 *     runtime state
 *     resource availability
 *     network state
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     environment variables
 *
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no target-language actions
 *     no semantic predicates
 *     no I/O
 *     no hardware access
 *     no runtime execution
 *
 * It therefore imposes no unsafe Rust requirement.
 *
 * The Rust frontend consuming generated ANTLR artifacts MUST remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Reliability requirements attached to quantum computation remain above the
 * physical quantum realization boundary.
 *
 * The conceptual pipeline is:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic reliability analysis
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
 * This grammar does NOT define:
 *
 *     physical qubits
 *     physical topology
 *     gate calibration
 *     QEC algorithms
 *     syndrome decoders
 *     pulse schedules
 *     QPU allocation
 *
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Hardware reliability is distinct from HDL behavioral semantics.
 *
 * HDL remains owned by:
 *
 *     grammar/hdl/
 *
 * Hardware realization intent remains owned by:
 *
 *     grammar/hardware/
 *
 * Reliability may constrain an HDL/hardware computation, but this grammar
 * does not define:
 *
 *     signals
 *     procedural blocks
 *     clock semantics
 *     synthesis
 *     RTL behavior
 *     physical layout
 *
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Hardware reliability may refer to resources:
 *
 *     requires resource::compute >= required_compute;
 *
 *     requires resource::memory >= required_memory;
 *
 *     resource resource::recovery;
 *
 * Such references are symbolic.
 *
 * They do not allocate resources.
 *
 * Resource interpretation remains owned by:
 *
 *     grammar/resources/
 *
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability references remain open-world.
 *
 * Examples:
 *
 *     capability quantum::error_correction;
 *
 *     capability distributed::failover;
 *
 *     capability resilience::recovery;
 *
 * No finite capability catalogue is encoded.
 *
 *
 * ============================================================================
 * RESILIENCE STATE / OUTCOME INTEGRATION
 * ============================================================================
 *
 * Standard semantic resilience states include:
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
 * Standard semantic outcomes include:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These are SEMANTIC VALUES.
 *
 * They are deliberately not implemented as a closed parser enumeration.
 *
 * Future resilience states and outcomes therefore remain representable.
 *
 *
 * ============================================================================
 * PROPERTY MODEL
 * ============================================================================
 *
 * Reliability properties use:
 *
 *     property-name relation expression ;
 *
 * Examples:
 *
 *     availability >= required_availability;
 *
 *     failure_probability <= acceptable_probability;
 *
 *     failure_rate <= acceptable_rate;
 *
 *     mtbf >= required_mtbf;
 *
 *     mttr <= recovery_budget;
 *
 *     redundancy >= required_redundancy;
 *
 *     resilience_state == Healthy;
 *
 *     resilience_outcome == ACCEPT;
 *
 *     failure_domain == execution::domain;
 *
 * Property names remain extensible.
 *
 * Only the currently reserved reliability/resource vocabulary that can appear
 * in property position is explicitly admitted below. All other names remain
 * ordinary identifiers.
 *
 *
 * ============================================================================
 * DECLARATION MODEL
 * ============================================================================
 *
 * Canonical forms include:
 *
 *     reliability {
 *         availability >= required_availability;
 *     }
 *
 *     reliability contract compute {
 *         requires availability >= required_availability;
 *         constraint failure_probability <= allowed_probability;
 *     }
 *
 *     reliability profile resilient {
 *         redundancy >= required_redundancy;
 *     }
 *
 *     reliability target {
 *         ...
 *     }
 *
 * The target clause is symbolic and never selects a physical device.
 *
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareReliabilityParser;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * hardware.g4 exposes this rule through its hardwareDeclaration dispatcher.
 *
 * No EOF is used here because this is a leaf grammar consumed inside the
 * universal program grammar.
 */

hardwareReliabilityDeclaration
    : RELIABILITY
      hardwareReliabilityDeclarationModifier*
      hardwareReliabilityDeclarationKind?
      IDENTIFIER?
      hardwareReliabilityTargetClause?
      hardwareReliabilityBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * DECLARATION KINDS
 * ============================================================================
 */

hardwareReliabilityDeclarationKind
    : CONTRACT
    | PROFILE
    ;


/*
 * ============================================================================
 * DECLARATION MODIFIERS
 * ============================================================================
 *
 * These are existing language modifiers.
 *
 * They carry no physical hardware meaning.
 */

hardwareReliabilityDeclarationModifier
    : STATIC
    | CONST
    | EXTERN
    | FINAL
    | ABSTRACT
    | SEALED
    | PARTIAL
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * Example:
 *
 *     reliability target hardware::accelerator {
 *         ...
 *     }
 *
 * `expression` owns the actual reference syntax.
 */

hardwareReliabilityTargetClause
    : TARGET
      expression
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 */

hardwareReliabilityBody
    : LBRACE
      hardwareReliabilityItem*
      RBRACE
    ;


/*
 * ============================================================================
 * BODY DISPATCH
 * ============================================================================
 *
 * Every alternative has a distinct structural owner.
 *
 * Generic property syntax is represented exactly once.
 *
 * This deliberately eliminates the old design in which multiple productions
 * began with IDENTIFIER and therefore competed for the same syntax.
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
    | hardwareReliabilityResilience
    | hardwareReliabilityAssertion
    | hardwareReliabilityEvidence
    | hardwareReliabilityPolicy
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     requires availability >= required_availability;
 *
 * The complete condition is delegated to the canonical expression grammar.
 */

hardwareReliabilityRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Example:
 *
 *     constraint failure_probability <= acceptable_probability;
 */

hardwareReliabilityConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     prefer availability >= preferred_availability;
 */

hardwareReliabilityPreference
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * A hint is advisory only.
 *
 * It MUST NOT acquire requirement or constraint semantics during parsing.
 */

hardwareReliabilityHint
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * Generic reliability property.
 *
 * Examples:
 *
 *     availability >= required_availability;
 *     failure_rate <= allowed_failure_rate;
 *     mttr <= recovery_budget;
 *     redundancy >= required_redundancy;
 *     resilience_state == Healthy;
 *
 * Property semantics are resolved downstream.
 */

hardwareReliabilityProperty
    : hardwareReliabilityPropertyName
      hardwareReliabilityComparisonOperator
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 *
 * Open-world property names remain possible through IDENTIFIER.
 *
 * Reserved language words that are valid semantic property names are admitted
 * explicitly because the lexer correctly emits them as keyword tokens.
 *
 * This prevents:
 *
 *     availability
 *
 * from becoming unusable as a property simply because the lexer reserves the
 * word.
 */

hardwareReliabilityPropertyName
    : hardwareReliabilityPropertySegment
      (
          DOUBLE_COLON
          hardwareReliabilityPropertySegment
      )*
    ;


hardwareReliabilityPropertySegment
    : IDENTIFIER
    | RELIABILITY
    | RESILIENCE
    | AVAILABILITY
    | CAPACITY
    | PERFORMANCE
    | LATENCY
    | THROUGHPUT
    | BANDWIDTH
    | ENERGY
    | POWER
    | COST
    | SCALABILITY
    | PORTABILITY
    | RESOURCE
    | CAPABILITY
    | TARGET
    | PROFILE
    | GROUP
    | PROPERTY
    ;


/*
 * ============================================================================
 * COMPARISON OPERATOR
 * ============================================================================
 *
 * Assignment is intentionally excluded.
 *
 * Therefore:
 *
 *     reliability = value;
 *
 * is not silently interpreted as a reliability property assignment.
 *
 * Property relations are semantic comparisons.
 *
 * If assignment-like reliability syntax is ever required, it must be added
 * deliberately and documented as a separate construct.
 */

hardwareReliabilityComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     capability quantum::error_correction;
 *
 *     capability resilience::recovery;
 *
 *     capability capability_reference();
 *
 * The value is an expression so future capability-reference mechanisms do not
 * require grammar changes.
 */

hardwareReliabilityCapability
    : CAPABILITY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESOURCE
 * ============================================================================
 *
 * A resource reference is symbolic.
 *
 * It does not allocate, reserve, or discover a physical resource.
 *
 * Examples:
 *
 *     resource resource::compute;
 *
 *     resource resource::memory;
 *
 *     resource resource::recovery;
 */

hardwareReliabilityResource
    : RESOURCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * TARGET REFERENCE
 * ============================================================================
 *
 * This is a semantic target reference.
 *
 * It is not physical device selection.
 */

hardwareReliabilityTarget
    : TARGET
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROFILE
 * ============================================================================
 *
 * A profile is a named collection of reliability intent.
 *
 * Profiles remain open-ended.
 *
 * Example:
 *
 *     profile resilient {
 *         availability >= required_availability;
 *         redundancy >= required_redundancy;
 *     }
 */

hardwareReliabilityProfile
    : PROFILE
      IDENTIFIER
      hardwareReliabilityBody
    ;


/*
 * ============================================================================
 * GROUP
 * ============================================================================
 *
 * A group is a logical grouping mechanism.
 *
 * It does not imply physical topology.
 *
 * A semantic implementation may later map a group to:
 *
 *     service
 *     component
 *     task
 *     execution region
 *     resource set
 *     deployment unit
 *     failure domain
 *
 * without changing source syntax.
 */

hardwareReliabilityGroup
    : GROUP
      IDENTIFIER
      hardwareReliabilityBody
    ;


/*
 * ============================================================================
 * RESILIENCE
 * ============================================================================
 *
 * Both scalar and block forms are supported.
 *
 * Scalar:
 *
 *     resilience >= required_resilience;
 *
 *     resilience == required_resilience;
 *
 * Block:
 *
 *     resilience {
 *         resilience_state == Healthy;
 *         resilience_outcome == ACCEPT;
 *         recovery_time <= recovery_budget;
 *     }
 *
 * The grammar does not implement recovery.
 */

hardwareReliabilityResilience
    : RESILIENCE
      hardwareReliabilityResilienceBody
      SEMICOLON?
    | RESILIENCE
      hardwareReliabilityComparisonOperator
      expression
      SEMICOLON
    ;


hardwareReliabilityResilienceBody
    : LBRACE
      hardwareReliabilityResilienceItem*
      RBRACE
    ;


hardwareReliabilityResilienceItem
    : hardwareReliabilityRequirement
    | hardwareReliabilityConstraint
    | hardwareReliabilityPreference
    | hardwareReliabilityHint
    | hardwareReliabilityProperty
    | hardwareReliabilityCapability
    | hardwareReliabilityResource
    | hardwareReliabilityTarget
    | hardwareReliabilityAssertion
    | hardwareReliabilityEvidence
    | hardwareReliabilityPolicy
    | hardwareReliabilityResilience
    ;


/*
 * ============================================================================
 * ASSERTION
 * ============================================================================
 *
 * Example:
 *
 *     assert(availability >= required_availability);
 *
 * This is a source-level assertion.
 *
 * It does not perform recovery or runtime fault handling.
 */

hardwareReliabilityAssertion
    : ASSERT
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Evidence is declarative information associated with reliability reasoning.
 *
 * It may ultimately be associated with:
 *
 *     measurements
 *     benchmarks
 *     calibration
 *     observations
 *     provenance
 *     verification
 *     external evidence
 *
 * The grammar does not validate evidence quality.
 */

hardwareReliabilityEvidence
    : EVIDENCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Policy references are symbolic.
 *
 * Policy evaluation remains owned by the policy/security semantic layers.
 */

hardwareReliabilityPolicy
    : POLICY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * COMMON PROFILE/GROUP ITEM BOUNDARY
 * ============================================================================
 *
 * This rule intentionally reuses the same canonical reliability items.
 *
 * There is no second reliability-property language for profiles.
 */

hardwareReliabilityScopedItem
    : hardwareReliabilityRequirement
    | hardwareReliabilityConstraint
    | hardwareReliabilityPreference
    | hardwareReliabilityHint
    | hardwareReliabilityProperty
    | hardwareReliabilityCapability
    | hardwareReliabilityResource
    | hardwareReliabilityTarget
    | hardwareReliabilityResilience
    | hardwareReliabilityAssertion
    | hardwareReliabilityEvidence
    | hardwareReliabilityPolicy
    | hardwareReliabilityProfile
    | hardwareReliabilityGroup
    ;


/*
 * ============================================================================
 * SEMANTIC PROPERTY VOCABULARY
 * ============================================================================
 *
 * These names are intentionally NOT converted into parser-level closed
 * enumerations.
 *
 * Examples include:
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
 *     noise_budget
 *     error_budget
 *     recovery_budget
 *
 * Names not reserved by the lexer remain IDENTIFIER tokens.
 *
 * The semantic subsystem decides:
 *
 *     whether a property exists;
 *     its type;
 *     its unit;
 *     its probability domain;
 *     its reliability model;
 *     whether it is applicable;
 *     whether it is satisfiable.
 *
 *
 * ============================================================================
 * STANDARD RESILIENCE STATES
 * ============================================================================
 *
 * These are semantic values rather than parser alternatives.
 *
 * Standard vocabulary:
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
 * The grammar accepts them through the canonical expression/name system.
 *
 * Future states remain possible without grammar modification.
 *
 *
 * ============================================================================
 * STANDARD RESILIENCE OUTCOMES
 * ============================================================================
 *
 * Standard vocabulary:
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
 * This grammar does not define a finite outcome enumeration.
 *
 *
 * ============================================================================
 * FAILURE DOMAIN SEMANTICS
 * ============================================================================
 *
 * Failure-domain intent is normally expressed as a property:
 *
 *     failure_domain == execution::domain;
 *
 *     common_mode_failure == false;
 *
 *     correlated_failure <= permitted_correlation;
 *
 * The parser therefore does not create a separate physical failure-domain
 * grammar.
 *
 * This is intentional.
 *
 * A failure domain can later map to:
 *
 *     process
 *     task
 *     service
 *     logical resource
 *     node
 *     device class
 *     deployment unit
 *     QEC region
 *     other semantic domain
 *
 * without changing the source grammar.
 *
 *
 * ============================================================================
 * REDUNDANCY SEMANTICS
 * ============================================================================
 *
 * Redundancy is expressed as a semantic property:
 *
 *     redundancy >= required_redundancy;
 *
 *     replication >= required_replication;
 *
 * The parser does not interpret the number as a fixed hardware allocation.
 *
 * Semantic analysis determines:
 *
 *     replication model
 *     independence
 *     correlated failure behavior
 *     placement implications
 *     resource cost
 *     feasibility
 *
 *
 * ============================================================================
 * RECOVERY SEMANTICS
 * ============================================================================
 *
 * Recovery objectives are expressed as properties:
 *
 *     recovery_time <= recovery_budget;
 *
 *     recovery_probability >= required_probability;
 *
 *     recovery_overhead <= permitted_overhead;
 *
 *     recoverability >= required_recoverability;
 *
 * This grammar does not implement:
 *
 *     retry
 *     rollback
 *     checkpoint restoration
 *     migration
 *     failover
 *     repair
 *     replacement
 *     quarantine
 *
 * Those are downstream resilience/runtime responsibilities.
 *
 *
 * ============================================================================
 * QUANTUM RELIABILITY
 * ============================================================================
 *
 * Quantum-specific reliability properties remain open-world:
 *
 *     quantum::error_rate
 *     quantum::logical_error_rate
 *     quantum::error_correction
 *     quantum::fault_tolerance
 *     quantum::noise_budget
 *     quantum::readout_fidelity
 *
 * Example:
 *
 *     reliability quantum {
 *         requires capability quantum::error_correction;
 *         quantum::noise_budget <= allowed_noise;
 *         quantum::logical_error_rate <= target_error_rate;
 *     }
 *
 * The parser does not know whether the realization uses:
 *
 *     physical qubits
 *     logical qubits
 *     a simulator
 *     a QPU
 *     a future quantum substrate
 *
 *
 * ============================================================================
 * CLASSICAL / GPU / FPGA / ASIC / ACCELERATOR RELIABILITY
 * ============================================================================
 *
 * The same grammar supports:
 *
 *     CPU reliability
 *     GPU reliability
 *     FPGA reliability
 *     ASIC reliability
 *     accelerator reliability
 *     embedded reliability
 *     distributed reliability
 *     hybrid reliability
 *
 * without creating a different reliability language for each target.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * A typical semantic pipeline is:
 *
 *     reliability requirement
 *          |
 *          v
 *     semantic validation
 *          |
 *          +--> resource analysis
 *          |
 *          +--> capability analysis
 *          |
 *          +--> contract analysis
 *          |
 *          +--> policy analysis
 *          |
 *          +--> provenance
 *          |
 *          v
 *     target feasibility
 *          |
 *          v
 *     execution planning
 *
 * The grammar itself performs none of these operations.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser output must preserve:
 *
 *     declaration kind
 *     declaration name
 *     modifiers
 *     target
 *     body item category
 *     property name
 *     comparison operator
 *     expression
 *     source spans
 *
 * The domain-neutral AST must not discard whether an item was:
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
 *     resilience
 *     assertion
 *     evidence
 *     policy
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     property recognition
 *     type checking
 *     unit checking
 *     dimensional checking
 *     probability validation
 *     reliability-model validation
 *     capability resolution
 *     resource resolution
 *     target compatibility
 *     failure-domain semantics
 *     redundancy semantics
 *     recovery semantics
 *     resilience semantics
 *     state validation
 *     outcome validation
 *     contradiction detection
 *     satisfiability
 *     diagnostics
 *
 * For example:
 *
 *     availability >= 0.999999;
 *
 * is syntactically valid.
 *
 * Whether the left side is a valid availability quantity and whether the
 * target can satisfy it is a semantic question.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY INTEGRATION
 * ============================================================================
 *
 * Generic contracts remain owned by:
 *
 *     grammar/validation/
 *
 * Generic policies remain owned by:
 *
 *     grammar/policies/
 *     grammar/security/
 *
 * This file only allows reliability declarations to reference those concepts
 * through their canonical semantic forms.
 *
 * It does not create competing contract or policy systems.
 *
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Reliability evidence and decisions may participate in the universal
 * provenance system.
 *
 * Provenance may record:
 *
 *     source
 *     derived value
 *     evidence
 *     verification
 *     transformation
 *     decision
 *     policy
 *     target realization
 *
 * Provenance is not generated by this parser.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Reliability intent is represented in the domain-neutral semantic model.
 *
 * It may subsequently influence:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representations
 *     distributed execution plans
 *
 * There is no second reliability-specific IR.
 *
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime observations may establish:
 *
 *     health
 *     availability
 *     degradation
 *     recovery
 *     failure
 *     failover
 *     quarantine
 *     retirement
 *
 * Those observations do not modify parsing.
 *
 * Runtime state cannot become hidden parser input.
 *
 *
 * ============================================================================
 * RESILIENCE OUTCOME CONTRACT
 * ============================================================================
 *
 * The semantic/runtime resilience subsystem owns the interpretation of:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * This grammar only preserves the source representation.
 *
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Semantic errors must remain semantic diagnostics.
 *
 * Examples of semantic errors:
 *
 *     invalid reliability unit
 *     invalid probability range
 *     unsupported capability
 *     impossible resource requirement
 *     contradictory requirements
 *     invalid resilience state
 *     incompatible property types
 *     unsatisfied target requirement
 *
 * This grammar MUST NOT attempt to diagnose those conditions using actions or
 * semantic predicates.
 *
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar is intentionally structured around:
 *
 *     explicit keyword dispatch
 *     one canonical expression grammar
 *     one generic property production
 *     iterative collections
 *
 * It does not recursively enumerate:
 *
 *     hardware devices
 *     topology
 *     failure domains
 *     resources
 *     quantum systems
 *
 * Large reliability contracts therefore grow with source size rather than
 * requiring grammar expansion for each new resource or reliability property.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no hardware capacities
 *     no device IDs
 *     no physical topology
 *     no provider catalogue
 *     no finite reliability-property universe
 *     no finite resilience-state universe
 *     no finite resilience-outcome universe
 *     no retry limit
 *     no recovery-step limit
 *     no replica limit
 *     no failure-domain limit
 *     no qubit limit
 *     no CPU limit
 *     no GPU limit
 *     no FPGA limit
 *     no node limit
 *     no memory limit
 *     no thread limit
 *     no unsafe Rust
 *
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing resource-level reliability syntax remains owned by:
 *
 *     grammar/resources/reliability.g4
 *
 * This file does not replace or rename that grammar.
 *
 * The distinction is:
 *
 *     resources/reliability.g4
 *         resource-level reliability syntax
 *
 *     hardware/reliability.g4
 *         hardware-domain reliability intent
 *
 * Both converge in semantic analysis.
 *
 *
 * ============================================================================
 * HARDWARE COMPOSITION CONTRACT
 * ============================================================================
 *
 * `grammar/hardware/hardware.g4` MUST import:
 *
 *     ZamaniHardwareReliabilityParser
 *
 * and add:
 *
 *     hardwareReliabilityDeclaration
 *
 * to:
 *
 *     hardwareDeclaration
 *
 * This is the ONLY hardware-root integration required for this leaf grammar.
 *
 * The hardware composition root remains responsible for exposing this rule
 * through the universal parser.
 *
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * ANTLR must have access to:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/expressions.g4
 *     grammar/hardware/reliability.g4
 *
 * and the transitive imported parser grammars.
 *
 * The repository's build tooling must make the grammar directories available
 * through ANTLR's grammar library path.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 *     reliability {
 *         availability >= required_availability;
 *     }
 *
 *     reliability contract compute {
 *         requires availability >= required_availability;
 *         constraint failure_probability <= allowed_probability;
 *         prefer reliability >= preferred_reliability;
 *         hint reliability::optimization;
 *     }
 *
 *     reliability profile resilient {
 *         redundancy >= required_redundancy;
 *         mttr <= recovery_budget;
 *     }
 *
 *     reliability group service {
 *         availability >= required_availability;
 *     }
 *
 *     reliability target hardware::accelerator {
 *         capability hardware::fault_tolerance;
 *     }
 *
 *     reliability quantum {
 *         requires capability::quantum::error_correction;
 *         quantum::noise_budget <= allowed_noise;
 *         quantum::logical_error_rate <= target_error_rate;
 *         resilience {
 *             resilience_state == Healthy;
 *             resilience_outcome == ACCEPT;
 *         }
 *     }
 *
 *
 * OPEN-WORLD PROPERTY TESTS
 * -------------------------
 *
 *     reliability {
 *         future::reliability_metric >= future_requirement;
 *         vendor_extension::property == expected;
 *         new_domain::metric <= limit;
 *     }
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Tests must generate collections based on available test resources rather
 * than repository constants.
 *
 * Test:
 *
 *     many properties
 *     many groups
 *     many profiles
 *     deep symbolic namespaces
 *     large expressions
 *     large source units
 *
 * No artificial language maximum may be used in these tests.
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     reliability {
 *         availability;
 *     }
 *
 *     reliability {
 *         availability = value;
 *     }
 *
 *     reliability contract {
 *         requires;
 *     }
 *
 *     reliability {
 *         requires availability >= ;
 *     }
 *
 *     reliability {
 *         resilience {
 *             resilience_state;
 *         }
 *     }
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Identical source and identical parser configuration must produce identical
 * parse structure and source spans.
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Test reliability requirements originating from:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     distributed
 *     accelerator
 *     networking
 *     AI
 *     data
 *     embedded
 *
 * without creating domain-specific reliability grammars.
 *
 *
 * ============================================================================
 * DEFINITION OF DONE
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It uses the actual canonical lexer token names.
 *     [x] It uses the canonical expression grammar.
 *     [x] It contains no K_* token aliases.
 *     [x] It contains no duplicate expression hierarchy.
 *     [x] It contains one generic property production.
 *     [x] It does not use competing IDENTIFIER-based reliability alternatives.
 *     [x] Requirement, constraint, preference and hint remain distinct.
 *     [x] Capability references remain open-world.
 *     [x] Resource references remain symbolic.
 *     [x] Targets remain abstract.
 *     [x] Resilience remains separate from reliability.
 *     [x] Resilience states remain extensible.
 *     [x] Resilience outcomes remain extensible.
 *     [x] Failure domains remain semantic rather than physical.
 *     [x] Redundancy remains semantic rather than physical allocation.
 *     [x] Recovery remains intent rather than implementation.
 *     [x] Evidence remains declarative.
 *     [x] Policy references remain downstream-owned.
 *     [x] No physical device selection is encoded.
 *     [x] No routing is encoded.
 *     [x] No scheduling is encoded.
 *     [x] No QEC implementation is encoded.
 *     [x] No ZQN implementation is encoded.
 *     [x] No second IR is created.
 *     [x] quantum::ir remains the quantum semantic boundary.
 *     [x] No hardware capacity constants exist.
 *     [x] No unsafe Rust is required.
 *     [x] Rust 1.97+ remains supported.
 *     [x] The grammar remains target-independent.
 *     [x] The grammar remains POCO-REAF compatible.
 *
 * Repository integration is complete when:
 *
 *     [ ] hardware.g4 imports this grammar.
 *     [ ] hardwareDeclaration dispatches hardwareReliabilityDeclaration.
 *     [ ] ANTLR generation succeeds.
 *     [ ] generated Rust compiles on Rust 1.97+.
 *     [ ] safe-Rust checks contain no unsafe requirement.
 *     [ ] positive tests pass.
 *     [ ] negative tests pass.
 *     [ ] scalability tests pass.
 *     [ ] cross-domain tests pass.
 *     [ ] determinism tests pass.
 *
 * ============================================================================
 */