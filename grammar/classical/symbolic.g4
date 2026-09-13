/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/symbolic.g4
 *
 * Status:
 *     Production-ready symbolic-computation parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific code.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime evaluation.
 *     - No hardware access.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYMBOLIC COMPUTATION SYNTAX.
 *
 * Symbolic computation means that a program may describe mathematical
 * quantities, expressions, relations, transformations, functions, domains,
 * and symbolic operations without requiring those quantities to be evaluated
 * during parsing.
 *
 * The grammar supports symbolic computation as a language-level abstraction
 * while leaving symbolic meaning and evaluation to later compiler stages.
 *
 * Symbolic syntax may therefore be used for:
 *
 *     - algebra;
 *     - symbolic arithmetic;
 *     - symbolic equations;
 *     - symbolic inequalities;
 *     - symbolic functions;
 *     - symbolic parameters;
 *     - symbolic constants;
 *     - symbolic differentiation;
 *     - symbolic integration;
 *     - symbolic summation;
 *     - symbolic products;
 *     - symbolic limits;
 *     - symbolic substitutions;
 *     - symbolic simplification;
 *     - symbolic expansion;
 *     - symbolic factorization;
 *     - symbolic transformations;
 *     - symbolic constraints;
 *     - symbolic domains;
 *     - symbolic numerical computation;
 *     - symbolic tensor/vector/matrix expressions;
 *     - compile-time symbolic computation;
 *     - runtime symbolic computation;
 *     - scientific computing;
 *     - optimization;
 *     - AI/ML mathematical expressions;
 *     - quantum parameter expressions;
 *     - hardware parameter expressions;
 *     - resource expressions.
 *
 * This file does NOT implement symbolic mathematics.
 *
 * For example, names such as:
 *
 *     sin
 *     cos
 *     tan
 *     exp
 *     log
 *     sqrt
 *     abs
 *     diff
 *     integrate
 *     simplify
 *     factor
 *     expand
 *     substitute
 *
 * are deliberately not hard-coded into the grammar as the only valid
 * symbolic operations.
 *
 * They may be:
 *
 *     library functions;
 *     user functions;
 *     compiler intrinsics;
 *     dialect operations;
 *     symbolic-system operations;
 *     accelerator operations;
 *     future mathematical operations.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     Expressions
 *          |
 *          v
 *     Symbolic
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     name resolution
 *          |
 *          v
 *     type / effect / capability analysis
 *          |
 *          v
 *     symbolic semantic model
 *          |
 *          +--> classical semantic representation
 *          +--> numerical semantic representation
 *          +--> tensor/data representation
 *          +--> constraint representation
 *          +--> resource metadata
 *          +--> compile-time representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     target realization
 *
 * IMPORTANT:
 *
 * This grammar does NOT directly construct:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware state
 *     scheduling state
 *     routing state
 *     QEC state
 *     ZQN state
 *     runtime state
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - symbolic-domain parser composition;
 *     - symbolic expression classification;
 *     - symbolic declaration composition;
 *     - symbolic binding composition;
 *     - symbolic assignment composition;
 *     - symbolic expression statements;
 *     - symbolic equations;
 *     - symbolic relations;
 *     - symbolic function expressions;
 *     - symbolic operation composition;
 *     - symbolic transformation composition;
 *     - symbolic substitution composition;
 *     - symbolic differentiation composition;
 *     - symbolic integration composition;
 *     - symbolic summation composition;
 *     - symbolic product composition;
 *     - symbolic limit composition;
 *     - symbolic range/domain composition;
 *     - symbolic sequence composition;
 *     - symbolic constraint composition;
 *     - symbolic-domain integration points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - token spelling;
 *     - identifier spelling;
 *     - numeric literal spelling;
 *     - string literal spelling;
 *     - character literal spelling;
 *     - operator precedence;
 *     - general expression syntax;
 *     - general assignment syntax;
 *     - general type syntax;
 *     - classical type definitions;
 *     - vector types;
 *     - matrix types;
 *     - tensor types;
 *     - numerical algorithms;
 *     - symbolic evaluation;
 *     - symbolic simplification algorithms;
 *     - theorem proving;
 *     - equation solving algorithms;
 *     - differentiation algorithms;
 *     - integration algorithms;
 *     - numerical precision;
 *     - numerical representation;
 *     - memory allocation;
 *     - parallelization;
 *     - accelerator selection;
 *     - CPU selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - ASIC selection;
 *     - quantum hardware;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - routing;
 *     - optimization implementation;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Symbolic source describes mathematical and computational intent.
 *
 * It MUST NOT encode:
 *
 *     MAX_SYMBOLS
 *     MAX_VARIABLES
 *     MAX_TERMS
 *     MAX_FACTORS
 *     MAX_POLYNOMIAL_DEGREE
 *     MAX_EXPRESSION_SIZE
 *     MAX_EXPRESSION_DEPTH
 *     MAX_FUNCTION_ARGUMENTS
 *     MAX_DERIVATIVE_ORDER
 *     MAX_INTEGRATION_ORDER
 *     MAX_SUMMATION_RANGE
 *     MAX_PRODUCT_RANGE
 *     MAX_MATRIX_SIZE
 *     MAX_TENSOR_RANK
 *     MAX_PRECISION
 *     MAX_DIGITS
 *     MAX_BITS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * No symbolic rule selects:
 *
 *     a processor;
 *     a machine word size;
 *     a register size;
 *     a vector width;
 *     a GPU;
 *     an accelerator;
 *     a quantum processor;
 *     a hardware topology;
 *     a memory capacity;
 *     a deployment topology.
 *
 * Practical limits belong to explicit implementation/resource policies.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Symbolic structures are represented structurally.
 *
 * Examples:
 *
 *     x + y
 *
 *     f(x)
 *
 *     f(x, y, z, ...)
 *
 *     sum(f(i), i, domain)
 *
 *     product(g(i), i, domain)
 *
 *     derivative(f, x)
 *
 *     integral(f, x)
 *
 *     limit(f, x, a)
 *
 *     substitute(expression, x, replacement)
 *
 *     equation(lhs, rhs)
 *
 * Symbolic domains may be represented using arbitrary expressions.
 *
 * The grammar imposes no mathematical bound on:
 *
 *     - number of symbols;
 *     - number of terms;
 *     - number of operations;
 *     - function arity;
 *     - derivative order;
 *     - integral nesting;
 *     - summation nesting;
 *     - product nesting;
 *     - domain magnitude;
 *     - symbolic dimension;
 *     - expression count.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical lexical vocabulary from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file MUST NOT declare lexer rules.
 *
 * Symbolic operation names remain ordinary identifiers unless the canonical
 * language specification explicitly reserves a keyword.
 *
 * Therefore names such as:
 *
 *     sin
 *     cos
 *     diff
 *     integrate
 *     simplify
 *     factor
 *     expand
 *     substitute
 *     solve
 *     determinant
 *
 * are not required to be lexer keywords.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Symbolic grammar must not create a second arithmetic-precedence hierarchy.
 *
 * Therefore:
 *
 *     x + y
 *     x * y
 *     -x
 *     x < y
 *     x == y
 *     f(x)
 *     a[i]
 *     object.member
 *
 * remain ordinary Zamani expressions.
 *
 * This grammar only adds symbolic-domain composition around those expressions.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Symbolic VALUE syntax is different from symbolic TYPE syntax.
 *
 * This file does not define symbolic types.
 *
 * Type syntax remains owned by the canonical type grammar.
 *
 * A symbolic expression may therefore eventually have a type such as:
 *
 *     Integer
 *     Float
 *     Complex
 *     Rational
 *     Vector<T>
 *     Matrix<T, R, C>
 *     Tensor<T, Shape>
 *     Symbolic<T>
 *
 * without requiring this grammar to redefine those types.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser MUST NOT decide:
 *
 *     - whether an expression is mathematically valid;
 *     - whether a symbol is declared;
 *     - whether a function exists;
 *     - whether a function is differentiable;
 *     - whether an integral exists;
 *     - whether an equation has a solution;
 *     - whether a transformation is valid;
 *     - whether an expression can be simplified;
 *     - whether evaluation terminates;
 *     - whether a numerical approximation is stable;
 *     - whether an expression is exact;
 *     - whether a symbolic result is representable;
 *     - whether an operation is compile-time evaluable.
 *
 * Those decisions belong downstream semantic/compiler systems.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Symbolic expressions may participate in:
 *
 *     classical computation
 *     numerical computation
 *     vector computation
 *     matrix computation
 *     tensor computation
 *     quantum parameterization
 *     quantum-classical control
 *     HDL parameters
 *     hardware parameters
 *     resource constraints
 *     compile-time expressions
 *     AI/ML mathematical models
 *     scientific computation
 *     optimization
 *     distributed computation
 *
 * Symbolic syntax therefore remains domain-neutral.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded target actions;
 *     - no runtime evaluation;
 *     - no hardware queries;
 *     - no resource discovery;
 *     - no nondeterministic parser decisions.
 *
 * The same source/token sequence therefore has the same syntactic structure
 * regardless of the target machine.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing symbolic syntax performs no:
 *
 *     - symbolic evaluation;
 *     - arbitrary code execution;
 *     - filesystem access;
 *     - network access;
 *     - device access;
 *     - dynamic library loading;
 *     - external process execution.
 *
 * ============================================================================
 */

parser grammar Symbolic;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/* ============================================================================
 * 1. PUBLIC SYMBOLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration point for consumers that explicitly recognize a symbolic
 * domain construct.
 *
 * ============================================================================
 */

symbolicConstruct
    : symbolicDeclaration
    | symbolicAssignment
    | symbolicExpressionStatement
    | symbolicEquation
    | symbolicRelation
    | symbolicOperation
    | symbolicTransformation
    | symbolicConstraint
    ;


/* ============================================================================
 * 2. SYMBOLIC DECLARATION
 * ============================================================================
 *
 * Declaration syntax is deliberately composed from canonical expression and
 * type syntax rather than redefining either subsystem.
 *
 * Examples:
 *
 *     let x = symbolicExpression;
 *     let x: SymbolicType = symbolicExpression;
 *
 * Semantic analysis determines whether the binding is genuinely symbolic.
 *
 * ============================================================================
 */

symbolicDeclaration
    : symbolicBindingKeyword
      identifier
      symbolicTypeAnnotation?
      ASSIGN
      symbolicExpression
      SEMICOLON
    | symbolicBindingKeyword
      identifier
      symbolicTypeAnnotation
      SEMICOLON
    ;


symbolicBindingKeyword
    : LET
    | VAR
    | CONST
    ;


symbolicTypeAnnotation
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 3. SYMBOLIC ASSIGNMENT
 * ============================================================================
 *
 * Assignment semantics remain owned by the general expression subsystem.
 *
 * ============================================================================
 */

symbolicAssignment
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * 4. SYMBOLIC EXPRESSION STATEMENT
 * ============================================================================
 */

symbolicExpressionStatement
    : symbolicExpression
      SEMICOLON
    ;


/* ============================================================================
 * 5. SYMBOLIC EXPRESSION
 * ============================================================================
 *
 * This is a semantic classification boundary.
 *
 * It intentionally delegates ordinary expression syntax to Expressions.
 *
 * ============================================================================
 */

symbolicExpression
    : expression
    ;


/* ============================================================================
 * 6. SYMBOLIC REFERENCE
 * ============================================================================
 *
 * A symbolic reference is represented by the ordinary identifier syntax.
 *
 * Identifier resolution is a semantic concern.
 *
 * ============================================================================
 */

symbolicReference
    : identifier
    ;


/* ============================================================================
 * 7. IDENTIFIER
 * ============================================================================
 *
 * The canonical lexical token owns identifier spelling.
 *
 * This local parser rule is an integration boundary only.
 *
 * ============================================================================
 */

identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 8. SYMBOLIC FUNCTION
 * ============================================================================
 *
 * A symbolic function is represented by an identifier followed by a general
 * expression argument list.
 *
 * This deliberately avoids a fixed list such as:
 *
 *     sin
 *     cos
 *     exp
 *     log
 *     sqrt
 *
 * The semantic layer resolves the function identity.
 *
 * ============================================================================
 */

symbolicFunction
    : identifier
      LPAREN
      symbolicArgumentList?
      RPAREN
    ;


symbolicArgumentList
    : symbolicExpression
      (
          COMMA
          symbolicExpression
      )*
    ;


/* ============================================================================
 * 9. SYMBOLIC FUNCTION DEFINITION REFERENCE
 * ============================================================================
 *
 * The grammar does not define function declaration syntax here.
 *
 * Function declarations remain owned by the functions subsystem.
 *
 * This rule merely provides a stable symbolic call boundary.
 *
 * ============================================================================
 */

symbolicFunctionReference
    : identifier
    ;


/* ============================================================================
 * 10. SYMBOLIC EQUATION
 * ============================================================================
 *
 * An equation establishes two symbolic expressions separated by an equality
 * operator.
 *
 * Equality-token spelling belongs to ZamaniLexer.
 *
 * Semantic analysis determines whether the relation represents an equation,
 * boolean comparison, constraint, or another language construct.
 *
 * ============================================================================
 */

symbolicEquation
    : symbolicExpression
      EQUAL
      symbolicExpression
      SEMICOLON
    ;


/* ============================================================================
 * 11. SYMBOLIC RELATION
 * ============================================================================
 *
 * Relations remain syntactic constructs.
 *
 * Semantic analysis determines their mathematical meaning.
 *
 * ============================================================================
 */

symbolicRelation
    : symbolicExpression
      symbolicRelationalOperator
      symbolicExpression
      SEMICOLON
    ;


symbolicRelationalOperator
    : LESS
    | GREATER
    | LESS_EQUAL
    | GREATER_EQUAL
    | EQUAL
    | NOT_EQUAL
    ;


/* ============================================================================
 * 12. SYMBOLIC OPERATION
 * ============================================================================
 *
 * Generic symbolic operations are intentionally identifier-based.
 *
 * This allows future operations without changing the grammar.
 *
 * Examples:
 *
 *     diff(f, x);
 *     integrate(f, x);
 *     simplify(expression);
 *     solve(equation, x);
 *     transform(expression, rule);
 *
 * The grammar does not prescribe which operation names exist.
 *
 * ============================================================================
 */

symbolicOperation
    : symbolicFunction
      SEMICOLON
    ;


/* ============================================================================
 * 13. DIFFERENTIATION COMPOSITION
 * ============================================================================
 *
 * Differentiation is represented as symbolic operation composition rather than
 * a hard-coded mathematical evaluator.
 *
 * The first argument is the expression being differentiated.
 *
 * The second argument identifies the differentiation variable or expression.
 *
 * An optional order expression permits arbitrary symbolic derivative order.
 *
 * ============================================================================
 */

symbolicDifferentiation
    : identifier
      LPAREN
      symbolicExpression
      COMMA
      symbolicExpression
      (
          COMMA
          symbolicExpression
      )?
      RPAREN
    ;


/* ============================================================================
 * 14. INTEGRATION COMPOSITION
 * ============================================================================
 *
 * Integration remains an operation-level syntactic construct.
 *
 * Optional bounds are expressions rather than fixed numeric values.
 *
 * ============================================================================
 */

symbolicIntegration
    : identifier
      LPAREN
      symbolicExpression
      COMMA
      symbolicExpression
      (
          COMMA
          symbolicExpression
          COMMA
          symbolicExpression
      )?
      RPAREN
    ;


/* ============================================================================
 * 15. SUMMATION
 * ============================================================================
 *
 * A symbolic summation consists of:
 *
 *     expression
 *     iterator
 *     domain
 *
 * All three are expressions.
 *
 * No finite iteration bound is imposed by the grammar.
 *
 * ============================================================================
 */

symbolicSummation
    : identifier
      LPAREN
      symbolicExpression
      COMMA
      symbolicIterator
      COMMA
      symbolicDomain
      RPAREN
    ;


/* ============================================================================
 * 16. PRODUCT
 * ============================================================================
 */

symbolicProduct
    : identifier
      LPAREN
      symbolicExpression
      COMMA
      symbolicIterator
      COMMA
      symbolicDomain
      RPAREN
    ;


/* ============================================================================
 * 17. LIMIT
 * ============================================================================
 *
 * A limit may contain:
 *
 *     expression
 *     variable
 *     target value
 *
 * All remain symbolic expressions.
 *
 * ============================================================================
 */

symbolicLimit
    : identifier
      LPAREN
      symbolicExpression
      COMMA
      symbolicExpression
      COMMA
      symbolicExpression
      RPAREN
    ;


/* ============================================================================
 * 18. SYMBOLIC ITERATOR
 * ============================================================================
 *
 * The iterator name is an ordinary identifier.
 *
 * Its scope and binding semantics belong to semantic analysis.
 *
 * ============================================================================
 */

symbolicIterator
    : identifier
    ;


/* ============================================================================
 * 19. SYMBOLIC DOMAIN
 * ============================================================================
 *
 * A symbolic domain may be:
 *
 *     - a general expression;
 *     - a range;
 *     - a collection;
 *     - a symbolic set;
 *     - a runtime value;
 *     - a compile-time value;
 *     - a future domain representation.
 *
 * This grammar does not impose a finite domain size.
 *
 * ============================================================================
 */

symbolicDomain
    : symbolicExpression
    ;


/* ============================================================================
 * 20. SYMBOLIC RANGE
 * ============================================================================
 *
 * Range syntax is delegated to the canonical expression subsystem.
 *
 * ============================================================================
 */

symbolicRange
    : expression
    ;


/* ============================================================================
 * 21. SYMBOLIC SUBSTITUTION
 * ============================================================================
 *
 * Generic substitution syntax:
 *
 *     substitute(expression, variable, replacement)
 *
 * The operation name remains an identifier.
 *
 * ============================================================================
 */

symbolicSubstitution
    : identifier
      LPAREN
      symbolicExpression
      COMMA
      symbolicExpression
      COMMA
      symbolicExpression
      RPAREN
    ;


/* ============================================================================
 * 22. SYMBOLIC TRANSFORMATION
 * ============================================================================
 *
 * Transformations operate on symbolic expressions and transformation
 * specifications.
 *
 * ============================================================================
 */

symbolicTransformation
    : identifier
      LPAREN
      symbolicExpression
      (
          COMMA
          symbolicExpression
      )*
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 23. SYMBOLIC CONSTRAINT
 * ============================================================================
 *
 * A symbolic constraint is a relational or general symbolic expression that
 * can later be interpreted by semantic constraint infrastructure.
 *
 * ============================================================================
 */

symbolicConstraint
    : symbolicRelation
    | symbolicExpression
      SEMICOLON
    ;


/* ============================================================================
 * 24. SYMBOLIC SET / COLLECTION COMPOSITION
 * ============================================================================
 *
 * Symbolic collections are deliberately represented through ordinary
 * expression syntax.
 *
 * This avoids inventing a second collection literal grammar here.
 *
 * ============================================================================
 */

symbolicCollection
    : expression
    ;


/* ============================================================================
 * 25. SYMBOLIC CALL
 * ============================================================================
 *
 * General call syntax remains owned by Expressions.
 *
 * This rule exists solely as a stable symbolic-domain integration point.
 *
 * ============================================================================
 */

symbolicCall
    : symbolicFunction
    ;


/* ============================================================================
 * 26. SYMBOLIC VALUE
 * ============================================================================
 *
 * A symbolic value may be an ordinary expression or a symbolic function.
 *
 * The semantic layer determines its symbolic classification.
 *
 * ============================================================================
 */

symbolicValue
    : symbolicExpression
    | symbolicFunction
    ;


/* ============================================================================
 * 27. SYMBOLIC DEFINITION REFERENCE
 * ============================================================================
 *
 * Symbolic definitions are resolved semantically.
 *
 * The grammar does not maintain a symbol table.
 *
 * ============================================================================
 */

symbolicDefinitionReference
    : symbolicReference
    ;


/* ============================================================================
 * 28. SYMBOLIC OPERATION INVOCATION
 * ============================================================================
 *
 * Generic extensibility boundary.
 *
 * This is intentionally identifier-driven so new symbolic operations do not
 * require grammar changes.
 *
 * ============================================================================
 */

symbolicOperationInvocation
    : identifier
      LPAREN
      symbolicArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 29. SYMBOLIC NESTING
 * ============================================================================
 *
 * Symbolic operations may be nested arbitrarily through ordinary expression
 * composition.
 *
 * No grammar-level nesting maximum exists.
 *
 * ============================================================================
 */

symbolicNestedExpression
    : symbolicExpression
    ;


/* ============================================================================
 * 30. SYMBOLIC PROGRAM REGION
 * ============================================================================
 *
 * Provides a stable domain boundary for tools that parse a sequence of
 * symbolic-domain constructs independently from the complete source grammar.
 *
 * ============================================================================
 */

symbolicRegion
    : symbolicConstruct*
    ;


/* ============================================================================
 * 31. SYMBOLIC SEQUENCE
 * ============================================================================
 *
 * A sequence is structurally unbounded.
 *
 * Practical limits belong to parser/compiler resource policies.
 *
 * ============================================================================
 */

symbolicSequence
    : symbolicConstruct*
    ;


/* ============================================================================
 * 32. SYMBOLIC EXPRESSION LIST
 * ============================================================================
 */

symbolicExpressionList
    : symbolicExpression
      (
          COMMA
          symbolicExpression
      )*
    ;


/* ============================================================================
 * 33. SYMBOLIC OPERATION ARGUMENTS
 * ============================================================================
 */

symbolicArguments
    : symbolicExpressionList?
    ;


/* ============================================================================
 * 34. SYMBOLIC MAPPING
 * ============================================================================
 *
 * Mapping syntax is represented by a generic symbolic operation rather than
 * introducing a second map/collection grammar.
 *
 * ============================================================================
 */

symbolicMapping
    : identifier
      LPAREN
      symbolicExpressionList?
      RPAREN
    ;


/* ============================================================================
 * 35. SYMBOLIC NORMALIZATION
 * ============================================================================
 *
 * Normalization is intentionally an operation name resolved downstream.
 *
 * ============================================================================
 */

symbolicNormalization
    : identifier
      LPAREN
      symbolicExpression
      RPAREN
    ;


/* ============================================================================
 * 36. SYMBOLIC SOLVE OPERATION
 * ============================================================================
 *
 * Equation/constraint solving remains semantic.
 *
 * The grammar merely accepts the operation composition.
 *
 * ============================================================================
 */

symbolicSolve
    : identifier
      LPAREN
      symbolicExpressionList
      RPAREN
    ;


/* ============================================================================
 * 37. SYMBOLIC EVALUATION REQUEST
 * ============================================================================
 *
 * Evaluation is not performed by the parser.
 *
 * The syntax merely represents an operation request.
 *
 * ============================================================================
 */

symbolicEvaluation
    : identifier
      LPAREN
      symbolicExpression
      RPAREN
    ;


/* ============================================================================
 * 38. SYMBOLIC EXPANSION / FACTORIZATION
 * ============================================================================
 *
 * Both are generic symbolic transformations.
 *
 * ============================================================================
 */

symbolicExpansion
    : identifier
      LPAREN
      symbolicExpression
      RPAREN
    ;


symbolicFactorization
    : identifier
      LPAREN
      symbolicExpression
      RPAREN
    ;


/* ============================================================================
 * 39. SYMBOLIC DIFFERENCE / VARIATION
 * ============================================================================
 *
 * Generic operation composition permits finite-difference, variation,
 * perturbation, and future mathematical operations without grammar changes.
 *
 * ============================================================================
 */

symbolicVariation
    : identifier
      LPAREN
      symbolicExpressionList
      RPAREN
    ;


/* ============================================================================
 * 40. SYMBOLIC DOMAIN OPERATION
 * ============================================================================
 *
 * Domain operations may describe transformations over arbitrary symbolic
 * domains.
 *
 * ============================================================================
 */

symbolicDomainOperation
    : identifier
      LPAREN
      symbolicExpressionList?
      RPAREN
    ;


/* ============================================================================
 * 41. SYMBOLIC PARAMETER
 * ============================================================================
 *
 * Parameters are ordinary identifiers.
 *
 * Parameter semantics are resolved by declarations, scopes, generics,
 * functions, and semantic analysis.
 *
 * ============================================================================
 */

symbolicParameter
    : identifier
    ;


/* ============================================================================
 * 42. SYMBOLIC CONSTANT
 * ============================================================================
 *
 * Constants remain ordinary references.
 *
 * This grammar does not reserve names such as:
 *
 *     pi
 *     e
 *     tau
 *     infinity
 *
 * A standard library may provide them.
 *
 * ============================================================================
 */

symbolicConstant
    : identifier
    ;


/* ============================================================================
 * 43. SYMBOLIC EXPRESSION GROUP
 * ============================================================================
 *
 * Parentheses and grouping semantics remain owned by Expressions.
 *
 * ============================================================================
 */

symbolicGroup
    : LPAREN
      symbolicExpression
      RPAREN
    ;


/* ============================================================================
 * 44. SYMBOLIC DOMAIN STATEMENT
 * ============================================================================
 *
 * Stable integration point for a domain-level symbolic statement.
 *
 * ============================================================================
 */

symbolicStatement
    : symbolicConstruct
    ;


/* ============================================================================
 * 45. SYMBOLIC BLOCK
 * ============================================================================
 *
 * Blocks contain arbitrary numbers of symbolic constructs.
 *
 * No finite size is imposed.
 *
 * ============================================================================
 */

symbolicBlock
    : LBRACE
      symbolicConstruct*
      RBRACE
    ;


/* ============================================================================
 * 46. SYMBOLIC DECLARATION REGION
 * ============================================================================
 */

symbolicDeclarationRegion
    : symbolicDeclaration*
    ;


/* ============================================================================
 * 47. SYMBOLIC TRANSFORMATION REGION
 * ============================================================================
 */

symbolicTransformationRegion
    : symbolicTransformation*
    ;


/* ============================================================================
 * 48. SYMBOLIC CONSTRAINT REGION
 * ============================================================================
 */

symbolicConstraintRegion
    : symbolicConstraint*
    ;


/* ============================================================================
 * 49. SYMBOLIC COMPUTATION REGION
 * ============================================================================
 *
 * This is the preferred high-level integration boundary for tooling that needs
 * to parse a symbolic computation region.
 *
 * ============================================================================
 */

symbolicComputationRegion
    : symbolicConstruct*
    ;


/* ============================================================================
 * 50. FINAL ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar establishes syntax only.
 *
 * Downstream responsibilities:
 *
 *     name resolution
 *         -> resolves symbolic references
 *
 *     type checking
 *         -> determines symbolic value/type semantics
 *
 *     semantic analysis
 *         -> validates mathematical/domain meaning
 *
 *     symbolic subsystem
 *         -> performs symbolic transformations/evaluation
 *
 *     numerical subsystem
 *         -> performs numerical realization where required
 *
 *     tensor/vector/matrix subsystems
 *         -> realize structured numerical objects
 *
 *     compiler
 *         -> selects representation and lowering strategy
 *
 *     resource subsystem
 *         -> determines available computational resources
 *
 *     optimization
 *         -> optimizes implementation
 *
 *     scheduling
 *         -> determines execution order/timing
 *
 *     hardware abstraction
 *         -> determines target realization
 *
 *     runtime
 *         -> executes the resulting representation
 *
 *     quantum semantic pipeline
 *         -> consumes canonical quantum semantics where symbolic expressions
 *            parameterize quantum operations
 *
 * Symbolic.g4 MUST NEVER become a second implementation of any of these
 * systems.
 *
 * ============================================================================
 */