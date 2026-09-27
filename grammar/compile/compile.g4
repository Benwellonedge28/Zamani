/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/compile.g4
 *
 * Grammar:
 *     Compile
 *
 * Status:
 *     PRODUCTION COMPOSITION ROOT
 *
 * Purpose:
 *     Canonical compilation-language composition boundary.
 *
 * This file owns the composition of source-level compilation constructs.
 * It does not duplicate the internal syntax owned by the dedicated grammar
 * modules under grammar/compile/.
 *
 * ============================================================================
 * LANGUAGE / TOOLCHAIN CONTRACT
 * ============================================================================
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR
 *
 * Compiler implementation:
 *     Rust 2021
 *
 * Supported Rust baseline:
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Safety:
 *     No unsafe Rust is required.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Compilation syntax describes portable source intent.
 *
 * It MUST NOT impose artificial universal limits on:
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
 *     vector width
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     channels
 *     timelines
 *     targets
 *     artifacts
 *     compilation stages
 *     requirements
 *     constraints
 *     capabilities
 *     optimization objectives
 *
 * Any actual limit is an implementation/resource/environment property and
 * belongs downstream of source parsing.
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
 *     Compile
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> names / types / effects
 *          +--> resources / capabilities
 *          +--> portability
 *          +--> target resolution
 *          +--> specialization
 *          +--> optimization planning
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN where applicable
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime / deployment
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the canonical compilation composition boundary;
 *     - the `Compile` parser grammar;
 *     - the public compilation declaration entry point;
 *     - composition of dedicated compilation grammar modules;
 *     - compilation-clause dispatch;
 *     - stable delegation names for compilation subsystems;
 *     - the boundary between compilation syntax and downstream semantics.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - ordinary declarations;
 *     - ordinary statements;
 *     - profile internals;
 *     - target internals;
 *     - target-selection internals;
 *     - optimization internals;
 *     - specialization internals;
 *     - artifact internals;
 *     - code-generation internals;
 *     - lowering internals;
 *     - cross-compilation internals;
 *     - reproducibility implementation;
 *     - caching implementation;
 *     - deployment execution;
 *     - hardware discovery;
 *     - resource discovery;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - canonical IR definitions.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST remain the only compilation composition root.
 *
 * It MUST NOT be accompanied by another grammar that claims to be the
 * authoritative general compilation grammar.
 *
 * In particular:
 *
 *     grammar/compile/compilation.g4
 *
 * must not become a second parser root.
 *
 * If compilation.g4 is retained as the canonical composition root, any older
 * overlapping composition grammar must be reduced to documentation,
 * compatibility support, or removed after dependency verification.
 *
 * The canonical public root remains:
 *
 *     grammar/Zamani.g4
 *
 * and that root is responsible for integrating Compile into the complete
 * Zamani language.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Canonical direction:
 *
 *     Zamani.g4
 *          |
 *          v
 *       Compile
 *          |
 *          +--> CompileProfiles
 *          +--> CompileTime
 *          +--> CompileTarget
 *          +--> CompileTargetSelection
 *          +--> CompileOptimization
 *          +--> CompileSpecialization
 *          +--> CompileArtifacts
 *          +--> CompileCodeGeneration
 *          +--> CompileLowering
 *          +--> CompileCrossCompilation
 *          +--> CompileReproducibility
 *          +--> CompileCaching
 *          +--> CompileDeployment
 *
 * Dedicated grammars may themselves import the canonical foundational
 * grammars, but no dedicated compilation grammar may import this composition
 * root.
 *
 * This prevents cycles such as:
 *
 *     Compile -> CompileOptimization -> Compile
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar, not a combined grammar.
 *
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * No lexer rules are defined here.
 *
 * No embedded host-language actions are permitted.
 *
 * No semantic predicates are required.
 *
 * No Rust code is embedded.
 *
 * ============================================================================
 */

parser grammar Compile;


/*
 * ============================================================================
 * CANONICAL TOKEN VOCABULARY
 * ============================================================================
 *
 * All lexical tokens come from the canonical Zamani lexer.
 *
 * The lexer remains the sole authority for:
 *
 *     keywords
 *     identifiers
 *     literals
 *     operators
 *     delimiters
 *     Unicode
 *     comments
 *
 * This grammar does not redefine them.
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTED COMPILATION COMPONENTS
 * ============================================================================
 *
 * Each imported parser grammar owns one independently completable concern.
 *
 * The composition root delegates to those grammars instead of copying their
 * productions.
 *
 * Foundational syntax is imported by the dedicated grammars where required.
 *
 * The compilation root therefore does not create another:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     blockExpression
 *
 * authority.
 */

import
    CompileProfiles,
    CompileTime,
    CompileTarget,
    CompileTargetSelection,
    CompileOptimization,
    CompileSpecialization,
    CompileArtifacts,
    CompileCodeGeneration,
    CompileLowering,
    CompileCrossCompilation,
    CompileReproducibility,
    CompileCaching,
    CompileDeployment;


/*
 * ============================================================================
 * 1. PUBLIC COMPILATION ENTRY POINT
 * ============================================================================
 *
 * This is the primary entry point exposed by this grammar.
 *
 * The canonical Zamani root should invoke:
 *
 *     compileDeclaration
 *
 * when a source-level compilation declaration is encountered.
 *
 * The grammar deliberately permits a sequence of compilation clauses.
 *
 * No finite clause count is imposed.
 */

compileDeclaration
    : COMPILE compileSpecification SEMI?
    ;


/*
 * ============================================================================
 * 2. COMPILATION SPECIFICATION
 * ============================================================================
 *
 * A compilation specification is a non-empty sequence of compilation clauses.
 *
 * This prevents a meaningless:
 *
 *     compile;
 *
 * from being silently accepted as a complete compilation specification.
 *
 * If a future language version gives an empty compile declaration a defined
 * meaning, that change must be explicitly versioned.
 */

compileSpecification
    : compileClause+
    ;


/*
 * ============================================================================
 * 3. COMPILATION CLAUSE DISPATCH
 * ============================================================================
 *
 * This is the central responsibility of this file.
 *
 * Each branch delegates to the grammar that owns its syntax.
 *
 * No branch below reproduces the internal implementation of the delegated
 * grammar.
 *
 * ============================================================================
 */

compileClause
    : compileProfileReference
    | compileTimeControlReference
    | compileTargetReference
    | compileTargetSelectionReference
    | compileOptimizationReference
    | compileSpecializationReference
    | compileArtifactReference
    | compileCodeGenerationReference
    | compileLoweringReference
    | compileCrossCompilationReference
    | compileReproducibilityReference
    | compileCachingReference
    | compileDeploymentReference
    ;


/*
 * ============================================================================
 * 4. PROFILE COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/profiles.g4
 *
 * Public grammar:
 *
 *     CompileProfiles
 *
 * This wrapper gives the Compile grammar a stable composition boundary while
 * preserving CompileProfiles as the sole owner of profile syntax.
 */

compileProfileReference
    : compileProfileDeclaration
    ;


/*
 * ============================================================================
 * 5. COMPILE-TIME CONTROL COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/compile-time.g4
 *
 * This grammar owns statement/control-level compile-time constructs.
 *
 * Expression-level compile-time constructs remain owned by the expressions
 * subsystem.
 */

compileTimeControlReference
    : compileTimeControl
    ;


/*
 * ============================================================================
 * 6. TARGET COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/target.g4
 *
 * Target declarations describe target intent.
 *
 * They do not describe:
 *
 *     physical devices
 *     hardware inventories
 *     physical addresses
 *     provider credentials
 *     live devices
 *     machine state
 */

compileTargetReference
    : targetDeclaration
    ;


/*
 * ============================================================================
 * 7. TARGET-SELECTION COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/target-selection.g4
 *
 * Target selection is different from target declaration.
 *
 * Target selection expresses policy for choosing an acceptable realization.
 *
 * It does not perform:
 *
 *     hardware discovery
 *     resource allocation
 *     physical placement
 *     routing
 *     scheduling
 */

compileTargetSelectionReference
    : targetSelectionDeclaration
    ;


/*
 * ============================================================================
 * 8. OPTIMIZATION COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/optimization.g4
 *
 * Optimization syntax expresses intent.
 *
 * It does not implement optimization algorithms.
 *
 * The optimizer remains a downstream compiler subsystem.
 */

compileOptimizationReference
    : optimizationDeclaration
    ;


/*
 * ============================================================================
 * 9. SPECIALIZATION COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/specialization.g4
 *
 * Explicit metaprogramming specialization remains separate from compilation
 * specialization policy.
 *
 * The compilation grammar must not redefine generic declarations or
 * metaprogramming syntax.
 */

compileSpecializationReference
    : compileSpecializationDeclaration
    ;


/*
 * ============================================================================
 * 10. ARTIFACT COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/artifacts.g4
 *
 * Artifact syntax describes desired compilation products or representations.
 *
 * It does not implement packaging, linking, serialization, or deployment.
 */

compileArtifactReference
    : artifactDeclaration
    ;


/*
 * ============================================================================
 * 11. CODE-GENERATION COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/code-generation.g4
 *
 * Code-generation syntax describes generation intent.
 *
 * The actual generator remains downstream.
 */

compileCodeGenerationReference
    : codeGenerationDeclaration
    ;


/*
 * ============================================================================
 * 12. LOWERING COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/lowering.g4
 *
 * Lowering syntax describes permitted/requested transformation boundaries.
 *
 * It must never introduce another IR hierarchy.
 */

compileLoweringReference
    : loweringDeclaration
    ;


/*
 * ============================================================================
 * 13. CROSS-COMPILATION COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/cross-compilation.g4
 *
 * Cross-compilation expresses portability and realization intent.
 *
 * It must not encode a finite target inventory.
 */

compileCrossCompilationReference
    : crossCompilationDeclaration
    ;


/*
 * ============================================================================
 * 14. REPRODUCIBILITY COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/reproducibility.g4
 *
 * Reproducibility is a compilation property, not an execution implementation.
 */

compileReproducibilityReference
    : reproducibilityDeclaration
    ;


/*
 * ============================================================================
 * 15. CACHING COMPOSITION
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/compile/caching.g4
 *
 * Cache syntax expresses compilation artifact reuse intent.
 *
 * It does not grant access to a particular filesystem, network cache, build
 * server, or provider.
 */

compileCachingReference
    : cachingDeclaration
    ;


/*
 * ============================================================================
 * 16. DEPLOYMENT COMPOSITION
 * ============================================================================
 *
 * Deployment grammar ownership remains separate.
 *
 * The compilation layer may reference deployment intent, but it does not
 * become the deployment implementation.
 */

compileDeploymentReference
    : compileDeploymentDeclaration
    ;


/*
 * ============================================================================
 * 17. COMPOSITION INVARIANTS
 * ============================================================================
 *
 * The following invariants are part of the grammar contract.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT A — ONE LEXER
 * ---------------------------------------------------------------------------
 *
 * Every token is supplied by ZamaniLexer.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT B — ONE COMPILE ROOT
 * ---------------------------------------------------------------------------
 *
 * Compile is the only general compilation composition grammar.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT C — ONE OWNER PER FEATURE
 * ---------------------------------------------------------------------------
 *
 * The composition root delegates rather than duplicates.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT D — NO SECOND EXPRESSION GRAMMAR
 * ---------------------------------------------------------------------------
 *
 * Compilation components consume the canonical expression contract.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT E — NO SECOND TYPE GRAMMAR
 * ---------------------------------------------------------------------------
 *
 * Compilation components consume the canonical type contract.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT F — NO SECOND AST
 * ---------------------------------------------------------------------------
 *
 * Grammar modules describe syntax.
 *
 * AST ownership remains with the domain-neutral frontend AST architecture.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT G — NO SECOND IR
 * ---------------------------------------------------------------------------
 *
 * This grammar creates no IR.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT H — CANONICAL QUANTUM IR
 * ---------------------------------------------------------------------------
 *
 * Quantum compilation continues through:
 *
 *     quantum::ir
 *
 * There is no compilation-specific quantum IR.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT I — TARGET INDEPENDENCE
 * ---------------------------------------------------------------------------
 *
 * Compile syntax does not require a particular machine realization.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT J — RESOURCE INDEPENDENCE
 * ---------------------------------------------------------------------------
 *
 * Compile syntax does not impose physical resource ceilings.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT K — SEMANTIC SEPARATION
 * ---------------------------------------------------------------------------
 *
 * Requirement, constraint, preference, hint, capability, resource and target
 * intent remain distinguishable semantic categories.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT L — DETERMINISTIC PARSING
 * ---------------------------------------------------------------------------
 *
 * Given identical source, lexer configuration and grammar version, parsing is
 * deterministic.
 *
 * ---------------------------------------------------------------------------
 * INVARIANT M — SAFE IMPLEMENTATION
 * ---------------------------------------------------------------------------
 *
 * No unsafe Rust is required by this grammar or its intended compiler
 * integration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. SEMANTIC CATEGORY CONTRACT
 * ============================================================================
 *
 * The composition grammar intentionally does not provide a generic:
 *
 *     compilerOption
 *
 * escape hatch that could silently absorb every semantic category.
 *
 * The downstream semantic model must preserve distinctions between:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *     target
 *     artifact
 *     optimization objective
 *     compilation stage
 *     deployment intent
 *
 * A backend must not reinterpret a preference as a requirement merely because
 * its own implementation prefers that interpretation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. POCO-REAF RESOURCE CONTRACT
 * ============================================================================
 *
 * Compilation source may express requirements such as:
 *
 *     requires qubits >= n
 *     requires memory >= required_memory
 *     requires capability("quantum.measurement")
 *     requires capability("tensor.compute")
 *     requires capability("gpu.compute")
 *
 * The composition grammar does not decide whether these are satisfiable.
 *
 * Satisfiability belongs to:
 *
 *     semantic analysis
 *     resource analysis
 *     capability resolution
 *     target resolution
 *     compiler policy
 *     runtime/deployment infrastructure
 *
 * The same source must therefore remain syntactically meaningful across:
 *
 *     atom-scale or very small systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     emulators
 *     clusters
 *     HPC
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * subject to the semantic requirements of the program and resources actually
 * available to the implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This composition grammar MUST NOT contain or introduce universal grammar
 * rules equivalent to:
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
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_ACCELERATOR_COUNT
 *
 * It also must not introduce disguised finite alternatives such as:
 *
 *     target0
 *     target1
 *     target2
 *
 * as a universal target inventory.
 *
 * Hardware/provider/device names belong to externally defined semantic
 * capability vocabularies or dialects.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * Compile is deliberately domain-neutral.
 *
 * It may compose compilation intent for:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     tensor/data
 *     distributed
 *     networking
 *     security
 *     accelerator
 *     embedded
 *     simulator
 *     future computing domains
 *
 * The domain-specific grammar remains owned by the relevant subsystem.
 *
 * For example:
 *
 *     quantum source
 *          ->
 *     quantum grammar
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic quantum representation
 *          ->
 *     quantum::ir
 *
 * Compile does not introduce:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     QuantumCircuitIR
 *
 * as compilation grammar constructs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. HARDWARE / HDL SEPARATION
 * ============================================================================
 *
 * Compile may compose hardware-related compilation intent.
 *
 * It must not turn compilation syntax into a hardware inventory.
 *
 * Therefore source-level compilation may express semantic requirements while
 * downstream systems resolve:
 *
 *     CPU realization
 *     GPU realization
 *     FPGA realization
 *     ASIC realization
 *     accelerator realization
 *     QPU realization
 *     memory realization
 *     interconnect realization
 *     topology
 *     placement
 *     timing
 *     routing
 *     scheduling
 *
 * HDL remains responsible for hardware-description syntax.
 *
 * Hardware remains responsible for target-independent hardware intent and
 * capabilities.
 *
 * Compile remains responsible only for composing their compilation intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. AST CONTRACT
 * ============================================================================
 *
 * This grammar does not prescribe concrete Rust AST types.
 *
 * Nevertheless, the parser/semantic architecture MUST maintain a traceable
 * relationship:
 *
 *     grammar rule
 *          ->
 *     source span
 *          ->
 *     domain-neutral AST node
 *          ->
 *     semantic compilation model
 *
 * The AST should preserve the category represented by each delegated
 * construct.
 *
 * In particular, semantic analysis must be able to distinguish:
 *
 *     profile
 *     target
 *     target-selection policy
 *     optimization intent
 *     specialization intent
 *     artifact intent
 *     lowering intent
 *     code-generation intent
 *     reproducibility intent
 *     caching intent
 *     deployment intent
 *
 * No parser action in this grammar constructs or mutates the AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Compilation intent is transformed by semantic analysis into the repository's
 * canonical semantic representation.
 *
 * Domain-specific lowering then proceeds through the established boundaries.
 *
 * Quantum:
 *
 *     semantic quantum representation
 *          ->
 *     quantum::ir
 *
 * Classical:
 *
 *     semantic classical representation
 *          ->
 *     canonical classical IR
 *
 * HDL/hardware:
 *
 *     semantic hardware representation
 *          ->
 *     appropriate canonical hardware/HDL representation
 *
 * Distributed:
 *
 *     semantic distributed representation
 *          ->
 *     appropriate downstream IR/planning representation
 *
 * There must not be:
 *
 *     CompileIR
 *     CompileQuantumIR
 *     CompileHardwareIR
 *
 * merely because this grammar exists.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. TARGET RESOLUTION CONTRACT
 * ============================================================================
 *
 * The grammar preserves target intent.
 *
 * Semantic analysis determines:
 *
 *     whether the target expression is valid;
 *     whether the target is available;
 *     whether capabilities are satisfied;
 *     whether resources are sufficient;
 *     whether portability requirements hold;
 *     whether constraints conflict;
 *     whether an acceptable realization exists.
 *
 * Actual target realization occurs downstream.
 *
 * Compile parsing must never perform target discovery.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Resource and capability resolution are semantic operations.
 *
 * Compilation syntax may refer to them through the canonical subsystem
 * contracts.
 *
 * The grammar must never convert a capability into a hard-coded hardware
 * identity.
 *
 * For example:
 *
 *     capability("quantum.measurement")
 *
 * is portable semantic intent.
 *
 * It does not mean:
 *
 *     QPU #0
 *
 * Likewise:
 *
 *     capability("gpu.compute")
 *
 * does not mean:
 *
 *     GPU #0
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     semantic predicates
 *     random decisions
 *     environment queries
 *     filesystem operations
 *     network operations
 *     process execution
 *     device discovery
 *     runtime calls
 *     mutable global state
 *
 * The parser must therefore remain deterministic.
 *
 * If a compilation policy has nondeterministic semantic behavior, that
 * behavior must be represented and controlled downstream through explicit
 * compiler policy, reproducibility, or execution semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. ERROR-DOMAIN CONTRACT
 * ============================================================================
 *
 * Syntax failures belong to parsing.
 *
 * Examples:
 *
 *     malformed compile declaration
 *         -> parser diagnostic
 *
 *     malformed target-selection syntax
 *         -> parser diagnostic
 *
 *     malformed optimization syntax
 *         -> parser diagnostic
 *
 * Semantic failures belong downstream.
 *
 * Examples:
 *
 *     unknown capability
 *         -> capability diagnostic
 *
 *     insufficient resources
 *         -> resource diagnostic
 *
 *     incompatible target
 *         -> target diagnostic
 *
 *     unsatisfied compilation requirement
 *         -> semantic/resource diagnostic
 *
 *     conflicting optimization requirements
 *         -> optimization semantic diagnostic
 *
 *     impossible deployment
 *         -> deployment semantic diagnostic
 *
 * The composition grammar must not attempt to solve those questions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. VERSIONING CONTRACT
 * ============================================================================
 *
 * Changes to this composition root are language-version-sensitive.
 *
 * Adding a new delegated grammar component should normally be backward
 * compatible when the new syntax is unambiguous and optional.
 *
 * Removing or changing the meaning of an existing compilation construct
 * requires the repository compatibility process.
 *
 * Canonical compatibility authority:
 *
 *     grammar/compatibility/
 *
 * and:
 *
 *     grammar/spec/compatibility.md
 *
 * Historical/aspirational syntax in:
 *
 *     grammar/Zamani-Grammar.md
 *
 * does not automatically become legal syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. GRAMMAR.MD CONTRACT
 * ============================================================================
 *
 * grammar/grammar.md remains the implementation-conformance reference.
 *
 * It should identify which delegated compilation features are:
 *
 *     SPECIFIED
 *     IMPLEMENTED
 *     PARTIALLY IMPLEMENTED
 *     PLANNED
 *     DEPRECATED
 *
 * It must not become a competing compilation grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. ZAMANI-GRAMMAR.MD CONTRACT
 * ============================================================================
 *
 * grammar/Zamani-Grammar.md remains a historical/extended design reference.
 *
 * It may describe future compilation capabilities.
 *
 * It cannot independently authorize source syntax.
 *
 * Promotion remains:
 *
 *     design
 *       ->
 *     proposal
 *       ->
 *     semantic contract
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
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. ROOT GRAMMAR INTEGRATION
 * ============================================================================
 *
 * grammar/Zamani.g4 must import or otherwise compose this Compile grammar
 * exactly once.
 *
 * The root language grammar should expose the compilation declaration through
 * the universal declaration/statement dispatch appropriate to the existing
 * Zamani architecture.
 *
 * Conceptually:
 *
 *     Zamani
 *        |
 *        +--> compileDeclaration
 *
 * No other compilation root should be inserted alongside this one.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. LEXER INTEGRATION
 * ============================================================================
 *
 * The canonical lexer must define the lexical vocabulary consumed by the
 * imported compilation grammars.
 *
 * This file intentionally does not define tokens.
 *
 * Token duplication must be rejected during validation.
 *
 * In particular, the lexer must remain the sole authority for concepts such
 * as:
 *
 *     COMPILE
 *     PROFILE
 *     REQUIRES
 *     CONSTRAIN
 *     PREFER
 *     HINT
 *     FEATURE
 *     ARTIFACT
 *     STAGE
 *     OPTION
 *
 * where those tokens are part of the current canonical vocabulary.
 *
 * If an existing token is renamed or consolidated, the lexer compatibility
 * process must update the affected grammar modules deliberately.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * ANTLR generation for the Zamani compiler targets:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * This grammar contains no embedded Rust.
 *
 * It therefore introduces no direct `unsafe` implementation requirement.
 *
 * Downstream compiler code must continue to enforce the repository-wide rule:
 *
 *     #![forbid(unsafe_code)]
 *
 * where that crate-level policy is applicable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This composition root contains no finite resource enumeration.
 *
 * Repetition is delegated to the relevant grammar components.
 *
 * Therefore there is no grammar-level ceiling on:
 *
 *     profiles
 *     targets
 *     target candidates
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capabilities
 *     resources
 *     optimization objectives
 *     stages
 *     artifacts
 *     specialization arguments
 *     generated artifacts
 *     deployment descriptions
 *
 * Practical limits may exist in:
 *
 *     parser memory
 *     compiler memory
 *     operating-system resources
 *     runtime resources
 *     target resources
 *     provider limits
 *
 * Those are not Zamani language limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. COMPOSITION TEST CONTRACT
 * ============================================================================
 *
 * This file is complete only when composition tests verify every delegated
 * public entry point.
 *
 * Required positive coverage:
 *
 *     compile profile ...
 *     compile-time control
 *     target declaration
 *     target selection
 *     optimization
 *     specialization
 *     artifact request
 *     code generation
 *     lowering
 *     cross compilation
 *     reproducibility
 *     caching
 *     deployment
 *
 * Required cross-domain coverage:
 *
 *     classical compilation
 *     quantum compilation
 *     hybrid compilation
 *     HDL compilation
 *     hardware compilation
 *     AI compilation
 *     tensor/data compilation
 *     distributed compilation
 *     accelerator compilation
 *     embedded compilation
 *     simulator compilation
 *     future/dialect extension
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The composition root must reject malformed composition.
 *
 * Examples:
 *
 *     compile;
 *     compile { ... }
 *     malformed delegated syntax
 *     unterminated profile
 *     unterminated target selection
 *     malformed optimization declaration
 *     malformed specialization declaration
 *     malformed artifact declaration
 *     malformed lowering declaration
 *
 * Exact diagnostics are owned by the parser/diagnostic subsystem.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. HARD-CODING TEST CONTRACT
 * ============================================================================
 *
 * Automated validation must inspect this file and its imported compilation
 * grammars for prohibited universal hardware ceilings and finite inventories.
 *
 * The validation layer must detect both direct and disguised forms.
 *
 * Examples of prohibited universal language assumptions:
 *
 *     exactly 32 cores
 *     exactly 64 GPUs
 *     maximum 128 qubits
 *     only 24 GB memory
 *     register width fixed to 32
 *     maximum tensor rank fixed by grammar
 *
 * Program-level constants remain legal when they are actual program data.
 *
 * The prohibition applies to artificial language/compiler grammar limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same source-level compilation intent must remain syntactically valid
 * when semantic realization changes.
 *
 * Test realization classes include:
 *
 *     embedded
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     emulator
 *     cluster
 *     HPC
 *     distributed
 *     cloud
 *     future architecture
 *
 * The parser does not decide whether a realization is feasible.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] There is exactly one `parser grammar Compile`.
 *
 * [x] The canonical lexer is ZamaniLexer.
 *
 * [x] No lexer rules are duplicated.
 *
 * [x] No expression grammar is duplicated.
 *
 * [x] No type grammar is duplicated.
 *
 * [x] No profile internals are duplicated.
 *
 * [x] No target internals are duplicated.
 *
 * [x] No optimization internals are duplicated.
 *
 * [x] No specialization internals are duplicated.
 *
 * [x] No artifact internals are duplicated.
 *
 * [x] No code-generation internals are duplicated.
 *
 * [x] No lowering internals are duplicated.
 *
 * [x] No deployment internals are duplicated.
 *
 * [x] Compilation composition has one authoritative dispatch boundary.
 *
 * [x] The grammar contains no hardware capacity constants.
 *
 * [x] The grammar contains no provider enumeration.
 *
 * [x] The grammar contains no physical device enumeration.
 *
 * [x] The grammar contains no physical qubit enumeration.
 *
 * [x] The grammar contains no fixed topology.
 *
 * [x] The grammar contains no artificial memory ceiling.
 *
 * [x] The grammar contains no artificial processor ceiling.
 *
 * [x] The grammar contains no artificial tensor ceiling.
 *
 * [x] The grammar contains no artificial network ceiling.
 *
 * [x] The grammar contains no semantic actions.
 *
 * [x] The grammar contains no filesystem access.
 *
 * [x] The grammar contains no network access.
 *
 * [x] The grammar contains no device discovery.
 *
 * [x] The grammar contains no runtime execution.
 *
 * [x] The grammar creates no IR.
 *
 * [x] Quantum lowering remains connected to canonical `quantum::ir`.
 *
 * [x] Compilation intent remains target-independent.
 *
 * [x] Rust integration requires Rust 1.97 / 1.97.1.
 *
 * [x] Rust integration requires safe Rust.
 *
 * [x] Positive composition tests exist.
 *
 * [x] Negative composition tests exist.
 *
 * [x] Cross-domain tests exist.
 *
 * [x] Scalability tests exist.
 *
 * [x] Portability tests exist.
 *
 * [x] Determinism tests exist.
 *
 * [x] Compatibility tests exist.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How are the independently owned compilation-language components
 *      composed into one canonical Compile grammar?"
 *
 * It does NOT answer:
 *
 *     "Which machine will execute the program?"
 *
 *     "Which CPU/GPU/FPGA/QPU will be selected?"
 *
 *     "Which physical qubit will be used?"
 *
 *     "How will resources be allocated?"
 *
 *     "How will routing occur?"
 *
 *     "How will scheduling occur?"
 *
 *     "How will optimization be implemented?"
 *
 *     "How will QEC occur?"
 *
 *     "How will ZQN operate?"
 *
 *     "How will HAL communicate with hardware?"
 *
 * Those remain downstream responsibilities.
 *
 * The resulting architecture is:
 *
 *     Zamani Source
 *          |
 *          v
 *     Zamani.g4
 *          |
 *          v
 *     Compile
 *          |
 *          v
 *     Domain-neutral AST
 *          |
 *          v
 *     Semantic analysis
 *          |
 *          v
 *     Canonical semantic representation
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> distributed
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * with no artificial language-level hardware ceiling.
 *
 * ============================================================================
 */