/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/core.g4
 *
 * GRAMMAR
 * -------
 * Core
 *
 * STATUS
 * ------
 * CANONICAL CORE-DIRECTORY ORCHESTRATOR
 *
 * PURPOSE
 * -------
 *
 * This file is the SINGLE COMPOSITION AUTHORITY for grammar/core/.
 *
 * It does not implement a second monolithic grammar.
 *
 * It imports and composes every foundational grammar component owned by
 * grammar/core/, making the complete core vocabulary available to the
 * canonical parser composition root:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The dependency direction is:
 *
 *     grammar/Zamani.g4
 *             |
 *             v
 *     ZamaniParser
 *             |
 *             v
 *           Core
 *             |
 *     +-------+---------------------------------------------+
 *     |                                                     |
 *     v                                                     v
 * foundational core grammars                         domain grammars
 *
 * The core layer remains domain-neutral.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar.
 *
 * Rust integration:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no environment inspection;
 *     - no randomness;
 *     - no unsafe implementation requirement.
 *
 * ============================================================================
 * CORE OWNERSHIP
 * ============================================================================
 *
 * grammar/core/ owns the universal syntactic foundations shared by the
 * complete Zamani language.
 *
 * The core composition includes:
 *
 *     annotations
 *     attributes
 *     blocks
 *     capabilities
 *     compilation units
 *     constraints
 *     hints
 *     identifiers
 *     metadata
 *     modifiers
 *     names
 *     paths
 *     policies
 *     pragmas
 *     program boundary
 *     punctuation
 *     qualified names
 *     requirements
 *     source units
 *     versioning
 *     visibility
 *
 * These are language-wide foundations.
 *
 * They are not separate computational domains.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This orchestrator does NOT own:
 *
 *     classical computation
 *     quantum operations
 *     quantum IR
 *     HDL semantics
 *     hardware realization
 *     AI/ML semantics
 *     distributed execution
 *     networking
 *     security enforcement
 *     resource allocation
 *     scheduling
 *     routing
 *     optimization
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     target selection
 *     device discovery
 *     physical topology
 *     compiler backend implementation
 *
 * Those remain owned by their respective grammar and semantic subsystems.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Each core concept has exactly one canonical owner.
 *
 * Examples:
 *
 *     identifier
 *         -> CoreIdentifiers / Names
 *
 *     qualifiedName
 *         -> Names / QualifiedNames
 *
 *     path
 *         -> Paths
 *
 *     attribute
 *         -> Attributes
 *
 *     annotation
 *         -> Annotations
 *
 *     metadata
 *         -> Metadata
 *
 *     capability
 *         -> Capabilities
 *
 *     requirement
 *         -> Requirements
 *
 *     constraint
 *         -> Constraints
 *
 *     hint
 *         -> Hints
 *
 *     pragma
 *         -> Pragmas
 *
 *     modifier
 *         -> Modifiers
 *
 *     visibility
 *         -> Visibility
 *
 *     policy
 *         -> ZamaniPolicies
 *
 *     sourceUnit
 *         -> ZamaniSourceUnit
 *
 *     compilationUnit
 *         -> CompilationUnit
 *
 *     program
 *         -> ZamaniProgram
 *
 * This file MUST NOT reproduce those rules.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * The core composition MUST remain open-ended.
 *
 * It must not enumerate:
 *
 *     CPU models
 *     GPU models
 *     FPGA families
 *     ASIC technologies
 *     QPU vendors
 *     accelerator models
 *     operating systems
 *     cloud providers
 *     databases
 *     network vendors
 *     AI models
 *     quantum gates
 *     hardware devices
 *     deployment platforms
 *
 * New computational domains should normally integrate through:
 *
 *     names
 *     capabilities
 *     requirements
 *     constraints
 *     hints
 *     policies
 *     metadata
 *     attributes
 *     annotations
 *     dialects
 *
 * rather than changing this orchestrator.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Core syntax describes portable source intent.
 *
 * It MUST NOT establish language-level limits on:
 *
 *     qubits
 *     logical qubits
 *     physical resources
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     nodes
 *     processes
 *     tasks
 *     channels
 *     memory
 *     storage
 *     registers
 *     vector widths
 *     tensor dimensions
 *     tensor rank
 *     devices
 *     links
 *     topology
 *     timelines
 *     declarations
 *     functions
 *     modules
 *     source elements
 *
 * Practical limits belong to implementation resources, semantic requirements,
 * target capabilities, compiler resources, runtime resources, and deployment
 * constraints.
 *
 * They are not core grammar constants.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * INPUT
 * -----
 *
 * Canonical token vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Every imported parser grammar consumes the canonical ZamaniLexer vocabulary.
 *
 *
 * IMPORTED CORE COMPONENTS
 * ------------------------
 *
 *     Annotations
 *     Attributes
 *     ZamaniCoreBlocks
 *     Capabilities
 *     CompilationUnit
 *     Constraints
 *     Hints
 *     CoreIdentifiers
 *     Metadata
 *     Modifiers
 *     Names
 *     Paths
 *     ZamaniPolicies
 *     Pragmas
 *     ZamaniProgram
 *     Punctuation
 *     QualifiedNames
 *     Requirements
 *     ZamaniSourceUnit
 *     Versioning
 *     Visibility
 *
 * ============================================================================
 * COMPOSITION ORDER
 * ============================================================================
 *
 * The order below follows conceptual dependency direction.
 *
 * It is intentionally explicit even where ANTLR could obtain some rules
 * transitively. This makes the core-directory contract auditable and prevents
 * a newly added core component from becoming accidentally orphaned.
 *
 * ============================================================================
 */

parser grammar Core;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CORE COMPOSITION IMPORTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * These are grammar names, not filesystem paths.
 *
 * The build system MUST include grammar/core/ in the ANTLR grammar source
 * search path.
 *
 * There MUST NOT be another competing grammar named Core in:
 *
 *     grammar/antlr/
 *     grammar/
 *     another generated source directory
 *
 * The canonical Core grammar is:
 *
 *     grammar/core/core.g4
 *
 * ============================================================================
 */

import
    CoreIdentifiers,
    Names,
    QualifiedNames,
    Paths,

    Punctuation,

    Attributes,
    Annotations,
    Metadata,

    Versioning,

    Capabilities,
    Requirements,
    Constraints,
    Hints,

    Pragmas,

    Visibility,
    Modifiers,

    ZamaniCoreBlocks,

    ZamaniPolicies,

    ZamaniSourceUnit,
    CompilationUnit,
    ZamaniProgram
;


/*
 * ============================================================================
 * CORE COMPATIBILITY BRIDGES
 * ============================================================================
 *
 * These are deliberately tiny.
 *
 * They exist only because the current canonical parser composition root
 * references the names:
 *
 *     coreAttribute
 *     corePragma
 *
 * while the canonical leaf grammars expose:
 *
 *     attributes
 *     pragma
 *
 * The bridges prevent the parser composition root from knowing the internal
 * names of the leaf grammars.
 *
 * No semantic behavior belongs here.
 * ============================================================================
 */


/*
 * Canonical source attribute bridge.
 *
 * Owner:
 *     Attributes
 *
 * Semantic meaning:
 *     downstream AST / semantic analysis
 */
coreAttribute
    : attributes
    ;


/*
 * Canonical source pragma bridge.
 *
 * Owner:
 *     Pragmas
 *
 * Semantic meaning:
 *     downstream semantic/compiler policy
 */
corePragma
    : pragma
    ;


/*
 * ============================================================================
 * CORE FACADES
 * ============================================================================
 *
 * These aliases make the composition boundary explicit without creating
 * duplicate language semantics.
 *
 * They are useful to higher-level composition grammars that want to depend on
 * "the core" rather than importing individual foundational grammars.
 *
 * ============================================================================
 */


/*
 * Universal core name facade.
 *
 * Names remains the actual owner of identifier/name syntax.
 */
coreName
    : nameReference
    ;


/*
 * Universal qualified-name facade.
 *
 * QualifiedNames remains the actual owner of qualified-name references.
 */
coreQualifiedName
    : qualifiedName
    ;


/*
 * Universal path facade.
 *
 * Paths remains the actual owner of path syntax.
 */
corePath
    : path
    ;


/*
 * Universal metadata facade.
 *
 * Metadata remains the actual owner of metadata syntax.
 */
coreMetadata
    : metadata
    ;


/*
 * Universal capability facade.
 *
 * Capabilities remains the actual owner.
 */
coreCapability
    : capabilityExpression
    ;


/*
 * Universal requirement facade.
 *
 * Requirements remains the actual owner.
 */
coreRequirement
    : requirementExpression
    ;


/*
 * Universal constraint facade.
 *
 * Constraints remains the actual owner.
 */
coreConstraint
    : constraintExpression
    ;


/*
 * Universal hint facade.
 *
 * Hints remains the actual owner.
 */
coreHint
    : hintExpression
    ;


/*
 * Universal modifier facade.
 *
 * Modifiers remains the actual owner.
 */
coreModifier
    : modifier
    ;


/*
 * Universal annotation facade.
 *
 * Annotations remains the actual owner.
 */
coreAnnotation
    : annotation
    ;


/*
 * Universal policy facade.
 *
 * ZamaniPolicies remains the actual owner.
 */
corePolicy
    : policyDeclaration
    ;


/*
 * ============================================================================
 * CORE COLLECTION FACADES
 * ============================================================================
 *
 * These do not introduce limits.
 *
 * Repetition is delegated to the canonical component grammars.
 *
 * ============================================================================
 */

coreAnnotations
    : annotations
    ;


coreAttributes
    : attributeList
    ;


coreCapabilities
    : capabilityReferenceList
    ;


coreRequirements
    : requirementExpressionList
    ;


coreModifiers
    : modifierList
    ;


corePolicies
    : policyList
    ;


/*
 * ============================================================================
 * CORE SOURCE COMPOSITION FACADES
 * ============================================================================
 *
 * These expose the canonical source boundaries supplied by the imported
 * grammars.
 *
 * The actual owners remain:
 *
 *     ZamaniProgram
 *     ZamaniSourceUnit
 *     CompilationUnit
 *
 * This orchestrator does not redefine their internals.
 * ============================================================================
 */

coreProgram
    : program
    ;


coreSourceUnit
    : sourceUnit
    ;


coreCompilationUnit
    : compilationUnit
    ;


/*
 * ============================================================================
 * CORE BLOCK FACADE
 * ============================================================================
 *
 * Blocks are reusable source structure.
 *
 * The block grammar remains the owner of block syntax.
 * ============================================================================
 */

coreBlock
    : block
    ;


/*
 * ============================================================================
 * CORE VERSION FACADE
 * ============================================================================
 */

coreVersion
    : versionExpression
    ;


/*
 * ============================================================================
 * CORE VISIBILITY FACADE
 * ============================================================================
 */

coreVisibility
    : visibilityModifier
    ;


/*
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * The following direction is mandatory:
 *
 *     Core
 *       |
 *       v
 *     higher-level parser composition
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> contract analysis
 *       +--> policy analysis
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> other domain representations
 *       |
 *       v
 *     optimization / lowering / routing / scheduling
 *       |
 *       v
 *     target realization
 *
 * Core MUST remain upstream of all physical realization.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Core may provide:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     hints
 *     policies
 *     metadata
 *     attributes
 *     annotations
 *
 * concerning quantum computation.
 *
 * Core MUST NOT define:
 *
 *     quantum operations
 *     gates
 *     qubit allocation
 *     physical qubits
 *     coupling maps
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * Quantum semantics remain owned by grammar/quantum/ and the canonical
 * quantum semantic boundary remains quantum::ir.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Core provides universal source structure to grammar/classical/.
 *
 * Core MUST NOT define:
 *
 *     arithmetic semantics
 *     instruction selection
 *     register allocation
 *     vectorization
 *     CPU-specific behavior
 *     accelerator-specific lowering
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Core may express universal:
 *
 *     capabilities
 *     requirements
 *     constraints
 *     hints
 *     policies
 *     metadata
 *
 * but MUST NOT define:
 *
 *     physical pin counts
 *     fixed bus widths
 *     fixed register widths
 *     fixed memory sizes
 *     fixed FPGA resource counts
 *     fixed ASIC cell counts
 *     physical placement
 *     physical routing
 *
 * ============================================================================
 * AI / REASONING / LEARNING BOUNDARY
 * ============================================================================
 *
 * Core provides the generic foundations needed by higher-level reasoning,
 * learning, knowledge, adaptation, uncertainty, evidence, provenance, agent,
 * and explanation grammars.
 *
 * Core MUST NOT become an AI-specific grammar.
 *
 * New reasoning or learning capabilities should consume:
 *
 *     expressions
 *     types
 *     effects
 *     capabilities
 *     requirements
 *     constraints
 *     policies
 *     provenance
 *
 * from their owning subsystems.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING BOUNDARY
 * ============================================================================
 *
 * Core can represent universal intent concerning:
 *
 *     distributed execution
 *     capabilities
 *     requirements
 *     constraints
 *     policies
 *     metadata
 *
 * Core MUST NOT define:
 *
 *     node discovery
 *     transport protocols
 *     network topology
 *     service placement
 *     cluster scheduling
 *     distributed consensus
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Core policy syntax can be consumed by security grammars.
 *
 * Parsing a policy MUST NOT grant permission.
 *
 * Parsing a capability MUST NOT grant the capability.
 *
 * Parsing a requirement MUST NOT satisfy the requirement.
 *
 * Authorization, trust, identity, credentials and enforcement belong
 * downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file defines NO Rust AST.
 *
 * Imported parser contexts are lowered by the canonical frontend into the
 * domain-neutral AST.
 *
 * The AST must preserve, where applicable:
 *
 *     source spans
 *     source ordering
 *     qualification
 *     attributes
 *     annotations
 *     metadata
 *     capabilities
 *     requirements
 *     constraints
 *     hints
 *     pragmas
 *     policies
 *     version information
 *     block structure
 *
 * The AST must remain independent of:
 *
 *     LLVM
 *     QIR
 *     MLIR
 *     vendor-specific IR
 *     physical quantum topology
 *     hardware placement
 *     calibration
 *     scheduling
 *     routing
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Core parsing establishes structure only.
 *
 * Semantic analysis decides:
 *
 *     whether a name resolves;
 *     whether a capability exists;
 *     whether a requirement is satisfiable;
 *     whether a constraint is contradictory;
 *     whether a hint is applicable;
 *     whether a policy is valid;
 *     whether a version is compatible;
 *     whether metadata is valid;
 *     whether a modifier is legal in its context;
 *     whether a target can realize the requested semantics.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Core owns NO IR.
 *
 * The only valid direction is:
 *
 *     core parser
 *         |
 *         v
 *     AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         +--> other domain IR
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime MUST NOT depend on grammar/core as an execution mechanism.
 *
 * Runtime receives compiled semantic artifacts.
 *
 * Parsing is not execution.
 *
 * Core grammar MUST NOT:
 *
 *     allocate;
 *     execute;
 *     schedule;
 *     route;
 *     retry;
 *     recover;
 *     discover hardware;
 *     access files;
 *     access networks;
 *     access credentials;
 *     invoke foreign code.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     token stream;
 *     grammar version;
 *     parser configuration;
 *
 * parsing must produce equivalent parse structure.
 *
 * No core rule may depend upon:
 *
 *     time;
 *     randomness;
 *     hardware;
 *     filesystem state;
 *     network state;
 *     environment state;
 *     runtime state;
 *     target availability.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No core rule may encode a finite semantic maximum for:
 *
 *     names;
 *     qualified-name components;
 *     attributes;
 *     annotations;
 *     metadata entries;
 *     requirements;
 *     capabilities;
 *     constraints;
 *     hints;
 *     policies;
 *     policy members;
 *     source items;
 *     declarations;
 *     modules;
 *     blocks;
 *     program size;
 *     target size;
 *     machine size.
 *
 * Repetition and recursion are delegated to the owning grammars.
 *
 * "Infinity" means:
 *
 *     no artificial language-level upper bound is introduced where the
 *     semantics do not require one.
 *
 * It does not claim that finite implementations possess infinite resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO MAX_QUBITS
 *     NO MAX_CPUS
 *     NO MAX_CORES
 *     NO MAX_THREADS
 *     NO MAX_GPUS
 *     NO MAX_FPGAS
 *     NO MAX_ASICS
 *     NO MAX_ACCELERATORS
 *     NO MAX_QPUS
 *     NO MAX_NODES
 *     NO MAX_MEMORY
 *     NO MAX_STORAGE
 *     NO MAX_REGISTER_WIDTH
 *     NO MAX_TENSOR_RANK
 *     NO MAX_NETWORK_SIZE
 *     NO MAX_DEVICE_COUNT
 *
 * It also contains no equivalent indirect cardinality restriction.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * DOWNSTREAM
 * ----------
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 *     grammar/types/
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/declarations/
 *     grammar/functions/
 *     grammar/modules/
 *     grammar/effects/
 *     grammar/memory/
 *     grammar/concurrency/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/security/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/macros/
 *     grammar/metaprogramming/
 *
 * The higher-level domains consume core syntax.
 *
 * They must not redefine core concepts.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * ANTLR source search paths MUST include:
 *
 *     grammar/core/
 *
 * and the canonical lexer location required by the repository build.
 *
 * The generated parser must resolve:
 *
 *     import Core;
 *
 * from:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The repository MUST NOT maintain another authoritative Core grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The core composition must be tested as a composition, not merely as
 * independent leaf grammars.
 *
 * Required parser-level tests:
 *
 *     empty source
 *     source unit
 *     compilation unit
 *     program
 *     identifiers
 *     qualified names
 *     paths
 *     attributes
 *     annotations
 *     metadata
 *     versioning
 *     capabilities
 *     requirements
 *     constraints
 *     hints
 *     pragmas
 *     modifiers
 *     visibility
 *     blocks
 *     policies
 *
 * Required cross-domain tests:
 *
 *     core + classical
 *     core + quantum
 *     core + hybrid
 *     core + HDL
 *     core + hardware
 *     core + distributed
 *     core + AI
 *     core + networking
 *     core + security
 *
 * Required compound tests:
 *
 *     classical + quantum
 *     classical + quantum + distributed
 *     classical + quantum + HDL
 *     quantum + hardware
 *     AI + quantum
 *     AI + distributed
 *     HDL + hardware
 *     policy + resource + capability
 *     requirement + constraint + policy
 *
 * Required non-functional tests:
 *
 *     deterministic parsing
 *     generated-parser reproducibility
 *     compatibility
 *     scalability
 *     malformed-input diagnostics
 *     source-preservation
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The composition must reject malformed syntax without performing semantic
 * decisions.
 *
 * Examples of semantic conditions that MUST remain downstream:
 *
 *     unavailable capability
 *     unsatisfied requirement
 *     contradictory constraint
 *     unauthorized permission
 *     unavailable hardware
 *     impossible placement
 *     unsupported target
 *
 * These are not parser errors.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new core leaf grammar requires:
 *
 *     1. adding its parser grammar to grammar/core/;
 *     2. assigning exactly one grammar owner;
 *     3. documenting its dependency contract;
 *     4. adding it to this orchestrator;
 *     5. adding composition tests;
 *     6. updating grammar/core/README.md;
 *     7. updating grammar/specification/grammar-authority.md if required;
 *     8. verifying ZamaniParser integration;
 *     9. verifying AST integration;
 *    10. verifying semantic integration.
 *
 * A new domain MUST NOT require changing Core merely because the domain has
 * become popular or physically important.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * core.g4 is COMPLETE when:
 *
 *     [x] It is a parser grammar.
 *     [x] It uses tokenVocab = ZamaniLexer.
 *     [x] It is the sole orchestrator of grammar/core/.
 *     [x] Every core .g4 component is explicitly imported.
 *     [x] No domain grammar is imported here.
 *     [x] No lexer rule is defined here.
 *     [x] No machine limit is defined here.
 *     [x] No hardware is selected here.
 *     [x] No resource is allocated here.
 *     [x] No IR is created here.
 *     [x] No runtime behavior is implemented here.
 *     [x] No embedded Rust exists here.
 *     [x] No unsafe Rust is required.
 *     [x] Compatibility bridges are minimal.
 *     [x] Core concepts retain single ownership.
 *     [x] Domain neutrality is preserved.
 *     [x] Open-world extensibility is preserved.
 *     [x] POCO-REAF constraints are preserved.
 *
 * Verification still required by the repository:
 *
 *     [ ] ANTLR generation succeeds.
 *     [ ] All imports resolve.
 *     [ ] No imported-rule collisions remain.
 *     [ ] ZamaniParser generation succeeds.
 *     [ ] Rust frontend generation/integration succeeds.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Cross-domain tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Compatibility tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Core is a composition boundary, not a second language.
 *
 * The architectural relationship is:
 *
 *     core syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> requirements
 *          +--> constraints
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> future domain representations
 *          |
 *          v
 *     target-independent optimization/lowering
 *          |
 *          v
 *     target realization
 *
 * Therefore the core grammar remains stable as Zamani expands from very small
 * computational systems through increasingly capable heterogeneous,
 * distributed, quantum, classical, HDL, accelerator, HPC, and future
 * computational environments.
 *
 * ============================================================================
 */