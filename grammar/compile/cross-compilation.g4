/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/cross-compilation.g4
 *
 * Grammar:
 *     CompileCrossCompilation
 *
 * Status:
 *     Production cross-compilation intent parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust actions and requires no unsafe Rust.
 *     The consuming compiler/frontend MUST use safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL CROSS-COMPILATION INTENT.
 *
 * It describes the relationship between:
 *
 *     source/host compilation context
 *     destination target intent
 *     target-independent compilation artifacts
 *     target-specific compilation artifacts
 *     cross-compilation requirements
 *     cross-compilation constraints
 *     cross-compilation preferences
 *     cross-compilation hints
 *     cross-compilation properties
 *
 * It does NOT perform cross-compilation.
 *
 * In particular, this grammar does not:
 *
 *     - invoke a compiler;
 *     - invoke a linker;
 *     - invoke a toolchain;
 *     - discover a sysroot;
 *     - discover a target;
 *     - discover hardware;
 *     - inspect the host machine;
 *     - inspect the destination machine;
 *     - allocate resources;
 *     - select a physical device;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - generate machine code;
 *     - execute binaries;
 *     - execute host commands;
 *     - access the filesystem;
 *     - access the network.
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
 *          +--> crossCompilationClause
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> source-context analysis
 *          +--> destination-target analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> portability analysis
 *          +--> ABI analysis
 *          +--> artifact analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical             quantum::ir           HDL/hardware
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                           optimization
 *                                 |
 *                        routing / scheduling
 *                                 |
 *                       resilience / QEC / ZQN
 *                                 |
 *                                 v
 *                                HAL
 *                                 |
 *                                 v
 *                         target realization
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Cross-compilation MUST preserve the distinction between:
 *
 *     SOURCE SEMANTICS
 *
 * and:
 *
 *     TARGET REALIZATION
 *
 * A Zamani program describes computation and portable intent.
 *
 * Cross-compilation describes how that semantic program may be realized for
 * another target environment.
 *
 * The grammar MUST NOT encode artificial universal machine limits such as:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_NETWORK_SIZE
 *     MAX_TARGETS
 *     MAX_ARTIFACTS
 *     MAX_TOOLCHAINS
 *     MAX_PROFILES
 *
 * Nor may equivalent limits be hidden behind grammar cardinality.
 *
 * Repetition therefore uses:
 *
 *     *
 *     +
 *
 * and not finite enumerations.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite target or resource ceiling.
 *
 * It does NOT mean:
 *
 *     infinite compiler memory;
 *     infinite compilation time;
 *     infinite storage;
 *     infinite network capacity;
 *     infinite hardware.
 *
 * Practical implementation limits belong to implementation/resource policy.
 *
 * ============================================================================
 * FUNDAMENTAL CROSS-COMPILATION MODEL
 * ============================================================================
 *
 * Cross-compilation is represented as:
 *
 *     source target intent
 *              |
 *              v
 *             ->
 *              |
 *              v
 *     destination target intent
 *
 * Conceptually:
 *
 *     compile cross <source> -> <destination>;
 *
 * or:
 *
 *     compile cross <source> -> <destination> {
 *         ...
 *     }
 *
 * Example:
 *
 *     compile cross host -> wasm;
 *
 * Example:
 *
 *     compile cross x86_64 -> aarch64 {
 *         requires capability("cross.compilation");
 *         prefer optimization.portable;
 *     }
 *
 * Example:
 *
 *     compile cross classical -> quantum {
 *         requires capability("quantum.execution");
 *     }
 *
 * The names `host`, `wasm`, `x86_64`, `aarch64`, `quantum`, etc. are NOT
 * enumerated by this grammar.
 *
 * They are target expressions interpreted by semantic analysis.
 *
 * ============================================================================
 * IMPORTANT DISTINCTIONS
 * ============================================================================
 *
 * This grammar preserves the distinction between:
 *
 *     source target
 *     destination target
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *     artifact
 *     ABI
 *     toolchain
 *     sysroot
 *     property
 *
 * These MUST NOT be collapsed into a generic backend-option map.
 *
 * In particular:
 *
 *     target requirement
 *         !=
 *     target preference
 *
 *     target capability
 *         !=
 *     target identity
 *
 *     toolchain preference
 *         !=
 *     compiler requirement
 *
 *     ABI requirement
 *         !=
 *     processor identity
 *
 *     sysroot
 *         !=
 *     source semantics
 *
 *     cross-compilation intent
 *         !=
 *     physical deployment
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - cross-compilation clause syntax;
 *     - source/destination target relationship;
 *     - cross-compilation-specific requirements;
 *     - cross-compilation-specific constraints;
 *     - cross-compilation-specific preferences;
 *     - cross-compilation-specific hints;
 *     - cross-compilation-specific artifact intent;
 *     - cross-compilation properties;
 *     - source/destination adaptation metadata;
 *     - target-independent cross-compilation policy syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - target expression semantics;
 *     - resource semantics;
 *     - hardware descriptions;
 *     - execution;
 *     - deployment;
 *     - optimization implementation;
 *     - lowering implementation;
 *     - linker implementation;
 *     - ABI implementation;
 *     - toolchain implementation;
 *     - canonical IR;
 *     - quantum::ir;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * EXISTING REPOSITORY INTEGRATION
 * ============================================================================
 *
 * This grammar integrates with:
 *
 *     grammar/compile/compile.g4
 *         owns the `compile` declaration and compile-clause dispatch.
 *
 *     grammar/compile/target.g4
 *         owns target expressions and target intent.
 *
 *     grammar/compile/target-selection.g4
 *         owns target-selection policy.
 *
 *     grammar/compile/profiles.g4
 *         owns reusable compilation profiles.
 *
 *     grammar/compile/specialization.g4
 *         owns specialization intent.
 *
 *     grammar/compile/optimization.g4
 *         owns optimization intent.
 *
 *     grammar/compile/reproducibility.g4
 *         owns reproducibility policy.
 *
 *     grammar/compile/deterministic-builds.g4
 *         owns deterministic-build policy.
 *
 *     grammar/compile/caching.g4
 *         owns compilation caching policy.
 *
 *     grammar/compile/deployment.g4
 *         owns deployment intent.
 *
 *     grammar/compile/provenance.g4
 *         owns compilation provenance.
 *
 *     grammar/resources/
 *         owns resource semantics.
 *
 *     grammar/hardware/
 *         owns hardware intent.
 *
 *     grammar/execution/
 *         owns execution/runtime intent.
 *
 *     grammar/interoperability/
 *         owns foreign formats and interoperability.
 *
 *     grammar/compatibility/
 *         owns language compatibility and migration.
 *
 *     grammar/specification/compilation-model.md
 *         defines the normative compilation artifact model.
 *
 *     grammar/specification/portability.md
 *         defines portability semantics.
 *
 *     grammar/validation/compatibility-rules.md
 *         defines compatibility boundaries.
 *
 *     src/compiler/linker.rs
 *         implements downstream linker behavior.
 *
 * Cross-compilation grammar MUST remain independent of all implementation
 * details in those layers.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It does not define lexer rules.
 *
 * One lexical addition is required:
 *
 *     CROSS
 *
 * owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * with the spelling:
 *
 *     cross
 *
 * `CROSS` is the only new reserved lexical concept required by this grammar.
 *
 * Other concepts such as:
 *
 *     source
 *     destination
 *     host
 *     sysroot
 *     toolchain
 *     abi
 *     format
 *     architecture
 *     environment
 *
 * remain ordinary identifiers/property names unless the language specification
 * later establishes an independent lexical requirement.
 *
 * This avoids turning every toolchain vocabulary item into a reserved keyword.
 *
 * ============================================================================
 * ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar consumes existing parser contracts:
 *
 *     Core
 *     Expressions
 *     CompileTarget
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
 * CompileTarget provides:
 *
 *     targetExpression
 *
 * No equivalent rules are redefined here.
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar CompileCrossCompilation;

options {
    tokenVocab = ZamaniLexer;
}

import Core,
       Expressions,
       CompileTarget;


/*
 * ============================================================================
 * 1. PUBLIC COMPILE CLAUSE
 * ============================================================================
 *
 * This is the primary public integration rule.
 *
 * It MUST be included by:
 *
 *     grammar/compile/compile.g4
 *
 * as one alternative of:
 *
 *     compileClause
 *
 * It intentionally does NOT consume `COMPILE`.
 *
 * The enclosing `compileDeclaration` has already consumed:
 *
 *     COMPILE
 *
 * Therefore:
 *
 *     compile cross ...
 *
 * is parsed as:
 *
 *     compileDeclaration
 *         -> compileSpecification
 *             -> compileClause
 *                 -> crossCompilationClause
 *
 * This avoids creating a second competing `compile` declaration.
 */

crossCompilationClause
    : CROSS crossCompilationSpecification
    ;


/*
 * ============================================================================
 * 2. CROSS-COMPILATION SPECIFICATION
 * ============================================================================
 *
 * Canonical minimum form:
 *
 *     cross <source> -> <destination>
 *
 * Optional body:
 *
 *     cross <source> -> <destination> {
 *         ...
 *     }
 *
 * The source and destination are semantic target expressions.
 *
 * There is no fixed enumeration of:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     WASM
 *     JVM
 *     ARM
 *     x86
 *     RISC-V
 *
 * or any future target class.
 */

crossCompilationSpecification
    : crossCompilationRoute crossCompilationBody?
    ;


/*
 * ============================================================================
 * 3. SOURCE -> DESTINATION
 * ============================================================================
 *
 * THIN_ARROW is the repository's canonical `->` token.
 *
 * It is already used elsewhere for semantic relationships such as function
 * return types and other language-level arrows.
 *
 * No second cross-compilation arrow token is introduced.
 */

crossCompilationRoute
    : crossSourceTarget
      THIN_ARROW
      crossDestinationTarget
    ;


/*
 * ============================================================================
 * 4. SOURCE TARGET
 * ============================================================================
 *
 * A source target describes the semantic compilation context from which the
 * cross-compilation operation begins.
 *
 * `targetExpression` remains owned by CompileTarget.
 *
 * Examples:
 *
 *     host
 *     x86_64
 *     aarch64
 *     wasm
 *     classical
 *     quantum
 *     accelerator
 *     vendor::target
 *
 * The grammar does not interpret any of these names.
 */

crossSourceTarget
    : targetExpression
    ;


/*
 * ============================================================================
 * 5. DESTINATION TARGET
 * ============================================================================
 *
 * The destination is another semantic target expression.
 *
 * It may represent:
 *
 *     a target class;
 *     a target family;
 *     a target capability set;
 *     a target composition;
 *     a future architecture;
 *     a portable target;
 *     a simulator;
 *     a quantum environment;
 *     an HDL/hardware realization class.
 *
 * It does not necessarily identify a physical device.
 */

crossDestinationTarget
    : targetExpression
    ;


/*
 * ============================================================================
 * 6. CROSS-COMPILATION BODY
 * ============================================================================
 *
 * The body contains zero or more cross-compilation clauses.
 *
 * There is no language-level maximum.
 */

crossCompilationBody
    : LBRACE crossCompilationEntry* RBRACE
    ;


/*
 * ============================================================================
 * 7. BODY ENTRY
 * ============================================================================
 *
 * Entries retain semantic categories.
 *
 * A requirement cannot silently become a preference.
 *
 * A property cannot silently become a physical device selection.
 */

crossCompilationEntry
    : crossCompilationRequirement
    | crossCompilationConstraint
    | crossCompilationPreference
    | crossCompilationHint
    | crossCompilationCapability
    | crossCompilationResource
    | crossCompilationArtifact
    | crossCompilationProfile
    | crossCompilationSpecialization
    | crossCompilationOptimization
    | crossCompilationReproducibility
    | crossCompilationDeterminism
    | crossCompilationProperty
    ;


/*
 * ============================================================================
 * 8. REQUIREMENT
 * ============================================================================
 *
 * A requirement MUST be satisfied by the selected cross-compilation strategy
 * or target realization.
 *
 * Examples:
 *
 *     requires capability("cross.compilation");
 *
 *     requires capability("quantum.execution");
 *
 *     requires memory >= required_memory;
 *
 *     requires source::feature;
 *
 * The parser does not determine satisfiability.
 */

crossCompilationRequirement
    : REQUIRES expression SEMI?
    ;


/*
 * ============================================================================
 * 9. CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts acceptable cross-compilation realizations.
 *
 * Examples:
 *
 *     constraint latency <= required_latency;
 *
 *     constraint format == required_format;
 *
 *     constraint reproducible == true;
 *
 * The expression remains an ordinary Zamani expression.
 */

crossCompilationConstraint
    : CONSTRAINT expression SEMI?
    ;


/*
 * ============================================================================
 * 10. PREFERENCE
 * ============================================================================
 *
 * A preference is non-mandatory guidance.
 *
 * It MUST NOT be promoted to a requirement merely because a backend
 * understands it.
 */

crossCompilationPreference
    : PREFER expression SEMI?
    ;


/*
 * ============================================================================
 * 11. HINT
 * ============================================================================
 *
 * A hint is non-binding implementation guidance.
 */

crossCompilationHint
    : HINT expression SEMI?
    ;


/*
 * ============================================================================
 * 12. CAPABILITY
 * ============================================================================
 *
 * Capability syntax is intentionally open-ended.
 *
 * Examples:
 *
 *     capability("cross.compilation");
 *
 *     capability("wasm");
 *
 *     capability("quantum.execution");
 *
 *     capability("tensor.compute");
 *
 * The grammar does not enumerate capability names.
 */

crossCompilationCapability
    : CAPABILITY expression SEMI?
    ;


/*
 * ============================================================================
 * 13. RESOURCE
 * ============================================================================
 *
 * Resource requirements remain semantic expressions.
 *
 * Examples:
 *
 *     resource memory >= required_memory;
 *
 *     resource bandwidth >= required_bandwidth;
 *
 *     resource availability >= required_availability;
 *
 * This grammar does not define a physical resource inventory.
 */

crossCompilationResource
    : RESOURCE expression SEMI?
    | RESOURCES expression SEMI?
    ;


/*
 * ============================================================================
 * 14. ARTIFACT
 * ============================================================================
 *
 * Cross-compilation may identify desired artifact classes.
 *
 * Examples:
 *
 *     artifact source;
 *
 *     artifact semantic;
 *
 *     artifact ir;
 *
 *     artifact executable;
 *
 *     artifact deployable;
 *
 * Artifact interpretation belongs to compilation artifact semantics.
 *
 * No finite artifact enumeration is embedded here.
 */

crossCompilationArtifact
    : ARTIFACT crossCompilationArtifactValue SEMI?
    ;

crossCompilationArtifactValue
    : identifier
    | qualifiedName
    | STRING
    | expression
    ;


/*
 * ============================================================================
 * 15. PROFILE
 * ============================================================================
 *
 * A cross-compilation body may reference a named compilation profile.
 *
 * Profile declaration/definition remains owned by profiles.g4.
 */

crossCompilationProfile
    : PROFILE qualifiedName SEMI?
    ;


/*
 * ============================================================================
 * 16. SPECIALIZATION
 * ============================================================================
 *
 * Specialization remains owned by:
 *
 *     grammar/compile/specialization.g4
 *
 * This rule only provides the integration boundary.
 *
 * Because specialization syntax may evolve independently, the cross body
 * references it through a qualified semantic property rather than duplicating
 * its declaration grammar.
 */

crossCompilationSpecialization
    : SPECIALIZATION expression SEMI?
    ;


/*
 * ============================================================================
 * 17. OPTIMIZATION
 * ============================================================================
 *
 * Optimization remains owned by:
 *
 *     grammar/compile/optimization.g4
 *
 * Cross-compilation may carry optimization intent without implementing an
 * optimization pass.
 *
 * The expression remains generic so new optimization policies do not require
 * new target-specific grammar keywords.
 */

crossCompilationOptimization
    : OPTIMIZE expression SEMI?
    ;


/*
 * ============================================================================
 * 18. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility policy remains owned by:
 *
 *     grammar/compile/reproducibility.g4
 *
 * This rule expresses cross-compilation-related reproducibility intent.
 */

crossCompilationReproducibility
    : REPRODUCIBLE expression SEMI?
    ;


/*
 * ============================================================================
 * 19. DETERMINISM
 * ============================================================================
 *
 * Determinism is an intent property.
 *
 * It does not execute deterministic compilation.
 *
 * The exact deterministic-build contract remains owned by:
 *
 *     grammar/compile/deterministic-builds.g4
 */

crossCompilationDeterminism
    : DETERMINISTIC expression SEMI?
    ;


/*
 * ============================================================================
 * 20. GENERIC PROPERTY
 * ============================================================================
 *
 * Generic properties are the primary future-extension mechanism.
 *
 * This prevents every new architecture/toolchain/ABI/environment concept from
 * becoming a reserved keyword.
 *
 * Examples:
 *
 *     property sysroot = path;
 *
 *     property toolchain = toolchain::name;
 *
 *     property abi = abi::name;
 *
 *     property format = format::name;
 *
 *     property architecture = architecture::name;
 *
 *     property environment = environment::name;
 *
 *     property linker = linker::name;
 *
 *     property compiler = compiler::name;
 *
 *     property host = host::name;
 *
 *     property destination = target::name;
 *
 * The semantic layer decides which properties are recognized and what they
 * mean.
 */

crossCompilationProperty
    : PROPERTY crossCompilationPropertyName
      crossCompilationPropertyValue?
      SEMI?
    ;

crossCompilationPropertyName
    : identifier
    | qualifiedName
    ;

crossCompilationPropertyValue
    : ASSIGN expression
    | COLON expression
    | crossCompilationPropertyBlock
    ;

crossCompilationPropertyBlock
    : LBRACE crossCompilationPropertyEntry* RBRACE
    ;

crossCompilationPropertyEntry
    : crossCompilationPropertyName
      (ASSIGN | COLON)
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * 21. SOURCE / DESTINATION PROPERTY SEPARATION
 * ============================================================================
 *
 * Generic properties may explicitly describe which side they belong to.
 *
 * The grammar does not force a fixed vocabulary.
 *
 * Examples:
 *
 *     property source {
 *         abi = source::abi;
 *     }
 *
 *     property destination {
 *         abi = destination::abi;
 *     }
 *
 * Semantic validation determines whether the property is meaningful.
 *
 * This keeps source/destination semantics extensible.
 */


/*
 * ============================================================================
 * 22. TOOLCHAIN CONTRACT
 * ============================================================================
 *
 * Toolchains are implementation resources, not language semantics.
 *
 * A source program may express:
 *
 *     property toolchain = toolchain::name;
 *
 * or:
 *
 *     prefer toolchain::name;
 *
 * but the grammar MUST NOT require:
 *
 *     gcc
 *     clang
 *     rustc
 *     nvcc
 *     vendor-specific compiler
 *
 * as universal language constructs.
 *
 * The actual toolchain is selected downstream.
 */


/*
 * ============================================================================
 * 23. SYSROOT CONTRACT
 * ============================================================================
 *
 * A sysroot is an implementation/environment property.
 *
 * It MUST NOT become a language-level filesystem requirement.
 *
 * Example:
 *
 *     property sysroot = configured_sysroot;
 *
 * The parser stores the expression.
 *
 * The compiler/toolchain layer resolves the actual value.
 *
 * The grammar does not access the filesystem.
 */


/*
 * ============================================================================
 * 24. ABI CONTRACT
 * ============================================================================
 *
 * ABI intent may be expressed as a property or requirement.
 *
 * Examples:
 *
 *     requires capability("abi.compatible");
 *
 *     property abi = abi::portable;
 *
 *     property calling_convention = convention::compatible;
 *
 * ABI compatibility is distinct from:
 *
 *     processor architecture;
 *     target identity;
 *     source compatibility;
 *     semantic compatibility;
 *     IR compatibility.
 *
 * The grammar does not implement ABI lowering.
 */


/*
 * ============================================================================
 * 25. HOST / BUILD / TARGET SEPARATION
 * ============================================================================
 *
 * Cross compilation commonly involves multiple environments:
 *
 *     build environment
 *     host environment
 *     destination/target environment
 *
 * The grammar MUST NOT assume they are identical.
 *
 * They are represented through target/property expressions rather than
 * physical-machine assumptions.
 *
 * A semantic model may therefore distinguish:
 *
 *     build target
 *     execution target
 *     artifact target
 *
 * without changing the grammar.
 */


/*
 * ============================================================================
 * 26. TARGET ADAPTATION
 * ============================================================================
 *
 * Cross-compilation MAY require downstream adaptation of:
 *
 *     instruction selection;
 *     data layout;
 *     calling convention;
 *     ABI;
 *     memory placement;
 *     vectorization;
 *     accelerator mapping;
 *     quantum operation decomposition;
 *     quantum routing;
 *     scheduling;
 *     HDL synthesis;
 *     hardware realization;
 *     distributed placement.
 *
 * Such adaptation MUST preserve the source program's semantic contract.
 *
 * The grammar does not implement any of these transformations.
 */


/*
 * ============================================================================
 * 27. CLASSICAL CROSS-COMPILATION
 * ============================================================================
 *
 * Classical programs may cross-compile between:
 *
 *     CPU architectures;
 *     operating environments;
 *     embedded environments;
 *     accelerators;
 *     portable runtimes;
 *     virtual machines;
 *     future execution substrates.
 *
 * No architecture list is embedded in this grammar.
 */


/*
 * ============================================================================
 * 28. QUANTUM CROSS-COMPILATION
 * ============================================================================
 *
 * Quantum source may cross-compile between:
 *
 *     quantum simulators;
 *     QPUs;
 *     logical quantum environments;
 *     physical quantum targets;
 *     hybrid targets;
 *     future quantum architectures.
 *
 * The source operation model remains generic.
 *
 * No gate set is enumerated here.
 *
 * Quantum source must eventually follow:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *
 * Cross-compilation MUST NOT introduce another quantum IR.
 */


/*
 * ============================================================================
 * 29. HDL / HARDWARE CROSS-COMPILATION
 * ============================================================================
 *
 * HDL/hardware programs may be adapted for:
 *
 *     FPGA;
 *     ASIC;
 *     programmable accelerators;
 *     simulation;
 *     emulation;
 *     future hardware substrates.
 *
 * This grammar does not encode:
 *
 *     register width;
 *     physical pin count;
 *     clock frequency;
 *     FPGA resource count;
 *     ASIC cell count;
 *     physical topology.
 *
 * Those are semantic/resource/target realization properties.
 */


/*
 * ============================================================================
 * 30. HYBRID CROSS-COMPILATION
 * ============================================================================
 *
 * A hybrid program may cross-compile its components independently while
 * preserving the semantic relationship between:
 *
 *     classical computation;
 *     quantum computation;
 *     accelerator computation;
 *     HDL/hardware computation;
 *     distributed computation.
 *
 * Cross-compilation therefore operates at the compilation-plan level rather
 * than requiring separate languages for each domain.
 */


/*
 * ============================================================================
 * 31. DISTRIBUTED CROSS-COMPILATION
 * ============================================================================
 *
 * A distributed program may be cross-compiled without specifying:
 *
 *     node count;
 *     node identifiers;
 *     network topology;
 *     CPU count;
 *     GPU count;
 *     memory size.
 *
 * Those are downstream realization constraints.
 *
 * A program may instead express:
 *
 *     requires capability("distributed.execution");
 *
 *     requires topology(...);
 *
 *     resource bandwidth >= required_bandwidth;
 *
 * where such expressions are interpreted downstream.
 */


/*
 * ============================================================================
 * 32. DATA / AI CROSS-COMPILATION
 * ============================================================================
 *
 * AI/data programs may cross-compile between:
 *
 *     CPU;
 *     GPU;
 *     accelerator;
 *     distributed;
 *     tensor-processing;
 *     simulator;
 *     future compute substrates.
 *
 * Framework-specific names remain semantic properties or interoperability
 * declarations rather than universal grammar keywords.
 */


/*
 * ============================================================================
 * 33. ARTIFACT MODEL
 * ============================================================================
 *
 * Cross-compilation MUST distinguish:
 *
 *     source artifact
 *     semantic artifact
 *     target-independent compilation artifact
 *     target-specific artifact
 *     binary artifact
 *     deployment artifact
 *
 * A target-specific artifact MUST NOT silently become the canonical semantic
 * representation.
 *
 * The same semantic artifact may produce multiple target artifacts:
 *
 *     one source
 *        |
 *        v
 *     one semantic meaning
 *        |
 *        +-------> target A artifact
 *        |
 *        +-------> target B artifact
 *        |
 *        +-------> target C artifact
 *        |
 *        +-------> future target artifact
 *
 * This is a central POCO-REAF property.
 */


/*
 * ============================================================================
 * 34. COMPILE-ONCE SEMANTIC IDENTITY
 * ============================================================================
 *
 * Cross-compilation MUST preserve semantic identity.
 *
 * A target artifact may differ in:
 *
 *     instruction set;
 *     data layout;
 *     scheduling;
 *     routing;
 *     optimization;
 *     ABI;
 *     linker format;
 *     memory placement;
 *     device mapping.
 *
 * Those differences do not constitute different source semantics.
 *
 * Persistent cross-compilation identities belong to the compilation/provenance
 * layer, not to frontend AST node identity.
 */


/*
 * ============================================================================
 * 35. RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Valid:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("quantum.measurement");
 *
 * Invalid architectural interpretation:
 *
 *     MAX_QUBITS = ...
 *
 *     use physical_qubit(17);
 *
 *     use gpu0;
 *
 *     use cpu0;
 *
 * as universal cross-compilation semantics.
 *
 * Physical mapping belongs downstream.
 */


/*
 * ============================================================================
 * 36. SOURCE SEMANTICS VERSUS IMPLEMENTATION OPTIONS
 * ============================================================================
 *
 * Cross-compilation properties must be classified downstream as one of:
 *
 *     semantic requirement
 *     implementation constraint
 *     implementation preference
 *     implementation hint
 *     target realization
 *
 * A compiler MUST NOT silently promote:
 *
 *     hint -> requirement
 *     preference -> requirement
 *     target property -> source semantic
 *     implementation choice -> language rule
 *
 * without an explicit semantic contract.
 */


/*
 * ============================================================================
 * 37. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO IR.
 *
 * It MUST NOT define:
 *
 *     CrossCompilationIR
 *     TargetCompilationIR
 *     ToolchainIR
 *     AbiIR
 *     QuantumCrossCompilationIR
 *
 * Cross-compilation intent is represented in the existing domain-neutral
 * semantic compilation model.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * as the single canonical quantum IR boundary.
 */


/*
 * ============================================================================
 * 38. SEMANTIC VALIDATION CONTRACT
 * ============================================================================
 *
 * The semantic layer is responsible for validating:
 *
 *     source target existence;
 *     destination target validity;
 *     target compatibility;
 *     capability satisfaction;
 *     resource satisfaction;
 *     ABI compatibility;
 *     artifact compatibility;
 *     toolchain availability;
 *     sysroot compatibility;
 *     linker compatibility;
 *     format compatibility;
 *     language-version compatibility;
 *     dialect compatibility;
 *     interoperability compatibility.
 *
 * Parser acceptance MUST NOT be interpreted as successful cross-compilation.
 */


/*
 * ============================================================================
 * 39. ERROR BOUNDARIES
 * ============================================================================
 *
 * Errors MUST occur at the earliest correct layer.
 *
 * Lexical problem
 *     -> lexer
 *
 * Malformed cross-compilation syntax
 *     -> parser
 *
 * Unknown target
 *     -> semantic target analysis
 *
 * Unsatisfied capability
 *     -> capability analysis
 *
 * Insufficient resource
 *     -> resource analysis
 *
 * Unsupported ABI
 *     -> ABI compatibility analysis
 *
 * Missing toolchain
 *     -> compilation environment
 *
 * Invalid sysroot
 *     -> toolchain/environment layer
 *
 * Unsupported target lowering
 *     -> backend/lowering layer
 *
 * Linker failure
 *     -> linker
 *
 * Runtime failure
 *     -> runtime/resilience
 *
 * The grammar MUST NOT convert downstream failures into parser actions.
 */


/*
 * ============================================================================
 * 40. DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to:
 *
 *     source tokens
 *     language version
 *     grammar version
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     host architecture;
 *     CPU count;
 *     GPU availability;
 *     QPU availability;
 *     network state;
 *     filesystem state;
 *     environment variables;
 *     wall-clock time;
 *     randomness.
 */


/*
 * ============================================================================
 * 41. SECURITY
 * ============================================================================
 *
 * Cross-compilation syntax MUST NOT grant authority.
 *
 * It cannot by itself grant:
 *
 *     filesystem access;
 *     network access;
 *     command execution;
 *     credentials;
 *     compiler process execution;
 *     linker execution;
 *     device access;
 *     hardware access.
 *
 * Toolchain execution and external-process access are controlled by the
 * compiler/build security model.
 */


/*
 * ============================================================================
 * 42. RESOURCE-SAFE SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no finite limits on:
 *
 *     number of cross-compilation entries;
 *     number of requirements;
 *     number of constraints;
 *     number of preferences;
 *     number of hints;
 *     number of artifacts;
 *     number of properties;
 *     target-expression depth;
 *     target alternatives.
 *
 * Practical compiler resource guards MAY exist.
 *
 * Such guards MUST be:
 *
 *     explicit;
 *     implementation-level;
 *     configurable where appropriate;
 *     diagnosable;
 *     separate from language semantics.
 *
 * They MUST NOT be exposed as universal Zamani language limits.
 */


/*
 * ============================================================================
 * 43. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve a structure conceptually equivalent to:
 *
 *     CrossCompilation
 *         source_target
 *         destination_target
 *         requirements[]
 *         constraints[]
 *         preferences[]
 *         hints[]
 *         capabilities[]
 *         resources[]
 *         artifacts[]
 *         profiles[]
 *         specialization
 *         optimization
 *         reproducibility
 *         determinism
 *         properties[]
 *         source_span
 *
 * Target expressions remain unresolved until semantic analysis.
 *
 * Properties preserve their source spelling and source spans.
 *
 * No physical device node should be constructed at this stage.
 */


/*
 * ============================================================================
 * 44. SEMANTIC MODEL CONTRACT
 * ============================================================================
 *
 * Semantic analysis should transform:
 *
 *     CrossCompilation AST
 *
 * into a target-independent semantic cross-compilation request containing:
 *
 *     source target intent;
 *     destination target intent;
 *     semantic requirements;
 *     semantic constraints;
 *     implementation preferences;
 *     non-binding hints;
 *     capability requirements;
 *     resource requirements;
 *     artifact requirements;
 *     compatibility requirements;
 *     provenance inputs.
 *
 * It may then resolve:
 *
 *     target capabilities;
 *     toolchains;
 *     sysroots;
 *     ABIs;
 *     formats;
 *     linker capabilities;
 *     lowering paths.
 *
 * None of those resolutions belongs in this parser.
 */


/*
 * ============================================================================
 * 45. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler should consume the semantic cross-compilation model to build:
 *
 *     source semantic representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     target-independent artifact where possible
 *          |
 *          v
 *     target-specific lowering
 *          |
 *          v
 *     target artifact
 *
 * For quantum:
 *
 *     canonical quantum semantics
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     target-specific decomposition/routing/scheduling
 *
 * For HDL:
 *
 *     hardware intent
 *          |
 *          v
 *     HDL/hardware representation
 *          |
 *          v
 *     synthesis/lowering
 *
 * No cross-compilation-specific IR is introduced.
 */


/*
 * ============================================================================
 * 46. LINKER INTEGRATION
 * ============================================================================
 *
 * The repository's linker implementation is downstream.
 *
 * In particular, cross-compilation grammar MUST NOT attempt to model:
 *
 *     linker process execution;
 *     command-line invocation;
 *     host process state;
 *     filesystem output;
 *     native linker implementation.
 *
 * Existing linker configuration concepts such as:
 *
 *     target;
 *     arguments;
 *     sysroot;
 *
 * remain implementation-level concerns.
 *
 * A source-level cross-compilation property may describe semantic intent that
 * later becomes linker configuration, but the parser must never execute or
 * validate the linker itself.
 */


/*
 * ============================================================================
 * 47. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Cross-compilation provenance should distinguish:
 *
 * semantic inputs
 *
 * from:
 *
 * target realization inputs.
 *
 * Provenance may include:
 *
 *     language version;
 *     source identity;
 *     grammar version;
 *     compiler version;
 *     dialect versions;
 *     dependency versions;
 *     source target;
 *     destination target;
 *     selected toolchain;
 *     selected ABI;
 *     sysroot identity;
 *     target description;
 *     resource context.
 *
 * The grammar does not generate provenance records itself.
 */


/*
 * ============================================================================
 * 48. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducible cross-compilation requires all semantic and realization inputs
 * that affect the resulting artifact to be identified.
 *
 * The grammar may carry intent.
 *
 * The reproducibility subsystem determines the actual cache/provenance inputs.
 *
 * A target artifact MUST NOT be reused merely because:
 *
 *     source text matches.
 *
 * Relevant target and compilation inputs must also match where they affect
 * artifact semantics.
 */


/*
 * ============================================================================
 * 49. COMPATIBILITY
 * ============================================================================
 *
 * Cross-compilation compatibility is multidimensional.
 *
 * The repository must distinguish:
 *
 *     source compatibility
 *     AST compatibility
 *     semantic compatibility
 *     IR compatibility
 *     ABI compatibility
 *     binary compatibility
 *     runtime compatibility
 *     target compatibility
 *     interoperability compatibility
 *
 * Successful parsing does not establish any of these downstream properties.
 */


/*
 * ============================================================================
 * 50. DIALECT INTEGRATION
 * ============================================================================
 *
 * A dialect may extend cross-compilation properties only through the normal
 * dialect mechanism.
 *
 * A dialect MUST NOT:
 *
 *     redefine crossCompilationClause;
 *     redefine targetExpression;
 *     introduce a second cross-compilation grammar;
 *     silently change the meaning of `->`;
 *     silently turn preferences into requirements.
 *
 * Dialect-specific properties remain explicitly namespaced where appropriate.
 */


/*
 * ============================================================================
 * 51. FUTURE COMPUTING SUBSTRATES
 * ============================================================================
 *
 * This grammar deliberately assumes no final hardware taxonomy.
 *
 * Future targets may include computational substrates not currently known.
 *
 * Existing source syntax remains usable when a future substrate can satisfy
 * the existing semantic contract.
 *
 * If genuinely new semantics are required, they enter through the normal:
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
 *
 * lifecycle.
 */


/*
 * ============================================================================
 * 52. TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests MUST include:
 *
 *     compile cross host -> wasm;
 *
 *     compile cross x86_64 -> aarch64;
 *
 *     compile cross classical -> quantum;
 *
 *     compile cross cpu -> accelerator {
 *         requires capability("tensor.compute");
 *     }
 *
 *     compile cross source::target -> destination::target {
 *         property abi = abi::portable;
 *         property toolchain = toolchain::portable;
 *     }
 *
 *     compile cross host -> target {
 *         requires memory >= required_memory;
 *         prefer optimization::portable;
 *         hint deployment::adaptive;
 *     }
 *
 * Negative tests MUST include:
 *
 *     compile cross;
 *
 *     compile cross host;
 *
 *     compile cross -> wasm;
 *
 *     compile cross host ->;
 *
 *     compile cross host wasm;
 *
 *     compile cross host -> wasm { requires; }
 *
 *     compile cross host -> wasm { property; }
 *
 * Boundary tests MUST include:
 *
 *     deeply qualified target names;
 *
 *     target expressions containing alternatives;
 *
 *     target intersections;
 *
 *     target differences;
 *
 *     large requirement sets;
 *
 *     large property sets;
 *
 *     deeply nested property blocks;
 *
 *     many cross-compilation clauses;
 *
 *     large artifact sets.
 *
 * Scalability tests MUST verify that no finite hardware cardinality is
 * introduced by the grammar.
 *
 * Determinism tests MUST verify identical token streams produce identical
 * parse structures.
 *
 * Cross-domain tests MUST cover:
 *
 *     classical -> classical
 *     classical -> quantum
 *     quantum -> quantum
 *     quantum -> classical
 *     classical -> HDL/hardware
 *     hybrid -> accelerator
 *     distributed -> distributed
 *     AI -> accelerator
 *     data -> distributed
 *     future-domain -> future-domain
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level constructs include:
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
 * Also prohibited as universal semantics:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     device0
 *     physical_qubit(17)
 *
 * when treated as compiler-wide mandatory realization rules.
 *
 * A concrete numeric value appearing inside an expression remains valid when
 * it is actual program data or an explicit semantic requirement.
 *
 * Example:
 *
 *     requires memory >= 64GiB;
 *
 * may be legitimate program intent.
 *
 * What is prohibited is turning:
 *
 *     64GiB
 *
 * into a universal language/compiler maximum or assumed hardware capacity.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * Every public construct must retain enough source-span information for:
 *
 *     diagnostics;
 *     IDE tooling;
 *     formatter;
 *     provenance;
 *     compatibility diagnostics;
 *     migration tooling.
 *
 * At minimum, semantic diagnostics should be able to identify:
 *
 *     `cross`;
 *     source target;
 *     destination target;
 *     arrow;
 *     body;
 *     offending property/requirement.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar must remain parser-oriented.
 *
 * It must not:
 *
 *     perform target discovery;
 *     enumerate hardware;
 *     resolve toolchains;
 *     inspect filesystem trees;
 *     invoke external commands;
 *     perform expensive semantic queries.
 *
 * Potentially expensive work belongs downstream.
 *
 * Repetition is intentionally expressed using ANTLR repetition operators.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] cross-compilation is represented as a compile clause;
 * [x] no second `compile` declaration exists;
 * [x] source target is represented by targetExpression;
 * [x] destination target is represented by targetExpression;
 * [x] THIN_ARROW is reused;
 * [x] identifiers are reused from Core;
 * [x] expressions are reused from Expressions;
 * [x] target syntax is reused from CompileTarget;
 * [x] no target enumeration exists;
 * [x] no hardware enumeration exists;
 * [x] no toolchain enumeration exists;
 * [x] no ABI enumeration exists;
 * [x] no sysroot implementation exists;
 * [x] no filesystem access exists;
 * [x] no network access exists;
 * [x] no command execution exists;
 * [x] no linker execution exists;
 * [x] no quantum IR is introduced;
 * [x] quantum::ir remains canonical;
 * [x] requirement/constraint/preference/hint distinctions remain explicit;
 * [x] resource/capability semantics remain downstream;
 * [x] target realization remains downstream;
 * [x] artifact identity remains separate from source semantics;
 * [x] provenance remains downstream;
 * [x] reproducibility remains downstream;
 * [x] deterministic parsing is preserved;
 * [x] no artificial cardinality limits exist;
 * [x] Rust 1.97 / 1.97.1 compatibility is documented;
 * [x] safe Rust / no-unsafe contract is documented;
 * [x] positive tests are specified;
 * [x] negative tests are specified;
 * [x] boundary tests are specified;
 * [x] scalability tests are specified;
 * [x] cross-domain tests are specified;
 * [x] hard-coding audit is specified.
 *
 * ============================================================================
 * REQUIRED INTEGRATION CHANGES
 * ============================================================================
 *
 * This file is independently complete as a parser component, but two
 * repository integration changes are required before the complete parser can
 * consume it.
 *
 * 1. Add the `CROSS` token to:
 *
 *     grammar/lexer/keywords.g4
 *
 * exactly once.
 *
 * 2. Add this parser grammar to the compile composition:
 *
 *     grammar/compile/compile.g4
 *
 * by importing:
 *
 *     CompileCrossCompilation
 *
 * and adding:
 *
 *     | crossCompilationClause
 *
 * to:
 *
 *     compileClause
 *
 * No other grammar should redefine crossCompilationClause.
 *
 * The canonical composition then becomes:
 *
 *     compileDeclaration
 *         |
 *         v
 *     compileSpecification
 *         |
 *         +--> compileClause
 *                |
 *                +--> crossCompilationClause
 *                         |
 *                         +--> source target
 *                         |
 *                         +--> destination target
 *
 * `grammar/compile/compilation.g4` should then consume the resulting
 * `compileDeclaration` through the existing Compile composition rather than
 * creating a second cross-compilation entry point.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Cross-compilation does NOT mean:
 *
 *     rewrite the program for another machine.
 *
 * It means:
 *
 *     preserve the program's semantic identity while allowing the compiler
 *     to derive another valid realization.
 *
 * Therefore:
 *
 *     ONE SOURCE PROGRAM
 *          |
 *          v
 *     ONE SEMANTIC MEANING
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *      TARGET A             TARGET B
 *          |                   |
 *          v                   v
 *     artifact A           artifact B
 *
 * and potentially:
 *
 *          +-------------------+
 *          |
 *          v
 *       TARGET N
 *
 * with no language-level limit on N.
 *
 * This is the cross-compilation component of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */