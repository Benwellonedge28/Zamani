/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/symbolic.g4
 *
 * Status:
 *     Production-ready symbolic-domain parser boundary.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime execution.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the CLASSICAL SYMBOLIC COMPUTATION DOMAIN BOUNDARY.
 *
 * Symbolic computation is intentionally represented using Zamani's canonical
 * expression language.
 *
 * This file does NOT create a second expression language.
 *
 * The canonical expression grammar owns:
 *
 *     expression
 *     assignment
 *     conditional expressions
 *     ranges
 *     logical operators
 *     bitwise operators
 *     equality
 *     relational operators
 *     shifts
 *     arithmetic
 *     prefix operators
 *     postfix operators
 *     calls
 *     indexing
 *     member access
 *     literals
 *     aggregates
 *     lambdas
 *
 * Symbolic semantics are determined after parsing.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     Expressions
 *       |
 *       v
 *     Symbolic
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     name / type / effect / capability analysis
 *       |
 *       v
 *     symbolic semantic interpretation
 *       |
 *       v
 *     canonical semantic model / IR
 *       |
 *       +--------------------+--------------------+
 *       |                    |                    |
 *       v                    v                    v
 *   classical            quantum::ir       other domains
 *       |                    |                    |
 *       +--------------------+--------------------+
 *                            |
 *                            v
 *                     optimization/lowering
 *                            |
 *                     routing/scheduling
 *                            |
 *                     resource analysis
 *                            |
 *                            v
 *                       target realization
 *
 * Symbolic.g4 MUST NOT bypass the canonical frontend AST or semantic model.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the classical symbolic-domain entry point;
 *     - symbolic-domain expression classification;
 *     - symbolic-domain semantic composition points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - token spelling;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - literals;
 *     - arithmetic precedence;
 *     - unary operators;
 *     - binary operators;
 *     - assignment;
 *     - calls;
 *     - indexing;
 *     - member access;
 *     - ranges;
 *     - collections;
 *     - declarations;
 *     - statements;
 *     - function declarations;
 *     - type declarations;
 *     - symbolic evaluation;
 *     - symbolic simplification;
 *     - theorem proving;
 *     - equation solving;
 *     - differentiation algorithms;
 *     - integration algorithms;
 *     - numerical algorithms;
 *     - tensor algorithms;
 *     - matrix algorithms;
 *     - vector algorithms;
 *     - optimization algorithms;
 *     - resource management;
 *     - scheduling;
 *     - routing;
 *     - hardware selection;
 *     - quantum operations;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative expression hierarchy.
 *
 * That authority is:
 *
 *     grammar/expressions/expressions.g4
 *
 * Symbolic.g4 MUST NOT redefine:
 *
 *     expression
 *     additiveExpression
 *     multiplicativeExpression
 *     prefixExpression
 *     postfixExpression
 *     equalityExpression
 *     relationalExpression
 *     assignmentExpression
 *
 * Symbolic computation therefore uses the exact same expression semantics as
 * every other Zamani domain.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * All lexical tokens are supplied by that lexer.
 *
 * This file MUST NOT declare lexer rules.
 *
 * Symbolic mathematical names remain identifiers.
 *
 * Therefore names such as:
 *
 *     sin
 *     cos
 *     tan
 *     exp
 *     log
 *     sqrt
 *     diff
 *     derivative
 *     integrate
 *     integral
 *     limit
 *     sum
 *     product
 *     simplify
 *     expand
 *     factor
 *     substitute
 *     solve
 *     determinant
 *     transpose
 *
 * are NOT hard-coded as an exhaustive symbolic keyword list.
 *
 * They may be:
 *
 *     - standard-library functions;
 *     - user functions;
 *     - compiler intrinsics;
 *     - symbolic-system operations;
 *     - dialect operations;
 *     - future operations.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Symbolic.g4 does not define symbolic types.
 *
 * Type syntax belongs to:
 *
 *     grammar/types/
 *
 * Examples of semantic types that may eventually participate in symbolic
 * computation include:
 *
 *     Integer
 *     Float
 *     Complex
 *     Rational
 *     Vector<T>
 *     Matrix<T, Shape>
 *     Tensor<T, Shape>
 *     Symbolic<T>
 *     Function<...>
 *
 * The grammar does not impose representation limits on those types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing determines structure.
 *
 * Semantic analysis determines meaning.
 *
 * The parser MUST NOT decide whether:
 *
 *     - an identifier denotes a symbol;
 *     - a function is differentiable;
 *     - an expression is integrable;
 *     - an equation is solvable;
 *     - a transformation is valid;
 *     - a symbolic expression is exact;
 *     - a numerical approximation is stable;
 *     - an operation terminates;
 *     - an expression can be simplified;
 *     - a symbolic value can be represented by a selected backend.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SYMBOLIC OPERATION MODEL
 * ============================================================================
 *
 * Symbolic operations are ordinary Zamani expressions.
 *
 * For example, all of the following can be represented without changing the
 * grammar:
 *
 *     sin(x)
 *     derivative(f, x)
 *     integrate(f, x)
 *     simplify(x + x)
 *     factor(p)
 *     expand(p)
 *     substitute(e, x, y)
 *     solve(equation, x)
 *     determinant(A)
 *     transpose(A)
 *
 * The parser does not need a separate rule for every mathematical operation.
 *
 * This is critical for extensibility.
 *
 * A future operation such as:
 *
 *     future_symbolic_operation(...)
 *
 * does not require a grammar change merely because the operation is new.
 *
 * ============================================================================
 * EQUATIONS AND RELATIONS
 * ============================================================================
 *
 * Equations and mathematical relations use the canonical expression
 * comparison/equality syntax.
 *
 * Symbolic.g4 MUST NOT introduce another equality or relational operator
 * vocabulary.
 *
 * For example:
 *
 *     lhs == rhs
 *     x < y
 *     x <= y
 *     x > y
 *     x >= y
 *
 * remain ordinary Zamani expressions.
 *
 * Whether such an expression is interpreted semantically as:
 *
 *     boolean comparison
 *     mathematical equation
 *     symbolic constraint
 *     proof obligation
 *     optimization constraint
 *     resource constraint
 *
 * is determined downstream.
 *
 * ============================================================================
 * CALCULUS
 * ============================================================================
 *
 * Calculus is represented through ordinary symbolic expressions and calls.
 *
 * Examples:
 *
 *     derivative(f, x)
 *     derivative(f, x, order)
 *     integral(f, x)
 *     integral(f, x, lower, upper)
 *     limit(f, x, value)
 *     gradient(f, x)
 *     jacobian(f, x)
 *     hessian(f, x)
 *
 * The grammar does not implement calculus.
 *
 * The grammar does not reserve these names.
 *
 * The symbolic semantic subsystem determines their meaning.
 *
 * ============================================================================
 * ALGEBRA
 * ============================================================================
 *
 * Algebraic computation uses canonical Zamani expressions.
 *
 * Examples:
 *
 *     x + y
 *     x * y
 *     x / y
 *     x ^ y
 *
 * where an exponentiation operator is supported by the canonical expression
 * specification.
 *
 * IMPORTANT:
 *
 * Symbolic.g4 MUST NOT introduce a local exponentiation token or precedence
 * rule.
 *
 * If exponentiation is standardized in Zamani, it belongs exactly once in the
 * canonical lexer/expression precedence system.
 *
 * ============================================================================
 * SYMBOLIC DOMAINS
 * ============================================================================
 *
 * Symbolic expressions may represent:
 *
 *     - variables;
 *     - constants;
 *     - functions;
 *     - equations;
 *     - inequalities;
 *     - constraints;
 *     - polynomials;
 *     - rational expressions;
 *     - series;
 *     - sequences;
 *     - sets;
 *     - symbolic vectors;
 *     - symbolic matrices;
 *     - symbolic tensors;
 *     - symbolic quantities;
 *     - symbolic units;
 *     - symbolic dimensions;
 *     - symbolic probabilities;
 *     - symbolic distributions;
 *     - symbolic optimization expressions;
 *     - symbolic physical models;
 *     - symbolic quantum parameters.
 *
 * These are semantic classifications rather than separate parser languages.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Symbolic expressions may participate in:
 *
 *     classical computation
 *     numerical computation
 *     vector computation
 *     matrix computation
 *     tensor computation
 *     AI/ML
 *     scientific computing
 *     optimization
 *     quantum parameterization
 *     hybrid quantum/classical computation
 *     HDL parameters
 *     hardware requirements
 *     resource expressions
 *     distributed computation
 *     compile-time computation
 *     runtime computation
 *
 * The same expression syntax is intentionally reused.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Symbolic values may parameterize quantum operations.
 *
 * Conceptually:
 *
 *     symbolic expression
 *          |
 *          v
 *     semantic quantum parameter
 *          |
 *          v
 *     quantum operation
 *          |
 *          v
 *     quantum::ir
 *
 * Symbolic.g4 MUST NOT:
 *
 *     - define quantum gates;
 *     - enumerate gate names;
 *     - define physical qubits;
 *     - define physical topology;
 *     - define QEC;
 *     - define routing;
 *     - define scheduling;
 *     - define QZN/ZQN behavior;
 *     - define HAL behavior.
 *
 * Quantum semantic ownership remains with the existing quantum pipeline.
 *
 * ============================================================================
 * HARDWARE / POCO-REAF CONTRACT
 * ============================================================================
 *
 * Symbolic.g4 imposes NO universal finite limits on:
 *
 *     - number of symbols;
 *     - number of variables;
 *     - number of terms;
 *     - polynomial degree;
 *     - expression size;
 *     - expression depth;
 *     - function arity;
 *     - derivative order;
 *     - integration nesting;
 *     - summation nesting;
 *     - product nesting;
 *     - matrix dimensions;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - numerical precision;
 *     - integer width;
 *     - floating-point width;
 *     - thread count;
 *     - core count;
 *     - GPU count;
 *     - accelerator count;
 *     - memory capacity;
 *     - node count;
 *     - device count.
 *
 * No MAX_* language constant is defined here.
 *
 * A program's mathematical values and dimensions may be finite program
 * semantics without becoming universal language limits.
 *
 * For example:
 *
 *     Matrix<3, 3>
 *
 * can describe a 3-by-3 mathematical object.
 *
 * It MUST NOT mean:
 *
 *     "Zamani matrices may never exceed 3-by-3."
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Symbolic expression lists and nested expressions use canonical expression
 * structures and therefore do not encode fixed mathematical capacities.
 *
 * Examples:
 *
 *     f(x)
 *
 *     f(x, y, z, ...)
 *
 *     operation(a, b, c, ...)
 *
 *     nested(nested(nested(expression)))
 *
 * are constrained only by actual parser/compiler/resource policies.
 *
 * The grammar itself establishes no artificial finite maximum.
 *
 * "Infinity" in the POCO-REAF context means that the language does not impose
 * an arbitrary finite hardware-derived ceiling on symbolic computation.
 *
 * Physical execution remains subject to actual resources and mathematical
 * computability.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Symbolic expressions MUST lower into the existing domain-neutral frontend
 * AST.
 *
 * The preferred operation representation remains structurally equivalent to:
 *
 *     Operation {
 *         name,
 *         namespace,
 *         operands,
 *         parameters,
 *         results,
 *         attributes,
 *         modifiers,
 *         effects,
 *         capabilities,
 *         source
 *     }
 *
 * Symbolic.g4 MUST NOT introduce a separate:
 *
 *     SymbolicExpression AST
 *     SymbolicOperation AST
 *     SymbolicIR
 *
 * merely because the source expression is interpreted symbolically.
 *
 * Semantic classification belongs downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Symbolic.g4 does not define an IR.
 *
 * Symbolic expressions lower through the repository's canonical semantic/IR
 * pipeline.
 *
 * Depending on semantic classification, an expression may eventually feed:
 *
 *     classical representation
 *     numerical representation
 *     tensor representation
 *     constraint representation
 *     optimization representation
 *     quantum::ir
 *     HDL/hardware semantic representation
 *
 * No second symbolic IR is introduced by this grammar.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages are responsible for:
 *
 *     - constant evaluation;
 *     - symbolic simplification;
 *     - algebraic transformation;
 *     - specialization;
 *     - numerical lowering;
 *     - differentiation;
 *     - integration;
 *     - solver selection;
 *     - optimization;
 *     - target lowering.
 *
 * Symbolic.g4 performs none of these operations.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The parser performs no symbolic evaluation.
 *
 * A source expression such as:
 *
 *     solve(problem, x)
 *
 * MUST NOT execute a solver while parsing.
 *
 * Runtime execution is determined only after semantic analysis and lowering.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token sequence;
 *     - active language version;
 *     - canonical grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - CPU availability;
 *     - GPU availability;
 *     - QPU availability;
 *     - memory capacity;
 *     - device state;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - random state;
 *     - scheduler state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Symbolic syntax is non-executing.
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no unsafe code;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no process execution;
 *     - no dynamic library loading;
 *     - no hardware discovery;
 *     - no device access;
 *     - no runtime evaluation.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Semantic errors belong downstream.
 *
 * Examples of semantic diagnostics include:
 *
 *     - unresolved symbolic name;
 *     - invalid symbolic type;
 *     - invalid operation;
 *     - invalid operand domain;
 *     - incompatible dimensions;
 *     - invalid symbolic constraint;
 *     - unsupported capability;
 *     - unavailable target realization.
 *
 * These MUST NOT be encoded as parser alternatives.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Historical symbolic syntax from older monolithic grammar documents must not
 * automatically become canonical syntax.
 *
 * Compatibility follows the repository language-version process:
 *
 *     specified
 *         ->
 *     implemented
 *         ->
 *     tested
 *         ->
 *     stable
 *
 * Legacy syntax must be explicitly classified as:
 *
 *     compatible
 *     deprecated
 *     experimental
 *     historical
 *     unsupported
 *
 * ============================================================================
 * PUBLIC GRAMMAR CONTRACT
 * ============================================================================
 *
 * The public symbolic entry point is:
 *
 *     symbolicConstruct
 *
 * It delegates to the canonical expression rule.
 *
 * ============================================================================
 */

parser grammar ClassicalSymbolic;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * PUBLIC SYMBOLIC DOMAIN ENTRY
 * ============================================================================
 *
 * Symbolic computation is structurally an ordinary Zamani expression.
 *
 * The symbolic semantic subsystem classifies the resulting AST according to
 * symbol/type/domain information.
 *
 * ============================================================================
 */

symbolicConstruct
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC EXPRESSION
 * ============================================================================
 *
 * This is intentionally a thin semantic-domain boundary.
 *
 * ALL expression precedence and syntax come from Expressions.
 *
 * This rule MUST NOT be expanded into another arithmetic hierarchy.
 *
 * ============================================================================
 */

symbolicExpression
    : expression
    ;


/*
 * ============================================================================
 * SYMBOLIC VALUE
 * ============================================================================
 *
 * Alias used by semantic/domain consumers that need to state explicitly that
 * an expression is being consumed as a symbolic value.
 *
 * It creates no new AST representation.
 *
 * ============================================================================
 */

symbolicValue
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC OPERATION
 * ============================================================================
 *
 * A symbolic operation is represented by the canonical expression language.
 *
 * This intentionally accepts calls, operators, member access, indexing,
 * nested expressions, lambdas, literals, and future expression forms through
 * the canonical `expression` rule.
 *
 * Operation names remain ordinary identifiers resolved semantically.
 *
 * ============================================================================
 */

symbolicOperation
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC RELATION
 * ============================================================================
 *
 * Relations are ordinary canonical expressions.
 *
 * Equality and relational operators are owned by Expressions.
 *
 * The semantic layer determines whether the expression is interpreted as a
 * mathematical relation, boolean predicate, equation, or constraint.
 *
 * ============================================================================
 */

symbolicRelation
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC CONSTRAINT
 * ============================================================================
 *
 * Constraint meaning belongs to semantic analysis.
 *
 * The syntax remains the canonical expression syntax.
 *
 * ============================================================================
 */

symbolicConstraint
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC TRANSFORMATION
 * ============================================================================
 *
 * Transformations such as:
 *
 *     simplify(...)
 *     expand(...)
 *     factor(...)
 *     substitute(...)
 *     transform(...)
 *
 * are ordinary expressions/calls.
 *
 * The operation name and transformation semantics are resolved downstream.
 *
 * ============================================================================
 */

symbolicTransformation
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC CALCULUS
 * ============================================================================
 *
 * Calculus operations such as:
 *
 *     derivative(...)
 *     integrate(...)
 *     limit(...)
 *     gradient(...)
 *     jacobian(...)
 *     hessian(...)
 *
 * remain ordinary expressions.
 *
 * ============================================================================
 */

symbolicCalculus
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC ALGEBRA
 * ============================================================================
 *
 * Algebraic operations remain canonical expressions.
 *
 * No algebra-specific precedence is introduced here.
 *
 * ============================================================================
 */

symbolicAlgebra
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC MATHEMATICAL OBJECT
 * ============================================================================
 *
 * Semantic analysis may classify an expression as:
 *
 *     scalar
 *     polynomial
 *     rational expression
 *     vector
 *     matrix
 *     tensor
 *     function
 *     set
 *     sequence
 *     relation
 *     constraint
 *     operator
 *     distribution
 *     another supported mathematical object
 *
 * ============================================================================
 */

symbolicMathematicalObject
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC PARAMETER
 * ============================================================================
 *
 * Parameter syntax is owned by the canonical expression/type/function
 * systems. This rule is only a semantic integration point.
 *
 * ============================================================================
 */

symbolicParameter
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC DOMAIN
 * ============================================================================
 *
 * Domains may be represented by arbitrary expressions.
 *
 * No finite domain cardinality is imposed here.
 *
 * ============================================================================
 */

symbolicDomain
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC SEQUENCE
 * ============================================================================
 *
 * Sequence construction remains an expression concern.
 *
 * ============================================================================
 */

symbolicSequence
    : symbolicExpression
    ;


/*
 * ============================================================================
 * SYMBOLIC REGION
 * ============================================================================
 *
 * This permits tools that explicitly consume a symbolic region to reuse the
 * canonical expression boundary.
 *
 * It does not define a second program grammar.
 *
 * ============================================================================
 */

symbolicRegion
    : symbolicConstruct*
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has one canonical lexer vocabulary.
 * [x] It contains no lexer rules.
 * [x] It imports the canonical Expressions grammar.
 * [x] It does not redefine expression precedence.
 * [x] It does not redefine identifiers.
 * [x] It does not redefine literals.
 * [x] It does not redefine assignment.
 * [x] It does not redefine declarations.
 * [x] It does not redefine statements.
 * [x] It does not redefine calls.
 * [x] It does not redefine indexing.
 * [x] It does not redefine member access.
 * [x] It does not redefine types.
 * [x] It does not enumerate mathematical function names.
 * [x] It does not create a symbolic AST.
 * [x] It does not create a symbolic IR.
 * [x] It does not create a quantum IR.
 * [x] It does not enumerate quantum gates.
 * [x] It does not select hardware.
 * [x] It contains no machine-size limits.
 * [x] It contains no MAX_* scalability constants.
 * [x] It contains no embedded Rust.
 * [x] It contains no unsafe Rust.
 * [x] It remains target-independent.
 * [x] It preserves POCO-REAF.
 *
 * Integration acceptance additionally requires:
 *
 *     - Classical imports this grammar exactly once;
 *     - the root composition grammar exposes the symbolic domain through
 *       Classical rather than importing a second symbolic grammar;
 *     - symbolic expressions lower to the existing domain-neutral AST;
 *     - semantic analysis classifies symbolic values;
 *     - canonical IR lowering exists for supported symbolic semantics;
 *     - parser diagnostics preserve source spans;
 *     - positive tests exist;
 *     - negative tests exist;
 *     - boundary tests exist;
 *     - scalability tests exist;
 *     - determinism tests exist;
 *     - compatibility tests exist.
 *
 * ============================================================================
 */