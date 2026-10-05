/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/deduce.g4
 *
 * GRAMMAR NAME
 * ------------
 * Deduce
 *
 * STATUS
 * ------
 * CANONICAL DEDUCTION STATEMENT ADAPTER
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
 * This file owns the statement-level syntax for:
 *
 *     deduce
 *
 * `deduce` expresses portable deductive-reasoning intent.
 *
 * It describes WHAT the program requests.
 *
 * It does not describe HOW deduction is implemented.
 *
 * This file therefore does NOT own:
 *
 *     deduction algorithms
 *     theorem proving
 *     rule engines
 *     knowledge storage
 *     machine learning
 *     probabilistic inference
 *     causal inference
 *     model execution
 *     hardware execution
 *     quantum execution
 *     resource allocation
 *     target selection
 *     scheduling
 *     routing
 *     QEC
 *     runtime execution
 *     IR generation
 *
 * Those responsibilities belong to downstream semantic, validation,
 * capability, resource, policy, execution and IR subsystems.
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
 *                              v
 *                       deduceStatement
 *                              |
 *                              v
 *                       domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *                              v
 *                        semantic analysis
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *       classical          quantum::ir         other domains
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                              v
 *                    lowering / specialization
 *                              |
 *                              v
 *                    routing / scheduling
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
 * CORE OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     deduceStatement
 *
 * and only the statement-level `DEDUCE` adapter.
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
 *     inference algorithms
 *     knowledge representation
 *     learning
 *     adaptation
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
 *     distributed execution
 *     networking
 *     runtime execution
 *     IR generation
 *
 * ============================================================================
 * GENERIC REASONING FAMILY
 * ============================================================================
 *
 * Zamani provides one semantic reasoning family:
 *
 *     infer
 *     deduce
 *     reason
 *
 * They MUST NOT become three unrelated semantic systems.
 *
 * Conceptually:
 *
 *     infer
 *       \
 *        +--> common reasoning semantic model
 *       /
 *     deduce
 *       \
 *        +--> common reasoning semantic model
 *       /
 *     reason
 *
 * This file owns only:
 *
 *     deduceStatement
 *
 * Shared structural operands are owned by:
 *
 *     grammar/statements/reason.g4
 *
 * through the:
 *
 *     ReasonStatements
 *
 * grammar.
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
 * INDIRECT DEPENDENCIES:
 *
 *     canonical expressions
 *     canonical identifiers
 *     canonical literals
 *     canonical types
 *     canonical patterns
 *     canonical guards
 *     canonical knowledge expressions
 *     canonical uncertainty expressions
 *     canonical query expressions
 *     canonical policy expressions
 *     canonical provenance expressions
 *
 * EXPORTS:
 *
 *     deduceStatement
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
 *     grammar/tests/statements/deduce/
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
 * This file consumes the canonical lexer.
 *
 * It MUST NOT define lexer rules.
 *
 * Required token:
 *
 *     DEDUCE
 *
 * Shared reasoning tokens are inherited through ReasonStatements:
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
 * Do not introduce alternative spellings such as:
 *
 *     deduction
 *     deduct
 *     conclude
 *     derive
 *
 * merely because another language uses them.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * The target, source and context are ordinary Zamani expressions.
 *
 * Examples:
 *
 *     deduce conclusion;
 *
 *     deduce result from premises;
 *
 *     deduce result with (policy);
 *
 *     deduce result from evidence with (confidence, policy);
 *
 *     deduce decision from knowledge.query(pattern);
 *
 * The expression grammar owns:
 *
 *     literals
 *     identifiers
 *     qualified names
 *     calls
 *     indexing
 *     member access
 *     arithmetic
 *     logical operations
 *     collections
 *     lambdas
 *     patterns
 *     guards
 *     quantum expressions
 *     knowledge expressions
 *     uncertainty expressions
 *     queries
 *     policy expressions
 *     effect expressions
 *     compile-time expressions
 *     metaprogramming expressions
 *
 * This file MUST NOT recreate any expression rule.
 *
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * The deduction target is mandatory.
 *
 * Therefore:
 *
 *     deduce conclusion;
 *
 * is valid.
 *
 * While:
 *
 *     deduce;
 *
 * is invalid.
 *
 * The grammar does not determine whether the target is a valid deductive
 * conclusion. That belongs to semantic analysis.
 *
 * ============================================================================
 * SOURCE CLAUSE
 * ============================================================================
 *
 * The optional source clause identifies an expression supplying material
 * relevant to deduction.
 *
 * The source may represent:
 *
 *     premises
 *     facts
 *     evidence
 *     observations
 *     knowledge
 *     model output
 *     measurements
 *     computed values
 *     distributed results
 *     hardware observations
 *     simulation results
 *     quantum-derived values
 *
 * Examples:
 *
 *     deduce conclusion from premises;
 *
 *     deduce result from knowledge.query(pattern);
 *
 *     deduce state_property from measurement_result;
 *
 * The parser does not decide what `from` means semantically.
 *
 * ============================================================================
 * CONTEXT CLAUSE
 * ============================================================================
 *
 * The optional context clause has the form:
 *
 *     with (expression, expression, ...)
 *
 * Context expressions can represent:
 *
 *     rules
 *     policies
 *     evidence
 *     confidence
 *     strategy
 *     provenance
 *     constraints
 *     assumptions
 *     configuration
 *     domain-specific semantic context
 *
 * Example:
 *
 *     deduce result with (policy, evidence);
 *
 * No fixed context vocabulary is required.
 *
 * ============================================================================
 * OPEN-WORLD DEDUCTION
 * ============================================================================
 *
 * This grammar deliberately does not enumerate deduction mechanisms.
 *
 * It therefore remains compatible with:
 *
 *     symbolic deduction
 *     rule-based deduction
 *     theorem proving
 *     constraint deduction
 *     deductive databases
 *     model-based deduction
 *     causal deduction
 *     probabilistic deduction
 *     neural-symbolic deduction
 *     quantum-assisted deduction
 *     distributed deduction
 *     hardware-assisted deduction
 *     future deduction mechanisms
 *
 * New mechanisms belong to:
 *
 *     semantic models
 *     libraries
 *     dialects
 *     capabilities
 *     policies
 *     execution backends
 *
 * rather than requiring a new core grammar rule.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Deduction may consume knowledge expressions.
 *
 * Examples:
 *
 *     deduce conclusion from knowledge.query(pattern);
 *
 *     deduce fact from observation;
 *
 *     deduce relation from known_relation;
 *
 * Knowledge ownership remains elsewhere.
 *
 * This file does not define:
 *
 *     assert
 *     retract
 *     query
 *     fact
 *     knowledge storage
 *
 * It consumes their resulting expressions.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence is treated as semantic input.
 *
 * Examples:
 *
 *     deduce conclusion from evidence;
 *
 *     deduce decision from observations with (confidence);
 *
 *     deduce result from measurement_result with (provenance);
 *
 * Evidence representation belongs to the evidence/provenance semantic layer.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Deduction may consume:
 *
 *     probability
 *     distribution
 *     confidence
 *     belief
 *     uncertainty
 *
 * Examples:
 *
 *     deduce result from evidence with (confidence);
 *
 *     deduce decision from distribution;
 *
 * The grammar imposes no:
 *
 *     probability precision
 *     confidence scale
 *     distribution implementation
 *     numeric representation
 *
 * Those are type and semantic concerns.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Deduction may occur inside code governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This grammar does not duplicate contract syntax.
 *
 * Contract ownership remains in:
 *
 *     grammar/validation/
 *
 * Semantic validation determines whether a deduction satisfies the relevant
 * contract.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may be supplied through normal expressions.
 *
 * Example:
 *
 *     deduce result with (policy);
 *
 * The parser does not decide whether the policy permits the deduction.
 *
 * Policy validation belongs downstream.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Deduction may semantically involve effects such as:
 *
 *     computation
 *     IO
 *     network
 *     foreign
 *     native
 *     distributed
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *
 * Effects are inferred and checked downstream.
 *
 * This grammar does not encode effect implementation.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A deduction operation may require capabilities such as:
 *
 *     reasoning
 *     reasoning.deduction
 *     knowledge.query
 *     theorem.proving
 *     probability.compute
 *     tensor.compute
 *     quantum.measurement
 *     distributed.compute
 *
 * Capability names remain open-world.
 *
 * This grammar does not enumerate hardware or vendor capabilities.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Deduction may require arbitrary resources.
 *
 * Examples of semantic requirements include:
 *
 *     memory
 *     computation
 *     accelerator capability
 *     quantum resources
 *     distributed capacity
 *     network capability
 *
 * The grammar MUST NOT encode:
 *
 *     maximum deduction depth
 *     maximum premise count
 *     maximum evidence count
 *     maximum memory
 *     maximum CPU count
 *     maximum GPU count
 *     maximum QPU count
 *     maximum node count
 *     maximum tensor rank
 *
 * Actual feasibility belongs to semantic/resource analysis.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * `deduce` expresses portable intent rather than a physical implementation.
 *
 * The same source can participate in compilation for:
 *
 *     tiny systems
 *     embedded systems
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
 * without changing this grammar because the target changes.
 *
 * This does NOT claim that every target can satisfy every requirement.
 *
 * It means that physical feasibility is determined downstream by:
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
 * rather than by grammar-level limits.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A deduction target or source may be derived from quantum computation.
 *
 * Examples:
 *
 *     deduce decision from measurement_result;
 *
 *     deduce state_property from quantum_result;
 *
 *     deduce conclusion from measured_state;
 *
 * This file does not define quantum operations.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/expressions/quantum_expressions.g4
 *     grammar/quantum/
 *
 * The canonical semantic boundary remains:
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
 * This statement grammar does not create a second quantum representation.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Deduction can operate over ordinary classical expressions:
 *
 *     deduce result;
 *
 *     deduce conclusion from premises;
 *
 *     deduce value from computed_expression;
 *
 * Classical semantic analysis determines types and validity.
 *
 * ============================================================================
 * AI / MODEL INTEGRATION
 * ============================================================================
 *
 * A deduction target or source may involve learned models.
 *
 * Example:
 *
 *     deduce result from model(input);
 *
 * The grammar does not enumerate:
 *
 *     model architectures
 *     training algorithms
 *     optimization algorithms
 *     accelerator types
 *
 * Those belong to semantic models, libraries, capabilities and execution
 * backends.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Deduction may consume:
 *
 *     verification results
 *     simulation results
 *     hardware observations
 *     synthesis information
 *     resource models
 *     timing observations
 *
 * Example:
 *
 *     deduce hardware_state from simulation_result;
 *
 * Hardware realization remains downstream.
 *
 * This grammar does not encode:
 *
 *     register widths
 *     bus widths
 *     physical addresses
 *     device identifiers
 *     FPGA regions
 *     ASIC resources
 *     fixed clock counts
 *     fixed pipeline depths
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Deduction may consume distributed results:
 *
 *     deduce global_result from distributed_result;
 *
 *     deduce decision from service_result;
 *
 * Network and distributed semantics remain downstream.
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * FFI, ABI, SQL, JSON, XML and other external formats are not hard-coded into
 * this statement.
 *
 * Their resulting expressions may be supplied as targets, sources or context.
 *
 * External syntax remains owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Deduction may consume compile-time or reflective expressions where the
 * language permits them.
 *
 * This grammar does not introduce:
 *
 *     reflection keywords
 *     code generation syntax
 *     type-level computation syntax
 *
 * Those remain owned by the metaprogramming subsystem.
 *
 * ============================================================================
 * SOURCE SPANS / PROVENANCE
 * ============================================================================
 *
 * The frontend AST must preserve enough source information to identify:
 *
 *     DEDUCE
 *     target
 *     source clause
 *     context clause
 *     individual context expressions
 *     terminating semicolon
 *
 * This information supports:
 *
 *     diagnostics
 *     formatting
 *     IDE/LSP
 *     refactoring
 *     reproducible compilation
 *     provenance
 *     audit
 *     explanation
 *
 * This grammar does not define concrete Rust AST structures.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual AST:
 *
 *     DeduceStatement
 *       target
 *       source?
 *       context[]
 *       source_span
 *
 * The exact Rust AST type is owned by the frontend AST subsystem.
 *
 * The AST must remain domain-neutral.
 *
 * It must not contain:
 *
 *     physical qubit IDs
 *     GPU IDs
 *     CPU IDs
 *     FPGA coordinates
 *     vendor topology
 *     QEC layout
 *     routing decisions
 *     calibration data
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     target validity
 *     source validity
 *     context validity
 *     name resolution
 *     type compatibility
 *     logical consistency
 *     evidence validity
 *     contract validity
 *     policy validity
 *     effect legality
 *     capability requirements
 *     resource requirements
 *     provenance
 *
 * The parser MUST NOT decide whether deduction is logically correct.
 *
 * A syntactically valid:
 *
 *     deduce result from evidence;
 *
 * may still fail semantic validation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * `deduceStatement` does not create IR directly.
 *
 * The semantic representation is lowered through the repository's canonical
 * semantic/IR pipeline.
 *
 * Classical deduction may lower through the classical representation.
 *
 * Quantum-derived deduction may consume values produced through:
 *
 *     quantum::ir
 *
 * HDL/hardware deduction may consume canonical hardware semantic data.
 *
 * There is no:
 *
 *     DeduceIR
 *     DeductionIR
 *     QuantumDeductionIR
 *
 * introduced by this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural parser errors include:
 *
 *     deduce
 *     deduce;
 *     deduce from evidence;
 *     deduce target from;
 *     deduce target with;
 *     deduce target with ();
 *     deduce target with (a,);
 *     deduce target with (,a);
 *     deduce target with (a,,b);
 *     deduce target from source from other;
 *     deduce target with (context) from source;
 *
 * Semantic errors include:
 *
 *     deduce unknown_name;
 *     deduce incompatible_value;
 *     deduce result from unavailable_source;
 *     deduce result with unauthorized_policy;
 *     deduce result with unavailable_capability;
 *
 * Semantic errors MUST remain downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical source forms:
 *
 *     deduce target;
 *
 *     deduce target from source;
 *
 *     deduce target with (context);
 *
 *     deduce target from source with (context);
 *
 * remain supported.
 *
 * No physical target information is introduced into the syntax.
 *
 * ============================================================================
 * INTEGRATION WITH reason.g4
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The current repository's reason.g4 contains:
 *
 *     reasoningOperator
 *         : INFER
 *         | DEDUCE
 *         | REASON
 *         ;
 *
 * That must be corrected during integration.
 *
 * `deduce.g4` is the sole owner of:
 *
 *     deduceStatement
 *
 * Therefore the assembled statement grammar must not expose another public
 * `deduceStatement`.
 *
 * The shared ReasonStatements grammar should remain the structural source for:
 *
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * The canonical ownership after integration is:
 *
 *     infer.g4
 *         -> inferStatement
 *
 *     deduce.g4
 *         -> deduceStatement
 *
 *     reason.g4
 *         -> reasonStatement
 *
 *     reason.g4
 *         -> shared reasoning structures
 *
 * The `reasoningOperator` rule must therefore no longer make DEDUCE reachable
 * through `reasonStatement`.
 *
 * ============================================================================
 * INTEGRATION WITH statements.g4
 * ============================================================================
 *
 * `grammar/statements/statements.g4` is the sole universal statement owner.
 *
 * It must import:
 *
 *     Deduce
 *
 * and its `statement` rule must eventually admit:
 *
 *     deduceStatement
 *
 * exactly once.
 *
 * Conceptually:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | assertionStatement
 *         | ...
 *         | inferStatement
 *         | deduceStatement
 *         | reasonStatement
 *         | ...
 *         | expressionStatement
 *         ;
 *
 * `statements.g4` owns the universal dispatcher.
 *
 * `deduce.g4` does not.
 *
 * ============================================================================
 * INTEGRATION WITH infer.g4
 * ============================================================================
 *
 * `infer.g4` and `deduce.g4` are sibling adapters.
 *
 * They share:
 *
 *     ReasonStatements
 *
 * but neither imports the other.
 *
 * Dependency direction:
 *
 *     ReasonStatements
 *          ^
 *          |
 *     +----+----+
 *     |         |
 *   Infer     Deduce
 *     |         |
 *     +----+----+
 *          |
 *      Statements
 *
 * There must be no:
 *
 *     Infer -> Deduce
 *     Deduce -> Infer
 *
 * dependency.
 *
 * ============================================================================
 * INTEGRATION WITH expressions.g4
 * ============================================================================
 *
 * This file does not import the expression grammar directly because
 * ReasonStatements already owns the expression dependency.
 *
 * The dependency remains:
 *
 *     Deduce
 *       ->
 *     ReasonStatements
 *       ->
 *     Expressions
 *
 * This avoids duplicate imports and competing expression ownership.
 *
 * ============================================================================
 * INTEGRATION WITH VALIDATION
 * ============================================================================
 *
 * Deduction may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Validation owns those constructs.
 *
 * Deduce merely provides the statement node that semantic validation examines.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Resource analysis may attach requirements to a deduction operation.
 *
 * Examples of semantic requirements:
 *
 *     capability("reasoning.deduction")
 *     capability("knowledge.query")
 *     memory >= required_memory
 *     compute >= required_compute
 *
 * These are semantic/resource constructs.
 *
 * This grammar does not enumerate or limit them.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * Semantic analysis may associate deduction with effects.
 *
 * Examples:
 *
 *     computation
 *     network
 *     distributed
 *     foreign
 *     measurement
 *     randomness
 *     simulation
 *
 * Effects remain owned by the effects subsystem.
 *
 * ============================================================================
 * INTEGRATION WITH POLICIES
 * ============================================================================
 *
 * Policies may constrain deduction:
 *
 *     permission
 *     prohibition
 *     requirement
 *     preference
 *     fallback
 *     security rule
 *
 * Policy analysis occurs after parsing.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Deduction is especially important for provenance.
 *
 * The semantic representation should be able to record:
 *
 *     source
 *     premises
 *     evidence
 *     derivation
 *     decision
 *     transformation
 *     verification
 *     version
 *
 * This grammar only preserves the source structure necessary for that
 * downstream model.
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
 * must produce equivalent parse-tree structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     resource availability
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     runtime state
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar has no language-level finite limit for:
 *
 *     target expression size
 *     source expression size
 *     context count
 *     context expression size
 *     statement count
 *     program size
 *     deduction complexity
 *     hardware scale
 *     resource scale
 *     quantum scale
 *     distributed scale
 *
 * Actual parser/compiler limits are implementation/resource limits, not
 * language semantics.
 *
 * No grammar-level:
 *
 *     MAX_*
 *
 * constants are permitted.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 * It also contains no fixed enumeration of:
 *
 *     processors
 *     accelerators
 *     QPUs
 *     vendors
 *     machines
 *     nodes
 *     memories
 *     devices
 *     quantum gates
 *     deduction algorithms
 *     model architectures
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The same:
 *
 *     deduce target from source;
 *
 * must parse identically regardless of whether later compilation targets:
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
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a deduction statement MUST NOT:
 *
 *     execute deduction
 *     execute a model
 *     query hardware
 *     access the filesystem
 *     access the network
 *     invoke an external process
 *     allocate hardware
 *     invoke FFI
 *     modify program state
 *
 * All such operations belong to explicitly controlled downstream stages.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no embedded Rust
 *     no unsafe code
 *     no semantic predicates
 *     no runtime callbacks
 *     no I/O
 *     no hardware access
 *
 * The generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must require no unsafe Rust.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     deduce conclusion;
 *
 *     deduce result from premises;
 *
 *     deduce result with (policy);
 *
 *     deduce result from evidence with (confidence, policy);
 *
 *     deduce decision from knowledge.query(pattern);
 *
 *     deduce state_property from measurement_result;
 *
 *     deduce hardware_state from simulation_result;
 *
 *     deduce distributed_result from service_result;
 *
 *     deduce result from model(input) with (evidence, provenance);
 *
 * NEGATIVE
 * --------
 *
 *     deduce
 *
 *     deduce;
 *
 *     deduce from evidence;
 *
 *     deduce target from;
 *
 *     deduce target with;
 *
 *     deduce target with ();
 *
 *     deduce target with (a,);
 *
 *     deduce target with (,a);
 *
 *     deduce target with (a,,b);
 *
 *     deduce target from source from other;
 *
 *     deduce target with (context) from source;
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 *     deduce unknown_name;
 *
 *     deduce incompatible_value;
 *
 *     deduce result from unavailable_source;
 *
 *     deduce result with unavailable_policy;
 *
 *     deduce result with unavailable_capability;
 *
 * These require semantic validation rather than parser rejection.
 *
 * BOUNDARY
 * --------
 *
 * Test deduction with:
 *
 *     literals
 *     identifiers
 *     qualified identifiers
 *     calls
 *     indexing
 *     member access
 *     arithmetic
 *     logical expressions
 *     conditional expressions
 *     collections
 *     lambdas
 *     knowledge queries
 *     uncertainty expressions
 *     policy expressions
 *     quantum-derived expressions
 *     hardware observations
 *     simulation results
 *     distributed results
 *     network results
 *
 * SCALABILITY
 * ----------
 *
 * Increase:
 *
 *     target expression depth
 *     source expression depth
 *     context count
 *     context expression depth
 *     statement count
 *     program size
 *
 * without changing this grammar.
 *
 * DETERMINISM
 * -----------
 *
 * Repeatedly parse identical input with identical parser configuration and
 * verify equivalent parse-tree structure.
 *
 * PORTABILITY
 * -----------
 *
 * Compile the same source against different target capability descriptions
 * without changing the source grammar.
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Required combinations:
 *
 *     deduction + classical
 *     deduction + quantum
 *     deduction + hybrid
 *     deduction + HDL
 *     deduction + hardware
 *     deduction + AI
 *     deduction + data
 *     deduction + distributed
 *     deduction + networking
 *     deduction + security
 *     deduction + resources
 *     deduction + execution
 *     deduction + contracts
 *     deduction + provenance
 *     deduction + policies
 *     deduction + simulation
 *
 * ROUND-TRIP
 * ----------
 *
 * Supported syntax must survive:
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
 *     operation
 *     target
 *     source
 *     ordered context
 *     statement boundary
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar.
 *
 * Canonical grammar name:
 *
 *     Deduce
 *
 * Canonical filename:
 *
 *     deduce.g4
 *
 * Therefore the declaration is:
 *
 *     parser grammar Deduce;
 *
 * The grammar is imported by the universal statement composition layer.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] grammar/statements/deduce.g4 exists.
 *
 *     [ ] grammar identity is Deduce.
 *
 *     [ ] token vocabulary is ZamaniLexer.
 *
 *     [ ] ReasonStatements is the only direct grammar dependency.
 *
 *     [ ] deduceStatement is the only statement rule owned here.
 *
 *     [ ] target is mandatory.
 *
 *     [ ] source is optional.
 *
 *     [ ] context is optional.
 *
 *     [ ] source precedes context.
 *
 *     [ ] empty context is rejected.
 *
 *     [ ] trailing context commas are rejected.
 *
 *     [ ] semicolon is mandatory.
 *
 *     [ ] expression syntax is not duplicated.
 *
 *     [ ] lexer syntax is not duplicated.
 *
 *     [ ] reasoning algorithms are not hard-coded.
 *
 *     [ ] knowledge algorithms are not hard-coded.
 *
 *     [ ] learning algorithms are not hard-coded.
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
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *
 *     [ ] no executable grammar actions exist.
 *
 *     [ ] no semantic predicates exist.
 *
 *     [ ] no unsafe Rust is required.
 *
 *     [ ] statements.g4 imports Deduce.
 *
 *     [ ] statements.g4 admits deduceStatement exactly once.
 *
 *     [ ] reason.g4 no longer provides a competing DEDUCE statement path.
 *
 *     [ ] infer.g4 and deduce.g4 remain sibling adapters.
 *
 *     [ ] positive tests exist.
 *
 *     [ ] negative tests exist.
 *
 *     [ ] semantic-negative tests exist.
 *
 *     [ ] boundary tests exist.
 *
 *     [ ] scalability tests exist.
 *
 *     [ ] portability tests exist.
 *
 *     [ ] determinism tests exist.
 *
 *     [ ] round-trip tests exist.
 *
 *     [ ] cross-domain tests exist.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar Deduce;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * SHARED REASONING DEPENDENCY
 * ============================================================================
 *
 * ReasonStatements owns the shared structural reasoning vocabulary:
 *
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * It also owns the canonical Expressions dependency.
 *
 * Do not import Expressions directly here.
 *
 * This keeps the dependency graph:
 *
 *     Deduce
 *       |
 *       v
 *     ReasonStatements
 *       |
 *       v
 *     Expressions
 *
 * and prevents duplicate expression ownership.
 */
import
    ReasonStatements
    ;

/*
 * ============================================================================
 * PUBLIC STATEMENT ENTRY
 * ============================================================================
 *
 * Canonical forms:
 *
 *     deduce TARGET;
 *
 *     deduce TARGET from SOURCE;
 *
 *     deduce TARGET with (OPTION);
 *
 *     deduce TARGET from SOURCE with (OPTION, OPTION);
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
deduceStatement
    : DEDUCE
      reasoningTarget
      reasoningSourceClause?
      reasoningContextClause?
      SEMICOLON
    ;