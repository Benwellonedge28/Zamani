/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/compile/optimization.g4
 *
 * Grammar:
 *     CompileOptimization
 *
 * Status:
 *     PRODUCTION SOURCE-LEVEL OPTIMIZATION-INTENT GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 or later
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the sole parser-level owner of SOURCE-LEVEL OPTIMIZATION
 * INTENT within grammar/compile/.
 *
 * It describes what optimization behavior a source program:
 *
 *     - requests;
 *     - permits;
 *     - requires;
 *     - prefers;
 *     - forbids through constraints;
 *     - hints;
 *     - verifies;
 *     - preserves;
 *     - approximates;
 *     - makes reproducible;
 *     - scopes;
 *     - composes into pipelines;
 *     - associates with symbolic optimizer strategies.
 *
 * This file does NOT implement optimization.
 *
 *
 * OWNS
 * -----
 *
 *     optimizationDeclaration
 *     optimizationSpecification
 *     optimizationClause
 *     optimizationObjective
 *     optimizationRequirement
 *     optimizationConstraint
 *     optimizationPreference
 *     optimizationHint
 *     optimizationPass
 *     optimizationPipeline
 *     optimizationBudget
 *     optimizationTermination
 *     optimizationVerification
 *     optimizationPreservation
 *     optimizationApproximation
 *     optimizationStochastic
 *     optimizationReproducibility
 *     optimizationScope
 *     optimizationFallback
 *     optimizationProvenance
 *     optimizationCostModel
 *     optimizationAnalysis
 *     optimizationProperty
 *     optimizationDirective
 *
 *
 * DOES NOT OWN
 * ------------
 *
 *     lexer vocabulary
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     generic requirements
 *     generic constraints
 *     generic hints
 *     general policies
 *     resources
 *     capabilities
 *     targets
 *     target selection
 *     feature selection
 *     conditional compilation
 *     specialization
 *     code generation
 *     lowering
 *     artifacts
 *     reproducibility implementation
 *     caching
 *     deployment
 *     classical IR
 *     quantum::ir
 *     quantum operations
 *     HDL semantics
 *     hardware realization
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/core.g4
 *     grammar/expressions/expressions.g4
 *     grammar/core/requirements.g4
 *     grammar/core/constraints.g4
 *     grammar/core/hints.g4
 *
 *
 * IMPORTS
 * -------
 *
 *     Core
 *     Expressions
 *     Requirements
 *     Constraints
 *     Hints
 *
 *
 * EXPORTS
 * -------
 *
 *     optimizationDeclaration
 *
 * Secondary reusable rules are exported only as parser-composition
 * implementation details unless explicitly promoted by the specification.
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/compile/compile.g4
 *     grammar/compile/compilation.g4
 *
 *
 * AST_OWNER
 * ---------
 *
 * Domain-neutral frontend AST.
 *
 * The AST must preserve:
 *
 *     - source span;
 *     - source order;
 *     - optimization clause kind;
 *     - symbolic names;
 *     - expressions;
 *     - nested pipeline structure;
 *     - argument structure;
 *     - explicit policy distinctions.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Compiler semantic-analysis / optimization-planning subsystem.
 *
 * Semantic analysis resolves:
 *
 *     objective meaning
 *     pass availability
 *     pass compatibility
 *     pipeline legality
 *     requirement satisfaction
 *     constraint consistency
 *     preference strength
 *     hint advisory status
 *     preservation obligations
 *     verification obligations
 *     approximation permissions
 *     stochastic policy
 *     reproducibility policy
 *     fallback legality
 *     scope
 *     cost-model identity
 *     provenance policy
 *     capability/resource interactions
 *
 *
 * TYPE_OWNER
 * ----------
 *
 * Existing type system.
 *
 * This grammar never defines a second type system for optimization.
 *
 *
 * EFFECT_OWNER
 * ------------
 *
 * Existing effect subsystem.
 *
 * Optimization intent does not itself execute an effect.
 *
 * Semantic analysis may determine that particular optimization strategies
 * interact with effects such as:
 *
 *     randomness
 *     native
 *     foreign
 *     reflection
 *     code_generation
 *     measurement
 *     distributed
 *     simulation
 *
 *
 * CAPABILITY_OWNER
 * ----------------
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *
 * Optimization capability references remain symbolic.
 *
 *
 * RESOURCE_OWNER
 * --------------
 *
 *     grammar/resources/
 *
 * Resource expressions are not redefined here.
 *
 *
 * CONTRACT_OWNER
 * --------------
 *
 *     grammar/core/requirements.g4
 *     grammar/core/constraints.g4
 *     grammar/validation/
 *
 *
 * POLICY_OWNER
 * ------------
 *
 *     grammar/core/policies.g4
 *     grammar/policies/
 *
 * Optimization-specific policy syntax is represented here only where it
 * is genuinely optimization-specific.
 *
 *
 * PROVENANCE_OWNER
 * ----------------
 *
 *     grammar/spec/provenance.md
 *     compiler provenance subsystem
 *
 * This grammar only records provenance intent.
 *
 *
 * IR_OWNER
 * --------
 *
 * This grammar owns NO IR.
 *
 * The canonical downstream representation remains the existing semantic
 * representation followed by the appropriate domain IR.
 *
 * Quantum computation MUST continue through:
 *
 *     quantum::ir
 *
 * No optimization-specific quantum IR is introduced.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *   classical                 quantum::ir
 *       |                         |
 *       +------------+------------+
 *                    |
 *                    v
 *                optimization
 *                    |
 *                    v
 *                 lowering
 *                    |
 *             routing/scheduling
 *                    |
 *            resilience / QEC
 *                    |
 *                   ZQN
 *                    |
 *                   HAL
 *                    |
 *             target realization
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Optimization intent MUST describe semantic goals and transformation policy,
 * not a finite hardware universe.
 *
 * This file MUST NOT define universal limits for:
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
 *     pipeline length
 *     pass count
 *     objective count
 *
 * Repetition is structural and uses ANTLR repetition operators.
 *
 * Practical implementation limits remain compiler-resource limits rather than
 * language semantics.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     devices
 *     filesystem
 *     network
 *     environment
 *     runtime state
 *     scheduler state
 *     randomness
 *     wall-clock time
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no semantic predicates;
 *     no executable callbacks;
 *     no filesystem access;
 *     no network access;
 *     no device access;
 *     no embedded Rust.
 *
 * Generated Rust integration MUST remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * No unsafe Rust is required.
 *
 *
 * ============================================================================
 * OWNERSHIP BOUNDARIES
 * ============================================================================
 *
 * OBJECTIVE
 * ---------
 *
 * What should improve.
 *
 * PASS
 * ----
 *
 * Which symbolic transformation family is requested.
 *
 * PIPELINE
 * --------
 *
 * How symbolic transformation stages are composed.
 *
 * REQUIREMENT
 * -----------
 *
 * What MUST hold.
 *
 * CONSTRAINT
 * ----------
 *
 * What restricts the legal optimization solution space.
 *
 * PREFERENCE
 * ----------
 *
 * What SHOULD be favored when feasible.
 *
 * HINT
 * ----
 *
 * Advisory information that may be ignored.
 *
 * VERIFICATION
 * ------------
 *
 * What correctness/verification obligation applies.
 *
 * PRESERVATION
 * ------------
 *
 * What semantic property must survive transformation.
 *
 * APPROXIMATION
 * -------------
 *
 * Whether controlled semantic approximation is permitted.
 *
 * REPRODUCIBILITY
 * ---------------
 *
 * What reproducibility behavior is required.
 *
 * None of these concepts represents the optimizer implementation itself.
 *
 *
 * ============================================================================
 */

parser grammar CompileOptimization;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    Requirements,
    Constraints,
    Hints;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical source-level form:
 *
 *     compile optimize;
 *
 *     compile optimize
 *         minimize execution::cost;
 *         maximize performance;
 *         pass classical::vectorize;
 *     ;
 *
 * The surrounding compile grammar owns the `compile` boundary.
 *
 * This grammar owns the `optimize` boundary.
 *
 * A dedicated OPTIMIZE token is required for an unambiguous production
 * composition boundary.
 */

optimizationDeclaration
    : OPTIMIZE optimizationSpecification? SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. SPECIFICATION
 * ============================================================================
 *
 * There is no finite maximum number of optimization clauses.
 */

optimizationSpecification
    : optimizationClause+
    ;


/*
 * ============================================================================
 * 3. CLAUSE DISPATCH
 * ============================================================================
 *
 * Standard clauses begin with reserved lexical boundaries.
 *
 * The final optimizationDirective branch is the open-world extension point.
 *
 * It begins with a qualified name and therefore cannot steal constructs whose
 * first token is one of the reserved optimization keywords above.
 */

optimizationClause
    : optimizationObjectiveClause
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
    | optimizationAnalysisClause
    | optimizationPropertyClause
    | optimizationDirective
    ;


/*
 * ============================================================================
 * 4. OBJECTIVES
 * ============================================================================
 *
 * Canonical forms:
 *
 *     minimize expression;
 *     maximize expression;
 *
 *     objective minimize expression;
 *     objective maximize expression;
 *
 * The objective expression is the canonical Zamani expression.
 *
 * Objective identity is semantic rather than a finite keyword catalogue.
 */

optimizationObjectiveClause
    : MINIMIZE optimizationObjectiveTarget optimizationObjectiveModifier* SEMICOLON?
    | MAXIMIZE optimizationObjectiveTarget optimizationObjectiveModifier* SEMICOLON?
    | OBJECTIVE optimizationObjectiveDirection optimizationObjectiveTarget
      optimizationObjectiveModifier*
      SEMICOLON?
    ;


optimizationObjectiveDirection
    : MINIMIZE
    | MAXIMIZE
    | qualifiedName
    ;


optimizationObjectiveTarget
    : expression
    ;


optimizationObjectiveModifier
    : optimizationNamedArgument
    ;


optimizationNamedArgument
    : qualifiedName ASSIGN expression
    ;


optimizationObjectiveGroup
    : OBJECTIVE LBRACE
      optimizationObjectiveClause+
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 5. REQUIREMENTS
 * ============================================================================
 *
 * Generic requirement semantics remain owned by Requirements.
 *
 * This file only composes the canonical requirement into optimization intent.
 *
 * Examples:
 *
 *     requires quantum::measurement;
 *
 *     requires tensor::compute and parallel::execution;
 *
 * No resource capacity is encoded here.
 */

optimizationRequirementClause
    : requirementClause SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. CONSTRAINTS
 * ============================================================================
 *
 * Generic constraint syntax remains owned by Constraints.
 */

optimizationConstraintClause
    : constraintClause SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. PREFERENCES
 * ============================================================================
 *
 * Preference meaning is advisory.
 *
 * The payload is an ordinary Zamani expression.
 *
 * No target or implementation is selected here.
 */

optimizationPreferenceClause
    : PREFER expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. HINTS
 * ============================================================================
 *
 * Hints remain advisory and may be ignored by a conforming optimizer.
 *
 * The canonical hint grammar owns the hint expression structure.
 */

optimizationHintClause
    : hintClause SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. SYMBOLIC OPTIMIZATION PASSES
 * ============================================================================
 *
 * A pass name is an OPEN-WORLD symbolic identifier.
 *
 * It is not:
 *
 *     a Rust path;
 *     a function call;
 *     a backend;
 *     a hardware device;
 *     a vendor catalogue entry.
 *
 * Examples:
 *
 *     pass classical::vectorize;
 *     pass quantum::cancel_adjacent;
 *     pass tensor::fusion;
 *     pass future::optimization;
 */

optimizationPassClause
    : PASS optimizationPassReference optimizationInvocation? SEMICOLON?
    ;


optimizationPassReference
    : qualifiedName
    ;


optimizationInvocation
    : optimizationArgumentBlock
    ;


optimizationArgumentBlock
    : LPAREN optimizationArgumentList? RPAREN
    ;


optimizationArgumentList
    : optimizationArgument
      (COMMA optimizationArgument)*
      COMMA?
    ;


optimizationArgument
    : qualifiedName ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 10. OPTIMIZATION PIPELINES
 * ============================================================================
 *
 * Pipelines are ordered semantic transformation requests.
 *
 * They are unbounded by grammar-level cardinality.
 *
 * Example:
 *
 *     pipeline quantum {
 *         quantum::cancel_adjacent;
 *         quantum::merge_rotations;
 *         quantum::constant_fold;
 *     }
 */

optimizationPipelineClause
    : PIPELINE qualifiedName? LBRACE
      optimizationPipelineItem+
      RBRACE
      SEMICOLON?
    ;


optimizationPipelineItem
    : optimizationPipelinePass
    | optimizationPipelinePipeline
    | optimizationPipelineProperty
    ;


optimizationPipelinePass
    : qualifiedName optimizationArgumentBlock? SEMICOLON?
    ;


optimizationPipelinePipeline
    : PIPELINE qualifiedName? LBRACE
      optimizationPipelineItem+
      RBRACE
      SEMICOLON?
    ;


optimizationPipelineProperty
    : qualifiedName
      (ASSIGN | COLON)
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. BUDGET
 * ============================================================================
 *
 * A budget is a semantic/compiler policy value.
 *
 * It is NOT a hardware capacity declaration.
 *
 * Example:
 *
 *     budget compilation_time <= compile_budget;
 *
 *     budget optimization_effort;
 *
 * The expression is interpreted downstream.
 */

optimizationBudgetClause
    : BUDGET expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. TERMINATION
 * ============================================================================
 *
 * Termination policy is symbolic.
 *
 * The optimizer implementation determines how the policy is realized.
 *
 * Examples:
 *
 *     terminate optimization::stable;
 *     terminate optimization::no_progress;
 *     terminate optimization::resource_bounded;
 */

optimizationTerminationClause
    : TERMINATE expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 13. VERIFICATION
 * ============================================================================
 *
 * Verification obligations are explicit.
 *
 * The grammar does not perform verification.
 */

optimizationVerificationClause
    : VERIFY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. PRESERVATION
 * ============================================================================
 *
 * Preservation identifies semantics that optimization must retain.
 *
 * Examples:
 *
 *     preserve semantic_equivalence;
 *     preserve measurement_semantics;
 *     preserve effects;
 *     preserve provenance;
 *     preserve contracts;
 */

optimizationPreservationClause
    : PRESERVE expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. APPROXIMATION
 * ============================================================================
 *
 * Approximation must be explicit.
 *
 * An optimizer MUST NOT silently introduce an approximation merely because
 * a target implementation cannot provide exact preservation.
 */

optimizationApproximationClause
    : APPROXIMATE expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. STOCHASTIC OPTIMIZATION
 * ============================================================================
 *
 * Stochastic optimization may be declared explicitly.
 *
 * Example:
 *
 *     stochastic optimization::search;
 *
 * Reproducibility and seed semantics remain compiler-owned.
 */

optimizationStochasticClause
    : STOCHASTIC expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility can be expressed independently from stochasticity.
 *
 * Examples:
 *
 *     reproducible;
 *     reproducible deterministic;
 *     reproducible optimization::stable;
 */

optimizationReproducibilityClause
    : REPRODUCIBLE optimizationReproducibilityBody? SEMICOLON?
    ;


optimizationReproducibilityBody
    : DETERMINISTIC
    | expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 18. SCOPE
 * ============================================================================
 *
 * Scope identifies where optimization intent applies.
 *
 * It does not impose a finite scope hierarchy.
 */

optimizationScopeClause
    : SCOPE qualifiedName SEMICOLON?
    ;


/*
 * ============================================================================
 * 19. FALLBACK
 * ============================================================================
 *
 * Fallback strategies are symbolic alternatives.
 *
 * Fallback does not imply semantic equivalence.
 *
 * Semantic analysis must establish that a fallback preserves all mandatory
 * contracts before it may be used.
 *
 * Examples:
 *
 *     fallback quantum::strategy classical::strategy;
 *
 *     fallback {
 *         quantum::preferred;
 *         classical::portable;
 *     }
 */

optimizationFallbackClause
    : FALLBACK optimizationFallbackSpecification SEMICOLON?
    ;


optimizationFallbackSpecification
    : optimizationFallbackArm+
    | optimizationPropertyBlock
    ;


optimizationFallbackArm
    : qualifiedName
    | optimizationPipeline
    ;


optimizationPipeline
    : PIPELINE qualifiedName? LBRACE
      optimizationPipelineItem+
      RBRACE
    ;


/*
 * ============================================================================
 * 20. PROVENANCE
 * ============================================================================
 *
 * Provenance is declarative metadata about optimization decisions.
 *
 * It may be consumed by:
 *
 *     reproducibility
 *     diagnostics
 *     auditing
 *     certification
 *     debugging
 *     scientific workflows
 *
 * The grammar does not generate provenance records.
 */

optimizationProvenanceClause
    : PROVENANCE optimizationProvenanceBody SEMICOLON?
    ;


optimizationProvenanceBody
    : expression
    | optimizationPropertyBlock
    ;


/*
 * ============================================================================
 * 21. COST MODEL
 * ============================================================================
 *
 * Cost models are symbolic.
 *
 * No finite metric catalogue is imposed.
 *
 * Examples:
 *
 *     cost quantum::logical_error;
 *     cost classical::latency;
 *     cost hardware::energy;
 *     cost tensor::memory_traffic;
 */

optimizationCostModelClause
    : COST qualifiedName optimizationCostModelArguments? SEMICOLON?
    ;


optimizationCostModelArguments
    : optimizationArgumentBlock
    ;


/*
 * ============================================================================
 * 22. ANALYSIS
 * ============================================================================
 *
 * Optimization may request analysis information.
 *
 * Analysis itself remains optimizer-owned.
 *
 * Examples:
 *
 *     analyze quantum::depth;
 *     analyze classical::aliasing;
 *     analyze tensor::locality;
 */

optimizationAnalysisClause
    : ANALYZE qualifiedName optimizationInvocation? SEMICOLON?
    ;


/*
 * ============================================================================
 * 23. GENERIC PROPERTY BLOCK
 * ============================================================================
 *
 * Property blocks provide structured extensibility without creating another
 * configuration language.
 *
 * Values remain ordinary Zamani expressions.
 */

optimizationPropertyClause
    : PROPERTY optimizationPropertyBlock SEMICOLON?
    ;


optimizationPropertyBlock
    : LBRACE optimizationPropertyEntry* RBRACE
    ;


optimizationPropertyEntry
    : qualifiedName
      (ASSIGN | COLON)
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 24. OPEN-WORLD OPTIMIZATION DIRECTIVES
 * ============================================================================
 *
 * Future optimization technologies may be represented symbolically.
 *
 * Example:
 *
 *     future::optimizer {
 *         mode = expression;
 *     }
 *
 * The semantic layer determines whether the directive is:
 *
 *     stable;
 *     experimental;
 *     dialect-provided;
 *     implementation-provided;
 *     deprecated;
 *     unknown.
 *
 * Unknown does not become a parser-level hardware limitation.
 */

optimizationDirective
    : qualifiedName optimizationDirectiveBody?
    ;


optimizationDirectiveBody
    : optimizationArgumentBlock
    | optimizationPropertyBlock
    | ASSIGN expression
    ;


/*
 * ============================================================================
 * 25. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. Resolve every symbolic optimization identity.
 *
 *     2. Distinguish objective from pass.
 *
 *     3. Distinguish requirement from constraint.
 *
 *     4. Distinguish preference from hint.
 *
 *     5. Validate objective expressions.
 *
 *     6. Validate pass arguments.
 *
 *     7. Validate pipeline ordering.
 *
 *     8. Validate nested pipeline legality.
 *
 *     9. Resolve capability references through the canonical capability model.
 *
 *    10. Resolve resource expressions through the canonical resource model.
 *
 *    11. Validate preservation obligations.
 *
 *    12. Validate verification obligations.
 *
 *    13. Reject approximation when policy/contracts forbid it.
 *
 *    14. Validate stochastic/reproducibility interaction.
 *
 *    15. Validate fallback semantic compatibility.
 *
 *    16. Resolve cost models.
 *
 *    17. Resolve optimization scope.
 *
 *    18. Attach provenance.
 *
 *    19. Preserve source spans.
 *
 *    20. Produce diagnostics that distinguish syntax from semantic
 *        infeasibility.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should expose stable domain-neutral nodes equivalent to:
 *
 *     OptimizationDeclaration
 *     OptimizationSpecification
 *     OptimizationObjective
 *     OptimizationRequirement
 *     OptimizationConstraint
 *     OptimizationPreference
 *     OptimizationHint
 *     OptimizationPass
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
 *     OptimizationProvenance
 *     OptimizationCostModel
 *     OptimizationAnalysis
 *     OptimizationProperty
 *     OptimizationDirective
 *
 * These are conceptual AST contracts, not Rust definitions.
 *
 * Expressions remain expression AST nodes.
 *
 * Qualified names remain structured qualified-name nodes.
 *
 * No AST node may contain:
 *
 *     PhysicalQubitId
 *     CpuCoreId
 *     GpuId
 *     FpgaResourceId
 *     DeviceId
 *
 * as a consequence of this grammar.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Optimization expressions are ordinary Zamani expressions.
 *
 * Their type correctness is established by semantic analysis.
 *
 * For example:
 *
 *     budget compilation_time <= required_budget;
 *
 * is accepted syntactically.
 *
 * Whether the compared values have compatible semantic units is a downstream
 * type/resource analysis concern.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Optimization syntax has no execution effect by itself.
 *
 * A downstream optimizer implementation may interact with effects, but the
 * optimization declaration itself does not perform:
 *
 *     I/O
 *     network communication
 *     native execution
 *     foreign execution
 *     mutation
 *     randomness
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *
 * unless the semantic/compiler policy explicitly associates those effects
 * with a selected implementation.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability references are symbolic.
 *
 * Examples include:
 *
 *     quantum::measurement
 *     tensor::compute
 *     distributed::collectives
 *     hardware::acceleration
 *     execution::deterministic
 *
 * No finite capability catalogue is defined here.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Optimization may depend on resource expressions through ordinary
 * expressions and canonical requirement/constraint models.
 *
 * This grammar does not define:
 *
 *     memory size limits;
 *     processor counts;
 *     GPU counts;
 *     QPU counts;
 *     node limits;
 *     tensor limits;
 *     topology limits.
 *
 * Resource feasibility remains downstream.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Optimization MUST NOT silently weaken:
 *
 *     requires;
 *     ensures;
 *     invariant;
 *     guarantees;
 *     security policies;
 *     provenance policies;
 *     semantic-preservation obligations.
 *
 * If no legal optimization satisfies a mandatory contract, semantic
 * compilation must report failure rather than silently changing program
 * meaning.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum optimization is target-independent at this grammar layer.
 *
 * The required boundary is:
 *
 *     source
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition/routing/scheduling
 *       |
 *       v
 *     resilience/QEC/ZQN
 *       |
 *       v
 *     HAL
 *
 * This file MUST NOT:
 *
 *     enumerate physical qubits;
 *     encode coupling maps;
 *     encode calibration;
 *     encode device topology;
 *     enumerate vendor gate sets;
 *     create a second quantum IR.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Optimization may express symbolic objectives such as:
 *
 *     area;
 *     timing;
 *     power;
 *     latency;
 *     throughput;
 *     resource utilization.
 *
 * These remain expressions/semantic identities.
 *
 * No fixed bus width, register width, FPGA capacity, or ASIC resource count
 * is defined here.
 *
 *
 * ============================================================================
 * CLASSICAL / AI / DATA BOUNDARY
 * ============================================================================
 *
 * The same optimization grammar applies to:
 *
 *     scalar computation;
 *     numerical computation;
 *     vector computation;
 *     matrix computation;
 *     tensor computation;
 *     symbolic computation;
 *     AI/ML;
 *     dataflow;
 *     distributed computation;
 *     networking;
 *     heterogeneous computation;
 *     future computational domains.
 *
 * Domain-specific optimization strategies are registered semantically rather
 * than enumerated here.
 *
 *
 * ============================================================================
 * LOWERING BOUNDARY
 * ============================================================================
 *
 * Optimization occurs before lowering.
 *
 * This grammar MUST NOT contain:
 *
 *     lowering rules;
 *     backend selection;
 *     target realization;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * The downstream boundary is:
 *
 *     semantic representation
 *         ->
 *     optimization
 *         ->
 *     lowering
 *         ->
 *     routing/scheduling
 *         ->
 *     resilience
 *         ->
 *     realization
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify malformed syntax such as:
 *
 *     missing optimization boundary;
 *     missing objective expression;
 *     missing pass name;
 *     malformed pipeline;
 *     malformed property assignment;
 *     malformed argument list;
 *     missing closing delimiter.
 *
 * Semantic diagnostics should separately identify:
 *
 *     unknown pass;
 *     unavailable pass implementation;
 *     incompatible pass;
 *     invalid objective;
 *     contradictory constraints;
 *     unsatisfied requirement;
 *     forbidden approximation;
 *     invalid fallback;
 *     incompatible cost model;
 *     invalid preservation claim;
 *     unsupported verification level;
 *     unavailable compiler resources.
 *
 * Hardware/resource infeasibility MUST NOT be reported as malformed syntax.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing composition ownership remains:
 *
 *     grammar/compile/compile.g4
 *         -> compileDeclaration / compileClause
 *
 *     grammar/compile/compilation.g4
 *         -> compilation composition
 *
 *     grammar/compile/optimization.g4
 *         -> optimizationDeclaration
 *
 *     grammar/resources/
 *         -> resource semantics
 *
 *     grammar/compile/target.g4
 *         -> target intent
 *
 *     grammar/compile/target-selection.g4
 *         -> target-selection policy
 *
 *     grammar/compile/lowering.g4
 *         -> lowering intent
 *
 * No existing major filename needs to be renamed.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal capacity constants.
 *
 * In particular, it contains no language-level maximum for:
 *
 *     quantum resources;
 *     CPU resources;
 *     GPU resources;
 *     FPGA resources;
 *     ASIC resources;
 *     nodes;
 *     memory;
 *     threads;
 *     tensor rank;
 *     tensor dimensions;
 *     network size;
 *     device count;
 *     pass count;
 *     pipeline length;
 *     objective count.
 *
 * Numeric values appearing inside expressions remain program or policy data.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     +
 *     *
 *     recursion
 *
 * for arbitrarily extensible:
 *
 *     clauses;
 *     objectives;
 *     arguments;
 *     pipeline items;
 *     nested pipelines;
 *     properties;
 *     fallback arms;
 *     qualified names.
 *
 * No artificial grammar ceiling is established.
 *
 * "Infinity" therefore means:
 *
 *     no language-level artificial maximum.
 *
 * It does not imply infinite physical memory, compiler time, or hardware.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing optimization intent MUST NOT:
 *
 *     execute an optimizer;
 *     execute host commands;
 *     load arbitrary native code;
 *     access credentials;
 *     access files;
 *     access networks;
 *     inspect hardware;
 *     allocate resources;
 *     modify runtime state.
 *
 * External optimizer integrations must be explicit compiler-controlled
 * interfaces after semantic validation.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Every implementation must provide:
 *
 * POSITIVE:
 *
 *     compile optimize;
 *     compile optimize minimize execution::cost;
 *     compile optimize maximize performance;
 *     compile optimize objective minimize execution::cost;
 *     compile optimize requires quantum::measurement;
 *     compile optimize constraint::semantic_preservation == true;
 *     compile optimize prefer tensor::locality;
 *     compile optimize hint optimization::vectorize;
 *     compile optimize pass classical::vectorize;
 *     compile optimize pass quantum::cancel_adjacent;
 *     compile optimize pipeline quantum {
 *         quantum::cancel_adjacent;
 *         quantum::merge_rotations;
 *     };
 *     compile optimize budget compilation_time <= compile_budget;
 *     compile optimize terminate optimization::stable;
 *     compile optimize verify semantic_equivalence;
 *     compile optimize preserve measurement_semantics;
 *     compile optimize approximate error <= tolerance;
 *     compile optimize stochastic optimization::search;
 *     compile optimize reproducible deterministic;
 *     compile optimize scope module::function;
 *     compile optimize fallback quantum::preferred classical::portable;
 *     compile optimize provenance compilation::optimization;
 *     compile optimize cost quantum::logical_error;
 *     compile optimize analyze quantum::depth;
 *     compile optimize property {
 *         optimization::policy = policy::portable;
 *     };
 *
 *
 * NEGATIVE:
 *
 *     compile optimize minimize;
 *     compile optimize maximize;
 *     compile optimize pass;
 *     compile optimize pipeline;
 *     compile optimize budget;
 *     compile optimize terminate;
 *     compile optimize verify;
 *     compile optimize preserve;
 *     compile optimize approximate;
 *     compile optimize pass quantum::;
 *     compile optimize pipeline quantum {};
 *     compile optimize property { invalid = };
 *
 *
 * BOUNDARY:
 *
 *     symbolic resource expressions;
 *     symbolic capability expressions;
 *     long qualified names;
 *     nested pipelines;
 *     nested property blocks;
 *     multiple objectives;
 *     multiple fallbacks;
 *     quantum + classical optimization;
 *     AI + quantum optimization;
 *     HDL + hardware optimization;
 *     distributed + networking optimization;
 *
 *
 * SCALABILITY:
 *
 *     arbitrarily many objectives;
 *     arbitrarily many passes;
 *     arbitrarily long pipelines;
 *     arbitrarily many requirements;
 *     arbitrarily many constraints;
 *     arbitrarily many properties;
 *     arbitrarily many fallback arms;
 *     deeply nested symbolic domains.
 *
 * The tests must scale with available test resources and MUST NOT establish
 * those sizes as language constants.
 *
 *
 * DETERMINISM:
 *
 *     identical source;
 *     identical lexer version;
 *     identical grammar version;
 *
 * must produce equivalent parse structure.
 *
 *
 * COMPATIBILITY:
 *
 *     old valid optimization syntax must either remain valid or receive an
 *     explicit migration path.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] There is one public optimization entry point.
 *     [x] Optimization has an unambiguous lexical boundary.
 *     [x] Generic requirements are reused.
 *     [x] Generic constraints are reused.
 *     [x] Generic hints are reused.
 *     [x] Expressions are reused.
 *     [x] Qualified names are reused.
 *     [x] Objectives are extensible.
 *     [x] Passes are symbolic.
 *     [x] Pipelines are recursively composable.
 *     [x] Budgets are semantic policy values.
 *     [x] Verification is explicit.
 *     [x] Preservation is explicit.
 *     [x] Approximation is explicit.
 *     [x] Stochastic behavior is explicit.
 *     [x] Reproducibility is explicit.
 *     [x] Fallback is explicit.
 *     [x] Provenance is representable.
 *     [x] Cost models are symbolic.
 *     [x] Analysis requests are symbolic.
 *     [x] Properties are extensible.
 *     [x] No optimizer implementation is embedded.
 *     [x] No IR is introduced.
 *     [x] quantum::ir remains canonical.
 *     [x] Target selection is not duplicated.
 *     [x] Resource discovery is not duplicated.
 *     [x] Routing is not duplicated.
 *     [x] Scheduling is not duplicated.
 *     [x] QEC is not duplicated.
 *     [x] ZQN is not duplicated.
 *     [x] HAL is not duplicated.
 *     [x] No hardware capacity is hard-coded.
 *     [x] No vendor catalogue is hard-coded.
 *     [x] No finite optimizer catalogue is hard-coded.
 *     [x] No unsafe Rust is required.
 *     [x] Rust 1.97+ compatibility is documented.
 *     [x] Positive tests are specified.
 *     [x] Negative tests are specified.
 *     [x] Boundary tests are specified.
 *     [x] Scalability tests are specified.
 *     [x] Determinism tests are specified.
 *     [x] Compatibility tests are specified.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What optimization intent does the source express?"
 *
 * It does NOT answer:
 *
 *     "Which optimizer implementation runs?"
 *     "Which CPU executes it?"
 *     "Which GPU executes it?"
 *     "Which QPU executes it?"
 *     "Which FPGA resources are selected?"
 *     "Which physical qubits are used?"
 *     "How is routing performed?"
 *     "How is scheduling performed?"
 *     "How is QEC performed?"
 *     "Which HAL implementation is used?"
 *
 * Those decisions remain downstream.
 *
 * The result is a target-independent optimization language boundary that can
 * serve tiny systems, classical systems, quantum systems, HDL flows,
 * accelerators, distributed systems, AI/data workloads, and future
 * computational architectures without introducing language-level capacity
 * ceilings.
 *
 * ============================================================================
 */