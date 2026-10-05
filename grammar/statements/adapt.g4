/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/adapt.g4
 *
 * GRAMMAR NAME
 * ------------
 * Adapt
 *
 * STATUS
 * ------
 * CANONICAL ADAPTATION STATEMENT GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the canonical source-level statement syntax for:
 *
 *     adapt
 *
 * Adaptation expresses PORTABLE COMPUTATIONAL INTENT.
 *
 * It allows a program to request an authorized change to a semantic
 * computation, strategy, model, state, execution choice, or other
 * adaptation target.
 *
 * The grammar describes WHAT is being requested.
 *
 * It does NOT decide:
 *
 *     - whether adaptation is permitted;
 *     - what adaptation algorithm is used;
 *     - what state is modified;
 *     - which model is selected;
 *     - which hardware is selected;
 *     - which target is selected;
 *     - how adaptation is executed;
 *     - whether recompilation is required;
 *     - whether routing changes;
 *     - whether scheduling changes;
 *     - whether QEC changes;
 *     - whether a backend changes;
 *     - whether resources are sufficient;
 *     - whether capabilities are available;
 *     - whether policy permits the operation;
 *     - whether provenance requirements are satisfied.
 *
 * Those responsibilities belong to semantic analysis, contracts, policies,
 * capabilities, resources, execution planning, provenance and downstream
 * target realization.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                           Zamani source
 *                                |
 *                                v
 *                         canonical lexer
 *                                |
 *                                v
 *                         canonical parser
 *                                |
 *                                v
 *                       statement composition
 *                                |
 *                                v
 *                         adaptStatement
 *                                |
 *                                v
 *                      domain-neutral frontend AST
 *                                |
 *                                v
 *                       structural validation
 *                                |
 *             +------------------+------------------+
 *             |                  |                  |
 *             v                  v                  v
 *           types             effects          contracts
 *             |                  |                  |
 *             +------------------+------------------+
 *                                |
 *             +------------------+------------------+
 *             |                  |                  |
 *             v                  v                  v
 *        capabilities        resources          policies
 *             |                  |                  |
 *             +------------------+------------------+
 *                                |
 *                                v
 *                         provenance / audit
 *                                |
 *                                v
 *                       semantic adaptation model
 *                                |
 *              +-----------------+-----------------+
 *              |                 |                 |
 *              v                 v                 v
 *          classical         quantum::ir       other domains
 *              |                 |                 |
 *              +-----------------+-----------------+
 *                                |
 *                                v
 *                    optimization / specialization
 *                                |
 *                                v
 *                     lowering / routing / scheduling
 *                                |
 *                                v
 *                       resilience / recovery
 *                                |
 *                                v
 *                           ZQN / HAL
 *                                |
 *                                v
 *                         target realization
 *
 * `adapt.g4` MUST remain above physical realization.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     adaptStatement
 *     adaptationTarget
 *     adaptationSourceClause
 *     adaptationContextClause
 *     adaptationContextList
 *     adaptationContext
 *
 * This file owns only the canonical statement-level composition of `adapt`.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer rules
 *     keyword definitions
 *     punctuation
 *     identifiers
 *     names
 *     qualified names
 *     paths
 *     expressions
 *     expression precedence
 *     types
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     learning
 *     inference
 *     reasoning
 *     knowledge
 *     actor syntax
 *     concurrency
 *     runtime lifecycle
 *     scheduling
 *     placement
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     quantum operations
 *     HDL syntax
 *     hardware syntax
 *     backend syntax
 *     AST implementation
 *     semantic implementation
 *     IR implementation
 *     runtime implementation
 *
 * ============================================================================
 * SINGLE OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative statement-level production:
 *
 *     adaptStatement
 *
 * for the canonical `adapt` statement.
 *
 * This file MUST NOT define:
 *
 *     statement
 *
 * The universal `statement` rule remains owned exclusively by:
 *
 *     grammar/statements/statements.g4
 *
 * The intended composition is:
 *
 *     statement
 *         |
 *         +--> adaptStatement
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     ReasonStatements
 *
 * ReasonStatements supplies the canonical expression dependency used by
 * reasoning-related statement adapters.
 *
 * This grammar deliberately reuses that established boundary rather than
 * introducing another expression hierarchy.
 *
 * Dependency direction:
 *
 *     Adapt
 *       |
 *       v
 *     ReasonStatements
 *       |
 *       v
 *     Expressions
 *
 * Adapt MUST NOT import:
 *
 *     Statements
 *     Domains
 *     Execution
 *     Quantum
 *     Hardware
 *     AI
 *
 * merely to obtain general expression syntax.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public export:
 *
 *     adaptStatement
 *
 * Private implementation rules:
 *
 *     adaptationTarget
 *     adaptationSourceClause
 *     adaptationContextClause
 *     adaptationContextList
 *     adaptationContext
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/statements/statements.g4
 *
 * Additional downstream consumers:
 *
 *     domain-neutral AST adapter
 *     semantic adaptation analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     provenance analysis
 *     execution planning
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes the canonical Zamani lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * The canonical lexical keyword for this statement is:
 *
 *     ADAPT
 *
 * Existing repository lexical authority:
 *
 *     grammar/lexer/keywords.g4
 *
 * already defines:
 *
 *     ADAPT : 'adapt' ;
 *
 * Therefore this grammar MUST NOT redefine ADAPT.
 *
 * It also MUST NOT introduce aliases such as:
 *
 *     adjust
 *     modify
 *     evolve
 *     change
 *     self_modify
 *
 * merely to increase vocabulary.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Adaptation operands are ordinary Zamani expressions.
 *
 * This is intentional.
 *
 * An adaptation target may therefore ultimately refer to:
 *
 *     a value
 *     a variable
 *     a state
 *     a model
 *     a strategy
 *     a policy
 *     a query result
 *     a learned result
 *     a measurement result
 *     a tensor
 *     a distributed result
 *     a hardware observation
 *     a simulation result
 *     a quantum-derived value
 *     a future computational value
 *
 * without requiring this grammar to enumerate those domains.
 *
 * This file MUST NOT define:
 *
 *     primaryExpression
 *     postfixExpression
 *     unaryExpression
 *     binaryExpression
 *     arithmeticExpression
 *     logicalExpression
 *     conditionalExpression
 *     assignmentExpression
 *
 * or any competing expression hierarchy.
 *
 * ============================================================================
 * CANONICAL SOURCE FORMS
 * ============================================================================
 *
 * The canonical minimal form is:
 *
 *     adapt TARGET;
 *
 * The optional source form is:
 *
 *     adapt TARGET from SOURCE;
 *
 * The optional context form is:
 *
 *     adapt TARGET with (CONTEXT);
 *
 * The combined form is:
 *
 *     adapt TARGET from SOURCE with (CONTEXT, CONTEXT);
 *
 * Therefore:
 *
 *     adapt strategy;
 *
 *     adapt model;
 *
 *     adapt execution_strategy from feedback;
 *
 *     adapt model from training_result;
 *
 *     adapt strategy with (policy);
 *
 *     adapt execution from observation with (feedback, policy);
 *
 * are syntactically represented by the same universal adaptation structure.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * The target is mandatory.
 *
 * Therefore:
 *
 *     adapt;
 *
 * is invalid.
 *
 * The target is an ordinary Zamani expression.
 *
 * The grammar does not decide whether the target is:
 *
 *     mutable
 *     adaptable
 *     a model
 *     a strategy
 *     an execution plan
 *     a quantum computation
 *     a classical computation
 *     an HDL construct
 *     a distributed computation
 *
 * Those questions belong to semantic analysis.
 *
 * ============================================================================
 * SOURCE CONTRACT
 * ============================================================================
 *
 * The optional `from` clause identifies information that can influence the
 * adaptation.
 *
 * It may semantically represent:
 *
 *     feedback
 *     observation
 *     measurement
 *     evidence
 *     model output
 *     performance information
 *     resource information
 *     execution state
 *     environment information
 *     simulation result
 *     distributed result
 *     hardware observation
 *     quantum measurement
 *     learned information
 *
 * The grammar intentionally does not distinguish those meanings.
 *
 * Example:
 *
 *     adapt strategy from feedback;
 *
 *     adapt model from training_result;
 *
 *     adapt execution from observation;
 *
 * ============================================================================
 * CONTEXT CONTRACT
 * ============================================================================
 *
 * The optional `with` clause supplies ordered adaptation context.
 *
 * Example:
 *
 *     adapt strategy with (policy);
 *
 *     adapt model with (objective, constraint);
 *
 *     adapt execution from feedback with (policy, provenance);
 *
 * Context entries are ordinary expressions.
 *
 * This avoids creating a closed list of adaptation-specific keywords.
 *
 * Future concepts can therefore be represented through ordinary expressions,
 * libraries, dialects, metadata, policies or semantic capabilities.
 *
 * ============================================================================
 * CLAUSE ORDER
 * ============================================================================
 *
 * The canonical clause order is:
 *
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *
 * Valid:
 *
 *     adapt target;
 *
 *     adapt target from source;
 *
 *     adapt target with (context);
 *
 *     adapt target from source with (context);
 *
 * Invalid:
 *
 *     adapt target with (context) from source;
 *
 * This gives adaptation one canonical source representation.
 *
 * ============================================================================
 * CONTEXT LIST
 * ============================================================================
 *
 * The context list is open-ended:
 *
 *     adaptationContext (',' adaptationContext)*
 *
 * No fixed number of context values is imposed.
 *
 * This means there is no language-level limit on:
 *
 *     policies
 *     constraints
 *     feedback values
 *     evidence values
 *     provenance references
 *     model references
 *     resource preferences
 *     capability references
 *
 * A trailing comma is intentionally rejected.
 *
 * Valid:
 *
 *     with (policy)
 *
 *     with (policy, feedback)
 *
 *     with (policy, feedback, provenance)
 *
 * Invalid:
 *
 *     with ()
 *
 *     with (policy,)
 *
 *     with (,policy)
 *
 *     with (policy,,feedback)
 *
 * ============================================================================
 * NO ADAPTATION ALGORITHM ENUMERATION
 * ============================================================================
 *
 * This grammar MUST NOT enumerate algorithms such as:
 *
 *     gradient
 *     reinforcement
 *     evolutionary
 *     Bayesian
 *     heuristic
 *     rule_based
 *     genetic
 *     online
 *     offline
 *     model_predictive
 *     recompilation
 *     remapping
 *     rerouting
 *     rescheduling
 *
 * as core syntax.
 *
 * Those are semantic strategies, library functionality, dialects, capabilities
 * or execution mechanisms.
 *
 * This keeps the language extensible without requiring grammar changes for
 * every future adaptation technique.
 *
 * ============================================================================
 * ADAPTATION IS NOT UNRESTRICTED SELF-MODIFICATION
 * ============================================================================
 *
 * The presence of:
 *
 *     adapt
 *
 * does NOT grant permission to arbitrarily modify:
 *
 *     source code
 *     executable code
 *     compiler state
 *     security state
 *     credentials
 *     policies
 *     protected memory
 *     hardware state
 *     external systems
 *
 * Authorization and permitted mutation are semantic properties.
 *
 * Adaptation should conceptually pass through:
 *
 *     adaptation intent
 *          |
 *          v
 *     policy validation
 *          |
 *          v
 *     capability validation
 *          |
 *          v
 *     effect validation
 *          |
 *          v
 *     resource validation
 *          |
 *          v
 *     contract validation
 *          |
 *          v
 *     provenance
 *          |
 *          v
 *     semantic adaptation
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume learning results.
 *
 * Examples:
 *
 *     adapt model from training_result;
 *
 *     adapt strategy from learned_policy;
 *
 *     adapt execution from prediction;
 *
 * Learning syntax remains owned by the learning subsystem.
 *
 * This file MUST NOT define:
 *
 *     train
 *     fit
 *     fine_tune
 *     reinforce
 *     transfer_learning
 *
 * as adaptation syntax.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume:
 *
 *     infer
 *     deduce
 *     reason
 *
 * results.
 *
 * Example:
 *
 *     adapt strategy from inferred_strategy;
 *
 * The reasoning subsystem remains the owner of reasoning syntax.
 *
 * Adaptation does not duplicate reasoning grammar.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume knowledge results:
 *
 *     adapt strategy from knowledge_result;
 *
 *     adapt model from knowledge.query(pattern);
 *
 * Knowledge operations remain owned by their respective grammars.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume evidence represented by ordinary expressions.
 *
 * Example:
 *
 *     adapt strategy from evidence;
 *
 * Evidence validation remains downstream.
 *
 * The semantic system may associate:
 *
 *     evidence
 *     source
 *     confidence
 *     derivation
 *     verification
 *     provenance
 *
 * with the resulting adaptation.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume:
 *
 *     probability
 *     confidence
 *     distribution
 *     uncertain values
 *     intervals
 *     observations
 *
 * without this grammar defining any probabilistic representation.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Adaptation participates in the universal contract system.
 *
 * Relevant semantic contracts may include:
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
 * Contract validation occurs downstream.
 *
 * Example semantic relationship:
 *
 *     requires capability("adaptation");
 *
 *     adapt strategy from feedback;
 *
 *     ensures valid(strategy);
 *
 * The grammar is concerned only with the `adapt` statement itself.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Adaptation may produce or require effects such as:
 *
 *     mutation
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
 * Exact effects are determined by semantic resolution.
 *
 * The parser does not infer effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Adaptation may require capabilities such as:
 *
 *     capability("adaptation")
 *     capability("model.update")
 *     capability("runtime.adaptation")
 *     capability("quantum.adaptation")
 *     capability("distributed.adaptation")
 *
 * These are examples of semantic capability identifiers, not a closed grammar
 * vocabulary.
 *
 * Capability availability is checked downstream.
 *
 * This grammar does not inspect the target machine.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Adaptation may require:
 *
 *     compute
 *     memory
 *     storage
 *     network
 *     accelerator
 *     quantum resources
 *     model resources
 *     distributed resources
 *
 * Resource requirements are semantic.
 *
 * This grammar MUST NOT encode:
 *
 *     maximum adaptation steps
 *     maximum adaptation depth
 *     maximum model count
 *     maximum feedback count
 *     maximum resource amount
 *     maximum CPU count
 *     maximum GPU count
 *     maximum FPGA count
 *     maximum QPU count
 *     maximum qubit count
 *     maximum node count
 *     maximum thread count
 *     maximum tensor rank
 *     maximum memory
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Adaptation is target-independent.
 *
 * The same source:
 *
 *     adapt strategy from feedback;
 *
 * may be semantically realized on:
 *
 *     tiny hardware
 *     embedded systems
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud
 *     future computational substrate
 *
 * without changing the source syntax merely because target size changes.
 *
 * "Scale to infinity" means:
 *
 *     no artificial finite machine/resource ceiling is imposed by this
 *     grammar.
 *
 * It does NOT mean physical machines possess infinite resources.
 *
 * Actual feasibility is determined by:
 *
 *     target capabilities
 *     available resources
 *     policies
 *     constraints
 *     execution environment
 *     compiler resources
 *     runtime resources
 *     physical feasibility
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO universal machine-size constants.
 *
 * Forbidden concepts include:
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
 * It also contains no adaptation-specific ceilings such as:
 *
 *     MAX_ADAPTATION_STEPS
 *     MAX_ADAPTATION_DEPTH
 *     MAX_FEEDBACK
 *     MAX_STRATEGIES
 *     MAX_MODELS
 *     MAX_POLICIES
 *     MAX_CONTEXT
 *
 * No physical device identifier is encoded.
 *
 * No target is selected here.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Adaptation may be driven by quantum results:
 *
 *     adapt strategy from measurement_result;
 *
 *     adapt execution from quantum_result;
 *
 *     adapt model from quantum_features;
 *
 * The grammar does not define quantum operations.
 *
 * Quantum semantics remain downstream:
 *
 *     source
 *       ->
 *     domain-neutral AST
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
 *     QEC / resilience
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * `adapt.g4` MUST NOT create another quantum IR.
 *
 * ============================================================================
 * QUANTUM ADAPTATION
 * ============================================================================
 *
 * Adaptation may semantically trigger:
 *
 *     rerouting
 *     rescheduling
 *     remapping
 *     reoptimization
 *     backend selection
 *     recompilation
 *     resilience changes
 *     recovery
 *
 * These are downstream semantic/runtime mechanisms.
 *
 * They MUST NOT become separate `adapt` grammar keywords.
 *
 * This preserves the separation:
 *
 *     source intent
 *          |
 *          v
 *     semantic adaptation
 *          |
 *          v
 *     execution planning
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Adaptation can participate in the existing resilience model.
 *
 * Semantic execution may react to states such as:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * and outcomes such as:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These states and outcomes are NOT grammar alternatives.
 *
 * They remain part of semantic/runtime resilience.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Adaptation may semantically operate on:
 *
 *     hardware intent
 *     synthesis decisions
 *     resource mappings
 *     execution strategy
 *     timing decisions
 *     placement decisions
 *     verification state
 *
 * The grammar does not encode:
 *
 *     bus width
 *     register width
 *     FPGA region
 *     ASIC resource count
 *     clock count
 *     device ID
 *     physical address
 *     physical topology
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume distributed observations and results.
 *
 * Example:
 *
 *     adapt strategy from distributed_feedback;
 *
 * The grammar does not encode:
 *
 *     worker count
 *     node count
 *     replica count
 *     partition count
 *     network size
 *
 * Those are resource and execution concerns.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Adaptation may occur within concurrent execution.
 *
 * The grammar does not own:
 *
 *     actor
 *     task
 *     channel
 *     spawn
 *     await
 *     parallel
 *     synchronization
 *
 * syntax.
 *
 * Concurrency remains owned by the concurrency subsystem.
 *
 * The semantic adaptation operation must respect the concurrency/effect model.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * The repository currently contains runtime-level adaptation syntax in:
 *
 *     grammar/execution/runtime.g4
 *
 * That grammar owns:
 *
 *     runtimeAdaptationStatement
 *     runtimeAdaptationClause
 *
 * Those rules represent runtime-specific adaptation intent.
 *
 * They MUST NOT become a second universal `adapt` statement authority.
 *
 * The intended relationship is:
 *
 *     adaptStatement
 *          |
 *          v
 *     generic adaptation semantic model
 *          |
 *          +--> runtime adaptation
 *          +--> learning adaptation
 *          +--> quantum adaptation
 *          +--> distributed adaptation
 *          +--> hardware adaptation
 *
 * Runtime-specific syntax remains a consumer of the common semantic model.
 *
 * ============================================================================
 * MIND / COGNITIVE EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Existing expression-level forms such as:
 *
 *     mind.adapt(strategy, feedback)
 *
 * remain expression syntax.
 *
 * They MUST NOT be rewritten as:
 *
 *     adapt ...
 *
 * automatically.
 *
 * Both may lower to the same semantic adaptation concept:
 *
 *     expression adaptation
 *          |
 *          v
 *     semantic adaptation operation
 *
 * and:
 *
 *     statement adaptation
 *          |
 *          v
 *     semantic adaptation operation
 *
 * This provides one semantic model without forcing one source spelling to
 * replace the other.
 *
 * ============================================================================
 * LEARNING / REASONING / KNOWLEDGE BOUNDARY
 * ============================================================================
 *
 * `adapt` may consume the outputs of:
 *
 *     learn
 *     infer
 *     deduce
 *     reason
 *     query
 *     measure
 *     simulate
 *
 * but this file does not duplicate those constructs.
 *
 * ============================================================================
 * POLICY BOUNDARY
 * ============================================================================
 *
 * Adaptation is policy-sensitive.
 *
 * Policies may constrain:
 *
 *     what may change
 *     who may authorize change
 *     which effects are permitted
 *     which capabilities are required
 *     which resources may be consumed
 *     whether external state may be modified
 *     whether model state may change
 *     whether code generation is permitted
 *     whether network operations are permitted
 *     whether adaptation is reversible
 *     whether provenance is mandatory
 *
 * A valid parse does NOT imply policy authorization.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The semantic adaptation record should preserve sufficient provenance to
 * reconstruct:
 *
 *     source adaptation request
 *     target
 *     source/feedback
 *     context
 *     selected adaptation strategy
 *     reason for adaptation
 *     evidence
 *     authorization
 *     policy
 *     capabilities
 *     resource decision
 *     resulting transformation
 *     resulting semantic representation
 *
 * This grammar provides the structural source information required for that
 * downstream record.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text
 *     lexer configuration
 *     grammar version
 *     parser configuration
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     memory availability
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *     deployment state.
 *
 * Identical source under identical parser configuration must produce
 * equivalent parse-tree structure and source spans.
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
 *     invoke a model;
 *     mutate state;
 *     modify source;
 *     invoke a compiler;
 *     invoke a runtime;
 *     access hardware;
 *     access a QPU;
 *     invoke a simulator;
 *     contact a network;
 *     inspect resources;
 *     load credentials;
 *     load plugins;
 *     bypass policy.
 *
 * The `adapt` keyword is an intent declaration, not an authorization.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST adapter should map:
 *
 *     adaptStatement
 *          ->
 *     domain-neutral AdaptationStatement
 *
 * Conceptually the node contains:
 *
 *     operation
 *     target
 *     source
 *     context
 *     source_span
 *
 * where:
 *
 *     operation = Adapt
 *
 * The AST MUST remain independent of:
 *
 *     LLVM
 *     QIR
 *     MLIR
 *     vendor IR
 *     physical topology
 *     physical qubit mapping
 *     QEC implementation
 *     calibration
 *     backend-specific hardware.
 *
 * If the existing AST uses a generic reasoning/execution operation node,
 * adaptation should be represented through that existing abstraction rather
 * than creating an incompatible parallel hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine independently:
 *
 *     whether the target exists;
 *     whether the target is adaptable;
 *     whether the source is valid;
 *     whether context values are valid;
 *     whether the adaptation is authorized;
 *     which effects are produced;
 *     which capabilities are required;
 *     which resources are required;
 *     which policies apply;
 *     which contracts apply;
 *     which provenance is required;
 *     whether the adaptation is reversible;
 *     whether the adaptation preserves program semantics;
 *     whether the adaptation preserves declared guarantees;
 *     whether the target can realize the adaptation;
 *     whether the resulting computation remains portable.
 *
 * Syntax validity is not adaptation validity.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type checking is downstream.
 *
 * The adaptation target, source and context expressions may have different
 * types.
 *
 * Semantic analysis determines whether their combination is legal.
 *
 * This grammar does not define an `Adaptable<T>` type.
 *
 * Such a type, if needed, belongs to the type/semantic system.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Adaptation effects are determined after parsing.
 *
 * The effect system may distinguish:
 *
 *     pure planning
 *     state mutation
 *     model mutation
 *     runtime control
 *     reflection
 *     code generation
 *     network
 *     native
 *     foreign
 *     distributed
 *     quantum measurement
 *     simulation
 *
 * The grammar does not assign effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability resolution is downstream.
 *
 * Example semantic requirements may include:
 *
 *     capability("adaptation")
 *     capability("model.update")
 *     capability("runtime.adaptation")
 *
 * or future capability identifiers.
 *
 * No finite capability catalogue is required by this grammar.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource analysis is downstream.
 *
 * Adaptation can be scaled according to actual available:
 *
 *     compute
 *     memory
 *     storage
 *     accelerator capacity
 *     quantum resources
 *     network capacity
 *     distributed capacity
 *
 * The source grammar remains unchanged.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The canonical path is:
 *
 *     adaptStatement
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic adaptation operation
 *          ->
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> other domain representation
 *          ->
 *     optimization
 *          ->
 *     specialization
 *          ->
 *     lowering
 *          ->
 *     routing / scheduling
 *          ->
 *     resilience
 *          ->
 *     target realization
 *
 * No `AdaptIR` is introduced by this grammar.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * If adaptation changes a quantum computation, its semantic lowering must
 * eventually enter:
 *
 *     quantum::ir
 *
 * before:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     QEC
 *     resilience
 *     ZQN
 *     HAL
 *
 * This grammar never manipulates physical qubits or quantum hardware.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * If adaptation affects HDL/hardware intent, the semantic operation is passed
 * to the HDL/hardware semantic layer.
 *
 * This grammar does not define:
 *
 *     signals
 *     fixed-width wires
 *     physical placement
 *     timing implementation
 *     synthesis implementation
 *     device mapping.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backend realization is downstream.
 *
 * An adaptation may result in:
 *
 *     recompilation
 *     respecialization
 *     rerouting
 *     rescheduling
 *     remapping
 *     backend substitution
 *     recovery
 *     model update
 *     strategy update
 *
 * but these are semantic/runtime outcomes, not parser alternatives.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Structural parser errors include:
 *
 *     adapt
 *     adapt;
 *     adapt from source;
 *     adapt with (context);
 *     adapt target from;
 *     adapt target with;
 *     adapt target with ();
 *     adapt target with (a,);
 *     adapt target with (,a);
 *     adapt target with (a,,b);
 *     adapt target with (a) from source;
 *     adapt target from source from other;
 *
 * Semantic errors are NOT parser errors.
 *
 * Examples:
 *
 *     target is not adaptable
 *     adaptation is unauthorized
 *     capability unavailable
 *     resources insufficient
 *     policy violated
 *     contract violated
 *     provenance unavailable
 *     resulting transformation invalid
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical source form introduced by this grammar is:
 *
 *     adapt TARGET;
 *
 * with optional:
 *
 *     from SOURCE
 *
 * and:
 *
 *     with (CONTEXT, ...).
 *
 * Existing expression-level forms such as:
 *
 *     mind.adapt(...)
 *
 * remain valid and independent.
 *
 * Existing runtime-level adaptation constructs remain valid through their
 * runtime grammar.
 *
 * Semantic convergence is preferred over syntactic duplication.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The universal statement composition grammar:
 *
 *     grammar/statements/statements.g4
 *
 * must import:
 *
 *     Adapt
 *
 * and admit:
 *
 *     adaptStatement
 *
 * in its `statement` rule.
 *
 * Conceptually:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | assertionStatement
 *         | ...
 *         | adaptStatement
 *         | expressionStatement
 *         ;
 *
 * The root statement grammar remains the owner of the ordering/composition.
 *
 * `adapt.g4` MUST NOT be edited merely because the ordering of statement
 * alternatives changes.
 *
 * ============================================================================
 * RUNTIME INTEGRATION CONTRACT
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/execution/runtime.g4
 *
 * owns:
 *
 *     runtimeAdaptationStatement
 *     runtimeAdaptationClause
 *
 * Those rules should eventually delegate semantically to the same adaptation
 * model as this file.
 *
 * They MUST NOT redefine the universal meaning of `adapt`.
 *
 * The runtime grammar may remain more specific because it describes runtime
 * intent.
 *
 * ============================================================================
 * MIND INTEGRATION CONTRACT
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/expressions/mind.g4
 *
 * contains expression-level forms such as:
 *
 *     mind.adapt(...)
 *
 * Those expressions remain expression-owned.
 *
 * They may lower into the same semantic AdaptationOperation used by:
 *
 *     adaptStatement
 *
 * without creating a second adaptation semantic model.
 *
 * ============================================================================
 * REASONING INTEGRATION CONTRACT
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/statements/reason.g4
 *     grammar/statements/infer.g4
 *
 * remain owners of reasoning statement syntax.
 *
 * `adapt.g4` only consumes their results through ordinary expressions.
 *
 * There must be no dependency:
 *
 *     Adapt -> Statements
 *
 * and no dependency:
 *
 *     Reason -> Adapt
 *
 * solely for syntax reuse.
 *
 * ============================================================================
 * LEARNING INTEGRATION CONTRACT
 * ============================================================================
 *
 * Learning remains independently owned.
 *
 * Adaptation may consume:
 *
 *     learning results
 *     model outputs
 *     feedback
 *     learned strategies
 *
 * through ordinary expressions.
 *
 * The semantic layer determines whether the resulting adaptation is legal.
 *
 * ============================================================================
 * POLICY INTEGRATION CONTRACT
 * ============================================================================
 *
 * Policy semantics remain outside this grammar.
 *
 * An adaptation may be constrained by:
 *
 *     authorization
 *     capability policy
 *     resource policy
 *     security policy
 *     execution policy
 *     model policy
 *     data policy
 *     quantum policy
 *     adaptation policy
 *     provenance policy
 *
 * A policy expression can be supplied through:
 *
 *     with (policy)
 *
 * without making `policy` a special adaptation keyword.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Source spans from:
 *
 *     ADAPT
 *     target
 *     source
 *     context
 *
 * must remain available to downstream tooling.
 *
 * Provenance can then record:
 *
 *     request
 *     evidence
 *     reason
 *     authorization
 *     policy
 *     transformation
 *     result
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Supported source should survive:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter/printer
 *       ->
 *     parser
 *
 * while preserving:
 *
 *     Adapt operation
 *     target
 *     source
 *     ordered context
 *     statement boundary.
 *
 * ============================================================================
 * DETERMINISTIC PARSING
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     parser configuration
 *     dialect configuration
 *
 * this grammar must produce equivalent parse-tree structure.
 *
 * It must not inspect:
 *
 *     target hardware
 *     available memory
 *     available CPUs
 *     available GPUs
 *     available QPUs
 *     network state
 *     runtime state
 *     scheduler state
 *     wall-clock time
 *     randomness.
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
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *     no unsafe Rust
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The grammar itself is target-language independent.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-level finite limit on:
 *
 *     adaptation statements
 *     target expression size
 *     source expression size
 *     context list size
 *     expression nesting
 *     program size
 *     machine size
 *     resource quantity
 *     hardware size
 *     quantum size
 *     distributed size
 *
 * Repetition is represented by grammar structure rather than machine-size
 * constants.
 *
 * Actual limits belong to:
 *
 *     compiler resources
 *     runtime resources
 *     target resources
 *     target capabilities
 *     operating environment
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     adapt strategy;
 *
 *     adapt model;
 *
 *     adapt execution_strategy from feedback;
 *
 *     adapt model from training_result;
 *
 *     adapt strategy with (policy);
 *
 *     adapt execution with (feedback);
 *
 *     adapt execution from observation with (policy);
 *
 *     adapt strategy from feedback with (policy, provenance);
 *
 *     adapt model from quantum_result with (confidence);
 *
 *     adapt strategy from distributed_result with (resource_state, policy);
 *
 *     adapt hardware_intent from simulation_result with (verification);
 *
 *     adapt tensor_strategy from performance_observation;
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     adapt
 *
 *     adapt;
 *
 *     adapt from source;
 *
 *     adapt with (context);
 *
 *     adapt target from;
 *
 *     adapt target with;
 *
 *     adapt target with ();
 *
 *     adapt target with (,);
 *
 *     adapt target with (a,);
 *
 *     adapt target with (,a);
 *
 *     adapt target with (a,,b);
 *
 *     adapt target with (a) from source;
 *
 *     adapt target from source from other;
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Target:
 *
 *     adapt identifier;
 *     adapt qualified.name;
 *     adapt object.member;
 *     adapt collection[index];
 *     adapt call(argument);
 *     adapt nested_expression;
 *
 * Source:
 *
 *     adapt target from identifier;
 *     adapt target from call(argument);
 *     adapt target from quantum_result;
 *     adapt target from measurement_result;
 *     adapt target from simulation_result;
 *     adapt target from distributed_result;
 *
 * Context:
 *
 *     adapt target with (policy);
 *     adapt target with (constraint);
 *     adapt target with (feedback, policy);
 *     adapt target with (evidence, provenance, capability);
 *
 * CROSS-DOMAIN TESTS
 * -----------------
 *
 *     adapt classical_strategy from classical_feedback;
 *
 *     adapt quantum_strategy from quantum_result;
 *
 *     adapt hybrid_strategy from measurement_result;
 *
 *     adapt hardware_strategy from simulation_result;
 *
 *     adapt distributed_strategy from distributed_feedback;
 *
 *     adapt ai_strategy from learned_model;
 *
 *     adapt execution_strategy from resource_observation;
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * The same source:
 *
 *     adapt strategy from feedback;
 *
 * must remain syntactically identical for:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     future target
 *
 * TARGET FEASIBILITY is tested downstream.
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Test increasing:
 *
 *     target expression complexity
 *     source expression complexity
 *     context count
 *     nesting depth
 *     statement count
 *     program size
 *
 * without changing this grammar.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parse identical source repeatedly and verify equivalent:
 *
 *     operation
 *     target
 *     source
 *     context order
 *     source spans
 *     statement boundary.
 *
 * COMPATIBILITY TESTS
 * -------------------
 *
 * Verify that:
 *
 *     mind.adapt(...)
 *
 * remains an expression.
 *
 * Verify that runtime adaptation remains available through:
 *
 *     runtimeAdaptationStatement
 *
 * without creating two meanings for the canonical `adapt` statement.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] The file is a parser grammar.
 *     [x] The grammar name matches the filename: Adapt.
 *     [x] tokenVocab is ZamaniLexer.
 *     [x] ADAPT comes from the canonical lexer.
 *     [x] No lexer rule is duplicated.
 *     [x] No expression hierarchy is duplicated.
 *     [x] The target is mandatory.
 *     [x] The source is optional.
 *     [x] The context is optional.
 *     [x] Source precedes context.
 *     [x] Empty context is rejected.
 *     [x] Trailing context commas are rejected.
 *     [x] Multiple context values are supported.
 *     [x] No algorithm catalogue is hard-coded.
 *     [x] No application-specific feature catalogue is hard-coded.
 *     [x] No hardware target is hard-coded.
 *     [x] No physical device identity is hard-coded.
 *     [x] No machine-size limit is hard-coded.
 *     [x] No resource-size limit is hard-coded.
 *     [x] No quantum-size limit is hard-coded.
 *     [x] No embedded Rust exists.
 *     [x] No unsafe Rust is required.
 *     [x] No runtime operation occurs during parsing.
 *     [x] AST ownership remains downstream.
 *     [x] Semantic ownership remains downstream.
 *     [x] Effect checking remains downstream.
 *     [x] Capability checking remains downstream.
 *     [x] Resource checking remains downstream.
 *     [x] Contract checking remains downstream.
 *     [x] Policy checking remains downstream.
 *     [x] Provenance remains downstream.
 *     [x] IR generation remains downstream.
 *     [x] quantum::ir remains the quantum boundary.
 *     [x] Backend selection remains downstream.
 *     [x] Runtime adaptation remains downstream.
 *
 * Repository integration gates:
 *
 *     [ ] Adapt is imported by the canonical statement composition grammar.
 *
 *     [ ] `adaptStatement` is admitted exactly once by `statement`.
 *
 *     [ ] No other grammar owns the canonical `adaptStatement`.
 *
 *     [ ] Runtime adaptation delegates semantically to the common adaptation
 *         model rather than becoming a competing language definition.
 *
 *     [ ] The AST adapter maps this syntax to the existing domain-neutral
 *         adaptation representation.
 *
 *     [ ] Semantic adaptation analysis is implemented.
 *
 *     [ ] Effect analysis is implemented.
 *
 *     [ ] Capability analysis is implemented.
 *
 *     [ ] Resource analysis is implemented.
 *
 *     [ ] Contract analysis is implemented.
 *
 *     [ ] Policy analysis is implemented.
 *
 *     [ ] Provenance analysis is implemented.
 *
 *     [ ] Canonical IR lowering is implemented.
 *
 *     [ ] Quantum adaptations reach quantum::ir where applicable.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Portability tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Round-trip tests pass.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar Adapt;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * SHARED REASONING DEPENDENCY
 * ============================================================================
 *
 * ReasonStatements already supplies the canonical expression dependency used
 * by the existing infer/reason statement architecture.
 *
 * Importing it prevents this grammar from creating another expression
 * hierarchy.
 *
 * ============================================================================
 */

import
    ReasonStatements
    ;

/*
 * ============================================================================
 * ADAPT STATEMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     adapt TARGET;
 *
 *     adapt TARGET from SOURCE;
 *
 *     adapt TARGET with (CONTEXT);
 *
 *     adapt TARGET from SOURCE with (CONTEXT, CONTEXT);
 *
 * The semicolon belongs to the statement boundary and is therefore consumed
 * here.
 * ============================================================================
 */

adaptStatement
    : ADAPT
      adaptationTarget
      adaptationSourceClause?
      adaptationContextClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * ADAPTATION TARGET
 * ============================================================================
 *
 * The target is an ordinary Zamani expression.
 *
 * The existing shared reasoning boundary is used because it ultimately
 * resolves to the canonical expression hierarchy.
 *
 * This grammar does not create a second expression grammar.
 * ============================================================================
 */

adaptationTarget
    : reasoningTarget
    ;

/*
 * ============================================================================
 * ADAPTATION SOURCE
 * ============================================================================
 *
 * The source is optional.
 *
 * It identifies information that may influence adaptation.
 * ============================================================================
 */

adaptationSourceClause
    : FROM
      adaptationTarget
    ;

/*
 * ============================================================================
 * ADAPTATION CONTEXT
 * ============================================================================
 *
 * Context is optional and ordered.
 *
 * Examples:
 *
 *     with (policy)
 *
 *     with (feedback, policy)
 *
 *     with (evidence, confidence, provenance)
 *
 * Empty context is deliberately rejected.
 * ============================================================================
 */

adaptationContextClause
    : WITH
      LPAREN
      adaptationContextList
      RPAREN
    ;

/*
 * ============================================================================
 * CONTEXT LIST
 * ============================================================================
 *
 * There is intentionally no fixed context-count limit.
 *
 * The `+` operator requires at least one context value and therefore prevents:
 *
 *     with ()
 *
 * A trailing comma is intentionally rejected.
 * ============================================================================
 */

adaptationContextList
    : adaptationContext
      (
          COMMA
          adaptationContext
      )*
    ;

/*
 * ============================================================================
 * CONTEXT VALUE
 * ============================================================================
 *
 * Context values are ordinary Zamani expressions.
 *
 * No closed adaptation vocabulary is created here.
 * ============================================================================
 */

adaptationContext
    : reasoningOption
    ;