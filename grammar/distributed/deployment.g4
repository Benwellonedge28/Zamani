/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/deployment.g4
 *
 * Grammar:
 *     DistributedDeployment
 *
 * Status:
 *     Production distributed-deployment composition grammar
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No mutable parser-global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the DISTRIBUTED-DOMAIN COMPOSITION BOUNDARY for
 * deployment intent.
 *
 * It does NOT create a second deployment language.
 *
 * Generic deployment syntax is owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * This file exists so the distributed grammar can explicitly expose
 * deployment as a distributed-domain construct while reusing the canonical
 * deployment grammar.
 *
 * Conceptually:
 *
 *     distributed deployment
 *             |
 *             v
 *     canonical deployment intent
 *             |
 *             v
 *     distributed semantic analysis
 *             |
 *             +--> resource analysis
 *             +--> capability analysis
 *             +--> placement
 *             +--> topology
 *             +--> routing
 *             +--> scheduling
 *             +--> resilience
 *             +--> target realization
 *             |
 *             v
 *          runtime
 *
 * ============================================================================
 * CRITICAL OWNERSHIP DECISION
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed deployment composition;
 *     - the distributed deployment entry point;
 *     - association of a deployment subject with the distributed domain;
 *     - distributed-domain parser delegation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic deployment declarations;
 *     - deployment bodies;
 *     - deployment clauses;
 *     - deployment artifacts;
 *     - deployment environments;
 *     - deployment requirements;
 *     - deployment constraints;
 *     - deployment preferences;
 *     - deployment hints;
 *     - deployment capabilities;
 *     - deployment resources;
 *     - deployment targets;
 *     - deployment placement syntax;
 *     - deployment policies;
 *     - deployment lifecycle;
 *     - deployment rollout;
 *     - deployment availability;
 *     - deployment portability;
 *     - deployment recovery;
 *     - deployment observability;
 *     - deployment parameters.
 *
 * Those remain owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * ============================================================================
 * WHY THIS FILE MUST BE SMALL
 * ============================================================================
 *
 * A common architectural error would be to create:
 *
 *     execution/deployment.g4
 *     distributed/deployment.g4
 *     hardware/deployment.g4
 *
 * and then copy the same deployment rules into each file.
 *
 * That would create multiple deployment authorities.
 *
 * Instead:
 *
 *     execution/deployment.g4
 *             |
 *             +--> canonical generic deployment syntax
 *
 *     distributed/deployment.g4
 *             |
 *             +--> distributed composition boundary
 *
 *     hardware/deployment.g4
 *             |
 *             +--> hardware deployment composition boundary
 *
 * All three ultimately describe one semantic deployment model.
 *
 * ============================================================================
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Lexical authority:
 *
 *     ZamaniLexer
 *
 * Generic names and expressions:
 *
 *     grammar/execution/deployment.g4
 *     grammar/core/
 *     grammar/expressions/
 *
 * Generic deployment authority:
 *
 *     grammar/execution/deployment.g4
 *
 * Distributed placement authority:
 *
 *     grammar/distributed/placement.g4
 *
 * Generic placement/resource authority:
 *
 *     grammar/resources/placement.g4
 *
 * Distributed topology authority:
 *
 *     grammar/distributed/topology.g4
 *
 * Distributed replication authority:
 *
 *     grammar/distributed/replication.g4
 *
 * Distributed consistency authority:
 *
 *     grammar/distributed/consistency.g4
 *
 * Hardware deployment authority:
 *
 *     grammar/hardware/deployment.g4
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * This file MUST NOT become an authority for any of the above systems.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Core / Expressions
 *          |
 *          v
 *     Execution Deployment
 *          |
 *          v
 *     Distributed Deployment
 *
 * Therefore this grammar imports:
 *
 *     Deployment
 *
 * and reuses its canonical deployment rules.
 *
 * This grammar MUST NOT redefine:
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
 * INTEGRATION WITH EXECUTION/DEPLOYMENT.G4
 * ============================================================================
 *
 * `grammar/execution/deployment.g4` is the canonical owner of:
 *
 *     deploymentDeclaration
 *
 * and its complete deployment body.
 *
 * This grammar delegates to that model.
 *
 * The distributed parser therefore does NOT introduce a second version of:
 *
 *     deploy subject { ... }
 *
 * Instead, the distributed entry point identifies the deployment as a
 * distributed-domain construct and delegates the actual deployment syntax
 * to the canonical deployment grammar.
 *
 * ============================================================================
 * DISTRIBUTED SUBJECT
 * ============================================================================
 *
 * A distributed deployment subject is intentionally an expression.
 *
 * This permits subjects such as:
 *
 *     distributed::service
 *     distributed::service::api
 *     distributed::task
 *     distributed::worker
 *     distributed::actor
 *     distributed::pipeline
 *     distributed::computation
 *     distributed::future
 *     distributed::quantum_workload
 *     distributed::hybrid_workload
 *
 * without enumerating a closed set of distributed entity kinds.
 *
 * Semantic analysis determines whether the expression actually denotes a
 * deployable distributed entity.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     cluster
 *     node
 *     service
 *     worker
 *     actor
 *     pod
 *     container
 *     VM
 *     process
 *     job
 *     function
 *     task
 *     quantum service
 *     accelerator
 *     cloud
 *     edge
 *     HPC
 *
 * as separate deployment grammar alternatives.
 *
 * Those are semantic concepts.
 *
 * Future distributed deployment entities remain syntactically representable
 * through the canonical expression/name system.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed deployment participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program expresses deployment intent.
 *
 * It MUST NOT require source rewriting merely because the program moves
 * between:
 *
 *     - one machine;
 *     - many machines;
 *     - embedded systems;
 *     - edge systems;
 *     - clusters;
 *     - HPC systems;
 *     - supercomputers;
 *     - clouds;
 *     - federated environments;
 *     - heterogeneous CPU/GPU/FPGA systems;
 *     - quantum-classical systems;
 *     - distributed quantum systems;
 *     - future computational substrates.
 *
 * Deployment realization remains downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no artificial deployment limits.
 *
 * It defines no:
 *
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_SERVICES
 *     MAX_WORKERS
 *     MAX_ACTORS
 *     MAX_TASKS
 *     MAX_INSTANCES
 *     MAX_REPLICAS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NETWORK_SIZE
 *     MAX_DEPLOYMENTS
 *     MAX_RESOURCES
 *     MAX_REGIONS
 *
 * Nor does it impose:
 *
 *     fixed node counts;
 *     fixed process counts;
 *     fixed service counts;
 *     fixed cluster sizes;
 *     fixed machine sizes;
 *     fixed memory sizes;
 *     fixed CPU/GPU/FPGA/QPU counts;
 *     fixed topology sizes;
 *     fixed network sizes.
 *
 * Any quantities used by deployment semantics are expressions owned by the
 * canonical deployment grammar and interpreted downstream.
 *
 * "Infinity" means that the language grammar introduces no artificial finite
 * ceiling. Actual execution remains bounded by:
 *
 *     - available resources;
 *     - semantic requirements;
 *     - compiler implementation limits;
 *     - runtime limits;
 *     - operating-system limits;
 *     - target capabilities;
 *     - explicitly declared program constraints.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Deployment must preserve the semantic distinction between:
 *
 *     REQUIREMENT
 *         Must be satisfied.
 *
 *     CONSTRAINT
 *         Restricts valid realizations.
 *
 *     PREFERENCE
 *         Desired realization but not mandatory.
 *
 *     HINT
 *         Advisory information.
 *
 *     CAPABILITY
 *         A capability that must be available.
 *
 *     RESOURCE
 *         A resource requirement or description.
 *
 *     TARGET
 *         An abstract target expression.
 *
 *     PLACEMENT
 *         Placement intent.
 *
 * This file does not reinterpret those categories.
 *
 * ============================================================================
 * NO PHYSICAL MACHINE ASSUMPTIONS
 * ============================================================================
 *
 * Deployment syntax MUST NOT require:
 *
 *     physical node identifiers;
 *     physical CPU identifiers;
 *     GPU identifiers;
 *     FPGA coordinates;
 *     QPU identifiers;
 *     physical qubit identifiers;
 *     memory addresses;
 *     fixed hostnames;
 *     fixed IP addresses;
 *     fixed ports;
 *     fixed cloud regions;
 *     fixed providers;
 *     fixed accelerator counts;
 *     fixed cluster topology.
 *
 * Such information, where semantically necessary, must remain explicit
 * program data or downstream realization metadata rather than becoming
 * universal grammar constraints.
 *
 * ============================================================================
 * DISTRIBUTED PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Deployment may contain placement intent through the canonical deployment
 * placement clause.
 *
 * Distributed placement remains owned by:
 *
 *     grammar/distributed/placement.g4
 *
 * Generic resource placement remains owned by:
 *
 *     grammar/resources/placement.g4
 *
 * This file MUST NOT reproduce placement rules.
 *
 * ============================================================================
 * DISTRIBUTED TOPOLOGY INTEGRATION
 * ============================================================================
 *
 * Deployment may refer semantically to a topology declaration or topology
 * requirement.
 *
 * Topology syntax remains owned by:
 *
 *     grammar/distributed/topology.g4
 *
 * This grammar does not:
 *
 *     - construct topology graphs;
 *     - select physical links;
 *     - perform routing;
 *     - select transport protocols;
 *     - resolve hardware topology.
 *
 * ============================================================================
 * DISTRIBUTED REPLICATION INTEGRATION
 * ============================================================================
 *
 * Deployment may deploy a logically replicated distributed entity.
 *
 * Replication syntax remains owned by:
 *
 *     grammar/distributed/replication.g4
 *
 * This grammar does not define:
 *
 *     replica counts;
 *     replication algorithms;
 *     replica placement algorithms;
 *     synchronization;
 *     consistency;
 *     failover;
 *     storage replication.
 *
 * Those concerns remain downstream.
 *
 * ============================================================================
 * DISTRIBUTED CONSISTENCY INTEGRATION
 * ============================================================================
 *
 * Deployment may refer to a consistency policy through the canonical
 * deployment policy/property mechanisms.
 *
 * Consistency syntax remains owned by:
 *
 *     grammar/distributed/consistency.g4
 *
 * This file does not define:
 *
 *     consensus algorithms;
 *     consistency algorithms;
 *     quorum algorithms;
 *     ordering algorithms;
 *     distributed transactions.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Deployment may require distributed communication capabilities.
 *
 * It MUST NOT select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     Ethernet
 *     wireless transport
 *
 * as universal deployment grammar alternatives.
 *
 * Networking realization remains downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A distributed deployment subject may denote a quantum or hybrid workload.
 *
 * Examples include semantic subjects such as:
 *
 *     distributed::quantum_workload
 *     distributed::hybrid_workload
 *     distributed::quantum_service
 *
 * The grammar does not enumerate those names.
 *
 * Quantum semantics continue through:
 *
 *     quantum source
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / QEC / ZQN
 *          |
 *          v
 *     HAL
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     pulse semantics
 *     QEC algorithms
 *     ZQN models
 *     quantum::ir.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A deployment subject may denote a hardware/software computation or
 * accelerator workload.
 *
 * Hardware deployment intent remains owned by:
 *
 *     grammar/hardware/deployment.g4
 *
 * Hardware realization remains downstream.
 *
 * This file MUST NOT define:
 *
 *     physical FPGA coordinates;
 *     ASIC placement;
 *     physical register widths;
 *     memory addresses;
 *     device inventory;
 *     physical pins;
 *     vendor-specific deployment primitives.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Deployment may carry security requirements through the canonical deployment
 * clauses.
 *
 * This grammar does not implement:
 *
 *     authentication;
 *     authorization;
 *     encryption;
 *     attestation;
 *     secret storage;
 *     key management.
 *
 * Security semantics remain downstream.
 *
 * ============================================================================
 * RESILIENCE / RECOVERY INTEGRATION
 * ============================================================================
 *
 * Deployment may express recovery and availability intent through the
 * canonical deployment grammar.
 *
 * It does not implement recovery.
 *
 * The distributed semantic/runtime layers determine the realization.
 *
 * This remains compatible with the resilience state vocabulary:
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
 * and outcomes:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * Those are semantic/runtime concepts, not parser-level deployment
 * algorithms.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT introduce a second deployment AST.
 *
 * The parser tree must provide enough structure for the existing
 * domain-neutral frontend AST to represent:
 *
 *     deployment;
 *     deployment subject;
 *     deployment clauses;
 *     deployment expressions;
 *     source ordering;
 *     source spans.
 *
 * Conceptually:
 *
 *     DistributedDeployment
 *         subject
 *         deployment_body
 *         source_span
 *
 * The body is the canonical Deployment body.
 *
 * No distributed-specific duplicate of DeploymentClause should be created.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the subject is deployable;
 *     - whether the subject is distributed;
 *     - whether distributed capabilities are satisfied;
 *     - whether resource requirements are satisfiable;
 *     - whether placement intent is satisfiable;
 *     - whether topology requirements are satisfiable;
 *     - whether replication/consistency policies are compatible;
 *     - whether security requirements are satisfied;
 *     - whether quantum/hybrid requirements are satisfied;
 *     - whether hardware requirements are satisfiable;
 *     - whether the deployment is portable;
 *     - whether constraints conflict;
 *     - whether preferences remain advisory;
 *     - whether hints remain advisory;
 *     - whether the target environment can realize the deployment.
 *
 * The parser performs none of these decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Deployment information follows the existing semantic pipeline:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic deployment model
 *       |
 *       +--> resource requirements
 *       +--> capability requirements
 *       +--> distributed metadata
 *       +--> placement intent
 *       +--> topology intent
 *       |
 *       v
 *     canonical IR / semantic representations
 *       |
 *       v
 *     optimization
 *       |
 *       +--> placement
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       |
 *       v
 *     target realization
 *
 * Quantum workloads continue through:
 *
 *     quantum::ir
 *
 * There must be no:
 *
 *     DistributedDeploymentIR
 *
 * introduced merely because deployment syntax originated in this directory.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parser/AST integration must preserve:
 *
 *     - deployment subject structure;
 *     - deployment clause ordering;
 *     - nested deployment body structure;
 *     - expression ordering;
 *     - source spans;
 *     - qualified-name segment ordering.
 *
 * Semantic normalization occurs downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no randomness;
 *     - no mutable global state.
 *
 * Parsing depends only on:
 *
 *     - the supplied token stream;
 *     - the imported grammar contracts;
 *     - the selected language version.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     - malformed deployment subject;
 *     - malformed deployment body;
 *     - invalid deployment clause structure;
 *     - missing deployment delimiters;
 *     - malformed expressions.
 *
 * Semantic diagnostics belong downstream and should distinguish:
 *
 *     - unsatisfied requirement;
 *     - violated constraint;
 *     - unavailable capability;
 *     - unavailable resource;
 *     - unsatisfied placement;
 *     - unsatisfied topology;
 *     - incompatible replication;
 *     - incompatible consistency;
 *     - unavailable target;
 *     - security-policy failure.
 *
 * The parser MUST NOT report a runtime resource shortage as a syntax error.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing generic deployment syntax remains authoritative.
 *
 * This file therefore does not change the syntax of:
 *
 *     grammar/execution/deployment.g4
 *
 * A future evolution of generic deployment syntax should occur in that
 * grammar and its specification/compatibility contracts.
 *
 * This file should only change when the distributed composition boundary
 * itself changes.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     - no machine capacity constants;
 *     - no node-count constants;
 *     - no device-count constants;
 *     - no fixed topology;
 *     - no fixed provider;
 *     - no fixed transport;
 *     - no physical hardware IDs;
 *     - no fixed quantum limits;
 *     - no fixed memory limits;
 *     - no fixed register limits;
 *     - no fixed deployment cardinality.
 *
 * All quantities are delegated to:
 *
 *     expressions;
 *     semantic requirements;
 *     resource analysis;
 *     capability analysis;
 *     target realization.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * This grammar intentionally contains only a small number of delegation
 * rules.
 *
 * It does not duplicate the complete deployment grammar.
 *
 * This minimizes:
 *
 *     - grammar size;
 *     - generated parser duplication;
 *     - ambiguity;
 *     - maintenance cost;
 *     - parser divergence.
 *
 * No semantic processing is performed by the grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The grammar has no:
 *
 *     - filesystem access;
 *     - network access;
 *     - shell execution;
 *     - dynamic code execution;
 *     - Rust actions;
 *     - unsafe code;
 *     - runtime callbacks.
 *
 * Deployment syntax is declarative.
 *
 * Actual deployment authorization and security policy remain downstream.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 *     [ ] It imports the canonical Deployment grammar.
 *     [ ] It does not duplicate generic deployment rules.
 *     [ ] It exposes one distributed deployment entry point.
 *     [ ] It accepts canonical deployment subjects.
 *     [ ] It preserves canonical deployment-body semantics.
 *     [ ] It introduces no hardware/resource limits.
 *     [ ] It introduces no fixed topology.
 *     [ ] It introduces no vendor-specific deployment syntax.
 *     [ ] It preserves POCO-REAF.
 *     [ ] It integrates with distributed.g4.
 *     [ ] It integrates with execution/deployment.g4.
 *     [ ] It integrates with distributed/placement.g4.
 *     [ ] It integrates with distributed/topology.g4.
 *     [ ] It integrates with distributed/replication.g4.
 *     [ ] It integrates with distributed/consistency.g4.
 *     [ ] It remains compatible with quantum::ir.
 *     [ ] It passes positive syntax tests.
 *     [ ] It passes negative syntax tests.
 *     [ ] It passes scalability tests.
 *     [ ] It passes determinism checks.
 *     [ ] It passes hard-coding validation.
 *
 * ============================================================================
 */

parser grammar DistributedDeployment;

options {
    tokenVocab = ZamaniLexer;
}

import Deployment;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only rule that distributed.g4 should invoke for distributed
 * deployment syntax.
 *
 * The actual deployment syntax is delegated to the canonical Deployment
 * grammar.
 */
distributedDeploymentDeclaration
    : deploymentDeclaration
    ;


/*
 * ============================================================================
 * DISTRIBUTED DEPLOYMENT SUBJECT
 * ============================================================================
 *
 * This wrapper exists as the semantic composition boundary.
 *
 * The canonical Deployment grammar intentionally accepts any expression as a
 * deployment subject. Distributed semantic analysis determines whether that
 * expression denotes a distributed entity.
 *
 * This grammar therefore does not enumerate:
 *
 *     node
 *     service
 *     worker
 *     actor
 *     cluster
 *     process
 *     task
 *     quantum workload
 *     hybrid workload
 *
 * Those are semantic names.
 *
 * ============================================================================
 */
distributedDeploymentSubject
    : deploymentSubject
    ;