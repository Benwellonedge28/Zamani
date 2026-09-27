/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/deployment.g4
 *
 * Grammar:
 *     CompileDeployment
 *
 * Status:
 *     PRODUCTION-READY COMPILATION/DEPLOYMENT INTEGRATION CONTRACT
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
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
 * This file defines the COMPILATION-LAYER DEPLOYMENT BOUNDARY.
 *
 * Deployment itself already has a canonical owner:
 *
 *     grammar/execution/deployment.g4
 *
 * Therefore this file MUST NOT create a second deployment language.
 *
 * Instead, this grammar provides the compilation-layer integration point for
 * deployment intent by delegating the actual deployment declaration to the
 * canonical execution Deployment grammar.
 *
 * This gives the compilation subsystem a stable rule for composing deployment
 * intent without duplicating:
 *
 *     deploymentDeclaration
 *     deploymentSubject
 *     deploymentBody
 *     deploymentClause
 *     deploymentArtifact
 *     deploymentEnvironment
 *     deploymentRequirement
 *     deploymentConstraint
 *     deploymentPreference
 *     deploymentHint
 *     deploymentCapability
 *     deploymentResource
 *     deploymentTarget
 *     deploymentPlacement
 *     deploymentPolicy
 *     deploymentLifecycle
 *     deploymentRollout
 *     deploymentAvailability
 *     deploymentPortability
 *     deploymentRecovery
 *     deploymentObservability
 *     deploymentParameter
 *     deploymentProperty
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical parser composition
 *          |
 *          v
 *     CompileDeployment
 *          |
 *          v
 *     canonical execution Deployment
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> portability analysis
 *          +--> target analysis
 *          +--> deployment validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical             quantum::ir          HDL/hardware
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                            optimization
 *                                 |
 *                       +---------+---------+
 *                       |                   |
 *                       v                   v
 *                    routing            scheduling
 *                       |                   |
 *                       +---------+---------+
 *                                 |
 *                                 v
 *                         resilience / QEC
 *                                 |
 *                                 v
 *                                ZQN
 *                                 |
 *                                 v
 *                                HAL
 *                                 |
 *                                 v
 *                         target realization
 *                                 |
 *                                 v
 *                              deployment
 *                                 |
 *                                 v
 *                               runtime
 *
 * ============================================================================
 * CORE DESIGN PRINCIPLE
 * ============================================================================
 *
 * Compilation deployment syntax describes:
 *
 *     WHAT deployment intent belongs to the compilation plan.
 *
 * It does NOT determine:
 *
 *     HOW deployment is physically performed.
 *
 * In particular, this grammar does not:
 *
 *     - discover hardware;
 *     - allocate devices;
 *     - select physical CPUs;
 *     - select physical GPUs;
 *     - select physical FPGAs;
 *     - select physical QPUs;
 *     - select physical qubits;
 *     - select cluster nodes;
 *     - contact cloud providers;
 *     - start containers;
 *     - start processes;
 *     - submit jobs;
 *     - open network connections;
 *     - perform authentication;
 *     - perform authorization;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform calibration;
 *     - perform QEC;
 *     - perform ZQN;
 *     - execute deployment.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Deployment syntax has ONE canonical owner:
 *
 *     grammar/execution/deployment.g4
 *
 * This file is an integration adapter.
 *
 * It MUST NOT redefine deploymentDeclaration or any deploymentClause rule.
 *
 * The ownership relationship is:
 *
 *     compile/deployment.g4
 *             |
 *             | delegates
 *             v
 *     execution/deployment.g4
 *
 * Therefore:
 *
 *     execution/deployment.g4
 *         = deployment syntax authority
 *
 *     compile/deployment.g4
 *         = compilation integration authority
 *
 * This avoids two subtly different meanings of:
 *
 *     deploy ...
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The dependency graph is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *        Core
 *          |
 *          v
 *     ExecutionContext
 *          |
 *          v
 *       Deployment
 *          |
 *          v
 *   CompileDeployment
 *
 * More precisely:
 *
 *     CompileDeployment
 *          -> Deployment
 *          -> ExecutionContext
 *          -> Core
 *
 * No dependency may point back from:
 *
 *     Deployment
 *
 * to:
 *
 *     CompileDeployment
 *
 * because that would create a grammar authority cycle.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILATION.G4
 * ============================================================================
 *
 * `grammar/compile/compilation.g4` is the compilation composition grammar.
 *
 * It currently composes independently-owned compilation constructs.
 *
 * This file should be imported there:
 *
 *     import Compile,
 *            CompileTime,
 *            ConditionalCompilation,
 *            FeatureSelection,
 *            CompileTarget,
 *            CompileTargetSelection,
 *            CompileOptimization,
 *            CompileCodeGeneration,
 *            CompileLowering,
 *            CompileDeployment;
 *
 * The compilation composition grammar should then expose:
 *
 *     compilationDeploymentReference
 *         : compileDeploymentDeclaration
 *         ;
 *
 * or equivalent delegation.
 *
 * It MUST NOT define another:
 *
 *     deploymentDeclaration
 *
 * rule.
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION/DEPLOYMENT.G4
 * ============================================================================
 *
 * The canonical deployment implementation remains:
 *
 *     grammar/execution/deployment.g4
 *
 * That grammar owns:
 *
 *     deploymentDeclaration
 *
 * CompileDeployment consumes that public entry point.
 *
 * Therefore this file imports:
 *
 *     Deployment
 *
 * and delegates through:
 *
 *     deploymentDeclaration
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION.G4
 * ============================================================================
 *
 * `grammar/execution/execution.g4` already identifies deployment as a
 * separate concern.
 *
 * It must continue to expose deployment through its own integration boundary.
 *
 * This file does not replace:
 *
 *     executionDeployment
 *
 * and does not modify execution semantics.
 *
 * The same canonical deployment declaration may therefore be composed by:
 *
 *     execution
 *
 * or:
 *
 *     compilation
 *
 * without creating two deployment languages.
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION-CONTEXT.G4
 * ============================================================================
 *
 * `execution/deployment.g4` already depends on:
 *
 *     ExecutionContext
 *
 * CompileDeployment deliberately does not reproduce execution-context rules.
 *
 * Therefore:
 *
 *     CompileDeployment
 *          -> Deployment
 *               -> ExecutionContext
 *
 * remains the only dependency path.
 *
 * Do NOT add:
 *
 *     ExecutionContext
 *
 * directly here merely to duplicate deployment context syntax.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE/ARTIFACTS.G4
 * ============================================================================
 *
 * Compilation artifacts and deployment are related but different concepts.
 *
 * Artifact ownership belongs to:
 *
 *     grammar/compile/artifacts.g4
 *
 * Deployment ownership belongs to:
 *
 *     grammar/execution/deployment.g4
 *
 * This file does not redefine artifact syntax.
 *
 * A deployment body may refer to an artifact through the canonical deployment
 * artifact mechanism already provided by Deployment.
 *
 * The semantic relationship is:
 *
 *     compilation artifact intent
 *          |
 *          v
 *     artifact semantic identity
 *          |
 *          v
 *     deployment artifact reference
 *
 * Artifact realization remains downstream.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE/TARGET-SELECTION.G4
 * ============================================================================
 *
 * Target selection and deployment are distinct.
 *
 * Target selection answers:
 *
 *     Which acceptable semantic realization may be selected?
 *
 * Deployment answers:
 *
 *     How should the resulting computation/artifact be made available to an
 *     execution environment?
 *
 * Therefore this file does NOT redefine:
 *
 *     targetSelectionDeclaration
 *
 * or any target-selection rule.
 *
 * A deployment declaration may reference target intent through the canonical
 * deployment expression/property mechanisms.
 *
 * Physical target resolution remains downstream.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE/PROFILES.G4
 * ============================================================================
 *
 * Compilation profiles may contain or reference deployment intent through
 * composition.
 *
 * Profile ownership remains:
 *
 *     grammar/compile/profiles.g4
 *
 * CompileDeployment does not define profile syntax.
 *
 * A profile reference is semantic data resolved by the compilation system.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE/PROVENANCE.G4
 * ============================================================================
 *
 * Deployment may consume provenance requirements.
 *
 * Provenance ownership remains:
 *
 *     grammar/compile/provenance.g4
 *
 * This grammar does not define:
 *
 *     hashes;
 *     signatures;
 *     attestations;
 *     certificate formats;
 *     transparency logs;
 *     signing algorithms.
 *
 * Such implementation details remain downstream.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE/DETERMINISTIC-BUILDS.G4
 * ============================================================================
 *
 * Deployment of reproducible artifacts may depend on deterministic-build
 * intent.
 *
 * Deterministic build syntax remains owned by:
 *
 *     grammar/compile/deterministic-builds.g4
 *
 * This file does not redefine determinism syntax.
 *
 * The semantic relationship is:
 *
 *     deterministic compilation
 *          ->
 *     reproducible artifact
 *          ->
 *     deployment
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE/CACHING.G4
 * ============================================================================
 *
 * Deployment may consume artifacts produced or retrieved through compilation
 * caching.
 *
 * Caching semantics remain owned by:
 *
 *     grammar/compile/caching.g4
 *
 * This grammar does not implement cache lookup, cache storage, invalidation,
 * or transport.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE/CODE-GENERATION.G4
 * ============================================================================
 *
 * Code generation produces semantic artifacts.
 *
 * Deployment consumes deployable semantic artifacts.
 *
 * Code generation remains owned by:
 *
 *     grammar/compile/code-generation.g4
 *
 * This grammar does not generate machine code.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE DEPLOYMENT
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/hardware/deployment.g4
 *
 * That grammar owns hardware-scoped deployment composition.
 *
 * CompileDeployment MUST NOT redefine hardware deployment semantics.
 *
 * The semantic layering is:
 *
 *     compile deployment
 *          |
 *          v
 *     execution deployment
 *          |
 *          +--> hardware deployment intent
 *          |
 *          v
 *     target/hardware realization
 *
 * Hardware deployment must remain target-specific downstream.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED DEPLOYMENT
 * ============================================================================
 *
 * The repository also contains:
 *
 *     grammar/distributed/deployment.g4
 *
 * Distributed deployment owns distributed-domain deployment composition.
 *
 * CompileDeployment must not duplicate:
 *
 *     node placement;
 *     replication;
 *     partitioning;
 *     distributed topology;
 *     distributed service deployment.
 *
 * Those concerns remain in the distributed domain.
 *
 * ============================================================================
 * INTEGRATION WITH AI MODEL DEPLOYMENT
 * ============================================================================
 *
 * The repository contains:
 *
 *     grammar/ai/model-deployment.g4
 *
 * Model deployment remains AI-domain syntax.
 *
 * CompileDeployment does not redefine:
 *
 *     model;
 *     training;
 *     inference;
 *     model serving;
 *     model rollout.
 *
 * AI deployment can be composed through the canonical deployment semantic
 * model.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Deployment must preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * wherever the program's semantic requirements permit it.
 *
 * A deployment declaration MUST NOT make the source program depend on a
 * particular physical machine merely because that machine was available when
 * the program was compiled.
 *
 * Deployment therefore distinguishes:
 *
 *     semantic requirement
 *     capability requirement
 *     resource requirement
 *     constraint
 *     preference
 *     hint
 *     target intent
 *     placement intent
 *     deployment policy
 *     artifact identity
 *     physical realization
 *
 * Only the downstream realization layer may bind these semantic intents to
 * actual hardware.
 *
 * ============================================================================
 * NO ARTIFICIAL HARDWARE LIMITS
 * ============================================================================
 *
 * This grammar contains NO universal limits for:
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
 *     MAX_REPLICAS
 *     MAX_INSTANCES
 *     MAX_REGIONS
 *     MAX_ARTIFACTS
 *     MAX_DEPLOYMENTS
 *
 * There is intentionally no finite grammar-level deployment capacity.
 *
 * Any actual limits belong to:
 *
 *     compiler resources;
 *     resource availability;
 *     target capabilities;
 *     operating-system limits;
 *     deployment environment;
 *     provider policy;
 *     explicit user constraints.
 *
 * ============================================================================
 * NO HARD-CODED TARGETS
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU models;
 *     GPU models;
 *     FPGA families;
 *     ASIC families;
 *     QPU vendors;
 *     cloud providers;
 *     operating systems;
 *     container engines;
 *     cluster schedulers;
 *     deployment platforms;
 *     physical devices.
 *
 * Examples such as:
 *
 *     gpu0
 *     qpu0
 *     node0
 *     device0
 *
 * remain ordinary semantic names unless another explicitly versioned
 * target-specific contract gives them meaning.
 *
 * ============================================================================
 * NO HARD-CODED TOPOLOGY
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     physical addresses;
 *     physical qubit mappings;
 *     CPU topology;
 *     GPU topology;
 *     FPGA topology;
 *     network topology;
 *     cluster topology;
 *     memory-bank layout;
 *     fixed accelerator placement.
 *
 * Deployment placement is intent.
 *
 * Actual placement belongs downstream.
 *
 * ============================================================================
 * SCALABILITY MODEL
 * ============================================================================
 *
 * Scalability is achieved through delegation and open semantic data.
 *
 * This file introduces no bounded alternatives.
 *
 * The canonical Deployment grammar uses repeated clauses and expression-based
 * values.
 *
 * Therefore deployment intent can describe:
 *
 *     one device;
 *     many devices;
 *     one node;
 *     many nodes;
 *     heterogeneous environments;
 *     distributed environments;
 *     embedded environments;
 *     accelerator environments;
 *     quantum environments;
 *     hybrid environments;
 *     future environments;
 *
 * without changing this grammar for each new machine class.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical resources.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM / HDL / HYBRID INTEGRATION
 * ============================================================================
 *
 * The deployment subject remains the canonical expression-based subject from
 * execution/deployment.g4.
 *
 * Consequently a deployment can semantically refer to:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware computation;
 *     AI computation;
 *     distributed computation;
 *     accelerator computation;
 *     scientific computation;
 *     embedded computation;
 *     future computation domains.
 *
 * No domain-specific deployment grammar is required here.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum deployment remains target-independent.
 *
 * This file MUST NOT encode:
 *
 *     fixed qubit counts;
 *     physical qubit IDs;
 *     gate sets;
 *     QPU vendor names;
 *     coupling maps;
 *     calibration data;
 *     pulse schedules;
 *     QEC implementations;
 *     noise models.
 *
 * The pipeline remains:
 *
 *     quantum source
 *          |
 *          v
 *     domain-neutral AST
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
 *          |
 *          v
 *     deployment
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Deployment may consume hardware/HDL semantic artifacts.
 *
 * It MUST NOT encode:
 *
 *     wire [31:0];
 *     fixed register widths;
 *     fixed FPGA resource counts;
 *     fixed ASIC resources;
 *     fixed memory sizes;
 *     physical addresses;
 *     fixed clock frequencies.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Deployment may consume requirements such as:
 *
 *     requires qubits >= n
 *     requires memory >= required_memory
 *     requires capability("tensor.compute")
 *     requires capability("gpu.compute")
 *     requires capability("quantum.measurement")
 *     requires topology(...)
 *
 * These are semantic contracts.
 *
 * They do NOT become parser-level hardware limits.
 *
 * Deployment MUST preserve the distinction:
 *
 *     requirement
 *         = mandatory semantic condition
 *
 *     constraint
 *         = mandatory realization restriction
 *
 *     preference
 *         = optimization preference
 *
 *     hint
 *         = advisory information
 *
 *     capability
 *         = facility that may be provided
 *
 *     resource
 *         = quantity/class of consumable or required capacity
 *
 *     target
 *         = acceptable semantic realization class
 *
 *     placement
 *         = realization intent
 *
 * ============================================================================
 * ARTIFACT / DEPLOYMENT DISTINCTION
 * ============================================================================
 *
 * An artifact is not a deployment.
 *
 * Artifact:
 *
 *     describes a compilation product or semantic output.
 *
 * Deployment:
 *
 *     describes how that product/computation is intended to become available
 *     to an execution environment.
 *
 * Therefore:
 *
 *     source
 *       ->
 *     semantic computation
 *       ->
 *     compilation
 *       ->
 *     artifact
 *       ->
 *     deployment
 *       ->
 *     execution
 *
 * remains the preferred conceptual pipeline.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT require a new deployment AST hierarchy.
 *
 * The parser output is the existing canonical:
 *
 *     deploymentDeclaration
 *
 * wrapped by:
 *
 *     compileDeploymentDeclaration
 *
 * only as an integration boundary.
 *
 * The frontend may represent the wrapper explicitly if the AST architecture
 * needs provenance of where the declaration was composed.
 *
 * If no such distinction is semantically required, the wrapper should lower
 * transparently to the existing deployment AST node.
 *
 * The semantic representation MUST preserve:
 *
 *     source span;
 *     deployment subject;
 *     deployment clauses;
 *     explicit values;
 *     ordering where meaningful;
 *     source-level identifiers;
 *     artifact references;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     capabilities;
 *     resources;
 *     targets;
 *     placement intent;
 *     lifecycle intent;
 *     provenance metadata.
 *
 * It MUST NOT contain:
 *
 *     physical device handles;
 *     live runtime objects;
 *     network sockets;
 *     filesystem handles;
 *     provider SDK objects;
 *     physical qubit IDs;
 *     scheduler state;
 *     routing state;
 *     calibration state.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - resolving the deployment subject;
 *     - validating deployment clauses;
 *     - resolving artifact identity;
 *     - checking artifact availability;
 *     - validating resource requirements;
 *     - validating capabilities;
 *     - validating target constraints;
 *     - checking portability;
 *     - checking deployment policy compatibility;
 *     - checking lifecycle semantics;
 *     - checking rollout semantics;
 *     - checking recovery policy;
 *     - checking observability policy;
 *     - checking provenance requirements;
 *     - checking conflicts between clauses;
 *     - checking whether deployment requirements are satisfiable.
 *
 * The grammar performs none of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * In particular, it MUST NOT create:
 *
 *     DeploymentIR
 *     CompileDeploymentIR
 *     HardwareDeploymentIR
 *     QuantumDeploymentIR
 *
 * as competing intermediate representations.
 *
 * Deployment intent lowers through the existing:
 *
 *     frontend AST
 *          ->
 *     semantic model
 *          ->
 *     canonical compilation / execution representations
 *
 * Quantum semantics continue through:
 *
 *     semantic quantum representation
 *          ->
 *     quantum::ir
 *
 * with exactly one canonical quantum IR boundary.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes deployment intent only after semantic validation.
 *
 * The compiler may:
 *
 *     - resolve artifacts;
 *     - resolve target classes;
 *     - resolve capabilities;
 *     - evaluate resource requirements;
 *     - produce deployment plans;
 *     - specialize artifacts;
 *     - select compatible realization paths.
 *
 * It must not treat source deployment syntax as permission to bypass semantic
 * validation.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime receives deployment plans/products generated downstream.
 *
 * Runtime MUST NOT parse this grammar directly as a substitute for semantic
 * validation.
 *
 * The intended direction is:
 *
 *     source
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     semantic deployment model
 *       ->
 *     compilation/deployment plan
 *       ->
 *     runtime/dispatcher
 *
 * ============================================================================
 * DEPLOYMENT IMPLEMENTATION BOUNDARY
 * ============================================================================
 *
 * Actual deployment may eventually involve:
 *
 *     local execution;
 *     embedded systems;
 *     CPU systems;
 *     GPU systems;
 *     FPGA systems;
 *     ASIC systems;
 *     QPU systems;
 *     simulators;
 *     emulators;
 *     clusters;
 *     HPC systems;
 *     distributed systems;
 *     edge systems;
 *     cloud systems;
 *     future systems.
 *
 * None of those implementations belongs in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no Rust code;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no environment inspection;
 *     no random behavior;
 *     no runtime calls.
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer configuration;
 *     imported grammar versions;
 *
 * the parser must produce the same syntactic structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar does not perform:
 *
 *     authentication;
 *     authorization;
 *     secret retrieval;
 *     credential access;
 *     command execution;
 *     filesystem access;
 *     network access;
 *     provider communication.
 *
 * Deployment security semantics belong to the security/runtime/deployment
 * systems.
 *
 * A deployment declaration is not itself authorization to deploy.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Downstream generated/frontend code MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * using safe Rust only.
 *
 * No `unsafe` Rust is required by this grammar.
 *
 * The grammar itself cannot introduce unsafe operations because it contains no
 * host-language actions.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors must remain syntax errors.
 *
 * Semantic deployment failures must remain semantic diagnostics.
 *
 * Examples:
 *
 *     malformed deploy declaration
 *         = parser diagnostic
 *
 *     unknown artifact
 *         = semantic diagnostic
 *
 *     unavailable capability
 *         = capability/resource diagnostic
 *
 *     insufficient resources
 *         = resource diagnostic
 *
 *     incompatible target
 *         = target/semantic diagnostic
 *
 *     rejected deployment policy
 *         = deployment-policy diagnostic
 *
 * The grammar must not attempt to determine whether a deployment is feasible.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical deployment syntax remains owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * This integration layer therefore preserves deployment syntax rather than
 * inventing a second spelling.
 *
 * New deployment capabilities, resources, environments, properties and
 * semantic target classes should normally be represented by the existing
 * expression/name extension points.
 *
 * A new physical platform MUST NOT require modifying this grammar merely to
 * make the platform name parse.
 *
 * Breaking changes to canonical deployment syntax require the normal Zamani
 * compatibility/versioning process.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 *     MAX_REPLICAS
 *     MAX_DEPLOYMENTS
 *
 * It contains no finite hardware enumeration.
 *
 * It contains no fixed:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     node;
 *     device;
 *     memory bank;
 *     physical qubit;
 *     network endpoint;
 *     accelerator.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * This integration grammar is intentionally minimal.
 *
 * It adds one delegation layer rather than reproducing the complete
 * deployment grammar.
 *
 * This avoids:
 *
 *     duplicate parsing;
 *     duplicate parse trees;
 *     duplicate semantic traversal;
 *     grammar divergence;
 *     larger generated parser state;
 *     competing deployment rules.
 *
 * Deployment scalability is therefore inherited from the canonical Deployment
 * grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The CompileDeployment integration MUST be tested at three levels.
 *
 * ---------------------------------------------------------------------------
 * A. UNIT / GRAMMAR TESTS
 * ---------------------------------------------------------------------------
 *
 * Verify:
 *
 *     compileDeploymentDeclaration
 *         accepts canonical deploymentDeclaration
 *
 * and rejects malformed deployment syntax through the canonical Deployment
 * grammar.
 *
 * ---------------------------------------------------------------------------
 * B. COMPOSITION TESTS
 * ---------------------------------------------------------------------------
 *
 * Verify compilation composition accepts:
 *
 *     deployment of a classical computation;
 *     deployment of a quantum computation;
 *     deployment of a hybrid computation;
 *     deployment of an HDL/hardware artifact;
 *     deployment of an AI computation;
 *     deployment of a distributed computation;
 *     deployment of an accelerator computation;
 *     deployment with resource requirements;
 *     deployment with capability requirements;
 *     deployment with target intent;
 *     deployment with placement intent;
 *     deployment with lifecycle intent;
 *     deployment with rollout intent;
 *     deployment with recovery intent;
 *     deployment with observability intent;
 *     deployment with provenance intent.
 *
 * ---------------------------------------------------------------------------
 * C. SCALABILITY TESTS
 * ---------------------------------------------------------------------------
 *
 * Generated tests must verify arbitrary source-level cardinality for:
 *
 *     deployment clauses;
 *     properties;
 *     artifacts;
 *     requirements;
 *     capabilities;
 *     resources;
 *     targets;
 *     environments;
 *     policies;
 *     deployment specifications.
 *
 * The tests MUST NOT establish a maximum as part of the language contract.
 *
 * ---------------------------------------------------------------------------
 * D. PORTABILITY TESTS
 * ---------------------------------------------------------------------------
 *
 * The same deployment source should remain syntactically valid when semantic
 * realization changes between:
 *
 *     embedded;
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     emulator;
 *     cluster;
 *     HPC;
 *     distributed;
 *     cloud;
 *     future architecture.
 *
 * Target feasibility is tested downstream rather than encoded here.
 *
 * ---------------------------------------------------------------------------
 * E. HARDWARE-INDEPENDENCE TESTS
 * ---------------------------------------------------------------------------
 *
 * Tests must prove that this grammar does not require:
 *
 *     physical device identifiers;
 *     physical addresses;
 *     fixed topology;
 *     fixed memory size;
 *     fixed register width;
 *     fixed qubit count;
 *     fixed accelerator count.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * `grammar/validation/` should verify:
 *
 *     1. CompileDeployment imports Deployment.
 *
 *     2. Deployment imports resolve.
 *
 *     3. No reverse import exists from Deployment to CompileDeployment.
 *
 *     4. `deploymentDeclaration` has exactly one syntax owner.
 *
 *     5. CompileDeployment does not define deployment clauses.
 *
 *     6. CompileDeployment uses the canonical ZamaniLexer vocabulary.
 *
 *     7. No second deployment AST is introduced solely for this grammar.
 *
 *     8. No second deployment IR is introduced.
 *
 *     9. No physical target list is encoded.
 *
 *    10. No universal resource limits are encoded.
 *
 *    11. No semantic actions exist.
 *
 *    12. No unsafe Rust dependency is introduced.
 *
 *    13. Rust 1.97 / 1.97.1 compatibility remains documented.
 *
 * ============================================================================
 * FEATURE-MANIFEST CONTRACT
 * ============================================================================
 *
 * This feature should have a corresponding machine-readable specification
 * manifest under:
 *
 *     grammar/specification/features/
 *
 * Recommended file:
 *
 *     compile-deployment.yaml
 *
 * The manifest should identify:
 *
 *     id:
 *     name:
 *     status:
 *     version:
 *     syntax:
 *     grammar:
 *     lexer_tokens:
 *     ast_nodes:
 *     semantic_rules:
 *     ir_mapping:
 *     compiler_consumers:
 *     runtime_consumers:
 *     domain:
 *     capabilities:
 *     resource_requirements:
 *     tests:
 *     compatibility:
 *     hard_coding_policy:
 *
 * The manifest is a contract, not another grammar.
 *
 * ============================================================================
 * DOCUMENTATION CONTRACT
 * ============================================================================
 *
 * `grammar/grammar.md` should report the implementation status of deployment
 * syntax.
 *
 * `grammar/Zamani-Grammar.md` may describe future or historical deployment
 * concepts, but it cannot independently authorize syntax.
 *
 * `grammar/DESIGN.md` remains the architectural authority.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when ALL of the following are true:
 *
 * [ ] `Deployment` remains the sole owner of deployment syntax.
 *
 * [ ] `CompileDeployment` compiles as an ANTLR parser grammar.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] The dependency direction is acyclic.
 *
 * [ ] `deploymentDeclaration` is delegated rather than duplicated.
 *
 * [ ] Compilation composition imports this grammar.
 *
 * [ ] Compilation composition exposes `compileDeploymentDeclaration`.
 *
 * [ ] Execution composition continues to use the canonical Deployment grammar.
 *
 * [ ] Artifact syntax remains owned by compile/artifacts.g4.
 *
 * [ ] Target syntax remains owned by compile/target.g4.
 *
 * [ ] Target-selection syntax remains owned by compile/target-selection.g4.
 *
 * [ ] Resource/capability syntax remains owned by resources/.
 *
 * [ ] Hardware deployment remains owned by hardware/deployment.g4.
 *
 * [ ] Distributed deployment remains owned by distributed/deployment.g4.
 *
 * [ ] AI model deployment remains owned by ai/model-deployment.g4.
 *
 * [ ] No deployment IR is introduced.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] `quantum::ir` remains the canonical quantum IR boundary.
 *
 * [ ] No physical hardware limit is encoded.
 *
 * [ ] No physical device is hard-coded.
 *
 * [ ] No provider is hard-coded.
 *
 * [ ] No topology is hard-coded.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97 / 1.97.1 compatibility is preserved.
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
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How does compilation composition reach the canonical deployment
 *      language without creating a second deployment authority?"
 *
 * It does NOT answer:
 *
 *     "How is deployment executed?"
 *
 *     "Which machine is selected?"
 *
 *     "Which device is allocated?"
 *
 *     "Which provider is contacted?"
 *
 *     "Which physical qubit is used?"
 *
 *     "How is an artifact packaged?"
 *
 *     "How is routing performed?"
 *
 *     "How is scheduling performed?"
 *
 *     "How is QEC performed?"
 *
 *     "How is ZQN performed?"
 *
 *     "How does HAL communicate with hardware?"
 *
 * Those remain downstream responsibilities.
 *
 * The final architecture is:
 *
 *     Program
 *       ->
 *     Compile
 *       ->
 *     Artifact
 *       ->
 *     Deployment intent
 *       ->
 *     Target/resource/capability resolution
 *       ->
 *     Realization
 *       ->
 *     Runtime
 *
 * with:
 *
 *     no artificial language-level hardware ceiling
 *
 * and with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * wherever the program's semantic requirements and available resources permit.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar CompileDeployment;


/*
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * The canonical parser vocabulary is the production Zamani lexer.
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Deployment itself is owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * This file delegates to that grammar.
 *
 * Core dependencies are inherited through Deployment and therefore are not
 * duplicated here.
 */

import Deployment;


/*
 * ============================================================================
 * PUBLIC COMPILATION DEPLOYMENT ENTRY POINT
 * ============================================================================
 *
 * This is the only public rule owned by this file.
 *
 * It provides a stable compilation-layer name while delegating the actual
 * deployment syntax to the canonical execution deployment grammar.
 *
 * The wrapper does not alter the deployment language.
 */

compileDeploymentDeclaration
    : deploymentDeclaration
    ;


/*
 * ============================================================================
 * EXPLICIT COMPOSITION ADAPTER
 * ============================================================================
 *
 * This named adapter is useful to compilation.g4 because it allows the
 * compilation composition layer to distinguish:
 *
 *     compilation deployment composition
 *
 * from:
 *
 *     execution deployment ownership.
 *
 * It still lowers to the same canonical deployment syntax and semantic model.
 */

compileDeploymentReference
    : compileDeploymentDeclaration
    ;


/*
 * ============================================================================
 * END
 * ============================================================================
 */