/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/resource-expressions.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar name:
 *     ResourceExpressions
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions.
 *     This grammar contains no semantic predicates.
 *     This grammar requires no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the RESOURCE-EXPRESSION COMPOSITION BOUNDARY for Zamani.
 *
 * Resource expressions are expressions used to describe:
 *
 *     resource quantities
 *     resource relationships
 *     resource requirements
 *     resource constraints
 *     resource preferences
 *     resource hints
 *     capability conditions
 *     target conditions
 *     capacity
 *     availability
 *     performance
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     portability
 *     scalability
 *     resource selection predicates
 *     resource-derived values
 *
 * The resource-expression layer MUST remain an expression COMPOSITION layer.
 *
 * It MUST NOT create a second arithmetic, logical, comparison, unary,
 * assignment, literal, or primary-expression implementation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser / expression parser
 *          |
 *          v
 *     ResourceExpressions
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic/resource analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-----------------------------+
 *          |             |               |
 *          v             v               v
 *       compiler      scheduler       runtime
 *          |             |               |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 target realization
 *
 * This grammar does NOT perform resource analysis.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the resource-expression composition boundary;
 *   - the canonical resource-expression entry point;
 *   - resource expression lists;
 *   - optional resource expression lists;
 *   - resource predicates;
 *   - resource comparisons;
 *   - resource logical conditions;
 *   - resource range expressions;
 *   - resource quantity expressions;
 *   - resource-derived expressions;
 *   - resource path expressions;
 *   - resource selector expressions;
 *   - resource capability predicates;
 *   - resource target predicates;
 *   - resource property access;
 *   - resource expression wrappers around canonical expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer rules;
 *   - identifier spelling;
 *   - numeric literal spelling;
 *   - string literal spelling;
 *   - operator spelling;
 *   - operator precedence;
 *   - arithmetic;
 *   - logical operator implementation;
 *   - comparison operator implementation;
 *   - unary operator implementation;
 *   - assignment;
 *   - general function calls;
 *   - general indexing;
 *   - general member access;
 *   - general types;
 *   - resource declarations;
 *   - resource lifecycle;
 *   - hardware discovery;
 *   - hardware allocation;
 *   - resource scheduling;
 *   - routing;
 *   - optimization;
 *   - runtime resource management;
 *   - quantum::ir;
 *   - classical IR;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - deployment.
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * The repository already has a canonical expression architecture.
 *
 * Therefore this grammar IMPORTS that architecture.
 *
 * It MUST NOT redefine:
 *
 *     expression
 *     assignmentExpression
 *     binaryExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     bitwiseOrExpression
 *     bitwiseXorExpression
 *     bitwiseAndExpression
 *     equalityExpression
 *     relationalExpression
 *     shiftExpression
 *     additiveExpression
 *     multiplicativeExpression
 *     unaryExpression
 *     primaryExpression
 *
 * Resource expressions use the canonical `expression` rule.
 *
 * This ensures that:
 *
 *     x + y
 *     x * y
 *     x >= y
 *     x && y
 *     x || y
 *     !x
 *     f(x)
 *     value[index]
 *     value.member
 *
 * have exactly the same syntax and precedence everywhere in Zamani.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar contains NO machine-size constants.
 *
 * It does NOT encode:
 *
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_NETWORK_LINKS
 *     MAX_RESOURCE_GROUPS
 *     MAX_RESOURCE_PROPERTIES
 *     MAX_EXPRESSION_COUNT
 *
 * Resource quantities are expressions.
 *
 * Therefore:
 *
 *     required_memory
 *     problem_size
 *     available_memory
 *     qubit_count
 *     parallelism
 *     workload_size
 *     node_count
 *
 * may all be represented symbolically.
 *
 * Physical feasibility is determined by downstream resource analysis.
 *
 * This is essential to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * A resource expression may describe:
 *
 *     what is required;
 *     what is allowed;
 *     what is preferred;
 *     what is available;
 *     what is capable;
 *     what is scalable;
 *     what is portable.
 *
 * It must not implicitly select:
 *
 *     a physical device;
 *     a physical address;
 *     a fixed topology;
 *     a fixed qubit;
 *     a fixed processor;
 *     a fixed accelerator;
 *     a fixed machine size.
 *
 * Concrete hardware realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE SEMANTIC DISTINCTION
 * ============================================================================
 *
 * The parser preserves the distinction between:
 *
 *     quantity
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     target
 *     capacity
 *     availability
 *
 * Semantic analysis determines their meaning.
 *
 * For example:
 *
 *     required_memory
 *
 * is not equivalent to:
 *
 *     available_memory
 *
 * and:
 *
 *     preferred_latency
 *
 * is not equivalent to:
 *
 *     maximum_allowed_latency
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is intended to be imported by:
 *
 *     grammar/resources/resources.g4
 *
 * Example:
 *
 *     parser grammar ZamaniResourcesParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import ResourceExpressions;
 *
 * The universal resource grammar should consume:
 *
 *     resourceExpression
 *
 * rather than defining another expression rule.
 *
 * ============================================================================
 * EXPRESSION DEPENDENCY
 * ============================================================================
 *
 * The canonical expression grammar is:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file imports it.
 *
 * Consequently:
 *
 *     ResourceExpressions
 *             |
 *             v
 *         Expressions
 *             |
 *             +--> AssignmentExpressions
 *             +--> BinaryExpressions
 *             +--> UnaryExpressions
 *
 * ResourceExpressions MUST NOT bypass the canonical expression hierarchy.
 *
 * ============================================================================
 */

parser grammar ResourceExpressions;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. CANONICAL RESOURCE EXPRESSION
 * ============================================================================
 *
 * This is the ONLY general-purpose resource expression entry point.
 *
 * Resource declarations, requirements, constraints, preferences, hints,
 * capabilities, targets, capacity expressions, availability expressions,
 * performance expressions and other resource constructs should consume this
 * rule.
 *
 * Example:
 *
 *     required_memory
 *
 *     workload_size * element_size
 *
 *     available_memory >= required_memory
 *
 *     latency <= latency_budget
 *
 *     energy <= energy_budget
 *
 *     problem_size * parallelism
 *
 * The actual semantic classification happens outside the parser.
 */
resourceExpression
    : expression
    ;


/*
 * ============================================================================
 * 2. RESOURCE EXPRESSION LIST
 * ============================================================================
 *
 * Arbitrary expression-list cardinality.
 *
 * No fixed number of resource values is permitted by the grammar.
 */
resourceExpressionList
    : resourceExpression
      (COMMA resourceExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 3. OPTIONAL RESOURCE EXPRESSION LIST
 * ============================================================================
 */

optionalResourceExpressionList
    : resourceExpressionList?
    ;


/*
 * ============================================================================
 * 4. RESOURCE CONDITION
 * ============================================================================
 *
 * Conditions deliberately use the canonical expression grammar.
 *
 * This prevents resource conditions from acquiring a separate boolean
 * language.
 */
resourceCondition
    : resourceExpression
    ;


/*
 * ============================================================================
 * 5. RESOURCE PREDICATE
 * ============================================================================
 *
 * A predicate is an expression whose semantic result is interpreted as a
 * resource condition.
 *
 * Examples:
 *
 *     available_memory >= required_memory
 *
 *     capability_value == required_capability
 *
 *     latency <= latency_budget
 *
 *     throughput >= required_throughput
 *
 * The parser does not decide whether an expression is actually boolean.
 * Type/semantic analysis does that.
 */
resourcePredicate
    : resourceExpression
    ;


/*
 * ============================================================================
 * 6. RESOURCE COMPARISON
 * ============================================================================
 *
 * This rule exists as an explicit semantic composition boundary.
 *
 * IMPORTANT:
 *
 * It intentionally consumes the canonical expression rather than defining
 * comparison precedence itself.
 *
 * This prevents two competing comparison grammars.
 */
resourceComparison
    : resourceExpression
    ;


/*
 * ============================================================================
 * 7. RESOURCE QUANTITY EXPRESSION
 * ============================================================================
 *
 * A quantity expression is any canonical expression whose semantic type can
 * represent a resource quantity.
 *
 * Examples:
 *
 *     n
 *
 *     problem_size
 *
 *     workload_size * lanes
 *
 *     available_memory - reserved_memory
 *
 *     logical_qubits
 *
 * The grammar does not decide whether the expression is dimensionally valid.
 *
 * Dimensional/unit validation belongs to semantic analysis.
 */
resourceQuantityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 8. RESOURCE DERIVATION EXPRESSION
 * ============================================================================
 *
 * A derived resource quantity may depend on arbitrary source expressions.
 *
 * Example:
 *
 *     workload_size * bytes_per_element
 *
 *     problem_size * required_parallelism
 *
 *     logical_qubits + ancilla_count
 *
 * The grammar does not evaluate the expression.
 */
resourceDerivationExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 9. RESOURCE CAPACITY EXPRESSION
 * ============================================================================
 *
 * Capacity may be supplied by:
 *
 *     compile-time context
 *     execution context
 *     runtime context
 *     hardware capability model
 *     resource manager
 *
 * This grammar merely parses the expression referring to that value.
 */
resourceCapacityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 10. RESOURCE AVAILABILITY EXPRESSION
 * ============================================================================
 *
 * Availability may be dynamic.
 *
 * The grammar MUST NOT query or inspect availability.
 */
resourceAvailabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 11. RESOURCE PERFORMANCE EXPRESSION
 * ============================================================================
 */

resourcePerformanceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 12. RESOURCE LATENCY EXPRESSION
 * ============================================================================
 */

resourceLatencyExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 13. RESOURCE THROUGHPUT EXPRESSION
 * ============================================================================
 */

resourceThroughputExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 14. RESOURCE BANDWIDTH EXPRESSION
 * ============================================================================
 */

resourceBandwidthExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 15. RESOURCE ENERGY EXPRESSION
 * ============================================================================
 */

resourceEnergyExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 16. RESOURCE POWER EXPRESSION
 * ============================================================================
 */

resourcePowerExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 17. RESOURCE RELIABILITY EXPRESSION
 * ============================================================================
 */

resourceReliabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 18. RESOURCE RESILIENCE EXPRESSION
 * ============================================================================
 */

resourceResilienceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 19. RESOURCE COST EXPRESSION
 * ============================================================================
 *
 * Cost remains abstract.
 *
 * No currency, provider or pricing system is embedded here.
 */
resourceCostExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 20. RESOURCE PORTABILITY EXPRESSION
 * ============================================================================
 */

resourcePortabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 21. RESOURCE SCALABILITY EXPRESSION
 * ============================================================================
 *
 * Examples:
 *
 *     problem_size
 *
 *     workload_size
 *
 *     input_dimension * parallelism
 *
 *     symbolic_scale
 *
 * No finite upper bound is represented.
 */
resourceScalabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 22. RESOURCE TARGET EXPRESSION
 * ============================================================================
 *
 * A target expression identifies an abstract target category.
 *
 * Examples may include symbolic forms such as:
 *
 *     cpu
 *     gpu
 *     quantum
 *     accelerator
 *     heterogeneous
 *
 * These names remain ordinary semantic identifiers unless the canonical
 * language specification explicitly reserves them.
 *
 * Concrete hardware selection remains outside this grammar.
 */
resourceTargetExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 23. RESOURCE CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Capability expressions refer to properties that may be supplied by the
 * execution environment.
 *
 * Examples:
 *
 *     capability
 *
 *     quantum.logical_operations
 *
 *     accelerator.vector
 *
 *     network.high_bandwidth
 *
 * Capability discovery is NOT performed here.
 */
resourceCapabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 24. RESOURCE SELECTOR
 * ============================================================================
 *
 * A selector is a symbolic resource path.
 *
 * This rule deliberately uses IDENTIFIER and DOUBLE_COLON from the canonical
 * lexer.
 *
 * It does not create a resource keyword vocabulary.
 *
 * Examples:
 *
 *     memory
 *
 *     compute
 *
 *     quantum::logical_qubits
 *
 *     hardware::accelerator
 *
 *     network::bandwidth
 *
 *     future::domain::resource
 *
 * Namespace depth is unbounded by the grammar.
 *
 * Semantic validation determines whether the selected resource exists.
 */
resourceSelector
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * 25. RESOURCE PROPERTY PATH
 * ============================================================================
 *
 * Resource properties may be represented as paths.
 *
 * Examples:
 *
 *     memory.capacity
 *
 *     quantum.logical_qubits
 *
 *     hardware.accelerator.capability
 *
 *     network.link.bandwidth
 *
 * This grammar does not decide whether a property is standard, vendor,
 * dialect-specific, experimental, or invalid.
 */
resourcePropertyPath
    : identifier
      (
          DOT identifier
      )*
    ;


/*
 * ============================================================================
 * 26. QUALIFIED RESOURCE SELECTOR
 * ============================================================================
 *
 * Supports arbitrary namespace and property depth without machine limits.
 *
 * Examples:
 *
 *     quantum::logical_qubits
 *
 *     hardware::gpu::memory
 *
 *     distributed::network::bandwidth
 *
 *     future::domain::subsystem::resource::property
 *
 * The semantic layer determines whether such a path is meaningful.
 */
qualifiedResourceSelector
    : identifier
      (
          DOUBLE_COLON identifier
      )*
      (
          DOT identifier
      )*
    ;


/*
 * ============================================================================
 * 27. RESOURCE REFERENCE EXPRESSION
 * ============================================================================
 *
 * This rule gives higher-level resource grammar a stable symbolic-reference
 * boundary without introducing a second expression language.
 */
resourceReferenceExpression
    : resourceSelector
    ;


/*
 * ============================================================================
 * 28. RESOURCE PROPERTY EXPRESSION
 * ============================================================================
 *
 * Property expressions are intentionally represented as canonical expressions.
 *
 * The selector/path rules above exist for contexts that need a symbolic
 * resource name or property path rather than an arbitrary value expression.
 */
resourcePropertyExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 29. RESOURCE RANGE EXPRESSION
 * ============================================================================
 *
 * Range syntax is composed from the canonical expression grammar.
 *
 * Examples:
 *
 *     lower .. upper
 *
 *     lower ..= upper
 *
 * This rule uses the canonical DOT_DOT and DOT_DOT_EQ lexer tokens.
 *
 * It does not define a new range-expression precedence hierarchy.
 */
resourceRangeExpression
    : resourceExpression DOT_DOT resourceExpression
    | resourceExpression DOT_DOT_EQ resourceExpression
    ;


/*
 * ============================================================================
 * 30. RESOURCE RANGE OR VALUE
 * ============================================================================
 *
 * Useful for declarations accepting either:
 *
 *     a single resource quantity
 *
 * or:
 *
 *     a resource interval.
 */
resourceRangeOrValue
    : resourceRangeExpression
    | resourceExpression
    ;


/*
 * ============================================================================
 * 31. RESOURCE COMPARISON FORM
 * ============================================================================
 *
 * Explicit comparison composition for resource-specific parent grammars.
 *
 * The comparison operator tokens are owned by ZamaniLexer.
 *
 * This rule does NOT replace the canonical comparison hierarchy.
 *
 * It is a resource-context grammar boundary.
 */
resourceComparisonForm
    : resourceExpression
      resourceComparisonOperator
      resourceExpression
    ;


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
 * 32. RESOURCE EQUALITY FORM
 * ============================================================================
 */

resourceEqualityForm
    : resourceExpression
      resourceEqualityOperator
      resourceExpression
    ;


resourceEqualityOperator
    : EQ_EQ
    | NOT_EQ
    ;


/*
 * ============================================================================
 * 33. RESOURCE ORDERING FORM
 * ============================================================================
 */

resourceOrderingForm
    : resourceExpression
      resourceOrderingOperator
      resourceExpression
    ;


resourceOrderingOperator
    : LESS
    | GREATER
    | LE
    | GE
    ;


/*
 * ============================================================================
 * 34. RESOURCE BOOLEAN FORM
 * ============================================================================
 *
 * Boolean operators are provided by the canonical lexer and expression
 * hierarchy.
 *
 * This rule is intentionally a composition boundary.
 */
resourceBooleanExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 35. RESOURCE VALUE LIST
 * ============================================================================
 *
 * Used by resource properties, selectors and extensible resource metadata
 * where arbitrary expression values are allowed.
 */
resourceValueList
    : resourceExpression
      (COMMA resourceExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 36. RESOURCE ASSIGNMENT VALUE
 * ============================================================================
 *
 * Resource declarations may use arbitrary canonical expressions as values.
 *
 * Assignment itself remains owned by the parent declaration grammar.
 *
 * This rule therefore does NOT consume ASSIGN.
 */
resourceAssignmentValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 37. RESOURCE ATTRIBUTE VALUE
 * ============================================================================
 *
 * Attributes remain syntax-level metadata.
 *
 * The attribute grammar remains the owner of attribute spelling and
 * structure.
 */
resourceAttributeValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 38. RESOURCE FILTER
 * ============================================================================
 *
 * A filter is an expression interpreted semantically as a selection
 * predicate.
 *
 * Examples:
 *
 *     available >= required
 *
 *     capability == required_capability
 *
 *     latency <= budget
 *
 * The parser does not determine truth.
 */
resourceFilter
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 39. RESOURCE SELECTION CONDITION
 * ============================================================================
 */

resourceSelectionCondition
    : resourceFilter
    ;


/*
 * ============================================================================
 * 40. RESOURCE NEGATION
 * ============================================================================
 *
 * The actual NOT/! semantics remain owned by the canonical expression grammar.
 *
 * This rule exists only as a resource semantic boundary.
 */
resourceNegation
    : resourceExpression
    ;


/*
 * ============================================================================
 * 41. RESOURCE FUNCTION VALUE
 * ============================================================================
 *
 * Function calls are already part of canonical expression syntax.
 *
 * This rule allows resource semantics to consume:
 *
 *     capacity()
 *     available(resource)
 *     required(workload)
 *
 * without making those functions parser-level built-ins.
 *
 * Name resolution determines whether such calls exist.
 */
resourceFunctionValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 42. RESOURCE SYMBOLIC VALUE
 * ============================================================================
 *
 * A symbolic value is intentionally not restricted to a numeric literal.
 *
 * It may ultimately depend on:
 *
 *     program input
 *     generic parameter
 *     compile-time value
 *     runtime observation
 *     workload
 *     configuration
 *     capability
 *     another resource
 *
 * Semantic analysis determines its type and evaluation phase.
 */
resourceSymbolicValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 43. RESOURCE CONSTANT VALUE
 * ============================================================================
 *
 * The expression remains canonical.
 *
 * Constantness is semantic/compile-time analysis, not parser behavior.
 */
resourceConstantValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 44. RESOURCE DYNAMIC VALUE
 * ============================================================================
 *
 * Dynamic resource information may be evaluated at runtime.
 *
 * This grammar does not perform runtime evaluation.
 */
resourceDynamicValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 45. RESOURCE CONSTRAINT VALUE
 * ============================================================================
 */

resourceConstraintValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 46. RESOURCE PREFERENCE VALUE
 * ============================================================================
 */

resourcePreferenceValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 47. RESOURCE HINT VALUE
 * ============================================================================
 */

resourceHintValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 48. RESOURCE REQUIREMENT VALUE
 * ============================================================================
 */

resourceRequirementValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 49. RESOURCE CAPABILITY VALUE
 * ============================================================================
 */

resourceCapabilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 50. RESOURCE AVAILABILITY VALUE
 * ============================================================================
 */

resourceAvailabilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 51. RESOURCE TARGET VALUE
 * ============================================================================
 */

resourceTargetValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 52. RESOURCE SCALING VALUE
 * ============================================================================
 */

resourceScalingValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 53. RESOURCE METRIC VALUE
 * ============================================================================
 *
 * A generic metric is represented by an expression.
 *
 * This permits future metrics without modifying this grammar solely because a
 * new metric is introduced.
 */
resourceMetricValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 54. RESOURCE EXTENSION VALUE
 * ============================================================================
 *
 * Dialects and future resource domains may attach values without forcing this
 * grammar to know every future resource kind.
 */
resourceExtensionValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 55. RESOURCE DIMENSIONAL VALUE
 * ============================================================================
 *
 * Dimensional correctness is semantic.
 *
 * Examples:
 *
 *     memory + memory
 *     bandwidth * duration
 *     energy / operation
 *
 * Whether such operations are meaningful is NOT determined by syntax.
 */
resourceDimensionalValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 56. RESOURCE QUANTITY RELATION
 * ============================================================================
 *
 * A relation such as:
 *
 *     available >= required
 *
 * is syntactically represented without embedding any physical assumption.
 */
resourceQuantityRelation
    : resourceComparisonForm
    ;


/*
 * ============================================================================
 * 57. RESOURCE CAPABILITY RELATION
 * ============================================================================
 */

resourceCapabilityRelation
    : resourceComparisonForm
    ;


/*
 * ============================================================================
 * 58. RESOURCE PERFORMANCE RELATION
 * ============================================================================
 */

resourcePerformanceRelation
    : resourceComparisonForm
    ;


/*
 * ============================================================================
 * 59. RESOURCE AVAILABILITY RELATION
 * ============================================================================
 */

resourceAvailabilityRelation
    : resourceComparisonForm
    ;


/*
 * ============================================================================
 * 60. RESOURCE PORTABILITY CONDITION
 * ============================================================================
 */

resourcePortabilityCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 61. RESOURCE SCALABILITY CONDITION
 * ============================================================================
 */

resourceScalabilityCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 62. RESOURCE RELIABILITY CONDITION
 * ============================================================================
 */

resourceReliabilityCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 63. RESOURCE RESILIENCE CONDITION
 * ============================================================================
 */

resourceResilienceCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 64. RESOURCE COST CONDITION
 * ============================================================================
 */

resourceCostCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 65. RESOURCE TARGET CONDITION
 * ============================================================================
 */

resourceTargetCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 66. RESOURCE CAPABILITY CONDITION
 * ============================================================================
 */

resourceCapabilityCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 67. RESOURCE PROPERTY CONDITION
 * ============================================================================
 */

resourcePropertyCondition
    : resourcePredicate
    ;


/*
 * ============================================================================
 * 68. RESOURCE EXPRESSION WITH OPTIONAL RANGE
 * ============================================================================
 *
 * A resource value may optionally specify an interval.
 *
 * Example:
 *
 *     preferred_memory .. maximum_memory
 *
 * or:
 *
 *     minimum_memory ..= maximum_memory
 */
resourceExpressionWithOptionalRange
    : resourceRangeExpression
    | resourceExpression
    ;


/*
 * ============================================================================
 * 69. RESOURCE SEQUENCE
 * ============================================================================
 *
 * This provides a stable composition boundary for parent grammars that need
 * multiple resource expressions.
 */
resourceSequence
    : resourceExpressionList
    ;


/*
 * ============================================================================
 * 70. RESOURCE EXPRESSION GROUP
 * ============================================================================
 *
 * Parentheses are already part of canonical expression syntax.
 *
 * This rule exists as a semantic integration boundary only.
 */
resourceExpressionGroup
    : LPAREN resourceExpression RPAREN
    ;


/*
 * ============================================================================
 * 71. RESOURCE PATH VALUE
 * ============================================================================
 */

resourcePathValue
    : resourceSelector
    | resourcePropertyPath
    | qualifiedResourceSelector
    ;


/*
 * ============================================================================
 * 72. RESOURCE PATH OR EXPRESSION
 * ============================================================================
 *
 * Used by parent grammars that accept either a symbolic resource path or a
 * computed expression.
 */
resourcePathOrExpression
    : resourcePathValue
    | resourceExpression
    ;


/*
 * ============================================================================
 * 73. RESOURCE IDENTIFIER VALUE
 * ============================================================================
 *
 * This is intentionally based on the canonical parser's identifier rule.
 *
 * It does not duplicate identifier lexical rules.
 */
resourceIdentifierValue
    : identifier
    ;


/*
 * ============================================================================
 * 74. RESOURCE QUALIFIED IDENTIFIER VALUE
 * ============================================================================
 */

resourceQualifiedIdentifierValue
    : resourceSelector
    ;


/*
 * ============================================================================
 * 75. RESOURCE PROPERTY VALUE
 * ============================================================================
 */

resourcePropertyValue
    : resourcePropertyPath
    ;


/*
 * ============================================================================
 * 76. RESOURCE EXPRESSION ARGUMENTS
 * ============================================================================
 *
 * Function-like resource operations can consume arbitrary canonical
 * expressions.
 */
resourceExpressionArguments
    : LPAREN optionalResourceExpressionList RPAREN
    ;


/*
 * ============================================================================
 * 77. RESOURCE SELECTOR ARGUMENTS
 * ============================================================================
 */

resourceSelectorArguments
    : LPAREN
      resourceExpressionList?
      RPAREN
    ;


/*
 * ============================================================================
 * 78. RESOURCE INDEX VALUE
 * ============================================================================
 *
 * Index expressions use canonical expression syntax.
 *
 * This rule intentionally does not impose an index-width or dimensionality
 * limit.
 */
resourceIndexValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 79. RESOURCE INDEX LIST
 * ============================================================================
 *
 * Arbitrary-dimensional resource selectors are therefore possible.
 *
 * Example:
 *
 *     resource[i]
 *
 *     resource[i, j]
 *
 *     resource[i, j, k]
 *
 *     resource[index_1, ..., index_n]
 *
 * The grammar contains no dimensionality limit.
 */
resourceIndexList
    : resourceIndexValue
      (COMMA resourceIndexValue)*
      COMMA?
    ;


/*
 * ============================================================================
 * 80. RESOURCE INDEXED SELECTOR
 * ============================================================================
 */

resourceIndexedSelector
    : resourceSelector
      LBRACKET
      resourceIndexList
      RBRACKET
    ;


/*
 * ============================================================================
 * 81. RESOURCE SELECTOR VALUE
 * ============================================================================
 */

resourceSelectorValue
    : resourceSelector
    | resourceIndexedSelector
    | qualifiedResourceSelector
    ;


/*
 * ============================================================================
 * 82. RESOURCE SELECTOR OR EXPRESSION
 * ============================================================================
 */

resourceSelectorOrExpression
    : resourceSelectorValue
    | resourceExpression
    ;


/*
 * ============================================================================
 * 83. RESOURCE EXPRESSION CONTRACT
 * ============================================================================
 *
 * All rules above are parser-level contracts.
 *
 * They MUST NOT be interpreted as:
 *
 *     resource availability;
 *     resource ownership;
 *     resource allocation;
 *     hardware discovery;
 *     scheduling;
 *     routing;
 *     optimization;
 *     runtime execution.
 *
 * ============================================================================
 * SEMANTIC ANALYSIS CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis must determine:
 *
 *     identifier resolution
 *     resource kind
 *     resource namespace
 *     expression type
 *     numeric type
 *     dimensional units
 *     compile-time/runtime phase
 *     capability meaning
 *     target meaning
 *     availability meaning
 *     requirement severity
 *     constraint severity
 *     preference priority
 *     hint advisory status
 *     feasibility
 *
 * The grammar MUST NOT make those decisions.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum resource expressions may refer to concepts such as:
 *
 *     quantum::logical_qubits
 *     quantum::physical_qubits
 *     quantum::measurement
 *     quantum::memory
 *     quantum::error_correction
 *     quantum::fidelity
 *
 * The resource expression grammar does not define those domain concepts.
 *
 * `grammar/quantum/quantum-resources.g4` remains responsible for quantum
 * resource-specific syntax.
 *
 * Quantum resource semantics may subsequently be associated with:
 *
 *     quantum::ir
 *
 * but this grammar MUST NOT construct or redefine quantum::ir.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-specific resource expressions may refer to:
 *
 *     hardware::cpu
 *     hardware::gpu
 *     hardware::fpga
 *     hardware::asic
 *     hardware::memory
 *     hardware::accelerator
 *
 * This grammar does not discover or select physical hardware.
 *
 * Hardware capability resolution belongs to the hardware abstraction layer.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Resource expressions may provide:
 *
 *     latency constraints
 *     throughput requirements
 *     resource quantities
 *     resource conflicts
 *     capacity requirements
 *
 * The scheduling subsystem consumes semantic resource information.
 *
 * This grammar does NOT implement:
 *
 *     ASAP
 *     ALAP
 *     list scheduling
 *     critical path
 *     RCPSP
 *     resource allocation
 *     temporal placement
 *     dynamic scheduling
 *
 * ============================================================================
 * ROUTING INTEGRATION
 * ============================================================================
 *
 * Resource expressions may identify abstract capabilities relevant to routing.
 *
 * They do not encode:
 *
 *     physical topology
 *     physical qubit IDs
 *     device connectivity
 *     routing algorithms.
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Preferences and hints may become optimization inputs.
 *
 * A preference MUST NOT be silently promoted to a hard requirement.
 *
 * A hint MAY be ignored.
 *
 * ============================================================================
 * QEC INTEGRATION
 * ============================================================================
 *
 * Resource expressions may describe abstract QEC-related requirements.
 *
 * The grammar does not implement:
 *
 *     syndrome extraction
 *     code construction
 *     decoding
 *     correction
 *     logical error estimation.
 *
 * ============================================================================
 * ZQN INTEGRATION
 * ============================================================================
 *
 * Resource expressions may describe abstract reliability or resilience
 * requirements.
 *
 * ZQN remains the owner of noise/fault semantics.
 *
 * This grammar does not define:
 *
 *     noise channels
 *     fault models
 *     correlated faults
 *     leakage
 *     loss
 *     erasure
 *     calibration noise.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime systems may evaluate dynamic expressions against current resource
 * state.
 *
 * This parser performs no runtime access.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing resource expressions MUST NOT:
 *
 *     access files;
 *     access networks;
 *     access hardware;
 *     allocate resources;
 *     reserve devices;
 *     execute commands;
 *     execute arbitrary code;
 *     invoke plugins;
 *     mutate runtime state.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For an identical token stream, this grammar must produce deterministic
 * parser behavior.
 *
 * Resource availability, hardware state and runtime conditions must never
 * influence parsing.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors:
 *
 *     malformed expression
 *     malformed selector
 *     malformed path
 *     malformed range
 *     malformed list
 *     malformed comparison
 *
 * belong to parser diagnostics.
 *
 * Semantic errors:
 *
 *     unknown resource
 *     unknown capability
 *     invalid unit
 *     incompatible dimensions
 *     unsatisfied requirement
 *     unavailable resource
 *     unsupported target
 *
 * belong to semantic/resource analysis.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     resourceExpression
 *     resourceExpressionList
 *     resourceQuantityExpression
 *     resourceDerivationExpression
 *     resourceCapacityExpression
 *     resourceAvailabilityExpression
 *     resourcePerformanceExpression
 *     resourceLatencyExpression
 *     resourceThroughputExpression
 *     resourceBandwidthExpression
 *     resourceEnergyExpression
 *     resourcePowerExpression
 *     resourceReliabilityExpression
 *     resourceResilienceExpression
 *     resourceCostExpression
 *     resourcePortabilityExpression
 *     resourceScalabilityExpression
 *     resourceTargetExpression
 *     resourceCapabilityExpression
 *     resourceSelector
 *     resourcePropertyPath
 *     qualifiedResourceSelector
 *     resourceReferenceExpression
 *     resourceRangeExpression
 *     resourceComparisonForm
 *     resourceEqualityForm
 *     resourceOrderingForm
 *     resourceIndexedSelector
 *     resourceSelectorValue
 *
 * Positive expressions MUST include:
 *
 *     identifiers
 *     integer values
 *     floating-point values
 *     symbolic values
 *     arithmetic
 *     comparison
 *     logical expressions
 *     function calls
 *     indexing
 *     member access
 *     nested expressions
 *     qualified resource paths
 *
 * Negative tests MUST include:
 *
 *     malformed resource path
 *     empty selector
 *     malformed namespace path
 *     malformed property path
 *     malformed range
 *     missing range endpoint
 *     malformed expression list
 *     malformed comparison
 *     malformed index list
 *
 * Cross-domain tests MUST include:
 *
 *     classical resource expressions
 *     quantum resource expressions
 *     hardware resource expressions
 *     hybrid resource expressions
 *     distributed resource expressions
 *     networking resource expressions
 *     AI/ML resource expressions
 *     HDL resource expressions
 *
 * Scalability tests MUST verify:
 *
 *     no fixed resource count;
 *     no fixed selector depth;
 *     no fixed namespace depth;
 *     no fixed index dimensionality;
 *     no fixed expression-list cardinality;
 *     no fixed numeric machine width;
 *     no fixed qubit count;
 *     no fixed device count.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     device IDs
 *     hardware addresses
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     ASIC counts
 *     qubit counts
 *     node counts
 *     memory limits
 *     topology limits.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * ANTLR-generated Rust parser/runtime integration is external to this grammar.
 *
 * Zamani compiler Rust code must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must contain no unsafe Rust.
 *
 * This grammar contains no embedded target-language actions and therefore
 * does not introduce target-language safety dependencies.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] ANTLR accepts the grammar.
 *
 * [ ] `ResourceExpressions` resolves its `Expressions` import.
 *
 * [ ] `Expressions` resolves the canonical expression hierarchy.
 *
 * [ ] `ZamaniLexer` supplies every referenced token.
 *
 * [ ] No nonexistent K_* token is referenced.
 *
 * [ ] No lexer rule is duplicated here.
 *
 * [ ] No expression precedence is duplicated here.
 *
 * [ ] Resource quantities remain arbitrary expressions.
 *
 * [ ] Resource paths support arbitrary namespace depth.
 *
 * [ ] Resource property paths support arbitrary depth.
 *
 * [ ] Resource indexes have no fixed dimensionality.
 *
 * [ ] Resource lists have no fixed cardinality.
 *
 * [ ] No physical machine limit is encoded.
 *
 * [ ] No runtime resource discovery occurs in parsing.
 *
 * [ ] No hardware-specific implementation is embedded.
 *
 * [ ] Quantum resource syntax can consume this boundary without creating a
 *     second resource-expression language.
 *
 * [ ] Hardware resource syntax can consume this boundary without creating a
 *     second resource-expression language.
 *
 * [ ] Hybrid resource syntax can compose it.
 *
 * [ ] Resource semantics can be lowered into the frontend semantic model.
 *
 * [ ] Resource information can subsequently reach compilation, scheduling,
 *     routing, hardware abstraction and runtime systems.
 *
 * [ ] The grammar remains independent of concrete hardware.
 *
 * [ ] Positive, negative, boundary, scalability and determinism tests pass.
 *
 * ============================================================================
 */