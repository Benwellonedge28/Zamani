/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/preferences.g4
 *
 * GRAMMAR
 * -------
 * ResourcePreferences
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-PREFERENCE LEAF GRAMMAR
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
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no parser actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware discovery;
 *   - no resource allocation;
 *   - no runtime execution;
 *   - no unsafe Rust requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-SYNTAX LEAF CONTRACT for RESOURCE PREFERENCES.
 *
 * A resource preference expresses desirable realization characteristics.
 *
 * A preference is NOT a requirement.
 *
 * A preference is NOT a constraint.
 *
 * A preference is NOT a hint.
 *
 * A preference MAY be traded off by downstream optimization/resource-selection
 * machinery according to the semantic policy of the program, compilation
 * profile, deployment environment, or execution environment.
 *
 * This grammar therefore describes:
 *
 *     WHAT IS PREFERRED
 *
 * and never:
 *
 *     WHICH PHYSICAL RESOURCE MUST BE USED.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Resources
 *          |
 *          v
 *     ResourcePreferences
 *          |
 *          v
 *     Domain-neutral AST
 *          |
 *          v
 *     semantic resource model
 *          |
 *     +----+---------+----------+-------------+
 *     |              |          |             |
 *     v              v          v             v
 *  compiler      optimizer   scheduler      runtime
 *     |              |          |             |
 *     +--------------+----------+-------------+
 *                    |
 *                    v
 *             target realization
 *                    |
 *       +------------+------------+
 *       |            |            |
 *       v            v            v
 *      CPU          GPU          QPU
 *       |            |            |
 *      FPGA       accelerator   future target
 *
 * This file is upstream of all physical realization.
 *
 * ============================================================================
 * 3. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * RESOURCE PREFERENCE OWNERSHIP IS DELIBERATELY SPLIT.
 *
 * grammar/resources/resources.g4
 *     owns the concrete universal resource statement:
 *
 *         prefer <resource-expression> ;
 *
 *     and the resource-domain composition boundary.
 *
 * grammar/resources/preferences.g4
 *     owns the reusable preference payload/block syntax:
 *
 *         resourcePreferenceSpecification
 *         resourcePreferenceClause
 *         resourcePreferenceProperty
 *         resourcePreferenceValue
 *         resourcePreferenceCondition
 *         resourcePreferenceObjective
 *         resourcePreferenceOrdering
 *         resourcePreferenceMetadata
 *         resourcePreferenceGroupBody
 *
 * grammar/resources/resource-expressions.g4
 *     owns:
 *
 *         resourceExpression
 *
 * grammar/core/names.g4
 *     owns:
 *
 *         identifier
 *         qualifiedName
 *
 * This file MUST NOT redefine:
 *
 *         resourcePreference
 *         resourcePreferenceExpression
 *         resourceExpression
 *         expression
 *         identifier
 *         qualifiedName
 *
 * This prevents circular or duplicate ownership.
 *
 * ============================================================================
 * 4. IMPORTS
 * ============================================================================
 *
 * ResourceExpressions is imported because every preference value and condition
 * must use the canonical Zamani resource-expression architecture.
 *
 * Names is imported because preference property names and group names use the
 * canonical name system.
 *
 * ============================================================================
 */

parser grammar ResourcePreferences;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 5. PUBLIC REUSABLE PREFERENCE PAYLOAD
 * ============================================================================
 *
 * This is the primary public entry point owned by this file.
 *
 * A parent grammar may use:
 *
 *     resourcePreferenceSpecification
 *
 * after its own preference introducer.
 *
 * Example intended composition:
 *
 *     prefer {
 *         objective = latency <= latency_budget;
 *         priority = latency_priority;
 *         weight = latency_weight;
 *     };
 *
 * The concrete `prefer` statement remains owned by Resources.
 *
 * ============================================================================
 */

resourcePreferenceSpecification
    : LBRACE
      resourcePreferenceClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. PREFERENCE CLAUSE
 * ============================================================================
 *
 * Preference clauses are intentionally open-world.
 *
 * The language therefore does not need a new lexer keyword whenever a new
 * optimization dimension is introduced.
 *
 * Examples:
 *
 *     objective = latency <= latency_budget;
 *     priority = priority_value;
 *     weight = preference_weight;
 *     scope = execution_region;
 *     when = workload_size > threshold;
 *     tie_breaker = energy <= energy_budget;
 *     portability = portability_goal;
 *     scalability = scalability_goal;
 *     metadata::origin = developer;
 *     vendor::accelerator::metric = desired_value;
 *
 * Semantic analysis determines whether a property is:
 *
 *     - standard;
 *     - experimental;
 *     - dialect-specific;
 *     - vendor-specific;
 *     - deprecated;
 *     - unknown.
 *
 * ============================================================================
 */

resourcePreferenceClause
    : resourcePreferenceObjective
    | resourcePreferenceOrdering
    | resourcePreferenceCondition
    | resourcePreferenceMetadata
    | resourcePreferencePropertyAssignment
    ;


/*
 * ============================================================================
 * 7. GENERIC PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * This is the open-world extension point.
 *
 * It deliberately does not enumerate:
 *
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     cost
 *     portability
 *     scalability
 *
 * Those concepts may be represented by symbolic property names and interpreted
 * by semantic analysis.
 *
 * This avoids turning every optimization dimension into a parser keyword.
 * ============================================================================
 */

resourcePreferencePropertyAssignment
    : resourcePreferenceProperty
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. OBJECTIVE
 * ============================================================================
 *
 * An objective identifies the expression being preferred.
 *
 * The value remains a canonical resource expression.
 *
 * Examples:
 *
 *     objective = latency <= latency_budget;
 *
 *     objective = throughput >= desired_throughput;
 *
 *     objective = energy <= energy_budget;
 *
 *     objective = capability("tensor.compute");
 *
 * The grammar does not decide whether the expression is actually a valid
 * objective. That is semantic analysis.
 * ============================================================================
 */

resourcePreferenceObjective
    : resourcePreferenceObjectiveName
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


resourcePreferenceObjectiveName
    : OBJECTIVE
    | identifier
    ;


/*
 * ============================================================================
 * 9. ORDERING / PRIORITY / WEIGHT
 * ============================================================================
 *
 * Ordering metadata expresses how preferences may be considered relative to
 * other preferences.
 *
 * The grammar deliberately accepts expressions rather than a closed numeric
 * domain.
 *
 * This allows the semantic layer to determine whether a value represents:
 *
 *     - an integer priority;
 *     - a real-valued weight;
 *     - a symbolic optimization class;
 *     - a domain-specific ordering;
 *     - another future representation.
 *
 * No source-level maximum or minimum exists.
 * ============================================================================
 */

resourcePreferenceOrdering
    : resourcePreferenceOrderingName
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


resourcePreferenceOrderingName
    : PRIORITY
    | WEIGHT
    | ORDER
    | identifier
    ;


/*
 * ============================================================================
 * 10. CONDITIONAL PREFERENCE
 * ============================================================================
 *
 * `when` is represented as a property assignment rather than a separate
 * executable control-flow construct.
 *
 * Example:
 *
 *     when = workload_size > threshold;
 *
 * The expression is declarative.
 *
 * Parsing it MUST NOT evaluate it.
 *
 * Semantic analysis determines:
 *
 *     - whether it is boolean;
 *     - which symbols it references;
 *     - which scope it applies to;
 *     - whether the referenced information is available.
 * ============================================================================
 */

resourcePreferenceCondition
    : resourcePreferenceConditionName
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


resourcePreferenceConditionName
    : WHEN
    | identifier
    ;


/*
 * ============================================================================
 * 11. PREFERENCE PROPERTY
 * ============================================================================
 *
 * Preference properties use the canonical open-world naming model.
 *
 * Supported examples include:
 *
 *     latency
 *     throughput
 *     energy
 *     reliability
 *     portability
 *     scalability
 *     performance::latency
 *     quantum::fidelity
 *     accelerator::throughput
 *     vendor::future_metric
 *
 * Namespace depth is unbounded by the language architecture.
 *
 * The parser does not decide whether a property is known.
 * ============================================================================
 */

resourcePreferenceProperty
    : qualifiedPreferenceName
    ;


qualifiedPreferenceName
    : preferenceNameSegment
      (
          DOT preferenceNameSegment
        | DOUBLE_COLON preferenceNameSegment
      )*
    ;


preferenceNameSegment
    : identifier
    ;


/*
 * ============================================================================
 * 12. PREFERENCE VALUE
 * ============================================================================
 *
 * Every value is a canonical resource expression.
 *
 * This means preference syntax inherits the same:
 *
 *     arithmetic;
 *     comparison;
 *     logical;
 *     call;
 *     indexing;
 *     member-access;
 *     literal;
 *     generic expression
 *
 * architecture as the rest of Zamani.
 *
 * This file MUST NOT create a second expression language.
 * ============================================================================
 */

resourcePreferenceValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 13. PREFERENCE METADATA
 * ============================================================================
 *
 * Metadata is syntactically identical to an open-world property assignment.
 *
 * Semantic analysis decides whether metadata affects:
 *
 *     optimization;
 *     diagnostics;
 *     provenance;
 *     tooling;
 *     compatibility;
 *     deployment policy.
 *
 * Metadata MUST NOT silently change a preference into a requirement.
 * ============================================================================
 */

resourcePreferenceMetadata
    : resourcePreferenceMetadataName
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


resourcePreferenceMetadataName
    : qualifiedPreferenceName
    ;


/*
 * ============================================================================
 * 14. PREFERENCE GROUP BODY
 * ============================================================================
 *
 * A group contains an arbitrary number of preference clauses.
 *
 * No finite group size is encoded.
 *
 * The group itself is a semantic collection. It is not a scheduling group,
 * execution group, hardware group, or physical device group.
 *
 * ============================================================================
 */

resourcePreferenceGroupBody
    : LBRACE
      resourcePreferenceClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 15. PREFERENCE GROUP ENTRY
 * ============================================================================
 *
 * This reusable boundary allows a parent resource grammar to attach its own
 * group introducer while keeping the body owned here.
 *
 * The concrete group statement remains outside this file.
 * ============================================================================
 */

resourcePreferenceGroupEntry
    : resourcePreferenceGroupName
      resourcePreferenceGroupBody
    ;


resourcePreferenceGroupName
    : qualifiedPreferenceName
    ;


/*
 * ============================================================================
 * 16. PREFERENCE CONDITION VALUE
 * ============================================================================
 *
 * Explicit semantic boundary for consumers that need to distinguish a
 * condition from an ordinary preference value.
 *
 * No boolean grammar is duplicated.
 * ============================================================================
 */

resourcePreferenceConditionValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 17. PREFERENCE OBJECTIVE VALUE
 * ============================================================================
 */

resourcePreferenceObjectiveValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 18. PREFERENCE ORDERING VALUE
 * ============================================================================
 */

resourcePreferenceOrderingValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 19. PREFERENCE METADATA VALUE
 * ============================================================================
 */

resourcePreferenceMetadataValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 20. PREFERENCE LIST
 * ============================================================================
 *
 * Lists are unbounded at the language level.
 *
 * The implementation may impose operational resource limits when parsing or
 * compiling hostile/oversized input, but those are implementation safeguards,
 * not language semantics.
 * ============================================================================
 */

resourcePreferenceClauseList
    : resourcePreferenceClause*
    ;


/*
 * ============================================================================
 * 21. OPTIONAL PREFERENCE SPECIFICATION
 * ============================================================================
 */

optionalResourcePreferenceSpecification
    : resourcePreferenceSpecification?
    ;


/*
 * ============================================================================
 * 22. PREFERENCE GROUP LIST
 * ============================================================================
 */

resourcePreferenceGroupList
    : resourcePreferenceGroupEntry*
    ;


/*
 * ============================================================================
 * 23. RESOURCE PREFERENCE EXTENSION
 * ============================================================================
 *
 * An extension is deliberately represented through the same open-world
 * property mechanism.
 *
 * This supports future domains without modifying the universal preference
 * grammar merely because a new resource type appears.
 *
 * Examples:
 *
 *     quantum::fidelity = target_fidelity;
 *     gpu::occupancy = desired_occupancy;
 *     fpga::throughput = desired_throughput;
 *     future::resource::metric = desired_metric;
 *
 * The grammar does not know whether those properties exist.
 * ============================================================================
 */

resourcePreferenceExtension
    : qualifiedPreferenceName
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. PREFERENCE CONTRACT BODY
 * ============================================================================
 *
 * A contract is only a grouping mechanism here.
 *
 * It does NOT convert preferences into requirements.
 *
 * The semantic layer preserves:
 *
 *     requirement != constraint != preference != hint
 *
 * ============================================================================
 */

resourcePreferenceContractBody
    : resourcePreferenceSpecification
    ;


/*
 * ============================================================================
 * 25. PREFERENCE SCOPE VALUE
 * ============================================================================
 *
 * Scope remains symbolic.
 *
 * It does not identify:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical qubit 0
 *     memory bank 0
 *     network node 0
 *
 * A scope such as:
 *
 *     program
 *     module
 *     function
 *     operation
 *     execution_region
 *
 * is interpreted semantically.
 * ============================================================================
 */

resourcePreferenceScopeValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 26. PREFERENCE TARGET VALUE
 * ============================================================================
 *
 * A preference may express a desired target category.
 *
 * It MUST NOT force physical target selection at parse time.
 *
 * Examples:
 *
 *     target = quantum;
 *     target = accelerator;
 *     target = heterogeneous;
 *
 * Since resource target semantics already have an owner in resources.g4,
 * this file merely supplies the reusable value boundary.
 * ============================================================================
 */

resourcePreferenceTargetValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 27. PREFERENCE CAPABILITY VALUE
 * ============================================================================
 *
 * Capability syntax remains owned by the resource capability architecture.
 *
 * This rule exists only as a reusable preference boundary.
 * ============================================================================
 */

resourcePreferenceCapabilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 28. SEMANTIC DISTINCTION
 * ============================================================================
 *
 * A parser consumer MUST preserve the following distinction:
 *
 *     preference
 *         |
 *         +--> objective
 *         +--> condition
 *         +--> ordering
 *         +--> metadata
 *         +--> extension property
 *
 * None of these is a requirement.
 *
 * None of these is a constraint.
 *
 * None of these is a hint.
 *
 * Semantic analysis owns the distinction.
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar is explicitly designed for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * It therefore contains NO language-level maximum for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     accelerators
 *     memory
 *     storage
 *     registers
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     timelines
 *     resource groups
 *     preference groups
 *     preference clauses
 *
 * In particular, this file contains no:
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
 * ============================================================================
 * 30. HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Preference syntax MUST NOT encode a physical realization.
 *
 * Therefore this file does not contain universal syntax for:
 *
 *     physical_cpu(0)
 *     physical_gpu(0)
 *     physical_qubit(0)
 *     memory_bank(0)
 *     device_address(...)
 *     fixed_topology(...)
 *     vendor_device(...)
 *
 * If a target-specific realization is required, that realization belongs to
 * the downstream target/resource/placement architecture and must remain
 * distinguishable from portable source preference semantics.
 *
 * ============================================================================
 * 31. RESOURCE AVAILABILITY
 * ============================================================================
 *
 * A preference can mention symbolic availability information:
 *
 *     available_capacity
 *     available_memory
 *     available_parallelism
 *     availability
 *
 * The grammar does not inspect those values.
 *
 * Availability is supplied later by:
 *
 *     compiler;
 *     resource manager;
 *     target description;
 *     runtime environment;
 *     deployment environment;
 *     hardware abstraction layer.
 *
 * ============================================================================
 * 32. SCALABILITY
 * ============================================================================
 *
 * The grammar uses recursive/open composition and ANTLR repetition rather than
 * finite resource cardinalities.
 *
 * This means:
 *
 *     one preference
 *
 * and:
 *
 *     arbitrarily many preferences
 *
 * have the same language architecture.
 *
 * "Infinity" here means:
 *
 *     no artificial language-level ceiling.
 *
 * It does not claim physically infinite hardware or infinite compiler memory.
 *
 * ============================================================================
 * 33. DETERMINISM
 * ============================================================================
 *
 * Preference syntax is declarative.
 *
 * Parsing the same source with the same language version MUST produce the same
 * syntactic structure.
 *
 * Preference optimization order is NOT determined by parser alternative order.
 *
 * Semantic analysis must explicitly interpret:
 *
 *     priority;
 *     weight;
 *     objective;
 *     tie-breaking;
 *     conflicting preferences.
 *
 * ============================================================================
 * 34. CONFLICTING PREFERENCES
 * ============================================================================
 *
 * The grammar permits multiple preferences even when their semantic objectives
 * conflict.
 *
 * Example:
 *
 *     objective = throughput >= desired_throughput;
 *     tie_breaker = energy <= energy_budget;
 *
 * or:
 *
 *     performance::latency = latency_goal;
 *     performance::energy = energy_goal;
 *
 * Conflict resolution is NOT grammar behavior.
 *
 * The semantic/resource optimization layer must define the applicable policy.
 *
 * ============================================================================
 * 35. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT BOUNDARY
 * ============================================================================
 *
 * The following are intentionally separate concepts:
 *
 *     REQUIRES
 *         mandatory feasibility condition
 *
 *     CONSTRAINT
 *         mandatory realization restriction
 *
 *     PREFER
 *         desirable objective that may be traded off
 *
 *     HINT
 *         advisory information that may be ignored
 *
 * A high preference priority MUST NOT change its category into REQUIRES.
 *
 * A preference weight MUST NOT change its category into CONSTRAINT.
 *
 * This category distinction belongs to the semantic model.
 *
 * ============================================================================
 * 36. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum programs may express preferences such as:
 *
 *     fidelity;
 *     latency;
 *     throughput;
 *     energy;
 *     coherence-related metrics;
 *     error-related metrics;
 *     communication cost;
 *     routing cost;
 *     execution duration.
 *
 * This grammar does not know how those metrics are implemented.
 *
 * Quantum preference consequences may eventually influence:
 *
 *     quantum::ir;
 *     optimization;
 *     routing;
 *     scheduling;
 *     resilience;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * No preference rule in this file directly depends on quantum::ir.
 *
 * ============================================================================
 * 37. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * The same syntax can express preferences over:
 *
 *     latency;
 *     throughput;
 *     memory;
 *     vectorization;
 *     energy;
 *     parallelism;
 *     portability;
 *     scalability;
 *     cost.
 *
 * Classical backend selection remains downstream.
 *
 * ============================================================================
 * 38. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Preferences may describe desired hardware characteristics without encoding
 * a particular implementation.
 *
 * Examples:
 *
 *     timing::latency = desired_latency;
 *     power::budget = power_budget;
 *     reliability::goal = reliability_goal;
 *     throughput = desired_throughput;
 *
 * This grammar does not define:
 *
 *     wire widths;
 *     fixed register widths;
 *     fixed FPGA resources;
 *     fixed ASIC structures;
 *     fixed clock counts;
 *     fixed physical placement.
 *
 * ============================================================================
 * 39. DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Preferences may express:
 *
 *     latency;
 *     bandwidth;
 *     throughput;
 *     communication_cost;
 *     locality;
 *     portability;
 *     energy;
 *     resilience.
 *
 * Node count and network topology remain downstream resource/target data.
 *
 * ============================================================================
 * 40. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Preferences may apply to:
 *
 *     training;
 *     inference;
 *     tensor computation;
 *     accelerator use;
 *     memory behavior;
 *     data movement;
 *     latency;
 *     throughput;
 *     energy;
 *     scalability.
 *
 * Framework-specific concepts remain outside this universal grammar.
 *
 * ============================================================================
 * 41. SECURITY
 * ============================================================================
 *
 * Parsing preference syntax must be side-effect free.
 *
 * This file does not:
 *
 *     - inspect hardware;
 *     - contact services;
 *     - read secrets;
 *     - resolve providers;
 *     - execute optimization;
 *     - allocate resources.
 *
 * Security policy is enforced downstream.
 *
 * ============================================================================
 * 42. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve, at minimum:
 *
 *     preference kind;
 *     source span;
 *     clauses;
 *     property name;
 *     value expression;
 *     condition expression;
 *     objective expression;
 *     ordering metadata;
 *     metadata/extensions;
 *     lexical/source locations.
 *
 * The AST must remain domain-neutral.
 *
 * It must not contain:
 *
 *     GPU-specific structures;
 *     QPU-specific structures;
 *     physical-device IDs;
 *     physical-qubit IDs;
 *     vendor-specific realization state.
 *
 * ============================================================================
 * 43. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - classify standard and extension properties;
 *     - type-check values;
 *     - validate dimensions/units where applicable;
 *     - validate objective expressions;
 *     - validate condition expressions;
 *     - preserve preference category;
 *     - detect invalid or contradictory metadata;
 *     - preserve source spans;
 *     - report unsupported properties according to language-version policy.
 *
 * Semantic analysis MAY use target information later.
 *
 * The parser must not.
 *
 * ============================================================================
 * 44. IR CONTRACT
 * ============================================================================
 *
 * This grammar defines NO IR.
 *
 * Preference information is lowered by semantic/resource analysis into the
 * repository's canonical resource-intent representation.
 *
 * A preference may influence downstream:
 *
 *     optimization;
 *     target selection;
 *     placement;
 *     routing;
 *     scheduling;
 *     resilience;
 *     runtime policy.
 *
 * It must never require a second domain-specific IR merely because its
 * preferred realization concerns quantum, HDL, GPU, FPGA, or another target.
 *
 * ============================================================================
 * 45. DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify malformed syntax only.
 *
 * Examples:
 *
 *     missing property name;
 *     missing assignment operator;
 *     missing value;
 *     missing semicolon;
 *     unbalanced preference block.
 *
 * Semantic diagnostics handle:
 *
 *     unknown standard property;
 *     invalid property type;
 *     invalid objective;
 *     invalid condition;
 *     incompatible dimensions;
 *     unsupported dialect property;
 *     conflicting semantic requirements.
 *
 * Resource infeasibility is NOT a syntax error.
 *
 * ============================================================================
 * 46. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable preference property spellings are compatibility-sensitive.
 *
 * Adding a new open-world property does not require changing this grammar.
 *
 * Removing or changing the meaning of a standardized property requires a
 * language-version/deprecation policy.
 *
 * Vendor/dialect properties should use qualified namespaces where appropriate.
 *
 * Example:
 *
 *     vendor::family::metric
 *
 * rather than adding vendor-specific global keywords.
 *
 * ============================================================================
 * 47. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must remain structurally simple:
 *
 *     property -> assignment -> expression
 *
 * Open-world names use bounded local alternatives followed by repetition.
 *
 * No semantic lookup occurs during parsing.
 *
 * No target discovery occurs during parsing.
 *
 * This prevents resource preference syntax from introducing target-dependent
 * parser behavior.
 *
 * ============================================================================
 * 48. TEST CONTRACT
 * ============================================================================
 *
 * This file is complete only when the following tests exist in the resource
 * preference test suite.
 *
 * POSITIVE
 * --------
 *
 *     prefer latency <= latency_budget;
 *     prefer throughput >= desired_throughput;
 *     prefer energy <= energy_budget;
 *
 *     prefer {
 *         objective = latency <= latency_budget;
 *     };
 *
 *     prefer {
 *         objective = throughput >= desired_throughput;
 *         priority = preference_priority;
 *         weight = preference_weight;
 *     };
 *
 *     prefer {
 *         performance::latency = latency_goal;
 *         performance::throughput = throughput_goal;
 *     };
 *
 *     prefer {
 *         quantum::fidelity = desired_fidelity;
 *         quantum::latency = latency_budget;
 *     };
 *
 *     prefer {
 *         vendor::future::metric = desired_value;
 *     };
 *
 * NEGATIVE
 * --------
 *
 *     prefer { = value; };
 *     prefer { property = ; };
 *     prefer { property value; };
 *     prefer { property = value };
 *
 *     malformed qualified property names;
 *     malformed nested blocks;
 *     missing delimiters.
 *
 * BOUNDARY
 * --------
 *
 *     zero clauses;
 *     one clause;
 *     many clauses;
 *     deeply qualified symbolic properties;
 *     deeply nested expressions;
 *     large symbolic quantities.
 *
 * SCALABILITY
 * ----------
 *
 *     many preferences;
 *     many groups;
 *     large expressions;
 *     arbitrarily large symbolic resource quantities;
 *     large cross-domain preference sets.
 *
 * HARD-CODING
 * -----------
 *
 * Static validation must reject accidental introduction of universal resource
 * limits or equivalent fixed-capacity grammar constructs.
 *
 * ============================================================================
 * 49. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [x] Preference payload ownership is isolated.
 *     [x] No duplicate resourcePreference statement is defined.
 *     [x] No duplicate resourcePreferenceExpression is defined.
 *     [x] Canonical resourceExpression is reused.
 *     [x] Canonical names are reused.
 *     [x] Actual repository token names are used.
 *     [x] Open-world properties are supported.
 *     [x] Qualified property namespaces are supported.
 *     [x] Preference category remains distinct from requirements/constraints.
 *     [x] No physical target is selected.
 *     [x] No machine-size limit is encoded.
 *     [x] No Rust code is embedded.
 *     [x] No unsafe Rust is required.
 *     [x] AST requirements are defined.
 *     [x] Semantic requirements are defined.
 *     [x] IR ownership remains downstream.
 *     [x] Cross-domain integration is defined.
 *     [x] Diagnostics are separated from resource feasibility.
 *     [x] Compatibility behavior is defined.
 *     [x] Positive/negative/boundary/scalability tests are specified.
 *
 * Remaining integration work belongs to the parent composition grammar, not
 * to this leaf file.
 *
 * ============================================================================
 */