/*
 * Zamani Programming Language
 * Copyright ...
 *
 * File:
 *   grammar/expressions/compile-time.g4
 *
 * Purpose:
 *   Defines the syntax of compile-time expression forms.
 *
 * Architectural position:
 *
 *   Source
 *      |
 *      v
 *   Zamani Lexer
 *      |
 *      v
 *   Zamani Parser
 *      |
 *      v
 *   Syntax / AST
 *      |
 *      v
 *   Semantic Analysis
 *      |
 *      +--> compile-time evaluation / specialization
 *      |
 *      v
 *   Canonical IR
 *
 * This grammar defines syntax only.
 *
 * It MUST NOT:
 *   - evaluate compile-time expressions;
 *   - execute arbitrary host-language code;
 *   - perform filesystem or network access;
 *   - select a physical device;
 *   - discover hardware;
 *   - define resource limits;
 *   - define quantum IR;
 *   - define classical IR;
 *   - define hardware IR;
 *   - perform optimization;
 *   - perform scheduling;
 *   - perform routing;
 *   - perform QEC;
 *   - define ZQN semantics;
 *   - define runtime semantics;
 *   - define macro expansion;
 *   - define a second type system;
 *   - impose fixed machine/resource cardinalities.
 *
 * Compile-time semantics are determined downstream by the semantic,
 * constant-evaluation, compilation, and metaprogramming subsystems.
 *
 * POCO-REAF:
 *   Compile-time expressions describe portable computation and compile-time
 *   intent. They must not encode accidental properties of the machine on
 *   which compilation happens.
 */

parser grammar CompileTimeExpressions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * --------------------------------------------------------------------------
 * Integration contract
 * --------------------------------------------------------------------------
 *
 * This grammar is a parser fragment.
 *
 * The authoritative expression hierarchy remains:
 *
 *     expressions/expressions.g4
 *
 * `CompileTimeExpressions` supplies:
 *
 *     compileTimeExpression
 *
 * to the canonical expression grammar.
 *
 * The host expression grammar owns:
 *
 *     expression
 *     primary expressions
 *     precedence
 *     calls
 *     indexing
 *     member access
 *     literals
 *     identifiers
 *     operators
 *
 * This file MUST NOT redefine those rules.
 *
 * The host grammar also owns the canonical:
 *
 *     identifier
 *     typeExpression
 *     expression
 *     pattern
 *
 * rules where those rules already exist.
 *
 * If a repository-wide shared rule is required, it must be factored into
 * the appropriate lower-level grammar rather than duplicated here.
 */


/*
 * --------------------------------------------------------------------------
 * Compile-time expression entry point
 * --------------------------------------------------------------------------
 *
 * Compile-time expressions are explicitly introduced by the canonical
 * compile-time marker/token.
 *
 * The marker is deliberately syntax-only. It does not imply:
 *
 *     - a particular compiler;
 *     - a particular target;
 *     - a particular machine;
 *     - a particular execution strategy;
 *     - host-language execution;
 *     - constant folding;
 *     - compile-time evaluation success.
 *
 * Semantic analysis determines whether the expression is actually
 * evaluable at compile time.
 */
compileTimeExpression
    : COMPTIME compileTimeOperand
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time operand
 * --------------------------------------------------------------------------
 *
 * A compile-time operand is an ordinary Zamani expression that is explicitly
 * requested in a compile-time context.
 *
 * The ordinary expression grammar remains authoritative for the expression
 * itself.
 *
 * This rule intentionally delegates to `expression` rather than reproducing
 * arithmetic, calls, indexing, conditionals, lambdas, comprehensions, etc.
 */
compileTimeOperand
    : expression
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time value declaration expression
 * --------------------------------------------------------------------------
 *
 * This form allows a compile-time expression to introduce a named value
 * where the surrounding declaration grammar supports expression-level
 * compile-time declarations.
 *
 * The identifier and expression semantics remain owned by the core
 * identifier and expression/type systems.
 *
 * The grammar does not establish:
 *
 *     - storage;
 *     - lifetime;
 *     - target placement;
 *     - machine representation;
 *     - evaluation order beyond ordinary expression syntax.
 */
compileTimeBindingExpression
    : COMPTIME LET identifier ASSIGN expression
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time conditional expression
 * --------------------------------------------------------------------------
 *
 * Conditional compilation is deliberately separated from ordinary
 * value-producing conditionals.
 *
 * This construct chooses a source-level branch according to a compile-time
 * condition.
 *
 * The branches are expressions, not statements.
 *
 * Runtime execution of the discarded branch is never implied.
 *
 * Semantic validation must still determine whether both branches are
 * syntactically and semantically valid according to the language's
 * conditional-compilation rules.
 */
compileTimeConditionalExpression
    : COMPTIME IF expression
      compileTimeExpressionBranch
      ELSE
      compileTimeExpressionBranch
    ;


/*
 * Compile-time conditional branch.
 *
 * A branch delegates to the canonical expression grammar.
 */
compileTimeExpressionBranch
    : expression
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time assertion
 * --------------------------------------------------------------------------
 *
 * A compile-time assertion requires a condition to be established during
 * compilation.
 *
 * This is an expression-level construct only.
 *
 * Diagnostics, failure classification, source spans, and error reporting
 * belong to semantic analysis/compiler diagnostics, not the grammar.
 */
compileTimeAssertionExpression
    : COMPTIME ASSERT LPAREN expression RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time type query
 * --------------------------------------------------------------------------
 *
 * Allows compile-time logic to query a type-level property without encoding
 * the implementation of the type system in the grammar.
 *
 * `typeExpression` is owned by the canonical type grammar.
 *
 * The semantic layer determines which properties are queryable.
 */
compileTimeTypeQueryExpression
    : COMPTIME TYPEOF LPAREN typeExpression RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time value query
 * --------------------------------------------------------------------------
 *
 * Allows compile-time logic to query the type/shape/properties of an
 * expression.
 *
 * The grammar does not define what metadata exists.
 *
 * This is important for POCO-REAF because properties such as:
 *
 *     dimensions
 *     capabilities
 *     resource requirements
 *     quantum properties
 *     hardware capabilities
 *     layout
 *     target properties
 *
 * must be represented by semantic models rather than hard-coded grammar
 * constructs.
 */
compileTimeValueQueryExpression
    : COMPTIME TYPEOF LPAREN expression RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time capability query
 * --------------------------------------------------------------------------
 *
 * Syntax for asking whether a compile-time-visible semantic capability is
 * available.
 *
 * The capability model is NOT owned by this grammar.
 *
 * Capability resolution belongs to semantic analysis / compilation context.
 *
 * This construct must never directly name a physical device or impose a
 * fixed hardware topology.
 */
compileTimeCapabilityQueryExpression
    : COMPTIME HAS LPAREN expression RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time specialization expression
 * --------------------------------------------------------------------------
 *
 * Explicit specialization requests are represented syntactically without
 * specifying the implementation strategy.
 *
 * Specialization may later be performed through:
 *
 *     constant evaluation
 *     generic specialization
 *     partial evaluation
 *     lowering
 *     optimization
 *     target-independent transformation
 *
 * The grammar does not choose among those mechanisms.
 */
compileTimeSpecializationExpression
    : COMPTIME SPECIALIZE LPAREN expression RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time evaluation expression
 * --------------------------------------------------------------------------
 *
 * Explicitly requests semantic compile-time evaluation.
 *
 * The compiler decides whether evaluation is:
 *
 *     - legal;
 *     - deterministic;
 *     - terminating;
 *     - resource-safe;
 *     - reproducible;
 *     - supported by the selected compilation context.
 *
 * The grammar does not establish evaluation limits.
 */
compileTimeEvaluationExpression
    : COMPTIME EVAL LPAREN expression RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time sequence expression
 * --------------------------------------------------------------------------
 *
 * Provides an expression-level construct for compile-time transformation
 * over an arbitrary source expression.
 *
 * No fixed cardinality is encoded.
 *
 * Resource availability, laziness, streaming, parallelism, and evaluation
 * strategy are semantic/compiler concerns.
 */
compileTimeSequenceExpression
    : COMPTIME FOR identifier IN expression
      compileTimeSequenceBody
    ;


/*
 * Compile-time sequence body.
 *
 * The body remains an ordinary Zamani expression.
 */
compileTimeSequenceBody
    : expression
    ;


/*
 * --------------------------------------------------------------------------
 * Compile-time block expression
 * --------------------------------------------------------------------------
 *
 * A compile-time block permits a sequence of compile-time expressions where
 * the surrounding language permits expression blocks.
 *
 * Statement/block ownership remains in the canonical statements grammar.
 *
 * This rule exists only as an integration point and MUST NOT be used to
 * duplicate statement syntax.
 */
compileTimeBlockExpression
    : COMPTIME LBRACE compileTimeBlockBody RBRACE
    ;


/*
 * The block body delegates to the canonical expression grammar.
 *
 * The surrounding parser/AST layer determines whether multiple expressions
 * are permitted and how their value is formed.
 *
 * No arbitrary maximum number of expressions is imposed.
 */
compileTimeBlockBody
    : expression
    | compileTimeBlockBody SEMICOLON expression
    ;


/*
 * --------------------------------------------------------------------------
 * Unified compile-time expression forms
 * --------------------------------------------------------------------------
 *
 * `compileTimeExpressionForm` is the internal dispatch point used by the
 * canonical expression grammar.
 *
 * It intentionally contains only constructs owned by this file.
 */
compileTimeExpressionForm
    : compileTimeConditionalExpression
    | compileTimeAssertionExpression
    | compileTimeTypeQueryExpression
    | compileTimeValueQueryExpression
    | compileTimeCapabilityQueryExpression
    | compileTimeSpecializationExpression
    | compileTimeEvaluationExpression
    | compileTimeSequenceExpression
    | compileTimeBlockExpression
    | compileTimeBindingExpression
    ;