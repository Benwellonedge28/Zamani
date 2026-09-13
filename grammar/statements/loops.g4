parser grammar Loops;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions, Blocks;

/*
 * ============================================================================
 * Zamani — Loop Statements
 * ============================================================================
 *
 * File:
 *     grammar/statements/loops.g4
 *
 * Purpose:
 *     Defines the statement-level syntax for portable iteration and looping.
 *
 * Architectural boundary:
 *
 *     Source
 *       ↓
 *     Lexer
 *       ↓
 *     Loop statement grammar  ← THIS FILE
 *       ↓
 *     Parse tree
 *       ↓
 *     AST construction
 *       ↓
 *     Semantic analysis
 *       ↓
 *     Canonical IR
 *       ↓
 *     Optimization / lowering
 *       ↓
 *     Scheduling / routing / resource mapping
 *       ↓
 *     Runtime / hardware / distributed execution
 *
 * This grammar describes WHAT iteration means syntactically.
 * It does not decide HOW iteration is implemented on a target.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *   - statement-level loop dispatch;
 *   - loop statement syntax;
 *   - while loops;
 *   - do-while loops;
 *   - for loops;
 *   - loop conditions;
 *   - loop iteration clauses;
 *   - loop-local statement bodies;
 *   - syntactic loop labels, where supported by the canonical name grammar.
 *
 * This file does NOT own:
 *
 *   - identifiers;
 *   - expressions;
 *   - expression precedence;
 *   - assignment expressions;
 *   - types;
 *   - variable declarations;
 *   - function declarations;
 *   - blocks;
 *   - break statements;
 *   - continue statements;
 *   - pattern matching;
 *   - iterator implementation;
 *   - collection semantics;
 *   - ownership or borrowing;
 *   - concurrency;
 *   - parallel execution;
 *   - scheduling;
 *   - resource allocation;
 *   - hardware topology;
 *   - quantum execution;
 *   - quantum resource mapping;
 *   - HDL timing;
 *   - runtime behavior;
 *   - canonical IR definitions;
 *   - optimization;
 *   - target selection.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * No machine-specific quantity is encoded here.
 *
 * In particular, this grammar MUST NOT impose:
 *
 *   - maximum iteration counts;
 *   - maximum nesting depth as a language rule;
 *   - maximum collection size;
 *   - maximum number of loop variables;
 *   - fixed CPU/core/thread counts;
 *   - fixed GPU/accelerator counts;
 *   - fixed quantum-resource counts;
 *   - fixed hardware topology;
 *   - fixed memory capacity;
 *   - fixed execution width.
 *
 * Resource and implementation limits belong to semantic validation,
 * compilation, scheduling, resource management, or runtime layers.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser accepts syntactically valid loop constructs.
 *
 * Semantic analysis subsequently determines:
 *
 *   - whether a condition is boolean-compatible;
 *   - whether an iterator is iterable;
 *   - whether loop bindings are valid;
 *   - whether variables are mutable;
 *   - ownership/borrowing legality;
 *   - whether a loop may execute concurrently;
 *   - whether a loop may be vectorized;
 *   - whether a loop may be lowered to accelerator execution;
 *   - whether a loop interacts with quantum/classical state;
 *   - whether a loop is valid in an HDL process;
 *   - whether timing constraints apply;
 *   - whether resource requirements can be satisfied.
 *
 * None of those semantic decisions belong in this grammar.
 *
 * ============================================================================
 */

/*
 * ---------------------------------------------------------------------------
 * Public entry point
 * ---------------------------------------------------------------------------
 *
 * A complete loop statement consists of one loop form.
 *
 * Statement terminators remain owned by the enclosing statement grammar when
 * the repository's statement dispatcher owns termination.
 *
 * This deliberately keeps loop syntax reusable inside blocks and other
 * statement contexts.
 */
loopStatement
    : whileStatement
    | doWhileStatement
    | forStatement
    ;

/*
 * ---------------------------------------------------------------------------
 * while
 * ---------------------------------------------------------------------------
 *
 * General form:
 *
 *     while <condition> <body>
 *
 * The condition is the canonical expression from expressions.g4.
 *
 * The body is the canonical block/statement body from blocks.g4.
 */
whileStatement
    : WHILE expression statementBody
    ;

/*
 * ---------------------------------------------------------------------------
 * do / while
 * ---------------------------------------------------------------------------
 *
 * General form:
 *
 *     do <body> while <condition>
 *
 * The exact statement terminator remains the responsibility of the enclosing
 * statement grammar.
 */
doWhileStatement
    : DO statementBody WHILE expression
    ;

/*
 * ---------------------------------------------------------------------------
 * for
 * ---------------------------------------------------------------------------
 *
 * The for construct intentionally supports two syntactic families:
 *
 *     for <initializer>; <condition>; <update> <body>
 *
 * and
 *
 *     for <binding> in <expression> <body>
 *
 * The first form provides C-style/general three-clause iteration.
 *
 * The second form provides semantic iteration over an iterable expression.
 *
 * Semantic analysis determines the meaning of the initializer, binding,
 * condition, update, and iterable expression.
 */
forStatement
    : FOR forControl statementBody
    ;

/*
 * ---------------------------------------------------------------------------
 * for control dispatcher
 * ---------------------------------------------------------------------------
 */
forControl
    : cStyleForControl
    | iteratorForControl
    ;

/*
 * ---------------------------------------------------------------------------
 * C-style/general for loop
 * ---------------------------------------------------------------------------
 *
 * The initializer and update are deliberately expressed using the existing
 * expression/declaration machinery rather than defining another assignment
 * or declaration language here.
 *
 * Empty clauses are legal:
 *
 *     for ; condition ; update ...
 *     for initializer ; ; update ...
 *     for initializer ; condition ; ...
 *     for ; ; ...
 *
 * Semantic analysis decides whether an empty condition means an unbounded
 * loop and whether that is valid in the surrounding language context.
 */
cStyleForControl
    : forInitializer? SEMICOLON expression? SEMICOLON forUpdate?
    ;

/*
 * ---------------------------------------------------------------------------
 * C-style initializer
 * ---------------------------------------------------------------------------
 *
 * The initializer is intentionally delegated to the canonical expression
 * grammar. Declaration statements, where permitted by the language, should
 * be admitted by the statement/declaration integration layer rather than
 * duplicated here.
 */
forInitializer
    : expression
    ;

/*
 * ---------------------------------------------------------------------------
 * C-style update
 * ---------------------------------------------------------------------------
 *
 * Multiple update expressions may be represented by the canonical expression
 * sequencing/comma mechanism if that mechanism exists in expressions.g4.
 *
 * This rule deliberately accepts one canonical expression rather than
 * inventing a second expression-list grammar.
 */
forUpdate
    : expression
    ;

/*
 * ---------------------------------------------------------------------------
 * Iterator/range-based for loop
 * ---------------------------------------------------------------------------
 *
 * General form:
 *
 *     for <binding> in <iterable-expression> <body>
 *
 * The binding syntax is intentionally delegated to the canonical binding
 * abstraction exposed by the expressions/declarations/type system.
 */
iteratorForControl
    : FOR_BINDING inKeyword expression
    ;

/*
 * ---------------------------------------------------------------------------
 * Iterator binding
 * ---------------------------------------------------------------------------
 *
 * This rule is a narrow syntactic integration point.
 *
 * A binding may be represented by the canonical identifier/pattern grammar
 * once that grammar is exposed by the repository's declaration/pattern
 * subsystem.
 *
 * The rule intentionally accepts an identifier as the minimum stable
 * repository-wide binding form.
 *
 * Pattern matching extensions must be integrated through the canonical
 * pattern grammar rather than duplicated here.
 */
FOR_BINDING
    : IDENTIFIER
    ;

/*
 * ---------------------------------------------------------------------------
 * `in`
 * ---------------------------------------------------------------------------
 *
 * `in` is consumed through the canonical lexer keyword when available.
 *
 * Keeping this as a parser rule gives the grammar a stable integration point
 * without duplicating lexer definitions.
 */
inKeyword
    : IN
    ;

/*
 * ---------------------------------------------------------------------------
 * Loop body
 * ---------------------------------------------------------------------------
 *
 * Loop bodies use the canonical statement-body abstraction.
 *
 * This allows:
 *
 *     while condition statement
 *
 * and:
 *
 *     while condition { ... }
 *
 * according to the rules established by statements/blocks.g4.
 *
 * No block syntax is duplicated here.
 */
statementBody
    : block
    | statement
    ;