/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/deployment.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * PolicyDeployment
 *
 * STATUS
 * ------
 * PRODUCTION-READY POLICY/DEPLOYMENT COMPOSITION GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the POLICY-LAYER INTEGRATION BOUNDARY for deployment.
 *
 * It does NOT create a second deployment language.
 *
 * Canonical deployment syntax remains owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * Canonical policy syntax remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * This file composes those two existing authorities so that the semantic
 * policy system can govern deployment intent without redefining deployment
 * syntax.
 *
 *
 * ARCHITECTURAL PRINCIPLE
 * -----------------------
 *
 * Deployment answers:
 *
 *     "What deployment intent has the program expressed?"
 *
 * Policy answers:
 *
 *     "Under what governing requirements, constraints, permissions,
 *      prohibitions, preferences, fallbacks, security rules, resource rules,
 *      and other policies may that deployment intent be realized?"
 *
 * This file establishes the boundary between those concerns.
 *
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * ZamaniLexer
 *      |
 *      v
 * canonical parser composition
 *      |
 *      +------------------------------+
 *      |                              |
 *      v                              v
 * Deployment                      Policy
 * execution/deployment.g4         policies/policy.g4
 *      |                              |
 *      +--------------+---------------+
 *                     |
 *                     v
 *             PolicyDeployment
 *                     |
 *                     v
 *             domain-neutral AST
 *                     |
 *                     +--> deployment intent
 *                     |
 *                     +--> policy intent
 *                     |
 *                     v
 *             semantic validation
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *       resources capabilities effects
 *          |          |          |
 *          +----------+----------+
 *                     |
 *                     v
 *                  contracts
 *                     |
 *                     v
 *                   policy
 *                     |
 *                     v
 *                 provenance
 *                     |
 *                     v
 *            target-independent plan
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      classical  quantum::ir   HDL/hardware
 *                     |
 *                     v
 *          optimization/lowering
 *                     |
 *                     v
 *          placement/routing
 *                     |
 *                     v
 *             scheduling/resilience
 *                     |
 *                     v
 *                  ZQN/HAL
 *                     |
 *                     v
 *             target realization
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the POLICY/DEPLOYMENT COMPOSITION BOUNDARY.
 *
 * Specifically:
 *
 *     policyDeployment
 *     deploymentPolicy
 *     policyDeploymentIntent
 *     deploymentPolicyIntent
 *
 * These rules are adapters over canonical grammar rules.
 *
 * They exist so downstream parser composition, AST construction, semantic
 * analysis, and conformance tooling can refer to the policy/deployment
 * relationship through a stable named interface.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     keywords
 *     operators
 *     punctuation
 *     identifiers
 *     qualified names
 *     literals
 *     expressions
 *
 *     policy declaration syntax
 *     policy body syntax
 *     policy member syntax
 *     policy rules
 *     policy requirements
 *     policy constraints
 *     policy capabilities
 *     policy resources
 *     policy permissions
 *     policy prohibitions
 *     policy preferences
 *     policy fallbacks
 *     policy selection
 *     policy negotiation
 *     policy recovery
 *     policy adaptation
 *     policy sandboxing
 *     policy provenance
 *
 *     deployment declaration syntax
 *     deployment subject syntax
 *     deployment body syntax
 *     deployment requirements
 *     deployment constraints
 *     deployment preferences
 *     deployment hints
 *     deployment capabilities
 *     deployment resources
 *     deployment targets
 *     deployment placement
 *     deployment lifecycle
 *     deployment rollout
 *     deployment availability
 *     deployment portability
 *     deployment recovery
 *     deployment observability
 *     deployment parameters
 *     deployment metadata
 *
 *     resource allocation
 *     capability discovery
 *     target selection
 *     physical placement
 *     topology resolution
 *     routing
 *     scheduling
 *     runtime dispatch
 *     orchestration
 *     provider APIs
 *     hardware discovery
 *     quantum mapping
 *     quantum routing
 *     QEC
 *     calibration
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one grammar authority for each concern.
 *
 * Deployment syntax:
 *
 *     grammar/execution/deployment.g4
 *
 * Policy syntax:
 *
 *     grammar/policies/policy.g4
 *
 * Policy/deployment composition:
 *
 *     grammar/policies/deployment.g4
 *
 * Therefore this file MUST NOT reproduce rules from either source grammar.
 *
 * In particular, it MUST NOT define another:
 *
 *     deploymentDeclaration
 *     deploymentBody
 *     deploymentClause
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *
 * Doing so would create competing grammar authorities and eventually cause
 * parser/AST/semantic divergence.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT PARSER DEPENDENCIES
 * --------------------------
 *
 *     grammar/execution/deployment.g4
 *     grammar/policies/policy.g4
 *
 * The imported grammar names are:
 *
 *     Deployment
 *     Policy
 *
 *
 * DEPENDENCY DIRECTION
 * --------------------
 *
 *     lexer
 *       |
 *       +----------------------+
 *       |                      |
 *       v                      v
 *   Deployment              Policy
 *       |                      |
 *       +----------+-----------+
 *                  |
 *                  v
 *          PolicyDeployment
 *
 *
 * This file MUST NOT become a dependency of either:
 *
 *     grammar/execution/deployment.g4
 *     grammar/policies/policy.g4
 *
 * Otherwise a cyclic grammar dependency would be introduced.
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This is an ANTLR parser grammar.
 *
 * It imports parser grammars by grammar name.
 *
 * It does NOT:
 *
 *     - define lexical tokens;
 *     - use filesystem-style grammar imports;
 *     - duplicate token vocabulary;
 *     - define Rust actions;
 *     - perform semantic actions.
 *
 * The build system MUST make the canonical grammar directories available to
 * the ANTLR grammar import path.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file MUST NOT require a new physical AST node merely because this
 * adapter exists.
 *
 * The parser may expose an adapter context named:
 *
 *     policyDeployment
 *
 * and/or:
 *
 *     deploymentPolicy
 *
 * but semantic lowering SHOULD normalize those contexts into the existing
 * deployment/policy semantic representation.
 *
 *
 * Recommended semantic relationship:
 *
 *     DeploymentIntent
 *          |
 *          +--> governing PolicySet / PolicyContext
 *
 * rather than:
 *
 *     PolicyDeploymentAST
 *
 * as a new competing domain-specific deployment representation.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A policy/deployment composition MUST preserve the distinction between:
 *
 *     deployment intent
 *     policy intent
 *     resource requirements
 *     capability requirements
 *     constraints
 *     preferences
 *     permissions
 *     prohibitions
 *     security requirements
 *     execution requirements
 *
 * Policy resolution MUST NOT mutate the meaning of the deployment syntax
 * during parsing.
 *
 * Semantic validation determines whether the policy permits, restricts,
 * prefers, rejects, or otherwise governs a deployment realization.
 *
 *
 * ============================================================================
 * IMPORTANT SEMANTIC RULE
 * ============================================================================
 *
 * A policy MUST NOT be interpreted as a physical deployment instruction merely
 * because it refers to a target, resource, capability, environment, or
 * placement concept.
 *
 * For example, policy intent may express:
 *
 *     prefer capability("quantum.measurement")
 *
 * or:
 *
 *     require capability("tensor.compute")
 *
 * or:
 *
 *     forbid effect("network")
 *
 * but this grammar does not resolve:
 *
 *     which QPU
 *     which CPU
 *     which GPU
 *     which FPGA
 *     which node
 *     which memory bank
 *     which topology
 *     which cloud region
 *     which provider
 *
 * Those are downstream semantic/resource/target concerns.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file MUST preserve Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 * semantics.
 *
 * Therefore this grammar introduces NO universal physical deployment limits.
 *
 * It MUST NOT define:
 *
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_REPLICAS
 *     MAX_INSTANCES
 *     MAX_REGIONS
 *     MAX_RESOURCES
 *     MAX_TARGETS
 *     MAX_ARTIFACTS
 *     MAX_DEPLOYMENTS
 *
 * No fixed machine size, topology size, processor count, accelerator count,
 * memory capacity, quantum capacity, network capacity, or deployment count is
 * encoded by this grammar.
 *
 * The same source-level deployment intent may therefore be evaluated against
 * environments ranging from very small systems to the largest environment
 * that the implementation and available resources can support.
 *
 *
 * ============================================================================
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * Resource quantities remain expressions.
 *
 * Capability identities remain open-ended semantic names.
 *
 * Target identities remain expressions/qualified names where the canonical
 * deployment grammar permits them.
 *
 * Physical feasibility is determined downstream.
 *
 * This file therefore has no language-level capacity ceiling.
 *
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not enumerate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     embedded processor
 *     cluster
 *     supercomputer
 *     cloud
 *     edge device
 *     network topology
 *     quantum topology
 *
 * These are capabilities/resources/targets represented by the canonical
 * deployment and policy semantics rather than grammar-level alternatives.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum deployment policy MUST remain separate from quantum operation
 * syntax.
 *
 * This file does NOT define:
 *
 *     gates
 *     circuits
 *     qubits
 *     measurement operations
 *     routing
 *     decomposition
 *     scheduling
 *     QEC
 *     calibration
 *
 * Quantum-related deployment policy is represented through the existing
 * policy/deployment/resource/capability abstractions.
 *
 * Downstream quantum compilation MUST converge on:
 *
 *     quantum::ir
 *
 * before target-specific realization.
 *
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical deployment policy uses the same policy/deployment model.
 *
 * It MUST NOT require a second classical deployment grammar.
 *
 *
 * ============================================================================
 * HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware deployment policy is represented through the same
 * deployment intent plus resource/capability/policy model.
 *
 * Hardware realization, synthesis, placement, timing, routing, and physical
 * constraints remain downstream concerns.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed deployment uses the canonical deployment grammar and policy
 * semantics.
 *
 * This file MUST NOT introduce fixed:
 *
 *     node counts
 *     replica counts
 *     shard counts
 *     process counts
 *     actor counts
 *     region counts
 *
 * Any such quantity is semantic program data or an evaluated resource
 * constraint, not a grammar-level limit.
 *
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Security policy remains owned by:
 *
 *     grammar/policies/security.g4
 *     grammar/security/
 *
 * This file MUST NOT recreate:
 *
 *     authorization
 *     credentials
 *     identity
 *     trust
 *     authentication
 *     sandbox semantics
 *
 * It only exposes the policy/deployment composition boundary.
 *
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource semantics remain owned by:
 *
 *     grammar/policies/resource.g4
 *     grammar/resources/
 *
 * This file MUST NOT define resource categories or physical allocation rules.
 *
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capabilities remain open-world.
 *
 * A future capability MUST NOT require this file to change merely because a
 * new computational technology becomes available.
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Effects remain owned by the canonical effects subsystem.
 *
 * A deployment policy may govern effects through the existing policy grammar,
 * but this file does not define effect syntax.
 *
 * Examples of semantic effects that may be governed downstream include:
 *
 *     io
 *     network
 *     native
 *     foreign
 *     mutation
 *     randomness
 *     measurement
 *     distributed
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts remain owned by the validation/contract subsystem.
 *
 * Deployment policy may be checked against:
 *
 *     requirements
 *     preconditions
 *     postconditions
 *     invariants
 *     guarantees
 *     properties
 *
 * This adapter does not redefine contract syntax.
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Policy/deployment decisions SHOULD preserve provenance through the semantic
 * pipeline.
 *
 * Relevant provenance may include:
 *
 *     source declaration
 *     policy source
 *     policy decision
 *     requirement
 *     capability evaluation
 *     resource evaluation
 *     target feasibility result
 *     fallback decision
 *     deployment plan
 *     realization decision
 *
 * Provenance syntax remains owned by the provenance subsystem.
 *
 *
 * ============================================================================
 * ADAPTATION AND FALLBACK
 * ============================================================================
 *
 * Adaptive deployment remains a semantic/runtime concern.
 *
 * Policy may govern:
 *
 *     preferred realization
 *     fallback realization
 *     recovery intent
 *     retry intent
 *     adaptation intent
 *
 * However, this file MUST NOT implement:
 *
 *     retry loops
 *     orchestration
 *     failover
 *     migration
 *     live traffic switching
 *     checkpoint restoration
 *
 * Those operations belong downstream.
 *
 *
 * ============================================================================
 * DETERMINISM AND REPRODUCIBILITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic actions;
 *     no predicates;
 *     no Rust code;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no resource discovery;
 *     no random behavior;
 *     no mutable global state.
 *
 * Parsing therefore depends only on the supplied token stream and imported
 * grammar rules.
 *
 * Deterministic deployment realization is NOT guaranteed by this grammar.
 *
 * The semantic/runtime system must distinguish:
 *
 *     deterministic parsing
 *     deterministic policy resolution
 *     deterministic planning
 *     reproducible compilation
 *     deterministic execution
 *
 * These are separate properties.
 *
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is an ANTLR grammar and therefore contains no Rust implementation
 * code.
 *
 * The consuming Zamani implementation MUST:
 *
 *     support Rust 1.97 or later;
 *     use Rust 2021 or later as declared by the repository;
 *     remain safe Rust;
 *     contain no unsafe Rust.
 *
 * This file MUST NOT require unsafe implementation techniques.
 *
 *
 * ============================================================================
 * PUBLIC ADAPTER RULES
 * ============================================================================
 *
 * The following rules are intentionally thin aliases/adapters.
 *
 * They MUST NOT duplicate the imported grammar definitions.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar PolicyDeployment;

import
    Deployment,
    Policy
;


/*
 * ============================================================================
 * 1. POLICY DEPLOYMENT
 * ============================================================================
 *
 * This is the canonical policy-layer adapter for deployment intent.
 *
 * The actual deployment syntax remains:
 *
 *     deploymentDeclaration
 *
 * from grammar/execution/deployment.g4.
 *
 * This rule gives policy-aware parser composition a stable name without
 * redefining deployment syntax.
 *
 * Semantic interpretation:
 *
 *     PolicyDeployment
 *          |
 *          +--> DeploymentIntent
 *          |
 *          +--> PolicyContext
 *
 * The policy context is supplied by the surrounding policy composition rather
 * than invented as a second deployment language here.
 */

policyDeployment
    : deploymentDeclaration
    ;


/*
 * ============================================================================
 * 2. DEPLOYMENT POLICY
 * ============================================================================
 *
 * This adapter exposes the canonical policy declaration as the policy side of
 * the deployment/policy boundary.
 *
 * Actual policy syntax remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * No policy rule is duplicated here.
 */

deploymentPolicy
    : policyDeclaration
    ;


/*
 * ============================================================================
 * 3. EXPLICIT POLICY-DEPLOYMENT INTENT
 * ============================================================================
 *
 * This rule provides a stable integration point for parser compositions that
 * want to accept either:
 *
 *     deployment intent
 *
 * or:
 *
 *     policy intent governing deployment
 *
 * without creating a third syntax.
 *
 * The semantic layer is responsible for determining the relationship between
 * the two declarations.
 *
 * This is intentionally a sequence rather than a new keyword-based language.
 *
 * No additional lexer token is required.
 */

policyDeploymentIntent
    : policyDeployment
    | deploymentPolicy
    ;


/*
 * ============================================================================
 * 4. DEPLOYMENT POLICY INTENT
 * ============================================================================
 *
 * Alias used by downstream grammar composition when it needs to state that
 * deployment policy is being consumed as a semantic input.
 *
 * It intentionally resolves to the canonical policy grammar.
 */

deploymentPolicyIntent
    : deploymentPolicy
    ;


/*
 * ============================================================================
 * 5. CANONICAL COMPOSITION ENTRY
 * ============================================================================
 *
 * This rule is useful to grammar composition and conformance tooling.
 *
 * It accepts the two independently authoritative source constructs without
 * merging their syntax.
 *
 * IMPORTANT:
 *
 * This does NOT imply that arbitrary ordering or co-location is semantically
 * valid in every source context.
 *
 * Contextual placement, declaration ordering, scope, precedence, policy
 * activation, and applicability remain semantic concerns.
 *
 * The rule exists as an integration boundary, not as a universal top-level
 * source rule.
 */

policyDeploymentComposition
    : policyDeploymentIntent
    | deploymentPolicyIntent
    ;


/*
 * ============================================================================
 * 6. DEPLOYMENT POLICY REFERENCE
 * ============================================================================
 *
 * A deployment policy reference uses the existing policy declaration model.
 *
 * The actual identity is represented by the policy grammar's qualified-name
 * and expression infrastructure.
 *
 * This rule intentionally does not introduce a new keyword or identifier
 * syntax.
 *
 * It is an adapter for downstream semantic tooling.
 */

deploymentPolicyReference
    : policyDeclaration
    ;


/*
 * ============================================================================
 * 7. INTEGRATION ALIASES
 * ============================================================================
 *
 * These aliases provide stable names for consumers without taking ownership of
 * the underlying syntax.
 *
 * They are deliberately simple so future changes to deployment or policy
 * syntax remain localized to their respective authorities.
 */

deploymentPolicySource
    : deploymentPolicy
    ;


policyDeploymentSource
    : policyDeployment
    ;


/*
 * ============================================================================
 * 8. SEMANTIC BOUNDARY DOCUMENTATION
 * ============================================================================
 *
 * The following conceptual relationship is normative:
 *
 *     deploymentDeclaration
 *             |
 *             v
 *       DeploymentIntent
 *             |
 *             +-------------------+
 *             |                   |
 *             v                   v
 *       PolicyContext       ResourceContext
 *             |                   |
 *             +---------+---------+
 *                       |
 *                       v
 *                Semantic Planner
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *          target    resource   capability
 *          intent    analysis   analysis
 *             |         |         |
 *             +---------+---------+
 *                       |
 *                       v
 *                  feasibility
 *                       |
 *                 +-----+-----+
 *                 |           |
 *                 v           v
 *              accepted    rejected
 *                 |
 *                 v
 *             execution plan
 *
 * This relationship MUST NOT be implemented as parser-time target selection.
 *
 *
 * ============================================================================
 * 9. NO DUPLICATE DEPLOYMENT SEMANTICS
 * ============================================================================
 *
 * The following constructs remain exclusively owned by
 * grammar/execution/deployment.g4:
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
 *     deploymentValue
 *     deploymentPropertyBlock
 *     deploymentList
 *     deploymentInvocation
 *
 * This file MUST NOT redefine any of them.
 *
 *
 * ============================================================================
 * 10. NO DUPLICATE POLICY SEMANTICS
 * ============================================================================
 *
 * The following constructs remain exclusively owned by
 * grammar/policies/policy.g4:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyRule
 *     policyRequirement
 *     policyConstraint
 *     policyCapability
 *     policyResource
 *     policyPermission
 *     policyProhibition
 *     policyPreference
 *     policyFallback
 *     policySelection
 *     policyNegotiation
 *     policyRetry
 *     policyRecovery
 *     policyEscalation
 *     policyRejection
 *     policySimulation
 *     policyAdaptation
 *     policySandbox
 *     policyDeterminism
 *     policyReproducibility
 *     policyEffect
 *     policyProvenance
 *     policyContractReference
 *     policyComposition
 *     policyProperty
 *
 * This file MUST NOT redefine them.
 *
 *
 * ============================================================================
 * 11. OPEN-WORLD GUARANTEE
 * ============================================================================
 *
 * Future deployment concepts MUST be introduced by extending the canonical
 * deployment grammar or semantic model, not by adding an ever-growing list
 * of deployment keywords here.
 *
 * Future policy concepts MUST be introduced through the canonical policy
 * grammar or policy semantic model.
 *
 * This file should remain stable even as new:
 *
 *     hardware
 *     accelerators
 *     quantum technologies
 *     execution environments
 *     distributed systems
 *     AI systems
 *     data systems
 *     HDL technologies
 *     interoperability mechanisms
 *
 * are added to Zamani.
 *
 *
 * ============================================================================
 * 12. NO APPLICATION-SPECIFIC DEPLOYMENT KEYWORDS
 * ============================================================================
 *
 * This file MUST NOT enumerate application concepts such as:
 *
 *     cloud_provider
 *     kubernetes
 *     docker
 *     serverless
 *     gpu_vendor
 *     qpu_vendor
 *     robot
 *     blockchain
 *     payment
 *     database_vendor
 *
 * Such concepts belong in:
 *
 *     dialects
 *     libraries
 *     capabilities
 *     resources
 *     policies
 *     interoperability adapters
 *     target-specific tooling
 *
 * rather than the universal grammar.
 *
 *
 * ============================================================================
 * 13. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This grammar introduces no special semantic diagnostics.
 *
 * Parser diagnostics come from the imported canonical grammars.
 *
 * Semantic diagnostics belong downstream and MUST distinguish:
 *
 *     syntax error
 *     malformed deployment
 *     malformed policy
 *     invalid policy/deployment relationship
 *     unsatisfied requirement
 *     unavailable capability
 *     insufficient resource
 *     prohibited operation
 *     violated contract
 *     security violation
 *     target infeasibility
 *     unsupported backend
 *
 * A target/resource failure MUST NOT be reported as a syntax error.
 *
 *
 * ============================================================================
 * 14. AST / SEMANTIC / IR INTEGRATION
 * ============================================================================
 *
 * AST OWNER:
 *
 *     Existing domain-neutral Zamani AST implementation.
 *
 * SEMANTIC OWNER:
 *
 *     Policy semantic subsystem
 *     Deployment semantic subsystem
 *
 * RESOURCE OWNER:
 *
 *     grammar/resources/ and corresponding semantic implementation.
 *
 * CAPABILITY OWNER:
 *
 *     grammar/resources/capabilities.g4 and corresponding semantic
 *     implementation.
 *
 * EFFECT OWNER:
 *
 *     grammar/effects/ and corresponding semantic implementation.
 *
 * SECURITY OWNER:
 *
 *     grammar/security/
 *     grammar/policies/security.g4
 *
 * PROVENANCE OWNER:
 *
 *     provenance subsystem.
 *
 * IR OWNER:
 *
 *     canonical semantic model;
 *     classical IR where applicable;
 *     quantum::ir for quantum computation;
 *     HDL/hardware representation where applicable.
 *
 * This file MUST NOT introduce:
 *
 *     PolicyDeploymentIR
 *     DeploymentPolicyIR
 *
 * as competing universal IRs.
 *
 *
 * ============================================================================
 * 15. TARGET RESOLUTION BOUNDARY
 * ============================================================================
 *
 * The policy/deployment grammar MUST stop before physical realization.
 *
 * The downstream sequence is:
 *
 *     policy + deployment
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     requirements
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     resource feasibility
 *          |
 *          v
 *     target resolution
 *          |
 *          v
 *     placement
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience/recovery
 *          |
 *          v
 *     target realization
 *
 * None of these downstream algorithms belong in this file.
 *
 *
 * ============================================================================
 * 16. QUANTUM TARGET BOUNDARY
 * ============================================================================
 *
 * If deployment concerns a quantum computation:
 *
 *     deployment policy
 *          |
 *          v
 *     quantum semantic validation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC/resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * This grammar does not define any of those quantum implementation details.
 *
 *
 * ============================================================================
 * 17. HDL TARGET BOUNDARY
 * ============================================================================
 *
 * If deployment concerns HDL/hardware:
 *
 *     deployment policy
 *          |
 *          v
 *     hardware/HDL semantic validation
 *          |
 *          v
 *     HDL/hardware representation
 *          |
 *          v
 *     verification
 *          |
 *          v
 *     synthesis/lowering
 *          |
 *          v
 *     placement/routing/timing
 *          |
 *          v
 *     physical realization
 *
 * This file remains unchanged as new hardware targets are introduced.
 *
 *
 * ============================================================================
 * 18. DISTRIBUTED TARGET BOUNDARY
 * ============================================================================
 *
 * If deployment concerns distributed execution:
 *
 *     deployment policy
 *          |
 *          v
 *     distributed semantic model
 *          |
 *          v
 *     topology/resource/capability analysis
 *          |
 *          v
 *     placement
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     communication planning
 *          |
 *          v
 *     runtime realization
 *
 * This file does not enumerate node counts, regions, replicas, processes,
 * actors, or devices.
 *
 *
 * ============================================================================
 * 19. ADAPTIVE EXECUTION BOUNDARY
 * ============================================================================
 *
 * Deployment policy may influence adaptive execution.
 *
 * The runtime may have semantic states such as:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * and outcomes such as:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These are runtime/semantic concepts.
 *
 * They MUST NOT become parser-level finite deployment states unless explicitly
 * established by the canonical semantic specification.
 *
 *
 * ============================================================================
 * 20. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical deployment syntax remains valid.
 *
 * Existing canonical policy syntax remains valid.
 *
 * This file adds a named composition boundary without requiring new lexical
 * tokens.
 *
 * Therefore:
 *
 *     lexer compatibility: preserved
 *     deployment grammar compatibility: preserved
 *     policy grammar compatibility: preserved
 *
 * Future extensions MUST prefer:
 *
 *     new semantic capabilities
 *     new resource names
 *     new capability names
 *     new policy properties
 *     new dialects
 *
 * over unnecessary universal keyword additions.
 *
 *
 * ============================================================================
 * 21. CONFORMANCE TEST CONTRACT
 * ============================================================================
 *
 * The implementation MUST test this file at all applicable levels.
 *
 * ---------------------------------------------------------------------------
 * Positive tests
 * ---------------------------------------------------------------------------
 *
 * 1. A canonical deployment declaration is accepted through policyDeployment.
 *
 * 2. A canonical policy declaration is accepted through deploymentPolicy.
 *
 * 3. Both adapters preserve the underlying canonical parse tree structure.
 *
 * 4. Existing deployment syntax remains accepted.
 *
 * 5. Existing policy syntax remains accepted.
 *
 *
 * ---------------------------------------------------------------------------
 * Negative tests
 * ---------------------------------------------------------------------------
 *
 * The test suite MUST verify rejection of:
 *
 * 1. invented deployment keywords;
 * 2. invented policy keywords;
 * 3. malformed imported deployment syntax;
 * 4. malformed imported policy syntax;
 * 5. duplicate competing deployment syntax introduced by this file;
 * 6. invalid tokens not owned by the canonical lexer.
 *
 *
 * ---------------------------------------------------------------------------
 * Boundary tests
 * ---------------------------------------------------------------------------
 *
 * Test combinations involving:
 *
 *     deployment + requirements
 *     deployment + capabilities
 *     deployment + resources
 *     deployment + constraints
 *     deployment + preferences
 *     deployment + security policy
 *     deployment + execution policy
 *     deployment + fallback policy
 *     deployment + provenance
 *     deployment + reproducibility
 *     deployment + simulation
 *     deployment + adaptation
 *     deployment + classical execution
 *     deployment + quantum execution
 *     deployment + HDL/hardware execution
 *     deployment + distributed execution
 *
 *
 * ---------------------------------------------------------------------------
 * Scalability tests
 * ---------------------------------------------------------------------------
 *
 * Tests MUST demonstrate that this file introduces no fixed universal limit
 * on:
 *
 *     deployment declarations
 *     policy declarations
 *     policy members
 *     resource expressions
 *     capability expressions
 *     target expressions
 *     deployment metadata
 *     nested semantic values
 *
 * Tests MUST NOT claim literal mathematical infinity.
 *
 * They MUST instead verify that there is no grammar-level artificial ceiling
 * and that finite implementation/resource exhaustion is reported correctly.
 *
 *
 * ---------------------------------------------------------------------------
 * Determinism tests
 * ---------------------------------------------------------------------------
 *
 * Given the same canonical token stream, the adapter grammar MUST produce
 * equivalent parse structure.
 *
 *
 * ---------------------------------------------------------------------------
 * Cross-domain tests
 * ---------------------------------------------------------------------------
 *
 * At minimum, conformance coverage SHOULD include:
 *
 *     classical deployment
 *     quantum deployment
 *     hybrid quantum/classical deployment
 *     HDL/hardware deployment
 *     AI/model deployment
 *     distributed deployment
 *     accelerator deployment
 *     simulation deployment
 *
 * The adapter grammar itself must remain unchanged across those domains.
 *
 *
 * ============================================================================
 * 22. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST pass the following audit.
 *
 * No:
 *
 *     fixed hardware capacity
 *     fixed resource capacity
 *     fixed target enumeration
 *     fixed deployment count
 *     fixed node count
 *     fixed replica count
 *     fixed processor count
 *     fixed quantum capacity
 *     fixed memory capacity
 *     fixed network capacity
 *     vendor-specific universal syntax
 *
 * The file contains no such constants or enumerations.
 *
 *
 * ============================================================================
 * 23. MAINTAINABILITY CONTRACT
 * ============================================================================
 *
 * This file is intentionally small and stable.
 *
 * A developer completing this file MUST NOT later need to edit it merely
 * because:
 *
 *     a new hardware target is added;
 *     a new quantum processor is supported;
 *     a new GPU is supported;
 *     a new accelerator appears;
 *     a new deployment provider exists;
 *     a new resource category exists;
 *     a new capability exists;
 *     a new policy property exists;
 *     a new AI model type exists;
 *     a new distributed topology exists;
 *     a new HDL technology exists.
 *
 * Such extensions belong to the appropriate canonical semantic owners.
 *
 *
 * ============================================================================
 * 24. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [ ] It imports only canonical Deployment and Policy parser grammars.
 *
 * [ ] It introduces no duplicate deployment syntax.
 *
 * [ ] It introduces no duplicate policy syntax.
 *
 * [ ] It introduces no new lexer vocabulary.
 *
 * [ ] It provides stable policy/deployment composition rules.
 *
 * [ ] Its dependency direction is acyclic.
 *
 * [ ] Deployment semantics remain owned by execution/deployment.g4.
 *
 * [ ] Policy semantics remain owned by policies/policy.g4.
 *
 * [ ] Resource semantics remain owned by resources/policies resource owners.
 *
 * [ ] Capability semantics remain open-world.
 *
 * [ ] Security semantics remain owned by security policy owners.
 *
 * [ ] Provenance remains owned by the provenance subsystem.
 *
 * [ ] AST representation remains domain-neutral.
 *
 * [ ] No competing deployment/policy IR is introduced.
 *
 * [ ] Quantum computation can pass through quantum::ir downstream.
 *
 * [ ] HDL/hardware realization remains downstream.
 *
 * [ ] Distributed realization remains downstream.
 *
 * [ ] No physical capacity limit is encoded.
 *
 * [ ] No vendor/platform is hard-coded.
 *
 * [ ] Positive conformance tests exist.
 *
 * [ ] Negative conformance tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Rust implementation remains compatible with Rust 1.97+.
 *
 * [ ] Rust implementation remains safe Rust with no unsafe.
 *
 *
 * ============================================================================
 * 25. FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file is deliberately a COMPOSITION BOUNDARY rather than a feature
 * catalog.
 *
 * The universal language remains extensible because deployment and policy
 * semantics are open-world and target-independent.
 *
 * The architecture is:
 *
 *     source
 *       |
 *       v
 *     deployment intent
 *       +
 *     policy intent
 *       |
 *       v
 *     semantic validation
 *       |
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> security
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       |
 *       v
 *     target-independent planning
 *       |
 *       v
 *     specialization
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * The source-level meaning remains independent of the physical deployment
 * environment.
 *
 * That is the required foundation for POCO-REAF.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */