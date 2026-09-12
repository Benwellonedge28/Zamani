/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/comprehensions.g4
 *
 * Role:
 *     Canonical source-level comprehension-expression syntax.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions.
 *     No unsafe code is used or required.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the syntax of expression-level comprehensions.
 *
 * A comprehension is a compact source-level expression for constructing or
 * lazily describing a collection from one or more iteration clauses,
 * optional filtering predicates, and a result expression.
 *
 * Conceptually:
 *
 *     [ transform(x) for x in source if predicate(x) ]
 *
 *     { transform(x) for x in source if predicate(x) }
 *
 *     { key(x): value(x) for x in source }
 *
 *     ( transform(x) for x in source )
 *
 *     [ for x in source yield transform(x) ]
 *
 * The grammar describes syntax only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - comprehension-expression syntax;
 *   - collection-comprehension delimiters;
 *   - generator clauses;
 *   - comprehension bindings;
 *   - comprehension source expressions;
 *   - comprehension filters;
 *   - optional binding guards;
 *   - result/yield expressions;
 *   - nested comprehension clauses;
 *   - map-comprehension entry syntax;
 *   - set-comprehension syntax;
 *   - list/array-comprehension syntax;
 *   - generator-expression syntax;
 *   - syntactic support for destructuring through the canonical pattern rule;
 *   - syntactic support for asynchronous comprehension clauses;
 *   - syntactic support for parallel comprehension intent;
 *   - syntactic support for reduction/folding intent where represented by the
 *     language syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - keywords;
 *   - punctuation;
 *   - identifiers;
 *   - patterns in general;
 *   - general expressions;
 *   - lambda expressions;
 *   - types;
 *   - type checking;
 *   - collection implementations;
 *   - iterator implementations;
 *   - lazy evaluation;
 *   - eager evaluation;
 *   - parallel execution;
 *   - scheduling;
 *   - resource allocation;
 *   - hardware selection;
 *   - accelerator selection;
 *   - quantum allocation;
 *   - physical qubit placement;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR;
 *   - optimization;
 *   - routing;
 *   - QEC;
 *   - ZQN;
 *   - runtime execution;
 *   - backend selection;
 *   - machine-specific resource limits.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *     |
 *     v
 * ZamaniLexer
 *     |
 *     v
 * Comprehensions parser rules
 *     |
 *     v
 * Frontend AST
 *     |
 *     +--> name resolution
 *     +--> pattern checking
 *     +--> type checking
 *     +--> effect checking
 *     +--> capability checking
 *     +--> resource analysis
 *     |
 *     v
 * Canonical semantic representation
 *     |
 *     +--> classical IR
 *     +--> quantum::ir
 *     +--> data/control IR
 *     +--> resource/effect metadata
 *     |
 *     v
 * optimization
 *     |
 *     v
 * lowering / routing / scheduling
 *     |
 *     v
 * target realization
 *
 * A comprehension is source syntax.
 *
 * It must not directly create an execution plan.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Comprehensions describe the computation rather than the machine used to
 * execute it.
 *
 * For example:
 *
 *     [f(x) for x in values]
 *
 * does NOT mean:
 *
 *     one CPU core
 *     one thread
 *     one GPU
 *     one accelerator
 *     one node
 *
 * The compiler/runtime may lower the same comprehension into:
 *
 *     sequential execution
 *     vectorized execution
 *     SIMD execution
 *     multicore execution
 *     GPU execution
 *     accelerator execution
 *     distributed execution
 *     streaming execution
 *     quantum-assisted execution
 *     hardware-generated implementation
 *
 * according to semantic requirements, effects, capabilities, resources,
 * optimization opportunities, scheduling policy, and target availability.
 *
 * The grammar therefore contains no machine-size assumptions.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *   - number of comprehension clauses;
 *   - number of nested comprehensions;
 *   - number of filters;
 *   - number of bindings;
 *   - number of collection elements;
 *   - collection size;
 *   - iterator size;
 *   - generated result size;
 *   - tensor dimensions;
 *   - quantum-resource counts;
 *   - accelerator counts;
 *   - distributed nodes;
 *   - execution targets.
 *
 * Any actual limit is imposed by the compiler, semantic validator,
 * resource manager, scheduler, runtime, or deployment environment.
 *
 * "Infinity" therefore means that this grammar introduces no artificial
 * finite machine-scale ceiling. Actual execution remains bounded only by
 * available resources and explicitly defined implementation policies.
 *
 * ============================================================================
 * ANTLR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Tokens come from the canonical Zamani lexer:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file MUST NOT define lexer rules.
 *
 * In particular, do not define local versions of:
 *
 *     IDENTIFIER
 *     FOR
 *     IN
 *     IF
 *     WHEN
 *     YIELD
 *     ASYNC
 *     PARALLEL
 *     COMMA
 *     COLON
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *
 * The canonical lexer owns token spelling.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file intentionally references canonical rules supplied by the parser
 * composition layer:
 *
 *     expression
 *     identifier
 *     pattern
 *
 * Where the repository's final composed parser uses a different canonical
 * pattern rule name, the composition layer MUST provide the documented bridge
 * without changing the semantics of this grammar.
 *
 * This file MUST NOT duplicate:
 *
 *     expression
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     pattern
 *
 * because doing so would create competing language definitions.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical expression grammar should expose:
 *
 *     comprehensionExpression
 *
 * from its primary-expression alternatives.
 *
 * Conceptually:
 *
 *     primaryExpression
 *         : ...
 *         | comprehensionExpression
 *         ;
 *
 * The canonical expression grammar remains responsible for determining where
 * an expression is legal.
 *
 * This file owns what a comprehension looks like once the parser enters that
 * construct.
 *
 * ============================================================================
 * BASIC FORMS
 * ============================================================================
 *
 * The canonical comprehension forms supported here are:
 *
 *     [expression for pattern in expression]
 *
 *     [expression for pattern in expression if expression]
 *
 *     {expression for pattern in expression}
 *
 *     {key : value for pattern in expression}
 *
 *     (expression for pattern in expression)
 *
 * and an explicit yield form:
 *
 *     [for pattern in expression yield expression]
 *
 * Multiple clauses are permitted:
 *
 *     [f(x, y)
 *         for x in xs
 *         for y in ys]
 *
 * Filters may occur after any generator clause:
 *
 *     [f(x)
 *         for x in xs
 *         if predicate(x)
 *         for y in ys
 *         if predicate2(y)]
 *
 * ============================================================================
 * SEMANTIC NEUTRALITY
 * ============================================================================
 *
 * The grammar does not decide whether a comprehension is:
 *
 *     eager
 *     lazy
 *     streaming
 *     parallel
 *     vectorized
 *     distributed
 *     asynchronous
 *     fused
 *     materialized
 *
 * Those properties are semantic or compiler decisions.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Comprehensions may syntactically operate on quantum-related values:
 *
 *     [prepare(x) for x in states]
 *
 *     [measure(q) for q in register]
 *
 *     [observable(q) for q in qubits]
 *
 * However, this grammar MUST NOT:
 *
 *     - allocate qubits;
 *     - determine physical qubits;
 *     - determine QPU size;
 *     - construct quantum::ir;
 *     - perform routing;
 *     - perform scheduling;
 *     - invoke QEC;
 *     - interpret ZQN faults.
 *
 * If a comprehension semantically represents quantum computation, semantic
 * lowering decides how that computation enters the canonical quantum::ir
 * boundary.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA INTEGRATION
 * ============================================================================
 *
 * The same syntax may operate over:
 *
 *     arrays
 *     lists
 *     maps
 *     sets
 *     streams
 *     tensors
 *     datasets
 *     distributed collections
 *     accelerator buffers
 *     symbolic collections
 *     user-defined iterable values
 *
 * No domain-specific collection implementation is embedded here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A comprehension may describe generated hardware structures or repeated
 * hardware-level computation when interpreted by the HDL/hardware semantic
 * layers.
 *
 * The grammar does not decide:
 *
 *     number of gates
 *     number of registers
 *     number of lanes
 *     number of hardware instances
 *     FPGA capacity
 *     ASIC area
 *     clock frequency
 *     physical placement
 *
 * Such properties belong to hardware semantics, resource constraints,
 * synthesis, scheduling, or target realization.
 *
 * ============================================================================
 */

parser grammar Comprehensions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Complete comprehension expression.
 *
 * The delimiter determines the source-level collection/computation form:
 *
 *     [ ... ]       list/array-style comprehension
 *     { ... }       set/map-style comprehension
 *     ( ... )       generator-style comprehension
 *
 * The semantic layer determines the concrete resulting abstraction.
 */
comprehensionExpression
    : listComprehension
    | setComprehension
    | mapComprehension
    | generatorComprehension
    ;


/* ============================================================================
 * 2. LIST / ARRAY COMPREHENSIONS
 * ========================================================================== */

/**
 * List/array comprehension.
 *
 * Examples:
 *
 *     [x * 2 for x in values]
 *
 *     [x * 2 for x in values if x > 0]
 *
 *     [f(x, y) for x in xs for y in ys]
 *
 *     [f(x) for x in xs if p(x) for y in ys if q(y)]
 */
listComprehension
    : LBRACKET
      comprehensionBody
      RBRACKET
    ;


/* ============================================================================
 * 3. SET COMPREHENSIONS
 * ========================================================================== */

/**
 * Set comprehension.
 *
 * Examples:
 *
 *     {x for x in values}
 *
 *     {f(x) for x in values if predicate(x)}
 *
 * The parser distinguishes set and map forms structurally.
 */
setComprehension
    : LBRACE
      setComprehensionBody
      RBRACE
    ;


/* ============================================================================
 * 4. MAP COMPREHENSIONS
 * ========================================================================== */

/**
 * Map/associative comprehension.
 *
 * Examples:
 *
 *     {key(x): value(x) for x in values}
 *
 *     {key(x): value(x)
 *         for x in values
 *         if predicate(x)}
 */
mapComprehension
    : LBRACE
      mapComprehensionBody
      RBRACE
    ;


/* ============================================================================
 * 5. GENERATOR COMPREHENSIONS
 * ========================================================================== */

/**
 * Generator expression.
 *
 * Examples:
 *
 *     (x for x in values)
 *
 *     (f(x) for x in values if predicate(x))
 *
 * Generator laziness is a semantic property and is not encoded as a runtime
 * implementation choice here.
 */
generatorComprehension
    : LPAREN
      generatorComprehensionBody
      RPAREN
    ;


/* ============================================================================
 * 6. COMMON COMPREHENSION BODY
 * ========================================================================== */

/**
 * List comprehension body.
 *
 * Two equivalent source-level families are supported:
 *
 *     result FOR clauses
 *
 * and:
 *
 *     FOR clauses YIELD result
 *
 * The explicit-yield form is useful when a comprehension needs to make the
 * binding portion visually dominant or when generated source is produced by
 * tooling.
 */
comprehensionBody
    : expression
      comprehensionClauses
    | comprehensionClauses
      K_YIELD
      expression
    ;


/**
 * Set comprehension body.
 *
 * A set comprehension produces one result expression per accepted iteration.
 */
setComprehensionBody
    : expression
      comprehensionClauses
    | comprehensionClauses
      K_YIELD
      expression
    ;


/**
 * Map comprehension body.
 *
 * The map key and value are both ordinary expressions.
 */
mapComprehensionBody
    : mapComprehensionResult
      comprehensionClauses
    | comprehensionClauses
      K_YIELD
      mapComprehensionResult
    ;


/**
 * Generator comprehension body.
 */
generatorComprehensionBody
    : expression
      comprehensionClauses
    | comprehensionClauses
      K_YIELD
      expression
    ;


/* ============================================================================
 * 7. MAP RESULT
 * ========================================================================== */

/**
 * Map key/value expression pair.
 *
 * The colon belongs to map-expression syntax, not to the general expression
 * grammar.
 */
mapComprehensionResult
    : expression
      COLON
      expression
    ;


/* ============================================================================
 * 8. CLAUSE SEQUENCE
 * ========================================================================== */

/**
 * One or more generator/filter clauses.
 *
 * No finite clause count is encoded.
 *
 * This permits arbitrarily nested Cartesian products, joins, pipelines, and
 * staged filtering, subject only to implementation resources.
 */
comprehensionClauses
    : comprehensionClause+
    ;


/**
 * A comprehension clause is either:
 *
 *     a generator
 *     a filter
 *
 * Generator clauses introduce bindings.
 *
 * Filter clauses constrain whether the current iteration contributes to the
 * result.
 */
comprehensionClause
    : comprehensionGenerator
    | comprehensionFilter
    ;


/* ============================================================================
 * 9. GENERATOR CLAUSES
 * ========================================================================== */

/**
 * Canonical generator:
 *
 *     for pattern in expression
 *
 * Examples:
 *
 *     for x in xs
 *
 *     for (x, y) in pairs
 *
 *     for q in qubits
 *
 *     for row in matrix
 *
 *     for sample in dataset
 *
 * The source expression may have any semantic type that supports the required
 * iteration protocol.
 */
comprehensionGenerator
    : K_FOR
      comprehensionPattern
      K_IN
      expression
      comprehensionGeneratorModifier*
    ;


/**
 * Optional generator modifiers.
 *
 * These express source-level execution intent only.
 *
 * They do not select hardware or impose machine topology.
 */
comprehensionGeneratorModifier
    : comprehensionAsyncModifier
    | comprehensionParallelModifier
    ;


/**
 * Explicit asynchronous iteration intent.
 *
 * The exact asynchronous semantics are determined by the concurrency/effects
 * subsystem.
 */
comprehensionAsyncModifier
    : K_AWAIT
    ;


/**
 * Explicit parallel intent.
 *
 * This does NOT mean:
 *
 *     one thread per element
 *
 *     one core per element
 *
 *     one GPU lane per element
 *
 * or any other fixed mapping.
 *
 * The compiler/runtime determines the legal execution strategy.
 */
comprehensionParallelModifier
    : K_PARALLEL
    ;


/* ============================================================================
 * 10. FILTER CLAUSES
 * ========================================================================== */

/**
 * Filtering predicate.
 *
 * Both forms are accepted:
 *
 *     if predicate
 *
 * and:
 *
 *     when predicate
 *
 * `when` is useful for Zamani's broader guarded-computation vocabulary and
 * remains semantically distinct at the AST level if desired.
 */
comprehensionFilter
    : K_IF
      expression
    | K_WHEN
      expression
    ;


/* ============================================================================
 * 11. PATTERN BINDINGS
 * ========================================================================== */

/**
 * Comprehension binding pattern.
 *
 * The canonical pattern grammar owns the actual pattern language.
 *
 * This bridge deliberately prevents comprehensions from defining a second
 * pattern system.
 *
 * Supported conceptual examples include:
 *
 *     x
 *     _
 *     (x, y)
 *     [x, y]
 *     Point { x, y }
 *     Some(value)
 *
 * The semantic layer determines whether the selected pattern is compatible
 * with the iteration value.
 */
comprehensionPattern
    : pattern
    ;


/* ============================================================================
 * 12. OPTIONAL INDEX / ENUMERATION INTENT
 * ========================================================================== */

/**
 * Explicit enumeration binding.
 *
 * This form allows source-level enumeration without imposing an implementation
 * strategy.
 *
 * Example:
 *
 *     [i, x for i, x in enumerate(values)]
 *
 * remains ordinary comprehension syntax because `enumerate` is an expression.
 *
 * Therefore no special index counter is required here.
 *
 * This rule exists only as documentation of the architectural decision:
 * enumeration belongs to expression/library semantics rather than hidden
 * machine state.
 */


/* ============================================================================
 * 13. NESTED COMPREHENSIONS
 * ========================================================================== */

/**
 * Nested comprehensions are naturally supported because `expression` may
 * contain another `comprehensionExpression`.
 *
 * Example:
 *
 *     [[f(x, y) for y in ys] for x in xs]
 *
 * No special recursion limit is encoded.
 */


/* ============================================================================
 * 14. COMPREHENSION + LAMBDA INTEGRATION
 * ========================================================================== */

/**
 * A result expression or filter may contain a lambda expression because both
 * are consumed through the canonical `expression` rule.
 *
 * Examples:
 *
 *     [map_fn(x) for x in values]
 *
 *     [apply(|x| x + 1, x) for x in values]
 *
 * This file does NOT redefine lambda syntax.
 *
 * Lambda syntax remains owned by:
 *
 *     grammar/expressions/lambdas.g4
 *
 * and is exposed through the canonical expression grammar.
 */


/* ============================================================================
 * 15. COMPREHENSION + QUANTUM INTEGRATION
 * ========================================================================== */

/**
 * Quantum expressions may appear anywhere this grammar accepts `expression`.
 *
 * Examples:
 *
 *     [measure(q) for q in qubits]
 *
 *     [expectation(observable, q) for q in register]
 *
 *     [prepare(state) for state in states]
 *
 * The parser does not determine whether these are legal quantum programs.
 *
 * Semantic analysis must determine:
 *
 *     - whether the values are quantum values;
 *     - whether iteration is semantically valid;
 *     - whether measurement is permitted;
 *     - whether operations may be reordered;
 *     - whether the resulting computation lowers into quantum::ir.
 *
 * This file does not construct quantum::ir.
 */


/* ============================================================================
 * 16. COMPREHENSION + CLASSICAL INTEGRATION
 * ========================================================================== */

/**
 * Classical computation may use comprehensions over:
 *
 *     arrays
 *     vectors
 *     maps
 *     sets
 *     streams
 *     iterators
 *     user-defined collections
 *     tensors
 *     symbolic values
 *     datasets
 *
 * Collection semantics are not defined here.
 */


/* ============================================================================
 * 17. COMPREHENSION + HDL / HARDWARE INTEGRATION
 * ========================================================================== */

/**
 * Hardware-oriented semantic layers may interpret comprehensions as a compact
 * description of repeated structures or computations.
 *
 * For example, a semantic layer could lower:
 *
 *     [cell(x) for x in cells]
 *
 * into a hardware-generation representation.
 *
 * This grammar does NOT decide:
 *
 *     - number of cells;
 *     - number of registers;
 *     - number of lanes;
 *     - number of pipeline stages;
 *     - FPGA capacity;
 *     - ASIC area;
 *     - clock frequency;
 *     - placement;
 *     - routing.
 */


/* ============================================================================
 * 18. DISTRIBUTED / PARALLEL INTEGRATION
 * ========================================================================== */

/**
 * A comprehension may carry `parallel` source-level intent.
 *
 * Example:
 *
 *     [f(x) for x in values parallel]
 *
 * The grammar records the intent.
 *
 * The semantic/compiler layers determine whether the computation can actually
 * be parallelized while preserving:
 *
 *     - ordering;
 *     - effects;
 *     - dependencies;
 *     - determinism;
 *     - resource constraints;
 *     - failure semantics;
 *     - numerical/quantum semantics.
 *
 * No node count, worker count, thread count, or device count is encoded.
 */


/* ============================================================================
 * 19. EFFECT / CAPABILITY INTEGRATION
 * ========================================================================== */

/**
 * Comprehensions can contain expressions with effects.
 *
 * The grammar does not classify those effects.
 *
 * Semantic analysis determines whether a comprehension is compatible with:
 *
 *     IO
 *     state
 *     mutation
 *     networking
 *     hardware
 *     quantum operations
 *     distributed execution
 *     security-sensitive operations
 *     user-defined effects
 *
 * The effect system remains the authority for those decisions.
 */


/* ============================================================================
 * 20. RESOURCE / CAPABILITY INTEGRATION
 * ========================================================================== */

/**
 * Resource and capability requirements may occur inside comprehension
 * expressions through ordinary expressions and surrounding declarations.
 *
 * This grammar MUST NOT introduce resource limits such as:
 *
 *     maximum elements
 *     maximum iterations
 *     maximum workers
 *     maximum memory
 *     maximum devices
 *     maximum qubits
 *     maximum accelerators
 *
 * Resource requirements belong to the resource/capability semantic layers.
 */


/* ============================================================================
 * 21. DETERMINISM
 * ========================================================================== */

/**
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment reads;
 *     - no randomness;
 *     - no target-dependent branches.
 *
 * Parsing therefore depends only on the supplied token stream and parser
 * configuration.
 *
 * Execution ordering is NOT determined by this grammar.
 */


/* ============================================================================
 * 22. AST CONTRACT
 * ========================================================================== */

/**
 * The frontend AST should preserve at least:
 *
 *     ComprehensionExpression
 *         kind
 *         result
 *         clauses
 *         source span
 *
 * where each generator contains:
 *
 *     pattern
 *     source
 *     modifiers
 *
 * and each filter contains:
 *
 *     predicate
 *     source span
 *
 * Map comprehensions additionally preserve:
 *
 *     key
 *     value
 *
 * The AST MUST preserve source ordering of clauses.
 *
 * This is necessary because:
 *
 *     for x in xs
 *     for y in ys
 *
 * is not necessarily equivalent to:
 *
 *     for y in ys
 *     for x in xs
 *
 * especially when effects, finite resources, ordering, or dependent sources
 * are involved.
 */


/* ============================================================================
 * 23. SEMANTIC CONTRACT
 * ========================================================================== */

/**
 * Semantic analysis is responsible for:
 *
 *     - resolving bound names;
 *     - determining iterable semantics;
 *     - checking pattern compatibility;
 *     - checking predicate type;
 *     - checking result type;
 *     - checking map key/value validity;
 *     - checking effect compatibility;
 *     - checking mutation rules;
 *     - checking ownership/borrowing rules;
 *     - checking async legality;
 *     - checking parallel legality;
 *     - checking ordering requirements;
 *     - checking resource requirements;
 *     - checking capability requirements;
 *     - checking quantum legality;
 *     - checking hardware legality;
 *     - determining whether lazy/eager lowering is valid.
 *
 * The parser performs none of these tasks.
 */


/* ============================================================================
 * 24. LOWERING CONTRACT
 * ========================================================================== */

/**
 * A comprehension may lower into different semantic forms.
 *
 * Possible lowerings include:
 *
 *     iterator construction
 *     map/filter/fold chains
 *     explicit loops
 *     data-parallel regions
 *     streaming pipelines
 *     tensor operations
 *     accelerator kernels
 *     distributed computation
 *     hardware-generation constructs
 *
 * Lowering must preserve the source semantics.
 *
 * The grammar does not select a lowering.
 */


/* ============================================================================
 * 25. QUANTUM IR BOUNDARY
 * ========================================================================== */

/**
 * If a comprehension eventually describes quantum computation, the pipeline is:
 *
 *     comprehension syntax
 *         ->
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     canonical quantum semantic lowering
 *         ->
 *     quantum::ir
 *
 * This grammar MUST NOT:
 *
 *     - instantiate quantum IR;
 *     - create gate objects;
 *     - assign physical qubits;
 *     - infer topology;
 *     - perform scheduling;
 *     - perform routing;
 *     - invoke QEC;
 *     - interpret ZQN noise/fault semantics.
 */


/* ============================================================================
 * 26. CLASSICAL IR BOUNDARY
 * ========================================================================== */

/**
 * Classical comprehension lowering belongs to the compiler's canonical
 * classical/data/control representation.
 *
 * This grammar MUST NOT define a competing classical IR.
 */


/* ============================================================================
 * 27. RUNTIME CONTRACT
 * ========================================================================== */

/**
 * Runtime code MUST NOT depend on this grammar for comprehension execution.
 *
 * The runtime consumes compiled/semantic representations produced downstream.
 *
 * Runtime resource availability may change independently of source syntax.
 *
 * Therefore:
 *
 *     source comprehension
 *
 * remains independent from:
 *
 *     current worker count
 *     current CPU count
 *     current GPU count
 *     current QPU
 *     current node count
 *     current memory capacity.
 */


/* ============================================================================
 * 28. TOOLING CONTRACT
 * ========================================================================== */

/**
 * Formatters and language servers should preserve:
 *
 *     clause ordering
 *     binding patterns
 *     filters
 *     result expression
 *     map key/value distinction
 *     explicit `yield`
 *     explicit `parallel`
 *     explicit asynchronous intent
 *
 * A formatter may change whitespace and line breaks but MUST NOT change the
 * semantic order of comprehension clauses.
 */


/* ============================================================================
 * 29. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * Existing expression syntax remains valid.
 *
 * In particular, ordinary collection literals remain distinct from
 * comprehensions:
 *
 *     [a, b, c]
 *
 *     {a, b, c}
 *
 *     {key: value}
 *
 * A comprehension requires at least one generator clause.
 *
 * This prevents an ordinary collection literal from accidentally becoming a
 * comprehension.
 *
 * Existing loop syntax remains owned by statement grammar.
 *
 * A comprehension:
 *
 *     [f(x) for x in xs]
 *
 * is an expression.
 *
 * A statement loop:
 *
 *     for x in xs { ... }
 *
 * remains a statement.
 */


/* ============================================================================
 * 30. AMBIGUITY CONTRACT
 * ========================================================================== */

/**
 * The canonical expression grammar MUST place comprehension recognition before
 * ordinary bracket/brace collection literals where necessary.
 *
 * The important distinction is structural:
 *
 *     [expression ...]
 *
 * is a normal collection literal when it contains no generator clause.
 *
 *     [expression FOR ...]
 *
 * is a comprehension.
 *
 * Likewise:
 *
 *     {expression, expression}
 *
 * remains a set/object-style literal according to the collection grammar.
 *
 *     {expression FOR ...}
 *
 * is a set comprehension.
 *
 *     {expression COLON expression FOR ...}
 *
 * is a map comprehension.
 *
 * The semantic layer remains responsible for final collection-kind validation.
 */


/* ============================================================================
 * 31. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * Forbidden in this file:
 *
 *     MAX_ELEMENTS
 *     MAX_ITERATIONS
 *     MAX_CLAUSES
 *     MAX_BINDINGS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_QPU_SIZE
 *     MAX_MEMORY
 *     MAX_TENSOR_DIMENSION
 *
 * No fixed machine topology is represented here.
 *
 * No physical device identifier is represented here.
 *
 * No hardware address is represented here.
 */


/* ============================================================================
 * 32. SECURITY CONTRACT
 * ========================================================================== */

/**
 * Comprehension syntax does not grant capabilities.
 *
 * A program containing:
 *
 *     [network_call(x) for x in values]
 *
 * does not automatically receive network permission.
 *
 * Capability and security analysis occurs downstream.
 *
 * The same applies to:
 *
 *     hardware access
 *     filesystem access
 *     quantum backend access
 *     distributed execution
 *     cryptographic operations
 *     privileged runtime operations.
 */


/* ============================================================================
 * 33. ERROR RECOVERY
 * ========================================================================== */

/**
 * Parser implementations should provide useful diagnostics for malformed
 * comprehensions, including:
 *
 *     missing opening delimiter
 *     missing closing delimiter
 *     missing FOR
 *     missing IN
 *     missing generator pattern
 *     missing source expression
 *     missing result expression
 *     malformed map key/value pair
 *     malformed filter
 *     malformed yield form
 *
 * Error recovery belongs to the parser/frontend implementation and must not
 * silently reinterpret malformed source as a different valid computation.
 */


/* ============================================================================
 * 34. TEST CONTRACT
 * ========================================================================== */

/**
 * Required positive cases:
 *
 *     [x for x in xs]
 *     [x * 2 for x in xs]
 *     [x for x in xs if x > 0]
 *     [f(x) for x in xs for y in ys]
 *     [f(x, y) for x in xs if p(x) for y in ys if q(y)]
 *     {x for x in xs}
 *     {x * 2 for x in xs if p(x)}
 *     {key(x): value(x) for x in xs}
 *     (x for x in xs)
 *     [for x in xs yield f(x)]
 *     {for x in xs yield f(x)}
 *     {for x in xs yield key(x): value(x)}
 *
 * Required pattern cases:
 *
 *     [x for x in xs]
 *     [(x, y) for (x, y) in pairs]
 *     [x for [x, y] in pairs]
 *     [x for item in items]
 *     [x for _ in items]
 *
 * Required domain cases:
 *
 *     [measure(q) for q in qubits]
 *     [f(t) for t in tensors]
 *     [kernel(x) for x in data]
 *     [hardware_op(x) for x in resources]
 *
 * Required negative cases:
 *
 *     [x]
 *     [for]
 *     [x for]
 *     [x for x]
 *     [x for in xs]
 *     [x for x in]
 *     [key: value]
 *     {x for}
 *     (x for)
 *
 * Required boundary cases:
 *
 *     deeply nested comprehensions;
 *     many generator clauses;
 *     many filters;
 *     large source expressions;
 *     deeply nested patterns;
 *     large generated programs.
 *
 * Tests MUST verify that no grammar-level finite machine/resource limit exists.
 */


/* ============================================================================
 * 35. ROUND-TRIP CONTRACT
 * ========================================================================== */

/**
 * Where a canonical Zamani formatter/printer exists:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve:
 *
 *     - comprehension kind;
 *     - result expression;
 *     - key/value expressions;
 *     - generator order;
 *     - binding patterns;
 *     - filter order;
 *     - explicit yield;
 *     - execution-intent modifiers.
 *
 * Formatting must not silently transform:
 *
 *     [f(x) for x in xs]
 *
 * into a semantically different computation.
 */


/* ============================================================================
 * 36. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is COMPLETE only when:
 *
 * [ ] It is a parser grammar using ZamaniLexer.
 *
 * [ ] It defines no lexer rules.
 *
 * [ ] It defines no machine-specific limits.
 *
 * [ ] It defines no hardware-specific resources.
 *
 * [ ] It defines no quantum IR.
 *
 * [ ] It defines no classical IR.
 *
 * [ ] It does not duplicate canonical identifier syntax.
 *
 * [ ] It does not duplicate canonical type syntax.
 *
 * [ ] It does not duplicate canonical expression syntax.
 *
 * [ ] It does not duplicate the canonical pattern system.
 *
 * [ ] It supports list/array comprehensions.
 *
 * [ ] It supports set comprehensions.
 *
 * [ ] It supports map comprehensions.
 *
 * [ ] It supports generator comprehensions.
 *
 * [ ] It supports multiple generator clauses.
 *
 * [ ] It supports filters.
 *
 * [ ] It supports optional guarded `when` syntax.
 *
 * [ ] It supports explicit `yield`.
 *
 * [ ] It supports canonical destructuring patterns.
 *
 * [ ] It supports asynchronous iteration intent.
 *
 * [ ] It supports parallel intent without fixing worker/resource counts.
 *
 * [ ] It preserves clause ordering.
 *
 * [ ] It integrates with the canonical expression rule.
 *
 * [ ] It integrates with the canonical pattern rule.
 *
 * [ ] It remains domain-neutral.
 *
 * [ ] It preserves the quantum::ir boundary.
 *
 * [ ] It preserves POCO-REAF.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Round-trip tests exist where the formatter is available.
 *
 * [ ] Rust integration remains compatible with Rust 1.97 / 1.97.1.
 *
 * [ ] Rust integration contains no unsafe code.
 *
 * ============================================================================
 */