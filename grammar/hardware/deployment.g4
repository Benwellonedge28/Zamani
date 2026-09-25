/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/deployment.g4
 *
 * Grammar:
 *     HardwareDeployment
 *
 * Status:
 *     Production-ready hardware-deployment intent leaf grammar
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL HARDWARE DEPLOYMENT INTENT.
 *
 * It describes the relationship between an abstract Zamani hardware
 * declaration and a deployment realization contract.
 *
 * This grammar is intentionally narrower than:
 *
 *     grammar/execution/deployment.g4
 *
 * Execution deployment owns general deployment of a computation.
 *
 * This file owns hardware-specific deployment intent only.
 *
 * The distinction is:
 *
 *     execution/deployment.g4
 *         WHAT deployment of a computation means.
 *
 *     hardware/deployment.g4
 *         WHAT hardware realization properties a deployment requires,
 *         prefers, permits, or describes.
 *
 * Neither grammar performs deployment.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         canonical lexer
 *                              |
 *                              v
 *                         canonical parser
 *                              |
 *               +--------------+---------------+
 *               |                              |
 *               v                              v
 *        execution deployment          hardware deployment
 *               |                              |
 *               |                              |
 *               +--------------+---------------+
 *                              |
 *                              v
 *                       domain-neutral AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *         capabilities     resources        targets
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                    hardware semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *          placement        routing         scheduling
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                         HAL / lowering
 *                              |
 *                              v
 *                         realization
 *
 * Quantum programs continue through:
 *
 *     quantum source
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization / routing / scheduling / QEC / ZQN
 *         |
 *         v
 *     HAL
 *
 * This file MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hardware deployment declaration syntax;
 *     - hardware deployment subject syntax;
 *     - hardware deployment contract bodies;
 *     - hardware realization requirements;
 *     - hardware realization constraints;
 *     - hardware realization preferences;
 *     - hardware realization hints;
 *     - hardware capability requirements;
 *     - hardware resource requirements;
 *     - abstract target requirements;
 *     - deployment placement intent at the hardware-contract layer;
 *     - deployment topology intent at the hardware-contract layer;
 *     - deployment environment properties;
 *     - deployment availability intent;
 *     - deployment lifecycle intent;
 *     - deployment rollout intent;
 *     - deployment recovery intent;
 *     - deployment observability intent;
 *     - deployment artifact references;
 *     - deployment parameters;
 *     - open-ended hardware deployment properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expression syntax;
 *     - general type syntax;
 *     - generic types;
 *     - execution deployment semantics;
 *     - resource discovery;
 *     - resource allocation;
 *     - hardware discovery;
 *     - target selection algorithms;
 *     - placement algorithms;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - synthesis;
 *     - calibration;
 *     - physical device allocation;
 *     - device drivers;
 *     - provider APIs;
 *     - cloud orchestration;
 *     - authentication;
 *     - authorization;
 *     - QEC;
 *     - ZQN;
 *     - quantum operations;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This grammar describes:
 *
 *     HARDWARE DEPLOYMENT INTENT
 *
 * It does NOT perform:
 *
 *     HARDWARE DEPLOYMENT
 *
 * Therefore:
 *
 *     deploy hardware::accelerator {
 *         ...
 *     }
 *
 * is a source-level contract.
 *
 * It does NOT:
 *
 *     - discover a device;
 *     - reserve a device;
 *     - allocate memory;
 *     - select a GPU;
 *     - select a QPU;
 *     - select a CPU;
 *     - contact a cloud;
 *     - start a process;
 *     - program an FPGA;
 *     - synthesize an ASIC;
 *     - open a hardware connection.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * The canonical parser architecture is:
 *
 *     ZamaniParser
 *         |
 *         v
 *     Hardware
 *         |
 *         v
 *     HardwareDeployment
 *
 * This grammar consumes canonical Core grammar concepts:
 *
 *     expression
 *     qualifiedName
 *
 * It does not recreate:
 *
 *     identifier syntax;
 *     qualified-name syntax;
 *     expression precedence;
 *     literal syntax.
 *
 * The canonical lexical vocabulary remains:
 *
 *     ZamaniTokens
 *
 * No local lexer rules are permitted.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE.G4
 * ============================================================================
 *
 * hardware.g4 owns the hardware composition boundary.
 *
 * hardware.g4 MUST delegate deployment through exactly one rule:
 *
 *     hardwareDeploymentDeclaration
 *
 * The hardware item dispatcher should contain:
 *
 *     | hardwareDeploymentDeclaration
 *
 * and MUST NOT reproduce the deployment productions contained in this file.
 *
 * This file therefore becomes the sole owner of hardware deployment syntax.
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION/DEPLOYMENT.G4
 * ============================================================================
 *
 * grammar/execution/deployment.g4 remains authoritative for general
 * computation deployment.
 *
 * It owns:
 *
 *     deploymentDeclaration
 *     deploymentSubject
 *     deploymentBody
 *     deploymentRequirement
 *     deploymentConstraint
 *     deploymentPreference
 *     deploymentCapability
 *     deploymentResource
 *     deploymentTarget
 *     deploymentPlacement
 *     deploymentLifecycle
 *     deploymentRollout
 *     deploymentAvailability
 *     deploymentRecovery
 *     deploymentObservability
 *     deploymentArtifact
 *     deploymentParameter
 *
 * This file MUST NOT duplicate those rule names.
 *
 * Instead, hardware deployment is a hardware-domain specialization that can
 * be represented semantically alongside the general deployment model.
 *
 * The semantic layer is responsible for merging:
 *
 *     execution deployment intent
 *              +
 *     hardware deployment intent
 *
 * into one deployment/realization contract.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE TARGETS
 * ============================================================================
 *
 * hardware/targets.g4 remains authoritative for target declarations.
 *
 * This file may reference target expressions.
 *
 * It MUST NOT duplicate target declaration syntax.
 *
 * A target here means:
 *
 *     target intent
 *
 * not:
 *
 *     physical device identity.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE PLACEMENT
 * ============================================================================
 *
 * hardware/placement.g4 remains authoritative for placement declarations.
 *
 * This file expresses placement requirements/preferences as deployment
 * properties and symbolic references.
 *
 * It MUST NOT implement placement algorithms.
 *
 * It MUST NOT introduce:
 *
 *     physical coordinates;
 *     device handles;
 *     routing paths;
 *     physical qubit IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     FPGA coordinates.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE RESOURCES
 * ============================================================================
 *
 * hardware/resources.g4 and grammar/resources/ remain authoritative for
 * resource semantics.
 *
 * This file references resources.
 *
 * It does not allocate them.
 *
 * Example:
 *
 *     resource memory >= required_memory;
 *
 * means:
 *
 *     the realization must satisfy the semantic requirement.
 *
 * It does NOT mean:
 *
 *     allocate memory now.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE CAPABILITIES
 * ============================================================================
 *
 * hardware/capabilities.g4 remains authoritative for capability declarations.
 *
 * This file may require capabilities:
 *
 *     capability quantum.measurement;
 *
 *     capability tensor.compute;
 *
 *     capability programmable_logic;
 *
 * The vocabulary is open.
 *
 * No closed enumeration of current hardware capabilities is permitted.
 *
 * ============================================================================
 * INTEGRATION WITH TOPOLOGY
 * ============================================================================
 *
 * hardware/topology.g4 remains authoritative for topology declarations and
 * topology contracts.
 *
 * This file may reference topology requirements through:
 *
 *     topology.connectivity >= required_connectivity;
 *
 * or:
 *
 *     topology: required_topology;
 *
 * It MUST NOT implement graph discovery or routing.
 *
 * ============================================================================
 * INTEGRATION WITH NEGOTIATION
 * ============================================================================
 *
 * hardware/negotiation.g4 owns general hardware capability/resource
 * negotiation constructs.
 *
 * Deployment may contain negotiation-related properties, but it MUST NOT
 * duplicate the complete negotiation language.
 *
 * If a deployment property requires alternative realization selection, the
 * semantic layer should normalize it into the existing negotiation model.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILATION
 * ============================================================================
 *
 * Compilation is downstream.
 *
 * Deployment intent may be consumed by:
 *
 *     compilation target resolution;
 *     resource analysis;
 *     optimization;
 *     placement;
 *     routing;
 *     scheduling;
 *     synthesis;
 *     HAL lowering.
 *
 * This grammar MUST NOT select a compiler backend.
 *
 * ============================================================================
 * INTEGRATION WITH RUNTIME
 * ============================================================================
 *
 * Runtime consumes semantic deployment plans.
 *
 * This grammar does not:
 *
 *     - dispatch;
 *     - submit;
 *     - allocate;
 *     - launch;
 *     - migrate;
 *     - restart;
 *     - communicate with providers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A hardware deployment declaration MUST describe portable realization
 * intent.
 *
 * It MUST NOT require the source program to be rewritten merely because the
 * available machine changes.
 *
 * Valid:
 *
 *     deploy computation {
 *         target: quantum;
 *         capability: quantum.measurement;
 *         resource: qubits >= required_qubits;
 *     }
 *
 * Also valid:
 *
 *     deploy computation {
 *         target: heterogeneous;
 *         capability: tensor.compute;
 *         resource: memory >= required_memory;
 *     }
 *
 * The following are NOT universal hardware semantics:
 *
 *     use_gpu_0
 *     cpu_core_7
 *     qpu_3
 *     physical_qubit_17
 *     memory_bank_2
 *     node_42
 *
 * If target-specific realization is genuinely required, it must be
 * represented as semantic data and resolved downstream under an explicit
 * target/deployment contract.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO language-level hardware/deployment capacity.
 *
 * It contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_NETWORK_SIZE
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_REPLICAS
 *     MAX_INSTANCES
 *     MAX_REGIONS
 *     MAX_ARTIFACTS
 *     MAX_TARGETS
 *
 * Collections use ANTLR repetition.
 *
 * Quantities are expressions.
 *
 * Therefore the grammar remains usable for:
 *
 *     tiny embedded systems;
 *     single CPUs;
 *     multicore CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     accelerators;
 *     clusters;
 *     HPC systems;
 *     distributed systems;
 *     cloud systems;
 *     heterogeneous systems;
 *     future architectures.
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level hardware ceiling.
 *
 * It does NOT claim infinite physical resources.
 *
 * Actual limits are determined by:
 *
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     deployment policy;
 *     operating environment;
 *     physical resources.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no predicates;
 *     - no embedded Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no environment inspection.
 *
 * Identical source token streams under the same grammar version produce the
 * same parse structure.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * Downstream implementation requirements:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar HardwareDeployment;

options {
    tokenVocab = ZamaniTokens;
}

import Core;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * hardware.g4 consumes exactly this rule.
 *
 * It is intentionally hardware-scoped so it cannot collide with:
 *
 *     executionDeployment
 *     deploymentDeclaration
 *
 * from grammar/execution/deployment.g4.
 * ============================================================================
 */

hardwareDeploymentDeclaration
    : DEPLOY
      hardwareDeploymentSubject
      hardwareDeploymentBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * DEPLOYMENT SUBJECT
 * ============================================================================
 *
 * The subject is an expression so deployment may refer to:
 *
 *     a hardware declaration;
 *     a hardware expression;
 *     a computation;
 *     a symbolic target;
 *     a generic hardware realization object;
 *     a future semantic object.
 *
 * No closed target list is required.
 * ============================================================================
 */

hardwareDeploymentSubject
    : expression
    ;


/*
 * ============================================================================
 * DEPLOYMENT BODY
 * ============================================================================
 *
 * A body contains one or more deployment clauses.
 *
 * An empty body is deliberately not accepted because:
 *
 *     deploy subject {}
 *
 * carries no hardware deployment information and is more likely to indicate
 * an accidental source error.
 * ============================================================================
 */

hardwareDeploymentBody
    : LBRACE
      hardwareDeploymentClause+
      RBRACE
    ;


/*
 * ============================================================================
 * CLAUSE DISPATCH
 * ============================================================================
 */

hardwareDeploymentClause
    : hardwareDeploymentRequirement
    | hardwareDeploymentConstraint
    | hardwareDeploymentPreference
    | hardwareDeploymentHint
    | hardwareDeploymentCapability
    | hardwareDeploymentResource
    | hardwareDeploymentTarget
    | hardwareDeploymentPlacement
    | hardwareDeploymentTopology
    | hardwareDeploymentEnvironment
    | hardwareDeploymentAvailability
    | hardwareDeploymentLifecycle
    | hardwareDeploymentRollout
    | hardwareDeploymentRecovery
    | hardwareDeploymentObservability
    | hardwareDeploymentArtifact
    | hardwareDeploymentParameter
    | hardwareDeploymentProperty
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * The expression remains open-ended.
 *
 * Examples:
 *
 *     requires qubits >= required_qubits;
 *     requires capability.quantum.measurement;
 *     requires memory >= required_memory;
 *
 * Semantic analysis determines satisfiability.
 * ============================================================================
 */

hardwareDeploymentRequirement
    : REQUIRES
      expression
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts acceptable hardware realizations.
 *
 * It does not allocate hardware.
 * ============================================================================
 */

hardwareDeploymentConstraint
    : CONSTRAINT
      hardwareDeploymentReference
      hardwareDeploymentComparisonOperator
      expression
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * Failure to satisfy a preference must not automatically invalidate the
 * program's semantics.
 * ============================================================================
 */

hardwareDeploymentPreference
    : PREFER
      expression
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Hints are advisory information for downstream realization.
 *
 * A hint must not silently become a mandatory requirement.
 * ============================================================================
 */

hardwareDeploymentHint
    : HINT
      expression
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Capability vocabulary is open.
 *
 * Examples:
 *
 *     capability quantum.measurement;
 *     capability tensor.compute;
 *     capability programmable_logic;
 *     capability future.domain.feature;
 *
 * No current hardware inventory is encoded here.
 * ============================================================================
 */

hardwareDeploymentCapability
    : CAPABILITY
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * RESOURCE
 * ============================================================================
 *
 * Resource expressions remain symbolic.
 *
 * Examples:
 *
 *     resource qubits >= required_qubits;
 *     resource memory >= required_memory;
 *     resource bandwidth >= required_bandwidth;
 *
 * ============================================================================
 */

hardwareDeploymentResource
    : RESOURCE
      hardwareDeploymentReference
      hardwareDeploymentComparisonOperator?
      expression?
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * Target is an abstract realization class.
 *
 * It is not a physical device.
 * ============================================================================
 */

hardwareDeploymentTarget
    : TARGET
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * PLACEMENT
 * ============================================================================
 *
 * Placement is intent.
 *
 * Physical placement is downstream.
 * ============================================================================
 */

hardwareDeploymentPlacement
    : hardwareDeploymentKeywordClause
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * TOPOLOGY
 * ============================================================================
 *
 * Topology is represented as an open semantic property.
 *
 * Routing and physical topology discovery remain downstream.
 * ============================================================================
 */

hardwareDeploymentTopology
    : hardwareDeploymentKeywordReference
      hardwareDeploymentTopologyValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * ENVIRONMENT
 * ============================================================================
 *
 * Environment is deliberately open-ended.
 *
 * The grammar does not enumerate:
 *
 *     local
 *     cloud
 *     edge
 *     embedded
 *     cluster
 *     quantum
 *     simulator
 *
 * Those are semantic values.
 * ============================================================================
 */

hardwareDeploymentEnvironment
    : hardwareDeploymentKeywordReference
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * AVAILABILITY
 * ============================================================================
 *
 * Availability describes desired deployment availability semantics.
 *
 * It does not discover whether the target is currently available.
 * ============================================================================
 */

hardwareDeploymentAvailability
    : AVAILABILITY
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 *
 * Lifecycle is represented through an open property namespace.
 *
 * This avoids creating a permanent keyword for every future lifecycle state.
 * ============================================================================
 */

hardwareDeploymentLifecycle
    : hardwareDeploymentNamedClause
    ;


/*
 * ============================================================================
 * ROLLOUT
 * ============================================================================
 *
 * Rollout remains semantic data.
 * ============================================================================
 */

hardwareDeploymentRollout
    : hardwareDeploymentNamedClause
    ;


/*
 * ============================================================================
 * RECOVERY
 * ============================================================================
 *
 * Recovery intent is declarative.
 *
 * Recovery algorithms remain downstream.
 * ============================================================================
 */

hardwareDeploymentRecovery
    : hardwareDeploymentNamedClause
    ;


/*
 * ============================================================================
 * OBSERVABILITY
 * ============================================================================
 *
 * Observability describes desired visibility/telemetry properties.
 *
 * It does not implement telemetry.
 * ============================================================================
 */

hardwareDeploymentObservability
    : OBSERVE
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * ARTIFACT
 * ============================================================================
 *
 * An artifact is a semantic reference.
 *
 * Storage, hashing, signing, retrieval and verification belong elsewhere.
 * ============================================================================
 */

hardwareDeploymentArtifact
    : hardwareDeploymentKeywordReference
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * PARAMETER
 * ============================================================================
 *
 * Deployment parameters are semantic values.
 * ============================================================================
 */

hardwareDeploymentParameter
    : hardwareDeploymentReference
      hardwareDeploymentAssignmentOperator
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * OPEN PROPERTY
 * ============================================================================
 *
 * This is the principal forward-compatibility mechanism.
 *
 * Future deployment properties should normally use:
 *
 *     qualified.name: expression;
 *
 * rather than requiring a new permanent keyword.
 *
 * Examples:
 *
 *     thermal.margin: required_margin;
 *     power.energy: energy_budget;
 *     reliability.availability: required_availability;
 *     vendor.future_property: value;
 *
 * ============================================================================
 */

hardwareDeploymentProperty
    : hardwareDeploymentReference
      hardwareDeploymentAssignmentOperator
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * NAMED OPEN CLAUSES
 * ============================================================================
 *
 * These clauses intentionally use qualified names instead of introducing
 * permanent keywords for every lifecycle, rollout, recovery or future
 * deployment concept.
 * ============================================================================
 */

hardwareDeploymentNamedClause
    : hardwareDeploymentReference
      hardwareDeploymentValue
      hardwareDeploymentTerminator
    ;


/*
 * ============================================================================
 * KEYWORD-BASED OPEN CLAUSE
 * ============================================================================
 *
 * A reserved keyword can be followed by a semantic value.
 *
 * The keyword itself is consumed by the surrounding grammar.
 * ============================================================================
 */

hardwareDeploymentKeywordClause
    : hardwareDeploymentReference
    ;


/*
 * ============================================================================
 * KEYWORD-REFERENCE ADAPTER
 * ============================================================================
 *
 * This rule exists as a named integration point.
 *
 * The actual semantic namespace remains open.
 * ============================================================================
 */

hardwareDeploymentKeywordReference
    : hardwareDeploymentReference
    ;


/*
 * ============================================================================
 * DEPLOYMENT REFERENCE
 * ============================================================================
 *
 * References use canonical qualified names.
 *
 * There is no fixed namespace depth.
 * ============================================================================
 */

hardwareDeploymentReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * DEPLOYMENT VALUE
 * ============================================================================
 *
 * Values reuse canonical expressions.
 *
 * Structured objects and lists are provided only as deployment data
 * structures; they do not constitute a second expression language.
 * ============================================================================
 */

hardwareDeploymentValue
    : expression
    | hardwareDeploymentObject
    | hardwareDeploymentList
    ;


/*
 * ============================================================================
 * OBJECT
 * ============================================================================
 *
 * Objects contain arbitrary deployment properties.
 *
 * ============================================================================
 */

hardwareDeploymentObject
    : LBRACE
      hardwareDeploymentObjectEntry+
      RBRACE
    ;


hardwareDeploymentObjectEntry
    : hardwareDeploymentReference
      hardwareDeploymentAssignmentOperator
      hardwareDeploymentValue
      hardwareDeploymentTerminator?
    ;


/*
 * ============================================================================
 * LIST
 * ============================================================================
 *
 * Lists are unbounded by grammar.
 * ============================================================================
 */

hardwareDeploymentList
    : LBRACKET
      hardwareDeploymentListElements?
      RBRACKET
    ;


hardwareDeploymentListElements
    : hardwareDeploymentValue
      (
          COMMA
          hardwareDeploymentValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * TOPOLOGY VALUE
 * ============================================================================
 *
 * Kept separate as an explicit semantic integration point.
 *
 * The syntax remains the canonical value language.
 * ============================================================================
 */

hardwareDeploymentTopologyValue
    : hardwareDeploymentValue
    ;


/*
 * ============================================================================
 * ASSIGNMENT OPERATORS
 * ============================================================================
 *
 * Both declarative and assignment forms are accepted.
 *
 * Examples:
 *
 *     target: quantum;
 *     target = quantum;
 * ============================================================================
 */

hardwareDeploymentAssignmentOperator
    : COLON
    | ASSIGN
    ;


/*
 * ============================================================================
 * COMPARISON OPERATORS
 * ============================================================================
 *
 * These are the canonical lexical operator names used by the Zamani
 * expression/constraint vocabulary.
 * ============================================================================
 */

hardwareDeploymentComparisonOperator
    : EQ_EQ
    | NOT_EQ
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * TERMINATOR
 * ============================================================================
 */

hardwareDeploymentTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve at least:
 *
 *     deployment subject
 *     clause order
 *     clause kind
 *     source spans
 *     expressions
 *     qualified names
 *     comparison operator
 *     object/list nesting
 *     source spelling where required
 *
 * Recommended conceptual representation:
 *
 *     HardwareDeployment
 *         subject
 *         clauses[]
 *         source_span
 *
 *     HardwareDeploymentClause
 *         kind
 *         key
 *         value
 *         operator
 *         source_span
 *
 * The exact Rust AST type names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar must not define or assume concrete Rust AST structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST preserve these distinctions:
 *
 *     requirement
 *         != constraint
 *
 *     constraint
 *         != preference
 *
 *     preference
 *         != hint
 *
 *     capability
 *         != resource
 *
 *     target
 *         != physical device
 *
 *     placement
 *         != physical allocation
 *
 *     topology
 *         != routing
 *
 *     availability
 *         != resource discovery
 *
 *     recovery
 *         != recovery implementation
 *
 *     observability
 *         != telemetry implementation
 *
 * A failed mandatory requirement must remain a semantic failure.
 *
 * A failed preference must not silently become a semantic failure unless the
 * semantic specification explicitly defines that behavior.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions must remain symbolic until resource analysis.
 *
 * Examples:
 *
 *     required_memory
 *     input.size * element_size
 *     required_qubits
 *     problem.size
 *     workload / duration
 *
 * No parser-level numeric capacity is imposed.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability names remain open.
 *
 * Examples:
 *
 *     quantum.measurement
 *     quantum.dynamic_control
 *     tensor.compute
 *     programmable_logic
 *     secure_execution
 *     future.domain.capability
 *
 * The parser does not determine whether the capability exists.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * Target values are symbolic.
 *
 * The grammar MUST NOT require:
 *
 *     physical device ID;
 *     vendor ID;
 *     PCI address;
 *     CPU core ID;
 *     GPU ID;
 *     FPGA coordinate;
 *     QPU ID;
 *     physical qubit ID.
 *
 * ============================================================================
 * PLACEMENT CONTRACT
 * ============================================================================
 *
 * Placement values describe intent.
 *
 * They may express:
 *
 *     locality;
 *     affinity;
 *     anti-affinity;
 *     co-location;
 *     separation;
 *     region;
 *     symbolic resource class;
 *     topology relationship.
 *
 * Actual placement is downstream.
 *
 * ============================================================================
 * TOPOLOGY CONTRACT
 * ============================================================================
 *
 * Topology values may describe:
 *
 *     connectivity;
 *     locality;
 *     distance;
 *     bandwidth;
 *     latency;
 *     topology class;
 *     symbolic topology requirements.
 *
 * Routing and topology discovery remain downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Hardware deployment may require quantum capabilities:
 *
 *     capability: quantum.measurement;
 *     resource qubits >= required_qubits;
 *     target: quantum;
 *
 * This grammar does not define:
 *
 *     gates;
 *     qubits;
 *     circuits;
 *     quantum states;
 *     physical qubits;
 *     native gate sets;
 *     pulse schedules.
 *
 * Quantum source remains owned by:
 *
 *     grammar/quantum/
 *
 * and lowers through:
 *
 *     quantum::ir
 *
 * ============================================================================
 * CLASSICAL CONTRACT
 * ============================================================================
 *
 * Hardware deployment may describe classical realization requirements:
 *
 *     target: classical;
 *     capability: tensor.compute;
 *     resource memory >= required_memory;
 *
 * It does not define classical algorithms.
 *
 * ============================================================================
 * HDL CONTRACT
 * ============================================================================
 *
 * Hardware deployment may describe deployment requirements for HDL-derived
 * hardware.
 *
 * It MUST NOT duplicate:
 *
 *     modules;
 *     signals;
 *     registers;
 *     processes;
 *     clocking;
 *     HDL behavioral semantics.
 *
 * Those remain owned by grammar/hdl/.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Distributed deployment may express:
 *
 *     resource nodes >= required_nodes;
 *     capability distributed.compute;
 *     topology: required_topology;
 *     placement: placement_policy;
 *
 * No maximum node count is encoded.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Deployment syntax does not grant permissions.
 *
 * It MUST NOT implicitly authorize:
 *
 *     device access;
 *     network access;
 *     memory access;
 *     secret access;
 *     privileged execution.
 *
 * Authentication and authorization remain owned by the security subsystem.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar owns no IR.
 *
 * It lowers conceptually to the common semantic deployment/hardware model:
 *
 *     HardwareDeployment
 *         |
 *         v
 *     semantic deployment contract
 *         |
 *         +--> resource model
 *         +--> capability model
 *         +--> target model
 *         +--> placement intent
 *         +--> topology intent
 *         |
 *         v
 *     compilation / realization
 *
 * Quantum computation remains:
 *
 *     quantum::ir
 *
 * No second quantum IR may be introduced.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may:
 *
 *     - evaluate requirements;
 *     - validate constraints;
 *     - inspect available capabilities;
 *     - inspect available resources;
 *     - resolve target classes;
 *     - construct realization candidates;
 *     - rank preferences;
 *     - apply hints;
 *     - construct deployment plans.
 *
 * The compiler must not change a mandatory requirement into a preference
 * merely because a current target cannot satisfy it.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime receives semantic deployment plans.
 *
 * Runtime may:
 *
 *     - realize an already validated deployment plan;
 *     - adapt within the declared contract;
 *     - report unavailable resources;
 *     - report unavailable capabilities.
 *
 * Runtime does not obtain authority from grammar syntax.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     active language/grammar version
 *
 * It must not depend on:
 *
 *     hardware discovery;
 *     device enumeration order;
 *     network state;
 *     scheduler timing;
 *     calibration state;
 *     environment variables;
 *     randomness.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Prohibited as universal language semantics:
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
 * Also prohibited as grammar-level physical realization:
 *
 *     CPU_0
 *     GPU_0
 *     FPGA_0
 *     QPU_0
 *     NODE_0
 *     DEVICE_0
 *     PHYSICAL_QUBIT_0
 *
 * Ordinary source identifiers containing such text are not intrinsically
 * forbidden; the semantic layer determines their meaning.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Diagnostics should distinguish:
 *
 *     syntax error
 *         malformed deployment declaration.
 *
 *     semantic requirement failure
 *         mandatory requirement cannot be satisfied.
 *
 *     capability failure
 *         required capability is unavailable.
 *
 *     resource failure
 *         required resource is unavailable.
 *
 *     target failure
 *         no target satisfies the declared target contract.
 *
 *     placement failure
 *         no realization satisfies placement constraints.
 *
 *     topology failure
 *         no realization satisfies topology constraints.
 *
 *     deployment/runtime failure
 *         realization failed after successful parsing/semantic validation.
 *
 * Grammar syntax errors must not masquerade as runtime resource failures.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must remain structurally linear with respect to the ordered
 * clause collection except where complexity is inherent in the canonical
 * expression grammar.
 *
 * It must not perform:
 *
 *     hardware discovery;
 *     network calls;
 *     filesystem access;
 *     resource enumeration;
 *     target probing.
 *
 * Large deployment specifications are represented through repetition rather
 * than fixed-size alternatives.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces the unique rule:
 *
 *     hardwareDeploymentDeclaration
 *
 * It MUST NOT replace:
 *
 *     deploymentDeclaration
 *     executionDeployment
 *
 * from grammar/execution/deployment.g4.
 *
 * Existing execution deployment syntax remains owned by execution/.
 *
 * Existing hardware target syntax remains owned by hardware/targets.g4.
 *
 * Existing hardware placement syntax remains owned by hardware/placement.g4.
 *
 * Existing hardware resource syntax remains owned by hardware/resources.g4.
 *
 * Existing hardware capability syntax remains owned by hardware/capabilities.g4.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS MUST COVER:
 *
 *     deploy hardware {
 *         target: classical;
 *     }
 *
 *     deploy hardware {
 *         target: quantum;
 *         capability: quantum.measurement;
 *     }
 *
 *     deploy accelerator {
 *         resource memory >= required_memory;
 *         capability: tensor.compute;
 *     }
 *
 *     deploy hardware {
 *         requires capability.quantum.dynamic_control;
 *         resource qubits >= required_qubits;
 *         topology: required_topology;
 *         placement: placement_policy;
 *     }
 *
 *     deploy hardware {
 *         target: heterogeneous;
 *         resource memory >= input.size * element_size;
 *         availability: required_availability;
 *     }
 *
 * NEGATIVE TESTS MUST COVER:
 *
 *     deploy;
 *
 *     deploy hardware {
 *     }
 *
 *     deploy hardware {
 *         requires;
 *     }
 *
 *     deploy hardware {
 *         resource;
 *     }
 *
 *     deploy hardware {
 *         constraint;
 *     }
 *
 *     deploy hardware {
 *         target:
 *     }
 *
 *     deploy hardware {
 *         property =
 *     }
 *
 * BOUNDARY TESTS MUST COVER:
 *
 *     - one deployment clause;
 *     - many deployment clauses;
 *     - deeply nested deployment objects;
 *     - deeply nested lists;
 *     - large symbolic expressions;
 *     - large qualified names;
 *     - arbitrarily large representable resource quantities;
 *     - many deployment properties;
 *     - many alternatives represented through semantic properties;
 *     - quantum/classical/hybrid/HDL targets.
 *
 * SCALABILITY TESTS MUST COVER:
 *
 *     tiny embedded target;
 *     single-device target;
 *     multicore target;
 *     GPU target;
 *     FPGA target;
 *     ASIC target;
 *     QPU target;
 *     heterogeneous target;
 *     distributed target;
 *     HPC target;
 *     cloud target;
 *     future/custom target.
 *
 * No scalability test may establish an artificial maximum.
 *
 * DETERMINISM TESTS MUST VERIFY:
 *
 *     identical source + identical grammar version
 *         =>
 *     identical parse structure.
 *
 * COMPATIBILITY TESTS MUST VERIFY:
 *
 *     execution deployment remains distinct;
 *     hardware target remains distinct;
 *     hardware placement remains distinct;
 *     hardware resource remains distinct;
 *     hardware capability remains distinct.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] unique hardware deployment grammar owner;
 *     [x] no duplicate execution deployment rule names;
 *     [x] canonical ZamaniTokens vocabulary;
 *     [x] canonical Core expression/name integration;
 *     [x] no embedded Rust;
 *     [x] no unsafe;
 *     [x] no hardware discovery;
 *     [x] no resource allocation;
 *     [x] no physical device assumptions;
 *     [x] no universal capacity constants;
 *     [x] open capability namespace;
 *     [x] open target namespace;
 *     [x] symbolic resource quantities;
 *     [x] deployment/realization separation;
 *     [x] AST contract;
 *     [x] semantic contract;
 *     [x] IR contract;
 *     [x] compiler contract;
 *     [x] runtime contract;
 *     [x] diagnostics contract;
 *     [x] deterministic parsing contract;
 *     [x] scalability contract;
 *     [x] compatibility contract;
 *     [x] test contract.
 *
 * Repository integration remains:
 *
 *     hardware.g4
 *         -> hardwareDeploymentDeclaration
 *
 * and:
 *
 *     Hardware
 *         -> HardwareDeployment
 *
 * through the canonical hardware composition boundary.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */