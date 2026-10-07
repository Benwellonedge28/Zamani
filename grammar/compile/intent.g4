/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/compile/intent.g4
 *
 * GRAMMAR
 * -------
 * CompileIntent
 *
 * STATUS
 * ------
 * Production compile-intent orchestration boundary
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 or later
 * Rust 2021 edition
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the repository-wide COMPILE INTENT BOUNDARY.
 *
 * It does not implement compilation.
 *
 * It does not select hardware.
 *
 * It does not execute metaprograms.
 *
 * It does not perform optimization.
 *
 * It does not lower IR.
 *
 * It does not perform routing.
 *
 * It does not perform scheduling.
 *
 * It does not perform resource negotiation.
 *
 * It does not authorize capabilities.
 *
 * It does not create AST nodes directly.
 *
 * Its responsibility is to ORCHESTRATE the already-owned compilation and
 * metaprogramming constructs into one stable compile-intent boundary.
 *
 * The intended architecture is:
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
 *       v
 *     semantic model
 *       |
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       +--> compile intent
 *                 |
 *                 +--> compile-time
 *                 +--> feature selection
 *                 +--> target intent
 *                 +--> optimization intent
 *                 +--> specialization intent
 *                 +--> code-generation intent
 *                 +--> lowering intent
 *                 +--> reproducibility intent
 *                 +--> deterministic-build intent
 *                 +--> artifact intent
 *                 +--> caching intent
 *                 +--> deployment intent
 *                 +--> cross-compilation intent
 *                 +--> conditional compilation
 *                 |
 *                 +--> metaprogramming
 *                       |
 *                       +--> compile-time execution
 *                       +--> reflection
 *                       +--> introspection
 *                       +--> quotation
 *                       +--> unquotation
 *                       +--> syntax trees
 *                       +--> source generation
 *                       +--> code generation
 *                       +--> specialization
 *                       +--> type-level computation
 *                       +--> schemas
 *                       +--> metaprogramming capabilities
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical IR
 *       |
 *       +--> quantum::ir
 *       |
 *       +--> HDL / hardware representation
 *       |
 *       +--> other domain representations
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
 *     resilience / recovery / QEC where applicable
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
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compile intent is one of the mechanisms that makes:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * possible.
 *
 * A source program describes semantic intent rather than a fixed machine.
 *
 * Compilation may therefore consume:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     hints
 *     policies
 *     portability requirements
 *     reproducibility requirements
 *     specialization requirements
 *     optimization objectives
 *
 * without encoding a fixed machine capacity into the language.
 *
 * This grammar MUST NOT encode universal limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     memory
 *     storage
 *     register widths
 *     vector widths
 *     tensor ranks
 *     network sizes
 *     topology sizes
 *     generated source size
 *     metaprogram expansion size
 *     specialization count
 *     quotation depth
 *     type-level complexity
 *
 * Operational limits may exist in compiler/runtime configuration.
 *
 * Such limits are implementation policy and MUST NOT become language-level
 * semantic ceilings.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     compile-intent composition
 *     stable compile-intent dispatch
 *     compile-intent integration boundaries
 *     compile-intent classification
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical tokens
 *     identifiers
 *     ordinary expressions
 *     ordinary statements
 *     ordinary declarations
 *     ordinary types
 *     target syntax
 *     optimization syntax
 *     lowering syntax
 *     specialization syntax
 *     compile-time syntax
 *     metaprogramming syntax
 *     reflection syntax
 *     quotation syntax
 *     source-generation syntax
 *     type-level syntax
 *     schema syntax
 *     capability authorization
 *     resource resolution
 *     policy evaluation
 *     AST construction
 *     semantic analysis
 *     IR construction
 *     backend generation
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Every construct appearing in this grammar MUST have exactly one detailed
 * syntax owner.
 *
 * This grammar only references those owners.
 *
 * In particular:
 *
 *     compile-time syntax
 *         -> grammar/compile/compile-time.g4
 *
 *     target syntax
 *         -> grammar/compile/target.g4
 *
 *     target selection
 *         -> grammar/compile/target-selection.g4
 *
 *     optimization
 *         -> grammar/compile/optimization.g4
 *
 *     specialization
 *         -> grammar/compile/specialization.g4
 *
 *     lowering
 *         -> grammar/compile/lowering.g4
 *
 *     code generation
 *         -> grammar/compile/code-generation.g4
 *
 *     feature selection
 *         -> grammar/compile/feature-selection.g4
 *
 *     reproducibility
 *         -> grammar/compile/reproducibility.g4
 *
 *     deterministic builds
 *         -> grammar/compile/deterministic-builds.g4
 *
 *     artifacts
 *         -> grammar/compile/artifacts.g4
 *
 *     caching
 *         -> grammar/compile/caching.g4
 *
 *     deployment
 *         -> grammar/compile/deployment.g4
 *
 *     cross compilation
 *         -> grammar/compile/cross-compilation.g4
 *
 *     conditional compilation
 *         -> grammar/compile/conditional-compilation.g4
 *
 *     compile profiles
 *         -> grammar/compile/profiles.g4
 *
 *     provenance
 *         -> grammar/compile/provenance.g4
 *
 *     metaprogramming
 *         -> grammar/metaprogramming/metaprogramming.g4
 *
 * No construct in this file should duplicate those definitions.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/compile/compile.g4
 *     grammar/compile/compilation.g4
 *     grammar/compile/compile-time.g4
 *     grammar/compile/conditional-compilation.g4
 *     grammar/compile/feature-selection.g4
 *     grammar/compile/cross-compilation.g4
 *     grammar/compile/target.g4
 *     grammar/compile/target-selection.g4
 *     grammar/compile/optimization.g4
 *     grammar/compile/specialization.g4
 *     grammar/compile/code-generation.g4
 *     grammar/compile/lowering.g4
 *     grammar/compile/reproducibility.g4
 *     grammar/compile/deterministic-builds.g4
 *     grammar/compile/provenance.g4
 *     grammar/compile/caching.g4
 *     grammar/compile/artifacts.g4
 *     grammar/compile/deployment.g4
 *     grammar/compile/profiles.g4
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * CONSUMES:
 *
 *     existing parser rules exported by those grammars.
 *
 * EXPORTS:
 *
 *     compileIntent
 *     compileIntentElement
 *     compileIntentDeclaration
 *     compileIntentSpecification
 *     compileIntentMetaprogramming
 *     compileIntentPipeline
 *
 * AST_OWNER:
 *
 *     repository AST implementation.
 *
 * SEMANTIC_OWNER:
 *
 *     compiler semantic-analysis pipeline.
 *
 * IR_OWNER:
 *
 *     canonical IR infrastructure.
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * TEST_OWNER:
 *
 *     grammar/tests/
 *     repository compiler tests.
 *
 * ============================================================================
 * IMPORT ARCHITECTURE
 * ============================================================================
 *
 * The import direction is intentionally one-way.
 *
 *     compile intent
 *          |
 *          +--> compilation composition
 *          |
 *          +--> compile facilities
 *          |
 *          +--> metaprogramming composition
 *
 * The metaprogramming composition root already owns the complete
 * grammar/metaprogramming/ leaf set.
 *
 * Therefore this file MUST NOT import every metaprogramming leaf independently.
 *
 * This is deliberate.
 *
 * Instead:
 *
 *     CompileIntent
 *          |
 *          v
 *     Metaprogramming
 *          |
 *          +--> capabilities.g4
 *          +--> code-generation.g4
 *          +--> compile-time-execution.g4
 *          +--> compile-time.g4
 *          +--> generation.g4
 *          +--> introspection.g4
 *          +--> quotation.g4
 *          +--> reflection.g4
 *          +--> schemas.g4
 *          +--> specialization.g4
 *          +--> syntax-tree.g4
 *          +--> type-level.g4
 *          +--> unquotation.g4
 *
 * This gives `intent.g4` a stable dependency on ONE metaprogramming boundary
 * rather than on every leaf file.
 *
 * ============================================================================
 */

parser grammar CompileIntent;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Existing compile grammars remain the syntax authorities.
 *
 * The metaprogramming subsystem remains independently composed.
 * ============================================================================
 */

import
    Compile,
    Compilation,
    CompileTime,
    ConditionalCompilation,
    FeatureSelection,
    CompileCrossCompilation,
    CompileTarget,
    CompileTargetSelection,
    CompileOptimization,
    CompileSpecialization,
    CompileCodeGeneration,
    CompileLowering,
    CompileReproducibility,
    CompileProvenance,
    CompileCaching,
    CompileArtifacts,
    CompileDeployment,
    Metaprogramming
;


/*
 * ============================================================================
 * 1. PRIMARY COMPILE INTENT
 * ============================================================================
 *
 * This is the stable public boundary for compilation intent.
 *
 * It does not impose an ordering on independent clauses unless an owning
 * grammar already defines that ordering.
 *
 * The semantic compiler is responsible for constructing the final normalized
 * compilation plan.
 */

compileIntent
    : compileIntentDeclaration
    | compileIntentSpecification
    | compileIntentPipeline
    | compileIntentMetaprogramming
    ;


/*
 * ============================================================================
 * 2. COMPILE INTENT DECLARATION
 * ============================================================================
 *
 * A declaration-level compile construct remains owned by its detailed grammar.
 */

compileIntentDeclaration
    : compileDeclaration
    | compileTimeDeclaration
    | compileSpecializationDeclaration
    | codeGenerationDeclaration
    | loweringDeclaration
    | targetDeclaration
    | targetSelectionDeclaration
    | optimizationDeclaration
    | compileDeploymentDeclaration
    | compileArtifactSpecification
    | compilationSpecification
    | metaSchemaDeclaration
    | typeLevelDeclaration
    ;


/*
 * ============================================================================
 * 3. COMPILE INTENT SPECIFICATION
 * ============================================================================
 *
 * This boundary exposes compilation specifications without redefining their
 * syntax.
 */

compileIntentSpecification
    : compileSpecification
    | compilationSpecification
    | compileTimeControl
    | featureSelection
    | targetSelectionSpecification
    | optimizationSpecification
    | reproducibilitySpecification
    | deterministicBuildSpecification
    | crossCompilationSpecification
    | compileSpecialization
    | codeGenerationDirective
    | loweringRequest
    | compileArtifactSpecification
    | compileDeploymentReference
    | compileProvenanceUnit
    ;


/*
 * ============================================================================
 * 4. COMPILE INTENT PIPELINE
 * ============================================================================
 *
 * The compilation composition grammar owns the richer pipeline model.
 *
 * This wrapper deliberately does not invent another pipeline grammar.
 */

compileIntentPipeline
    : compilationPlan
    | compilationPipeline
    | loweringPipelineDeclaration
    | optimizationPipelineClause
    | codeGenerationDeclaration
    ;


/*
 * ============================================================================
 * 5. METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * All metaprogramming features are orchestrated through the existing
 * grammar/metaprogramming/metaprogramming.g4 composition root.
 *
 * This means this file automatically integrates:
 *
 *     compile-time execution
 *     compile-time constructs
 *     reflection
 *     introspection
 *     quotation
 *     unquotation
 *     syntax-tree manipulation
 *     source generation
 *     code generation
 *     specialization
 *     type-level computation
 *     schemas
 *     metaprogramming capabilities
 *
 * without duplicating their syntax here.
 *
 * IMPORTANT:
 *
 * Metaprogramming remains domain-neutral.
 *
 * A metaprogram may ultimately generate:
 *
 *     classical constructs
 *     quantum constructs
 *     hybrid constructs
 *     HDL constructs
 *     hardware-intent constructs
 *     AI/data constructs
 *     distributed constructs
 *     networking constructs
 *     security constructs
 *     future domain constructs
 *
 * Those generated constructs MUST re-enter the canonical frontend pipeline.
 */

compileIntentMetaprogramming
    : metaprogrammingDeclaration
    | metaprogrammingStatement
    | metaprogrammingFacilityExpression
    | metaprogrammingCapability
    | metaprogrammingSyntaxTree
    ;


/*
 * ============================================================================
 * 6. COMPILE-TIME INTENT
 * ============================================================================
 *
 * Compile-time constructs are distinct from runtime constructs.
 *
 * Parsing them never executes them.
 */

compileIntentCompileTime
    : compileTimeControl
    | compileTimeConditional
    | compileTimeSelection
    | compileTimeRequirement
    | compileTimeAssertion
    | compileTimeSpecialization
    | compileTimeFeatureSelection
    | compileTimeConfigurationSelection
    | compileTimeInclude
    | compileTimeGeneratedRegion
    | compileTimeFor
    | compileTimeMatch
    | compileTimeBlock
    | compileTimeDeclaration
    | compileTimeExpressionBridge
    ;


/*
 * ============================================================================
 * 7. TARGET INTENT
 * ============================================================================
 *
 * Target intent describes semantic target requirements/preferences.
 *
 * It MUST NOT become physical hardware discovery.
 *
 * Examples of valid semantic intent include:
 *
 *     target capability requirements
 *     portability constraints
 *     target profiles
 *     target alternatives
 *     resource requirements
 *     capability requirements
 *
 * Physical realization belongs downstream.
 */

compileIntentTarget
    : targetDeclaration
    | targetSelectionDeclaration
    | targetSelectionSpecification
    | targetSelectionExpression
    | targetPortability
    | targetScalability
    | targetResourceIntent
    | targetCapability
    | targetRequirement
    | targetConstraint
    | targetPreference
    | targetHint
    | targetAlternative
    ;


/*
 * ============================================================================
 * 8. FEATURE INTENT
 * ============================================================================
 *
 * Feature selection is semantic selection, not hardware enumeration.
 */

compileIntentFeature
    : featureSelection
    | featureSelectionExpression
    | featureSelectionPredicate
    | featureSelectionAlternative
    | featureSelectionDirective
    ;


/*
 * ============================================================================
 * 9. OPTIMIZATION INTENT
 * ============================================================================
 *
 * Optimization describes intent and constraints.
 *
 * It does not commit the source program to a particular optimization
 * implementation.
 */

compileIntentOptimization
    : optimizationDeclaration
    | optimizationSpecification
    | optimizationClause
    | optimizationDirective
    | optimizationPipelineClause
    | optimizationPassClause
    | optimizationObjectiveClause
    | optimizationRequirementClause
    | optimizationConstraintClause
    | optimizationPreferenceClause
    | optimizationHintClause
    | optimizationBudgetClause
    | optimizationTerminationClause
    ;


/*
 * ============================================================================
 * 10. SPECIALIZATION INTENT
 * ============================================================================
 *
 * Specialization is a semantic compilation request.
 *
 * It must remain separate from target selection.
 */

compileIntentSpecialization
    : compileSpecializationDeclaration
    | compileSpecialization
    | compileSpecializationExpression
    | specializationRequest
    | specializationExpression
    ;


/*
 * ============================================================================
 * 11. LOWERING INTENT
 * ============================================================================
 *
 * Lowering describes representation transitions.
 *
 * It does not create backend instructions.
 */

compileIntentLowering
    : loweringDeclaration
    | loweringRequest
    | loweringStageDeclaration
    | loweringPipelineDeclaration
    | loweringPolicyDeclaration
    ;


/*
 * ============================================================================
 * 12. CODE-GENERATION INTENT
 * ============================================================================
 *
 * Code-generation intent remains separate from metaprogram source generation.
 *
 * Source-generation constructs are owned by the metaprogramming subsystem.
 *
 * Backend artifact generation remains a compiler concern.
 */

compileIntentCodeGeneration
    : codeGenerationDeclaration
    | codeGenerationDirective
    | codeGenerationArtifact
    | codeGenerationOutput
    | codeGenerationEntryPoint
    | codeGenerationSymbolPolicy
    | codeGenerationLinkage
    | codeGenerationInterface
    | codeGenerationDebug
    | codeGenerationProvenance
    | codeGenerationLayout
    | codeGenerationEmission
    ;


/*
 * ============================================================================
 * 13. CROSS-COMPILATION INTENT
 * ============================================================================
 *
 * Cross compilation expresses source-to-realization relationships without
 * embedding a finite target universe into the grammar.
 */

compileIntentCrossCompilation
    : crossCompilationClause
    | crossCompilationSpecification
    | crossCompilationRoute
    | crossSourceTarget
    | crossDestinationTarget
    | crossCompilationBody
    ;


/*
 * ============================================================================
 * 14. REPRODUCIBILITY AND DETERMINISM
 * ============================================================================
 *
 * These are compilation properties.
 *
 * They do not imply that every runtime operation is deterministic.
 *
 * They describe the requested reproducibility/determinism properties of the
 * compilation process and artifacts.
 */

compileIntentReproducibility
    : reproducibilityClause
    | reproducibilitySpecification
    | reproducibilityRequirement
    | reproducibilityConstraint
    | reproducibilityPreference
    | reproducibilityHint
    | reproducibilityCapability
    | reproducibilityResource
    ;

compileIntentDeterminism
    : deterministicBuildClause
    | deterministicBuildSpecification
    | deterministicRequirement
    | deterministicConstraint
    | deterministicPreference
    | deterministicHint
    | deterministicCapability
    | deterministicResource
    ;


/*
 * ============================================================================
 * 15. ARTIFACT INTENT
 * ============================================================================
 *
 * Artifacts remain compiler products.
 *
 * This grammar only exposes their source-level specification boundary.
 */

compileIntentArtifact
    : compileArtifactSpecification
    | compileArtifactPropertyBlock
    | compileArtifactRequirement
    | compileArtifactConstraint
    | compileArtifactPreference
    | compileArtifactHint
    ;


/*
 * ============================================================================
 * 16. CACHING INTENT
 * ============================================================================
 *
 * Caching must remain semantic/compiler policy rather than a source-level
 * machine-capacity mechanism.
 */

compileIntentCaching
    : cachingDeclaration
    | cachingSpecification
    | cachingPropertyBlock
    ;


/*
 * ============================================================================
 * 17. DEPLOYMENT INTENT
 * ============================================================================
 *
 * Deployment remains downstream of compilation semantics.
 */

compileIntentDeployment
    : compileDeploymentDeclaration
    | compileDeploymentReference
    ;


/*
 * ============================================================================
 * 18. PROVENANCE INTENT
 * ============================================================================
 *
 * Compilation provenance connects source, transformations and artifacts.
 *
 * Provenance does not alter program meaning merely by being recorded.
 */

compileIntentProvenance
    : compileProvenanceUnit
    | compileProvenanceDeclaration
    | compileProvenanceBody
    | compileProvenanceTransformation
    | compileProvenanceArtifact
    | compileProvenanceOutput
    | compileProvenanceCompiler
    | compileProvenanceToolchain
    | compileProvenanceProfile
    | compileProvenancePolicy
    | compileProvenanceTarget
    | compileProvenancePhase
    ;


/*
 * ============================================================================
 * 19. CONDITIONAL COMPILATION
 * ============================================================================
 *
 * Conditional compilation remains semantic selection.
 *
 * It must not become an implicit hardware-detection language.
 *
 * Target/capability predicates are evaluated by downstream compilation
 * context.
 */

compileIntentConditional
    : conditionalCompilation
    | conditionalCompilationElseIf
    | conditionalCompilationElse
    | conditionalCompilationPredicate
    | conditionalCompilationFeaturePredicate
    | conditionalCompilationTargetPredicate
    | conditionalCompilationCapabilityPredicate
    | conditionalCompilationVersionPredicate
    | conditionalCompilationDialectPredicate
    ;


/*
 * ============================================================================
 * 20. COMPILE PROFILE
 * ============================================================================
 *
 * Profiles are named compilation policy/configuration structures.
 *
 * They do not define physical machine limits.
 */

compileIntentProfile
    : compileProfile
    | compileProfileInheritance
    | compileProfileReference
    | compileProfileRequirement
    | compileProfileConstraint
    | compileProfilePreference
    | compileProfileHint
    | compileProfileFeature
    | compileProfileArtifact
    | compileProfileStage
    | compileProfileOptimization
    | compileProfileTarget
    | compileProfileOption
    | compileProfileProperty
    ;


/*
 * ============================================================================
 * 21. UNIFIED COMPILE-INTENT ELEMENT
 * ============================================================================
 *
 * This rule is intentionally broad.
 *
 * It is the stable adapter for consumers that need to inspect compilation
 * intent without knowing which compilation facility produced it.
 *
 * It does not replace the canonical source-element grammar.
 */

compileIntentElement
    : compileIntentDeclaration
    | compileIntentSpecification
    | compileIntentPipeline
    | compileIntentCompileTime
    | compileIntentTarget
    | compileIntentFeature
    | compileIntentOptimization
    | compileIntentSpecialization
    | compileIntentLowering
    | compileIntentCodeGeneration
    | compileIntentCrossCompilation
    | compileIntentReproducibility
    | compileIntentDeterminism
    | compileIntentArtifact
    | compileIntentCaching
    | compileIntentDeployment
    | compileIntentProvenance
    | compileIntentConditional
    | compileIntentProfile
    | compileIntentMetaprogramming
    ;


/*
 * ============================================================================
 * 22. COMPILATION PLAN ADAPTER
 * ============================================================================
 *
 * A compilation plan is an ordered semantic object downstream.
 *
 * This grammar does not impose a fixed execution order on independent
 * declarations.
 *
 * The compiler is responsible for dependency analysis and normalization.
 *
 * The conceptual semantic model is:
 *
 *     intent
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> preferences
 *       +--> policies
 *       +--> features
 *       +--> specialization
 *       +--> optimization
 *       +--> lowering
 *       +--> reproducibility
 *       +--> provenance
 *       +--> artifacts
 *       +--> deployment
 *
 * followed by dependency-resolved compilation.
 */

compileIntentPlan
    : compilationPlan
    | compilationPipeline
    ;


/*
 * ============================================================================
 * 23. METAPROGRAMMING RE-ENTRY CONTRACT
 * ============================================================================
 *
 * Metaprogramming may generate or transform source.
 *
 * Generated source MUST NOT bypass the canonical parser/semantic pipeline.
 *
 * Conceptually:
 *
 *     metaprogram
 *          |
 *          v
 *     generated source / semantic fragment
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical IR
 *
 * This grammar therefore provides only the compile-intent boundary.
 *
 * It does not define a second frontend for generated programs.
 */

compileIntentSourceReentry
    : metaprogrammingSourceReentry
    ;


/*
 * ============================================================================
 * 24. DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * Compile intent is domain-neutral.
 *
 * It may eventually influence compilation of:
 *
 *     classical computation
 *     numerical computation
 *     scientific computation
 *     AI / machine learning
 *     knowledge systems
 *     probabilistic computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     embedded systems
 *     accelerators
 *     concurrency
 *     parallel computing
 *     distributed computing
 *     networking
 *     data systems
 *     security
 *     interoperability
 *     simulation
 *     future computational domains
 *
 * This grammar MUST NOT add a domain-specific compile-intent grammar for
 * every future technology.
 *
 * Domain-specific requirements are represented through the existing semantic
 * capability/resource/policy mechanisms.
 */


/*
 * ============================================================================
 * 25. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Compile intent MUST NOT define quantum operations.
 *
 * It may express compilation requirements or constraints that eventually
 * influence quantum lowering.
 *
 * The canonical quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * Compile intent does not select a physical qubit or encode a finite gate
 * catalogue.
 */


/*
 * ============================================================================
 * 26. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Compile intent may constrain compilation of HDL/hardware intent, but does
 * not define signals, timing, physical placement, device topology or
 * implementation-specific widths.
 *
 * Hardware realization remains downstream.
 */


/*
 * ============================================================================
 * 27. RESOURCE AND CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Compile intent may consume constructs whose semantic meaning includes:
 *
 *     requirement
 *     capability
 *     resource
 *     constraint
 *     preference
 *     hint
 *
 * Example semantic intent:
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("quantum.measurement");
 *
 *     requires memory >= required_memory;
 *
 *     requires topology(required_topology);
 *
 * These are source-level requirements.
 *
 * This grammar does not determine whether the requirements are satisfiable.
 *
 * Satisfaction belongs downstream to:
 *
 *     semantic analysis
 *         ->
 *     resource analysis
 *         ->
 *     capability negotiation
 *         ->
 *     compilation planning
 *         ->
 *     target realization
 */


/*
 * ============================================================================
 * 28. EFFECT INTEGRATION
 * ============================================================================
 *
 * Compile-time and metaprogramming constructs may carry effects.
 *
 * Relevant semantic effects can include:
 *
 *     IO
 *     network
 *     filesystem
 *     native
 *     foreign
 *     reflection
 *     code generation
 *     compile-time execution
 *     randomness
 *     simulation
 *     resource access
 *     hardware interaction
 *
 * The grammar does not authorize those effects.
 *
 * Effect checking is downstream.
 */


/*
 * ============================================================================
 * 29. CONTRACT INTEGRATION
 * ============================================================================
 *
 * Compilation intent may be constrained by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * These constructs remain owned by the validation/contract subsystem.
 *
 * This file does not redefine them.
 */


/*
 * ============================================================================
 * 30. POLICY INTEGRATION
 * ============================================================================
 *
 * Compilation may be governed by policies concerning:
 *
 *     security
 *     resource selection
 *     target selection
 *     reproducibility
 *     adaptation
 *     code generation
 *     deployment
 *     metaprogramming
 *     reflection
 *     native/foreign operations
 *
 * Policy evaluation is downstream.
 *
 * Parsing a policy-bearing construct never grants permission.
 */


/*
 * ============================================================================
 * 31. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Compile intent participates in provenance.
 *
 * A semantic compilation record may preserve relationships such as:
 *
 *     source
 *     derived_from
 *     transformed_by
 *     generated_by
 *     specialized_from
 *     lowered_from
 *     optimized_from
 *     compiled_by
 *     verified_by
 *     emitted_as
 *
 * Provenance recording belongs downstream.
 */


/*
 * ============================================================================
 * 32. DETERMINISM AND REPRODUCIBILITY
 * ============================================================================
 *
 * Parsing of this grammar is deterministic.
 *
 * This grammar MUST NOT:
 *
 *     read wall-clock time
 *     inspect hardware
 *     inspect filesystem state
 *     inspect network state
 *     execute metaprograms
 *     execute generated code
 *     perform target selection
 *
 * Deterministic compilation is a semantic/compiler property represented by
 * the appropriate compilation clauses.
 */


/*
 * ============================================================================
 * 33. SECURITY
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions.
 *
 * It performs no:
 *
 *     filesystem access
 *     network access
 *     subprocess execution
 *     environment access
 *     credential access
 *     secret access
 *     device access
 *     hardware access
 *     QPU access
 *
 * No source construct becomes authorized merely because this grammar accepts
 * it.
 *
 * The Rust implementation MUST remain safe Rust.
 */


/*
 * ============================================================================
 * 34. NO HARD-CODED SCALABILITY CEILINGS
 * ============================================================================
 *
 * This file intentionally contains no finite capacity constants.
 *
 * In particular, it contains no definitions corresponding to:
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
 * Nor does it encode equivalent limits under another name.
 *
 * A source program may express quantities symbolically or dynamically.
 *
 * The actual realization is bounded only by:
 *
 *     language semantics
 *     implementation correctness
 *     available resources
 *     target capabilities
 *     explicit policies
 *     execution environment
 *
 * "Scale to infinity" therefore means that the language does not introduce
 * an artificial finite machine ceiling.
 */


/*
 * ============================================================================
 * 35. FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * A future compile facility MUST NOT require this grammar to duplicate its
 * detailed syntax.
 *
 * The preferred process is:
 *
 *     new feature
 *         |
 *         v
 *     dedicated grammar/<owner>.g4
 *         |
 *         v
 *     AST contract
 *         |
 *         v
 *     semantic contract
 *         |
 *         v
 *     compile/intent integration
 *         |
 *         v
 *     tests
 *
 * Where possible, the new feature should be added behind an existing
 * composition boundary.
 *
 * This minimizes the number of files that must be reopened as the language
 * grows.
 */


/*
 * ============================================================================
 * 36. INTEGRATION WITH grammar/metaprogramming/
 * ============================================================================
 *
 * `intent.g4` is deliberately the COMPILE-SIDE consumer of the
 * metaprogramming composition root.
 *
 * The dependency is:
 *
 *     grammar/compile/intent.g4
 *                 |
 *                 v
 *     grammar/metaprogramming/metaprogramming.g4
 *                 |
 *        +--------+--------+--------+--------+
 *        |        |        |        |        |
 *        v        v        v        v        v
 *     compile  reflect  quote   generate  type-level
 *     time     inspect  splice  schemas   specialization
 *
 * Therefore:
 *
 *     intent.g4
 *
 * MUST NOT import:
 *
 *     capabilities.g4
 *     code-generation.g4
 *     compile-time-execution.g4
 *     compile-time.g4
 *     generation.g4
 *     introspection.g4
 *     quotation.g4
 *     reflection.g4
 *     schemas.g4
 *     specialization.g4
 *     syntax-tree.g4
 *     type-level.g4
 *     unquotation.g4
 *
 * individually.
 *
 * The existing `Metaprogramming` composition grammar is their integration
 * boundary.
 *
 * This is what makes the subsystem maintainable.
 */


/*
 * ============================================================================
 * 37. INTEGRATION WITH grammar/Zamani.g4
 * ============================================================================
 *
 * `grammar/Zamani.g4` remains the complete-language composition root.
 *
 * `intent.g4` MUST NOT become the complete-language root.
 *
 * The canonical parser should consume compile-intent constructs through the
 * parser composition layer.
 *
 * The conceptual direction is:
 *
 *     Zamani.g4
 *         |
 *         v
 *     ZamaniParser
 *         |
 *         +--> compilation
 *         |       |
 *         |       +--> compile intent
 *         |
 *         +--> ordinary language
 *         |
 *         +--> domains
 *         |
 *         +--> metaprogramming
 *
 * There must remain one complete-language parser authority.
 */


/*
 * ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * This grammar does not instantiate AST objects.
 *
 * The parser adapter should map accepted compile-intent constructs to the
 * domain-neutral AST representation already owned by the repository.
 *
 * The AST should preserve semantic information such as:
 *
 *     intent kind
 *     source span
 *     arguments
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     preferences
 *     policies
 *     provenance
 *     child constructs
 *
 * The AST MUST NOT contain physical target implementation details merely
 * because a compile-intent construct mentions a target capability.
 */


/*
 * ============================================================================
 * 39. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes compile intent after parsing.
 *
 * The semantic pipeline should normalize:
 *
 *     syntax
 *         ->
 *     compile intent
 *         ->
 *     requirements
 *         ->
 *     constraints
 *         ->
 *     capabilities
 *         ->
 *     resources
 *         ->
 *     policies
 *         ->
 *     specialization decisions
 *         ->
 *     optimization objectives
 *         ->
 *     lowering plan
 *
 * This is a semantic process, not a parser process.
 */


/*
 * ============================================================================
 * 40. IR CONTRACT
 * ============================================================================
 *
 * `intent.g4` creates no IR.
 *
 * Compile intent eventually influences the compiler planning/lowering layers.
 *
 * It MUST NOT create:
 *
 *     LLVM IR
 *     QIR
 *     vendor IR
 *     physical quantum topology
 *     FPGA routing representation
 *     ASIC implementation representation
 *
 * Quantum constructs eventually cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * The compile-intent grammar remains independent of the physical realization.
 */


/*
 * ============================================================================
 * 41. GENERATED SOURCE CONTRACT
 * ============================================================================
 *
 * Metaprogramming may generate source.
 *
 * Generated source MUST be treated as source and re-enter the canonical
 * frontend.
 *
 * The compiler MUST NOT silently treat generated text as already validated
 * semantic IR.
 *
 * Conceptually:
 *
 *     metaprogram
 *       |
 *       v
 *     generated source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *
 * This preserves the same correctness rules for handwritten and generated
 * programs.
 */


/*
 * ============================================================================
 * 42. ERROR CONTRACT
 * ============================================================================
 *
 * This grammar should report syntactic errors through the canonical parser
 * diagnostic mechanism.
 *
 * It MUST NOT:
 *
 *     execute fallback compilation;
 *     silently ignore unknown compile-intent constructs;
 *     reinterpret target requirements as ordinary expressions;
 *     silently select a physical target;
 *     silently discard unsupported requirements.
 *
 * Unsupported semantic intent should produce a semantic diagnostic rather
 * than silently changing program meaning.
 */


/*
 * ============================================================================
 * 43. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Language-version compatibility belongs to the repository compatibility
 * system.
 *
 * This grammar must not create ad-hoc historical syntax aliases.
 *
 * Deprecated compile-intent constructs should be represented by the
 * appropriate compatibility/deprecation mechanism.
 *
 * One semantic concept should have one canonical parser representation.
 */


/*
 * ============================================================================
 * 44. TEST CONTRACT
 * ============================================================================
 *
 * The production test matrix for this file must cover:
 *
 * --------------------------------------------------------------------------
 * STRUCTURAL
 * --------------------------------------------------------------------------
 *
 *     parser grammar compiles
 *     all imports resolve
 *     no import cycle is introduced
 *     token vocabulary resolves
 *     all exported rules resolve
 *
 * --------------------------------------------------------------------------
 * COMPILE
 * --------------------------------------------------------------------------
 *
 *     compile declarations
 *     compilation plans
 *     compile-time constructs
 *     feature selection
 *     target intent
 *     target selection
 *     optimization
 *     specialization
 *     lowering
 *     code generation
 *     reproducibility
 *     deterministic builds
 *     provenance
 *     artifacts
 *     caching
 *     deployment
 *     cross compilation
 *     conditional compilation
 *     profiles
 *
 * --------------------------------------------------------------------------
 * METAPROGRAMMING
 * --------------------------------------------------------------------------
 *
 *     compile-time execution
 *     reflection
 *     introspection
 *     quotation
 *     unquotation
 *     syntax-tree operations
 *     generation
 *     code generation
 *     specialization
 *     type-level computation
 *     schemas
 *     capability declarations/requirements
 *
 * --------------------------------------------------------------------------
 * CROSS-DOMAIN
 * --------------------------------------------------------------------------
 *
 *     classical + compile intent
 *     quantum + compile intent
 *     hybrid + compile intent
 *     HDL + compile intent
 *     hardware intent + compile intent
 *     AI/data + compile intent
 *     distributed + compile intent
 *     networking + compile intent
 *     interoperability + compile intent
 *     simulation + compile intent
 *
 * --------------------------------------------------------------------------
 * POCO-REAF
 * --------------------------------------------------------------------------
 *
 *     source contains no physical device identity
 *     source contains no fixed hardware capacity
 *     capability requirements remain symbolic
 *     resource requirements remain semantic
 *     target selection remains downstream
 *     generated source re-enters the canonical frontend
 *
 * --------------------------------------------------------------------------
 * NEGATIVE
 * --------------------------------------------------------------------------
 *
 *     malformed compile intent
 *     malformed target intent
 *     malformed feature predicates
 *     malformed specialization
 *     malformed lowering
 *     malformed metaprogramming construct
 *     invalid nested compile-intent placement
 *
 * --------------------------------------------------------------------------
 * SECURITY
 * --------------------------------------------------------------------------
 *
 *     parsing never executes compile-time code
 *     parsing never grants capabilities
 *     parsing never accesses filesystem
 *     parsing never accesses network
 *     parsing never accesses hardware
 *
 * --------------------------------------------------------------------------
 * DETERMINISM
 * --------------------------------------------------------------------------
 *
 *     identical source produces identical parse structure under identical
 *     parser configuration.
 *
 * --------------------------------------------------------------------------
 * SCALABILITY
 * --------------------------------------------------------------------------
 *
 *     no language-level finite capacity constants
 *     no fixed target catalogue
 *     no fixed quantum capacity
 *     no fixed hardware capacity
 *     no fixed metaprogram expansion ceiling
 *     no fixed specialization ceiling
 *
 * Tests may use finite values because tests execute on finite machines.
 *
 * Those values MUST NOT become language constants.
 */


/*
 * ============================================================================
 * 45. COMPLETION CRITERIA
 * ============================================================================
 *
 * `grammar/compile/intent.g4` is DONE when:
 *
 * [ ] It exists as a parser grammar named `CompileIntent`.
 *
 * [ ] It imports the existing compilation authorities rather than duplicating
 *     their syntax.
 *
 * [ ] It imports `Metaprogramming` as the sole metaprogramming composition
 *     boundary.
 *
 * [ ] It does not independently import every metaprogramming leaf grammar.
 *
 * [ ] It does not define lexer rules.
 *
 * [ ] It does not define ordinary expression syntax.
 *
 * [ ] It does not define ordinary declaration syntax.
 *
 * [ ] It does not define ordinary type syntax.
 *
 * [ ] It does not define target-specific hardware syntax.
 *
 * [ ] It does not define quantum gate catalogues.
 *
 * [ ] It does not define physical resource limits.
 *
 * [ ] It does not execute metaprograms.
 *
 * [ ] It does not construct IR.
 *
 * [ ] It does not select hardware.
 *
 * [ ] It does not perform resource negotiation.
 *
 * [ ] It does not authorize capabilities.
 *
 * [ ] It does not bypass semantic validation.
 *
 * [ ] Generated source re-enters the canonical frontend.
 *
 * [ ] The canonical quantum boundary remains `quantum::ir`.
 *
 * [ ] Rust integration remains compatible with Rust 1.97 or later.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Metaprogramming integration tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass within available implementation resources.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * The purpose of this file is ORCHESTRATION.
 *
 * It is intentionally not a second implementation of the compilation
 * subsystem.
 *
 * The architecture is:
 *
 *     compile/intent.g4
 *             |
 *             +------------------------------+
 *             |                              |
 *             v                              v
 *     compilation subsystem          metaprogramming/
 *             |                              |
 *             |                              v
 *             |                   metaprogramming.g4
 *             |                              |
 *             |                +-------------+-------------+
 *             |                |             |             |
 *             v                v             v             v
 *         compile-time     reflection    quotation     generation
 *         target           type-level    syntax-tree   schemas
 *         optimization     specialization capabilities
 *         lowering         introspection unquotation
 *             |
 *             v
 *       semantic planning
 *             |
 *             v
 *       canonical IR
 *             |
 *             +--> classical
 *             +--> quantum::ir
 *             +--> HDL/hardware
 *             +--> other domains
 *             |
 *             v
 *       target-independent optimization
 *             |
 *             v
 *       lowering / routing / scheduling / resilience
 *             |
 *             v
 *       ZQN / HAL / realization
 *
 * This preserves one language, one semantic model, one canonical frontend,
 * one metaprogramming composition boundary, and target-independent program
 * intent while allowing the implementation to scale with available
 * resources.
 *
 * ============================================================================
 */