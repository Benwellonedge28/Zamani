/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 *     grammar/resources/constraints.g4
 *
 * GRAMMAR
 *     constraints
 *
 * ROLE
 *     Resource-domain constraint atoms.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 * This grammar defines the resource-specific syntactic atoms consumed by
 * Zamani's generic constraint system.
 *
 * It describes portable resource intent such as:
 *
 *     memory.capacity >= required_memory
 *     quantum.logical_qubits >= logical_qubits
 *     network.bandwidth >= required_bandwidth
 *     execution.latency <= latency_budget
 *     capability("quantum.measurement")
 *     capability(quantum::dynamic_control)
 *
 * It does NOT:
 *
 *     - allocate resources;
 *     - reserve resources;
 *     - select a physical target;
 *     - discover hardware;
 *     - inspect runtime state;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform optimization;
 *     - perform QEC;
 *     - construct quantum::ir;
 *     - implement ZQN;
 *     - implement HAL behavior.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS
 * ----
 *     resourceConstraintAtom
 *     resourceComparisonConstraint
 *     resourceCapabilityConstraint
 *     resourcePredicateConstraint
 *     resourcePropertyConstraint
 *     resourceRelationshipConstraint
 *     resourceQuantityConstraint
 *     resourceCapacityConstraint
 *     resourceAvailabilityConstraint
 *     resourcePerformanceConstraint
 *     resourceLatencyConstraint
 *     resourceThroughputConstraint
 *     resourceBandwidthConstraint
 *     resourceEnergyConstraint
 *     resourcePowerConstraint
 *     resourceReliabilityConstraint
 *     resourceResilienceConstraint
 *     resourceCostConstraint
 *     resourceScalabilityConstraint
 *     resourcePortabilityConstraint
 *     resourceCompatibilityConstraint
 *
 * DOES NOT OWN
 * -----------
 *     Generic Boolean constraint composition.
 *     Generic `where ...` clauses.
 *     Generic comparison operators.
 *     General expressions.
 *     Names.
 *     Literals.
 *     Generic capabilities.
 *     Resource declarations.
 *     Concrete `constraint ...;` statement placement.
 *     Resource allocation.
 *     Resource negotiation.
 *     Target selection.
 *
 * ============================================================================
 * AUTHORITY BOUNDARIES
 * ============================================================================
 *
 * Generic constraints:
 *
 *     grammar/core/constraints.g4
 *
 * Resource expressions:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Generic capabilities:
 *
 *     grammar/core/capabilities.g4
 *
 * Resource orchestration:
 *
 *     grammar/resources/resources.g4
 *
 * Canonical root composition:
 *
 *     grammar/Zamani.g4
 *
 * Semantic resource model:
 *
 *     compiler/frontend semantic layer
 *
 * Quantum boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Resource constraints express PROGRAM INTENT, not machine identity.
 *
 * The grammar therefore contains:
 *
 *     - no maximum resource counts;
 *     - no fixed hardware capacities;
 *     - no finite device catalogue;
 *     - no vendor-specific resource universe;
 *     - no fixed quantum-operation universe;
 *     - no fixed tensor dimensions;
 *     - no fixed network size;
 *     - no fixed topology size.
 *
 * There are deliberately no language-level constants representing limits for:
 *
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     nodes
 *     memory
 *     threads
 *     tensor rank
 *     register width
 *     devices
 *     network size
 *
 * A value appearing in a program is a PROGRAM VALUE.
 *
 * It never becomes a language maximum merely because it is used in a
 * constraint.
 *
 * ============================================================================
 * OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * Resource names are semantic names.
 *
 * Examples:
 *
 *     memory
 *     memory.capacity
 *     quantum.logical_qubits
 *     accelerator.tensor.compute
 *     network.bandwidth
 *     future.compute.resource
 *
 * New resource kinds, properties, capabilities, architectures, technologies,
 * accelerators and future computational systems must be introducible through
 * semantic registration, dialects, capabilities or target metadata without
 * requiring a new universal parser rule merely because a new resource exists.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Resource:
 *     describes a resource/property/value.
 *
 * Capability:
 *     describes something a realization can provide.
 *
 * Requirement:
 *     describes something that must be satisfied.
 *
 * Constraint:
 *     describes a condition that must hold.
 *
 * Preference:
 *     influences realization without becoming a mandatory constraint.
 *
 * Allocation:
 *     reserves or acquires a resource.
 *
 * These concepts MUST NOT be collapsed into one grammar.
 *
 * ============================================================================
 * RUST / SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no unsafe Rust requirement.
 *
 * Generated Zamani frontend code targets:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Repository Rust implementation must remain safe Rust.
 *
 * ============================================================================
 */

parser grammar constraints;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names, Capabilities;


/*
 * ============================================================================
 * PUBLIC RESOURCE-CONSTRAINT ENTRY POINT
 * ============================================================================
 *
 * This rule is the resource-domain atomic boundary.
 *
 * Boolean composition is intentionally absent.
 *
 * Generic composition belongs to:
 *
 *     grammar/core/constraints.g4
 *
 * This prevents this grammar from creating a second AND/OR/NOT language.
 */
resourceConstraintAtom
    : resourceCapabilityConstraint
    | resourceComparisonConstraint
    | resourcePredicateConstraint
    ;


/*
 * ============================================================================
 * GENERIC RESOURCE COMPARISON
 * ============================================================================
 *
 * This is the primary scalable resource constraint form.
 *
 * Examples:
 *
 *     memory.capacity >= required_memory
 *
 *     quantum.logical_qubits >= logical_qubits
 *
 *     network.bandwidth >= required_bandwidth
 *
 *     execution.latency <= latency_budget
 *
 *     compute.parallelism >= desired_parallelism
 *
 *     future.resource.property == expected_value
 *
 * Both sides are canonical resource expressions.
 *
 * No resource dimension is hard-coded.
 */
resourceComparisonConstraint
    : resourceExpression
      resourceComparisonOperator
      resourceExpression
    ;


/*
 * ============================================================================
 * RESOURCE COMPARISON OPERATOR
 * ============================================================================
 *
 * ResourceExpressions owns the canonical operator vocabulary.
 *
 * This grammar deliberately does not redefine:
 *
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *
 * Therefore there is exactly one comparison-token authority.
 */
resourceComparisonOperator
    : EQ_EQ
    | NOT_EQ
    | LE
    | GE
    | LESS
    | GREATER
    ;


/*
 * ============================================================================
 * CAPABILITY CONSTRAINT
 * ============================================================================
 *
 * Capability constraints express that a realization must expose a capability.
 *
 * Supported forms include:
 *
 *     capability("quantum.measurement")
 *
 *     capability(quantum::measurement)
 *
 *     capability(future::architecture::feature)
 *
 * Capability identity is semantic and open-world.
 *
 * This grammar does not enumerate capability names.
 */
resourceCapabilityConstraint
    : CAPABILITY
      LPAREN
      resourceCapabilityArgument
      RPAREN
    | CAPABILITY
      capabilityReference
    ;


/*
 * ============================================================================
 * CAPABILITY ARGUMENT
 * ============================================================================
 *
 * A capability may use:
 *
 *     a string identity
 *     a qualified capability name
 *
 * String capability identities are intentionally permitted because they allow
 * capabilities supplied by future targets, dialects and external systems
 * without adding lexer keywords.
 */
resourceCapabilityArgument
    : STRING
    | qualifiedName
    ;


/*
 * ============================================================================
 * RESOURCE PREDICATE
 * ============================================================================
 *
 * A named resource predicate is an open-world semantic call.
 *
 * Examples:
 *
 *     topology(required_topology)
 *
 *     compatible_with(target_property)
 *
 *     supports(feature)
 *
 *     satisfies(policy)
 *
 * The semantic layer determines whether the predicate exists and what it
 * means.
 *
 * The grammar does not maintain a finite predicate catalogue.
 */
resourcePredicateConstraint
    : resourcePredicateCall
    ;


resourcePredicateCall
    : resourcePredicateName
      LPAREN
      optionalResourceExpressionList
      RPAREN
    ;


resourcePredicateName
    : qualifiedName
    ;


/*
 * ============================================================================
 * RESOURCE PROPERTY CONSTRAINT
 * ============================================================================
 *
 * Stable semantic alias for integrations that need to identify a comparison
 * as a resource-property constraint.
 *
 * The actual syntax remains the canonical resource comparison.
 *
 * Keeping this alias avoids forcing downstream consumers to duplicate syntax.
 */
resourcePropertyConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * RESOURCE RELATIONSHIP CONSTRAINT
 * ============================================================================
 *
 * Stable semantic alias for relationships between resource expressions.
 *
 * Examples:
 *
 *     memory.capacity >= workload.memory
 *
 *     network.bandwidth >= workload.required_bandwidth
 *
 *     quantum.logical_qubits >= algorithm.logical_qubits
 *
 *     accelerator.memory >= tensor.required_memory
 *
 * The grammar does not determine whether either side denotes a resource,
 * workload, model, target, or derived quantity. Semantic analysis does.
 */
resourceRelationshipConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * QUANTITY CONSTRAINT
 * ============================================================================
 *
 * Stable resource-domain alias for quantity-oriented comparisons.
 *
 * The quantity itself remains an arbitrary resource expression.
 *
 * Examples:
 *
 *     quantum.logical_qubits >= required_qubits
 *
 *     nodes >= required_nodes
 *
 *     threads >= required_parallelism
 *
 *     tensor.work_items >= workload_size
 */
resourceQuantityConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * CAPACITY CONSTRAINT
 * ============================================================================
 *
 * Examples:
 *
 *     memory.capacity >= required_memory
 *
 *     storage.capacity >= required_storage
 *
 *     accelerator.memory.capacity >= model_memory
 */
resourceCapacityConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * AVAILABILITY CONSTRAINT
 * ============================================================================
 *
 * Availability is a semantic property.
 *
 * The parser does not query the current machine or runtime.
 */
resourceAvailabilityConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * PERFORMANCE CONSTRAINT
 * ============================================================================
 */
resourcePerformanceConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * LATENCY CONSTRAINT
 * ============================================================================
 */
resourceLatencyConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * THROUGHPUT CONSTRAINT
 * ============================================================================
 */
resourceThroughputConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * BANDWIDTH CONSTRAINT
 * ============================================================================
 */
resourceBandwidthConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * ENERGY CONSTRAINT
 * ============================================================================
 */
resourceEnergyConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * POWER CONSTRAINT
 * ============================================================================
 */
resourcePowerConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * RELIABILITY CONSTRAINT
 * ============================================================================
 */
resourceReliabilityConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * RESILIENCE CONSTRAINT
 * ============================================================================
 *
 * Resilience is a semantic property.
 *
 * This grammar does not define:
 *
 *     recovery algorithms
 *     fault models
 *     QEC
 *     mitigation
 *     scheduling
 *     retry policy
 *
 * Those belong to their respective semantic/execution systems.
 */
resourceResilienceConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * COST CONSTRAINT
 * ============================================================================
 *
 * Cost is deliberately abstract.
 *
 * No currency, cloud provider, billing provider or commercial system is
 * embedded in the language grammar.
 */
resourceCostConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * SCALABILITY CONSTRAINT
 * ============================================================================
 *
 * A scalability constraint is a normal resource comparison.
 *
 * It MUST NOT establish a language maximum.
 *
 * Examples:
 *
 *     scalability.factor >= required_scale
 *
 *     compute.parallelism >= workload.parallelism
 */
resourceScalabilityConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * PORTABILITY CONSTRAINT
 * ============================================================================
 *
 * Portability is a semantic property rather than a target enumeration.
 */
resourcePortabilityConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * COMPATIBILITY CONSTRAINT
 * ============================================================================
 */
resourceCompatibilityConstraint
    : resourceComparisonConstraint
    ;


/*
 * ============================================================================
 * RESOURCE CONSTRAINT LIST
 * ============================================================================
 *
 * Arbitrary cardinality.
 *
 * No language-level maximum is defined.
 *
 * Practical compiler memory/time limits are implementation properties, not
 * language semantics.
 */
resourceConstraintList
    : resourceConstraintAtom*
    ;


optionalResourceConstraintList
    : resourceConstraintList?
    ;


/*
 * ============================================================================
 * RESOURCE CONSTRAINT GROUP
 * ============================================================================
 *
 * This is only a resource-domain grouping boundary.
 *
 * Boolean grouping belongs to grammar/core/constraints.g4.
 */
resourceConstraintGroup
    : LPAREN
      resourceConstraintAtom
      RPAREN
    ;


/*
 * ============================================================================
 * RESOURCE CONSTRAINT PREDICATE ALIAS
 * ============================================================================
 *
 * Stable public integration rule.
 */
resourceConstraintPredicate
    : resourceConstraintAtom
    ;


/*
 * ============================================================================
 * RESOURCE CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * This is the rule resources.g4 should consume after importing this grammar.
 *
 * It deliberately does not contain AND/OR/NOT.
 *
 * Generic Boolean composition remains in grammar/core/constraints.g4.
 */
resourceConstraintExpression
    : resourceConstraintAtom
    ;


/*
 * ============================================================================
 * RESOURCE EXPRESSION INTEGRATION
 * ============================================================================
 *
 * ResourceExpressions remains the sole authority for:
 *
 *     arithmetic
 *     logical expressions
 *     function calls
 *     member access
 *     indexing
 *     symbolic values
 *     nested expressions
 *     resource selectors
 *     resource property paths
 *     numeric values
 *     unit-bearing values
 *
 * This grammar never recreates those productions.
 */
optionalResourceExpressionList
    : resourceExpressionList?
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The following meanings are intentionally downstream.
 *
 * resourceComparisonConstraint
 *     -> semantic resource predicate
 *
 * resourceCapabilityConstraint
 *     -> capability requirement/constraint
 *
 * resourcePredicateConstraint
 *     -> named semantic predicate
 *
 * resourcePropertyConstraint
 *     -> resource property relation
 *
 * resourceRelationshipConstraint
 *     -> resource-to-resource/workload relation
 *
 * The AST/semantic layer may normalize these into a common resource
 * constraint representation without changing source syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not create AST nodes itself.
 *
 * Parser contexts must preserve:
 *
 *     - source span;
 *     - operator;
 *     - left expression;
 *     - right expression;
 *     - capability identity;
 *     - predicate name;
 *     - predicate arguments;
 *     - nested resource expressions.
 *
 * The AST must remain domain-neutral.
 *
 * It must not contain:
 *
 *     physical CPU IDs;
 *     physical GPU IDs;
 *     physical qubit IDs;
 *     vendor topology;
 *     calibration;
 *     routing;
 *     scheduling;
 *     physical QEC layout.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving resource names;
 *     - resolving capability identities;
 *     - checking units;
 *     - checking dimensional compatibility;
 *     - checking value types;
 *     - checking predicate signatures;
 *     - determining satisfiability;
 *     - determining whether a constraint is conditional;
 *     - determining target feasibility;
 *     - resolving policies;
 *     - producing diagnostics.
 *
 * Parsing MUST succeed even when a requirement is currently unsatisfied or
 * when the referenced resource/capability is supplied only by a future target
 * or dialect.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource resolution occurs after parsing.
 *
 * Possible semantic outcomes include:
 *
 *     satisfied
 *     unsatisfied
 *     unknown
 *     conditional
 *     deferred
 *
 * None of these states are encoded as parser-level hardware decisions.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability resolution occurs after parsing.
 *
 * A capability may originate from:
 *
 *     language semantics
 *     compiler
 *     runtime
 *     target
 *     dialect
 *     accelerator
 *     QPU
 *     simulator
 *     distributed platform
 *     external implementation.
 *
 * The grammar remains open-world.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Resource constraints themselves do not introduce execution effects.
 *
 * Downstream operations may have effects such as:
 *
 *     IO
 *     network
 *     native
 *     foreign
 *     quantum measurement
 *     mutation
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *
 * Effect analysis remains owned by grammar/effects and semantic analysis.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may:
 *
 *     permit
 *     prohibit
 *     restrict
 *     prioritize
 *     condition
 *     override realization choices
 *
 * Policy syntax is not duplicated here.
 *
 * Resource constraints are inputs to policy evaluation.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Resource constraints must preserve source provenance through the frontend.
 *
 * Downstream provenance may record:
 *
 *     source span
 *     normalized constraint
 *     resolved resource
 *     resolved capability
 *     evidence
 *     target evaluation
 *     policy decision
 *     fallback decision
 *     execution realization
 *
 * This grammar itself remains deterministic and side-effect free.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum source may use resource constraints such as:
 *
 *     quantum.logical_qubits >= logical_qubits
 *
 *     capability("quantum.measurement")
 *
 *     capability(quantum::dynamic_control)
 *
 * These constraints feed:
 *
 *     semantic resource model
 *          |
 *          v
 *     quantum analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * This grammar never constructs quantum::ir directly.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical resource constraints can influence:
 *
 *     optimization
 *     parallelization
 *     vectorization
 *     memory planning
 *     accelerator selection
 *     execution planning
 *
 * They do not select a particular processor.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware semantic layers may consume the same resource constraint
 * representation for:
 *
 *     timing
 *     capacity
 *     power
 *     energy
 *     bandwidth
 *     latency
 *     reconfiguration
 *     reliability
 *     thermal properties
 *     implementation constraints
 *
 * Hardware-specific meaning remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * The same grammar supports:
 *
 *     nodes >= required_nodes
 *
 *     network.bandwidth >= required_bandwidth
 *
 *     network.latency <= latency_budget
 *
 *     capability("distributed.collectives")
 *
 * No node-count ceiling is encoded.
 *
 * ============================================================================
 * AI / DATA / TENSOR INTEGRATION
 * ============================================================================
 *
 * Resource constraints can express:
 *
 *     memory >= model_memory
 *
 *     tensor.work_items >= workload_size
 *
 *     capability("tensor.compute")
 *
 *     capability("distributed.training")
 *
 * The grammar does not enumerate machine-learning algorithms or hardware
 * products.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Classical, quantum, accelerator and other resources may occur in the same
 * source program.
 *
 * No separate resource-expression language is permitted for each domain.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Resource constraints may be evaluated against a simulator when the selected
 * execution policy permits simulation.
 *
 * Parsing is independent of whether the realization is:
 *
 *     hardware
 *     simulator
 *     emulator
 *     accelerator
 *     distributed execution
 *     future execution system
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Resource constraints may participate in:
 *
 *     fallback
 *     retry
 *     recovery
 *     adaptive target selection
 *     execution planning
 *
 * The grammar does not implement those behaviors.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar is intentionally unbounded at the language level.
 *
 * It permits:
 *
 *     arbitrary expression size;
 *     arbitrary qualified-name depth;
 *     arbitrary resource-property depth;
 *     arbitrary symbolic values;
 *     arbitrary resource domains;
 *     arbitrary capability namespaces;
 *     arbitrary predicate arguments;
 *     arbitrary numbers of constraints.
 *
 * Actual implementation resource limits remain implementation/runtime
 * properties and MUST NOT be promoted into grammar constants.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 * It contains no fixed:
 *
 *     processor count
 *     GPU count
 *     FPGA count
 *     node count
 *     qubit count
 *     tensor rank
 *     network size
 *     resource-list cardinality.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/resources/resources.g4
 * --------------------------------
 *
 * MUST import this grammar:
 *
 *     import ResourceExpressions, Names, constraints;
 *
 * Its concrete statement remains:
 *
 *     resourceConstraint
 *         : CONSTRAINT
 *           resourceConstraintExpression
 *           SEMICOLON
 *         ;
 *
 * and:
 *
 *     resourceConstraintExpression
 *         : resourceConstraintAtom
 *         ;
 *
 * This keeps the statement wrapper in resources.g4 while this file owns the
 * resource-domain atom.
 *
 * IMPORTANT:
 *
 * Do not make this file import resources.g4.
 *
 * That would create a dependency cycle.
 *
 * ============================================================================
 * grammar/core/constraints.g4
 * --------------------------------
 *
 * Remains the authority for:
 *
 *     AND
 *     OR
 *     NOT
 *     parentheses
 *     generic predicates
 *     generic type constraints
 *     generic constraint clauses.
 *
 * Resource-specific atoms can be admitted by the semantic/parser composition
 * layer without duplicating Boolean constraint syntax here.
 *
 * ============================================================================
 * grammar/resources/resource-expressions.g4
 * --------------------------------
 *
 * Remains the sole authority for resource expressions.
 *
 * This file consumes:
 *
 *     resourceExpression
 *     resourceExpressionList
 *
 * and never recreates expression precedence.
 *
 * ============================================================================
 * grammar/core/capabilities.g4
 * --------------------------------
 *
 * Remains the capability authority.
 *
 * This file consumes:
 *
 *     capabilityReference
 *     capabilityName
 *
 * but permits string capability identities at the resource boundary for
 * dynamically registered capabilities.
 *
 * ============================================================================
 * AST / SEMANTIC / IR INTEGRATION
 * ============================================================================
 *
 * AST:
 *     domain-neutral resource constraint representation.
 *
 * Semantic model:
 *     resource/capability predicate.
 *
 * Classical IR:
 *     consumed as compilation/execution intent where applicable.
 *
 * quantum::ir:
 *     consumed indirectly through quantum resource analysis.
 *
 * HDL:
 *     consumed indirectly through hardware/resource analysis.
 *
 * Backend:
 *     resolved only after semantic/resource/capability analysis.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     constraint memory.capacity >= required_memory;
 *
 *     constraint quantum.logical_qubits >= logical_qubits;
 *
 *     constraint network.bandwidth >= required_bandwidth;
 *
 *     constraint execution.latency <= latency_budget;
 *
 *     constraint capability("quantum.measurement");
 *
 *     constraint capability(quantum::dynamic_control);
 *
 *     constraint capability("tensor.compute");
 *
 *     constraint capability(future::architecture::feature);
 *
 *     constraint memory.capacity >= dataset.size * element_size;
 *
 *     constraint quantum.logical_qubits >= algorithm.qubits + ancilla;
 *
 * NEGATIVE
 * --------
 *
 *     constraint;
 *
 *     constraint >= memory;
 *
 *     constraint memory >=;
 *
 *     constraint capability();
 *
 *     constraint capability(;
 *
 *     constraint capability("unterminated);
 *
 *     constraint memory >= required_memory
 *
 * where the surrounding statement syntax requires the terminating semicolon.
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 * These must remain parser-valid but may be semantic errors:
 *
 *     constraint memory.capacity >= qubit_count;
 *
 *     constraint unknown.resource.property >= value;
 *
 *     constraint capability("unavailable.capability");
 *
 *     constraint latency >= memory;
 *
 * The parser must not confuse semantic infeasibility with syntax failure.
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     zero values;
 *     negative values where expression syntax permits them;
 *     large integer literals;
 *     large floating-point literals;
 *     symbolic quantities;
 *     computed quantities;
 *     deeply qualified resource names;
 *     deeply nested expressions;
 *     many constraints;
 *     many capability namespaces;
 *     mixed classical/quantum/distributed/hardware constraints.
 *
 * SCALABILITY
 * ----------
 *
 * Test progressively larger source programs without translating test size
 * into a grammar maximum.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For identical token streams and grammar configuration:
 *
 *     parser structure must be deterministic.
 *
 * Parsing must not depend on:
 *
 *     hardware availability;
 *     runtime state;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     scheduler state.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It is the sole resource-domain constraint-atom authority.
 *
 *     [ ] It imports only canonical dependency grammars.
 *
 *     [ ] It does not define generic Boolean constraints.
 *
 *     [ ] It does not define general expressions.
 *
 *     [ ] It does not define names.
 *
 *     [ ] It does not redefine capability-name semantics.
 *
 *     [ ] It does not define concrete resource statements.
 *
 *     [ ] It is open-world for resource names.
 *
 *     [ ] It is open-world for capabilities.
 *
 *     [ ] It has no hardware-capacity constants.
 *
 *     [ ] It has no finite device catalogue.
 *
 *     [ ] It has no physical placement semantics.
 *
 *     [ ] It has no runtime resource discovery.
 *
 *     [ ] It preserves symbolic resource expressions.
 *
 *     [ ] It supports quantum/classical/HDL/hardware/distributed/AI/data
 *         consumers through the same resource boundary.
 *
 *     [ ] It preserves the quantum::ir boundary.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Rust 1.97 generation/integration passes.
 *
 *     [ ] Rust 1.97.1 generation/integration passes.
 *
 *     [ ] No unsafe Rust is introduced.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * SOURCE
 *     describes intent.
 *
 * RESOURCE CONSTRAINT
 *     describes a condition on realization.
 *
 * SEMANTIC ANALYSIS
 *     determines meaning and feasibility.
 *
 * CAPABILITY NEGOTIATION
 *     determines whether a realization can provide what is required.
 *
 * EXECUTION PLANNING
 *     determines how to realize the program.
 *
 * TARGET LOWERING
 *     specializes only after portable meaning is established.
 *
 * Therefore this grammar remains compatible with:
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
 *     HPC
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational systems
 *
 * without changing the universal resource-constraint grammar merely because
 * the underlying machine becomes larger, smaller, newer, or different.
 *
 * ============================================================================
 */