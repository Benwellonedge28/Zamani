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
 *     Rust 1.97+
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     This file contains ANTLR4 grammar only.
 *     It contains no embedded Rust, target-language actions, semantic
 *     predicates, filesystem access, network access, hardware access, or
 *     runtime execution.
 *
 *     The Rust implementation consuming this grammar MUST use safe Rust.
 *     No unsafe Rust is required or permitted by the language architecture.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns SOURCE-LEVEL REPRODUCIBILITY INTENT for compilation.
 *
 * Reproducibility expresses the conditions under which compilation inputs,
 * declared compilation context, and resulting logical compilation artifacts
 * are expected to remain reproducible according to semantic/compiler policy.
 *
 * Reproducibility is a SOURCE CONTRACT.
 *
 * It is not a compiler implementation.
 *
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *     reproducibilityDeclaration
 *     reproducibilitySpecification
 *     reproducibilityBody
 *     reproducibilityEntry
 *
 * and the syntax for:
 *
 *     - reproducibility requirements;
 *     - reproducibility constraints;
 *     - reproducibility preferences;
 *     - reproducibility hints;
 *     - reproducibility capability requirements;
 *     - reproducibility resource intent;
 *     - reproducibility input declarations;
 *     - reproducibility profile references;
 *     - reproducibility properties;
 *     - reproducibility property blocks.
 *
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     - lexical vocabulary;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - types;
 *     - effects;
 *     - general resource semantics;
 *     - general capability semantics;
 *     - policies;
 *     - contracts;
 *     - deterministic-build implementation;
 *     - deterministic execution;
 *     - caching;
 *     - artifact hashing;
 *     - cryptography;
 *     - provenance records;
 *     - dependency resolution;
 *     - package management;
 *     - compiler execution;
 *     - optimization;
 *     - specialization;
 *     - lowering;
 *     - code generation;
 *     - target selection;
 *     - cross-compilation;
 *     - routing;
 *     - scheduling;
 *     - deployment;
 *     - runtime execution;
 *     - hardware discovery;
 *     - hardware allocation;
 *     - quantum::ir;
 *     - HDL realization;
 *     - any physical machine capacity.
 *
 *
 * PUBLIC RULES
 * ------------
 *
 *     reproducibilityDeclaration
 *
 * `compile.g4` consumes this rule.
 *
 * This file intentionally does not consume COMPILE.
 *
 *
 * PRIVATE RULES
 * -------------
 *
 * All other rules are internal implementation rules of this grammar unless
 * another grammar explicitly consumes them through a documented composition
 * boundary.
 *
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/lexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/core/core.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * EXPORTS
 * -------
 *
 *     reproducibilityDeclaration
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/compile/compile.g4
 *
 *
 * AST_OWNER
 * ---------
 *
 * The frontend AST/semantic compilation model owns the concrete AST
 * representation.
 *
 * This grammar MUST NOT define AST classes or embed Rust actions.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * The compiler semantic-analysis / compilation-policy subsystem owns:
 *
 *     - reproducibility interpretation;
 *     - policy resolution;
 *     - equivalence definition;
 *     - input identity;
 *     - dependency identity;
 *     - environment identity;
 *     - artifact identity;
 *     - satisfiability;
 *     - conflict detection;
 *     - reproducibility guarantees.
 *
 *
 * IR_OWNER
 * --------
 *
 * No reproducibility-specific IR is created by this grammar.
 *
 * Reproducibility information is attached to the canonical semantic
 * compilation representation and/or canonical IR metadata by downstream
 * compiler infrastructure.
 *
 * Quantum programs remain connected to:
 *
 *     quantum::ir
 *
 * without creating a reproducibility-specific quantum IR.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/compile/
 *
 * with reproducibility-specific conformance tests beneath that hierarchy.
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * The normative reproducibility specification MUST define semantic meaning.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The complete architecture is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> contract analysis
 *       +--> policy analysis
 *       +--> provenance analysis
 *       +--> reproducibility analysis
 *       |
 *       v
 *     semantic compilation model
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representations
 *       +--> other domain IRs
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / recovery / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * Reproducibility participates in semantic compilation analysis.
 *
 * It does not move below target realization and does not replace any
 * downstream compilation subsystem.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Reproducibility MUST preserve source portability.
 *
 * A reproducibility declaration MUST NOT encode a universal physical machine
 * model.
 *
 * It MUST NOT impose language-level limits on:
 *
 *     quantum resources
 *     processors
 *     cores
 *     threads
 *     accelerators
 *     devices
 *     nodes
 *     memory
 *     storage
 *     registers
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     circuit size
 *     artifact count
 *     dependency count
 *     input count
 *
 * There is no artificial finite capacity encoded by this grammar.
 *
 * Repeated grammar constructs use ANTLR repetition operators such as:
 *
 *     *
 *     +
 *
 * rather than fixed cardinalities.
 *
 * "Unbounded" here means:
 *
 *     no artificial language-level ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical resources;
 *     infinite compiler resources;
 *     infinite memory;
 *     infinite storage;
 *     infinite execution time.
 *
 * Physical feasibility remains a downstream resource/capability question.
 *
 * ============================================================================
 * REPRODUCIBILITY MODEL
 * ============================================================================
 *
 * The semantic model is conceptually:
 *
 *     source identity
 *          +
 *     declared inputs
 *          +
 *     declared dependencies
 *          +
 *     declared compilation context
 *          +
 *     compilation policies
 *          +
 *     reproducibility requirements
 *          |
 *          v
 *     reproducibility contract
 *          |
 *          v
 *     semantic compilation result
 *          |
 *          v
 *     logical artifact identity
 *
 * The grammar records intent.
 *
 * It does NOT determine whether two artifacts are equivalent.
 *
 * Equivalence is defined by the semantic/compiler reproducibility model.
 *
 * ============================================================================
 * IMPORTANT DISTINCTIONS
 * ============================================================================
 *
 * REPRODUCIBILITY
 * ---------------
 *
 * Concerns repeatability/equivalence of compilation results under declared
 * inputs and policy.
 *
 *
 * DETERMINISTIC BUILD
 * -------------------
 *
 * Concerns deterministic compiler/build behavior.
 *
 * Owned separately by:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 *
 * DETERMINISTIC EXECUTION
 * -----------------------
 *
 * Concerns runtime/execution behavior.
 *
 * It is NOT owned by this grammar.
 *
 *
 * CACHING
 * -------
 *
 * Concerns reuse of previously produced compilation artifacts.
 *
 * Owned by:
 *
 *     grammar/compile/caching.g4
 *
 *
 * PROVENANCE
 * ----------
 *
 * Concerns lineage and transformation history.
 *
 * Owned by:
 *
 *     grammar/compile/provenance.g4
 *
 *
 * OPTIMIZATION
 * ------------
 *
 * Concerns transformation for selected semantic objectives.
 *
 * Owned by:
 *
 *     grammar/compile/optimization.g4
 *
 *
 * LOWERING
 * --------
 *
 * Concerns transformation from canonical semantic/IR representation toward
 * downstream realization.
 *
 * Owned by:
 *
 *     grammar/compile/lowering.g4
 *
 *
 * TARGET SELECTION
 * ----------------
 *
 * Concerns realization target intent and selection.
 *
 * Owned by:
 *
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 *
 * Reproducibility may reference these concepts as semantic properties, but
 * it must not duplicate their grammar.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * All lexical tokens come from:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The canonical repository token vocabulary already provides the tokens
 * required by this grammar, including:
 *
 *     REPRODUCIBLE
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     RESOURCE
 *     RESOURCES
 *     PROPERTY
 *     PROFILE
 *     INPUT
 *     DETERMINISTIC
 *
 * No reproducibility-specific token such as:
 *
 *     REPRODUCIBILITY_INPUT
 *     REPRODUCIBILITY_DEPENDENCY
 *     REPRODUCIBILITY_ENVIRONMENT
 *
 * is required.
 *
 * This is intentional.
 *
 * Reproducibility-specific concepts that are not universal language keywords
 * are represented through existing semantic vocabulary and properties.
 *
 * For example:
 *
 *     property dependency_identity = ...
 *     property environment_identity = ...
 *     property artifact_identity = ...
 *
 * This keeps the lexical vocabulary open-ended.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * Core
 * ----
 *
 * `Core` supplies the canonical:
 *
 *     identifier
 *     qualifiedName
 *
 * hierarchy.
 *
 * Expressions
 * ----------
 *
 * `Expressions` supplies the canonical:
 *
 *     expression
 *
 * hierarchy.
 *
 * This grammar MUST NOT redefine any of these rules.
 *
 * ============================================================================
 * DECLARATION BOUNDARY
 * ============================================================================
 *
 * `compile.g4` is the composition root.
 *
 * It owns:
 *
 *     COMPILE
 *
 * and delegates:
 *
 *     compileReproducibilityReference
 *         ->
 *     reproducibilityDeclaration
 *
 * Therefore this grammar MUST NOT contain:
 *
 *     COMPILE reproducible ...
 *
 * and MUST NOT define another compile declaration.
 *
 * Canonical structure:
 *
 *     compile
 *         reproducible
 *             { ... }
 *
 * becomes:
 *
 *     compileDeclaration
 *         ->
 *     compileSpecification
 *         ->
 *     compileReproducibilityReference
 *         ->
 *     reproducibilityDeclaration
 *
 * ============================================================================
 * 1. PUBLIC REPRODUCIBILITY DECLARATION
 * ============================================================================
 *
 * An empty declaration is valid:
 *
 *     compile reproducible;
 *
 * The semantic layer supplies the language-defined default reproducibility
 * policy.
 *
 * A populated declaration may contain an arbitrary number of entries.
 *
 * No finite entry count is imposed.
 *
 * ============================================================================
 */

parser grammar CompileReproducibility;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `compile.g4` currently delegates to:
 *
 *     reproducibilityDeclaration
 *
 * Therefore this exact rule name is part of the public integration contract.
 */

reproducibilityDeclaration
    : REPRODUCIBLE reproducibilitySpecification
    ;


/*
 * ============================================================================
 * REPRODUCIBILITY SPECIFICATION
 * ============================================================================
 *
 * Supported forms:
 *
 *     compile reproducible;
 *
 *     compile reproducible {
 *         ...
 *     }
 *
 * The declaration itself establishes reproducibility intent.
 */

reproducibilitySpecification
    : reproducibilityBody?
    ;


/*
 * ============================================================================
 * REPRODUCIBILITY BODY
 * ============================================================================
 *
 * No artificial number of entries is permitted.
 */

reproducibilityBody
    : LBRACE reproducibilityEntry* RBRACE
    ;


/*
 * ============================================================================
 * REPRODUCIBILITY ENTRY DISPATCH
 * ============================================================================
 *
 * Every branch begins with a canonical lexical construct.
 *
 * This prevents a generic identifier-first alternative from stealing input
 * from other compile clauses.
 *
 * Open-ended semantic properties are represented through PROPERTY.
 */

reproducibilityEntry
    : reproducibilityRequirement
    | reproducibilityConstraint
    | reproducibilityPreference
    | reproducibilityHint
    | reproducibilityCapability
    | reproducibilityResource
    | reproducibilityInput
    | reproducibilityProfile
    | reproducibilityProperty
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Examples:
 *
 *     requires capability("reproducible.build");
 *
 *     requires source_identity;
 *
 *     requires dependency_identity;
 *
 * The expression remains open-ended.
 *
 * Resource feasibility and capability satisfaction are downstream concerns.
 */

reproducibilityRequirement
    : REQUIRES expression SEMICOLON?
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts the acceptable reproducibility solution space.
 *
 * Examples:
 *
 *     constraint source_identity == declared_source;
 *
 *     constraint serialization == stable;
 *
 *     constraint external_inputs == declared;
 *
 * The grammar does not enumerate constraint kinds.
 */

reproducibilityConstraint
    : CONSTRAINT expression SEMICOLON?
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * A preference is non-mandatory guidance.
 *
 * A compiler MUST NOT silently promote a preference into a hard requirement.
 */

reproducibilityPreference
    : PREFER expression SEMICOLON?
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * A hint is informational/non-binding semantic guidance.
 *
 * It MUST NOT alter the required semantics of the program.
 */

reproducibilityHint
    : HINT expression SEMICOLON?
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Capability requirements remain symbolic.
 *
 * Examples:
 *
 *     capability("reproducible.build");
 *
 *     capability("stable.serialization");
 *
 *     capability("content.addressed.artifacts");
 *
 * The grammar does not enumerate capability names.
 *
 * This is essential for future compiler implementations and future hardware.
 */

reproducibilityCapability
    : CAPABILITY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * RESOURCE INTENT
 * ============================================================================
 *
 * Reproducibility may require semantic resources, such as:
 *
 *     verification resources
 *     artifact retention
 *     metadata retention
 *     deterministic build infrastructure
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * Examples:
 *
 *     resource verification >= required_verification;
 *
 *     resources storage >= required_storage;
 *
 * This grammar does not define a resource inventory.
 */

reproducibilityResource
    : RESOURCE expression SEMICOLON?
    | RESOURCES expression SEMICOLON?
    ;


/*
 * ============================================================================
 * INPUT DECLARATION
 * ============================================================================
 *
 * INPUT is already a canonical Zamani lexical concept.
 *
 * Reproducibility input declarations identify semantic inputs participating
 * in the reproducibility contract.
 *
 * The grammar does not inspect the referenced input.
 *
 * It does not read files, datasets, devices, networks, or external systems.
 *
 * Examples:
 *
 *     input source_manifest;
 *
 *     input build_metadata;
 *
 *     input dataset_identity;
 *
 *     input dependency_lock;
 *
 * The semantic layer determines what each reference denotes.
 */

reproducibilityInput
    : INPUT reproducibilityReferenceValue SEMICOLON?
    ;


/*
 * ============================================================================
 * PROFILE REFERENCE
 * ============================================================================
 *
 * A named compilation profile may provide part of the reproducibility policy.
 *
 * Profile ownership remains:
 *
 *     grammar/compile/profiles.g4
 *
 * This grammar merely references a profile.
 */

reproducibilityProfile
    : PROFILE qualifiedName SEMICOLON?
    ;


/*
 * ============================================================================
 * OPEN-WORLD PROPERTY SYSTEM
 * ============================================================================
 *
 * PROPERTY is the primary extensibility mechanism.
 *
 * Reproducibility must not require a new keyword every time a new compiler,
 * serialization format, artifact system, dependency mechanism, language
 * version, provenance mechanism, or future computational substrate appears.
 *
 * Examples:
 *
 *     property source_identity = source;
 *
 *     property dependency_identity = dependency_lock;
 *
 *     property compiler_identity = compiler;
 *
 *     property toolchain_identity = toolchain;
 *
 *     property language_identity = language;
 *
 *     property environment_identity = environment;
 *
 *     property artifact_identity = artifact;
 *
 *     property serialization = stable;
 *
 *     property timestamps = normalized;
 *
 *     property paths = normalized;
 *
 *     property randomness = declared;
 *
 *     property external_inputs = declared;
 *
 *     property generated_metadata = stable;
 *
 * These identifiers are semantic names, not hard-coded grammar concepts.
 */

reproducibilityProperty
    : PROPERTY reproducibilityPropertyName
      reproducibilityPropertyAssignment?
      SEMICOLON?
    ;


reproducibilityPropertyName
    : identifier
    | qualifiedName
    ;


reproducibilityPropertyAssignment
    : ASSIGN expression
    | COLON expression
    | reproducibilityPropertyBlock
    ;


/*
 * ============================================================================
 * NESTED PROPERTY BLOCK
 * ============================================================================
 *
 * Nested properties allow structured reproducibility metadata without
 * introducing another domain-specific grammar.
 *
 * Example:
 *
 *     property environment {
 *         compiler = compiler_identity;
 *         toolchain = toolchain_identity;
 *         language = language_identity;
 *     }
 *
 * The property names remain open-ended.
 */

reproducibilityPropertyBlock
    : LBRACE reproducibilityPropertyEntry* RBRACE
    ;


reproducibilityPropertyEntry
    : reproducibilityPropertyName
      reproducibilityPropertyAssignment
      SEMICOLON?
    ;


/*
 * ============================================================================
 * REFERENCE VALUES
 * ============================================================================
 *
 * A reproducibility reference may be:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *
 * This intentionally allows semantic references without creating a second
 * value language.
 */

reproducibilityReferenceValue
    : identifier
    | qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * SEMANTIC PROPERTY VOCABULARY
 * ============================================================================
 *
 * The following are examples of semantic property identities.
 *
 * They are NOT grammar alternatives.
 *
 * They therefore do not create a closed-world reproducibility vocabulary.
 *
 * Possible properties include:
 *
 *     source_identity
 *     source_version
 *     language_identity
 *     grammar_identity
 *     ast_identity
 *     semantic_identity
 *     dependency_identity
 *     dependency_lock
 *     compiler_identity
 *     toolchain_identity
 *     profile_identity
 *     target_identity
 *     environment_identity
 *     artifact_identity
 *     serialization
 *     ordering
 *     timestamps
 *     paths
 *     locale
 *     timezone
 *     randomness
 *     external_inputs
 *     generated_metadata
 *     build_configuration
 *     compilation_context
 *     provenance
 *     verification
 *     equivalence
 *
 * Implementations may define additional properties without modifying this
 * grammar.
 *
 * ============================================================================
 * DETERMINISTIC-BUILD INTEGRATION
 * ============================================================================
 *
 * Deterministic build syntax belongs to:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 * This grammar MUST NOT duplicate deterministic-build declarations.
 *
 * A program may therefore contain compilation intent such as:
 *
 *     compile deterministic;
 *     compile reproducible;
 *
 * The semantic layer determines their relationship.
 *
 * Reproducibility may depend upon deterministic compilation, but:
 *
 *     reproducible != deterministic-build
 *
 * and:
 *
 *     deterministic-build != deterministic-execution.
 *
 * A reproducibility property may reference determinism semantically:
 *
 *     property determinism = ...
 *
 * without making this grammar the owner of deterministic-build syntax.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Compilation provenance belongs to:
 *
 *     grammar/compile/provenance.g4
 *
 * Reproducibility may require provenance properties, for example:
 *
 *     requires capability("provenance.integrity");
 *
 * or:
 *
 *     property provenance = required;
 *
 * However, this grammar MUST NOT define provenance relationships such as:
 *
 *     derived_from
 *     transformed_by
 *     produced_by
 *     verified_by
 *
 * Those belong to the provenance grammar and semantic model.
 *
 * ============================================================================
 * CACHING INTEGRATION
 * ============================================================================
 *
 * Caching belongs to:
 *
 *     grammar/compile/caching.g4
 *
 * Reproducibility can improve cache safety by establishing stronger identity
 * requirements, but:
 *
 *     reproducibility != caching.
 *
 * A cache hit MUST NOT be considered valid merely because a reproducibility
 * declaration exists.
 *
 * The cache subsystem must establish identity using the complete semantic
 * compilation input set.
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization belongs to:
 *
 *     grammar/compile/optimization.g4
 *
 * Reproducibility does not prohibit optimization.
 *
 * Instead, the semantic/compiler layers must ensure that optimization choices
 * are compatible with the selected reproducibility contract.
 *
 * This grammar MUST NOT introduce:
 *
 *     optimization passes
 *     optimization pipelines
 *     optimization algorithms
 *     optimization cost models
 *     optimization objectives.
 *
 * ============================================================================
 * LOWERING INTEGRATION
 * ============================================================================
 *
 * Lowering belongs to:
 *
 *     grammar/compile/lowering.g4
 *
 * Reproducibility may require the lowering process to preserve declared
 * semantic identity.
 *
 * The lowering subsystem remains responsible for transformation.
 *
 * This grammar does not define lowering stages or transformations.
 *
 * ============================================================================
 * SPECIALIZATION INTEGRATION
 * ============================================================================
 *
 * Specialization belongs to the existing compilation specialization subsystem.
 *
 * Reproducibility may constrain specialization decisions through properties
 * or semantic requirements.
 *
 * It MUST NOT duplicate specialization syntax here.
 *
 * ============================================================================
 * TARGET INTEGRATION
 * ============================================================================
 *
 * Target intent and selection remain owned by:
 *
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 *
 * Reproducibility MUST NOT require a fixed physical target.
 *
 * For example, it must be possible for one source program to remain
 * reproducible across:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     clusters
 *     distributed systems
 *     HPC systems
 *     cloud environments
 *     future computational substrates
 *
 * provided the selected reproducibility contract can be satisfied.
 *
 * ============================================================================
 * CROSS-COMPILATION INTEGRATION
 * ============================================================================
 *
 * Cross-compilation syntax belongs to:
 *
 *     grammar/compile/cross-compilation.g4
 *
 * Reproducibility may apply to cross-compilation.
 *
 * It MUST NOT make a source program dependent upon one architecture.
 *
 * Source portability and physical target feasibility remain separate concepts.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * This grammar only records reproducibility-related resource and capability
 * intent.
 *
 * Resource and capability meaning is resolved downstream.
 *
 * Examples:
 *
 *     requires capability("stable.serialization");
 *
 *     requires capability("reproducible.build");
 *
 *     resource verification >= required_verification;
 *
 * There is no physical resource inventory here.
 *
 * The compiler may discover that a requested capability or resource cannot be
 * satisfied on a particular realization.
 *
 * That is a semantic feasibility result, not a parser failure.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Reproducibility may depend upon effects such as:
 *
 *     randomness
 *     native
 *     foreign
 *     network
 *     filesystem
 *     distributed
 *     simulation
 *     reflection
 *     code generation
 *
 * Effect ownership remains in:
 *
 *     grammar/effects/
 *
 * This grammar MUST NOT create a second effect system.
 *
 * Semantic analysis determines whether an effect is:
 *
 *     declared
 *     controlled
 *     reproducible
 *     externally determined
 *     prohibited by policy
 *     or otherwise relevant.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Reproducibility may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The general contract system remains owned by:
 *
 *     grammar/validation/
 *
 * This grammar uses existing REQUIREMENT/CONSTRAINT/PROPERTY concepts rather
 * than creating a second contract language.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policy semantics remain outside this grammar.
 *
 * A reproducibility property or requirement may reference an existing policy.
 *
 * For example:
 *
 *     property policy = compilation::reproducibility;
 *
 * or:
 *
 *     requires capability("policy.reproducibility");
 *
 * The semantic policy system determines authority and applicability.
 *
 * A reproducibility declaration MUST NOT grant security or execution
 * authority merely by being present.
 *
 * ============================================================================
 * AI / KNOWLEDGE / LEARNING INTEGRATION
 * ============================================================================
 *
 * Reproducibility applies equally to programs containing:
 *
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     agents
 *     neural-symbolic composition
 *
 * For example, reproducibility may constrain:
 *
 *     model identity
 *     dataset identity
 *     training configuration
 *     random sources
 *     external inputs
 *     generated artifacts
 *     semantic compilation decisions
 *
 * Those concepts remain owned by their respective domains.
 *
 * This grammar merely supplies the compilation-level reproducibility contract.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Reproducibility applies to quantum programs without requiring physical
 * hardware identity.
 *
 * It may concern:
 *
 *     quantum semantic identity
 *     operation parameters
 *     source circuit description
 *     compilation context
 *     decomposition policy
 *     declared noise model
 *     declared simulation model
 *     generated compilation artifacts
 *
 * The canonical quantum representation remains:
 *
 *     quantum::ir
 *
 * This grammar does NOT define:
 *
 *     quantum operations
 *     qubits
 *     coupling maps
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     physical QPU topology.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Reproducibility may apply to HDL and hardware/software co-design.
 *
 * It may constrain semantic properties such as:
 *
 *     source identity
 *     parameter identity
 *     synthesis intent
 *     toolchain identity
 *     generated representation
 *     verification configuration
 *
 * It MUST NOT encode a fixed:
 *
 *     device
 *     register width
 *     signal count
 *     memory size
 *     FPGA size
 *     ASIC capacity
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Reproducibility may reference:
 *
 *     logical topology
 *     declared dependencies
 *     protocol identity
 *     external inputs
 *     service identity
 *     serialization policy
 *     ordering policy
 *
 * It MUST NOT impose a fixed number of:
 *
 *     nodes
 *     services
 *     endpoints
 *     channels
 *     devices
 *
 * Physical network availability is resolved downstream.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Compile-time execution, reflection, generation, and macros may affect
 * reproducibility.
 *
 * Their semantics remain owned by:
 *
 *     grammar/metaprogramming/
 *     grammar/macros/
 *     grammar/compile/compile-time.g4
 *
 * Reproducibility analysis must account for any declared compilation inputs
 * that influence generated semantic artifacts.
 *
 * This grammar does not duplicate those mechanisms.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, the semantic layer MUST:
 *
 *     1. identify the reproducibility declaration;
 *     2. resolve symbolic properties;
 *     3. classify requirements;
 *     4. classify constraints;
 *     5. classify preferences;
 *     6. classify hints;
 *     7. resolve capability references;
 *     8. resolve resource references;
 *     9. resolve input identities;
 *    10. resolve profile references;
 *    11. validate property assignments;
 *    12. detect contradictory declarations;
 *    13. preserve source provenance;
 *    14. integrate deterministic-build intent where present;
 *    15. integrate caching identity where applicable;
 *    16. integrate compilation provenance;
 *    17. integrate optimization/lowering decisions;
 *    18. integrate target-independent semantic identity;
 *    19. construct the canonical semantic compilation representation.
 *
 * The parser MUST NOT perform these operations.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Reproducibility syntax does not introduce a new type system.
 *
 * Expressions used by reproducibility requirements/properties are checked by
 * the canonical Zamani type system.
 *
 * Semantic validation MUST reject property values whose types are incompatible
 * with their declared semantic meaning.
 *
 * The grammar does not impose a fixed representation for identities.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a reproducibility declaration creates no runtime effect.
 *
 * Semantic processing may observe declared effects and determine whether they
 * affect reproducibility.
 *
 * No filesystem/network/hardware operation is performed by this grammar.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability references are symbolic.
 *
 * Examples:
 *
 *     capability("reproducible.build")
 *
 *     capability("stable.serialization")
 *
 * Capability availability is determined downstream.
 *
 * The presence of a capability expression does NOT grant that capability.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions are symbolic semantic requirements.
 *
 * They may participate in:
 *
 *     resource analysis
 *     capability negotiation
 *     compilation planning
 *     deployment planning
 *
 * No physical capacity is encoded here.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source locations and declaration identity must remain available downstream.
 *
 * Semantic/compiler provenance may record:
 *
 *     source
 *     reproducibility policy
 *     declared inputs
 *     declared dependencies
 *     compilation context
 *     compiler/toolchain identity
 *     transformations
 *     resulting artifact identity
 *
 * Actual provenance graph construction is owned elsewhere.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR directly.
 *
 * Reproducibility metadata is carried through semantic compilation structures
 * and attached to canonical IR/artifact metadata where required.
 *
 * No:
 *
 *     ReproducibilityIR
 *     QuantumReproducibilityIR
 *     HardwareReproducibilityIR
 *
 * is defined here.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar is intentionally open-world.
 *
 * It contains no:
 *
 *     fixed hardware cardinalities
 *     fixed quantum cardinalities
 *     fixed target catalogues
 *     fixed compiler catalogue
 *     fixed dependency count
 *     fixed input count
 *     fixed property count
 *     fixed artifact count
 *     fixed domain count.
 *
 * Examples that MUST NOT be introduced into this grammar include:
 *
 *     fixed processor counts
 *     fixed device counts
 *     fixed qubit counts
 *     fixed node counts
 *     fixed memory capacities
 *     fixed tensor dimensions
 *     fixed network sizes.
 *
 * Large programs are limited only by implementation/compiler resources and
 * declared semantic constraints.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * The same source text under the same lexical/language configuration must
 * produce the same parse structure.
 *
 * Reproducibility semantics may refer to deterministic compilation, but this
 * parser does not itself guarantee deterministic execution.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural parser diagnostics should identify:
 *
 *     - missing REPRODUCIBLE declaration body delimiter;
 *     - malformed requirement;
 *     - malformed constraint;
 *     - malformed preference;
 *     - malformed hint;
 *     - malformed capability expression;
 *     - malformed resource expression;
 *     - malformed input reference;
 *     - malformed profile reference;
 *     - malformed property name;
 *     - malformed property assignment;
 *     - malformed nested property block.
 *
 * Semantic diagnostics, not parser diagnostics, should report:
 *
 *     - unsatisfied capability;
 *     - insufficient resources;
 *     - conflicting reproducibility constraints;
 *     - invalid property meaning;
 *     - unavailable profile;
 *     - invalid dependency identity;
 *     - impossible reproducibility guarantee;
 *     - target-specific incompatibility;
 *     - nondeterministic compilation input where determinism was required.
 *
 * A target/resource failure MUST NOT be converted into a syntax error.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The public integration rule:
 *
 *     reproducibilityDeclaration
 *
 * MUST remain stable.
 *
 * Existing source using:
 *
 *     compile reproducible;
 *
 * must remain valid.
 *
 * Existing source using:
 *
 *     compile reproducible { ... }
 *
 * remains valid where its entries use canonical vocabulary.
 *
 * Reproducibility-specific aliases must be handled through the repository's
 * compatibility subsystem rather than by creating duplicate grammar rules.
 *
 * Deprecated properties should remain semantic compatibility concerns whenever
 * possible.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 * The conformance suite MUST include:
 *
 *     compile reproducible;
 *
 *     compile reproducible {
 *         requires capability("reproducible.build");
 *     }
 *
 *     compile reproducible {
 *         constraint source_identity == declared_source;
 *     }
 *
 *     compile reproducible {
 *         prefer serialization::stable;
 *         hint paths::normalized;
 *     }
 *
 *     compile reproducible {
 *         resource verification >= required_verification;
 *     }
 *
 *     compile reproducible {
 *         input source_manifest;
 *         input dependency_lock;
 *     }
 *
 *     compile reproducible {
 *         profile compilation::reproducible;
 *     }
 *
 *     compile reproducible {
 *         property source_identity = source;
 *         property dependency_identity = dependency_lock;
 *         property artifact_identity = semantic_artifact;
 *     }
 *
 *     compile reproducible {
 *         property environment {
 *             compiler = compiler_identity;
 *             toolchain = toolchain_identity;
 *         }
 *     }
 *
 * NEGATIVE TESTS
 * --------------
 *
 * The suite MUST reject malformed constructs including:
 *
 *     compile reproducible {
 *         requires;
 *     }
 *
 *     compile reproducible {
 *         constraint;
 *     }
 *
 *     compile reproducible {
 *         property;
 *     }
 *
 *     compile reproducible {
 *         profile;
 *     }
 *
 *     compile reproducible {
 *         input;
 *     }
 *
 *     compile reproducible {
 *         property environment {
 *             compiler;
 *         }
 *     }
 *
 * BOUNDARY TESTS
 * --------------
 *
 * The suite MUST cover:
 *
 *     - empty reproducibility declaration;
 *     - one entry;
 *     - many entries;
 *     - nested property blocks;
 *     - deeply nested expressions;
 *     - qualified property names;
 *     - Unicode identifiers where supported;
 *     - classical compilation;
 *     - quantum compilation;
 *     - hybrid compilation;
 *     - HDL compilation;
 *     - accelerator compilation;
 *     - distributed compilation;
 *     - AI/learning compilation;
 *     - metaprogram-generated compilation inputs.
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * The suite MUST verify:
 *
 *     - no fixed number of entries;
 *     - no fixed property count;
 *     - no fixed input count;
 *     - no fixed dependency count;
 *     - no fixed artifact count;
 *     - no fixed domain count;
 *     - no fixed hardware capacity.
 *
 * Large tests must be bounded by test infrastructure resources, not by
 * grammar-defined capacity.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * At minimum:
 *
 *     classical + reproducibility
 *     quantum + reproducibility
 *     hybrid + reproducibility
 *     HDL + reproducibility
 *     AI + reproducibility
 *     distributed + reproducibility
 *     resource requirements + reproducibility
 *     capabilities + reproducibility
 *     contracts + reproducibility
 *     provenance + reproducibility
 *     optimization + reproducibility
 *     lowering + reproducibility
 *     deterministic-build + reproducibility
 *     caching + reproducibility.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Repeated parser invocations on identical source/configuration MUST produce
 * equivalent parse structures.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain:
 *
 *     no hardware capacities;
 *     no target-specific capacities;
 *     no finite domain catalogue;
 *     no finite capability catalogue;
 *     no finite resource catalogue;
 *     no fixed number of reproducibility entries;
 *     no fixed number of properties;
 *     no fixed number of inputs;
 *     no fixed number of artifacts.
 *
 * It MUST NOT introduce machine-specific grammar such as:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     memory size
 *     register width
 *     tensor rank limit
 *     network size.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * A reproducibility declaration MUST NOT grant:
 *
 *     filesystem authority;
 *     network authority;
 *     native execution authority;
 *     FFI authority;
 *     reflection authority;
 *     compiler-plugin authority;
 *     hardware authority.
 *
 * Capability references express requirements.
 *
 * They do not grant permissions.
 *
 * Security authorization remains owned by:
 *
 *     grammar/security/
 *
 * and the downstream security model.
 *
 * ============================================================================
 * FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * Future reproducibility features SHOULD normally be introduced as:
 *
 *     property
 *     qualified property
 *     expression
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * rather than adding a new reserved keyword.
 *
 * A new reserved keyword is justified only when the construct has a stable,
 * universal, parser-level grammatical role that cannot be expressed safely
 * through existing generic mechanisms.
 *
 * This is the primary mechanism preventing grammar growth from becoming a
 * closed-world catalogue.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * `grammar/compile/reproducibility.g4` is COMPLETE when:
 *
 * [x] The grammar name is CompileReproducibility.
 *
 * [x] tokenVocab is ZamaniLexer.
 *
 * [x] Core is imported.
 *
 * [x] Expressions is imported.
 *
 * [x] `reproducibilityDeclaration` is exported.
 *
 * [x] `compile.g4` can consume `reproducibilityDeclaration`.
 *
 * [x] COMPILE is not duplicated here.
 *
 * [x] REPRODUCIBLE is consumed from the canonical lexer.
 *
 * [x] No invented reproducibility-specific lexical tokens are required.
 *
 * [x] Requirements use canonical REQUIRES.
 *
 * [x] Constraints use canonical CONSTRAINT.
 *
 * [x] Preferences use canonical PREFER.
 *
 * [x] Hints use canonical HINT.
 *
 * [x] Capabilities use canonical CAPABILITY.
 *
 * [x] Resources use canonical RESOURCE/RESOURCES.
 *
 * [x] Inputs use canonical INPUT.
 *
 * [x] Profiles use canonical PROFILE.
 *
 * [x] Properties provide open-world extensibility.
 *
 * [x] Identifiers come from Core.
 *
 * [x] Expressions come from Expressions.
 *
 * [x] No AST actions are embedded.
 *
 * [x] No IR is created.
 *
 * [x] No optimization syntax is duplicated.
 *
 * [x] No lowering syntax is duplicated.
 *
 * [x] No target-selection syntax is duplicated.
 *
 * [x] No caching implementation is duplicated.
 *
 * [x] No provenance implementation is duplicated.
 *
 * [x] No deterministic-build implementation is duplicated.
 *
 * [x] No physical hardware capacity is encoded.
 *
 * [x] No fixed quantum capacity is encoded.
 *
 * [x] No finite reproducibility-property catalogue is encoded.
 *
 * [x] Repeated constructs are structurally unbounded.
 *
 * [x] The grammar contains no embedded unsafe Rust.
 *
 * Remaining repository-level verification:
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Compile grammar composition succeeds.
 *
 * [ ] Canonical Zamani parser generation succeeds.
 *
 * [ ] Rust 1.97+ frontend integration succeeds.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass within available test resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Reproducibility is a compilation contract, not a target.
 *
 * The source describes what reproducibility properties matter.
 *
 * The compiler determines how those properties are satisfied.
 *
 * The resource system determines whether required capabilities/resources exist.
 *
 * The deterministic-build subsystem determines build determinism.
 *
 * The provenance subsystem records lineage.
 *
 * The cache subsystem determines reusable artifacts.
 *
 * The optimization subsystem transforms semantic representations.
 *
 * The lowering subsystem transforms canonical representations toward
 * realization.
 *
 * Target selection determines realization intent.
 *
 * Routing and scheduling determine physical execution arrangements.
 *
 * Resilience/QEC/ZQN/HAL determine downstream execution realization where
 * applicable.
 *
 * This separation is what permits the same Zamani source program to remain
 * portable from very small systems through increasingly capable systems,
 * without turning the grammar into a catalogue of today's machines.
 *
 * ============================================================================
 */