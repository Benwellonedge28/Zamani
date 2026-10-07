/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/compile/compile.g4
 *
 * GRAMMAR
 * -------
 * Compile
 *
 * STATUS
 * ------
 * PRODUCTION COMPOSITION ROOT
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE composition root for the source-level compilation
 * subsystem.
 *
 * It does not implement the individual compilation features. Instead, it
 * composes their independently-owned parser grammars behind one stable
 * `Compile` boundary.
 *
 * The compilation subsystem covers:
 *
 *     intent
 *     profiles
 *     compile-time control
 *     conditional compilation
 *     feature selection
 *     target intent
 *     target selection
 *     optimization intent
 *     specialization intent
 *     artifact intent
 *     code-generation intent
 *     lowering intent
 *     cross-compilation
 *     reproducibility
 *     deterministic builds
 *     caching
 *     provenance
 *     deployment intent
 *
 * This file therefore answers:
 *
 *     "Which source-level compilation constructs are being composed?"
 *
 * It does NOT answer:
 *
 *     "Which physical machine executes them?"
 *
 *     "Which device is allocated?"
 *
 *     "How are resources discovered?"
 *
 *     "How are quantum operations routed?"
 *
 *     "How are operations scheduled?"
 *
 *     "How is QEC performed?"
 *
 *     "How does the HAL realize the result?"
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Compile
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> names
 *       +--> types
 *       +--> effects
 *       +--> resources
 *       +--> capabilities
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       +--> portability
 *       |
 *       v
 *     semantic compilation model
 *       |
 *       +-------------------+---------------------+
 *       |                   |                     |
 *       v                   v                     v
 *   classical          quantum::ir          HDL/hardware
 *       |                   |                     |
 *       +-------------------+---------------------+
 *                           |
 *                           v
 *                      optimization
 *                           |
 *                           v
 *                      specialization
 *                           |
 *                           v
 *                        lowering
 *                           |
 *                           v
 *                  routing / scheduling
 *                           |
 *                           v
 *                   resilience / QEC
 *                           |
 *                           v
 *                          ZQN
 *                           |
 *                           v
 *                          HAL
 *                           |
 *                           v
 *                  target realization
 *                           |
 *                           v
 *                      deployment/runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Compilation syntax MUST describe portable computational intent.
 *
 * A program may therefore be compiled for different realizations without
 * requiring a change to the source grammar merely because the realization
 * changes.
 *
 * The compilation grammar MUST NOT establish artificial universal limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     QPUs
 *     accelerators
 *     nodes
 *     processes
 *     devices
 *     memory
 *     storage
 *     registers
 *     register width
 *     vector width
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     topology size
 *     channels
 *     targets
 *     artifacts
 *     profiles
 *     compilation stages
 *     optimization objectives
 *     transformations
 *     requirements
 *     constraints
 *     capabilities
 *
 * No MAX_* language-level capacity constants are permitted.
 *
 * "Scale to infinity" means that the language does not establish an
 * artificial finite machine ceiling.
 *
 * Physical and implementation limits remain properties of:
 *
 *     compiler resources
 *     target capabilities
 *     resource availability
 *     operating environments
 *     runtime environments
 *     deployment environments
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the parser grammar named `Compile`;
 *     - the public `compileDeclaration` entry point;
 *     - the compilation specification boundary;
 *     - compilation-clause dispatch;
 *     - stable adapter rules connecting the compilation root to its
 *       independently-owned compilation grammars;
 *     - the distinction between compilation syntax and downstream
 *       compilation semantics.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - ordinary declarations;
 *     - ordinary statements;
 *     - resource definitions;
 *     - capability definitions;
 *     - hardware descriptions;
 *     - optimization algorithms;
 *     - specialization algorithms;
 *     - lowering algorithms;
 *     - code generators;
 *     - artifact serializers;
 *     - cache implementations;
 *     - deployment implementations;
 *     - target discovery;
 *     - resource allocation;
 *     - routing;
 *     - scheduling;
 *     - resilience;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - AST construction;
 *     - semantic analysis;
 *     - canonical IR construction;
 *     - `quantum::ir` implementation.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Every detailed compilation concern MUST have exactly one syntax authority.
 *
 * The composition root delegates to that authority.
 *
 * It MUST NOT copy the internal rules of:
 *
 *     profiles.g4
 *     compile-time.g4
 *     conditional-compilation.g4
 *     feature-selection.g4
 *     target.g4
 *     target-selection.g4
 *     optimization.g4
 *     specialization.g4
 *     artifacts.g4
 *     code-generation.g4
 *     lowering.g4
 *     cross-compilation.g4
 *     reproducibility.g4
 *     deterministic-builds.g4
 *     caching.g4
 *     provenance.g4
 *     deployment.g4
 *
 * ============================================================================
 * COMPOSITION-ROOT RULE
 * ============================================================================
 *
 * `Compile` is the canonical compilation composition root.
 *
 * `grammar/Zamani.g4`
 *     remains the complete-language composition root.
 *
 * `grammar/antlr/ZamaniParser.g4`
 *     imports `Compile` and exposes compilation through `compileElement`.
 *
 * This file MUST therefore remain usable as a parser-library component.
 *
 * ============================================================================
 * NON-ROOT COMPILATION FILES
 * ============================================================================
 *
 * The following existing files are deliberately NOT imported here:
 *
 *     grammar/compile/compilation.g4
 *     grammar/compile/intent.g4
 *
 * `compilation.g4` is an overlapping legacy composition root that already
 * imports `Compile`. Importing it here would create a composition cycle and
 * duplicate authority.
 *
 * `intent.g4` already imports `Compile`. Importing it here would likewise
 * create:
 *
 *     Compile -> CompileIntent -> Compile
 *
 * and is therefore prohibited.
 *
 * Their future role is:
 *
 *     compatibility adapter / migration boundary
 *
 * rather than independent compilation authorities.
 *
 * ============================================================================
 * FEATURE-GRAMMAR OWNERSHIP
 * ============================================================================
 *
 * `features.g4` provides the canonical reusable feature-reference machinery.
 *
 * `feature-selection.g4` owns feature-selection structure.
 *
 * They MUST have a one-way dependency:
 *
 *     CompileFeatures
 *          |
 *          v
 *     FeatureSelection
 *          |
 *          v
 *       Compile
 *
 * The two grammars MUST NOT independently define duplicate:
 *
 *     featureName
 *     featureReference
 *     featureNameList
 *
 * rules.
 *
 * Until that dependency is normalized, `Compile` MUST NOT directly import both
 * grammars because ANTLR would receive duplicate imported rule authorities.
 *
 * `Compile` therefore consumes the feature-selection composition boundary.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *       Compile
 *          |
 *          +--> CompileProfiles
 *          +--> CompileTime
 *          +--> ConditionalCompilation
 *          +--> FeatureSelection
 *          +--> CompileTarget
 *          +--> CompileTargetSelection
 *          +--> CompileOptimization
 *          +--> CompileSpecialization
 *          +--> CompileArtifacts
 *          +--> CompileCodeGeneration
 *          +--> CompileLowering
 *          +--> CompileCrossCompilation
 *          +--> CompileReproducibility
 *          +--> CompileDeterministicBuilds
 *          +--> CompileProvenance
 *          +--> CompileCaching
 *          +--> CompileDeployment
 *
 * A dedicated grammar MUST NOT import this composition root.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * No lexer rules are declared here.
 *
 * No embedded Rust actions are permitted.
 *
 * No semantic predicates are required.
 *
 * No filesystem, network, hardware, environment, or runtime state may be
 * inspected while parsing.
 *
 * ============================================================================
 */

parser grammar Compile;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * COMPILATION COMPONENT IMPORTS
 * ============================================================================
 *
 * Each imported grammar is an independently-owned syntax component.
 *
 * The imports below intentionally use grammar names rather than filesystem
 * paths.
 *
 * ANTLR resolves them through the configured grammar/import search path.
 * ============================================================================
 */

import
    CompileProfiles,
    CompileTime,
    ConditionalCompilation,
    FeatureSelection,
    CompileTarget,
    CompileTargetSelection,
    CompileOptimization,
    CompileSpecialization,
    CompileArtifacts,
    CompileCodeGeneration,
    CompileLowering,
    CompileCrossCompilation,
    CompileReproducibility,
    CompileDeterministicBuilds,
    CompileProvenance,
    CompileCaching,
    CompileDeployment
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A compile declaration begins with the canonical COMPILE token.
 *
 * The declaration contains one or more compilation clauses.
 *
 * No finite number of clauses is imposed.
 *
 * ============================================================================
 */

compileDeclaration
    : COMPILE compileSpecification SEMI?
    ;


/*
 * ============================================================================
 * COMPILATION SPECIFICATION
 * ============================================================================
 *
 * One or more compilation clauses form the compilation specification.
 *
 * Repetition is intentionally structural rather than bounded.
 * ============================================================================
 */

compileSpecification
    : compileClause+
    ;


/*
 * ============================================================================
 * COMPILATION CLAUSE DISPATCH
 * ============================================================================
 *
 * This is the central orchestration point of the compilation subsystem.
 *
 * The rules below are adapters only.
 *
 * Detailed syntax remains owned by the imported grammar that defines the
 * referenced rule.
 * ============================================================================
 */

compileClause
    : compileProfileReference
    | compileTimeControlReference
    | conditionalCompilationReference
    | featureSelectionReference
    | compileTargetReference
    | compileTargetSelectionReference
    | compileOptimizationReference
    | compileSpecializationReference
    | compileArtifactReference
    | compileCodeGenerationReference
    | compileLoweringReference
    | compileCrossCompilationReference
    | compileReproducibilityReference
    | compileDeterministicBuildReference
    | compileProvenanceReference
    | compileCachingReference
    | compileDeploymentReference
    ;


/*
 * ============================================================================
 * PROFILE
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/profiles.g4
 *
 * Actual public rule:
 *
 *     compileProfile
 *
 * The previous `compileProfileDeclaration` reference was invalid because that
 * rule does not exist in the current repository.
 * ============================================================================
 */

compileProfileReference
    : compileProfile
    ;


/*
 * ============================================================================
 * COMPILE-TIME CONTROL
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/compile-time.g4
 *
 * Compile-time control remains distinct from ordinary runtime execution.
 * ============================================================================
 */

compileTimeControlReference
    : compileTimeControl
    ;


/*
 * ============================================================================
 * CONDITIONAL COMPILATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/conditional-compilation.g4
 *
 * Conditional compilation selects source constructs according to compile-time
 * predicates.
 *
 * It does not perform target discovery or runtime execution.
 * ============================================================================
 */

conditionalCompilationReference
    : conditionalCompilation
    ;


/*
 * ============================================================================
 * FEATURE SELECTION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/feature-selection.g4
 *
 * Feature identity and feature-reference semantics remain owned by the
 * canonical feature grammar relationship described above.
 * ============================================================================
 */

featureSelectionReference
    : featureSelection
    ;


/*
 * ============================================================================
 * TARGET INTENT
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/target.g4
 *
 * Target intent describes acceptable realization classes.
 *
 * It does not select or allocate physical devices.
 * ============================================================================
 */

compileTargetReference
    : targetDeclaration
    ;


/*
 * ============================================================================
 * TARGET SELECTION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/target-selection.g4
 *
 * Selection is policy.
 *
 * Physical realization remains downstream.
 * ============================================================================
 */

compileTargetSelectionReference
    : targetSelectionDeclaration
    ;


/*
 * ============================================================================
 * OPTIMIZATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/optimization.g4
 *
 * The grammar represents optimization intent only.
 *
 * Optimization algorithms remain compiler implementation concerns.
 * ============================================================================
 */

compileOptimizationReference
    : optimizationDeclaration
    ;


/*
 * ============================================================================
 * SPECIALIZATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/specialization.g4
 *
 * Specialization remains distinct from metaprogramming and from target
 * selection.
 * ============================================================================
 */

compileSpecializationReference
    : compileSpecializationDeclaration
    ;


/*
 * ============================================================================
 * ARTIFACTS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/artifacts.g4
 *
 * The existing artifact grammar does not export `artifactDeclaration`.
 *
 * Its authoritative reusable entry is:
 *
 *     compileArtifactSpecification
 *
 * Therefore the COMPILE composition boundary supplies the ARTIFACT keyword
 * while the artifact grammar owns the specification following it.
 *
 * This fixes the invalid `artifactDeclaration` reference in the previous
 * version without duplicating artifact internals.
 * ============================================================================
 */

compileArtifactReference
    : ARTIFACT compileArtifactSpecification
    ;


/*
 * ============================================================================
 * CODE GENERATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/code-generation.g4
 * ============================================================================
 */

compileCodeGenerationReference
    : codeGenerationDeclaration
    ;


/*
 * ============================================================================
 * LOWERING
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/lowering.g4
 *
 * Lowering transforms one semantic representation into another permitted
 * representation.
 *
 * It does not define a second IR.
 *
 * Quantum transformations continue toward:
 *
 *     quantum::ir
 *
 * and subsequently toward domain-specific realization.
 * ============================================================================
 */

compileLoweringReference
    : loweringDeclaration
    ;


/*
 * ============================================================================
 * CROSS-COMPILATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/cross-compilation.g4
 *
 * Cross-compilation expresses source/realization portability.
 *
 * It does not define a finite target catalogue.
 * ============================================================================
 */

compileCrossCompilationReference
    : crossCompilationClause
    ;


/*
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/reproducibility.g4
 *
 * Reproducibility intent describes requirements for repeatable compilation
 * results and traceable compilation inputs.
 * ============================================================================
 */

compileReproducibilityReference
    : reproducibilityDeclaration
    ;


/*
 * ============================================================================
 * DETERMINISTIC BUILDS
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 * Deterministic build intent is kept separate from the broader reproducibility
 * declaration because the repository already provides both authorities.
 * ============================================================================
 */

compileDeterministicBuildReference
    : deterministicBuildClause
    ;


/*
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/provenance.g4
 *
 * Provenance records compilation relationships such as:
 *
 *     source
 *     derived-from
 *     transformed-by
 *     verified-by
 *     justified-by
 *     generated-from
 *
 * The grammar only preserves source intent.
 *
 * The actual provenance graph/record implementation remains downstream.
 * ============================================================================
 */

compileProvenanceReference
    : compileProvenanceDeclaration
    ;


/*
 * ============================================================================
 * CACHING
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/caching.g4
 *
 * Cache syntax expresses reuse intent.
 *
 * It does not grant filesystem, network, remote-cache, or provider access.
 * ============================================================================
 */

compileCachingReference
    : cachingDeclaration
    ;


/*
 * ============================================================================
 * DEPLOYMENT
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/compile/deployment.g4
 *
 * Deployment intent remains separate from deployment execution.
 * ============================================================================
 */

compileDeploymentReference
    : compileDeploymentDeclaration
    ;


/*
 * ============================================================================
 * COMPOSITION INVARIANTS
 * ============================================================================
 *
 * 1. ONE LEXER
 *
 *     All tokens originate from ZamaniLexer.
 *
 * 2. ONE COMPILATION ROOT
 *
 *     Compile is the canonical compilation composition root.
 *
 * 3. ONE COMPLETE-LANGUAGE ROOT
 *
 *     grammar/Zamani.g4 remains the complete-language root.
 *
 * 4. ONE OWNER PER FEATURE
 *
 *     Detailed syntax belongs to its dedicated grammar.
 *
 * 5. NO DUPLICATE EXPRESSIONS
 *
 *     This grammar does not redefine expression syntax.
 *
 * 6. NO DUPLICATE TYPES
 *
 *     This grammar does not redefine type syntax.
 *
 * 7. NO DUPLICATE IDENTIFIERS
 *
 *     This grammar consumes canonical identifier/name rules.
 *
 * 8. NO DUPLICATE RESOURCE MODEL
 *
 *     Resource requirements are consumed from their canonical semantic
 *     subsystem.
 *
 * 9. NO DUPLICATE CAPABILITY MODEL
 *
 *     Capability semantics remain outside this composition root.
 *
 * 10. NO DUPLICATE EFFECT SYSTEM
 *
 *     Compilation effects are analyzed downstream.
 *
 * 11. NO AST IMPLEMENTATION
 *
 *     The grammar only supplies parse structure.
 *
 * 12. NO IR IMPLEMENTATION
 *
 *     The grammar creates no IR.
 *
 * 13. CANONICAL QUANTUM BOUNDARY
 *
 *     Quantum compilation continues through quantum::ir.
 *
 * 14. NO BACKEND IMPLEMENTATION
 *
 *     CPU/GPU/FPGA/ASIC/QPU/HPC/distributed/cloud realization is downstream.
 *
 * 15. NO HARDWARE DISCOVERY
 *
 *     The parser never inspects actual hardware.
 *
 * 16. NO RESOURCE ALLOCATION
 *
 *     Resource feasibility is resolved downstream.
 *
 * 17. NO ROUTING
 *
 *     Routing is downstream.
 *
 * 18. NO SCHEDULING
 *
 *     Scheduling is downstream.
 *
 * 19. NO QEC
 *
 *     Error correction is downstream.
 *
 * 20. NO ZQN
 *
 *     Quantum-network/target realization remains downstream.
 *
 * 21. NO HAL
 *
 *     Hardware abstraction and backend realization remain downstream.
 *
 * ============================================================================
 * POCO-REAF INVARIANTS
 * ============================================================================
 *
 * Changing the eventual realization from one available computational
 * environment to another MUST NOT require a different compilation grammar
 * merely because the target has:
 *
 *     more or fewer processors
 *     more or fewer accelerators
 *     more or fewer qubits
 *     more or less memory
 *     a different topology
 *     a different instruction set
 *     a different quantum architecture
 *     a different HDL implementation
 *     a different distributed topology
 *
 * Instead:
 *
 *     source intent
 *         |
 *         v
 *     semantic requirements
 *         |
 *         v
 *     capability negotiation
 *         |
 *         v
 *     target selection
 *         |
 *         v
 *     specialization
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     lowering
 *         |
 *         v
 *     routing / scheduling
 *         |
 *         v
 *     target realization
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * This grammar deliberately does not enumerate:
 *
 *     CPU models
 *     GPU models
 *     FPGA families
 *     ASIC families
 *     QPU vendors
 *     accelerator models
 *     operating systems
 *     cloud providers
 *     network providers
 *     machine sizes
 *     device identifiers
 *     quantum gates
 *     HDL primitive inventories
 *
 * Such names are supplied by:
 *
 *     symbolic identifiers
 *     qualified names
 *     capabilities
 *     resources
 *     dialects
 *     profiles
 *     target metadata
 *     compiler configuration
 *
 * This keeps the compilation language open to future computational systems.
 *
 * ============================================================================
 * SOURCE/SEMANTIC SEPARATION
 * ============================================================================
 *
 * A source-level declaration may express:
 *
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capabilities
 *     target intent
 *     target-selection policy
 *     optimization intent
 *     specialization intent
 *     lowering intent
 *     reproducibility
 *     deterministic-build requirements
 *     provenance
 *     caching
 *     deployment intent
 *
 * The parser does not determine whether those requirements are satisfiable.
 *
 * Example semantic intent:
 *
 *     requires capability("quantum.measurement");
 *     requires capability("tensor.compute");
 *     requires memory >= required_memory;
 *     requires qubits >= required_qubits;
 *     requires topology(required_topology);
 *
 * Such expressions remain semantic requirements.
 *
 * They are not universal language limits.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The compilation subsystem can therefore orchestrate compilation involving:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     AI/ML computation
 *     symbolic reasoning
 *     probabilistic computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     embedded computation
 *     accelerator computation
 *     parallel computation
 *     distributed computation
 *     networking
 *     data computation
 *     simulation
 *     future computational domains
 *
 * The compilation grammar does not need a new universal grammar branch merely
 * because a new computational backend is introduced, provided that backend
 * participates through the established semantic contracts.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * PARSER ERRORS
 *
 * The parser reports structural problems such as:
 *
 *     missing COMPILE
 *     missing compilation clause
 *     malformed profile
 *     malformed target
 *     malformed optimization declaration
 *     malformed specialization
 *     malformed artifact specification
 *     malformed lowering
 *     malformed cross-compilation declaration
 *     malformed reproducibility declaration
 *     malformed deterministic-build declaration
 *     malformed provenance declaration
 *     malformed caching declaration
 *     malformed deployment declaration
 *
 * SEMANTIC ERRORS
 *
 * Semantic analysis reports:
 *
 *     unknown names
 *     contradictory intent
 *     incompatible types
 *     incompatible requirements
 *     impossible transformation paths
 *     incompatible policies
 *     unsupported capabilities
 *     invalid specialization
 *     invalid lowering
 *     invalid target relationship
 *
 * RESOURCE/TARGET ERRORS
 *
 * Downstream resolution reports:
 *
 *     unavailable capability
 *     insufficient resources
 *     unsupported target realization
 *     infeasible topology
 *     unavailable deployment environment
 *
 * These must NOT be converted into parser errors.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source token sequence
 *     canonical lexer vocabulary
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     resource availability
 *     target selection
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *
 * Identical source and parser configuration must produce identical parse
 * structure.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every compilation construct must remain reconstructable from the domain-
 * neutral frontend AST.
 *
 * The AST must preserve, where applicable:
 *
 *     source span
 *     source ordering
 *     declaration kind
 *     explicit versus omitted values
 *     expressions
 *     qualified names
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     target intent
 *     selection intent
 *     optimization intent
 *     specialization intent
 *     lowering intent
 *     artifact relationships
 *     reproducibility requirements
 *     deterministic-build requirements
 *     provenance references
 *     caching intent
 *     deployment intent
 *
 * The AST MUST NOT contain:
 *
 *     physical device handles
 *     hardware addresses
 *     runtime scheduler state
 *     physical qubit mappings
 *     calibration data
 *     machine instructions
 *     backend objects
 *     mutable global compiler state
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * After parsing:
 *
 *     Compile AST
 *         |
 *         +--> name resolution
 *         |
 *         +--> type analysis
 *         |
 *         +--> effect analysis
 *         |
 *         +--> contract analysis
 *         |
 *         +--> policy analysis
 *         |
 *         +--> capability analysis
 *         |
 *         +--> resource analysis
 *         |
 *         +--> provenance analysis
 *         |
 *         +--> portability analysis
 *         |
 *         v
 *     compilation semantic model
 *
 * Compilation intent may then participate in:
 *
 *     canonical semantic representation
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         +--> distributed representation
 *         +--> other domain representations
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * THIS FILE DEFINES NO IR.
 *
 * In particular it must not introduce:
 *
 *     CompileIR
 *     CompilationIR
 *     TargetIR
 *     OptimizationIR
 *     LoweringIR
 *     QuantumCompileIR
 *     HardwareCompileIR
 *
 * Quantum compilation MUST preserve:
 *
 *     semantic model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     quantum optimization/decomposition/routing/scheduling
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * Repository implementation requirements remain:
 *
 *     Rust 2021
 *     Rust 1.97 or later
 *     safe Rust only
 *
 * The generated compiler/frontend implementation MUST NOT require `unsafe`.
 *
 * Scalable implementation should prefer:
 *
 *     Vec<T>
 *     String
 *     growable collections
 *     iterators
 *     streaming/incremental processing where appropriate
 *
 * and MUST NOT introduce fixed-size arrays merely to establish artificial
 * language ceilings.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     +
 *     *
 *     ?
 *
 * and symbolic references rather than finite machine enumerations.
 *
 * Therefore there is no grammar-level maximum for:
 *
 *     compilation clauses
 *     profiles
 *     target alternatives
 *     optimization directives
 *     specialization declarations
 *     lowering stages
 *     artifacts
 *     provenance entries
 *     cache properties
 *     deployment properties
 *     deterministic-build properties
 *
 * Actual limits are implementation/environment limits, not language limits.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Compilation decisions should be traceable through the repository's
 * provenance subsystem.
 *
 * At minimum, source intent should be capable of being related to:
 *
 *     source
 *     transformation
 *     specialization
 *     optimization
 *     lowering
 *     generated artifact
 *     verification
 *     target realization
 *
 * The parser only preserves the declared relationships.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The following distinctions are stable:
 *
 *     target intent
 *         != target selection
 *
 *     target selection
 *         != target realization
 *
 *     optimization
 *         != specialization
 *
 *     specialization
 *         != lowering
 *
 *     lowering
 *         != code generation
 *
 *     code generation
 *         != deployment
 *
 *     reproducibility
 *         != deterministic execution
 *
 *     compilation
 *         != execution
 *
 * New computational domains should normally integrate through symbolic
 * qualified names, capabilities, profiles, dialects, and semantic contracts
 * instead of requiring a new universal machine-specific keyword.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Formatter:
 *
 *     preserves compilation-clause order and source representation.
 *
 * IDE/LSP:
 *
 *     resolves names and semantics through compiler services rather than a
 *     hard-coded target catalogue.
 *
 * Documentation:
 *
 *     may render compilation intent without instantiating hardware backends.
 *
 * Static analysis:
 *
 *     may inspect requirements, policies, provenance, reproducibility,
 *     optimization and lowering intent.
 *
 * Incremental compilation:
 *
 *     may use source spans and provenance to determine affected compilation
 *     regions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * REQUIRED POSITIVE TESTS
 *
 *     minimal compile declaration
 *     profile composition
 *     compile-time control
 *     conditional compilation
 *     feature selection
 *     target intent
 *     target selection
 *     optimization
 *     specialization
 *     artifact intent
 *     code generation
 *     lowering
 *     cross compilation
 *     reproducibility
 *     deterministic builds
 *     provenance
 *     caching
 *     deployment
 *
 * REQUIRED NEGATIVE TESTS
 *
 *     empty compile declaration
 *     malformed compile declaration
 *     missing clause
 *     malformed profile
 *     malformed target
 *     malformed selection
 *     malformed optimization
 *     malformed specialization
 *     malformed artifact
 *     malformed code generation
 *     malformed lowering
 *     malformed cross-compilation
 *     malformed reproducibility
 *     malformed deterministic-build declaration
 *     malformed provenance
 *     malformed caching
 *     malformed deployment
 *
 * REQUIRED SEMANTIC TESTS
 *
 *     conflicting target intent
 *     conflicting requirements
 *     unsatisfied capability
 *     insufficient resources
 *     incompatible optimization
 *     impossible specialization
 *     invalid lowering path
 *     incompatible policy
 *     invalid provenance relationship
 *
 * REQUIRED CROSS-DOMAIN TESTS
 *
 *     classical compilation
 *     quantum compilation
 *     hybrid compilation
 *     HDL compilation
 *     hardware/software co-design
 *     accelerator compilation
 *     distributed compilation
 *     tensor compilation
 *     AI/ML compilation
 *     future symbolic target compilation
 *
 * REQUIRED POCO-REAF TEST
 *
 * The same source-level compilation intent must remain syntactically valid
 * when the eventual realization changes among available:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     quantum processors
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed environments
 *     cloud environments
 *     future computational substrates
 *
 * The test must verify that target scaling is handled downstream rather than
 * by changing the compilation grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain no:
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
 * or disguised equivalents.
 *
 * It also MUST NOT enumerate:
 *
 *     processor models
 *     accelerator models
 *     QPU models
 *     vendor devices
 *     machine sizes
 *     fixed network sizes
 *     fixed quantum topologies
 *     fixed deployment topologies
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         v
 *     grammar/antlr/ZamaniParser.g4
 *         |
 *         v
 *     Compile
 *
 * Downstream:
 *
 *     domain-neutral AST
 *         |
 *         v
 *     structural validation
 *         |
 *         v
 *     semantic analysis
 *         |
 *         +--> types
 *         +--> effects
 *         +--> resources
 *         +--> capabilities
 *         +--> contracts
 *         +--> policies
 *         +--> provenance
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         +--> distributed representation
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     specialization
 *         |
 *         v
 *     lowering
 *         |
 *         v
 *     routing / scheduling
 *         |
 *         v
 *     resilience / QEC / ZQN
 *         |
 *         v
 *     HAL
 *         |
 *         v
 *     target realization
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It is the only canonical `Compile` composition root.
 *
 * [ ] `parser grammar Compile;` is preserved.
 *
 * [ ] `tokenVocab = ZamaniLexer` is preserved.
 *
 * [ ] Every imported grammar is an actual existing parser grammar.
 *
 * [ ] Every referenced public rule exists.
 *
 * [ ] No nonexistent `compileProfileDeclaration` reference remains.
 *
 * [ ] No nonexistent `artifactDeclaration` reference remains.
 *
 * [ ] Provenance is composed.
 *
 * [ ] Deterministic-build intent is composed.
 *
 * [ ] Conditional compilation is composed.
 *
 * [ ] Feature selection is composed through one canonical feature authority.
 *
 * [ ] No lexer rules are defined here.
 *
 * [ ] No duplicate expression grammar exists here.
 *
 * [ ] No duplicate type grammar exists here.
 *
 * [ ] No duplicate identifier grammar exists here.
 *
 * [ ] No duplicate resource/capability grammar exists here.
 *
 * [ ] No AST is defined here.
 *
 * [ ] No IR is defined here.
 *
 * [ ] `quantum::ir` remains the canonical quantum IR boundary.
 *
 * [ ] No hardware discovery occurs during parsing.
 *
 * [ ] No resource allocation occurs during parsing.
 *
 * [ ] No target probing occurs during parsing.
 *
 * [ ] No routing occurs during parsing.
 *
 * [ ] No scheduling occurs during parsing.
 *
 * [ ] No QEC occurs during parsing.
 *
 * [ ] No HAL implementation occurs here.
 *
 * [ ] No finite machine/resource ceiling is introduced.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] POCO-REAF tests exist.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * `Compile` is the orchestration boundary, not the implementation of the
 * compiler.
 *
 * It composes source-level compilation intent into one stable parser boundary
 * while leaving semantic resolution and physical realization completely
 * open-ended.
 *
 * The resulting architecture is:
 *
 *     ONE LANGUAGE
 *          |
 *          v
 *     ONE LEXER
 *          |
 *          v
 *     ONE PARSER COMPOSITION
 *          |
 *          v
 *     ONE DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     ONE SEMANTIC MODEL
 *          |
 *          +-------------------+--------------------+
 *          |                   |                    |
 *          v                   v                    v
 *      classical          quantum::ir         HDL/hardware
 *          |                   |                    |
 *          +-------------------+--------------------+
 *                              |
 *                              v
 *                         compilation
 *                         optimization
 *                         lowering
 *                         realization
 *
 * Source meaning remains independent of the size, architecture, vendor,
 * topology, or physical realization available at execution time.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */