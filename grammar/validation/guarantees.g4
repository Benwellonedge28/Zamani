/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/guarantees.g4
 *
 * GRAMMAR
 * -------
 * GuaranteesValidation
 *
 * STATUS
 * ------
 * Production validation/conformance component
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides an isolated, complete-input validation entry point for
 * the canonical Zamani `guarantee` contract statement.
 *
 * IMPORTANT
 * ---------
 *
 * This file is NOT a second owner of `guarantee` syntax.
 *
 * Canonical source-level ownership remains:
 *
 *     grammar/statements/contract.g4
 *         |
 *         +--> ContractStatements
 *                 |
 *                 +--> guaranteeStatement
 *
 * This file only exposes that canonical rule through a validation-specific
 * complete-input entry point.
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ContractStatements
 *       |
 *       +--> guaranteeStatement
 *       |
 *       v
 *     GuaranteesValidation facade
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * ANTLR:
 *
 *     ANTLR4 parser grammar
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no semantic actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no resource allocation;
 *     - no target selection;
 *     - no backend selection;
 *     - no machine-capacity constants;
 *     - no physical-device knowledge;
 *     - no quantum-device knowledge;
 *     - no vendor-specific syntax;
 *     - no domain-specific expression grammar.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide a stable isolated validation/conformance entry point for exactly
 * one complete canonical guarantee statement:
 *
 *     guarantee(condition);
 *
 * The concrete syntax, expression syntax, punctuation, and lexical vocabulary
 * remain owned by their canonical repository components.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the validation facade:
 *
 *     guaranteesValidationUnit
 *     guaranteesValidationItem
 *
 * `guaranteesValidationUnit` is the complete-input entry point.
 *
 * `guaranteesValidationItem` is the isolated validation wrapper.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     guaranteeStatement
 *     contractStatement
 *     contractCondition
 *
 * Those remain owned by:
 *
 *     grammar/statements/contract.g4
 *
 * It also does not own:
 *
 *     lexer rules
 *     keywords
 *     identifiers
 *     qualified names
 *     punctuation
 *     expressions
 *     types
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     contracts
 *     AST structures
 *     semantic validation
 *     verification
 *     proof search
 *     theorem proving
 *     runtime checking
 *     optimization
 *     lowering
 *     quantum operations
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     HDL syntax
 *     hardware syntax
 *     backend selection
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/contract.g4
 *         ContractStatements
 *         guaranteeStatement
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         canonical lexer boundary
 *
 *     grammar/expressions/
 *         indirectly through ContractStatements
 *
 *     grammar/core/punctuation.g4
 *         indirectly through ContractStatements
 *
 * EXPORTS
 * -------
 *
 *     guaranteesValidationUnit
 *     guaranteesValidationItem
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar validation tooling
 *     isolated contract parser tests
 *     grammar regression tests
 *     source-span tests
 *     diagnostics tests
 *     validation orchestration
 *
 * AST_OWNER
 * ---------
 *
 * The repository frontend AST subsystem owns AST construction.
 *
 * This grammar creates parser contexts only.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Contract semantic analysis and verification infrastructure own the meaning
 * of a guarantee.
 *
 * IR_OWNER
 * --------
 *
 * The canonical semantic representation and applicable domain IR own lowering.
 *
 * For quantum computations the canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/
 *     validation conformance tests
 *     contract tests
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/contracts.md
 *     grammar/spec/contracts.md
 *
 * ============================================================================
 * CANONICAL SOURCE OWNERSHIP
 * ============================================================================
 *
 * The canonical source grammar is:
 *
 *     grammar/statements/contract.g4
 *
 * Its grammar name is:
 *
 *     ContractStatements
 *
 * Its canonical dispatcher is:
 *
 *     contractStatement
 *
 * Its canonical guarantee rule is:
 *
 *     guaranteeStatement
 *
 * The canonical production is:
 *
 *     guaranteeStatement
 *         : GUARANTEE contractCondition statementTerminator
 *         ;
 *
 * `contractCondition` is also owned by ContractStatements and delegates to
 * the canonical expression grammar.
 *
 * This file MUST NOT reproduce that production.
 *
 * In particular, DO NOT introduce another rule such as:
 *
 *     guaranteeStatement
 *         : GUARANTEE LPAREN expression RPAREN statementTerminator
 *         ;
 *
 * Such duplication would create competing grammar ownership.
 *
 * ============================================================================
 * WHY THIS VALIDATION FACADE EXISTS
 * ============================================================================
 *
 * The production Zamani compiler does not require a separate guarantee
 * grammar.
 *
 * The canonical production parser already handles:
 *
 *     guarantee(condition);
 *
 * However, repository validation benefits from an isolated complete-input
 * entry point that can verify one guarantee construct without accepting
 * trailing source as part of the same validation unit.
 *
 * Therefore:
 *
 *     guaranteesValidationUnit
 *         -> guaranteesValidationItem EOF
 *
 * provides an explicit complete-input boundary.
 *
 * The production source parser continues to use:
 *
 *     ContractStatements
 *
 * through normal statement dispatch.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes the repository's canonical lexer vocabulary.
 *
 * It does NOT define:
 *
 *     GUARANTEE
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     PROPERTY
 *
 * Those lexical identities belong to the canonical lexical subsystem.
 *
 * Therefore this file contains no lexer rules and no token aliases.
 *
 * This prevents validation/guarantees.g4 from becoming a competing lexical
 * authority.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * The dependency is:
 *
 *     GuaranteesValidation
 *             |
 *             v
 *     ContractStatements
 *             |
 *             v
 *     guaranteeStatement
 *             |
 *             v
 *     contractCondition
 *             |
 *             v
 *     expression
 *
 * The validation facade therefore inherits exactly the same concrete syntax
 * as the production source parser.
 *
 * No validation-specific variation of guarantee syntax is permitted.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * guaranteesValidationUnit
 * -------------------------
 *
 * Complete-input validation entry point.
 *
 * It requires exactly one canonical guarantee statement followed by EOF.
 *
 * guaranteesValidationItem
 * ------------------------
 *
 * Isolated validation wrapper around the canonical guaranteeStatement rule.
 *
 * ============================================================================
 * PRIVATE RULES
 * ============================================================================
 *
 * None.
 *
 * This is intentional.
 *
 * This file must not duplicate:
 *
 *     contractCondition
 *     expression
 *     statementTerminator
 *     identifier
 *     qualifiedName
 *     punctuation
 *
 * Every such rule already has an established owner.
 *
 * ============================================================================
 * SOURCE SYNTAX CONTRACT
 * ============================================================================
 *
 * The canonical source form is inherited from:
 *
 *     ContractStatements.guaranteeStatement
 *
 * Conceptually:
 *
 *     guarantee(condition);
 *
 * The condition is the ordinary Zamani expression grammar.
 *
 * This validation facade places no additional restrictions on the expression.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A guarantee is a domain-neutral semantic contract obligation.
 *
 * It states a condition that the relevant enclosing computation, operation,
 * declaration, execution context, or semantic boundary promises to satisfy.
 *
 * The grammar itself does NOT establish that the guarantee is true.
 *
 * It only establishes that the source contains a structurally valid guarantee
 * statement.
 *
 * Semantic analysis determines:
 *
 *     - whether the guarantee is legal in its enclosing context;
 *     - whether its condition is well typed;
 *     - whether referenced names exist;
 *     - whether required effects are permitted;
 *     - whether referenced capabilities are meaningful;
 *     - whether resource expressions are valid;
 *     - whether policies permit the guarantee;
 *     - whether the guarantee is statically provable;
 *     - whether runtime verification is required;
 *     - whether the guarantee can be preserved through lowering.
 *
 * A syntactically valid guarantee is therefore not automatically a verified
 * guarantee.
 *
 * ============================================================================
 * GUARANTEE SEMANTICS
 * ============================================================================
 *
 * A guarantee has the semantic shape:
 *
 *     Guarantee
 *     {
 *         condition
 *         scope
 *         source_span
 *         semantic_context
 *         provenance
 *     }
 *
 * The concrete Rust representation belongs to the AST/semantic subsystem.
 *
 * This grammar MUST NOT introduce target-specific semantic nodes such as:
 *
 *     QuantumGuarantee
 *     GPUGuarantee
 *     FPGAGuarantee
 *     HDLGuarantee
 *     AIGuarantee
 *     DistributedGuarantee
 *     HardwareGuarantee
 *
 * One domain-neutral guarantee model must be usable across domains.
 *
 * ============================================================================
 * RELATION TO THE CONTRACT MODEL
 * ============================================================================
 *
 * The guarantee participates in the common contract family:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Ownership of those source constructs remains:
 *
 *     grammar/statements/contract.g4
 *
 * This file does not redefine any other contract kind.
 *
 * A guarantee may coexist with:
 *
 *     requires(...)
 *     ensures(...)
 *     invariant(...)
 *     assume(...)
 *     property(...)
 *
 * according to the semantic context rules established by the contract
 * specification.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * The guarantee condition is an ordinary Zamani expression.
 *
 * There is deliberately no:
 *
 *     guaranteeExpression
 *     quantumGuaranteeExpression
 *     hardwareGuaranteeExpression
 *     resourceGuaranteeExpression
 *     AIGuaranteeExpression
 *
 * This is essential for long-term language scalability.
 *
 * The expression grammar can evolve independently while this validation
 * facade remains stable.
 *
 * The condition may therefore refer, where semantically valid, to:
 *
 *     values
 *     variables
 *     function results
 *     records
 *     arrays
 *     tensors
 *     data
 *     measurement results
 *     quantum-derived values
 *     hardware state
 *     distributed state
 *     model state
 *     capabilities
 *     resources
 *     provenance
 *     policies
 *     other semantic entities
 *
 * The validation grammar does not define those expressions.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file introduces no special guarantee type.
 *
 * The condition is type checked by the canonical type/semantic system.
 *
 * Depending on semantic context, a guarantee may require a predicate-like
 * condition or another explicitly specified contract-compatible type.
 *
 * This grammar does not decide that typing rule.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a guarantee does not execute its condition.
 *
 * The condition may nevertheless reference expressions whose semantic
 * evaluation carries effects.
 *
 * Existing effect analysis remains authoritative.
 *
 * Possible effect categories may include, where defined by the repository:
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
 * This file neither defines nor duplicates those effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Guarantee conditions may depend on capability-related semantic values.
 *
 * For example:
 *
 *     guarantee(capability("quantum.measurement"));
 *
 * The parser only accepts the expression structurally.
 *
 * Capability resolution remains downstream:
 *
 *     expression
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     capability resolution
 *
 * No physical device is selected by this file.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource-related conditions remain ordinary expressions.
 *
 * Examples include:
 *
 *     guarantee(memory_used <= allowed_memory);
 *     guarantee(qubits_used <= required_qubits);
 *     guarantee(result_size <= available_capacity);
 *     guarantee(topology.supports(required_topology));
 *
 * These are semantic statements about requirements, state, or outcomes.
 *
 * They are NOT universal machine-capacity declarations.
 *
 * This file therefore contains no limits for:
 *
 *     memory
 *     qubits
 *     CPUs
 *     cores
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     nodes
 *     devices
 *     threads
 *     tensor rank
 *     registers
 *     network size
 *
 * Actual resource feasibility belongs to semantic analysis, planning,
 * capability negotiation, target realization, and runtime reporting.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may determine whether a guarantee is:
 *
 *     permitted
 *     enforceable
 *     verifiable
 *     deployable
 *     admissible
 *     runtime-checkable
 *     compatible with an execution mode
 *
 * Policy interpretation is downstream.
 *
 * This file does not define policy syntax or policy evaluation.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser/AST/semantic pipeline must preserve sufficient source provenance
 * to associate the guarantee with:
 *
 *     source file
 *     source span
 *     module
 *     enclosing declaration
 *     execution scope
 *     contract kind
 *     condition
 *
 * Later semantic and compiler stages may additionally record:
 *
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     derivation
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * VERIFICATION CONTRACT
 * ============================================================================
 *
 * This file performs structural parsing only.
 *
 * It does NOT:
 *
 *     prove the guarantee;
 *     execute the condition;
 *     invoke a theorem prover;
 *     invoke a solver;
 *     inspect hardware;
 *     inspect runtime state;
 *     execute quantum operations;
 *     measure a quantum system;
 *     evaluate a learned model;
 *     allocate resources;
 *     select a target;
 *     generate backend code.
 *
 * Verification belongs to the downstream semantic/verification architecture.
 *
 * A backend may eventually:
 *
 *     statically discharge a guarantee;
 *     generate a runtime check;
 *     generate a proof obligation;
 *     propagate the guarantee;
 *     reject an unsupported guarantee;
 *     preserve the guarantee through lowering.
 *
 * None of those actions belong in this grammar.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A guarantee condition may refer to quantum semantic information.
 *
 * Examples:
 *
 *     guarantee(measurement.is_valid());
 *
 *     guarantee(capability("quantum.measurement"));
 *
 * The validation grammar remains completely quantum-domain neutral.
 *
 * It does NOT:
 *
 *     enumerate gates;
 *     enumerate qubits;
 *     allocate physical qubits;
 *     encode coupling maps;
 *     encode calibration;
 *     perform routing;
 *     perform scheduling;
 *     perform decomposition;
 *     implement QEC;
 *     implement resilience;
 *     create a quantum IR.
 *
 * The downstream boundary remains:
 *
 *     source
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic quantum model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     decomposition
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     resilience / QEC
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *         |
 *         v
 *     target realization
 *
 * This file must remain unchanged when new quantum operations, hardware
 * topologies, QEC mechanisms, or QPU vendors are introduced.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Guarantee conditions may describe hardware or HDL semantic properties.
 *
 * Examples:
 *
 *     guarantee(output.is_valid());
 *
 *     guarantee(signal.is_stable());
 *
 *     guarantee(hardware_state.is_consistent());
 *
 * The grammar does not encode:
 *
 *     fixed signal widths
 *     fixed register widths
 *     fixed memory sizes
 *     fixed FPGA dimensions
 *     fixed ASIC resource counts
 *     fixed device identifiers
 *     fixed clock counts
 *     fixed physical topology
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical computation may use guarantees for:
 *
 *     result correctness
 *     numerical properties
 *     data invariants
 *     algorithmic postconditions
 *     resource outcomes
 *     deterministic behavior
 *
 * None of these require a classical-only guarantee grammar.
 *
 * ============================================================================
 * AI / MODEL BOUNDARY
 * ============================================================================
 *
 * Guarantees may describe properties involving model computation, inference,
 * learning, adaptation, uncertainty, evidence, or decisions where the
 * semantic system permits them.
 *
 * Examples:
 *
 *     guarantee(prediction.is_valid());
 *
 *     guarantee(confidence >= required_confidence);
 *
 *     guarantee(decision.has_evidence());
 *
 * The grammar does not introduce AI-specific guarantee syntax.
 *
 * Model semantics remain owned by their respective semantic/domain systems.
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * A guarantee may combine classical, quantum, hardware, distributed, and
 * model-derived values in one ordinary expression when the semantic model
 * permits that combination.
 *
 * Example:
 *
 *     guarantee(classical_result.is_consistent(measurement));
 *
 * This does not create a hybrid-specific grammar rule.
 *
 * The same guarantee syntax remains valid across domains.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Guarantees may express distributed semantic properties such as:
 *
 *     guarantee(message.is_authenticated());
 *     guarantee(state.is_consistent());
 *     guarantee(result.is_replicated());
 *
 * Distribution semantics remain owned by the distributed/concurrency/runtime
 * subsystems.
 *
 * This file introduces no node count, topology size, or replication limit.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing a guarantee validation unit depends only on:
 *
 *     source text
 *     canonical lexer vocabulary
 *     imported parser grammars
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
 *     resource availability
 *
 * Given identical source and parser configuration, equivalent parse structure
 * and source-span information must be produced.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file defines no finite language-level capacity for:
 *
 *     guarantee statements
 *     expression size
 *     expression nesting
 *     source-unit size
 *     module count
 *     program size
 *     contract count
 *     quantum operation count
 *     qubit count
 *     processor count
 *     accelerator count
 *     node count
 *     device count
 *     memory capacity
 *     tensor rank
 *     network size
 *
 * Repetition is structural rather than capacity-defined.
 *
 * "Infinity" means:
 *
 *     no artificial finite ceiling is imposed by this grammar.
 *
 * It does NOT mean:
 *
 *     infinite memory;
 *     infinite compute;
 *     infinite execution time;
 *     infinite physical resources.
 *
 * Actual limits arise from:
 *
 *     implementation representation;
 *     configured compiler budgets;
 *     available memory;
 *     available processing resources;
 *     security policy;
 *     runtime policy;
 *     target capabilities;
 *     actual physical resources.
 *
 * Such limits MUST NOT be encoded as universal language constants here.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT define universal capacity constants such as:
 *
 *     MAX_GUARANTEES
 *     MAX_CONTRACTS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICE_COUNT
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * No target-specific numeric capacity belongs in this grammar.
 *
 * Numeric literals inside a guarantee condition remain ordinary program
 * values. They are not language-capacity declarations.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical standalone source form remains:
 *
 *     guarantee(condition);
 *
 * Function-level contract blocks remain owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * This file MUST NOT reinterpret:
 *
 *     contract { ... }
 *
 * as a standalone validation construct.
 *
 * Compatibility and legacy syntax remain governed by:
 *
 *     grammar/compatibility/
 *
 * If a historical spelling is supported, its compatibility status must be
 * explicitly classified there.
 *
 * This validation facade must continue to accept exactly the syntax accepted
 * by the canonical guaranteeStatement rule.
 *
 * Therefore a syntax change belongs first to:
 *
 *     grammar/statements/contract.g4
 *
 * and the validation facade automatically follows that canonical rule without
 * duplicating its production.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * This facade must allow the parser to report structural errors for malformed
 * guarantee input.
 *
 * Examples of invalid complete inputs include:
 *
 *     guarantee
 *     guarantee()
 *     guarantee(
 *     guarantee(;
 *     guarantee(, condition);
 *     guarantee(condition;
 *     guarantee(condition,);
 *     guarantee(condition, other);
 *     guarantee(condition) trailing
 *
 * The exact diagnostic wording is owned by the parser/diagnostic subsystem.
 *
 * This grammar must not embed diagnostic text into semantic actions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     guarantee(result >= 0);
 *
 *     guarantee(output.is_valid());
 *
 *     guarantee(state.is_consistent());
 *
 *     guarantee(capability("quantum.measurement"));
 *
 *     guarantee(memory_used <= allowed_memory);
 *
 *     guarantee(tensor.shape == expected_shape);
 *
 *     guarantee(measurement.is_valid());
 *
 *     guarantee(hardware_state.is_consistent());
 *
 *     guarantee(decision.has_evidence());
 *
 *     guarantee(classical_result.is_consistent(measurement));
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     guarantee
 *     guarantee()
 *     guarantee(
 *     guarantee(;
 *     guarantee(, condition);
 *     guarantee(condition;
 *     guarantee(condition,);
 *     guarantee(condition, other);
 *     guarantee(condition) trailing
 *
 * The validation harness must verify that malformed inputs are rejected or
 * diagnosed according to the canonical parser error policy.
 *
 * BOUNDARY TESTS
 * --------------
 *
 *     guarantee((deeply_nested_expression));
 *
 *     guarantee(large_expression);
 *
 *     guarantee(quantum_derived_condition);
 *
 *     guarantee(hardware_condition);
 *
 *     guarantee(distributed_condition);
 *
 *     guarantee(model_condition);
 *
 *     guarantee(hybrid_condition);
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generate increasing source sizes and expression complexities without
 * modifying this grammar.
 *
 * The tests must demonstrate that no artificial grammar capacity is introduced
 * by this file.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * The same canonical guarantee syntax must be testable around:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     model computation
 *     data processing
 *     distributed computation
 *     networking
 *     accelerators
 *     scientific computation
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * The source text:
 *
 *     guarantee(condition);
 *
 * must remain syntactically identical when:
 *
 *     target architecture changes;
 *     processor count changes;
 *     accelerator count changes;
 *     memory availability changes;
 *     QPU changes;
 *     hardware topology changes;
 *     execution environment changes.
 *
 * A target's inability to satisfy the guarantee is a semantic/target
 * realization issue, not a reason for this grammar to change.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parse identical validation input repeatedly with identical configuration and
 * verify equivalent parse-tree structure and source spans.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no execution and therefore has no direct access to:
 *
 *     files
 *     networks
 *     secrets
 *     credentials
 *     hardware
 *     external processes
 *
 * It contains no embedded Rust.
 *
 * The downstream Rust implementation remains required to use:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * Rust `unsafe` is prohibited.
 *
 * Security policy enforcement remains outside this grammar.
 *
 * ============================================================================
 * POCO-REAF INTEGRATION
 * ============================================================================
 *
 * Guarantee syntax is part of portable semantic intent.
 *
 * It therefore follows:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic guarantee
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> policy analysis
 *       +--> provenance
 *       +--> verification
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> applicable domain IR
 *       |
 *       v
 *     target-independent optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     resilience / recovery
 *       |
 *       v
 *     ZQN / HAL where applicable
 *       |
 *       v
 *     target realization
 *
 * The guarantee remains source-level semantic intent throughout this process.
 *
 * A backend must not silently change the meaning of a guarantee merely
 * because the available target is different.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. CANONICAL CONTRACT GRAMMAR
 * -----------------------------
 *
 * This file consumes:
 *
 *     grammar/statements/contract.g4
 *
 * It MUST NOT duplicate guaranteeStatement.
 *
 * 2. LEXER
 * --------
 *
 * `GUARANTEE` is supplied by the canonical lexer vocabulary.
 *
 * No lexer changes are required merely to introduce this validation facade.
 *
 * 3. EXPRESSIONS
 * --------------
 *
 * The guarantee condition remains owned by:
 *
 *     grammar/expressions/
 *
 * No expression syntax is duplicated here.
 *
 * 4. PUNCTUATION
 * --------------
 *
 * Statement termination remains owned by:
 *
 *     grammar/core/punctuation.g4
 *
 * indirectly through ContractStatements.
 *
 * 5. STATEMENT DISPATCH
 * ---------------------
 *
 * Production source parsing remains owned by:
 *
 *     grammar/statements/statements.g4
 *
 * which already consumes ContractStatements.
 *
 * This validation facade does NOT participate in normal source statement
 * dispatch.
 *
 * 6. FUNCTION CONTRACTS
 * ---------------------
 *
 * `grammar/functions/contracts.g4` remains the owner of function-attached
 * contract blocks.
 *
 * No changes are required there for this validation facade.
 *
 * 7. AST
 * -----
 *
 * The validation facade must map to the same canonical guarantee AST/semantic
 * representation as the production parser.
 *
 * It must not require a validation-specific AST node.
 *
 * 8. SEMANTICS
 * ------------
 *
 * Semantic analysis consumes the canonical guarantee representation and
 * determines:
 *
 *     context validity
 *     expression validity
 *     type validity
 *     effect validity
 *     capability validity
 *     resource validity
 *     policy validity
 *     verification requirements
 *
 * 9. PROVENANCE
 * -------------
 *
 * Source spans from the canonical parser must remain available for validation
 * diagnostics and later semantic provenance.
 *
 * 10. IR
 * ------
 *
 * This file produces no IR.
 *
 * Guarantee semantics flow through the canonical semantic model before
 * applicable IR lowering.
 *
 * 11. QUANTUM
 * -----------
 *
 * Quantum guarantees continue through:
 *
 *     semantic quantum model
 *         ->
 *     quantum::ir
 *
 * This validation facade introduces no quantum-specific grammar.
 *
 * 12. HDL / HARDWARE
 * ------------------
 *
 * Hardware guarantees remain target-independent until semantic lowering and
 * target realization.
 *
 * 13. VALIDATION ORCHESTRATION
 * ----------------------------
 *
 * The validation subsystem may invoke:
 *
 *     guaranteesValidationUnit
 *
 * when validating an isolated complete guarantee source unit.
 *
 * Repository-wide validation must still use the canonical grammar composition
 * and must not treat this facade as a replacement for the program entry point.
 *
 * ============================================================================
 * FILE-LEVEL INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is independently complete.
 *
 * Its contract is stable even when implementation details of:
 *
 *     AST
 *     semantic analysis
 *     verification
 *     IR
 *     quantum lowering
 *     hardware backends
 *     runtime
 *
 * change.
 *
 * Changes in those systems are discovered through their integration contracts
 * and conformance tests rather than requiring this grammar file to duplicate
 * their implementation details.
 *
 * Required relationships:
 *
 *     grammar/statements/contract.g4
 *         -> canonical guarantee syntax
 *
 *     grammar/validation/README.md
 *         -> validation architecture
 *
 *     grammar/validation/ast-coverage.md
 *         -> AST traceability
 *
 *     grammar/validation/semantic-coverage.md
 *         -> semantic traceability
 *
 *     grammar/validation/ir-coverage.md
 *         -> IR traceability
 *
 *     grammar/validation/portability.md
 *         -> portability requirements
 *
 *     grammar/validation/scalability.md
 *         -> scalability requirements
 *
 *     grammar/validation/determinism.md
 *         -> deterministic validation requirements
 *
 *     grammar/specification/contracts.md
 *         -> normative contract semantics
 *
 *     grammar/spec/contracts.md
 *         -> formal contract model
 *
 *     grammar/lexer/keywords.g4
 *         -> GUARANTEE lexical identity
 *
 *     grammar/expressions/
 *         -> condition syntax
 *
 *     grammar/core/punctuation.g4
 *         -> statement termination
 *
 *     grammar/statements/statements.g4
 *         -> production statement dispatch
 *
 *     grammar/functions/contracts.g4
 *         -> function contract block ownership
 *
 *     src/frontend/ast/
 *         -> canonical AST
 *
 *     src/semantic.rs
 *         -> semantic validation
 *
 *     src/ir_gen.rs
 *         -> canonical lowering
 *
 *     src/quantum/ir/
 *         -> canonical quantum semantic boundary
 *
 * No one of these files is allowed to reinterpret this facade as a new source
 * language construct.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] The grammar name is `GuaranteesValidation`.
 *
 * [ ] `tokenVocab = ZamaniLexer` is present.
 *
 * [ ] `ContractStatements` is imported.
 *
 * [ ] Exactly one validation facade is exposed for complete guarantee input.
 *
 * [ ] `guaranteesValidationItem` delegates to canonical
 *     `guaranteeStatement`.
 *
 * [ ] `guaranteesValidationUnit` requires EOF.
 *
 * [ ] `guaranteeStatement` is NOT redefined here.
 *
 * [ ] `contractCondition` is NOT redefined here.
 *
 * [ ] `expression` is NOT redefined here.
 *
 * [ ] No lexer rule is defined here.
 *
 * [ ] No keyword is redefined here.
 *
 * [ ] No punctuation is redefined here.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No embedded Rust is used.
 *
 * [ ] No target-specific semantics are encoded.
 *
 * [ ] No physical hardware assumptions are encoded.
 *
 * [ ] No universal capacity limits are encoded.
 *
 * [ ] No quantum operation catalogue is encoded.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] No backend is selected.
 *
 * [ ] No runtime execution is performed.
 *
 * [ ] The same guarantee syntax is accepted independently of target size.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Portability tests exist.
 *
 * [ ] AST coverage is validated downstream.
 *
 * [ ] Semantic coverage is validated downstream.
 *
 * [ ] IR coverage is validated downstream.
 *
 * [ ] Rust frontend integration passes on Rust 1.97 / 1.97.1.
 *
 * [ ] The implementation remains Safe Rust with no `unsafe`.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar GuaranteesValidation;

options {
    tokenVocab = ZamaniLexer;
}

import
    ContractStatements
    ;


/*
 * ============================================================================
 * COMPLETE GUARANTEE VALIDATION UNIT
 * ============================================================================
 *
 * Exactly one complete canonical guarantee statement is accepted.
 *
 * EOF is intentional.
 *
 * Without EOF, an isolated validation tool could accept:
 *
 *     guarantee(condition); trailing
 *
 * as though the complete input were valid.
 *
 * Complete-input validation therefore requires:
 *
 *     guaranteeStatement EOF
 *
 * while the production program parser continues to accept a sequence of
 * statements through its normal source-unit architecture.
 */
guaranteesValidationUnit
    : guaranteesValidationItem EOF
    ;


/*
 * ============================================================================
 * ISOLATED GUARANTEE VALIDATION ITEM
 * ============================================================================
 *
 * This rule is only a validation facade.
 *
 * The canonical guarantee syntax remains owned by:
 *
 *     ContractStatements.guaranteeStatement
 *
 * Delegating here is essential because it guarantees that isolated validation
 * and normal source parsing cannot silently acquire different syntax.
 */
guaranteesValidationItem
    : guaranteeStatement
    ;