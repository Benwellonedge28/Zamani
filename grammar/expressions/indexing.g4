/*
 * Zamani Programming Language
 * File: grammar/expressions/indexing.g4
 *
 * Purpose:
 *   Defines the syntax of indexing/postfix selection operations.
 *
 * Architectural role:
 *   This grammar owns the syntactic form of indexing and the delimiters
 *   that compose an index operation. It does not own the semantics of
 *   arrays, tensors, matrices, maps, slices, quantum registers, hardware
 *   memories, or runtime resources.
 *
 * Design goals:
 *   - Backend independent.
 *   - No fixed dimensionality.
 *   - No fixed collection size.
 *   - No fixed qubit/register count.
 *   - No fixed tensor rank.
 *   - No machine-specific assumptions.
 *   - No embedded Rust actions.
 *   - No semantic predicates.
 *   - No runtime/resource discovery.
 *   - Deterministic parser structure.
 *   - Suitable for arbitrary-length index chains subject only to available
 *     parser/runtime resources.
 *
 * Integration:
 *   The aggregate expression grammar is responsible for placing indexing
 *   into the postfix-expression precedence level.
 *
 *   A lower-level `indexOperandExpression` contract is consumed here.
 *   The aggregate expression grammar must provide that contract without
 *   importing this grammar back into itself.
 *
 *   The semantic analyzer determines whether an indexed value is:
 *     - an array,
 *     - vector,
 *     - matrix,
 *     - tensor,
 *     - map,
 *     - slice,
 *     - string,
 *     - classical memory,
 *     - quantum register,
 *     - quantum resource,
 *     - hardware memory/resource,
 *     - user-defined indexable value,
 *     - or another future indexable abstraction.
 *
 *   This grammar therefore deliberately does not encode those meanings.
 *
 * Important:
 *   Do not introduce fixed index counts such as:
 *
 *       exactly one index
 *       exactly two indices
 *       exactly three dimensions
 *
 *   Repeated index components are intentionally represented by `*`.
 */

parser grammar indexing;

options {
    /*
     * The concrete lexer grammar is the single owner of token definitions.
     *
     * The aggregate lexer is expected to expose these stable tokens:
     *
     *   LBRACK       [
     *   RBRACK       ]
     *   COMMA        ,
     *   COLON        :
     *
     * and the expression/operator tokens required by the imported
     * expression layers.
     */
    tokenVocab = ZamaniLexer;
}


/*
 * --------------------------------------------------------------------------
 * Public indexing entry point
 * --------------------------------------------------------------------------
 *
 * `indexingExpression` represents one or more postfix indexing operations.
 *
 * Examples:
 *
 *   value[index]
 *   value[i, j]
 *   value[i][j]
 *   tensor[i, j, k]
 *   matrix[row][column]
 *   memory[address]
 *   register[index]
 *
 * The grammar does not impose a maximum number of chained operations.
 */
indexingExpression
    : indexBaseExpression indexingOperation+
    ;


/*
 * --------------------------------------------------------------------------
 * Single indexing operation
 * --------------------------------------------------------------------------
 *
 * One pair of brackets constitutes one indexing operation.
 *
 * The contents may contain:
 *   - a single index expression,
 *   - multiple index expressions,
 *   - a range/slice expression,
 *   - a mixture of scalar and range components,
 *   - a trailing/omitted component where the language's range syntax
 *     permits it.
 *
 * Semantic validation belongs downstream.
 */
indexingOperation
    : LBRACK indexArgumentList? RBRACK
    ;


/*
 * --------------------------------------------------------------------------
 * Index argument list
 * --------------------------------------------------------------------------
 *
 * There is intentionally no fixed arity.
 *
 * This permits:
 *
 *   value[i]
 *   value[i, j]
 *   value[i, j, k]
 *   value[i, j, k, ...]
 *
 * without making tensor rank or collection dimensionality a grammar limit.
 */
indexArgumentList
    : indexArgument (COMMA indexArgument)*
    ;


/*
 * --------------------------------------------------------------------------
 * Index argument
 * --------------------------------------------------------------------------
 *
 * An index argument is either:
 *
 *   - an ordinary index expression, or
 *   - a range/index-slice expression.
 *
 * The actual interpretation is semantic:
 *
 *   integer       -> positional index
 *   key           -> associative lookup
 *   range         -> slice
 *   symbolic      -> symbolic indexing
 *   runtime value -> dynamic indexing
 *
 * The grammar must not decide which interpretation applies.
 */
indexArgument
    : indexRangeExpression
    | indexOperandExpression
    ;


/*
 * --------------------------------------------------------------------------
 * Range/slice indexing
 * --------------------------------------------------------------------------
 *
 * The three components are independently optional:
 *
 *   [:]
 *   [start:]
 *   [:end]
 *   [start:end]
 *
 * A step may also be present:
 *
 *   [::step]
 *   [start::step]
 *   [:end:step]
 *   [start:end:step]
 *
 * The exact semantic validity of omitted components is checked later.
 *
 * This rule intentionally does not impose numeric limits on start/end/step.
 */
indexRangeExpression
    : indexRangeStart? COLON indexRangeEnd? (COLON indexRangeStep?)?
    ;


/*
 * --------------------------------------------------------------------------
 * Range components
 * --------------------------------------------------------------------------
 *
 * These aliases establish explicit semantic boundaries without duplicating
 * the general expression grammar.
 *
 * They are deliberately kept separate so future semantic analysis can
 * distinguish:
 *
 *   start
 *   end
 *   step
 *
 * without making the parser responsible for their types.
 */
indexRangeStart
    : indexOperandExpression
    ;

indexRangeEnd
    : indexOperandExpression
    ;

indexRangeStep
    : indexOperandExpression
    ;


/*
 * --------------------------------------------------------------------------
 * Index base expression
 * --------------------------------------------------------------------------
 *
 * This is the expression being indexed.
 *
 * It is intentionally delegated to a lower expression-level contract rather
 * than importing the complete expression grammar. This prevents an ANTLR
 * dependency cycle such as:
 *
 *   expressions.g4
 *       -> indexing.g4
 *       -> expressions.g4
 *
 * The aggregate expression grammar supplies this rule through the parser
 * composition layer.
 */
indexBaseExpression
    : indexablePrimaryExpression
    ;


/*
 * --------------------------------------------------------------------------
 * Index operand expression
 * --------------------------------------------------------------------------
 *
 * Index expressions must be able to refer to ordinary expression values,
 * including symbolic/runtime-computed indices.
 *
 * The concrete expression precedence hierarchy is supplied by the aggregate
 * expression grammar.
 *
 * This rule is deliberately a named integration boundary rather than a
 * duplicate copy of arithmetic/logical/conditional/etc. expressions.
 */
indexOperandExpression
    : indexablePrimaryExpression
    ;


/*
 * --------------------------------------------------------------------------
 * Lower-level expression integration boundary
 * --------------------------------------------------------------------------
 *
 * `indexablePrimaryExpression` is the deliberately narrow dependency point
 * for the expression composition layer.
 *
 * The aggregate expression grammar must bind this contract to the
 * appropriate lower-level expression rule.
 *
 * It must be capable of representing at least:
 *
 *   identifiers
 *   literals
 *   parenthesized expressions
 *   qualified names
 *   callable/indexable values
 *   future expression atoms
 *
 * It must NOT encode:
 *
 *   machine size
 *   collection size
 *   tensor rank
 *   qubit count
 *   hardware count
 *   memory capacity
 *   device topology
 *
 * The semantic layer determines whether the resulting value is actually
 * indexable.
 */
indexablePrimaryExpression
    : primaryIndexExpression
    ;


/*
 * --------------------------------------------------------------------------
 * Primary expression integration boundary
 * --------------------------------------------------------------------------
 *
 * This rule is intentionally a forwarding boundary for the aggregate
 * expression grammar.
 *
 * The final expression composition must bind `primaryIndexExpression` to
 * the canonical primary-expression rule owned by the expression foundation.
 *
 * Keeping this boundary explicit documents the dependency without embedding
 * a second primary-expression grammar here.
 */
primaryIndexExpression
    : expressionAtom
    ;


/*
 * --------------------------------------------------------------------------
 * Expression atom integration boundary
 * --------------------------------------------------------------------------
 *
 * `expressionAtom` is supplied by the foundational expression grammar.
 *
 * It represents an atomic expression before postfix operations such as:
 *
 *   calls
 *   indexing
 *   member access
 *
 * are composed.
 *
 * The aggregate expression grammar is responsible for wiring this boundary
 * to the canonical atom/primary expression implementation.
 */
expressionAtom
    : identifierExpression
    | literalExpression
    | parenthesizedExpression
    ;