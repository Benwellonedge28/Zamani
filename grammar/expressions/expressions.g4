/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/expressions/expressions.g4
 *
 * Status:
 *     CANONICAL PRODUCTION EXPRESSION COMPOSITION ROOT
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AUTHORITATIVE EXPRESSION COMPOSITION ROOT.
 *
 * It owns:
 *
 *     - expression;
 *     - the complete precedence hierarchy;
 *     - expression-layer composition;
 *     - assignment precedence;
 *     - conditional precedence integration;
 *     - range precedence integration;
 *     - logical precedence;
 *     - bitwise precedence;
 *     - equality precedence;
 *     - relational precedence;
 *     - shift precedence;
 *     - additive precedence;
 *     - multiplicative precedence;
 *     - prefix/unary boundary;
 *     - postfix boundary;
 *     - primary-expression composition;
 *     - null-coalescing precedence;
 *     - expression-list composition;
 *     - type-expression-list composition where required by expression syntax.
 *
 * Specialized expression grammars own the syntax they implement.
 *
 * This file composes those components without creating competing expression
 * hierarchies.
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
 *     ZamaniParser
 *       |
 *       v
 *     Expressions
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 *   classical          quantum::ir        HDL/hardware
 *       |                   |                   |
 *       +-------------------+-------------------+
 *                           |
 *                           v
 *                      optimization
 *                           |
 *                           v
 *                 lowering / specialization
 *                           |
 *                  routing / scheduling
 *                           |
 *                resilience / QEC / ZQN
 *                           |
 *                           v
 *                          HAL
 *                           |
 *                           v
 *                    target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar creates NO IR.
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     expression
 *     assignment-expression precedence
 *     conditional-expression precedence integration
 *     range-expression precedence integration
 *     logical precedence
 *     bitwise precedence
 *     equality precedence
 *     relational precedence
 *     shift precedence
 *     additive precedence
 *     multiplicative precedence
 *     prefix boundary
 *     postfix boundary
 *     primary composition
 *     null-coalescing precedence
 *     expression lists
 *     type-expression lists
 *
 * DOES NOT OWN
 * -------------
 *
 *     lexer rules
 *     token spellings
 *     identifiers
 *     names
 *     types
 *     blocks
 *     declarations
 *     statements
 *     conditional implementation
 *     assignment component implementation
 *     unary implementation
 *     postfix implementation
 *     calls
 *     indexing
 *     member access
 *     literals
 *     match implementation
 *     quantum operation implementation
 *     reasoning semantics
 *     learning semantics
 *     knowledge semantics
 *     adaptation semantics
 *     uncertainty semantics
 *     policy semantics
 *     provenance semantics
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     AST implementation
 *     semantic analysis
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     hardware selection
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 *     Canonical parser composition:
 *         grammar/antlr/ZamaniParser.g4
 *
 *     Expression components:
 *         grammar/expressions/assignment.g4
 *         grammar/expressions/conditionals.g4
 *         grammar/expressions/ranges.g4
 *         grammar/expressions/unary.g4
 *         grammar/expressions/postfix.g4
 *         grammar/expressions/literals.g4
 *         grammar/expressions/match.g4
 *         grammar/expressions/quantum.g4
 *
 *     Core/type/block/name ownership is supplied by the parent parser
 *     composition hierarchy.
 *
 * EXPORTS
 * -------
 *
 *     expression
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/antlr/ZamaniParser.g4
 *     statement grammars
 *     declaration grammars
 *     function grammars
 *     module grammars
 *     classical grammars
 *     quantum grammars
 *     hybrid grammars
 *     HDL grammars
 *     data grammars
 *     AI grammars
 *     distributed grammars
 *     networking grammars
 *     metaprogramming grammars
 *     compile-time grammars
 *
 * AST_OWNER
 * ---------
 *
 *     src/frontend/ast/
 *
 * This grammar produces parse structure only.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     semantic/type/effect/capability/resource analysis
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic model
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representation
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/parser/
 *     grammar/tests/expressions/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/
 *     grammar/spec/
 *     grammar/expressions/precedence.md
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one public expression entry point:
 *
 *     expression
 *
 * This file owns it.
 *
 * There MUST be exactly one precedence hierarchy.
 *
 * Specialized files MUST NOT create another public `expression` hierarchy.
 *
 * In particular:
 *
 *     assignment.g4
 *     conditionals.g4
 *     ranges.g4
 *     unary.g4
 *     postfix.g4
 *     calls.g4
 *     indexing.g4
 *     member-access.g4
 *     literals.g4
 *     match.g4
 *     quantum.g4
 *
 * must provide reusable components rather than competing complete expression
 * roots.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * All lexical tokens are owned by:
 *
 *     grammar/lexer/
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This parser grammar defines NO lexer rules.
 *
 * The canonical operator names currently exposed by the lexical architecture
 * include:
 *
 *     ELLIPSIS
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     THIN_ARROW
 *     FAT_ARROW
 *     DOUBLE_COLON
 *
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *
 *     LOGICAL_AND
 *     LOGICAL_OR
 *
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     PERCENT_ASSIGN
 *     AMP_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
 *
 *     INCREMENT
 *     DECREMENT
 *
 *     QUESTION_DOT
 *     NULL_COALESCE
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     ASSIGN
 *     AMPERSAND
 *     PIPE
 *     CARET
 *     TILDE
 *     LESS
 *     GREATER
 *     NOT
 *
 * Punctuation is owned separately:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT
 *     SEMICOLON
 *     COLON
 *     AT
 *     HASH
 *
 * IMPORTANT:
 *
 * This file deliberately uses the actual canonical token names rather than
 * historical aliases.
 *
 * ============================================================================
 * PRECEDENCE CONTRACT
 * ============================================================================
 *
 * LOWEST
 *
 *     assignment
 *
 *     conditional
 *
 *     range
 *
 *     logical OR
 *
 *     logical AND
 *
 *     null coalescing
 *
 *     bitwise OR
 *
 *     bitwise XOR
 *
 *     bitwise AND
 *
 *     equality
 *
 *     relational
 *
 *     shift
 *
 *     additive
 *
 *     multiplicative
 *
 *     prefix
 *
 *     postfix
 *
 *     primary
 *
 * HIGHEST
 *
 * The exact semantic precedence of `??` is owned by this composition layer
 * and documented in precedence.md.
 *
 * It is intentionally below bitwise operators and above logical conjunction.
 *
 * If the normative specification changes that relationship, precedence.md
 * and this file must be changed together as one language-versioned change.
 *
 * ============================================================================
 * ASSOCIATIVITY CONTRACT
 * ============================================================================
 *
 * Assignment:
 *
 *     right associative.
 *
 * Binary operators:
 *
 *     left associative unless explicitly specified otherwise by the
 *     corresponding semantic specification.
 *
 * Conditional expressions:
 *
 *     owned by conditionals.g4.
 *
 * Prefix operators:
 *
 *     recursively right associative.
 *
 * Postfix operations:
 *
 *     left-to-right structural chaining.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO universal finite limit on:
 *
 *     expression count
 *     expression depth
 *     operand count
 *     argument count
 *     generic argument count
 *     index count
 *     tuple size
 *     collection size
 *     member-chain depth
 *     call-chain depth
 *     operator-chain depth
 *     unary depth
 *     range domain size
 *     tensor rank
 *     vector width
 *     matrix dimensions
 *     quantum operation count
 *     qubit count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     node count
 *     device count
 *     memory capacity
 *     register width
 *     network size
 *
 * No `MAX_*` machine or expression constants belong here.
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level ceiling.
 *
 * It does NOT mean the implementation is required to have infinite memory,
 * storage, parser stack, compilation time, or execution resources.
 *
 * Exhaustion is an implementation/resource condition, not a language rule.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Expressions describe source-level computation.
 *
 * They MUST NOT encode:
 *
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     FPGA identifiers
 *     ASIC identifiers
 *     physical qubit identifiers
 *     fixed topology
 *     fixed register width
 *     machine size
 *     device count
 *     memory-bank identifiers
 *     network-node count
 *
 * Those belong to:
 *
 *     resources
 *     capabilities
 *     compilation context
 *     target descriptions
 *     routing
 *     scheduling
 *     deployment
 *     HAL
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum expressions remain source-level expressions.
 *
 * This file MUST NOT enumerate a fixed universal gate set.
 *
 * Therefore names such as:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * are NOT special expression alternatives here.
 *
 * A quantum operation may enter the expression system through:
 *
 *     quantum.g4
 *
 * or ordinary callable/member/qualified expression syntax.
 *
 * Semantic analysis determines whether the expression denotes:
 *
 *     quantum operation
 *     measurement
 *     state transformation
 *     classical computation
 *     hybrid operation
 *     library operation
 *     vendor operation
 *     future operation
 *
 * Quantum semantics ultimately cross:
 *
 *     quantum::ir
 *
 * There is no second quantum expression IR.
 *
 * ============================================================================
 * GENERAL COMPUTATION CONTRACT
 * ============================================================================
 *
 * The expression layer is intentionally reusable for:
 *
 *     classical computation
 *     numerical computation
 *     symbolic computation
 *     tensors
 *     AI/ML
 *     reasoning
 *     knowledge queries
 *     learning
 *     adaptation
 *     uncertainty
 *     probabilistic computation
 *     evidence
 *     provenance
 *     policy evaluation
 *     distributed values
 *     networking
 *     hardware intent
 *     HDL expressions
 *     quantum computation
 *     hybrid computation
 *     accelerator computation
 *     future domains
 *
 * The core expression grammar does NOT need a new keyword for each application.
 *
 * For example, constructs such as:
 *
 *     infer(...)
 *     deduce(...)
 *     reason(...)
 *     assert(...)
 *     query(...)
 *     learn(...)
 *     adapt(...)
 *     explain(...)
 *
 * can be ordinary calls when their semantic model does not require dedicated
 * syntax.
 *
 * This keeps the language open-ended and prevents an application-specific
 * keyword explosion.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is determined only by:
 *
 *     source token sequence
 *     selected language/grammar version
 *     explicit dialect configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     target availability
 *     memory availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * No expression is evaluated while parsing.
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no process execution
 *     no hardware discovery
 *     no runtime calls
 *     no plugin loading
 *     no unsafe Rust
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parse tree must preserve enough information for the frontend AST to
 * represent:
 *
 *     literals
 *     identifiers
 *     qualified names
 *     unary operations
 *     binary operations
 *     assignments
 *     conditional expressions
 *     ranges
 *     calls
 *     indexing
 *     member access
 *     optional member access
 *     generic invocation
 *     tuples
 *     arrays
 *     maps/records where supported
 *     lambdas/closures
 *     match expressions
 *     quantum expressions
 *     construction expressions
 *
 * Source spans remain available through the parser/frontend infrastructure.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar does NOT decide:
 *
 *     operator types
 *     overload resolution
 *     implicit conversions
 *     mutability
 *     ownership
 *     borrowing
 *     lifetimes
 *     effects
 *     capabilities
 *     resource requirements
 *     contracts
 *     policies
 *     provenance
 *     quantum legality
 *     hardware feasibility
 *     distributed placement
 *     execution strategy
 *
 * Those decisions happen downstream.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE CONTRACT
 * ============================================================================
 *
 * An expression can eventually have semantic metadata describing:
 *
 *     effects
 *     required capabilities
 *     required resources
 *     constraints
 *     preferences
 *     policies
 *     provenance
 *
 * This file does not encode those properties into the expression grammar
 * unless a dedicated source-level construct explicitly requires syntax.
 *
 * Therefore:
 *
 *     infer(...)
 *     learn(...)
 *     adapt(...)
 *     measure(...)
 *     network(...)
 *     ffi(...)
 *
 * may be parsed as ordinary expressions while semantic analysis determines
 * their effects and requirements.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * The specialized parser components are imported here so that this grammar
 * remains the single composition root while specialized files retain their
 * own ownership.
 *
 * IMPORTANT:
 *
 * The component files MUST NOT import this file back.
 *
 * The dependency graph must remain directed:
 *
 *     Expressions
 *         |
 *         +--> Assignment
 *         +--> Conditionals
 *         +--> Ranges
 *         +--> Unary
 *         +--> Postfix
 *         +--> Literals
 *         +--> Match
 *         +--> Quantum
 *
 * The parent parser composition additionally supplies:
 *
 *     Core
 *     Types
 *     Declarations
 *     Statements
 *     Functions
 *     Modules
 *     Effects
 *     Memory
 *     Concurrency
 *     Classical
 *     Hybrid
 *     HDL
 *     Hardware
 *     Distributed
 *     AI
 *     Data
 *     Networking
 *     Security
 *     Resources
 *     Compile
 *     Execution
 *     Interoperability
 *     Dialects
 *     Macros
 *     Metaprogramming
 *
 * ============================================================================
 */

parser grammar Expressions;

options {
    tokenVocab = ZamaniLexer;
}

import
    Assignment,
    Conditionals,
    Ranges,
    Unary,
    Postfix,
    Literals,
    Match,
    Quantum
;


/* ============================================================================
 * 1. PUBLIC EXPRESSION ENTRY
 * ========================================================================== */

/*
 * There is exactly one public expression entry point.
 *
 * This is the rule consumed by the rest of the language.
 */
expression
    : assignmentExpression
    ;


/* ============================================================================
 * 2. ASSIGNMENT PRECEDENCE
 * ========================================================================== */

/*
 * Assignment syntax is owned by Assignment.
 *
 * This rule is intentionally a composition alias rather than a second
 * implementation.
 *
 * Right associativity remains the responsibility of Assignment.
 */
assignmentExpression
    : conditionalExpression
    | assignmentTarget assignmentOperator assignmentExpression
    ;


/*
 * Assignment target.
 *
 * The syntactic target is broad by design.
 *
 * Semantic analysis determines whether it is actually assignable.
 */
assignmentTarget
    : postfixExpression
    ;


/*
 * Canonical assignment operators currently exposed by the lexer.
 *
 * Do not add parser aliases for operators that do not exist in the canonical
 * lexical vocabulary.
 */
assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    | PERCENT_ASSIGN
    | AMP_ASSIGN
    | PIPE_ASSIGN
    | CARET_ASSIGN
    ;


/* ============================================================================
 * 3. CONDITIONAL PRECEDENCE
 * ========================================================================== */

/*
 * The implementation of conditional expressions belongs to Conditionals.
 *
 * This composition layer only places conditional expressions in the
 * precedence hierarchy.
 */
conditionalExpression
    : ifExpression
    | ternaryConditionalExpression
    | rangeExpression
    ;


/* ============================================================================
 * 4. RANGE PRECEDENCE
 * ========================================================================== */

/*
 * Range syntax is owned by Ranges.
 *
 * The canonical range layer is kept above logical expressions.
 */
rangeExpression
    : range
    | logicalOrExpression
    ;


/*
 * `range` is expected to be supplied by the canonical Ranges component.
 *
 * This alias provides the composition boundary without creating another
 * range grammar.
 */


/* ============================================================================
 * 5. LOGICAL OR
 * ========================================================================== */

logicalOrExpression
    : logicalAndExpression
      (
          LOGICAL_OR
          logicalAndExpression
      )*
    ;


/* ============================================================================
 * 6. LOGICAL AND
 * ========================================================================== */

logicalAndExpression
    : nullCoalescingExpression
      (
          LOGICAL_AND
          nullCoalescingExpression
      )*
    ;


/* ============================================================================
 * 7. NULL COALESCING
 * ========================================================================== */

/*
 * Example:
 *
 *     primary ?? fallback
 *
 * Nullability remains semantic.
 *
 * This rule only establishes source-level precedence.
 */
nullCoalescingExpression
    : bitwiseOrExpression
      (
          NULL_COALESCE
          bitwiseOrExpression
      )*
    ;


/* ============================================================================
 * 8. BITWISE OR
 * ========================================================================== */

bitwiseOrExpression
    : bitwiseXorExpression
      (
          PIPE
          bitwiseXorExpression
      )*
    ;


/* ============================================================================
 * 9. BITWISE XOR
 * ========================================================================== */

bitwiseXorExpression
    : bitwiseAndExpression
      (
          CARET
          bitwiseAndExpression
      )*
    ;


/* ============================================================================
 * 10. BITWISE AND
 * ========================================================================== */

bitwiseAndExpression
    : equalityExpression
      (
          AMPERSAND
          equalityExpression
      )*
    ;


/* ============================================================================
 * 11. EQUALITY
 * ========================================================================== */

equalityExpression
    : relationalExpression
      (
          equalityOperator
          relationalExpression
      )*
    ;

equalityOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    ;


/* ============================================================================
 * 12. RELATIONAL
 * ========================================================================== */

relationalExpression
    : shiftExpression
      (
          relationalOperator
          shiftExpression
      )*
    ;

relationalOperator
    : LESS
    | GREATER
    | LESS_EQUAL
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 13. SHIFT
 * ========================================================================== */

shiftExpression
    : additiveExpression
      (
          shiftOperator
          additiveExpression
      )*
    ;

shiftOperator
    : LEFT_SHIFT
    | RIGHT_SHIFT
    ;


/* ============================================================================
 * 14. ADDITIVE
 * ========================================================================== */

additiveExpression
    : multiplicativeExpression
      (
          additiveOperator
          multiplicativeExpression
      )*
    ;

additiveOperator
    : PLUS
    | MINUS
    ;


/* ============================================================================
 * 15. MULTIPLICATIVE
 * ========================================================================== */

multiplicativeExpression
    : prefixExpression
      (
          multiplicativeOperator
          prefixExpression
      )*
    ;

multiplicativeOperator
    : STAR
    | SLASH
    | MODULO
    ;


/* ============================================================================
 * 16. PREFIX / UNARY BOUNDARY
 * ========================================================================== */

/*
 * Prefix syntax is owned by Unary.
 *
 * This composition rule intentionally delegates to the canonical unary
 * component.
 */
prefixExpression
    : unaryExpression
    ;


/* ============================================================================
 * 17. POSTFIX BOUNDARY
 * ========================================================================== */

/*
 * Postfix syntax is owned by Postfix.
 */
postfixExpression
    : canonicalPostfixExpression
    ;


/*
 * Integration alias.
 *
 * The Postfix component must export `postfixExpression` under its canonical
 * component name. If the repository keeps that exact public name, this alias
 * must be replaced by a direct component import during parser assembly rather
 * than introducing a duplicate rule.
 *
 * The preferred final architecture is:
 *
 *     postfixExpression
 *         -> Postfix.postfixExpression
 *
 * with no second postfix implementation.
 */


/* ============================================================================
 * 18. PRIMARY EXPRESSION
 * ========================================================================== */

/*
 * Primary expressions are source-level atoms.
 *
 * Domain-specific semantics are deliberately not enumerated here.
 */
primaryExpression
    : literalExpression
    | identifier
    | parenthesizedExpression
    | tupleExpression
    | arrayExpression
    | mapExpression
    | lambdaExpression
    | matchExpression
    | quantumExpression
    | newExpression
    | thisExpression
    | superExpression
    ;


/* ============================================================================
 * 19. IDENTIFIER
 * ========================================================================== */

/*
 * Identifier spelling is owned by the canonical lexical subsystem.
 *
 * No second identifier lexer rule is defined here.
 */
identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 20. PARENTHESIZED EXPRESSION
 * ========================================================================== */

parenthesizedExpression
    : LPAREN expression RPAREN
    ;


/* ============================================================================
 * 21. TUPLE EXPRESSION
 * ========================================================================== */

/*
 * A parenthesized expression with no comma is grouping.
 *
 * A comma establishes tuple syntax.
 *
 * Examples:
 *
 *     (a, b)
 *     (a, b, c)
 *     (a, b,)
 *
 * A tuple has at least two expressions.
 */
tupleExpression
    : LPAREN expression COMMA tupleElementTail? COMMA? RPAREN
    ;

tupleElementTail
    : expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 22. ARRAY EXPRESSION
 * ========================================================================== */

/*
 * Array/sequence literals are represented independently from call arguments.
 *
 * This prevents collection syntax from becoming coupled to callable syntax.
 */
arrayExpression
    : LBRACKET arrayElementList? RBRACKET
    ;

arrayElementList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 23. MAP / ASSOCIATIVE COLLECTION EXPRESSION
 * ========================================================================== */

/*
 * Map/associative collection syntax.
 *
 * Example:
 *
 *     { key: value, other: value }
 *
 * Empty maps are accepted.
 */
mapExpression
    : LBRACE mapEntryList? RBRACE
    ;

mapEntryList
    : mapEntry
      (
          COMMA
          mapEntry
      )*
      COMMA?
    ;

mapEntry
    : expression COLON expression
    ;


/* ============================================================================
 * 24. LAMBDA / CLOSURE EXPRESSION
 * ========================================================================== */

/*
 * Lambda syntax remains an expression.
 *
 * Parameter cardinality is unbounded.
 *
 * The semantic/type layer determines:
 *
 *     parameter types
 *     captures
 *     ownership
 *     effects
 *     return type
 *     capability requirements
 *     resource requirements
 */
lambdaExpression
    : PIPE lambdaParameterList? PIPE lambdaBody
    ;

lambdaParameterList
    : lambdaParameter
      (
          COMMA
          lambdaParameter
      )*
      COMMA?
    ;

lambdaParameter
    : identifier
    ;

lambdaBody
    : expression
    | blockExpression
    ;


/* ============================================================================
 * 25. CONSTRUCTION EXPRESSION
 * ========================================================================== */

newExpression
    : NEW typeExpression
      (
          LPAREN argumentList? RPAREN
      )?
    ;


/* ============================================================================
 * 26. THIS / SUPER
 * ========================================================================== */

thisExpression
    : THIS
    ;

superExpression
    : SUPER
    ;


/* ============================================================================
 * 27. LITERAL INTEGRATION
 * ========================================================================== */

/*
 * Literal implementation is owned by Literals.
 *
 * The composition root merely consumes its canonical literalExpression rule.
 */


/* ============================================================================
 * 28. MATCH INTEGRATION
 * ========================================================================== */

/*
 * Match syntax is owned by Match.
 *
 * This allows pattern matching and guards to participate in expression
 * position without moving pattern semantics into the general expression
 * precedence hierarchy.
 */


/* ============================================================================
 * 29. QUANTUM INTEGRATION
 * ========================================================================== */

/*
 * Quantum expression syntax is owned by Quantum.
 *
 * The general expression hierarchy therefore remains independent of:
 *
 *     gate sets
 *     physical qubits
 *     topology
 *     routing
 *     calibration
 *     QEC
 *     QPU vendors
 *
 * The semantic path remains:
 *
 *     expression
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     quantum semantic analysis
 *         |
 *         v
 *     quantum::ir
 */


/* ============================================================================
 * 30. EXPRESSION LIST
 * ========================================================================== */

/*
 * General comma-separated expression list.
 *
 * No artificial cardinality limit exists.
 */
expressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 31. ARGUMENT LIST
 * ========================================================================== */

/*
 * General positional argument list.
 *
 * The specialized call grammar may extend argument syntax with:
 *
 *     named arguments
 *     spread arguments
 *     explicit generic arguments
 *
 * Those semantics remain owned by calls/postfix.
 */
argumentList
    : argument
      (
          COMMA
          argument
      )*
      COMMA?
    ;

argument
    : expression
    ;


/* ============================================================================
 * 32. TYPE EXPRESSION LIST
 * ========================================================================== */

typeExpressionList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/* ============================================================================
 * 33. DOMAIN-NEUTRAL EXTENSIBILITY
 * ========================================================================== */

/*
 * The expression system is deliberately open-world.
 *
 * The following semantic capabilities do NOT require dedicated grammar
 * keywords merely because they exist:
 *
 *     infer(...)
 *     deduce(...)
 *     reason(...)
 *     assert(...)
 *     retract(...)
 *     query(...)
 *     learn(...)
 *     adapt(...)
 *     explain(...)
 *     simulate(...)
 *     measure(...)
 *     observe(...)
 *     optimize(...)
 *     plan(...)
 *     decide(...)
 *     validate(...)
 *
 * Their names can remain ordinary identifiers/callables unless a dedicated
 * syntax specification establishes that a distinct grammatical form is
 * genuinely necessary.
 *
 * This permits future libraries, models, reasoning systems, data systems,
 * hardware systems, quantum operations and computational domains to be added
 * without modifying the universal expression grammar.
 *
 * ============================================================================
 * 34. UNCERTAINTY / PROBABILITY
 * ============================================================================
 *
 * Probability, distributions, confidence, belief and uncertainty are semantic
 * types/operations rather than fixed expression operators.
 *
 * Examples can therefore remain ordinary expressions:
 *
 *     probability(...)
 *     distribution(...)
 *     confidence(...)
 *     belief(...)
 *
 * Their semantics belong to the type/AI/data semantic layers.
 *
 * No probability implementation is hard-coded here.
 *
 * ============================================================================
 * 35. KNOWLEDGE / REASONING
 * ============================================================================
 *
 * Knowledge assertions, retractions, queries and reasoning operations may be
 * represented as ordinary expressions unless their specification explicitly
 * requires dedicated statement syntax.
 *
 * This avoids making the expression grammar dependent on one reasoning
 * implementation.
 *
 * ============================================================================
 * 36. LEARNING / ADAPTATION
 * ============================================================================
 *
 * Learning and adaptation are semantic operations.
 *
 * Their semantic model may attach:
 *
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     contracts
 *
 * to the resulting AST/semantic node.
 *
 * The expression grammar itself remains target-neutral.
 *
 * In particular:
 *
 *     adapt(...)
 *
 * MUST NOT mean unrestricted self-modifying source code.
 *
 * Authorization and adaptation policy belong downstream.
 *
 * ============================================================================
 * 37. CONTRACTS / POLICIES
 * ============================================================================
 *
 * Contracts and policies are primarily owned by:
 *
 *     grammar/validation/
 *     grammar/policies/
 *     grammar/resources/
 *     grammar/security/
 *
 * Expressions may occur inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     policy conditions
 *     capability requirements
 *     resource constraints
 *
 * Therefore expression syntax must remain sufficiently general to represent
 * arbitrary conditions without introducing domain-specific operators.
 *
 * ============================================================================
 * 38. PROVENANCE / EVIDENCE
 * ============================================================================
 *
 * Evidence, confidence, provenance and explanation are semantic metadata.
 *
 * Expressions can serve as:
 *
 *     claims
 *     evidence predicates
 *     decision conditions
 *     derivation expressions
 *     explanation expressions
 *
 * Their provenance is attached by the frontend/semantic pipeline rather than
 * encoded into the expression grammar.
 *
 * ============================================================================
 * 39. EFFECT / CAPABILITY / RESOURCE INTEGRATION
 * ============================================================================
 *
 * An expression may semantically produce or require:
 *
 *     effect("io")
 *     effect("network")
 *     effect("measurement")
 *     effect("learning")
 *     effect("adaptation")
 *     effect("reflection")
 *     effect("foreign")
 *
 * and capabilities/resources such as:
 *
 *     capability("tensor.compute")
 *     capability("quantum.measurement")
 *     capability("accelerator.compute")
 *
 * These are semantic contracts, not parser-level hardware limits.
 *
 * ============================================================================
 * 40. POCO-REAF INVARIANT
 * ============================================================================
 *
 * This expression grammar is portable across:
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
 * The expression itself describes computation.
 *
 * Target realization is determined later through:
 *
 *     semantic analysis
 *     capability negotiation
 *     resource analysis
 *     compilation
 *     specialization
 *     lowering
 *     routing
 *     scheduling
 *     resilience
 *     HAL
 *
 * ============================================================================
 * 41. NO HARD-CODED MACHINE LIMITS
 * ============================================================================
 *
 * This file MUST NOT contain:
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
 * or equivalent constants.
 *
 * Source-level numeric values remain program data.
 *
 * A source expression such as:
 *
 *     tensor[dimension]
 *
 * does not imply a fixed maximum tensor dimension.
 *
 * A source expression such as:
 *
 *     q[index]
 *
 * does not imply a fixed number of qubits.
 *
 * Target feasibility belongs downstream.
 *
 * ============================================================================
 * 42. ERROR CONTRACT
 * ============================================================================
 *
 * Parser errors must remain structural parser errors.
 *
 * They MUST NOT be silently converted into semantic success.
 *
 * Examples of semantic errors that do NOT belong here:
 *
 *     invalid assignment target
 *     invalid operand type
 *     unavailable capability
 *     insufficient resource
 *     illegal quantum operation
 *     incompatible hardware
 *     invalid effect
 *     forbidden policy transition
 *     invalid adaptation
 *     invalid provenance
 *
 * These belong to downstream diagnostics.
 *
 * ============================================================================
 * 43. DETERMINISTIC PARSING
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     grammar version
 *     language version
 *     dialect configuration
 *
 * must produce the same parse structure.
 *
 * The parser must not inspect:
 *
 *     time
 *     randomness
 *     hardware
 *     filesystem
 *     network
 *     environment
 *     scheduler state
 *     target availability
 *
 * ============================================================================
 * 44. SOURCE PRESERVATION
 * ============================================================================
 *
 * Expression parsing must preserve enough source structure for:
 *
 *     diagnostics
 *     formatting
 *     IDE/LSP
 *     source maps
 *     AST construction
 *     semantic diagnostics
 *     provenance
 *     reproducibility
 *     incremental compilation
 *     macro tooling
 *     compatibility tooling
 *
 * This grammar must not silently normalize or discard semantic source
 * distinctions.
 *
 * ============================================================================
 * 45. AST INTEGRATION
 * ============================================================================
 *
 * Expected semantic/AST categories include:
 *
 *     Literal
 *     Identifier
 *     Unary
 *     Binary
 *     Assignment
 *     Conditional
 *     Range
 *     Call
 *     Index
 *     Member
 *     OptionalMember
 *     QualifiedMember
 *     Tuple
 *     Array
 *     Map/Record
 *     Lambda
 *     Match
 *     Construction
 *     QuantumExpression
 *
 * The AST remains domain-neutral.
 *
 * The grammar MUST NOT introduce:
 *
 *     QuantumGateAst
 *     PhysicalQubitAst
 *     CpuSpecificExpressionAst
 *     GpuSpecificExpressionAst
 *     VendorExpressionAst
 *
 * unless such a node is explicitly part of the domain-neutral AST contract.
 *
 * ============================================================================
 * 46. IR INTEGRATION
 * ============================================================================
 *
 * Expression syntax follows:
 *
 *     source
 *       |
 *       v
 *     parse tree
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic model
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical              quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *               lowering
 *
 * Expressions.g4 does not directly emit IR.
 *
 * ============================================================================
 * 47. COMPATIBILITY
 * ============================================================================
 *
 * Historical aliases must NOT be silently introduced.
 *
 * In particular, this file must use the canonical lexer names:
 *
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *     LOGICAL_AND
 *     LOGICAL_OR
 *     MODULO
 *     AMP_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
 *     INCREMENT
 *     DECREMENT
 *
 * rather than creating aliases such as:
 *
 *     EQ_EQ
 *     NOT_EQ
 *     LE
 *     GE
 *     SHIFT_LEFT
 *     SHIFT_RIGHT
 *     AND_AND
 *     OR_OR
 *     PERCENT
 *     AMPERSAND_ASSIGN
 *     PIPE_ASSIGN
 *     PLUS_PLUS
 *     MINUS_MINUS
 *
 * This prevents lexical/parser drift.
 *
 * ============================================================================
 * 48. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 *     [ ] exactly one public `expression` rule exists;
 *     [ ] assignment precedence is correct;
 *     [ ] conditional precedence is correct;
 *     [ ] range precedence is correct;
 *     [ ] logical precedence is correct;
 *     [ ] null-coalescing precedence is correct;
 *     [ ] bitwise precedence is correct;
 *     [ ] equality precedence is correct;
 *     [ ] relational precedence is correct;
 *     [ ] shift precedence is correct;
 *     [ ] additive precedence is correct;
 *     [ ] multiplicative precedence is correct;
 *     [ ] prefix/postfix precedence is correct;
 *     [ ] canonical lexer tokens are used;
 *     [ ] no duplicate lexer rules exist;
 *     [ ] no second identifier grammar exists here;
 *     [ ] no machine-size limits exist;
 *     [ ] no quantum gate enumeration exists;
 *     [ ] no target-specific syntax exists;
 *     [ ] no IR is created here;
 *     [ ] no runtime action exists;
 *     [ ] no unsafe Rust is required;
 *     [ ] expression lists are unbounded;
 *     [ ] argument lists are unbounded;
 *     [ ] generic lists are unbounded;
 *     [ ] index lists are unbounded;
 *     [ ] tuple/collection forms are unbounded;
 *     [ ] expression nesting has no language-level finite ceiling;
 *     [ ] quantum expressions integrate through the canonical quantum path;
 *     [ ] AI/reasoning/learning/adaptation remain open-world;
 *     [ ] contracts can consume arbitrary expressions;
 *     [ ] policies can consume arbitrary expressions;
 *     [ ] resource/capability conditions can consume arbitrary expressions;
 *     [ ] provenance/evidence conditions can consume arbitrary expressions;
 *     [ ] deterministic parsing is preserved;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] precedence tests exist;
 *     [ ] associativity tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION NOTE
 * ============================================================================
 *
 * The repository currently contains several specialized expression grammars
 * whose rule names and ownership are not yet completely aligned.
 *
 * Before generating the final ANTLR parser, the following invariant MUST be
 * enforced:
 *
 *     Expressions owns composition.
 *
 *     Assignment owns assignment implementation.
 *
 *     Conditionals owns conditional implementation.
 *
 *     Ranges owns range implementation.
 *
 *     Unary owns unary implementation.
 *
 *     Postfix owns postfix implementation.
 *
 *     Calls owns call implementation.
 *
 *     Indexing owns indexing implementation.
 *
 *     Member access owns member implementation.
 *
 *     Literals owns literal implementation.
 *
 *     Match owns match implementation.
 *
 *     Quantum owns quantum expression implementation.
 *
 * There must be no circular import and no duplicate public rule.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */