/*
 * ============================================================================
 * Zamani Programming Language
 *
 * File:       grammar/macros/parameters.g4
 * Grammar:    MacroParameters
 * Status:     Canonical macro-parameter parser component
 *
 * Rust implementation baseline:
 *   Rust 1.97 / Rust 1.97.1
 *   Rust 2021
 *   Safe Rust only; no unsafe Rust
 *
 * Purpose:
 *   Define the source syntax of macro parameter lists, individual parameters,
 *   and parameter defaults.
 *
 * This file defines syntax only. It does not bind arguments, evaluate defaults,
 * resolve types, expand macros, or impose compiler resource limits.
 * ============================================================================
 *
 * OWNERSHIP
 *
 * This file owns exactly:
 *
 *   macroParameterList
 *   macroParameter
 *   macroParameterDefault
 *
 * It does not own:
 *
 *   identifier
 *   genericParameters
 *   typeExpression
 *   expression
 *   visibilityModifier
 *   blockExpression
 *   macroDeclaration
 *   macroInvocation
 *
 * Those rules belong to their canonical grammar components and are supplied
 * by the complete Zamani parser composition.
 *
 * INTEGRATION
 *
 * grammar/macros/declarations.g4 must:
 *
 *   1. Import this grammar by its grammar identity: MacroParameters.
 *   2. Retain its use of macroParameterList.
 *   3. Remove its local definitions of the three rules owned here.
 *   4. Continue to use the canonical visibilityModifier, genericParameters,
 *      typeExpression, expression, and blockExpression rules.
 *
 * grammar/macros/macros.g4 remains the macro-subsystem composition boundary.
 * The canonical parser composition must make this component available through
 * the repository's existing ANTLR build/import mechanism.
 *
 * AST CONTRACT
 *
 * The parser/AST lowering layer maps parameters in source order into the
 * canonical macro-declaration representation. Each parameter must preserve:
 *
 *   - its identifier and source span;
 *   - optional type syntax and source span;
 *   - optional default expression and source span;
 *   - its position in the declaration.
 *
 * This grammar does not prescribe Rust AST structs or allocate NodeIds.
 *
 * SEMANTIC CONTRACT
 *
 * Semantic analysis, not this grammar, determines:
 *
 *   - duplicate parameter names;
 *   - whether a type is valid in a macro parameter position;
 *   - whether a default is legal;
 *   - whether defaults may refer to earlier parameters;
 *   - argument-to-parameter correspondence;
 *   - generic constraints and type compatibility.
 *
 * A default is parsed as an ordinary canonical Zamani expression. It is not
 * evaluated by the lexer or parser.
 *
 * SCALABILITY / POCO-REAF
 *
 * Repetition is used for parameter lists. No grammar-defined maximum is
 * imposed on the number of parameters or the size of a default expression.
 * The grammar contains no hardware, quantum, memory, thread, device, or
 * topology limits.
 *
 * Implementations may enforce configurable resource budgets outside language
 * semantics. Such budgets must be diagnosed by the compiler and must not be
 * encoded as fixed grammar limits.
 *
 * DETERMINISM / SAFETY
 *
 * This grammar contains no target-language actions, predicates, embedded Rust,
 * I/O, macro execution, or mutable global state. Parsing is structural and
 * side-effect free. Rust safety requirements apply to the compiler
 * implementation, not to ANTLR grammar syntax.
 * ============================================================================
 */

parser grammar MacroParameters;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * A parameter list is either absent at the declaration call site or contains
 * one or more comma-separated parameters.
 *
 * Trailing commas are intentionally not accepted here unless the canonical
 * language-wide parameter-list policy explicitly adopts them. If adopted,
 * change this rule once and apply the same policy consistently to functions,
 * closures, and other parameter-list constructs.
 */
macroParameterList
    : macroParameter
      (COMMA macroParameter)*
    ;

/*
 * Macro parameter syntax:
 *
 *   name
 *   name: Type
 *   name = defaultExpression
 *   name: Type = defaultExpression
 *
 * Identifier, typeExpression, and expression are shared canonical rules.
 * They are referenced here and must not be redefined by this component.
 */
macroParameter
    : identifier
      (COLON typeExpression)?
      macroParameterDefault?
    ;

/*
 * Defaults are ordinary Zamani expressions. This rule only identifies the
 * assignment delimiter and expression boundary; it does not evaluate,
 * substitute, or validate the expression.
 */
macroParameterDefault
    : ASSIGN expression
    ;