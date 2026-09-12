/**
 * Zamani Programming Language
 * ===========================
 *
 * Tuple Type Grammar
 *
 * File:
 *     grammar/types/tuple-types.g4
 *
 * Responsibility:
 *     Defines the syntax of tuple types only.
 *
 * Architecture:
 *     This is an ANTLR parser-grammar delegate imported by the
 *     authoritative `grammar/types/types.g4` composition.
 *
 * Important:
 *     This grammar does NOT define `typeExpression`.
 *
 *     `typeExpression` is supplied by the enclosing type grammar.
 *     This allows tuple types to recursively contain every type
 *     recognized by Zamani without duplicating the type system here.
 *
 * Examples:
 *
 *     ()
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     (Array<T>, Option<U>)
 *     (fn(T) -> U, Result<V, E>)
 *     ((T, U), V)
 *
 * Tuple arity is intentionally unbounded by the grammar.
 *
 * The parser imposes no machine-size, memory-size, register-count,
 * qubit-count, device-count, or hardware-topology limitation.
 *
 * Semantic validation of tuple arity, layout, ABI representation,
 * resource requirements, and target-specific lowering belongs outside
 * the grammar.
 *
 * Rust compatibility:
 *     The generated parser is consumed by the repository's ANTLR/Rust
 *     frontend using the repository-supported Rust version
 *     (Rust 1.97 / 1.97.1).
 *
 * Safety:
 *     This grammar introduces no Rust implementation code and therefore
 *     requires no `unsafe` implementation.
 */

parser grammar TupleTypes;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ----------------------------------------------------------------------
 * Tuple types
 * ----------------------------------------------------------------------
 *
 * Tuple syntax is deliberately distinguished from a parenthesized type.
 *
 * Parenthesized type:
 *
 *     (T)
 *
 * is NOT a tuple and therefore remains the responsibility of the
 * enclosing/general type-expression grammar.
 *
 * Tuple forms:
 *
 *     ()
 *     (T,)
 *     (T, U)
 *     (T, U, V)
 *     ...
 *
 * The comma is therefore semantically significant.
 *
 * This distinction prevents the tuple grammar from stealing ordinary
 * parenthesized type expressions from the canonical type-expression
 * grammar.
 */

tupleType
    : LPAREN RPAREN
    | LPAREN typeExpression COMMA RPAREN
    | LPAREN typeExpression COMMA typeExpression tupleTypeAdditionalElement* COMMA? RPAREN
    ;

/*
 * Additional tuple elements.
 *
 * Keeping the repetition here rather than imposing an explicit arity
 * keeps the grammar independent of machine or implementation limits.
 *
 * There is intentionally no rule such as:
 *
 *     tuple3
 *     tuple4
 *     tuple8
 *     tuple16
 *
 * and no maximum tuple arity.
 */

tupleTypeAdditionalElement
    : COMMA typeExpression
    ;