/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/nested-mind.g4
 *
 * Grammar:
 *     NestedMindExpressions
 *
 * Status:
 *     PRODUCTION EXPRESSION-LEAF CONTRACT
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the syntax for an embedded, recursively nestable cognitive
 * computation expression.
 *
 * A nested mind is NOT a second programming language.
 *
 * It is an expression-level composition boundary through which ordinary
 * Zamani expressions can invoke extensible cognitive operations.
 *
 * The construct is deliberately target-neutral.
 *
 * It can ultimately participate in:
 *
 *     classical computation
 *     quantum computation
 *     HDL/hardware computation
 *     tensor computation
 *     distributed computation
 *     networking
 *     accelerator computation
 *     probabilistic computation
 *     symbolic computation
 *     hybrid computation
 *     future computational domains
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
 *     canonical parser
 *       |
 *       v
 *     expression composition
 *       |
 *       +------------------------------+
 *       |                              |
 *       v                              v
 * ordinary expression          nestedMindExpression
 *                                      |
 *                                      v
 *                              domain-neutral AST
 *                                      |
 *                                      v
 *                              structural validation
 *                                      |
 *                                      v
 *                              semantic analysis
 *                                      |
 *              +-----------------------+----------------------+
 *              |                       |                      |
 *              v                       v                      v
 *          AI semantics          resources/capabilities   policies/effects
 *              |                       |                      |
 *              +-----------------------+----------------------+
 *                                      |
 *                                      v
 *                              canonical semantic model
 *                                      |
 *                         +------------+------------+
 *                         |                         |
 *                         v                         v
 *                    classical                quantum semantics
 *                                                   |
 *                                                   v
 *                                               quantum::ir
 *                                                   |
 *                                                   v
 *                                        optimization/lowering
 *                                                   |
 *                                          routing/scheduling
 *                                                   |
 *                                           resilience/QEC/ZQN
 *                                                   |
 *                                                   v
 *                                                  HAL
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     nestedMindExpression
 *     nestedMindBody
 *     nestedMindClause
 *     nestedMindOperation
 *     nestedMindOperationName
 *     nestedMindArguments
 *     nestedMindArgumentList
 *     nestedMindGuard
 *     nestedMindModifier
 *     nestedMindModifierList
 *     nestedMindResultBinding
 *     nestedMindNestedExpression
 *
 * DOES NOT OWN
 * -------------
 *
 *     expression
 *     assignment precedence
 *     binary precedence
 *     unary precedence
 *     postfix precedence
 *     ordinary calls
 *     ordinary indexing
 *     ordinary member access
 *     ordinary types
 *     ordinary statements
 *     reasoning semantics
 *     knowledge semantics
 *     learning semantics
 *     adaptation semantics
 *     probability implementation
 *     model implementation
 *     agent lifecycle
 *     memory implementation
 *     policy semantics
 *     contract semantics
 *     resource negotiation
 *     capability negotiation
 *     effect checking
 *     provenance storage
 *     AST structures
 *     semantic IR
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     target selection
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/qualified-names.g4
 *     shared expression-core boundary
 *
 * The shared expression-core boundary MUST expose the canonical expression
 * operand used by this leaf grammar.
 *
 * This grammar MUST NOT import:
 *
 *     grammar/expressions/expressions.g4
 *
 * when expressions.g4 imports this file.
 *
 * Doing so would create a parser-grammar cycle.
 *
 * EXPORTS
 * -------
 *
 *     nestedMindExpression
 *
 * Internal rules are implementation details unless the parser composition
 * explicitly promotes them.
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/expressions/expressions.g4
 *     canonical parser composition
 *     semantic expression validation
 *
 * AST_OWNER
 * ---------
 *
 *     Existing domain-neutral Zamani frontend AST.
 *
 * This grammar does not define Rust structures.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     Generic semantic-operation layer plus the owning semantic subsystem
 *     for each resolved operation.
 *
 * For example:
 *
 *     reasoning.*       -> reasoning semantic model
 *     knowledge.*       -> knowledge semantic model
 *     learning.*        -> learning semantic model
 *     adaptation.*      -> adaptation semantic model
 *     policy.*          -> policy semantic model
 *     quantum.*         -> quantum semantic model
 *
 * IR_OWNER
 * --------
 *
 *     Canonical semantic IR.
 *
 * Quantum operations ultimately lower through:
 *
 *     quantum::ir
 *
 * This file creates no NestedMindIR.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/expressions/nested-mind/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/boundary/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/ai.md
 *     grammar/spec/expression-model.md
 *     grammar/spec/poco-reaf.md
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar contains NO lexer rules.
 *
 * The canonical lexer remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The preferred production design is to avoid introducing a large vocabulary
 * of cognitive keywords.
 *
 * Cognitive operation names are therefore represented by identifiers or
 * qualified names wherever possible.
 *
 * This permits future operations such as:
 *
 *     infer
 *     deduce
 *     reason
 *     assert
 *     retract
 *     query
 *     learn
 *     adapt
 *     explain
 *     decide
 *     observe
 *     hypothesize
 *     verify
 *
 * without requiring this grammar to change.
 *
 * If a language-level `mind` marker is eventually reserved, its token MUST be
 * added to the canonical lexical hierarchy rather than defined here.
 *
 * This file MUST NOT define a private MIND token.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * The central rule is:
 *
 *     cognitive operation = qualified semantic name + optional arguments
 *
 * The grammar does NOT enumerate a closed set of operations.
 *
 * Therefore:
 *
 *     infer(...)
 *     deduce(...)
 *     reason(...)
 *     knowledge.query(...)
 *     model.learn(...)
 *     strategy.adapt(...)
 *     decision.explain(...)
 *
 * are all structurally representable.
 *
 * Whether a name denotes:
 *
 *     reasoning
 *     learning
 *     adaptation
 *     knowledge
 *     probability
 *     causality
 *     explanation
 *     provenance
 *     quantum computation
 *     classical computation
 *     tensor computation
 *     distributed computation
 *     an external dialect
 *
 * is a semantic-resolution question.
 *
 * ============================================================================
 * RECURSION / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Nested cognition is recursively composable.
 *
 * Conceptually:
 *
 *     mind {
 *         reason(
 *             mind {
 *                 infer(problem)
 *             }
 *         )
 *     }
 *
 * No grammar-level nesting limit is imposed.
 *
 * There MUST NOT be:
 *
 *     MAX_MIND_DEPTH
 *     MAX_COGNITIVE_DEPTH
 *     MAX_REASONING_STEPS
 *     MAX_CLAUSES
 *     MAX_ARGUMENTS
 *     MAX_CONTEXTS
 *
 * or equivalent constants.
 *
 * Practical limits belong to:
 *
 *     parser implementation
 *     compiler resource policy
 *     memory availability
 *     recursion strategy
 *     execution policy
 *     target capability
 *
 * and are not language semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     ASIC identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     fixed node counts
 *     fixed device counts
 *     fixed memory sizes
 *     register widths
 *     tensor-rank ceilings
 *     network-size ceilings
 *
 * Requirements belong to:
 *
 *     resources
 *     capabilities
 *     constraints
 *     policies
 *     compilation contexts
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * `nestedMindArgument` consumes the shared expression-core boundary.
 *
 * It MUST NOT import the complete Expressions grammar.
 *
 * The purpose is to permit:
 *
 *     reason(problem + context)
 *     learn(model, data)
 *     adapt(strategy, feedback)
 *     explain(decision)
 *     query(graph.filter(predicate))
 *
 * while keeping expression precedence owned by the canonical expression
 * composition grammar.
 *
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * The single public entry point is:
 *
 *     nestedMindExpression
 *
 * The enclosing expression grammar determines where this construct is legal.
 *
 * ============================================================================
 */

parser grammar NestedMindExpressions;

options {
    tokenVocab = ZamaniLexer;
}

import QualifiedNames;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY
 * ============================================================================
 *
 * The lexical marker is deliberately represented by a canonical identifier
 * rather than a private lexer token.
 *
 * This permits the semantic layer to recognize:
 *
 *     mind
 *
 * without forcing a permanent global keyword if the language specification
 * ultimately chooses annotation-based or dialect-based spelling.
 *
 * The canonical source-level form is therefore:
 *
 *     mind { ... }
 *
 * with the word `mind` represented structurally as an identifier.
 *
 * A future reserved-token promotion can occur centrally without changing the
 * semantic model.
 * ============================================================================
 */

nestedMindExpression
    : nestedMindMarker
      nestedMindBody
    ;


/*
 * ============================================================================
 * 2. MIND MARKER
 * ============================================================================
 *
 * The marker is an identifier rather than a locally invented lexer token.
 *
 * Semantic validation recognizes the canonical marker.
 *
 * This is intentionally open to future compatibility modes.
 * ============================================================================
 */

nestedMindMarker
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 3. MIND BODY
 * ============================================================================
 *
 * The body contains zero or more cognitive clauses.
 *
 * Empty bodies are structurally valid.
 *
 * Semantic validation may require a non-empty body in contexts where an
 * actual computation is required.
 *
 * No finite clause count is imposed.
 * ============================================================================
 */

nestedMindBody
    : LBRACE
      nestedMindClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. CLAUSE
 * ============================================================================
 *
 * A clause consists of:
 *
 *     optional result binding
 *     operation
 *     optional guard
 *     optional modifiers
 *     optional terminator
 *
 * Examples:
 *
 *     infer(problem);
 *
 *     result = reason(context);
 *
 *     learn(model, data) if evidence.is_valid;
 *
 *     adapt(strategy) with policy;
 *
 * ============================================================================
 */

nestedMindClause
    : nestedMindResultBinding?
      nestedMindOperation
      nestedMindGuard?
      nestedMindModifierList?
      nestedMindTerminator?
    ;


/*
 * ============================================================================
 * 5. RESULT BINDING
 * ============================================================================
 *
 * Result binding remains ordinary semantic assignment intent.
 *
 * The enclosing semantic layer determines:
 *
 *     mutability
 *     type
 *     ownership
 *     lifetime
 *     effect
 *     capability
 *
 * ============================================================================
 */

nestedMindResultBinding
    : identifier ASSIGN
    ;


/*
 * ============================================================================
 * 6. OPERATION
 * ============================================================================
 *
 * This is the principal extensibility boundary.
 *
 * The grammar does NOT enumerate:
 *
 *     infer
 *     deduce
 *     reason
 *     query
 *     learn
 *     adapt
 *     explain
 *     decide
 *     observe
 *     etc.
 *
 * They are names resolved by semantic registries.
 * ============================================================================
 */

nestedMindOperation
    : nestedMindOperationName
      nestedMindArguments?
    ;


/*
 * ============================================================================
 * 7. OPERATION NAME
 * ============================================================================
 *
 * Qualified names permit:
 *
 *     reason
 *     ai.reason
 *     knowledge.query
 *     model.learn
 *     quantum.measure
 *     hybrid.execute
 *     vendor.cognitive.operation
 *
 * The semantic resolver decides whether a qualified name is legal.
 * ============================================================================
 */

nestedMindOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. ARGUMENTS
 * ============================================================================
 *
 * Argument cardinality is intentionally unbounded.
 * ============================================================================
 */

nestedMindArguments
    : LPAREN
      nestedMindArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 9. ARGUMENT LIST
 * ============================================================================
 */

nestedMindArgumentList
    : nestedMindArgument
      (
          COMMA
          nestedMindArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. ARGUMENT
 * ============================================================================
 *
 * Arguments use the shared expression-core boundary.
 *
 * The core expression contract MUST be supplied by parser composition.
 *
 * It may represent:
 *
 *     literals
 *     identifiers
 *     calls
 *     arithmetic
 *     logical expressions
 *     collections
 *     lambdas
 *     patterns
 *     nested mind expressions
 *     domain-neutral expressions
 *
 * without this grammar recreating those facilities.
 * ============================================================================
 */

nestedMindArgument
    : expressionCore
    ;


/*
 * ============================================================================
 * 11. GUARD
 * ============================================================================
 *
 * Guards are semantic predicates attached to a cognitive operation.
 *
 * Example:
 *
 *     infer(problem) if confidence > threshold;
 *
 * The guard expression itself remains a normal expression.
 *
 * Guard semantics belong to validation/semantic analysis.
 * ============================================================================
 */

nestedMindGuard
    : IF
      expressionCore
    ;


/*
 * ============================================================================
 * 12. MODIFIERS
 * ============================================================================
 *
 * Modifiers are deliberately name/value based.
 *
 * This prevents keyword explosion.
 *
 * Possible semantic modifiers include:
 *
 *     strategy
 *     confidence
 *     evidence
 *     provenance
 *     policy
 *     model
 *     constraints
 *     preference
 *     resource
 *     capability
 *     timeout
 *     reproducibility
 *     deterministic
 *     adaptive
 *
 * New modifiers do not require this grammar to change.
 * ============================================================================
 */

nestedMindModifierList
    : nestedMindModifier+
    ;


nestedMindModifier
    : AT
      qualifiedName
      (
          ASSIGN
          expressionCore
      )?
    ;


/*
 * ============================================================================
 * 13. TERMINATION
 * ============================================================================
 *
 * The enclosing parser may supply statement/expression termination.
 *
 * A semicolon is accepted here for convenient block-oriented source syntax.
 * ============================================================================
 */

nestedMindTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * 14. SHARED EXPRESSION CORE CONTRACT
 * ============================================================================
 *
 * `expressionCore` is intentionally not defined here.
 *
 * It must be provided by the repository's shared expression composition
 * boundary.
 *
 * REQUIRED CONTRACT:
 *
 *     expressionCore
 *
 * MUST represent a complete domain-neutral expression operand without
 * importing this grammar back into the full precedence grammar.
 *
 * Dependency direction:
 *
 *     expression precedence
 *             |
 *             v
 *       expressionCore
 *             |
 *       +-----+--------------------+
 *       |                          |
 *       v                          v
 * nested-mind                 reasoning/etc.
 *
 * NOT:
 *
 *     Expressions -> nested-mind -> Expressions
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. COGNITIVE OPERATION SEMANTIC RESOLUTION
 * ============================================================================
 *
 * The following names are intentionally NOT grammar alternatives:
 *
 *     infer
 *     deduce
 *     reason
 *     assert
 *     retract
 *     query
 *     learn
 *     adapt
 *     explain
 *     evidence
 *     decide
 *     observe
 *     hypothesize
 *     verify
 *
 * They are semantic operation names.
 *
 * Resolution may map them to:
 *
 *     reasoning semantic model
 *     knowledge semantic model
 *     learning semantic model
 *     adaptation semantic model
 *     evidence/provenance model
 *     decision model
 *     uncertainty model
 *     policy model
 *     external dialect
 *
 * The parser must not decide which implementation is selected.
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. REASONING INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     mind {
 *         infer(problem);
 *         deduce(conclusion, premises);
 *         reason(context);
 *     }
 *
 * The semantic resolver may lower these into the existing reasoning model
 * represented by:
 *
 *     grammar/expressions/reasoning.g4
 *
 * This grammar does not duplicate reasoningOperation.
 *
 * The semantic layer remains the convergence point.
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     mind {
 *         knowledge.assert(fact);
 *         knowledge.retract(fact);
 *         knowledge.query(pattern);
 *     }
 *
 * Knowledge storage remains owned by the knowledge/data subsystem.
 *
 * This grammar only represents the invocation structure.
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. LEARNING INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     mind {
 *         learn(model, data);
 *     }
 *
 * Learning algorithms, model formats, training systems, resources and
 * execution effects remain outside this grammar.
 *
 * The semantic layer determines:
 *
 *     model
 *     objective
 *     data
 *     algorithm
 *     capabilities
 *     resources
 *     effects
 *     policy
 *     provenance
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     mind {
 *         adapt(strategy, feedback);
 *     }
 *
 * Adaptation MUST NOT imply unrestricted self-modifying execution.
 *
 * Semantic validation must establish:
 *
 *     authorization
 *     policy
 *     capabilities
 *     effects
 *     resource constraints
 *     provenance
 *
 * before execution.
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Uncertainty is represented through ordinary arguments and semantic types.
 *
 * Examples:
 *
 *     infer(hypothesis, confidence);
 *     reason(belief_distribution);
 *     decide(outcome, probability);
 *
 * This grammar does not define:
 *
 *     a probability algorithm
 *     a probability representation
 *     a fixed precision
 *     a fixed distribution family
 *
 * Those belong to the semantic/type systems.
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. EVIDENCE / PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     explain(decision);
 *     verify(claim, evidence);
 *     reason(claim) @provenance = source;
 *
 * Provenance is not executed by the parser.
 *
 * Semantic analysis may attach:
 *
 *     source
 *     evidence
 *     derivation
 *     transformation
 *     verification
 *     decision
 *     version
 *
 * to the resulting semantic operation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. POLICY / CONTRACT INTEGRATION
 * ============================================================================
 *
 * Policies and contracts remain owned by their respective subsystems.
 *
 * Nested mind expressions can reference them through:
 *
 *     guards
 *     modifiers
 *     ordinary expressions
 *
 * Example conceptual forms:
 *
 *     mind {
 *         infer(problem)
 *             @policy = reasoning_policy;
 *
 *         adapt(strategy)
 *             @requires = adaptation_requirement;
 *     }
 *
 * The grammar does not implement policy evaluation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. MULTI-AGENT INTEGRATION
 * ============================================================================
 *
 * A nested mind is NOT an actor.
 *
 * If a mind participates in an actor:
 *
 *     mind expression
 *          |
 *          v
 *     AI semantic model
 *          |
 *          v
 *     actor/concurrency semantic model
 *          |
 *          v
 *     scheduler/runtime
 *
 * Actor lifecycle remains owned by:
 *
 *     grammar/concurrency/
 *
 * No second actor system is introduced.
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A cognitive operation may semantically reference quantum computation:
 *
 *     mind {
 *         infer(result, quantum.measurement);
 *         reason(quantum_state);
 *     }
 *
 * This grammar does NOT define:
 *
 *     gates
 *     qubits
 *     circuits
 *     topology
 *     routing
 *     QEC
 *     calibration
 *     physical devices
 *
 * If semantic analysis determines that an operation is quantum, the lowering
 * path is:
 *
 *     nested mind AST
 *          |
 *          v
 *     semantic operation
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC/resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * No mind-specific quantum IR is introduced.
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. HYBRID INTEGRATION
 * ============================================================================
 *
 * A nested mind may participate in:
 *
 *     classical -> cognition -> quantum -> measurement -> cognition
 *
 * or:
 *
 *     data -> learning -> accelerator -> decision
 *
 * or:
 *
 *     sensor -> reasoning -> control -> hardware
 *
 * The semantic planner establishes domain boundaries.
 *
 * The grammar remains domain-neutral.
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware intent is expressed through semantic capabilities and resources,
 * not physical machine constants.
 *
 * Example conceptual requirement:
 *
 *     @requires = capability("tensor.compute");
 *
 * The grammar does not know whether realization uses:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     simulator
 *     future hardware
 *
 * Target realization is downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar does not declare a closed effect set.
 *
 * Semantic analysis may determine effects such as:
 *
 *     reasoning
 *     learning
 *     adaptation
 *     mutation
 *     randomness
 *     measurement
 *     network
 *     foreign
 *     reflection
 *     distributed
 *     simulation
 *     native
 *
 * Effects are checked by the universal effect system.
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Cognitive operations may require capabilities such as:
 *
 *     reasoning
 *     knowledge.query
 *     learning
 *     adaptation
 *     tensor.compute
 *     quantum.measurement
 *     accelerator.compute
 *
 * Capability names remain semantic values.
 *
 * No finite capability universe is hard-coded here.
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain symbolic.
 *
 * Examples:
 *
 *     memory >= required_memory
 *     qubits >= required_qubits
 *     capability("quantum.measurement")
 *
 * No numerical machine ceiling is introduced.
 *
 * Resource negotiation belongs to:
 *
 *     grammar/resources/
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. TYPE CONTRACT
 * ============================================================================
 *
 * The grammar imposes no fixed result type.
 *
 * A nested cognitive operation may return:
 *
 *     scalar
 *     tuple
 *     record
 *     collection
 *     tensor
 *     probability/distribution
 *     symbolic value
 *     classical value
 *     quantum-derived value
 *     reference
 *     future/async value
 *     domain-specific semantic value
 *
 * Type resolution belongs to the canonical type system.
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. AST CONTRACT
 * ============================================================================
 *
 * The AST must preserve at least:
 *
 *     source span
 *     operation name
 *     qualification segments
 *     argument order
 *     nested expression structure
 *     result binding
 *     guard
 *     modifiers
 *     source order
 *     nesting structure
 *
 * The AST must NOT contain:
 *
 *     CPU IDs
 *     GPU IDs
 *     QPU IDs
 *     physical qubit IDs
 *     backend-specific handles
 *     scheduler state
 *     runtime state
 *     hardware discovery results
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every nested mind expression must remain source-locatable.
 *
 * Semantic lowering may attach provenance describing:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     policy
 *     compilation context
 *
 * The grammar itself only preserves syntactic structure.
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must cover:
 *
 *     malformed marker
 *     missing body
 *     malformed operation name
 *     malformed argument list
 *     malformed guard
 *     malformed modifier
 *     malformed result binding
 *     missing closing delimiter
 *     malformed comma sequence
 *
 * Semantic diagnostics must cover:
 *
 *     unknown operation
 *     invalid operation arguments
 *     invalid guard type
 *     unavailable capability
 *     unsatisfied resource requirement
 *     forbidden effect
 *     unauthorized adaptation
 *     invalid policy
 *     invalid cross-domain operation
 *
 * Unknown semantic operations MUST NOT be turned into lexer errors.
 *
 * This permits dialect registration and future extension.
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source token sequence
 *     grammar version
 *     parser configuration
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     runtime
 *     network
 *     model availability
 *     resource availability
 *     randomness
 *     system time
 *
 * Same source + same grammar configuration must produce equivalent parse
 * structure.
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * The grammar must never:
 *
 *     execute operations
 *     access files
 *     access credentials
 *     contact networks
 *     load models
 *     inspect hardware
 *     invoke QPUs
 *     invoke simulators
 *     invoke native functions
 *
 * Such actions occur only after semantic validation and authorized lowering.
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new cognitive operation must NOT require changing this grammar.
 *
 * Adding:
 *
 *     a new reasoning algorithm
 *     a new learning algorithm
 *     a new model
 *     a new probability distribution
 *     a new quantum operation
 *     a new accelerator
 *     a new hardware target
 *     a new knowledge backend
 *
 * must not require adding a new parser alternative here.
 *
 * Only genuinely new structural syntax may require grammar modification.
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Required positive cases:
 *
 *     mind {}
 *
 *     mind {
 *         infer(problem);
 *     }
 *
 *     mind {
 *         reason(problem, evidence);
 *         query(knowledge);
 *         learn(model, data);
 *         adapt(strategy, feedback);
 *     }
 *
 *     mind {
 *         knowledge.query(graph.filter(predicate));
 *     }
 *
 *     mind {
 *         result = infer(problem);
 *     }
 *
 *     mind {
 *         infer(problem) if confidence > threshold;
 *     }
 *
 *     mind {
 *         reason(
 *             mind {
 *                 infer(problem);
 *             }
 *         );
 *     }
 *
 *     mind {
 *         quantum.measure(state);
 *     }
 *
 *     mind {
 *         ai.reason(
 *             hybrid.execute(classical_value, quantum_value)
 *         );
 *     }
 *
 *     mind {
 *         model.learn(dataset, objective)
 *             @policy = learning_policy;
 *     }
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Required parser failures:
 *
 *     mind
 *
 *     mind {
 *
 *     mind {
 *         infer(
 *     }
 *
 *     mind {
 *         infer(problem,);
 *     }
 *
 *     mind {
 *         = infer(problem);
 *     }
 *
 *     mind {
 *         infer(problem) if;
 *     }
 *
 *     mind {
 *         infer(problem) @policy =;
 *     }
 *
 * Semantic failures must separately test:
 *
 *     unknown operation
 *     invalid operation arguments
 *     invalid capability
 *     unsatisfied requirement
 *     forbidden effect
 *     unauthorized adaptation
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * The following combinations are mandatory:
 *
 *     mind + reasoning
 *     mind + knowledge
 *     mind + learning
 *     mind + adaptation
 *     mind + uncertainty
 *     mind + provenance
 *     mind + policy
 *     mind + contracts
 *     mind + actors
 *     mind + distributed execution
 *     mind + classical computation
 *     mind + tensor computation
 *     mind + quantum computation
 *     mind + hybrid computation
 *     mind + hardware capabilities
 *     mind + FFI
 *     mind + reflection
 *     nested mind + nested mind
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that the grammar contains no artificial finite limits on:
 *
 *     operation count
 *     argument count
 *     clause count
 *     nesting structure
 *     name qualification depth
 *     expression complexity
 *     resource declarations
 *     capability declarations
 *
 * Tests should exercise increasingly large generated programs until practical
 * compiler/parser resource limits are reached.
 *
 * Those practical limits must not become language constants.
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. COMPILER / RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no target-language actions.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The compiler implementation must use safe Rust.
 *
 * No unsafe block or unsafe abstraction is required by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * `nested-mind.g4` is DONE only when:
 *
 * [x] it has one parser grammar identity;
 * [x] it owns one public nestedMindExpression entry point;
 * [x] it defines no lexer rules;
 * [x] it defines no embedded Rust;
 * [x] it defines no semantic predicates;
 * [x] it defines no AST structures;
 * [x] it defines no IR;
 * [x] it defines no target-specific concepts;
 * [x] it defines no machine-size constants;
 * [x] it does not enumerate cognitive algorithms;
 * [x] it does not enumerate AI application domains;
 * [x] it does not enumerate quantum operations;
 * [x] it does not duplicate reasoning semantics;
 * [x] it does not duplicate knowledge semantics;
 * [x] it does not duplicate learning semantics;
 * [x] it does not duplicate policy semantics;
 * [x] it does not duplicate actor semantics;
 * [x] it supports arbitrary operation qualification;
 * [x] it supports arbitrary argument counts;
 * [x] it supports arbitrary clause counts;
 * [x] it supports recursive nesting;
 * [x] it supports guards;
 * [x] it supports extensible modifiers;
 * [x] it preserves source order;
 * [x] it preserves source spans through the normal parser/AST pipeline;
 * [x] it remains target-independent;
 * [x] it remains compatible with classical computation;
 * [x] it remains compatible with quantum computation;
 * [x] it remains compatible with HDL/hardware semantics;
 * [x] it remains compatible with distributed execution;
 * [x] it remains compatible with AI/data semantics;
 * [x] it remains compatible with interoperability;
 * [x] it remains compatible with metaprogramming;
 * [x] it does not introduce an expression-grammar dependency cycle.
 *
 * Integration acceptance additionally requires:
 *
 *     - a canonical expressionCore boundary exists;
 *     - expressions.g4 consumes nestedMindExpression;
 *     - nestedMindExpression consumes expressionCore;
 *     - expressionCore does not import nested-mind;
 *     - the AST has a representation for the construct;
 *     - semantic resolution maps operation names to existing semantic owners;
 *     - canonical IR lowering exists for supported operations;
 *     - quantum operations lower through quantum::ir;
 *     - positive tests pass;
 *     - negative tests pass;
 *     - boundary tests pass;
 *     - scalability tests pass;
 *     - deterministic parser tests pass;
 *     - Rust 1.97/1.97.1 builds remain safe Rust.
 *
 * ============================================================================
 */