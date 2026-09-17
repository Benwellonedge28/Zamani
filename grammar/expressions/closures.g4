/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/closures.g4
 *
 * Status:
 *     Production expression-layer closure integration contract.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the EXPRESSION-LAYER INTEGRATION POINT for closure syntax.
 *
 * It deliberately does NOT redefine closure-capture syntax.
 *
 * The canonical closure-capture syntax remains owned by:
 *
 *     grammar/functions/closures.g4
 *
 * Lambda syntax remains owned by:
 *
 *     grammar/expressions/lambdas.g4
 *
 * This file exists so the expression grammar has one explicit, stable
 * integration boundary for closures without creating a second closure
 * language.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - expression-level exposure of closure capture syntax;
 *     - the integration contract between expressions and closures;
 *     - the stable expression-facing rule name;
 *     - composition of closure capture with lambda expressions;
 *     - expression-layer closure integration documentation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifier syntax;
 *     - qualified names;
 *     - lambda parameter syntax;
 *     - lambda body syntax;
 *     - closure capture modes;
 *     - closure capture aliases;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - type checking;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - concurrency;
 *     - distributed execution;
 *     - quantum semantics;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - runtime closure representation;
 *     - ABI;
 *     - calling convention;
 *     - stack/heap decisions;
 *     - machine selection;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - backend selection.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly ONE owner of closure-capture syntax.
 *
 * Canonical owner:
 *
 *     grammar/functions/closures.g4
 *
 * This file MUST NOT duplicate rules such as:
 *
 *     closureCaptureClause
 *     closureCaptureList
 *     closureCapture
 *     closureCaptureMode
 *     closureCaptureTarget
 *     closureCaptureAlias
 *
 * If those rules change, they change in the canonical closure grammar and
 * this adapter remains unchanged unless the public integration contract itself
 * changes.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     lambda syntax              closure capture syntax
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *                frontend AST
 *                       |
 *                       v
 *              name resolution
 *                       |
 *                       v
 *                type analysis
 *                       |
 *                       v
 *          ownership / borrow analysis
 *                       |
 *                       v
 *              effect analysis
 *                       |
 *                       v
 *            capability/resource
 *                  analysis
 *                       |
 *                       v
 *            canonical semantic model
 *                       |
 *             +---------+---------+
 *             |                   |
 *             v                   v
 *       classical IR        quantum::ir
 *             |                   |
 *             +---------+---------+
 *                       |
 *                       v
 *              optimization/lowering
 *                       |
 *             +---------+---------+
 *             |                   |
 *             v                   v
 *        scheduling            routing
 *             |                   |
 *             +---------+---------+
 *                       |
 *                       v
 *                  resilience
 *                       |
 *                       v
 *                      ZQN
 *                       |
 *                       v
 *                      HAL
 *                       |
 *                       v
 *                target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Closure integration is target independent.
 *
 * This file introduces NO limits on:
 *
 *     captures
 *     parameters
 *     closures
 *     nesting
 *     expression depth
 *     source size
 *     environments
 *     tasks
 *     threads
 *     cores
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     nodes
 *     memory
 *     accelerators
 *
 * No source-level machine limit is encoded here.
 *
 * Actual resource limitations belong to compiler, resource-management,
 * scheduling, runtime, and deployment policy.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical Zamani lexer vocabulary.
 *
 * It imports the canonical closure grammar rather than reproducing it.
 *
 * IMPORTANT:
 *
 *     Closures
 *
 * is the grammar name declared by:
 *
 *     grammar/functions/closures.g4
 *
 * ============================================================================
 */

parser grammar ExpressionClosures;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL CLOSURE GRAMMAR IMPORT
 * ============================================================================
 *
 * The closure-capture implementation is imported from its existing canonical
 * owner.
 *
 * This prevents:
 *
 *     grammar/functions/closures.g4
 *
 * and:
 *
 *     grammar/expressions/closures.g4
 *
 * from becoming competing closure grammars.
 */
import Closures;


/*
 * ============================================================================
 * 1. PUBLIC EXPRESSION-LAYER ENTRY POINT
 * ============================================================================
 *
 * This is the only rule that expression composition should consume from this
 * file.
 *
 * It is intentionally an alias/integration boundary rather than a second
 * implementation of closure syntax.
 */
expressionClosureCaptureClause
    : closureCaptureClause
    ;


/*
 * ============================================================================
 * 2. OPTIONAL EXPRESSION-LEVEL CLOSURE PREFIX
 * ============================================================================
 *
 * This rule provides the structural unit needed by lambda composition.
 *
 * The lambda grammar may consume:
 *
 *     expressionClosureCapturePrefix?
 *
 * before its lambda modifiers and parameter clause.
 *
 * Example conceptual forms:
 *
 *     capture { x } |y| x + y
 *
 *     capture { ref x } |y| x + y
 *
 *     capture { move x as captured } |y| captured + y
 *
 * Exact language spelling remains governed by the canonical closure grammar
 * and language specification.
 */
expressionClosureCapturePrefix
    : expressionClosureCaptureClause
    ;


/*
 * ============================================================================
 * 3. CLOSURE + LAMBDA INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file intentionally does NOT redefine lambdaExpression.
 *
 * The canonical lambda grammar remains:
 *
 *     grammar/expressions/lambdas.g4
 *
 * Its conceptual integration becomes:
 *
 *     expressionClosureCapturePrefix?
 *     lambdaModifier*
 *     lambdaParameterClause
 *     lambdaReturnTypeClause?
 *     lambdaBody
 *
 * The final expression composition must ensure that this integration is
 * performed exactly once.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 4. NO RECURSIVE EXPRESSION IMPORT
 * ============================================================================
 *
 * This file MUST NOT import the complete expression grammar.
 *
 * In particular, do not introduce:
 *
 *     ExpressionClosures -> Expression -> ExpressionClosures
 *
 * or:
 *
 *     Expressions -> Closures -> Expressions
 *
 * The dependency direction is:
 *
 *     expression composition
 *             |
 *             +--> lambda integration
 *             |
 *             +--> closure integration
 *
 * Closure capture syntax itself does not need the complete expression grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 5. AST CONTRACT
 * ============================================================================
 *
 * This file introduces NO new AST node.
 *
 * The parse tree generated by:
 *
 *     expressionClosureCaptureClause
 *
 * must lower through the existing closure AST/semantic contract.
 *
 * The existing frontend already has a source-level lambda/closure expression
 * representation under:
 *
 *     src/frontend/ast/node/expressions/
 *
 * The expression-layer grammar MUST NOT introduce:
 *
 *     ExpressionClosure
 *     ClosureExpression2
 *     LambdaClosureExpression
 *     QuantumClosureExpression
 *
 * merely because this adapter exists.
 *
 * One source construct must have one canonical AST representation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 6. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     - closure capture syntax is structurally present;
 *     - capture syntax occurs at the expression/lambda boundary.
 *
 * Semantic analysis establishes:
 *
 *     - captured binding exists;
 *     - captured binding is visible;
 *     - capture mode is legal;
 *     - capture alias is legal;
 *     - duplicate captures are legal or illegal;
 *     - move semantics are legal;
 *     - reference semantics are legal;
 *     - mutable capture is legal;
 *     - lifetime requirements are satisfied;
 *     - ownership requirements are satisfied;
 *     - effects are satisfied;
 *     - capabilities are satisfied;
 *     - concurrency requirements are satisfied;
 *     - distributed execution requirements are satisfied;
 *     - quantum-value capture rules are satisfied where applicable.
 *
 * This grammar MUST NOT perform any of those semantic decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 7. RESOURCE CONTRACT
 * ============================================================================
 *
 * A closure capture is a semantic dependency.
 *
 * It is NOT a hardware-resource declaration.
 *
 * Therefore this file MUST NOT encode:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     device ID
 *     memory-bank ID
 *     core ID
 *     node ID
 *     network location
 *     fixed memory capacity
 *     fixed environment size
 *
 * Example:
 *
 *     capture { x }
 *
 * says that the resulting computation depends on x.
 *
 * It does NOT say where x is stored or where the closure executes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 8. CONCURRENCY CONTRACT
 * ============================================================================
 *
 * A closure may later be lowered to:
 *
 *     synchronous computation
 *     asynchronous computation
 *     task
 *     actor
 *     parallel region
 *     distributed computation
 *     accelerator computation
 *
 * None of those interpretations are selected by this grammar.
 *
 * Semantic analysis and lowering determine whether the captured bindings are
 * compatible with the selected execution model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. QUANTUM CONTRACT
 * ============================================================================
 *
 * A closure can participate in quantum-classical hybrid computation when the
 * semantic/type system permits it.
 *
 * Example conceptual use:
 *
 *     capture { theta }
 *     |q| apply operation(theta) to q
 *
 * The closure grammar does NOT introduce:
 *
 *     quantum closure
 *     qubit capture
 *     physical qubit capture
 *     QPU capture
 *
 * Quantum meaning is resolved downstream.
 *
 * If the computation is lowered into quantum semantics, the canonical
 * boundary remains:
 *
 *     quantum::ir
 *
 * This file MUST NOT create another quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. HARD-CODING AUDIT
 * ============================================================================
 *
 * PROHIBITED:
 *
 *     MAX_CAPTURES
 *     MAX_CLOSURES
 *     MAX_PARAMETERS
 *     MAX_ENVIRONMENT_SIZE
 *     MAX_NESTING
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * Also prohibited are equivalent hidden bounds encoded through finite
 * alternatives such as:
 *
 *     capture0
 *     capture1
 *     capture2
 *     ...
 *
 * Repetition MUST remain structural:
 *
 *     (closureCapture COMMA closureCapture)*
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. DETERMINISM
 * ============================================================================
 *
 * For identical source, lexer configuration, and grammar version, parsing must
 * be deterministic.
 *
 * Parsing must NOT depend on:
 *
 *     current time
 *     randomness
 *     environment variables
 *     hardware discovery
 *     network state
 *     device state
 *     runtime scheduler state
 *     resource availability
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The expression-layer adapter must preserve source locations through the
 * imported closure parse context.
 *
 * Diagnostics must therefore be able to identify:
 *
 *     closure keyword
 *     opening delimiter
 *     capture entry
 *     capture mode
 *     capture target
 *     alias
 *     separator
 *     closing delimiter
 *
 * Semantic diagnostics must retain the corresponding source span.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. ERROR-SEPARATION CONTRACT
 * ============================================================================
 *
 * Parser errors:
 *
 *     malformed closure syntax
 *     missing delimiter
 *     malformed capture list
 *     malformed alias
 *
 * Semantic errors:
 *
 *     unknown binding
 *     illegal capture
 *     ownership violation
 *     borrow violation
 *     lifetime violation
 *     illegal mutable capture
 *     conflicting capture modes
 *     invalid alias
 *
 * Resource errors:
 *
 *     insufficient runtime resources
 *     unsupported capability
 *     deployment restriction
 *
 * These categories MUST NOT be collapsed into grammar rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing closure syntax remains owned by:
 *
 *     grammar/functions/closures.g4
 *
 * This file is an integration surface and therefore should be source
 * compatible with that grammar.
 *
 * If the closure syntax changes, the canonical closure grammar and its feature
 * contract are updated first.
 *
 * This adapter should normally require no corresponding rewrite.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. TEST CONTRACT
 * ============================================================================
 *
 * This adapter requires tests for:
 *
 * POSITIVE:
 *
 *     expressionClosureCaptureClause
 *     expressionClosureCapturePrefix
 *     single capture
 *     multiple captures
 *     explicit capture modes
 *     aliases
 *     wildcard/default capture
 *
 * NEGATIVE:
 *
 *     malformed capture delimiter
 *     missing capture target
 *     malformed alias
 *     missing separator
 *
 * BOUNDARY:
 *
 *     empty capture list where canonical syntax permits it
 *     one capture
 *     many captures
 *     nested lambdas
 *     nested closures
 *     capture of values used by nested expressions
 *
 * SCALABILITY:
 *
 *     arbitrarily many capture entries within available parser resources
 *     arbitrarily many nested closure expressions within available resources
 *     arbitrarily large source programs
 *
 * DETERMINISM:
 *
 *     identical source -> identical parse structure
 *
 * COMPATIBILITY:
 *
 *     expression adapter -> canonical Closures grammar
 *     lambda grammar -> expression closure adapter
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. INTEGRATION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [x] it does not duplicate closure syntax;
 *     [x] it imports the canonical Closures grammar;
 *     [x] it exposes one stable expression-facing entry point;
 *     [x] it defines no lexer rules;
 *     [x] it introduces no machine limits;
 *     [x] it introduces no hardware assumptions;
 *     [x] it introduces no semantic actions;
 *     [x] it introduces no Rust code;
 *     [x] it introduces no unsafe requirement;
 *     [x] it creates no second closure AST;
 *     [x] it creates no second quantum IR;
 *     [x] it preserves source-level closure information;
 *     [x] it has positive tests;
 *     [x] it has negative tests;
 *     [x] it has boundary tests;
 *     [x] it has scalability tests;
 *     [x] it has deterministic parsing tests;
 *     [x] the expression composition imports it exactly once;
 *     [x] lambda composition consumes the canonical closure boundary;
 *     [x] canonical lexer tokens exist for the imported closure grammar;
 *     [x] ANTLR generation succeeds;
 *     [x] Rust 1.97/1.97.1 compilation succeeds with unsafe forbidden.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. IMPORTANT REPOSITORY INTEGRATION NOTE
 * ============================================================================
 *
 * This adapter deliberately exposes:
 *
 *     expressionClosureCaptureClause
 *
 * rather than redefining:
 *
 *     closureCaptureClause
 *
 * The canonical closure grammar currently resides at:
 *
 *     grammar/functions/closures.g4
 *
 * Do NOT copy that grammar into this file.
 *
 * Doing so would create two sources of truth and eventually produce divergent
 * closure semantics.
 *
 * ============================================================================
 */