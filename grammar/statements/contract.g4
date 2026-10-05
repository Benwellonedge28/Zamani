/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/contract.g4
 *
 * GRAMMAR
 * -------
 * ContractStatements
 *
 * STATUS
 * ------
 * Production parser component
 *
 * PURPOSE
 * -------
 * Owns standalone source-level contract-obligation statements:
 *
 *     requires(...)
 *     ensures(...)
 *     invariant(...)
 *     assume(...)
 *     guarantee(...)
 *     property(...)
 *
 * These constructs express portable semantic intent.
 *
 * They do NOT perform verification, resource allocation, hardware selection,
 * execution, proof search, scheduling, routing, quantum mapping, synthesis,
 * or runtime evaluation.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no target-specific actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no mutable global state;
 *     - no resource allocation;
 *     - no target selection;
 *     - no backend selection;
 *     - no machine-capacity constants.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide one parser-level ownership boundary for standalone semantic
 * obligations and properties.
 *
 * OWNS
 * ----
 *
 *     contractStatement
 *     requiresStatement
 *     ensuresStatement
 *     invariantStatement
 *     assumeStatement
 *     guaranteeStatement
 *     propertyStatement
 *     contractCondition
 *
 * DOES NOT OWN
 * -------------
 *
 *     lexer rules
 *     keyword spellings
 *     punctuation spellings
 *     identifiers
 *     names
 *     expressions
 *     types
 *     function contracts
 *     declarations
 *     resource allocation
 *     capability resolution
 *     effect implementation
 *     policy evaluation
 *     proof execution
 *     theorem proving
 *     runtime verification
 *     hardware selection
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     backend implementation
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/lexer/keywords.g4
 *         REQUIRES
 *         ENSURES
 *         INVARIANT
 *         ASSUME
 *         GUARANTEE
 *         PROPERTY
 *
 *     grammar/core/punctuation.g4
 *         statementTerminator
 *
 *     grammar/expressions/expressions.g4
 *         expression
 *
 * EXPORTS
 * -------
 *
 *     contractStatement
 *
 *     The subordinate statement rules are intentionally exposed as named
 *     parser contexts for AST construction and diagnostics.
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/statements/statements.g4
 *
 * AST_OWNER
 * ---------
 *
 *     frontend AST subsystem under src/frontend/ast/
 *
 * This grammar creates parser contexts only. It does not create Rust AST
 * structures.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     semantic validation / contract semantic model
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic model followed by the applicable domain IR
 *
 * Quantum semantic lowering, where applicable:
 *
 *     quantum::ir
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/statements/contract/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/contracts.md
 *     grammar/spec/contracts.md
 *
 * ============================================================================
 * ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * This file and grammar/functions/contracts.g4 have deliberately different
 * responsibilities.
 *
 * FUNCTION CONTRACTS
 * ------------------
 *
 * grammar/functions/contracts.g4 owns:
 *
 *     contract {
 *         requires(...);
 *         ensures(...);
 *         invariant(...);
 *     }
 *
 * when that block is attached to a callable/function declaration.
 *
 * STANDALONE CONTRACT STATEMENTS
 * ------------------------------
 *
 * This file owns:
 *
 *     requires(...);
 *     ensures(...);
 *     invariant(...);
 *     assume(...);
 *     guarantee(...);
 *     property(...);
 *
 * when they occur as ordinary statements.
 *
 * This distinction prevents two competing owners for the `contract` block
 * syntax while still allowing the same semantic contract model to consume
 * both forms.
 *
 * ============================================================================
 * SEMANTIC MODEL
 * ============================================================================
 *
 * All six statement forms ultimately represent a domain-neutral semantic
 * contract obligation.
 *
 * Conceptually:
 *
 *     ContractObligation
 *     {
 *         kind
 *         condition
 *         source_span
 *         enclosing_scope
 *         semantic_context
 *         provenance
 *     }
 *
 * The exact Rust representation belongs to the AST/semantic subsystems.
 *
 * The grammar MUST NOT introduce a target-specific AST such as:
 *
 *     QuantumContract
 *     GPUContract
 *     FPGAContract
 *     AIContract
 *     HDLContract
 *     DistributedContract
 *
 * ============================================================================
 * CONTRACT KINDS
 * ============================================================================
 *
 * requires
 * --------
 *
 * States a condition that must hold before the relevant semantic boundary.
 *
 * Examples:
 *
 *     requires(x >= 0);
 *     requires(capability("quantum.measurement"));
 *     requires(memory >= required_memory);
 *
 * The grammar does not decide whether the expression represents:
 *
 *     a logical precondition;
 *     a resource requirement;
 *     a capability requirement;
 *     a type-state requirement;
 *     an authorization requirement;
 *     a hardware requirement.
 *
 * Semantic analysis decides this.
 *
 * ensures
 * -------
 *
 * States a condition associated with successful completion of the enclosing
 * semantic operation.
 *
 * Examples:
 *
 *     ensures(result >= 0);
 *     ensures(output.is_valid());
 *
 * invariant
 * ---------
 *
 * States a condition that must remain valid according to the enclosing
 * semantic context.
 *
 * The grammar does not decide when or how frequently an invariant is checked.
 *
 * assume
 * ------
 *
 * Introduces an explicit assumption for the relevant semantic scope.
 *
 * An assumption is not automatically proof.
 *
 * Semantic and verification layers determine how it may be relied upon.
 *
 * guarantee
 * ---------
 *
 * States a semantic guarantee supplied by the enclosing construct or
 * computation.
 *
 * A guarantee must not be fabricated by the compiler merely because a target
 * happens to be stronger than required.
 *
 * property
 * --------
 *
 * Declares a semantic property associated with the current context.
 *
 * A property is intentionally generic so that future domains can attach
 * meaning through semantic models and dialects without requiring a new core
 * keyword for every application.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Every condition is an ordinary Zamani expression.
 *
 * This grammar does NOT create:
 *
 *     contractExpression
 *     quantumContractExpression
 *     resourceContractExpression
 *     hardwareContractExpression
 *     AIContractExpression
 *
 * Instead:
 *
 *     contractCondition
 *         -> expression
 *
 * This keeps one expression authority across the language.
 *
 * Expressions may therefore refer to:
 *
 *     values
 *     variables
 *     functions
 *     types where permitted
 *     capabilities
 *     resources
 *     data
 *     tensor values
 *     quantum-derived values
 *     measurement results
 *     hardware state
 *     distributed state
 *     AI/model values
 *     provenance
 *     policies
 *     other semantic entities
 *
 * Semantic analysis determines validity.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar imposes no concrete condition type.
 *
 * Semantic analysis determines whether a condition is valid for its contract
 * kind and enclosing context.
 *
 * Examples:
 *
 *     requires(x >= 0);
 *     ensures(result.is_valid());
 *     invariant(state.is_consistent());
 *
 * may require predicate-like semantics.
 *
 * A future contract category may have richer semantic typing without requiring
 * a grammar rewrite.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a contract does not execute its expression.
 *
 * Evaluation effects, if any, are determined downstream.
 *
 * Contract expressions therefore participate in the existing effect system:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * The grammar does not hard-code those effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A contract may contain capability requirements.
 *
 * Example:
 *
 *     requires(capability("quantum.measurement"));
 *
 * The parser merely accepts the expression.
 *
 * Capability resolution belongs downstream:
 *
 *     expression
 *       ->
 *     semantic analysis
 *       ->
 *     capability resolution
 *
 * No physical device is selected by this grammar.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions remain expressions.
 *
 * Examples:
 *
 *     requires(memory >= required_memory);
 *     requires(qubits >= required_qubits);
 *     requires(topology.supports(required_topology));
 *
 * These are semantic requirements, not grammar-level machine limits.
 *
 * The grammar therefore contains no constants describing:
 *
 *     memory capacity
 *     qubit capacity
 *     CPU count
 *     GPU count
 *     FPGA count
 *     node count
 *     device count
 *     thread count
 *     tensor rank
 *     register width
 *     network size
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Contract validity may be affected by policies.
 *
 * Policy evaluation is downstream.
 *
 * Examples include policies governing:
 *
 *     verification
 *     execution
 *     security
 *     adaptation
 *     simulation
 *     resource negotiation
 *     deployment
 *     reproducibility
 *
 * This grammar does not evaluate policies.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The AST/semantic model must preserve source provenance sufficient to
 * associate each contract statement with:
 *
 *     source span
 *     source file/module
 *     enclosing declaration or execution scope
 *     contract kind
 *     condition
 *     semantic derivation
 *
 * Later compiler transformations may additionally record:
 *
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *
 * Provenance implementation is not owned here.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The pipeline remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     semantic contract model
 *       ->
 *     type/effect/capability/resource analysis
 *       ->
 *     policy analysis
 *       ->
 *     provenance
 *       ->
 *     canonical semantic representation
 *       ->
 *     applicable domain IR
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     target realization
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A contract expression may depend on quantum semantics.
 *
 * Example:
 *
 *     requires(capability("quantum.measurement"));
 *     ensures(measurement.is_valid());
 *
 * The contract grammar does not create a quantum representation.
 *
 * When the surrounding computation is quantum, the downstream path remains:
 *
 *     AST
 *       ->
 *     quantum semantic analysis
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
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * This grammar MUST NOT:
 *
 *     - enumerate quantum operations;
 *     - allocate physical qubits;
 *     - encode topology;
 *     - encode calibration;
 *     - implement QEC;
 *     - create another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Contract expressions may describe HDL or hardware properties.
 *
 * Example:
 *
 *     requires(signal.is_defined(clock));
 *     ensures(output.is_valid());
 *
 * This file does not encode:
 *
 *     fixed signal widths
 *     fixed registers
 *     fixed memories
 *     fixed FPGA dimensions
 *     fixed ASIC resources
 *     fixed clock counts
 *     fixed device identifiers
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backend realization may:
 *
 *     verify a contract statically;
 *     generate runtime checks;
 *     discharge a proof obligation;
 *     propagate a guarantee;
 *     reject an unsupported requirement;
 *     preserve a property through lowering.
 *
 * None of those behaviors belong to this grammar.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntactic diagnostics include:
 *
 *     requires;
 *     requires();
 *     requires(;
 *     requires(, condition);
 *     requires(condition;
 *     requires(condition,);
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property;
 *     requires(condition) trailing;
 *
 * Semantic diagnostics include:
 *
 *     unknown name
 *     invalid expression
 *     invalid contract context
 *     invalid condition type
 *     unavailable capability
 *     unavailable resource
 *     policy violation
 *     invalid provenance
 *     unsupported verification obligation
 *
 * Semantic diagnostics are NOT parser errors merely because the source is
 * structurally valid.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Canonical standalone forms are:
 *
 *     requires(condition);
 *     ensures(condition);
 *     invariant(condition);
 *     assume(condition);
 *     guarantee(condition);
 *     property(condition);
 *
 * Existing function-contract syntax remains owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * This file MUST NOT reinterpret:
 *
 *     contract { ... }
 *
 * as a standalone statement.
 *
 * Any legacy standalone spelling must be explicitly classified through:
 *
 *     grammar/compatibility/
 *
 * before being removed or changed.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-defined maximum for:
 *
 *     contract statements
 *     expression size
 *     expression nesting
 *     source-unit size
 *     program size
 *     number of functions
 *     number of modules
 *     number of quantum operations
 *     number of qubits
 *     number of processors
 *     number of devices
 *     number of nodes
 *     amount of memory
 *     tensor rank
 *     network size
 *
 * Repetition is structural and unbounded at the language level.
 *
 * "Infinity" means that this grammar introduces no artificial finite capacity
 * ceiling. Actual execution remains bounded by available implementation,
 * compiler, runtime, target, and physical resources.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexer vocabulary
 *     parser grammar
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     runtime state
 *     target selection
 *     scheduler state
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no executable implementation.
 *
 * It cannot:
 *
 *     execute expressions;
 *     access files;
 *     access networks;
 *     access secrets;
 *     discover hardware;
 *     allocate resources;
 *     select devices.
 *
 * The consuming Zamani frontend must remain compatible with Rust 1.97 /
 * Rust 1.97.1 and safe Rust.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     requires(x > 0);
 *     ensures(result >= 0);
 *     invariant(state.is_valid());
 *     assume(input.is_normalized());
 *     guarantee(output.is_valid());
 *     property(result.is_consistent());
 *
 * Cross-domain:
 *
 *     requires(capability("quantum.measurement"));
 *     requires(memory >= required_memory);
 *     requires(tensor.shape == expected_shape);
 *     ensures(measurement.is_valid());
 *     invariant(hardware_state.is_consistent());
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     requires;
 *     requires();
 *     requires(;
 *     requires(, x);
 *     requires(x;
 *     requires(x,);
 *     requires(x, y);
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property;
 *     requires(x) trailing;
 *
 * BOUNDARY TESTS
 * --------------
 *
 *     requires((deeply_nested_expression));
 *     ensures(large_expression);
 *     invariant(quantum_derived_condition);
 *     assume(distributed_condition);
 *     guarantee(hardware_condition);
 *     property(hybrid_condition);
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generate programs with increasing:
 *
 *     contract statement count
 *     expression size
 *     expression nesting
 *     module size
 *     source-unit size
 *
 * without modifying this grammar.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Verify the same contract syntax around:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/model computation
 *     data processing
 *     distributed computation
 *     networking
 *     accelerators
 *     scientific computation
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Repeated parsing of identical source under identical configuration must
 * produce equivalent parse-tree structure and source spans.
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * The same contract source must remain syntactically identical when target
 * descriptions, resource availability, processor counts, device counts, or
 * hardware topology change.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST NOT contain universal capacity constants.
 *
 * In particular, it contains no:
 *
 *     MAX_CONTRACTS
 *     MAX_PRECONDITIONS
 *     MAX_POSTCONDITIONS
 *     MAX_INVARIANTS
 *     MAX_PROPERTIES
 *     MAX_EXPRESSION_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_REGISTER_WIDTH
 *
 * Numeric literals inside expressions remain program values and are never
 * language-capacity declarations.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 * --------
 *
 * No lexer modification is required because the canonical keyword grammar
 * already owns:
 *
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     PROPERTY
 *
 * 2. EXPRESSIONS
 * --------------
 *
 * `expression` is imported from:
 *
 *     grammar/expressions/
 *
 * No expression syntax is duplicated here.
 *
 * 3. PUNCTUATION
 * -------------
 *
 * `statementTerminator` is imported from:
 *
 *     grammar/core/punctuation.g4
 *
 * This keeps statement termination under one owner.
 *
 * 4. STATEMENT DISPATCH
 * ---------------------
 *
 * `grammar/statements/statements.g4` must import:
 *
 *     ContractStatements
 *
 * and add:
 *
 *     | contractStatement
 *
 * to the universal `statement` rule.
 *
 * 5. FUNCTION CONTRACTS
 * ---------------------
 *
 * `grammar/functions/contracts.g4` remains unchanged by this grammar.
 *
 * It owns:
 *
 *     contract { ... }
 *
 * for function declarations.
 *
 * Both grammars feed the same downstream semantic contract model.
 *
 * 6. VALIDATION
 * ------------
 *
 * Structural validation consumes the parser contexts and verifies:
 *
 *     condition structure
 *     context legality
 *     contract-kind legality
 *     scope
 *     expression typing
 *
 * 7. EFFECTS
 * ---------
 *
 * Contract expressions participate in the existing effect analysis.
 *
 * 8. CAPABILITIES / RESOURCES
 * ---------------------------
 *
 * Requirements and capabilities are resolved semantically.
 *
 * This grammar never selects hardware.
 *
 * 9. POLICIES
 * ----------
 *
 * Policy evaluation remains downstream.
 *
 * 10. PROVENANCE
 * -------------
 *
 * Source spans and semantic derivation remain available to provenance.
 *
 * 11. IR
 * -----
 *
 * This file creates no IR.
 *
 * Contract obligations are lowered through the canonical semantic model.
 *
 * 12. QUANTUM
 * ----------
 *
 * Quantum-related contract expressions continue through:
 *
 *     semantic quantum model -> quantum::ir
 *
 * 13. HDL / HARDWARE
 * -----------------
 *
 * HDL and hardware contract expressions remain target independent until
 * semantic lowering.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] The grammar name is `ContractStatements`.
 *
 * [ ] `tokenVocab = ZamaniLexer` is present.
 *
 * [ ] `Expressions` is imported.
 *
 * [ ] `Punctuation` is imported.
 *
 * [ ] `contractStatement` is the sole statement-level contract dispatcher.
 *
 * [ ] Each contract kind has exactly one parser owner here.
 *
 * [ ] Function contract blocks remain owned by FunctionContracts.
 *
 * [ ] No lexer rule is defined here.
 *
 * [ ] No expression grammar is duplicated here.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No embedded Rust is used.
 *
 * [ ] No resource/hardware capacity is encoded.
 *
 * [ ] No target selection is encoded.
 *
 * [ ] No quantum gate catalogue is encoded.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] Standalone contract statements require canonical termination.
 *
 * [ ] Cross-domain expressions are accepted structurally.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] AST integration is implemented downstream.
 *
 * [ ] Semantic contract integration is implemented downstream.
 *
 * [ ] Rust frontend integration passes on Rust 1.97 / 1.97.1.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar ContractStatements;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Punctuation
    ;


/*
 * ============================================================================
 * UNIVERSAL CONTRACT STATEMENT
 * ============================================================================
 *
 * Exactly one public statement-level dispatcher exists in this file.
 *
 * The dispatcher preserves the concrete contract kind so AST construction and
 * diagnostics can distinguish the six semantic forms.
 */
contractStatement
    : requiresStatement
    | ensuresStatement
    | invariantStatement
    | assumeStatement
    | guaranteeStatement
    | propertyStatement
    ;


/*
 * ============================================================================
 * REQUIRES
 * ============================================================================
 *
 *     requires(condition);
 *
 * The condition is an ordinary Zamani expression.
 */
requiresStatement
    : REQUIRES contractCondition statementTerminator
    ;


/*
 * ============================================================================
 * ENSURES
 * ============================================================================
 *
 *     ensures(condition);
 */
ensuresStatement
    : ENSURES contractCondition statementTerminator
    ;


/*
 * ============================================================================
 * INVARIANT
 * ============================================================================
 *
 *     invariant(condition);
 */
invariantStatement
    : INVARIANT contractCondition statementTerminator
    ;


/*
 * ============================================================================
 * ASSUME
 * ============================================================================
 *
 *     assume(condition);
 *
 * An assumption is semantic input to the contract/verification model. It is
 * not automatically a proof.
 */
assumeStatement
    : ASSUME contractCondition statementTerminator
    ;


/*
 * ============================================================================
 * GUARANTEE
 * ============================================================================
 *
 *     guarantee(condition);
 *
 * A guarantee is a semantic promise and must be validated by the downstream
 * semantic/verification model.
 */
guaranteeStatement
    : GUARANTEE contractCondition statementTerminator
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 *     property(condition);
 *
 * Property semantics are intentionally generic and domain neutral.
 */
propertyStatement
    : PROPERTY contractCondition statementTerminator
    ;


/*
 * ============================================================================
 * CONTRACT CONDITION
 * ============================================================================
 *
 * Parentheses are part of the canonical standalone contract syntax.
 *
 * The inner expression remains owned by Expressions.
 *
 * This rule deliberately prevents a contract statement from consuming
 * unrelated following source tokens.
 */
contractCondition
    : LPAREN expression RPAREN
    ;