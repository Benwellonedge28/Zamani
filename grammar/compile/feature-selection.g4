/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/feature-selection.g4
 *
 * Grammar:
 *     FeatureSelection
 *
 * Status:
 *     Production parser fragment.
 *
 * Purpose:
 *     Defines the SOURCE SYNTAX for selecting language/compiler features
 *     during compilation.
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
 *     FeatureSelection parser fragment
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Semantic analysis
 *          |
 *          +--> feature registry
 *          +--> capability analysis
 *          +--> requirement analysis
 *          +--> compilation context
 *          +--> dialect compatibility
 *          +--> target-independent specialization
 *          |
 *          v
 *     canonical semantic representation / IR
 *          |
 *          +--> classical lowering
 *          +--> quantum::ir
 *          +--> HDL/hardware lowering
 *          +--> heterogeneous lowering
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling / routing
 *          |
 *          v
 *     hardware / backend / runtime
 *
 * ============================================================================
 * FUNDAMENTAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - feature-selection syntax;
 *   - feature-selection predicates;
 *   - feature-selection boolean composition;
 *   - feature alternatives;
 *   - feature-selection branch syntax;
 *   - feature-selection fallback syntax;
 *   - optional feature-selection metadata;
 *   - syntactic feature references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - feature registry implementation;
 *   - feature availability;
 *   - feature capability discovery;
 *   - hardware discovery;
 *   - target selection;
 *   - target descriptions;
 *   - resource requirements;
 *   - resource constraints;
 *   - compile-time expression syntax;
 *   - ordinary expression syntax;
 *   - ordinary if statements;
 *   - macros;
 *   - compile-time functions;
 *   - specialization algorithms;
 *   - optimization;
 *   - scheduling;
 *   - routing;
 *   - placement;
 *   - QEC;
 *   - ZQN;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR;
 *   - runtime dispatch;
 *   - hardware implementation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Feature selection is a SOURCE-LEVEL PORTABILITY mechanism.
 *
 * A feature identifies a semantic/language/compiler capability.
 *
 * It MUST NOT inherently identify:
 *
 *   - a physical device;
 *   - a vendor;
 *   - a CPU model;
 *   - a GPU model;
 *   - a QPU;
 *   - an FPGA;
 *   - an ASIC;
 *   - a machine address;
 *   - a topology;
 *   - a fixed qubit count;
 *   - a fixed processor count;
 *   - a fixed memory size;
 *   - a fixed accelerator count.
 *
 * Therefore:
 *
 *     feature != device
 *     feature != target
 *     feature != resource
 *     feature != capability instance
 *
 * Semantic analysis may determine that a compilation environment satisfies
 * a feature through one or more capabilities, but that relationship does
 * not belong to this grammar.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO finite limits on:
 *
 *   - number of features;
 *   - number of alternatives;
 *   - number of branches;
 *   - number of nested selections;
 *   - number of feature predicates;
 *   - number of feature arguments;
 *   - number of source elements in a selected branch.
 *
 * Resource limits belong to compiler policy and execution infrastructure,
 * not syntax.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar:
 *
 *   - contains no embedded Rust;
 *   - contains no semantic actions;
 *   - contains no filesystem access;
 *   - contains no network access;
 *   - contains no host-language execution;
 *   - contains no unsafe code;
 *   - requires no Rust unsafe;
 *   - is compatible with Rust 1.97 / Rust 1.97.1 generated-parser consumers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For a fixed token stream, parsing must be deterministic.
 *
 * Semantic feature availability MUST NOT influence parsing.
 *
 * In particular:
 *
 *     unknown feature
 *
 * is a semantic error/unknown-resolution condition, not a syntax error.
 *
 * ============================================================================
 */

parser grammar FeatureSelection;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `featureSelection` is the ONLY public top-level rule owned by this file.
 *
 * The canonical compilation grammar may expose it as one of its compilation
 * constructs.
 *
 * Example conceptual source:
 *
 *     feature quantum {
 *         ...
 *     }
 *
 *     feature classical {
 *         ...
 *     }
 *
 * The exact feature identifier is intentionally open-ended.
 */
featureSelection
    : featureSelectionKeyword
      featureSelectionPredicate
      featureSelectionBody
      featureSelectionAlternative*
      featureSelectionElse?
    ;


/*
 * ============================================================================
 * 2. KEYWORD BOUNDARY
 * ============================================================================
 *
 * The current Zamani lexer deliberately avoids turning every extensible
 * domain concept into a reserved keyword.
 *
 * Consequently, this grammar does NOT create a new lexer authority.
 *
 * `feature` is represented as an identifier spelling at the lexical layer
 * until/unless the canonical lexer promotes it to a reserved token.
 *
 * Semantic/parser integration MUST validate the spelling:
 *
 *     feature
 *
 * without coupling the grammar to a finite feature vocabulary.
 *
 * If a future canonical lexer introduces FEATURE, this rule may be changed
 * during a versioned grammar migration:
 *
 *     featureSelectionKeyword
 *         : FEATURE
 *         ;
 *
 * The semantic AST contract MUST remain unchanged.
 */
featureSelectionKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 3. FEATURE PREDICATE
 * ============================================================================
 *
 * A feature-selection predicate identifies the semantic feature being tested.
 *
 * The grammar does not enumerate features such as:
 *
 *     quantum
 *     classical
 *     hdl
 *     gpu
 *     ai
 *     distributed
 *
 * Those are extensible semantic identifiers.
 *
 * A feature name therefore remains portable across future computing models.
 */
featureSelectionPredicate
    : featureReference
    | featureSelectionNot
    | featureSelectionAll
    | featureSelectionAny
    | featureSelectionExpression
    ;


/*
 * ============================================================================
 * 4. FEATURE REFERENCE
 * ============================================================================
 *
 * A feature reference is a symbolic semantic name.
 *
 * It does NOT identify a concrete implementation.
 */
featureReference
    : featureName
    ;


/*
 * ============================================================================
 * 5. FEATURE NAME
 * ============================================================================
 *
 * Feature names support qualified namespaces.
 *
 * Examples:
 *
 *     quantum
 *     classical
 *     hardware
 *     quantum.dynamic_circuits
 *     vendor.extension
 *     domain.subdomain.feature
 *
 * The number of components is not bounded by the grammar.
 */
featureName
    : IDENTIFIER
      (
          DOT IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 6. FEATURE NEGATION
 * ============================================================================
 *
 * Negation means:
 *
 *     this feature predicate must not be satisfied.
 *
 * It does not mean:
 *
 *     select another physical implementation.
 */
featureSelectionNot
    : NOT featureSelectionPredicate
    ;


/*
 * ============================================================================
 * 7. FEATURE ALL
 * ============================================================================
 *
 * Requires all contained feature predicates to participate in the selection.
 *
 * There is no fixed number of operands.
 */
featureSelectionAll
    : featureSelectionAllKeyword
      LPAREN
      featureSelectionPredicateList?
      RPAREN
    ;


featureSelectionAllKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 8. FEATURE ANY
 * ============================================================================
 *
 * Requires at least one compatible feature predicate.
 *
 * There is no fixed number of alternatives.
 */
featureSelectionAny
    : featureSelectionAnyKeyword
      LPAREN
      featureSelectionPredicateList?
      RPAREN
    ;


featureSelectionAnyKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 9. FEATURE PREDICATE LIST
 * ============================================================================
 *
 * Lists remain unbounded by grammar-level cardinality.
 */
featureSelectionPredicateList
    : featureSelectionPredicate
      (
          COMMA featureSelectionPredicate
      )*
    ;


/*
 * ============================================================================
 * 10. GENERAL FEATURE EXPRESSION
 * ============================================================================
 *
 * This provides future semantic extensibility without making this grammar
 * responsible for ordinary expression syntax.
 *
 * Parenthesized expressions are accepted as a syntactic composition point.
 *
 * Operators are represented using the canonical lexer vocabulary.
 */
featureSelectionExpression
    : LPAREN featureSelectionPredicate RPAREN
    ;


/*
 * ============================================================================
 * 11. FEATURE BRANCH
 * ============================================================================
 *
 * A selected branch contains canonical Zamani source elements.
 *
 * IMPORTANT:
 *
 * This file does not create a second statement/declaration grammar.
 *
 * The canonical parser assembly must bind:
 *
 *     featureSelectionItem
 *
 * to the repository's authoritative source-element aggregation rule.
 *
 * The rule below is therefore deliberately a composition boundary.
 */
featureSelectionBody
    : LBRACE featureSelectionItem* RBRACE
    ;


/*
 * ============================================================================
 * 12. ALTERNATIVE FEATURE BRANCH
 * ============================================================================
 *
 * Multiple alternatives are permitted without a hard-coded limit.
 *
 * Conceptually:
 *
 *     feature A { ... }
 *     else feature B { ... }
 *     else feature C { ... }
 *
 * The actual branch-selection semantics are downstream.
 */
featureSelectionAlternative
    : featureSelectionElseKeyword
      featureSelectionPredicate
      featureSelectionBody
    ;


/*
 * ============================================================================
 * 13. FALLBACK BRANCH
 * ============================================================================
 *
 * The fallback branch is selected only when no preceding feature branch
 * is semantically selected.
 */
featureSelectionElse
    : featureSelectionElseKeyword
      featureSelectionBody
    ;


featureSelectionElseKeyword
    : ELSE
    ;


/*
 * ============================================================================
 * 14. OPTIONAL FEATURE NEGATION FORM
 * ============================================================================
 *
 * This provides a compact form for explicitly testing absence.
 *
 * Example conceptual source:
 *
 *     feature not quantum { ... }
 *
 * This rule remains syntactic only.
 */
featureSelectionAbsent
    : featureSelectionKeyword
      NOT
      featureReference
      featureSelectionBody
    ;


/*
 * ============================================================================
 * 15. FEATURE SET FORM
 * ============================================================================
 *
 * Some compilation policies need to express a set of accepted features.
 *
 * This is still a semantic set, not a machine/resource set.
 */
featureSelectionSet
    : featureSelectionSetKeyword
      LBRACE
      featureNameList?
      RBRACE
    ;


featureSelectionSetKeyword
    : IDENTIFIER
    ;


featureNameList
    : featureName
      (
          COMMA featureName
      )*
    ;


/*
 * ============================================================================
 * 16. FEATURE ALIAS
 * ============================================================================
 *
 * Feature aliases are syntactic references only.
 *
 * Ownership of alias declarations remains elsewhere.
 *
 * This rule permits a selection expression to refer to a qualified semantic
 * feature name without introducing a feature registry into this grammar.
 */
featureSelectionAliasReference
    : featureName
    ;


/*
 * ============================================================================
 * 17. FEATURE VALUE
 * ============================================================================
 *
 * Feature systems may eventually carry structured compile-time metadata.
 *
 * The grammar deliberately does not define a second expression language.
 *
 * A feature value is therefore represented by an existing lexical literal
 * or a symbolic feature reference.
 */
featureSelectionValue
    : STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    | featureName
    ;


/*
 * ============================================================================
 * 18. FEATURE ARGUMENTS
 * ============================================================================
 *
 * Parameterized semantic features are permitted.
 *
 * Example conceptual forms:
 *
 *     feature quantum.version(...)
 *     feature numeric.precision(...)
 *
 * The meaning of arguments belongs to semantic feature resolution.
 */
featureSelectionArguments
    : LPAREN
      featureSelectionArgumentList?
      RPAREN
    ;


featureSelectionArgumentList
    : featureSelectionArgument
      (
          COMMA featureSelectionArgument
      )*
    ;


featureSelectionArgument
    : featureSelectionValue
    | featureName
    ;


/*
 * ============================================================================
 * 19. PARAMETERIZED FEATURE REFERENCE
 * ============================================================================
 *
 * A feature reference may carry semantic parameters.
 */
parameterizedFeatureReference
    : featureName
      featureSelectionArguments
    ;


/*
 * ============================================================================
 * 20. CANONICAL FEATURE ITEM INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally a single semantic/parser assembly boundary.
 *
 * The canonical parser must provide the authoritative source-item vocabulary.
 *
 * It must NOT be implemented here by copying the entire Zamani language.
 *
 * REQUIRED ASSEMBLY CONTRACT:
 *
 *     featureSelectionItem
 *         -> canonical source element
 *
 * The assembled parser may map this rule to its existing:
 *
 *     sourceElement
 *     declaration
 *     statement
 *     item
 *
 * production according to the final parser architecture.
 *
 * This file therefore owns the FEATURE-SELECTION CONTAINER, while the
 * repository's core parser owns the language elements contained within it.
 *
 * This prevents:
 *
 *     FeatureSelection <-> Core
 *
 * circular grammar ownership.
 *
 * The canonical assembly must bind this rule exactly once.
 */
featureSelectionItem
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 21. PUBLIC INTEGRATION ADAPTER
 * ============================================================================
 *
 * `featureSelectionDirective` is the stable integration name consumed by
 * compile-time compilation-control grammar.
 *
 * Keeping this adapter stable means the internal feature grammar can evolve
 * without forcing every downstream compilation grammar to change.
 */
featureSelectionDirective
    : featureSelection
    ;


/*
 * ============================================================================
 * 22. SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser produces syntax.
 *
 * Semantic analysis MUST subsequently resolve:
 *
 *     featureName
 *
 * against an explicit compilation context.
 *
 * The context may contain:
 *
 *     language features
 *     compiler features
 *     dialect features
 *     semantic capabilities
 *     target-independent capabilities
 *     compilation profile information
 *
 * The context MUST NOT be inferred from:
 *
 *     arbitrary host state
 *     filesystem state
 *     network state
 *     undocumented environment variables
 *     physical machine probing performed by the parser
 *
 * Feature resolution therefore belongs downstream.
 */


/*
 * ============================================================================
 * 23. UNKNOWN FEATURE CONTRACT
 * ============================================================================
 *
 * An unknown feature MUST NOT silently become false.
 *
 * Semantic analysis must distinguish at least:
 *
 *     KnownEnabled
 *     KnownDisabled
 *     Unknown
 *     Invalid
 *
 * The grammar itself does not encode those states.
 */


/*
 * ============================================================================
 * 24. FEATURE VS CAPABILITY
 * ============================================================================
 *
 * A feature is a semantic language/compilation selection concept.
 *
 * A capability is a property offered by a compilation/execution environment.
 *
 * Therefore:
 *
 *     feature quantum
 *
 * does not mean:
 *
 *     select QPU X
 *
 * and:
 *
 *     feature dynamic_quantum_control
 *
 * does not imply:
 *
 *     use hardware with N control channels.
 *
 * Capability matching is downstream.
 */


/*
 * ============================================================================
 * 25. FEATURE VS TARGET
 * ============================================================================
 *
 * This grammar MUST NOT become a target-selection grammar.
 *
 * A source feature may influence compilation strategy, but the actual target
 * remains owned by:
 *
 *     grammar/compile/target.g4
 *
 * and downstream target-resolution infrastructure.
 *
 * No rule in this file may encode:
 *
 *     CPU model lists
 *     GPU model lists
 *     QPU IDs
 *     FPGA IDs
 *     ASIC IDs
 *     vendor device names
 *     physical addresses
 */


/*
 * ============================================================================
 * 26. FEATURE VS RESOURCE
 * ============================================================================
 *
 * Features must never encode fixed resource counts.
 *
 * Forbidden semantic assumptions include:
 *
 *     feature quantum_32
 *     feature cpu_8
 *     feature gpu_4
 *     feature memory_64gb
 *
 * unless such names are explicitly user-defined semantic features.
 *
 * Even then, the grammar treats them as opaque symbolic names.
 *
 * It does not interpret the numeric suffix as a machine limit.
 */


/*
 * ============================================================================
 * 27. FEATURE VS OPTIMIZATION
 * ============================================================================
 *
 * Selecting a feature does not directly request an optimization pass.
 *
 * For example:
 *
 *     feature vectorization
 *
 * may permit a compiler strategy, but the optimization subsystem decides:
 *
 *     whether;
 *     how;
 *     when;
 *     and to what representation
 *
 * optimization occurs.
 *
 * This preserves the boundary:
 *
 *     grammar
 *         ->
 *     semantic intent
 *         ->
 *     optimization
 */


/*
 * ============================================================================
 * 28. FEATURE VS QUANTUM IR
 * ============================================================================
 *
 * This grammar MUST NEVER construct quantum::ir.
 *
 * A feature such as:
 *
 *     quantum
 *
 * only participates in semantic source selection.
 *
 * If the selected source contains quantum computation, that source is later
 * lowered through the normal frontend/semantic pipeline into the canonical
 * quantum::ir boundary.
 */


/*
 * ============================================================================
 * 29. FEATURE VS QEC / ZQN
 * ============================================================================
 *
 * Feature selection does not own:
 *
 *     error-correction algorithms
 *     error models
 *     noise models
 *     fault channels
 *     resilience decisions
 *
 * Those remain owned by:
 *
 *     QEC
 *     ZQN
 *     resilience
 *
 * respectively.
 *
 * A feature may select source intended for a QEC-capable or noise-aware
 * compilation environment, but this grammar does not implement those systems.
 */


/*
 * ============================================================================
 * 30. FEATURE SELECTION AND HARDWARE
 * ============================================================================
 *
 * Hardware capabilities are discovered and represented downstream.
 *
 * Feature selection may express:
 *
 *     feature quantum
 *
 * but must not itself discover whether a machine is quantum-capable.
 *
 * The compiler context performs that resolution through the hardware/resource
 * capability model.
 */


/*
 * ============================================================================
 * 31. FEATURE SELECTION AND POCO-REAF
 * ============================================================================
 *
 * A portable program may contain multiple feature-specific implementations:
 *
 *     feature classical {
 *         ...
 *     }
 *
 *     feature quantum {
 *         ...
 *     }
 *
 *     feature hardware {
 *         ...
 *     }
 *
 * The semantic compiler determines which source participates in a particular
 * compilation context.
 *
 * The source program remains the single portable specification.
 *
 * No source branch may assume that a feature corresponds to one permanent
 * machine architecture.
 */


/*
 * ============================================================================
 * 32. NESTING
 * ============================================================================
 *
 * Feature selections may be nested.
 *
 * The grammar imposes no finite nesting limit.
 *
 * Actual compiler recursion/evaluation limits are implementation-policy
 * concerns and MUST NOT be encoded as grammar constants.
 */
featureSelectionNested
    : featureSelection
    ;


/*
 * ============================================================================
 * 33. SOURCE-LOCATION PRESERVATION
 * ============================================================================
 *
 * The generated parser must preserve token/source locations through its normal
 * ANTLR parse tree.
 *
 * The AST builder must retain locations for:
 *
 *     feature selector
 *     feature name
 *     arguments
 *     branch body
 *     alternative branches
 *     fallback branch
 *
 * This is required for deterministic and actionable diagnostics.
 *
 * No grammar action code is used to implement location tracking.
 */


/*
 * ============================================================================
 * 34. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     malformed feature-selection syntax
 *
 * Semantic diagnostics:
 *
 *     unknown feature
 *     incompatible feature
 *     invalid feature argument
 *     contradictory feature selection
 *     unreachable branch
 *     duplicate feature alternative
 *     invalid feature combination
 *
 * Capability diagnostics:
 *
 *     required capability unavailable
 *
 * Target diagnostics:
 *
 *     selected implementation cannot satisfy the resulting semantic program
 *
 * These diagnostic classes MUST remain distinct.
 */


/*
 * ============================================================================
 * 35. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source characters
 *     lexer configuration
 *     grammar version
 *
 * It must NOT depend on:
 *
 *     hardware availability
 *     network availability
 *     current machine
 *     filesystem contents
 *     runtime state
 *     compiler cache state
 *
 * Feature resolution happens after parsing.
 */


/*
 * ============================================================================
 * 36. SECURITY CONTRACT
 * ============================================================================
 *
 * Feature-selection syntax cannot:
 *
 *     execute arbitrary code;
 *     access environment variables directly;
 *     read arbitrary files;
 *     open sockets;
 *     execute commands;
 *     inspect credentials;
 *     access secrets.
 *
 * If a compiler context exposes environment-derived information, that
 * information must arrive through an explicit, policy-controlled semantic
 * interface.
 */


/*
 * ============================================================================
 * 37. COMPILATION CONTRACT
 * ============================================================================
 *
 * Feature selection happens before target-specific lowering.
 *
 * Conceptually:
 *
 *     source
 *       |
 *       v
 *     parse
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     feature resolution
 *       |
 *       v
 *     semantic validation
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> optimization
 *       +--> routing
 *       +--> scheduling
 *       +--> hardware lowering
 *
 * This grammar MUST NOT bypass the canonical semantic pipeline.
 */


/*
 * ============================================================================
 * 38. INTEGRATION WITH compile-time.g4
 * ============================================================================
 *
 * `grammar/compile/compile-time.g4` previously described feature selection
 * directly.
 *
 * That duplication must be removed from the ownership model.
 *
 * The final assembly must instead use:
 *
 *     compileTimeFeatureSelection
 *         : featureSelectionDirective
 *         ;
 *
 * or an equivalent one-way adapter.
 *
 * compile-time.g4 MUST NOT redefine:
 *
 *     featureSelection
 *     featureReference
 *     featureName
 *     featureSelectionPredicate
 *
 * This establishes:
 *
 *     FeatureSelection
 *          |
 *          v
 *     CompileTime
 *          |
 *          v
 *     canonical parser
 *
 * and prevents competing definitions.
 */


/*
 * ============================================================================
 * 39. INTEGRATION WITH conditional-compilation.g4
 * ============================================================================
 *
 * `grammar/compile/conditional-compilation.g4` already identifies feature
 * predicates as a semantic dependency.
 *
 * Therefore conditional compilation must consume this grammar's public
 * feature-predicate boundary rather than reproduce feature syntax.
 *
 * Required integration direction:
 *
 *     FeatureSelection
 *          |
 *          +--> ConditionalCompilation
 *
 * NOT:
 *
 *     FeatureSelection <--> ConditionalCompilation
 *
 * This prevents circular grammar ownership.
 */


/*
 * ============================================================================
 * 40. INTEGRATION WITH target.g4
 * ============================================================================
 *
 * Target syntax remains external.
 *
 * Feature selection may coexist with target predicates, but this grammar must
 * not import or duplicate target syntax.
 *
 * The semantic layer may evaluate:
 *
 *     feature requirement
 *          +
 *     target capability
 *
 * as a compilation-context decision.
 */


/*
 * ============================================================================
 * 41. INTEGRATION WITH resources/
 * ============================================================================
 *
 * Resource requirements remain external.
 *
 * A feature can imply semantic requirements only through an explicit semantic
 * registry.
 *
 * This grammar does not translate:
 *
 *     feature quantum
 *
 * into:
 *
 *     N qubits
 *
 * or any other resource quantity.
 */


/*
 * ============================================================================
 * 42. INTEGRATION WITH dialects/
 * ============================================================================
 *
 * Dialects may introduce feature names.
 *
 * FeatureSelection treats them as symbolic names.
 *
 * Dialect registration and compatibility remain owned by:
 *
 *     grammar/dialects/
 *
 * This provides forward compatibility for future computing domains.
 */


/*
 * ============================================================================
 * 43. INTEGRATION WITH INTEROPERABILITY
 * ============================================================================
 *
 * Feature selection may select source that later interoperates with:
 *
 *     C
 *     C++
 *     Python
 *     OpenQASM
 *     Verilog
 *     SystemVerilog
 *     other supported interfaces
 *
 * This grammar does not own those foreign-language grammars or ABI semantics.
 */


/*
 * ============================================================================
 * 44. AST CONTRACT
 * ============================================================================
 *
 * The AST builder should conceptually produce:
 *
 *     FeatureSelection {
 *         predicate,
 *         then_branch,
 *         alternatives,
 *         fallback
 *     }
 *
 * where:
 *
 *     predicate
 *
 * represents source-level semantic intent;
 *
 *     then_branch
 *
 * contains canonical source elements;
 *
 *     alternatives
 *
 * contain ordered alternative predicates and source elements;
 *
 *     fallback
 *
 * is optional.
 *
 * The AST MUST preserve source locations.
 *
 * The AST MUST NOT contain:
 *
 *     physical device IDs
 *     hardware handles
 *     scheduler operations
 *     quantum IR instructions
 *     runtime handles
 *     optimizer state
 */


/*
 * ============================================================================
 * 45. SEMANTIC AST INVARIANTS
 * ============================================================================
 *
 * Semantic analysis must enforce:
 *
 *   1. Feature names resolve against the compilation context.
 *
 *   2. Unknown features are not silently disabled.
 *
 *   3. Feature predicates are evaluated deterministically.
 *
 *   4. Feature resolution is separate from target resolution.
 *
 *   5. Feature resolution is separate from resource allocation.
 *
 *   6. Feature resolution is separate from hardware discovery.
 *
 *   7. Feature selection does not mutate canonical IR directly.
 *
 *   8. Selected source is validated normally after selection.
 *
 *   9. Discarded source remains available for diagnostics according to
 *      compiler policy.
 *
 *  10. Feature-selection decisions are recorded in compilation provenance.
 *
 *  11. Different compilation contexts may legitimately select different
 *      branches without changing the source program's syntax.
 *
 *  12. Semantic incompatibility is diagnosed before target-specific lowering.
 */


/*
 * ============================================================================
 * 46. PROVENANCE CONTRACT
 * ============================================================================
 *
 * A production compiler should record:
 *
 *     feature-selection source location
 *     feature predicate
 *     compilation-context identity
 *     feature-resolution result
 *     selected branch
 *     compiler/language version
 *
 * This information belongs to compiler provenance rather than this grammar.
 *
 * The grammar must nevertheless preserve the syntax required to create that
 * provenance record.
 */


/*
 * ============================================================================
 * 47. REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * For:
 *
 *     same source
 *     same language version
 *     same compilation context
 *
 * feature selection must resolve identically.
 *
 * Hidden host state MUST NOT affect the result.
 */


/*
 * ============================================================================
 * 48. CACHE / COMPILE-ONCE CONTRACT
 * ============================================================================
 *
 * Feature-selection decisions form part of the semantic compilation context.
 *
 * Therefore a cached compiled artifact must not be reused solely because the
 * source bytes match.
 *
 * The compilation identity must account for all semantic feature-selection
 * inputs relevant to the resulting artifact.
 *
 * Cache-key implementation belongs to compiler infrastructure.
 */


/*
 * ============================================================================
 * 49. FORWARD COMPATIBILITY
 * ============================================================================
 *
 * New features do not require grammar changes when they are represented by
 * symbolic feature names.
 *
 * Examples:
 *
 *     quantum
 *     quantum.dynamic
 *     quantum.error_correction
 *     classical.simd
 *     hardware.reconfigurable
 *     accelerator.tensor
 *     distributed.consensus
 *     future.computation_model
 *
 * are syntactically equivalent categories at this layer.
 *
 * Their meanings belong to semantic registries and specifications.
 */


/*
 * ============================================================================
 * 50. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain none of the following:
 *
 *     MAX_FEATURES
 *     MAX_BRANCHES
 *     MAX_FEATURE_ARGUMENTS
 *     MAX_NESTING
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *
 * No physical machine quantity belongs in this grammar.
 */


/*
 * ============================================================================
 * 51. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 * [ ] It has exactly one authoritative FeatureSelection grammar.
 *
 * [ ] It does not redefine ordinary expressions.
 *
 * [ ] It does not redefine identifiers owned by the core grammar.
 *
 * [ ] It does not define target syntax.
 *
 * [ ] It does not define capability implementation.
 *
 * [ ] It does not define resource implementation.
 *
 * [ ] It does not define conditional-compilation implementation.
 *
 * [ ] It does not construct AST implementation objects directly.
 *
 * [ ] It does not construct IR.
 *
 * [ ] It does not construct quantum::ir.
 *
 * [ ] It contains no embedded Rust.
 *
 * [ ] It contains no unsafe code.
 *
 * [ ] It contains no filesystem/network execution.
 *
 * [ ] It has no fixed machine/resource limits.
 *
 * [ ] Unknown features remain distinguishable from false features.
 *
 * [ ] Feature selection is deterministic for a fixed compilation context.
 *
 * [ ] Nested feature selection is supported.
 *
 * [ ] Multiple alternatives are supported without fixed cardinality.
 *
 * [ ] Fallback selection is supported.
 *
 * [ ] Feature names are extensible.
 *
 * [ ] Feature names can be qualified.
 *
 * [ ] Feature arguments can be represented without creating a second
 *     expression language.
 *
 * [ ] Source locations are preserved through the parser tree.
 *
 * [ ] compile-time.g4 consumes this grammar instead of duplicating it.
 *
 * [ ] conditional-compilation.g4 consumes this grammar instead of duplicating
 *     feature predicate syntax.
 *
 * [ ] target.g4 remains the owner of target syntax.
 *
 * [ ] resources/ remains the owner of resource syntax.
 *
 * [ ] dialects/ remains the owner of dialect registration.
 *
 * [ ] semantic analysis owns feature resolution.
 *
 * [ ] compiler infrastructure owns feature-context construction.
 *
 * [ ] backend/runtime layers never need to depend directly on this grammar.
 */


/*
 * ============================================================================
 * 52. TEST CONTRACT
 * ============================================================================
 *
 * The repository must provide tests covering at minimum:
 *
 * POSITIVE
 * --------
 *
 *     feature quantum { ... }
 *     feature classical { ... }
 *     feature hardware { ... }
 *     feature quantum.dynamic { ... }
 *     feature vendor.extension { ... }
 *
 *     feature quantum { ... }
 *     else feature classical { ... }
 *     else { ... }
 *
 *     feature quantum { ... }
 *     else feature hardware { ... }
 *     else feature classical { ... }
 *     else { ... }
 *
 *     nested feature selections
 *
 *     qualified feature names
 *
 *     parameterized feature references
 *
 *     feature conjunction/disjunction forms
 *
 * NEGATIVE
 * --------
 *
 *     missing feature predicate
 *     missing branch body
 *     malformed qualified name
 *     malformed argument list
 *     malformed alternative
 *     malformed fallback
 *     missing closing brace
 *     missing separator
 *
 * SEMANTIC-BOUNDARY
 * -----------------
 *
 *     unknown feature
 *     disabled feature
 *     incompatible feature
 *     conflicting feature predicates
 *     unsupported feature argument
 *
 * SCALABILITY
 * -----------
 *
 *     many feature alternatives
 *     deeply nested feature selections
 *     long qualified feature names
 *     large feature predicate lists
 *
 *     No test may introduce artificial finite grammar limits.
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     classical + feature selection
 *     quantum + feature selection
 *     hybrid + feature selection
 *     HDL + feature selection
 *     hardware + feature selection
 *     distributed + feature selection
 *     AI + feature selection
 *
 * The tests verify parsing and semantic-boundary preservation, not hardware
 * availability.
 */


/*
 * ============================================================================
 * 53. INTEGRATION TEST CONTRACT
 * ============================================================================
 *
 * The final parser integration must verify:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     FeatureSelection
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic feature resolver
 *
 * and subsequently:
 *
 *     selected source
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical IR
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL
 *          +--> heterogeneous
 *
 * FeatureSelection MUST NOT become a separate compilation pipeline.
 */


/*
 * ============================================================================
 * 54. NON-DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally has NO direct dependency on:
 *
 *     src/quantum/ir/
 *     src/quantum/qec/
 *     src/quantum/zqn/
 *     src/quantum/scheduling/
 *     src/quantum/optimization/
 *     src/quantum/hardware/
 *     src/quantum/resilience/
 *
 * Those subsystems consume semantic/IR results downstream.
 *
 * This prevents the grammar from becoming coupled to any current quantum
 * implementation and preserves Zamani's universal-computation architecture.
 */


/*
 * ============================================================================
 * 55. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file implements:
 *
 *     FEATURE SELECTION SYNTAX
 *
 * It does NOT implement:
 *
 *     FEATURE SELECTION POLICY
 *     FEATURE DISCOVERY
 *     FEATURE RESOLUTION
 *     TARGET SELECTION
 *     HARDWARE SELECTION
 *     RESOURCE ALLOCATION
 *     COMPILATION
 *     OPTIMIZATION
 *     SCHEDULING
 *     EXECUTION
 *
 * Therefore:
 *
 *     Zamani source
 *         describes intent
 *              |
 *              v
 *         feature selection
 *              |
 *              v
 *         semantic resolution
 *              |
 *              v
 *         canonical IR
 *              |
 *              v
 *         target realization
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and:
 *
 *     Zamani — From Atom to Everywhere.
 *
 * ============================================================================
 */