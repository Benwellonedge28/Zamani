/*
 * Zamani Programming Language
 * File: grammar/resources/constraints.g4
 *
 * Purpose
 * -------
 * Resource-domain constraint syntax for Zamani.
 *
 * Architectural ownership
 * -----------------------
 * This grammar owns the syntax needed to express constraints about:
 *
 *   - resources
 *   - resource classes
 *   - resource capabilities
 *   - resource quantities
 *   - performance
 *   - latency
 *   - energy
 *   - reliability
 *   - scalability
 *   - portability
 *   - availability
 *   - capacity
 *   - compatibility
 *
 * It does NOT own:
 *
 *   - generic boolean constraint composition
 *   - generic expressions
 *   - resource discovery
 *   - hardware discovery
 *   - target selection
 *   - physical placement
 *   - scheduling
 *   - routing
 *   - optimization
 *   - quantum IR
 *   - QEC semantics
 *   - ZQN fault/noise semantics
 *   - runtime policy
 *
 * Those concerns remain owned by their respective repository subsystems.
 *
 * IMPORTANT
 * ---------
 * This grammar deliberately contains NO machine-size constants.
 *
 * There is no:
 *
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_MEMORY
 *   MAX_NODES
 *   MAX_THREADS
 *   MAX_DEVICES
 *   fixed topology
 *   fixed device identifier
 *   fixed hardware address
 *
 * Quantities are expressions and are resolved against the compilation,
 * capability, resource, scheduling, or runtime context downstream.
 *
 * POCO-REAF
 * ---------
 * Resource constraints describe semantic requirements and preferences,
 * not accidental characteristics of today's machines.
 *
 * A program may therefore express:
 *
 *   require quantum
 *   require scalable
 *   require capability("...") 
 *   require memory >= expression
 *   prefer low_latency
 *
 * without embedding a particular physical machine into the source.
 *
 * Integration boundary
 * --------------------
 *
 * Source
 *   |
 *   v
 * ANTLR lexer/parser
 *   |
 *   v
 * Zamani AST / semantic syntax representation
 *   |
 *   +--> resource requirement analysis
 *   +--> capability analysis
 *   +--> target selection
 *   +--> resource manager
 *   +--> compilation context
 *   +--> scheduling context
 *   +--> runtime capability negotiation
 *
 * This file must never create a second IR.
 *
 * Quantum constraints eventually lower through the existing canonical
 * quantum::ir boundary where applicable.
 *
 * Rust compatibility
 * ------------------
 * The generated parser and surrounding implementation must remain
 * compatible with Rust 1.97 / 1.97.1 and must require no unsafe Rust.
 */

grammar ZamaniResourceConstraints;

/*
 * --------------------------------------------------------------------------
 * Imports / shared grammar contracts
 * --------------------------------------------------------------------------
 *
 * This grammar is intentionally designed as a composable grammar fragment.
 *
 * The canonical Zamani grammar integration layer is responsible for wiring
 * the following concepts to their authoritative definitions:
 *
 *   expression
 *   qualifiedName
 *   identifier
 *   literal
 *   resourceReference
 *   capabilityReference
 *   requirement
 *   constraint
 *
 * Do not duplicate those definitions here.
 *
 * The rules below therefore use semantic fragment names that must be mapped
 * by the grammar integration layer to the canonical Zamani parser rules.
 *
 * If ANTLR grammar composition requires imports, the authoritative root
 * grammar must import this grammar fragment rather than this fragment
 * depending on domain-specific grammars.
 */

/*
 * --------------------------------------------------------------------------
 * Resource constraint root
 * --------------------------------------------------------------------------
 *
 * A resource constraint is a domain-specific constraint expression.
 *
 * Generic AND / OR / NOT composition remains owned by
 * grammar/core/constraints.g4.
 */

resourceConstraint
    : resourceConstraintAtom
    ;

/*
 * --------------------------------------------------------------------------
 * Atomic resource constraints
 * --------------------------------------------------------------------------
 */

resourceConstraintAtom
    : resourceKindConstraint
    | resourceExistenceConstraint
    | resourceQuantityConstraint
    | resourceCapacityConstraint
    | resourcePerformanceConstraint
    | resourceLatencyConstraint
    | resourceEnergyConstraint
    | resourceReliabilityConstraint
    | resourceScalabilityConstraint
    | resourcePortabilityConstraint
    | resourceAvailabilityConstraint
    | resourceCompatibilityConstraint
    | resourceCapabilityConstraint
    | resourcePropertyConstraint
    ;

/*
 * --------------------------------------------------------------------------
 * Resource kind
 * --------------------------------------------------------------------------
 *
 * Resource kind identifies a semantic resource category.
 *
 * It does NOT identify a particular physical device.
 *
 * Examples:
 *
 *   quantum
 *   cpu
 *   gpu
 *   fpga
 *   accelerator
 *   memory
 *   storage
 *   network
 *   compute
 *
 * The actual open-world vocabulary should be extensible rather than
 * permanently enumerated in this grammar.
 */

resourceKindConstraint
    : resourceConstraintSubject resourceKindOperator resourceKindReference
    ;

resourceKindOperator
    : IS
    | IS_NOT
    | SUPPORTS
    ;

resourceKindReference
    : qualifiedName
    | STRING_LITERAL
    ;

/*
 * --------------------------------------------------------------------------
 * Resource existence
 * --------------------------------------------------------------------------
 */

resourceExistenceConstraint
    : resourceConstraintSubject resourceExistenceOperator
    ;

resourceExistenceOperator
    : EXISTS
    | AVAILABLE
    | UNAVAILABLE
    ;

/*
 * --------------------------------------------------------------------------
 * Resource quantities
 * --------------------------------------------------------------------------
 *
 * Quantities are expressions.
 *
 * This is essential for scalability.
 *
 * Examples:
 *
 *   resources.compute >= requested
 *   resources.quantum >= required_qubits
 *   resources.memory >= workload_memory
 *
 * The grammar never establishes an upper bound.
 */

resourceQuantityConstraint
    : resourceConstraintSubject resourceQuantityOperator expression
    ;

resourceQuantityOperator
    : EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_THAN_OR_EQUAL
    | GREATER_THAN
    | GREATER_THAN_OR_EQUAL
    ;

/*
 * --------------------------------------------------------------------------
 * Capacity
 * --------------------------------------------------------------------------
 */

resourceCapacityConstraint
    : resourceConstraintSubject CAPACITY resourceQuantityOperator expression
    ;

/*
 * --------------------------------------------------------------------------
 * Performance
 * --------------------------------------------------------------------------
 *
 * Performance is expressed semantically.
 *
 * No hardware-specific performance values are embedded in the grammar.
 */

resourcePerformanceConstraint
    : resourceConstraintSubject PERFORMANCE resourceMetricComparison
    ;

/*
 * --------------------------------------------------------------------------
 * Latency
 * --------------------------------------------------------------------------
 */

resourceLatencyConstraint
    : resourceConstraintSubject LATENCY resourceMetricComparison
    ;

/*
 * --------------------------------------------------------------------------
 * Energy
 * --------------------------------------------------------------------------
 */

resourceEnergyConstraint
    : resourceConstraintSubject ENERGY resourceMetricComparison
    ;

/*
 * --------------------------------------------------------------------------
 * Reliability
 * --------------------------------------------------------------------------
 */

resourceReliabilityConstraint
    : resourceConstraintSubject RELIABILITY resourceMetricComparison
    ;

/*
 * --------------------------------------------------------------------------
 * Scalability
 * --------------------------------------------------------------------------
 *
 * Scalability is a semantic property.
 *
 * It must not be interpreted as a compile-time maximum.
 */

resourceScalabilityConstraint
    : resourceConstraintSubject SCALABILITY resourceScalabilityPredicate
    ;

resourceScalabilityPredicate
    : IS SCALABLE
    | IS_NOT SCALABLE
    | SUPPORTS SCALING
    | SUPPORTS SCALE_TO expression
    ;

/*
 * --------------------------------------------------------------------------
 * Portability
 * --------------------------------------------------------------------------
 */

resourcePortabilityConstraint
    : resourceConstraintSubject PORTABILITY resourcePortabilityPredicate
    ;

resourcePortabilityPredicate
    : IS PORTABLE
    | IS_NOT PORTABLE
    | SUPPORTS PORTABILITY
    ;

/*
 * --------------------------------------------------------------------------
 * Availability
 * --------------------------------------------------------------------------
 */

resourceAvailabilityConstraint
    : resourceConstraintSubject AVAILABILITY resourceMetricComparison
    ;

/*
 * --------------------------------------------------------------------------
 * Compatibility
 * --------------------------------------------------------------------------
 *
 * Compatibility is expressed in terms of semantic capabilities or named
 * requirements, not physical device identities.
 */

resourceCompatibilityConstraint
    : resourceConstraintSubject COMPATIBLE_WITH compatibilityReference
    | resourceConstraintSubject INCOMPATIBLE_WITH compatibilityReference
    ;

compatibilityReference
    : qualifiedName
    | STRING_LITERAL
    ;

/*
 * --------------------------------------------------------------------------
 * Capability constraints
 * --------------------------------------------------------------------------
 *
 * Capabilities are intentionally open-world.
 *
 * This permits future hardware and execution capabilities without requiring
 * a grammar release merely because a new capability name is introduced.
 */

resourceCapabilityConstraint
    : resourceConstraintSubject CAPABILITY capabilityReference
    ;

capabilityReference
    : qualifiedName
    | STRING_LITERAL
    ;

/*
 * --------------------------------------------------------------------------
 * Generic resource property constraints
 * --------------------------------------------------------------------------
 *
 * The property name is intentionally symbolic.
 *
 * This is the escape hatch for future resource properties without turning
 * this grammar into a closed enumeration of every possible resource metric.
 */

resourcePropertyConstraint
    : resourceConstraintSubject DOT propertyName resourceMetricComparison
    ;

propertyName
    : identifier
    ;

/*
 * --------------------------------------------------------------------------
 * Metric comparison
 * --------------------------------------------------------------------------
 *
 * A metric comparison always separates:
 *
 *   metric
 *   operator
 *   value
 *
 * The value is an expression and can therefore depend on:
 *
 *   constants
 *   generic parameters
 *   compile-time expressions
 *   workload properties
 *   resource variables
 *   contextual values
 *
 * Resolution happens outside the grammar.
 */

resourceMetricComparison
    : metricReference resourceQuantityOperator expression
    ;

/*
 * --------------------------------------------------------------------------
 * Metric references
 * --------------------------------------------------------------------------
 */

metricReference
    : qualifiedName
    | identifier
    ;

/*
 * --------------------------------------------------------------------------
 * Resource subjects
 * --------------------------------------------------------------------------
 *
 * The subject is a semantic reference to a resource class, resource set,
 * capability domain, or named resource abstraction.
 *
 * It is NOT a physical address.
 */

resourceConstraintSubject
    : resourceReference
    | resourceSetReference
    | resourceDomainReference
    ;

resourceReference
    : RESOURCE qualifiedName
    ;

resourceSetReference
    : RESOURCES qualifiedName
    ;

resourceDomainReference
    : RESOURCE_DOMAIN qualifiedName
    ;

/*
 * --------------------------------------------------------------------------
 * Capability references
 * --------------------------------------------------------------------------
 */

resourceCapabilityReference
    : CAPABILITY qualifiedName
    ;

/*
 * --------------------------------------------------------------------------
 * Open-world resource references
 * --------------------------------------------------------------------------
 *
 * Qualified names allow future domains without modifying this grammar.
 *
 * Examples of possible semantic namespaces:
 *
 *   compute.cpu
 *   compute.gpu
 *   compute.fpga
 *   compute.quantum
 *   memory.local
 *   memory.distributed
 *   network.fabric
 *   accelerator.tensor
 *
 * The grammar does not decide whether a name is currently realizable.
 * Semantic/capability analysis does that.
 */

/*
 * --------------------------------------------------------------------------
 * Reserved semantic keywords
 * --------------------------------------------------------------------------
 *
 * These tokens are expected to be provided by the canonical Zamani lexer.
 *
 * They are listed here only as integration requirements; they must not be
 * duplicated as independently competing lexer definitions.
 *
 * Required concepts:
 *
 *   RESOURCE
 *   RESOURCES
 *   RESOURCE_DOMAIN
 *   CAPABILITY
 *   CAPACITY
 *   PERFORMANCE
 *   LATENCY
 *   ENERGY
 *   RELIABILITY
 *   SCALABILITY
 *   PORTABILITY
 *   AVAILABILITY
 *   COMPATIBLE_WITH
 *   INCOMPATIBLE_WITH
 *   EXISTS
 *   AVAILABLE
 *   UNAVAILABLE
 *   SUPPORTS
 *   SCALABLE
 *   SCALING
 *   SCALE_TO
 *   PORTABLE
 *   IS
 *   IS_NOT
 *   EQUAL
 *   NOT_EQUAL
 *   LESS_THAN
 *   LESS_THAN_OR_EQUAL
 *   GREATER_THAN
 *   GREATER_THAN_OR_EQUAL
 *
 * Operators and identifiers must remain owned by the canonical lexer.
 */

/*
 * --------------------------------------------------------------------------
 * Integration contract
 * --------------------------------------------------------------------------
 *
 * Canonical parser integration:
 *
 *   Zamani.g4
 *       |
 *       +--> resources.g4
 *                |
 *                +--> constraints.g4
 *
 * resources/constraints.g4 must not become the root grammar.
 *
 * Semantic integration:
 *
 *   ParsedResourceConstraint
 *            |
 *            v
 *   semantic constraint representation
 *            |
 *            +--> ResourceManager
 *            +--> CapabilityResolver
 *            +--> TargetResolver
 *            +--> CompilationContext
 *            +--> Scheduling
 *            +--> Execution
 *            +--> Runtime capability negotiation
 *
 * Quantum integration:
 *
 * Resource constraints involving quantum resources are interpreted by the
 * resource/capability layer and eventually lowered into the existing quantum
 * compilation pipeline.
 *
 * This grammar does NOT construct quantum::ir.
 *
 * Hardware integration:
 *
 * Hardware capability/resource realization belongs to the Hardware HAL and
 * target/resource layers.
 *
 * This grammar does NOT discover hardware.
 *
 * Scheduling integration:
 *
 * Timing/performance/latency constraints may be consumed by scheduling, but
 * scheduling owns the actual schedule.
 *
 * Optimization integration:
 *
 * Optimization may use resource constraints as legality or preference
 * information, but optimization owns transformations.
 *
 * Runtime integration:
 *
 * Runtime may evaluate unresolved contextual constraints, subject to the
 * language's explicit execution/validation policy.
 *
 * ZQN integration:
 *
 * ZQN owns noise/fault semantics.
 *
 * A resource constraint may refer to reliability or capability properties,
 * but must not redefine ZQN's fault/noise model.
 *
 * QEC integration:
 *
 * QEC owns error detection/correction mechanisms.
 *
 * This grammar may express a requirement for an error-correction-related
 * capability if such a capability exists, but it does not define QEC
 * algorithms.
 */

/*
 * --------------------------------------------------------------------------
 * Safety / security boundary
 * --------------------------------------------------------------------------
 *
 * Resource constraints are declarative.
 *
 * They must not imply:
 *
 *   filesystem access
 *   network access
 *   process execution
 *   device access
 *   hardware discovery
 *   arbitrary code execution
 *
 * Evaluation belongs to explicitly authorized compiler/runtime services.
 */

/*
 * --------------------------------------------------------------------------
 * Determinism requirements
 * --------------------------------------------------------------------------
 *
 * Parsing this grammar must be deterministic for identical source and
 * grammar versions.
 *
 * Resource discovery, scheduling, placement, or runtime negotiation must
 * never alter parsing.
 */

/*
 * --------------------------------------------------------------------------
 * Versioning requirements
 * --------------------------------------------------------------------------
 *
 * New resource kinds, capability names, and property names should normally
 * be introduced as semantic vocabulary rather than grammar productions.
 *
 * Grammar changes are reserved for changes to syntactic structure.
 *
 * This keeps POCO-REAF source compatibility strong across future hardware
 * generations.
 */