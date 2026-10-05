/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/infer.g4
 *
 * GRAMMAR NAME
 * ------------
 * Infer
 *
 * STATUS
 * ------
 * CANONICAL INFER STATEMENT ADAPTER
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
 * This file owns the dedicated statement-level syntax for:
 *
 *     infer
 *
 * `infer` expresses portable reasoning intent.
 *
 * It does NOT implement:
 *
 *     inference algorithms
 *     theorem proving
 *     machine learning
 *     probabilistic inference
 *     causal inference
 *     knowledge storage
 *     model execution
 *     hardware execution
 *     quantum execution
 *     resource allocation
 *     target selection
 *     runtime execution
 *
 * Those responsibilities belong to downstream semantic, capability,
 * resource, policy, execution and IR subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *                              v
 *                    statement composition
 *                              |
 *                    +---------+---------+
 *                    |                   |
 *                    v                   v
 *              inferStatement       reasonStatement
 *                    |                   |
 *                    +---------+---------+
 *                              |
 *                              v
 *                       domain-neutral AST
 *                              |
 *                              v
 *                       structural validation
 *                              |
 *                              v
 *                        semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *          classical       quantum::ir       other domains
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                              v
 *                      lowering / planning
 *                              |
 *                              v
 *                   routing / scheduling
 *                              |
 *                              v
 *                    resilience / recovery
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     inferStatement
 *
 * and only the statement-level composition of:
 *
 *     INFER
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *
 * The shared reasoning operand/option structures remain owned by:
 *
 *     grammar/statements/reason.g4
 *
 * through:
 *
 *     ReasonStatements
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     statement
 *     statements
 *     expression
 *     expression precedence
 *     identifiers
 *     names
 *     types
 *     literals
 *     patterns
 *     guards
 *     reasoning algorithms
 *     knowledge representation
 *     learning algorithms
 *     adaptation algorithms
 *     uncertainty implementation
 *     probability implementation
 *     evidence storage
 *     provenance storage
 *     policies
 *     effects
 *     capabilities
 *     resources
 *     quantum operations
 *     quantum topology
 *     quantum routing
 *     QEC
 *     ZQN
 *     HDL
 *     hardware
 *     CPU selection
 *     GPU selection
 *     FPGA selection
 *     ASIC selection
 *     QPU selection
 *     runtime execution
 *     IR generation
 *
 * ============================================================================
 * SINGLE OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one universal statement rule.
 *
 * That rule is owned by:
 *
 *     grammar/statements/statements.g4
 *
 * This file MUST NOT define:
 *
 *     statement
 *
 * or:
 *
 *     statements
 *
 * The integration relationship is:
 *
 *     statement
 *         |
 *         +--> inferStatement
 *
 * ============================================================================
 * REASONING FAMILY
 * ============================================================================
 *
 * Zamani provides a unified reasoning family:
 *
 *     infer
 *     deduce
 *     reason
 *
 * They share the same structural operand model.
 *
 * This file owns:
 *
 *     infer
 *
 * The shared structures are inherited from:
 *
 *     ReasonStatements
 *
 * The future canonical division is:
 *
 *     infer.g4
 *         -> inferStatement
 *
 *     reason.g4
 *         -> deduceStatement / reasonStatement
 *
 * This avoids three unrelated reasoning syntaxes.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/statements/reason.g4
 *     grammar/expressions/expressions.g4
 *
 * INDIRECTLY CONSUMES:
 *
 *     canonical expressions
 *     canonical identifiers
 *     canonical literals
 *     canonical types
 *     canonical quantum expressions
 *     canonical data expressions
 *     canonical knowledge expressions
 *     canonical uncertainty expressions
 *     canonical policy expressions
 *     canonical provenance expressions
 *
 * EXPORTS:
 *
 *     inferStatement
 *
 * INHERITS:
 *
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * CONSUMED_BY:
 *
 *     grammar/statements/statements.g4
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     reasoning semantic subsystem
 *
 * TYPE_OWNER:
 *
 *     canonical type subsystem
 *
 * EFFECT_OWNER:
 *
 *     canonical effect subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     canonical capability subsystem
 *
 * RESOURCE_OWNER:
 *
 *     canonical resource subsystem
 *
 * CONTRACT_OWNER:
 *
 *     canonical validation/contract subsystem
 *
 * POLICY_OWNER:
 *
 *     canonical policy subsystem
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR pipeline
 *
 * TEST_OWNER:
 *
 *     grammar/tests/statements/infer/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical lexer vocabulary.
 *
 * It does NOT define lexer rules.
 *
 * Required token:
 *
 *     INFER
 *
 * Shared tokens are inherited through ReasonStatements:
 *
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * The lexer remains the single lexical authority.
 *
 * Do NOT add alternative keywords such as:
 *
 *     inference
 *     infer_from
 *     derive
 *     conclude
 *     think
 *     analyze
 *
 * merely to increase the vocabulary.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * The target, source and context options are normal Zamani expressions.
 *
 * Examples:
 *
 *     infer hypothesis;
 *
 *     infer model(input);
 *
 *     infer result from evidence;
 *
 *     infer result with (policy);
 *
 *     infer result from evidence with (confidence, policy);
 *
 * The expression grammar determines:
 *
 *     literals
 *     identifiers
 *     calls
 *     indexing
 *     member access
 *     arithmetic
 *     logical expressions
 *     collections
 *     lambdas
 *     patterns
 *     quantum expressions
 *     knowledge expressions
 *     uncertainty expressions
 *     queries
 *     policies
 *     effects
 *     compile-time expressions
 *     metaprogramming expressions
 *
 * This file MUST NOT recreate any expression rule.
 *
 * ============================================================================
 * TARGET EXPRESSION
 * ============================================================================
 *
 * The target is mandatory.
 *
 * Therefore:
 *
 *     infer target;
 *
 * is valid.
 *
 * while:
 *
 *     infer;
 *
 * is invalid.
 *
 * The grammar does not determine whether the target is semantically suitable
 * for inference.
 *
 * That decision belongs to semantic analysis.
 *
 * ============================================================================
 * SOURCE CLAUSE
 * ============================================================================
 *
 * The optional source clause identifies an expression supplying:
 *
 *     evidence
 *     premises
 *     observations
 *     knowledge
 *     model output
 *     measurements
 *     data
 *     computed values
 *     distributed results
 *     hardware observations
 *     simulation results
 *
 * Examples:
 *
 *     infer conclusion from evidence;
 *
 *     infer result from knowledge.query(pattern);
 *
 *     infer decision from measurement_result;
 *
 *     infer hardware_state from simulation_result;
 *
 * The parser does not assign domain-specific meaning to `from`.
 *
 * ============================================================================
 * CONTEXT CLAUSE
 * ============================================================================
 *
 * The optional context clause has the form:
 *
 *     with (expression, expression, ...)
 *
 * Examples:
 *
 *     infer result with (model);
 *
 *     infer result with (confidence, policy);
 *
 *     infer result from evidence with (strategy, provenance);
 *
 * Context entries remain ordinary expressions.
 *
 * This prevents the grammar from requiring a new keyword whenever a future
 * reasoning capability is introduced.
 *
 * ============================================================================
 * OPEN-ENDED SEMANTICS
 * ============================================================================
 *
 * The grammar deliberately does not enumerate reasoning algorithms.
 *
 * Therefore the following can be represented semantically without changing
 * this grammar:
 *
 *     symbolic inference
 *     statistical inference
 *     probabilistic inference
 *     causal inference
 *     deductive inference
 *     inductive inference
 *     abductive inference
 *     model-based inference
 *     neural inference
 *     neural-symbolic inference
 *     quantum-assisted inference
 *     distributed inference
 *     hardware-assisted inference
 *     future inference mechanisms
 *
 * New algorithms belong to semantic models, libraries, dialects, capabilities
 * or execution backends.
 *
 * ============================================================================
 * AI / KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * `infer` can consume expressions representing:
 *
 *     knowledge
 *     facts
 *     queries
 *     learned models
 *     evidence
 *     uncertainty
 *     probability
 *     confidence
 *     causal relationships
 *     explanations
 *     provenance
 *     policies
 *
 * No AI-specific grammar is required for every algorithm.
 *
 * The syntax expresses intent.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     infer conclusion from knowledge.query(pattern);
 *
 *     infer fact from observation;
 *
 *     infer relation from knowledge_value;
 *
 * The knowledge subsystem owns:
 *
 *     assertion
 *     retraction
 *     query
 *     fact
 *     provenance
 *
 * This file only consumes their expressions.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     infer result from observation with (confidence);
 *
 *     infer decision with (probability);
 *
 *     infer result from distribution;
 *
 * The grammar does not impose:
 *
 *     numeric precision
 *     probability representation
 *     distribution implementation
 *     confidence scale
 *
 * Those are type and semantic concerns.
 *
 * ============================================================================
 * EVIDENCE / PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Evidence and provenance are semantic metadata, not hard-coded inference
 * algorithms.
 *
 * Examples:
 *
 *     infer conclusion from evidence with (provenance);
 *
 *     infer decision with (evidence, confidence);
 *
 * Provenance must remain available to downstream:
 *
 *     diagnostics
 *     explainability
 *     reproducibility
 *     audit
 *     compiler transformations
 *     scientific workflows
 *     quantum transformations
 *     hardware decisions
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policy is represented through ordinary expressions.
 *
 * Example:
 *
 *     infer result with (policy);
 *
 * The grammar does not determine whether a policy permits the inference.
 *
 * Semantic policy analysis performs that validation.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * `infer` may appear within code subject to:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This file does not duplicate those constructs.
 *
 * Contract ownership remains in:
 *
 *     grammar/validation/
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * An inference operation may eventually carry effects such as:
 *
 *     computation
 *     IO
 *     network
 *     native
 *     foreign
 *     distributed
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *
 * Effects are determined semantically.
 *
 * This grammar does not encode an effect implementation.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * An inference operation may require capabilities such as:
 *
 *     reasoning
 *     knowledge.query
 *     model.inference
 *     probability.compute
 *     quantum.measurement
 *     tensor.compute
 *     distributed.compute
 *
 * Capabilities are resolved downstream.
 *
 * This grammar contains no hardware capability constants.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements remain symbolic and dynamic.
 *
 * The grammar MUST NOT contain:
 *
 *     maximum model size
 *     maximum evidence count
 *     maximum inference depth
 *     maximum tensor rank
 *     maximum memory
 *     maximum CPU count
 *     maximum GPU count
 *     maximum QPU count
 *     maximum node count
 *
 * Resource feasibility belongs to semantic analysis and execution planning.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * `infer` expresses portable intent.
 *
 * The same source may be considered for:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore CPUs
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
 * without changing this grammar.
 *
 * The grammar does not claim physically infinite resources.
 *
 * It only avoids imposing artificial language-level ceilings.
 *
 * Actual feasibility is determined by:
 *
 *     capabilities
 *     resources
 *     constraints
 *     policies
 *     execution environment
 *     compiler
 *     runtime
 *     target
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An infer target or source may be a quantum-derived expression.
 *
 * Examples:
 *
 *     infer result from quantum_measurement;
 *
 *     infer decision from quantum_result;
 *
 *     infer state_property from measurement_result;
 *
 * This grammar does not define quantum operations.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/expressions/quantum_expressions.g4
 *     grammar/quantum/
 *
 * The semantic boundary remains:
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
 * `infer.g4` does not create or modify quantum::ir.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * An inference target or source may refer to:
 *
 *     hardware observations
 *     simulation results
 *     verification results
 *     synthesis metadata
 *     resource models
 *     device capabilities
 *
 * Hardware realization remains downstream.
 *
 * This grammar does not encode:
 *
 *     bus widths
 *     register widths
 *     device identifiers
 *     physical addresses
 *     FPGA regions
 *     ASIC resources
 *     clock counts
 *     pipeline depths
 *     physical topology
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * `infer` may consume distributed values and results.
 *
 * Example:
 *
 *     infer result from distributed_value;
 *
 * The grammar does not encode node counts, topology sizes or placement.
 *
 * Distributed realization belongs to:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *     execution
 *     resource negotiation
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * An inference source may be produced by simulation:
 *
 *     infer result from simulation_result;
 *
 * Simulation remains an execution strategy rather than a second language.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * An inference operation may participate in an adaptive execution pipeline:
 *
 *     detect
 *     evaluate
 *     infer
 *     select
 *     fallback
 *     retry
 *     recover
 *     adapt
 *
 * This file only owns the `infer` syntax.
 *
 * Adaptation policy, authorization, effects, provenance and resource changes
 * belong downstream.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     grammar version
 *     lexer configuration
 *     parser configuration
 *     explicitly selected dialect configuration
 *
 * Parsing must NOT depend on:
 *
 *     hardware availability
 *     current memory
 *     network state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     filesystem state
 *     target selection
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parse tree must preserve enough information for the frontend to recover:
 *
 *     operation kind
 *     target
 *     optional source
 *     optional context
 *     context ordering
 *     source spans
 *     statement boundary
 *
 * This supports:
 *
 *     diagnostics
 *     formatting
 *     IDE/LSP
 *     refactoring
 *     provenance
 *     reproducible compilation
 *     incremental compilation
 *     compatibility analysis
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST adapter should produce a domain-neutral reasoning statement
 * representation containing at least:
 *
 *     operation = Infer
 *     target
 *     source?
 *     context[]
 *     source span
 *
 * This grammar does not require a new domain-specific AST merely because
 * `infer` may operate on quantum, classical, AI, HDL or distributed values.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing:
 *
 *     infer x;
 *
 * does NOT imply:
 *
 *     x is valid
 *     x is provable
 *     x is true
 *     x is executable
 *     x is authorized
 *     x is deterministic
 *     x is resource-feasible
 *     x is available on the selected target
 *
 * These are independent semantic questions.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type checking determines whether:
 *
 *     target
 *     source
 *     context
 *
 * are type-correct for the selected reasoning operation.
 *
 * This file does not impose a universal reasoning type.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Contract analysis may determine whether an inference operation satisfies:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract semantics remain outside this grammar.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policy analysis may permit, restrict or reject inference based on:
 *
 *     authorization
 *     capabilities
 *     effects
 *     resources
 *     provenance
 *     deployment context
 *     security policy
 *
 * This grammar does not encode those decisions.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Downstream provenance should be capable of recording:
 *
 *     source
 *     target
 *     evidence
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     decision
 *     version
 *
 * The parser only preserves the syntactic structure needed for that model.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces no IR.
 *
 * The downstream pipeline is:
 *
 *     inferStatement
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic reasoning operation
 *         ->
 *     canonical semantic representation
 *         |
 *         +--> classical representation
 *         |
 *         +--> quantum::ir
 *         |
 *         +--> other domain representation
 *         ->
 *     optimization
 *         ->
 *     lowering
 *         ->
 *     routing / scheduling
 *         ->
 *     execution / target realization
 *
 * ============================================================================
 * GRAMMAR DEPENDENCY
 * ============================================================================
 *
 * This grammar imports:
 *
 *     ReasonStatements
 *
 * This gives the grammar access to the shared reasoning structures:
 *
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * It does NOT import:
 *
 *     Statements
 *     Expressions directly
 *     any domain grammar
 *     quantum grammar
 *     AI grammar
 *     hardware grammar
 *
 * ReasonStatements already owns the canonical Expressions dependency.
 *
 * ============================================================================
 * CIRCULAR DEPENDENCY PROHIBITION
 * ============================================================================
 *
 * Dependency direction MUST remain:
 *
 *     Infer
 *       |
 *       v
 *     ReasonStatements
 *       |
 *       v
 *     Expressions
 *
 * and:
 *
 *     Statements
 *       |
 *       +--> Infer
 *       |
 *       +--> ReasonStatements
 *
 * Therefore:
 *
 *     Infer MUST NOT import Statements.
 *
 * ReasonStatements MUST NOT import Infer.
 *
 * This keeps the grammar dependency graph acyclic.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces the dedicated:
 *
 *     inferStatement
 *
 * entry point.
 *
 * Existing source syntax:
 *
 *     infer target;
 *
 * remains supported.
 *
 * Existing forms:
 *
 *     infer target from source;
 *
 *     infer target with (context);
 *
 *     infer target from source with (context, policy);
 *
 * remain supported.
 *
 * No new mandatory syntax is introduced.
 *
 * ============================================================================
 * LEGACY INTEGRATION
 * ============================================================================
 *
 * The repository currently has `reason.g4` owning:
 *
 *     infer
 *     deduce
 *     reason
 *
 * That arrangement must be migrated to:
 *
 *     infer.g4
 *         -> inferStatement
 *
 *     reason.g4
 *         -> deduce/reason statement family
 *
 * Shared helper rules remain in ReasonStatements.
 *
 * The migration MUST NOT create two authoritative `infer` statement rules.
 *
 * ============================================================================
 * STATEMENT-DISPATCH CONTRACT
 * ============================================================================
 *
 * The canonical statement dispatcher must eventually contain the equivalent
 * composition:
 *
 *     statement
 *         : inferStatement
 *         | reasonStatement
 *         | ...
 *         ;
 *
 * It MUST NOT contain:
 *
 *     statement
 *         : inferStatement
 *         | reasonStatement
 *         | ... another infer alternative ...
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Structural parser errors include:
 *
 *     infer
 *     infer ;
 *     infer from evidence;
 *     infer target from;
 *     infer target with;
 *     infer target with ();
 *     infer target with (a,);
 *     infer target with (,a);
 *     infer target with (a,,b);
 *     infer target from source from other;
 *     infer target with (a) from source;
 *
 * Semantic errors belong downstream:
 *
 *     infer unknown_name;
 *     infer incompatible_value;
 *     infer unavailable_model;
 *     infer result from unavailable_source;
 *     infer result with unauthorized_policy;
 *     infer result with unavailable_capability;
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimal:
 *
 *     infer hypothesis;
 *
 * Target expressions:
 *
 *     infer model(input);
 *     infer f(g(x));
 *     infer tensor[index];
 *     infer object.member;
 *     infer a + b;
 *     infer condition ? left : right;
 *
 * Source:
 *
 *     infer conclusion from evidence;
 *     infer result from observation;
 *     infer result from knowledge.query(pattern);
 *
 * Context:
 *
 *     infer result with (model);
 *     infer result with (confidence);
 *     infer result with (policy, evidence);
 *
 * Combined:
 *
 *     infer result from evidence with (model, confidence, policy);
 *
 * Cross-domain:
 *
 *     infer decision from quantum_result;
 *     infer hardware_state from simulation_result;
 *     infer tensor_result from model(input);
 *     infer distributed_result from service_result;
 *     infer classical_result from measurement_result;
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must fail structurally:
 *
 *     infer
 *
 *     infer;
 *
 *     infer from evidence;
 *
 *     infer target from;
 *
 *     infer target with;
 *
 *     infer target with ();
 *
 *     infer target with (a,);
 *
 *     infer target with (, a);
 *
 *     infer target with (a,,b);
 *
 *     infer target from source from another;
 *
 *     infer target with (context) from source;
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     infer literal;
 *     infer identifier;
 *     infer qualified.name;
 *     infer call(arg);
 *     infer collection[index];
 *     infer member.value;
 *     infer nested_expression;
 *     infer parenthesized_expression;
 *     infer quantum_result;
 *     infer knowledge_result;
 *     infer uncertainty_result;
 *     infer distributed_result;
 *     infer hardware_observation;
 *     infer simulation_result;
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test increasing:
 *
 *     target expression depth
 *     source expression depth
 *     context expression count
 *     expression size
 *     statement count
 *     program size
 *
 * without modifying this grammar.
 *
 * There is no grammar-level finite maximum.
 *
 * Any implementation/resource limit must be external to the language grammar.
 *
 * ============================================================================
 * PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same source:
 *
 *     infer result from evidence;
 *
 * must remain syntactically identical when compiled for:
 *
 *     embedded
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed system
 *     cloud
 *     future target
 *
 * Target availability must not alter parsing.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required combinations:
 *
 *     infer + classical
 *     infer + quantum
 *     infer + hybrid
 *     infer + HDL
 *     infer + hardware
 *     infer + AI
 *     infer + data
 *     infer + distributed
 *     infer + networking
 *     infer + security
 *     infer + resources
 *     infer + execution
 *     infer + contracts
 *     infer + provenance
 *     infer + policies
 *     infer + simulation
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing the same source with the same:
 *
 *     lexer version
 *     grammar version
 *     parser configuration
 *     dialect configuration
 *
 * must produce equivalent parse structure.
 *
 * No:
 *
 *     randomness
 *     hardware state
 *     network state
 *     wall-clock state
 *     filesystem state
 *
 * may affect parsing.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Supported source must survive:
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
 *     Infer operation
 *     target
 *     source
 *     ordered context
 *     statement boundary
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
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] grammar/statements/infer.g4 exists.
 *
 *     [ ] grammar name is `Infer`.
 *
 *     [ ] token vocabulary is ZamaniLexer.
 *
 *     [ ] ReasonStatements is the only grammar dependency.
 *
 *     [ ] inferStatement is the only statement rule owned here.
 *
 *     [ ] target is mandatory.
 *
 *     [ ] source is optional.
 *
 *     [ ] context is optional.
 *
 *     [ ] empty context is rejected.
 *
 *     [ ] trailing context commas are rejected.
 *
 *     [ ] source precedes context.
 *
 *     [ ] semicolon is mandatory.
 *
 *     [ ] expressions are not duplicated.
 *
 *     [ ] lexer rules are not duplicated.
 *
 *     [ ] reasoning algorithms are not hard-coded.
 *
 *     [ ] AI algorithms are not hard-coded.
 *
 *     [ ] quantum operations are not hard-coded.
 *
 *     [ ] hardware is not hard-coded.
 *
 *     [ ] machine-size limits are not hard-coded.
 *
 *     [ ] resource limits are not hard-coded.
 *
 *     [ ] capability resolution remains downstream.
 *
 *     [ ] effect resolution remains downstream.
 *
 *     [ ] contract resolution remains downstream.
 *
 *     [ ] policy resolution remains downstream.
 *
 *     [ ] provenance remains downstream.
 *
 *     [ ] IR generation remains downstream.
 *
 *     [ ] quantum::ir remains the quantum semantic boundary.
 *
 *     [ ] positive tests exist.
 *
 *     [ ] negative tests exist.
 *
 *     [ ] boundary tests exist.
 *
 *     [ ] scalability tests exist.
 *
 *     [ ] portability tests exist.
 *
 *     [ ] cross-domain tests exist.
 *
 *     [ ] determinism tests exist.
 *
 *     [ ] round-trip tests exist.
 *
 *     [ ] statement dispatcher integration exists.
 *
 *     [ ] duplicate infer ownership has been removed from the assembled
 *         statement grammar.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar Infer;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * SHARED REASONING DEPENDENCY
 * ============================================================================
 *
 * ReasonStatements supplies the common reasoning operand model:
 *
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * It also owns the canonical Expressions dependency.
 *
 * This file deliberately does not import Expressions directly because doing
 * so would duplicate the dependency already supplied by ReasonStatements.
 */
import
    ReasonStatements
    ;

/*
 * ============================================================================
 * INFER STATEMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     infer TARGET;
 *
 *     infer TARGET from SOURCE;
 *
 *     infer TARGET with (OPTION);
 *
 *     infer TARGET from SOURCE with (OPTION, OPTION);
 *
 * TARGET:
 *     mandatory
 *
 * SOURCE:
 *     optional
 *
 * CONTEXT:
 *     optional
 *
 * TERMINATOR:
 *     mandatory
 */
inferStatement
    : INFER
      reasoningTarget
      reasoningSourceClause?
      reasoningContextClause?
      SEMICOLON
    ;