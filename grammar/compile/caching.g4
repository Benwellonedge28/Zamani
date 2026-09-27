/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/caching.g4
 *
 * Grammar:
 *     CompileCaching
 *
 * Status:
 *     Production-ready source-level compilation-caching intent grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust, semantic actions, filesystem
 *     access, network access, hardware access, or unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL CACHING INTENT for compilation.
 *
 * It describes what compilation results MAY be reused, under which semantic
 * identity and validity conditions, and what cache-related policies the
 * compiler may consider.
 *
 * It does NOT implement a cache.
 *
 * It does NOT:
 *
 *     - read or write cache storage;
 *     - inspect the filesystem;
 *     - inspect environment variables;
 *     - inspect hardware;
 *     - discover devices;
 *     - contact remote cache servers;
 *     - select a cache backend;
 *     - allocate cache storage;
 *     - evict cache entries;
 *     - execute compilation;
 *     - generate artifacts;
 *     - perform hashing;
 *     - verify cryptographic signatures;
 *     - determine whether a cache entry is actually valid;
 *     - bypass semantic validation;
 *     - create an IR.
 *
 * Those responsibilities belong downstream to compiler, provenance,
 * reproducibility, security, artifact, storage, and runtime infrastructure.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     CompileCaching
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> dependency analysis
 *          +--> provenance analysis
 *          +--> determinism analysis
 *          +--> reproducibility analysis
 *          +--> security/trust analysis
 *          +--> target/capability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / lowering / generation
 *          |
 *          v
 *     artifact identity
 *          |
 *          v
 *     cache admission / lookup / validation / storage
 *
 * Caching is therefore an optimization and build-infrastructure concern.
 *
 * It MUST NOT change the meaning of the Zamani program.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Caching MUST support portability rather than undermine it.
 *
 * A cache entry MUST NOT become valid merely because:
 *
 *     - a machine has the same CPU model;
 *     - a device has the same name;
 *     - a physical qubit has the same identifier;
 *     - a filesystem happens to contain the same path;
 *     - a compiler happens to be running on the same host.
 *
 * Cache validity is determined from the complete semantic identity required
 * by the requested artifact and cache policy.
 *
 * ============================================================================
 * CRITICAL SEMANTIC INVARIANT
 * ============================================================================
 *
 * CACHING MUST NEVER CHANGE PROGRAM SEMANTICS.
 *
 * Therefore:
 *
 *     cache hit
 *         == reuse of a semantically valid previously-produced result
 *
 * and NEVER:
 *
 *     cache hit
 *         == permission to skip correctness checks
 *
 * If the cache cannot establish validity, the compiler MUST treat the result
 * as unavailable and perform the required downstream work.
 *
 * A false cache miss is normally a performance concern.
 *
 * A false cache hit is a correctness/security concern.
 *
 * ============================================================================
 * CACHE IDENTITY
 * ============================================================================
 *
 * A cache identity MAY depend on any semantically relevant input, including:
 *
 *     - normalized source;
 *     - source dependencies;
 *     - imported modules;
 *     - dependency versions;
 *     - feature configuration;
 *     - language version;
 *     - grammar version where relevant;
 *     - dialect configuration;
 *     - macro expansion inputs;
 *     - compile-time evaluation inputs;
 *     - type-system configuration;
 *     - semantic configuration;
 *     - optimization policy;
 *     - deterministic-build policy;
 *     - reproducibility policy;
 *     - target-independent artifact requirements;
 *     - target capabilities;
 *     - target realization;
 *     - toolchain identity;
 *     - compiler identity;
 *     - compiler configuration;
 *     - relevant environment inputs;
 *     - security/trust domain;
 *     - provenance;
 *     - external inputs explicitly declared by the program.
 *
 * The grammar does NOT hard-code the cache-key algorithm.
 *
 * ============================================================================
 * DYNAMIC INPUTS
 * ============================================================================
 *
 * Resource and environment information may be dynamic.
 *
 * A cache entry MUST NOT be reused merely because the textual source is equal
 * when dynamic information affects the semantic result.
 *
 * Examples include:
 *
 *     hardware capability;
 *     available accelerator features;
 *     device calibration;
 *     topology;
 *     resource availability;
 *     security policy;
 *     external data;
 *     environment-provided configuration;
 *     time-dependent compilation inputs.
 *
 * The compiler must classify such inputs appropriately.
 *
 * The grammar only declares intent.
 *
 * ============================================================================
 * NO HARD-CODED LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT impose universal limits such as:
 *
 *     MAX_CACHE_ENTRIES
 *     MAX_CACHE_SIZE
 *     MAX_CACHE_KEYS
 *     MAX_ARTIFACTS
 *     MAX_DEPENDENCIES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Nor may disguised equivalents be introduced.
 *
 * Repetition is represented through parser repetition.
 *
 * Actual storage limits belong to the cache implementation and available
 * resources.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * Cache policy MUST distinguish:
 *
 *     semantic identity
 *     artifact identity
 *     target identity
 *     execution environment identity
 *     storage identity
 *
 * These concepts must not be conflated.
 *
 * For example, a source-level semantic result may be reusable across multiple
 * target machines while a machine-code artifact may only be reusable when the
 * target contract is compatible.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Cache policy MUST preserve security boundaries.
 *
 * A cache entry MUST NOT cross an authorization, trust, identity, tenancy,
 * confidentiality, integrity, or provenance boundary merely because its
 * content is otherwise reusable.
 *
 * Cache credentials, secret material, private keys, passwords, tokens, or
 * other sensitive values MUST NOT become cache keys merely because they exist
 * in the compilation environment.
 *
 * Security-sensitive cache behavior belongs downstream to security and
 * provenance analysis.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Caching and deterministic builds are related but distinct.
 *
 * Deterministic builds answer:
 *
 *     "Does equivalent input produce the same declared result?"
 *
 * Caching answers:
 *
 *     "May a previously produced result be reused safely?"
 *
 * A deterministic build MAY be cached.
 *
 * A cache MAY also contain reusable results whose declared semantics do not
 * require byte-identical artifacts, provided the semantic validity contract
 * permits reuse.
 *
 * This grammar therefore does NOT duplicate deterministic-build semantics.
 *
 * See:
 *
 *     grammar/compile/deterministic-builds.g4
 *     grammar/compile/reproducibility.g4
 *
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility determines which inputs are required to establish that a
 * result corresponds to a particular source/build identity.
 *
 * This grammar references that concept but does not redefine it.
 *
 * ============================================================================
 * ARTIFACT OWNERSHIP
 * ============================================================================
 *
 * This grammar does NOT define artifact formats.
 *
 * Artifact semantics remain owned by the artifact/generation subsystem.
 *
 * Caching may refer to an artifact class through an expression or property,
 * but the cache grammar does not enumerate:
 *
 *     object
 *     executable
 *     binary
 *     quantum::ir
 *     classical IR
 *     HDL
 *     QIR
 *     OpenQASM
 *     machine code
 *
 * as a closed list.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum compilation results MAY be cached.
 *
 * However, cache identity MUST distinguish semantically relevant information
 * such as:
 *
 *     - quantum operation semantics;
 *     - parameter values;
 *     - logical qubit requirements;
 *     - required capabilities;
 *     - noise/resilience assumptions where relevant;
 *     - target constraints where relevant;
 *     - routing/scheduling assumptions where relevant;
 *     - QEC configuration where relevant.
 *
 * The caching grammar MUST NOT create a quantum cache IR.
 *
 * Quantum semantic lowering continues through:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * There remains exactly one canonical quantum IR boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware-related compilation results MAY be cached.
 *
 * Cache identity must account for any hardware intent or target realization
 * that materially affects the artifact.
 *
 * The grammar does NOT hard-code:
 *
 *     register width;
 *     bus width;
 *     memory capacity;
 *     FPGA family;
 *     ASIC family;
 *     CPU model;
 *     GPU model;
 *     accelerator count;
 *     device identifier.
 *
 * ============================================================================
 * CLASSICAL / AI / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same caching model applies to:
 *
 *     classical computation;
 *     tensor computation;
 *     AI/ML;
 *     distributed compilation;
 *     networking;
 *     security;
 *     data processing;
 *     hardware/software co-design;
 *     future computing domains.
 *
 * Domain-specific cache identity remains semantic rather than being encoded
 * as a fixed grammar enumeration.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - source-level caching intent;
 *     - cache enable/disable intent;
 *     - cache scope intent;
 *     - cache identity intent;
 *     - cache input declarations;
 *     - cache invalidation intent;
 *     - cache reuse intent;
 *     - cache trust/security intent;
 *     - cache provenance intent;
 *     - cache portability intent;
 *     - cache policy properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - actual cache storage;
 *     - cache implementation;
 *     - filesystem layout;
 *     - remote cache protocol;
 *     - cache eviction algorithm;
 *     - cache replacement algorithm;
 *     - hashing implementation;
 *     - cryptographic implementation;
 *     - artifact serialization;
 *     - compiler execution;
 *     - target selection;
 *     - hardware discovery;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - canonical IR;
 *     - AST implementation.
 *
 * ============================================================================
 * EXPRESSION OWNERSHIP
 * ============================================================================
 *
 * This grammar imports the canonical expression grammar.
 *
 * It MUST NOT define:
 *
 *     expression
 *     identifier
 *     qualifiedName
 *     type
 *     literal
 *
 * locally.
 *
 * ============================================================================
 * PROPERTY MODEL
 * ============================================================================
 *
 * Caching is intentionally extensible through named properties.
 *
 * This avoids turning every future cache concept into a new reserved keyword.
 *
 * Examples of property names may include:
 *
 *     enabled
 *     scope
 *     key
 *     inputs
 *     dependencies
 *     provenance
 *     invalidation
 *     trust
 *     portability
 *     target
 *     artifact
 *     environment
 *     toolchain
 *     compiler
 *     policy
 *     lifetime
 *     validation
 *
 * These names are semantic data unless separately reserved by the language.
 *
 * ============================================================================
 * PUBLIC AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax corresponding to:
 *
 *     CachingDeclaration
 *         specification
 *             expression | property-list
 *
 * The AST/semantic layer should preserve:
 *
 *     source span
 *     property name
 *     property value
 *     declaration order where source ordering is semantically relevant
 *
 * The AST must not contain a backend cache implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. validate property names;
 *     2. validate property value types;
 *     3. detect duplicate mutually-exclusive properties;
 *     4. classify cache identity inputs;
 *     5. establish whether dynamic inputs participate in validity;
 *     6. establish security/trust boundaries;
 *     7. establish artifact applicability;
 *     8. establish interaction with deterministic builds;
 *     9. establish interaction with reproducibility;
 *    10. establish interaction with specialization;
 *    11. reject unsafe cache reuse;
 *    12. produce diagnostics with source spans.
 *
 * None of those operations occurs in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO IR.
 *
 * Cache intent is represented in the semantic compilation model and consumed
 * by the compiler/build infrastructure.
 *
 * It MUST NOT become:
 *
 *     CacheIR
 *     CompilationCacheIR
 *     QuantumCacheIR
 *     HardwareCacheIR
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler consumers may use this declaration to determine:
 *
 *     - whether reuse is permitted;
 *     - which semantic inputs participate in identity;
 *     - which invalidation conditions apply;
 *     - which provenance/trust constraints apply;
 *     - which artifact classes are eligible.
 *
 * The compiler MUST independently validate cache entries.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime caching is outside this grammar unless a separate runtime grammar
 * explicitly imports an appropriate cache concept.
 *
 * Compilation caching MUST NOT silently become runtime memoization.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is designed to integrate into:
 *
 *     grammar/compile/compile.g4
 *
 * through:
 *
 *     import CompileCaching;
 *
 * and:
 *
 *     compileClause
 *         : ...
 *         | cachingDeclaration
 *         ;
 *
 * `compilation.g4` already composes the Compile grammar, so once Compile owns
 * this clause, the higher-level compilation composition receives caching
 * automatically without importing this grammar a second time.
 *
 * This avoids competing ownership.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar requires one reserved lexical word:
 *
 *     CACHING : 'caching'
 *
 * It MUST be added to the canonical keyword vocabulary:
 *
 *     grammar/lexer/keywords.g4
 *
 * No additional cache-specific keywords are required.
 *
 * Property names intentionally remain identifiers/qualified names.
 *
 * ============================================================================
 * EXAMPLES
 * ============================================================================
 *
 * Basic:
 *
 *     compile caching {
 *         enabled: true;
 *     }
 *
 * Scoped:
 *
 *     compile caching {
 *         enabled: true;
 *         scope: "module";
 *     }
 *
 * Semantic identity:
 *
 *     compile caching {
 *         key: source;
 *         inputs: dependencies;
 *     }
 *
 * Reproducibility-aware:
 *
 *     compile caching {
 *         key: build_identity;
 *         provenance: required_provenance;
 *         validation: strict;
 *     }
 *
 * Target-aware artifact reuse:
 *
 *     compile caching {
 *         artifact: artifact_kind;
 *         target: target_identity;
 *     }
 *
 * The grammar does not prescribe what those expressions mean. Semantic
 * analysis does.
 *
 * ============================================================================
 * NEGATIVE EXAMPLES
 * ============================================================================
 *
 * These MUST be rejected by the parser:
 *
 *     compile caching
 *
 * when no specification is provided.
 *
 *     compile caching {}
 *
 * when the language contract requires at least one property.
 *
 *     compile caching {
 *         enabled
 *     }
 *
 * because a property requires `:` or `=`.
 *
 *     compile caching {
 *         enabled: ;
 *     }
 *
 * because a property value is missing.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     recursive expressions
 *
 * rather than finite alternatives for cache properties.
 *
 * Therefore there is no grammar-level maximum for:
 *
 *     cache properties;
 *     cache identity inputs;
 *     dependency expressions;
 *     artifact classes;
 *     cache policies;
 *     cache declarations.
 *
 * Physical limits remain implementation/resource concerns.
 *
 * ============================================================================
 * DETERMINISTIC PARSING
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text;
 *     lexer configuration;
 *     grammar version;
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     cache contents;
 *     cache hits;
 *     cache misses;
 *     filesystem state;
 *     network state;
 *     hardware;
 *     environment state;
 *     wall-clock time;
 *     randomness.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Diagnostics generated downstream should identify:
 *
 *     - unknown cache property;
 *     - invalid property value;
 *     - conflicting cache policy;
 *     - incomplete cache identity;
 *     - unsafe dynamic-input reuse;
 *     - incompatible artifact scope;
 *     - security boundary violation;
 *     - unsupported cache capability.
 *
 * Diagnostics must retain source spans.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CACHE_ENTRIES
 *     MAX_CACHE_SIZE
 *     MAX_CACHE_KEYS
 *     MAX_CACHE_ARTIFACTS
 *     MAX_CACHE_DEPENDENCIES
 *     fixed cache backend names;
 *     fixed storage paths;
 *     fixed device identifiers;
 *     fixed vendor identifiers;
 *     fixed hardware capacities.
 *
 * Allowed:
 *
 *     source-level numeric values used as actual program semantics;
 *     semantic cache requirements;
 *     capability requirements;
 *     artifact requirements;
 *     target constraints;
 *     policy expressions.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one clear ownership boundary.
 *     [x] It introduces no cache implementation.
 *     [x] It introduces no IR.
 *     [x] It uses canonical expressions.
 *     [x] It supports arbitrary cache properties.
 *     [x] It has no fixed resource limits.
 *     [x] It preserves POCO-REAF.
 *     [x] It preserves semantic correctness.
 *     [x] It distinguishes caching from determinism.
 *     [x] It distinguishes caching from reproducibility.
 *     [x] It preserves security/provenance boundaries.
 *     [x] It supports quantum/classical/HDL/hybrid/future domains.
 *     [x] It has a defined AST contract.
 *     [x] It has a defined semantic contract.
 *     [x] It has a defined IR boundary.
 *     [x] It has a defined compiler integration.
 *     [x] It has a defined lexer integration.
 *     [x] It has positive/negative/boundary/scalability tests specified.
 *     [x] It requires no unsafe Rust.
 *
 * ============================================================================
 */

parser grammar CompileCaching;

options {
    tokenVocab = ZamaniLexer;
}

import Core,
       Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule is the only public declaration boundary exported by this grammar.
 *
 * The enclosing `compileDeclaration` remains owned by:
 *
 *     grammar/compile/compile.g4
 *
 * The enclosing compile grammar is responsible for consuming `COMPILE`.
 *
 * This grammar therefore consumes only:
 *
 *     caching <specification>
 *
 * when integrated as a compile clause.
 */

cachingDeclaration
    : CACHING cachingSpecification SEMI?
    ;


/*
 * ============================================================================
 * CACHING SPECIFICATION
 * ============================================================================
 *
 * Two forms are supported:
 *
 *     caching <expression>
 *
 * and:
 *
 *     caching {
 *         property: expression;
 *         ...
 *     }
 *
 * The block form is the extensible production form.
 *
 * The expression form provides a compact semantic representation for future
 * or implementation-specific policies without requiring grammar changes.
 */

cachingSpecification
    : cachingPropertyBlock
    | expression
    ;


/*
 * ============================================================================
 * PROPERTY BLOCK
 * ============================================================================
 *
 * At least one property is required.
 *
 * An empty caching declaration therefore cannot silently acquire a default
 * policy whose semantics might change between compiler versions.
 */

cachingPropertyBlock
    : LBRACE cachingProperty+ RBRACE
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * Both `:` and `=` are accepted because both already belong to the general
 * Zamani expression/property syntax family.
 *
 * The value is always a canonical expression.
 *
 * No cache-specific value grammar is created here.
 */

cachingProperty
    : cachingPropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 *
 * Property names remain open-ended.
 *
 * This prevents this grammar from becoming a finite registry of cache
 * implementation concepts.
 *
 * Semantic analysis owns recognition and validation.
 */

cachingPropertyName
    : identifier
    | qualifiedName
    ;