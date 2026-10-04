/*
 * Zamani — Result type grammar
 *
 * Feature contract
 * ----------------
 * Purpose:
 *   Define the two type arguments required by the Result<T, E> type
 *   constructor without creating a separate generic-type system.
 *
 * Owns:
 *   - resultTypeArguments
 *
 * Does not own:
 *   - generic type constructors or general type arguments
 *   - the spelling/resolution of the Result type constructor
 *   - Ok/Err value constructors or pattern-matching syntax
 *   - Result runtime representation or error-handling semantics
 *
 * Public rule:
 *   resultTypeArguments
 *
 * Dependencies:
 *   - ZamaniLexer token vocabulary
 *   - typeExpression, supplied by the composing type grammar
 *
 * AST contract:
 *   The enclosing type parser lowers Result<T, E> to the existing
 *   generic-type representation, with arguments [T, E].
 *   Do not add a Result-specific AST node unless the existing AST
 *   explicitly requires a distinct semantic type.
 *
 * Semantic contract:
 *   The semantic/type-checking layer resolves the constructor as Result
 *   and validates that it receives exactly two type arguments:
 *     T = success value type
 *     E = error value type
 *
 *   This grammar does not decide whether either type is inhabited,
 *   copyable, sendable, serializable, or valid for a particular target.
 *
 * Effects/capabilities/resources:
 *   This type declaration introduces no effect, capability, or resource
 *   requirement by itself. Operations consuming or producing Result
 *   values declare their own effects and requirements.
 *
 * Portability/scalability:
 *   No fixed capacity, target, register width, memory size, or collection
 *   limit is encoded here. Type complexity is bounded only by compiler
 *   implementation resources and configured operational safeguards.
 *
 * Rust:
 *   This file is an ANTLR parser grammar, not Rust source. The Rust
 *   frontend/conformance implementation must use safe Rust compatible
 *   with Rust 1.97/1.97.1; no unsafe code is required by this grammar.
 */

parser grammar Result;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * The caller owns the Result constructor and its name resolution.
 *
 * This rule owns only the Result-specific arity and argument structure.
 * The composing grammar should use it when the resolved type constructor
 * is Result, rather than treating every generic type as a Result.
 *
 * The semantic layer must enforce that exactly two arguments are present.
 */
resultTypeArguments
    : LESS typeExpression COMMA typeExpression GREATER
    ;