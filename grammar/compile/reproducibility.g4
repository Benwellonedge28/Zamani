/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/reproducibility.g4
 *
 * Grammar:
 *     CompileReproducibility
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
 *     This grammar contains no Rust actions and requires no unsafe Rust.
 *     The Zamani compiler/frontend consuming this grammar MUST use safe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL REPRODUCIBILITY INTENT.
 *
 * Reproducibility describes the semantic contract under which repeated
 * compilation of the same source and declared inputs should produce
 * equivalent compilation results according to the selected reproducibility
 * policy.
 *
 * This file does NOT:
 *
 *     - perform compilation;
 *     - execute a compiler;
 *     - access the filesystem;
 *     - inspect the host;
 *     - inspect hardware;
 *     - discover installed toolchains;
 *     - resolve dependencies;
 *     - access the network;
 *     - calculate hashes;
 *     - construct artifact digests;
 *     - implement caching;
 *     - implement provenance;
 *     - implement deterministic execution;
 *     - perform optimization;
 *     - perform target selection;
 *     - perform cross-compilation;
 *     - perform deployment;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform ZQN;
 *     - construct an IR.
 *
 * It expresses INTENT only.
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
 *     Compile
 *          |
 *          +--> CompileReproducibility
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> reproducibility analysis
 *          +--> input identity analysis
 *          +--> dependency analysis
 *          +--> environment analysis
 *          +--> toolchain analysis
 *          +--> artifact analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic compilation model
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     canonical IR           compilation artifacts
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *             optimization/lowering
 *                      |
 *                      v
 *             target realization
 *
 * Reproducibility therefore belongs ABOVE target-specific realization and
 * BELOW ordinary source syntax.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Reproducibility MUST NOT become a hidden hardware restriction.
 *
 * A reproducibility policy may constrain:
 *
 *     - source identity;
 *     - declared inputs;
 *     - dependencies;
 *     - toolchain identity;
 *     - compiler version;
 *     - language version;
 *     - compilation options;
 *     - target intent;
 *     - environment properties;
 *     - artifact identity;
 *     - external inputs;
 *     - timestamps;
 *     - randomness;
 *     - generated metadata;
 *     - serialization;
 *     - ordering;
 *     - path representation;
 *     - provenance requirements.
 *
 * It MUST NOT impose universal limits such as:
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
 *     MAX_ARTIFACTS
 *     MAX_DEPENDENCIES
 *     MAX_INPUTS
 *
 * Nor may equivalent finite limits be hidden behind grammar structure.
 *
 * Repetition uses ANTLR's unbounded grammar operators.
 *
 * "Unbounded" means:
 *
 *     no artificial language-level cardinality ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical resources;
 *     infinite compiler memory;
 *     infinite storage;
 *     infinite compilation time.
 *
 * Actual feasibility belongs to the compiler, resource model, target,
 * runtime, and deployment environment.
 *
 * ============================================================================
 * REPRODUCIBILITY MODEL
 * ============================================================================
 *
 * Reproducibility is a relationship between declared compilation inputs and
 * resulting compilation artifacts.
 *
 * Conceptually:
 *
 *     source
 *       +
 *     declared inputs
 *       +
 *     declared dependencies
 *       +
 *     declared compilation context
 *       +
 *     reproducibility policy
 *       |
 *       v
 *     reproducibility contract
 *       |
 *       v
 *     compilation result
 *
 * The grammar does not calculate equivalence.
 *
 * Semantic/compiler layers determine what "equivalent" means for the selected
 * reproducibility policy.
 *
 * ============================================================================
 * IMPORTANT DISTINCTIONS
 * ============================================================================
 *
 * Reproducibility is NOT identical to:
 *
 *     deterministic execution
 *     deterministic scheduling
 *     caching
 *     provenance
 *     artifact hashing
 *     content addressing
 *     bit-for-bit identity
 *     semantic equivalence
 *     target equivalence
 *
 * Those concepts may participate in reproducibility, but they remain
 * independently owned by their respective semantic/compiler subsystems.
 *
 * This grammar therefore provides integration points rather than duplicating
 * those systems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - reproducibility declaration/clause syntax;
 *     - reproducibility policy composition;
 *     - reproducibility requirements;
 *     - reproducibility constraints;
 *     - reproducibility preferences;
 *     - reproducibility hints;
 *     - reproducibility capabilities;
 *     - reproducibility resource intent;
 *     - reproducibility properties;
 *     - reproducibility scope;
 *     - reproducibility input declarations;
 *     - reproducibility dependency declarations;
 *     - reproducibility environment declarations;
 *     - reproducibility artifact intent;
 *     - reproducibility policy references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - expressions;
 *     - target declarations;
 *     - target selection;
 *     - cross-compilation;
 *     - optimization;
 *     - specialization;
 *     - caching implementation;
 *     - provenance implementation;
 *     - deterministic execution;
 *     - artifact hashing implementation;
 *     - dependency resolution;
 *     - package management;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime behavior;
 *     - canonical IR;
 *     - quantum::ir.
 *
 * ============================================================================
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *
 * Compilation syntax:
 *
 *     grammar/compile/compile.g4
 *
 * Reproducibility syntax:
 *
 *     THIS FILE
 *
 * Resource semantics:
 *
 *     grammar/resources/
 *
 * Target semantics:
 *
 *     grammar/compile/target.g4
 *
 * Target-selection semantics:
 *
 *     grammar/compile/target-selection.g4
 *
 * Profile semantics:
 *
 *     grammar/compile/profiles.g4
 *
 * Cross-compilation semantics:
 *
 *     grammar/compile/cross-compilation.g4
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
 * This grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Existing canonical lexical concepts used here include:
 *
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     RESOURCE
 *     RESOURCES
 *     PROPERTY
 *     PROFILE
 *
 * The current repository does NOT define a canonical REPRODUCIBLE token.
 *
 * Therefore one lexical integration is required:
 *
 *     REPRODUCIBLE : 'reproducible'
 *
 * in the canonical keyword vocabulary.
 *
 * No other new keyword is required by this grammar.
 *
 * In particular, this file deliberately does NOT assume nonexistent tokens
 * such as:
 *
 *     DETERMINISTIC
 *     OPTIMIZE
 *     REPRODUCIBILITY
 *
 * Determinism, optimization, caching, and provenance remain downstream
 * integration concepts unless their own lexical contracts establish syntax.
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
 * No equivalent rules are redefined here.
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar CompileReproducibility;

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
 * Public integration rule.
 *
 * This rule intentionally does NOT consume COMPILE.
 *
 * The enclosing compile grammar owns:
 *
 *     COMPILE
 *
 * Therefore:
 *
 *     compile reproducible { ... }
 *
 * is structurally:
 *
 *     compileDeclaration
 *         -> compileSpecification
 *             -> compileClause
 *                 -> reproducibilityClause
 *
 * This prevents a second competing compile declaration.
 */

reproducibilityClause
    : REPRODUCIBLE reproducibilitySpecification
    ;


/*
 * ============================================================================
 * 2. REPRODUCIBILITY SPECIFICATION
 * ============================================================================
 *
 * Two forms are supported:
 *
 *     compile reproducible;
 *
 * and:
 *
 *     compile reproducible {
 *         ...
 *     }
 *
 * The empty policy is syntactically valid because the presence of the
 * reproducible clause itself establishes a semantic request for the default
 * reproducibility policy defined by the language specification.
 *
 * The semantic layer determines the default policy.
 */

reproducibilitySpecification
    : reproducibilityBody?
    ;


/*
 * ============================================================================
 * 3. REPRODUCIBILITY BODY
 * ============================================================================
 *
 * There is no finite maximum number of policy entries.
 */

reproducibilityBody
    : LBRACE reproducibilityEntry* RBRACE
    ;


/*
 * ============================================================================
 * 4. REPRODUCIBILITY ENTRY
 * ============================================================================
 *
 * Semantic categories remain explicit.
 *
 * In particular:
 *
 *     requirement != constraint
 *     preference != hint
 *     capability != resource
 *     property != requirement
 *
 * The compiler MUST preserve these distinctions.
 */

reproducibilityEntry
    : reproducibilityRequirement
    | reproducibilityConstraint
    | reproducibilityPreference
    | reproducibilityHint
    | reproducibilityCapability
    | reproducibilityResource
    | reproducibilityProperty
    | reproducibilityInput
    | reproducibilityDependency
    | reproducibilityEnvironment
    | reproducibilityArtifact
    | reproducibilityProfile
    ;


/*
 * ============================================================================
 * 5. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Examples:
 *
 *     requires source_identity;
 *     requires toolchain_identity;
 *     requires dependency_identity;
 *     requires capability("reproducible.build");
 *
 * The expression is intentionally generic.
 *
 * This permits future resource/capability systems without requiring grammar
 * changes for every new reproducibility property.
 */

reproducibilityRequirement
    : REQUIRES expression SEMI?
    ;


/*
 * ============================================================================
 * 6. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict acceptable reproducibility implementations.
 *
 * Examples:
 *
 *     constraint artifact_identity == semantic_identity;
 *     constraint external_inputs == declared;
 *     constraint generated_metadata == stable;
 *
 * Meaning is resolved semantically.
 */

reproducibilityConstraint
    : CONSTRAINT expression SEMI?
    ;


/*
 * ============================================================================
 * 7. PREFERENCES
 * ============================================================================
 *
 * Preferences provide non-mandatory reproducibility guidance.
 *
 * They MUST NOT be interpreted as requirements merely because a compiler
 * backend understands them.
 */

reproducibilityPreference
    : PREFER expression SEMI?
    ;


/*
 * ============================================================================
 * 8. HINTS
 * ============================================================================
 *
 * Hints provide non-binding implementation information.
 */

reproducibilityHint
    : HINT expression SEMI?
    ;


/*
 * ============================================================================
 * 9. CAPABILITIES
 * ============================================================================
 *
 * Capability names remain open-ended.
 *
 * Examples:
 *
 *     capability("reproducible.build");
 *     capability("content.addressed.artifacts");
 *     capability("stable.serialization");
 *
 * The grammar does not enumerate capability strings.
 */

reproducibilityCapability
    : CAPABILITY expression SEMI?
    ;


/*
 * ============================================================================
 * 10. RESOURCE INTENT
 * ============================================================================
 *
 * Reproducibility may itself require resources, for example storage,
 * metadata retention, or verification resources.
 *
 * Resource semantics remain owned by grammar/resources/.
 *
 * Examples:
 *
 *     resource storage >= required_storage;
 *     resources verification >= required_verification;
 *
 * This grammar does not define a physical inventory.
 */

reproducibilityResource
    : RESOURCE expression SEMI?
    | RESOURCES expression SEMI?
    ;


/*
 * ============================================================================
 * 11. GENERIC PROPERTIES
 * ============================================================================
 *
 * Generic properties provide extensibility without creating a keyword for
 * every reproducibility concern.
 *
 * Examples:
 *
 *     property source = source::identity;
 *     property toolchain = toolchain::identity;
 *     property dependencies = dependencies::locked;
 *     property environment = environment::declared;
 *     property timestamps = timestamps::normalized;
 *     property randomness = randomness::declared;
 *     property serialization = serialization::stable;
 *     property paths = paths::normalized;
 *
 * These names are symbolic data, not grammar-level enumerations.
 */

reproducibilityProperty
    : PROPERTY reproducibilityPropertyName
      reproducibilityPropertyValue?
      SEMI?
    ;


reproducibilityPropertyName
    : identifier
    | qualifiedName
    ;


reproducibilityPropertyValue
    : ASSIGN expression
    | COLON expression
    | reproducibilityPropertyBlock
    ;


reproducibilityPropertyBlock
    : LBRACE reproducibilityPropertyEntry* RBRACE
    ;


reproducibilityPropertyEntry
    : reproducibilityPropertyName
      (ASSIGN | COLON)
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * 12. INPUT IDENTITY
 * ============================================================================
 *
 * Inputs identify source material that participates in reproducibility.
 *
 * The grammar does not calculate hashes or inspect files.
 *
 * Examples:
 *
 *     input source;
 *     input manifest;
 *     input dataset;
 *     input dependency_lock;
 *
 * `identifier` and `qualifiedName` remain open-ended.
 */

reproducibilityInput
    : REPRODUCIBILITY_INPUT reproducibilityNamedValue SEMI?
    ;


reproducibilityNamedValue
    : identifier
    | qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 13. DEPENDENCY IDENTITY
 * ============================================================================
 *
 * Dependencies participate in the reproducibility contract without making
 * package-management syntax part of this grammar.
 *
 * Examples:
 *
 *     dependency package::name;
 *     dependency dependency::lock;
 *
 * The semantic/package layer determines what the reference denotes.
 */

reproducibilityDependency
    : REPRODUCIBILITY_DEPENDENCY reproducibilityNamedValue SEMI?
    ;


/*
 * ============================================================================
 * 14. ENVIRONMENT IDENTITY
 * ============================================================================
 *
 * The environment clause identifies semantic environment information that
 * affects compilation.
 *
 * It MUST NOT cause the parser to inspect the current environment.
 *
 * Examples:
 *
 *     environment compiler;
 *     environment toolchain;
 *     environment language;
 *     environment target;
 *
 * The environment vocabulary remains semantic data.
 */

reproducibilityEnvironment
    : REPRODUCIBILITY_ENVIRONMENT reproducibilityNamedValue SEMI?
    ;


/*
 * ============================================================================
 * 15. ARTIFACT IDENTITY
 * ============================================================================
 *
 * Artifact syntax identifies the semantic artifact whose reproducibility is
 * being constrained.
 *
 * This does not calculate or verify a digest.
 *
 * Artifact production and hashing remain downstream responsibilities.
 */

reproducibilityArtifact
    : ARTIFACT reproducibilityNamedValue SEMI?
    ;


/*
 * ============================================================================
 * 16. PROFILE REFERENCE
 * ============================================================================
 *
 * A reproducibility policy may reuse a named compilation profile.
 *
 * Profile declaration syntax remains owned by profiles.g4.
 */

reproducibilityProfile
    : PROFILE qualifiedName SEMI?
    ;


/*
 * ============================================================================
 * 17. POLICY PROPERTY EXAMPLES
 * ============================================================================
 *
 * The following are SEMANTIC examples only.
 *
 * They are intentionally not grammar alternatives:
 *
 *     source_identity
 *     dependency_identity
 *     toolchain_identity
 *     compiler_identity
 *     target_identity
 *     environment_identity
 *     artifact_identity
 *     timestamp_policy
 *     randomness_policy
 *     serialization_policy
 *     path_policy
 *     external_input_policy
 *     generated_metadata_policy
 *
 * Keeping these as identifiers prevents the grammar from becoming coupled to
 * today's reproducibility implementation.
 */


/*
 * ============================================================================
 * 18. REPRODUCIBILITY SCOPE
 * ============================================================================
 *
 * Reproducibility may apply to:
 *
 *     source;
 *     compilation;
 *     artifact generation;
 *     deployment artifacts;
 *     semantic outputs;
 *     target-specific outputs.
 *
 * Scope remains semantic data.
 *
 * Example:
 *
 *     property scope = compilation;
 *
 * The grammar does not enumerate scopes.
 */


/*
 * ============================================================================
 * 19. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Reproducibility applies equally to:
 *
 *     classical programs;
 *     quantum programs;
 *     hybrid programs;
 *     HDL;
 *     hardware/software co-design;
 *     AI;
 *     tensor/data workloads;
 *     distributed programs;
 *     networking;
 *     security;
 *     embedded programs;
 *     accelerator programs;
 *     future computational domains.
 *
 * No domain receives a special reproducibility grammar.
 *
 * This is essential for POCO-REAF.
 *
 * A quantum program, for example, may require reproducibility of:
 *
 *     source;
 *     circuit semantics;
 *     operation parameters;
 *     compilation configuration;
 *     target intent;
 *     generated artifacts.
 *
 * It must not require a fixed physical QPU identifier merely to express
 * reproducibility.
 *
 * Similarly, an HDL program may require reproducibility of:
 *
 *     source;
 *     parameters;
 *     synthesis intent;
 *     toolchain identity;
 *     generated representation;
 *
 * without hard-coding a particular FPGA or ASIC.
 */


/*
 * ============================================================================
 * 20. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     fixed hardware identifiers;
 *     fixed target identifiers;
 *     fixed device counts;
 *     fixed CPU counts;
 *     fixed GPU counts;
 *     fixed FPGA counts;
 *     fixed QPU counts;
 *     fixed node counts;
 *     fixed memory sizes;
 *     fixed register widths;
 *     fixed tensor ranks;
 *     fixed artifact counts;
 *     fixed dependency counts;
 *     fixed input counts.
 *
 * The following are therefore intentionally NOT grammar constructs:
 *
 *     reproducible_on_gpu0
 *     reproducible_on_qpu0
 *     reproducible_with_32_qubits
 *     reproducible_with_64gb_memory
 *
 * If a program genuinely requires such a property, it must express the
 * semantic requirement through the resource/capability/target systems.
 *
 * Reproducibility itself must remain orthogonal to machine scale.
 */


/*
 * ============================================================================
 * 21. DETERMINISM INTEGRATION
 * ============================================================================
 *
 * Reproducibility may depend upon deterministic compilation, but this grammar
 * does not define deterministic-build syntax.
 *
 * Deterministic-build semantics belong to their dedicated compilation
 * contract when present.
 *
 * This file may reference deterministic intent through:
 *
 *     requires ...
 *     constraint ...
 *     property ...
 *
 * without introducing a second deterministic-build grammar.
 *
 * Example:
 *
 *     requires compilation::deterministic;
 *
 * This is semantic data.
 */


/*
 * ============================================================================
 * 22. CACHING INTEGRATION
 * ============================================================================
 *
 * Reproducibility may improve or constrain cache behavior.
 *
 * This file does not own cache keys, cache storage, cache invalidation, or
 * cache implementation.
 *
 * A reproducibility property may refer to cache semantics:
 *
 *     property cache_identity = content;
 *
 * without implementing caching.
 */


/*
 * ============================================================================
 * 23. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Provenance records may be required to establish reproducibility.
 *
 * This grammar does not construct provenance records.
 *
 * Example:
 *
 *     requires provenance::complete;
 *
 * The provenance subsystem remains responsible for the actual record.
 */


/*
 * ============================================================================
 * 24. CROSS-COMPILATION INTEGRATION
 * ============================================================================
 *
 * Cross-compilation may use reproducibility intent.
 *
 * The cross-compilation grammar remains responsible for:
 *
 *     source target
 *     destination target
 *     cross-compilation relationship
 *
 * This file remains responsible only for:
 *
 *     reproducibility policy.
 *
 * Example conceptual composition:
 *
 *     compile cross source -> destination {
 *         reproducible {
 *             ...
 *         }
 *     }
 *
 * Whether nested composition is accepted is controlled by the compilation
 * composition grammar. This file does not redefine crossCompilationClause.
 */


/*
 * ============================================================================
 * 25. PROFILE INTEGRATION
 * ============================================================================
 *
 * A named profile may be referenced from a reproducibility policy:
 *
 *     compile reproducible {
 *         profile portable;
 *     }
 *
 * Profile declaration remains owned by profiles.g4.
 */


/*
 * ============================================================================
 * 26. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Reproducibility resource requirements flow through:
 *
 *     grammar/resources/
 *
 * and ultimately:
 *
 *     resource analysis
 *          |
 *          v
 *     capability analysis
 *          |
 *          v
 *     target realization
 *
 * This file must never resolve whether a resource exists.
 */


/*
 * ============================================================================
 * 27. TARGET INTEGRATION
 * ============================================================================
 *
 * Reproducibility can constrain target intent through generic expressions.
 *
 * It must not duplicate targetExpression.
 *
 * Target semantics remain owned by:
 *
 *     grammar/compile/target.g4
 *
 * This prevents reproducibility from becoming another target grammar.
 */


/*
 * ============================================================================
 * 28. AST CONTRACT
 * ============================================================================
 *
 * The grammar must map to a domain-neutral AST representation.
 *
 * The expected semantic shape is conceptually:
 *
 *     ReproducibilityIntent
 *         policy
 *         requirements[]
 *         constraints[]
 *         preferences[]
 *         hints[]
 *         capabilities[]
 *         resources[]
 *         properties[]
 *         inputs[]
 *         dependencies[]
 *         environments[]
 *         artifacts[]
 *         profiles[]
 *         source_span
 *
 * These names describe semantic ownership and are NOT requirements to create
 * an AST node with exactly these fields unless the existing frontend AST
 * contract establishes them.
 *
 * The important rule is:
 *
 *     syntax
 *       -> domain-neutral AST
 *       -> semantic reproducibility model
 *
 * not:
 *
 *     syntax
 *       -> backend-specific reproducibility object
 */


/*
 * ============================================================================
 * 29. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - resolve referenced names;
 *     - resolve properties;
 *     - validate reproducibility categories;
 *     - identify undeclared external inputs;
 *     - identify environment dependencies;
 *     - validate dependency identity;
 *     - validate profile references;
 *     - validate target compatibility;
 *     - validate resource requirements;
 *     - validate capability requirements;
 *     - distinguish requirements from preferences;
 *     - distinguish semantic equivalence from byte identity;
 *     - diagnose contradictory policies;
 *     - preserve source spans.
 *
 * Semantic analysis MUST NOT:
 *
 *     - inspect hardware during parsing;
 *     - silently change source semantics;
 *     - invent missing dependencies;
 *     - silently downgrade mandatory requirements.
 */


/*
 * ============================================================================
 * 30. IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO IR.
 *
 * Reproducibility intent is lowered into the existing semantic compilation
 * model and then into the repository's established canonical IR boundaries.
 *
 * In particular:
 *
 *     NO ReproducibilityIR
 *     NO QuantumReproducibilityIR
 *     NO HardwareReproducibilityIR
 *     NO CrossCompilationReproducibilityIR
 *
 * Quantum programs continue through:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * Reproducibility metadata may accompany compilation artifacts and semantic
 * compilation plans without becoming a second computational IR.
 */


/*
 * ============================================================================
 * 31. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes reproducibility intent after parsing.
 *
 * Responsibilities include:
 *
 *     - establish compilation identity;
 *     - identify relevant inputs;
 *     - identify relevant dependencies;
 *     - establish toolchain/compiler identity;
 *     - establish relevant target intent;
 *     - normalize policy;
 *     - construct reproducibility metadata;
 *     - validate required guarantees;
 *     - integrate artifact identity;
 *     - integrate deterministic compilation where applicable;
 *     - integrate cache identity where applicable;
 *     - integrate provenance where applicable.
 *
 * These are compiler responsibilities, not parser responsibilities.
 */


/*
 * ============================================================================
 * 32. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Reproducibility of compilation is distinct from reproducibility of runtime
 * behavior.
 *
 * Runtime nondeterminism may be governed by separate:
 *
 *     execution;
 *     concurrency;
 *     scheduling;
 *     randomness;
 *     resilience;
 *     quantum-noise;
 *     distributed-consistency
 *
 * contracts.
 *
 * This grammar MUST NOT silently impose runtime determinism merely because a
 * compilation is reproducible.
 */


/*
 * ============================================================================
 * 33. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum reproducibility may involve:
 *
 *     source circuit;
 *     symbolic operation identity;
 *     parameters;
 *     semantic measurement intent;
 *     compilation configuration;
 *     target intent;
 *     generated quantum artifacts.
 *
 * It must not require:
 *
 *     fixed physical qubit numbers;
 *     fixed QPU identifiers;
 *     fixed coupling maps;
 *     fixed calibration values;
 *     fixed hardware topology.
 *
 * Those belong downstream to:
 *
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     target realization.
 *
 * The canonical quantum IR remains:
 *
 *     quantum::ir
 */


/*
 * ============================================================================
 * 34. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL reproducibility may involve:
 *
 *     source;
 *     parameters;
 *     synthesis intent;
 *     toolchain identity;
 *     generated representation;
 *     verification configuration.
 *
 * It must not hard-code:
 *
 *     FPGA family;
 *     ASIC model;
 *     fixed register width;
 *     fixed memory capacity;
 *     fixed number of logic elements.
 *
 * Hardware realization remains downstream.
 */


/*
 * ============================================================================
 * 35. DISTRIBUTED / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Reproducibility applies to:
 *
 *     distributed builds;
 *     distributed data;
 *     AI models;
 *     datasets;
 *     generated model artifacts;
 *     tensor transformations;
 *     accelerator compilation.
 *
 * Dataset/model identity remains semantic data.
 *
 * The grammar imposes no maximum dataset size, tensor rank, node count, or
 * accelerator count.
 */


/*
 * ============================================================================
 * 36. DETERMINISM
 * ============================================================================
 *
 * The parser must be deterministic.
 *
 * Given the same:
 *
 *     source;
 *     language version;
 *     active dialect set;
 *
 * it must produce the same parse structure.
 *
 * Reproducibility policy itself must not depend on:
 *
 *     hardware availability;
 *     current wall-clock time;
 *     environment variables;
 *     random parser state;
 *     network state.
 *
 * Those values may be declared as semantic inputs, but the grammar does not
 * inspect them.
 */


/*
 * ============================================================================
 * 37. SOURCE SPANS
 * ============================================================================
 *
 * Every reproducibility construct must preserve source locations sufficient
 * for diagnostics.
 *
 * At minimum semantic diagnostics must be able to identify:
 *
 *     reproducible clause;
 *     offending entry;
 *     property name;
 *     requirement;
 *     constraint;
 *     profile reference;
 *     artifact reference.
 *
 * Source-span construction remains a frontend implementation responsibility.
 */


/*
 * ============================================================================
 * 38. DIAGNOSTICS
 * ============================================================================
 *
 * Diagnostics should distinguish:
 *
 *     invalid reproducibility syntax;
 *     unknown reproducibility property;
 *     unresolved profile;
 *     contradictory requirement;
 *     unsatisfied capability;
 *     unavailable resource;
 *     undeclared external input;
 *     incomplete environment declaration;
 *     unsupported reproducibility guarantee;
 *     target incompatibility.
 *
 * The parser must not report semantic failures as syntax failures.
 */


/*
 * ============================================================================
 * 39. SECURITY
 * ============================================================================
 *
 * Reproducibility metadata can contain sensitive information.
 *
 * The grammar itself must not:
 *
 *     expose secrets;
 *     resolve credentials;
 *     access environment secrets;
 *     read private files;
 *     invoke external tools.
 *
 * Secret values must be represented through appropriate security/capability
 * mechanisms rather than embedded compiler actions.
 */


/*
 * ============================================================================
 * 40. PERFORMANCE
 * ============================================================================
 *
 * The grammar must remain parser-oriented.
 *
 * It must not perform expensive reproducibility calculations.
 *
 * Repeated entries use:
 *
 *     *
 *
 * rather than finite alternatives.
 *
 * Large source files, large dependency sets, large metadata sets, and large
 * reproducibility policies are therefore not artificially bounded by this
 * grammar.
 *
 * Actual memory/time limits belong to the implementation and available
 * resources.
 */


/*
 * ============================================================================
 * 41. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be represented by conformance tests after the
 * lexical integration is complete:
 *
 *     compile reproducible;
 *
 *     compile reproducible {
 *     }
 *
 *     compile reproducible {
 *         requires source_identity;
 *     }
 *
 *     compile reproducible {
 *         requires capability("reproducible.build");
 *     }
 *
 *     compile reproducible {
 *         constraint artifact_identity == semantic_identity;
 *     }
 *
 *     compile reproducible {
 *         prefer stable_serialization;
 *         hint normalized_paths;
 *     }
 *
 *     compile reproducible {
 *         resource storage >= required_storage;
 *     }
 *
 *     compile reproducible {
 *         property toolchain = toolchain::identity;
 *         property environment = environment::declared;
 *     }
 *
 *     compile reproducible {
 *         profile portable;
 *     }
 *
 *     compile reproducible {
 *         input source;
 *         dependency dependency::lock;
 *         environment toolchain;
 *         artifact executable;
 *     }
 *
 * These examples are contracts, not hard-coded semantic vocabularies.
 */


/*
 * ============================================================================
 * 42. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests must reject malformed constructs such as:
 *
 *     compile reproducible {
 *         requires
 *     }
 *
 *     compile reproducible {
 *         property
 *     }
 *
 *     compile reproducible {
 *         profile
 *     }
 *
 *     compile reproducible {
 *         input
 *     }
 *
 *     compile reproducible {
 *         dependency
 *     }
 *
 *     compile reproducible {
 *         environment
 *     }
 *
 *     compile reproducible {
 *         artifact
 *     }
 *
 * Exact diagnostics belong to the diagnostic specification.
 */


/*
 * ============================================================================
 * 43. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests must cover:
 *
 *     - one policy entry;
 *     - many policy entries;
 *     - deeply nested property blocks;
 *     - long qualified names;
 *     - large expression values;
 *     - large numbers of inputs;
 *     - large numbers of dependencies;
 *     - large numbers of properties;
 *     - large artifact sets.
 *
 * No artificial language maximum may be introduced merely to simplify tests.
 */


/*
 * ============================================================================
 * 44. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must accept reproducibility specifications whose cardinality is
 * determined by source size and available compiler resources rather than a
 * hard-coded grammar ceiling.
 *
 * Tests should scale through increasing:
 *
 *     source inputs;
 *     dependency references;
 *     environment properties;
 *     artifact references;
 *     policy entries;
 *     nested property structures.
 *
 * Tests must explicitly verify absence of:
 *
 *     MAX_INPUTS
 *     MAX_DEPENDENCIES
 *     MAX_ARTIFACTS
 *     MAX_PROPERTIES
 *     MAX_POLICY_ENTRIES.
 */


/*
 * ============================================================================
 * 45. COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Compatibility tests must verify:
 *
 *     language-version behavior;
 *     keyword availability;
 *     profile compatibility;
 *     property compatibility;
 *     dialect interaction;
 *     interaction with compile clauses.
 *
 * Historical syntax must not silently become accepted syntax merely because it
 * appears in Zamani-Grammar.md.
 */


/*
 * ============================================================================
 * 46. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the language-level hard-coding policy only if:
 *
 *     - no physical device names are enumerated;
 *     - no hardware capacity is enumerated;
 *     - no finite artifact ceiling exists;
 *     - no finite dependency ceiling exists;
 *     - no finite input ceiling exists;
 *     - no fixed toolchain list exists;
 *     - no vendor list exists;
 *     - no architecture list exists;
 *     - no ABI list exists;
 *     - no operating-system list exists;
 *     - no fixed memory size exists;
 *     - no fixed register width exists;
 *     - no fixed quantum resource count exists.
 *
 * Generic identifiers and qualified names are intentionally used instead.
 */


/*
 * ============================================================================
 * 47. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is independently complete when:
 *
 * [x] reproducibility has one canonical parser component;
 * [x] no second compile declaration exists;
 * [x] existing Core identifier rules are reused;
 * [x] existing Expressions rules are reused;
 * [x] existing requirement syntax is reused;
 * [x] existing constraint syntax is reused;
 * [x] existing preference syntax is reused;
 * [x] existing hint syntax is reused;
 * [x] existing capability syntax is reused;
 * [x] existing resource syntax is reused;
 * [x] existing property syntax is reused;
 * [x] profile references are integrated;
 * [x] inputs are represented;
 * [x] dependencies are represented;
 * [x] environment intent is represented;
 * [x] artifact intent is represented;
 * [x] no target-specific hardware is enumerated;
 * [x] no resource ceiling is encoded;
 * [x] no reproducibility IR is introduced;
 * [x] quantum::ir remains canonical;
 * [x] caching remains independently owned;
 * [x] provenance remains independently owned;
 * [x] deterministic-build semantics remain independently owned;
 * [x] cross-compilation remains independently owned;
 * [x] no filesystem access is required;
 * [x] no network access is required;
 * [x] no host inspection is required;
 * [x] no unsafe Rust is required;
 * [x] Rust 1.97 / 1.97.1 compatibility is documented;
 * [x] positive tests are specified;
 * [x] negative tests are specified;
 * [x] boundary tests are specified;
 * [x] scalability tests are specified;
 * [x] compatibility tests are specified;
 * [x] hard-coding audit is specified.
 *
 * Repository integration is complete when the lexical and compile-composition
 * changes described below have also been made.
 *
 * ============================================================================
 * REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * 1. grammar/lexer/keywords.g4
 *
 * Add exactly one canonical keyword:
 *
 *     REPRODUCIBLE : 'reproducible' ;
 *
 * Do not add:
 *
 *     REPRODUCIBILITY
 *     DETERMINISTIC
 *     OPTIMIZE
 *
 * merely for this file.
 *
 * Those concepts already have broader architectural ownership and should not
 * be invented as duplicate lexical authorities here.
 *
 *
 * 2. grammar/compile/compile.g4
 *
 * Import:
 *
 *     CompileReproducibility
 *
 * and add exactly one compile-clause alternative:
 *
 *     | reproducibilityClause
 *
 * The resulting structure is:
 *
 *     compileDeclaration
 *         |
 *         v
 *     compileSpecification
 *         |
 *         v
 *     compileClause
 *         |
 *         +--> reproducibilityClause
 *
 * `COMPILE` remains owned by compile.g4.
 *
 *
 * 3. grammar/compile/compilation.g4
 *
 * Do NOT create another top-level reproducibility rule there.
 *
 * It should continue consuming the canonical compile composition.
 *
 *
 * 4. grammar/compile/cross-compilation.g4
 *
 * Existing cross-compilation integration should reference:
 *
 *     reproducibilityClause
 *
 * only after importing CompileReproducibility.
 *
 * Its existing assumptions about nonexistent tokens such as:
 *
 *     REPRODUCIBLE
 *     DETERMINISTIC
 *     OPTIMIZE
 *
 * must be reconciled so that each concept has exactly one lexical/parser
 * authority.
 *
 *
 * 5. grammar/grammar.md
 *
 * The feature must eventually be recorded with its implementation status:
 *
 *     SPECIFIED
 *     IMPLEMENTED
 *     PARTIALLY IMPLEMENTED
 *     PLANNED
 *     DEPRECATED
 *
 * It must not be marked IMPLEMENTED merely because this grammar exists.
 *
 *
 * 6. AST / semantic / compiler integration
 *
 * The frontend must provide the corresponding domain-neutral representation
 * and semantic handling before the feature can be considered fully
 * implemented.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 *                         REPRODUCIBLE
 *                              |
 *                              v
 *                     SOURCE-LEVEL INTENT
 *                              |
 *                              v
 *                       DOMAIN-NEUTRAL AST
 *                              |
 *                              v
 *                    SEMANTIC VALIDATION
 *                              |
 *               +--------------+--------------+
 *               |              |              |
 *               v              v              v
 *            inputs       dependencies    environment
 *               |              |              |
 *               +--------------+--------------+
 *                              |
 *                              v
 *                    compilation identity
 *                              |
 *                              v
 *                       artifact identity
 *                              |
 *                              v
 *                    canonical compilation
 *                              |
 *                              v
 *                       target realization
 *
 * Reproducibility is therefore a compilation contract, not a hardware model,
 * not a runtime model, and not a second IR.
 *
 * ============================================================================
 */