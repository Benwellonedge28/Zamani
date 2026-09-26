/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/constraints.g4
 *
 * GRAMMAR
 * -------
 * ZamaniResourceConstraints
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-DOMAIN CONSTRAINT GRAMMAR
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * Grammar-only.
 *
 * No embedded Rust actions.
 * No semantic predicates.
 * No filesystem access.
 * No network access.
 * No hardware discovery.
 * No runtime execution.
 * No unsafe Rust requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-SYNTAX ATOMS for constraints whose subject
 * belongs to the Zamani resource domain.
 *
 * It describes restrictions and predicates over:
 *
 *   - resource quantities;
 *   - resource capacities;
 *   - availability;
 *   - performance;
 *   - latency;
 *   - throughput;
 *   - bandwidth;
 *   - energy;
 *   - power;
 *   - reliability;
 *   - resilience;
 *   - scalability;
 *   - portability;
 *   - cost;
 *   - capabilities;
 *   - resource properties;
 *   - resource relationships;
 *   - resource compatibility;
 *   - target-independent resource characteristics.
 *
 * This grammar does NOT choose a physical resource.
 *
 * This grammar does NOT discover hardware.
 *
 * This grammar does NOT allocate resources.
 *
 * This grammar does NOT perform placement.
 *
 * This grammar does NOT perform routing.
 *
 * This grammar does NOT perform scheduling.
 *
 * This grammar does NOT perform optimization.
 *
 * This grammar does NOT perform QEC.
 *
 * This grammar does NOT implement ZQN.
 *
 * This grammar does NOT construct quantum::ir.
 *
 * ============================================================================
 * 2. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Resource constraint ownership is split deliberately:
 *
 *     grammar/core/constraints.g4
 *         |
 *         | owns generic constraint composition
 *         | boolean grouping
 *         | generic predicates
 *         | generic relational structure
 *         |
 *         v
 *     grammar/resources/constraints.g4
 *         |
 *         | owns resource-domain constraint atoms
 *         |
 *         v
 *     grammar/resources/resources.g4
 *         |
 *         | owns the concrete:
 *         |
 *         |     constraint <expression>;
 *         |
 *         v
 *     ZamaniParser.g4
 *         |
 *         v
 *     grammar/Zamani.g4
 *
 * IMPORTANT:
 *
 * This file MUST NOT define the concrete `resourceConstraint` statement
 * because `grammar/resources/resources.g4` already owns that statement.
 *
 * This file therefore owns:
 *
 *     resourceConstraintAtom
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
 *     resourceCapabilityConstraint
 *     resourcePropertyConstraint
 *     resourceRelationshipConstraint
 *
 * The concrete statement wrapper remains:
 *
 *     resourceConstraint
 *
 * in:
 *
 *     grammar/resources/resources.g4
 *
 * ============================================================================
 * 3. POCO-REAF
 * ============================================================================
 *
 * Resource constraints are part of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A constraint therefore describes PORTABLE PROGRAM INTENT.
 *
 * It must not encode accidental properties of today's hardware.
 *
 * The grammar contains NO universal limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     nodes
 *     processes
 *     devices
 *     memory
 *     storage
 *     registers
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     timelines
 *     channels
 *     resource groups
 *
 * A numeric literal in source is a PROGRAM VALUE.
 *
 * For example:
 *
 *     constraint quantum::logical_qubits >= 1024;
 *
 * means that 1024 is part of this program's semantic requirement.
 *
 * It does NOT establish:
 *
 *     MAX_QUBITS = 1024
 *
 * for the language.
 *
 * ============================================================================
 * 4. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce or imply:
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
 * or equivalent hidden limits.
 *
 * It also MUST NOT enumerate physical device identities.
 *
 * These are NOT universal grammar constructs:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     device0
 *     node0
 *
 * They remain ordinary semantic names unless a downstream, explicitly
 * target-specific language feature gives them meaning.
 *
 * ============================================================================
 * 5. OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Resource kinds, capabilities, properties, metrics, and future resource
 * technologies are OPEN-WORLD semantic vocabulary.
 *
 * The grammar therefore does not contain a closed list such as:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     TPU
 *
 * as the only possible resource kinds.
 *
 * Names are represented by the canonical resource-expression/name system.
 *
 * Future resources can therefore be introduced through semantic registration
 * and capability/resource models without requiring a parser redesign merely
 * because a new kind of computer exists.
 *
 * ============================================================================
 * 6. IMPORT CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical resource-expression layer.
 *
 * The resource-expression layer is:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * whose grammar name is:
 *
 *     ResourceExpressions
 *
 * That grammar already imports the canonical expression grammar.
 *
 * Therefore this file MUST NOT define:
 *
 *     expression
 *     arithmeticExpression
 *     logicalExpression
 *     comparisonExpression
 *     equalityExpression
 *     primaryExpression
 *     identifier
 *     qualifiedName
 *     literal
 *
 * itself.
 *
 * This prevents a second expression/comparison/name authority.
 *
 * ============================================================================
 * 7. ANTLR GRAMMAR
 * ============================================================================
 */

parser grammar ZamaniResourceConstraints;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;

/*
 * ============================================================================
 * 8. RESOURCE CONSTRAINT ATOM
 * ============================================================================
 *
 * This is the resource-domain atomic boundary.
 *
 * Boolean composition belongs to:
 *
 *     grammar/core/constraints.g4
 *
 * The rule deliberately does not contain AND/OR/NOT.
 *
 * A consumer that needs a complete constraint expression should compose this
 * atom with the canonical generic constraint machinery.
 *
 * ============================================================================
 */

resourceConstraintAtom
    : resourceQuantityConstraint
    | resourceCapacityConstraint
    | resourceAvailabilityConstraint
    | resourcePerformanceConstraint
    | resourceLatencyConstraint
    | resourceThroughputConstraint
    | resourceBandwidthConstraint
    | resourceEnergyConstraint
    | resourcePowerConstraint
    | resourceReliabilityConstraint
    | resourceResilienceConstraint
    | resourceCostConstraint
    | resourceScalabilityConstraint
    | resourcePortabilityConstraint
    | resourceCompatibilityConstraint
    | resourceCapabilityConstraint
    | resourcePropertyConstraint
    | resourceRelationshipConstraint
    ;


/*
 * ============================================================================
 * 9. GENERIC RESOURCE COMPARISON
 * ============================================================================
 *
 * This is intentionally based on the already canonical:
 *
 *     resourceComparisonOperator
 *
 * from:
 *
 *     resource-expressions.g4
 *
 * Therefore this file does NOT redefine comparison operators.
 *
 * Existing canonical operators include:
 *
 *     EQ_EQ
 *     NOT_EQ
 *     LESS
 *     GREATER
 *     LE
 *     GE
 *
 * The lexical spellings remain owned by grammar/lexer/.
 *
 * ============================================================================
 */

resourceConstraintComparison
    : resourceExpression
      resourceComparisonOperator
      resourceExpression
    ;


/*
 * ============================================================================
 * 10. QUANTITY CONSTRAINT
 * ============================================================================
 *
 * Examples:
 *
 *     quantum::logical_qubits >= required_qubits
 *     memory::capacity >= required_memory
 *     compute::parallelism >= desired_parallelism
 *
 * The values are expressions.
 *
 * Therefore the grammar imposes no quantity ceiling.
 *
 * ============================================================================
 */

resourceQuantityConstraint
    : resourceQuantitySubject
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceQuantitySubject
    : resourceExpression
    ;

resourceQuantityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 11. CAPACITY CONSTRAINT
 * ============================================================================
 *
 * Capacity is a semantic resource property.
 *
 * Example:
 *
 *     memory::capacity >= required_memory
 *
 * The actual capacity is supplied by the resource/capability/target
 * infrastructure downstream.
 *
 * ============================================================================
 */

resourceCapacityConstraint
    : resourceCapacityReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceCapacityReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 12. AVAILABILITY CONSTRAINT
 * ============================================================================
 *
 * Availability can be dynamic.
 *
 * The grammar only records the requested relationship.
 *
 * It MUST NOT query availability while parsing.
 *
 * ============================================================================
 */

resourceAvailabilityConstraint
    : resourceAvailabilityReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceAvailabilityReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 13. PERFORMANCE CONSTRAINT
 * ============================================================================
 *
 * Performance is represented through an open resource property path.
 *
 * Examples:
 *
 *     compute::performance >= required_performance
 *     accelerator::throughput >= required_throughput
 *
 * No processor benchmark is embedded in this grammar.
 *
 * ============================================================================
 */

resourcePerformanceConstraint
    : resourcePerformanceReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourcePerformanceReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 14. LATENCY CONSTRAINT
 * ============================================================================
 *
 * Examples:
 *
 *     network::latency <= latency_budget
 *     execution::latency <= workload_latency
 *
 * Units and dimensional correctness belong to semantic analysis.
 *
 * ============================================================================
 */

resourceLatencyConstraint
    : resourceLatencyReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceLatencyReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 15. THROUGHPUT CONSTRAINT
 * ============================================================================
 */

resourceThroughputConstraint
    : resourceThroughputReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceThroughputReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 16. BANDWIDTH CONSTRAINT
 * ============================================================================
 */

resourceBandwidthConstraint
    : resourceBandwidthReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceBandwidthReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 17. ENERGY CONSTRAINT
 * ============================================================================
 *
 * Energy semantics are intentionally target-independent.
 *
 * The source can constrain energy use without specifying a physical device.
 *
 * ============================================================================
 */

resourceEnergyConstraint
    : resourceEnergyReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceEnergyReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 18. POWER CONSTRAINT
 * ============================================================================
 */

resourcePowerConstraint
    : resourcePowerReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourcePowerReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 19. RELIABILITY CONSTRAINT
 * ============================================================================
 *
 * Reliability is distinct from QEC and ZQN.
 *
 * This grammar records a resource-level property constraint.
 *
 * ZQN owns noise/fault semantics.
 *
 * QEC owns error-correction mechanisms.
 *
 * ============================================================================
 */

resourceReliabilityConstraint
    : resourceReliabilityReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceReliabilityReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 20. RESILIENCE CONSTRAINT
 * ============================================================================
 *
 * Resilience is a resource/realization property.
 *
 * It may be consumed by the resilience subsystem downstream.
 *
 * This grammar does not define resilience algorithms or state transitions.
 *
 * ============================================================================
 */

resourceResilienceConstraint
    : resourceResilienceReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceResilienceReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 21. COST CONSTRAINT
 * ============================================================================
 *
 * Cost is intentionally an abstract property.
 *
 * It may represent an explicitly defined semantic cost model.
 *
 * The grammar does not assume:
 *
 *     currency
 *     pricing provider
 *     cloud vendor
 *     billing model
 *
 * ============================================================================
 */

resourceCostConstraint
    : resourceCostReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceCostReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 22. SCALABILITY CONSTRAINT
 * ============================================================================
 *
 * Scalability must describe a property or relationship.
 *
 * It must not become a compiler maximum.
 *
 * Examples:
 *
 *     scalability::scale_factor >= requested_scale
 *
 *     compute::parallelism >= workload_parallelism
 *
 * ============================================================================
 */

resourceScalabilityConstraint
    : resourceScalabilityReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourceScalabilityReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 23. PORTABILITY CONSTRAINT
 * ============================================================================
 *
 * Portability is represented as a resource property or symbolic predicate.
 *
 * Boolean semantic interpretation is downstream.
 *
 * Examples:
 *
 *     portability::level == required_portability
 *
 *     target::portability >= required_level
 *
 * ============================================================================
 */

resourcePortabilityConstraint
    : resourcePortabilityReference
      resourceComparisonOperator
      resourceQuantityValue
    ;

resourcePortabilityReference
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 24. COMPATIBILITY CONSTRAINT
 * ============================================================================
 *
 * Compatibility is represented through the canonical resource expression
 * system.
 *
 * It does not select a physical device.
 *
 * Examples:
 *
 *     resource::compatibility == required_compatibility
 *
 *     backend::compatibility != incompatible_backend
 *
 * ============================================================================
 */

resourceCompatibilityConstraint
    : resourceCompatibilityReference
      resourceComparisonOperator
      resourceCompatibilityValue
    ;

resourceCompatibilityReference
    : resourcePropertyPath
    ;

resourceCompatibilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 25. CAPABILITY CONSTRAINT
 * ============================================================================
 *
 * Capabilities are OPEN-WORLD.
 *
 * No capability enumeration belongs here.
 *
 * Canonical examples:
 *
 *     capability("quantum.measurement")
 *
 *     capability("quantum.mid_circuit_measurement")
 *
 *     capability("tensor.compute")
 *
 *     capability("gpu.compute")
 *
 *     capability("future.compute.capability")
 *
 * The capability identity is semantic data.
 *
 * ============================================================================
 */

resourceCapabilityConstraint
    : resourceCapabilityReference
    ;

resourceCapabilityReference
    : CAPABILITY
      LPAREN
      optionalResourceExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 26. RESOURCE PROPERTY CONSTRAINT
 * ============================================================================
 *
 * Properties are open-world semantic names.
 *
 * This permits future properties without creating a new grammar keyword for
 * every resource technology.
 *
 * Examples:
 *
 *     memory::capacity >= required_memory
 *
 *     quantum::logical_qubits >= logical_qubits
 *
 *     network::bandwidth >= required_bandwidth
 *
 *     accelerator::tensor_width >= required_width
 *
 * The semantic layer determines whether a property exists and what its type
 * and units are.
 *
 * ============================================================================
 */

resourcePropertyConstraint
    : resourcePropertyPath
      resourceComparisonOperator
      resourceExpression
    ;


/*
 * ============================================================================
 * 27. RESOURCE RELATIONSHIP CONSTRAINT
 * ============================================================================
 *
 * Resource relationships are intentionally represented through expressions.
 *
 * This permits semantic models such as:
 *
 *     memory::capacity >= compute::required_memory
 *
 *     network::bandwidth >= workload::required_bandwidth
 *
 *     quantum::logical_qubits >= workload::logical_qubits
 *
 *     accelerator::memory >= tensor::memory_requirement
 *
 * The grammar does not decide whether a relationship is satisfiable.
 *
 * ============================================================================
 */

resourceRelationshipConstraint
    : resourceExpression
      resourceComparisonOperator
      resourceExpression
    ;


/*
 * ============================================================================
 * 28. RESOURCE CONSTRAINT PREDICATE
 * ============================================================================
 *
 * This is the integration boundary for consumers that need one resource
 * predicate without importing the concrete `constraint ...;` statement.
 *
 * It intentionally contains one atomic predicate.
 *
 * ============================================================================
 */

resourceConstraintPredicate
    : resourceConstraintAtom
    ;


/*
 * ============================================================================
 * 29. RESOURCE CONSTRAINT LIST
 * ============================================================================
 *
 * Cardinality is intentionally unbounded at the language level.
 *
 * There is no:
 *
 *     MAX_CONSTRAINTS
 *
 * A practical implementation may encounter host/compiler/resource limits,
 * but those are implementation constraints rather than language semantics.
 *
 * ============================================================================
 */

resourceConstraintList
    : resourceConstraintPredicate*
    ;

optionalResourceConstraintList
    : resourceConstraintList?
    ;


/*
 * ============================================================================
 * 30. RESOURCE CONSTRAINT GROUP
 * ============================================================================
 *
 * Grouping is delegated to the generic constraint grammar.
 *
 * This rule exists only as a resource-domain integration boundary.
 *
 * ============================================================================
 */

resourceConstraintGroup
    : LPAREN
      resourceConstraintPredicate
      RPAREN
    ;


/*
 * ============================================================================
 * 31. RESOURCE QUANTITY INTEGRATION
 * ============================================================================
 *
 * Quantity expressions are deliberately inherited from:
 *
 *     resource-expressions.g4
 *
 * They can therefore contain:
 *
 *     constants
 *     generic parameters
 *     workload values
 *     symbolic values
 *     derived values
 *     runtime/contextual values
 *     arbitrary supported expressions
 *
 * Examples:
 *
 *     n
 *     problem_size
 *     workload.size
 *     workload_size * element_size
 *     logical_qubits + ancilla_count
 *     input.size * bytes_per_element
 *
 * This grammar does not evaluate them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. NO PHYSICAL RESOURCE SELECTION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     physicalQubit
 *     physicalCpu
 *     physicalGpu
 *     physicalFpga
 *     physicalNode
 *     physicalMemoryBank
 *
 * as universal resource constraint syntax.
 *
 * Physical realization is downstream.
 *
 * If a target-specific dialect explicitly exposes physical mapping, it must
 * use the target/dialect architecture and must not redefine this resource
 * constraint model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum resource constraints may express:
 *
 *     quantum::logical_qubits >= logical_qubits
 *
 *     quantum::measurement_capability == required_capability
 *
 *     quantum::reliability >= required_reliability
 *
 *     capability("quantum.measurement")
 *
 *     capability("quantum.mid_circuit_measurement")
 *
 *     capability("quantum.dynamic_control")
 *
 * The quantum grammar owns quantum operation syntax.
 *
 * The quantum semantic subsystem owns quantum meaning.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT create:
 *
 *     QuantumResourceIR
 *
 *     QuantumConstraintIR
 *
 * or another competing quantum intermediate representation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical programs may use:
 *
 *     compute::parallelism >= required_parallelism
 *
 *     memory::capacity >= required_memory
 *
 *     capability("vector.compute")
 *
 *     capability("tensor.compute")
 *
 *     capability("parallel.compute")
 *
 * No CPU/core/thread maximum is implied.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Portable source may express capabilities such as:
 *
 *     capability("gpu.compute")
 *
 *     capability("fpga.compute")
 *
 *     capability("accelerator.tensor")
 *
 *     capability("accelerator.reconfigurable")
 *
 * The grammar does not enumerate:
 *
 *     vendors
 *     models
 *     device IDs
 *     memory sizes
 *     compute-unit counts
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL domains may consume resource constraints for:
 *
 *     memory capacity
 *     timing
 *     throughput
 *     bandwidth
 *     power
 *     energy
 *     reliability
 *     scalability
 *     capability
 *
 * The actual hardware realization remains owned by:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *     compiler
 *     HAL
 *     target infrastructure
 *
 * This file does not duplicate hardware constraints.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed computation may express:
 *
 *     distributed::nodes >= required_nodes
 *
 *     network::bandwidth >= required_bandwidth
 *
 *     network::latency <= latency_budget
 *
 *     capability("distributed.execution")
 *
 * The source does not establish a universal node limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI/data workloads may express:
 *
 *     memory::capacity >= model_memory
 *
 *     accelerator::throughput >= required_throughput
 *
 *     capability("tensor.compute")
 *
 *     capability("distributed.training")
 *
 * Tensor rank, dimensions, model size, and dataset size remain program
 * semantics or resource-context values rather than grammar-level maxima.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. RESOURCE REQUIREMENT DISTINCTION
 * ============================================================================
 *
 * This file owns CONSTRAINT ATOMS.
 *
 * It does NOT own mandatory requirement statements.
 *
 * Requirements belong to:
 *
 *     grammar/resources/requirements.g4
 *
 * The distinction is intentional:
 *
 *     requirement
 *         =
 *     something that must be provided/satisfied
 *
 *     constraint
 *         =
 *     a restriction that valid realizations must obey
 *
 *     capability
 *         =
 *     an ability exposed by a realization
 *
 *     preference
 *         =
 *     advisory optimization intent
 *
 *     hint
 *         =
 *     advisory implementation guidance
 *
 * These semantic categories MUST NOT silently collapse into one another.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. GENERIC CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * grammar/core/constraints.g4 remains responsible for:
 *
 *     generic constraint syntax
 *     boolean composition
 *     NOT
 *     AND
 *     OR
 *     grouping
 *     generic references
 *     generic relational structure
 *     type/bound constraints
 *
 * This file supplies resource-domain atoms to that system.
 *
 * The intended conceptual composition is:
 *
 *     generic constraint expression
 *             |
 *             +--> resourceConstraintAtom
 *             |
 *             +--> other domain atom
 *
 * This avoids creating a second boolean/precedence system.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. CONCRETE RESOURCE STATEMENT INTEGRATION
 * ============================================================================
 *
 * grammar/resources/resources.g4 currently owns:
 *
 *     resourceConstraint
 *
 * with the concrete form:
 *
 *     constraint resourceConstraintExpression SEMICOLON
 *
 * That file should integrate this grammar through:
 *
 *     resourceConstraintExpression
 *         : resourceConstraintPredicate
 *         | resourceExpression
 *         ;
 *
 * or an equivalent composition that preserves the repository's canonical
 * generic constraint architecture.
 *
 * IMPORTANT:
 *
 * Do not define another `resourceConstraint` rule here.
 *
 * This file is therefore independently complete without requiring a later
 * rename or ownership transfer.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. RESOURCE EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `resource-expressions.g4` is already the resource-expression composition
 * boundary.
 *
 * This file consumes:
 *
 *     resourceExpression
 *     resourcePropertyPath
 *     optionalResourceExpressionList
 *     resourceComparisonOperator
 *
 * from that grammar.
 *
 * It does not recreate those rules.
 *
 * This guarantees:
 *
 *     one expression authority
 *     one comparison-token authority
 *     one resource-expression authority
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. LEXER INTEGRATION
 * ============================================================================
 *
 * Lexical ownership remains entirely in:
 *
 *     grammar/lexer/
 *
 * and:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar relies on the repository's existing comparison tokens:
 *
 *     EQ_EQ
 *     NOT_EQ
 *     LESS
 *     GREATER
 *     LE
 *     GE
 *
 * It does NOT invent:
 *
 *     EQUAL
 *     LESS_THAN
 *     LESS_THAN_OR_EQUAL
 *     GREATER_THAN
 *     GREATER_THAN_OR_EQUAL
 *
 * because those names are not the canonical operator vocabulary observed in
 * the current repository.
 *
 * Likewise, this grammar does not invent:
 *
 *     IS
 *     IS_NOT
 *     SUPPORTS
 *
 * merely to make resource constraints appear more natural.
 *
 * New lexical keywords are a language-wide change and must go through the
 * canonical lexer authority and compatibility process.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. NAME INTEGRATION
 * ============================================================================
 *
 * Resource names and property paths use the canonical resource-expression
 * infrastructure.
 *
 * Namespace depth is not bounded.
 *
 * Examples:
 *
 *     memory
 *
 *     memory::capacity
 *
 *     quantum::logical_qubits
 *
 *     distributed::network::bandwidth
 *
 *     future::domain::subsystem::resource::property
 *
 * The grammar does not impose a maximum namespace depth.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. UNITS AND DIMENSIONS
 * ============================================================================
 *
 * This grammar deliberately does not validate units.
 *
 * Examples of semantic quantities may include:
 *
 *     memory
 *     latency
 *     bandwidth
 *     throughput
 *     energy
 *     power
 *
 * Unit compatibility belongs to semantic/type/resource analysis.
 *
 * Therefore:
 *
 *     latency <= latency_budget
 *
 * is syntactic.
 *
 * Whether both sides have compatible dimensions is semantic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. OVERFLOW / NUMERIC SAFETY
 * ============================================================================
 *
 * The grammar must not truncate resource quantities.
 *
 * It must not clamp them.
 *
 * It must not convert them to a fixed machine integer merely because the
 * target compiler happens to use one internally.
 *
 * Numeric representation and overflow behavior belong to the canonical
 * literal/type/semantic implementation.
 *
 * The resource grammar imposes no artificial magnitude limit.
 *
 * Rust implementation of the surrounding compiler must remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * with no unsafe requirement.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar depends only on:
 *
 *     source
 *     lexical grammar
 *     parser grammar
 *     selected language version/dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     resource discovery
 *     target selection
 *     scheduler state
 *     runtime state
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *
 * Identical source and grammar/version must produce identical parse structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. SECURITY BOUNDARY
 * ============================================================================
 *
 * Resource constraints are declarative source syntax.
 *
 * They must not cause parser-time:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     device access
 *     hardware discovery
 *     secret access
 *     arbitrary code execution
 *
 * A later resource-resolution service may evaluate constraints against an
 * explicitly supplied resource/capability context.
 *
 * Such evaluation is outside this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. DOWNSTREAM SEMANTIC CONTRACT
 * ============================================================================
 *
 * A parsed resource constraint should lower conceptually as:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     resource constraint syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic ResourceConstraint
 *       |
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> portability analysis
 *       +--> target analysis
 *       +--> compiler
 *       +--> scheduler
 *       +--> runtime
 *       |
 *       v
 *     target realization
 *
 * The AST/semantic representation must preserve at minimum:
 *
 *     subject
 *     operator
 *     value
 *     source span
 *     domain
 *     constraint kind
 *
 * Additional semantic metadata may include:
 *
 *     units
 *     dimensional type
 *     provenance
 *     evaluation phase
 *     hardness
 *     applicability
 *
 * but those are not parser responsibilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. IR CONTRACT
 * ============================================================================
 *
 * Resource constraints are semantic metadata/legality conditions.
 *
 * They do not require a standalone resource IR merely because they appear in
 * source.
 *
 * The compiler may lower them into the canonical semantic/target contract
 * system already used by the repository.
 *
 * Quantum constraints remain associated with the existing:
 *
 *     quantum::ir
 *
 * boundary where quantum semantics require it.
 *
 * This grammar MUST NOT create:
 *
 *     ResourceIR
 *     QuantumResourceIR
 *     ConstraintIR
 *
 * merely to represent this syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. RESOURCE DISCOVERY
 * ============================================================================
 *
 * Discovery belongs downstream.
 *
 * For example:
 *
 *     constraint memory::capacity >= required_memory;
 *
 * does not cause the parser to ask:
 *
 *     "How much RAM does this machine have?"
 *
 * Likewise:
 *
 *     constraint capability::quantum::measurement == required;
 *
 * does not cause the parser to query a QPU.
 *
 * Resource resolution happens after parsing against an explicit semantic
 * context.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. PLACEMENT / ROUTING / SCHEDULING
 * ============================================================================
 *
 * This grammar does not decide:
 *
 *     which CPU
 *     which GPU
 *     which FPGA
 *     which QPU
 *     which node
 *     which memory bank
 *     which physical qubit
 *     which network path
 *
 * It also does not decide:
 *
 *     placement
 *     routing
 *     scheduling
 *
 * Those remain downstream implementation decisions subject to the semantic
 * constraints represented here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. QEC / ZQN / RESILIENCE
 * ============================================================================
 *
 * Resource reliability/resilience constraints can be consumed by those
 * subsystems, but they do not define those subsystems.
 *
 * QEC owns:
 *
 *     error detection
 *     error correction
 *     code selection
 *     correction procedures
 *
 * ZQN owns:
 *
 *     quantum noise/fault semantics
 *     relevant fault models
 *     quantum network/noise semantics
 *
 * Resilience owns:
 *
 *     recovery policy
 *     degraded execution
 *     retry/recover/escalation behavior
 *
 * The resource grammar only expresses resource-level conditions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. EXPECTED RESOURCE EXAMPLES
 * ============================================================================
 *
 * These examples are semantic examples, not parser-enforced capacities.
 *
 *     constraint quantum::logical_qubits >= logical_qubits;
 *
 *     constraint memory::capacity >= required_memory;
 *
 *     constraint network::bandwidth >= required_bandwidth;
 *
 *     constraint network::latency <= latency_budget;
 *
 *     constraint accelerator::throughput >= required_throughput;
 *
 *     constraint power::budget <= allowed_power;
 *
 *     constraint reliability::level >= required_reliability;
 *
 *     constraint scalability::factor >= requested_scale;
 *
 *     constraint capability::quantum::measurement == required_capability;
 *
 *     constraint tensor::rank == requested_rank;
 *
 * The implementation determines whether the target context can satisfy them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar is recursively compositional and contains no fixed resource
 * cardinalities.
 *
 * Therefore it permits source semantics involving:
 *
 *     one resource
 *     many resources
 *     resource sets
 *     resource hierarchies
 *     heterogeneous resources
 *     dynamically derived quantities
 *     distributed resources
 *     future resource classes
 *
 * There is no grammar-level maximum number of:
 *
 *     constraints
 *     resource references
 *     namespace segments
 *     expressions
 *     resource properties
 *
 * Practical limits belong to the implementation environment.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable resource syntax must not be silently changed by this file.
 *
 * In particular:
 *
 *     resourceConstraint
 *
 * remains owned by:
 *
 *     grammar/resources/resources.g4
 *
 * Existing resource-expression comparison tokens remain owned by:
 *
 *     grammar/lexer/operators.g4
 *
 * New syntax must use the language compatibility process.
 *
 * Historical/aspirational syntax in:
 *
 *     grammar/Zamani-Grammar.md
 *
 * does not become legal merely because it is documented there.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *   [ ] exactly one owner for resource constraint atoms;
 *   [ ] resources.g4 owns resourceConstraint statement;
 *   [ ] constraints.g4 does not define resourceConstraint statement;
 *   [ ] resource-expressions.g4 owns resourceComparisonOperator;
 *   [ ] lexer owns comparison token spelling;
 *   [ ] no duplicate expression grammar exists;
 *   [ ] no duplicate name grammar exists;
 *   [ ] no hardware discovery occurs during parsing;
 *   [ ] no physical resource allocation occurs during parsing;
 *   [ ] no fixed machine-size limit exists;
 *   [ ] no fixed quantum gate vocabulary is introduced;
 *   [ ] capabilities remain open-world;
 *   [ ] resource properties remain extensible;
 *   [ ] quantities remain expressions;
 *   [ ] constraint cardinality is unbounded by language semantics;
 *   [ ] namespace depth is unbounded by language semantics;
 *   [ ] source spans can be preserved;
 *   [ ] AST mapping exists;
 *   [ ] semantic mapping exists;
 *   [ ] downstream resource analysis exists;
 *   [ ] compiler integration exists;
 *   [ ] runtime integration exists;
 *   [ ] quantum integration preserves quantum::ir;
 *   [ ] QEC is not implemented in grammar;
 *   [ ] ZQN is not implemented in grammar;
 *   [ ] routing is not implemented in grammar;
 *   [ ] scheduling is not implemented in grammar;
 *   [ ] no unsafe Rust requirement exists.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. REQUIRED TEST CONTRACT
 * ============================================================================
 *
 * The repository's resource test suite must eventually cover:
 *
 * POSITIVE
 * --------
 *
 *     constraint quantum::logical_qubits >= logical_qubits;
 *
 *     constraint memory::capacity >= required_memory;
 *
 *     constraint network::latency <= latency_budget;
 *
 *     constraint network::bandwidth >= required_bandwidth;
 *
 *     constraint accelerator::throughput >= required_throughput;
 *
 *     constraint capability::quantum::measurement == required_capability;
 *
 *     constraint tensor::rank == requested_rank;
 *
 * NEGATIVE
 * --------
 *
 *     malformed comparison
 *     missing operand
 *     missing semicolon
 *     malformed property path
 *     malformed capability call
 *
 * BOUNDARY
 * --------
 *
 *     zero where semantically valid
 *     symbolic quantities
 *     very large representable quantities
 *     nested resource paths
 *     large constraint collections
 *
 * SCALABILITY
 * ----------
 *
 *     tiny workload
 *     large workload
 *     symbolic workload
 *     dynamic workload
 *     distributed workload
 *     heterogeneous resource set
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and grammar/version must yield identical parse structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 *   [x] it has one clear ownership boundary;
 *   [x] it does not redefine the concrete resourceConstraint statement;
 *   [x] it consumes canonical resource expressions;
 *   [x] it consumes canonical comparison operators;
 *   [x] it does not create another expression grammar;
 *   [x] it does not create another lexer;
 *   [x] it does not enumerate hardware;
 *   [x] it does not impose resource maxima;
 *   [x] it is open-world for properties/capabilities;
 *   [x] quantities are expression-based;
 *   [x] resource names remain semantic references;
 *   [x] it is deterministic;
 *   [x] it has no executable actions;
 *   [x] it requires no unsafe Rust;
 *   [x] it has explicit AST/semantic/IR boundaries;
 *   [x] it preserves quantum::ir as the canonical quantum boundary.
 *
 * Repository integration is complete when:
 *
 *   [ ] resources.g4 imports this grammar and delegates its constraint
 *       expression/atom to it;
 *   [ ] ZamaniParser.g4 composes Resources exactly once;
 *   [ ] Zamani.g4 remains the root composition boundary;
 *   [ ] generated ANTLR parser generation succeeds;
 *   [ ] Rust 1.97.1 compilation succeeds;
 *   [ ] positive tests pass;
 *   [ ] negative tests pass;
 *   [ ] scalability tests pass;
 *   [ ] determinism tests pass;
 *   [ ] AST coverage passes;
 *   [ ] semantic coverage passes;
 *   [ ] downstream resource/target integration passes.
 *
 * ============================================================================
 * 60. FINAL INVARIANT
 * ============================================================================
 *
 * The resource constraint grammar describes:
 *
 *     WHAT must be true of a valid realization.
 *
 * It does not describe:
 *
 *     WHICH machine must be used.
 *
 * Therefore:
 *
 *     SOURCE
 *        |
 *        v
 *     RESOURCE INTENT
 *        |
 *        v
 *     RESOURCE CONSTRAINT
 *        |
 *        v
 *     DOMAIN-NEUTRAL AST
 *        |
 *        v
 *     SEMANTIC RESOURCE MODEL
 *        |
 *        +--> capability resolution
 *        +--> resource analysis
 *        +--> portability analysis
 *        +--> target analysis
 *        |
 *        v
 *     CANONICAL IR / SEMANTIC BOUNDARY
 *        |
 *        +--> classical
 *        +--> quantum::ir
 *        +--> HDL/hardware
 *        |
 *        v
 *     optimization
 *        |
 *        +--> routing
 *        +--> scheduling
 *        +--> resilience
 *        +--> QEC where applicable
 *        +--> ZQN where applicable
 *        |
 *        v
 *     HAL
 *        |
 *        v
 *     TARGET REALIZATION
 *
 * The fundamental rule is:
 *
 *     PROGRAM DESCRIBES WHAT IT NEEDS.
 *
 *     IMPLEMENTATION DETERMINES HOW THOSE NEEDS ARE REALIZED.
 *
 * This separation is mandatory for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */