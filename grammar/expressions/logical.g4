/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/logical.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the single authoritative grammar component for logical
 * expressions.
 *
 * It owns:
 *
 *     - logical OR;
 *     - logical AND;
 *     - unary logical NOT;
 *     - logical operator precedence;
 *     - logical operator associativity;
 *     - logical-expression source structure;
 *     - the boundary between logical operators and lower-precedence
 *       expression operators.
 *
 * It does NOT own:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - literals;
 *     - arithmetic;
 *     - shifts;
 *     - comparisons;
 *     - bitwise operators;
 *     - assignment;
 *     - conditional expressions;
 *     - ranges;
 *     - postfix expressions;
 *     - function calls;
 *     - indexing;
 *     - member access;
 *     - type checking;
 *     - truthiness;
 *     - boolean conversion;
 *     - overload resolution;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - quantum semantics;
 *     - HDL semantics;
 *     - hardware topology;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - IR construction.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * Canonical expression hierarchy:
 *
 *     expression
 *         |
 *         v
 *     assignmentExpression
 *         |
 *         v
 *     conditionalExpression
 *         |
 *         v
 *     rangeExpression
 *         |
 *         v
 *     logicalExpression
 *         |
 *         v
 *     logicalOrExpression
 *         |
 *         v
 *     logicalAndExpression
 *         |
 *         v
 *     logicalNotExpression
 *         |
 *         v
 *     bitwiseOrExpression
 *         |
 *         v
 *     ...
 *
 * The lower-precedence expression hierarchy is owned by the binary-expression
 * component.
 *
 * This file therefore MUST NOT duplicate:
 *
 *     bitwiseOrExpression
 *     bitwiseXorExpression
 *     bitwiseAndExpression
 *     equalityExpression
 *     relationalExpression
 *     shiftExpression
 *     additiveExpression
 *     multiplicativeExpression
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * `logical.g4` is the sole owner of:
 *
 *     logicalExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     logicalNotExpression
 *
 * No other grammar file may define competing versions of those rules.
 *
 * In particular:
 *
 *     grammar/expressions/binary.g4
 *     grammar/antlr/Core.g4
 *     grammar/expressions/expressions.g4
 *     grammar/Zamani.g4
 *
 * must delegate to this grammar instead of redefining logical precedence.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Token spelling belongs exclusively to:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar consumes the canonical logical tokens:
 *
 *     AND_AND
 *     OR_OR
 *     NOT_OPERATOR
 *
 * and the existing word-form logical operators:
 *
 *     AND
 *     OR
 *     NOT
 *
 * The symbolic forms are:
 *
 *     &&
 *     ||
 *     !
 *
 * The word forms are:
 *
 *     and
 *     or
 *     not
 *
 * This grammar does not define or redefine those tokens.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines whether operands are valid logical operands.
 *
 * A logical operand may eventually represent:
 *
 *     - Boolean;
 *     - predicate;
 *     - symbolic proposition;
 *     - compile-time predicate;
 *     - capability predicate;
 *     - resource predicate;
 *     - classical-control predicate;
 *     - quantum/classical control predicate;
 *     - domain-specific predicate;
 *     - future predicate types.
 *
 * This grammar must remain unchanged when such semantic types are added.
 *
 * ============================================================================
 * SHORT-CIRCUIT CONTRACT
 * ============================================================================
 *
 * `&&` / `and` and `||` / `or` describe logical operators.
 *
 * Whether evaluation is short-circuiting is a semantic/execution concern.
 *
 * The parser must preserve:
 *
 *     - operator identity;
 *     - left operand;
 *     - right operand;
 *     - source ordering;
 *     - nesting.
 *
 * ============================================================================
 * PRECEDENCE
 * ============================================================================
 *
 * From lower to higher precedence:
 *
 *     logical OR
 *     logical AND
 *     logical NOT
 *
 * Therefore:
 *
 *     a || b && c
 *
 * is:
 *
 *     a || (b && c)
 *
 * and:
 *
 *     !a && b
 *
 * is:
 *
 *     (!a) && b
 *
 * ============================================================================
 * ASSOCIATIVITY
 * ============================================================================
 *
 * Logical OR and logical AND are left associative.
 *
 *     a || b || c
 *
 * is:
 *
 *     (a || b) || c
 *
 * Likewise:
 *
 *     a && b && c
 *
 * is:
 *
 *     (a && b) && c
 *
 * Logical NOT is unary and recursively nestable:
 *
 *     !!!value
 *
 * is:
 *
 *     !(!(!value))
 *
 * No artificial chain-depth limit is encoded.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar introduces no machine-dependent limits.
 *
 * It MUST NOT define:
 *
 *     MAX_OPERANDS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *
 * Logical-expression chains may grow according to available implementation
 * resources.
 *
 * "Infinity" means that the language grammar introduces no artificial finite
 * semantic limit.
 *
 * Actual parser/compiler resource limits remain implementation-policy concerns.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * Logical expressions may participate in:
 *
 *     classical computation
 *     quantum/classical control
 *     HDL conditions
 *     hardware predicates
 *     resource constraints
 *     capability constraints
 *     distributed conditions
 *     AI/data predicates
 *     networking conditions
 *     security policies
 *     compile-time conditions
 *     runtime conditions
 *
 * This file does not specialize itself for any domain.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Logical expressions may control quantum/classical hybrid computation.
 *
 * For example, after semantic validation, a program may express a condition
 * involving a measurement result.
 *
 * This grammar does NOT define:
 *
 *     - qubits;
 *     - physical qubits;
 *     - quantum states;
 *     - gates;
 *     - measurement semantics;
 *     - QEC;
 *     - noise;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - HAL.
 *
 * Valid quantum constructs continue through the established pipeline:
 *
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing / scheduling / resilience / QEC / ZQN
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * No second quantum IR is introduced here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Each logical operation must lower to the existing domain-neutral frontend
 * expression representation.
 *
 * Conceptually:
 *
 *     LogicalOr
 *         left
 *         right
 *
 *     LogicalAnd
 *         left
 *         right
 *
 *     LogicalNot
 *         operand
 *
 * The AST must preserve:
 *
 *     - operator identity;
 *     - operand ordering;
 *     - source spans;
 *     - nesting;
 *     - source-level syntax.
 *
 * The grammar must not introduce:
 *
 *     QuantumLogicalExpression
 *     HardwareLogicalExpression
 *     BooleanOnlyExpression
 *
 * merely to support a domain.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates no IR.
 *
 * Lowering is performed after semantic analysis.
 *
 * Depending on the program:
 *
 *     logical expression
 *         ->
 *     semantic expression
 *         ->
 *     classical/control IR
 *
 * or:
 *
 *     logical expression
 *         ->
 *     semantic quantum/classical control
 *         ->
 *     canonical quantum::ir
 *
 * The grammar must never bypass the established semantic boundary.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source tokens;
 *     - grammar version;
 *     - grammar composition.
 *
 * It must not depend on:
 *
 *     - hardware discovery;
 *     - runtime state;
 *     - device state;
 *     - network state;
 *     - randomness;
 *     - system time;
 *     - scheduling state.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     a &&
 *     a ||
 *     &&
 *     ||
 *     !
 *     a && || b
 *     a || && b
 *
 * Semantic errors are handled downstream, for example:
 *
 *     1 && 2
 *     object && function()
 *
 * if those operands are not valid logical operands under the semantic/type
 * system.
 *
 * The parser must not perform type checking.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * These forms must remain structurally supported without fixed language limits:
 *
 *     a && b
 *
 *     a && b && c
 *
 *     a || b || c || d
 *
 *     a || b && c || d && e
 *
 *     !(a && b)
 *
 *     !(!(!condition))
 *
 *     condition_0 && condition_1 && ... && condition_n
 *
 * The grammar uses repetition for associative binary chains and recursion only
 * for unary NOT.
 *
 * ============================================================================
 * INTEGRATION
 * ============================================================================
 *
 * This grammar imports the binary-expression component because the operand
 * immediately below logical NOT is the canonical `bitwiseOrExpression`.
 *
 * The resulting hierarchy is:
 *
 *     logicalExpression
 *         -> logicalOrExpression
 *             -> logicalAndExpression
 *                 -> logicalNotExpression
 *                     -> bitwiseOrExpression
 *
 * `BinaryExpressions` owns everything below that boundary.
 *
 * ============================================================================
 */

parser grammar LogicalExpressions;

options {
    tokenVocab = ZamaniLexer;
}

import BinaryExpressions;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Complete logical-expression syntax.
 *
 * This is the public logical-expression rule consumed by the canonical
 * expression composition grammar.
 */
logicalExpression
    : logicalOrExpression
    ;


/* ============================================================================
 * LOGICAL OR
 * ========================================================================== */

/**
 * Logical OR is lower precedence than logical AND.
 *
 * Both symbolic and word-form operators are accepted:
 *
 *     a || b
 *     a or b
 *
 * Repetition gives left-associative structure.
 */
logicalOrExpression
    : logicalAndExpression
      (
          OR_OR
        | OR
      )
      logicalAndExpression
      (
          (
              OR_OR
            | OR
          )
          logicalAndExpression
      )*
    ;


/* ============================================================================
 * LOGICAL AND
 * ========================================================================== */

/**
 * Logical AND binds more tightly than logical OR.
 *
 * Both symbolic and word-form operators are accepted:
 *
 *     a && b
 *     a and b
 *
 * Repetition gives left-associative structure.
 */
logicalAndExpression
    : logicalNotExpression
      (
          (
              AND_AND
            | AND
          )
          logicalNotExpression
      )*
    ;


/* ============================================================================
 * LOGICAL NOT
 * ========================================================================== */

/**
 * Unary logical negation.
 *
 * Both symbolic and word-form operators are accepted:
 *
 *     !a
 *     not a
 *
 * Recursive nesting supports:
 *
 *     !!a
 *     !!!a
 *     not not a
 *     !not a
 *
 * without introducing a finite language-level depth limit.
 */
logicalNotExpression
    : (
          NOT_OPERATOR
        | NOT
      )
      logicalNotExpression
    | bitwiseOrExpression
    ;