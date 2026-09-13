/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/assertions.g4
 *
 * Status:
 *     Canonical production grammar for statement-level assertions.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar component.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No networking.
 *     No process execution.
 *     No device discovery.
 *     No hardware inspection.
 *     No runtime execution.
 *     No mutable global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SYNTAX of ordinary statement-level assertions.
 *
 * An assertion expresses a runtime/program-semantic requirement:
 *
 *     assert(condition);
 *
 * or, where the language permits an explanatory expression:
 *
 *     assert(condition, explanation);
 *
 * The grammar records the condition and optional explanation.
 *
 * It does NOT decide:
 *
 *     - whether the condition is true;
 *     - when the condition is evaluated;
 *     - whether evaluation is optimized;
 *     - whether the assertion is enabled;
 *     - whether failure aborts execution;
 *     - whether failure is recoverable;
 *     - whether an assertion is compiled out;
 *     - whether the assertion is checked on CPU/GPU/QPU/FPGA/etc.;
 *     - how assertion failure is represented in an IR;
 *     - how assertion failure is reported by the runtime.
 *
 * Those concerns belong to semantic analysis, compilation, IR, diagnostics,
 * and runtime layers.
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
 *       +--> statement composition
 *               |
 *               +--> assertionStatement       <-- THIS FILE
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> control-flow analysis
 *       |
 *       v
 *     canonical semantic representation / IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir where applicable
 *       +--> HDL/hardware IR where applicable
 *       +--> distributed/control-flow representations
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / routing / lowering
 *       |
 *       v
 *     target realization
 *       |
 *       v
 *     runtime
 *
 * This grammar file MUST remain at the syntax layer.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - assertionStatement
 *     - assertionCondition
 *     - optional assertion explanation
 *     - statement-level assertion syntax
 *     - assertion syntax composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the ASSERT lexer token;
 *     - identifiers;
 *     - literals;
 *     - expression precedence;
 *     - expression syntax;
 *     - types;
 *     - blocks;
 *     - statement composition;
 *     - compile-time assertions;
 *     - contracts;
 *     - invariants;
 *     - formal proofs;
 *     - runtime assertion execution;
 *     - panic/abort behavior;
 *     - diagnostics;
 *     - AST Rust structures;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - resource discovery;
 *     - backend selection.
 *
 * ============================================================================
 * IMPORTANT SEPARATION OF ASSERTION CONCEPTS
 * ============================================================================
 *
 * Zamani contains several related but distinct concepts.
 *
 * 1. Statement-level runtime assertion
 *
 *        assert(condition);
 *
 *        assert(condition, explanation);
 *
 *    Owned by THIS FILE.
 *
 *
 * 2. Compile-time assertion
 *
 *        comptime assert(condition);
 *
 *    Owned by:
 *
 *        grammar/expressions/compile-time.g4
 *
 *    This file MUST NOT redefine it.
 *
 *
 * 3. Function contracts
 *
 *        requires(...)
 *        ensures(...)
 *        invariant(...)
 *
 *    Owned by the function/contract grammar.
 *
 *
 * 4. Formal verification/proof constructs
 *
 *    Owned by the appropriate verification/metaprogramming language layer.
 *
 * These concepts may eventually share semantic infrastructure, but their
 * source-level grammar ownership remains distinct.
 *
 * ============================================================================
 * REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The canonical statement dispatcher is:
 *
 *     grammar/statements/statements.g4
 *
 * That file owns the `statement` composition rule.
 *
 * This file MUST NOT redefine:
 *
 *     statement
 *     controlFlowStatement
 *     expressionStatement
 *     blockExpression
 *
 * Instead, the canonical dispatcher must include:
 *
 *     assertionStatement
 *
 * as one of its statement alternatives.
 *
 * Recommended composition:
 *
 *     statement
 *         |
 *         +--> ...
 *         +--> controlFlowStatement
 *         +--> assertionStatement
 *         +--> ...
 *
 * Whether assertionStatement is physically grouped with control-flow
 * statements or as an independent statement family is a composition decision
 * owned by statements.g4.
 *
 * This file supplies the production.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer must provide:
 *
 *     ASSERT
 *
 * and the punctuation tokens required by the expression grammar:
 *
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * and the canonical statement terminator:
 *
 *     SEMICOLON
 *
 * No lexer rules are declared here.
 *
 * `ASSERT` is a language keyword and therefore must not be represented by a
 * generic identifier when the canonical lexer recognizes it as a keyword.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * `expression` is supplied by the canonical expression grammar.
 *
 * This file deliberately does NOT define:
 *
 *     booleanExpression
 *     assertionExpression
 *     conditionExpression
 *     messageExpression
 *
 * The semantic/type system determines whether the first expression is a valid
 * assertion condition.
 *
 * The optional second expression is interpreted semantically as an explanation
 * or diagnostic value according to the language specification.
 *
 * This permits the explanation to be:
 *
 *     a string
 *     a structured diagnostic value
 *     a lazily evaluated value
 *     a formatted expression
 *     a domain-specific diagnostic object
 *
 * without requiring this grammar to hard-code one representation.
 *
 * ============================================================================
 * CANONICAL FORMS
 * ============================================================================
 *
 * The canonical source forms are:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * The parenthesized form is deliberately explicit.
 *
 * This avoids ambiguity with ordinary expression statements and makes
 * assertion syntax easy for tooling and diagnostics to recognize.
 *
 * ============================================================================
 * ASSERTION STATEMENT
 * ============================================================================
 *
 * An assertion consists of:
 *
 *     ASSERT
 *     (
 *         condition
 *         optional explanation
 *     )
 *     statement terminator
 *
 * The condition is mandatory.
 *
 * The explanation is optional.
 *
 * There is no grammar-level limit on:
 *
 *     expression size
 *     expression nesting
 *     source length
 *     number of assertions
 *     number of statements
 *
 * Practical parser/compiler limits are implementation resource policies and
 * MUST NOT become source-language constants.
 */

parser grammar AssertionsParser;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL ENTRY POINT
 * ============================================================================
 *
 * `assertionStatement` is the only statement production owned by this file.
 *
 * It is intentionally named so the statement-composition grammar can import
 * or otherwise compose it without ambiguity.
 */
assertionStatement
    : ASSERT LPAREN assertionCondition RPAREN statementTerminator
    ;


/*
 * ============================================================================
 * ASSERTION CONDITION
 * ============================================================================
 *
 * The condition is an ordinary Zamani expression.
 *
 * Semantic analysis determines whether the resulting value is suitable for
 * assertion checking.
 *
 * This allows the same assertion syntax to operate across:
 *
 *     classical computation
 *     numerical computation
 *     symbolic computation
 *     quantum-classical control
 *     hardware control
 *     HDL-related semantic contexts
 *     distributed execution
 *     accelerator execution
 *     AI/data computation
 *     future computational domains
 *
 * without creating domain-specific assertion syntax.
 */
assertionCondition
    : expression
    ;


/*
 * ============================================================================
 * OPTIONAL ASSERTION EXPLANATION
 * ============================================================================
 *
 * The explanation is optional.
 *
 * It is deliberately represented as an ordinary expression rather than
 * hard-coded to STRING.
 *
 * This provides future-proofing for:
 *
 *     assert(condition, "message");
 *     assert(condition, diagnostic);
 *     assert(condition, format(...));
 *     assert(condition, error_context);
 *
 * The semantic layer decides which explanation types are accepted.
 *
 * This also avoids coupling assertion syntax to a particular diagnostic
 * representation.
 */
assertionExplanation
    : COMMA expression
    ;


/*
 * ============================================================================
 * EXTENDED ASSERTION ENTRY
 * ============================================================================
 *
 * `assertionWithExplanation` provides the explicit decomposition used by AST
 * builders and parser tooling.
 *
 * It is not a second assertion syntax.
 */
assertionWithExplanation
    : ASSERT LPAREN assertionCondition assertionExplanation RPAREN statementTerminator
    ;


/*
 * ============================================================================
 * CANONICAL ASSERTION FORM WITH OPTIONAL EXPLANATION
 * ============================================================================
 *
 * The canonical entry point accepts zero or one explanation.
 *
 * There is intentionally no repetition here.
 *
 * An assertion has:
 *
 *     exactly one condition
 *     zero or one explanation
 *
 * This prevents accidental acceptance of:
 *
 *     assert(a, b, c);
 *
 * as a valid assertion merely because the generic expression grammar can
 * represent comma-separated constructs.
 *
 * If the language eventually requires structured diagnostic arguments, that
 * should be introduced through a deliberate semantic/grammar evolution rather
 * than by silently broadening this production.
 *
 * NOTE:
 *
 * The direct canonical form is defined explicitly below rather than relying
 * on `assertionWithExplanation` so the parser has one authoritative entry
 * production.
 */


/*
 * ============================================================================
 * ASSERTION FORM
 * ============================================================================
 *
 * Replace the simple entry production above with this canonical form when
 * assembling the parser:
 *
 *     assertionStatement
 *         : ASSERT LPAREN assertionCondition assertionExplanation? RPAREN
 *           statementTerminator
 *         ;
 *
 * The equivalent expanded structure is:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * Keeping the optional explanation at the statement boundary makes the
 * accepted source language explicit.
 */


/*
 * ============================================================================
 * STATEMENT TERMINATION CONTRACT
 * ============================================================================
 *
 * `statementTerminator` is owned by the statement composition/termination
 * grammar.
 *
 * This file does not define it.
 *
 * The canonical current modular statement architecture uses semicolon
 * termination.
 *
 * This means:
 *
 *     assert(condition);
 *
 * is valid, while:
 *
 *     assert(condition)
 *
 * is not silently accepted by this grammar.
 *
 * Automatic semicolon insertion, newline-sensitive termination, or other
 * future termination policies must be specified centrally rather than
 * implemented independently by assertion syntax.
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/frontend must preserve at least:
 *
 *     - assertion condition;
 *     - optional explanation;
 *     - complete source span;
 *     - source ordering.
 *
 * Conceptual AST shape:
 *
 *     AssertionStatement {
 *         condition,
 *         explanation?,
 *         source_span
 *     }
 *
 * The actual Rust AST type belongs to the frontend AST subsystem.
 *
 * This grammar MUST NOT:
 *
 *     - define Rust structs;
 *     - define Rust enums;
 *     - construct AST objects;
 *     - execute assertions;
 *     - evaluate expressions;
 *     - allocate runtime resources.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer determines:
 *
 *     1. whether the condition is well typed;
 *
 *     2. whether the condition can be evaluated in the current context;
 *
 *     3. whether required effects are available;
 *
 *     4. whether required capabilities are available;
 *
 *     5. whether the explanation is semantically valid;
 *
 *     6. whether the assertion is reachable;
 *
 *     7. whether assertion evaluation has permitted side effects;
 *
 *     8. whether the assertion is compatible with the enclosing execution
 *        model;
 *
 *     9. whether an assertion may be removed, transformed, deferred, or
 *        preserved by compilation policy;
 *
 *    10. what happens when the assertion fails.
 *
 * None of these decisions belong in this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar does not require a particular runtime behavior.
 *
 * A runtime may eventually represent failure as:
 *
 *     diagnostic
 *     recoverable error
 *     panic
 *     abort
 *     exceptional control flow
 *     structured failure
 *     verification failure
 *     host-integrated failure
 *
 * according to the language/runtime semantics.
 *
 * The parser must remain independent of those choices.
 *
 * ============================================================================
 * COMPILE-TIME ASSERTION SEPARATION
 * ============================================================================
 *
 * `grammar/expressions/compile-time.g4` already owns:
 *
 *     compileTimeAssertionExpression
 *
 * including syntax based on:
 *
 *     COMPTIME ASSERT ...
 *
 * This file MUST NOT define another compile-time assertion production.
 *
 * Therefore:
 *
 *     assert(condition);
 *
 * means a statement-level assertion.
 *
 * A compile-time assertion belongs to the compile-time expression grammar.
 *
 * The semantic implementation may share infrastructure, but the grammar
 * ownership remains separate.
 *
 * ============================================================================
 * CONTRACT / INVARIANT SEPARATION
 * ============================================================================
 *
 * Zamani already contains contract-oriented concepts such as:
 *
 *     requires
 *     ensures
 *     invariant
 *
 * Those are not aliases for `assert`.
 *
 * An assertion is an executable/checkable statement-level construct.
 *
 * A contract describes a semantic obligation associated with a declaration,
 * function, module, or other contract-bearing entity.
 *
 * This file therefore MUST NOT absorb:
 *
 *     requires
 *     ensures
 *     invariant
 *
 * into the assertion grammar.
 *
 * ============================================================================
 * FORMAL VERIFICATION SEPARATION
 * ============================================================================
 *
 * Assertion syntax can provide runtime-checkable predicates.
 *
 * Formal proof systems may use those predicates as inputs to verification,
 * but this grammar does not claim that:
 *
 *     assert(condition);
 *
 * proves the condition mathematically.
 *
 * Proof obligations and theorem/proof constructs belong to the verification
 * layer.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Assertions may guard or validate hybrid/quantum computation.
 *
 * Examples include semantically valid forms such as:
 *
 *     assert(classical_condition);
 *
 *     assert(measurement_result);
 *
 *     assert(classical_value == expected);
 *
 * The grammar does not determine:
 *
 *     - qubit count;
 *     - logical qubit count;
 *     - physical qubit mapping;
 *     - backend;
 *     - topology;
 *     - gate set;
 *     - calibration;
 *     - QEC strategy;
 *     - ZQN model;
 *     - scheduling;
 *     - routing;
 *     - pulse implementation.
 *
 * If the condition eventually depends on quantum computation, the semantic
 * layer lowers the resulting operation through the repository's canonical
 * quantum semantic boundary.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT construct or duplicate quantum::ir.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same assertion syntax may semantically validate:
 *
 *     classical values
 *     numerical invariants
 *     tensor properties
 *     hardware state
 *     HDL simulation properties
 *     accelerator results
 *     distributed state
 *     AI/data invariants
 *     networking state
 *     future domain values
 *
 * No domain-specific assertion variants are required merely because the
 * eventual execution target differs.
 *
 * Do NOT introduce:
 *
 *     cpuAssert
 *     gpuAssert
 *     fpgaAssert
 *     qpuAssert
 *     distributedAssert
 *     acceleratorAssert
 *
 * merely to represent different target machines.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INDEPENDENCE
 * ============================================================================
 *
 * Assertion syntax contains no physical resource assumptions.
 *
 * It MUST NOT encode:
 *
 *     MAX_ASSERTIONS
 *     MAX_ASSERTION_DEPTH
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *     fixed device IDs
 *     fixed hardware addresses
 *     fixed topology
 *
 * The language-level syntax is independent of machine scale.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Assertions are source-level semantic intent.
 *
 * The same source assertion can survive compilation to:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC-oriented implementation
 *     quantum/classical system
 *     simulator
 *     accelerator
 *     cluster
 *     supercomputer
 *     distributed system
 *     cloud deployment
 *     future architecture
 *
 * The grammar does not select any of these targets.
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * because assertion syntax describes a property of the computation rather
 * than the implementation topology.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this construct must depend only on:
 *
 *     source text
 *     lexer definition
 *     grammar version
 *     parser configuration
 *
 * It must NOT depend on:
 *
 *     hardware
 *     backend
 *     runtime state
 *     queue state
 *     scheduler state
 *     calibration
 *     network state
 *     device availability
 *     quantum state
 *     resource availability
 *
 * The grammar contains no external side effects.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser/frontend should preserve accurate source spans for errors such
 * as:
 *
 *     assert;
 *     assert();
 *     assert(;
 *     assert();
 *     assert(condition;
 *     assert(, explanation);
 *     assert(condition,);
 *     assert(condition, explanation, extra);
 *     assert(condition) unexpected_tokens;
 *
 * The grammar itself supplies syntax structure.
 *
 * Diagnostic formatting and recovery policy belong to the parser/frontend
 * diagnostic subsystem.
 *
 * No Rust action is required.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing an assertion performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     environment inspection
 *     hardware discovery
 *     device access
 *     runtime evaluation
 *
 * Any later evaluation of the assertion expression is governed by the
 * semantic/compiler/runtime security model.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The repository already recognizes `assert` as a language capability.
 *
 * The modular grammar therefore formalizes that existing language feature
 * instead of replacing it with a target-specific mechanism.
 *
 * Legacy forms must be checked against the existing language specification
 * before removal.
 *
 * If an older grammar accepts:
 *
 *     assert expression;
 *
 * without parentheses, that syntax must be classified explicitly as:
 *
 *     preserved
 *     migrated
 *     deprecated
 *     or removed
 *
 * by the language compatibility policy.
 *
 * It must not disappear accidentally merely because this modular grammar
 * chooses the canonical parenthesized form.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 *
 * The grammar test suite must cover:
 *
 *     assert(true);
 *
 *     assert(condition);
 *
 *     assert(value == expected);
 *
 *     assert(condition, "message");
 *
 *     assert(condition, diagnostic);
 *
 *     assert(computation());
 *
 *     assert((nested_expression));
 *
 *     assert(quantum_related_classical_condition);
 *
 *     assert(hardware_related_condition);
 *
 *     assert(distributed_condition);
 *
 *     assert(ai_or_data_condition);
 *
 *     nested assertions inside blocks;
 *
 *     assertions inside conditional branches;
 *
 *     assertions inside loops;
 *
 *     assertions surrounding domain statements.
 *
 *
 * NEGATIVE TESTS
 *
 * The grammar must reject:
 *
 *     assert;
 *
 *     assert();
 *
 *     assert(, message);
 *
 *     assert(condition,);
 *
 *     assert(condition, message, extra);
 *
 *     assert(condition) unexpected;
 *
 *     assert((condition);
 *
 *     assert(condition));
 *
 *
 * BOUNDARY TESTS
 *
 * Verify:
 *
 *     deeply nested expressions;
 *     very large assertion expressions;
 *     many assertions in a source unit;
 *     large explanation expressions;
 *     large blocks containing assertions.
 *
 * The grammar must not contain arbitrary numeric limits for any of these.
 *
 *
 * CROSS-DOMAIN TESTS
 *
 * Test assertions occurring in programs combining:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + HDL
 *     quantum + hardware
 *     quantum + distributed
 *     AI + quantum
 *     AI + hardware
 *     classical + quantum + distributed
 *     classical + quantum + HDL + hardware
 *
 *
 * DETERMINISM TESTS
 *
 * Identical source and parser configuration must yield identical parse
 * structure regardless of:
 *
 *     machine size
 *     CPU count
 *     GPU count
 *     QPU availability
 *     FPGA availability
 *     cluster size
 *     runtime state.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain:
 *
 *     no fixed resource counts;
 *     no fixed machine dimensions;
 *     no fixed assertion count;
 *     no fixed expression size;
 *     no fixed nesting depth;
 *     no device identifiers;
 *     no topology assumptions;
 *     no backend assumptions;
 *     no target-specific syntax.
 *
 * Any finite limit introduced by an implementation must be represented by an
 * external parser/compiler resource policy rather than by grammar literals.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before marking this file complete:
 *
 * [ ] `AssertionsParser` is the canonical parser-fragment name.
 *
 * [ ] `tokenVocab = ZamaniLexer` matches the repository's canonical lexer.
 *
 * [ ] `ASSERT` is supplied by the canonical lexer.
 *
 * [ ] `LPAREN`, `RPAREN`, `COMMA`, and `SEMICOLON` are supplied by the
 *     canonical lexer.
 *
 * [ ] `expression` resolves to the canonical expression grammar.
 *
 * [ ] `statementTerminator` resolves to the canonical statement grammar.
 *
 * [ ] No duplicate `statement` rule exists here.
 *
 * [ ] No duplicate expression grammar exists here.
 *
 * [ ] No compile-time assertion rule is duplicated here.
 *
 * [ ] No contract/invariant grammar is duplicated here.
 *
 * [ ] `statements.g4` admits `assertionStatement`.
 *
 * [ ] The AST layer has a corresponding assertion-statement representation.
 *
 * [ ] Semantic analysis consumes the AST representation.
 *
 * [ ] Assertion lowering does not create a second quantum IR.
 *
 * [ ] No runtime/hardware dependency exists in this grammar.
 *
 * [ ] No unsafe Rust is introduced by this grammar.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests cover the previous Zamani assertion syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     1. assertionStatement is the sole owner of statement-level assertion
 *        syntax;
 *
 *     2. the lexer provides ASSERT and required punctuation;
 *
 *     3. expressions are delegated to the canonical expression grammar;
 *
 *     4. statement termination is delegated to the canonical statement
 *        termination grammar;
 *
 *     5. compile-time assertions remain owned by compile-time expressions;
 *
 *     6. contracts and formal verification remain independently owned;
 *
 *     7. statements.g4 integrates assertionStatement exactly once;
 *
 *     8. the frontend AST preserves condition, optional explanation, and
 *        source span;
 *
 *     9. semantic analysis validates assertion types and behavior;
 *
 *    10. no target-specific or hardware-specific assumptions exist;
 *
 *    11. no machine/resource cardinality is hard-coded;
 *
 *    12. all required positive, negative, boundary, cross-domain and
 *        determinism tests pass;
 *
 *    13. the grammar remains independent of quantum::ir, QEC, ZQN, routing,
 *        scheduling, hardware discovery, calibration and runtime execution;
 *
 *    14. the implementation remains compatible with Rust 1.97 / 1.97.1
 *        because the grammar introduces no Rust implementation dependency;
 *
 *    15. the assembled ANTLR grammar has no duplicate or ambiguous assertion
 *        ownership.
 *
 * ============================================================================
 */