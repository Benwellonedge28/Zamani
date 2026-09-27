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
 *     Production source-level optimization-intent grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL OPTIMIZATION INTENT.
 *
 * It describes what optimization the programmer permits, requires, prefers,
 * requests, excludes, constrains, or wishes to observe.
 *
 * It does NOT implement optimization.
 *
 * Optimization implementation remains outside the grammar and is performed
 * by the existing compiler/optimizer pipeline after semantic analysis.
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
 *     ZamaniParser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> optimization intent
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical semantics      quantum::ir
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *                 optimization
 *                      |
 *          +-----------+------------+
 *          |           |            |
 *          v           v            v
 *       analysis    rewriting    verification
 *                      |
 *                      v
 *              routing / scheduling
 *                      |
 *                 resilience / QEC
 *                      |
 *                     ZQN
 *                      |
 *                     HAL
 *                      |
 *               target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT create:
 *
 *     QuantumOptimizationIR
 *     OptimizationIR
 *     HardwareOptimizationIR
 *     TargetOptimizationIR
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - optimization-intent syntax;
 *     - optimization specification composition;
 *     - optimization objectives;
 *     - optimization policies;
 *     - optimization requirements;
 *     - optimization constraints;
 *     - optimization preferences;
 *     - optimization hints;
 *     - optimization pass references;
 *     - optimization pass configuration;
 *     - optimization pipeline intent;
 *     - optimization budgets;
 *     - optimization termination intent;
 *     - optimization verification intent;
 *     - optimization reproducibility intent;
 *     - optimization approximation intent;
 *     - optimization stochastic intent;
 *     - optimization scope;
 *     - optimization fallback intent;
 *     - optimization provenance intent;
 *     - optimization cost-model references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - ordinary expression syntax;
 *     - type syntax;
 *     - AST implementation;
 *     - semantic optimization algorithms;
 *     - optimizer implementations;
 *     - cost-model implementations;
 *     - canonical IR;
 *     - quantum::ir;
 *     - quantum operations;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - HAL;
 *     - hardware discovery;
 *     - target discovery;
 *     - resource discovery;
 *     - runtime execution.
 *
 * ============================================================================
 * AUTHORITATIVE DEPENDENCIES
 * ============================================================================
 *
 * Names are owned by the canonical core grammar.
 *
 * Expressions are owned by the canonical expression grammar.
 *
 * This file therefore imports those authorities instead of redefining:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *
 * No local replacement such as:
 *
 *     DCOLON
 *     qualifiedIdentifier
 *
 * is permitted.
 *
 * The canonical separator is:
 *
 *     DOUBLE_COLON
 *
 * through:
 *
 *     qualifiedName
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 *     grammar/compile/compilation.g4
 *              |
 *              +--> CompileOptimization
 *                       |
 *                       +--> Core
 *                       |
 *                       +--> Expressions
 *
 * `compilation.g4` remains the compilation-domain composition owner.
 *
 * It imports `CompileOptimization` and exposes:
 *
 *     optimizationDeclaration
 *
 * through:
 *
 *     compilationOptimizationReference
 *
 * No other grammar should duplicate optimizationDeclaration.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Optimization intent MUST describe semantic goals and permitted
 * transformations rather than accidentally binding the program to today's
 * hardware.
 *
 * This grammar therefore contains NO universal limits for:
 *
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     nodes
 *     devices
 *     memory
 *     storage
 *     registers
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     pipeline stages
 *     optimization passes
 *     objectives
 *     iterations
 *     timelines
 *
 * Repetition is represented structurally using:
 *
 *     *
 *     +
 *
 * or recursive composition.
 *
 * A numeric literal appearing in an optimization expression is program
 * semantics or an explicitly supplied policy value. It is NOT a universal
 * machine capacity.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These four concepts are intentionally distinct.
 *
 * requirement
 *     MUST be satisfied.
 *
 * constraint
 *     Restricts the legal optimization solution space.
 *
 * preference
 *     SHOULD be preferred when feasible.
 *
 * hint
 *     MAY guide optimization but MUST NOT be treated as a correctness
 *     guarantee.
 *
 * The semantic layer is responsible for enforcing these distinctions.
 *
 * ============================================================================
 * OBJECTIVE / PASS / IMPLEMENTATION
 * ============================================================================
 *
 * An objective describes WHAT should improve.
 *
 * A pass reference describes WHICH symbolic transformation family is
 * requested.
 *
 * The actual implementation is resolved through the compiler's optimization
 * registry/planner.
 *
 * Therefore:
 *
 *     objective != pass
 *     pass != implementation
 *     implementation != hardware
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Optimization may consume target-derived information downstream.
 *
 * This grammar MUST NOT:
 *
 *     - discover hardware;
 *     - select physical devices;
 *     - allocate qubits;
 *     - select CPU cores;
 *     - select GPU indices;
 *     - select FPGA resources;
 *     - select memory banks;
 *     - encode topology;
 *     - encode calibration;
 *     - perform routing;
 *     - perform scheduling.
 *
 * Target selection remains owned by:
 *
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Optimization intent can apply to quantum computation without enumerating
 * quantum gates.
 *
 * Examples of semantic objectives include:
 *
 *     minimize depth
 *     minimize two-qubit operations
 *     minimize estimated error
 *     maximize estimated fidelity
 *     minimize logical cost
 *     preserve measurement semantics
 *
 * Quantum operation names remain extensible.
 *
 * The canonical pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC / ZQN
 *          |
 *          v
 *     HAL
 *
 * No second quantum IR is introduced here.
 *
 * ============================================================================
 * CLASSICAL / HDL / AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same optimization-intent model may apply to:
 *
 *     classical computation
 *     numerical computation
 *     vector computation
 *     matrix computation
 *     tensor computation
 *     AI/ML
 *     dataflow
 *     distributed computation
 *     networking
 *     HDL
 *     hardware/software co-design
 *     heterogeneous computation
 *     future computational domains
 *
 * Domain semantics are resolved downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - token sequence;
 *     - active grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - hardware availability;
 *     - device state;
 *     - runtime scheduler state.
 *
 * Optimization determinism is a compiler/semantic concern.
 *
 * Reproducibility intent may request deterministic behavior, stable ordering,
 * explicit seeds, provenance, or equivalent policies.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no embedded Rust;
 *     - contains no semantic actions;
 *     - contains no executable predicates;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no device discovery;
 *     - performs no runtime execution.
 *
 * Generated Rust parser code MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST remain safe Rust.
 *
 * ============================================================================
 */

parser grammar CompileOptimization;

import Core,
       Expressions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `optimizationDeclaration` is the ONLY public optimization entry point.
 *
 * The surrounding compilation grammar owns placement of this construct.
 *
 * The first name is intentionally contextual rather than a new lexer token.
 *
 * This is necessary because the current canonical lexer does not define a
 * dedicated OPTIMIZE token.
 *
 * Semantic analysis MUST recognize the supported contextual operation names.
 *
 * The grammar consequently remains extensible without modifying the lexer for
 * every future optimization technology.
 */

optimizationDeclaration
    : optimizationSpecification
    ;


/*
 * ============================================================================
 * 2. OPTIMIZATION SPECIFICATION
 * ============================================================================
 *
 * An optimization specification contains one or more optimization clauses.
 *
 * At least one clause is required.
 *
 * An empty optimization specification has no semantic optimization intent and
 * is therefore rejected structurally.
 */

optimizationSpecification
    : optimizationClause+
    ;


/*
 * ============================================================================
 * 3. OPTIMIZATION CLAUSE
 * ============================================================================
 *
 * A clause is one semantically classified optimization-intent construct.
 *
 * The leading name is interpreted by semantic analysis as a contextual
 * optimization keyword.
 *
 * Supported standard contextual names include:
 *
 *     optimize
 *     objective
 *     minimize
 *     maximize
 *     require
 *     constraint
 *     prefer
 *     hint
 *     pass
 *     pipeline
 *     budget
 *     terminate
 *     verify
 *     preserve
 *     approximate
 *     stochastic
 *     reproducible
 *     scope
 *     fallback
 *     provenance
 *     cost_model
 *     property
 *
 * These names remain contextual semantic vocabulary rather than a finite list
 * of lexer-level optimization tokens.
 */

optimizationClause
    : optimizationDirective
    | optimizationObjectiveClause
    | optimizationRequirementClause
    | optimizationConstraintClause
    | optimizationPreferenceClause
    | optimizationHintClause
    | optimizationPassClause
    | optimizationPipelineClause
    | optimizationBudgetClause
    | optimizationTerminationClause
    | optimizationVerificationClause
    | optimizationPreservationClause
    | optimizationApproximationClause
    | optimizationStochasticClause
    | optimizationReproducibilityClause
    | optimizationScopeClause
    | optimizationFallbackClause
    | optimizationProvenanceClause
    | optimizationCostModelClause
    | optimizationPropertyClause
    ;


/*
 * ============================================================================
 * 4. GENERIC OPTIMIZATION DIRECTIVE
 * ============================================================================
 *
 * Generic directives are the forward-compatible extension mechanism.
 *
 * They preserve arbitrary optimization technologies without requiring a
 * grammar rewrite merely because a new optimizer family appears.
 *
 * Examples:
 *
 *     optimize { ... }
 *     optimize::domain { ... }
 *     optimizer::future { ... }
 *
 * Semantic analysis decides whether a directive is known, experimental,
 * dialect-provided, or invalid.
 */

optimizationDirective
    : optimizationName optimizationDirectiveBody?
    ;


optimizationDirectiveBody
    : optimizationArgumentBlock
    | optimizationPropertyBlock
    | optimizationAssignment
    ;


optimizationName
    : qualifiedName
    ;


optimizationArgumentBlock
    : LPAREN optimizationArgumentList? RPAREN
    ;


optimizationPropertyBlock
    : LBRACE optimizationPropertyEntry* RBRACE
    ;


optimizationAssignment
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * 5. OBJECTIVES
 * ============================================================================
 *
 * Objectives express what should improve.
 *
 * The objective direction is contextual:
 *
 *     minimize
 *     maximize
 *
 * Additional objective systems may be supplied by dialects/semantic
 * extensions without modifying this grammar.
 *
 * The objective expression remains the canonical Zamani expression grammar.
 *
 * Examples:
 *
 *     minimize depth
 *     minimize gate_count
 *     minimize quantum::cost
 *     maximize fidelity
 *     maximize throughput
 */

optimizationObjectiveClause
    : optimizationObjective
    ;


optimizationObjective
    : optimizationObjectiveDirection
      optimizationObjectiveTarget
      optimizationObjectiveModifier*
      SEMICOLON?
    ;


optimizationObjectiveDirection
    : qualifiedName
    ;


optimizationObjectiveTarget
    : expression
    ;


optimizationObjectiveModifier
    : optimizationWeight
    | optimizationPriority
    | optimizationTolerance
    | optimizationObjectiveOrdering
    | optimizationObjectiveProperty
    ;


optimizationWeight
    : qualifiedName expression
    ;


optimizationPriority
    : qualifiedName expression
    ;


optimizationTolerance
    : qualifiedName expression
    ;


optimizationObjectiveOrdering
    : qualifiedName expression
    ;


optimizationObjectiveProperty
    : qualifiedName expression
    ;


/*
 * ============================================================================
 * 6. MULTI-OBJECTIVE GROUP
 * ============================================================================
 *
 * Arbitrary numbers of objectives are supported.
 *
 * No fixed objective count exists.
 *
 * Examples of semantic policies include:
 *
 *     lexicographic
 *     weighted
 *     Pareto
 *     priority-based
 *
 * Their exact mathematical meaning belongs to semantic analysis.
 */

optimizationObjectiveGroup
    : qualifiedName
      LBRACE optimizationObjective* RBRACE
    ;


/*
 * ============================================================================
 * 7. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * It may contain:
 *
 *     capability predicates;
 *     resource predicates;
 *     semantic preservation conditions;
 *     optimizer properties;
 *     target-independent constraints.
 *
 * Examples:
 *
 *     require capability("tensor.compute")
 *     require resource.memory >= required_memory
 *     require semantic_equivalence == true
 */

optimizationRequirementClause
    : qualifiedName optimizationRequirementBody
    ;


optimizationRequirementBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 8. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict the legal optimization solution space.
 *
 * They do not necessarily identify a specific implementation.
 */

optimizationConstraintClause
    : CONSTRAINT optimizationConstraintBody
    ;


optimizationConstraintBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 9. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory.
 *
 * The optimizer may ignore a preference when satisfying stronger semantic
 * requirements or constraints requires another solution.
 */

optimizationPreferenceClause
    : PREFER optimizationPreferenceBody
    ;


optimizationPreferenceBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 10. HINTS
 * ============================================================================
 *
 * A hint MUST NOT be treated as a correctness requirement.
 *
 * Hints may be ignored by a conforming optimizer.
 */

optimizationHintClause
    : HINT optimizationHintBody
    ;


optimizationHintBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 11. PASS REFERENCES
 * ============================================================================
 *
 * A pass reference is symbolic.
 *
 * It is NOT a Rust type, function, module, or implementation identifier.
 *
 * Examples:
 *
 *     optimize::depth
 *     quantum::cancel_adjacent
 *     classical::vectorize
 *     hardware::aware
 *     future::pass
 *
 * The optimizer registry resolves the symbolic reference.
 */

optimizationPassClause
    : optimizationPassKeyword
      optimizationPassReference
      optimizationPassBody?
      SEMICOLON?
    ;


optimizationPassKeyword
    : qualifiedName
    ;


optimizationPassReference
    : qualifiedName
    ;


optimizationPassBody
    : optimizationArgumentBlock
    | optimizationPropertyBlock
    ;


optimizationPassList
    : optimizationPassReference
      (COMMA optimizationPassReference)*
    ;


/*
 * ============================================================================
 * 12. PIPELINES
 * ============================================================================
 *
 * Pipelines are ordered semantic optimization requests.
 *
 * Arbitrary pipeline length is supported.
 *
 * The grammar does not enumerate pass implementations.
 */

optimizationPipelineClause
    : optimizationPipelineKeyword
      qualifiedName?
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationPipelineKeyword
    : qualifiedName
    ;


optimizationPipelineItem
    : optimizationPipelinePass
    | optimizationPipelineGroup
    | optimizationPipelineReference
    | optimizationPipelineConditional
    | optimizationPipelineRepeated
    | optimizationPipelineProperty
    ;


optimizationPipelinePass
    : qualifiedName optimizationArgumentBlock?
      SEMICOLON?
    ;


optimizationPipelineGroup
    : LBRACKET optimizationPassList? RBRACKET
      SEMICOLON?
    ;


optimizationPipelineReference
    : qualifiedName SEMICOLON?
    ;


optimizationPipelineConditional
    : qualifiedName
      expression
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationPipelineRepeated
    : qualifiedName
      expression
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationPipelineProperty
    : qualifiedName
      (ASSIGN | COLON)
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. PASS ENABLE/DISABLE/EXCLUSION
 * ============================================================================
 *
 * These are symbolic policy operations.
 *
 * They do not mutate the optimizer registry.
 */

optimizationPassEnable
    : qualifiedName optimizationPassReference SEMICOLON?
    ;


optimizationPassDisable
    : qualifiedName optimizationPassReference SEMICOLON?
    ;


optimizationPassExclusion
    : qualifiedName
      LBRACE optimizationPassList? RBRACE
    ;


/*
 * ============================================================================
 * 14. BUDGETS
 * ============================================================================
 *
 * A budget limits compilation effort or an optimization policy.
 *
 * It is NOT a universal hardware limit.
 *
 * Valid semantic dimensions may include:
 *
 *     time
 *     memory
 *     evaluations
 *     rewrites
 *     iterations
 *     compilation effort
 *
 * The grammar does not enumerate or cap these dimensions.
 */

optimizationBudgetClause
    : qualifiedName optimizationBudgetBody
    ;


optimizationBudgetBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 15. TERMINATION
 * ============================================================================
 *
 * Termination describes when an optimization process may stop.
 *
 * Examples:
 *
 *     fixed_point
 *     until_stable
 *     until_no_progress
 *     resource_bounded
 *
 * The actual termination algorithm remains compiler-owned.
 */

optimizationTerminationClause
    : qualifiedName optimizationTerminationBody
    ;


optimizationTerminationBody
    : expression
    | optimizationPropertyBlock
    ;


optimizationFixedPoint
    : qualifiedName
      LBRACE optimizationPipelineItem* RBRACE
    ;


optimizationFixedPointPolicy
    : qualifiedName expression
    ;


/*
 * ============================================================================
 * 16. VERIFICATION
 * ============================================================================
 *
 * Optimization verification expresses confidence/correctness requirements.
 *
 * It is especially important for:
 *
 *     quantum computation;
 *     reversible computation;
 *     floating-point transformations;
 *     numerical algorithms;
 *     HDL;
 *     approximate transformations.
 *
 * Verification implementation remains outside the grammar.
 */

optimizationVerificationClause
    : qualifiedName optimizationVerificationBody
    ;


optimizationVerificationBody
    : expression
    | optimizationPropertyBlock
    ;


optimizationVerificationLevel
    : qualifiedName
    ;


optimizationVerificationLevelList
    : optimizationVerificationLevel
      (COMMA optimizationVerificationLevel)*
    ;


/*
 * ============================================================================
 * 17. SEMANTIC PRESERVATION
 * ============================================================================
 *
 * Optimization normally requires semantic preservation.
 *
 * This clause permits the programmer/compiler profile to state the required
 * preservation model explicitly.
 */

optimizationPreservationClause
    : qualifiedName optimizationPreservationBody
    ;


optimizationPreservationBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 18. APPROXIMATION
 * ============================================================================
 *
 * Approximation MUST NOT be silently introduced by an optimizer.
 *
 * A source program may explicitly permit approximate transformations and
 * provide an error/tolerance policy.
 */

optimizationApproximationClause
    : qualifiedName optimizationApproximationBody
    ;


optimizationApproximationBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 19. STOCHASTIC OPTIMIZATION
 * ============================================================================
 *
 * Stochastic behavior is explicit.
 *
 * If reproducibility is required, the semantic/compiler layer may require an
 * explicit seed or another deterministic source of randomness.
 *
 * The grammar does not impose a seed width or numeric range.
 */

optimizationStochasticClause
    : qualifiedName optimizationStochasticBody
    ;


optimizationStochasticBody
    : expression
    | optimizationPropertyBlock
    ;


optimizationSeed
    : qualifiedName expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 20. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility policy may cover:
 *
 *     deterministic transformation;
 *     stable pass ordering;
 *     explicit random seed;
 *     provenance;
 *     stable artifact identity;
 *     reproducible compilation.
 */

optimizationReproducibilityClause
    : qualifiedName optimizationReproducibilityBody
    ;


optimizationReproducibilityBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 21. SCOPE
 * ============================================================================
 *
 * Optimization can apply to a semantic scope.
 *
 * The grammar does not impose a fixed scope hierarchy.
 */

optimizationScopeClause
    : qualifiedName optimizationScopeBody
    ;


optimizationScopeBody
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 22. FALLBACK
 * ============================================================================
 *
 * Fallback describes alternate optimization strategies.
 *
 * A fallback does NOT imply that two strategies are semantically equivalent.
 *
 * Equivalence and legality are semantic-analysis responsibilities.
 */

optimizationFallbackClause
    : qualifiedName optimizationFallbackBody
    ;


optimizationFallbackBody
    : optimizationFallbackArm+
    | optimizationPropertyBlock
    ;


optimizationFallbackArm
    : optimizationPipelineItem
    | qualifiedName
    ;


/*
 * ============================================================================
 * 23. COST MODEL
 * ============================================================================
 *
 * Cost-model references are symbolic.
 *
 * Examples:
 *
 *     cost::gate_count
 *     cost::depth
 *     cost::latency
 *     quantum::logical_error
 *     hardware::energy
 *
 * The implementation of the cost model remains compiler-owned.
 */

optimizationCostModelClause
    : qualifiedName optimizationCostModelBody
    ;


optimizationCostModelBody
    : qualifiedName
    | expression
    | optimizationPropertyBlock
    ;


optimizationCostMetric
    : qualifiedName
    | expression
    ;


optimizationCostMetricList
    : optimizationCostMetric
      (COMMA optimizationCostMetric)*
    ;


/*
 * ============================================================================
 * 24. PROVENANCE
 * ============================================================================
 *
 * Provenance is important for:
 *
 *     reproducibility;
 *     auditability;
 *     debugging;
 *     certification;
 *     optimization comparison;
 *     deterministic builds.
 *
 * Serialization remains owned by the implementation.
 */

optimizationProvenanceClause
    : qualifiedName optimizationProvenanceBody
    ;


optimizationProvenanceBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 25. GENERIC PROPERTY
 * ============================================================================
 *
 * Properties provide an extensibility boundary for optimization dialects.
 *
 * They are semantic key/value information and do not create optimizer
 * implementations.
 */

optimizationPropertyClause
    : PROPERTY optimizationPropertyBlock
    ;


optimizationPropertyEntry
    : qualifiedName
      (ASSIGN | COLON)
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 26. OBJECTIVE COMPOSITION
 * ============================================================================
 *
 * Objective groups may be nested.
 *
 * This supports arbitrary multi-objective policies without imposing a finite
 * number of objectives or levels.
 */

optimizationComposition
    : qualifiedName
      LBRACE optimizationCompositionEntry* RBRACE
    ;


optimizationCompositionEntry
    : optimizationObjective
    | optimizationRequirementClause
    | optimizationConstraintClause
    | optimizationPreferenceClause
    | optimizationHintClause
    | optimizationPropertyClause
    | optimizationComposition
    ;


/*
 * ============================================================================
 * 27. ANALYSIS POLICY
 * ============================================================================
 *
 * Optimization may request analysis information before transformation.
 *
 * Analysis implementation remains owned by the optimizer.
 */

optimizationAnalysisPolicy
    : qualifiedName optimizationAnalysisBody
    ;


optimizationAnalysisBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 28. RESOURCE-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Optimization may consume resource intent.
 *
 * Resource definitions and capacities remain owned by:
 *
 *     grammar/resources/
 *
 * Hardware capability definitions remain owned by:
 *
 *     grammar/hardware/
 *
 * This grammar only represents optimization policy involving those semantic
 * values.
 */

optimizationResourcePolicy
    : RESOURCE optimizationResourceBody
    ;


optimizationResourceBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 29. TARGET-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Optimization may be influenced by a target-selection policy.
 *
 * Actual target selection remains outside this grammar.
 */

optimizationTargetPolicy
    : TARGET optimizationTargetBody
    ;


optimizationTargetBody
    : qualifiedName
    | expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 30. CAPABILITY-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Capability references are symbolic.
 *
 * Examples:
 *
 *     capability::tensor::compute
 *     capability::gpu::compute
 *     capability::quantum::measurement
 *
 * No capability list is hard-coded here.
 */

optimizationCapabilityPolicy
    : CAPABILITY optimizationCapabilityBody
    ;


optimizationCapabilityBody
    : qualifiedName
    | expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 31. GENERIC PROPERTY MAP
 * ============================================================================
 *
 * Property blocks are deliberately recursive only through expressions and
 * qualified names.
 *
 * They do not define a second object/data language.
 */

optimizationPropertyBlockEntryList
    : optimizationPropertyEntry*
    ;


/*
 * ============================================================================
 * 32. ARGUMENTS
 * ============================================================================
 *
 * Optimization arguments use the canonical expression grammar.
 *
 * This permits:
 *
 *     constants;
 *     variables;
 *     symbolic values;
 *     resource-derived values;
 *     compile-time values;
 *     capability predicates;
 *     future semantic expressions.
 */

optimizationArgumentList
    : optimizationArgument
      (COMMA optimizationArgument)*
    ;


optimizationArgument
    : qualifiedName ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 33. ASSIGNMENT BRIDGE
 * ============================================================================
 *
 * Kept as a named rule for consumers that need to identify optimization
 * property assignment structurally.
 */

optimizationAssignment
    : qualifiedName ASSIGN expression
    ;


/*
 * ============================================================================
 * 34. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser produces syntax only.
 *
 * Semantic analysis MUST:
 *
 *     - classify contextual optimization names;
 *     - resolve objective directions;
 *     - resolve objective expressions;
 *     - resolve pass references;
 *     - validate pass availability;
 *     - resolve profiles;
 *     - resolve capabilities;
 *     - resolve resource expressions;
 *     - validate requirements;
 *     - validate constraints;
 *     - distinguish preferences from requirements;
 *     - distinguish hints from guarantees;
 *     - validate fallback legality;
 *     - validate semantic-preservation requirements;
 *     - validate approximation permissions;
 *     - validate stochastic policy;
 *     - validate reproducibility requirements;
 *     - validate deterministic policies;
 *     - validate scope;
 *     - validate cost models;
 *     - validate provenance requests;
 *     - enforce dialect/version compatibility.
 *
 * The parser MUST NOT perform any of these semantic operations.
 *
 * ============================================================================
 * 35. AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral frontend AST should preserve, at minimum:
 *
 *     OptimizationSpecification
 *     OptimizationDirective
 *     OptimizationObjective
 *     OptimizationRequirement
 *     OptimizationConstraint
 *     OptimizationPreference
 *     OptimizationHint
 *     OptimizationPassReference
 *     OptimizationPipeline
 *     OptimizationPipelineItem
 *     OptimizationBudget
 *     OptimizationTermination
 *     OptimizationVerification
 *     OptimizationPreservation
 *     OptimizationApproximation
 *     OptimizationStochastic
 *     OptimizationReproducibility
 *     OptimizationScope
 *     OptimizationFallback
 *     OptimizationCostModel
 *     OptimizationProvenance
 *     OptimizationProperty
 *
 * Every AST node MUST preserve source spans.
 *
 * Expressions remain expression AST nodes.
 *
 * Qualified names remain structured qualified-name nodes.
 *
 * Optimization pass names MUST NOT be flattened into backend-specific types.
 *
 * ============================================================================
 * 36. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This file introduces NO IR.
 *
 * The transformation is:
 *
 *     source optimization intent
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic optimization policy
 *             |
 *             v
 *     existing canonical semantic representation
 *             |
 *             +--------------------+
 *             |                    |
 *             v                    v
 *        classical             quantum::ir
 *             |                    |
 *             +---------+----------+
 *                       |
 *                       v
 *                  optimizer
 *
 * Optimization metadata MUST NOT require a second quantum IR.
 *
 * ============================================================================
 * 37. EXISTING RUST OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * The grammar is intentionally independent from the concrete Rust optimizer
 * API.
 *
 * Existing implementation areas include optimization/optimizer infrastructure,
 * quantum optimization passes, objective representations, scheduling
 * optimization, fault-tolerant optimization, and compiler optimization
 * strategy infrastructure.
 *
 * Semantic lowering should map symbolic source references to those existing
 * registries/models rather than creating grammar-specific Rust implementations.
 *
 * In particular:
 *
 *     optimization pass name
 *         ->
 *     semantic pass identifier
 *         ->
 *     optimizer registry
 *         ->
 *     implementation
 *
 * The grammar MUST NOT reference Rust paths such as:
 *
 *     crate::optimizer::...
 *     crate::quantum::optimization::...
 *
 * ============================================================================
 * 38. QUANTUM OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Quantum optimization MUST operate on the established quantum semantic path:
 *
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimizer
 *
 * Quantum optimization may use objectives such as:
 *
 *     depth
 *     gate_count
 *     two_qubit_count
 *     estimated_error
 *     fidelity
 *     logical_cost
 *     T_depth
 *     communication_cost
 *
 * but this grammar MUST NOT enumerate these as a finite language-level list.
 *
 * This permits future quantum models, gate sets, hardware generations,
 * logical encodings, and optimization techniques without grammar changes.
 *
 * ============================================================================
 * 39. HDL OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * HDL optimization may involve:
 *
 *     area
 *     timing
 *     power
 *     latency
 *     throughput
 *     resource utilization
 *     pipeline depth
 *     verification cost
 *
 * but no fixed:
 *
 *     bus width;
 *     register width;
 *     LUT count;
 *     memory size;
 *     FPGA capacity
 *
 * is encoded here.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * 40. CLASSICAL / AI / DATA CONTRACT
 * ============================================================================
 *
 * Optimization intent may apply to:
 *
 *     scalar;
 *     vector;
 *     matrix;
 *     tensor;
 *     numerical;
 *     symbolic;
 *     AI/ML;
 *     dataflow;
 *     distributed;
 *     heterogeneous;
 *     future computations.
 *
 * Framework-specific optimizer implementations remain outside the grammar.
 *
 * ============================================================================
 * 41. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no artificial finite maximum on:
 *
 *     objectives;
 *     objective groups;
 *     passes;
 *     pipeline items;
 *     pipeline nesting;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     properties;
 *     fallback arms;
 *     qualified-name depth;
 *     expression complexity.
 *
 * The `+` and `*` operators deliberately represent unbounded language-level
 * repetition subject only to implementation resources.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-level ceiling.
 *
 * It does NOT claim:
 *
 *     infinite memory;
 *     infinite compiler time;
 *     infinite hardware;
 *     infinite target resources.
 *
 * Resource exhaustion must be diagnosed separately from invalid syntax.
 *
 * ============================================================================
 * 42. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST NOT contain universal limits such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_ACCELERATOR_COUNT
 *     MAX_PIPELINE_STAGES
 *     MAX_PASSES
 *     MAX_OBJECTIVES
 *
 * It MUST NOT encode disguised equivalents of these values.
 *
 * Numeric values inside expressions remain program/policy data.
 *
 * ============================================================================
 * 43. SECURITY CONTRACT
 * ============================================================================
 *
 * Optimization syntax is declarative.
 *
 * It MUST NOT cause parsing or semantic lowering to:
 *
 *     - execute shell commands;
 *     - execute arbitrary host code;
 *     - access credentials;
 *     - access secrets;
 *     - read arbitrary files;
 *     - contact arbitrary networks;
 *     - inspect hardware directly;
 *     - invoke devices;
 *     - invoke external optimizers implicitly.
 *
 * Any external integration must occur through explicit compiler-controlled
 * interfaces after semantic validation.
 *
 * ============================================================================
 * 44. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Diagnostics should distinguish:
 *
 *     syntax error
 *     unknown optimization directive
 *     unknown objective direction
 *     unresolved objective
 *     unresolved pass
 *     invalid pass configuration
 *     invalid pipeline
 *     contradictory requirement
 *     violated constraint
 *     unavailable preference
 *     invalid hint
 *     invalid fallback
 *     invalid preservation policy
 *     approximation not permitted
 *     reproducibility requirement unsatisfied
 *     unsupported cost model
 *     unavailable compiler resources
 *
 * An unavailable optimization implementation MUST NOT automatically be
 * reported as malformed source syntax.
 *
 * ============================================================================
 * 45. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing ownership remains:
 *
 *     grammar/compile/optimization.g4
 *         -> optimization intent
 *
 *     grammar/compile/compilation.g4
 *         -> compilation composition
 *
 *     grammar/compile/target.g4
 *         -> target declarations
 *
 *     grammar/compile/target-selection.g4
 *         -> target selection policy
 *
 *     grammar/resources/
 *         -> resource semantics
 *
 *     grammar/hardware/
 *         -> hardware capability/target semantics
 *
 *     grammar/expressions/
 *         -> expression syntax
 *
 *     grammar/core/
 *         -> names and core syntax
 *
 * No existing filename needs to be renamed.
 *
 * ============================================================================
 * 46. REQUIRED COMPOSITION
 * ============================================================================
 *
 * `grammar/compile/compilation.g4` already imports:
 *
 *     CompileOptimization
 *
 * and exposes:
 *
 *     compilationOptimizationReference
 *
 * which delegates to:
 *
 *     optimizationDeclaration
 *
 * Therefore no duplicate optimization dispatcher should be added elsewhere.
 *
 * The canonical parser hierarchy remains:
 *
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *          Compilation
 *             |
 *             v
 *      CompileOptimization
 *
 * ============================================================================
 * 47. REQUIRED LEXICAL POLICY
 * ============================================================================
 *
 * The current canonical lexer deliberately does not require a finite
 * optimization keyword enumeration.
 *
 * Contextual names such as:
 *
 *     optimize
 *     minimize
 *     maximize
 *
 * may therefore remain ordinary identifiers.
 *
 * This is intentional.
 *
 * It prevents every future optimizer feature from requiring a lexer change.
 *
 * If a future language revision promotes one of these names to a globally
 * reserved keyword, that change belongs to:
 *
 *     grammar/lexer/keywords.g4
 *     grammar/spec/lexical.md
 *     grammar/compatibility/
 *
 * and is NOT silently introduced here.
 *
 * ============================================================================
 * 48. POSITIVE CONFORMANCE FORMS
 * ============================================================================
 *
 * The semantic test suite should cover forms equivalent to:
 *
 *     optimize;
 *
 *     minimize depth;
 *
 *     maximize fidelity;
 *
 *     require capability::quantum::measurement;
 *
 *     require resource::memory >= required_memory;
 *
 *     constraint latency <= deadline;
 *
 *     prefer capability::gpu::compute;
 *
 *     hint optimization::vectorize;
 *
 *     pass quantum::cancel_adjacent;
 *
 *     pipeline optimization {
 *         quantum::cancel_adjacent;
 *         quantum::merge_rotations;
 *     }
 *
 *     budget compilation_time;
 *
 *     verify semantic_equivalence;
 *
 *     preserve measurement_semantics;
 *
 *     approximate error <= tolerance;
 *
 *     stochastic seed;
 *
 *     reproducible deterministic;
 *
 *     scope function_name;
 *
 *     fallback quantum::strategy classical::strategy;
 *
 *     cost_model quantum::logical_cost;
 *
 * The exact interpretation of each contextual word belongs to semantic
 * analysis.
 *
 * ============================================================================
 * 49. NEGATIVE CONFORMANCE
 * ============================================================================
 *
 * Structural tests should reject malformed constructs such as:
 *
 *     optimize(
 *
 *     optimize {
 *
 *     pipeline {
 *
 *     require;
 *
 *     constraint;
 *
 *     prefer;
 *
 *     hint;
 *
 *     pass;
 *
 *     minimize;
 *
 *     maximize;
 *
 *     optimize {
 *         invalid =
 *     }
 *
 * Semantic tests should reject:
 *
 *     unresolved pass references;
 *     contradictory constraints;
 *     impossible requirements;
 *     invalid objective expressions;
 *     unsupported preservation claims;
 *     forbidden approximation;
 *     invalid fallback semantics.
 *
 * ============================================================================
 * 50. SCALABILITY CONFORMANCE
 * ============================================================================
 *
 * Tests MUST include:
 *
 *     many objectives;
 *     many requirements;
 *     many constraints;
 *     many preferences;
 *     many hints;
 *     many passes;
 *     long pipelines;
 *     nested pipeline groups;
 *     deeply qualified names;
 *     large symbolic expressions;
 *     many fallback alternatives.
 *
 * Tests MUST NOT turn those cases into a language-level capacity limit.
 *
 * Resource exhaustion belongs to compiler/resource diagnostics.
 *
 * ============================================================================
 * 51. DETERMINISM CONFORMANCE
 * ============================================================================
 *
 * Given the same:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *
 * parsing MUST produce the same parse structure.
 *
 * Semantic optimization determinism must be separately tested through:
 *
 *     reproducibility policy;
 *     deterministic pass ordering;
 *     explicit seed policy;
 *     stable identifiers;
 *     provenance.
 *
 * ============================================================================
 * 52. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] optimization has one public entry point;
 *     [x] canonical expression grammar is reused;
 *     [x] canonical qualified-name grammar is reused;
 *     [x] no local identifier rule exists;
 *     [x] no local qualified-name rule exists;
 *     [x] DOUBLE_COLON is inherited from canonical names;
 *     [x] objectives are extensible;
 *     [x] requirements are distinct;
 *     [x] constraints are distinct;
 *     [x] preferences are distinct;
 *     [x] hints are distinct;
 *     [x] passes are symbolic;
 *     [x] pipelines are arbitrary-length;
 *     [x] budgets are policy values;
 *     [x] termination is compiler-owned;
 *     [x] verification is compiler-owned;
 *     [x] approximation is explicit;
 *     [x] stochastic behavior is explicit;
 *     [x] reproducibility is explicit;
 *     [x] fallback is semantic policy;
 *     [x] provenance is representable;
 *     [x] cost models are symbolic;
 *     [x] resources remain owned by resources/;
 *     [x] hardware remains owned by hardware/;
 *     [x] target selection remains downstream;
 *     [x] routing remains downstream;
 *     [x] scheduling remains downstream;
 *     [x] QEC remains downstream;
 *     [x] ZQN remains downstream;
 *     [x] quantum::ir remains canonical;
 *     [x] no machine capacity is encoded;
 *     [x] no vendor enumeration is encoded;
 *     [x] no implementation code is embedded;
 *     [x] no unsafe Rust is required;
 *     [x] Rust 1.97 / 1.97.1 remains the implementation baseline;
 *     [x] scalability is structural;
 *     [x] deterministic parsing is specified;
 *     [x] diagnostics are classified;
 *     [x] compatibility ownership is explicit.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What optimization intent does the source program express?"
 *
 * It does NOT answer:
 *
 *     "Which optimizer implementation executes it?"
 *
 *     "Which physical device is used?"
 *
 *     "Which qubit is used?"
 *
 *     "Which CPU core is used?"
 *
 *     "Which GPU is used?"
 *
 *     "Which FPGA resource is used?"
 *
 *     "How is the computation routed?"
 *
 *     "How is the computation scheduled?"
 *
 *     "How is QEC performed?"
 *
 *     "How does ZQN model the target?"
 *
 * Those decisions remain downstream.
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     optimization intent
 *       |
 *       v
 *     domain-neutral semantic model
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *   classical             quantum::ir
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *             optimizer
 *                 |
 *          routing/scheduling
 *                 |
 *          resilience/QEC/ZQN
 *                 |
 *                HAL
 *                 |
 *          target realization
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * while allowing optimization quality to scale with the resources actually
 * available at compilation and execution time.
 *
 * ============================================================================
 */