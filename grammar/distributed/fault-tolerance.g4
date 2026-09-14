/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/fault-tolerance.g4
 *
 * Grammar:
 *     Distributed Fault Tolerance
 *
 * Status:
 *     Production distributed fault-tolerance SOURCE grammar.
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *     No unsafe code
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines source-level syntax for expressing DISTRIBUTED
 * FAULT-TOLERANCE INTENT.
 *
 * It describes what fault-tolerance properties a Zamani program requires,
 * permits, prefers, or exposes to downstream compilation/runtime systems.
 *
 * It does NOT implement fault tolerance.
 *
 * In particular, this grammar does not implement:
 *
 *     - failure detectors;
 *     - retry algorithms;
 *     - restart algorithms;
 *     - failover algorithms;
 *     - leader election;
 *     - consensus;
 *     - quorum algorithms;
 *     - checkpoint storage;
 *     - checkpoint restoration;
 *     - replica repair;
 *     - state reconstruction;
 *     - distributed transactions;
 *     - network protocols;
 *     - placement;
 *     - scheduling;
 *     - routing;
 *     - hardware discovery;
 *     - resource discovery;
 *     - resilience orchestration;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - runtime execution.
 *
 * Those responsibilities belong to downstream semantic, compiler, scheduling,
 * resource, networking, resilience, hardware, quantum, and runtime systems.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     Zamani lexer
 *          |
 *          v
 *     FaultTolerance parser component
 *          |
 *          v
 *     Frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> distributed semantic analysis
 *          +--> security analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed execution metadata
 *          +--> resilience policy representation
 *          |
 *          v
 *     optimization / routing / scheduling
 *          |
 *          v
 *     deployment / runtime
 *
 * Grammar is therefore upstream of all implementation decisions.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani follows:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Fault-tolerance syntax must therefore describe SEMANTIC FAILURE HANDLING
 * INTENT rather than the physical machine used to realize that intent.
 *
 * This grammar MUST NOT encode:
 *
 *     - maximum nodes;
 *     - maximum replicas;
 *     - maximum retries;
 *     - maximum recovery attempts;
 *     - fixed cluster size;
 *     - fixed process count;
 *     - fixed CPU count;
 *     - fixed GPU count;
 *     - fixed QPU count;
 *     - fixed memory size;
 *     - fixed network capacity;
 *     - fixed network topology;
 *     - fixed hostnames;
 *     - fixed IP addresses;
 *     - fixed ports;
 *     - fixed machine IDs;
 *     - fixed device IDs;
 *     - fixed cloud providers;
 *     - fixed availability zones.
 *
 * Any numerical quantity in the source is a PROGRAM REQUIREMENT or POLICY
 * EXPRESSION. It is not a declaration of the physical machine.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level finite limit is imposed on:
 *
 *     - fault-tolerance declarations;
 *     - failure classes;
 *     - policies;
 *     - recovery actions;
 *     - retry policies;
 *     - recovery stages;
 *     - checkpoint references;
 *     - escalation rules;
 *     - dependencies;
 *     - constraints;
 *     - requirements;
 *     - nested scopes;
 *     - extension properties;
 *     - expressions;
 *     - qualified names.
 *
 * Repetition uses ANTLR '*' and '+' operators.
 *
 * There is deliberately NO:
 *
 *     MAX_RETRIES
 *     MAX_FAILURES
 *     MAX_RECOVERY_STEPS
 *     MAX_CHECKPOINTS
 *     MAX_POLICIES
 *     MAX_FAILURE_CLASSES
 *     MAX_NODES
 *     MAX_REPLICAS
 *
 * Practical limits belong to the compiler/runtime/resource environment.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - fault-tolerance declaration syntax;
 *     - failure-domain intent;
 *     - failure classification references;
 *     - detection intent;
 *     - retry intent;
 *     - restart intent;
 *     - failover intent;
 *     - recovery intent;
 *     - checkpoint/recovery intent;
 *     - degradation intent;
 *     - escalation intent;
 *     - availability requirements;
 *     - durability requirements;
 *     - recovery ordering intent;
 *     - fault-tolerance constraints;
 *     - fault-tolerance preferences;
 *     - fault-tolerance hints;
 *     - fault-tolerance extension syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - lexical tokens;
 *     - replication;
 *     - consistency;
 *     - networking;
 *     - messaging;
 *     - nodes;
 *     - services;
 *     - placement;
 *     - scheduling;
 *     - routing;
 *     - resource discovery;
 *     - hardware discovery;
 *     - optimization;
 *     - resilience implementation;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HDL semantics;
 *     - classical IR;
 *     - runtime execution.
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL SEPARATIONS
 * ============================================================================
 *
 * Fault tolerance is not replication.
 *
 *     replication.g4
 *
 * owns replication intent.
 *
 * Fault tolerance may reference replication-related semantics, but it MUST NOT
 * redefine replica syntax.
 *
 * Fault tolerance is not consistency.
 *
 *     consistency.g4
 *
 * owns consistency intent.
 *
 * Fault tolerance may refer to consistency requirements, but it MUST NOT
 * redefine consistency syntax.
 *
 * Fault tolerance is not resilience orchestration.
 *
 * The resilience subsystem decides WHEN and HOW to invoke recovery mechanisms.
 *
 * This grammar merely records source-level intent.
 *
 * Fault tolerance is not resource management.
 *
 * Resource availability is determined downstream by the resource and hardware
 * systems.
 *
 * Fault tolerance is not scheduling.
 *
 * Recovery ordering may be expressed as semantic intent, but actual scheduling
 * belongs to the scheduling subsystem.
 *
 * ============================================================================
 * FAILURE MODEL BOUNDARY
 * ============================================================================
 *
 * A program may identify a semantic failure class using an identifier or
 * qualified name.
 *
 * Examples include:
 *
 *     transient
 *     permanent
 *     process_failure
 *     node_failure
 *     communication_failure
 *     storage_failure
 *     resource_exhaustion
 *     corruption
 *     timeout
 *     cancellation
 *     unknown
 *     custom::failure
 *
 * These are NOT closed grammar keywords.
 *
 * The semantic layer determines whether a referenced failure model is known,
 * supported, or meaningful for the target.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Fault-tolerance policies are represented through identifiers and expressions
 * rather than a permanently closed enumeration.
 *
 * Future policies can therefore be introduced without necessarily modifying
 * this grammar.
 *
 * Examples:
 *
 *     distributed::fault_tolerance::retry
 *     distributed::fault_tolerance::restart
 *     distributed::fault_tolerance::failover
 *     distributed::fault_tolerance::recovery
 *     distributed::fault_tolerance::degraded
 *     distributed::fault_tolerance::custom
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file introduces NO lexer rules.
 *
 * It does not introduce global tokens for:
 *
 *     fault
 *     failure
 *     retry
 *     restart
 *     failover
 *     recover
 *     checkpoint
 *     degrade
 *     escalate
 *     availability
 *     durability
 *
 * These remain contextual semantic names.
 *
 * This prevents the distributed domain from becoming a second lexical
 * authority and preserves long-term extensibility.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This parser component consumes:
 *
 *     ZamaniLexer
 *     Names
 *     Expressions
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *     nameReference
 *
 * Expressions owns:
 *
 *     expression
 *     expressionList
 *     operators
 *     calls
 *     indexing
 *     member access
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * AGGREGATE INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/distributed/distributed.g4 MUST import this grammar component:
 *
 *     import FaultTolerance;
 *
 * and MUST route its fault-tolerance declaration branch to:
 *
 *     distributedFaultToleranceDeclaration
 *
 * It MUST NOT create a second grammar rule with the same responsibility.
 *
 * The stable public integration rule is:
 *
 *     distributedFaultToleranceDeclaration
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST representation must preserve:
 *
 *     - source span;
 *     - declaration name;
 *     - target name;
 *     - member ordering;
 *     - failure-model references;
 *     - policy references;
 *     - action expressions;
 *     - condition expressions;
 *     - recovery stage ordering;
 *     - checkpoint references;
 *     - escalation relationships;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - extensions.
 *
 * The AST MUST NOT resolve:
 *
 *     - physical machines;
 *     - physical nodes;
 *     - device identifiers;
 *     - network addresses;
 *     - transport protocols;
 *     - physical replica locations;
 *     - hardware topology.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether a target is fault-tolerance-capable;
 *     - whether a failure model exists;
 *     - whether an action is supported;
 *     - whether recovery actions are compatible;
 *     - whether retry conditions are meaningful;
 *     - whether recovery is semantically legal;
 *     - whether checkpointing is valid;
 *     - whether the requested durability is achievable;
 *     - whether availability requirements are achievable;
 *     - whether requirements conflict;
 *     - whether resource requirements are satisfiable;
 *     - whether quantum-state recovery is legal;
 *     - whether a requested recovery boundary is reconstructible.
 *
 * Syntax alone MUST NOT imply any of these properties.
 *
 * ============================================================================
 * QUANTUM SAFETY BOUNDARY
 * ============================================================================
 *
 * Fault-tolerance syntax may surround distributed quantum computation.
 *
 * It MUST NOT imply that arbitrary unknown quantum state can be copied,
 * serialized, checkpointed, restored, or replicated.
 *
 * In particular, this grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     Gate
 *     Circuit
 *     QuantumOperation
 *     QuantumState
 *     QEC code
 *     QEC decoder
 *     ZQN noise model
 *     calibration
 *     pulse
 *     quantum topology
 *
 * The quantum subsystem owns quantum semantics.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Recovery of quantum computation must be interpreted according to the
 * downstream quantum execution/recovery model.
 *
 * Valid recovery mechanisms may instead involve:
 *
 *     - classical execution state;
 *     - compiled program state;
 *     - logical checkpoint boundaries;
 *     - measurement boundaries;
 *     - QEC-supported logical state;
 *     - reconstructible provider-supported state.
 *
 * The grammar does not choose among them.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Fault tolerance MUST NOT encode physical hardware topology.
 *
 * This file does not define:
 *
 *     CPU count
 *     GPU count
 *     QPU count
 *     FPGA count
 *     memory capacity
 *     device address
 *     machine address
 *     rack
 *     host
 *     topology
 *     physical location
 *
 * Those belong to hardware, resources, placement, scheduling, and deployment.
 *
 * ============================================================================
 * NETWORK BOUNDARY
 * ============================================================================
 *
 * Fault tolerance does not select a transport.
 *
 * It does not imply:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     RPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor-specific transport
 *
 * Network realization belongs to the networking/runtime layers.
 *
 * ============================================================================
 * RESILIENCE BOUNDARY
 * ============================================================================
 *
 * The resilience subsystem is the policy/orchestration layer responsible for
 * deciding when and how recovery mechanisms are invoked.
 *
 * This grammar may express source-level fault-tolerance requirements that
 * resilience consumes.
 *
 * It does NOT implement:
 *
 *     diagnosis;
 *     incident correlation;
 *     health estimation;
 *     action selection;
 *     recovery orchestration;
 *     quarantine;
 *     backend switching;
 *     adaptive recompilation;
 *     retry execution;
 *     mitigation execution.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no filesystem operations;
 *     - no networking;
 *     - no runtime callbacks;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no mutable compiler-global state.
 *
 * Parsing is therefore a pure function of the token stream and grammar.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser/AST layer must preserve source order.
 *
 * It MUST NOT reorder:
 *
 *     - failure classes;
 *     - policies;
 *     - recovery actions;
 *     - recovery stages;
 *     - constraints;
 *     - preferences;
 *     - escalation rules.
 *
 * Canonical semantic normalization happens downstream.
 *
 * ============================================================================
 * DUPLICATE PROPERTY POLICY
 * ============================================================================
 *
 * Repeated properties are syntactically preserved.
 *
 * Example:
 *
 *     fault_tolerance workload {
 *         requirement: available;
 *         requirement: durable;
 *     }
 *
 * The grammar does not silently select one.
 *
 * Semantic analysis determines whether repeated requirements:
 *
 *     - compose;
 *     - conflict;
 *     - override;
 *     - duplicate;
 *     - are invalid.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

parser grammar FaultTolerance;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration rule consumed by distributed/distributed.g4.
 */
distributedFaultToleranceDeclaration
    : faultToleranceDeclaration
    ;


/*
 * ============================================================================
 * 2. TOP-LEVEL DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     fault_tolerance <target> { ... }
 *
 * Both contextual words are parsed through identifier.
 *
 * Semantic analysis performs contextual classification.
 */
faultToleranceDeclaration
    : identifier
      identifier
      faultToleranceBody
    ;


/*
 * ============================================================================
 * 3. BODY
 * ============================================================================
 */

faultToleranceBody
    : LBRACE
      faultToleranceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. MEMBERS
 * ============================================================================
 */

faultToleranceMember
    : faultFailureModel
    | faultDetectionPolicy
    | faultRetryPolicy
    | faultRestartPolicy
    | faultFailoverPolicy
    | faultRecoveryPolicy
    | faultCheckpointPolicy
    | faultDegradationPolicy
    | faultEscalationPolicy
    | faultAvailabilityRequirement
    | faultDurabilityRequirement
    | faultRecoveryStage
    | faultRecoveryCondition
    | faultRecoveryAction
    | faultDependency
    | faultConstraint
    | faultRequirement
    | faultPreference
    | faultHint
    | faultExtension
    ;


/*
 * ============================================================================
 * 5. FAILURE MODEL
 * ============================================================================
 *
 * Associates a semantic failure model with the declaration.
 *
 * Example:
 *
 *     failure: transient;
 *     failure: node_failure;
 *     failure: custom::failure;
 */
faultFailureModel
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. DETECTION POLICY
 * ============================================================================
 *
 * Describes detection intent.
 *
 * The actual detector is selected downstream.
 *
 * Examples:
 *
 *     detection: health_monitor;
 *     detection: timeout_based;
 *     detection: custom_detector();
 */
faultDetectionPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. RETRY POLICY
 * ============================================================================
 *
 * Records retry intent.
 *
 * No finite retry limit is imposed by the grammar.
 *
 * Examples:
 *
 *     retry: bounded(policy);
 *     retry: adaptive;
 *     retry: custom_retry();
 */
faultRetryPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. RESTART POLICY
 * ============================================================================
 *
 * Records restart intent without specifying process or machine identity.
 *
 * Examples:
 *
 *     restart: process;
 *     restart: service;
 *     restart: reconstructible;
 */
faultRestartPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. FAILOVER POLICY
 * ============================================================================
 *
 * Records semantic failover intent.
 *
 * Physical target selection belongs to placement/deployment/runtime.
 */
faultFailoverPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. RECOVERY POLICY
 * ============================================================================
 *
 * Describes recovery intent.
 *
 * The grammar does not execute recovery.
 */
faultRecoveryPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CHECKPOINT POLICY
 * ============================================================================
 *
 * Checkpoint syntax describes checkpoint intent only.
 *
 * It does not imply that arbitrary program state is serializable.
 */
faultCheckpointPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. DEGRADATION POLICY
 * ============================================================================
 *
 * Allows a program to express degraded-operation intent.
 *
 * Examples:
 *
 *     degradation: allowed;
 *     degradation: preferred;
 *     degradation: policy;
 */
faultDegradationPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. ESCALATION POLICY
 * ============================================================================
 *
 * Describes what semantic escalation is permitted.
 *
 * Examples:
 *
 *     escalation: retry;
 *     escalation: recover;
 *     escalation: abort;
 *     escalation: custom_policy();
 */
faultEscalationPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. AVAILABILITY REQUIREMENT
 * ============================================================================
 *
 * Availability is expressed as semantic intent.
 *
 * The grammar does not convert availability into a physical node count.
 */
faultAvailabilityRequirement
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. DURABILITY REQUIREMENT
 * ============================================================================
 *
 * Durability is distinct from replication.
 */
faultDurabilityRequirement
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. RECOVERY STAGE
 * ============================================================================
 *
 * A stage provides explicit source ordering without defining execution order
 * on a particular machine.
 *
 * Example:
 *
 *     stage recover_classical { ... }
 *
 * The scheduler/runtime determines the realizable execution order.
 */
faultRecoveryStage
    : identifier
      identifier
      LBRACE
      faultRecoveryStageMember*
      RBRACE
    ;


faultRecoveryStageMember
    : faultRecoveryCondition
    | faultRecoveryAction
    | faultDependency
    | faultRequirement
    | faultPreference
    | faultHint
    | faultExtension
    ;


/*
 * ============================================================================
 * 17. RECOVERY CONDITION
 * ============================================================================
 *
 * A condition determines when an action is semantically applicable.
 */
faultRecoveryCondition
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. RECOVERY ACTION
 * ============================================================================
 *
 * An action is represented as a semantic expression.
 *
 * This deliberately does not enumerate every possible future action.
 *
 * Examples:
 *
 *     action: retry;
 *     action: restart;
 *     action: resume;
 *     action: rollback;
 *     action: remap;
 *     action: reroute;
 *     action: reschedule;
 *     action: recompile;
 *     action: switch_backend;
 *     action: quarantine;
 *     action: abort;
 *
 * These names are interpreted by downstream resilience/runtime systems.
 */
faultRecoveryAction
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. DEPENDENCY
 * ============================================================================
 *
 * Declares a semantic dependency between recovery requirements.
 *
 * Example:
 *
 *     depends_on: checkpoint_state;
 *
 * No physical dependency is implied.
 */
faultDependency
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. CONSTRAINT
 * ============================================================================
 *
 * A constraint is a semantic restriction.
 *
 * It is distinct from:
 *
 *     requirement
 *     preference
 *     hint
 *
 * Example:
 *
 *     constraint: preserve_semantics;
 */
faultConstraint
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 */
faultRequirement
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. PREFERENCE
 * ============================================================================
 *
 * A preference guides implementation without becoming an absolute semantic
 * requirement.
 */
faultPreference
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. HINT
 * ============================================================================
 *
 * A hint is advisory and may be ignored by a valid implementation.
 */
faultHint
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. EXTENSION
 * ============================================================================
 *
 * Future fault-tolerance systems may attach domain-specific semantic
 * properties without requiring a new grammar keyword.
 *
 * Example:
 *
 *     custom_policy: provider::extension(...);
 */
faultExtension
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. NESTED POLICY BLOCK
 * ============================================================================
 *
 * A generic named policy block permits future policy structures while keeping
 * this grammar open-world.
 *
 * Example:
 *
 *     policy recovery {
 *         action: retry;
 *         action: restart;
 *     }
 *
 * This is intentionally separate from the property rules above so the AST can
 * preserve the difference between a scalar property and a nested policy.
 */
faultPolicyBlock
    : identifier
      identifier
      LBRACE
      faultPolicyMember*
      RBRACE
    ;


faultPolicyMember
    : faultRecoveryCondition
    | faultRecoveryAction
    | faultDependency
    | faultConstraint
    | faultRequirement
    | faultPreference
    | faultHint
    | faultExtension
    | faultPolicyBlock
    ;


/*
 * ============================================================================
 * 26. RECOVERY PLAN
 * ============================================================================
 *
 * A recovery plan is a source-level grouping of recovery intent.
 *
 * It does not become a runtime recovery engine.
 */
faultRecoveryPlan
    : identifier
      identifier
      LBRACE
      faultRecoveryPlanMember*
      RBRACE
    ;


faultRecoveryPlanMember
    : faultRecoveryStage
    | faultRecoveryCondition
    | faultRecoveryAction
    | faultDependency
    | faultConstraint
    | faultRequirement
    | faultPreference
    | faultHint
    | faultExtension
    | faultPolicyBlock
    ;


/*
 * ============================================================================
 * 27. FAILURE DOMAIN
 * ============================================================================
 *
 * A failure domain groups semantically related failure conditions.
 *
 * It does not identify a physical topology domain.
 *
 * Example:
 *
 *     domain communication {
 *         failure: timeout;
 *         failure: link_loss;
 *     }
 */
faultFailureDomain
    : identifier
      identifier
      LBRACE
      faultFailureDomainMember*
      RBRACE
    ;


faultFailureDomainMember
    : faultFailureModel
    | faultRecoveryCondition
    | faultRecoveryAction
    | faultDependency
    | faultConstraint
    | faultRequirement
    | faultPreference
    | faultHint
    | faultExtension
    ;


/*
 * ============================================================================
 * 28. FAILURE CLASSIFICATION
 * ============================================================================
 *
 * Classification remains semantic and extensible.
 */
faultFailureClassification
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 29. RECOVERY ORDER
 * ============================================================================
 *
 * Expresses semantic precedence without selecting a scheduler.
 *
 * Example:
 *
 *     before: retry;
 *     after: detect;
 *
 * The actual schedule remains downstream.
 */
faultRecoveryOrder
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 30. RECOVERY BARRIER
 * ============================================================================
 *
 * Represents a semantic boundary after which recovery may proceed.
 *
 * This is not a hardware barrier and not a runtime synchronization primitive.
 */
faultRecoveryBarrier
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. SEMANTIC ACCEPTANCE
 * ============================================================================
 *
 * Fault tolerance can expose desired acceptance behavior.
 *
 * Examples:
 *
 *     accept: success;
 *     accept: degraded;
 *     accept: recoverable;
 *     accept: reject;
 *
 * The final decision belongs to semantic validation/resilience.
 */
faultAcceptancePolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 32. RESOURCE-AWARE RECOVERY
 * ============================================================================
 *
 * Recovery may reference generic resource/capability expressions.
 *
 * This grammar does not define resource vocabulary.
 */
faultResourceRequirement
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. SECURITY-AWARE RECOVERY
 * ============================================================================
 *
 * Security constraints remain generic semantic expressions.
 *
 * This prevents this grammar from duplicating security/permissions grammar.
 */
faultSecurityRequirement
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. CROSS-DOMAIN RECOVERY
 * ============================================================================
 *
 * Fault tolerance may surround computations crossing:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     networking
 *
 * The grammar only records the cross-domain target/reference.
 */
faultCrossDomainReference
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 35. OPTIONAL EXTENSION SCOPE
 * ============================================================================
 *
 * Future domains may attach a nested semantic namespace.
 */
faultExtensionScope
    : identifier
      identifier
      LBRACE
      faultExtensionMember*
      RBRACE
    ;


faultExtensionMember
    : faultExtension
    | faultPolicyBlock
    | faultRequirement
    | faultConstraint
    | faultPreference
    | faultHint
    | faultDependency
    ;


/*
 * ============================================================================
 * 36. INTEGRATION NOTES
 * ============================================================================
 *
 * This grammar intentionally uses canonical shared rules:
 *
 *     identifier
 *     expression
 *
 * and shared lexer tokens:
 *
 *     LBRACE
 *     RBRACE
 *     COLON
 *     SEMICOLON
 *
 * It therefore introduces no duplicate lexical authority.
 *
 * The aggregate distributed grammar should import:
 *
 *     FaultTolerance
 *
 * and expose:
 *
 *     distributedFaultToleranceDeclaration
 *
 * No other distributed grammar should redefine that public entry point.
 *
 * ============================================================================
 * 37. SEMANTIC OWNERSHIP AFTER PARSING
 * ============================================================================
 *
 * After parsing:
 *
 *     Name resolution
 *         -> resolves references
 *
 *     Type analysis
 *         -> validates expressions
 *
 *     Capability analysis
 *         -> determines whether requested capabilities exist
 *
 *     Resource analysis
 *         -> determines feasibility against available resources
 *
 *     Distributed semantic analysis
 *         -> validates distributed meaning
 *
 *     Resilience
 *         -> decides recovery/orchestration strategy
 *
 *     Scheduling
 *         -> determines executable ordering
 *
 *     Routing
 *         -> determines physical/network realization
 *
 *     Hardware abstraction
 *         -> determines target capabilities
 *
 *     Runtime
 *         -> executes the resulting plan
 *
 * The grammar MUST remain independent of these implementation choices.
 *
 * ============================================================================
 * 38. DETERMINISTIC PARSING CONTRACT
 * ============================================================================
 *
 * No rule in this file:
 *
 *     - performs I/O;
 *     - accesses a file;
 *     - accesses a network;
 *     - accesses hardware;
 *     - calls runtime code;
 *     - performs randomness;
 *     - depends on mutable global state.
 *
 * Therefore the same token stream must yield the same parse structure.
 *
 * ============================================================================
 * 39. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file deliberately contains no:
 *
 *     MAX_NODES
 *     MAX_RETRIES
 *     MAX_RECOVERY_STEPS
 *     MAX_FAILURES
 *     MAX_CHECKPOINTS
 *     MAX_REPLICAS
 *     MAX_DEVICES
 *     MAX_WORKERS
 *     MAX_PROCESSES
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_CPUS
 *
 * It contains no:
 *
 *     device IDs;
 *     hostnames;
 *     IP addresses;
 *     physical addresses;
 *     provider IDs;
 *     topology IDs.
 *
 * Any such limitation must be introduced by a downstream target/resource
 * policy rather than this grammar.
 *
 * ============================================================================
 * 40. RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains no embedded Rust.
 *
 * Consequently:
 *
 *     - no unsafe block;
 *     - no unsafe function;
 *     - no raw pointer;
 *     - no FFI;
 *     - no filesystem operation;
 *     - no network operation.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * and the repository's no-unsafe policy.
 *
 * ============================================================================
 * 41. TEST CONTRACT
 * ============================================================================
 *
 * The grammar component is complete only when tests cover at least:
 *
 * POSITIVE:
 *
 *     fault_tolerance workload {
 *         failure: transient;
 *         retry: adaptive;
 *         recovery: reconstructible;
 *     }
 *
 *     fault_tolerance service {
 *         detection: health_monitor;
 *         restart: service;
 *         failover: available_target;
 *     }
 *
 *     fault_tolerance computation {
 *         checkpoint: logical_boundary;
 *         degradation: allowed;
 *         escalation: recover;
 *     }
 *
 *     fault_tolerance quantum_job {
 *         failure: execution_failure;
 *         recovery: qec_supported_boundary;
 *         requirement: preserve_semantics;
 *     }
 *
 * NEGATIVE:
 *
 *     - missing declaration name;
 *     - missing body;
 *     - missing colon;
 *     - missing expression;
 *     - missing semicolon;
 *     - malformed nested block;
 *     - unbalanced braces;
 *     - malformed recovery stage;
 *     - malformed recovery plan.
 *
 * BOUNDARY:
 *
 *     - one declaration;
 *     - many declarations;
 *     - one property;
 *     - arbitrarily many properties;
 *     - deeply nested policy scopes;
 *     - long qualified names;
 *     - large expressions.
 *
 * CROSS-DOMAIN:
 *
 *     classical + distributed + fault tolerance
 *     quantum + distributed + fault tolerance
 *     quantum + classical + distributed + fault tolerance
 *     HDL + distributed + fault tolerance
 *     hardware + distributed + fault tolerance
 *     AI + distributed + fault tolerance
 *
 * SCALABILITY:
 *
 * Tests must demonstrate that no grammar-level machine-size limit exists.
 *
 * DETERMINISM:
 *
 * The same source/token stream must produce the same parse structure.
 *
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 *     [ ] It parses the complete fault-tolerance syntax contract.
 *     [ ] It imports only canonical shared dependencies.
 *     [ ] It defines distributedFaultToleranceDeclaration.
 *     [ ] distributed.g4 integrates that public rule.
 *     [ ] No duplicate lexer keywords are introduced.
 *     [ ] No machine-size limits are encoded.
 *     [ ] No physical topology is encoded.
 *     [ ] Replication is not redefined.
 *     [ ] Consistency is not redefined.
 *     [ ] Networking is not redefined.
 *     [ ] Resource management is not redefined.
 *     [ ] Resilience implementation is not redefined.
 *     [ ] Quantum IR is not redefined.
 *     [ ] QEC is not redefined.
 *     [ ] ZQN is not redefined.
 *     [ ] Source ordering can be preserved.
 *     [ ] AST mapping is deterministic.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Cross-domain tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Rust 1.97/1.97.1 integration passes.
 *     [ ] No unsafe implementation is introduced.
 *
 * ============================================================================
 */