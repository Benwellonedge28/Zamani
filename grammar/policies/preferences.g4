/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/preferences.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * PolicyPreferences
 *
 * STATUS
 * ------
 * CANONICAL POLICY-PREFERENCE PAYLOAD GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the reusable SOURCE-SYNTAX PAYLOAD for policy preferences.
 *
 * A policy preference expresses a desirable choice among otherwise
 * semantically valid alternatives.
 *
 * A preference is ADVISORY.
 *
 * It is not automatically:
 *
 *     a requirement;
 *     a constraint;
 *     a capability;
 *     a permission;
 *     a prohibition;
 *     a budget;
 *     a resource allocation;
 *     a target selection;
 *     a scheduling command;
 *     a hardware assignment;
 *     a runtime command.
 *
 * The outer policy statement:
 *
 *     prefer ... ;
 *
 * remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * This file owns what appears after the policy-level PREFER keyword.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The policy preference pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     grammar/policies/policy.g4
 *       |
 *       v
 *     PolicyPreferences
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic policy model
 *       |
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> effect analysis
 *       +--> contract analysis
 *       +--> security analysis
 *       +--> execution planning
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> other domain IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * This file participates only in SOURCE PARSING.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     policyPreferenceExpression
 *     policyPreferenceSpecification
 *     policyPreferenceClause
 *     policyPreferenceAssignment
 *     policyPreferenceObjective
 *     policyPreferenceOrdering
 *     policyPreferenceCondition
 *     policyPreferenceProperty
 *     policyPreferenceGroup
 *     policyPreferenceGroupBody
 *
 *     policyPreferenceKey
 *     policyPreferenceValue
 *     policyPreferenceConditionValue
 *
 *     policyPreferenceClauseList
 *     policyPreferenceValueList
 *     policyPreferenceKeyList
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyPreference
 *     PREFER statement syntax
 *
 *     general expressions
 *     identifiers
 *     qualified names
 *     operators
 *     literals
 *
 *     resource preferences
 *     resource expressions
 *     resource requirements
 *     resource constraints
 *     capabilities
 *     budgets
 *     hints
 *     negotiation
 *
 *     security authorization
 *     permission semantics
 *     prohibition semantics
 *
 *     contracts
 *     effects
 *     provenance semantics
 *
 *     hardware discovery
 *     device selection
 *     placement
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *
 *
 * ============================================================================
 * IMPORTANT DISTINCTION FROM grammar/resources/preferences.g4
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/resources/preferences.g4
 *
 * That file owns RESOURCE-PREFERENCE PAYLOAD syntax.
 *
 * This file owns POLICY-PREFERENCE PAYLOAD syntax.
 *
 * They are intentionally separate semantic concepts.
 *
 *
 * RESOURCE PREFERENCE
 * -------------------
 *
 * Expresses desirable resource-realization characteristics.
 *
 * Example:
 *
 *     prefer latency <= latency_goal;
 *
 *
 * POLICY PREFERENCE
 * -----------------
 *
 * Expresses desirable policy behavior or ordering.
 *
 * Example:
 *
 *     policy execution {
 *         prefer execution::deterministic;
 *     }
 *
 *
 * The two concepts may interact semantically, but they must not become two
 * competing spellings for the same AST node.
 *
 * Resource preference semantics remain owned by:
 *
 *     grammar/resources/preferences.g4
 *
 * Policy preference semantics remain owned by:
 *
 *     grammar/policies/preferences.g4
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY policy-preference payload grammar.
 *
 * grammar/policies/policy.g4 MUST delegate its policyPreference payload to
 * this grammar rather than reproducing the payload rules.
 *
 * No other domain grammar may recreate policy-preference syntax.
 *
 * Domain grammars may consume the resulting semantic policy preference.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 * Via:
 *
 *     tokenVocab = ZamaniLexer
 *     Names
 *     Expressions
 *
 *
 * EXPORTS
 * -------
 *
 *     policyPreferenceExpression
 *     policyPreferenceSpecification
 *     policyPreferenceClause
 *     policyPreferenceAssignment
 *     policyPreferenceObjective
 *     policyPreferenceOrdering
 *     policyPreferenceCondition
 *     policyPreferenceProperty
 *     policyPreferenceGroup
 *     policyPreferenceGroupBody
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/policies/policy.g4
 *
 * Future policy adapters may consume these rules through grammar composition,
 * but must not redefine them.
 *
 *
 * AST_OWNER
 * ---------
 *
 * Domain-neutral Zamani frontend AST.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Policy semantic analysis.
 *
 *
 * IR_OWNER
 * --------
 *
 * Canonical semantic model and downstream domain IR owners.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/policies/preferences/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/policies.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * This grammar MUST remain open-ended.
 *
 * It MUST NOT enumerate:
 *
 *     CPU models
 *     GPU models
 *     FPGA families
 *     ASIC families
 *     QPU models
 *     quantum gates
 *     device counts
 *     node counts
 *     thread counts
 *     memory limits
 *     register widths
 *     tensor ranks
 *     network sizes
 *     topology sizes
 *     vendor catalogues
 *     finite preference dimensions
 *
 * Preference keys are symbolic language values.
 *
 * New policy dimensions can therefore be introduced through identifiers and
 * qualified names without changing the universal grammar.
 *
 * Examples:
 *
 *     execution::deterministic
 *     execution::reproducible
 *     quantum::resilience
 *     quantum::fidelity
 *     hardware::energy_efficiency
 *     deployment::portability
 *     ai::explainability
 *     future::policy_dimension
 *
 * The grammar does not need a new rule for each such concept.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Preferences express PORTABLE INTENT.
 *
 * They MUST NOT encode physical target identity.
 *
 * For example:
 *
 *     prefer execution::parallel;
 *
 * does not mean:
 *
 *     use CPU 0;
 *     use GPU 0;
 *     use device 0;
 *     use node 0.
 *
 * Instead, downstream compilation may determine the appropriate realization
 * according to:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     policies
 *     effects
 *     contracts
 *     target availability
 *     optimization
 *     scheduling
 *     runtime state
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No parser-level finite maximum is imposed on:
 *
 *     preference clauses
 *     preference groups
 *     policy properties
 *     preference nesting
 *     qualified-name depth
 *     expression complexity
 *     preference collections
 *
 * Actual implementation limits may arise from:
 *
 *     available memory
 *     available processing time
 *     parser implementation limits
 *     operating-system limits
 *     build-system limits
 *
 * Such limits are implementation/resource constraints, not language ceilings.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text
 *     canonical lexical vocabulary
 *     grammar version
 *     explicitly selected compatibility configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     runtime scheduling
 *
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar PolicyPreferences;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * 1. PUBLIC PAYLOAD ROOT
 * ============================================================================
 *
 * This rule is deliberately NOT the outer policyPreference statement.
 *
 * grammar/policies/policy.g4 owns:
 *
 *     policyPreference
 *         : PREFER policyPreferenceExpression SEMICOLON
 *         ;
 *
 * This grammar owns:
 *
 *     policyPreferenceExpression
 *
 * ============================================================================
 */

policyPreferenceExpression
    : policyPreferenceSpecification
    | policyPreferenceAssignment
    | policyPreferenceObjective
    | policyPreferenceOrdering
    | policyPreferenceCondition
    | policyPreferenceProperty
    | expression
    ;


/*
 * ============================================================================
 * 2. STRUCTURED PREFERENCE SPECIFICATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     prefer {
 *         objective = execution::deterministic;
 *         priority = execution::priority;
 *         condition = execution::available;
 *     };
 *
 * The outer PREFER and terminating semicolon belong to policy.g4.
 *
 * The contents are owned here.
 *
 * ============================================================================
 */

policyPreferenceSpecification
    : LBRACE
      policyPreferenceClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. PREFERENCE CLAUSE
 * ============================================================================
 *
 * A preference specification is an ordered collection.
 *
 * There is no fixed clause count.
 *
 * ============================================================================
 */

policyPreferenceClause
    : policyPreferenceAssignment
    | policyPreferenceObjective
    | policyPreferenceOrdering
    | policyPreferenceCondition
    | policyPreferenceProperty
    | policyPreferenceGroup
    ;


/*
 * ============================================================================
 * 4. GENERIC PREFERENCE ASSIGNMENT
 * ============================================================================
 *
 * This is the primary open-world extension mechanism.
 *
 * Examples:
 *
 *     deterministic = true;
 *
 *     reproducible = true;
 *
 *     explainability = required_level;
 *
 *     execution::mode = preferred_mode;
 *
 *     quantum::resilience = desired_resilience;
 *
 *     future::policy::dimension = desired_value;
 *
 * The key is not interpreted by the parser.
 *
 * Semantic analysis determines whether the key is:
 *
 *     standardized;
 *     dialect-defined;
 *     vendor-defined;
 *     experimental;
 *     deprecated;
 *     unknown.
 *
 * ============================================================================
 */

policyPreferenceAssignment
    : policyPreferenceKey
      ASSIGN
      policyPreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. OBJECTIVE
 * ============================================================================
 *
 * OBJECTIVE is already a canonical lexer token in the repository.
 *
 * Therefore it must be accepted explicitly rather than pretending it is an
 * ordinary identifier.
 *
 * Example:
 *
 *     objective = execution::deterministic;
 *
 * ============================================================================
 */

policyPreferenceObjective
    : OBJECTIVE
      ASSIGN
      policyPreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. ORDERING
 * ============================================================================
 *
 * Policy ordering metadata remains OPEN-WORLD.
 *
 * The repository currently does not establish separate canonical lexer tokens
 * for every possible ordering vocabulary such as:
 *
 *     priority
 *     weight
 *     rank
 *     order
 *
 * Therefore those names remain identifiers.
 *
 * This rule accepts any symbolic ordering key.
 *
 * Examples:
 *
 *     priority = preferred_priority;
 *
 *     weight = preference_weight;
 *
 *     rank = preferred_rank;
 *
 *     ordering::class = preferred_class;
 *
 * No finite numeric priority range is encoded.
 *
 * ============================================================================
 */

policyPreferenceOrdering
    : policyPreferenceOrderingKey
      ASSIGN
      policyPreferenceValue
      SEMICOLON
    ;


policyPreferenceOrderingKey
    : identifier
    ;


/*
 * ============================================================================
 * 7. CONDITIONAL PREFERENCE
 * ============================================================================
 *
 * A preference may be conditional.
 *
 * Example:
 *
 *     when = workload::large;
 *
 * The expression is parsed but never evaluated by this grammar.
 *
 * Semantic analysis determines:
 *
 *     type compatibility;
 *     scope;
 *     availability;
 *     policy interaction;
 *     provenance;
 *     applicability.
 *
 * ============================================================================
 */

policyPreferenceCondition
    : WHEN
      ASSIGN
      policyPreferenceConditionValue
      SEMICOLON
    | policyPreferenceConditionKey
      ASSIGN
      policyPreferenceConditionValue
      SEMICOLON
    ;


policyPreferenceConditionKey
    : identifier
    ;


policyPreferenceConditionValue
    : expression
    ;


/*
 * ============================================================================
 * 8. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * This rule exists so policy preference vocabulary does not become a closed
 * keyword catalogue.
 *
 * Example:
 *
 *     explainability = preferred_level;
 *
 *     provenance::detail = desired_level;
 *
 *     adaptation::conservatism = preferred_value;
 *
 *     vendor::policy::mode = preferred_mode;
 *
 * The semantic layer decides whether the property is meaningful.
 *
 * ============================================================================
 */

policyPreferenceProperty
    : policyPreferenceKey
      ASSIGN
      policyPreferenceValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PREFERENCE KEY
 * ============================================================================
 *
 * Preference keys are intentionally open-world.
 *
 * A key may be:
 *
 *     identifier
 *     qualifiedName
 *
 * The qualified-name alternative is handled by the Names grammar.
 *
 * Because ANTLR tokenization reserves some universal vocabulary, selected
 * canonical keywords that can reasonably occur as preference dimensions are
 * admitted explicitly.
 *
 * This is NOT a finite semantic preference catalogue.
 *
 * It is only lexical compatibility for already-reserved language words.
 * ============================================================================
 */

policyPreferenceKey
    : qualifiedName
    | OBJECTIVE
    | PREFERENCE
    | identifier
    ;


/*
 * ============================================================================
 * 10. PREFERENCE VALUE
 * ============================================================================
 *
 * Values use the canonical Zamani expression grammar.
 *
 * This grammar MUST NOT create another expression language.
 *
 * Therefore preference values may eventually represent:
 *
 *     literals
 *     names
 *     calls
 *     arithmetic
 *     comparisons
 *     logical expressions
 *     collections
 *     symbolic values
 *     computed values
 *     domain values
 *     capability expressions
 *     resource-derived values
 *     runtime-derived values
 *
 * according to the capabilities of the canonical expression grammar.
 *
 * ============================================================================
 */

policyPreferenceValue
    : expression
    ;


/*
 * ============================================================================
 * 11. GROUP
 * ============================================================================
 *
 * A group provides semantic organization for related preferences.
 *
 * It does NOT represent a physical hardware group.
 *
 * It does NOT select:
 *
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     nodes
 *     devices
 *
 * Example:
 *
 *     group execution {
 *         objective = execution::deterministic;
 *         priority = preferred_priority;
 *     }
 *
 * GROUP is already a canonical lexical token.
 *
 * ============================================================================
 */

policyPreferenceGroup
    : GROUP
      policyPreferenceGroupName
      policyPreferenceGroupBody
    ;


policyPreferenceGroupName
    : qualifiedName
    | identifier
    ;


policyPreferenceGroupBody
    : LBRACE
      policyPreferenceClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. REUSABLE LIST RULES
 * ============================================================================
 *
 * These rules provide composition points for future grammar adapters without
 * imposing finite cardinality.
 * ============================================================================
 */

policyPreferenceClauseList
    : policyPreferenceClause*
    ;


policyPreferenceKeyList
    : policyPreferenceKey
      (COMMA policyPreferenceKey)*
    ;


policyPreferenceValueList
    : policyPreferenceValue
      (COMMA policyPreferenceValue)*
    ;


/*
 * ============================================================================
 * 13. OPTIONAL SPECIFICATION
 * ============================================================================
 */

optionalPolicyPreferenceSpecification
    : policyPreferenceSpecification?
    ;


/*
 * ============================================================================
 * 14. EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future dialects may consume the generic assignment/property structure.
 *
 * They must not modify this grammar merely because a new policy preference
 * property has been introduced.
 *
 * Example:
 *
 *     vendor::scheduler::policy = preferred_value;
 *
 * remains valid without adding a new parser rule.
 *
 * Semantic registration belongs downstream.
 *
 * ============================================================================
 */

policyPreferenceExtension
    : policyPreferenceAssignment
    ;


/*
 * ============================================================================
 * 15. RESOURCE-PREFERENCE BOUNDARY
 * ============================================================================
 *
 * Resource preferences are intentionally NOT imported here.
 *
 * The repository's resource preference grammar is:
 *
 *     grammar/resources/preferences.g4
 *
 * and its owner is:
 *
 *     ResourcePreferences
 *
 * Resource-specific preference syntax must remain there.
 *
 * A policy can semantically reference or govern resource preferences through
 * normal policy expressions.
 *
 * This prevents a second resource-preference parser from being created.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. CONTRACT BOUNDARY
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * They do not become contracts merely because a contract-related expression
 * occurs as their value.
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * The semantic layer determines interactions among:
 *
 *     preference
 *     requirement
 *     constraint
 *     contract
 *     policy
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. CAPABILITY BOUNDARY
 * ============================================================================
 *
 * A preference may reference capabilities through an expression.
 *
 * Example:
 *
 *     capability::quantum::measurement = preferred;
 *
 * This grammar does not:
 *
 *     discover capabilities;
 *     verify capabilities;
 *     authorize capabilities;
 *     allocate capabilities.
 *
 * Those responsibilities remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. EFFECT BOUNDARY
 * ============================================================================
 *
 * A preference may reference effect-related policy values.
 *
 * Example:
 *
 *     effect::network = disfavored;
 *
 * The grammar does not infer or validate effects.
 *
 * Effect semantics remain owned by:
 *
 *     grammar/effects/
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Preference source locations must remain available to the AST/provenance
 * subsystem.
 *
 * This grammar itself creates no provenance records.
 *
 * The frontend must preserve:
 *
 *     source file
 *     source span
 *     policy identity
 *     preference ordering
 *     nested group structure
 *
 * so semantic and compiler provenance can later explain why a realization was
 * selected.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum preferences remain target-neutral.
 *
 * Examples:
 *
 *     prefer quantum::fidelity;
 *
 *     prefer quantum::resilience;
 *
 *     prefer quantum::dynamic_control;
 *
 *     prefer {
 *         objective = quantum::fidelity;
 *         priority = quantum::resilience_priority;
 *     };
 *
 * These are policy preferences only.
 *
 * This grammar MUST NOT:
 *
 *     enumerate quantum operations;
 *     identify physical qubits;
 *     identify QPU topology;
 *     select calibration data;
 *     select routing paths;
 *     select QEC codes;
 *     construct quantum::ir;
 *
 * The canonical quantum boundary remains:
 *
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Policy preferences may express hardware-related intent:
 *
 *     hardware::energy_efficiency
 *     hardware::reliability
 *     hardware::latency
 *     hardware::portability
 *
 * They MUST NOT encode a universal hardware model.
 *
 * No rule here may require:
 *
 *     a fixed register width;
 *     a fixed number of devices;
 *     a fixed memory size;
 *     a fixed number of cores;
 *     a fixed number of hardware resources.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. AI / REASONING / LEARNING BOUNDARY
 * ============================================================================
 *
 * Policy preferences may govern universal computational behavior such as:
 *
 *     explainability
 *     reproducibility
 *     evidence quality
 *     adaptation behavior
 *     learning behavior
 *     uncertainty handling
 *     decision transparency
 *
 * Example:
 *
 *     prefer ai::explainability;
 *
 *     prefer {
 *         objective = ai::reproducibility;
 *         evidence::quality = evidence_goal;
 *     };
 *
 * No application-specific AI keyword catalogue is required.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. DISTRIBUTED / NETWORKING BOUNDARY
 * ============================================================================
 *
 * Policy preferences may express:
 *
 *     distributed::locality
 *     distributed::resilience
 *     network::latency
 *     network::reliability
 *     network::security
 *
 * These remain symbolic.
 *
 * This grammar does not:
 *
 *     discover nodes;
 *     enumerate endpoints;
 *     choose routes;
 *     allocate network resources;
 *     impose a maximum node count.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. ADAPTIVE EXECUTION BOUNDARY
 * ============================================================================
 *
 * Preferences may influence adaptive execution.
 *
 * Example:
 *
 *     prefer execution::recover;
 *
 *     prefer {
 *         objective = execution::reliability;
 *         fallback::strategy = preferred_strategy;
 *     };
 *
 * Actual behavior is determined by:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     policies
 *     execution planning
 *     runtime state
 *     resilience semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * Preferences may request desirable properties such as:
 *
 *     execution::deterministic
 *     execution::reproducible
 *
 * They do not prove those properties.
 *
 * Verification belongs to semantic analysis and downstream execution/compile
 * systems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. SEMANTIC NORMALIZATION CONTRACT
 * ============================================================================
 *
 * The semantic layer should normalize every policy preference into a common
 * representation conceptually equivalent to:
 *
 *     PolicyPreference {
 *         key
 *         value
 *         objective
 *         ordering
 *         condition
 *         group
 *         source
 *         provenance
 *     }
 *
 * The exact Rust AST type is owned by the Rust frontend and is intentionally
 * NOT embedded in this grammar.
 *
 * Multiple syntactic forms may therefore converge on one semantic model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. PREFERENCE VS REQUIREMENT
 * ============================================================================
 *
 * The semantic implementation MUST preserve this distinction:
 *
 *     requires X
 *
 * means:
 *
 *     X is necessary.
 *
 *     prefer X
 *
 * means:
 *
 *     X is desirable when feasible.
 *
 * A preference MUST NOT silently be upgraded into a requirement because it is
 * difficult to satisfy.
 *
 * Conversely, a requirement MUST NOT be weakened into a preference because
 * the preferred realization is unavailable.
 *
 * Explicit policy semantics determine conflict resolution.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. PREFERENCE VS CONSTRAINT
 * ============================================================================
 *
 * Constraint:
 *
 *     must remain true.
 *
 * Preference:
 *
 *     is desirable.
 *
 * Example:
 *
 *     constraint execution::deterministic;
 *
 * versus:
 *
 *     prefer execution::deterministic;
 *
 * The parser deliberately does not collapse these constructs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. PREFERENCE VS PROHIBITION
 * ============================================================================
 *
 * A preference does not prohibit alternatives.
 *
 * For example:
 *
 *     prefer execution::local;
 *
 * does not mean:
 *
 *     forbid execution::distributed;
 *
 * If prohibition is intended, the explicit policy prohibition construct must
 * be used.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. PREFERENCE VS PERMISSION
 * ============================================================================
 *
 * A preference does not grant authority.
 *
 * For example:
 *
 *     prefer network::local;
 *
 * does not grant network access.
 *
 * Authorization remains owned by the security/policy authorization layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. CONFLICT RESOLUTION BOUNDARY
 * ============================================================================
 *
 * This grammar does not decide which preference wins.
 *
 * Semantic policy resolution may consider:
 *
 *     explicit ordering;
 *     policy scope;
 *     specificity;
 *     applicability;
 *     requirements;
 *     constraints;
 *     capabilities;
 *     resources;
 *     permissions;
 *     prohibitions;
 *     contracts;
 *     deployment context;
 *     compatibility;
 *     provenance.
 *
 * Resolution MUST remain deterministic for the same semantic inputs and
 * explicitly selected policy configuration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SOURCE ORDER
 * ============================================================================
 *
 * Source order is semantically significant only where the policy-resolution
 * specification explicitly says so.
 *
 * This grammar preserves source order through parser context ordering.
 *
 * It does not assign implicit priority based solely on parser position.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. AST CONTRACT
 * ============================================================================
 *
 * Required semantic information preserved by the frontend:
 *
 *     policy identity
 *     preference key
 *     preference value
 *     objective
 *     ordering metadata
 *     condition
 *     group membership
 *     source span
 *     source ordering
 *
 * The AST MUST remain domain-neutral.
 *
 * It MUST NOT contain:
 *
 *     physical CPU IDs;
 *     physical GPU IDs;
 *     physical QPU IDs;
 *     physical FPGA IDs;
 *     vendor routing commands;
 *     calibration identifiers;
 *     backend instruction sequences.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. TYPE CONTRACT
 * ============================================================================
 *
 * Preference values are canonical expressions.
 *
 * Their types are therefore checked by the normal Zamani type system.
 *
 * Examples:
 *
 *     boolean preference
 *     numeric preference
 *     symbolic preference
 *     capability-related preference
 *     resource-related preference
 *     structured preference
 *
 * This grammar does not impose a universal type on all preferences.
 *
 * The semantic owner determines the required type for a particular key.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a preference has no runtime effect.
 *
 * Preference evaluation may later influence execution planning, but this
 * grammar itself does not:
 *
 *     perform I/O;
 *     mutate state;
 *     access hardware;
 *     access the network;
 *     execute code;
 *     invoke foreign functions;
 *     perform quantum operations;
 *     perform learning;
 *     perform adaptation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. CAPABILITY CONTRACT
 * ============================================================================
 *
 * A preference may mention a capability symbol.
 *
 * The presence of that symbol does not prove capability availability.
 *
 * Capability checking occurs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. RESOURCE CONTRACT
 * ============================================================================
 *
 * This file introduces no physical resource limit.
 *
 * Resource preference semantics remain separately owned by:
 *
 *     grammar/resources/preferences.g4
 *
 * Policy preferences can govern resource preference behavior semantically,
 * but must not recreate resource syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. POLICY CONTRACT
 * ============================================================================
 *
 * This file is itself a component of the policy system.
 *
 * Its output can be affected by:
 *
 *     policy scope
 *     permission
 *     prohibition
 *     requirement
 *     constraint
 *     fallback
 *     security
 *     compatibility
 *
 * The grammar only parses the preference payload.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Preference provenance should permit downstream tooling to answer:
 *
 *     Which source declared this preference?
 *     Which policy contained it?
 *     Which condition made it applicable?
 *     Which semantic rule interpreted it?
 *     Which realization was selected?
 *     Which alternatives were rejected?
 *     Which constraints affected the decision?
 *
 * The parser preserves source structure required to answer these questions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Required parser diagnostics include:
 *
 *     missing preference value;
 *     missing assignment operator;
 *     missing semicolon;
 *     malformed structured specification;
 *     malformed group;
 *     malformed condition;
 *     malformed ordering expression;
 *     malformed preference key;
 *     unterminated preference group.
 *
 * Diagnostics must preserve source locations.
 *
 * Semantic diagnostics are downstream and include:
 *
 *     unknown preference property where prohibited by policy;
 *     incompatible preference value;
 *     contradictory preference;
 *     preference/requirement conflict;
 *     preference/constraint conflict;
 *     prohibited preference;
 *     unavailable capability;
 *     impossible realization.
 *
 *
 * ============================================================================
 * 41. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms MUST be accepted by the policy grammar once
 * policy.g4 delegates its preference payload here:
 *
 *     policy portable {
 *         prefer execution::deterministic;
 *     }
 *
 *     policy reproducible {
 *         prefer execution::reproducible;
 *     }
 *
 *     policy quantum {
 *         prefer quantum::resilience;
 *     }
 *
 *     policy hybrid {
 *         prefer hybrid::classical_control;
 *     }
 *
 *     policy governed {
 *         prefer ai::explainability;
 *     }
 *
 *     policy structured {
 *         prefer {
 *             objective = execution::deterministic;
 *             priority = preferred_priority;
 *             condition = workload::stable;
 *         };
 *     }
 *
 *     policy grouped {
 *         prefer {
 *             group execution {
 *                 objective = execution::reproducible;
 *                 priority = preferred_priority;
 *             }
 *         };
 *     }
 *
 *     policy future {
 *         prefer future::policy::dimension;
 *     }
 *
 *
 * ============================================================================
 * 42. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following MUST fail structurally:
 *
 *     policy invalid {
 *         prefer;
 *     }
 *
 *     policy invalid {
 *         prefer =;
 *     }
 *
 *     policy invalid {
 *         prefer {
 *     }
 *
 *     policy invalid {
 *         prefer {
 *             objective;
 *         };
 *     }
 *
 *     policy invalid {
 *         prefer {
 *             priority =;
 *         };
 *     }
 *
 *     policy invalid {
 *         prefer {
 *             group {
 *         };
 *     }
 *
 * Exact diagnostic wording belongs to the frontend diagnostic subsystem.
 *
 *
 * ============================================================================
 * 43. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations including:
 *
 *     preference + requirement
 *     preference + constraint
 *     preference + capability
 *     preference + resource
 *     preference + prohibition
 *     preference + permission
 *     preference + fallback
 *     preference + adaptation
 *     preference + simulation
 *     preference + provenance
 *     preference + contract
 *     preference + quantum
 *     preference + HDL
 *     preference + AI
 *     preference + distributed execution
 *     preference + networking
 *
 * The grammar must remain target-neutral in every case.
 *
 *
 * ============================================================================
 * 44. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST verify that the grammar accepts, subject only to implementation
 * resources:
 *
 *     many preference clauses;
 *     deeply qualified preference keys;
 *     large preference groups;
 *     large expressions;
 *     many independent policies;
 *     many nested policy scopes;
 *     symbolic resource-independent values;
 *     future domain namespaces.
 *
 * No test may assume a universal finite capacity.
 *
 *
 * ============================================================================
 * 45. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At minimum test:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     simulation
 *     interoperability
 *     metaprogramming
 *
 * A policy preference must remain syntactically domain-neutral.
 *
 *
 * ============================================================================
 * 46. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Compatibility aliases belong under:
 *
 *     grammar/compatibility/
 *
 * This grammar must not duplicate historical token definitions.
 *
 * If an older spelling maps to a canonical preference concept, compatibility
 * handling must occur through the repository's compatibility architecture.
 *
 *
 * ============================================================================
 * 47. RUST CONTRACT
 * ============================================================================
 *
 * This file contains:
 *
 *     no Rust;
 *     no embedded actions;
 *     no semantic predicates;
 *     no unsafe implementation requirement;
 *     no target-specific code.
 *
 * The generated parser must be consumable by the Rust frontend using:
 *
 *     Rust 1.97+
 *
 * and safe Rust.
 *
 *
 * ============================================================================
 * 48. INTEGRATION WITH grammar/policies/policy.g4
 * ============================================================================
 *
 * REQUIRED CHANGE
 * ---------------
 *
 * Add:
 *
 *     PolicyPreferences
 *
 * to the import list:
 *
 *     import
 *         Names,
 *         Requirements,
 *         Constraints,
 *         Capabilities,
 *         Expressions,
 *         PolicyPreferences
 *     ;
 *
 *
 * Then replace the current policyPreference rule:
 *
 *     policyPreference
 *         : PREFER
 *           policyExpression
 *           SEMICOLON
 *         ;
 *
 * with:
 *
 *     policyPreference
 *         : PREFER
 *           policyPreferenceExpression
 *           SEMICOLON
 *         ;
 *
 * No second PREFER statement is created.
 *
 * `policy.g4` continues to own the outer policy member.
 *
 * `PolicyPreferences` owns the payload.
 *
 *
 * ============================================================================
 * 49. INTEGRATION WITH grammar/resources/preferences.g4
 * ============================================================================
 *
 * NO DIRECT IMPORT IS REQUIRED.
 *
 * The resource preference grammar remains independent.
 *
 * Existing ownership remains:
 *
 *     grammar/resources/preferences.g4
 *         -> ResourcePreferences
 *
 * Policy preference ownership:
 *
 *     grammar/policies/preferences.g4
 *         -> PolicyPreferences
 *
 * If a policy governs a resource preference, it should express that through
 * normal policy semantics rather than importing and duplicating resource
 * preference syntax.
 *
 *
 * ============================================================================
 * 50. INTEGRATION WITH grammar/resources/resources.g4
 * ============================================================================
 *
 * No change is required merely because this file is introduced.
 *
 * `resources.g4` must continue delegating resource preference syntax to:
 *
 *     ResourcePreferences
 *
 * It must NOT delegate resource preference syntax to PolicyPreferences.
 *
 *
 * ============================================================================
 * 51. INTEGRATION WITH LEXER
 * ============================================================================
 *
 * No new lexer token is required for this file.
 *
 * Existing canonical tokens used here include:
 *
 *     PREFER
 *     OBJECTIVE
 *     PREFERENCE
 *     GROUP
 *     WHEN
 *     ASSIGN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     SEMICOLON
 *
 * Ordering names such as:
 *
 *     priority
 *     weight
 *     rank
 *     order
 *
 * remain identifiers because the current canonical lexer does not require
 * them to be universal reserved words.
 *
 * This is intentional.
 *
 * Adding every policy vocabulary word as a reserved keyword would unnecessarily
 * shrink the identifier namespace and increase lexical coupling.
 *
 *
 * ============================================================================
 * 52. INTEGRATION WITH AST
 * ============================================================================
 *
 * `policyPreferenceExpression` must map to a domain-neutral preference node.
 *
 * Recommended semantic shape:
 *
 *     PolicyPreference
 *         key
 *         value
 *         objective
 *         ordering
 *         condition
 *         groups
 *         source
 *
 * Structured forms may normalize into this same semantic representation.
 *
 * No domain-specific AST is permitted here.
 *
 *
 * ============================================================================
 * 53. INTEGRATION WITH SEMANTICS
 * ============================================================================
 *
 * Semantic processing should perform:
 *
 *     parse
 *       ->
 *     normalize
 *       ->
 *     resolve policy scope
 *       ->
 *     type-check value
 *       ->
 *     validate applicability
 *       ->
 *     compare requirements
 *       ->
 *     compare constraints
 *       ->
 *     check capabilities
 *       ->
 *     check resources
 *       ->
 *     apply permissions/prohibitions
 *       ->
 *     resolve competing preferences
 *       ->
 *     record provenance
 *       ->
 *     produce canonical semantic intent
 *
 * Preference resolution must not occur inside ANTLR.
 *
 *
 * ============================================================================
 * 54. INTEGRATION WITH COMPILATION
 * ============================================================================
 *
 * Preferences can influence:
 *
 *     target-independent optimization;
 *     specialization;
 *     resource selection;
 *     execution planning;
 *     scheduling;
 *     deployment;
 *     simulation;
 *     fallback selection;
 *     resilience planning.
 *
 * They must not directly emit target instructions.
 *
 *
 * ============================================================================
 * 55. INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Policy preferences may influence the quantum semantic pipeline:
 *
 *     policy preference
 *       ->
 *     semantic policy
 *       ->
 *     quantum resource/capability analysis
 *       ->
 *     quantum semantic model
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
 * This grammar must never bypass `quantum::ir`.
 *
 *
 * ============================================================================
 * 56. INTEGRATION WITH HDL / HARDWARE
 * ============================================================================
 *
 * Policy preferences may influence hardware realization planning but never
 * become HDL netlist syntax or physical device commands.
 *
 * The downstream boundary remains:
 *
 *     semantic policy
 *       ->
 *     hardware intent
 *       ->
 *     HDL/hardware representation
 *       ->
 *     synthesis / lowering
 *       ->
 *     target realization
 *
 *
 * ============================================================================
 * 57. INTEGRATION WITH AI / KNOWLEDGE / REASONING
 * ============================================================================
 *
 * Policy preferences may govern:
 *
 *     reasoning strategy
 *     evidence preference
 *     explainability
 *     learning behavior
 *     adaptation behavior
 *     uncertainty handling
 *     decision transparency
 *     reproducibility
 *
 * These concepts remain generic semantic capabilities rather than an
 * application-specific keyword catalogue.
 *
 *
 * ============================================================================
 * 58. INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Every resolved preference should be traceable to:
 *
 *     source policy;
 *     source span;
 *     preference key;
 *     preference value;
 *     applicability condition;
 *     semantic resolution;
 *     selected realization;
 *     rejected alternatives where applicable.
 *
 * This enables explainability of compiler and execution decisions.
 *
 *
 * ============================================================================
 * 59. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST contain none of:
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
 * It also MUST NOT contain finite enumerations of:
 *
 *     hardware;
 *     quantum operations;
 *     vendors;
 *     processors;
 *     accelerators;
 *     network topologies;
 *     AI models;
 *     resource quantities.
 *
 *
 * ============================================================================
 * 60. SECURITY AUDIT
 * ============================================================================
 *
 * A preference MUST NOT grant authority.
 *
 * A preference MUST NOT bypass:
 *
 *     authorization;
 *     capability checks;
 *     sandbox policy;
 *     prohibition policy;
 *     resource checks;
 *     contract validation.
 *
 *
 * ============================================================================
 * 61. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It owns policy-preference PAYLOAD syntax.
 *
 * [x] It does not own the outer PREFER statement.
 *
 * [x] It does not duplicate resource preference syntax.
 *
 * [x] It uses the canonical Zamani expression grammar.
 *
 * [x] It uses canonical Names grammar.
 *
 * [x] It contains no target-specific semantics.
 *
 * [x] It contains no hardware limits.
 *
 * [x] It contains no quantum-operation catalogue.
 *
 * [x] It contains no Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is open-world for future preference properties.
 *
 * [x] It supports structured preference specifications.
 *
 * [x] It supports conditional preference metadata.
 *
 * [x] It supports ordering metadata without finite priority ranges.
 *
 * [x] It supports nested semantic groups.
 *
 * [x] It preserves a clear resource-preference boundary.
 *
 * [x] It has explicit AST, semantic, IR, provenance and backend boundaries.
 *
 * [x] It has integration instructions for policy.g4.
 *
 * [x] It has integration instructions for lexer, resources, semantics,
 *     quantum, HDL and hardware.
 *
 * [x] It is compatible with Rust 1.97+ through generated safe Rust.
 *
 * Repository verification still required:
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] `PolicyPreferences` resolves through the configured grammar library path.
 *
 * [ ] `Policy` imports `PolicyPreferences`.
 *
 * [ ] `policyPreference` delegates to `policyPreferenceExpression`.
 *
 * [ ] Existing resource preference tests remain unchanged and passing.
 *
 * [ ] Policy preference positive tests pass.
 *
 * [ ] Policy preference negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass within available implementation resources.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * The policy preference architecture is:
 *
 *     PREFER
 *       |
 *       v
 *     PolicyPreferences
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic policy preference
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> resources
 *       +--> permissions
 *       +--> prohibitions
 *       +--> contracts
 *       +--> effects
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL
 *       +--> distributed
 *       +--> future domains
 *
 * One policy-preference syntax.
 *
 * One semantic preference model.
 *
 * Unlimited future preference dimensions through open-world names.
 *
 * No physical resource ceiling.
 *
 * No target-specific parser logic.
 *
 * No competing resource-preference grammar.
 *
 * No unsafe implementation requirement.
 *
 * ============================================================================
 */