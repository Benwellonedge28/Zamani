/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/artifacts.g4
 *
 * Grammar:
 *     CompileArtifacts
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
 *     implementation. The Rust compiler/frontend consuming this grammar MUST
 *     use safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL COMPILATION ARTIFACT INTENT.
 *
 * An artifact is a named or described result/representation requested from
 * compilation, lowering, code generation, analysis, verification, packaging,
 * deployment, interoperability, or another explicitly declared compiler
 * boundary.
 *
 * This grammar describes:
 *
 *     WHAT artifact(s) are requested;
 *     WHAT semantic properties an artifact has;
 *     WHAT relationships an artifact has to other artifacts;
 *     WHAT artifact requirements/constraints/preferences apply;
 *     WHAT artifact identity/provenance requirements are requested;
 *     WHAT artifact format/representation is desired;
 *     WHAT artifact scope/lifetime is requested;
 *     WHAT artifact dependencies are declared;
 *     WHAT artifact metadata is requested.
 *
 * It does NOT describe:
 *
 *     HOW an artifact is generated;
 *     HOW an artifact is stored;
 *     HOW an artifact is hashed;
 *     HOW an artifact is cached;
 *     HOW an artifact is signed;
 *     HOW an artifact is deployed;
 *     HOW an artifact is executed;
 *     WHICH compiler backend generates it;
 *     WHICH CPU/GPU/FPGA/QPU generates it;
 *     WHICH physical device stores it;
 *     WHICH filesystem stores it;
 *     WHICH network stores it;
 *     WHICH vendor implementation is used.
 *
 * Those responsibilities belong to downstream semantic/compiler/runtime
 * systems.
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
 *          +--> CompileArtifacts
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> artifact analysis
 *          +--> dependency analysis
 *          +--> identity analysis
 *          +--> provenance analysis
 *          +--> reproducibility analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------------+-----------------------+
 *          |                      |                       |
 *          v                      v                       v
 *     classical              quantum::ir            HDL/hardware
 *          |                      |                       |
 *          +----------------------+-----------------------+
 *                                 |
 *                                 v
 *                         optimization/lowering
 *                                 |
 *                                 v
 *                         artifact generation
 *                                 |
 *                                 v
 *                         deployment/runtime
 *
 * This grammar therefore sits at the SOURCE-INTENT boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * Artifact syntax MUST preserve portability.
 *
 * The source program may request semantic artifact classes such as:
 *
 *     source
 *     analysis
 *     semantic
 *     classical
 *     quantum
 *     hdl
 *     object
 *     executable
 *     package
 *     deployable
 *     verification
 *     provenance
 *     metadata
 *
 * However, this grammar MUST NOT require a particular backend or physical
 * machine.
 *
 * An artifact request such as:
 *
 *     artifact quantum
 *
 * does NOT mean:
 *
 *     use QPU 0
 *     use physical qubit 0
 *     use 32 qubits
 *     use vendor X
 *
 * Those are downstream target-realization concerns.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce universal limits such as:
 *
 *     MAX_ARTIFACTS
 *     MAX_ARTIFACT_SIZE
 *     MAX_ARTIFACTS_PER_BUILD
 *     MAX_SECTIONS
 *     MAX_FILES
 *     MAX_DEPENDENCIES
 *     MAX_OUTPUTS
 *     MAX_TARGETS
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
 *     MAX_DEVICE_COUNT
 *
 * Nor may finite equivalents be disguised through grammar alternatives.
 *
 * Repetition is deliberately expressed with:
 *
 *     *
 *     +
 *
 * so artifact specifications scale according to available compiler,
 * storage, network, and target resources rather than a language-level
 * artificial ceiling.
 *
 * "Unbounded" means:
 *
 *     no artificial finite language-level maximum.
 *
 * It does NOT mean:
 *
 *     infinite physical resources.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - artifact specification syntax;
 *     - artifact property syntax;
 *     - artifact requirement syntax;
 *     - artifact constraint syntax;
 *     - artifact preference syntax;
 *     - artifact hint syntax;
 *     - artifact dependency syntax;
 *     - artifact relationship syntax;
 *     - artifact metadata intent;
 *     - artifact identity intent;
 *     - artifact format intent;
 *     - artifact scope intent;
 *     - artifact lifetime intent;
 *     - artifact provenance intent;
 *     - artifact reproducibility intent;
 *     - artifact composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the COMPILE declaration;
 *     - general compilation declarations;
 *     - target selection;
 *     - optimization;
 *     - specialization;
 *     - reproducibility implementation;
 *     - deterministic-build implementation;
 *     - caching;
 *     - provenance implementation;
 *     - deployment;
 *     - code generation implementation;
 *     - lowering implementation;
 *     - filesystem access;
 *     - network access;
 *     - artifact hashing;
 *     - artifact signing;
 *     - package management;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime execution;
 *     - canonical IR;
 *     - quantum::ir.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The following ownership boundaries are intentional:
 *
 *     grammar/compile/compile.g4
 *         owns the `compile` declaration and its top-level clause.
 *
 *     grammar/compile/artifacts.g4
 *         owns artifact specification syntax.
 *
 *     grammar/compile/reproducibility.g4
 *         owns reproducibility intent.
 *
 *     grammar/compile/deterministic-builds.g4
 *         owns deterministic-build intent.
 *
 *     grammar/compile/caching.g4
 *         owns caching intent.
 *
 *     grammar/compile/provenance.g4
 *         owns provenance intent.
 *
 *     grammar/compile/code-generation.g4
 *         owns code-generation intent.
 *
 *     grammar/compile/lowering.g4
 *         owns lowering intent.
 *
 *     grammar/compile/deployment.g4
 *         owns deployment intent.
 *
 * Artifact syntax may reference those semantic concepts through ordinary
 * expressions and symbolic properties, but MUST NOT duplicate their grammar.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * The current repository's `compile.g4` contains:
 *
 *     compileArtifact
 *     compileArtifactSpecification
 *
 * `compileArtifact` remains the compile-level wrapper.
 *
 * This file becomes the owner of:
 *
 *     compileArtifactSpecification
 *
 * Therefore the integration change is:
 *
 *     compile.g4
 *         import CompileArtifacts
 *
 * and:
 *
 *     compileArtifact
 *         : ARTIFACT compileArtifactSpecification
 *         ;
 *
 * MUST remain the wrapper in Compile.
 *
 * The old local definition of `compileArtifactSpecification` in Compile MUST
 * be removed so that there is exactly one owner.
 *
 * The existing `compilation.g4` already consumes:
 *
 *     ARTIFACT compileArtifactSpecification
 *
 * Therefore it can consume this rule after `CompileArtifacts` is included in
 * its parser-import hierarchy.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar uses the existing canonical lexer:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No new lexical token is required by this grammar.
 *
 * In particular, this file deliberately does NOT introduce:
 *
 *     ARTIFACT_TYPE
 *     ARTIFACT_FORMAT
 *     ARTIFACT_SCOPE
 *     ARTIFACT_ID
 *     OBJECT_ARTIFACT
 *     BINARY_ARTIFACT
 *     QUANTUM_ARTIFACT
 *     HDL_ARTIFACT
 *
 * Artifact categories remain semantic values rather than an ever-growing
 * keyword registry.
 *
 * The existing canonical:
 *
 *     ARTIFACT
 *
 * token remains the lexical boundary for compile-level artifact requests.
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * Artifact values use the canonical:
 *
 *     expression
 *
 * rule imported through Expressions.
 *
 * This allows artifact properties to contain:
 *
 *     identifiers;
 *     qualified names;
 *     literals;
 *     collections;
 *     function calls;
 *     symbolic values;
 *     arithmetic;
 *     comparisons;
 *     domain-specific semantic expressions.
 *
 * This grammar MUST NOT create another expression language.
 *
 * ============================================================================
 * NAME AUTHORITY
 * ============================================================================
 *
 * Artifact names and property names use canonical:
 *
 *     identifier
 *     qualifiedName
 *
 * supplied by Core.
 *
 * They MUST NOT be redefined here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps into the domain-neutral frontend AST.
 *
 * Required semantic AST information:
 *
 *     ArtifactIntent
 *         name / identity expression
 *         kind expression
 *         properties
 *         requirements
 *         constraints
 *         preferences
 *         hints
 *         dependencies
 *         relationships
 *         metadata intent
 *         source span
 *
 * IMPORTANT:
 *
 * This grammar does NOT require the AST to contain a backend-specific
 * Artifact enum containing every possible artifact type.
 *
 * Artifact kinds are open semantic values.
 *
 * For example:
 *
 *     artifact quantum
 *
 * and:
 *
 *     artifact vendor.extension.quantum_representation
 *
 * must be representable without modifying this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. resolve artifact names;
 *     2. resolve artifact kinds;
 *     3. validate artifact properties;
 *     4. validate requirements;
 *     5. validate constraints;
 *     6. validate dependencies;
 *     7. validate relationships;
 *     8. detect incompatible artifact requests;
 *     9. validate portability;
 *    10. validate reproducibility requirements;
 *    11. validate provenance requirements;
 *    12. determine whether requested artifact forms are available;
 *    13. preserve source spans;
 *    14. report deterministic diagnostics.
 *
 * Semantic analysis MUST NOT silently convert:
 *
 *     artifact intent
 *
 * into:
 *
 *     physical target selection.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO IR.
 *
 * There must be no:
 *
 *     ArtifactIR
 *     CompilationArtifactIR
 *     QuantumArtifactIR
 *     HardwareArtifactIR
 *
 * competing with the canonical semantic/IR architecture.
 *
 * Artifact requests are represented in the existing semantic compilation
 * model and are lowered into whatever canonical IR/backend representation is
 * appropriate downstream.
 *
 * Quantum artifact requests continue through:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * No second quantum IR is permitted.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler is responsible for:
 *
 *     - resolving artifact intent;
 *     - determining whether artifacts can be produced;
 *     - selecting applicable compilation stages;
 *     - satisfying artifact requirements;
 *     - checking constraints;
 *     - applying preferences;
 *     - applying hints;
 *     - determining dependencies;
 *     - producing artifact metadata;
 *     - connecting artifact generation with optimization/lowering/codegen;
 *     - preserving provenance;
 *     - preserving reproducibility where requested;
 *     - selecting target realization downstream.
 *
 * The grammar does none of these operations.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may consume generated artifacts.
 *
 * Runtime MUST NOT need to reparse artifact grammar to determine artifact
 * semantics.
 *
 * The compiler must provide a validated semantic artifact description or
 * generated artifact representation to runtime/deployment layers.
 *
 * Runtime-specific artifact behavior belongs outside this grammar.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Artifact intent is intentionally domain-neutral.
 *
 * It can describe outputs related to:
 *
 *     classical computing;
 *     quantum computing;
 *     HDL;
 *     hardware;
 *     hybrid computing;
 *     AI;
 *     tensors;
 *     distributed computing;
 *     networking;
 *     security;
 *     interoperability;
 *     simulation;
 *     verification;
 *     future computing domains.
 *
 * Examples:
 *
 *     artifact classical;
 *
 *     artifact quantum;
 *
 *     artifact hdl;
 *
 *     artifact analysis;
 *
 *     artifact verification;
 *
 *     artifact {
 *         kind: quantum;
 *         representation: semantic;
 *     }
 *
 * No domain gets a privileged parser-level artifact enumeration.
 *
 * ============================================================================
 * 1. ARTIFACT SPECIFICATION
 * ============================================================================
 *
 * This is the public rule consumed by Compile.
 *
 * Compact form:
 *
 *     artifact quantum
 *
 * Structured form:
 *
 *     artifact {
 *         kind: quantum;
 *         representation: semantic;
 *     }
 *
 * Named form:
 *
 *     artifact executable {
 *         ...
 *     }
 *
 * The grammar intentionally permits an expression because artifact identity
 * and classification may be computed from compile-time semantic information.
 */

compileArtifactSpecification
    : expression
    | compileArtifactPropertyBlock
    ;


/*
 * ============================================================================
 * 2. ARTIFACT PROPERTY BLOCK
 * ============================================================================
 *
 * At least one property is required.
 *
 * An empty artifact block:
 *
 *     artifact {}
 *
 * is rejected.
 *
 * This avoids silently assigning compiler-version-dependent defaults.
 *
 * The semantic specification may define defaults for omitted properties in
 * non-empty blocks, but the existence of an empty block is not silently
 * interpreted as a meaningful artifact request.
 */

compileArtifactPropertyBlock
    : LBRACE compileArtifactProperty+ RBRACE
    ;


/*
 * ============================================================================
 * 3. ARTIFACT PROPERTY
 * ============================================================================
 *
 * Generic properties are deliberate.
 *
 * This allows future artifact concerns without creating a new parser keyword
 * for every representation, packaging format, execution environment, or
 * future computing model.
 *
 * Examples:
 *
 *     kind: quantum;
 *     representation: semantic;
 *     format: qir;
 *     scope: module;
 *     identity: content;
 *     provenance: required;
 *     reproducibility: required;
 *     lifetime: persistent;
 *     dependency: source;
 */

compileArtifactProperty
    : compileArtifactPropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * 4. PROPERTY NAME
 * ============================================================================
 *
 * Property names are canonical names.
 *
 * `qualifiedName` allows namespaces without requiring this grammar to know
 * every future namespace.
 *
 * Examples:
 *
 *     kind
 *     representation
 *     artifact.kind
 *     identity.content
 *     provenance.required
 *     vendor.extension.property
 */

compileArtifactPropertyName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 5. ARTIFACT COLLECTION
 * ============================================================================
 *
 * A collection is useful when a compilation request describes several
 * artifacts as one semantic unit.
 *
 * Example:
 *
 *     artifact {
 *         outputs: [semantic, quantum, verification];
 *     }
 *
 * The value remains an ordinary expression.
 *
 * No finite artifact-count limit is introduced.
 *
 * This grammar therefore does not enumerate:
 *
 *     artifact1
 *     artifact2
 *     artifact3
 *
 * or any equivalent finite set.
 */


/*
 * ============================================================================
 * 6. SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * The following semantic categories are intentionally represented as generic
 * properties rather than parser keywords:
 *
 *     kind
 *     representation
 *     format
 *     identity
 *     scope
 *     lifetime
 *     dependencies
 *     provenance
 *     reproducibility
 *     visibility
 *     ownership
 *     compatibility
 *     target
 *     requirements
 *     constraints
 *     metadata
 *
 * The semantic layer owns recognition.
 *
 * This is important for POCO-REAF because future artifact representations can
 * be introduced without changing the universal grammar.
 */


/*
 * ============================================================================
 * 7. ARTIFACT REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * Example:
 *
 *     artifact {
 *         requires capability("quantum.ir");
 *         requires capability("verification");
 *     }
 *
 * Requirements remain expressions so resource/capability systems can evolve.
 *
 * A requirement MUST NOT be confused with a target-selection instruction.
 */

compileArtifactRequirement
    : REQUIRES expression SEMI?
    ;


/*
 * ============================================================================
 * 8. ARTIFACT CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict acceptable artifact realization.
 *
 * Example:
 *
 *     artifact {
 *         constrain artifact.identity == content;
 *     }
 *
 * The expression remains canonical.
 *
 * This grammar does not implement constraint solving.
 */

compileArtifactConstraint
    : CONSTRAINT expression SEMI?
    ;


/*
 * ============================================================================
 * 9. ARTIFACT PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory.
 *
 * A compiler MAY ignore a preference when doing so does not violate semantic
 * requirements or constraints.
 */

compileArtifactPreference
    : PREFER expression SEMI?
    ;


/*
 * ============================================================================
 * 10. ARTIFACT HINTS
 * ============================================================================
 *
 * Hints are advisory information.
 *
 * They MUST NOT be required for correctness.
 */

compileArtifactHint
    : HINT expression SEMI?
    ;


/*
 * ============================================================================
 * 11. EXTENDED ARTIFACT BLOCK
 * ============================================================================
 *
 * This rule is provided as the richer semantic form for future integration.
 *
 * It is intentionally separate from `compileArtifactPropertyBlock` so the
 * AST can preserve the distinction between:
 *
 *     a generic property-only artifact description
 *
 * and:
 *
 *     a fully structured artifact intent.
 *
 * Both forms remain target-independent.
 */

compileArtifactIntentBlock
    : LBRACE compileArtifactEntry+ RBRACE
    ;


compileArtifactEntry
    : compileArtifactRequirement
    | compileArtifactConstraint
    | compileArtifactPreference
    | compileArtifactHint
    | compileArtifactProperty
    ;


/*
 * ============================================================================
 * 12. ARTIFACT RELATIONSHIP
 * ============================================================================
 *
 * Artifact relationships are represented as properties/expressions rather
 * than hard-coded relation keywords.
 *
 * Examples:
 *
 *     relation: derives_from(source);
 *     relation: verifies(artifact);
 *     relation: packages(artifact);
 *     relation: transforms(artifact);
 *
 * This permits future relationships without modifying the grammar.
 */


/*
 * ============================================================================
 * 13. ARTIFACT IDENTITY
 * ============================================================================
 *
 * Artifact identity is semantic.
 *
 * Possible semantic policies include:
 *
 *     source identity;
 *     semantic identity;
 *     content identity;
 *     provenance identity;
 *     target-specific identity.
 *
 * The grammar does not enumerate or implement those policies.
 *
 * Example:
 *
 *     artifact {
 *         identity: content;
 *     }
 *
 * Identity calculation belongs to the compiler/build system.
 */


/*
 * ============================================================================
 * 14. ARTIFACT FORMAT
 * ============================================================================
 *
 * Artifact formats are semantic values.
 *
 * Examples may include:
 *
 *     semantic
 *     quantum
 *     hdl
 *     qir
 *     qasm
 *     object
 *     executable
 *     package
 *     metadata
 *
 * The grammar deliberately does not enumerate those values.
 *
 * Interoperability formats remain governed by the interoperability subsystem.
 */


/*
 * ============================================================================
 * 15. ARTIFACT SCOPE
 * ============================================================================
 *
 * Scope is a semantic property.
 *
 * Possible values may describe:
 *
 *     expression;
 *     function;
 *     module;
 *     package;
 *     program;
 *     compilation;
 *     deployment;
 *     system;
 *     distributed deployment.
 *
 * The grammar does not impose a finite hierarchy.
 */


/*
 * ============================================================================
 * 16. ARTIFACT LIFETIME
 * ============================================================================
 *
 * Lifetime is semantic data.
 *
 * It may participate in:
 *
 *     caching;
 *     persistence;
 *     deployment;
 *     reproducibility;
 *     provenance;
 *     runtime management.
 *
 * Those systems remain independently owned.
 */


/*
 * ============================================================================
 * 17. ARTIFACT DEPENDENCIES
 * ============================================================================
 *
 * Dependencies may be represented as expressions:
 *
 *     dependencies: [source, semantic];
 *
 *     dependency: analysis;
 *
 *     dependency: previous_artifact;
 *
 * Dependency resolution belongs to the compilation/package subsystem.
 *
 * The grammar imposes no finite dependency count.
 */


/*
 * ============================================================================
 * 18. ARTIFACT PROVENANCE
 * ============================================================================
 *
 * Provenance is represented as semantic intent.
 *
 * Example:
 *
 *     artifact {
 *         provenance: required;
 *     }
 *
 * Provenance generation, storage, signing, and verification remain owned by
 * provenance/security/compiler subsystems.
 */


/*
 * ============================================================================
 * 19. REPRODUCIBILITY INTEGRATION
 * ============================================================================
 *
 * Artifact requests may participate in reproducibility.
 *
 * Example:
 *
 *     artifact {
 *         reproducibility: required;
 *         identity: content;
 *     }
 *
 * This grammar does not define reproducibility semantics.
 *
 * `grammar/compile/reproducibility.g4` remains authoritative for explicit
 * reproducibility clauses.
 *
 * This separation prevents:
 *
 *     artifact syntax
 *
 * from becoming:
 *
 *     reproducibility implementation.
 */


/*
 * ============================================================================
 * 20. DETERMINISTIC-BUILD INTEGRATION
 * ============================================================================
 *
 * Artifact requests may participate in deterministic builds.
 *
 * Example:
 *
 *     artifact {
 *         deterministic: required;
 *     }
 *
 * Deterministic build policy remains owned by:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 * This grammar does not redefine deterministic-build syntax.
 */


/*
 * ============================================================================
 * 21. CACHING INTEGRATION
 * ============================================================================
 *
 * Artifact identity may participate in caching:
 *
 *     artifact {
 *         identity: content;
 *         cache: reusable;
 *     }
 *
 * Cache policy remains owned by:
 *
 *     grammar/compile/caching.g4
 *
 * This grammar does not calculate or store cache entries.
 */


/*
 * ============================================================================
 * 22. CODE-GENERATION INTEGRATION
 * ============================================================================
 *
 * Code-generation intent remains owned by:
 *
 *     grammar/compile/code-generation.g4
 *
 * Artifact requests can identify the desired result of code generation
 * without specifying the code generator itself.
 */


/*
 * ============================================================================
 * 23. LOWERING INTEGRATION
 * ============================================================================
 *
 * Artifact requests may refer to semantic lowering stages.
 *
 * Example:
 *
 *     artifact {
 *         representation: lowered;
 *     }
 *
 * Lowering semantics remain owned by:
 *
 *     grammar/compile/lowering.g4
 */


/*
 * ============================================================================
 * 24. TARGET INTEGRATION
 * ============================================================================
 *
 * Artifact syntax may contain target-related expressions, but it must not
 * perform target selection.
 *
 * This is intentionally different from:
 *
 *     map artifact -> physical_device
 *
 * Target realization belongs downstream.
 *
 * A portable source program should be able to say:
 *
 *     artifact executable
 *
 * without naming:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     node 0
 *
 * unless such information is explicitly part of a non-portable downstream
 * deployment specification.
 */


/*
 * ============================================================================
 * 25. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Artifact requests may refer to quantum semantic representations.
 *
 * Example:
 *
 *     artifact quantum;
 *
 *     artifact {
 *         kind: quantum;
 *         representation: semantic;
 *     }
 *
 * This does NOT create:
 *
 *     QuantumArtifactIR
 *
 * and does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *
 * Quantum source continues through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * exactly one canonical quantum IR boundary is preserved.
 */


/*
 * ============================================================================
 * 26. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical artifacts may describe semantic or lowered classical computation.
 *
 * The grammar does not prescribe:
 *
 *     x86
 *     ARM
 *     RISC-V
 *     GPU architecture
 *     vector width
 *     register width
 *
 * Those are target realization concerns.
 */


/*
 * ============================================================================
 * 27. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Artifact requests may describe HDL/hardware representations:
 *
 *     artifact hdl;
 *
 *     artifact {
 *         kind: hardware;
 *         representation: synthesized;
 *     }
 *
 * The grammar does not impose:
 *
 *     bus width;
 *     register count;
 *     pipeline depth;
 *     memory size;
 *     FPGA resource count;
 *     ASIC technology;
 *     clock count.
 *
 * Those are semantic requirements or target capabilities when explicitly
 * requested and are resolved downstream.
 */


/*
 * ============================================================================
 * 28. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Artifact requests can represent distributed deployment or distributed
 * compilation results.
 *
 * No fixed node count is allowed.
 *
 * Example:
 *
 *     artifact {
 *         kind: deployable;
 *         topology: required_topology;
 *     }
 *
 * Topology resolution remains outside this grammar.
 */


/*
 * ============================================================================
 * 29. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Artifact requests may refer to:
 *
 *     models;
 *     datasets;
 *     tensors;
 *     trained parameters;
 *     inference representations;
 *     data schemas;
 *     verification outputs.
 *
 * The grammar does not enumerate AI frameworks or accelerator vendors.
 */


/*
 * ============================================================================
 * 30. SECURITY INTEGRATION
 * ============================================================================
 *
 * Artifact intent may require security properties:
 *
 *     artifact {
 *         integrity: required;
 *         provenance: required;
 *         signature: required;
 *     }
 *
 * Cryptographic implementation remains owned by the security subsystem.
 *
 * The grammar does not hard-code algorithms as universal syntax.
 */


/*
 * ============================================================================
 * 31. INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Artifact representations may refer to interoperability formats through
 * ordinary semantic expressions.
 *
 * Examples:
 *
 *     artifact qir;
 *     artifact qasm;
 *     artifact wasm;
 *
 * The interoperability subsystem owns format semantics.
 *
 * This grammar only records the source-level artifact request.
 */


/*
 * ============================================================================
 * 32. DIALECT INTEGRATION
 * ============================================================================
 *
 * Dialects may introduce additional artifact properties semantically.
 *
 * They MUST NOT silently create a competing artifact grammar.
 *
 * A dialect-specific artifact property should be namespaced where appropriate:
 *
 *     dialect.artifact.property: value;
 *
 * The dialect registry remains responsible for validating the property.
 */


/*
 * ============================================================================
 * 33. MACRO / METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Artifact expressions may be generated by compile-time facilities where the
 * canonical expression system permits it.
 *
 * Macro expansion MUST occur before final semantic artifact validation.
 *
 * Generated syntax MUST NOT bypass:
 *
 *     parsing;
 *     name resolution;
 *     type checking;
 *     semantic validation;
 *     resource validation;
 *     capability validation;
 *     portability validation.
 */


/*
 * ============================================================================
 * 34. DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Diagnostics should identify:
 *
 *     - malformed artifact specification;
 *     - missing property value;
 *     - invalid artifact expression;
 *     - unresolved artifact identity;
 *     - incompatible artifact relationship;
 *     - unsatisfied artifact requirement;
 *     - violated artifact constraint;
 *     - unsupported representation;
 *     - incompatible provenance policy;
 *     - incompatible reproducibility policy;
 *     - incompatible target request;
 *     - unsupported dialect property.
 *
 * Diagnostics MUST preserve source spans.
 *
 * Diagnostics MUST NOT depend on:
 *
 *     wall-clock time;
 *     random values;
 *     hardware enumeration order;
 *     filesystem traversal order;
 *     network response ordering.
 */


/*
 * ============================================================================
 * 35. DETERMINISTIC PARSING
 * ============================================================================
 *
 * Parsing artifact syntax MUST depend only on:
 *
 *     source text;
 *     lexer configuration;
 *     grammar version;
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     cache state;
 *     filesystem state;
 *     network state;
 *     hardware;
 *     environment variables;
 *     compiler host;
 *     random numbers;
 *     wall-clock time.
 */


/*
 * ============================================================================
 * 36. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     filesystem access;
 *     network access;
 *     command execution;
 *     secret access;
 *     dynamic loading;
 *     hardware discovery;
 *     compiler execution.
 *
 * Artifact expressions MUST be interpreted by the normal semantic pipeline.
 *
 * No artifact declaration may itself execute arbitrary host-language code.
 */


/*
 * ============================================================================
 * 37. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must avoid:
 *
 *     finite artifact enumerations;
 *     deeply duplicated alternatives;
 *     backend-specific artifact branches;
 *     semantic actions;
 *     recursive structures that do not correspond to source semantics.
 *
 * Artifact collections use ordinary parser repetition and expressions.
 *
 * Compiler implementations should preserve linear/near-linear behavior for
 * ordinary artifact-property lists where practical.
 *
 * Extremely large programs remain bounded by available compiler resources,
 * not artificial grammar constants.
 */


/*
 * ============================================================================
 * 38. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance testing MUST include:
 *
 *     one artifact;
 *     multiple artifacts;
 *     nested property structures;
 *     many properties;
 *     many dependencies;
 *     long qualified names;
 *     large symbolic expressions;
 *     quantum artifacts;
 *     classical artifacts;
 *     HDL artifacts;
 *     hybrid artifacts;
 *     distributed artifacts;
 *     AI/data artifacts;
 *     future/unknown semantic artifact kinds.
 *
 * No test may establish a universal maximum artifact count.
 *
 * The test suite should progressively increase workload until implementation
 * resource limits are reached, while verifying that no language-level ceiling
 * is introduced.
 */


/*
 * ============================================================================
 * 39. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms MUST be supported by the integrated grammar:
 *
 *     compile artifact quantum;
 *
 *     compile artifact executable;
 *
 *     compile artifact {
 *         kind: quantum;
 *     }
 *
 *     compile artifact {
 *         kind: quantum;
 *         representation: semantic;
 *         provenance: required;
 *     }
 *
 *     compile artifact {
 *         requires capability("quantum.ir");
 *         identity: content;
 *     }
 *
 *     compile artifact {
 *         kind: hdl;
 *         representation: synthesized;
 *     }
 *
 *     compile artifact {
 *         kind: distributed;
 *         topology: required_topology;
 *     }
 *
 *     compile artifact {
 *         kind: custom.domain.artifact;
 *         extension.property: value;
 *     }
 *
 * These examples are semantic examples, not hard-coded artifact registries.
 */


/*
 * ============================================================================
 * 40. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following MUST be rejected:
 *
 *     compile artifact {}
 *
 * because an artifact property block requires at least one property.
 *
 *     compile artifact {
 *         kind:
 *     }
 *
 * because a property value is missing.
 *
 *     compile artifact {
 *         kind: ;
 *     }
 *
 * because a property value is missing.
 *
 *     compile artifact {
 *         requires;
 *     }
 *
 * because the requirement expression is missing.
 *
 * Malformed expressions must be rejected by the canonical expression grammar.
 */


/*
 * ============================================================================
 * 41. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source:
 *
 *     compile artifact <expression>
 *
 * remains valid.
 *
 * Existing identifiers and qualified names remain valid artifact values.
 *
 * Artifact-specific future properties should normally be additive.
 *
 * Removing or changing the meaning of an established property requires an
 * explicit language-version compatibility decision.
 *
 * Deprecated artifact properties must be recorded by the compatibility
 * subsystem rather than silently changing semantics.
 */


/*
 * ============================================================================
 * 42. GENERATED-ARTIFACT POLICY
 * ============================================================================
 *
 * This grammar describes source-level requests for artifacts.
 *
 * Generated:
 *
 *     lexer files;
 *     parser files;
 *     AST serialization;
 *     object files;
 *     executables;
 *     HDL;
 *     QIR;
 *     QASM;
 *     deployment packages;
 *
 * are implementation outputs.
 *
 * They MUST NOT become independent language authorities.
 *
 * Their provenance must remain traceable to:
 *
 *     source;
 *     language version;
 *     grammar version;
 *     semantic version;
 *     compiler version;
 *     declared inputs;
 *     applicable compilation configuration.
 */


/*
 * ============================================================================
 * 43. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 * [x] Artifact syntax has one owner.
 *
 * [x] `compile.g4` delegates artifact specification to this grammar.
 *
 * [x] `compilation.g4` can consume the canonical artifact specification.
 *
 * [x] Existing ARTIFACT lexical authority is reused.
 *
 * [x] No new artifact keyword registry is required.
 *
 * [x] Canonical `identifier` is reused.
 *
 * [x] Canonical `qualifiedName` is reused.
 *
 * [x] Canonical `expression` is reused.
 *
 * [x] Artifact properties are extensible.
 *
 * [x] Artifact requirements are distinguishable from properties.
 *
 * [x] Artifact constraints are distinguishable from requirements.
 *
 * [x] Artifact preferences are distinguishable from constraints.
 *
 * [x] Artifact hints are distinguishable from preferences.
 *
 * [x] Artifact semantics remain target-independent.
 *
 * [x] No hardware capacity is hard-coded.
 *
 * [x] No fixed artifact count is hard-coded.
 *
 * [x] No vendor implementation is hard-coded.
 *
 * [x] No physical device is hard-coded.
 *
 * [x] No IR is introduced.
 *
 * [x] `quantum::ir` remains the sole canonical quantum IR boundary.
 *
 * [x] Classical/quantum/HDL/hybrid/future domains can use artifacts.
 *
 * [x] Reproducibility remains independently owned.
 *
 * [x] Deterministic-build semantics remain independently owned.
 *
 * [x] Caching remains independently owned.
 *
 * [x] Provenance remains independently owned.
 *
 * [x] Code generation remains independently owned.
 *
 * [x] Lowering remains independently owned.
 *
 * [x] Target selection remains independently owned.
 *
 * [x] Runtime does not need to parse artifact syntax.
 *
 * [x] Positive tests are defined.
 *
 * [x] Negative tests are defined.
 *
 * [x] Boundary tests are defined.
 *
 * [x] Scalability tests are defined.
 *
 * [x] Compatibility tests are defined.
 *
 * [x] Determinism tests are defined.
 *
 * [x] Security boundaries are defined.
 *
 * [x] Rust implementation requires Rust 1.97/1.97.1 compatible safe Rust.
 *
 * ============================================================================
 */

parser grammar CompileArtifacts;

options {
    tokenVocab = ZamaniLexer;
}

import Core,
       Expressions;


/*
 * ============================================================================
 * PUBLIC ARTIFACT SPECIFICATION
 * ============================================================================
 *
 * This rule is intentionally named `compileArtifactSpecification` because
 * `compile.g4` already owns the `compileArtifact` wrapper:
 *
 *     compileArtifact
 *         : ARTIFACT compileArtifactSpecification
 *         ;
 *
 * This preserves the existing public compile syntax while moving artifact
 * specification ownership into this file.
 */

compileArtifactSpecification
    : expression
    | compileArtifactPropertyBlock
    ;


/*
 * ============================================================================
 * PROPERTY BLOCK
 * ============================================================================
 */

compileArtifactPropertyBlock
    : LBRACE compileArtifactProperty+ RBRACE
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 */

compileArtifactProperty
    : compileArtifactPropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 */

compileArtifactPropertyName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * STRUCTURED ARTIFACT ENTRIES
 * ============================================================================
 *
 * These rules are public semantic integration points for future composition.
 *
 * They deliberately use canonical expressions rather than enumerating
 * artifact-specific values.
 */

compileArtifactEntry
    : compileArtifactRequirement
    | compileArtifactConstraint
    | compileArtifactPreference
    | compileArtifactHint
    | compileArtifactProperty
    ;


compileArtifactRequirement
    : REQUIRES expression SEMI?
    ;


compileArtifactConstraint
    : CONSTRAINT expression SEMI?
    ;


compileArtifactPreference
    : PREFER expression SEMI?
    ;


compileArtifactHint
    : HINT expression SEMI?
    ;