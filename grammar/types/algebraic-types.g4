/*
 * Zamani Programming Language
 * Algebraic Type Grammar
 *
 * File:
 *     grammar/types/algebraic-types.g4
 *
 * Purpose:
 *     Defines the grammar fragment for algebraic type expressions.
 *
 * Architectural ownership:
 *     This file owns syntax that expresses algebraic type constructors
 *     themselves: named variants/constructors and their associated type
 *     payloads where that syntax is part of a type expression.
 *
 *     It does NOT own:
 *       - the complete type-expression grammar;
 *       - generic type application;
 *       - tuple syntax;
 *       - option syntax;
 *       - result syntax;
 *       - struct/enum declarations;
 *       - type declarations;
 *       - primitive types;
 *       - references/pointers;
 *       - quantum IR;
 *       - hardware descriptions;
 *       - resource discovery;
 *       - runtime behavior.
 *
 * Integration:
 *     The canonical type-expression composition belongs to types.g4.
 *     This grammar must therefore never redefine `typeExpression`.
 *
 * Canonical lexer:
 *     ZamaniTokens
 *
 * Scalability:
 *     No machine/resource/topology/cardinality limits are encoded here.
 *     Recursion and collection cardinality are bounded only by the parser
 *     implementation/runtime resources, not by language-defined constants.
 *
 * Rust:
 *     This grammar is consumed by the Zamani Rust compiler/frontend.
 *     Rust 1.97 / 1.97.1 compatibility and the repository's `unsafe` ban
 *     are enforced by the Rust implementation and CI, not by this grammar.
 */

parser grammar AlgebraicTypes;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * --------------------------------------------------------------------------
 * Algebraic type expression
 * --------------------------------------------------------------------------
 *
 * Algebraic types are a semantic category rather than a requirement that
 * every algebraic type have a special keyword.
 *
 * A named algebraic type may therefore be introduced by the declaration
 * grammar and subsequently referenced through the normal type-name grammar.
 *
 * This grammar owns the reusable constructor/payload syntax needed by
 * declaration/type-system integration.
 *
 * IMPORTANT:
 *
 * Do not duplicate generic type application here.
 *
 * For example:
 *
 *     Option<T>
 *     Result<T, E>
 *     Either<A, B>
 *     List<T>
 *
 * are parsed by the generic type grammar when they use generic application
 * syntax. Semantic analysis determines whether a named generic type is an
 * algebraic type.
 */


/*
 * An algebraic constructor consists of a constructor name with an optional
 * sequence of associated types.
 *
 * Examples of the semantic forms represented by this rule:
 *
 *     None
 *     Some(T)
 *     Ok(T)
 *     Err(E)
 *     Pair(A, B)
 *
 * The constructor identifier and its associated types are intentionally
 * represented independently so the semantic layer can construct the
 * canonical AST without introducing another type representation.
 */
algebraicConstructor
    : algebraicConstructorName
      algebraicConstructorPayload?
    ;


/*
 * Constructor names are ordinary language identifiers.
 *
 * The actual identifier/path rules remain owned by the core/name grammar.
 *
 * This rule intentionally uses `identifier` rather than introducing a
 * second identifier token or naming convention.
 */
algebraicConstructorName
    : identifier
    ;


/*
 * A constructor may carry zero or more associated type values.
 *
 * The payload uses `typeExpression`, which is supplied by the delegating
 * type grammar (`types.g4`) and therefore permits arbitrary nesting.
 *
 * This is intentionally not limited to:
 *
 *     Primitive
 *     Tuple
 *     Generic
 *     Classical
 *     Quantum
 *     HDL
 *     Hardware
 *
 * Any type that the canonical Zamani type system accepts can therefore
 * participate in an algebraic constructor.
 */
algebraicConstructorPayload
    : LPAREN algebraicConstructorTypeList? RPAREN
    ;


/*
 * Associated constructor types.
 *
 * There is no fixed maximum number of fields.
 *
 * A one-element payload is valid:
 *
 *     Some(T)
 *
 * Multiple elements are valid:
 *
 *     Pair(A, B)
 *
 * Nested and arbitrarily complex types are valid:
 *
 *     Node(List<T>, Option<U>)
 *
 *     QuantumState<Result<A, E>>
 *
 *     DeviceHandle<Resource>
 *
 *     Pipeline<Tensor<T>, Result<U, E>>
 *
 * Trailing commas are accepted consistently with the generic/tuple grammar
 * policy where the surrounding language permits them.
 */
algebraicConstructorTypeList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/*
 * --------------------------------------------------------------------------
 * Algebraic sum/product helpers
 * --------------------------------------------------------------------------
 *
 * These rules provide reusable structure for semantic/declaration grammars.
 *
 * A sum represents alternatives:
 *
 *     A | B | C
 *
 * A product represents associated fields/types:
 *
 *     (A, B, C)
 *
 * IMPORTANT:
 *
 * These rules are syntax fragments only. They do not decide whether a
 * particular declaration is an enum, tagged union, variant, union,
 * coproduct, or another algebraic construct. That decision belongs to
 * the declaration/type semantic layer.
 */


/*
 * One or more algebraic alternatives.
 *
 * The separator token is intentionally represented by the canonical lexer
 * vocabulary. The grammar does not assign physical meaning to alternatives.
 */
algebraicSum
    : algebraicAlternative
      (PIPE algebraicAlternative)*
    ;


/*
 * An individual alternative may be a constructor or an existing type
 * expression, depending on the declaration grammar that consumes it.
 *
 * Keeping this rule small prevents this grammar from becoming a second
 * declaration grammar.
 */
algebraicAlternative
    : algebraicConstructor
    | typeExpression
    ;


/*
 * Product types are represented by the canonical tuple/type-expression
 * machinery rather than inventing a second tuple representation.
 *
 * This helper is intentionally named separately so declaration grammars can
 * distinguish "product payload" from an ordinary parenthesized expression
 * without changing the canonical AST representation.
 */
algebraicProduct
    : LPAREN algebraicProductElementList? RPAREN
    ;


/*
 * Product elements.
 *
 * No fixed arity is imposed.
 */
algebraicProductElementList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;