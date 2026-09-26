/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/optimization.g4
 *
 * Grammar:
 *     AIOptimization
 *
 * Status:
 *     CANONICAL AI OPTIMIZATION SOURCE-SYNTAX COMPONENT
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL OPTIMIZATION INTENT for Zamani AI/ML and
 * general computational optimization.
 *
 * It describes optimization intent such as:
 *
 *     - optimization declarations;
 *     - optimization invocations;
 *     - objectives;
 *     - objective direction;
 *     - optimization variables;
 *     - parameter spaces;
 *     - search spaces;
 *     - constraints;
 *     - strategies;
 *     - passes;
 *     - pass composition;
 *     - schedules;
 *     - stopping conditions;
 *     - validation;
 *     - checkpoints;
 *     - reproducibility;
 *     - numerical/stability requirements;
 *     - resource requirements;
 *     - capability requirements;
 *     - preferences;
 *     - hints;
 *     - distributed optimization intent;
 *     - accelerator intent;
 *     - quantum/hybrid optimization intent;
 *     - hardware/software co-design optimization intent;
 *     - extensible optimization directives.
 *
 * This file describes WHAT optimization means at source level.
 *
 * It does NOT implement:
 *
 *     - an optimizer algorithm;
 *     - gradient descent;
 *     - Adam;
 *     - SGD;
 *     - Newton methods;
 *     - evolutionary algorithms;
 *     - Bayesian optimization;
 *     - simulated annealing;
 *     - automatic differentiation;
 *     - tensor execution;
 *     - model execution;
 *     - scheduling;
 *     - resource allocation;
 *     - accelerator selection;
 *     - hardware discovery;
 *     - device placement;
 *     - routing;
 *     - calibration;
 *     - quantum error correction;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - target-specific optimization.
 *
 * Algorithm names and implementation strategies are semantic data, not a
 * closed parser-level enumeration.
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
 *     AIOptimization
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> numerical/stability analysis
 *          +--> portability analysis
 *          +--> determinism analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical lowering
 *          +--> quantum lowering
 *          +--> AI lowering
 *          +--> distributed lowering
 *          +--> accelerator lowering
 *          +--> HDL/hardware lowering
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization/lowering pipeline
 *          |
 *          +--> scheduling
 *          +--> routing
 *          +--> resilience
 *          +--> QEC where applicable
 *          +--> ZQN where applicable
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime / deployment
 *
 * This grammar MUST NOT construct or define IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - optimizationConstruct;
 *     - optimizationDeclaration;
 *     - optimizationInvocation;
 *     - optimizationBody;
 *     - optimizationMember;
 *     - optimizationDirective;
 *     - objective direction syntax;
 *     - optimization-local contracts;
 *     - optimization-local nested regions;
 *     - optimization-local extensibility boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - paths;
 *     - literals;
 *     - general expressions;
 *     - general types;
 *     - ordinary statements;
 *     - model declarations;
 *     - dataset declarations;
 *     - tensor declarations;
 *     - training declarations;
 *     - inference declarations;
 *     - automatic differentiation;
 *     - optimizer implementations;
 *     - numerical kernels;
 *     - hardware descriptions;
 *     - resource discovery;
 *     - placement;
 *     - scheduling implementation;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - canonical IR.
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * General types are owned by:
 *
 *     grammar/types/types.g4
 *
 * General statements are owned by:
 *
 *     grammar/statements/statements.g4
 *
 * This grammar MUST reuse those public rules.
 *
 * It MUST NOT define another:
 *
 *     expression
 *     typeExpression
 *     statement
 *     identifier
 *     argumentList
 *     operator hierarchy
 *     literal system
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally does NOT import:
 *
 *     models.g4
 *     datasets.g4
 *     tensors.g4
 *     training.g4
 *     inference.g4
 *
 * Optimization may semantically reference models, datasets, tensors,
 * inference results, training results, classical values, quantum values,
 * hardware-backed values, and future computational objects through ordinary
 * expressions.
 *
 * This prevents dependency cycles between AI leaf grammars.
 *
 * Example:
 *
 *     @optimize {
 *         @objective minimize loss(model, data);
 *     }
 *
 * `model` and `data` are expressions/references. Their declarations and
 * meanings are resolved by semantic analysis.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * AI optimization vocabulary MUST NOT require a growing lexer keyword list.
 *
 * The canonical lexer provides:
 *
 *     AT
 *
 * and ordinary identifiers.
 *
 * Optimization roles such as:
 *
 *     objective
 *     minimize
 *     maximize
 *     variable
 *     strategy
 *     pass
 *     schedule
 *     stopping
 *     validation
 *     checkpoint
 *     reproducible
 *     distributed
 *     accelerator
 *     quantum
 *
 * are therefore represented as semantic directive names unless the central
 * language specification later promotes a word to a true language keyword.
 *
 * Resource contracts reuse canonical language tokens where already defined:
 *
 *     REQUIRES
 *     CAPABILITY
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *
 * No optimization-specific lexer token is introduced here.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal limit on:
 *
 *     - number of optimization variables;
 *     - number of objectives;
 *     - number of constraints;
 *     - number of optimization passes;
 *     - number of strategies;
 *     - number of stages;
 *     - number of search dimensions;
 *     - number of parameters;
 *     - number of candidates;
 *     - number of iterations;
 *     - number of workers;
 *     - number of processes;
 *     - number of nodes;
 *     - number of accelerators;
 *     - number of devices;
 *     - tensor dimensions;
 *     - tensor rank;
 *     - quantum resources;
 *     - classical resources.
 *
 * There are deliberately no grammar-level constants such as:
 *
 *     MAX_OBJECTIVES
 *     MAX_VARIABLES
 *     MAX_CONSTRAINTS
 *     MAX_PASSES
 *     MAX_ITERATIONS
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QPUS
 *     MAX_TENSOR_RANK
 *
 * or equivalent artificial ceilings.
 *
 * Repetition is structural.
 *
 * Actual limits belong to:
 *
 *     - parser implementation policy;
 *     - compiler resource policy;
 *     - target capabilities;
 *     - runtime resources;
 *     - operating environment;
 *     - explicitly declared semantic constraints.
 *
 * Such implementation safeguards MUST NOT become language semantics.
 *
 * ============================================================================
 * POCO-REAF RESOURCE MODEL
 * ============================================================================
 *
 * Optimization source distinguishes:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     realization
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *
 *     capability("tensor.compute");
 *
 *     constraint latency <= budget;
 *
 *     prefer capability("gpu.compute");
 *
 *     hint locality;
 *
 * None of these selects a physical device.
 *
 * The grammar MUST NOT encode:
 *
 *     GPU 0
 *     CPU 7
 *     QPU 2
 *     physical qubit 17
 *     FPGA region 3
 *
 * as universal optimization semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The same optimization source may ultimately be lowered to:
 *
 *     - scalar CPU computation;
 *     - multicore CPU;
 *     - SIMD/vector hardware;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - NPU;
 *     - TPU;
 *     - QPU;
 *     - distributed clusters;
 *     - heterogeneous systems;
 *     - embedded targets;
 *     - cloud targets;
 *     - future computational targets.
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Optimization expressions may reference:
 *
 *     scalar values;
 *     vectors;
 *     matrices;
 *     tensors;
 *     symbolic expressions;
 *     numerical functions;
 *     objective functions;
 *     constraints;
 *     statistics;
 *     scientific-computing values.
 *
 * Their semantic types are resolved by the canonical type system.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Optimization may optimize computations involving quantum values or quantum
 * execution through ordinary expressions and semantic directives.
 *
 * This grammar does NOT define:
 *
 *     - qubits;
 *     - gates;
 *     - physical qubits;
 *     - topology;
 *     - routing;
 *     - pulse scheduling;
 *     - QEC;
 *     - ZQN;
 *     - quantum hardware.
 *
 * The downstream path remains:
 *
 *     optimization source
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / decomposition / routing / scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *
 * There MUST NOT be an AI-specific second quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Optimization may describe hardware-aware intent through:
 *
 *     requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     hints.
 *
 * It MUST NOT hard-code:
 *
 *     - register width;
 *     - bus width;
 *     - memory-bank count;
 *     - FPGA resource count;
 *     - accelerator count;
 *     - physical clock topology;
 *     - physical addresses;
 *     - vendor-specific devices.
 *
 * Hardware realization belongs downstream to:
 *
 *     hardware/
 *     hdl/
 *     resources/
 *     compile/
 *     execution/
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every optimization construct MUST preserve:
 *
 *     - source span;
 *     - declaration/invocation form;
 *     - annotation name;
 *     - optimization name where present;
 *     - optional declared type;
 *     - initializer;
 *     - ordered members;
 *     - directive names;
 *     - directive arguments;
 *     - directive expressions;
 *     - nested regions;
 *     - objective direction;
 *     - resource/capability contracts;
 *     - source provenance.
 *
 * The frontend AST MUST remain domain-neutral.
 *
 * The parser must not construct optimizer-specific implementation objects.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - validating the canonical optimization annotation;
 *     - resolving optimization names;
 *     - resolving referenced models/datasets/tensors/values;
 *     - validating objective expressions;
 *     - validating objective direction;
 *     - validating variable domains;
 *     - validating search-space expressions;
 *     - validating constraints;
 *     - validating strategy names;
 *     - validating pass applicability;
 *     - validating schedule semantics;
 *     - validating stopping criteria;
 *     - validating numerical requirements;
 *     - validating determinism/reproducibility requirements;
 *     - validating distributed intent;
 *     - validating accelerator intent;
 *     - validating quantum/hybrid intent;
 *     - validating resource requirements;
 *     - validating capability requirements;
 *     - distinguishing preferences from requirements;
 *     - checking portability;
 *     - checking type/effect/resource compatibility.
 *
 * The parser performs none of those semantic checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Optimization intent is lowered through the canonical semantic model into
 * the repository's canonical IR architecture.
 *
 * There is no:
 *
 *     OptimizationIR
 *     AIOptimizationIR
 *     QuantumOptimizationIR
 *
 * created by this grammar.
 *
 * If optimization affects quantum computation, the semantic pipeline reaches:
 *
 *     quantum::ir
 *
 * through the existing canonical quantum boundary.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a fixed:
 *
 *     source
 *     lexer
 *     grammar
 *     parser configuration.
 *
 * Runtime optimization may be:
 *
 *     deterministic;
 *     stochastic;
 *     distributed;
 *     adaptive;
 *
 * without changing parser determinism.
 *
 * Reproducibility requirements are source-level semantic information and are
 * not parser execution behavior.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no target-specific actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no unsafe implementation.
 *
 * Generated/consuming Rust code MUST remain compatible with:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] public optimization construct exists;
 *     [x] declaration and invocation boundaries are distinct;
 *     [x] general expressions are reused;
 *     [x] general types are reused;
 *     [x] general statements are reused;
 *     [x] resource semantics are separated from realization;
 *     [x] capability semantics are separated from realization;
 *     [x] requirements/preferences/hints remain distinguishable;
 *     [x] objective direction is structurally represented;
 *     [x] optimization variables are extensible;
 *     [x] constraints are extensible;
 *     [x] strategies are extensible;
 *     [x] passes are extensible;
 *     [x] scheduling intent is extensible;
 *     [x] distributed intent is extensible;
 *     [x] accelerator intent is extensible;
 *     [x] quantum/hybrid intent remains downstream;
 *     [x] no optimizer implementation is encoded;
 *     [x] no fixed algorithm enumeration exists;
 *     [x] no artificial hardware limits exist;
 *     [x] no target-specific resource selection exists;
 *     [x] no second IR exists;
 *     [x] no second type system exists;
 *     [x] no second expression grammar exists;
 *     [x] no semantic predicates are required;
 *     [x] no unsafe Rust is required;
 *     [x] positive/negative/boundary/scalability tests are defined downstream;
 *     [x] AST contract is predetermined;
 *     [x] semantic contract is predetermined;
 *     [x] IR integration is predetermined.
 *
 * ============================================================================
 */

parser grammar AIOptimization;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical AI optimization construct.
 *
 * Declaration:
 *
 *     @optimization optimize_model {
 *         ...
 *     }
 *
 * Invocation:
 *
 *     @optimize(model, objective);
 *
 * The annotation spelling is validated semantically.
 *
 * This avoids introducing a new lexer keyword merely for optimization.
 */
optimizationConstruct
    : optimizationDeclaration
    | optimizationInvocation
    ;


/* ============================================================================
 * 2. OPTIMIZATION DECLARATION
 * ========================================================================== */

/**
 * Named optimization region.
 *
 * Examples:
 *
 *     @optimization training_tuning {
 *         ...
 *     }
 *
 *     @optimization training_tuning: Result {
 *         ...
 *     }
 *
 *     @optimization training_tuning = search_space {
 *         ...
 *     }
 */
optimizationDeclaration
    : AT
      optimizationDeclarationAnnotation
      identifier
      optimizationTypeClause?
      optimizationInitializer?
      optimizationBody
    ;


optimizationDeclarationAnnotation
    : identifier
    ;


optimizationTypeClause
    : COLON
      typeExpression
    ;


optimizationInitializer
    : ASSIGN
      expression
    ;


optimizationBody
    : LBRACE
      optimizationMember*
      RBRACE
    ;


/* ============================================================================
 * 3. OPTIMIZATION INVOCATION
 * ========================================================================== */

/**
 * Optimization invocation.
 *
 * Examples:
 *
 *     @optimize(model, objective);
 *
 *     @optimize(search_space);
 *
 *     @optimize(model, parameters, constraints);
 *
 * The invocation arguments remain ordinary Zamani expressions.
 */
optimizationInvocation
    : AT
      optimizationInvocationAnnotation
      LPAREN
      argumentList?
      RPAREN
      SEMI?
    ;


optimizationInvocationAnnotation
    : identifier
    ;


/* ============================================================================
 * 4. OPTIMIZATION MEMBERS
 * ========================================================================== */

/**
 * Optimization bodies contain:
 *
 *     - extensible directives;
 *     - explicit resource contracts;
 *     - explicit capability contracts;
 *     - explicit constraints;
 *     - preferences;
 *     - hints;
 *     - objective-direction anchors;
 *     - ordinary Zamani statements.
 *
 * The generic directive is intentionally open-ended.
 *
 * Semantic analysis owns the directive registry.
 */
optimizationMember
    : optimizationDirective
    | optimizationRequirement
    | optimizationCapability
    | optimizationConstraint
    | optimizationPreference
    | optimizationHint
    | optimizationObjectiveDirection
    | statement
    ;


/* ============================================================================
 * 5. GENERIC OPTIMIZATION DIRECTIVE
 * ========================================================================== */

/**
 * Extensible optimization-local directive.
 *
 * Examples:
 *
 *     @objective minimize loss;
 *
 *     @objective(maximize, accuracy);
 *
 *     @variable learning_rate: Float = initial_rate;
 *
 *     @strategy search;
 *
 *     @pass simplify(model);
 *
 *     @schedule adaptive_schedule;
 *
 *     @stopping convergence;
 *
 *     @checkpoint checkpoint_policy;
 *
 *     @validation validation_policy;
 *
 *     @reproducible seed;
 *
 *     @distributed cluster_policy;
 *
 *     @accelerator capability("tensor.compute");
 *
 *     @quantum quantum_objective;
 *
 *     @hardware co_design_policy;
 *
 * Directive names are semantic identifiers rather than a fixed keyword list.
 */
optimizationDirective
    : AT
      optimizationDirectiveName
      optimizationDirectivePayload?
      SEMI?
    ;


optimizationDirectiveName
    : identifier
    ;


optimizationDirectivePayload
    : optimizationCallPayload
    | optimizationBindingPayload
    | optimizationTypedBindingPayload
    | optimizationAssignmentPayload
    | optimizationExpressionPayload
    | optimizationRegionPayload
    ;


/**
 * Function-like directive payload.
 *
 * Example:
 *
 *     @strategy(search, objective);
 */
optimizationCallPayload
    : LPAREN
      argumentList?
      RPAREN
    ;


/**
 * Named directive payload.
 *
 * Example:
 *
 *     @strategy search;
 */
optimizationBindingPayload
    : identifier
      optimizationCallPayload?
    ;


/**
 * Typed named directive payload.
 *
 * Example:
 *
 *     @variable learning_rate: Float;
 */
optimizationTypedBindingPayload
    : identifier
      COLON
      typeExpression
      optimizationCallPayload?
    ;


/**
 * Assignment payload.
 *
 * Example:
 *
 *     @variable learning_rate = initial_rate;
 */
optimizationAssignmentPayload
    : identifier?
      ASSIGN
      expression
    ;


/**
 * General expression payload.
 *
 * This allows future optimization constructs to reuse the ordinary
 * expression language without modifying this grammar.
 */
optimizationExpressionPayload
    : expression
    ;


/**
 * Nested optimization region.
 *
 * Example:
 *
 *     @phase {
 *         @objective minimize loss;
 *         @pass simplify;
 *     }
 */
optimizationRegionPayload
    : LBRACE
      optimizationMember*
      RBRACE
    ;


/* ============================================================================
 * 6. RESOURCE REQUIREMENTS
 * ========================================================================== */

/**
 * Hard resource requirement.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *
 *     requires compute >= required_compute;
 *
 *     requires throughput >= required_throughput;
 *
 *     requires qubits >= required_qubits;
 *
 * The grammar does not interpret the resource expression.
 *
 * No physical device is selected.
 */
optimizationRequirement
    : REQUIRES
      expression
      SEMI?
    ;


/* ============================================================================
 * 7. CAPABILITY REQUIREMENTS
 * ========================================================================== */

/**
 * Capability requirement.
 *
 * Examples:
 *
 *     capability("tensor.compute");
 *
 *     capability("gpu.compute");
 *
 *     capability("quantum.measurement");
 *
 *     capability("distributed.collective");
 *
 * Capability satisfaction is downstream.
 */
optimizationCapability
    : CAPABILITY
      expression
      SEMI?
    ;


/* ============================================================================
 * 8. CONSTRAINTS
 * ========================================================================== */

/**
 * Hard optimization constraint.
 *
 * Examples:
 *
 *     constraint latency <= budget;
 *
 *     constraint error <= tolerance;
 *
 *     constraint memory <= available_memory;
 *
 * The constraint remains semantic data until validation/lowering.
 */
optimizationConstraint
    : CONSTRAINT
      expression
      SEMI?
    ;


/* ============================================================================
 * 9. PREFERENCES
 * ========================================================================== */

/**
 * Non-binding optimization preference.
 *
 * Example:
 *
 *     prefer capability("gpu.compute");
 *
 * A preference MUST NOT silently become a requirement.
 */
optimizationPreference
    : PREFER
      expression
      SEMI?
    ;


/* ============================================================================
 * 10. HINTS
 * ========================================================================== */

/**
 * Non-semantic implementation hint.
 *
 * Example:
 *
 *     hint locality;
 *
 * Hints may influence optimization but MUST NOT redefine program meaning.
 */
optimizationHint
    : HINT
      expression
      SEMI?
    ;


/* ============================================================================
 * 11. OBJECTIVE DIRECTION
 * ========================================================================== */

/**
 * Explicit objective direction.
 *
 * This provides a structural parser anchor for objective direction while
 * keeping the objective expression generic.
 *
 * Examples:
 *
 *     minimize loss;
 *
 *     maximize accuracy;
 *
 *     minimize cost;
 *
 * The words `minimize` and `maximize` are intentionally identifiers here.
 * Semantic analysis determines whether they are valid objective-direction
 * names in this context.
 *
 * This avoids requiring dedicated lexer keywords for every optimization
 * vocabulary term.
 */
optimizationObjectiveDirection
    : optimizationObjectiveDirectionName
      expression
      SEMI?
    ;


optimizationObjectiveDirectionName
    : identifier
    ;


/* ============================================================================
 * 12. CANONICAL OPTIMIZATION SEMANTIC ANCHORS
 * ========================================================================== */

/*
 * These rules are intentionally defined as reusable parser-level anchors.
 *
 * They are not independently dispatched by optimizationMember because all
 * begin with an open-ended annotation name and would therefore duplicate the
 * generic optimizationDirective boundary.
 *
 * Their semantic role is documented here so tooling/AST contracts can map
 * normalized directive names without creating a second grammar authority.
 *
 * Canonical semantic names include:
 *
 *     objective
 *     variable
 *     parameter
 *     search
 *     space
 *     strategy
 *     pass
 *     stage
 *     phase
 *     schedule
 *     stopping
 *     validation
 *     checkpoint
 *     reproducibility
 *     distributed
 *     accelerator
 *     quantum
 *     hybrid
 *     hardware
 *
 * The actual accepted spelling is resolved by semantic analysis.
 */


/* ============================================================================
 * 13. OBJECTIVE CONTRACT
 * ========================================================================== */

/**
 * Generic objective payload.
 *
 * Examples:
 *
 *     objective(loss)
 *     objective(minimize, loss)
 *     objective(maximize, accuracy)
 *     objective(cost + penalty)
 */
optimizationObjective
    : optimizationDirectiveCall
    | optimizationExpressionPayload
    ;


/* ============================================================================
 * 14. VARIABLE / PARAMETER CONTRACT
 * ========================================================================== */

/**
 * Optimization variables remain ordinary source identifiers associated with
 * ordinary Zamani expressions/types.
 *
 * Examples:
 *
 *     learning_rate
 *     batch_size
 *     depth
 *     tolerance
 *     architecture
 *
 * Their legal domain is determined semantically.
 */
optimizationVariable
    : identifier
    ;


optimizationVariableBinding
    : identifier
      COLON
      typeExpression
      (ASSIGN expression)?
    ;


/* ============================================================================
 * 15. SEARCH SPACE CONTRACT
 * ========================================================================== */

/**
 * Search-space syntax is expression-based.
 *
 * Examples:
 *
 *     range(learning_rate)
 *     domain(parameters)
 *     choices(architecture)
 *     expression_based_space
 *
 * No finite grammar-level number of dimensions is imposed.
 */
optimizationSearchSpace
    : expression
    ;


/* ============================================================================
 * 16. STRATEGY CONTRACT
 * ========================================================================== */

/**
 * Strategy syntax is deliberately opaque to the parser.
 *
 * Examples may include:
 *
 *     gradient
 *     stochastic
 *     evolutionary
 *     bayesian
 *     symbolic
 *     hybrid
 *     custom_strategy
 *
 * The parser does not enumerate these algorithms.
 */
optimizationStrategy
    : expression
    ;


/* ============================================================================
 * 17. PASS CONTRACT
 * ========================================================================== */

/**
 * Optimization passes are generic expressions/directives.
 *
 * Examples:
 *
 *     simplify
 *     fuse
 *     tile
 *     vectorize
 *     quantize
 *     prune
 *     custom_pass
 *
 * The grammar does not turn these into a closed algorithm list.
 */
optimizationPass
    : expression
    ;


/* ============================================================================
 * 18. PASS SEQUENCE
 * ========================================================================== */

/**
 * An optimization pass sequence is represented structurally by a sequence
 * of directives or nested optimization regions.
 *
 * No fixed number of passes is permitted.
 */
optimizationPassSequence
    : optimizationBody
    ;


/* ============================================================================
 * 19. SCHEDULE CONTRACT
 * ========================================================================== */

/**
 * Scheduling intent remains expression-based.
 *
 * Actual scheduling is owned downstream.
 */
optimizationSchedule
    : expression
    ;


/* ============================================================================
 * 20. STOPPING CONTRACT
 * ========================================================================== */

/**
 * Stopping conditions are semantic expressions.
 *
 * Examples:
 *
 *     convergence < tolerance
 *     budget <= limit
 *     condition(...)
 */
optimizationStoppingCondition
    : expression
    ;


/* ============================================================================
 * 21. VALIDATION CONTRACT
 * ========================================================================== */

/**
 * Validation policy remains semantic data.
 */
optimizationValidation
    : expression
    ;


/* ============================================================================
 * 22. CHECKPOINT CONTRACT
 * ========================================================================== */

/**
 * Checkpointing policy remains semantic data.
 */
optimizationCheckpoint
    : expression
    ;


/* ============================================================================
 * 23. REPRODUCIBILITY CONTRACT
 * ========================================================================== */

/**
 * Reproducibility requirements are semantic data.
 *
 * Examples:
 *
 *     deterministic
 *     seed
 *     provenance
 *     reproducible(...)
 */
optimizationReproducibility
    : expression
    ;


/* ============================================================================
 * 24. DISTRIBUTED OPTIMIZATION CONTRACT
 * ========================================================================== */

/**
 * Distributed optimization intent.
 *
 * Examples:
 *
 *     distributed
 *     collective(...)
 *     partition(...)
 *     replica(...)
 *
 * The grammar does not encode a fixed node/worker count.
 */
optimizationDistributedIntent
    : expression
    ;


/* ============================================================================
 * 25. ACCELERATOR CONTRACT
 * ========================================================================== */

/**
 * Accelerator intent.
 *
 * Examples:
 *
 *     accelerator
 *     capability("tensor.compute")
 *     capability("gpu.compute")
 *
 * No physical accelerator is selected.
 */
optimizationAcceleratorIntent
    : expression
    ;


/* ============================================================================
 * 26. HYBRID CONTRACT
 * ========================================================================== */

/**
 * Hybrid classical/quantum optimization intent.
 *
 * Examples:
 *
 *     hybrid
 *     quantum_objective
 *     classical_feedback
 *
 * Actual quantum semantics remain owned by the quantum subsystem.
 */
optimizationHybridIntent
    : expression
    ;


/* ============================================================================
 * 27. QUANTUM CONTRACT
 * ========================================================================== */

/**
 * Quantum optimization intent.
 *
 * This is deliberately expression-based.
 *
 * The grammar does not define quantum operations.
 */
optimizationQuantumIntent
    : expression
    ;


/* ============================================================================
 * 28. HARDWARE / CO-DESIGN CONTRACT
 * ========================================================================== */

/**
 * Hardware/software co-design intent.
 *
 * Examples:
 *
 *     latency
 *     throughput
 *     energy
 *     area
 *     thermal
 *     power
 *     capability(...)
 *
 * Physical realization remains downstream.
 */
optimizationHardwareIntent
    : expression
    ;


/* ============================================================================
 * 29. NUMERICAL / STABILITY CONTRACT
 * ========================================================================== */

/**
 * Numerical requirements are semantic expressions.
 *
 * Examples:
 *
 *     tolerance
 *     precision
 *     stability
 *     conditioning
 *     error_budget
 *
 * The grammar does not prescribe a fixed numerical representation.
 */
optimizationNumericalIntent
    : expression
    ;


/* ============================================================================
 * 30. OBJECTIVE LIST
 * ========================================================================== */

/**
 * A declaration may contain arbitrarily many objectives through repeated
 * optimization members.
 *
 * This rule is provided as a structural helper for AST/tooling contracts.
 */
optimizationObjectiveList
    : optimizationObjectiveItem*
    ;


optimizationObjectiveItem
    : optimizationObjectiveDirection
    | optimizationDirective
    ;


/* ============================================================================
 * 31. CONSTRAINT LIST
 * ========================================================================== */

/**
 * Constraints are structurally unbounded.
 */
optimizationConstraintList
    : optimizationConstraintItem*
    ;


optimizationConstraintItem
    : optimizationConstraint
    | optimizationDirective
    ;


/* ============================================================================
 * 32. EXTENSION REGION
 * ========================================================================== */

/**
 * Nested optimization regions permit future domains without modifying the
 * core optimization grammar.
 *
 * A semantic registry determines which directive names are legal and what
 * AST/semantic contract each one has.
 */
optimizationExtensionRegion
    : LBRACE
      optimizationMember*
      RBRACE
    ;


/* ============================================================================
 * 33. CROSS-DOMAIN STATEMENT INTEGRATION
 * ========================================================================== */

/**
 * Ordinary Zamani statements remain legal inside optimization regions.
 *
 * This is intentional.
 *
 * An optimization region may therefore contain:
 *
 *     classical computation;
 *     tensor operations;
 *     quantum interactions;
 *     measurement-driven decisions;
 *     data transformations;
 *     distributed operations;
 *     resource-aware statements;
 *     hardware/HDL intent exposed by the universal statement layer.
 *
 * The statement grammar remains the sole owner of statement syntax.
 */
optimizationStatement
    : statement
    ;


/* ============================================================================
 * 34. CROSS-DOMAIN EXPRESSION INTEGRATION
 * ========================================================================== */

/**
 * Optimization values are ordinary Zamani expressions.
 */
optimizationExpression
    : expression
    ;


/* ============================================================================
 * 35. CROSS-DOMAIN TYPE INTEGRATION
 * ========================================================================== */

/**
 * Optimization variable/result types are ordinary Zamani types.
 */
optimizationType
    : typeExpression
    ;


/* ============================================================================
 * 36. PORTABILITY CONTRACT
 * ========================================================================== */

/**
 * This rule intentionally accepts a semantic expression rather than a target
 * identifier.
 *
 * For example:
 *
 *     requires capability("gpu.compute")
 *
 * is portable intent.
 *
 * A physical mapping such as:
 *
 *     GPU 0
 *
 * is NOT part of the portable optimization contract and belongs downstream.
 */
optimizationPortabilityRequirement
    : expression
    ;


/* ============================================================================
 * 37. COMPLETENESS ANCHOR
 * ========================================================================== */

/**
 * This rule provides a single explicit parser-level representation for the
 * complete optimization domain.
 *
 * It is intentionally equivalent to optimizationConstruct so tooling can use
 * a stable semantic name without introducing another parser entry point.
 */
optimizationDomain
    : optimizationConstruct
    ;