/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/tuples.g4
 *
 * Status:
 *     Production-ready expression grammar component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar delegate.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of tuple expressions.
 *
 * It is deliberately a parser delegate rather than an independent parser
 * grammar. The authoritative expression composition grammar imports this
 * component and consumes `tupleExpression`.
 *
 * Tuple expressions are ordered, heterogeneous expression aggregates.
 *
 * Examples:
 *
 *     ()
 *     (value,)
 *     (value1, value2)
 *     (value1, value2, value3)
 *     (a, b, c,)
 *     ((a, b), c)
 *     (f(x), g(y))
 *     (q0, q1, q2)
 *     (measurement, classical_result)
 *
 * The grammar imposes NO maximum tuple arity.
 *
 * Actual resource limitations are determined downstream by:
 *
 *     - parser resource policy;
 *     - compiler resources;
 *     - available memory;
 *     - target capabilities;
 *     - ABI constraints;
 *     - runtime resources;
 *     - deployment policy.
 *
 * None of those limitations belong in this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     expression parser
 *       |
 *       +--> tupleExpression       <-- this component
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
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     generic semantic model      domain IR
 *                                     |
 *                         +-----------+-----------+
 *                         |           |           |
 *                         v           v           v
 *                    classical   quantum::ir    HDL
 *                         |           |           |
 *                         +-----------+-----------+
 *                                     |
 *                                     v
 *                         optimization / lowering
 *                                     |
 *                              routing / scheduling
 *                                     |
 *                              runtime / target
 *
 * Tuple syntax MUST NOT depend on:
 *
 *     - AST implementation details;
 *     - semantic type checking;
 *     - ownership analysis;
 *     - borrowing analysis;
 *     - effects;
 *     - capability resolution;
 *     - resource allocation;
 *     - hardware discovery;
 *     - quantum hardware;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime state.
 *
 * ============================================================================
 * SINGLE RESPONSIBILITY
 * ============================================================================
 *
 * OWNS:
 *
 *     - tuple-expression delimiters;
 *     - tuple-expression element sequencing;
 *     - tuple-expression comma structure;
 *     - empty tuple syntax;
 *     - singleton tuple syntax;
 *     - multi-element tuple syntax;
 *     - optional trailing comma syntax;
 *     - arbitrary recursive nesting through `expression`.
 *
 * DOES NOT OWN:
 *
 *     - tuple types;
 *     - tuple patterns;
 *     - tuple destructuring declarations;
 *     - tuple assignment semantics;
 *     - tuple layout;
 *     - tuple ABI representation;
 *     - tuple memory representation;
 *     - tuple optimization;
 *     - tuple lowering;
 *     - tuple serialization;
 *     - tuple runtime implementation.
 *
 * Tuple types are owned by:
 *
 *     grammar/types/tuple-types.g4
 *
 * Tuple patterns are owned by the pattern grammar.
 *
 * Tuple semantic representation is owned by the frontend/semantic layers.
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The canonical lexer authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This parser delegate therefore consumes the canonical lexer vocabulary.
 *
 * Required tokens:
 *
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * The element grammar is:
 *
 *     expression
 *
 * which is supplied by the canonical expression composition grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successfully parsed tuple expression must map to the existing
 * domain-neutral frontend AST tuple-expression representation.
 *
 * The repository already defines:
 *
 *     NodeKind::TupleExpression
 *
 * and an expression builder for tuple expressions.
 *
 * This grammar therefore MUST NOT introduce a new tuple AST concept.
 *
 * The AST must preserve:
 *
 *     - source span;
 *     - tuple element order;
 *     - every element expression;
 *     - empty/singleton/multi-element distinction;
 *     - trailing-comma syntax where source fidelity requires it.
 *
 * Tuple element count is represented structurally by the AST collection.
 *
 * There is no grammar-level tuple arity constant.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only that the source has valid tuple-expression
 * structure.
 *
 * Semantic analysis determines:
 *
 *     - the type of every element;
 *     - tuple type construction;
 *     - whether the tuple is usable in its context;
 *     - ownership and borrowing;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - domain-specific legality;
 *     - ABI/layout decisions;
 *     - optimization opportunities.
 *
 * In particular:
 *
 *     (q0, q1, q2)
 *
 * is syntactically only a tuple expression.
 *
 * Whether q0/q1/q2 are:
 *
 *     - classical values;
 *     - qubits;
 *     - logical qubits;
 *     - handles;
 *     - resources;
 *     - distributed values;
 *     - hardware references;
 *
 * is determined downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Tuple expressions are part of the language's universal expression model.
 *
 * The grammar therefore imposes no universal maximum for:
 *
 *     tuple arity
 *     nesting depth
 *     element expression size
 *     expression complexity
 *
 * No rules such as the following are permitted:
 *
 *     tuple2
 *     tuple3
 *     tuple4
 *     tuple8
 *     tuple16
 *
 * Likewise, this file MUST NOT contain:
 *
 *     MAX_TUPLE_ARITY
 *     MAX_ELEMENTS
 *     MAX_NESTING
 *     MAX_FIELDS
 *     MAX_VALUES
 *
 * or equivalent artificial limits.
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * "Infinity" here means that the language grammar does not establish an
 * artificial finite resource ceiling. Actual execution remains bounded by
 * available resources and implementation policy.
 *
 * ============================================================================
 * TUPLE / PARENTHESIZED-EXPRESSION DISTINCTION
 * ============================================================================
 *
 * Parenthesized expression:
 *
 *     (expression)
 *
 * is NOT a tuple expression.
 *
 * Tuple expression:
 *
 *     (expression,)
 *
 * or:
 *
 *     (expression, expression)
 *
 * The comma is therefore syntactically significant.
 *
 * This distinction is essential because the expression grammar must support
 * both:
 *
 *     (x)
 *
 * and:
 *
 *     (x,)
 *
 * with different AST meaning.
 *
 * Empty tuple:
 *
 *     ()
 *
 * is also distinct from an empty parenthesized expression, which is not
 * syntactically valid as a normal parenthesized expression.
 *
 * ============================================================================
 * SINGLETON TUPLES
 * ============================================================================
 *
 * A singleton tuple requires its comma:
 *
 *     (value,)
 *
 * whereas:
 *
 *     (value)
 *
 * is a parenthesized expression.
 *
 * This rule is intentional and prevents ambiguity between tuple construction
 * and grouping.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * Multi-element tuples may contain a trailing comma:
 *
 *     (a, b,)
 *
 * The trailing comma is syntactic sugar and does not create an additional
 * tuple element.
 *
 * The grammar therefore accepts:
 *
 *     (a, b)
 *
 * and:
 *
 *     (a, b,)
 *
 * as the same structural tuple arity.
 *
 * Source fidelity may preserve the presence of the trailing comma in the
 * syntax tree/source-span representation if the frontend requires it.
 *
 * ============================================================================
 * RECURSION AND NESTING
 * ============================================================================
 *
 * Tuple elements consume the canonical `expression` rule.
 *
 * Consequently nested tuples are naturally supported:
 *
 *     ((a, b), c)
 *     (a, (b, c))
 *     ((a,), ((b, c), d))
 *
 * No explicit nesting-depth limit is encoded.
 *
 * Parser-stack/resource limits remain implementation concerns.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Tuple expressions are intentionally domain-neutral.
 *
 * They can contain:
 *
 *     classical expressions;
 *     quantum expressions;
 *     hybrid expressions;
 *     HDL expressions;
 *     hardware/resource expressions;
 *     distributed expressions;
 *     AI/data expressions;
 *     networking expressions;
 *     cryptographic expressions;
 *     compile-time expressions;
 *     future dialect expressions.
 *
 * This file must not create domain-specific tuple alternatives such as:
 *
 *     quantumTuple
 *     hardwareTuple
 *     gpuTuple
 *     tensorTuple
 *
 * unless a future language specification establishes a genuinely different
 * syntax-level construct.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum expressions may occur as tuple elements.
 *
 * For example, syntactically:
 *
 *     (q0, q1)
 *     (measurement, correction)
 *     (logical_q0, logical_q1, logical_q2)
 *
 * are ordinary tuple expressions.
 *
 * This grammar does NOT decide:
 *
 *     - physical qubit mapping;
 *     - logical-to-physical mapping;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - noise;
 *     - ZQN;
 *     - HAL selection;
 *     - QPU selection.
 *
 * Any quantum meaning is resolved after parsing and eventually integrates
 * with the canonical `quantum::ir` boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical values can be tuple elements without any special syntax:
 *
 *     (x, y)
 *     (vector, matrix)
 *     (numerator, denominator)
 *     (input, output)
 *
 * The grammar does not impose representation or machine-width constraints.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Tuple expressions may occur wherever the enclosing HDL/hardware grammar
 * permits a general expression.
 *
 * This file does not define:
 *
 *     - signal layout;
 *     - bus width;
 *     - register width;
 *     - physical pin allocation;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - device topology.
 *
 * Such concerns remain downstream.
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL INTEGRATION
 * ============================================================================
 *
 * Tuple expressions may contain values associated with multiple distributed
 * participants or parallel computations.
 *
 * The tuple grammar does not impose:
 *
 *     - node-count limits;
 *     - participant-count limits;
 *     - worker-count limits;
 *     - thread-count limits;
 *     - accelerator-count limits.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples of syntactic errors:
 *
 *     (
 *     (a
 *     (a,)
 *     (a,,b)
 *     (a b)
 *     (,a)
 *     (a,)
 *
 * The last example is NOT an error: `(a,)` is a valid singleton tuple.
 *
 * Semantic errors such as:
 *
 *     incompatible tuple assignment;
 *     invalid tuple element type;
 *     unsupported tuple resource;
 *     invalid quantum tuple use;
 *     invalid ABI layout;
 *
 * MUST NOT be encoded here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing tuple expressions must depend only on:
 *
 *     - token sequence;
 *     - grammar version.
 *
 * It must not depend on:
 *
 *     - time;
 *     - randomness;
 *     - environment variables;
 *     - hardware;
 *     - device state;
 *     - network state;
 *     - runtime scheduling.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     ()
 *     (a,)
 *     (a, b)
 *     (a, b,)
 *     (a, b, c)
 *     ((a, b), c)
 *     (a, (b, c))
 *     (f(), g(x))
 *     (array[i], matrix[i, j])
 *     (q0, q1)
 *     (measure(q), result)
 *
 * Negative syntax cases:
 *
 *     (
 *     (a
 *     (a b)
 *     (,a)
 *     (a,,b)
 *     (a b,)
 *     (a, b,,)
 *
 * Boundary cases:
 *
 *     ()
 *     (a,)
 *     (a,b)
 *     deeply nested tuples
 *     very large tuple element counts
 *
 * Scalability cases:
 *
 *     generated tuples with increasing arity;
 *     generated nested tuples;
 *     tuple elements containing arbitrarily large expressions.
 *
 * The tests MUST NOT establish a language-level maximum tuple arity.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is independently complete when:
 *
 *     1. Its tuple syntax is defined.
 *     2. It consumes only canonical lexer tokens.
 *     3. It does not define lexer rules.
 *     4. It does not define tuple types.
 *     5. It does not define tuple patterns.
 *     6. It exposes exactly the intended tuple-expression rule.
 *     7. It imposes no tuple arity limit.
 *     8. Empty tuples are supported.
 *     9. Singleton tuples are supported only with a comma.
 *    10. Multi-element tuples are supported.
 *    11. Trailing commas are supported.
 *    12. Nested tuples are supported.
 *    13. Tuple elements use the canonical `expression` rule.
 *    14. The existing frontend `TupleExpression` AST contract is preserved.
 *    15. Semantic/type/IR responsibilities remain downstream.
 *    16. Positive and negative conformance tests exist.
 *    17. No hard-coded machine/resource limits exist.
 *
 * The enclosing expression composition file then needs only to:
 *
 *     - import this parser delegate;
 *     - expose `tupleExpression` from its `primaryExpression` composition.
 *
 * No modification of this file should be required when unrelated expression
 * components are subsequently added.
 *
 * ============================================================================
 */

parser grammar Tuples;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * Complete tuple expression.
 *
 * Forms:
 *
 *     ()
 *     (element,)
 *     (element1, element2)
 *     (element1, element2, ...)
 *     (element1, element2, ...,)
 *
 * The comma distinguishes tuples from ordinary parenthesized expressions.
 */
tupleExpression
    : LPAREN
      RPAREN
    | LPAREN
      expression
      COMMA
      RPAREN
    | LPAREN
      expression
      COMMA
      expression
      tupleElementTail*
      COMMA?
      RPAREN
    ;


/*
 * ============================================================================
 * TUPLE ELEMENT TAIL
 * ============================================================================
 *
 * Each occurrence contributes exactly one additional tuple element.
 *
 * There is deliberately no finite sequence of tuple arities.
 */
tupleElementTail
    : COMMA
      expression
    ;