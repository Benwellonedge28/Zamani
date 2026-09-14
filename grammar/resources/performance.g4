/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/resources/performance.g4
* 
* Grammar kind:
* ANTLR4 parser grammar
* 
* Grammar name:
* ResourcePerformance
* 
* Runtime/compiler baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* safe Rust only
* no unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file defines the canonical RESOURCE PERFORMANCE-INTENT grammar.
* 
* Performance intent describes desirable or declared computational
* performance characteristics without selecting a concrete machine,
* accelerator, device, processor, topology, or provider.
* 
* Performance may include:
* 
* - performance objectives;
* - performance targets;
* - performance bounds;
* - performance budgets;
* - throughput objectives;
* - latency objectives;
* - bandwidth objectives;
* - efficiency objectives;
* - utilization objectives;
* - scalability objectives;
* - responsiveness objectives;
* - cost/performance objectives;
* - energy/performance objectives;
* - reliability/performance objectives;
* - percentile-oriented objectives;
* - conditional objectives;
* - weighted objectives;
* - prioritized objectives;
* - lexicographic objectives;
* - symbolic objectives;
* - domain-specific future performance properties.
* 
* This grammar describes SOURCE-LEVEL INTENT.
* 
* It does NOT:
* 
* - measure performance;
* - benchmark hardware;
* - discover devices;
* - select hardware;
* - allocate resources;
* - schedule operations;
* - route operations;
* - optimize programs;
* - execute programs;
* - query runtime state;
* - define a performance model implementation;
* - define a benchmark implementation.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
*    lexer
*      |
*      v
*    parser
*      |
*      v
* ResourcePerformance
*      |
*      v
*  frontend AST
*      |
*      v
* semantic analysis
*      |
*      v
* performance-intent model
*      |
* +----+---------+------------------+
* |              |                  |
* v              v                  v
* compiler       optimizer          resource manager
* |              |                  |
* +--------------+------------------+
*                |
*                v
*         scheduling / routing
*                |
*                v
*            hardware HAL
*                |
*                v
*              runtime
* 
* Performance measurement and realization are downstream concerns.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - performance-intent entry points;
* - performance specifications;
* - performance clauses;
* - performance objectives;
* - performance bounds;
* - performance budgets;
* - performance targets;
* - performance priorities;
* - performance weights;
* - performance conditions;
* - performance objective groups;
* - performance dimensions;
* - performance aggregation intent;
* - performance ordering intent;
* - extensible performance properties.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical tokens;
* - identifiers;
* - literals;
* - general expressions;
* - expression precedence;
* - units;
* - general constraints;
* - general requirements;
* - capability identity;
* - resource identity;
* - target identity;
* - hardware discovery;
* - resource allocation;
* - benchmarking implementation;
* - performance measurement;
* - optimization algorithms;
* - scheduling algorithms;
* - routing algorithms;
* - hardware abstraction;
* - runtime execution;
* - quantum::ir;
* - classical IR;
* - QEC;
* - ZQN;
* - simulation.
* 
* ============================================================================
* NON-DUPLICATION CONTRACT
* ============================================================================
* 
* Performance values MUST consume the canonical resource-expression
* architecture.
* 
* This file MUST NOT redefine:
* 
* expression
* resourceExpression
* arithmetic precedence
* logical precedence
* comparison precedence
* literals
* identifiers
* function calls
* indexing
* member access
* 
* Resource expressions are owned by:
* 
* grammar/resources/resource-expressions.g4
* 
* General expression syntax is owned by the canonical expression grammar.
* 
* Therefore performance syntax composes those existing boundaries.
* 
* ============================================================================
* PERFORMANCE VS OTHER RESOURCE CONCEPTS
* ============================================================================
* 
* PERFORMANCE
* A measurable or semantically meaningful computational characteristic.
* 
* REQUIREMENT
* A mandatory condition that must be satisfied.
* 
* CONSTRAINT
* A mandatory restriction on valid realizations.
* 
* PREFERENCE
* A desirable objective that may be traded off against other objectives.
* 
* HINT
* Advisory information that may be ignored.
* 
* CAPABILITY
* Something an execution environment can provide.
* 
* TARGET
* An abstract compilation/execution category.
* 
* PERFORMANCE syntax MUST NOT silently convert one category into another.
* 
* In particular:
* 
* performance target
* 
* does not automatically mean:
* 
* hard requirement.
* 
* Whether an objective is mandatory, preferred, advisory, or merely
* descriptive is determined by its containing semantic construct.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Performance intent MUST preserve:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* Therefore this grammar MUST NOT encode:
* 
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QUBITS
* MAX_NODES
* MAX_MEMORY
* MAX_DEVICES
* MAX_ACCELERATORS
* 
* It MUST NOT encode:
* 
* physical device identifiers;
* physical addresses;
* provider-specific machine identifiers;
* fixed topology;
* fixed machine sizes;
* fixed accelerator counts.
* 
* Performance values must instead be expressed through:
* 
* symbolic expressions;
* workload-dependent expressions;
* resource-derived expressions;
* capability-derived expressions;
* compilation-context values;
* runtime observations;
* semantic budgets;
* target-independent objectives.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* No finite performance dimension count is imposed.
* 
* No finite objective count is imposed.
* 
* No finite group size is imposed.
* 
* No finite namespace depth is imposed.
* 
* No finite expression-list size is imposed.
* 
* No machine size is encoded.
* 
* No performance value is limited to a fixed numeric range by the grammar.
* 
* Examples of scalable symbolic expressions include:
* 
* workload_size
* problem_size
* desired_throughput
* latency_budget
* energy_budget
* available_bandwidth
* requested_parallelism
* target_efficiency
* 
* A literal such as 100 is merely a program value.
* 
* It MUST NOT be interpreted as a language-level hardware maximum.
* 
* ============================================================================
* OPEN-WORLD PERFORMANCE MODEL
* ============================================================================
* 
* Performance dimensions are intentionally extensible.
* 
* The language may therefore support future dimensions such as:
* 
* compute throughput;
* memory throughput;
* communication throughput;
* quantum execution fidelity;
* accelerator utilization;
* tensor throughput;
* inference throughput;
* training throughput;
* latency;
* jitter;
* responsiveness;
* energy efficiency;
* thermal efficiency;
* reliability-adjusted throughput;
* future domain-specific metrics.
* 
* The grammar does not require every future metric to become a lexer keyword.
* 
* Unknown properties are preserved syntactically and validated semantically.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Parsing performance intent MUST NOT:
* 
* - inspect hardware;
* - benchmark hardware;
* - access the filesystem;
* - access the network;
* - invoke a backend;
* - execute external commands;
* - allocate resources;
* - reserve devices;
* - query runtime state;
* - invoke arbitrary code.
* 
* ============================================================================
  */

parser grammar ResourcePerformance;

options {
tokenVocab = ZamaniLexer;
}

import ResourceExpressions;

/*

* ============================================================================
* 1. PUBLIC ENTRY POINT
* ============================================================================
* 
* Zero or more performance items may occur.
* 
* No finite cardinality is imposed.
  /
  resourcePerformance
  : resourcePerformanceItem
  ;

/*

* ============================================================================
* 2. PERFORMANCE ITEM
* ============================================================================
  */

resourcePerformanceItem
: resourcePerformanceDeclaration
| resourcePerformanceGroup
;

/*

* ============================================================================
* 3. PERFORMANCE DECLARATION
* ============================================================================
* 
* Canonical form:
* 
* performance throughput >= desired_throughput;
* 
* performance latency <= latency_budget;
* 
* performance efficiency >= target_efficiency;
* 
* performance custom::metric >= target;
* 
* The metric is open-world and semantic validation determines its meaning.
  */
  resourcePerformanceDeclaration
  : K_PERFORMANCE
  resourcePerformanceMetric
  resourcePerformanceRelation
  SEMICOLON
  ;

/*

* ============================================================================
* 4. PERFORMANCE SPECIFICATION
* ============================================================================
* 
* Structured form for multiple performance properties.
* 
* Example:
* 
* performance {
*     throughput = desired_throughput;
*     latency = latency_budget;
*     priority = performance_priority;
* }
* 
* This grammar records intent only.
  /
  resourcePerformanceSpecification
  : K_PERFORMANCE
  LBRACE
  resourcePerformanceClause
  RBRACE
  SEMICOLON?
  ;

/*

* ============================================================================
* 5. PERFORMANCE CLAUSE
* ============================================================================
  */

resourcePerformanceClause
: resourcePerformanceObjectiveClause
| resourcePerformanceTargetClause
| resourcePerformanceBoundClause
| resourcePerformanceBudgetClause
| resourcePerformancePriorityClause
| resourcePerformanceWeightClause
| resourcePerformanceConditionClause
| resourcePerformanceAggregationClause
| resourcePerformanceOrderingClause
| resourcePerformancePropertyClause
;

/*

* ============================================================================
* 6. PERFORMANCE OBJECTIVE
* ============================================================================
* 
* Example:
* 
* objective = throughput >= desired_throughput;
* 
* objective = latency <= latency_budget;
* 
* The semantic layer determines whether the objective is maximization,
* minimization, satisfaction-oriented, or another supported objective type.
  */
  resourcePerformanceObjectiveClause
  : K_OBJECTIVE
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 7. PERFORMANCE TARGET
* ============================================================================
* 
* A target describes a desired performance value.
* 
* Example:
* 
* target = desired_throughput;
* 
* target = target_latency;
* 
* It does not automatically become a hard constraint.
  */
  resourcePerformanceTargetClause
  : K_TARGET
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 8. PERFORMANCE BOUND
* ============================================================================
* 
* Bounds express a relation between a performance metric and a symbolic
* boundary.
* 
* Examples:
* 
* bound = latency <= latency_budget;
* 
* bound = throughput >= minimum_throughput;
* 
* bound = efficiency >= minimum_efficiency;

*/
resourcePerformanceBoundClause
: K_BOUND
ASSIGN
resourcePerformanceRelation
SEMICOLON
;

/*

* ============================================================================
* 9. PERFORMANCE BUDGET
* ============================================================================
* 
* A budget is a semantic allowance.
* 
* The grammar does not impose a numeric range.
* 
* Examples:
* 
* budget = latency_budget;
* 
* budget = energy_budget;
* 
* budget = communication_budget;

*/
resourcePerformanceBudgetClause
: K_BUDGET
ASSIGN
resourcePerformanceExpression
SEMICOLON
;

/*

* ============================================================================
* 10. PRIORITY
* ============================================================================
* 
* Priority is metadata for downstream policy/optimization.
* 
* It does not alter semantic feasibility by itself.
  */
  resourcePerformancePriorityClause
  : K_PRIORITY
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 11. WEIGHT
* ============================================================================
* 
* Weight is intentionally an expression.
* 
* The grammar does not assume:
* 
* 0..1
* 0..100
* integer-only values
* 
* Semantic analysis determines the applicable policy and type.
  */
  resourcePerformanceWeightClause
  : K_WEIGHT
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 12. CONDITIONAL PERFORMANCE
* ============================================================================
* 
* Example:
* 
* when = workload_size > threshold;
* 
* The condition remains declarative.
  */
  resourcePerformanceConditionClause
  : K_WHEN
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 13. AGGREGATION
* ============================================================================
* 
* Aggregation describes how multiple performance objectives are conceptually
* combined.
* 
* Examples:
* 
* aggregation = weighted;
* 
* aggregation = lexicographic;
* 
* aggregation = pareto;
* 
* aggregation = custom::policy;
* 
* The grammar accepts an expression so future policies do not require a new
* parser keyword.
  */
  resourcePerformanceAggregationClause
  : K_AGGREGATION
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 14. ORDERING
* ============================================================================
* 
* Ordering describes semantic preference between objectives.
* 
* Example:
* 
* ordering = latency_before_energy;
* 
* The optimization policy interprets this value.
  */
  resourcePerformanceOrderingClause
  : K_ORDERING
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 15. OPEN PERFORMANCE PROPERTY
* ============================================================================
* 
* Future performance properties use qualified symbolic names.
* 
* Examples:
* 
* throughput = desired_throughput;
* 
* latency = latency_budget;
* 
* energy_efficiency = target_efficiency;
* 
* accelerator::occupancy = desired_occupancy;
* 
* quantum::fidelity = desired_fidelity;
* 
* future::metric = target;
* 
* The grammar does not close the property vocabulary.
  */
  resourcePerformancePropertyClause
  : resourcePerformancePropertyName
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 16. PERFORMANCE METRIC
* ============================================================================
* 
* A metric is an open-world symbolic name.
* 
* The metric is not interpreted by the parser.
  */
  resourcePerformanceMetric
  : resourcePerformancePropertyName
  ;

/*

* ============================================================================
* 17. PERFORMANCE PROPERTY NAME
* ============================================================================
* 
* Names are intentionally open-world.
* 
* Both ordinary and qualified names are supported.
* 
* Namespace depth is unbounded by the grammar.
  /
  resourcePerformancePropertyName
  : identifier
  (
  DOT identifier
  | DOUBLE_COLON identifier
  )
  ;

/*

* ============================================================================
* 18. PERFORMANCE EXPRESSION
* ============================================================================
* 
* This is a semantic boundary around the canonical resource expression.
* 
* NO expression language is redefined here.
  */
  resourcePerformanceExpression
  : resourceExpression
  ;

/*

* ============================================================================
* 19. PERFORMANCE RELATION
* ============================================================================
* 
* The relation is explicitly represented so a metric can be compared against
* a target without introducing another comparison grammar.
  */
  resourcePerformanceRelation
  : resourcePerformanceExpression
  resourcePerformanceOperator
  resourcePerformanceExpression
  ;

/*

* ============================================================================
* 20. PERFORMANCE OPERATOR
* ============================================================================
* 
* Operators come from the canonical lexer.
  */
  resourcePerformanceOperator
  : EQ_EQ
  | NOT_EQ
  | LE
  | GE
  | LESS
  | GREATER
  ;

/*

* ============================================================================
* 21. PERFORMANCE GROUP
* ============================================================================
* 
* Groups allow arbitrary numbers of performance clauses.
* 
* Example:
* 
* performance group execution {
*     objective = throughput >= desired_throughput;
*     objective = latency <= latency_budget;
*     priority = performance_priority;
* }

/
resourcePerformanceGroup
: K_PERFORMANCE
K_GROUP
resourcePerformanceGroupName
LBRACE
resourcePerformanceClause
RBRACE
SEMICOLON?
;

/*

* ============================================================================
* 22. PERFORMANCE GROUP NAME
* ============================================================================
  */

resourcePerformanceGroupName
: resourcePerformancePropertyName
;

/*

* ============================================================================
* 23. PERFORMANCE OBJECTIVE LIST
* ============================================================================
* 
* Arbitrary objective cardinality.
  /
  resourcePerformanceObjectiveList
  : resourcePerformanceExpression
  (
  COMMA resourcePerformanceExpression
  )
  COMMA?
  ;

/*

* ============================================================================
* 24. PERFORMANCE PROPERTY LIST
* ============================================================================
* 
* Arbitrary property cardinality.
  /
  resourcePerformancePropertyList
  : resourcePerformanceClause
  ;

/*

* ============================================================================
* 25. OPTIONAL PERFORMANCE SPECIFICATION
* ============================================================================
  */

optionalResourcePerformanceSpecification
: resourcePerformanceSpecification?
;

/*

* ============================================================================
* 26. PERFORMANCE CONDITION
* ============================================================================
* 
* Explicit condition boundary for parent resource grammars.
  */
  resourcePerformanceCondition
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 27. PERFORMANCE TARGET EXPRESSION
* ============================================================================
  */

resourcePerformanceTargetExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 28. PERFORMANCE BUDGET EXPRESSION
* ============================================================================
  */

resourcePerformanceBudgetExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 29. PERFORMANCE PRIORITY EXPRESSION
* ============================================================================
  */

resourcePerformancePriorityExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 30. PERFORMANCE WEIGHT EXPRESSION
* ============================================================================
  */

resourcePerformanceWeightExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 31. PERFORMANCE AGGREGATION EXPRESSION
* ============================================================================
  */

resourcePerformanceAggregationExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 32. PERFORMANCE ORDERING EXPRESSION
* ============================================================================
  */

resourcePerformanceOrderingExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 33. PERFORMANCE METRIC VALUE
* ============================================================================
* 
* A metric value may be computed from arbitrary resource expressions.
  */
  resourcePerformanceMetricValue
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 34. PERFORMANCE RELATION LIST
* ============================================================================
* 
* Useful to parent grammars that need several relations.
  /
  resourcePerformanceRelationList
  : resourcePerformanceRelation
  (
  COMMA resourcePerformanceRelation
  )
  COMMA?
  ;

/*

* ============================================================================
* 35. PERFORMANCE PROPERTY PATH
* ============================================================================
* 
* Qualified symbolic properties remain open-world.
  */
  resourcePerformancePropertyPath
  : resourcePerformancePropertyName
  ;

/*

* ============================================================================
* 36. PERFORMANCE VALUE LIST
* ============================================================================
  */

resourcePerformanceValueList
: resourcePerformanceExpression
(
COMMA resourcePerformanceExpression
)*
COMMA?
;

/*

* ============================================================================
* 37. PERFORMANCE EXTENSION
* ============================================================================
* 
* Future domains may attach performance properties without changing this
* grammar.
  */
  resourcePerformanceExtension
  : resourcePerformancePropertyName
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 38. PERFORMANCE CONTRACT
* ============================================================================
* 
* A contract is still a syntax boundary.
* 
* Whether the contract is mandatory, preferred, or advisory is determined by
* the parent semantic model.
  /
  resourcePerformanceContract
  : K_PERFORMANCE
  LBRACE
  resourcePerformanceClause
  RBRACE
  SEMICOLON?
  ;

/*

* ============================================================================
* 39. PERFORMANCE OBJECTIVE RELATION
* ============================================================================
* 
* Explicit semantic boundary for downstream consumers.
  */
  resourcePerformanceObjectiveRelation
  : resourcePerformanceRelation
  ;

/*

* ============================================================================
* 40. PERFORMANCE SCALABILITY EXPRESSION
* ============================================================================
* 
* Scaling behavior remains symbolic.
* 
* Examples:
* 
* performance = workload_size;
* 
* scalability = workload_size / available_parallelism;
* 
* throughput = problem_size / execution_time;
* 
* No finite machine size is implied.
  */
  resourcePerformanceScalabilityExpression
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 41. PERFORMANCE EFFICIENCY EXPRESSION
* ============================================================================
  */

resourcePerformanceEfficiencyExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 42. PERFORMANCE THROUGHPUT EXPRESSION
* ============================================================================
  */

resourcePerformanceThroughputExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 43. PERFORMANCE LATENCY EXPRESSION
* ============================================================================
  */

resourcePerformanceLatencyExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 44. PERFORMANCE BANDWIDTH EXPRESSION
* ============================================================================
  */

resourcePerformanceBandwidthExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 45. PERFORMANCE RESPONSIVENESS EXPRESSION
* ============================================================================
  */

resourcePerformanceResponsivenessExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 46. PERFORMANCE UTILIZATION EXPRESSION
* ============================================================================
  */

resourcePerformanceUtilizationExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 47. PERFORMANCE RESOURCE EXPRESSION
* ============================================================================
* 
* This rule provides a stable boundary for resource-aware performance
* semantics without introducing resource identity or allocation semantics.
  */
  resourcePerformanceResourceExpression
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 48. PERFORMANCE TARGET RESOURCE EXPRESSION
* ============================================================================
  */

resourcePerformanceTargetResourceExpression
: resourcePerformanceExpression
;

/*

* ============================================================================
* 49. PERFORMANCE CONDITIONAL GROUP
* ============================================================================
* 
* Conditional semantics remain declarative.
  /
  resourcePerformanceConditionalGroup
  : K_PERFORMANCE
  K_GROUP
  resourcePerformanceGroupName
  LBRACE
  K_WHEN
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  resourcePerformanceClause
  RBRACE
  SEMICOLON?
  ;

/*

* ============================================================================
* 50. PERFORMANCE ATTRIBUTE-LIKE EXTENSION
* ============================================================================
* 
* Open property mechanism for future performance systems.
  */
  resourcePerformanceAttribute
  : resourcePerformancePropertyName
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 51. PERFORMANCE LIST
* ============================================================================
  */

resourcePerformanceList
: resourcePerformanceItem
resourcePerformanceItem*
;

/*

* ============================================================================
* 52. OPTIONAL PERFORMANCE LIST
* ============================================================================
  */

optionalResourcePerformanceList
: resourcePerformanceList?
;

/*

* ============================================================================
* 53. PERFORMANCE EXPRESSION LIST
* ============================================================================
  */

resourcePerformanceExpressionList
: resourcePerformanceExpression
(
COMMA resourcePerformanceExpression
)*
COMMA?
;

/*

* ============================================================================
* 54. PERFORMANCE SEMANTIC BOUNDARY
* ============================================================================
* 
* This rule exists for parent grammars that need to explicitly state that a
* value is intended to be interpreted as performance.
  */
  resourcePerformanceSemanticValue
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 55. PERFORMANCE MEASUREMENT REFERENCE
* ============================================================================
* 
* A measurement reference is symbolic.
* 
* It does not execute measurement.
* 
* Examples:
* 
* measured_throughput
* 
* observed_latency
* 
* benchmark_result
* 
* runtime_performance

*/
resourcePerformanceMeasurementReference
: resourcePerformanceExpression
;

/*

* ============================================================================
* 56. PERFORMANCE OBSERVATION
* ============================================================================
* 
* Runtime or compiler observations may be represented symbolically.
* 
* The parser never obtains the observation itself.
  */
  resourcePerformanceObservation
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 57. PERFORMANCE BASELINE
* ============================================================================
* 
* Baselines remain symbolic and are interpreted by semantic analysis or
* performance tooling.
  */
  resourcePerformanceBaseline
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 58. PERFORMANCE DELTA
* ============================================================================
* 
* A performance delta is represented as an expression.
  */
  resourcePerformanceDelta
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 59. PERFORMANCE SCORE
* ============================================================================
* 
* Score semantics are deliberately not restricted to a particular numeric
* domain.
  */
  resourcePerformanceScore
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 60. PERFORMANCE RATIO
* ============================================================================
  */

resourcePerformanceRatio
: resourcePerformanceExpression
;

/*

* ============================================================================
* 61. PERFORMANCE OBJECTIVE VALUE
* ============================================================================
  */

resourcePerformanceObjectiveValue
: resourcePerformanceExpression
;

/*

* ============================================================================
* 62. PERFORMANCE PROPERTY VALUE
* ============================================================================
  */

resourcePerformancePropertyValue
: resourcePerformanceExpression
;

/*

* ============================================================================
* 63. PERFORMANCE NORMALIZATION
* ============================================================================
* 
* Normalization is semantic intent, not computation performed by the parser.
  */
  resourcePerformanceNormalization
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 64. PERFORMANCE TOLERANCE
* ============================================================================
* 
* Tolerance is intentionally symbolic.
* 
* The grammar imposes no fixed tolerance range.
  */
  resourcePerformanceTolerance
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 65. PERFORMANCE VARIABILITY
* ============================================================================
  */

resourcePerformanceVariability
: resourcePerformanceExpression
;

/*

* ============================================================================
* 66. PERFORMANCE DISTRIBUTION REFERENCE
* ============================================================================
* 
* Distribution semantics are interpreted downstream.
  */
  resourcePerformanceDistribution
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 67. PERFORMANCE PERCENTILE
* ============================================================================
* 
* Percentile expressions are intentionally generic.
* 
* The grammar does not impose a fixed percentile domain.
  */
  resourcePerformancePercentile
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 68. PERFORMANCE QUANTILE
* ============================================================================
  */

resourcePerformanceQuantile
: resourcePerformanceExpression
;

/*

* ============================================================================
* 69. PERFORMANCE WINDOW
* ============================================================================
  */

resourcePerformanceWindow
: resourcePerformanceExpression
;

/*

* ============================================================================
* 70. PERFORMANCE SAMPLE
* ============================================================================
  */

resourcePerformanceSample
: resourcePerformanceExpression
;

/*

* ============================================================================
* 71. PERFORMANCE TIME CONTEXT
* ============================================================================
* 
* Time/duration syntax remains owned by the canonical literal/expression
* architecture.
  */
  resourcePerformanceTimeContext
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 72. PERFORMANCE WORKLOAD CONTEXT
* ============================================================================
  */

resourcePerformanceWorkloadContext
: resourcePerformanceExpression
;

/*

* ============================================================================
* 73. PERFORMANCE SCALING CONTEXT
* ============================================================================
  */

resourcePerformanceScalingContext
: resourcePerformanceExpression
;

/*

* ============================================================================
* 74. PERFORMANCE RESOURCE CONTEXT
* ============================================================================
  */

resourcePerformanceResourceContext
: resourcePerformanceExpression
;

/*

* ============================================================================
* 75. PERFORMANCE TARGET CONTEXT
* ============================================================================
  */

resourcePerformanceTargetContext
: resourcePerformanceExpression
;

/*

* ============================================================================
* 76. PERFORMANCE EXTENSION PROPERTY
* ============================================================================
* 
* This is the explicit extension boundary for future domains and dialects.
  */
  resourcePerformanceExtensionProperty
  : resourcePerformancePropertyName
  ASSIGN
  resourcePerformanceExpression
  SEMICOLON
  ;

/*

* ============================================================================
* 77. PERFORMANCE DECLARATIVE VALUE
* ============================================================================
  */

resourcePerformanceDeclarativeValue
: resourcePerformanceExpression
;

/*

* ============================================================================
* 78. PERFORMANCE POLICY VALUE
* ============================================================================
* 
* Policy semantics belong downstream.
  */
  resourcePerformancePolicyValue
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 79. PERFORMANCE OPTIMIZATION VALUE
* ============================================================================
* 
* Optimization may consume this value, but optimization is not implemented
* here.
  */
  resourcePerformanceOptimizationValue
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 80. PERFORMANCE SCHEDULING VALUE
* ============================================================================
* 
* Scheduling may consume performance intent, but scheduling is not implemented
* here.
  */
  resourcePerformanceSchedulingValue
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 81. PERFORMANCE RUNTIME VALUE
* ============================================================================
* 
* Runtime systems may evaluate this expression.
  */
  resourcePerformanceRuntimeValue
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 82. PERFORMANCE CAPABILITY VALUE
* ============================================================================
* 
* Capability resolution belongs to semantic analysis and the capability model.
  */
  resourcePerformanceCapabilityValue
  : resourcePerformanceExpression
  ;

/*

* ============================================================================
* 83. PERFORMANCE TARGET VALUE
* ============================================================================
  */

resourcePerformanceTargetValue
: resourcePerformanceExpression
;

/*

* ============================================================================
* 84. PERFORMANCE RESOURCE VALUE
* ============================================================================
  */

resourcePerformanceResourceValue
: resourcePerformanceExpression
;

/*

* ============================================================================
* 85. PERFORMANCE CONTRACT VALUE
* ============================================================================
  */

resourcePerformanceContractValue
: resourcePerformanceExpression
;

/*

* ============================================================================
* 86. PERFORMANCE EXPRESSION CONTRACT
* ============================================================================
* 
* All performance expressions ultimately delegate to resourceExpression.
* 
* This is the critical anti-duplication boundary.
  */
  resourcePerformanceExpressionContract
  : resourceExpression
  ;

/*

* ============================================================================
* 87. PERFORMANCE HARD-CODING BOUNDARY
* ============================================================================
* 
* This grammar MUST NOT introduce:
* 
* MAX_PERFORMANCE
* MAX_THROUGHPUT
* MAX_LATENCY
* MAX_BANDWIDTH
* MAX_UTILIZATION
* MAX_EFFICIENCY
* MAX_RESOURCES
* 
* Nor may it introduce machine-specific constants such as:
* 
* cpu_count
* gpu_count
* qpu_count
* device_id
* topology_id
* 
* as grammar-level limitations.
* 
* Symbolic expressions remain valid regardless of scale.
  */

/*

* ============================================================================
* 88. QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum-specific performance intent may refer semantically to:
* 
* quantum throughput;
* circuit latency;
* measurement latency;
* fidelity;
* logical performance;
* error-correction overhead;
* execution duration;
* shot throughput;
* compilation overhead.
* 
* This grammar MUST NOT define:
* 
* quantum gates;
* qubits;
* physical qubits;
* logical qubits;
* QEC algorithms;
* noise channels;
* quantum::ir.
* 
* Quantum semantic lowering remains responsible for associating performance
* intent with canonical quantum semantics.
  */

/*

* ============================================================================
* 89. HARDWARE / HDL INTEGRATION
* ============================================================================
* 
* Hardware and HDL domains may consume performance intent for:
* 
* latency;
* throughput;
* clock-related objectives;
* pipeline performance;
* memory throughput;
* interface bandwidth;
* accelerator utilization;
* energy/performance trade-offs.
* 
* This grammar MUST NOT define:
* 
* clocks;
* wires;
* ports;
* pipeline stages;
* physical devices;
* FPGA resources;
* ASIC implementation.
* 
* Those belong to their respective domain grammars.
  */

/*

* ============================================================================
* 90. CLASSICAL / AI / DISTRIBUTED INTEGRATION
* ============================================================================
* 
* The same performance grammar can describe intent for:
* 
* scalar computation;
* vector computation;
* matrix computation;
* tensor computation;
* AI training;
* AI inference;
* distributed execution;
* networking;
* storage;
* embedded systems;
* HPC;
* future computational domains.
* 
* Domain-specific semantics remain outside this grammar.
  */

/*

* ============================================================================
* 91. OPTIMIZATION INTEGRATION
* ============================================================================
* 
* Optimization may consume:
* 
* objective;
* target;
* bound;
* budget;
* priority;
* weight;
* ordering;
* aggregation.
* 
* Optimization decides how to act on those semantics.
* 
* This grammar does not implement optimization.
* 
* A performance preference MUST NOT be silently promoted to a hard
* requirement by the parser.
  */

/*

* ============================================================================
* 92. SCHEDULING INTEGRATION
* ============================================================================
* 
* Scheduling may consume:
* 
* latency intent;
* throughput intent;
* workload expressions;
* resource-performance relationships;
* performance budgets.
* 
* Scheduling algorithms remain outside this grammar.
  */

/*

* ============================================================================
* 93. RESOURCE-MANAGEMENT INTEGRATION
* ============================================================================
* 
* Resource management may use performance intent to select among feasible
* resource realizations.
* 
* The parser does not allocate or reserve resources.
  */

/*

* ============================================================================
* 94. HARDWARE-HAL INTEGRATION
* ============================================================================
* 
* Hardware HAL supplies capabilities and observations.
* 
* Performance grammar may reference those values symbolically.
* 
* It does not query the HAL.
  */

/*

* ============================================================================
* 95. RUNTIME INTEGRATION
* ============================================================================
* 
* Runtime systems may evaluate dynamic performance observations against
* semantic objectives.
* 
* Runtime state MUST NOT influence parsing.
  */

/*

* ============================================================================
* 96. QEC / ZQN INTEGRATION
* ============================================================================
* 
* QEC owns error-correction algorithms.
* 
* ZQN owns noise and fault semantics.
* 
* Performance intent may refer semantically to:
* 
* logical error performance;
* fidelity-related objectives;
* error-correction overhead;
* reliability-adjusted performance.
* 
* This grammar does not define QEC or ZQN semantics.
  */

/*

* ============================================================================
* 97. SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis must preserve:
* 
* performance construct identity;
* metric identity;
* performance relation;
* expression;
* scope;
* priority;
* weight;
* condition;
* aggregation;
* ordering;
* extensible properties.
* 
* Semantic analysis must additionally determine:
* 
* metric meaning;
* value type;
* units/dimensions;
* evaluation phase;
* whether an objective is mandatory or advisory;
* feasibility;
* compatibility with the selected target;
* conflict with other objectives.
* 
* None of these decisions belong to parsing.
  */

/*

* ============================================================================
* 98. AST CONTRACT
* ============================================================================
* 
* The parser must produce syntax information sufficient for a frontend AST to
* represent performance intent.
* 
* The AST should preserve source spans and the original symbolic names.
* 
* The grammar MUST NOT define Rust AST structures.
  */

/*

* ============================================================================
* 99. IR CONTRACT
* ============================================================================
* 
* This grammar does not define an IR.
* 
* Performance intent is lowered by semantic analysis into the repository's
* canonical semantic resource model.
* 
* If the performance intent affects quantum compilation, the resulting
* semantics may eventually influence quantum::ir consumers.
* 
* The grammar itself MUST NOT depend on quantum::ir.
  */

/*

* ============================================================================
* 100. DETERMINISM CONTRACT
* ============================================================================
* 
* For the same token stream, parser behavior must be deterministic.
* 
* Parsing must not depend on:
* 
* hardware;
* runtime state;
* resource availability;
* network state;
* filesystem contents;
* benchmark results.

*/

/*

* ============================================================================
* 101. ERROR BOUNDARY
* ============================================================================
* 
* Syntax errors belong to parsing:
* 
* missing metric;
* malformed relation;
* malformed property name;
* missing expression;
* malformed group;
* malformed clause;
* malformed list.
* 
* Semantic errors belong downstream:
* 
* unknown metric;
* incompatible units;
* invalid comparison;
* impossible target;
* unsatisfied hard performance condition;
* unsupported performance property;
* incompatible aggregation policy.

*/

/*

* ============================================================================
* 102. SECURITY BOUNDARY
* ============================================================================
* 
* No parser action in this grammar may:
* 
* execute code;
* invoke plugins;
* access hardware;
* inspect runtime state;
* access files;
* access networks;
* allocate resources.
* 
* This grammar contains no embedded Rust actions.
  */

/*

* ============================================================================
* 103. TEST CONTRACT
* ============================================================================
* 
* POSITIVE TESTS
* 
* Must cover:
* 
* performance declarations;
* structured performance specifications;
* performance groups;
* objectives;
* targets;
* bounds;
* budgets;
* priorities;
* weights;
* conditions;
* aggregations;
* ordering;
* open properties;
* qualified properties;
* arbitrary resource expressions;
* symbolic values;
* numeric values;
* computed values;
* nested expressions.
* 
* 
* NEGATIVE TESTS
* 
* Must cover:
* 
* missing performance metric;
* missing relation;
* missing expression;
* malformed property path;
* malformed qualified name;
* malformed group;
* malformed clause;
* missing semicolon;
* malformed comparison;
* empty group clause where syntax requires a value.
* 
* 
* BOUNDARY TESTS
* 
* Must cover:
* 
* one performance objective;
* many objectives;
* deeply qualified property names;
* large expression lists;
* large performance groups;
* nested symbolic expressions;
* very large numeric literals where supported by the canonical lexer;
* symbolic workload sizes;
* symbolic scaling values.
* 
* 
* SCALABILITY TESTS
* 
* Must verify that the grammar contains no fixed:
* 
* resource count;
* device count;
* CPU count;
* GPU count;
* FPGA count;
* QPU count;
* node count;
* qubit count;
* memory limit;
* objective count;
* group size;
* namespace depth.
* 
* 
* CROSS-DOMAIN TESTS
* 
* Must cover performance intent attached to:
* 
* classical;
* quantum;
* hybrid;
* HDL;
* hardware;
* distributed;
* AI/ML;
* networking;
* storage;
* embedded;
* HPC;
* accelerators.
* 
* 
* DETERMINISM TESTS
* 
* The same token stream must produce equivalent parse trees independent of
* runtime environment.
  */

/*

* ============================================================================
* 104. COMPATIBILITY CONTRACT
* ============================================================================
* 
* This grammar must remain compatible with:
* 
* Rust 1.97;
* Rust 1.97.1;
* Rust 2021;
* ANTLR4;
* the repository's canonical lexer;
* ResourceExpressions;
* resources.g4;
* resource requirements;
* resource constraints;
* resource capabilities;
* resource preferences.
* 
* Existing valid syntax must not be silently reinterpreted as a different
* semantic category.
  */

/*

* ============================================================================
* 105. HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden:
* 
* MAX_PERFORMANCE
* MAX_OBJECTIVES
* MAX_METRICS
* MAX_GROUPS
* MAX_THROUGHPUT
* MAX_LATENCY
* MAX_BANDWIDTH
* MAX_EFFICIENCY
* MAX_UTILIZATION
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_DEVICES
* 
* Forbidden:
* 
* physical device identifiers;
* fixed hardware addresses;
* fixed topology;
* fixed machine dimensions;
* provider-specific implementation assumptions.
* 
* All scalable quantities remain expressions.
  */

/*

* ============================================================================
* 106. RUST SAFETY CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* no Rust actions;
* no Rust semantic predicates;
* no unsafe code;
* no external execution;
* no filesystem operations;
* no network operations.
* 
* Generated parser/compiler integration must remain safe Rust and target
* Rust 1.97 / Rust 1.97.1.
  */

/*

* ============================================================================
* 107. COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [ ] The repository's ANTLR build accepts ResourcePerformance.
* 
* [ ] ResourceExpressions resolves successfully.
* 
* [ ] The canonical Zamani lexer supplies every referenced token.
* 
* [ ] "resourceExpression" resolves to the canonical resource-expression
* boundary.
* 
* [ ] No general expression grammar is duplicated.
* 
* [ ] No machine-specific resource limit is encoded.
* 
* [ ] No performance-specific fixed numeric range is encoded.
* 
* [ ] Performance metrics remain open-world.
* 
* [ ] Qualified performance property names remain extensible.
* 
* [ ] Performance objectives remain distinct from requirements.
* 
* [ ] Performance objectives remain distinct from constraints.
* 
* [ ] Performance values remain expressions.
* 
* [ ] Performance groups have no finite cardinality limit.
* 
* [ ] Performance property namespace depth has no grammar-level limit.
* 
* [ ] The grammar performs no benchmarking.
* 
* [ ] The grammar performs no hardware discovery.
* 
* [ ] The grammar performs no resource allocation.
* 
* [ ] The grammar performs no runtime queries.
* 
* [ ] The grammar remains independent of quantum::ir.
* 
* [ ] Quantum semantics can consume performance intent downstream.
* 
* [ ] Hardware/HDL semantics can consume performance intent downstream.
* 
* [ ] Classical, AI, distributed, networking and accelerator domains can
* consume performance intent downstream.
* 
* [ ] Resource optimization can consume performance intent without changing
* its semantic category.
* 
* [ ] Scheduling can consume performance intent without being implemented by
* this grammar.
* 
* [ ] Resource management can consume performance intent without parser-level
* allocation.
* 
* [ ] Positive tests pass.
* 
* [ ] Negative tests pass.
* 
* [ ] Boundary tests pass.
* 
* [ ] Cross-domain tests pass.
* 
* [ ] Scalability tests pass.
* 
* [ ] Determinism tests pass.
* 
* [ ] Rust integration remains safe and compatible with Rust 1.97/1.97.1.
* 
* ============================================================================
  */