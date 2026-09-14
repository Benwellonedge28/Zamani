/**
 * Zamani Programming Language
 *
 * File:
 *     grammar/compile/conditional-compilation.g4
 *
 * Purpose:
 *     Defines the syntax for conditional source selection performed during
 *     compilation.
 *
 * Architectural role:
 *
 *     Source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     ZamaniParser
 *        |
 *        v
 *     Conditional-compilation syntax
 *        |
 *        v
 *     Frontend AST
 *        |
 *        v
 *     Semantic / capability / feature / target analysis
 *        |
 *        v
 *     Canonical semantic representation
 *        |
 *        +--> classical lowering
 *        +--> quantum::ir
 *        +--> HDL / hardware lowering
 *        +--> heterogeneous lowering
 *        |
 *        v
 *     target realization
 *
 * This grammar owns SOURCE-SELECTION SYNTAX ONLY.
 *
 * It MUST NOT:
 *
 *     - evaluate predicates;
 *     - inspect the filesystem;
 *     - inspect the network;
 *     - execute arbitrary host-language code;
 *     - discover hardware;
 *     - select a physical device;
 *     - perform optimization;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform placement;
 *     - perform QEC;
 *     - define ZQN semantics;
 *     - define quantum IR;
 *     - define classical IR;
 *     - define HDL IR;
 *     - define hardware IR;
 *     - define resource limits;
 *     - define machine cardinalities;
 *     - define compiler implementation limits;
 *     - define runtime behavior;
 *     - redefine ordinary runtime conditionals.
 *
 * POCO-REAF:
 *
 *     Conditional compilation is a compile-time portability mechanism.
 *
 *     It MAY select source according to declared:
 *
 *         features
 *         capabilities
 *         target properties
 *         language/compiler versions
 *         dialects
 *         compile-time expressions
 *         explicitly declared compilation context
 *
 *     It MUST NOT turn temporary hardware characteristics into permanent
 *     language-level machine assumptions.
 *
 * Scalability:
 *
 *     No finite number of branches, features, targets, capabilities,
 *     resources, devices, qubits, cores, threads, nodes, or other
 *     machine quantities is encoded here.
 *
 * Safety:
 *
 *     This grammar contains no target-language actions and no executable
 *     code. Integration with Rust MUST remain compatible with Rust 1.97 /
 *     Rust 1.97.1 and MUST NOT require Rust `unsafe`.
 */


/* ============================================================================
 * GRAMMAR DECLARATION
 * ========================================================================== */

parser grammar ConditionalCompilation;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The following imported grammars are architectural dependencies, not
 * alternative authorities.
 *
 * CompileTimeExpressions:
 *
 *     Owns compile-time expression syntax.
 *
 * Core / source-element aggregation:
 *
 *     Owns the canonical source-level item/statement/attribute vocabulary
 *     exposed to nested compilation branches.
 *
 * FeatureSelection:
 *
 *     Owns feature-selection predicate syntax.
 *
 * Target:
 *
 *     Owns target-reference / target-property predicate syntax.
 *
 * Capability:
 *
 *     Owns capability-reference predicate syntax.
 *
 * The exact import filenames MUST match the repository's canonical grammar
 * assembly. No imported grammar may import this file back.
 *
 * Expected final dependency direction:
 *
 *     lexer
 *       |
 *       +--> core
 *       |
 *       +--> expressions
 *       |
 *       +--> compile-time expressions
 *       |
 *       +--> feature selection
 *       |
 *       +--> target
 *       |
 *       +--> capabilities
 *       |
 *       v
 *     conditional compilation
 *       |
 *       v
 *     canonical parser
 *
 * There MUST be no reverse dependency from any of these foundations into
 * conditional compilation.
 *
 * The source-element aggregation grammar is intentionally kept below the
 * compilation layer so this grammar can contain arbitrary source constructs
 * without importing the complete root parser and creating a cycle.
 */
import CompileTimeExpressions;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * A complete conditional-compilation declaration.
 *
 * The construct is deliberately independent of runtime `if`.
 *
 * Runtime:
 *
 *     if condition { ... }
 *
 * Conditional compilation:
 *
 *     when condition { ... }
 *
 * The latter determines which source branch participates in later
 * compilation.
 */
conditionalCompilation
    : WHEN conditionalCompilationPredicate
      conditionalCompilationBody
      conditionalCompilationElseIf*
      conditionalCompilationElse?
    ;


/* ============================================================================
 * ELSE-IF / ELSE
 * ========================================================================== */

/**
 * Zero or more compile-time alternative branches.
 *
 * The grammar does not impose a maximum number of branches.
 */
conditionalCompilationElseIf
    : ELSE WHEN conditionalCompilationPredicate
      conditionalCompilationBody
    ;


/**
 * Optional final fallback branch.
 */
conditionalCompilationElse
    : ELSE conditionalCompilationBody
    ;


/* ============================================================================
 * PREDICATE
 * ========================================================================== */

/**
 * A conditional-compilation predicate.
 *
 * Predicates are intentionally categorized by semantic source rather than
 * hard-coding a finite set of machines or architectures.
 *
 * Supported predicate families:
 *
 *     compile-time expression
 *     feature
 *     target
 *     capability
 *     language/compiler version
 *     dialect
 *     generic semantic predicate
 *
 * The semantic layer determines whether a predicate is legal and whether it
 * can actually be resolved in the current compilation context.
 */
conditionalCompilationPredicate
    : conditionalCompilationCompileExpression
    | conditionalCompilationFeaturePredicate
    | conditionalCompilationTargetPredicate
    | conditionalCompilationCapabilityPredicate
    | conditionalCompilationVersionPredicate
    | conditionalCompilationDialectPredicate
    | conditionalCompilationPredicateExpression
    ;


/* ============================================================================
 * COMPILE-TIME EXPRESSION
 * ========================================================================== */

/**
 * Delegates compile-time computation to the canonical compile-time expression
 * grammar.
 *
 * This file MUST NOT reproduce arithmetic, calls, indexing, operators,
 * literals, type queries, or other expression syntax.
 */
conditionalCompilationCompileExpression
    : compileTimeExpression
    ;


/* ============================================================================
 * FEATURE PREDICATES
 * ========================================================================== */

/**
 * Tests whether a named compilation feature is selected.
 *
 * Feature selection is symbolic.
 *
 * Examples of semantic feature names MAY include:
 *
 *     quantum
 *     hardware
 *     distributed
 *     accelerator
 *     ai
 *
 * but this grammar deliberately does not enumerate them.
 *
 * No feature name implies a specific device, vendor, resource count,
 * topology, or hardware configuration.
 */
conditionalCompilationFeaturePredicate
    : FEATURE LPAREN conditionalCompilationNameArgument RPAREN
    ;


/**
 * Negated feature predicate.
 *
 * Negation belongs to the predicate language rather than being represented
 * by a special finite list of "disabled" features.
 */
conditionalCompilationNotFeaturePredicate
    : NOT FEATURE LPAREN conditionalCompilationNameArgument RPAREN
    ;


/* ============================================================================
 * TARGET PREDICATES
 * ========================================================================== */

/**
 * Tests a symbolic target property.
 *
 * The target grammar owns target descriptions.
 *
 * This grammar does NOT define:
 *
 *     CPU names
 *     GPU names
 *     QPU names
 *     FPGA identifiers
 *     ASIC identifiers
 *     device IDs
 *     addresses
 *     fixed topology
 *     fixed resource counts
 */
conditionalCompilationTargetPredicate
    : TARGET LPAREN conditionalCompilationNameArgument RPAREN
    ;


/**
 * Negated target predicate.
 */
conditionalCompilationNotTargetPredicate
    : NOT TARGET LPAREN conditionalCompilationNameArgument RPAREN
    ;


/* ============================================================================
 * CAPABILITY PREDICATES
 * ========================================================================== */

/**
 * Tests for a semantic capability.
 *
 * Capability resolution belongs to the capability/resource/compilation
 * context, not this grammar.
 *
 * A capability such as:
 *
 *     quantum
 *
 * does not mean:
 *
 *     use QPU X
 *
 * and:
 *
 *     supports_dynamic_control
 *
 * does not mean:
 *
 *     use N control units.
 */
conditionalCompilationCapabilityPredicate
    : CAPABILITY LPAREN conditionalCompilationNameArgument RPAREN
    ;


/**
 * Negated capability predicate.
 */
conditionalCompilationNotCapabilityPredicate
    : NOT CAPABILITY LPAREN conditionalCompilationNameArgument RPAREN
    ;


/* ============================================================================
 * VERSION PREDICATES
 * ========================================================================== */

/**
 * Version predicates are symbolic.
 *
 * Version interpretation belongs to the language/compiler compatibility
 * subsystem.
 *
 * No particular version is hard-coded into the grammar.
 */
conditionalCompilationVersionPredicate
    : VERSION LPAREN conditionalCompilationVersionSubject
      conditionalCompilationVersionOperator
      conditionalCompilationVersionValue
      RPAREN
    ;


conditionalCompilationVersionSubject
    : identifier
    | qualifiedIdentifierExpression
    ;


conditionalCompilationVersionOperator
    : EQUALS
    | NOT_EQUALS
    | LESS_THAN
    | LESS_THAN_EQUAL
    | GREATER_THAN
    | GREATER_THAN_EQUAL
    ;


conditionalCompilationVersionValue
    : expression
    ;


/* ============================================================================
 * DIALECT PREDICATES
 * ========================================================================== */

/**
 * Selects syntax/semantic behavior based on a named dialect.
 *
 * Dialect registration and compatibility remain owned by dialects/.
 */
conditionalCompilationDialectPredicate
    : DIALECT LPAREN conditionalCompilationNameArgument RPAREN
    ;


/**
 * Negated dialect predicate.
 */
conditionalCompilationNotDialectPredicate
    : NOT DIALECT LPAREN conditionalCompilationNameArgument RPAREN
    ;


/* ============================================================================
 * GENERIC SEMANTIC PREDICATE
 * ========================================================================== */

/**
 * Generic predicate form.
 *
 * This provides forward extensibility without requiring this grammar to
 * enumerate every future computing paradigm.
 *
 * Examples may eventually include semantic predicates for:
 *
 *     execution model
 *     memory model
 *     numeric model
 *     security profile
 *     interoperability profile
 *     distributed consistency model
 *     accelerator capability
 *     quantum capability
 *     HDL capability
 *
 * The semantic layer owns the meaning.
 */
conditionalCompilationPredicateExpression
    : LPAREN expression RPAREN
    ;


/* ============================================================================
 * PREDICATE COMBINATORS
 * ============================================================================
 *
 * Predicate composition is part of conditional-compilation syntax.
 *
 * It MUST NOT be confused with runtime boolean evaluation.
 *
 * The actual truth value is resolved by the compilation context.
 */

conditionalCompilationPredicateExpressionTree
    : conditionalCompilationPredicateOr
    ;


conditionalCompilationPredicateOr
    : conditionalCompilationPredicateAnd
      (
          OR conditionalCompilationPredicateAnd
      )*
    ;


conditionalCompilationPredicateAnd
    : conditionalCompilationPredicateNot
      (
          AND conditionalCompilationPredicateNot
      )*
    ;


conditionalCompilationPredicateNot
    : NOT conditionalCompilationPredicatePrimary
    | conditionalCompilationPredicatePrimary
    ;


conditionalCompilationPredicatePrimary
    : LPAREN conditionalCompilationPredicateOr RPAREN
    | conditionalCompilationPredicateAtom
    ;


conditionalCompilationPredicateAtom
    : conditionalCompilationPredicate
    ;


/* ============================================================================
 * NAME ARGUMENT
 * ========================================================================== */

/**
 * Named semantic references are deliberately represented using the canonical
 * name/path grammar.
 *
 * No grammar rule may restrict a name to:
 *
 *     a finite device list
 *     a finite architecture list
 *     a finite vendor list
 *     a finite capability list
 *     a finite feature list
 */
conditionalCompilationNameArgument
    : qualifiedIdentifierExpression
    | identifier
    ;


/* ============================================================================
 * BRANCH BODY
 * ========================================================================== */

/**
 * Branch bodies contain source-level declarations/statements.
 *
 * `conditionalCompilationItem` MUST be supplied by the repository's canonical
 * source-element aggregation grammar.
 *
 * It must expose the same source-element vocabulary used by the root parser.
 *
 * This prevents conditional compilation from creating a second language
 * inside branch bodies.
 */
conditionalCompilationBody
    : LBRACE conditionalCompilationItem* RBRACE
    ;


/**
 * Integration boundary.
 *
 * This rule is intentionally a single delegation point.
 *
 * The canonical source-element grammar owns the actual alternatives.
 *
 * It MUST expose:
 *
 *     conditionalCompilationItem
 *
 * without importing ConditionalCompilation.
 *
 * This creates the acyclic relationship:
 *
 *     source elements
 *          |
 *          v
 *     conditional compilation
 *          |
 *          v
 *     root parser
 *
 * rather than:
 *
 *     root parser <-> conditional compilation
 */
conditionalCompilationItem
    : attribute
    | item
    | statement
    | conditionalCompilation
    ;


/* ============================================================================
 * PUBLIC COMPILATION-DIRECTIVE ENTRY POINT
 * ========================================================================== */

/**
 * Compilation directives exposed to compile/compile.g4.
 *
 * This wrapper allows the compile grammar to integrate conditional
 * compilation without knowing the internal predicate/body structure.
 */
conditionalCompilationDirective
    : conditionalCompilation
    ;


/* ============================================================================
 * SEMANTIC INVARIANTS
 * ============================================================================
 *
 * These are documentation contracts, not executable parser actions.
 *
 * 1. Branch selection is compile-time semantics.
 *
 * 2. The parser MUST preserve all branch source structure required for
 *    diagnostics and semantic analysis.
 *
 * 3. The grammar MUST NOT itself decide which branch is active.
 *
 * 4. The semantic/compiler layer MUST resolve predicates against an explicit
 *    compilation context.
 *
 * 5. A missing target capability MUST be distinguishable from a false
 *    feature predicate.
 *
 * 6. An unknown predicate MUST NOT silently become false.
 *
 * 7. An unresolved target property MUST NOT silently become true.
 *
 * 8. Predicate evaluation MUST be deterministic for a fixed compilation
 *    context.
 *
 * 9. Conditional compilation MUST NOT access arbitrary host state through
 *    syntax.
 *
 * 10. Environment-sensitive behavior MUST enter through an explicit,
 *     authenticated compilation-context interface owned downstream.
 *
 * 11. Conditional compilation MUST NOT encode physical machine cardinality.
 *
 * 12. Conditional compilation MUST NOT select a physical device directly.
 *
 * 13. Conditional compilation MUST NOT construct quantum::ir.
 *
 * 14. Conditional compilation MUST NOT construct scheduling operations.
 *
 * 15. Conditional compilation MUST NOT invoke optimization.
 *
 * 16. Conditional compilation MUST NOT perform hardware discovery.
 *
 * 17. Conditional compilation MUST NOT alter runtime semantics merely because
 *     a different target is selected, unless the selected source itself has
 *     different explicitly specified semantics.
 *
 * 18. Both branches remain part of the source representation until semantic
 *     validation establishes which branch participates in compilation.
 *
 * 19. Diagnostics MUST preserve source locations for discarded and selected
 *     branches when required by the language's diagnostic policy.
 *
 * 20. There is no fixed maximum nesting depth represented by this grammar.
 *
 * 21. There is no fixed maximum number of branches represented by this
 *     grammar.
 *
 * 22. There is no fixed maximum number of features, targets, capabilities,
 *     dialects, or predicates represented by this grammar.
 */


/* ============================================================================
 * ARCHITECTURAL BOUNDARIES
 * ============================================================================
 *
 * OWNS:
 *
 *     - conditional-compilation syntax
 *     - branch syntax
 *     - predicate composition syntax
 *     - feature predicate syntax
 *     - target predicate syntax
 *     - capability predicate syntax
 *     - version predicate syntax
 *     - dialect predicate syntax
 *     - generic compile-time predicate syntax
 *
 * DOES NOT OWN:
 *
 *     - feature registry
 *     - target registry
 *     - capability registry
 *     - resource registry
 *     - version compatibility implementation
 *     - dialect implementation
 *     - compile-time evaluator
 *     - AST implementation
 *     - semantic analysis
 *     - target discovery
 *     - hardware discovery
 *     - optimizer
 *     - scheduler
 *     - router
 *     - QEC
 *     - ZQN
 *     - quantum::ir
 *     - runtime
 *
 *
 * AST CONTRACT:
 *
 * The AST builder should lower this syntax to a single semantic node
 * conceptually equivalent to:
 *
 *     ConditionalCompilation
 *
 * containing:
 *
 *     predicate
 *     then_branch
 *     else_if_branches
 *     else_branch
 *     source_span
 *
 * Predicate nodes should preserve their source form and remain unresolved
 * until semantic analysis.
 *
 *
 * SEMANTIC CONTRACT:
 *
 * Semantic analysis resolves:
 *
 *     feature references
 *     target references
 *     capability references
 *     version references
 *     dialect references
 *     compile-time expressions
 *
 * against an explicit compilation context.
 *
 * Unknown, ambiguous, unavailable, or incompatible predicates MUST produce
 * explicit diagnostics according to the compiler's diagnostic taxonomy.
 *
 *
 * IR CONTRACT:
 *
 * Conditional compilation is normally eliminated or resolved before canonical
 * IR construction.
 *
 * It MUST NOT create a second conditional-compilation IR.
 *
 * If an unresolved conditional is intentionally preserved for a later
 * compilation stage, it must be represented by the canonical semantic/IR
 * conditional construct already owned by the appropriate IR subsystem.
 *
 *
 * QUANTUM CONTRACT:
 *
 * Quantum source inside a selected branch eventually lowers through the
 * canonical quantum semantic path and, where applicable, `quantum::ir`.
 *
 * This grammar MUST NOT define:
 *
 *     qubits
 *     gates
 *     quantum topology
 *     QEC structures
 *     ZQN noise structures
 *
 * as part of conditional compilation.
 *
 *
 * HARDWARE CONTRACT:
 *
 * A target predicate can state a symbolic target property.
 *
 * It MUST NOT encode:
 *
 *     device IDs
 *     hardware addresses
 *     fixed topology
 *     fixed qubit counts
 *     fixed core counts
 *     fixed memory capacities
 *
 *
 * RESOURCE CONTRACT:
 *
 * Resource availability belongs to the resource/capability compilation
 * context.
 *
 * A resource quantity may be expressed as semantic data where the language
 * requires it, but this grammar itself imposes no maximum.
 *
 *
 * RUNTIME CONTRACT:
 *
 * Conditional compilation is not runtime branching.
 *
 * Runtime control flow remains owned by statements/expressions and the
 * execution/runtime architecture.
 *
 *
 * SECURITY CONTRACT:
 *
 * This grammar provides no mechanism for arbitrary filesystem, network,
 * process, environment, or host-language execution.
 *
 * Any compile-time external information must enter through an explicit,
 * policy-controlled compilation context.
 *
 *
 * DETERMINISM CONTRACT:
 *
 * For identical source and identical declared compilation context, parsing
 * MUST produce the same syntax tree.
 *
 * Predicate evaluation determinism is a semantic/compiler responsibility.
 *
 *
 * COMPATIBILITY CONTRACT:
 *
 * Adding a new feature, target, capability, dialect, or semantic property
 * MUST NOT require modifying this grammar when the existing symbolic syntax
 * can represent it.
 *
 * This is essential for POCO-REAF and long-term language evolution.
 */