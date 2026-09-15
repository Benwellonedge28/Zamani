/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/expansion.g4
 *
 * Role:
 *     Canonical parser boundary for source-level macro-expansion metadata and
 *     expansion directives.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded Rust actions.
 *     No predicates.
 *     No filesystem access.
 *     No network access.
 *     No process execution.
 *     No arbitrary compile-time host execution.
 *     No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This file defines SYNTAX, not macro expansion.
 *
 * The architecture is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> macro resolution
 *       |
 *       +--> hygiene/provenance
 *       |
 *       +--> expansion planning
 *       |
 *       +--> controlled expansion
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed/data/effect/resource representations
 *       |
 *       v
 *     optimization / routing / scheduling / target lowering
 *       |
 *       v
 *     runtime
 *
 * This file MUST remain above those semantic and execution layers.
 *
 * ============================================================================
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * Macro declaration syntax is owned by:
 *
 *     grammar/macros/declarations.g4
 *
 * Macro invocation syntax is owned by:
 *
 *     grammar/macros/invocations.g4
 *
 * Macro hygiene syntax is owned by:
 *
 *     grammar/macros/hygiene.g4
 *
 * This file owns only expansion-specific syntax.
 *
 * It MUST NOT redefine:
 *
 *     macroDeclaration
 *     macroParameter
 *     macroInvocation
 *     macroPath
 *     argumentList
 *     annotation
 *     identifier
 *     qualifiedName
 *     expression
 *     blockExpression
 *
 * ============================================================================
 * CURRENT REPOSITORY COMPATIBILITY
 * ============================================================================
 *
 * The repository's macro grammar deliberately does not currently establish
 * canonical lexer tokens for quotation/splicing/interpolation.
 *
 * Therefore this file does NOT invent:
 *
 *     QUOTE
 *     UNQUOTE
 *     SPLICE
 *     QUASIQUOTE
 *     EXPAND
 *     EVAL
 *     TOKEN
 *     SYNTAX
 *
 * lexer tokens.
 *
 * Nor does it interpret ordinary identifiers named:
 *
 *     quote
 *     splice
 *     expand
 *     eval
 *
 * as keywords.
 *
 * This is intentional.
 *
 * A future first-class quotation/splicing language feature must first establish
 * its canonical lexical, syntactic, AST, semantic, security, and compatibility
 * contracts.
 *
 * ============================================================================
 * WHAT "EXPANSION" MEANS HERE
 * ============================================================================
 *
 * There are three distinct concepts:
 *
 *     1. Macro invocation
 *        Requests a macro by name.
 *
 *     2. Expansion
 *        Compiler transformation of a resolved macro invocation into source
 *        structure.
 *
 *     3. Evaluation
 *        Execution of a computation.
 *
 * These MUST NOT be conflated.
 *
 * `foo!(x)` is a macro invocation.
 *
 * The parser does not execute `foo`.
 *
 * The macro engine may subsequently expand it.
 *
 * The resulting program is then semantically analyzed and eventually compiled
 * and/or executed according to the normal Zamani pipeline.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - expansion-specific parser boundaries;
 *     - reusable expansion metadata syntax;
 *     - the parser-level connection between macro invocation and expansion
 *       metadata, where standardized;
 *     - source-level expansion descriptors that do not themselves execute
 *       anything.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - macro declarations;
 *     - macro invocations;
 *     - macro names;
 *     - identifiers;
 *     - paths;
 *     - expressions;
 *     - blocks;
 *     - annotations;
 *     - hygiene semantics;
 *     - name resolution;
 *     - binding resolution;
 *     - expansion algorithms;
 *     - expansion ordering;
 *     - expansion caching;
 *     - expansion memoization;
 *     - token generation;
 *     - AST mutation;
 *     - source-file generation;
 *     - arbitrary compile-time execution;
 *     - filesystem access;
 *     - network access;
 *     - process execution;
 *     - compiler plugins;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - backend selection;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - canonical IR;
 *     - quantum::ir.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Macro expansion is source-language infrastructure.
 *
 * A macro may generate:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware descriptions
 *     distributed computation
 *     AI/ML structures
 *     data pipelines
 *     networking structures
 *     security constructs
 *     future dialect constructs
 *
 * This file MUST NOT contain domain-specific expansion rules.
 *
 * The expanded syntax is interpreted by the appropriate semantic layer after
 * expansion.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Macro expansion must preserve the semantic portability of the program.
 *
 * Expansion must not implicitly select:
 *
 *     CPU
 *     core
 *     thread
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     quantum simulator
 *     device
 *     node
 *     cluster
 *     topology
 *     backend
 *     scheduler
 *     routing strategy
 *
 * A macro may express semantic requirements through the canonical language
 * constructs, but expansion itself must not secretly turn portable source
 * semantics into target-specific implementation choices.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite machine-dependent limits are encoded here.
 *
 * There is no:
 *
 *     MAX_EXPANSIONS
 *     MAX_MACROS
 *     MAX_EXPANSION_DEPTH
 *     MAX_GENERATED_NODES
 *     MAX_TOKENS
 *     MAX_QUANTUM_RESOURCES
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_NODES
 *
 * Compiler resource policies may impose configurable budgets for:
 *
 *     source size
 *     token count
 *     AST size
 *     expansion steps
 *     expansion depth
 *     generated output size
 *     memory consumption
 *     compilation time
 *
 * Those are implementation/resource policies, not grammar semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Expansion must subsequently be deterministic with respect to the complete
 * declared compilation context, including:
 *
 *     source
 *     macro definitions
 *     arguments
 *     language version
 *     semantic environment
 *     explicit compiler policy
 *     explicitly selected resource/target context
 *
 * Expansion must not depend on:
 *
 *     timestamps
 *     process IDs
 *     memory addresses
 *     random values
 *     machine-local incidental state
 *
 * as semantic inputs.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Expansion is a source transformation and therefore requires complete
 * provenance.
 *
 * The AST/expansion subsystem must be able to distinguish:
 *
 *     original source
 *     macro definition source
 *     invocation source
 *     generated source
 *     nested expansion source
 *
 * The parser itself only provides source structure and source spans.
 *
 * ============================================================================
 * HYGIENE
 * ============================================================================
 *
 * Hygiene is owned by:
 *
 *     grammar/macros/hygiene.g4
 *
 * and the downstream semantic hygiene engine.
 *
 * Expansion MUST preserve the information required by hygiene analysis.
 *
 * Expansion must not:
 *
 *     rename identifiers textually;
 *     invent capture semantics;
 *     bypass lexical scopes;
 *     merge unrelated bindings;
 *     discard provenance.
 *
 * Fresh binding identities and capture avoidance belong to the semantic
 * expansion implementation.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * A source-level expansion construct MUST NOT automatically grant permission
 * to:
 *
 *     read files;
 *     write files;
 *     read environment variables;
 *     access networks;
 *     execute programs;
 *     invoke arbitrary host APIs;
 *     download dependencies;
 *     modify the compiler;
 *     access secrets;
 *     access hardware;
 *     bypass security policy.
 *
 * Compile-time computation, if Zamani eventually supports it, must be governed
 * by an explicit capability/effect/security model outside this grammar.
 *
 * ============================================================================
 * 1. EXPANSION METADATA
 * ============================================================================
 *
 * Expansion metadata is represented using the canonical annotation system.
 *
 * `macroExpansionAnnotation` is therefore an integration alias, not a second
 * annotation grammar.
 *
 * The semantic annotation registry determines which annotations actually have
 * expansion meaning.
 *
 * This allows new expansion policies to be introduced without changing the
 * parser for every new semantic annotation.
 */

parser grammar expansion;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 2. EXPANSION ANNOTATION
 * ============================================================================
 *
 * Reuse the canonical annotation rule.
 *
 * Do NOT define:
 *
 *     expansionAnnotationName
 *     expansionAttribute
 *     expansionArguments
 *
 * here.
 */

macroExpansionAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * 3. EXPANSION ANNOTATION LIST
 * ============================================================================
 *
 * A standardized expansion-bearing construct may carry multiple annotations.
 *
 * There is no grammar-level finite maximum.
 */

macroExpansionAnnotations
    : macroExpansionAnnotation+
    ;


/*
 * ============================================================================
 * 4. OPTIONAL EXPANSION ANNOTATIONS
 * ============================================================================
 */

optionalMacroExpansionAnnotations
    : macroExpansionAnnotation*
    ;


/*
 * ============================================================================
 * 5. EXPANSION PREFIX
 * ============================================================================
 *
 * This is the reusable attachment boundary for macro syntax that explicitly
 * permits expansion metadata.
 */

macroExpansionPrefix
    : macroExpansionAnnotations
    ;


optionalMacroExpansionPrefix
    : optionalMacroExpansionAnnotations
    ;


/*
 * ============================================================================
 * 6. INVOCATION EXPANSION BOUNDARY
 * ============================================================================
 *
 * `macroInvocation` remains owned by invocations.g4.
 *
 * This rule does not redefine it.
 *
 * It provides a semantic/parser integration point for a future standardized
 * invocation-side expansion annotation.
 *
 * The consuming grammar must decide the legal source position.
 */

macroInvocationExpansion
    : macroExpansionAnnotations
    ;


/*
 * ============================================================================
 * 7. DECLARATION EXPANSION BOUNDARY
 * ============================================================================
 *
 * Macro declarations remain owned by declarations.g4.
 *
 * This boundary permits declaration-level expansion metadata without creating
 * a second declaration grammar.
 */

macroDeclarationExpansion
    : macroExpansionAnnotations
    ;


/*
 * ============================================================================
 * 8. EXPANSION-DIRECTIVE BOUNDARY
 * ============================================================================
 *
 * This is intentionally an annotation-based boundary rather than a speculative
 * keyword grammar.
 *
 * The semantic registry may eventually define expansion directives such as
 * semantic equivalents of:
 *
 *     expand
 *     no_expand
 *     once
 *     recursive
 *     generated
 *     transparent
 *     deferred
 *
 * but those names are NOT language keywords here.
 *
 * Their exact legality and meaning are semantic policy.
 */

macroExpansionDirective
    : macroExpansionAnnotation
    ;


/*
 * ============================================================================
 * 9. EXPANSION-DIRECTIVE SET
 * ============================================================================
 */

macroExpansionDirectives
    : macroExpansionDirective+
    ;


optionalMacroExpansionDirectives
    : macroExpansionDirective*
    ;


/*
 * ============================================================================
 * 10. EXPANSION CONTEXT BOUNDARY
 * ============================================================================
 *
 * Expansion context is a semantic object.
 *
 * This grammar does not attempt to encode the compiler's internal context
 * structure.
 *
 * A macro expansion context may eventually include:
 *
 *     language version
 *     macro definition identity
 *     invocation identity
 *     lexical scope
 *     hygiene context
 *     provenance
 *     capability context
 *     effect context
 *     resource policy
 *     deterministic compilation context
 *
 * Those are NOT grammar-level machine properties.
 *
 * This rule exists solely as a stable named boundary for consumers that need
 * to attach expansion metadata.
 */

macroExpansionContext
    : macroExpansionAnnotations
    ;


/*
 * ============================================================================
 * 11. EXPANSION PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Provenance is represented by the AST/compiler infrastructure.
 *
 * This rule allows expansion-related metadata to remain attached to the
 * source construct without creating a second provenance grammar.
 */

macroExpansionProvenance
    : macroExpansionAnnotations
    ;


/*
 * ============================================================================
 * 12. CONTROLLED EXPANSION BOUNDARY
 * ============================================================================
 *
 * A macro invocation itself is NOT an expansion instruction in the execution
 * sense.
 *
 * The semantic pipeline may classify it as:
 *
 *     unresolved
 *     resolved
 *     eligible-for-expansion
 *     deferred
 *     expanded
 *     rejected
 *
 * Those states belong to the macro semantic model.
 *
 * This grammar does not encode those states as parser keywords.
 */

macroExpansionBoundary
    : macroExpansionAnnotation
    ;


/*
 * ============================================================================
 * 13. NO QUOTE / SPLICE IMPLEMENTATION
 * ============================================================================
 *
 * Quotation and splicing are intentionally NOT implemented here.
 *
 * In particular, do not use ordinary identifiers to fake:
 *
 *     quote(...)
 *     splice(...)
 *     unquote(...)
 *     quasiquote(...)
 *
 * as parser keywords.
 *
 * Doing so would make ordinary identifiers context-sensitive and would create
 * a language feature without a canonical lexer/AST/semantic contract.
 *
 * A future quotation system should establish:
 *
 *     lexer tokens
 *     grammar rules
 *     AST representation
 *     source/provenance model
 *     hygiene model
 *     type model
 *     capability/effect model
 *     expansion security model
 *     compatibility policy
 *     tests
 *
 * before becoming canonical syntax.
 */


/*
 * ============================================================================
 * 14. NO ARBITRARY COMPILE-TIME EXECUTION
 * ============================================================================
 *
 * This grammar does not define a mechanism by which source text can execute
 * arbitrary Rust or host-language code during parsing.
 *
 * In particular, the grammar contains no:
 *
 *     actions
 *     semantic predicates
 *     host-language callbacks
 *     embedded Rust
 *
 * Macro expansion is a controlled compiler transformation.
 *
 * If compile-time functions are supported by the language, they must operate
 * through the separately specified compile-time execution model.
 */


/*
 * ============================================================================
 * 15. NO SOURCE-FILE GENERATION SEMANTICS
 * ============================================================================
 *
 * Expansion may conceptually produce AST/source structures, but this parser
 * grammar does not define filesystem-backed source generation.
 *
 * The compiler may represent generated syntax in memory.
 *
 * Provenance must associate generated nodes with their origin.
 *
 * Whether generated source can be materialized is a separate tooling/compiler
 * concern.
 */


/*
 * ============================================================================
 * 16. NO STRING-BASED MACRO EXPANSION
 * ============================================================================
 *
 * The macro engine must operate on structured source/AST/token representations
 * according to its semantic contract.
 *
 * This grammar does not define:
 *
 *     textual replacement
 *     regex replacement
 *     string interpolation as macro expansion
 *     raw source concatenation
 *
 * as the semantic macro model.
 *
 * This prevents malformed syntax, accidental capture, source-provenance loss,
 * and domain-specific parsing inconsistencies.
 */


/*
 * ============================================================================
 * 17. EXPANSION INPUT CONTRACT
 * ============================================================================
 *
 * The downstream expansion engine receives, conceptually:
 *
 *     resolved macro definition
 *     invocation arguments
 *     lexical environment
 *     hygiene context
 *     source provenance
 *     language version
 *     semantic/compiler policy
 *
 * It MUST NOT require:
 *
 *     hardware topology
 *     number of CPUs
 *     number of GPUs
 *     number of qubits
 *     device identifier
 *
 * unless those are independently represented by a semantic resource/target
 * context.
 */


/*
 * ============================================================================
 * 18. EXPANSION OUTPUT CONTRACT
 * ============================================================================
 *
 * Expansion produces source-semantic structure suitable for ordinary semantic
 * analysis.
 *
 * It does not directly produce:
 *
 *     machine instructions
 *     quantum hardware schedules
 *     physical qubit mappings
 *     HDL netlists
 *     device commands
 *     runtime execution requests
 *
 * unless later compiler stages explicitly lower the resulting semantics into
 * those representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A macro may generate quantum syntax.
 *
 * Example conceptually:
 *
 *     macro make_circuit(...) { ... }
 *
 * may produce a quantum program.
 *
 * Expansion itself must remain quantum-domain-neutral.
 *
 * After expansion:
 *
 *     generated quantum syntax
 *             |
 *             v
 *     quantum semantic analysis
 *             |
 *             v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT:
 *
 *     create quantum IR;
 *     define another QubitId;
 *     define PhysicalQubitId;
 *     allocate qubits;
 *     choose gates based on hardware;
 *     select a QPU;
 *     choose topology;
 *     schedule quantum operations.
 */


/*
 * ============================================================================
 * 20. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A macro may generate HDL/hardware syntax.
 *
 * Expansion does not decide:
 *
 *     FPGA family
 *     ASIC process
 *     clock implementation
 *     physical placement
 *     routing
 *     device pin
 *     physical address
 *     resource count
 *
 * Those are later semantic/target/hardware decisions.
 */


/*
 * ============================================================================
 * 21. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Macro expansion may produce ordinary Zamani expressions, statements,
 * declarations, functions, types, memory constructs, and concurrency
 * constructs.
 *
 * The generated structures enter the same canonical semantic pipeline as
 * source-written structures.
 *
 * A generated construct must not receive weaker semantic checking merely
 * because it originated from a macro.
 */


/*
 * ============================================================================
 * 22. HYBRID INTEGRATION
 * ============================================================================
 *
 * Macros may generate programs combining:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     distributed
 *     AI/data
 *
 * Expansion does not choose a domain.
 *
 * The semantic analyzer determines the domain composition after expansion.
 */


/*
 * ============================================================================
 * 23. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Macro expansion may generate resource requirements or constraints only when
 * the canonical resource grammar explicitly represents them.
 *
 * Expansion itself does not inspect actual resources.
 *
 * The distinction remains:
 *
 *     requirement
 *         !=
 *     capability
 *         !=
 *     constraint
 *         !=
 *     preference
 *         !=
 *     target
 *         !=
 *     actual resource
 *
 * A macro saying that a computation semantically requires a capability must
 * not silently mean that a particular physical device must be selected.
 */


/*
 * ============================================================================
 * 24. EFFECT / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Generated code is subject to normal:
 *
 *     type checking
 *     effect checking
 *     capability checking
 *     security checking
 *     resource checking
 *
 * Macro expansion must not be an escape hatch around those systems.
 */


/*
 * ============================================================================
 * 25. RESOLUTION
 * ============================================================================
 *
 * `macroPath` remains owned by the invocation grammar.
 *
 * Expansion does not perform name resolution.
 *
 * The semantic macro resolver must determine:
 *
 *     which macro definition a path denotes
 *     whether it is visible
 *     whether it is compatible
 *     whether overload resolution applies
 *     whether the invocation is permitted
 *
 * The parser merely preserves the path.
 */


/*
 * ============================================================================
 * 26. RECURSION
 * ============================================================================
 *
 * Recursive macro expansion is a semantic/compiler policy.
 *
 * This grammar does not encode a fixed recursion depth.
 *
 * The compiler may provide configurable limits for:
 *
 *     expansion depth
 *     expansion steps
 *     generated structure
 *
 * Such limits are resource-protection mechanisms and must not alter the
 * language's semantic model.
 */


/*
 * ============================================================================
 * 27. EXPANSION CYCLES
 * ============================================================================
 *
 * Expansion-cycle detection belongs to the macro expansion engine.
 *
 * Conceptually:
 *
 *     A -> B
 *     B -> A
 *
 * is a semantic expansion-cycle problem.
 *
 * It is NOT a parser problem.
 *
 * Diagnostics should preserve the complete provenance chain when a cycle is
 * detected.
 */


/*
 * ============================================================================
 * 28. EXPANSION ORDER
 * ============================================================================
 *
 * Expansion ordering belongs to the macro engine.
 *
 * The parser must not encode a machine-dependent ordering.
 *
 * Nested expansion must have a deterministic semantic order defined by the
 * macro system.
 */


/*
 * ============================================================================
 * 29. CACHING / MEMOIZATION
 * ============================================================================
 *
 * Macro expansion caching is not a grammar concern.
 *
 * If implemented, cache identity must include all semantic inputs that affect
 * expansion.
 *
 * A cache must never cause semantically different expansions to be treated as
 * identical merely because their textual invocation happens to match.
 *
 * This file creates no cache keys.
 */


/*
 * ============================================================================
 * 30. VERSIONING
 * ============================================================================
 *
 * Expansion semantics must be version-aware.
 *
 * A compiler may distinguish:
 *
 *     language version
 *     macro language version
 *     annotation schema version
 *     expansion semantic version
 *
 * The parser should remain stable where the syntax remains compatible.
 *
 * A new expansion semantic operation should normally be introduced through
 * the annotation registry rather than by adding a new parser keyword.
 */


/*
 * ============================================================================
 * 31. COMPATIBILITY
 * ============================================================================
 *
 * Existing valid macro invocation syntax must remain valid.
 *
 * In particular, the canonical invocation structure remains owned by
 * invocations.g4.
 *
 * This file must not alter the interpretation of:
 *
 *     foo!()
 *     foo!(x)
 *     foo!(x, y)
 *     module::foo!(x)
 *
 * unless an explicit language-version migration changes that syntax.
 *
 * Such a breaking change must be documented in the compatibility subsystem.
 */


/*
 * ============================================================================
 * 32. AST CONTRACT
 * ============================================================================
 *
 * This grammar does not create a separate "Expansion AST".
 *
 * The canonical frontend AST should represent expansion metadata as part of
 * the canonical annotation structure or as a semantic reference to that
 * structure.
 *
 * The AST must preserve:
 *
 *     source span
 *     annotation span
 *     enclosing macro construct
 *     macro invocation identity
 *     provenance
 *
 * Generated AST nodes must subsequently retain expansion provenance.
 */


/*
 * ============================================================================
 * 33. SOURCE MAP CONTRACT
 * ============================================================================
 *
 * Tooling must be able to answer:
 *
 *     "Where did this generated construct come from?"
 *
 * A generated node may have provenance such as:
 *
 *     generated from macro definition
 *     generated from invocation argument
 *     generated from nested expansion
 *
 * This mapping is a compiler/frontend concern.
 *
 * The grammar must not destroy the information needed to establish it.
 */


/*
 * ============================================================================
 * 34. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser errors include only syntax failures.
 *
 * Examples:
 *
 *     malformed annotation
 *     malformed annotation arguments
 *
 * Semantic expansion errors include:
 *
 *     unresolved macro
 *     inaccessible macro
 *     invalid macro argument
 *     expansion cycle
 *     illegal expansion policy
 *     hygiene violation
 *     capability violation
 *     resource-policy violation
 *     expansion-budget exhaustion
 *
 * Those errors must not be represented as parser syntax errors merely because
 * they occur during expansion.
 */


/*
 * ============================================================================
 * 35. TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, language servers, documentation tools, and source printers
 * must be able to distinguish:
 *
 *     original source
 *     macro invocation
 *     expansion metadata
 *     generated source
 *
 * without executing macro expansion merely to parse the source.
 *
 * This is important for:
 *
 *     fast diagnostics
 *     syntax highlighting
 *     navigation
 *     formatting
 *     refactoring
 *     incremental compilation
 */


/*
 * ============================================================================
 * 36. INCREMENTAL COMPILATION
 * ============================================================================
 *
 * The expansion engine may eventually support incremental compilation.
 *
 * This grammar must not make incremental correctness dependent on:
 *
 *     machine size
 *     hardware identity
 *     runtime state
 *
 * Expansion invalidation must be based on semantic dependencies.
 */


/*
 * ============================================================================
 * 37. DISTRIBUTED / REMOTE COMPILATION
 * ============================================================================
 *
 * Parsing and expansion metadata must remain reproducible when compilation is
 * performed:
 *
 *     locally
 *     remotely
 *     in CI
 *     in a build farm
 *     in a distributed compiler
 *
 * The grammar must not rely on local machine paths or physical device state.
 */


/*
 * ============================================================================
 * 38. FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * Future macro facilities may include:
 *
 *     declarative expansion
 *     procedural-but-sandboxed expansion
 *     syntax generation
 *     AST transformation
 *     typed macro expansion
 *     compile-time functions
 *     reflection
 *     specialization
 *     dialect-aware generation
 *
 * These facilities must not automatically become grammar keywords.
 *
 * Each facility requires its own:
 *
 *     syntax contract
 *     AST contract
 *     semantic contract
 *     capability/effect contract
 *     security contract
 *     determinism contract
 *     compatibility contract
 *
 * before becoming canonical.
 */


/*
 * ============================================================================
 * 39. NO DUPLICATION WITH METAPROGRAMMING
 * ============================================================================
 *
 * `grammar/metaprogramming/` may eventually own broader compile-time language
 * facilities.
 *
 * Macro expansion remains the macro subsystem's responsibility.
 *
 * The two systems must not create competing versions of:
 *
 *     expansion
 *     quotation
 *     reflection
 *     generation
 *
 * Their boundaries must be explicit.
 *
 * If metaprogramming invokes macros, it consumes the canonical macro API.
 *
 * It does not redefine macro expansion syntax.
 */


/*
 * ============================================================================
 * 40. NO DUPLICATION WITH HYGIENE
 * ============================================================================
 *
 * `hygiene.g4` owns the syntax boundary for macro-hygiene annotations.
 *
 * This file consumes/reuses that boundary where needed.
 *
 * It must not redefine:
 *
 *     capture
 *     fresh identity
 *     definition-site context
 *     invocation-site context
 *     binding identity
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. MACROS.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/macros/macros.g4` is the composition boundary.
 *
 * Final conceptual architecture:
 *
 *     macros.g4
 *         |
 *         +--> declarations.g4
 *         |
 *         +--> invocations.g4
 *         |
 *         +--> hygiene.g4
 *         |
 *         +--> expansion.g4
 *
 * `macros.g4` must not redefine any expansion rules from this file.
 *
 * This file must not assume that `macros.g4` owns semantic expansion.
 */


/*
 * ============================================================================
 * 42. DECLARATIONS.G4 INTEGRATION
 * ============================================================================
 *
 * declarations.g4 owns:
 *
 *     macroDeclaration
 *     macroParameterList
 *     macroParameter
 *     macroParameterDefault
 *     macroBody
 *
 * If macro declarations accept expansion metadata, they consume:
 *
 *     optionalMacroExpansionPrefix
 *
 * rather than redefining annotation syntax.
 */


/*
 * ============================================================================
 * 43. INVOCATIONS.G4 INTEGRATION
 * ============================================================================
 *
 * invocations.g4 owns:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * If invocation-side expansion metadata is standardized, it may consume:
 *
 *     macroInvocationExpansion
 *
 * The invocation grammar remains the sole owner of invocation syntax.
 */


/*
 * ============================================================================
 * 44. HYGIENE.G4 INTEGRATION
 * ============================================================================
 *
 * hygiene.g4 owns:
 *
 *     macroHygieneAnnotation
 *     macroHygieneAnnotations
 *     macroHygienePrefix
 *     macroInvocationHygiene
 *     macroDeclarationHygiene
 *     macroExpansionHygiene
 *
 * Expansion metadata must coexist with hygiene metadata rather than replacing
 * it.
 *
 * Conceptually:
 *
 *     macro construct
 *         |
 *         +--> expansion metadata
 *         |
 *         +--> hygiene metadata
 *         |
 *         +--> canonical macro syntax
 *
 * The semantic layer combines these independent dimensions.
 */


/*
 * ============================================================================
 * 45. META.G4 RECONCILIATION
 * ============================================================================
 *
 * The repository contains macro/metaprogramming constructs in:
 *
 *     grammar/antlr/Meta.g4
 *
 * That grammar includes its own macro-related concepts.
 *
 * It must not become a second canonical expansion grammar.
 *
 * The migration target is:
 *
 *     Meta.g4
 *         |
 *         +--> consume canonical macro subsystem
 *         OR
 *         +--> migrate/deprecate duplicated macro productions
 *
 * There must ultimately be one authoritative macro-expansion syntax model.
 */


/*
 * ============================================================================
 * 46. CORE EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Macro invocation remains an expression through:
 *
 *     expression
 *         |
 *         +--> macroExpression
 *                 |
 *                 +--> macroInvocation
 *
 * This file does not add another expression production.
 *
 * Expansion happens after parsing.
 */


/*
 * ============================================================================
 * 47. CANONICAL IR INTEGRATION
 * ============================================================================
 *
 * This grammar MUST NOT depend on canonical IR.
 *
 * The dependency direction is:
 *
 *     grammar
 *       ->
 *     AST
 *       ->
 *     semantic analysis / expansion
 *       ->
 *     canonical IR
 *
 * Never:
 *
 *     grammar -> IR -> grammar
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. QUANTUM::IR INTEGRATION
 * ============================================================================
 *
 * This file has no direct dependency on `quantum::ir`.
 *
 * If expansion generates quantum syntax:
 *
 *     macro expansion
 *         ->
 *     quantum semantic analysis
 *         ->
 *     quantum::ir
 *
 * `quantum::ir` remains the sole canonical quantum semantic boundary.
 */


/*
 * ============================================================================
 * 49. OPTIMIZATION / SCHEDULING / ROUTING
 * ============================================================================
 *
 * Expansion occurs before:
 *
 *     optimization
 *     routing
 *     scheduling
 *
 * Those subsystems consume semantic representations after expansion.
 *
 * This file has no dependency on their policies.
 */


/*
 * ============================================================================
 * 50. ZQN / QEC
 * ============================================================================
 *
 * Expansion has no direct dependency on:
 *
 *     ZQN
 *     QEC
 *
 * If generated quantum semantics later require fault/noise or error-correction
 * processing, those systems operate after normal semantic lowering.
 */


/*
 * ============================================================================
 * 51. HARDWARE / RUNTIME
 * ============================================================================
 *
 * Expansion has no direct dependency on:
 *
 *     hardware HAL
 *     calibration
 *     runtime
 *     deployment
 *     backend selection
 *
 * This preserves POCO-REAF.
 */


/*
 * ============================================================================
 * 52. TEST CONTRACT — POSITIVE
 * ============================================================================
 *
 * Tests must verify:
 *
 *     - expansion annotations parse through canonical annotation syntax;
 *     - ordinary macro invocations remain valid;
 *     - qualified macro paths remain valid;
 *     - expansion metadata can coexist with macro syntax;
 *     - generated semantics can later be consumed by all relevant domains.
 */


/*
 * ============================================================================
 * 53. TEST CONTRACT — NEGATIVE
 * ============================================================================
 *
 * Tests must verify rejection of malformed annotation/expansion syntax.
 *
 * Tests must also ensure that ordinary identifiers such as:
 *
 *     quote
 *     splice
 *     expand
 *     eval
 *
 * do not become parser keywords merely because macro expansion exists.
 */


/*
 * ============================================================================
 * 54. TEST CONTRACT — HYGIENE
 * ============================================================================
 *
 * Verify that expansion metadata does not bypass hygiene.
 *
 * Test cases must include:
 *
 *     caller binding
 *     macro-generated binding
 *     nested expansion
 *     explicit capture where supported
 *     accidental capture
 *     provenance preservation
 */


/*
 * ============================================================================
 * 55. TEST CONTRACT — CROSS-DOMAIN
 * ============================================================================
 *
 * Expansion tests must cover macros producing:
 *
 *     classical constructs
 *     quantum constructs
 *     hybrid constructs
 *     HDL constructs
 *     hardware-independent constructs
 *     distributed constructs
 *     AI/data constructs
 *
 * The expansion parser must remain domain-neutral.
 */


/*
 * ============================================================================
 * 56. TEST CONTRACT — QUANTUM SCALABILITY
 * ============================================================================
 *
 * Tests must verify that expansion syntax itself does not impose limits on:
 *
 *     number of generated qubits
 *     number of generated gates
 *     circuit size
 *     register size
 *     number of quantum resources
 *
 * Any finite test size is test configuration only.
 */


/*
 * ============================================================================
 * 57. TEST CONTRACT — CLASSICAL SCALABILITY
 * ============================================================================
 *
 * Tests must verify that expansion syntax does not impose limits on:
 *
 *     functions
 *     expressions
 *     declarations
 *     generated statements
 *     data structures
 *     parallel tasks
 *
 * Compiler budgets remain configurable resource policies.
 */


/*
 * ============================================================================
 * 58. TEST CONTRACT — HDL/HARDWARE SCALABILITY
 * ============================================================================
 *
 * Tests must verify that macro expansion does not hard-code:
 *
 *     device count
 *     port count
 *     register count
 *     FPGA resources
 *     ASIC resources
 *     topology
 *     physical addresses
 *     fixed clock hardware
 */


/*
 * ============================================================================
 * 59. TEST CONTRACT — DETERMINISM
 * ============================================================================
 *
 * Repeated parsing of identical source must produce equivalent parse trees.
 *
 * Repeated semantic expansion under identical compilation inputs must produce
 * equivalent semantic output.
 */


/*
 * ============================================================================
 * 60. TEST CONTRACT — ROUND TRIP
 * ============================================================================
 *
 * Where a source printer exists:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     printer
 *       ->
 *     parser
 *
 * must preserve the intended macro/expansion metadata semantics.
 */


/*
 * ============================================================================
 * 61. TEST CONTRACT — PROVENANCE
 * ============================================================================
 *
 * Verify that diagnostics can identify:
 *
 *     macro definition location
 *     macro invocation location
 *     expansion metadata location
 *     generated-node provenance
 *     nested expansion origin
 */


/*
 * ============================================================================
 * 62. TEST CONTRACT — RESOURCE BUDGETS
 * ============================================================================
 *
 * Compiler resource-limit tests belong outside the grammar semantics.
 *
 * They should verify that configurable policies can reject excessive expansion
 * without changing the grammar's accepted language.
 */


/*
 * ============================================================================
 * 63. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain no language-level constants representing:
 *
 *     maximum macro depth
 *     maximum expansion count
 *     maximum generated nodes
 *     maximum qubits
 *     maximum cores
 *     maximum threads
 *     maximum devices
 *     maximum nodes
 *     fixed memory
 *     fixed topology
 *     fixed hardware
 *     fixed accelerator
 *     fixed backend
 *
 * No hardware-specific literal may become part of expansion semantics.
 */


/*
 * ============================================================================
 * 64. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 *     [ ] File name is expansion.g4.
 *
 *     [ ] Grammar name is expansion.
 *
 *     [ ] It is a parser grammar.
 *
 *     [ ] tokenVocab resolves to ZamaniLexer.
 *
 *     [ ] No lexer tokens are invented.
 *
 *     [ ] No Rust actions exist.
 *
 *     [ ] No unsafe implementation is required.
 *
 *     [ ] Rust 1.97 / 1.97.1 compatibility is maintained.
 *
 *     [ ] Macro declaration syntax is not duplicated.
 *
 *     [ ] Macro invocation syntax is not duplicated.
 *
 *     [ ] Hygiene syntax is not duplicated.
 *
 *     [ ] Annotation syntax is not duplicated.
 *
 *     [ ] Expression syntax is not duplicated.
 *
 *     [ ] Quote/splice syntax is not invented.
 *
 *     [ ] Expansion is not executed by the parser.
 *
 *     [ ] Arbitrary host-code execution is not introduced.
 *
 *     [ ] Filesystem/network/process access is absent.
 *
 *     [ ] Provenance requirements are preserved.
 *
 *     [ ] Hygiene requirements are preserved.
 *
 *     [ ] Expansion semantics remain downstream.
 *
 *     [ ] No machine-dependent limits exist.
 *
 *     [ ] No quantum-device assumptions exist.
 *
 *     [ ] No hardware assumptions exist.
 *
 *     [ ] No resource counts are hard-coded.
 *
 *     [ ] macros.g4 integrates this grammar as a component.
 *
 *     [ ] declarations.g4 has a predefined expansion integration point.
 *
 *     [ ] invocations.g4 has a predefined expansion integration point.
 *
 *     [ ] hygiene.g4 remains the hygiene owner.
 *
 *     [ ] Meta.g4 is prevented from becoming a duplicate canonical owner.
 *
 *     [ ] AST integration preserves source spans.
 *
 *     [ ] AST integration preserves macro provenance.
 *
 *     [ ] Semantic expansion consumes this syntax.
 *
 *     [ ] Generated syntax returns through ordinary semantic analysis.
 *
 *     [ ] quantum-generated syntax ultimately lowers through quantum::ir.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Hygiene tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Round-trip tests pass where supported.
 *
 *     [ ] Provenance tests pass.
 *
 *     [ ] Hard-coding audit passes.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file establishes the boundary:
 *
 *     expansion syntax
 *         !=
 *     expansion algorithm
 *
 *     expansion
 *         !=
 *     evaluation
 *
 *     generated syntax
 *         !=
 *     machine implementation
 *
 *     macro hygiene
 *         !=
 *     textual renaming
 *
 *     source semantics
 *         !=
 *     hardware topology
 *
 * Therefore:
 *
 *     one Zamani source program
 *          |
 *          v
 *     one semantic program
 *          |
 *          v
 *     macro expansion
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL
 *          +--> hardware
 *          +--> distributed
 *          +--> future domains
 *          |
 *          v
 *     many targets
 *
 * remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */