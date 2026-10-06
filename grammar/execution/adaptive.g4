/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/execution/adaptive.g4
 *
 * Grammar:
 *     AdaptiveExpressions
 *
 * Status:
 *     CANONICAL EXPRESSION-LEVEL ADAPTATION GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021 edition
 *     Safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the UNIVERSAL EXPRESSION-LEVEL syntax for adaptive
 * computational intent.
 *
 * It provides the value-producing form:
 *
 *     adapt(target)
 *     adapt(target, context)
 *     adapt(target, context_a, context_b, ...)
 *
 * The result of an adaptive expression is an ordinary semantic value and can
 * therefore participate in the normal Zamani expression hierarchy.
 *
 * Examples:
 *
 *     adapt(strategy)
 *
 *     adapt(model, training_result)
 *
 *     adapt(plan, observation, policy)
 *
 *     adapt(strategy, evidence, provenance, constraints)
 *
 * The grammar intentionally treats all operands after the target as generic
 * expressions. Their semantic roles are determined downstream.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Expressions
 *       |
 *       +--> AdaptiveExpressions
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> contract analysis
 *       +--> policy analysis
 *       +--> provenance analysis
 *       |
 *       v
 *     semantic adaptation operation
 *       |
 *       +--> classical semantics
 *       +--> quantum semantics
 *       +--> HDL/hardware semantics
 *       +--> AI/ML semantics
 *       +--> distributed semantics
 *       +--> data semantics
 *       +--> future domain semantics
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> domain-specific semantic representations
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     specialization / lowering
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     resilience / recovery
 *       |
 *       v
 *     ZQN / HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar never selects the physical realization.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     adaptiveExpression
 *     adaptiveOperation
 *     adaptiveArgumentList
 *     adaptiveTarget
 *     adaptiveContext
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer rules
 *     keyword definitions
 *     punctuation definitions
 *     identifiers
 *     qualified names
 *     expression precedence
 *     assignments
 *     conditionals
 *     unary operators
 *     binary operators
 *     calls
 *     indexing
 *     member access
 *     literals
 *     statements
 *     statement-level adaptation
 *     learning
 *     reasoning
 *     knowledge
 *     uncertainty
 *     contracts
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     AI-specific semantics
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     execution scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     AST implementation
 *     semantic implementation
 *     IR implementation
 *     runtime implementation
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There are two source-level adaptation forms:
 *
 *     statement:
 *
 *         adapt target;
 *
 *     expression:
 *
 *         adapt(target)
 *
 * They are intentionally different syntactic forms.
 *
 * Their semantics MUST converge on the same canonical adaptation operation.
 *
 * The statement form is owned by:
 *
 *     grammar/statements/adapt.g4
 *
 * This file owns only the expression form.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     ZamaniLexer
 *     expressionCore
 *
 * `expressionCore` is supplied by the canonical expression composition.
 *
 * This grammar MUST NOT import the complete `Expressions` grammar because
 * `Expressions` imports this grammar.
 *
 * That would create a circular dependency.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical integration direction is:
 *
 *     AdaptiveExpressions
 *             |
 *             v
 *     Expressions
 *             |
 *             v
 *     ZamaniParser
 *
 * Therefore:
 *
 *     grammar/expressions/expressions.g4
 *
 * must import:
 *
 *     AdaptiveExpressions
 *
 * exactly once.
 *
 * The expression composition must expose `adaptiveExpression` through its
 * primary-expression/value-expression layer.
 *
 * No parent grammar should independently recreate:
 *
 *     adapt(...)
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer supplies:
 *
 *     ADAPT
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * This grammar creates NO lexer rules.
 *
 * `adapt` remains one canonical lexical token:
 *
 *     ADAPT : 'adapt' ;
 *
 * No additional universal keywords such as:
 *
 *     ADAPTIVE
 *     EVOLVE
 *     MODIFY
 *     CHANGE
 *     SELF_MODIFY
 *
 * are required.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * The expression form is:
 *
 *     ADAPT LPAREN adaptiveArgumentList RPAREN
 *
 * At least one argument is required.
 *
 * The first argument is the adaptation target.
 *
 * Every subsequent argument is an ordinary expression.
 *
 * The grammar deliberately does NOT assign fixed meanings to argument
 * positions after the target.
 *
 * Consequently:
 *
 *     adapt(target, feedback)
 *
 *     adapt(target, evidence)
 *
 *     adapt(target, policy)
 *
 *     adapt(target, observation, evidence)
 *
 *     adapt(target, learned_result, constraint, provenance)
 *
 * are all syntactically valid.
 *
 * Their semantic interpretation belongs downstream.
 *
 * ============================================================================
 * WHY GENERIC CONTEXT IS REQUIRED
 * ============================================================================
 *
 * Adaptation can occur in many domains:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI/ML
 *     data
 *     distributed
 *     networking
 *     simulation
 *     compilation
 *     execution
 *     future computational domains
 *
 * A universal grammar MUST NOT require a new keyword for every new adaptation
 * source.
 *
 * Therefore the grammar uses:
 *
 *     expressionCore
 *
 * as the universal context boundary.
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * This grammar MUST NOT enumerate adaptation algorithms or strategies.
 *
 * It must NOT contain grammar alternatives for:
 *
 *     gradient
 *     reinforcement
 *     Bayesian
 *     evolutionary
 *     genetic
 *     heuristic
 *     predictive
 *     transfer
 *     fine_tune
 *     reroute
 *     reschedule
 *     remap
 *     recompile
 *     specialize
 *     optimize
 *
 * Such concepts belong to:
 *
 *     libraries
 *     semantic capabilities
 *     dialects
 *     policies
 *     execution strategies
 *     optimization systems
 *     domain-specific implementations
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning remains owned by:
 *
 *     grammar/ai/
 *     grammar/statements/
 *
 * Adaptive expressions can consume learning results naturally:
 *
 *     adapt(model, learning_result)
 *
 * The adaptation grammar does not need to know that the second operand is a
 * learning result.
 *
 * Semantic analysis establishes that relationship.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Reasoning remains independently owned.
 *
 * Examples:
 *
 *     adapt(strategy, inferred_strategy)
 *
 *     adapt(plan, reason_result)
 *
 *     adapt(model, deduced_configuration)
 *
 * No reasoning syntax is duplicated here.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Knowledge/query results can become adaptation context:
 *
 *     adapt(strategy, query_result)
 *
 *     adapt(model, knowledge_result)
 *
 * Knowledge semantics remain outside this file.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Any uncertainty-bearing expression may be supplied:
 *
 *     adapt(strategy, probability)
 *     adapt(model, confidence)
 *     adapt(plan, distribution)
 *     adapt(state, belief)
 *
 * This grammar imposes no fixed precision, representation, or number of
 * possible outcomes.
 *
 * ============================================================================
 * EVIDENCE / PROVENANCE
 * ============================================================================
 *
 * Evidence and provenance may be passed as expressions:
 *
 *     adapt(strategy, evidence, provenance)
 *
 * The semantic system determines:
 *
 *     evidence validity
 *     origin
 *     derivation
 *     confidence
 *     verification
 *     authorization
 *     decision provenance
 *
 * This grammar does not create a second provenance language.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing an adaptive expression performs no effect.
 *
 * Semantic analysis MAY determine effects such as:
 *
 *     adaptation
 *     mutation
 *     learning
 *     randomness
 *     reflection
 *     code_generation
 *     simulation
 *     measurement
 *     network
 *     distributed
 *     native
 *     foreign
 *     runtime_control
 *
 * The final effect set depends on semantic resolution.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Semantic analysis MAY require arbitrary capabilities.
 *
 * Examples include:
 *
 *     capability("adaptation")
 *     capability("model.update")
 *     capability("strategy.update")
 *     capability("runtime.adaptation")
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *
 * Capability names remain open-world.
 *
 * No capability name is physically hard-coded into this grammar.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Adaptation may require arbitrary resources:
 *
 *     compute
 *     memory
 *     storage
 *     bandwidth
 *     network
 *     accelerator resources
 *     tensor resources
 *     quantum resources
 *     distributed resources
 *     simulation resources
 *     energy
 *     future resource classes
 *
 * Resource quantities are resolved downstream.
 *
 * This file contains no:
 *
 *     MAX_ADAPTATIONS
 *     MAX_CONTEXT
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Adaptive expressions participate in the universal contract system.
 *
 * Applicable semantic constructs include:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This grammar does not duplicate those constructs.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies remain owned by the policy subsystem.
 *
 * Adaptation may be constrained by:
 *
 *     security policies
 *     execution policies
 *     resource policies
 *     adaptation policies
 *     deployment policies
 *     data policies
 *     quantum policies
 *
 * `adapt(...)` does NOT grant authorization.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     does not execute adaptation;
 *     does not invoke models;
 *     does not access hardware;
 *     does not access files;
 *     does not access networks;
 *     does not access credentials;
 *     does not invoke a QPU;
 *     does not invoke a simulator;
 *     does not modify source code;
 *     does not modify generated code;
 *     does not bypass sandbox rules;
 *     does not bypass policy;
 *     does not bypass capability checks.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar itself creates no Rust AST implementation.
 *
 * The conceptual AST shape is:
 *
 *     AdaptiveExpression {
 *         target,
 *         context,
 *         source_span
 *     }
 *
 * Existing generic operation/expression nodes SHOULD be reused where
 * appropriate.
 *
 * The AST MUST remain domain-neutral.
 *
 * It MUST NOT contain:
 *
 *     physical CPU IDs
 *     physical GPU IDs
 *     physical FPGA IDs
 *     physical QPU IDs
 *     physical qubit mappings
 *     routing decisions
 *     scheduling decisions
 *     calibration
 *     vendor topology
 *     QEC layout
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     target validity
 *     target adaptability
 *     operand compatibility
 *     result type
 *     required effects
 *     required capabilities
 *     required resources
 *     applicable contracts
 *     applicable policies
 *     provenance requirements
 *     determinism requirements
 *     target feasibility
 *
 * Syntax validity is not semantic validity.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * No universal result type is encoded here.
 *
 * Semantic analysis determines whether the result is:
 *
 *     value
 *     model
 *     strategy
 *     configuration
 *     execution plan
 *     computation
 *     domain object
 *     other semantic value
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The required lowering path is:
 *
 *     adaptiveExpression
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic adaptation operation
 *         |
 *         +---------------------+
 *         |                     |
 *         v                     v
 *     classical semantics   quantum semantics
 *                                 |
 *                                 v
 *                             quantum::ir
 *         |
 *         v
 *     target-independent optimization
 *         |
 *         v
 *     lowering
 *         |
 *         v
 *     routing / scheduling
 *         |
 *         v
 *     resilience
 *         |
 *         v
 *     ZQN / HAL
 *
 * If adaptation affects quantum computation, the quantum portion MUST cross
 * the canonical `quantum::ir` boundary before quantum optimization,
 * decomposition, routing, scheduling, resilience or physical realization.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Valid semantic examples include:
 *
 *     adapt(quantum_strategy, measurement_result)
 *
 *     adapt(hybrid_plan, quantum_result)
 *
 *     adapt(classical_control, quantum_observation)
 *
 * This grammar does not enumerate quantum gates, physical qubits, coupling
 * maps, calibration data, routing or QEC.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Adaptive expressions may consume:
 *
 *     hardware observations
 *     simulation results
 *     verification results
 *     telemetry
 *     resource observations
 *
 * The grammar does not encode:
 *
 *     register width
 *     bus width
 *     fixed memory capacity
 *     fixed device count
 *     fixed topology
 *     physical placement
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Adaptation may consume distributed results:
 *
 *     adapt(strategy, distributed_result)
 *
 * Actor, channel, task and service syntax remains owned by the concurrency
 * and distributed subsystems.
 *
 * This grammar creates no second concurrency model.
 *
 * ============================================================================
 * SIMULATION CONTRACT
 * ============================================================================
 *
 * Simulation results may be supplied as ordinary expressions:
 *
 *     adapt(strategy, simulation_result)
 *
 * Simulation remains an execution strategy and does not become a second
 * programming language.
 *
 * ============================================================================
 * METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * `adapt(...)` does NOT automatically imply:
 *
 *     reflection
 *     self-modification
 *     source rewriting
 *     code generation
 *     recompilation
 *
 * If those semantics are selected, the semantic system must require the
 * corresponding effect, capability and policy.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Downstream provenance may record:
 *
 *     operation
 *     target
 *     context operands
 *     source location
 *     evidence
 *     reason
 *     decision
 *     policy
 *     authorization
 *     capability decision
 *     resource decision
 *     transformation
 *     resulting state
 *     verification
 *     language version
 *     compiler version
 *     target capability snapshot
 *
 * The grammar preserves the source structure needed to construct that record.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing statement-level syntax remains valid:
 *
 *     adapt strategy;
 *     adapt strategy from feedback;
 *     adapt strategy with (policy);
 *
 * Expression-level syntax is distinct:
 *
 *     adapt(strategy)
 *     adapt(strategy, feedback)
 *     adapt(strategy, feedback, policy)
 *
 * Both MUST lower to the same semantic adaptation abstraction.
 *
 * Existing qualified forms such as:
 *
 *     mind.adapt(...)
 *
 * remain valid where their owning grammar permits them.
 *
 * They are not replaced by this grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexer configuration
 *     grammar version
 *     parser configuration
 *     selected dialect configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     CPU availability
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     memory availability
 *     filesystem state
 *     network state
 *     scheduler state
 *     runtime state
 *     wall-clock time
 *     randomness
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no runtime execution;
 *     no filesystem access;
 *     no network access;
 *     no hardware access.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar has no language-level finite ceiling on:
 *
 *     number of adaptive expressions
 *     number of context operands
 *     expression nesting
 *     program size
 *     target size
 *     model size
 *     strategy size
 *     tensor rank
 *     quantum operation count
 *     qubit count
 *     processor count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     node count
 *     device count
 *     memory capacity
 *     network size
 *
 * The argument repetition:
 *
 *     (COMMA adaptiveContext)*
 *
 * is intentionally open-ended.
 *
 * "Infinity" means that the language does not define an artificial universal
 * finite machine-capacity ceiling. It does not claim physically infinite
 * hardware or unlimited compiler resources.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The source program expresses adaptation intent without encoding its physical
 * realization.
 *
 * The compiler/runtime may therefore specialize the same source for:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * Resource feasibility is determined by:
 *
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *     execution planning
 *     target negotiation
 *
 * It is never determined by this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * STRUCTURAL/PARSER ERRORS
 * ------------------------
 *
 *     adapt()
 *     adapt(,)
 *     adapt(target,)
 *     adapt(target,,context)
 *     adapt(target context)
 *     adapt(target, context
 *     adapt(target, context,)
 *
 * SEMANTIC ERRORS
 * ---------------
 *
 *     target is not adaptable
 *     incompatible operand types
 *     unauthorized adaptation
 *     unavailable capability
 *     insufficient resources
 *     policy violation
 *     contract violation
 *     invalid provenance
 *     unsupported realization
 *
 * Target feasibility errors MUST NOT be reported as lexical or parser errors.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AdaptiveExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 */

adaptiveExpression
    : adaptiveOperation
    ;


/*
 * ============================================================================
 * ADAPTIVE OPERATION
 * ============================================================================
 *
 * Parentheses distinguish the expression form from the statement form.
 *
 *     adapt(...)
 *
 * is therefore unambiguously an expression-level adaptation request.
 */

adaptiveOperation
    : ADAPT
      LPAREN
      adaptiveArgumentList
      RPAREN
    ;


/*
 * ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 *
 * At least one argument is required.
 *
 * The first argument is the target.
 *
 * Additional arguments are open-ended context.
 */

adaptiveArgumentList
    : adaptiveTarget
      (
          COMMA
          adaptiveContext
      )*
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * The target is an ordinary canonical expression.
 */

adaptiveTarget
    : expressionCore
    ;


/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * Context is deliberately an ordinary expression.
 *
 * Semantic analysis may classify it as:
 *
 *     source
 *     feedback
 *     evidence
 *     policy
 *     constraint
 *     observation
 *     model
 *     strategy
 *     provenance
 *     configuration
 *     capability information
 *     resource information
 *     learning result
 *     reasoning result
 *     quantum result
 *     simulation result
 *     distributed result
 *     future semantic context
 */

adaptiveContext
    : expressionCore
    ;


/*
 * ============================================================================
 * INTEGRATION INVARIANTS
 * ============================================================================
 *
 * 1. `adaptiveExpression` is the sole universal expression-level adaptation
 *    boundary.
 *
 * 2. `adaptStatement` remains the sole statement-level adaptation boundary.
 *
 * 3. Both forms converge on one semantic adaptation operation.
 *
 * 4. This grammar introduces no expression precedence hierarchy.
 *
 * 5. This grammar introduces no lexer rules.
 *
 * 6. This grammar introduces no physical hardware model.
 *
 * 7. This grammar introduces no fixed resource limits.
 *
 * 8. This grammar introduces no adaptation algorithm catalogue.
 *
 * 9. This grammar performs no runtime operation.
 *
 * 10. Future adaptation mechanisms can use ordinary expressions, semantic
 *     capabilities, policies, dialects and libraries without modifying this
 *     universal grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     adapt(strategy)
 *     adapt(model)
 *     adapt(strategy, feedback)
 *     adapt(model, training_result)
 *     adapt(strategy, feedback, policy)
 *     adapt(plan, observation, policy, evidence)
 *     adapt(hybrid_plan, measurement_result, policy)
 *     adapt(quantum_strategy, quantum_result, provenance)
 *     adapt(distributed_strategy, distributed_result, resource_state)
 *     adapt(hardware_plan, simulation_result, verification)
 *     adapt(tensor_strategy, performance_observation)
 *     adapt(strategy, infer(candidate))
 *     adapt(strategy, query_result)
 *     adapt(model, learning_result, confidence)
 *     adapt(plan, evidence, provenance, policy, constraint)
 *
 * NEGATIVE
 * --------
 *
 *     adapt()
 *     adapt(,)
 *     adapt(strategy,)
 *     adapt(strategy,,feedback)
 *     adapt(strategy feedback)
 *     adapt(strategy, feedback
 *     adapt(strategy, feedback,)
 *
 * BOUNDARY
 * --------
 *
 *     outer(adapt(strategy))
 *     call(adapt(model, observation))
 *     match adapt(strategy, evidence) { ... }
 *     adapt(adapt(strategy), feedback)
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     adapt(classical_strategy, classical_feedback)
 *     adapt(quantum_strategy, measurement_result)
 *     adapt(hybrid_strategy, quantum_result)
 *     adapt(hardware_strategy, simulation_result)
 *     adapt(distributed_strategy, distributed_result)
 *     adapt(tensor_strategy, performance_observation)
 *
 * SCALABILITY
 * -----------
 *
 *     adapt(target)
 *     adapt(target, context_a, context_b, context_c, ...)
 *
 * The parser imposes no language-level cardinality ceiling.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] canonical ADAPT token is consumed;
 *     [ ] no lexer rules are duplicated;
 *     [ ] expressionCore is reused;
 *     [ ] no competing expression precedence hierarchy exists;
 *     [ ] expression-level adapt syntax parses;
 *     [ ] statement-level adapt syntax remains separate;
 *     [ ] both forms converge semantically;
 *     [ ] AST remains domain-neutral;
 *     [ ] type analysis remains downstream;
 *     [ ] effect analysis remains downstream;
 *     [ ] capability analysis remains downstream;
 *     [ ] resource analysis remains downstream;
 *     [ ] contract analysis remains downstream;
 *     [ ] policy analysis remains downstream;
 *     [ ] provenance remains downstream;
 *     [ ] quantum semantics cross through quantum::ir;
 *     [ ] no physical hardware is encoded;
 *     [ ] no finite machine capacity is encoded;
 *     [ ] no adaptation algorithm is enumerated;
 *     [ ] parsing is deterministic;
 *     [ ] parser contains no runtime behavior;
 *     [ ] Rust 1.97+ compatibility is preserved;
 *     [ ] safe Rust integration is preserved;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */