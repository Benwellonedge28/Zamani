/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/preferences.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Grammar name:
 *     ResourcePreferences
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the canonical SOURCE-LEVEL RESOURCE PREFERENCE
 * COMPOSITION GRAMMAR for Zamani.
 *
 * A preference expresses desirable resource realization characteristics
 * without turning those characteristics into mandatory requirements.
 *
 * Preferences may describe:
 *
 *     - preferred resource properties;
 *     - preferred capabilities;
 *     - preferred targets;
 *     - preferred performance;
 *     - preferred latency;
 *     - preferred throughput;
 *     - preferred bandwidth;
 *     - preferred energy behaviour;
 *     - preferred power behaviour;
 *     - preferred reliability;
 *     - preferred resilience;
 *     - preferred portability;
 *     - preferred scalability;
 *     - preferred cost;
 *     - preferred placement characteristics;
 *     - preferred resource relationships;
 *     - preferred execution characteristics;
 *     - conditional preferences;
 *     - preference weights;
 *     - preference priorities;
 *     - preference scopes;
 *     - preference groups;
 *     - extensible future preference properties.
 *
 * The grammar describes PREFERENCE INTENT.
 *
 * It does NOT decide:
 *
 *     - which machine is selected;
 *     - which device is selected;
 *     - which CPU is selected;
 *     - which GPU is selected;
 *     - which FPGA is selected;
 *     - which QPU is selected;
 *     - how many physical resources exist;
 *     - how resources are allocated;
 *     - how routing is performed;
 *     - how scheduling is performed;
 *     - how optimization is performed;
 *     - whether a preference can be satisfied;
 *     - whether a preference is globally optimal.
 *
 * Those decisions belong to semantic analysis, compilation, optimization,
 * resource management, routing, scheduling, hardware abstraction, and
 * runtime layers.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *       lexer
 *          |
 *          v
 *       parser
 *          |
 *          v
 *   ResourcePreferences
 *          |
 *          v
 *      frontend AST
 *          |
 *          v
 *   semantic analysis
 *          |
 *          v
 * resource-preference model
 *          |
 *     +----+----+-------------------+
 *     |         |                   |
 *     v         v                   v
 *  compiler  optimizer          resource manager
 *     |         |                   |
 *     +---------+-------------------+
 *               |
 *               v
 *        scheduling/routing
 *               |
 *               v
 *          hardware HAL
 *               |
 *               v
 *             runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - resource preference entry points;
 *     - resource preference composition;
 *     - resource preference expressions;
 *     - resource preference groups;
 *     - resource preference attributes;
 *     - preference scope;
 *     - preference condition;
 *     - preference weight;
 *     - preference priority;
 *     - preference objective metadata;
 *     - preference ordering metadata;
 *     - extensible preference properties;
 *     - resource-scoped preference syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - expression precedence;
 *     - general types;
 *     - generic constraints;
 *     - generic requirements;
 *     - capability identity;
 *     - resource identity;
 *     - resource declarations;
 *     - resource allocation;
 *     - target discovery;
 *     - hardware discovery;
 *     - calibration;
 *     - topology;
 *     - routing;
 *     - scheduling algorithms;
 *     - optimization algorithms;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - simulation;
 *     - runtime implementation;
 *     - deployment implementation.
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * The canonical expression architecture owns:
 *
 *     expression
 *
 * The canonical resource expression architecture owns:
 *
 *     resourceExpression
 *
 * Therefore this file MUST NOT redefine:
 *
 *     expression
 *     assignmentExpression
 *     binaryExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     equalityExpression
 *     relationalExpression
 *     additiveExpression
 *     multiplicativeExpression
 *     unaryExpression
 *     primaryExpression
 *
 * Resource preference values and conditions consume:
 *
 *     resourceExpression
 *
 * This guarantees that resource preferences use exactly the same expression
 * syntax and semantics as the rest of Zamani.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT SEPARATION
 * ============================================================================
 *
 * REQUIREMENT
 *     Mandatory condition required for a valid realization.
 *
 * CONSTRAINT
 *     Mandatory condition restricting valid realizations.
 *
 * PREFERENCE
 *     Desirable condition or objective that may be traded off.
 *
 * HINT
 *     Advisory information that may be ignored.
 *
 * A preference MUST NOT become a requirement merely because it is expressed
 * strongly, given a high priority, or assigned a high weight.
 *
 * Semantic analysis owns the policy determining how preferences are optimized.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Resource preferences MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *
 * It also MUST NOT encode:
 *
 *     physical device identifiers;
 *     physical addresses;
 *     fixed topology;
 *     provider-specific device names;
 *     fixed machine sizes.
 *
 * A preference may instead refer to symbolic values such as:
 *
 *     workload_size
 *     requested_parallelism
 *     latency_budget
 *     energy_budget
 *     available_capacity
 *     required_memory
 *
 * without imposing a machine-specific upper bound.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Preference properties MUST remain open-world.
 *
 * The grammar must therefore allow semantic properties introduced by:
 *
 *     - future Zamani versions;
 *     - domain extensions;
 *     - dialects;
 *     - vendors;
 *     - hardware families;
 *     - execution environments;
 *     - future computing paradigms.
 *
 * The parser preserves the symbolic property.
 *
 * Semantic validation determines whether the property is known, supported,
 * deprecated, experimental, dialect-specific, or unknown.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Preference expressions are declarative.
 *
 * Parsing a preference MUST NOT:
 *
 *     - access the filesystem;
 *     - access the network;
 *     - inspect hardware;
 *     - discover devices;
 *     - allocate resources;
 *     - execute external commands;
 *     - invoke runtime APIs.
 *
 * Runtime and compilation systems may later interpret the resulting semantic
 * representation under their own security policies.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no finite machine-resource limit.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Preference groups may contain an arbitrary number of preference entries.
 *
 * Preference properties are symbolic.
 *
 * Preference values are expressions.
 *
 * Therefore the grammar can represent preferences for workloads ranging from
 * extremely small programs to arbitrarily large programs, subject only to
 * implementation/resource availability downstream.
 *
 * ============================================================================
 */

parser grammar ResourcePreferences;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A preference section contains zero or more preferences.
 *
 * There is intentionally no grammar-level cardinality limit.
 */
resourcePreferences
    : resourcePreferenceItem*
    ;


/*
 * ============================================================================
 * 2. PREFERENCE ITEM
 * ============================================================================
 */

resourcePreferenceItem
    : resourcePreference
    | resourcePreferenceGroup
    ;


/*
 * ============================================================================
 * 3. CANONICAL RESOURCE PREFERENCE
 * ============================================================================
 *
 * Canonical compact form:
 *
 *     preference resource latency <= latency_budget;
 *
 *     preference resource throughput >= required_throughput;
 *
 *     preference resource energy <= energy_budget;
 *
 *     preference resource capability_value == preferred_capability;
 *
 * The expression remains target-independent.
 *
 * Semantic analysis determines the objective represented by the expression.
 */
resourcePreference
    : K_PREFERENCE
      K_RESOURCE
      resourcePreferenceExpression
      SEMI
    ;


/*
 * ============================================================================
 * 4. RESOURCE-SCOPED PREFERENCE
 * ============================================================================
 *
 * This form permits a symbolic scope to be associated with the preference.
 *
 * Example:
 *
 *     preference resource compute {
 *         objective = throughput >= desired_throughput;
 *     };
 *
 * The scope is symbolic and does not identify a physical resource.
 */
resourceScopedPreference
    : K_PREFERENCE
      K_RESOURCE
      resourcePreferenceScope
      resourcePreferenceSpecification
      SEMI?
    ;


/*
 * ============================================================================
 * 5. PREFERENCE SCOPE
 * ============================================================================
 *
 * A scope is an abstract semantic reference.
 *
 * It may refer to:
 *
 *     program;
 *     module;
 *     function;
 *     operation;
 *     resource;
 *     resource group;
 *     execution region;
 *     logical domain;
 *     another semantic scope.
 *
 * The semantic layer resolves its meaning.
 */
resourcePreferenceScope
    : resourceExpression
    ;


/*
 * ============================================================================
 * 6. PREFERENCE SPECIFICATION
 * ============================================================================
 */

resourcePreferenceSpecification
    : LBRACE
      resourcePreferenceClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. PREFERENCE CLAUSE
 * ============================================================================
 *
 * Preference clauses deliberately use symbolic property names rather than a
 * closed keyword catalogue.
 *
 * This allows future preference dimensions without changing the grammar.
 *
 * Examples:
 *
 *     objective = latency <= latency_budget;
 *
 *     priority = requested_priority;
 *
 *     weight = preference_weight;
 *
 *     scope = execution_region;
 *
 *     when = workload_condition;
 *
 *     tie_breaker = energy <= energy_budget;
 *
 *     portability = portability_score;
 *
 *     custom::future_metric = target_value;
 */
resourcePreferenceClause
    : resourcePreferenceProperty
      ASSIGN
      resourcePreferenceValue
      SEMI
    ;


/*
 * ============================================================================
 * 8. PREFERENCE PROPERTY
 * ============================================================================
 *
 * The property name is open-world.
 *
 * A property can be:
 *
 *     objective
 *     priority
 *     weight
 *     scope
 *     when
 *     tie_breaker
 *     latency
 *     throughput
 *     energy
 *     reliability
 *     portability
 *     scalability
 *     custom::future_property
 *
 * The grammar does not reserve a finite list.
 */
resourcePreferenceProperty
    : qualifiedPreferenceName
    ;


/*
 * ============================================================================
 * 9. PREFERENCE VALUE
 * ============================================================================
 *
 * Values use the canonical resource expression grammar.
 */
resourcePreferenceValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 10. PREFERENCE EXPRESSION
 * ============================================================================
 *
 * The compact preference form delegates completely to the canonical resource
 * expression grammar.
 *
 * This prevents a second comparison/logical/arithmetic language from emerging.
 */
resourcePreferenceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 11. PREFERENCE GROUP
 * ============================================================================
 *
 * Groups allow multiple preferences to be represented together.
 *
 * No fixed group size exists.
 *
 * Example:
 *
 *     preference group performance {
 *         objective = throughput >= desired_throughput;
 *         tie_breaker = latency <= latency_budget;
 *         weight = performance_weight;
 *     };
 *
 * The group name is symbolic.
 */
resourcePreferenceGroup
    : K_PREFERENCE
      K_GROUP
      preferenceGroupName
      resourcePreferenceSpecification
      SEMI?
    ;


/*
 * ============================================================================
 * 12. PREFERENCE GROUP NAME
 * ============================================================================
 */

preferenceGroupName
    : qualifiedPreferenceName
    ;


/*
 * ============================================================================
 * 13. QUALIFIED PREFERENCE NAME
 * ============================================================================
 *
 * Preference namespaces are open-ended.
 *
 * Examples:
 *
 *     objective
 *
 *     performance.objective
 *
 *     zamani::performance::objective
 *
 *     vendor::accelerator::throughput
 *
 *     future::resource::metric
 *
 * Namespace depth is intentionally unbounded by the grammar.
 */
qualifiedPreferenceName
    : identifier
      (
          DOT identifier
        | DOUBLE_COLON identifier
      )*
    ;


/*
 * ============================================================================
 * 14. OBJECTIVE PREFERENCE
 * ============================================================================
 *
 * This rule provides a semantic naming boundary for the most common
 * preference property while keeping the value generic.
 *
 * Example:
 *
 *     objective = latency <= latency_budget;
 *
 * No finite set of objective types is encoded.
 */
resourcePreferenceObjective
    : identifier
      ASSIGN
      resourcePreferenceValue
      SEMI
    ;


/*
 * ============================================================================
 * 15. CONDITIONAL PREFERENCE
 * ============================================================================
 *
 * A conditional preference allows a preference to apply only when an
 * expression evaluates to the relevant semantic condition.
 *
 * Example:
 *
 *     preference resource {
 *         when = workload_size > threshold;
 *         objective = latency <= latency_budget;
 *     };
 *
 * The condition is interpreted by semantic analysis.
 *
 * It does not cause runtime execution during parsing.
 */
resourceConditionalPreference
    : K_PREFERENCE
      K_RESOURCE
      resourcePreferenceSpecification
      SEMI?
    ;


/*
 * ============================================================================
 * 16. PREFERENCE METADATA
 * ============================================================================
 *
 * Metadata remains symbolic and extensible.
 *
 * Example:
 *
 *     preference resource {
 *         objective = throughput >= desired_throughput;
 *         metadata::origin = "developer";
 *     };
 *
 * Metadata does not itself alter the semantic meaning unless the downstream
 * semantic model explicitly recognizes it.
 */
resourcePreferenceMetadata
    : resourcePreferenceProperty
      ASSIGN
      resourcePreferenceValue
      SEMI
    ;


/*
 * ============================================================================
 * 17. PREFERENCE LIST
 * ============================================================================
 *
 * Useful when a parent grammar wants to consume preferences as a list.
 */
resourcePreferenceList
    : resourcePreferenceItem
      resourcePreferenceItem*
    ;


/*
 * ============================================================================
 * 18. OPTIONAL PREFERENCE LIST
 * ============================================================================
 */

optionalResourcePreferenceList
    : resourcePreferenceList?
    ;


/*
 * ============================================================================
 * 19. RESOURCE PREFERENCE EXPRESSION LIST
 * ============================================================================
 */

resourcePreferenceExpressionList
    : resourcePreferenceExpression
      (
          COMMA resourcePreferenceExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. PREFERENCE PROPERTY LIST
 * ============================================================================
 *
 * Arbitrary property cardinality.
 */
resourcePreferencePropertyList
    : resourcePreferenceClause*
    ;


/*
 * ============================================================================
 * 21. WEIGHT / PRIORITY / ORDERING
 * ============================================================================
 *
 * These are intentionally expressed through symbolic property names rather
 * than dedicated lexer keywords.
 *
 * Consequently the language can evolve from:
 *
 *     weight = expression;
 *
 * to future domain-specific preference systems without modifying this grammar.
 *
 * The semantic layer determines:
 *
 *     whether the value is valid;
 *     whether it is numeric;
 *     whether it is ordered;
 *     whether it is comparable;
 *     whether it is meaningful for the selected optimization policy.
 *
 * No source-level maximum or minimum is imposed here.
 */


/*
 * ============================================================================
 * 22. PREFERENCE ATTRIBUTE
 * ============================================================================
 *
 * Attributes remain delegated to the canonical attribute architecture when
 * integrated by a parent grammar.
 *
 * This rule intentionally does not recreate attribute syntax.
 *
 * Parent grammars may attach their canonical attributes around preference
 * declarations.
 */


/*
 * ============================================================================
 * 23. PREFERENCE EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This rule is an explicit semantic boundary used by consumers that need to
 * distinguish a preference expression from an ordinary resource expression.
 *
 * It does not create a new expression language.
 */
resourcePreferenceCondition
    : resourceExpression
    ;


/*
 * ============================================================================
 * 24. PREFERENCE TARGET EXPRESSION
 * ============================================================================
 *
 * Targets remain abstract.
 *
 * A target expression may describe a preferred computational category without
 * selecting a concrete device.
 *
 * Example:
 *
 *     target = accelerator;
 *
 *     target = quantum;
 *
 *     target = heterogeneous;
 *
 * Concrete target realization is outside this grammar.
 */
resourcePreferenceTargetExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 25. PREFERENCE CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Capability syntax itself belongs to the canonical capability grammar.
 *
 * This rule only provides a resource-preference expression boundary.
 */
resourcePreferenceCapabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 26. PREFERENCE RESOURCE EXPRESSION
 * ============================================================================
 *
 * This rule exists for downstream grammars that need an explicitly named
 * resource-preference expression boundary.
 */
resourcePreferenceResourceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 27. PREFERENCE EXTENSION
 * ============================================================================
 *
 * Extension properties use the same open-world property mechanism.
 *
 * Examples:
 *
 *     vendor::gpu::occupancy = desired_occupancy;
 *
 *     vendor::qpu::queue_time = queue_budget;
 *
 *     future::accelerator::metric = desired_value;
 *
 * The grammar does not validate the namespace.
 */
resourcePreferenceExtension
    : qualifiedPreferenceName
      ASSIGN
      resourcePreferenceValue
      SEMI
    ;


/*
 * ============================================================================
 * 28. PREFERENCE CONTRACT
 * ============================================================================
 *
 * A contract groups preference information without making it a requirement.
 *
 * Example:
 *
 *     preference resource {
 *         objective = latency <= latency_budget;
 *         weight = latency_weight;
 *         portability = portability_score;
 *     };
 *
 * The semantic layer preserves the preference status.
 */
resourcePreferenceContract
    : K_PREFERENCE
      K_RESOURCE
      resourcePreferenceSpecification
      SEMI?
    ;


/*
 * ============================================================================
 * 29. PREFERENCE ASSERTION BOUNDARY
 * ============================================================================
 *
 * This is intentionally only a syntax boundary.
 *
 * Whether a preference can be evaluated is a semantic question.
 */
resourcePreferenceAssertion
    : resourcePreferenceCondition
    ;


/*
 * ============================================================================
 * 30. HARD-CODING BOUNDARY
 * ============================================================================
 *
 * The following concepts MUST remain semantic expressions:
 *
 *     resource count
 *     resource capacity
 *     machine size
 *     memory capacity
 *     qubit count
 *     processor count
 *     accelerator count
 *     node count
 *     topology size
 *     workload size
 *     throughput
 *     latency
 *     energy
 *     reliability
 *     scalability
 *
 * This grammar contains no finite constants representing any of them.
 *
 * Any literal appearing inside a resource expression is a program-level value
 * and MUST NOT be interpreted by this grammar as a machine-size limit.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Every parsed preference must preserve at least these semantic distinctions:
 *
 *     preference identity
 *     preference expression
 *     preference scope
 *     preference condition
 *     preference properties
 *     preference metadata
 *
 * The semantic layer must additionally preserve the fact that the construct
 * is a PREFERENCE rather than a REQUIREMENT or CONSTRAINT.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT define a canonical IR.
 *
 * The parser produces syntax/AST information.
 *
 * Semantic analysis lowers that information into the repository's canonical
 * resource-intent representation.
 *
 * If a preference affects quantum compilation, its semantic consequences may
 * eventually influence:
 *
 *     quantum::ir
 *     optimization
 *     routing
 *     scheduling
 *     hardware selection
 *     runtime policy
 *
 * but this grammar does not directly depend on quantum::ir.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * The same preference syntax must be usable for:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     distributed
 *     AI/ML
 *     networking
 *     storage
 *     memory
 *     embedded
 *     HPC
 *     future domains
 *
 * Domain-specific grammars may specialize preference attachment, but they
 * MUST NOT redefine the universal preference semantics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It compiles as an ANTLR4 parser grammar.
 *
 *     2. Its imported ResourceExpressions grammar resolves successfully.
 *
 *     3. Its token vocabulary resolves through ZamaniLexer.
 *
 *     4. It introduces no duplicate general expression grammar.
 *
 *     5. It introduces no machine-size constants.
 *
 *     6. It supports arbitrary preference-group cardinality.
 *
 *     7. It supports open-world preference properties.
 *
 *     8. It preserves preference-vs-requirement-vs-constraint semantics.
 *
 *     9. It can be consumed by resources.g4.
 *
 *    10. It can be reused by future resource-domain grammars without
 *        redefining the preference model.
 *
 *    11. Positive, negative, boundary, scalability, and cross-domain tests
 *        pass.
 *
 *    12. No Rust semantic action or unsafe code exists in the grammar.
 *
 * ============================================================================
 */