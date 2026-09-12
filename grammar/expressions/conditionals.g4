/*
 * Zamani Programming Language
 * File: grammar/expressions/conditionals.g4
 *
 * Purpose
 * -------
 * Defines value-producing conditional expressions.
 *
 * Architectural ownership
 * -----------------------
 * This grammar owns:
 *   - conditional expressions
 *   - branch expressions
 *   - conditional-expression chaining
 *   - value-producing `if` expressions
 *
 * This grammar does NOT own:
 *   - statement-level `if` control flow
 *   - loops
 *   - pattern matching
 *   - exceptions
 *   - scheduling
 *   - hardware placement
 *   - resource allocation
 *   - quantum execution
 *   - quantum IR
 *   - classical IR
 *   - runtime decisions
 *   - target selection
 *   - optimization
 *
 * Semantic boundary
 * -----------------
 * This file defines syntax only.
 *
 * Parser output is consumed by the Zamani AST/frontend layer.
 * Semantic analysis determines:
 *   - condition type validity
 *   - branch type compatibility
 *   - reachability
 *   - effect compatibility
 *   - capability requirements
 *   - contextual typing
 *
 * This grammar MUST NOT encode:
 *   - maximum branch count
 *   - maximum nesting depth
 *   - maximum expression size
 *   - machine size
 *   - qubit count
 *   - CPU/GPU/FPGA count
 *   - memory capacity
 *   - topology
 *   - device identifiers
 *   - timing assumptions
 *
 * Scalability
 * -----------
 * Repetition is represented recursively/iteratively through parser rules
 * rather than finite enumerations.
 *
 * POCO-REAF
 * ---------
 * Conditional expressions describe program semantics, not the machine on
 * which those semantics are executed.
 *
 * Example:
 *
 *   let result =
 *       if temperature > threshold {
 *           cool()
 *       } else {
 *           continue_work()
 *       };
 *
 * The grammar does not decide whether this executes on a CPU, GPU, FPGA,
 * quantum-classical system, distributed machine, or another target.
 *
 * Rust compatibility
 * ------------------
 * This grammar is consumed by the repository's ANTLR/frontend infrastructure.
 * Generated Rust integration MUST remain compatible with Rust 1.97/1.97.1
 * and MUST NOT require unsafe Rust.
 *
 * IMPORTANT
 * ---------
 * Token names referenced here are intentionally kept to the lexical/core
 * contract:
 *
 *   IF
 *   ELSE
 *
 * Braces and semicolon tokens are expected to come from the canonical lexer.
 *
 * The expression rule referenced below MUST be the canonical expression
 * entry rule owned by grammar/expressions/expressions.g4.
 *
 * Do not introduce another expression hierarchy here.
 */

parser grammar ConditionalsParser;

/*
 * Import/grammar integration
 * --------------------------
 *
 * The repository's grammar composition layer should import this grammar
 * alongside the other expression fragments.
 *
 * Conceptual integration:
 *
 *   expressions
 *       ├── unary
 *       ├── binary
 *       ├── calls
 *       ├── indexing
 *       ├── member-access
 *       ├── ranges
 *       ├── conditionals   <-- this file
 *       ├── lambdas
 *       └── comprehensions
 *
 * `expression` remains the canonical expression entry point.
 *
 * If the repository uses a combined grammar rather than ANTLR grammar
 * imports, these rules must be incorporated into the canonical expression
 * grammar without duplicating the expression rule.
 */

/*
 * --------------------------------------------------------------------------
 * CONDITIONAL EXPRESSIONS
 * --------------------------------------------------------------------------
 *
 * A conditional expression evaluates to a value.
 *
 * Canonical form:
 *
 *   if <condition> {
 *       <expression>
 *   } else {
 *       <expression>
 *   }
 *
 * Optional chained branches:
 *
 *   if condition_a {
 *       value_a
 *   } else if condition_b {
 *       value_b
 *   } else {
 *       value_c
 *   }
 *
 * The grammar permits arbitrary nesting and chaining through parser
 * recursion/repetition. No fixed branch count is encoded.
 */

conditionalExpression
    : IF expression conditionalBranch elseIfBranch* ELSE conditionalBranch
    ;

/*
 * --------------------------------------------------------------------------
 * BRANCH
 * --------------------------------------------------------------------------
 *
 * A branch is an expression-producing block.
 *
 * The exact block grammar must remain owned by the core/statement grammar.
 * This rule deliberately uses a canonical block construct rather than
 * defining another block syntax.
 *
 * Replace `blockExpression` with the repository's authoritative block
 * expression rule if its existing name differs.
 */

conditionalBranch
    : blockExpression
    ;

/*
 * --------------------------------------------------------------------------
 * ELSE-IF
 * --------------------------------------------------------------------------
 *
 * `else if` is represented as a recursive conditional branch rather than
 * being expanded into a finite number of alternatives.
 *
 * This permits arbitrary nesting without introducing an artificial grammar
 * limit.
 */

elseIfBranch
    : ELSE IF expression conditionalBranch
    ;

/*
 * --------------------------------------------------------------------------
 * BLOCK EXPRESSION CONTRACT
 * --------------------------------------------------------------------------
 *
 * `blockExpression` is intentionally referenced rather than redefined.
 *
 * Its owning grammar must provide:
 *
 *   blockExpression
 *
 * with semantics equivalent to:
 *
 *   { ... }
 *
 * The block may contain the language's permitted expression-producing
 * statements and/or a final value expression according to the canonical
 * Zamani block-expression specification.
 *
 * Ownership:
 *   grammar/statements/blocks.g4
 *   or the repository's authoritative block-expression grammar.
 *
 * This file must never create a second block grammar.
 */

/*
 * --------------------------------------------------------------------------
 * INTEGRATION CONTRACT
 * --------------------------------------------------------------------------
 *
 * expressions.g4
 * --------------
 * Must expose `conditionalExpression` as one expression alternative.
 *
 * Conceptually:
 *
 *   expression
 *       : ...
 *       | conditionalExpression
 *       | ...
 *       ;
 *
 * The actual precedence/associativity structure belongs to expressions.g4.
 *
 * conditionals.g4 MUST NOT redefine `expression`.
 *
 *
 * statements/conditionals.g4
 * --------------------------
 * Owns statement-level:
 *
 *   if (...)
 *   ...
 *   else
 *   ...
 *
 * It MUST NOT redefine `conditionalExpression`.
 *
 *
 * core/block grammar
 * ------------------
 * Owns block syntax.
 *
 * This grammar consumes the canonical block-expression rule.
 *
 *
 * lexer
 * -----
 * Supplies:
 *
 *   IF
 *   ELSE
 *
 * along with canonical braces and other punctuation.
 *
 * Keywords MUST NOT be duplicated here.
 *
 *
 * AST
 * ---
 * The parser layer should lower this syntax into the canonical conditional
 * expression AST node.
 *
 * Recommended semantic shape:
 *
 *   ConditionalExpression {
 *       condition
 *       then_branch
 *       else_if_branches
 *       else_branch
 *       source_span
 *   }
 *
 * The AST owns source structure.
 * It does not own machine resources or execution decisions.
 *
 *
 * Semantic analysis
 * -----------------
 * Semantic analysis must verify:
 *
 *   1. condition is a valid boolean/conditional value
 *   2. every branch is semantically valid
 *   3. branch result types can be unified
 *   4. effects are compatible with the enclosing context
 *   5. required capabilities are available
 *   6. unreachable branches are diagnosed where applicable
 *   7. branch-local bindings obey scope rules
 *
 * These checks MUST NOT be performed by the grammar.
 *
 *
 * IR
 * --
 * This grammar does not create IR.
 *
 * Classical conditional expressions eventually lower through the repository's
 * canonical classical/control-flow representation.
 *
 * Quantum conditional expressions remain syntax until semantic lowering.
 *
 * If a conditional controls quantum operations, the frontend/semantic layer
 * determines the corresponding representation before integration with
 * `quantum::ir`.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT define quantum IR nodes.
 *
 *
 * QEC / ZQN
 * --------
 * No direct dependency.
 *
 * A conditional expression may eventually influence quantum error-correction
 * or noise-aware execution, but those semantics belong to their respective
 * subsystems after lowering.
 *
 *
 * Scheduling
 * ----------
 * No direct dependency.
 *
 * Scheduling may later schedule operations resulting from a conditional.
 * The grammar must not encode scheduling decisions.
 *
 *
 * Routing
 * -------
 * No direct dependency.
 *
 * Routing may later determine physical realization.
 *
 *
 * Optimization
 * ------------
 * No direct dependency.
 *
 * Optimizers may simplify conditional expressions after semantic lowering.
 *
 *
 * Hardware
 * --------
 * No direct dependency.
 *
 * Hardware capabilities are interpreted after parsing and semantic analysis.
 *
 *
 * Runtime
 * -------
 * No direct dependency.
 *
 * Runtime execution semantics are downstream of compilation/lowering.
 */

/*
 * --------------------------------------------------------------------------
 * SEMANTIC REQUIREMENTS
 * --------------------------------------------------------------------------
 *
 * A conditional expression is a value-producing construct.
 *
 * Therefore:
 *
 *   if condition {
 *       expression_a
 *   } else {
 *       expression_b
 *   }
 *
 * must have a semantic result.
 *
 * The branches must be compatible according to Zamani's type system.
 *
 * The grammar deliberately does NOT force the branches to have identical
 * syntactic forms.
 *
 * Examples that may be semantically valid depending on the type system:
 *
 *   if ready { 1 } else { 0 }
 *
 *   if enabled {
 *       compute()
 *   } else {
 *       fallback()
 *   }
 *
 *   if mode == Quantum {
 *       quantum_result()
 *   } else {
 *       classical_result()
 *   }
 *
 * Whether the last example is valid is a semantic/type/capability question,
 * not a grammar question.
 */

/*
 * --------------------------------------------------------------------------
 * NESTING
 * --------------------------------------------------------------------------
 *
 * Conditional expressions may occur anywhere the canonical `expression`
 * rule is accepted.
 *
 * Examples:
 *
 *   let x = if condition {
 *       1
 *   } else {
 *       2
 *   };
 *
 *   foo(
 *       if condition {
 *           a
 *       } else {
 *           b
 *       }
 *   );
 *
 *   array[
 *       if condition {
 *           index_a
 *       } else {
 *           index_b
 *       }
 *   ];
 *
 *   return if condition {
 *       value_a
 *   } else {
 *       value_b
 *   };
 *
 * No finite nesting limit is encoded.
 */

/*
 * --------------------------------------------------------------------------
 * ASSOCIATION / AMBIGUITY
 * --------------------------------------------------------------------------
 *
 * The grammar must distinguish:
 *
 *   if A {
 *       X
 *   } else if B {
 *       Y
 *   } else {
 *       Z
 *   }
 *
 * from statement-level conditional constructs.
 *
 * The explicit `conditionalExpression` rule requires a final `else`.
 *
 * This is intentional.
 *
 * A value-producing conditional without an else branch has no universally
 * valid result unless the language type system explicitly provides an
 * option/unit/partial-value semantics.
 *
 * Therefore:
 *
 *   if condition { value }
 *
 * is NOT accepted by this grammar.
 *
 * If Zamani later defines an optional conditional expression, it must receive
 * a separate semantic design rather than weakening this rule accidentally.
 */

/*
 * --------------------------------------------------------------------------
 * NO MACHINE LIMITS
 * --------------------------------------------------------------------------
 *
 * This grammar introduces no constants representing:
 *
 *   MAX_BRANCHES
 *   MAX_DEPTH
 *   MAX_EXPRESSIONS
 *   MAX_NODES
 *   MAX_THREADS
 *   MAX_CORES
 *   MAX_QUBITS
 *   MAX_DEVICES
 *   MAX_MEMORY
 *
 * Any parser implementation limits are implementation/runtime concerns and
 * MUST NOT become language semantic restrictions.
 *
 * Resource exhaustion must be handled by the parser infrastructure through
 * bounded execution/configurable resource policies where required, without
 * changing the language's semantic grammar.
 */

/*
 * --------------------------------------------------------------------------
 * DETERMINISM
 * --------------------------------------------------------------------------
 *
 * Given identical source text and grammar/version configuration, parsing must
 * produce the same parse structure.
 *
 * No runtime hardware state may influence parsing.
 *
 * No:
 *
 *   CPU count
 *   GPU availability
 *   quantum backend
 *   topology
 *   network state
 *   calibration state
 *   scheduler state
 *
 * may alter the interpretation of this grammar.
 */

/*
 * --------------------------------------------------------------------------
 * SECURITY
 * --------------------------------------------------------------------------
 *
 * Grammar parsing must be pure with respect to external resources.
 *
 * This grammar:
 *
 *   - performs no filesystem access
 *   - performs no network access
 *   - performs no process execution
 *   - performs no device discovery
 *   - performs no hardware probing
 *   - performs no runtime dispatch
 *
 * Macro/metaprogramming facilities, if later permitted, must remain governed
 * by their own security/capability model.
 */

/*
 * --------------------------------------------------------------------------
 * VERSIONING
 * --------------------------------------------------------------------------
 *
 * Changes to this rule affect the Zamani language grammar version.
 *
 * Compatibility handling belongs to:
 *
 *   grammar/compatibility/
 *   grammar/specification/language-version.md
 *
 * This file should not silently introduce syntax incompatible with an existing
 * stable language version.
 */

/*
 * --------------------------------------------------------------------------
 * ERROR RECOVERY
 * --------------------------------------------------------------------------
 *
 * Parser diagnostics should identify:
 *
 *   - missing condition
 *   - malformed condition
 *   - missing opening brace
 *   - malformed branch
 *   - missing `else`
 *   - malformed `else if`
 *   - malformed final branch
 *
 * Diagnostic wording and structured error representation belong to the
 * frontend/parser diagnostic subsystem, not this grammar file.
 */

/*
 * --------------------------------------------------------------------------
 * TEST CONTRACT
 * --------------------------------------------------------------------------
 *
 * Required positive tests:
 *
 *   1. Basic conditional expression.
 *   2. Multiple else-if branches.
 *   3. Nested conditional expression.
 *   4. Conditional used as a function argument.
 *   5. Conditional used as an index.
 *   6. Conditional used as an initializer.
 *   7. Conditional used as a return expression.
 *   8. Conditional inside another conditional.
 *   9. Classical conditional.
 *  10. Quantum/classical conditional syntax where the surrounding grammar
 *      permits it.
 *  11. HDL expression context where expressions are legal.
 *  12. Distributed/accelerator expression context where expressions are legal.
 *
 * Required negative tests:
 *
 *   1. Missing condition.
 *   2. Missing opening brace.
 *   3. Missing closing brace.
 *   4. Missing else.
 *   5. Missing else branch.
 *   6. Malformed else-if.
 *   7. Invalid token between condition and branch.
 *   8. Statement-only construct incorrectly embedded as an expression.
 *
 * Boundary tests:
 *
 *   1. Deeply nested conditional expressions.
 *   2. Long else-if chains.
 *   3. Large branch bodies.
 *   4. Large expressions in conditions.
 *   5. Large expressions in branch results.
 *
 * Scalability tests:
 *
 *   Verify that no source-level limit is imposed on:
 *
 *     - branch count
 *     - nesting depth
 *     - expression size
 *     - program size
 *
 *   Any practical parser-resource limit must be external to the language
 *   semantics and configurable by the compiler/frontend infrastructure.
 *
 * Determinism tests:
 *
 *   Parse identical source repeatedly and verify identical parse/AST output.
 *
 * Round-trip tests:
 *
 *   source
 *     -> lexer
 *     -> parser
 *     -> AST
 *     -> formatter/printer
 *     -> parser
 *
 *   must preserve conditional-expression semantics.
 */

/*
 * --------------------------------------------------------------------------
 * COMPLETION CRITERIA
 * --------------------------------------------------------------------------
 *
 * This file is complete only when:
 *
 * [ ] Conditional-expression ownership is documented.
 * [ ] Statement-level conditionals are not duplicated here.
 * [ ] Canonical expression integration is established.
 * [ ] Canonical block-expression integration is established.
 * [ ] IF and ELSE come exclusively from the canonical lexer.
 * [ ] No lexer tokens are redefined here.
 * [ ] No AST types are defined here.
 * [ ] No IR types are defined here.
 * [ ] No quantum IR is defined here.
 * [ ] No hardware assumptions exist.
 * [ ] No machine-size limits exist.
 * [ ] No resource counts are hard-coded.
 * [ ] Arbitrary valid nesting is syntactically supported.
 * [ ] Arbitrary valid else-if chains are syntactically supported.
 * [ ] Diagnostics are covered by parser tests.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Determinism tests exist.
 * [ ] Round-trip tests exist.
 * [ ] Rust frontend integration is compatible with Rust 1.97/1.97.1.
 * [ ] No unsafe Rust is required.
 * [ ] Repository-wide grammar composition is validated.
 * [ ] AST lowering is validated.
 * [ ] Semantic analysis integration is validated.
 * [ ] Downstream IR integration is validated without creating a grammar→IR
 *     dependency cycle.
 */