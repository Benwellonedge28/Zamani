/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/optimization.g4
 *
 * Grammar:
 *     CompileOptimization
 *
 * Status:
 *     Production optimization-intent parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL OPTIMIZATION INTENT.
 *
 * It describes:
 *
 *     - whether optimization is requested;
 *     - optimization profiles;
 *     - optimization objectives;
 *     - objective priorities;
 *     - optimization preferences;
 *     - optimization constraints;
 *     - optimization requirements;
 *     - optimization hints;
 *     - optimization pass selection;
 *     - optimization pass exclusion;
 *     - optimization pipeline composition;
 *     - optimization budgets;
 *     - optimization termination policy;
 *     - optimization verification policy;
 *     - optimization reproducibility policy;
 *     - optimization scope;
 *     - target-aware optimization intent;
 *     - resource-aware optimization intent;
 *     - approximation/stochastic policy;
 *     - semantic-preservation requirements.
 *
 * It does NOT implement optimization.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     lexer / parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> optimization intent
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization planner
 *          |
 *          +--> analysis
 *          +--> pass selection
 *          +--> rewrite
 *          +--> verification
 *          +--> provenance
 *          |
 *          v
 *     optimized canonical IR
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> hardware lowering
 *          +--> runtime
 *
 * For quantum programs:
 *
 *     source
 *       |
 *       v
 *     quantum frontend
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / hardware
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - source-level optimization intent;
 *     - optimization declarations;
 *     - optimization profiles;
 *     - optimization objectives;
 *     - objective priorities;
 *     - optimization policies;
 *     - optimization constraints;
 *     - optimization requirements;
 *     - optimization preferences;
 *     - optimization hints;
 *     - pass-selection intent;
 *     - pass-exclusion intent;
 *     - pipeline composition intent;
 *     - optimization budgets;
 *     - termination intent;
 *     - verification intent;
 *     - reproducibility intent;
 *     - approximation intent;
 *     - stochastic-optimization intent;
 *     - optimization scope.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - optimization algorithms;
 *     - optimizer implementations;
 *     - canonical IR;
 *     - quantum IR;
 *     - quantum gates;
 *     - QubitId;
 *     - hardware discovery;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - backend APIs;
 *     - compiler implementation;
 *     - resource discovery.
 *
 * ============================================================================
 * CRITICAL SEPARATION
 * ============================================================================
 *
 * The following distinctions are mandatory:
 *
 *     optimization intent != optimization implementation
 *     optimization pass name != pass implementation
 *     objective != cost-model implementation
 *     target != hardware device
 *     capability != device
 *     preference != requirement
 *     hint != guarantee
 *     budget != machine limit
 *     resource requirement != fixed resource count
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Optimization syntax MUST NOT make a temporary implementation choice part
 * of permanent program semantics unless the programmer explicitly requests
 * that semantic property.
 *
 * For example, the grammar must allow:
 *
 *     minimize depth
 *
 * without requiring:
 *
 *     use device X
 *     use N qubits
 *     use topology Y
 *     use exactly M cores
 *
 * The optimizer and target-resolution layers determine how the objective is
 * achieved on the available execution environment.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no fixed machine limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_PASSES
 *     MAX_OPERATIONS
 *     MAX_OBJECTIVES
 *     MAX_PIPELINE_STAGES
 *
 * Repeated grammar constructs are intentionally represented by repetition
 * rather than fixed-size alternatives.
 *
 * Actual optimization limits belong to the compiler/runtime policy layer.
 *
 * The repository already provides explicit optimization limits and resource
 * policies. The grammar must express them, not replace them with constants.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Optimization determinism is a semantic/compiler concern and may be
 * controlled through:
 *
 *     - reproducibility;
 *     - deterministic policy;
 *     - explicit seed;
 *     - stable pass identifiers;
 *     - stable pipeline ordering.
 *
 * No ambient randomness is introduced by this grammar.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no predicates requiring unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no device discovery;
 *     - no runtime execution.
 *
 * Compiler integration MUST use:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST NOT use `unsafe`.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING OPTIMIZATION SUBSYSTEM
 * ============================================================================
 *
 * The repository's optimization subsystem already contains independent
 * contracts for:
 *
 *     config
 *     profile
 *     limits
 *     context
 *     analysis
 *     planner
 *     pipeline
 *     pass
 *     registry
 *     rules
 *     pattern
 *     matcher
 *     rewrite
 *     local optimization
 *     algebra
 *     synthesis
 *     fault-tolerant optimization
 *     stochastic optimization
 *     verification
 *     statistics
 *     provenance
 *     result
 *     serialization
 *
 * This grammar provides SOURCE INTENT that can be lowered into those
 * contracts.
 *
 * It must never expose their Rust implementation types directly.
 *
 * ============================================================================
 */

parser grammar CompileOptimization;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. TOP-LEVEL OPTIMIZATION DECLARATION
 * ============================================================================
 *
 * Canonical integration entry point.
 *
 * The enclosing compilation grammar should reference:
 *
 *     optimizationDeclaration
 *
 * rather than duplicating these rules.
 */
optimizationDeclaration
    : optimizationDirective
    | optimizationProfile
    | optimizationPolicy
    | optimizationObjectiveDeclaration
    | optimizationPipelineDeclaration
    | optimizationPassDeclaration
    | optimizationBudgetDeclaration
    | optimizationVerificationDeclaration
    | optimizationReproducibilityDeclaration
    ;


/*
 * ============================================================================
 * 2. GENERIC OPTIMIZATION DIRECTIVE
 * ============================================================================
 *
 * Generic extensibility point.
 *
 * The identifier is intentionally open-ended.
 *
 * This prevents future optimization technologies from requiring grammar
 * changes merely because a new optimization concept is introduced.
 */
optimizationDirective
    : optimizationKeyword optimizationDirectiveBody?
    ;


optimizationDirectiveBody
    : LPAREN optimizationArgumentList? RPAREN
    | LBRACE optimizationEntry* RBRACE
    | ASSIGN expression
    ;


optimizationKeyword
    : identifier
    ;


optimizationEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    | identifier expression SEMICOLON
    ;


/*
 * ============================================================================
 * 3. OPTIMIZATION PROFILE
 * ============================================================================
 *
 * A profile is a reusable semantic optimization policy.
 *
 * It is NOT an optimizer implementation.
 *
 * Example conceptual form:
 *
 *     optimization profile portable {
 *         ...
 *     }
 *
 * Profile names remain open-ended.
 */
optimizationProfile
    : optimizationKeyword identifier
      LBRACE optimizationProfileEntry* RBRACE
    ;


optimizationProfileEntry
    : optimizationObjectiveDeclaration
    | optimizationPolicy
    | optimizationPassDeclaration
    | optimizationBudgetDeclaration
    | optimizationVerificationDeclaration
    | optimizationReproducibilityDeclaration
    | optimizationProperty
    ;


/*
 * ============================================================================
 * 4. OPTIMIZATION POLICY
 * ============================================================================
 *
 * A policy controls optimizer behavior without naming an implementation.
 */
optimizationPolicy
    : optimizationKeyword optimizationPolicyBody
    ;


optimizationPolicyBody
    : expression
    | LBRACE optimizationPolicyEntry* RBRACE
    ;


optimizationPolicyEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 5. OPTIMIZATION OBJECTIVES
 * ============================================================================
 *
 * Objectives describe WHAT should improve.
 *
 * They do not define HOW improvement is achieved.
 *
 * Examples:
 *
 *     minimize gate count
 *     minimize depth
 *     minimize two-qubit operations
 *     minimize latency
 *     minimize energy
 *     minimize logical resource cost
 *     maximize fidelity
 *
 * The grammar does not enumerate a finite objective list.
 *
 * Future objective classes therefore do not require a grammar rewrite.
 */
optimizationObjectiveDeclaration
    : optimizationObjective
    | optimizationObjectiveGroup
    ;


optimizationObjective
    : optimizationObjectiveDirection
      optimizationObjectiveExpression
      optimizationObjectiveModifier*
      SEMICOLON?
    ;


optimizationObjectiveDirection
    : identifier
    ;


optimizationObjectiveExpression
    : expression
    ;


optimizationObjectiveModifier
    : optimizationWeight
    | optimizationPriority
    | optimizationTolerance
    | optimizationConstraintModifier
    | optimizationPropertyModifier
    ;


optimizationWeight
    : identifier expression
    ;


optimizationPriority
    : identifier expression
    ;


optimizationTolerance
    : identifier expression
    ;


optimizationConstraintModifier
    : identifier expression
    ;


optimizationPropertyModifier
    : identifier expression
    ;


/*
 * ============================================================================
 * 6. OBJECTIVE GROUPS
 * ============================================================================
 *
 * Allows arbitrary numbers of objectives.
 *
 * No fixed objective count is encoded.
 */
optimizationObjectiveGroup
    : optimizationKeyword
      LBRACE optimizationObjective* RBRACE
    ;


/*
 * ============================================================================
 * 7. MULTI-OBJECTIVE OPTIMIZATION
 * ============================================================================
 *
 * Supports semantic policies such as:
 *
 *     lexicographic optimization
 *     weighted optimization
 *     Pareto-style optimization
 *     priority-based optimization
 *
 * The exact mathematical interpretation belongs to semantic analysis and the
 * optimization planner.
 */
optimizationMultiObjective
    : optimizationKeyword
      LBRACE optimizationObjective* RBRACE
    ;


optimizationObjectiveOrdering
    : optimizationKeyword expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. OPTIMIZATION PIPELINE
 * ============================================================================
 *
 * Describes the requested optimization pipeline.
 *
 * A pipeline is an ordered semantic request.
 *
 * The actual pass implementations are resolved through the optimizer's
 * registry/planner.
 */
optimizationPipelineDeclaration
    : optimizationPipelineKeyword identifier?
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationPipelineKeyword
    : identifier
    ;


optimizationPipelineItem
    : optimizationPassReference
    | optimizationPassGroup
    | optimizationPipelineReference
    | optimizationConditionalPass
    | optimizationRepeatedPass
    | optimizationPipelineProperty
    ;


optimizationPipelineReference
    : identifier identifier?
    ;


optimizationPassGroup
    : LBRACKET optimizationPassReferenceList? RBRACKET
    ;


optimizationPassReferenceList
    : optimizationPassReference
      (COMMA optimizationPassReference)*
    ;


optimizationConditionalPass
    : optimizationKeyword
      expression
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationRepeatedPass
    : optimizationKeyword
      expression
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationPipelineProperty
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PASS DECLARATION
 * ============================================================================
 *
 * Declares a pass-selection request.
 *
 * A pass identifier is symbolic.
 *
 * It MUST NOT be interpreted by the grammar as an implementation type.
 *
 * The optimization registry/planner resolves the identifier.
 */
optimizationPassDeclaration
    : optimizationPassReference
      optimizationPassBody?
    ;


optimizationPassReference
    : optimizationKeyword
      identifier?
    ;


optimizationPassBody
    : LPAREN optimizationArgumentList? RPAREN
    | LBRACE optimizationPassEntry* RBRACE
    ;


optimizationPassEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 10. PASS ENABLEMENT
 * ============================================================================
 *
 * Explicitly requests a pass.
 */
optimizationPassEnable
    : optimizationKeyword optimizationPassReference
    ;


/*
 * ============================================================================
 * 11. PASS DISABLEMENT
 * ============================================================================
 *
 * Prevents a pass from being selected by an optimizer policy.
 *
 * This is a policy request, not a mutation of the optimizer registry.
 */
optimizationPassDisable
    : optimizationKeyword optimizationPassReference SEMICOLON?
    ;


optimizationPassExclusion
    : optimizationKeyword
      LBRACE optimizationPassReferenceList? RBRACE
    ;


/*
 * ============================================================================
 * 12. PASS CONFIGURATION
 * ============================================================================
 *
 * Pass configuration is intentionally expression-based.
 *
 * This allows:
 *
 *     compile-time values;
 *     resource expressions;
 *     symbolic parameters;
 *     target-derived values;
 *     capability-derived values.
 *
 * No machine-size constants are required.
 */
optimizationPassConfiguration
    : optimizationPassReference
      LPAREN optimizationArgumentList? RPAREN
    ;


optimizationArgumentList
    : optimizationArgument
      (COMMA optimizationArgument)*
    ;


optimizationArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 13. OPTIMIZATION BUDGETS
 * ============================================================================
 *
 * A budget is a compilation-resource policy.
 *
 * It is NOT a source-language machine limit.
 *
 * Examples:
 *
 *     time budget
 *     memory budget
 *     evaluation budget
 *     rewrite budget
 *     iteration budget
 *     pass budget
 *
 * The actual enforcement belongs to optimization::limits / pipeline/context.
 */
optimizationBudgetDeclaration
    : optimizationBudgetKeyword
      optimizationBudgetBody
    ;


optimizationBudgetKeyword
    : identifier
    ;


optimizationBudgetBody
    : expression
    | LBRACE optimizationBudgetEntry* RBRACE
    ;


optimizationBudgetEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 14. TERMINATION POLICY
 * ============================================================================
 *
 * Allows source-level selection of termination semantics.
 *
 * Examples:
 *
 *     fixed point
 *     bounded
 *     until stable
 *     until no progress
 *     resource bounded
 *
 * Exact policy semantics belong to the optimization pipeline.
 */
optimizationTermination
    : optimizationKeyword
      optimizationTerminationBody
    ;


optimizationTerminationBody
    : expression
    | LBRACE optimizationTerminationEntry* RBRACE
    ;


optimizationTerminationEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 15. FIXED-POINT INTENT
 * ============================================================================
 *
 * Supports repeated optimization until semantic stabilization.
 *
 * The grammar imposes no fixed iteration count.
 */
optimizationFixedPoint
    : optimizationKeyword
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationFixedPointPolicy
    : optimizationKeyword expression
    ;


/*
 * ============================================================================
 * 16. OPTIMIZATION VERIFICATION
 * ============================================================================
 *
 * Optimization must preserve program semantics unless the programmer has
 * explicitly requested an approximation or semantics-changing transformation.
 *
 * Verification policy is therefore a first-class source-level concept.
 */
optimizationVerificationDeclaration
    : optimizationVerificationKeyword
      optimizationVerificationBody
    ;


optimizationVerificationKeyword
    : identifier
    ;


optimizationVerificationBody
    : expression
    | LBRACE optimizationVerificationEntry* RBRACE
    ;


optimizationVerificationEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 17. SEMANTIC PRESERVATION
 * ============================================================================
 *
 * Explicit semantic-preservation intent.
 *
 * This is especially important for:
 *
 *     quantum circuits;
 *     reversible computation;
 *     HDL;
 *     numerical programs;
 *     floating-point transformations;
 *     probabilistic programs;
 *     approximate optimization.
 */
optimizationSemanticPreservation
    : optimizationKeyword
      optimizationSemanticPreservationBody
    ;


optimizationSemanticPreservationBody
    : expression
    | LBRACE optimizationSemanticPreservationEntry* RBRACE
    ;


optimizationSemanticPreservationEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 18. APPROXIMATION POLICY
 * ============================================================================
 *
 * Approximate optimization must never be silently introduced.
 *
 * The semantic layer must know whether an optimization is:
 *
 *     exact
 *     bounded-error
 *     approximate
 *     heuristic
 *     stochastic
 *
 * The grammar only expresses the request.
 */
optimizationApproximation
    : optimizationKeyword
      optimizationApproximationBody
    ;


optimizationApproximationBody
    : expression
    | LBRACE optimizationApproximationEntry* RBRACE
    ;


optimizationApproximationEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 19. STOCHASTIC OPTIMIZATION
 * ============================================================================
 *
 * The repository already has a stochastic optimization subsystem.
 *
 * This grammar therefore describes stochastic intent without implementing
 * randomness.
 *
 * Explicit seeds are expressions and are not mandatory unless the semantic
 * policy requires reproducibility.
 */
optimizationStochastic
    : optimizationKeyword
      optimizationStochasticBody
    ;


optimizationStochasticBody
    : expression
    | LBRACE optimizationStochasticEntry* RBRACE
    ;


optimizationStochasticEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 20. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility may include:
 *
 *     deterministic execution;
 *     explicit seed;
 *     stable pipeline;
 *     stable pass order;
 *     provenance recording.
 *
 * No global mutable state is implied.
 */
optimizationReproducibilityDeclaration
    : optimizationReproducibilityKeyword
      optimizationReproducibilityBody
    ;


optimizationReproducibilityKeyword
    : identifier
    ;


optimizationReproducibilityBody
    : expression
    | LBRACE optimizationReproducibilityEntry* RBRACE
    ;


optimizationReproducibilityEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 21. RANDOM SEED
 * ============================================================================
 *
 * A seed is symbolic/expression-valued.
 *
 * No finite integer range is encoded here.
 */
optimizationSeed
    : optimizationKeyword expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 22. OPTIMIZATION SCOPE
 * ============================================================================
 *
 * Optimization can apply to semantic regions without defining a fixed
 * structural hierarchy in this grammar.
 *
 * Examples:
 *
 *     module
 *     function
 *     region
 *     quantum circuit
 *     classical region
 *     hardware block
 *     complete program
 *
 * The actual scope model belongs to the AST/semantic layer.
 */
optimizationScope
    : optimizationKeyword
      optimizationScopeExpression
    ;


optimizationScopeExpression
    : identifier
    | qualifiedIdentifier
    | expression
    ;


/*
 * ============================================================================
 * 23. TARGET-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Target information may influence optimization.
 *
 * IMPORTANT:
 *
 *     optimization target != hardware device
 *
 * Target resolution remains owned by `grammar/compile/target.g4` and the
 * compiler's target-resolution layer.
 */
optimizationTargetPolicy
    : optimizationKeyword
      targetReference
    ;


targetReference
    : identifier
    | qualifiedIdentifier
    | STRING
    | expression
    ;


/*
 * ============================================================================
 * 24. RESOURCE-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Optimization may express resource-aware policies without encoding machine
 * capacities directly.
 *
 * Examples:
 *
 *     optimize under memory budget
 *     optimize for available parallelism
 *     minimize energy
 *     minimize latency
 *
 * Resource semantics remain owned by grammar/resources and the compiler
 * resource model.
 */
optimizationResourcePolicy
    : optimizationKeyword
      optimizationResourceExpression
    ;


optimizationResourceExpression
    : identifier
    | qualifiedIdentifier
    | expression
    ;


/*
 * ============================================================================
 * 25. HARDWARE-INDEPENDENT COST INTENT
 * ============================================================================
 *
 * Cost names remain symbolic.
 *
 * This prevents the grammar from becoming coupled to a particular hardware
 * generation.
 *
 * Examples:
 *
 *     gate_count
 *     depth
 *     latency
 *     energy
 *     communication
 *     logical_cost
 *     physical_cost
 *
 * The semantic cost model decides what each metric means.
 */
optimizationCostMetric
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


optimizationCostMetricList
    : optimizationCostMetric
      (COMMA optimizationCostMetric)*
    ;


/*
 * ============================================================================
 * 26. COST MODEL REFERENCE
 * ============================================================================
 *
 * A cost model is symbolic.
 *
 * Its implementation belongs to optimization::cost / planner / target
 * infrastructure.
 */
optimizationCostModel
    : optimizationKeyword
      (identifier | qualifiedIdentifier | STRING)
    ;


/*
 * ============================================================================
 * 27. OPTIMIZATION PROPERTY
 * ============================================================================
 *
 * Generic extensibility mechanism.
 *
 * New semantic optimization properties can be added without introducing
 * fixed parser enumerations.
 */
optimizationProperty
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 28. OBJECTIVE COMPOSITION
 * ============================================================================
 *
 * Supports arbitrary nesting of objective policies.
 */
optimizationComposition
    : optimizationKeyword
      LBRACE optimizationCompositionEntry* RBRACE
    ;


optimizationCompositionEntry
    : optimizationObjectiveDeclaration
    | optimizationPolicy
    | optimizationProperty
    | optimizationComposition
    ;


/*
 * ============================================================================
 * 29. OPTIMIZATION FALLBACK
 * ============================================================================
 *
 * Allows semantic fallback policies when an optimization strategy is not
 * applicable.
 *
 * This does NOT choose a hardware backend.
 */
optimizationFallback
    : optimizationKeyword
      optimizationFallbackBody
    ;


optimizationFallbackBody
    : optimizationPipelineItem
    | LBRACE optimizationPipelineItem* RBRACE
    ;


/*
 * ============================================================================
 * 30. OPTIMIZATION CONDITIONAL
 * ============================================================================
 *
 * Conditional optimization depends on a semantic predicate.
 *
 * It is distinct from runtime control flow.
 */
optimizationConditional
    : optimizationKeyword
      expression
      LBRACE optimizationPipelineItem* RBRACE
    ;


/*
 * ============================================================================
 * 31. OPTIMIZATION REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * This is deliberately separate from:
 *
 *     preference
 *     hint
 *     objective
 */
optimizationRequirement
    : optimizationKeyword
      optimizationRequirementBody
    ;


optimizationRequirementBody
    : expression
    | LBRACE optimizationRequirementEntry* RBRACE
    ;


optimizationRequirementEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 32. OPTIMIZATION CONSTRAINT
 * ============================================================================
 *
 * A constraint limits legal optimizer choices.
 */
optimizationConstraint
    : optimizationKeyword
      optimizationConstraintBody
    ;


optimizationConstraintBody
    : expression
    | LBRACE optimizationConstraintEntry* RBRACE
    ;


optimizationConstraintEntry
    : identifier comparisonOperator expression SEMICOLON
    | identifier COLON expression SEMICOLON
    | identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 33. OPTIMIZATION PREFERENCE
 * ============================================================================
 *
 * Preferences are non-mandatory.
 */
optimizationPreference
    : optimizationKeyword
      optimizationPreferenceBody
    ;


optimizationPreferenceBody
    : expression
    | LBRACE optimizationPreferenceEntry* RBRACE
    ;


optimizationPreferenceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 34. OPTIMIZATION HINT
 * ============================================================================
 *
 * Hints may be ignored without changing semantic correctness.
 */
optimizationHint
    : optimizationKeyword
      optimizationHintBody
    ;


optimizationHintBody
    : expression
    | LBRACE optimizationHintEntry* RBRACE
    ;


optimizationHintEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 35. PASS DEPENDENCY INTENT
 * ============================================================================
 *
 * Allows source-level pipeline dependencies without defining implementation
 * details.
 */
optimizationPassDependency
    : optimizationKeyword
      optimizationPassReference
      optimizationPassReference
    ;


/*
 * ============================================================================
 * 36. PASS ORDERING
 * ============================================================================
 *
 * Ordering is an optimization-pipeline concern.
 *
 * It does not describe execution scheduling of the program.
 */
optimizationPassOrdering
    : optimizationKeyword
      optimizationPassReference
      optimizationPassReference
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 37. PASS INVALIDATION / ANALYSIS INTENT
 * ============================================================================
 *
 * Source code may request analysis or invalidation behavior when such policy
 * is part of the language's compilation contract.
 *
 * The actual analysis implementation belongs to the optimizer.
 */
optimizationAnalysisPolicy
    : optimizationKeyword
      optimizationAnalysisPolicyBody
    ;


optimizationAnalysisPolicyBody
    : expression
    | LBRACE optimizationAnalysisPolicyEntry* RBRACE
    ;


optimizationAnalysisPolicyEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 38. VERIFICATION LEVEL
 * ============================================================================
 *
 * Verification is symbolic and open-ended.
 *
 * Examples:
 *
 *     structural
 *     semantic
 *     exhaustive
 *     randomized
 *     certificate
 *
 * The grammar does not hard-code the available verification implementations.
 */
optimizationVerificationLevel
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


optimizationVerificationLevelList
    : optimizationVerificationLevel
      (COMMA optimizationVerificationLevel)*
    ;


/*
 * ============================================================================
 * 39. OPTIMIZATION SERIALIZATION / PROVENANCE INTENT
 * ============================================================================
 *
 * Optimization provenance is important for reproducibility and POCO-REAF.
 *
 * The grammar only requests provenance behavior.
 *
 * Serialization implementation belongs to the optimization subsystem.
 */
optimizationProvenancePolicy
    : optimizationKeyword
      optimizationProvenanceBody
    ;


optimizationProvenanceBody
    : expression
    | LBRACE optimizationProvenanceEntry* RBRACE
    ;


optimizationProvenanceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 40. COMPILATION CONTEXT REFERENCE
 * ============================================================================
 *
 * Optimization may depend on a compilation context.
 *
 * The context is supplied by semantic/compiler infrastructure.
 *
 * This grammar does not discover or construct the context.
 */
optimizationContext
    : optimizationKeyword
      (identifier | qualifiedIdentifier | expression)
    ;


/*
 * ============================================================================
 * 41. COMPARISON OPERATORS
 * ============================================================================
 *
 * These are shared lexical operators.
 *
 * Semantic type checking remains outside this grammar.
 */
comparisonOperator
    : EQ
    | NEQ
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * 42. SHARED NAME REFERENCES
 * ============================================================================
 *
 * These rules are compatibility references to the canonical name grammar.
 *
 * They must be replaced by/imported from the repository's canonical
 * `grammar/core` name rules during parser assembly.
 *
 * This file must NOT become a second name-system authority.
 */
identifier
    : IDENTIFIER
    ;


qualifiedIdentifier
    : identifier
      (DCOLON identifier)*
    ;