/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/compilation.g4
 *
 * Grammar:
 *     Compilation
 *
 * Status:
 *     Production compilation-orchestration parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     Compiler/frontend implementation MUST use safe Rust.
 *     Rust `unsafe` is not required by this grammar and MUST NOT be used by
 *     the production Zamani compiler implementation.
 *
 * Primary portability objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the COMPOSITION of source-level compilation intent.
 *
 * It is deliberately NOT another implementation of:
 *
 *     compile.g4
 *     target.g4
 *     optimization.g4
 *     compile-time.g4
 *     conditional-compilation.g4
 *     feature-selection.g4
 *     code-generation.g4
 *     lowering.g4
 *
 * Those files already own their respective syntax.
 *
 * This file answers:
 *
 *     "How are the independently-owned compilation-intent constructs
 *      composed into one compilation specification?"
 *
 * It therefore provides:
 *
 *     - the canonical compilation-specification boundary;
 *     - compilation-plan composition;
 *     - compilation-phase ordering intent;
 *     - compilation-unit composition;
 *     - compilation-profile composition;
 *     - compilation-policy composition;
 *     - source-level compilation artifact relationships;
 *     - compile-once / reusable compilation intent;
 *     - portability-preserving compilation orchestration;
 *     - explicit integration boundaries for existing compile grammars.
 *
 * It does NOT:
 *
 *     - execute compilation;
 *     - perform optimization;
 *     - perform target discovery;
 *     - allocate resources;
 *     - route operations;
 *     - schedule operations;
 *     - perform QEC;
 *     - perform ZQN;
 *     - discover hardware;
 *     - select physical devices;
 *     - generate machine code;
 *     - construct quantum::ir;
 *     - construct a second IR;
 *     - access the filesystem;
 *     - access the network;
 *     - execute host-language code.
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
 *     Compilation grammar
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> portability analysis
 *          +--> compilation planning
 *          |
 *          v
 *     canonical semantic representation
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
 *                    +----------+----------+
 *                    |                     |
 *                    v                     v
 *                 routing              scheduling
 *                    |                     |
 *                    +----------+----------+
 *                               |
 *                               v
 *                         resilience / QEC
 *                               |
 *                               v
 *                              ZQN
 *                               |
 *                               v
 *                              HAL
 *                               |
 *                               v
 *                       target realization
 *                               |
 *                               v
 *                            runtime
 *
 * ============================================================================
 * FUNDAMENTAL OWNERSHIP RULE
 * ============================================================================
 *
 * compilation.g4 OWNS:
 *
 *     - compilation specification composition;
 *     - compilation-plan structure;
 *     - compilation phase composition;
 *     - reusable compilation profiles;
 *     - compilation policy composition;
 *     - relationships between independently-owned compilation intents;
 *     - source-level compilation artifact relationships;
 *     - explicit compilation boundaries;
 *     - compile-once artifact intent;
 *     - portable compilation orchestration.
 *
 * compilation.g4 DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - types;
 *     - ordinary statements;
 *     - target semantics;
 *     - optimization semantics;
 *     - compile-time expression semantics;
 *     - feature-selection semantics;
 *     - conditional-compilation semantics;
 *     - code-generation semantics;
 *     - lowering semantics;
 *     - resources;
 *     - hardware;
 *     - execution;
 *     - quantum operations;
 *     - classical operations;
 *     - HDL operations;
 *     - canonical IR;
 *     - backend implementations.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The existing files remain authoritative for their domains:
 *
 *     compile.g4
 *         general compilation-intent declarations
 *
 *     target.g4
 *         compilation target intent
 *
 *     optimization.g4
 *         optimization intent
 *
 *     compile-time.g4
 *         compilation-time control
 *
 *     conditional-compilation.g4
 *         conditional source selection
 *
 *     feature-selection.g4
 *         feature-selection syntax
 *
 *     code-generation.g4
 *         code-generation intent
 *
 *     lowering.g4
 *         lowering intent
 *
 * This file MUST reference those constructs rather than duplicate them.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Compilation orchestration MUST preserve source portability.
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
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_ACCELERATOR_COUNT
 *     MAX_PIPELINE_STAGES
 *     MAX_COMPILATION_PHASES
 *     MAX_ARTIFACTS
 *     MAX_PROFILES
 *     MAX_TARGETS
 *     MAX_FEATURES
 *
 * Nor may it encode disguised equivalents.
 *
 * The following are also prohibited as universal language assumptions:
 *
 *     fixed physical device identifiers;
 *     fixed physical qubit identifiers;
 *     fixed CPU identifiers;
 *     fixed GPU identifiers;
 *     fixed FPGA identifiers;
 *     fixed node identifiers;
 *     fixed memory addresses;
 *     fixed hardware topology;
 *     fixed vendor implementation;
 *     fixed register width;
 *     fixed accelerator count.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is deliberately represented with:
 *
 *     *
 *     +
 *     recursive composition
 *
 * rather than finite alternatives.
 *
 * A compilation specification can therefore contain:
 *
 *     any number of phases;
 *     any number of policies;
 *     any number of artifacts;
 *     any number of profiles;
 *     any number of requirements;
 *     any number of constraints;
 *     any number of preferences;
 *     any number of hints;
 *     any number of target intents;
 *     any number of feature requests;
 *     any number of lowering requests;
 *     any number of generation requests.
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical memory;
 *     infinite compiler memory;
 *     infinite execution time;
 *     infinite hardware;
 *     infinite network capacity.
 *
 * Actual feasibility belongs downstream to:
 *
 *     resource analysis;
 *     capability analysis;
 *     compilation policy;
 *     target resolution;
 *     scheduler;
 *     runtime;
 *     deployment;
 *     hardware.
 *
 * ============================================================================
 * COMPILATION SEMANTIC CATEGORIES
 * ============================================================================
 *
 * The grammar deliberately preserves these distinctions:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     target
 *     resource
 *     artifact
 *     phase
 *     policy
 *     profile
 *     lowering
 *     generation
 *
 * They MUST NOT be collapsed into an untyped "compiler options" map.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * A portable program describes:
 *
 *     WHAT computation means;
 *     WHAT must be preserved;
 *     WHAT capabilities are required;
 *     WHAT resources are required;
 *     WHAT constraints apply;
 *     WHAT implementations are preferred;
 *     WHAT transformations are permitted.
 *
 * It does not unnecessarily describe:
 *
 *     WHERE the program executes;
 *     WHICH physical device executes it;
 *     WHICH physical qubit is used;
 *     WHICH CPU core is used;
 *     WHICH GPU is used;
 *     WHICH node is used;
 *     WHICH memory bank is used.
 *
 * Compilation intent therefore describes semantic compilation policy, not
 * physical realization.
 *
 * ============================================================================
 * CANONICAL IR RULE
 * ============================================================================
 *
 * This grammar MUST NOT introduce an IR.
 *
 * In particular, it MUST NOT introduce:
 *
 *     CompilationIR
 *     QuantumCompilationIR
 *     HardwareCompilationIR
 *     TargetCompilationIR
 *
 * as competing semantic representations.
 *
 * Compilation syntax lowers into the existing domain-neutral frontend AST,
 * semantic compilation model, and canonical IR architecture.
 *
 * Quantum computation continues through:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * There is exactly one canonical quantum IR boundary.
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Compilation;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Existing grammars remain the owners of their individual constructs.
 *
 * Compilation imports them only to expose them through one orchestration
 * boundary.
 *
 * Dependency direction:
 *
 *     lexer
 *       |
 *       +--> Compile
 *       +--> CompileTime
 *       +--> ConditionalCompilation
 *       +--> FeatureSelection
 *       +--> CompileTarget
 *       +--> CompileOptimization
 *       +--> CompileCodeGeneration
 *       +--> CompileLowering
 *       |
 *       v
 *     Compilation
 *       |
 *       v
 *     canonical parser composition
 *
 * No imported compilation grammar may import Compilation back.
 *
 * This prevents cyclic grammar authority.
 */

import Compile,
       CompileTime,
       ConditionalCompilation,
       FeatureSelection,
       CompileTarget,
       CompileOptimization,
       CompileCodeGeneration,
       CompileLowering;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A complete compilation specification is a source-level declaration.
 *
 * This rule is intentionally named differently from `compileDeclaration`.
 *
 * `compileDeclaration` remains owned by:
 *
 *     grammar/compile/compile.g4
 *
 * `compilation` owns the composition of compilation constructs.
 *
 * This prevents this file from stealing ownership of the existing `compile`
 * declaration.
 */

compilation
    : compilationSpecification EOF
    ;


/*
 * ============================================================================
 * 2. COMPILATION SPECIFICATION
 * ============================================================================
 *
 * A compilation specification contains one or more compilation elements.
 *
 * There is no finite maximum number of elements.
 *
 * An empty specification is rejected because it has no compilation meaning.
 *
 * If a future language revision requires an empty compilation unit to have
 * semantics, that change must be specified explicitly rather than silently
 * inferred by this grammar.
 */

compilationSpecification
    : compilationElement+
    ;


/*
 * ============================================================================
 * 3. COMPILATION ELEMENT
 * ============================================================================
 *
 * Each existing grammar retains ownership of its syntax.
 *
 * This rule only composes those independently-defined constructs.
 */

compilationElement
    : compileDeclaration
    | compilationPlan
    | compilationProfile
    | compilationPolicy
    | compilationPipeline
    | compilationArtifactSet
    | compilationBoundary
    | compilationReference
    ;


/*
 * ============================================================================
 * 4. COMPILATION PLAN
 * ============================================================================
 *
 * A compilation plan groups compilation intent without executing it.
 *
 * Example:
 *
 *     compile plan {
 *         ...
 *     }
 *
 * The internal clauses remain independently owned.
 *
 * A plan may contain an arbitrary number of entries.
 */

compilationPlan
    : PLAN LBRACE compilationPlanEntry+ RBRACE
    ;


compilationPlanEntry
    : compilationPlanReference
    | compilationPhase
    | compilationPolicyEntry
    | compilationArtifactReference
    | compilationRequirementReference
    | compilationConstraintReference
    | compilationPreferenceReference
    | compilationHintReference
    | compilationFeatureReference
    | compilationTargetReference
    | compilationOptimizationReference
    | compilationCodeGenerationReference
    | compilationLoweringReference
    | compilationConditionalReference
    | compilationFeatureSelectionReference
    ;


/*
 * ============================================================================
 * 5. PLAN REFERENCES
 * ============================================================================
 *
 * These wrappers provide stable composition points without reproducing the
 * underlying grammar.
 *
 * The wrappers are deliberately small so that semantic analysis can preserve
 * ownership information.
 */

compilationPlanReference
    : STAGE compileStageSpecification SEMI?
    ;


compilationRequirementReference
    : REQUIRES compileRequirementValue
    ;


compilationConstraintReference
    : CONSTRAIN compileConstraintValue
    ;


compilationPreferenceReference
    : PREFER compilePreferenceValue
    ;


compilationHintReference
    : HINT compileHintValue
    ;


compilationFeatureReference
    : FEATURE compileFeatureValue
    ;


compilationArtifactReference
    : ARTIFACT compileArtifactSpecification
    ;


/*
 * ============================================================================
 * 6. TARGET REFERENCE
 * ============================================================================
 *
 * Target syntax remains owned by CompileTarget.
 *
 * This rule is only an integration boundary.
 *
 * No physical target information is introduced here.
 */

compilationTargetReference
    : compilationTargetIntent
    ;


compilationTargetIntent
    : targetDeclaration
    ;


/*
 * ============================================================================
 * 7. OPTIMIZATION REFERENCE
 * ============================================================================
 *
 * Optimization intent remains owned by CompileOptimization.
 *
 * This file does not reproduce optimization grammar.
 */

compilationOptimizationReference
    : optimizationDeclaration
    ;


/*
 * ============================================================================
 * 8. CODE-GENERATION REFERENCE
 * ============================================================================
 *
 * Code generation remains source-level intent.
 *
 * This does not generate code.
 */

compilationCodeGenerationReference
    : codeGenerationDeclaration
    ;


/*
 * ============================================================================
 * 9. LOWERING REFERENCE
 * ============================================================================
 *
 * Lowering remains downstream of semantic analysis.
 */

compilationLoweringReference
    : loweringDeclaration
    ;


/*
 * ============================================================================
 * 10. CONDITIONAL COMPILATION REFERENCE
 * ============================================================================
 *
 * Conditional source selection remains owned by
 *
 *     conditional-compilation.g4
 *
 * It is not runtime branching.
 */

compilationConditionalReference
    : conditionalCompilationDirective
    ;


/*
 * ============================================================================
 * 11. FEATURE SELECTION REFERENCE
 * ============================================================================
 *
 * Feature selection remains owned by feature-selection.g4.
 */

compilationFeatureSelectionReference
    : featureSelectionDirective
    ;


/*
 * ============================================================================
 * 12. COMPILATION PROFILE
 * ============================================================================
 *
 * A profile is a reusable semantic grouping of compilation intent.
 *
 * Profiles are symbolic declarations.
 *
 * A profile MUST NOT encode a physical machine inventory.
 *
 * Example:
 *
 *     compile profile portable_quantum {
 *         ...
 *     }
 *
 * The profile may contain arbitrarily many entries.
 */

compilationProfile
    : PROFILE identifier LBRACE compilationProfileEntry+ RBRACE
    ;


compilationProfileEntry
    : compilationProfileReference
    | compilationPolicyEntry
    | compilationPhase
    | compilationArtifactReference
    | compilationRequirementReference
    | compilationConstraintReference
    | compilationPreferenceReference
    | compilationHintReference
    | compilationFeatureReference
    | compilationTargetReference
    | compilationOptimizationReference
    | compilationCodeGenerationReference
    | compilationLoweringReference
    ;


compilationProfileReference
    : identifier SEMI?
    ;


/*
 * ============================================================================
 * 13. COMPILATION POLICY
 * ============================================================================
 *
 * A policy describes how compilation decisions are constrained or guided.
 *
 * A policy is not a backend implementation.
 */

compilationPolicy
    : POLICY identifier compilationPolicyBody
    ;


compilationPolicyBody
    : LBRACE compilationPolicyEntry+ RBRACE
    ;


compilationPolicyEntry
    : compilationPolicyAssignment
    | compilationPolicyReference
    ;


compilationPolicyAssignment
    : compilationPropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * 14. COMPILATION PIPELINE
 * ============================================================================
 *
 * A pipeline is a source-level description of desired compilation-stage
 * composition.
 *
 * It does NOT execute stages.
 *
 * The compiler constructs the actual implementation pipeline later.
 */

compilationPipeline
    : PIPELINE identifier? LBRACE compilationPipelineEntry+ RBRACE
    ;


compilationPipelineEntry
    : compilationPhase
    | compilationPolicyEntry
    | compilationArtifactReference
    | compilationReference
    ;


compilationPhase
    : STAGE compilationStageReference SEMI?
    ;


compilationStageReference
    : identifier
    | qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * 15. COMPILATION ARTIFACT SET
 * ============================================================================
 *
 * This describes relationships between requested compilation artifacts.
 *
 * It does not define an artifact implementation or file format.
 */

compilationArtifactSet
    : ARTIFACTS LBRACE compilationArtifactSetEntry+ RBRACE
    ;


compilationArtifactSetEntry
    : compilationArtifactRelation
    | compilationArtifactReference
    | compilationArtifactProperty
    ;


compilationArtifactRelation
    : compilationArtifactReference
      compilationArtifactRelationOperator
      compilationArtifactReference
      SEMI?
    ;


compilationArtifactRelationOperator
    : DERIVES
    | PRODUCES
    | CONSUMES
    | PRESERVES
    | TRANSFORMS
    ;


compilationArtifactProperty
    : compilationPropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * 16. COMPILATION BOUNDARY
 * ============================================================================
 *
 * A boundary explicitly marks a semantic compilation boundary.
 *
 * This is useful for:
 *
 *     - portable compilation artifacts;
 *     - reusable compiled representations;
 *     - specialization boundaries;
 *     - target realization boundaries;
 *     - domain lowering boundaries.
 *
 * It does NOT create another IR.
 */

compilationBoundary
    : BOUNDARY identifier compilationBoundaryBody
    ;


compilationBoundaryBody
    : LBRACE compilationBoundaryEntry+ RBRACE
    ;


compilationBoundaryEntry
    : compilationBoundaryInput
    | compilationBoundaryOutput
    | compilationBoundaryPolicy
    | compilationArtifactReference
    | compilationReference
    ;


compilationBoundaryInput
    : INPUT compilationBoundaryReference SEMI?
    ;


compilationBoundaryOutput
    : OUTPUT compilationBoundaryReference SEMI?
    ;


compilationBoundaryPolicy
    : POLICY compilationPolicyReference SEMI?
    ;


compilationBoundaryReference
    : identifier
    | qualifiedName
    | STRING
    ;


compilationPolicyReference
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 17. COMPILATION REFERENCE
 * ============================================================================
 *
 * A reference allows a compilation specification to reuse an existing
 * compilation profile, policy, pipeline, boundary, or named compilation
 * contract.
 *
 * It is symbolic.
 *
 * It does not cause execution.
 */

compilationReference
    : USE COMPILATION identifier SEMI?
    ;


/*
 * ============================================================================
 * 18. GENERIC PROPERTY NAME
 * ============================================================================
 *
 * The property namespace is intentionally open-ended.
 *
 * New semantic properties do not require a parser modification merely because
 * the property vocabulary grows.
 */

compilationPropertyName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 19. INDEPENDENT COMPILATION INTENT INTEGRATION
 * ============================================================================
 *
 * The following existing constructs remain independently owned:
 *
 *     compileDeclaration
 *     targetDeclaration
 *     optimizationDeclaration
 *     codeGenerationDeclaration
 *     loweringDeclaration
 *     conditionalCompilationDirective
 *     featureSelectionDirective
 *
 * This file MUST NOT redefine any of those rules.
 *
 * Their AST and semantic contracts remain owned by their respective
 * specifications.
 */


/*
 * ============================================================================
 * 20. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST preserve the distinction between:
 *
 *     CompilationSpecification
 *     CompilationPlan
 *     CompilationProfile
 *     CompilationPolicy
 *     CompilationPipeline
 *     CompilationArtifactSet
 *     CompilationBoundary
 *     CompilationReference
 *
 * and the independently-owned child constructs.
 *
 * Conceptually:
 *
 *     CompilationSpecification
 *         elements: Vec<CompilationElement>
 *
 *     CompilationPlan
 *         entries: Vec<CompilationPlanEntry>
 *
 *     CompilationProfile
 *         name
 *         entries
 *
 *     CompilationPolicy
 *         name
 *         entries
 *
 *     CompilationPipeline
 *         name
 *         stages
 *         policies
 *
 *     CompilationArtifactSet
 *         entries
 *
 *     CompilationBoundary
 *         name
 *         inputs
 *         outputs
 *         policies
 *
 *     CompilationReference
 *         name
 *
 * Every node MUST preserve:
 *
 *     source span
 *     attributes where applicable
 *     semantic ownership
 *
 * The AST MUST NOT prematurely resolve:
 *
 *     target
 *     device
 *     resource
 *     capability
 *     optimization implementation
 *     lowering implementation
 *     backend
 *     schedule
 *     route
 *     physical mapping
 *
 * Those belong downstream.
 */


/*
 * ============================================================================
 * 21. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. resolve compilation profile names;
 *     2. resolve policy names;
 *     3. resolve stage names;
 *     4. resolve artifact references;
 *     5. validate stage relationships;
 *     6. validate policy composition;
 *     7. validate target intent;
 *     8. validate capability requirements;
 *     9. validate resource requirements;
 *    10. validate optimization intent;
 *    11. validate lowering intent;
 *    12. validate code-generation intent;
 *    13. validate feature-selection semantics;
 *    14. validate conditional-compilation semantics;
 *    15. validate portability requirements;
 *    16. preserve semantic distinctions between requirement, constraint,
 *        preference and hint;
 *    17. reject conflicting compilation policies;
 *    18. reject unknown required capabilities;
 *    19. reject unresolved mandatory compilation references;
 *    20. produce deterministic diagnostics.
 *
 * Semantic analysis MUST NOT:
 *
 *     - probe hardware directly from grammar nodes;
 *     - mutate runtime state;
 *     - execute arbitrary host code;
 *     - allocate physical resources;
 *     - route quantum operations;
 *     - schedule hardware operations;
 *     - construct a second quantum IR.
 */


/*
 * ============================================================================
 * 22. RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain abstract.
 *
 * Valid semantic examples include:
 *
 *     requires qubits >= n
 *     requires memory >= required_memory
 *     requires capability("tensor.compute")
 *     requires capability("gpu.compute")
 *     requires capability("quantum.measurement")
 *     requires topology(...)
 *
 * This grammar MUST NOT transform these into:
 *
 *     use GPU 0
 *     use CPU 7
 *     use qubit 31
 *     use node 4
 *
 * unless an explicit target-specific, non-portable language construct exists
 * elsewhere and has been deliberately selected by the programmer.
 *
 * No universal resource maximum is defined here.
 */


/*
 * ============================================================================
 * 23. HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware descriptions remain owned by:
 *
 *     grammar/hardware/
 *
 * Compilation orchestration can consume hardware-related semantic intent
 * through:
 *
 *     target intent
 *     capability requirements
 *     resource requirements
 *     portability constraints
 *
 * It MUST NOT reproduce:
 *
 *     CPU topology
 *     GPU topology
 *     FPGA fabric
 *     ASIC layout
 *     QPU topology
 *     physical qubit map
 *     memory bank map
 *     network topology
 *     calibration data
 */


/*
 * ============================================================================
 * 24. QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * Compilation syntax MUST NOT enumerate gates.
 *
 * It MUST NOT introduce:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *
 * as compilation-level constructs.
 *
 * Quantum operations are parsed and represented by the quantum subsystem.
 *
 * Their canonical downstream path remains:
 *
 *     quantum source
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic quantum operation
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
 * This grammar does not create any competing quantum representation.
 */


/*
 * ============================================================================
 * 25. CLASSICAL CONTRACT
 * ============================================================================
 *
 * Classical semantics remain owned by:
 *
 *     grammar/classical/
 *     grammar/expressions/
 *     grammar/types/
 *
 * Compilation orchestration treats classical computation as one domain of the
 * same language rather than a separate language.
 */


/*
 * ============================================================================
 * 26. HDL CONTRACT
 * ============================================================================
 *
 * HDL syntax remains owned by:
 *
 *     grammar/hdl/
 *
 * Compilation orchestration can express:
 *
 *     synthesis intent
 *     representation boundaries
 *     lowering intent
 *     code-generation intent
 *
 * but cannot encode a fixed hardware implementation.
 *
 * Therefore:
 *
 *     wire [31:0]
 *
 * MUST NOT become a universal compilation assumption.
 *
 * Width, timing, resource, and implementation constraints remain semantic
 * properties resolved by the appropriate hardware/HDL systems.
 */


/*
 * ============================================================================
 * 27. OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimization remains owned by optimization.g4 and downstream optimization
 * infrastructure.
 *
 * This file only composes optimization intent.
 *
 * It MUST NOT:
 *
 *     - execute passes;
 *     - define rewrite algorithms;
 *     - define cost models;
 *     - define backend-specific optimization;
 *     - mutate IR;
 *     - schedule operations.
 */


/*
 * ============================================================================
 * 28. LOWERING CONTRACT
 * ============================================================================
 *
 * Lowering remains owned by lowering.g4 and downstream lowering infrastructure.
 *
 * This file can compose lowering requests but MUST NOT define:
 *
 *     machine instructions;
 *     ISA encodings;
 *     physical registers;
 *     physical memory;
 *     physical qubits;
 *     backend-specific instruction selection.
 */


/*
 * ============================================================================
 * 29. CODE-GENERATION CONTRACT
 * ============================================================================
 *
 * Code-generation intent remains owned by code-generation.g4.
 *
 * A requested artifact is not automatically:
 *
 *     executable;
 *     machine code;
 *     binary;
 *     device image.
 *
 * The compiler determines whether the requested representation is available
 * and whether it can be produced from the canonical semantic representation.
 */


/*
 * ============================================================================
 * 30. COMPILE-ONCE CONTRACT
 * ============================================================================
 *
 * Compilation boundaries MAY represent reusable semantic compilation results.
 *
 * The conceptual pipeline is:
 *
 *     source
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     canonical IR
 *       |
 *       v
 *     portable compilation artifact
 *       |
 *       +--> CPU realization
 *       +--> GPU realization
 *       +--> FPGA realization
 *       +--> QPU realization
 *       +--> simulator realization
 *       +--> distributed realization
 *       +--> future realization
 *
 * The portable artifact remains a compiler product, not a new source
 * language.
 */


/*
 * ============================================================================
 * 31. TARGET REALIZATION CONTRACT
 * ============================================================================
 *
 * Target realization occurs after semantic analysis.
 *
 * A target can be:
 *
 *     capability-compatible;
 *     resource-compatible;
 *     constraint-compatible;
 *     deployment-compatible;
 *     execution-compatible.
 *
 * The grammar does not decide which target wins.
 *
 * It also does not predict whether a target will have enough resources.
 *
 * That is a compilation-context and resource-analysis responsibility.
 */


/*
 * ============================================================================
 * 32. DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical source text and identical lexer/parser configuration:
 *
 *     parsing(source) = deterministic AST structure
 *
 * The grammar contains no:
 *
 *     time-based behavior;
 *     random behavior;
 *     filesystem inspection;
 *     network inspection;
 *     environment inspection;
 *     hardware probing.
 *
 * Semantic compilation may depend on an explicitly supplied compilation
 * context, but that context must remain outside parser execution.
 */


/*
 * ============================================================================
 * 33. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     embedded Rust;
 *     semantic actions;
 *     arbitrary code execution;
 *     filesystem operations;
 *     network operations;
 *     environment-variable access;
 *     process spawning;
 *     device discovery;
 *     host callbacks.
 *
 * Compilation-time external information must enter through an explicit,
 * policy-controlled compiler context.
 *
 * The grammar itself cannot grant authority to access host resources.
 */


/*
 * ============================================================================
 * 34. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics MUST distinguish:
 *
 *     malformed compilation syntax
 *
 * from:
 *
 *     unknown compilation profile
 *     unknown compilation stage
 *     unknown artifact
 *     unknown target
 *     unsupported capability
 *     insufficient resource
 *     invalid optimization policy
 *     invalid lowering request
 *     incompatible code-generation request
 *
 * Parser errors belong to syntax.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Resource errors belong to resource analysis.
 *
 * Target-feasibility errors belong to target/resource resolution.
 *
 * Runtime failures belong to runtime execution.
 */


/*
 * ============================================================================
 * 35. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This new file MUST NOT invalidate existing constructs merely by existing.
 *
 * Existing:
 *
 *     compileDeclaration
 *
 * remains valid through Compile.
 *
 * Existing target, optimization, lowering, feature-selection, conditional
 * compilation, and code-generation constructs retain their existing grammar
 * ownership.
 *
 * Future extensions should preferably be representable by:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     property
 *     symbolic reference
 *
 * rather than requiring a new reserved keyword for every future capability.
 *
 * This is important for long-term POCO-REAF compatibility.
 */


/*
 * ============================================================================
 * 36. VERSIONING CONTRACT
 * ============================================================================
 *
 * Changes to this grammar must be classified as:
 *
 *     additive
 *     compatible
 *     source-compatible
 *     AST-compatible
 *     semantic-compatible
 *     breaking
 *     deprecated
 *
 * Adding a new symbolic artifact, profile, policy, capability, target class,
 * optimization profile, backend, or hardware implementation MUST NOT require
 * a grammar change when existing open-ended syntax can represent it.
 *
 * Grammar changes are required when the structural language syntax itself
 * changes.
 */


/*
 * ============================================================================
 * 37. NO VENDOR LOCK-IN
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     vendor CPU names;
 *     vendor GPU names;
 *     vendor FPGA names;
 *     vendor QPU names;
 *     cloud provider names;
 *     device model names;
 *     vendor instruction names;
 *     vendor gate sets.
 *
 * Such concepts may exist in external target descriptions or dialects.
 *
 * Vendor-specific syntax must be explicitly isolated through:
 *
 *     grammar/dialects/
 *     grammar/interoperability/
 *
 * and must not silently become universal Zamani syntax.
 */


/*
 * ============================================================================
 * 38. NO ARTIFICIAL COMPILATION LIMITS
 * ============================================================================
 *
 * This grammar imposes no finite maximum on:
 *
 *     compilation elements
 *     plan entries
 *     phases
 *     profiles
 *     policies
 *     artifacts
 *     boundaries
 *     references
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     features
 *     targets
 *     optimization requests
 *     lowering requests
 *     code-generation requests
 *
 * Repetition is intentionally open-ended.
 */


/*
 * ============================================================================
 * 39. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every AST node derived from this grammar MUST preserve the source span of
 * the syntactic construct that created it.
 *
 * At minimum:
 *
 *     start position
 *     end position
 *     source/file identity
 *
 * Where the frontend supports it, diagnostics should additionally preserve:
 *
 *     related spans
 *     expansion provenance
 *     profile provenance
 *     policy provenance
 *     imported compilation context
 *
 * This is necessary for deterministic diagnostics and large-program tooling.
 */


/*
 * ============================================================================
 * 40. FRONTEND AST INTEGRATION
 * ============================================================================
 *
 * The parser must lower this grammar into the existing domain-neutral
 * frontend AST architecture.
 *
 * It MUST NOT create:
 *
 *     HardwareCompilationNode
 *     QuantumCompilationNode
 *     GPUCompilationNode
 *     FPGACompilationNode
 *
 * merely because a compilation specification references those domains.
 *
 * The AST should represent compilation intent generically and preserve domain
 * information as semantic references.
 */


/*
 * ============================================================================
 * 41. SEMANTIC RESOLUTION ORDER
 * ============================================================================
 *
 * A conforming compiler SHOULD resolve compilation intent in the following
 * conceptual order:
 *
 *     syntax
 *       |
 *       v
 *     names
 *       |
 *       v
 *     profiles/policies
 *       |
 *       v
 *     feature semantics
 *       |
 *       v
 *     target semantics
 *       |
 *       v
 *     resource/capability semantics
 *       |
 *       v
 *     portability validation
 *       |
 *       v
 *     optimization/lowering/generation planning
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     canonical IR
 *
 * This is a semantic dependency order, not a requirement that every compiler
 * implementation use these exact internal functions.
 */


/*
 * ============================================================================
 * 42. RESOURCE FEASIBILITY
 * ============================================================================
 *
 * Resource feasibility MUST remain separate from syntax.
 *
 * For example:
 *
 *     requires qubits >= n
 *
 * means:
 *
 *     semantic requirement
 *
 * It does NOT mean:
 *
 *     allocate n physical qubits now.
 *
 * Similarly:
 *
 *     requires memory >= required_memory
 *
 * does not define:
 *
 *     a universal memory size.
 *
 * The compiler may:
 *
 *     - select a larger target;
 *     - distribute computation;
 *     - use a simulator;
 *     - use an accelerator;
 *     - use logical resources;
 *     - defer execution;
 *     - reject an infeasible target.
 *
 * It MUST NOT silently alter program semantics merely to fit a smaller target.
 */


/*
 * ============================================================================
 * 43. CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * One compilation specification may coordinate:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     embedded
 *     accelerator
 *     future domains
 *
 * without creating separate languages.
 *
 * The domain-specific grammar remains responsible for domain syntax.
 *
 * compilation.g4 is responsible only for composition of compilation intent.
 */


/*
 * ============================================================================
 * 44. HYBRID COMPUTATION CONTRACT
 * ============================================================================
 *
 * A compilation plan may contain classical, quantum, HDL, AI, distributed, or
 * other domain artifacts.
 *
 * It MUST NOT assume that one domain is the "main" language.
 *
 * For example:
 *
 *     classical semantics
 *          |
 *          +--> quantum::ir
 *          |
 *          +--> hardware intent
 *          |
 *          +--> distributed realization
 *
 * remains a single semantic program.
 */


/*
 * ============================================================================
 * 45. DISTRIBUTED-SCALING CONTRACT
 * ============================================================================
 *
 * Compilation orchestration MUST NOT encode:
 *
 *     node 0
 *     node 1
 *     ...
 *     node N
 *
 * as a universal machine model.
 *
 * Distributed resource requirements are semantic information consumed by the
 * distributed/resource/target systems.
 *
 * The same compilation specification can therefore be realized on:
 *
 *     one device;
 *     multiple devices;
 *     a cluster;
 *     a supercomputer;
 *     a cloud deployment;
 *     a future distributed architecture.
 */


/*
 * ============================================================================
 * 46. HARDWARE-SOFTWARE CO-DESIGN CONTRACT
 * ============================================================================
 *
 * Compilation may coordinate:
 *
 *     software artifact
 *     hardware artifact
 *     HDL artifact
 *     accelerator artifact
 *     verification artifact
 *
 * but it MUST NOT collapse hardware intent into a fixed physical design.
 *
 * The hardware/HDL subsystems remain responsible for:
 *
 *     synthesis;
 *     physical realization;
 *     timing realization;
 *     placement;
 *     routing;
 *     implementation.
 */


/*
 * ============================================================================
 * 47. INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * Compilation artifacts may refer to interoperable representations such as:
 *
 *     classical IR
 *     quantum::ir
 *     QIR
 *     OpenQASM
 *     HDL representations
 *     object representations
 *     executable representations
 *
 * but interoperability formats MUST NOT silently become Zamani's semantic
 * authority.
 *
 * Zamani semantics remain canonical.
 */


/*
 * ============================================================================
 * 48. NO SECOND QUANTUM IR
 * ============================================================================
 *
 * This file explicitly prohibits creation of:
 *
 *     compilation quantum IR
 *     compilation circuit IR
 *     compilation gate IR
 *     compilation qubit IR
 *
 * Quantum lowering MUST continue through:
 *
 *     quantum::ir
 *
 * as established by the repository architecture.
 */


/*
 * ============================================================================
 * 49. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Grammar-level scalability is achieved through:
 *
 *     repetition;
 *     symbolic references;
 *     compositional structures;
 *     semantic resolution;
 *
 * rather than finite enumerations.
 *
 * The parser implementation should avoid unnecessary recursive processing of
 * arbitrarily large flat lists where an iterative representation is possible.
 *
 * Compiler implementations should use appropriate:
 *
 *     worklists;
 *     iterative traversals;
 *     streaming;
 *     incremental processing;
 *     bounded resource policies;
 *
 * when processing very large compilation specifications.
 *
 * These implementation techniques MUST NOT change the language semantics.
 */


/*
 * ============================================================================
 * 50. CONFORMANCE TEST CONTRACT
 * ============================================================================
 *
 * The following categories are REQUIRED for repository-level completion:
 *
 *     positive syntax
 *     negative syntax
 *     boundary syntax
 *     scalability
 *     determinism
 *     AST mapping
 *     semantic validation
 *     resource validation
 *     capability validation
 *     portability
 *     compatibility
 *     cross-domain integration
 *     diagnostic quality
 *     source-span preservation
 *
 * Tests MUST include:
 *
 *     one compilation element;
 *     many compilation elements;
 *     one plan stage;
 *     many plan stages;
 *     nested profiles;
 *     nested policies;
 *     many artifacts;
 *     many requirements;
 *     many constraints;
 *     many targets;
 *     many optimization requests;
 *     many lowering requests;
 *     many code-generation requests;
 *     large symbolic identifiers;
 *     qualified identifiers;
 *     empty/invalid structures;
 *     deeply nested valid structures where supported;
 *     large flat lists.
 *
 * No scalability test may establish an artificial language maximum.
 */


/*
 * ============================================================================
 * 51. NEGATIVE CONFORMANCE
 * ============================================================================
 *
 * The following structural forms MUST be rejected:
 *
 *     compile plan { }
 *
 *     compile profile foo { }
 *
 *     compile policy foo { }
 *
 *     compile pipeline foo { }
 *
 *     compile artifacts { }
 *
 *     compile boundary foo { }
 *
 * where the construct requires one or more entries.
 *
 * Malformed references MUST also be rejected:
 *
 *     use compilation;
 *
 *     use compilation.;
 *
 *     compile plan { stage ; }
 *
 * Semantic invalidity must remain distinct from syntax invalidity.
 */


/*
 * ============================================================================
 * 52. SEMANTICALLY INVALID BUT STRUCTURALLY VALID EXAMPLES
 * ============================================================================
 *
 * These may parse but MUST be rejected downstream when their semantics are
 * invalid:
 *
 *     use compilation unknown_profile;
 *
 *     compile profile nonexistent { ... }
 *
 *     compile plan {
 *         stage nonexistent;
 *     }
 *
 *     compile requires capability("unknown.capability");
 *
 *     compile requires qubits >= impossible_requirement;
 *
 *     compile target unsupported_target;
 *
 *     compile optimization unsupported_policy;
 *
 * The parser MUST NOT attempt to discover whether these resources,
 * capabilities, targets, optimizations, or backends exist.
 */


/*
 * ============================================================================
 * 53. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains no universal machine capacities.
 *
 * A repository-level hard-coding audit MUST reject future additions that
 * introduce fixed universal limits such as:
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
 * It must also detect disguised finite enumerations of physical resources.
 *
 * Allowed:
 *
 *     let n = 1024;
 *     requires qubits >= n;
 *
 * Not allowed as a language ceiling:
 *
 *     compiler supports at most 1024 qubits
 *
 * The distinction is:
 *
 *     program semantics = allowed
 *     implementation ceiling = prohibited
 */


/*
 * ============================================================================
 * 54. RUST 1.97 / 1.97.1 INTEGRATION
 * ============================================================================
 *
 * This grammar is ANTLR syntax only.
 *
 * It contains no Rust actions.
 *
 * The generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Compiler implementation MUST use safe Rust.
 *
 * No grammar construct requires:
 *
 *     unsafe
 *     unsafe fn
 *     unsafe impl
 *     unsafe trait
 *     unsafe block
 *
 * The grammar's source-language token vocabulary may contain an `unsafe`
 * keyword if that keyword remains part of the Zamani language; that is
 * independent from the prohibition on Rust `unsafe` implementation code.
 */


/*
 * ============================================================================
 * 55. COMPLETION CRITERIA
 * ============================================================================
 *
 * compilation.g4 is complete when:
 *
 * [x] It has one canonical parser grammar declaration.
 *
 * [x] It uses the canonical ZamaniLexer vocabulary.
 *
 * [x] It does not define a second expression grammar.
 *
 * [x] It does not define a second identifier grammar.
 *
 * [x] It does not define a second type grammar.
 *
 * [x] It does not define a second target grammar.
 *
 * [x] It does not define a second optimization grammar.
 *
 * [x] It does not define a second lowering grammar.
 *
 * [x] It does not define a second code-generation grammar.
 *
 * [x] It does not define quantum gates.
 *
 * [x] It does not define physical qubits.
 *
 * [x] It does not define hardware topology.
 *
 * [x] It does not define machine instructions.
 *
 * [x] It does not define a second IR.
 *
 * [x] It does not define resource ceilings.
 *
 * [x] It does not contain embedded Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It composes existing compilation grammar authorities.
 *
 * [x] It preserves requirement/constraint/preference/hint distinctions.
 *
 * [x] It supports open-ended compilation composition.
 *
 * [x] It supports reusable profiles.
 *
 * [x] It supports compilation policies.
 *
 * [x] It supports compilation pipelines.
 *
 * [x] It supports artifact relationships.
 *
 * [x] It supports explicit semantic boundaries.
 *
 * [x] It preserves POCO-REAF.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * [x] It keeps target realization downstream.
 *
 * [x] It keeps resource feasibility downstream.
 *
 * [x] It keeps optimization downstream.
 *
 * [x] It keeps routing downstream.
 *
 * [x] It keeps scheduling downstream.
 *
 * [x] It keeps QEC downstream.
 *
 * [x] It keeps ZQN downstream.
 *
 * [x] It keeps HAL downstream.
 *
 * Repository integration still requires:
 *
 * [ ] The canonical Zamani parser composition imports Compilation exactly once.
 *
 * [ ] No other grammar defines a competing `compilation` entry point.
 *
 * [ ] Token names used by this file are present in the canonical lexer.
 *
 * [ ] The imported grammar rule names match the current canonical grammar
 *     declarations.
 *
 * [ ] The frontend AST has the corresponding domain-neutral compilation nodes.
 *
 * [ ] Semantic analysis resolves compilation profiles, policies, stages,
 *     artifacts, targets, capabilities and resources.
 *
 * [ ] Compilation intent lowers into the existing canonical semantic model.
 *
 * [ ] No compilation-specific IR is introduced.
 *
 * [ ] Quantum paths reach `quantum::ir`.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Rust 1.97 / 1.97.1 builds pass.
 *
 * [ ] Repository-wide `unsafe` audit passes for compiler implementation code.
 *
 * ============================================================================
 * 56. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * BEFORE MERGING THIS FILE:
 *
 * 1. Verify `Compile` grammar exposes:
 *
 *        compileDeclaration
 *        compileStageSpecification
 *        compileRequirementValue
 *        compileConstraintValue
 *        compilePreferenceValue
 *        compileHintValue
 *        compileFeatureValue
 *        compileArtifactSpecification
 *
 * 2. Verify `CompileTarget` exposes:
 *
 *        targetDeclaration
 *
 *    or update ONLY this integration boundary to the repository's actual
 *    canonical target entry rule.
 *
 * 3. Verify `CompileOptimization` exposes:
 *
 *        optimizationDeclaration
 *
 *    or update ONLY this integration boundary to its canonical rule.
 *
 * 4. Verify `CompileCodeGeneration` exposes:
 *
 *        codeGenerationDeclaration
 *
 * 5. Verify `CompileLowering` exposes:
 *
 *        loweringDeclaration
 *
 * 6. Verify `ConditionalCompilation` exposes:
 *
 *        conditionalCompilationDirective
 *
 * 7. Verify `FeatureSelection` exposes:
 *
 *        featureSelectionDirective
 *
 * 8. Verify lexer tokens:
 *
 *        PLAN
 *        PROFILE
 *        POLICY
 *        PIPELINE
 *        ARTIFACTS
 *        BOUNDARY
 *        INPUT
 *        OUTPUT
 *        USE
 *        COMPILATION
 *        DERIVES
 *        PRODUCES
 *        CONSUMES
 *        PRESERVES
 *        TRANSFORMS
 *
 *    If any of these are not present in the canonical lexer, the language
 *    keyword must NOT be silently invented here.
 *
 *    Instead, either:
 *
 *        a) use an existing canonical token;
 *        b) add the token through the lexer authority;
 *        c) use an existing identifier-based extension point.
 *
 * 9. Verify there is no import cycle:
 *
 *        Compilation -> Compile
 *        Compile -> Compilation
 *
 *    is prohibited.
 *
 * 10. Verify root composition imports Compilation once.
 *
 * 11. Verify no existing grammar claims ownership of the same public
 *     `compilation` rule.
 *
 * 12. Verify frontend AST mapping before enabling the grammar as stable.
 *
 * 13. Verify semantic integration before marking implementation status
 *     `IMPLEMENTED`.
 *
 * 14. Verify canonical IR integration before marking a feature production
 *     ready.
 *
 * 15. Verify scalability tests use generated/parameterized input rather than
 *     finite hand-written maxima.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */