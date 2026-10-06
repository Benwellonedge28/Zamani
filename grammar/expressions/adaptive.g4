/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/expressions/adaptive.g4
 *
 * Grammar:
 *     AdaptiveExpressions
 *
 * Role:
 *     CANONICAL EXPRESSION-LEVEL ADAPTATION BOUNDARY
 *
 * Status:
 *     PRODUCTION-READY DESIGN
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1 or later
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
 * This file owns the EXPRESSION-LEVEL representation of adaptive
 * computational intent.
 *
 * It complements, but does not replace:
 *
 *     grammar/statements/adapt.g4
 *
 * The statement grammar owns:
 *
 *     adapt TARGET;
 *     adapt TARGET from SOURCE;
 *     adapt TARGET with (CONTEXT);
 *     adapt TARGET from SOURCE with (CONTEXT);
 *
 * This file owns only the value-producing expression form:
 *
 *     adapt(TARGET)
 *     adapt(TARGET, SOURCE)
 *     adapt(TARGET, SOURCE, CONTEXT...)
 *
 * The expression form represents an adaptation operation whose result may be
 * consumed by another expression.
 *
 * Examples:
 *
 *     let next_strategy = adapt(strategy);
 *
 *     let next_model = adapt(model, training_result);
 *
 *     let next_plan = adapt(
 *         execution_plan,
 *         observation,
 *         policy,
 *         evidence
 *     );
 *
 * The exact semantic interpretation is downstream.
 *
 * This grammar does NOT decide:
 *
 *     - whether adaptation is permitted;
 *     - whether a target is adaptable;
 *     - what adaptation algorithm is selected;
 *     - whether the result mutates state;
 *     - whether code is regenerated;
 *     - whether recompilation occurs;
 *     - whether execution is rescheduled;
 *     - whether routing changes;
 *     - whether quantum computation changes;
 *     - whether hardware changes;
 *     - whether a model changes;
 *     - whether a policy authorizes the operation;
 *     - whether resources are sufficient;
 *     - whether capabilities are available.
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
 *     ANTLR parser
 *          |
 *          v
 *     adaptiveExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *        types             effects            contracts
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                  +-----------+-----------+
 *                  |           |           |
 *                  v           v           v
 *             capabilities  resources   policies
 *                  |           |           |
 *                  +-----------+-----------+
 *                              |
 *                              v
 *                         provenance
 *                              |
 *                              v
 *                   semantic adaptation model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *         classical       quantum::ir       other domains
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         specialization
 *                              |
 *                           lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / recovery
 *                              |
 *                         ZQN / HAL
 *                              |
 *                              v
 *                       target realization
 *
 * This file remains entirely above physical realization.
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
 *     adaptiveTarget
 *     adaptiveSource
 *     adaptiveContextList
 *     adaptiveContext
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer rules
 *     keyword definitions
 *     token spellings
 *     expression precedence
 *     primary-expression composition
 *     postfix expressions
 *     identifiers
 *     names
 *     types
 *     statements
 *     statement-level adapt syntax
 *     learning syntax
 *     reasoning syntax
 *     knowledge syntax
 *     uncertainty syntax
 *     policies
 *     contracts
 *     effects
 *     capabilities
 *     resources
 *     provenance
 *     AI semantics
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     execution planning
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime implementation
 *     AST implementation
 *     semantic implementation
 *     IR implementation
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There are intentionally TWO source-level adaptation forms, but they have
 * different syntactic roles:
 *
 *     grammar/statements/adapt.g4
 *         -> statement-level adaptation
 *
 *     grammar/expressions/adaptive.g4
 *         -> expression-level adaptation
 *
 * They MUST converge on the SAME semantic adaptation operation.
 *
 * They MUST NOT become two unrelated adaptation semantics.
 *
 * The statement form is effectful/statement-oriented.
 *
 * The expression form produces a value/result that can participate in the
 * ordinary expression hierarchy.
 *
 * The semantic layer determines whether either form represents:
 *
 *     state mutation
 *     model transformation
 *     strategy transformation
 *     execution-plan transformation
 *     pure planning
 *     runtime adaptation
 *     recompilation intent
 *     respecialization
 *     other future adaptation semantics
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     canonical Zamani lexer
 *     canonical expression-core boundary
 *
 * The expression operands are deliberately represented through:
 *
 *     expressionCore
 *
 * This follows the same architectural boundary already used by the
 * repository's reasoning-expression grammar.
 *
 * This grammar MUST NOT import:
 *
 *     Expressions
 *
 * when Expressions imports AdaptiveExpressions.
 *
 * Doing so would create a circular parser-grammar dependency.
 *
 * ============================================================================
 * EXPRESSION-CORE CONTRACT
 * ============================================================================
 *
 * `expressionCore` is an integration boundary.
 *
 * It represents the canonical expression hierarchy supplied by the assembled
 * parser.
 *
 * This grammar MUST NOT reproduce:
 *
 *     assignment precedence
 *     conditional precedence
 *     logical precedence
 *     comparison precedence
 *     arithmetic precedence
 *     bitwise precedence
 *     unary precedence
 *     postfix precedence
 *     primary-expression precedence
 *
 * The repository's canonical expression composition remains the sole owner
 * of those concerns.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     tokenVocab = ZamaniLexer
 *
 * The canonical keyword already established by the repository is:
 *
 *     ADAPT
 *
 * This file MUST NOT define:
 *
 *     ADAPT
 *
 * or any other lexer rule.
 *
 * No new keyword such as:
 *
 *     ADAPTIVE
 *     ADAPTATION_EXPRESSION
 *     EVOLVE
 *     MODIFY
 *     ADJUST
 *     CHANGE
 *
 * is introduced merely to create expression syntax.
 *
 * The same `adapt` vocabulary is intentionally shared between the statement
 * and expression forms.
 *
 * ============================================================================
 * WHY THE EXPRESSION FORM USES PARENTHESES
 * ============================================================================
 *
 * Statement form:
 *
 *     adapt strategy from feedback;
 *
 * Expression form:
 *
 *     adapt(strategy, feedback)
 *
 * This distinction is deliberate.
 *
 * It prevents the expression grammar from duplicating the statement's
 * `from` / `with` clause structure while allowing adaptation to participate
 * naturally anywhere an expression is accepted.
 *
 * It also provides an unambiguous boundary for the parser:
 *
 *     ADAPT LPAREN ...
 *
 * therefore denotes the expression form.
 *
 * ============================================================================
 * CANONICAL EXPRESSION FORMS
 * ============================================================================
 *
 * Minimal:
 *
 *     adapt(target)
 *
 * Target plus source:
 *
 *     adapt(target, source)
 *
 * Target plus one context value:
 *
 *     adapt(target, context)
 *
 * Target plus source and context:
 *
 *     adapt(target, source, context)
 *
 * Multiple context values:
 *
 *     adapt(target, source, context_a, context_b, context_c)
 *
 * Because the expression form is intentionally positional, semantic analysis
 * determines whether the additional operands represent:
 *
 *     source
 *     policy
 *     evidence
 *     feedback
 *     observation
 *     provenance
 *     configuration
 *     constraint
 *     resource information
 *     capability information
 *     model information
 *     strategy information
 *     other future context
 *
 * No finite vocabulary is encoded here.
 *
 * ============================================================================
 * ARGUMENT MODEL
 * ============================================================================
 *
 * The first argument is always the adaptation target.
 *
 * All remaining arguments are ordered adaptation inputs/context values.
 *
 * The grammar deliberately does not assign a fixed semantic meaning to
 * positions after the first argument.
 *
 * Semantic analysis may interpret:
 *
 *     adapt(target)
 *
 * as a minimal adaptation request.
 *
 * It may interpret:
 *
 *     adapt(target, source)
 *
 * as target + source.
 *
 * It may interpret:
 *
 *     adapt(target, source, policy)
 *
 * as target + source + policy.
 *
 * Such interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * NO KEYWORD EXPLOSION
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     gradient
 *     reinforcement
 *     Bayesian
 *     evolutionary
 *     heuristic
 *     genetic
 *     predictive
 *     online
 *     offline
 *     transfer
 *     fine_tune
 *     reroute
 *     reschedule
 *     remap
 *     recompile
 *     specialize
 *     optimize
 *
 * as adaptation-expression alternatives.
 *
 * Those are algorithms, strategies, execution mechanisms, libraries,
 * capabilities, dialects or semantic policies.
 *
 * Future adaptation mechanisms MUST NOT require a new universal grammar rule
 * merely because the mechanism is new.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning remains owned by:
 *
 *     grammar/ai/learning.g4
 *     grammar/statements/learn.g4
 *
 * An adaptive expression may consume a learning result:
 *
 *     adapt(model, training_result)
 *
 * or:
 *
 *     adapt(strategy, learned_policy)
 *
 * The expression grammar does not know that the second argument came from
 * learning.
 *
 * Semantic analysis resolves that relationship.
 *
 * Learning does not automatically authorize adaptation.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Adaptive expressions may consume reasoning results:
 *
 *     adapt(strategy, inferred_strategy)
 *
 *     adapt(plan, reason_result)
 *
 *     adapt(model, deduced_configuration)
 *
 * The reasoning grammar remains the owner of:
 *
 *     infer
 *     deduce
 *     reason
 *
 * No reasoning syntax is duplicated here.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Knowledge may supply adaptation input:
 *
 *     adapt(strategy, knowledge_result)
 *
 *     adapt(model, query_result)
 *
 * Knowledge grammar remains independently owned.
 *
 * This grammar simply accepts the resulting expression as an operand.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Adaptive expressions may consume:
 *
 *     probabilities
 *     confidence
 *     distributions
 *     beliefs
 *     observations
 *     intervals
 *     uncertain values
 *
 * No probabilistic implementation is encoded here.
 *
 * No fixed precision is encoded here.
 *
 * No fixed number of outcomes is encoded here.
 *
 * ============================================================================
 * EVIDENCE AND PROVENANCE
 * ============================================================================
 *
 * Adaptive expressions may consume evidence and provenance values.
 *
 * Example:
 *
 *     adapt(strategy, evidence, provenance)
 *
 * The grammar records only the expressions.
 *
 * The semantic/provenance layers determine:
 *
 *     evidence validity
 *     evidence origin
 *     derivation
 *     confidence
 *     verification
 *     authorization
 *     decision provenance
 *     resulting transformation
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing this expression performs no runtime effect.
 *
 * Semantic analysis may classify the resolved operation with effects such as:
 *
 *     mutation
 *     adaptation
 *     learning
 *     randomness
 *     reflection
 *     code_generation
 *     native
 *     foreign
 *     network
 *     distributed
 *     measurement
 *     simulation
 *     runtime_control
 *
 * The grammar MUST NOT hard-code the final effect set.
 *
 * An apparently simple adaptive expression may resolve to different effects
 * depending on the target and context.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Semantic analysis may require capabilities such as:
 *
 *     capability("adaptation")
 *     capability("model.update")
 *     capability("strategy.update")
 *     capability("runtime.adaptation")
 *     capability("learning.update")
 *     capability("knowledge.read")
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *
 * These are semantic identifiers, not grammar-level reserved words.
 *
 * Capability names remain open-world.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Adaptive expressions may require arbitrary resources, including:
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
 *     other future resources
 *
 * No quantities are encoded here.
 *
 * This grammar MUST NOT introduce:
 *
 *     maximum adaptation count
 *     maximum context count
 *     maximum model size
 *     maximum strategy size
 *     maximum CPU count
 *     maximum GPU count
 *     maximum FPGA count
 *     maximum QPU count
 *     maximum qubit count
 *     maximum node count
 *     maximum thread count
 *     maximum memory
 *     maximum tensor rank
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Adaptive expressions participate in the universal contract system.
 *
 * Applicable contracts may include:
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
 * A surrounding contract may constrain an adaptive expression.
 *
 * Semantic analysis determines whether the adaptation preserves the relevant
 * guarantees.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Adaptive expressions may be constrained by:
 *
 *     security policy
 *     resource policy
 *     execution policy
 *     adaptation policy
 *     model policy
 *     data policy
 *     network policy
 *     quantum policy
 *     deployment policy
 *
 * Policy syntax remains owned by the policy subsystem.
 *
 * The presence of `adapt(...)` does not grant authorization.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * This grammar MUST NOT:
 *
 *     execute adaptation;
 *     invoke models;
 *     access files;
 *     access networks;
 *     inspect hardware;
 *     access credentials;
 *     invoke a QPU;
 *     invoke a simulator;
 *     modify compiler state;
 *     modify source code;
 *     modify executable code;
 *     bypass sandbox restrictions;
 *     bypass policy;
 *     bypass capability checks.
 *
 * An adaptive expression is an intent/value expression, not an authorization
 * mechanism.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces no Rust AST implementation.
 *
 * The conceptual AST representation is:
 *
 *     AdaptiveExpression {
 *         target,
 *         context,
 *         source_span
 *     }
 *
 * The implementation may reuse the repository's generic operation/expression
 * representation.
 *
 * If the repository already has a generic semantic operation node, the
 * frontend SHOULD represent:
 *
 *     adapt(...)
 *
 * through that existing abstraction rather than creating an incompatible
 * second hierarchy.
 *
 * The AST MUST remain domain-neutral.
 *
 * It MUST NOT contain:
 *
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     physical qubit mappings
 *     routing decisions
 *     scheduling decisions
 *     calibration
 *     vendor-specific topology
 *     QEC layout
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the target exists;
 *     - whether the target is adaptable;
 *     - whether the operands have valid types;
 *     - whether source/context values are compatible;
 *     - whether adaptation is authorized;
 *     - which effects are required;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - which contracts apply;
 *     - which policies apply;
 *     - which provenance is required;
 *     - whether the result is deterministic;
 *     - whether the adaptation preserves declared semantics;
 *     - whether the result can be realized on a selected target.
 *
 * Syntax validity is not semantic validity.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The grammar imposes no universal adaptation type.
 *
 * The type system determines the result type.
 *
 * Depending on semantic resolution, an adaptive expression may produce:
 *
 *     a value
 *     a model
 *     a strategy
 *     a configuration
 *     an execution plan
 *     a computation
 *     a policy state
 *     a domain object
 *     another semantic value
 *
 * No physical machine type is implied.
 *
 * ============================================================================
 * VALUE / EXPRESSION CONTRACT
 * ============================================================================
 *
 * `adaptiveExpression` MUST remain usable anywhere the canonical expression
 * grammar permits a primary expression.
 *
 * Examples:
 *
 *     let x = adapt(strategy);
 *
 *     call(adapt(model, training_result));
 *
 *     result = adapt(plan, observation);
 *
 *     if adapt(policy, evidence) {
 *         ...
 *     }
 *
 *     match adapt(strategy, feedback) {
 *         ...
 *     }
 *
 * Whether a particular use is type-correct is semantic/type-system behavior.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The canonical lowering path is:
 *
 *     adaptiveExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic adaptation operation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     classical semantics   quantum semantics
 *                               |
 *                               v
 *                          quantum::ir
 *          |
 *          +--------------------+
 *                     |
 *                     v
 *                optimization
 *                     |
 *                 lowering
 *                     |
 *              routing/scheduling
 *                     |
 *                resilience
 *                     |
 *                  ZQN/HAL
 *                     |
 *                     v
 *              target realization
 *
 * If adaptation affects quantum computation, the quantum portion MUST cross:
 *
 *     quantum::ir
 *
 * before quantum optimization, decomposition, routing, scheduling, QEC,
 * resilience or target realization.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * The expression may consume or produce quantum-related semantic values:
 *
 *     adapt(strategy, quantum_result)
 *
 *     adapt(hybrid_plan, measurement_result)
 *
 *     adapt(classical_control, quantum_observation)
 *
 * The grammar does not define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     physical qubits
 *     coupling maps
 *     calibration
 *     routing
 *     QEC
 *
 * Quantum semantics remain outside this file.
 *
 * ============================================================================
 * HYBRID CONTRACT
 * ============================================================================
 *
 * Adaptive expressions can participate in:
 *
 *     classical -> adaptation
 *     quantum -> adaptation
 *     adaptation -> classical
 *     adaptation -> quantum control
 *     AI -> adaptation
 *     adaptation -> AI
 *     HDL observation -> adaptation
 *     simulation -> adaptation
 *     distributed observation -> adaptation
 *
 * These relationships are semantic.
 *
 * No domain-specific grammar is introduced here.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Adaptive expressions may semantically consume:
 *
 *     hardware observations
 *     simulation results
 *     verification results
 *     execution telemetry
 *     resource observations
 *
 * The grammar does not encode:
 *
 *     register width
 *     bus width
 *     fixed memory size
 *     device count
 *     node count
 *     topology
 *     physical placement
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED / CONCURRENT CONTRACT
 * ============================================================================
 *
 * Adaptive expressions may consume distributed observations or results.
 *
 * Examples:
 *
 *     adapt(strategy, distributed_result)
 *
 *     adapt(plan, resource_observation)
 *
 * They may subsequently participate in:
 *
 *     actors
 *     tasks
 *     channels
 *     services
 *     workflows
 *     collective computation
 *
 * Actor and message syntax remains owned by:
 *
 *     grammar/concurrency/
 *     grammar/distributed/
 *
 * This file does not create a second concurrency model.
 *
 * ============================================================================
 * SIMULATION CONTRACT
 * ============================================================================
 *
 * Adaptive expressions may consume simulation results:
 *
 *     adapt(strategy, simulation_result)
 *
 * Simulation remains an execution strategy.
 *
 * This grammar does not distinguish:
 *
 *     classical simulation
 *     quantum simulation
 *     hardware simulation
 *     distributed simulation
 *     AI simulation
 *
 * Semantic execution planning resolves the actual mode.
 *
 * ============================================================================
 * METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * An adaptive expression MUST NOT automatically imply reflection,
 * self-modification or code generation.
 *
 * If adaptation involves:
 *
 *     reflection
 *     syntax-tree transformation
 *     code generation
 *     recompilation
 *
 * the semantic system MUST require the appropriate capability/effect/policy.
 *
 * Metaprogramming remains owned by:
 *
 *     grammar/metaprogramming/
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend must preserve enough source structure for provenance.
 *
 * Downstream provenance may record:
 *
 *     operation
 *     target
 *     operands
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
 *     compiler version
 *     language version
 *     target capability snapshot
 *
 * This grammar does not define a duplicate provenance language.
 *
 * ============================================================================
 * COMPATIBILITY WITH STATEMENT-LEVEL ADAPTATION
 * ============================================================================
 *
 * Statement:
 *
 *     adapt strategy from feedback;
 *
 * Expression:
 *
 *     adapt(strategy, feedback)
 *
 * These are intentionally different syntactic forms.
 *
 * Both MUST map to the same semantic adaptation abstraction.
 *
 * The statement form is appropriate where the operation itself is the
 * statement-level action.
 *
 * The expression form is appropriate where the resulting value participates
 * in another expression.
 *
 * Neither form may silently acquire different fundamental semantics merely
 * because of syntax position.
 *
 * ============================================================================
 * COMPATIBILITY WITH `mind.adapt(...)`
 * ============================================================================
 *
 * The repository already has an open-world expression form through the
 * cognitive expression grammar:
 *
 *     mind.adapt(...)
 *
 * That construct MUST remain valid.
 *
 * It is NOT duplicated or replaced by this file.
 *
 * Instead:
 *
 *     mind.adapt(...)
 *          |
 *          v
 *     semantic adaptation operation
 *
 * and:
 *
 *     adapt(...)
 *          |
 *          v
 *     semantic adaptation operation
 *
 * should converge downstream when their semantics are equivalent.
 *
 * The two source forms may remain distinct because:
 *
 *     mind.adapt(...)
 *
 * is namespace-qualified/general cognitive composition, while:
 *
 *     adapt(...)
 *
 * is the universal adaptation primitive.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * AI-specific adaptation remains owned by:
 *
 *     grammar/ai/adaptation.g4
 *
 * That grammar MUST remain an adapter to the universal adaptation operation.
 *
 * It MUST NOT create:
 *
 *     AIAdaptationExpression
 *
 * as a competing universal semantic model.
 *
 * The AI subsystem may consume:
 *
 *     adaptiveExpression
 *
 * when an adaptive expression is semantically associated with:
 *
 *     models
 *     learning
 *     inference
 *     reasoning
 *     agents
 *     knowledge
 *     uncertainty
 *     evidence
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/effects/adaptation.g4
 *
 * remains the effect-side owner of adaptation effects.
 *
 * This grammar MUST NOT redefine the effect taxonomy.
 *
 * Semantic flow:
 *
 *     adaptiveExpression
 *          |
 *          v
 *     effect analysis
 *          |
 *          v
 *     canonical adaptation effect set
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Adaptive expressions may participate in:
 *
 *     grammar/resources/
 *
 * for:
 *
 *     capability requirements
 *     resource requirements
 *     constraints
 *     budgets
 *     preferences
 *     negotiation
 *     scalability
 *
 * The grammar itself remains independent of actual resource quantities.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policy ownership remains outside this file.
 *
 * An adaptive expression can be constrained by:
 *
 *     permissions
 *     prohibitions
 *     requirements
 *     constraints
 *     preferences
 *     fallback policies
 *     security policies
 *     execution policies
 *
 * The parser only records the operands.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_ADAPTATIONS
 *     MAX_ADAPTATION_CONTEXT
 *     MAX_MODELS
 *     MAX_STRATEGIES
 *     MAX_RESOURCES
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
 *     MAX_QUBITS
 *
 * It MUST NOT enumerate:
 *
 *     hardware providers
 *     physical devices
 *     CPU architectures
 *     GPU models
 *     FPGA families
 *     QPU vendors
 *     fixed topologies
 *
 * It MUST NOT contain resource-sized arrays or grammar alternatives that
 * imply a fixed machine capacity.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The expression supports an open-ended operand sequence after the target.
 *
 * Therefore:
 *
 *     adapt(target)
 *
 *     adapt(target, source)
 *
 *     adapt(target, source, context_a, context_b, ...)
 *
 * are structurally represented without a language-level cardinality ceiling.
 *
 * Practical parser/compiler limits are implementation/resource limits.
 *
 * They MUST NOT become language-level constants.
 *
 * Scaling may occur across:
 *
 *     tiny embedded systems
 *     CPU systems
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
 *     cloud environments
 *     future computational targets
 *
 * The source expression remains target-independent.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * POCO-REAF is achieved here by keeping adaptation intent independent of
 * realization.
 *
 * The program describes:
 *
 *     WHAT should adapt
 *     WHAT information may influence adaptation
 *
 * It does not prescribe:
 *
 *     WHERE adaptation executes
 *     HOW many processors execute it
 *     WHICH accelerator executes it
 *     WHICH quantum device executes it
 *     WHICH network topology executes it
 *     WHICH memory system executes it
 *
 * Target realization is selected only after:
 *
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     provenance requirements
 *     execution planning
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     adapt()
 *     adapt(,)
 *     adapt(target,)
 *     adapt(target,,source)
 *     adapt(target source)
 *     adapt(target, source
 *     adapt(target, source,)
 *
 * Semantic diagnostics:
 *
 *     target is not adaptable
 *     incompatible operand types
 *     unauthorized adaptation
 *     capability unavailable
 *     resources insufficient
 *     policy violation
 *     contract violation
 *     invalid provenance
 *     unsupported target realization
 *
 * A target-feasibility error MUST NOT be reported as a parser error.
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
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU availability
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     memory availability
 *     network state
 *     filesystem state
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
 *     no embedded Rust
 *     no Rust actions
 *     no semantic predicates
 *     no runtime execution
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no unsafe Rust
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
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
 * 1. PUBLIC EXPRESSION ENTRY
 * ============================================================================
 */

adaptiveExpression
    : adaptiveOperation
    ;


/*
 * ============================================================================
 * 2. ADAPTIVE OPERATION
 * ============================================================================
 *
 * The parentheses are mandatory for the expression form.
 *
 * This makes:
 *
 *     adapt(...)
 *
 * unambiguously different from:
 *
 *     adapt ...;
 *
 * at the statement boundary.
 */

adaptiveOperation
    : ADAPT
      LPAREN
      adaptiveArgumentList
      RPAREN
    ;


/*
 * ============================================================================
 * 3. ARGUMENT LIST
 * ============================================================================
 *
 * At least one argument is mandatory because the first argument is the
 * adaptation target.
 *
 * The list is intentionally open-ended.
 *
 * No fixed number of context operands is imposed.
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
 * 4. TARGET
 * ============================================================================
 *
 * The target is an ordinary canonical expression.
 *
 * This named boundary exists for AST/semantic tooling.
 */

adaptiveTarget
    : expressionCore
    ;


/*
 * ============================================================================
 * 5. CONTEXT
 * ============================================================================
 *
 * Every operand after the first is intentionally an ordinary expression.
 *
 * The semantic layer determines whether an operand is:
 *
 *     source
 *     feedback
 *     evidence
 *     policy
 *     constraint
 *     model
 *     strategy
 *     observation
 *     provenance
 *     configuration
 *     resource information
 *     capability information
 *     learned information
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
 * 6. EXPRESSION-CORE BOUNDARY
 * ============================================================================
 *
 * `expressionCore` is supplied by the canonical assembled expression system.
 *
 * This grammar intentionally does not define it.
 *
 * The repository MUST maintain exactly one expression precedence hierarchy.
 *
 * No local expression implementation is permitted here.
 */


/*
 * ============================================================================
 * 7. NO STATEMENT TERMINATOR
 * ============================================================================
 *
 * `adaptiveExpression` deliberately does not consume:
 *
 *     SEMICOLON
 *
 * The enclosing expression/statement grammar owns statement termination.
 */


/*
 * ============================================================================
 * 8. NO CLOSED ADAPTATION VOCABULARY
 * ============================================================================
 *
 * This grammar deliberately does not contain alternatives for:
 *
 *     model
 *     strategy
 *     policy
 *     evidence
 *     feedback
 *     learning
 *     reasoning
 *     quantum
 *     hardware
 *     simulation
 *     distributed
 *
 * Those are values/semantic concepts, not universal syntax categories.
 */


/*
 * ============================================================================
 * 9. FINAL INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 * ----------
 *
 * `adaptiveExpression` is the sole expression-level universal adaptation
 * boundary.
 *
 * INVARIANT 2
 * ----------
 *
 * `adaptStatement` remains the sole statement-level adaptation boundary.
 *
 * INVARIANT 3
 * ----------
 *
 * Both converge on one semantic adaptation model.
 *
 * INVARIANT 4
 * ----------
 *
 * No second expression precedence hierarchy is introduced.
 *
 * INVARIANT 5
 * ----------
 *
 * No lexer rule is introduced.
 *
 * INVARIANT 6
 * ----------
 *
 * No application-specific adaptation syntax is introduced.
 *
 * INVARIANT 7
 * ----------
 *
 * No hardware target is encoded.
 *
 * INVARIANT 8
 * ----------
 *
 * No physical resource limit is encoded.
 *
 * INVARIANT 9
 * ----------
 *
 * No adaptation algorithm is hard-coded.
 *
 * INVARIANT 10
 * -----------
 *
 * Parsing performs no execution.
 */


/*
 * ============================================================================
 * 10. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     adapt(strategy)
 *
 *     adapt(model)
 *
 *     adapt(strategy, feedback)
 *
 *     adapt(model, training_result)
 *
 *     adapt(strategy, feedback, policy)
 *
 *     adapt(plan, observation, policy, evidence)
 *
 *     adapt(hybrid_plan, measurement_result, policy)
 *
 *     adapt(quantum_strategy, quantum_result, provenance)
 *
 *     adapt(distributed_strategy, distributed_result, resource_state)
 *
 *     adapt(hardware_plan, simulation_result, verification)
 *
 *     adapt(tensor_strategy, performance_observation)
 *
 *     adapt(strategy, infer(candidate))
 *
 *     adapt(strategy, query_result)
 *
 *     adapt(model, learning_result, confidence)
 *
 *     adapt(plan, evidence, provenance, policy, constraint)
 *
 * NEGATIVE
 * --------
 *
 *     adapt()
 *
 *     adapt(,)
 *
 *     adapt(strategy,)
 *
 *     adapt(strategy,,feedback)
 *
 *     adapt(strategy feedback)
 *
 *     adapt(strategy, feedback
 *
 *     adapt(strategy, feedback,)
 *
 * BOUNDARY
 * --------
 *
 * The expression must work wherever a primary expression is valid:
 *
 *     let x = adapt(strategy);
 *
 *     call(adapt(strategy, feedback));
 *
 *     result = adapt(plan, observation);
 *
 *     outer(adapt(inner(strategy)));
 *
 *     match adapt(strategy, evidence) { ... }
 *
 * The exact surrounding syntax is owned by the relevant expression/statement
 * grammars.
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     adapt(classical_strategy, classical_feedback)
 *
 *     adapt(quantum_strategy, measurement_result)
 *
 *     adapt(hybrid_strategy, quantum_result)
 *
 *     adapt(hardware_strategy, simulation_result)
 *
 *     adapt(distributed_strategy, distributed_result)
 *
 *     adapt(ai_strategy, learning_result)
 *
 *     adapt(execution_strategy, resource_observation)
 *
 * SCALABILITY
 * ----------
 *
 * Test increasing:
 *
 *     target expression size
 *     context operand count
 *     nesting depth
 *     program size
 *
 * without introducing grammar-level ceilings.
 *
 * DETERMINISM
 * -----------
 *
 * Repeated parsing of identical source under identical configuration must
 * produce equivalent parse structure and source spans.
 *
 * COMPATIBILITY
 * -------------
 *
 * Verify:
 *
 *     adapt strategy;
 *
 * continues to resolve through Adapt.adaptStatement.
 *
 * Verify:
 *
 *     mind.adapt(...)
 *
 * remains independent and valid through its existing expression grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. grammar/expressions/expressions.g4
 *
 *    MUST import:
 *
 *        AdaptiveExpressions
 *
 *    and integrate:
 *
 *        adaptiveExpression
 *
 *    into its canonical primary-expression alternatives.
 *
 * 2. grammar/statements/adapt.g4
 *
 *    REMAINS unchanged as the owner of:
 *
 *        adaptStatement
 *
 *    It must NOT import this grammar merely to obtain expression operands.
 *
 * 3. grammar/ai/adaptation.g4
 *
 *    REMAINS the AI-domain adapter.
 *
 *    It MUST NOT define a second adaptation-expression grammar.
 *
 * 4. grammar/effects/adaptation.g4
 *
 *    Remains the effect-side semantic grammar/specification boundary.
 *
 * 5. grammar/resources/
 *
 *    Owns resource and capability semantics consumed downstream.
 *
 * 6. grammar/validation/
 *
 *    Owns contract semantics consumed downstream.
 *
 * 7. grammar/policies/
 *
 *    Owns policy semantics consumed downstream.
 *
 * 8. grammar/expressions/mind.g4
 *
 *    `mind.adapt(...)` remains valid.
 *
 *    It may converge semantically with `adapt(...)`.
 *
 * 9. grammar/Zamani.g4
 *
 *    Receives the expression through the existing Expressions composition
 *    root. It MUST NOT duplicate `adaptiveExpression` directly.
 *
 * 10. AST
 *
 *    The frontend maps both:
 *
 *        adaptiveExpression
 *        adaptStatement
 *
 *    to the same domain-neutral adaptation semantic representation where
 *    their semantics are equivalent.
 *
 * 11. IR
 *
 *    No adaptation-specific IR is created here.
 *
 *    If the adaptation affects quantum computation, the affected quantum
 *    semantics MUST eventually enter:
 *
 *        quantum::ir
 *
 * 12. Rust
 *
 *    No Rust implementation changes are required merely to parse the grammar,
 *    but the generated parser and frontend integration MUST remain compatible
 *    with Rust 1.97+ and Rust 2021.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS DONE WHEN:
 *
 * [ ] `AdaptiveExpressions` is the canonical grammar name.
 *
 * [ ] `adaptiveExpression` is the only public expression-level adaptation
 *     rule.
 *
 * [ ] `adaptiveOperation` owns only the expression form.
 *
 * [ ] `adaptStatement` remains owned by `Adapt`.
 *
 * [ ] Both forms converge semantically.
 *
 * [ ] `expressionCore` resolves through the canonical assembled expression
 *     architecture.
 *
 * [ ] No second expression precedence hierarchy exists.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] No new keyword is required.
 *
 * [ ] No application-specific feature is hard-coded.
 *
 * [ ] No adaptation algorithm is hard-coded.
 *
 * [ ] No hardware target is hard-coded.
 *
 * [ ] No resource capacity is hard-coded.
 *
 * [ ] No physical topology is hard-coded.
 *
 * [ ] No quantum operation catalogue is hard-coded.
 *
 * [ ] No runtime operation occurs during parsing.
 *
 * [ ] No semantic decision occurs during parsing.
 *
 * [ ] Source spans are preserved by the parser/AST pipeline.
 *
 * [ ] Type analysis remains downstream.
 *
 * [ ] Effect analysis remains downstream.
 *
 * [ ] Capability analysis remains downstream.
 *
 * [ ] Resource analysis remains downstream.
 *
 * [ ] Contract analysis remains downstream.
 *
 * [ ] Policy analysis remains downstream.
 *
 * [ ] Provenance remains downstream.
 *
 * [ ] Quantum semantics ultimately use `quantum::ir`.
 *
 * [ ] `mind.adapt(...)` remains compatible.
 *
 * [ ] Statement-level `adapt ...;` remains compatible.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */