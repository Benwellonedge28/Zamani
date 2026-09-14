/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/scalability.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Grammar name:
 *     ResourceScalability
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SCALE-INTENT grammar boundary for Zamani.
 *
 * It describes how a program expresses scalability intent without encoding
 * the physical limits of the machine on which the program eventually runs.
 *
 * Scalability is therefore a PROPERTY OF COMPUTATIONAL INTENT, not a fixed
 * machine-size declaration.
 *
 * This grammar supports semantic descriptions such as:
 *
 *     scalability = input_size;
 *
 *     scalability = workload_size;
 *
 *     scalability = problem_size * parallelism;
 *
 *     scalability = dimensions;
 *
 *     scalability grows_with input_size;
 *
 *     scalability bounded_by available_capacity;
 *
 *     scalability independent_of machine_size;
 *
 *     scalability requires capability;
 *
 *     scalability prefers resource;
 *
 * The exact interpretation of these declarations belongs to semantic
 * analysis and downstream compilation/resource systems.
 *
 * This grammar MUST NOT decide:
 *
 *     how many CPUs exist;
 *     how many cores exist;
 *     how many GPUs exist;
 *     how many FPGAs exist;
 *     how many qubits exist;
 *     how much memory exists;
 *     how many nodes exist;
 *     what topology exists;
 *     what device is selected;
 *     what physical resource is allocated;
 *     what scheduler is used;
 *     what routing strategy is used;
 *     what backend is selected.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     resource/scalability syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     resource semantics      program semantics
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *             canonical semantic model
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *       compiler    scheduler   runtime
 *          |           |           |
 *          +-----------+-----------+
 *                      |
 *                      v
 *               target realization
 *
 * There is intentionally NO direct dependency from this grammar to:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling algorithms
 *     optimization algorithms
 *     hardware discovery
 *     runtime allocation
 *
 * Those systems consume semantic information after parsing.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - scalability intent composition;
 *   - scalability declarations;
 *   - scalability expressions;
 *   - scaling dimensions;
 *   - scaling relationships;
 *   - scaling direction;
 *   - scaling dependency expressions;
 *   - scalability bounds as semantic expressions;
 *   - scalability growth expressions;
 *   - scalability invariance expressions;
 *   - scalability adaptation intent;
 *   - scalability portability intent;
 *   - scalability extensibility hooks.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - literals;
 *   - general expressions;
 *   - arithmetic;
 *   - comparison precedence;
 *   - resource declarations;
 *   - resource quantities;
 *   - resource capabilities;
 *   - resource targets;
 *   - resource requirements;
 *   - resource constraints;
 *   - resource preferences;
 *   - hardware descriptions;
 *   - machine discovery;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - allocation;
 *   - deployment;
 *   - runtime execution;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - QEC;
 *   - ZQN;
 *   - simulation.
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * The following repository components already own broader concepts:
 *
 *     grammar/resources/resources.g4
 *         universal resource semantics
 *
 *     grammar/resources/resource-expressions.g4
 *         resource-expression composition
 *
 *     grammar/expressions/expressions.g4
 *         canonical expressions
 *
 *     grammar/expressions/binary.g4
 *         binary expression hierarchy
 *
 *     grammar/expressions/unary.g4
 *         unary expressions
 *
 *     grammar/expressions/assignment.g4
 *         assignment expressions
 *
 * This file MUST NOT redefine those concepts.
 *
 * In particular, this file MUST NOT define another:
 *
 *     expression
 *     resourceExpression
 *     arithmeticExpression
 *     comparisonExpression
 *     identifier
 *     literal
 *     resource
 *     capability
 *     target
 *
 * Instead, this grammar composes the canonical `resourceExpression` rule.
 *
 * ============================================================================
 * SCALABILITY PRINCIPLE
 * ============================================================================
 *
 * Zamani scalability means:
 *
 *     source semantics remain stable while realization scale changes.
 *
 * A valid program may therefore execute on:
 *
 *     one resource;
 *     many resources;
 *     one processor;
 *     many processors;
 *     one accelerator;
 *     many accelerators;
 *     one quantum processor;
 *     many quantum processors;
 *     one node;
 *     many nodes;
 *     local execution;
 *     distributed execution;
 *     embedded execution;
 *     cloud execution;
 *     future execution environments.
 *
 * The grammar imposes NO finite upper bound.
 *
 * It MUST NOT contain:
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
 *     MAX_ACCELERATORS
 *     MAX_CLUSTER_SIZE
 *     MAX_SCALE
 *     MAX_DIMENSIONS
 *
 * No equivalent hidden limit may be introduced through fixed alternatives.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Scalability syntax must therefore describe semantic relationships rather
 * than temporary machine characteristics.
 *
 * For example:
 *
 *     scalability = input_size;
 *
 * describes a dependency on program/input scale.
 *
 * It does NOT mean:
 *
 *     maximum input = N;
 *     machine = X;
 *     cores = N;
 *     qubits = N;
 *     devices = N.
 *
 * Similarly:
 *
 *     bounded_by available_capacity
 *
 * describes a relationship to execution-context capacity.
 *
 * The actual capacity is supplied by downstream compilation/runtime context.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * The following concepts MUST remain semantically distinct:
 *
 *     SCALE
 *         A dimension along which computation varies.
 *
 *     GROWTH
 *         A relationship between workload and resource demand.
 *
 *     BOUND
 *         A semantic restriction expressed as a relationship.
 *
 *     CAPACITY
 *         A property supplied by an execution environment.
 *
 *     REQUIREMENT
 *         A mandatory condition.
 *
 *     PREFERENCE
 *         An advisory optimization preference.
 *
 *     HINT
 *         An advisory implementation hint.
 *
 *     CONSTRAINT
 *         A condition that a realization must satisfy.
 *
 *     ADAPTATION
 *         Permission/intent to change realization while preserving semantics.
 *
 * This grammar preserves those distinctions syntactically where useful but
 * leaves their final interpretation to semantic analysis.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is designed to be imported by:
 *
 *     grammar/resources/resources.g4
 *
 * and potentially consumed by:
 *
 *     grammar/hardware/resources.g4
 *     grammar/quantum/quantum-resources.g4
 *     grammar/hybrid/hybrid-resources.g4
 *     grammar/distributed/...
 *     grammar/classical/...
 *     grammar/ai/...
 *
 * Domain grammars MAY specialize scalability syntax but MUST normalize it
 * into the universal resource/scalability semantic representation.
 *
 * Domain grammars MUST NOT create incompatible definitions of:
 *
 *     scaling;
 *     capacity;
 *     growth;
 *     scalability;
 *     portability.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains NO lexer rules.
 *
 * Tokens are supplied by the canonical Zamani lexer through tokenVocab.
 *
 * The following tokens are expected to be provided by that lexer:
 *
 *     IDENT
 *     K_SCALABILITY
 *     K_GROWS
 *     K_WITH
 *     K_AS
 *     K_FROM
 *     K_TO
 *     K_OVER
 *     K_ACCORDING
 *     K_TO
 *     K_BOUNDED
 *     K_BY
 *     K_INDEPENDENT
 *     K_OF
 *     K_DEPENDS
 *     K_ON
 *     K_ADAPTS
 *     K_ACCORDING
 *     K_AVAILABLE
 *     K_CAPACITY
 *     K_PRESERVES
 *     K_SEMANTICS
 *     K_PORTABLE
 *     K_ACROSS
 *     K_SCALE
 *     K_DIMENSION
 *     K_DIMENSIONS
 *     K_GROWTH
 *     K_BOUND
 *     K_LIMIT
 *     K_FACTOR
 *     K_RATE
 *     K_WHEN
 *     K_WHERE
 *     K_UNTIL
 *     K_IF
 *     K_THEN
 *
 * plus canonical punctuation:
 *
 *     ASSIGN
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *
 * and all tokens required by `resourceExpression`.
 *
 * IMPORTANT:
 *
 * Token names must be reconciled with the canonical lexer before generation.
 *
 * If the repository's lexer uses a different spelling for an equivalent
 * keyword/token, the lexer authority must define the canonical spelling.
 *
 * This file must not silently create lexer aliases.
 *
 * ============================================================================
 * IMPORT
 * ============================================================================
 */

parser grammar ResourceScalability;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A scalability section may contain zero or more scalability items.
 *
 * Repetition is intentionally unbounded at the grammar level.
 *
 * Actual parser resource limits, if any, are implementation configuration and
 * are not part of the language semantics.
 * ============================================================================
 */

scalabilitySection
    : scalabilityItem*
    ;


/*
 * ============================================================================
 * 2. SCALABILITY ITEM
 * ============================================================================
 */

scalabilityItem
    : scalabilityDeclaration
    | scalabilityRelationship
    | scalabilityGrowth
    | scalabilityBound
    | scalabilityDependency
    | scalabilityInvariance
    | scalabilityAdaptation
    | scalabilityPortability
    ;


/*
 * ============================================================================
 * 3. SCALABILITY DECLARATION
 * ============================================================================
 *
 * General form:
 *
 *     scalability = expression;
 *
 * Example:
 *
 *     scalability = input_size;
 *
 * The expression determines the semantic scaling quantity.
 *
 * No fixed upper bound is implied.
 * ============================================================================
 */

scalabilityDeclaration
    : K_SCALABILITY
      ASSIGN
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. NAMED SCALABILITY DECLARATION
 * ============================================================================
 *
 * Allows multiple independently named scalability dimensions.
 *
 * Example:
 *
 *     scalability input = input_size;
 *     scalability memory = workload_size;
 *
 * Names are symbolic and do not identify physical resources.
 * ============================================================================
 */

namedScalabilityDeclaration
    : K_SCALABILITY
      scalabilityName
      ASSIGN
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. SCALABILITY NAME
 * ============================================================================
 */

scalabilityName
    : identifier
    ;


/*
 * ============================================================================
 * 6. SCALABILITY EXPRESSION
 * ============================================================================
 *
 * This is deliberately a wrapper around the canonical resource expression.
 *
 * It does not introduce a second expression language.
 * ============================================================================
 */

scalabilityExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 7. SCALABILITY RELATIONSHIP
 * ============================================================================
 *
 * Describes how one scaling quantity relates to another.
 *
 * Examples:
 *
 *     scalability grows_with input_size;
 *
 *     scalability grows_with problem_size;
 *
 *     scalability scales_with parallelism;
 *
 * The semantic analyzer determines the precise relationship.
 * ============================================================================
 */

scalabilityRelationship
    : K_SCALABILITY
      scalabilityRelationshipOperator
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. SCALABILITY RELATIONSHIP OPERATOR
 * ============================================================================
 *
 * These are semantic relationship words rather than arithmetic operators.
 *
 * They must not encode a particular growth algorithm.
 * ============================================================================
 */

scalabilityRelationshipOperator
    : K_GROWS K_WITH
    | K_SCALE K_WITH
    | K_DEPENDS K_ON
    ;


/*
 * ============================================================================
 * 9. GROWTH RELATIONSHIP
 * ============================================================================
 *
 * Allows an explicit growth statement.
 *
 * Examples:
 *
 *     growth with input_size;
 *
 *     growth according_to workload;
 *
 * The expression remains canonical.
 * ============================================================================
 */

scalabilityGrowth
    : K_GROWTH
      K_WITH
      scalabilityExpression
      SEMICOLON
    | K_GROWTH
      K_ACCORDING
      K_TO
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. GROWTH RATE
 * ============================================================================
 *
 * A growth factor/rate remains an expression.
 *
 * Examples:
 *
 *     growth rate_expression;
 *
 *     growth factor_expression;
 *
 * No fixed numeric type or precision is imposed here.
 * ============================================================================
 */

scalabilityGrowthRate
    : K_GROWTH
      K_RATE
      scalabilityExpression
      SEMICOLON
    | K_GROWTH
      K_FACTOR
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. SCALABILITY BOUND
 * ============================================================================
 *
 * A bound is a relationship against another semantic expression.
 *
 * Example:
 *
 *     scalability bounded_by available_capacity;
 *
 * This does NOT define a fixed capacity.
 *
 * The capacity may be supplied dynamically by:
 *
 *     compiler context;
 *     target description;
 *     runtime context;
 *     hardware capability model;
 *     resource manager.
 * ============================================================================
 */

scalabilityBound
    : K_SCALABILITY
      K_BOUNDED
      K_BY
      scalabilityExpression
      SEMICOLON
    | K_BOUND
      K_BY
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. EXPLICIT LIMIT RELATIONSHIP
 * ============================================================================
 *
 * This is intentionally relational rather than a fixed numeric declaration.
 *
 * Examples:
 *
 *     scalability limit available_capacity;
 *
 *     limit according_to workload_budget;
 *
 * Semantic analysis determines whether the expression represents:
 *
 *     capacity;
 *     constraint;
 *     requirement;
 *     preference;
 *     policy.
 *
 * ============================================================================
 */

scalabilityLimit
    : K_LIMIT
      scalabilityExpression
      SEMICOLON
    | K_LIMIT
      K_ACCORDING
      K_TO
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. SCALABILITY DEPENDENCY
 * ============================================================================
 *
 * Expresses dependency on an input/workload/program quantity.
 *
 * Example:
 *
 *     scalability depends_on input_size;
 *
 * ============================================================================
 */

scalabilityDependency
    : K_SCALABILITY
      K_DEPENDS
      K_ON
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. SCALABILITY INVARIANCE
 * ============================================================================
 *
 * Allows the source author to state that a semantic property should not
 * change as realization scale changes.
 *
 * Example:
 *
 *     scalability independent_of machine_size;
 *
 * The identifier/expression remains symbolic.
 *
 * The grammar does not reserve `machine_size` or any finite machine model.
 * ============================================================================
 */

scalabilityInvariance
    : K_SCALABILITY
      K_INDEPENDENT
      K_OF
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. SCALABILITY ADAPTATION
 * ============================================================================
 *
 * Indicates that realization may adapt to available resources.
 *
 * Example:
 *
 *     scalability adapts according_to available_capacity;
 *
 * Adaptation is semantic permission/intent.
 *
 * It does not select a scheduler, allocator, backend, or topology.
 * ============================================================================
 */

scalabilityAdaptation
    : K_SCALABILITY
      K_ADAPTS
      K_ACCORDING
      K_TO
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. SCALABILITY PORTABILITY
 * ============================================================================
 *
 * Explicitly expresses portability across scales.
 *
 * Example:
 *
 *     scalability portable_across scale_expression;
 *
 * The expression may identify an abstract scale domain.
 *
 * It does not enumerate physical machines.
 * ============================================================================
 */

scalabilityPortability
    : K_SCALABILITY
      K_PORTABLE
      K_ACROSS
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. SCALABILITY DIMENSION
 * ============================================================================
 *
 * A dimension is a symbolic semantic dimension along which scaling occurs.
 *
 * Examples:
 *
 *     dimension = input_size;
 *     dimension = problem_size;
 *     dimension = workload_size;
 *     dimension = parallelism;
 *
 * There is no finite vocabulary of dimensions.
 * ============================================================================
 */

scalabilityDimension
    : K_DIMENSION
      ASSIGN
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. MULTIPLE SCALABILITY DIMENSIONS
 * ============================================================================
 *
 * Arbitrary dimensionality is supported.
 *
 * No fixed number of dimensions is encoded.
 * ============================================================================
 */

scalabilityDimensions
    : K_DIMENSIONS
      ASSIGN
      LPAREN
      resourceExpressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. SCALABILITY OVER A DOMAIN
 * ============================================================================
 *
 * Describes scaling over an abstract expression.
 *
 * Example:
 *
 *     scalability over input_size;
 *
 * The domain may be:
 *
 *     input size;
 *     problem size;
 *     workload size;
 *     data size;
 *     logical resource demand;
 *     another semantic quantity.
 *
 * ============================================================================
 */

scalabilityOver
    : K_SCALABILITY
      K_OVER
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. CONDITIONAL SCALABILITY
 * ============================================================================
 *
 * Allows scalability intent to depend on a semantic condition.
 *
 * Example:
 *
 *     scalability when condition then expression;
 *
 * The condition remains a canonical expression.
 *
 * Semantic validation determines whether the condition is appropriate.
 * ============================================================================
 */

conditionalScalability
    : K_SCALABILITY
      K_WHEN
      resourceExpression
      K_THEN
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. SCALABILITY BY CONTEXT
 * ============================================================================
 *
 * Expresses that realization may depend on execution context.
 *
 * Example:
 *
 *     scalability according_to available_capacity;
 *
 * The context is not queried by the parser.
 * ============================================================================
 */

contextualScalability
    : K_SCALABILITY
      K_ACCORDING
      K_TO
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. SCALABILITY CONTRACT
 * ============================================================================
 *
 * A scalability contract groups one or more scalability statements.
 *
 * Example:
 *
 *     scalability {
 *         dimension = input_size;
 *         scalability grows_with input_size;
 *         scalability bounded_by available_capacity;
 *     }
 *
 * The body has arbitrary cardinality.
 * ============================================================================
 */

scalabilityContract
    : K_SCALABILITY
      LBRACE
      scalabilityContractItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 23. SCALABILITY CONTRACT ITEM
 * ============================================================================
 */

scalabilityContractItem
    : scalabilityDeclaration
    | namedScalabilityDeclaration
    | scalabilityRelationship
    | scalabilityGrowth
    | scalabilityGrowthRate
    | scalabilityBound
    | scalabilityLimit
    | scalabilityDependency
    | scalabilityInvariance
    | scalabilityAdaptation
    | scalabilityPortability
    | scalabilityDimension
    | scalabilityDimensions
    | scalabilityOver
    | conditionalScalability
    | contextualScalability
    ;


/*
 * ============================================================================
 * 24. SCALABILITY REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * Resource requirements remain owned by resources.g4.
 *
 * This wrapper exists only so downstream grammar composition can distinguish
 * the syntactic context before semantic lowering.
 * ============================================================================
 */

scalabilityRequirementExpression
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 25. SCALABILITY CONSTRAINT EXPRESSION
 * ============================================================================
 */

scalabilityConstraintExpression
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 26. SCALABILITY PREFERENCE EXPRESSION
 * ============================================================================
 */

scalabilityPreferenceExpression
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 27. SCALABILITY HINT EXPRESSION
 * ============================================================================
 */

scalabilityHintExpression
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 28. SCALABILITY CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Capability lookup/discovery remains outside the grammar.
 * ============================================================================
 */

scalabilityCapabilityExpression
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 29. SCALABILITY CAPACITY EXPRESSION
 * ============================================================================
 *
 * Capacity is context-provided.
 *
 * The parser merely preserves the expression referring to that capacity.
 * ============================================================================
 */

scalabilityCapacityExpression
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 30. SCALABILITY RESOURCE EXPRESSION
 * ============================================================================
 *
 * A generic resource expression wrapper for domain grammar integration.
 * ============================================================================
 */

scalabilityResourceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 31. SCALABILITY RELATIONSHIP LIST
 * ============================================================================
 *
 * Arbitrary relationship cardinality.
 * ============================================================================
 */

scalabilityRelationshipList
    : scalabilityRelationship+
    ;


/*
 * ============================================================================
 * 32. OPTIONAL SCALABILITY RELATIONSHIP LIST
 * ============================================================================
 */

optionalScalabilityRelationshipList
    : scalabilityRelationship*
    ;


/*
 * ============================================================================
 * 33. SCALABILITY EXPRESSION LIST
 * ============================================================================
 *
 * Delegates cardinality and expression semantics to the canonical resource
 * expression list.
 * ============================================================================
 */

scalabilityExpressionList
    : resourceExpressionList
    ;


/*
 * ============================================================================
 * 34. SCALABILITY PATH
 * ============================================================================
 *
 * A scalability path remains an expression-level semantic path.
 *
 * No finite namespace depth is encoded here.
 * ============================================================================
 */

scalabilityPath
    : resourceExpression
    ;


/*
 * ============================================================================
 * 35. FUTURE / EXTENSIBLE SCALABILITY PROPERTY
 * ============================================================================
 *
 * Future scalability properties must be expressible without modifying this
 * grammar for every new resource domain.
 *
 * The property name is symbolic.
 *
 * Its value is a canonical resource expression.
 *
 * Example:
 *
 *     scalability_property = future_scaling_model;
 *
 * Semantic validation determines whether the property is recognized by the
 * active language version/dialect.
 * ============================================================================
 */

scalabilityProperty
    : scalabilityName
      ASSIGN
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 36. SCALABILITY PROPERTY LIST
 * ============================================================================
 */

scalabilityPropertyList
    : scalabilityProperty*
    ;


/*
 * ============================================================================
 * 37. SCALABILITY PROPERTY BLOCK
 * ============================================================================
 */

scalabilityPropertyBlock
    : LBRACE
      scalabilityPropertyList
      RBRACE
    ;


/*
 * ============================================================================
 * 38. CANONICAL SCALABILITY VALUE
 * ============================================================================
 *
 * This is the final composition boundary exposed to other resource grammars.
 *
 * Domain grammars should consume this rule when they need a value describing
 * scaling behavior.
 * ============================================================================
 */

scalabilityValue
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 39. CANONICAL SCALABILITY CONDITION
 * ============================================================================
 */

scalabilityCondition
    : resourceExpression
    ;


/*
 * ============================================================================
 * 40. CANONICAL SCALABILITY PREDICATE
 * ============================================================================
 *
 * The parser does not enforce boolean typing.
 *
 * Semantic/type analysis determines whether the resulting expression is a
 * valid predicate.
 * ============================================================================
 */

scalabilityPredicate
    : resourceExpression
    ;


/*
 * ============================================================================
 * 41. CANONICAL SCALABILITY RELATION
 * ============================================================================
 *
 * Generic relationship wrapper for semantic normalization.
 * ============================================================================
 */

scalabilityRelation
    : scalabilityExpression
    ;


/*
 * ============================================================================
 * 42. IDENTIFIER ADAPTER
 * ============================================================================
 *
 * The canonical identifier rule is imported through ResourceExpressions.
 *
 * This adapter exists solely to provide a stable named integration point.
 * ============================================================================
 */

scalabilityIdentifier
    : identifier
    ;


/*
 * ============================================================================
 * 43. SEMANTICALLY UNBOUNDED SCALE
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This rule does NOT mean mathematical infinity.
 *
 * It means that the grammar does not impose a finite machine-size ceiling.
 *
 * Whether a program can actually execute at a particular scale depends on
 * available resources and semantic/resource feasibility.
 *
 * Therefore this rule is represented by a symbolic expression rather than a
 * special numeric infinity token.
 * ============================================================================
 */

unboundedScalability
    : K_SCALABILITY
      K_ACCORDING
      K_TO
      scalabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 44. RESOURCE-AVAILABLE SCALING
 * ============================================================================
 *
 * Explicitly expresses adaptation to available capacity.
 *
 * Example:
 *
 *     scalability according_to available_capacity;
 *
 * The parser does not inspect capacity.
 * ============================================================================
 */

availableResourceScalability
    : K_SCALABILITY
      K_ACCORDING
      K_TO
      K_AVAILABLE
      K_CAPACITY
      SEMICOLON
    ;


/*
 * ============================================================================
 * 45. SEMANTIC-PRESERVATION SCALABILITY
 * ============================================================================
 *
 * Scaling may change realization while preserving program semantics.
 *
 * Example:
 *
 *     scalability preserves_semantics;
 *
 * The semantic checker is responsible for proving/validating preservation.
 *
 * This grammar does not claim that arbitrary scaling is automatically valid.
 * ============================================================================
 */

scalabilitySemanticPreservation
    : K_SCALABILITY
      K_PRESERVES
      K_SEMANTICS
      SEMICOLON
    ;


/*
 * ============================================================================
 * 46. COMPLETE SCALABILITY SPECIFICATION
 * ============================================================================
 *
 * This rule is the broad reusable integration boundary.
 *
 * It permits arbitrary combinations of the independently defined scalability
 * constructs.
 * ============================================================================
 */

scalabilitySpecification
    : scalabilityItem*
    | scalabilityContract
    ;


/*
 * ============================================================================
 * 47. DOMAIN-INDEPENDENT SCALABILITY SPECIFICATION
 * ============================================================================
 *
 * Domain-specific grammars may use this entry point when attaching scaling
 * intent to:
 *
 *     classical computation;
 *     quantum computation;
 *     HDL;
 *     hardware;
 *     distributed execution;
 *     AI/ML;
 *     accelerators;
 *     future domains.
 * ============================================================================
 */

domainScalabilitySpecification
    : scalabilitySpecification
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * The following invariants are part of this file's production contract.
 *
 * 1. NO MACHINE LIMITS
 *
 *    This grammar contains no fixed machine-size constants.
 *
 * 2. NO DEVICE SELECTION
 *
 *    This grammar cannot select a concrete device.
 *
 * 3. NO PHYSICAL TOPOLOGY
 *
 *    This grammar cannot encode a fixed hardware topology.
 *
 * 4. NO QUANTUM LIMIT
 *
 *    No qubit count is hard-coded.
 *
 * 5. NO CPU LIMIT
 *
 *    No processor/core/thread count is hard-coded.
 *
 * 6. NO MEMORY LIMIT
 *
 *    No memory capacity is hard-coded.
 *
 * 7. NO NODE LIMIT
 *
 *    No distributed-node count is hard-coded.
 *
 * 8. NO RESOURCE ENUMERATION LIMIT
 *
 *    Lists and repeated constructs use canonical repetition.
 *
 * 9. NO SECOND EXPRESSION LANGUAGE
 *
 *    All values delegate to resourceExpression.
 *
 * 10. NO SECOND RESOURCE MODEL
 *
 *     Resource semantics remain owned by resources.g4.
 *
 * 11. NO IR
 *
 *     The grammar produces parse structure only.
 *
 * 12. NO RUNTIME BEHAVIOR
 *
 *     The grammar performs no allocation, discovery, scheduling or execution.
 *
 * 13. NO UNSAFE
 *
 *     There are no embedded Rust actions.
 *
 * 14. NO TARGET-SPECIFIC CODE
 *
 *     The grammar is target-independent.
 *
 * 15. FUTURE EXTENSIBILITY
 *
 *     Symbolic expressions permit new scaling dimensions without requiring
 *     machine-specific grammar rewrites.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST subsequently determine:
 *
 *     whether expressions are well typed;
 *     whether quantities are dimensionally valid;
 *     whether relationships are meaningful;
 *     whether bounds are satisfiable;
 *     whether requirements are satisfiable;
 *     whether preferences are achievable;
 *     whether adaptation preserves semantics;
 *     whether portability claims are valid;
 *     whether capability references exist;
 *     whether target constraints are compatible.
 *
 * A parser success MUST NOT be interpreted as resource feasibility.
 *
 * ============================================================================
 * DOWNSTREAM INTEGRATION
 * ============================================================================
 *
 * Frontend:
 *
 *     consumes parse tree and constructs the language AST.
 *
 * Semantic analysis:
 *
 *     resolves names;
 *     checks types;
 *     checks dimensions/units;
 *     classifies scalability intent;
 *     validates relationships;
 *     preserves source provenance.
 *
 * Resource subsystem:
 *
 *     converts scalability intent into the canonical resource semantic model.
 *
 * Classical compilation:
 *
 *     may use scalability intent for parallelization/vectorization/placement.
 *
 * Quantum compilation:
 *
 *     may use scalability intent when determining logical-resource feasibility.
 *
 *     It MUST then lower quantum semantics through quantum::ir.
 *
 * QEC:
 *
 *     may consume resource requirements associated with error correction.
 *
 *     QEC algorithms remain outside this grammar.
 *
 * ZQN:
 *
 *     may provide fault/noise/resource information to downstream feasibility
 *     and resilience decisions.
 *
 *     ZQN remains the owner of quantum noise/fault semantics.
 *
 * Optimization:
 *
 *     may use scalability preferences/hints.
 *
 * Scheduling:
 *
 *     may use capacity/latency/parallelism intent.
 *
 * Hardware HAL:
 *
 *     provides actual capabilities and available capacity.
 *
 * Runtime:
 *
 *     evaluates dynamic availability and realizes the semantic intent.
 *
 * Resilience:
 *
 *     may adapt execution when available resources change, provided semantic
 *     correctness and declared constraints remain satisfied.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic for a fixed:
 *
 *     source;
 *     grammar version;
 *     lexer version;
 *     dialect configuration.
 *
 * This file contains no semantic predicates or embedded target-language
 * actions.
 *
 * ============================================================================
 * ERROR HANDLING CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser diagnostics.
 *
 * Semantic errors belong to semantic/resource analysis.
 *
 * Examples of semantic errors include:
 *
 *     non-numeric scaling expression where a quantity is required;
 *     invalid dimensional relationship;
 *     unsatisfiable mandatory bound;
 *     invalid capability reference;
 *     incompatible scalability policy.
 *
 * This grammar MUST NOT encode those semantic errors as lexer/parser hacks.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Changes to this grammar MUST preserve:
 *
 *     source compatibility where promised;
 *     parse-tree compatibility where promised;
 *     AST normalization contracts;
 *     semantic meaning;
 *     dialect/version rules.
 *
 * New scalability properties should preferably be additive.
 *
 * Removing syntax requires an explicit language-version/deprecation policy.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_*
 *     fixed machine capacities;
 *     fixed hardware counts;
 *     fixed qubit counts;
 *     fixed processor counts;
 *     fixed accelerator counts;
 *     fixed cluster sizes;
 *     fixed topology;
 *     fixed physical addresses;
 *     fixed device identifiers.
 *
 * Numeric expressions are allowed because numbers can be legitimate program
 * values.
 *
 * A numeric literal in an expression MUST NOT be interpreted by this grammar
 * as a universal machine maximum.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The repository test suite MUST cover at least:
 *
 * POSITIVE:
 *
 *     scalability = input_size;
 *     scalability = workload_size * parallelism;
 *     scalability grows_with input_size;
 *     scalability bounded_by available_capacity;
 *     scalability depends_on problem_size;
 *     scalability independent_of machine_size;
 *     scalability adapts according_to available_capacity;
 *     scalability portable_across scale;
 *     scalability { ... };
 *
 * QUANTUM:
 *
 *     scaling of logical-qubit requirements;
 *     scaling of circuit/workload expressions;
 *     scaling with shot/workload expressions;
 *
 * CLASSICAL:
 *
 *     scaling of vectors/matrices/tensors;
 *     scaling of parallel workloads;
 *
 * HDL/HARDWARE:
 *
 *     parameterized hardware scale;
 *     symbolic resource capacity;
 *     hardware generation scale;
 *
 * DISTRIBUTED:
 *
 *     symbolic node/workload relationships;
 *     dynamic capacity;
 *
 * AI:
 *
 *     dataset/model/tensor scaling;
 *
 * NEGATIVE:
 *
 *     malformed scalability declarations;
 *     missing expressions;
 *     malformed contracts;
 *     invalid separators;
 *     incomplete relationships;
 *
 * BOUNDARY:
 *
 *     zero-length lists where permitted;
 *     arbitrarily many scalability items;
 *     arbitrarily many expressions;
 *     deeply composed symbolic expressions;
 *     large numeric values as ordinary expressions;
 *     symbolic dimensions;
 *
 * SCALABILITY:
 *
 *     source programs with no grammar-defined machine ceiling;
 *     programs expressing very large symbolic resource requirements;
 *     programs whose required scale is determined by runtime context.
 *
 * DETERMINISM:
 *
 *     identical source produces identical parse structure.
 *
 * ROUND-TRIP:
 *
 *     source -> lexer -> parser -> AST -> printer -> parser
 *
 *     preserves intended scalability semantics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It compiles with the repository's ANTLR configuration.
 *
 * [ ] Its token names match the canonical lexer.
 *
 * [ ] ResourceExpressions resolves successfully.
 *
 * [ ] No duplicate canonical expression rules exist.
 *
 * [ ] No duplicate resource semantic model exists.
 *
 * [ ] resources.g4 can import/use this grammar without architectural cycles.
 *
 * [ ] Quantum resource grammar can consume its scalability boundary without
 *     redefining scalability semantics.
 *
 * [ ] Hardware resource grammar can consume its scalability boundary without
 *     redefining scalability semantics.
 *
 * [ ] Hybrid resource grammar can consume its scalability boundary without
 *     redefining scalability semantics.
 *
 * [ ] The resulting AST has a stable semantic representation for scalability
 *     intent.
 *
 * [ ] Semantic analysis distinguishes requirement, constraint, preference,
 *     hint, capacity and availability.
 *
 * [ ] No physical machine assumptions occur in this grammar.
 *
 * [ ] No fixed resource maximum occurs in this grammar.
 *
 * [ ] No fixed quantum limit occurs in this grammar.
 *
 * [ ] No fixed processor/device limit occurs in this grammar.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass where supported.
 *
 * [ ] Documentation describes this file as a syntax boundary, not an IR.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */