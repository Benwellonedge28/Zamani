parser grammar ClassicalStatistics;

options { tokenVocab=ZamaniLexer; }

import Expressions;

/*
 * ============================================================================
 * Zamani Classical Statistics Grammar
 * ============================================================================
 *
 * File:
 *     grammar/classical/statistics.g4
 *
 * Purpose:
 *     Defines the classical-statistics grammar boundary for Zamani.
 *
 * Architectural role:
 *     Statistics is a semantic domain layered on top of the canonical
 *     Zamani expression/type system. This grammar provides the domain entry
 *     point without creating a second expression language or a catalogue of
 *     statistical keywords.
 *
 * Authority:
 *     - Canonical lexical authority: ZamaniLexer
 *     - Canonical expression authority: Expressions
 *     - Canonical type authority: Types, through Expressions
 *     - Canonical AST authority: existing domain-neutral frontend AST
 *     - Canonical semantic authority: Zamani semantic analysis
 *     - Canonical IR authority: existing classical semantic/IR pipeline
 *
 * Owns:
 *     - Classical-statistics domain boundary.
 *     - Classification of an already-valid Zamani expression as statistical
 *       when the surrounding grammar/domain dispatcher requires that boundary.
 *
 * Does NOT own:
 *     - Lexer rules or token definitions.
 *     - Identifiers.
 *     - Literals.
 *     - Operator precedence or associativity.
 *     - Assignment syntax.
 *     - Function-call syntax.
 *     - Indexing or member-access syntax.
 *     - Collection syntax.
 *     - Type syntax.
 *     - Variable/function/declaration syntax.
 *     - Statistical algorithms.
 *     - Statistical function names.
 *     - Probability distributions as keywords.
 *     - Fixed sample sizes.
 *     - Fixed dimensions, ranks, widths, precisions, or population sizes.
 *     - Hardware/resource limits.
 *     - Runtime implementation.
 *     - Backend-specific statistical libraries.
 *     - A statistics-specific IR.
 *
 * Design principle:
 *
 *     Statistical mathematics is expressed using the ordinary Zamani
 *     expression language and semantic operations.
 *
 * Examples of valid semantic operations include, but are not limited to:
 *
 *     mean(x)
 *     variance(x)
 *     stddev(x)
 *     median(x)
 *     quantile(x, p)
 *     covariance(x, y)
 *     correlation(x, y)
 *     histogram(x, bins)
 *     distribution(...)
 *     sample(...)
 *     fit(...)
 *     estimate(...)
 *     likelihood(...)
 *     log_likelihood(...)
 *     posterior(...)
 *
 * These names remain identifiers/function names rather than lexer keywords.
 * Consequently, new statistical algorithms can be introduced without
 * modifying the core grammar.
 *
 * This also permits qualified/library operations such as:
 *
 *     statistics::mean(x)
 *     probability::posterior(model, data)
 *
 * whenever the canonical path/call grammar permits them.
 *
 * Scalability:
 *     No statistical limit is encoded here.
 *
 *     In particular, this grammar does NOT impose limits on:
 *       - observations
 *       - samples
 *       - variables
 *       - dimensions
 *       - distribution parameters
 *       - matrix dimensions
 *       - tensor dimensions
 *       - model complexity
 *       - precision
 *       - numerical range
 *       - population size
 *       - feature count
 *       - dataset size
 *       - iteration count
 *       - parallel workers
 *       - machines
 *       - accelerators
 *
 * Resource availability, numerical precision, execution strategy, parallel
 * decomposition, accelerator selection, memory placement, and deployment are
 * determined by later semantic/compiler/runtime/resource layers.
 *
 * POCO-REAF:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     A statistical program expresses computation and requirements rather than
 *     the physical machine on which the computation must execute.
 *
 * Cross-domain examples:
 *
 *     Classical:
 *         mean(values)
 *
 *     Data:
 *         mean(dataset.column)
 *
 *     Tensor:
 *         mean(tensor, axis)
 *
 *     Distributed:
 *         mean(distributed_dataset)
 *
 *     AI:
 *         likelihood(model, observations)
 *
 *     Quantum/hybrid:
 *         estimate(observable_samples)
 *
 *     Hardware:
 *         statistical expressions may appear in constraints, parameters,
 *         verification expressions, or control decisions when semantically
 *         permitted.
 *
 * No domain-specific syntax is duplicated for those cases.
 *
 * Integration:
 *
 *     source
 *       -> ZamaniLexer
 *       -> Expressions
 *       -> ClassicalStatistics
 *       -> domain-neutral AST
 *       -> semantic analysis
 *       -> classical semantic model / canonical IR
 *       -> optimization
 *       -> scheduling/resource analysis
 *       -> target lowering
 *
 *     If a statistical expression participates in a quantum program, it
 *     remains an ordinary expression until semantic analysis and is lowered
 *     through the existing quantum::ir boundary where appropriate.
 *
 * AST:
 *     This grammar introduces no statistics-specific AST node requirement.
 *     The existing generic expression/call/operation representation is used.
 *
 * Semantic analysis:
 *     Determines whether an identifier/call represents a statistical
 *     intrinsic, library operation, user-defined operation, dialect operation,
 *     or invalid operation for its context.
 *
 * IR:
 *     No statistics::ir is introduced.
 *
 * Diagnostics:
 *     Syntax errors are produced by the parser.
 *     Unknown statistical operations, invalid statistical types, invalid
 *     domains, capability failures, numerical restrictions, and unsupported
 *     backend requirements are semantic/compiler/runtime diagnostics.
 *
 * Security:
 *     This grammar contains no embedded actions, code execution, predicates
 *     with side effects, unsafe Rust, or target-specific execution logic.
 *
 * Determinism:
 *     The grammar is deterministic with respect to the canonical expression
 *     grammar and introduces no semantic dispatch based on mutable runtime
 *     state.
 *
 * Compatibility:
 *     Existing generic Zamani expressions remain valid.
 *     Statistical operation names remain ordinary identifiers, avoiding
 *     keyword collisions and preserving source compatibility.
 *
 * Rust:
 *     This is an ANTLR parser grammar consumed by the Rust frontend.
 *     It requires no unsafe Rust and imposes no Rust-version-specific
 *     implementation behavior. The repository target remains Rust 1.97 /
 *     Rust 1.97.1.
 * ============================================================================
 */


/*
 * --------------------------------------------------------------------------
 * Public domain entry point
 * --------------------------------------------------------------------------
 *
 * Classical.g4 imports this grammar as Symbolic/Statistics/etc. and can
 * expose this rule through its classical-domain dispatcher.
 *
 * The rule deliberately delegates the complete expression language to the
 * canonical expression grammar.
 */
classicalStatisticsConstruct
    : classicalStatisticsExpression
    ;


/*
 * --------------------------------------------------------------------------
 * Statistical expression
 * --------------------------------------------------------------------------
 *
 * Statistics does not redefine arithmetic, comparison, logical, indexing,
 * calls, assignment, ranges, collections, lambdas, or other expression
 * syntax.
 *
 * All such syntax remains owned by Expressions.
 */
classicalStatisticsExpression
    : expression
    ;