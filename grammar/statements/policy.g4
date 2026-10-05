/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/policy.g4
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION STATEMENT ADAPTER
 *
 * PURPOSE
 * -------
 * Makes the canonical Zamani policy expression available in statement
 * position without redefining policy syntax.
 *
 * This file is intentionally SMALL at the grammar-production level.
 *
 * The policy language itself is NOT owned here.
 *
 * Canonical policy ownership:
 *
 *     grammar/core/policies.g4
 *
 * Canonical policy-expression ownership:
 *
 *     grammar/expressions/policy.g4
 *
 * This file owns only:
 *
 *     policyStatement
 *
 * That distinction prevents policy syntax from being duplicated between:
 *
 *     declarations
 *     expressions
 *     statements
 *     security
 *     resources
 *     execution
 *     quantum
 *     hardware
 *     AI
 *     distributed computing
 *     dialects
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no I/O;
 *     - no filesystem access;
 *     - no networking;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no target inspection;
 *     - no resource allocation;
 *     - no mutable global state;
 *     - no machine-capacity constants.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * A policy statement applies an inline policy expression at the current
 * source/semantic scope.
 *
 * Canonical conceptual form:
 *
 *     policy {
 *         require capability("quantum.measurement");
 *         forbid effect("network");
 *     };
 *
 * Or, where a policy selector is supported:
 *
 *     policy quantum {
 *         require capability("quantum.measurement");
 *     };
 *
 * The exact policy clauses are owned by PolicyExpressions.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns exactly:
 *
 *     policyStatement
 *
 * It owns the fact that a policy expression can occur in statement position.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     policyExpression
 *     policyBody
 *     policyClause
 *     policyDirective
 *     policySelector
 *     policyScope
 *     policyBinding
 *     policyReference
 *     policy declaration syntax
 *     policy semantics
 *     authorization
 *     capability resolution
 *     resource resolution
 *     effect checking
 *     contract checking
 *     policy conflict resolution
 *     policy inheritance
 *     policy composition
 *     security enforcement
 *     execution planning
 *     target selection
 *     hardware selection
 *     quantum routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/expressions/policy.g4
 *     grammar/core/punctuation.g4
 *
 * Imported grammar identities:
 *
 *     PolicyExpressions
 *     Punctuation
 *
 * ============================================================================
 * EXPORTS
 * -------
 *
 *     policyStatement
 *
 * ============================================================================
 * CONSUMED_BY
 * -----------
 *
 *     grammar/statements/statements.g4
 *
 * The universal statement dispatcher must reference:
 *
 *     policyStatement
 *
 * It must not duplicate the policy expression syntax.
 *
 * ============================================================================
 * AST_OWNER
 * ---------
 *
 * The domain-neutral frontend AST.
 *
 * The AST representation should preserve:
 *
 *     policy expression
 *     source span
 *     source ordering
 *     enclosing semantic scope
 *
 * Conceptually:
 *
 *     PolicyStatement {
 *         policy: PolicyExpression,
 *         source_span
 *     }
 *
 * The exact Rust type belongs to the AST subsystem.
 *
 * This grammar MUST NOT define Rust AST structures.
 *
 * ============================================================================
 * SEMANTIC_OWNER
 * --------------
 *
 * The canonical Zamani policy semantic subsystem.
 *
 * Semantic analysis is responsible for:
 *
 *     policy identity
 *     policy scope
 *     policy applicability
 *     policy composition
 *     policy conflicts
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     effects
 *     contracts
 *     permissions
 *     prohibitions
 *     preferences
 *     fallbacks
 *     adaptation rules
 *     execution rules
 *     provenance
 *     diagnostics
 *
 * Parsing a policy statement MUST NOT evaluate the policy.
 *
 * ============================================================================
 * POLICY SEMANTIC MODEL
 * ============================================================================
 *
 * A policy statement expresses GOVERNING INTENT.
 *
 * It is distinct from:
 *
 *     requirement
 *     capability
 *     resource
 *     effect
 *     contract
 *     authorization
 *     implementation decision
 *
 * The distinction is:
 *
 *     requirement
 *         what must be satisfied
 *
 *     capability
 *         what an environment can provide
 *
 *     resource
 *         what computational material is available/requested
 *
 *     effect
 *         what computation may observe/change/do
 *
 *     contract
 *         what semantic property must hold
 *
 *     policy
 *         how those semantic choices are governed
 *
 * Therefore:
 *
 *     capability != permission
 *
 *     permission != resource
 *
 *     policy != authorization
 *
 * The security subsystem may consume the resulting policy semantics.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing this statement has NO runtime effect.
 *
 * Semantic policy interpretation may constrain or require effects such as:
 *
 *     io
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * Effect ownership remains in:
 *
 *     grammar/effects/
 *
 * This grammar must not enumerate effect implementations.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Policy expressions may refer to capabilities symbolically.
 *
 * Examples:
 *
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *     capability("network.transport")
 *     capability("hardware.reconfigurable")
 *
 * The policy statement does not determine whether those capabilities exist.
 *
 * Capability resolution occurs downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Policy statements may constrain or prefer resources through the existing
 * policy-expression and resource semantic systems.
 *
 * The grammar MUST NOT encode:
 *
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     QPU counts
 *     qubit counts
 *     node counts
 *     memory capacities
 *     register widths
 *     tensor-rank limits
 *     network-size limits
 *     device counts
 *
 * A policy may express symbolic resource intent, while the compiler/runtime
 * determines whether that intent can be realized.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Policies may govern how contracts are applied.
 *
 * They MUST NOT redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Those constructs retain their canonical ownership in the validation/
 * contract subsystem.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The semantic representation produced from a policy statement must retain
 * sufficient information for provenance.
 *
 * At minimum:
 *
 *     source span
 *     policy structure
 *     directive structure
 *     enclosing scope
 *
 * The downstream provenance subsystem may additionally record:
 *
 *     originating declaration
 *     transformation
 *     decision
 *     evidence
 *     verification
 *     policy version
 *     semantic version
 *
 * This grammar does not implement provenance.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Pipeline:
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
 *     semantic policy model
 *       ->
 *     canonical semantic representation
 *       ->
 *     domain IR
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     target realization
 *
 * A policy statement MUST NOT lower directly to:
 *
 *     machine instructions
 *     physical resources
 *     quantum gates
 *     physical qubits
 *     FPGA regions
 *     network routes
 *     scheduler commands
 *     QEC operations
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A policy may govern quantum computation.
 *
 * Example:
 *
 *     policy quantum {
 *         require capability("quantum.measurement");
 *         prefer quantum.fault_tolerant;
 *     };
 *
 * The policy statement does not create quantum IR.
 *
 * Quantum semantic lowering remains:
 *
 *     AST
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
 *     target realization
 *
 * `quantum::ir` remains the canonical quantum boundary.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A policy may govern HDL or hardware intent.
 *
 * Examples of semantic intent include:
 *
 *     hardware::reconfigurable
 *     hardware::low_latency
 *     hardware::verified
 *     synthesis::preferred
 *
 * Such names remain semantic identifiers.
 *
 * This grammar does not encode:
 *
 *     bus widths
 *     register widths
 *     device identifiers
 *     FPGA resource counts
 *     ASIC geometry
 *     clock counts
 *     physical placement
 *     routing
 *     timing closure
 *
 * Those belong downstream.
 *
 * ============================================================================
 * AI / KNOWLEDGE BOUNDARY
 * ============================================================================
 *
 * Policies can govern:
 *
 *     inference
 *     learning
 *     adaptation
 *     knowledge access
 *     evidence requirements
 *     explanation requirements
 *     provenance requirements
 *     agent behavior
 *     model selection
 *
 * No AI-specific policy keyword catalog is required.
 *
 * New AI capabilities are represented through:
 *
 *     policy names
 *     capabilities
 *     requirements
 *     expressions
 *     dialects
 *     semantic registrations
 *
 * ============================================================================
 * DISTRIBUTED / CONCURRENCY BOUNDARY
 * ============================================================================
 *
 * Policies may govern:
 *
 *     parallel execution
 *     distributed execution
 *     retry
 *     fallback
 *     consistency
 *     communication
 *     resilience
 *     scheduling preferences
 *
 * This grammar does not encode:
 *
 *     worker counts
 *     thread counts
 *     node counts
 *     actor counts
 *     channel counts
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * A policy may contain:
 *
 *     allow
 *     forbid
 *     permission
 *     prohibition
 *     authorization-related intent
 *
 * but a policy statement does not itself grant authorization.
 *
 * Security enforcement belongs to:
 *
 *     grammar/security/
 *
 * and the corresponding semantic/runtime layers.
 *
 * ============================================================================
 * EXECUTION BOUNDARY
 * ============================================================================
 *
 * Execution policies may govern:
 *
 *     determinism
 *     simulation
 *     fallback
 *     recovery
 *     retry
 *     adaptation
 *     reproducibility
 *     deployment
 *
 * The statement layer does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     cluster
 *     cloud
 *     physical device
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A policy statement expresses portable semantic intent.
 *
 * The same policy source may therefore participate in realization on:
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
 *     heterogeneous systems
 *     future computational substrates
 *
 * provided that the semantic requirements can be satisfied.
 *
 * The source grammar does not need to change merely because the target
 * becomes larger or different.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar introduces no fixed maximum for:
 *
 *     policy statements
 *     policy clauses
 *     directives
 *     arguments
 *     nested expressions
 *     source units
 *     program size
 *     policy scopes
 *     policy metadata
 *     policy composition
 *
 * Repetition and nesting are delegated to the canonical policy expression
 * grammar and normal parser structure.
 *
 * "Infinity" here means that the language introduces no artificial
 * application/hardware cardinality limit. Actual implementations remain
 * constrained by available computational resources and implementation
 * limits.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing this construct must depend only on:
 *
 *     source text
 *     lexer configuration
 *     parser grammar
 *     parser configuration
 *
 * It must NOT depend on:
 *
 *     hardware
 *     runtime state
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     resource availability
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics include:
 *
 *     missing policy body
 *     malformed policy selector
 *     malformed policy clause
 *     malformed policy expression
 *     missing statement terminator
 *     unexpected token
 *
 * Semantic diagnostics include:
 *
 *     unknown policy
 *     invalid policy scope
 *     conflicting policy
 *     unsatisfied requirement
 *     unavailable capability
 *     forbidden effect
 *     invalid authorization
 *     invalid contract interaction
 *     unsupported policy composition
 *
 * Semantic diagnostics MUST remain downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This new statement adapter introduces no new keyword.
 *
 * It therefore does not change the lexical vocabulary.
 *
 * Existing policy declarations remain governed by their existing grammar.
 *
 * Existing policy expressions remain governed by PolicyExpressions.
 *
 * Future policy directives can be added without changing this statement
 * adapter, provided they remain expressible by the canonical policy-expression
 * grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     policy {
 *         require capability("quantum.measurement");
 *     };
 *
 *     policy quantum {
 *         require capability("quantum.measurement");
 *     };
 *
 *     policy execution {
 *         forbid effect("network");
 *     };
 *
 *     policy {
 *         prefer execution.deterministic;
 *     };
 *
 *     policy {
 *         require capability("tensor.compute");
 *         prefer hardware.reconfigurable;
 *     };
 *
 * NEGATIVE
 * --------
 *
 *     policy
 *
 *     policy;
 *
 *     policy {
 *
 *     policy { };
 *
 *     policy { malformed syntax };
 *
 * BOUNDARY
 * --------
 *
 *     policy {} ;
 *
 *     policy scope {} ;
 *
 *     policy a.b.c {} ;
 *
 *     policy {
 *         require capability(symbolic_requirement);
 *         prefer resource.preference;
 *         forbid effect("network");
 *     };
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     policy quantum { ... };
 *     policy hardware { ... };
 *     policy execution { ... };
 *     policy distributed { ... };
 *     policy learning { ... };
 *     policy security { ... };
 *
 * The statement syntax remains identical regardless of domain.
 *
 * SCALABILITY
 * -----------
 *
 * Test policies with:
 *
 *     many clauses
 *     many directives
 *     deeply nested expressions
 *     long qualified names
 *     large policy compositions
 *
 * without introducing language-level artificial cardinality constants.
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and parser configuration must produce equivalent parse
 * trees.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO hardware limits
 *     NO resource-capacity limits
 *     NO quantum limits
 *     NO processor limits
 *     NO accelerator limits
 *     NO network limits
 *     NO tensor limits
 *     NO device limits
 *     NO fixed policy cardinality limits
 *
 * It contains no target-specific selection.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. grammar/statements/statements.g4
 *
 *    Add:
 *
 *        PolicyStatements
 *
 *    to the import list.
 *
 *    Add:
 *
 *        | policyStatement
 *
 *    to the universal `statement` rule.
 *
 *    `statements.g4` remains the sole owner of the universal statement rule.
 *
 * 2. grammar/expressions/policy.g4
 *
 *    Must expose:
 *
 *        policyExpression
 *
 *    and must use the canonical Zamani lexer token names.
 *
 *    The current repository version refers to token spellings such as
 *    `KeywordPolicy`, `LeftBrace`, `RightBrace`, `Terminator`, etc., while the
 *    canonical lexer currently exposes names such as `POLICY`, `LBRACE`,
 *    `RBRACE`, and `SEMICOLON`.
 *
 *    This mismatch MUST be corrected in the expression grammar rather than
 *    worked around here.
 *
 * 3. grammar/core/policies.g4
 *
 *    Remains the owner of named/declarative policy declarations.
 *
 *    This statement adapter MUST NOT duplicate `policyDeclaration`.
 *
 * 4. grammar/security/
 *
 *    Security policy grammars may consume the resulting semantic policy.
 *
 *    They MUST NOT replace this universal policy statement.
 *
 * 5. grammar/resources/
 *
 *    Resource policies consume semantic resource constraints/preferences.
 *
 *    They MUST NOT redefine `policyStatement`.
 *
 * 6. grammar/execution/
 *
 *    Execution policies consume policy semantics downstream.
 *
 *    They MUST NOT import runtime implementation into this parser grammar.
 *
 * 7. grammar/ai/
 *
 *    AI policies consume the common policy semantic model.
 *
 *    They MUST NOT create an AI-only universal policy syntax.
 *
 * 8. grammar/quantum/
 *
 *    Quantum policies consume the same semantic policy representation.
 *
 *    No quantum-specific policy statement dispatcher is required.
 *
 * 9. grammar/hardware/
 *
 *    Hardware policies remain target-neutral.
 *
 *    Physical realization remains downstream.
 *
 * 10. grammar/spec/policies.md
 *
 *     Define policy statement semantics and its distinction from policy
 *     declarations and policy expressions.
 *
 * 11. grammar/specification/
 *
 *     Document the normative policy model and source-level scope behavior.
 *
 * 12. grammar/tests/
 *
 *     Add parser, semantic, negative, boundary, scalability, determinism,
 *     compatibility, and cross-domain tests.
 *
 * ============================================================================
 * INDEPENDENT-FILE COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is considered independently complete when:
 *
 *     [x] It owns only `policyStatement`.
 *     [x] It delegates policy syntax to PolicyExpressions.
 *     [x] It delegates termination to Punctuation.
 *     [x] It defines no lexer rules.
 *     [x] It defines no semantic actions.
 *     [x] It defines no predicates.
 *     [x] It defines no runtime behavior.
 *     [x] It defines no hardware behavior.
 *     [x] It defines no resource capacity.
 *     [x] It defines no target selection.
 *     [x] It defines no IR.
 *     [x] It defines no fixed cardinality limits.
 *     [x] It is compatible with safe Rust generated-parser consumption.
 *
 * Repository integration is complete only after:
 *
 *     [ ] PolicyExpressions token names are normalized.
 *     [ ] Statements imports PolicyStatements.
 *     [ ] Statements dispatches policyStatement.
 *     [ ] AST recognizes PolicyStatement.
 *     [ ] Semantic analysis recognizes policy statements.
 *     [ ] Policy semantic model consumes the AST node.
 *     [ ] Policy tests pass.
 *     [ ] Cross-domain tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Scalability tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The architecture is:
 *
 *     policy source
 *          |
 *          v
 *     PolicyExpressions
 *          |
 *          v
 *     policyStatement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     policy semantic model
 *          |
 *     +----+----+----+----+
 *     |    |    |    |    |
 *   resource security execution quantum hardware
 *     |    |    |    |    |
 *     +----+----+----+----+
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     domain IR
 *          |
 *          v
 *     target realization
 *
 * Therefore this file remains stable while Zamani expands across classical,
 * quantum, HDL, AI, distributed, networking, accelerator, embedded, HPC and
 * future computational domains.
 *
 * ============================================================================
 */

parser grammar PolicyStatements;

options {
    tokenVocab = ZamaniLexer;
}

import
    PolicyExpressions,
    Punctuation
    ;

/*
 * ============================================================================
 * PUBLIC STATEMENT
 * ============================================================================
 *
 * The policy language is owned by PolicyExpressions.
 *
 * The statement layer adds only statement-position termination.
 *
 * ============================================================================
 */

policyStatement
    : policyExpression statementTerminator
    ;