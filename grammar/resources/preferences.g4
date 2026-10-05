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
 * CANONICAL RESOURCE-PREFERENCE PAYLOAD GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the reusable SOURCE-SYNTAX PAYLOAD for resource
 * preferences.
 *
 * It does NOT own the concrete `prefer` statement.
 *
 * The concrete resource statement remains owned by:
 *
 *     grammar/resources/resources.g4
 *
 * Conceptual ownership:
 *
 *     resources.g4
 *          |
 *          +--> resourcePreference
 *                    |
 *                    v
 *              ResourcePreferences
 *                    |
 *                    v
 *              resourcePreferenceSpecification
 *
 * A preference expresses a desirable realization characteristic.
 *
 * A preference is distinct from:
 *
 *     requirement
 *     constraint
 *     capability
 *     hint
 *     budget
 *     policy
 *     implementation decision
 *     physical allocation
 *
 * Preferences are therefore advisory semantic intent. A downstream
 * optimizer, resource resolver, deployment planner, or execution planner
 * may trade one preference against another according to explicit policy.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/resources/ResourceExpressions.g4
 *     grammar/core/names.g4
 *
 * EXPORTS
 * -------
 *
 * Primary:
 *
 *     resourcePreferenceExpression
 *     resourcePreferenceSpecification
 *     resourcePreferenceClause
 *
 * Secondary reusable rules:
 *
 *     resourcePreferenceAssignment
 *     resourcePreferenceObjective
 *     resourcePreferenceOrdering
 *     resourcePreferenceCondition
 *     resourcePreferenceProperty
 *     resourcePreferenceValue
 *     resourcePreferenceName
 *     resourcePreferenceGroup
 *     resourcePreferenceGroupBody
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/resources/resources.g4
 *     grammar/resources/negotiation.g4
 *     grammar/resources/hints.g4
 *     grammar/resources/scalability.g4
 *     grammar/policies/
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/dialects/
 *
 * AST_OWNER
 * ---------
 *
 * Domain-neutral frontend AST.
 *
 * This grammar creates parser contexts only.
 *
 * The AST representation must preserve:
 *
 *     - preference kind;
 *     - preference key;
 *     - value expression;
 *     - ordering metadata;
 *     - condition;
 *     - group structure;
 *     - source span;
 *     - source ordering.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Resource-preference semantic analysis.
 *
 * Semantic analysis determines:
 *
 *     - whether a preference key is known;
 *     - whether it is standardized;
 *     - whether it is dialect-specific;
 *     - whether its value has the required type;
 *     - whether it conflicts with another preference;
 *     - whether policy permits it;
 *     - whether it can participate in target negotiation.
 *
 * IR_OWNER
 * --------
 *
 * No direct IR is owned by this grammar.
 *
 * Preferences lower into the canonical semantic resource-intent model.
 *
 * They MUST NOT directly create:
 *
 *     CPU instructions;
 *     GPU instructions;
 *     FPGA primitives;
 *     ASIC structures;
 *     quantum operations;
 *     quantum::ir operations;
 *     HDL netlists;
 *     routing decisions;
 *     scheduling decisions;
 *     backend objects.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/resources/preferences/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/resources.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     resourcePreference
 *     prefer
 *     resourceExpression
 *     expression
 *     identifier
 *     qualifiedName
 *     requirements
 *     constraints
 *     capabilities
 *     budgets
 *     hints
 *     negotiation
 *     scalability semantics
 *     policy semantics
 *     effect semantics
 *     provenance semantics
 *     target selection
 *     hardware discovery
 *     allocation
 *     routing
 *     scheduling
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * RESOURCE-SEMANTIC SEPARATION
 * ============================================================================
 *
 * Requirement:
 *
 *     what the program must have or satisfy.
 *
 * Constraint:
 *
 *     what must remain true.
 *
 * Capability:
 *
 *     what an environment can provide.
 *
 * Preference:
 *
 *     which otherwise-valid realization is desirable.
 *
 * Hint:
 *
 *     non-binding implementation guidance.
 *
 * Budget:
 *
 *     an explicit resource accounting boundary.
 *
 * Policy:
 *
 *     governing permission/prohibition/selection rules.
 *
 * These concepts MUST remain distinguishable in the AST and semantic model.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT enumerate a finite set of physical resources.
 *
 * It therefore does NOT define:
 *
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     QPU counts
 *     node counts
 *     memory capacities
 *     thread counts
 *     tensor-rank limits
 *     register widths
 *     network-size limits
 *     device counts
 *
 * A preference value may be:
 *
 *     literal;
 *     symbolic;
 *     computed;
 *     generic;
 *     dependent;
 *     runtime-derived;
 *     externally supplied.
 *
 * No parser-level maximum is imposed on:
 *
 *     preference count;
 *     clause count;
 *     group count;
 *     nesting depth;
 *     namespace depth;
 *     expression size.
 *
 * Practical implementation limits are implementation constraints, not
 * language semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Preferences describe portable intent.
 *
 * They MUST NOT force physical target selection.
 *
 * Example:
 *
 *     prefer latency <= latency_goal;
 *
 * means that lower latency is desirable subject to the semantic expression.
 *
 * It does NOT mean:
 *
 *     use CPU 0;
 *     use GPU 0;
 *     use QPU 0;
 *     use FPGA 0.
 *
 * The downstream pipeline remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic resource model
 *       ->
 *     capability/resource analysis
 *       ->
 *     negotiation
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     target realization
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar ResourcePreferences;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * PUBLIC ROOT
 * ============================================================================
 *
 * This rule is intentionally a payload rule.
 *
 * `resources.g4` owns the outer:
 *
 *     PREFER ... SEMICOLON
 *
 * statement.
 *
 * This grammar owns the expression following `PREFER`.
 *
 * Supported forms include:
 *
 *     prefer latency <= latency_goal;
 *
 *     prefer throughput >= desired_throughput;
 *
 *     prefer energy <= energy_goal;
 *
 *     prefer {
 *         objective = latency_goal;
 *         priority = priority_value;
 *     };
 *
 * The outer statement is not defined here.
 * ============================================================================
 */

resourcePreferenceExpression
    : resourcePreferenceSpecification
    | resourcePreferenceAssignment
    | resourcePreferenceObjective
    | resourcePreferenceOrdering
    | resourcePreferenceCondition
    | resourcePreferenceProperty
    ;


/*
 * ============================================================================
 * PREFERENCE SPECIFICATION
 * ============================================================================
 *
 * A specification is an unbounded collection of preference clauses.
 *
 * Example:
 *
 *     prefer {
 *         objective = latency_goal;
 *         priority = latency_priority;
 *         condition = workload_size > threshold;
 *     };
 *
 * The surrounding `prefer` and terminating semicolon remain owned by
 * resources.g4.
 * ============================================================================
 */

resourcePreferenceSpecification
    : LBRACE
      resourcePreferenceClause*
      RBRACE
    ;


/*
 * ============================================================================
 * CLAUSE DISPATCH
 * ============================================================================
 */

resourcePreferenceClause
    : resourcePreferenceAssignment
    | resourcePreferenceObjective
    | resourcePreferenceOrdering
    | resourcePreferenceCondition
    | resourcePreferenceProperty
    | resourcePreferenceGroup
    ;


/*
 * ============================================================================
 * GENERIC ASSIGNMENT
 * ============================================================================
 *
 * This is the principal open-world extension point.
 *
 * Examples:
 *
 *     latency = latency_goal;
 *
 *     throughput = desired_throughput;
 *
 *     quantum::fidelity = fidelity_goal;
 *
 *     accelerator::occupancy = occupancy_goal;
 *
 *     future::architecture::metric = desired_value;
 *
 * The parser records the symbolic key and expression.
 *
 * Semantic analysis determines whether the key is:
 *
 *     standardized;
 *     dialect-defined;
 *     vendor-defined;
 *     experimental;
 *     deprecated;
 *     unknown.
 * ============================================================================
 */

resourcePreferenceAssignment
    : resourcePreferenceName
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * OBJECTIVE
 * ============================================================================
 *
 * An objective explicitly identifies the principal preference objective.
 *
 * Example:
 *
 *     objective = latency_goal;
 *
 * or:
 *
 *     objective = latency <= latency_goal;
 *
 * The value remains a canonical resource expression.
 * ============================================================================
 */

resourcePreferenceObjective
    : OBJECTIVE
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Ordering metadata is intentionally expression-based.
 *
 * It may represent:
 *
 *     priority;
 *     weight;
 *     ordering class;
 *     symbolic ranking;
 *     dialect-defined ordering metadata.
 *
 * No finite priority range is encoded.
 * ============================================================================
 */

resourcePreferenceOrdering
    : resourcePreferenceOrderingKey
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


resourcePreferenceOrderingKey
    : PRIORITY
    | WEIGHT
    | ORDER
    | identifier
    ;


/*
 * ============================================================================
 * CONDITION
 * ============================================================================
 *
 * A condition controls when a preference is applicable.
 *
 * It is declarative.
 *
 * Parsing MUST NOT evaluate the condition.
 *
 * Semantic analysis determines whether the expression is boolean-compatible
 * and whether all referenced information is available in the relevant scope.
 * ============================================================================
 */

resourcePreferenceCondition
    : WHEN
      ASSIGN
      resourcePreferenceConditionValue
      SEMICOLON
    | CONDITION
      ASSIGN
      resourcePreferenceConditionValue
      SEMICOLON
    ;


resourcePreferenceConditionValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * PREFERENCE PROPERTY
 * ============================================================================
 *
 * A property is a symbolic resource-preference key followed by a value.
 *
 * Standard resource words that are already reserved lexical tokens must be
 * admitted through the canonical preference-key vocabulary.
 *
 * This rule deliberately does NOT enumerate physical devices.
 *
 * It only admits resource-domain vocabulary already defined by the language.
 *
 * New resource concepts that are not reserved keywords use qualified names.
 * ============================================================================
 */

resourcePreferenceProperty
    : resourcePreferenceName
      ASSIGN
      resourcePreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * PREFERENCE NAME
 * ============================================================================
 *
 * The name model is intentionally open-world.
 *
 * There are two forms:
 *
 *     1. canonical identifier/qualified-name;
 *     2. standardized resource vocabulary tokens.
 *
 * The second form exists because Zamani's lexer reserves certain universal
 * resource words as keywords.
 *
 * It prevents a lexical keyword such as `latency` or `energy` from becoming
 * unusable as a preference key.
 *
 * This is syntax compatibility, not a closed resource catalogue.
 * ============================================================================
 */

resourcePreferenceName
    : qualifiedName
    | resourcePreferenceKeywordName
    ;


resourcePreferenceKeywordName
    : AVAILABILITY
    | BANDWIDTH
    | COST
    | ENERGY
    | FIDELITY
    | LATENCY
    | PERFORMANCE
    | PORTABILITY
    | POWER
    | RELIABILITY
    | RESILIENCE
    | SCALABILITY
    | THROUGHPUT
    ;


/*
 * ============================================================================
 * VALUE
 * ============================================================================
 *
 * Every preference value uses the canonical resource-expression language.
 *
 * This file MUST NOT introduce another arithmetic, comparison, logical, call,
 * indexing, literal, or symbolic-expression language.
 * ============================================================================
 */

resourcePreferenceValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * GROUP
 * ============================================================================
 *
 * Groups provide reusable semantic organization without imposing physical
 * grouping.
 *
 * A group is NOT:
 *
 *     a CPU group;
 *     a GPU group;
 *     a QPU group;
 *     a node group;
 *     a scheduling group.
 *
 * It is simply a semantic preference collection.
 * ============================================================================
 */

resourcePreferenceGroup
    : GROUP
      resourcePreferenceName
      resourcePreferenceGroupBody
    ;


resourcePreferenceGroupBody
    : LBRACE
      resourcePreferenceClause*
      RBRACE
    ;


/*
 * ============================================================================
 * REUSABLE LISTS
 * ============================================================================
 *
 * Lists are deliberately unbounded at the language level.
 * ============================================================================
 */

resourcePreferenceClauseList
    : resourcePreferenceClause*
    ;


resourcePreferenceNameList
    : resourcePreferenceName
      (COMMA resourcePreferenceName)*
    ;


resourcePreferenceValueList
    : resourcePreferenceValue
      (COMMA resourcePreferenceValue)*
    ;


optionalResourcePreferenceSpecification
    : resourcePreferenceSpecification?
    ;


/*
 * ============================================================================
 * METADATA / EXTENSION BOUNDARY
 * ============================================================================
 *
 * Metadata remains an ordinary symbolic preference property.
 *
 * There is no separate metadata language here.
 *
 * Examples:
 *
 *     metadata::origin = developer;
 *
 *     metadata::confidence = confidence_value;
 *
 *     vendor::accelerator::metric = desired_value;
 *
 * Semantic analysis determines whether the namespace is recognized.
 * ============================================================================
 */

resourcePreferenceExtension
    : resourcePreferenceAssignment
    ;


/*
 * ============================================================================
 * CONTRACT BOUNDARY
 * ============================================================================
 *
 * This rule is a reusable grouping boundary only.
 *
 * It does NOT turn a preference into a requirement or contract.
 *
 * Contract semantics belong to validation/contracts and the universal
 * contract model.
 * ============================================================================
 */

resourcePreferenceContractBody
    : resourcePreferenceSpecification
    ;


/*
 * ============================================================================
 * SCOPE VALUE
 * ============================================================================
 *
 * Scope is symbolic and target-neutral.
 *
 * Examples:
 *
 *     program;
 *     module;
 *     function;
 *     operation;
 *     execution_region;
 *
 * Physical resource identifiers are not introduced here.
 * ============================================================================
 */

resourcePreferenceScopeValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * TARGET VALUE
 * ============================================================================
 *
 * A target preference may express an abstract realization category.
 *
 * It does not select a physical target.
 * ============================================================================
 */

resourcePreferenceTargetValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * CAPABILITY VALUE
 * ============================================================================
 *
 * Capability references remain semantic capability references.
 *
 * This grammar does not resolve or authorize capabilities.
 * ============================================================================
 */

resourcePreferenceCapabilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics must identify malformed preference syntax such as:
 *
 *     missing preference value;
 *     missing assignment operator;
 *     missing semicolon;
 *     unterminated preference specification;
 *     malformed group;
 *     malformed qualified name.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unknown standardized property;
 *     invalid value type;
 *     contradictory preference;
 *     invalid ordering semantics;
 *     forbidden policy interaction;
 *     unavailable capability;
 *     impossible target realization.
 *
 * The grammar must not attempt to perform semantic validation.
 * ============================================================================
 */


/*
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Preference values are resource expressions.
 *
 * Their eventual semantic type may be:
 *
 *     quantity;
 *     duration;
 *     energy;
 *     power;
 *     bandwidth;
 *     latency;
 *     probability;
 *     confidence;
 *     symbolic value;
 *     boolean;
 *     domain-specific resource type;
 *     future resource type.
 *
 * Type interpretation belongs to semantic analysis.
 *
 * No physical representation is fixed by this grammar.
 * ============================================================================
 */


/*
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring a preference has no execution effect.
 *
 * It does not:
 *
 *     allocate;
 *     reserve;
 *     consume;
 *     release;
 *     migrate;
 *     execute;
 *     measure;
 *     communicate;
 *     invoke foreign code.
 *
 * If a downstream decision caused by a preference produces an effect, that
 * effect belongs to the execution/effect subsystem.
 * ============================================================================
 */


/*
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Preferences may refer to capabilities symbolically through their resource
 * expressions.
 *
 * They do not:
 *
 *     declare capabilities;
 *     provide capabilities;
 *     authorize capabilities;
 *     prove capabilities exist.
 *
 * Capability semantics belong to grammar/core/capabilities.g4 and its semantic
 * owner.
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Preferences are resource-selection intent.
 *
 * They may describe desired properties of:
 *
 *     compute;
 *     memory;
 *     storage;
 *     network;
 *     accelerators;
 *     quantum resources;
 *     HDL/hardware realization;
 *     distributed execution;
 *     future resource classes.
 *
 * The actual resource provider is resolved downstream.
 *
 * This grammar never binds a preference to:
 *
 *     CPU 0;
 *     GPU 0;
 *     FPGA 0;
 *     QPU 0;
 *     node 0;
 *     physical qubit 0;
 *     memory bank 0.
 * ============================================================================
 */


/*
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Preference expressions may be referenced by:
 *
 *     requires;
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property;
 *     assertion;
 *
 * but this grammar does not own those constructs.
 *
 * A preference MUST NOT silently become a contract.
 * ============================================================================
 */


/*
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may:
 *
 *     permit;
 *     prohibit;
 *     prioritize;
 *     override;
 *     constrain;
 *     contextualize;
 *     negotiate
 *
 * preference use.
 *
 * Policy semantics are downstream.
 *
 * A preference alone never grants permission.
 * ============================================================================
 */


/*
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The AST/semantic model must preserve enough information to trace:
 *
 *     source location;
 *     preference key;
 *     preference value;
 *     enclosing resource context;
 *     applicable condition;
 *     ordering metadata.
 *
 * Downstream provenance may additionally record:
 *
 *     resolution;
 *     optimization decision;
 *     selected realization;
 *     rejected alternatives;
 *     policy decision;
 *     target capability evidence.
 *
 * This grammar does not create those runtime records.
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum preferences are represented generically.
 *
 * Examples:
 *
 *     prefer fidelity = target_fidelity;
 *
 *     prefer quantum::fidelity = target_fidelity;
 *
 *     prefer quantum::latency = latency_goal;
 *
 *     prefer quantum::error_rate = error_goal;
 *
 * The grammar does NOT define:
 *
 *     gates;
 *     qubits;
 *     coupling maps;
 *     routing;
 *     calibration;
 *     QEC;
 *     physical topology.
 *
 * If a preference influences quantum compilation, its semantic path is:
 *
 *     preference
 *       ->
 *     semantic resource model
 *       ->
 *     quantum semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * This grammar remains independent of that realization pipeline.
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-related preferences may use symbolic names such as:
 *
 *     performance
 *     power
 *     energy
 *     timing
 *     reliability
 *     portability
 *     scalability
 *     hardware::throughput
 *
 * The grammar does not select a board, device, FPGA, ASIC, or vendor.
 *
 * HDL synthesis and physical realization remain downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backends consume semantic preference information after:
 *
 *     parsing;
 *     AST construction;
 *     semantic analysis;
 *     type analysis;
 *     resource analysis;
 *     capability analysis;
 *     policy analysis.
 *
 * This grammar has no backend dependency.
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no I/O;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no environment inspection;
 *     no random behavior.
 *
 * Parsing therefore depends solely on the token stream.
 *
 * The same source and lexical configuration must produce the same parse tree.
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The public rules:
 *
 *     resourcePreferenceExpression
 *     resourcePreferenceSpecification
 *     resourcePreferenceClause
 *
 * form the stable component API.
 *
 * The concrete `prefer` statement remains in resources.g4.
 *
 * Future preference dimensions should normally be introduced semantically,
 * through metadata/registries/dialects, rather than by changing this grammar.
 *
 * A new standardized reserved keyword is compatibility-sensitive because it
 * may change identifier interpretation elsewhere.
 *
 * Therefore adding a new preference dimension MUST NOT automatically require
 * a new reserved keyword.
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 * The test suite must parse:
 *
 *     prefer latency <= latency_goal;
 *
 *     prefer throughput >= desired_throughput;
 *
 *     prefer energy <= energy_goal;
 *
 *     prefer performance = target_performance;
 *
 *     prefer quantum::fidelity = target_fidelity;
 *
 *     prefer hardware::throughput = desired_throughput;
 *
 *     prefer {
 *         objective = latency_goal;
 *         priority = priority_value;
 *         when = workload_size > threshold;
 *     };
 *
 *
 * NEGATIVE TESTS
 * -------------
 *
 * Must reject:
 *
 *     prefer;
 *
 *     prefer latency;
 *
 *     prefer =;
 *
 *     prefer latency =;
 *
 *     prefer {;
 *
 *     prefer { latency = goal;
 *
 *     prefer latency = goal
 *
 * where the enclosing grammar requires the statement terminator.
 *
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Test:
 *
 *     keyword resource names;
 *     identifier resource names;
 *     qualified names;
 *     symbolic values;
 *     very large numeric literals;
 *     deeply nested resource expressions;
 *     empty preference blocks;
 *     multiple clauses;
 *     multiple groups;
 *     nested groups where supported;
 *     quantum resource names;
 *     HDL resource names;
 *     distributed resource names;
 *     future namespaces.
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Verify that grammar correctness does not depend on:
 *
 *     number of preferences;
 *     number of resource dimensions;
 *     number of clauses;
 *     name depth;
 *     expression magnitude;
 *     hardware scale;
 *     quantum scale;
 *     node count.
 *
 * Practical test-resource limits are allowed.
 *
 * Language-level artificial limits are not.
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Preferences must be usable with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     AI;
 *     distributed;
 *     networking;
 *     data;
 *     interoperability;
 *     simulation.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Repeated parsing of identical input must produce equivalent parse structure
 * and diagnostics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     fixed CPU capacity;
 *     fixed GPU capacity;
 *     fixed FPGA capacity;
 *     fixed QPU capacity;
 *     fixed node capacity;
 *     fixed memory capacity;
 *     fixed thread capacity;
 *     fixed tensor-rank capacity;
 *     fixed register width;
 *     fixed network size;
 *     fixed device count;
 *     fixed preference count.
 *
 * ALLOWED:
 *
 *     finite lexical keyword vocabulary;
 *     standardized semantic property names;
 *     unbounded symbolic names;
 *     arbitrary resource-expression values.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] ResourcePreferences has one grammar authority.
 *
 * [ ] `resourcePreferenceExpression` is owned here.
 *
 * [ ] `resources.g4` owns the outer `resourcePreference` statement.
 *
 * [ ] `resourceExpression` is not redefined here.
 *
 * [ ] `identifier` is not redefined here.
 *
 * [ ] `qualifiedName` is not redefined here.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] No hardware resource is selected here.
 *
 * [ ] No physical allocation occurs here.
 *
 * [ ] No target backend is referenced here.
 *
 * [ ] Preferences remain distinct from requirements.
 *
 * [ ] Preferences remain distinct from constraints.
 *
 * [ ] Preferences remain distinct from capabilities.
 *
 * [ ] Preferences remain distinct from budgets.
 *
 * [ ] Preferences remain distinct from hints.
 *
 * [ ] Preferences remain distinct from policies.
 *
 * [ ] Preference values use canonical resource expressions.
 *
 * [ ] Open-world preference namespaces remain representable.
 *
 * [ ] No finite hardware limit is encoded.
 *
 * [ ] Quantum syntax remains target-neutral.
 *
 * [ ] HDL/hardware syntax remains target-neutral.
 *
 * [ ] AST ownership is external to this grammar.
 *
 * [ ] Semantic ownership is external to this grammar.
 *
 * [ ] IR ownership is external to this grammar.
 *
 * [ ] Provenance remains source-traceable.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Rust 1.97 / 1.97.1 generated-parser integration is verified.
 *
 * [ ] Generated code requires no application-level unsafe Rust.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */