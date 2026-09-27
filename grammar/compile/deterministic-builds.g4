/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/deterministic-builds.g4
 *
 * Grammar:
 *     CompileDeterministicBuilds
 *
 * Status:
 *     PRODUCTION SOURCE-SYNTAX CONTRACT
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions and requires no unsafe
 *     Rust. The consuming Zamani compiler/frontend MUST use safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL DETERMINISTIC-BUILD INTENT.
 *
 * A deterministic build is a compilation contract requiring the compiler to
 * treat all semantically relevant build inputs and build-ordering decisions
 * according to an explicitly defined deterministic policy.
 *
 * This file describes INTENT.
 *
 * It does NOT:
 *
 *     - execute a build;
 *     - execute a compiler;
 *     - execute a linker;
 *     - inspect the host;
 *     - inspect hardware;
 *     - discover toolchains;
 *     - resolve dependencies;
 *     - access the filesystem;
 *     - access the network;
 *     - calculate hashes;
 *     - implement caching;
 *     - generate provenance;
 *     - perform optimization;
 *     - perform target selection;
 *     - perform cross-compilation;
 *     - perform deployment;
 *     - schedule runtime work;
 *     - perform quantum routing;
 *     - perform QEC;
 *     - perform ZQN;
 *     - construct an IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     compile.g4
 *          |
 *          +--> deterministicBuildClause
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> deterministic-build analysis
 *          +--> input identity analysis
 *          +--> dependency analysis
 *          +--> environment analysis
 *          +--> toolchain analysis
 *          +--> artifact analysis
 *          +--> reproducibility analysis
 *          |
 *          v
 *     canonical semantic compilation model
 *          |
 *          +--------------------+----------------------+
 *          |                    |                      |
 *          v                    v                      v
 *      classical            quantum::ir          HDL/hardware
 *          |                    |                      |
 *          +--------------------+----------------------+
 *                               |
 *                               v
 *                         optimization
 *                               |
 *                       lowering / routing
 *                               |
 *                         scheduling
 *                               |
 *                     resilience / QEC / ZQN
 *                               |
 *                               v
 *                              HAL
 *                               |
 *                               v
 *                       target realization
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Deterministic-build intent MUST NOT make the language dependent on a
 * particular machine.
 *
 * The grammar MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_TARGETS
 *     MAX_DEPENDENCIES
 *     MAX_INPUTS
 *     MAX_ARTIFACTS
 *     MAX_BUILD_STEPS
 *
 * Nor may equivalent finite limits be hidden in parser structure.
 *
 * Repetition therefore uses:
 *
 *     *
 *     +
 *
 * rather than finite alternatives.
 *
 * "Unbounded" means:
 *
 *     no artificial language-level finite cardinality.
 *
 * It does NOT mean:
 *
 *     infinite physical memory;
 *     infinite compiler memory;
 *     infinite storage;
 *     infinite compilation time;
 *     infinite hardware.
 *
 * Practical implementation limits belong to compiler/resource policy and
 * MUST remain distinguishable from language semantics.
 *
 * ============================================================================
 * DETERMINISTIC BUILD != DETERMINISTIC EXECUTION
 * ============================================================================
 *
 * This file owns BUILD determinism.
 *
 * It does NOT own:
 *
 *     runtime scheduling;
 *     concurrent execution order;
 *     distributed execution order;
 *     quantum measurement outcomes;
 *     physical timing;
 *     runtime randomness;
 *     hardware behavior.
 *
 * Therefore:
 *
 *     deterministic build
 *         !=
 *     deterministic execution
 *
 *     deterministic build
 *         !=
 *     reproducibility
 *
 *     deterministic build
 *         !=
 *     bit-for-bit artifact identity
 *
 * These concepts may interact semantically but retain separate ownership.
 *
 * Reproducibility is owned by:
 *
 *     grammar/compile/reproducibility.g4
 *
 * Runtime/execution determinism belongs to:
 *
 *     grammar/execution/
 *
 * ============================================================================
 * DETERMINISM MODEL
 * ============================================================================
 *
 * A deterministic build requires the compiler to establish a deterministic
 * relationship between declared semantic inputs and compilation decisions.
 *
 * Conceptually:
 *
 *     source
 *       +
 *     declared inputs
 *       +
 *     declared dependencies
 *       +
 *     declared build context
 *       +
 *     deterministic policy
 *       |
 *       v
 *     deterministic compilation process
 *       |
 *       v
 *     compilation result
 *
 * The grammar does not decide whether two artifacts are equivalent.
 *
 * Semantic/compiler layers determine:
 *
 *     - which inputs are semantically relevant;
 *     - which ordering constraints matter;
 *     - whether a build is deterministic;
 *     - whether artifacts are reproducible;
 *     - whether bit-for-bit identity is required;
 *     - whether target realization affects artifact identity.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - deterministic-build clause syntax;
 *     - deterministic-build policy composition;
 *     - deterministic-build requirements;
 *     - deterministic-build constraints;
 *     - deterministic-build preferences;
 *     - deterministic-build hints;
 *     - deterministic-build capability requirements;
 *     - deterministic-build resource intent;
 *     - deterministic-build input declarations;
 *     - deterministic-build dependency declarations;
 *     - deterministic-build environment declarations;
 *     - deterministic-build artifact intent;
 *     - deterministic-build ordering intent;
 *     - deterministic-build property syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - target declarations;
 *     - target selection;
 *     - cross-compilation;
 *     - reproducibility;
 *     - caching implementation;
 *     - provenance implementation;
 *     - optimization implementation;
 *     - specialization implementation;
 *     - code generation;
 *     - lowering;
 *     - linker implementation;
 *     - deployment;
 *     - hardware discovery;
 *     - resource allocation;
 *     - quantum operations;
 *     - classical operations;
 *     - HDL;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *
 * General compilation intent:
 *
 *     grammar/compile/compile.g4
 *
 * Deterministic-build syntax:
 *
 *     THIS FILE
 *
 * Reproducibility syntax:
 *
 *     grammar/compile/reproducibility.g4
 *
 * Cross-compilation syntax:
 *
 *     grammar/compile/cross-compilation.g4
 *
 * Target syntax:
 *
 *     grammar/compile/target.g4
 *
 * Target-selection syntax:
 *
 *     grammar/compile/target-selection.g4
 *
 * Caching syntax/semantics:
 *
 *     grammar/compile/caching.g4
 *
 * Provenance syntax/semantics:
 *
 *     grammar/compile/provenance.g4
 *
 * Current implementation conformance:
 *
 *     grammar/grammar.md
 *
 * Historical/extended design:
 *
 *     grammar/Zamani-Grammar.md
 *
 * This file MUST NOT become a second language specification.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This parser grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The current canonical keyword vocabulary does not define a DETERMINISTIC
 * token.
 *
 * One lexical integration is therefore required:
 *
 *     DETERMINISTIC : 'deterministic' ;
 *
 * in:
 *
 *     grammar/lexer/keywords.g4
 *
 * This is the only new reserved word required by this grammar.
 *
 * The grammar deliberately does NOT require reserved keywords for:
 *
 *     input
 *     dependency
 *     environment
 *     artifact
 *     ordering
 *     seed
 *     timestamp
 *     locale
 *     path
 *     metadata
 *     toolchain
 *     compiler
 *     linker
 *
 * Those remain ordinary identifiers/property names so future build systems
 * do not require lexical expansion merely because a new semantic property is
 * introduced.
 *
 * ============================================================================
 * ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Core
 *     Expressions
 *
 * Core provides:
 *
 *     identifier
 *     qualifiedName
 *
 * Expressions provides:
 *
 *     expression
 *
 * No equivalent rule is redefined here.
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar CompileDeterministicBuilds;

options {
    tokenVocab = ZamaniLexer;
}

import Core,
       Expressions;


/*
 * ============================================================================
 * 1. PUBLIC COMPILE CLAUSE
 * ============================================================================
 *
 * The enclosing compile grammar owns:
 *
 *     COMPILE
 *
 * Therefore this rule deliberately consumes only:
 *
 *     deterministic ...
 *
 * Canonical source form:
 *
 *     compile deterministic;
 *
 * or:
 *
 *     compile deterministic {
 *         ...
 *     }
 *
 * This file MUST NOT define another `compile` declaration.
 */

deterministicBuildClause
    : DETERMINISTIC deterministicBuildSpecification
    ;


/*
 * ============================================================================
 * 2. DETERMINISTIC-BUILD SPECIFICATION
 * ============================================================================
 *
 * An empty deterministic clause is meaningful:
 *
 *     compile deterministic;
 *
 * It requests the language-defined default deterministic-build policy.
 *
 * The semantic layer, not the parser, defines that default.
 */

deterministicBuildSpecification
    : deterministicBuildBody?
    ;


/*
 * ============================================================================
 * 3. DETERMINISTIC-BUILD BODY
 * ============================================================================
 *
 * There is no finite maximum number of entries.
 */

deterministicBuildBody
    : LBRACE deterministicBuildEntry* RBRACE
    ;


/*
 * ============================================================================
 * 4. ENTRY DISPATCH
 * ============================================================================
 *
 * Semantic categories remain distinct.
 *
 * In particular:
 *
 *     requirement != constraint
 *     preference != hint
 *     capability != resource
 *     input != dependency
 *     environment != property
 *     artifact != metadata
 *
 * The frontend AST MUST preserve these distinctions.
 */

deterministicBuildEntry
    : deterministicRequirement
    | deterministicConstraint
    | deterministicPreference
    | deterministicHint
    | deterministicCapability
    | deterministicResource
    | deterministicInput
    | deterministicDependency
    | deterministicEnvironment
    | deterministicArtifact
    | deterministicOrdering
    | deterministicProperty
    ;


/*
 * ============================================================================
 * 5. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory deterministic-build intent.
 *
 * Examples:
 *
 *     requires capability("deterministic.build");
 *
 *     requires compiler_identity;
 *
 *     requires dependency_identity;
 *
 *     requires stable_ordering;
 *
 * The parser does not determine satisfiability.
 */

deterministicRequirement
    : REQUIRES expression SEMI?
    ;


/*
 * ============================================================================
 * 6. CONSTRAINT
 * ============================================================================
 *
 * A constraint limits acceptable build implementations.
 *
 * The expression remains an ordinary Zamani expression.
 */

deterministicConstraint
    : CONSTRAINT expression SEMI?
    ;


/*
 * ============================================================================
 * 7. PREFERENCE
 * ============================================================================
 *
 * A preference provides non-mandatory deterministic-build guidance.
 *
 * Ignoring a preference MUST NOT invalidate program semantics.
 */

deterministicPreference
    : PREFER expression SEMI?
    ;


/*
 * ============================================================================
 * 8. HINT
 * ============================================================================
 *
 * A hint is non-binding implementation guidance.
 */

deterministicHint
    : HINT expression SEMI?
    ;


/*
 * ============================================================================
 * 9. CAPABILITY
 * ============================================================================
 *
 * Capability names are deliberately open-ended.
 *
 * Examples:
 *
 *     capability("deterministic.build");
 *
 *     capability("stable.artifact.order");
 *
 *     capability("hermetic.build");
 *
 * The grammar does not enumerate capability names.
 */

deterministicCapability
    : CAPABILITY expression SEMI?
    ;


/*
 * ============================================================================
 * 10. RESOURCE
 * ============================================================================
 *
 * Resource intent remains semantic.
 *
 * Examples:
 *
 *     resource memory >= required_memory;
 *
 *     resources storage >= required_storage;
 *
 * Determinism does not create machine-size limits.
 */

deterministicResource
    : RESOURCE expression SEMI?
    | RESOURCES expression SEMI?
    ;


/*
 * ============================================================================
 * 11. INPUT DECLARATION
 * ============================================================================
 *
 * Build inputs identify semantic inputs that may affect deterministic
 * compilation.
 *
 * The input itself remains symbolic.
 *
 * The grammar does not access or hash it.
 *
 * Supported forms include:
 *
 *     input source = source_identity;
 *     input schema = schema_identity;
 *     input data = data_identity;
 */

deterministicInput
    : deterministicNamedValue
    ;


/*
 * ============================================================================
 * 12. DEPENDENCY DECLARATION
 * ============================================================================
 *
 * Dependencies remain semantic references.
 *
 * Dependency resolution belongs downstream.
 */

deterministicDependency
    : deterministicNamedValue
    ;


/*
 * ============================================================================
 * 13. ENVIRONMENT DECLARATION
 * ============================================================================
 *
 * An environment entry may describe semantically relevant environment
 * properties.
 *
 * It does not cause the parser to inspect the host environment.
 *
 * Examples:
 *
 *     environment compiler = compiler_identity;
 *     environment locale = locale_identity;
 *     environment timezone = timezone_identity;
 */

deterministicEnvironment
    : deterministicNamedValue
    ;


/*
 * ============================================================================
 * 14. ARTIFACT DECLARATION
 * ============================================================================
 *
 * Artifact identity is downstream semantic/compiler data.
 *
 * This grammar only records intent.
 */

deterministicArtifact
    : ARTIFACT deterministicNamedValue
    ;


/*
 * ============================================================================
 * 15. ORDERING DECLARATION
 * ============================================================================
 *
 * Deterministic builds frequently require stable ordering of otherwise
 * unordered semantic collections.
 *
 * Ordering remains an intent expression.
 *
 * Examples:
 *
 *     ordering modules = stable;
 *     ordering dependencies = canonical;
 *     ordering artifacts = canonical;
 *
 * The actual ordering algorithm belongs downstream.
 */

deterministicOrdering
    : deterministicOrderingHead deterministicValueAssignment
    ;


deterministicOrderingHead
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 16. GENERIC PROPERTY
 * ============================================================================
 *
 * `property` is the extensibility mechanism for deterministic-build
 * properties that are not yet first-class semantic categories.
 *
 * Example:
 *
 *     property timestamp = fixed;
 *
 *     property path = normalized;
 *
 *     property metadata = canonical;
 *
 *     property toolchain = toolchain.identity;
 *
 * Property names are data, not a second keyword registry.
 */

deterministicProperty
    : PROPERTY deterministicNamedValue
    ;


/*
 * ============================================================================
 * 17. NAMED VALUE
 * ============================================================================
 *
 * Named values deliberately use ordinary identifiers.
 *
 * This permits future deterministic-build concepts without adding a reserved
 * keyword for every new property.
 */

deterministicNamedValue
    : deterministicValueName deterministicValueAssignment SEMI?
    ;


deterministicValueName
    : identifier
    | qualifiedName
    ;


deterministicValueAssignment
    : COLON expression
    | ASSIGN expression
    ;


/*
 * ============================================================================
 * 18. GENERIC DETERMINISTIC PROPERTY EXTENSION
 * ============================================================================
 *
 * The property system is intentionally expression-based.
 *
 * Therefore a deterministic-build property can refer to:
 *
 *     literals;
 *     names;
 *     qualified names;
 *     functions;
 *     compile-time values;
 *     resource requirements;
 *     capabilities;
 *     target-independent policies;
 *     domain-specific semantic values.
 *
 * Interpretation belongs to semantic analysis.
 *
 * The parser MUST NOT evaluate expressions.
 */


/*
 * ============================================================================
 * 19. RELATIONSHIP WITH REPRODUCIBILITY
 * ============================================================================
 *
 * Deterministic build and reproducibility are related but separate.
 *
 * Deterministic build:
 *
 *     controls semantic ordering and build-process determinism.
 *
 * Reproducibility:
 *
 *     controls the conditions under which equivalent compilation inputs and
 *     declared contexts produce equivalent compilation results.
 *
 * A program may request both:
 *
 *     compile deterministic {
 *         ...
 *     }
 *
 *     compile reproducible {
 *         ...
 *     }
 *
 * These clauses MUST remain separate AST nodes.
 *
 * Neither grammar may silently absorb the other.
 *
 * Deterministic-build semantics MAY contribute inputs to reproducibility
 * analysis, but that relationship belongs to semantic/compiler integration.
 */


/*
 * ============================================================================
 * 20. RELATIONSHIP WITH CACHING
 * ============================================================================
 *
 * Caching belongs to the caching subsystem.
 *
 * Deterministic-build intent may make cache keys more stable, but this grammar
 * does not define:
 *
 *     cache keys;
 *     cache storage;
 *     cache invalidation;
 *     content-addressed storage;
 *     remote cache protocols.
 *
 * Those remain downstream.
 *
 * A deterministic build MUST NOT imply:
 *
 *     cache;
 *
 * and caching MUST NOT imply:
 *
 *     deterministic build.
 */


/*
 * ============================================================================
 * 21. RELATIONSHIP WITH PROVENANCE
 * ============================================================================
 *
 * Provenance records the origin and identity of compilation inputs.
 *
 * Deterministic-build intent may require provenance-relevant properties, but
 * this grammar does not construct provenance records.
 *
 * Provenance belongs downstream.
 */


/*
 * ============================================================================
 * 22. RELATIONSHIP WITH TARGET SELECTION
 * ============================================================================
 *
 * Deterministic builds MUST remain target-independent.
 *
 * A deterministic-build declaration may constrain target-dependent inputs
 * through ordinary expressions, but it must not enumerate physical targets.
 *
 * Target selection remains owned by:
 *
 *     grammar/compile/target-selection.g4
 *
 * This grammar MUST NOT define:
 *
 *     CPU models;
 *     GPU models;
 *     FPGA families;
 *     QPU models;
 *     physical device identifiers;
 *     physical qubit identifiers;
 *     node identifiers.
 */


/*
 * ============================================================================
 * 23. RELATIONSHIP WITH CROSS-COMPILATION
 * ============================================================================
 *
 * Cross-compilation remains owned by:
 *
 *     grammar/compile/cross-compilation.g4
 *
 * Deterministic-build intent may apply to a cross-compilation request, but
 * this grammar does not redefine:
 *
 *     source target;
 *     destination target;
 *     ABI;
 *     sysroot;
 *     toolchain;
 *     target route.
 *
 * Example composition:
 *
 *     compile deterministic {
 *         requires capability("deterministic.build");
 *         property ordering = canonical;
 *     }
 *
 *     compile cross host -> target;
 *
 * Semantic analysis determines how the policies interact.
 */


/*
 * ============================================================================
 * 24. RELATIONSHIP WITH SPECIALIZATION
 * ============================================================================
 *
 * Specialization may legitimately produce target- or input-specific builds.
 *
 * Deterministic-build semantics therefore require specialization inputs to be
 * explicit whenever they affect the result.
 *
 * This grammar does not redefine specialization.
 *
 * Specialization remains owned by:
 *
 *     grammar/compile/specialization.g4
 *
 * The semantic layer determines whether specialization choices are:
 *
 *     deterministic;
 *     reproducible;
 *     cache-relevant;
 *     artifact-relevant.
 */


/*
 * ============================================================================
 * 25. RELATIONSHIP WITH OPTIMIZATION
 * ============================================================================
 *
 * Optimization remains owned by:
 *
 *     grammar/compile/optimization.g4
 *
 * A deterministic-build policy does not forbid optimization.
 *
 * Instead, the semantic/compiler layers must ensure that optimization choices
 * that affect the requested deterministic contract are stable and explicitly
 * represented in the relevant compilation context.
 *
 * This grammar does not define optimization algorithms or pass ordering.
 */


/*
 * ============================================================================
 * 26. RELATIONSHIP WITH RESOURCE SEMANTICS
 * ============================================================================
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *
 * Deterministic-build syntax may reference resource requirements.
 *
 * Example:
 *
 *     requires memory >= required_memory;
 *
 * This means:
 *
 *     semantic requirement
 *
 * not:
 *
 *     compiler maximum memory
 *
 * The language MUST remain capable of expressing programs from very small
 * machines to arbitrarily large machines subject to actual resources.
 */


/*
 * ============================================================================
 * 27. RELATIONSHIP WITH QUANTUM COMPUTING
 * ============================================================================
 *
 * Deterministic build does not imply deterministic quantum execution.
 *
 * Quantum programs may contain:
 *
 *     measurement;
 *     probabilistic outcomes;
 *     noise;
 *     adaptive control;
 *     error correction.
 *
 * Those semantics remain owned by the quantum subsystem.
 *
 * A deterministic build means the compilation process itself is governed by a
 * deterministic policy.
 *
 * Quantum computation continues through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar MUST NOT create a QuantumCompilationIR.
 */


/*
 * ============================================================================
 * 28. RELATIONSHIP WITH HDL
 * ============================================================================
 *
 * HDL synthesis may have implementation-dependent choices.
 *
 * Deterministic-build policy can require those choices to be stable when they
 * are part of the semantic compilation contract.
 *
 * This grammar does not define:
 *
 *     synthesis algorithms;
 *     FPGA inventories;
 *     ASIC libraries;
 *     routing resources;
 *     clock frequencies;
 *     physical placement.
 *
 * Those belong downstream.
 */


/*
 * ============================================================================
 * 29. RELATIONSHIP WITH CLASSICAL / AI / DATA / DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * The deterministic-build contract is domain-neutral.
 *
 * It applies equally to:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     AI;
 *     tensor;
 *     data;
 *     distributed;
 *     networking;
 *     security;
 *     future computing domains.
 *
 * Domain-specific semantic inputs remain owned by their domain subsystems.
 */


/*
 * ============================================================================
 * 30. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve a structure conceptually equivalent to:
 *
 *     DeterministicBuild
 *         source_span
 *         requirements[]
 *         constraints[]
 *         preferences[]
 *         hints[]
 *         capabilities[]
 *         resources[]
 *         inputs[]
 *         dependencies[]
 *         environments[]
 *         artifacts[]
 *         orderings[]
 *         properties[]
 *
 * Every child must retain its own source span.
 *
 * The AST MUST NOT collapse all entries into:
 *
 *     HashMap<String, String>
 *
 * because that would erase semantic categories and prevent reliable
 * diagnostics, validation, compatibility analysis, and future evolution.
 */


/*
 * ============================================================================
 * 31. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. identify the deterministic-build declaration;
 *     2. preserve all semantic categories;
 *     3. resolve names and qualified names;
 *     4. validate expressions;
 *     5. identify semantically relevant inputs;
 *     6. identify semantically relevant dependencies;
 *     7. identify environment dependencies;
 *     8. identify ordering requirements;
 *     9. validate deterministic-build capabilities;
 *    10. validate resource requirements;
 *    11. distinguish requirements from preferences;
 *    12. distinguish deterministic build from reproducibility;
 *    13. distinguish deterministic build from runtime determinism;
 *    14. detect contradictory mandatory policies;
 *    15. preserve source ordering where the language requires it;
 *    16. produce deterministic semantic representation;
 *    17. preserve source spans for diagnostics.
 *
 * The grammar performs none of these semantic operations.
 */


/*
 * ============================================================================
 * 32. DETERMINISTIC SEMANTIC NORMALIZATION
 * ============================================================================
 *
 * The semantic/compiler layer may normalize deterministic-build declarations.
 *
 * Normalization MUST be deterministic.
 *
 * If two syntactically different declarations are semantically equivalent,
 * normalization may canonicalize them only when the language specification
 * explicitly defines that equivalence.
 *
 * The parser MUST NOT perform normalization.
 */


/*
 * ============================================================================
 * 33. ORDERING CONTRACT
 * ============================================================================
 *
 * The presence of multiple deterministic entries does not automatically mean
 * that source order is semantically meaningful.
 *
 * Semantic rules must explicitly define which collections are:
 *
 *     ordered;
 *     unordered;
 *     canonically ordered.
 *
 * When canonical ordering is required, the ordering algorithm belongs to the
 * compiler semantic/build layer.
 *
 * The grammar merely records the user's policy.
 */


/*
 * ============================================================================
 * 34. RANDOMNESS CONTRACT
 * ============================================================================
 *
 * Deterministic builds must make build-affecting randomness explicit.
 *
 * This grammar intentionally does not introduce a `seed` keyword.
 *
 * A build policy may express it as data:
 *
 *     property seed = build_seed;
 *
 * or:
 *
 *     property randomness = disabled;
 *
 * or:
 *
 *     property randomness = canonical;
 *
 * The semantic/compiler layer determines whether a given property is
 * sufficient to establish determinism.
 *
 * Runtime randomness remains outside this grammar.
 */


/*
 * ============================================================================
 * 35. TIME CONTRACT
 * ============================================================================
 *
 * Wall-clock time MUST NOT silently influence a deterministic build when it
 * affects semantic output.
 *
 * Time-related policy may be represented through ordinary properties:
 *
 *     property timestamp = fixed;
 *
 *     property timestamp = source_time;
 *
 *     property timestamp = excluded;
 *
 * The compiler decides how timestamps affect artifacts.
 *
 * This grammar does not inspect the clock.
 */


/*
 * ============================================================================
 * 36. PATH CONTRACT
 * ============================================================================
 *
 * Host filesystem paths may differ between machines.
 *
 * Therefore deterministic-build policy may describe path handling:
 *
 *     property path = normalized;
 *
 *     property path = canonical;
 *
 *     property path = source_relative;
 *
 * The grammar does not inspect or normalize paths.
 */


/*
 * ============================================================================
 * 37. LOCALE / ENVIRONMENT CONTRACT
 * ============================================================================
 *
 * Environment-sensitive compilation must be explicit when relevant.
 *
 * Examples:
 *
 *     environment locale = locale_identity;
 *
 *     environment timezone = timezone_identity;
 *
 *     environment compiler = compiler_identity;
 *
 * The parser does not inspect the environment.
 *
 * Semantic analysis determines whether the referenced environment value is
 * part of the deterministic-build contract.
 */


/*
 * ============================================================================
 * 38. TOOLCHAIN CONTRACT
 * ============================================================================
 *
 * Toolchains remain symbolic semantic inputs.
 *
 * Examples:
 *
 *     property toolchain = toolchain::stable;
 *
 *     property compiler = compiler_identity;
 *
 * The grammar does not enumerate:
 *
 *     GCC;
 *     Clang;
 *     LLVM;
 *     Rust;
 *     CUDA;
 *     ROCm;
 *     vendor toolchains;
 *     FPGA tools;
 *     quantum vendor tools.
 *
 * Such identities remain data, dialect vocabulary, or semantic configuration.
 */


/*
 * ============================================================================
 * 39. ABI CONTRACT
 * ============================================================================
 *
 * ABI identity may affect deterministic artifact generation.
 *
 * It is therefore permitted as an expression/property value:
 *
 *     property abi = abi::portable;
 *
 * This grammar does not enumerate ABIs and does not implement ABI checking.
 */


/*
 * ============================================================================
 * 40. ARTIFACT CONTRACT
 * ============================================================================
 *
 * Deterministic-build intent may describe artifact properties:
 *
 *     artifact executable = executable_identity;
 *
 *     property artifact_order = canonical;
 *
 *     property metadata = canonical;
 *
 * Artifact hashing and identity remain downstream.
 */


/*
 * ============================================================================
 * 41. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO IR.
 *
 * It MUST NOT define:
 *
 *     DeterministicBuildIR
 *     BuildIR
 *     ReproducibleBuildIR
 *     QuantumBuildIR
 *     HardwareBuildIR
 *
 * Deterministic-build information is represented in the existing semantic
 * compilation model.
 *
 * Quantum programs continue through:
 *
 *     quantum::ir
 *
 * as the sole canonical quantum IR boundary.
 */


/*
 * ============================================================================
 * 42. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes the semantic deterministic-build model to establish a
 * deterministic compilation process.
 *
 * Conceptually:
 *
 *     deterministic-build AST
 *          |
 *          v
 *     semantic deterministic policy
 *          |
 *          +--> input identity
 *          +--> dependency identity
 *          +--> ordering policy
 *          +--> environment policy
 *          +--> toolchain policy
 *          +--> artifact policy
 *          |
 *          v
 *     compilation plan
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          v
 *     optimization/lowering
 *          |
 *          v
 *     target realization
 *
 * The grammar itself never executes any stage.
 */


/*
 * ============================================================================
 * 43. REPRODUCIBILITY INTEGRATION
 * ============================================================================
 *
 * A deterministic build can contribute to a reproducibility contract.
 *
 * However:
 *
 *     deterministic build
 *
 * does not automatically mean:
 *
 *     reproducible artifact.
 *
 * Reproducibility analysis must account for all relevant compilation inputs,
 * target information, dependencies, toolchains, environment properties, and
 * artifact semantics.
 *
 * The two AST constructs remain separate.
 */


/*
 * ============================================================================
 * 44. CACHE INTEGRATION
 * ============================================================================
 *
 * A deterministic build may improve cacheability.
 *
 * Cache validity remains owned by the caching subsystem.
 *
 * Cache keys MUST account for all semantic inputs that affect the artifact.
 *
 * A deterministic-build clause must never cause a cache hit to be accepted
 * merely because source text is identical.
 */


/*
 * ============================================================================
 * 45. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Provenance may record:
 *
 *     language version;
 *     grammar version;
 *     source identity;
 *     dependency identities;
 *     compiler identity;
 *     toolchain identity;
 *     deterministic-build policy;
 *     target intent;
 *     artifact identity.
 *
 * Provenance generation is downstream.
 */


/*
 * ============================================================================
 * 46. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics MUST identify malformed syntax only.
 *
 * Examples:
 *
 *     compile deterministic {
 *         requires;
 *     }
 *
 * is a parser error.
 *
 * By contrast:
 *
 *     compile deterministic {
 *         requires capability("does.not.exist");
 *     }
 *
 * may parse successfully and become a semantic capability error.
 *
 * Likewise:
 *
 *     compile deterministic {
 *         property toolchain = unavailable_toolchain;
 *     }
 *
 * is not necessarily a parser error.
 *
 * Toolchain availability belongs downstream.
 */


/*
 * ============================================================================
 * 47. SECURITY CONTRACT
 * ============================================================================
 *
 * Deterministic-build syntax MUST NOT grant authority.
 *
 * Parsing this grammar cannot by itself:
 *
 *     execute commands;
 *     access files;
 *     access networks;
 *     access credentials;
 *     invoke compilers;
 *     invoke linkers;
 *     discover hardware;
 *     allocate devices.
 *
 * External-process and filesystem access are controlled by the compiler/build
 * security model.
 */


/*
 * ============================================================================
 * 48. DETERMINISM OF THE PARSER
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source tokens;
 *     language version;
 *     grammar version;
 *     lexical configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU availability;
 *     GPU availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     environment variables.
 */


/*
 * ============================================================================
 * 49. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no finite limit on:
 *
 *     deterministic entries;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     capabilities;
 *     resources;
 *     inputs;
 *     dependencies;
 *     environments;
 *     artifacts;
 *     properties;
 *     ordering declarations.
 *
 * Practical compiler resource limits MAY exist.
 *
 * Such limits must be:
 *
 *     implementation-level;
 *     explicit;
 *     diagnosable;
 *     separate from language semantics.
 *
 * They MUST NOT become universal language constants.
 */


/*
 * ============================================================================
 * 50. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding new deterministic-build properties should normally be source
 * compatible when they use the existing generic property mechanism.
 *
 * Making a new spelling a reserved keyword is potentially source-breaking.
 *
 * Therefore future deterministic-build concepts SHOULD prefer:
 *
 *     property <name> = <expression>;
 *
 * over adding a new reserved keyword unless structural ambiguity requires it.
 *
 * The DETERMINISTIC keyword itself is reserved because it establishes the
 * deterministic-build declaration boundary.
 */


/*
 * ============================================================================
 * 51. DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects may extend deterministic-build properties through the existing
 * dialect mechanism.
 *
 * A dialect MUST NOT:
 *
 *     redefine deterministicBuildClause;
 *     redefine deterministicBuildSpecification;
 *     redefine deterministicBuildEntry;
 *     silently redefine DETERMINISTIC;
 *     turn preferences into requirements;
 *     create a second deterministic-build language.
 *
 * Dialect-specific properties should be explicitly namespaced where needed.
 */


/*
 * ============================================================================
 * 52. FUTURE COMPUTING CONTRACT
 * ============================================================================
 *
 * Deterministic-build syntax is deliberately independent of a fixed hardware
 * taxonomy.
 *
 * It therefore remains applicable to:
 *
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     emulators;
 *     clusters;
 *     HPC systems;
 *     distributed systems;
 *     cloud systems;
 *     future computational substrates.
 *
 * No future target requires modification of this grammar merely because it is
 * a new kind of machine.
 *
 * New semantics follow:
 *
 *     proposal
 *       ->
 *     semantic design
 *       ->
 *     AST contract
 *       ->
 *     grammar
 *       ->
 *     implementation
 *       ->
 *     IR contract
 *       ->
 *     tests
 *       ->
 *     stable
 */


/*
 * ============================================================================
 * 53. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimal deterministic build:
 *
 *     compile deterministic;
 *
 * Explicit policy:
 *
 *     compile deterministic {
 *         requires capability("deterministic.build");
 *     }
 *
 * Multiple categories:
 *
 *     compile deterministic {
 *         requires capability("deterministic.build");
 *         constraint ordering == canonical;
 *         prefer toolchain::stable;
 *         hint artifact::portable;
 *     }
 *
 * Input/dependency declarations:
 *
 *     compile deterministic {
 *         input source = source_identity;
 *         dependency core = dependency_identity;
 *         environment compiler = compiler_identity;
 *         artifact executable = artifact_identity;
 *     }
 *
 * Ordering:
 *
 *     compile deterministic {
 *         ordering modules = canonical;
 *         ordering dependencies = canonical;
 *         ordering artifacts = canonical;
 *     }
 *
 * Generic properties:
 *
 *     compile deterministic {
 *         property timestamp = fixed;
 *         property path = normalized;
 *         property metadata = canonical;
 *         property toolchain = toolchain::stable;
 *     }
 *
 * Resource-aware deterministic build:
 *
 *     compile deterministic {
 *         requires memory >= required_memory;
 *         requires capability("tensor.compute");
 *     }
 *
 * Quantum-aware compilation:
 *
 *     compile deterministic {
 *         requires capability("quantum.measurement");
 *         property quantum_ir = canonical;
 *     }
 *
 * HDL-aware compilation:
 *
 *     compile deterministic {
 *         requires capability("hdl.synthesis");
 *         property synthesis_order = canonical;
 *     }
 */


/*
 * ============================================================================
 * 54. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must be rejected syntactically:
 *
 *     compile deterministic {
 *         requires;
 *     }
 *
 *     compile deterministic {
 *         constraint;
 *     }
 *
 *     compile deterministic {
 *         prefer;
 *     }
 *
 *     compile deterministic {
 *         hint;
 *     }
 *
 *     compile deterministic {
 *         capability;
 *     }
 *
 *     compile deterministic {
 *         resource;
 *     }
 *
 *     compile deterministic {
 *         property;
 *     }
 *
 *     compile deterministic {
 *         ordering;
 *     }
 *
 *     compile deterministic garbage;
 *
 * when `garbage` is not valid deterministic-build syntax.
 *
 * Also reject malformed assignments:
 *
 *     compile deterministic {
 *         property timestamp;
 *     }
 *
 *     compile deterministic {
 *         input;
 *     }
 *
 *     compile deterministic {
 *         artifact;
 *     }
 */


/*
 * ============================================================================
 * 55. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     empty deterministic body;
 *     one entry;
 *     many entries;
 *     deeply qualified property names;
 *     deeply qualified values;
 *     large expressions;
 *     large requirement sets;
 *     large dependency sets;
 *     large artifact sets;
 *     nested expression structures;
 *     Unicode identifiers;
 *     arbitrary numeric values;
 *     quantum resource expressions;
 *     distributed resource expressions;
 *     HDL resource expressions.
 *
 * The grammar must not introduce a finite domain-specific ceiling.
 */


/*
 * ============================================================================
 * 56. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexical configuration;
 *     grammar version;
 *
 * the parser MUST produce equivalent parse structures.
 *
 * The parse result must not depend on:
 *
 *     hardware;
 *     host architecture;
 *     environment;
 *     filesystem;
 *     network;
 *     wall-clock time;
 *     randomness.
 */


/*
 * ============================================================================
 * 57. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal constants include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Also prohibited as universal deterministic-build assumptions:
 *
 *     CPU_0
 *     GPU_0
 *     QPU_0
 *     FPGA_0
 *     NODE_0
 *     DEVICE_0
 *
 * Concrete values are allowed as program data or explicit requirements.
 *
 * For example:
 *
 *     requires memory >= 64GiB;
 *
 * is valid semantic intent.
 *
 * It does NOT establish:
 *
 *     MAX_MEMORY = 64GiB
 *
 * No such compiler-wide limit may be derived from the grammar.
 */


/*
 * ============================================================================
 * 58. RUST CONTRACT
 * ============================================================================
 *
 * The grammar contains no Rust implementation.
 *
 * Generated parser/semantic integration MUST support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Production compiler code MUST:
 *
 *     - use safe Rust;
 *     - contain no unsafe blocks;
 *     - require no unsafe implementation for deterministic-build parsing;
 *     - keep filesystem/network/process operations outside the parser.
 */


/*
 * ============================================================================
 * 59. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This grammar is intentionally parser-oriented.
 *
 * It does not:
 *
 *     enumerate dependencies;
 *     hash inputs;
 *     inspect files;
 *     inspect hardware;
 *     discover toolchains;
 *     calculate artifact identities;
 *     execute build steps.
 *
 * Potentially expensive operations belong downstream.
 *
 * Unbounded grammar repetition means no artificial language-level ceiling,
 * not unlimited host-process resource consumption.
 */


/*
 * ============================================================================
 * 60. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] deterministic build has one public compile-clause entry;
 * [x] no second `compile` declaration exists;
 * [x] deterministic build is distinct from reproducibility;
 * [x] deterministic build is distinct from runtime determinism;
 * [x] Core identifiers are reused;
 * [x] canonical expressions are reused;
 * [x] semantic categories remain distinct;
 * [x] inputs remain symbolic;
 * [x] dependencies remain symbolic;
 * [x] environment values remain symbolic;
 * [x] artifacts remain symbolic;
 * [x] ordering remains semantic intent;
 * [x] properties remain extensible;
 * [x] no hardware taxonomy is enumerated;
 * [x] no vendor/toolchain enumeration exists;
 * [x] no resource limits exist;
 * [x] no physical device selection exists;
 * [x] no filesystem access exists;
 * [x] no network access exists;
 * [x] no process execution exists;
 * [x] no linker execution exists;
 * [x] no IR is introduced;
 * [x] quantum::ir remains canonical;
 * [x] target selection remains downstream;
 * [x] cross-compilation remains separately owned;
 * [x] caching remains separately owned;
 * [x] provenance remains separately owned;
 * [x] reproducibility remains separately owned;
 * [x] safe-Rust requirement is explicit;
 * [x] Rust 1.97 / 1.97.1 compatibility is documented;
 * [x] positive tests are specified;
 * [x] negative tests are specified;
 * [x] boundary tests are specified;
 * [x] scalability tests are specified;
 * [x] determinism tests are specified;
 * [x] hard-coding audit is specified.
 *
 * ============================================================================
 * REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * This file is independently specified, but the following composition changes
 * are required for the repository to consume it.
 *
 * ---------------------------------------------------------------------------
 * 1. grammar/lexer/keywords.g4
 * ---------------------------------------------------------------------------
 *
 * Add exactly one canonical reserved word:
 *
 *     DETERMINISTIC : 'deterministic' ;
 *
 * It must not be duplicated elsewhere.
 *
 * ---------------------------------------------------------------------------
 * 2. grammar/compile/compile.g4
 * ---------------------------------------------------------------------------
 *
 * Add:
 *
 *     CompileDeterministicBuilds
 *
 * to the parser imports.
 *
 * Then add:
 *
 *     | deterministicBuildClause
 *
 * to:
 *
 *     compileClause
 *
 * The existing:
 *
 *     compileDeclaration
 *
 * remains the sole owner of `COMPILE`.
 *
 * ---------------------------------------------------------------------------
 * 3. grammar/compile/compilation.g4
 * ---------------------------------------------------------------------------
 *
 * Do NOT create another deterministic-build declaration here.
 *
 * `compilation.g4` should continue to consume the canonical `Compile`
 * composition boundary.
 *
 * If `Compilation` directly imports compilation-specific component grammars
 * in a future refactor, it may import `CompileDeterministicBuilds`, but it
 * must not create another public deterministic-build rule.
 *
 * ---------------------------------------------------------------------------
 * 4. grammar/compile/reproducibility.g4
 * ---------------------------------------------------------------------------
 *
 * No rule duplication is required.
 *
 * Reproducibility remains independent.
 *
 * ---------------------------------------------------------------------------
 * 5. grammar/compile/caching.g4
 * ---------------------------------------------------------------------------
 *
 * No rule duplication is required.
 *
 * Caching remains independent.
 *
 * ---------------------------------------------------------------------------
 * 6. grammar/compile/cross-compilation.g4
 * ---------------------------------------------------------------------------
 *
 * No rule duplication is required.
 *
 * Existing cross-compilation integration may reference:
 *
 *     deterministicBuildClause
 *
 * only through the canonical `compile` composition boundary.
 *
 * It must not redefine the deterministic-build grammar.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * The production relationship is:
 *
 *     compile deterministic
 *             |
 *             v
 *     DeterministicBuild AST
 *             |
 *             v
 *     semantic deterministic policy
 *             |
 *       +-----+------+----------------+
 *       |            |                |
 *       v            v                v
 *   inputs       dependencies     environment
 *       |            |                |
 *       +------------+----------------+
 *                    |
 *                    v
 *            compilation model
 *                    |
 *                    v
 *              canonical IR
 *                    |
 *             +------+------+
 *             |             |
 *             v             v
 *       classical      quantum::ir
 *             |             |
 *             +------+------+
 *                    |
 *                    v
 *              optimization
 *                    |
 *            lowering/routing
 *                    |
 *                scheduling
 *                    |
 *            resilience/QEC/ZQN
 *                    |
 *                   HAL
 *                    |
 *             target realization
 *
 * Deterministic-build syntax therefore constrains the BUILD PROCESS without
 * turning current hardware characteristics into language limitations.
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * while allowing the same source semantics to scale from very small systems
 * to arbitrarily large systems when the required resources and capabilities
 * are available.
 *
 * ============================================================================
 */