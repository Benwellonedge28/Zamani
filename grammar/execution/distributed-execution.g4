/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/distributed-execution.g4
 *
 * Grammar:
 *     DistributedExecution
 *
 * Status:
 *     Production-ready execution-level distributed-execution parser
 *     component.
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     ANTLR4
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, filesystem
 *     access, networking, hardware access, runtime calls, or unsafe code.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL EXECUTION INTENT for computations whose
 * execution may span multiple logical execution contexts.
 *
 * It describes:
 *
 *     - distributed execution declarations;
 *     - distributed execution subjects;
 *     - logical execution domains;
 *     - execution participants as abstract references;
 *     - distributed execution policies;
 *     - distributed requirements;
 *     - distributed constraints;
 *     - distributed preferences;
 *     - distributed hints;
 *     - capability requirements;
 *     - resource intent;
 *     - placement intent references;
 *     - scheduling intent references;
 *     - synchronization intent references;
 *     - lifecycle intent;
 *     - failure/recovery policy references;
 *     - result aggregation intent;
 *     - data locality intent;
 *     - communication intent references;
 *     - checkpoint/state policy references;
 *     - extensible execution properties.
 *
 * This grammar does NOT perform distributed execution.
 *
 * ============================================================================
 *
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes computation and semantic intent.
 *
 * It does not permanently describe the physical machine on which the
 * computation happens.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     - fixed node counts;
 *     - fixed process counts;
 *     - fixed worker counts;
 *     - fixed CPU counts;
 *     - fixed GPU counts;
 *     - fixed QPU counts;
 *     - fixed FPGA counts;
 *     - fixed memory capacities;
 *     - fixed network sizes;
 *     - fixed topology;
 *     - fixed hosts;
 *     - fixed IP addresses;
 *     - fixed ports;
 *     - fixed device identifiers;
 *     - fixed cluster identifiers;
 *     - fixed providers;
 *     - fixed transport protocols;
 *     - fixed retry counts;
 *     - fixed execution durations;
 *     - fixed deployment layouts.
 *
 * Physical realization belongs downstream to:
 *
 *     - resource analysis;
 *     - capability resolution;
 *     - target resolution;
 *     - placement;
 *     - scheduling;
 *     - routing;
 *     - deployment;
 *     - hardware abstraction;
 *     - networking;
 *     - runtime;
 *     - resilience.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Distributed execution syntax therefore expresses:
 *
 *     WHAT distributed behavior is required or preferred.
 *
 * It does not prescribe:
 *
 *     WHERE physical execution occurs.
 *
 * A single source program may therefore be realized as:
 *
 *     - one execution context;
 *     - many execution contexts;
 *     - a heterogeneous system;
 *     - a local machine;
 *     - a cluster;
 *     - an edge/cloud system;
 *     - a quantum network;
 *     - a future distributed architecture.
 *
 * The actual scale is determined by semantic requirements, available
 * resources, capabilities, target constraints, and runtime policy.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - execution-level distributed declaration syntax;
 *     - distributed execution subjects;
 *     - distributed execution scopes;
 *     - abstract participant references;
 *     - distributed execution policy references;
 *     - distributed execution requirement expressions;
 *     - distributed execution constraint expressions;
 *     - distributed execution preference expressions;
 *     - distributed execution hint expressions;
 *     - distributed execution capability references;
 *     - distributed execution resource references;
 *     - distributed execution lifecycle intent;
 *     - distributed result aggregation intent;
 *     - distributed state/checkpoint intent;
 *     - extensible distributed execution properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - types;
 *     - distributed nodes;
 *     - physical topology;
 *     - network endpoints;
 *     - network protocols;
 *     - message transport;
 *     - communication algorithms;
 *     - replication;
 *     - consistency algorithms;
 *     - consensus;
 *     - remote-execution transport;
 *     - placement algorithms;
 *     - routing algorithms;
 *     - scheduling algorithms;
 *     - resource allocation;
 *     - hardware discovery;
 *     - target discovery;
 *     - deployment;
 *     - optimization;
 *     - resilience algorithms;
 *     - checkpoint implementation;
 *     - serialization implementation;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - provider APIs.
 *
 * ============================================================================
 *
 * IMPORTANT OWNERSHIP SEPARATION
 * ============================================================================
 *
 * This file is NOT a replacement for:
 *
 *     grammar/distributed/remote-execution.g4
 *     grammar/distributed/replication.g4
 *     grammar/distributed/nodes.g4
 *     grammar/distributed/placement.g4
 *     grammar/distributed/communication.g4
 *     grammar/distributed/consistency.g4
 *     grammar/distributed/fault-tolerance.g4
 *
 * Those files own their respective distributed-domain semantics.
 *
 * This file only provides the EXECUTION-LAYER composition boundary.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     DistributedExecution
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> distributed semantic analysis
 *          +--> target resolution
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed execution metadata
 *          |
 *          v
 *     optimization
 *          |
 *          +--> routing
 *          +--> placement
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          v
 *     deployment planning
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 *
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A distributed execution declaration MAY refer to a quantum computation.
 *
 * This grammar MUST NOT define:
 *
 *     - QubitId;
 *     - PhysicalQubitId;
 *     - gates;
 *     - circuits;
 *     - quantum operations;
 *     - quantum topology;
 *     - pulses;
 *     - calibration;
 *     - QEC;
 *     - ZQN faults.
 *
 * Quantum source semantics remain owned by the quantum frontend and canonical
 * `quantum::ir`.
 *
 * Distributed execution may surround a quantum computation, but it does not
 * become a second quantum representation.
 *
 * In particular, distributed execution syntax MUST NOT imply that an
 * arbitrary unknown quantum state may be copied between execution contexts.
 *
 * Quantum-state legality belongs to semantic analysis and the quantum
 * execution/runtime layers.
 *
 * ============================================================================
 *
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical computations may be distributed through this execution layer.
 *
 * This grammar does not redefine:
 *
 *     - variables;
 *     - functions;
 *     - loops;
 *     - data structures;
 *     - numerical operations;
 *     - tensors;
 *     - classical IR.
 *
 * The distributed subject remains an expression.
 *
 * ============================================================================
 *
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A distributed execution declaration may refer to hardware or HDL
 * computation through semantic expressions.
 *
 * This grammar does not define:
 *
 *     - ports;
 *     - wires;
 *     - clocks;
 *     - registers;
 *     - FPGA fabric;
 *     - ASIC cells;
 *     - physical addresses;
 *     - device topology.
 *
 * Hardware semantics remain owned by the HDL and hardware grammars.
 *
 * ============================================================================
 *
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Resource requirements are references or expressions.
 *
 * This grammar does not redefine the universal resource taxonomy.
 *
 * The following concepts remain semantically distinct:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *
 * For example:
 *
 *     requirement: distributed;
 *
 * does not mean:
 *
 *     use N nodes.
 *
 * Likewise:
 *
 *     capability: accelerator;
 *
 * does not mean:
 *
 *     use accelerator X.
 *
 * ============================================================================
 *
 * PLACEMENT BOUNDARY
 * ============================================================================
 *
 * Placement is referenced, not implemented.
 *
 * This grammar does not:
 *
 *     - choose nodes;
 *     - bind processes to CPUs;
 *     - bind kernels to GPUs;
 *     - bind QPUs;
 *     - construct topology;
 *     - choose physical links;
 *     - calculate routes.
 *
 * Placement semantics belong to:
 *
 *     grammar/execution/placement.g4
 *     grammar/distributed/placement.g4
 *
 * and downstream placement planners.
 *
 * ============================================================================
 *
 * SCHEDULING BOUNDARY
 * ============================================================================
 *
 * Scheduling intent may be referenced.
 *
 * This grammar does not implement:
 *
 *     - ASAP;
 *     - ALAP;
 *     - list scheduling;
 *     - critical-path scheduling;
 *     - RCPSP;
 *     - dependency scheduling;
 *     - resource allocation;
 *     - timing calculation;
 *     - pulse scheduling.
 *
 * Scheduling remains owned by the scheduling subsystem.
 *
 * ============================================================================
 *
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * Distributed execution does not imply a transport.
 *
 * This grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     RPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor-specific transport.
 *
 * A communication or transport property is semantic metadata only.
 *
 * Actual transport selection belongs to networking, deployment, and runtime.
 *
 * ============================================================================
 *
 * REMOTE EXECUTION BOUNDARY
 * ============================================================================
 *
 * `grammar/distributed/remote-execution.g4` owns remote-execution declarations.
 *
 * This grammar does not redefine that declaration.
 *
 * A distributed execution declaration may reference a remote-execution
 * abstraction through an expression or qualified name.
 *
 * Therefore:
 *
 *     distributed execution
 *
 * is the execution composition concept, while:
 *
 *     remote execution
 *
 * is a distributed-domain realization concept.
 *
 * ============================================================================
 *
 * REPLICATION BOUNDARY
 * ============================================================================
 *
 * Replication remains owned by:
 *
 *     grammar/distributed/replication.g4
 *
 * This grammar may reference a replication policy or replication declaration.
 *
 * It does not redefine:
 *
 *     - replication factor syntax;
 *     - replica declarations;
 *     - consistency algorithms;
 *     - replica repair;
 *     - failover.
 *
 * ============================================================================
 *
 * CONSISTENCY BOUNDARY
 * ============================================================================
 *
 * Consistency is a semantic property.
 *
 * This grammar may carry a consistency policy expression but does not
 * implement:
 *
 *     - linearizability;
 *     - sequential consistency;
 *     - causal consistency;
 *     - eventual consistency;
 *     - quorum algorithms;
 *     - consensus.
 *
 * ============================================================================
 *
 * RESILIENCE BOUNDARY
 * ============================================================================
 *
 * Failure and recovery properties are intent only.
 *
 * This grammar does not implement:
 *
 *     - retry;
 *     - restart;
 *     - rollback;
 *     - checkpoint reconstruction;
 *     - failover;
 *     - fault diagnosis;
 *     - mitigation;
 *     - backend switching.
 *
 * Resilience consumes the resulting semantic execution intent.
 *
 * ============================================================================
 *
 * CHECKPOINT BOUNDARY
 * ============================================================================
 *
 * A checkpoint/state property is a request or policy reference only.
 *
 * The grammar does not assume that arbitrary execution state is serializable.
 *
 * Downstream systems must distinguish:
 *
 *     - classical checkpoints;
 *     - compiled-program checkpoints;
 *     - logical checkpoints;
 *     - measurement-boundary checkpoints;
 *     - QEC-supported checkpoints;
 *     - provider-supported state;
 *     - reconstructible state;
 *     - non-checkpointable state.
 *
 * ============================================================================
 *
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO lexer rules.
 *
 * Distributed execution-specific vocabulary remains contextual.
 *
 * The canonical lexer already provides the shared lexical foundation,
 * including identifiers, expressions, delimiters, and general execution
 * vocabulary.
 *
 * This design prevents every future distributed execution policy from
 * requiring a new global keyword.
 *
 * Semantic validation MUST classify contextual names.
 *
 * ============================================================================
 *
 * DEPENDENCIES
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     ZamaniLexer
 *     Names
 *     Expressions
 *
 * `Names` owns:
 *
 *     identifier
 *     qualified names
 *     name references
 *
 * `Expressions` owns:
 *
 *     expression
 *     calls
 *     operators
 *     indexing
 *     member access
 *     expression composition
 *
 * This grammar MUST NOT duplicate those definitions.
 *
 * ============================================================================
 *
 * PUBLIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The stable public parser entry point is:
 *
 *     distributedExecutionDeclaration
 *
 * The aggregate:
 *
 *     grammar/execution/execution.g4
 *
 * MUST import/compose this grammar and delegate its distributed-execution
 * branch to:
 *
 *     distributedExecutionDeclaration
 *
 * `execution.g4` MUST NOT maintain a second implementation of this syntax.
 *
 * ============================================================================
 *
 * DECLARATION FORM
 * ============================================================================
 *
 * Canonical contextual form:
 *
 *     distributed_execution workload {
 *         ...
 *     }
 *
 * The marker remains an identifier at lexical level.
 *
 * Semantic analysis validates the canonical declaration spelling.
 *
 * This preserves an open lexical vocabulary while maintaining a stable
 * semantic language construct.
 *
 * ============================================================================
 */

parser grammar DistributedExecution;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Stable execution-layer distributed declaration.
 *
 * The declaration name identifies a LOGICAL execution intent.
 *
 * It MUST NOT be interpreted as:
 *
 *     - a hostname;
 *     - a node identifier;
 *     - a device identifier;
 *     - a network address;
 *     - a physical cluster identifier.
 */
distributedExecutionDeclaration
    : distributedExecutionMarker
      identifier
      distributedExecutionBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 2. CONTEXTUAL DECLARATION MARKER
 * ========================================================================== */

/*
 * The canonical spelling is:
 *
 *     distributed_execution
 *
 * It remains an identifier lexically.
 *
 * Semantic analysis is responsible for contextual classification.
 */
distributedExecutionMarker
    : identifier
    ;


/* ============================================================================
 * 3. DECLARATION BODY
 * ========================================================================== */

distributedExecutionBody
    : LBRACE
      distributedExecutionMember*
      RBRACE
    ;


/* ============================================================================
 * 4. MEMBERS
 * ========================================================================== */

/*
 * Members are deliberately open-ended while retaining explicit syntax
 * boundaries.
 *
 * This permits future distributed execution properties without requiring
 * global lexer changes.
 */
distributedExecutionMember
    : distributedExecutionAssignment
    | distributedExecutionExpressionMember
    | distributedExecutionBlock
    ;


/* ============================================================================
 * 5. ASSIGNMENT / PROPERTY
 * ========================================================================== */

/*
 * Generic semantic property:
 *
 *     key: expression;
 *
 * Examples:
 *
 *     subject: workload;
 *     target: distributed;
 *     requirement: distributed;
 *     capability: quantum;
 *     placement: placement_policy;
 *     schedule: scheduling_policy;
 *     communication: communication_policy;
 *
 * Semantic analysis determines the ownership and meaning of the key.
 */
distributedExecutionAssignment
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 6. EXPRESSION MEMBER
 * ========================================================================== */

/*
 * Allows an execution expression to appear as a member:
 *
 *     synchronize();
 *     coordinate();
 *     distributed::operation();
 *
 * Whether an expression is valid in this context is a semantic concern.
 */
distributedExecutionExpressionMember
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * 7. NESTED PROPERTY / POLICY BLOCK
 * ========================================================================== */

/*
 * Generic nested structure:
 *
 *     policy {
 *         retry: policy;
 *         checkpoint: policy;
 *     }
 *
 *     resources {
 *         requirement: expression;
 *     }
 *
 *     placement {
 *         policy: expression;
 *     }
 *
 * The block name remains contextual.
 */
distributedExecutionBlock
    : identifier
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


/* ============================================================================
 * 8. SEMANTICALLY NAMED EXECUTION FORMS
 * ========================================================================== */

/*
 * The following rules provide stable semantic entry points for the frontend
 * without creating separate lexical vocabularies.
 *
 * They all consume the same underlying expression/property representation.
 */


/*
 * Distributed execution subject.
 *
 * The subject may be:
 *
 *     - a function;
 *     - a pipeline;
 *     - a classical computation;
 *     - a quantum computation;
 *     - an HDL computation;
 *     - an accelerator computation;
 *     - a heterogeneous computation;
 *     - a future computation domain.
 */
distributedExecutionSubject
    : expression
    ;


/*
 * Logical execution domain.
 *
 * This is a symbolic semantic reference, not a physical location.
 */
distributedExecutionDomain
    : identifier
    | qualifiedName
    ;


/*
 * Abstract participant reference.
 *
 * It does not define physical nodes.
 */
distributedExecutionParticipant
    : identifier
    | qualifiedName
    | expression
    ;


/*
 * Policy reference.
 *
 * Policy implementations are owned by downstream semantic/runtime systems.
 */
distributedExecutionPolicy
    : expression
    ;


/*
 * Capability requirement.
 *
 * Capability resolution occurs downstream.
 */
distributedExecutionCapability
    : expression
    ;


/*
 * Resource requirement/reference.
 *
 * Resource semantics remain owned by the resource subsystem.
 */
distributedExecutionResource
    : expression
    ;


/*
 * Placement reference.
 *
 * Placement remains downstream.
 */
distributedExecutionPlacement
    : expression
    ;


/*
 * Scheduling reference.
 *
 * Scheduling remains downstream.
 */
distributedExecutionSchedule
    : expression
    ;


/*
 * Synchronization reference.
 *
 * This is execution-level intent only.
 *
 * It does not redefine low-level concurrency synchronization.
 */
distributedExecutionSynchronization
    : expression
    ;


/*
 * Lifecycle policy.
 */
distributedExecutionLifecycle
    : expression
    ;


/*
 * Failure/recovery policy.
 */
distributedExecutionFailurePolicy
    : expression
    ;


/*
 * Result aggregation policy.
 */
distributedExecutionAggregation
    : expression
    ;


/*
 * State/checkpoint policy.
 */
distributedExecutionStatePolicy
    : expression
    ;


/*
 * Communication policy reference.
 *
 * Transport selection remains outside this grammar.
 */
distributedExecutionCommunication
    : expression
    ;


/* ============================================================================
 * 9. STRUCTURED DISTRIBUTED EXECUTION BLOCKS
 * ========================================================================== */

/*
 * Optional semantic adapter forms.
 *
 * These rules intentionally use contextual identifiers.
 *
 * The aggregate parser/semantic layer may map a recognized property name to
 * one of these rules without adding lexer keywords.
 */


/*
 * Subject declaration:
 *
 *     subject: computation;
 */
distributedExecutionSubjectMember
    : identifier
      COLON
      distributedExecutionSubject
      SEMICOLON
    ;


/*
 * Domain declaration:
 *
 *     domain: logical_domain;
 */
distributedExecutionDomainMember
    : identifier
      COLON
      distributedExecutionDomain
      SEMICOLON
    ;


/*
 * Participant declaration:
 *
 *     participant: logical_participant;
 */
distributedExecutionParticipantMember
    : identifier
      COLON
      distributedExecutionParticipant
      SEMICOLON
    ;


/*
 * Policy declaration:
 *
 *     policy: distributed_policy;
 */
distributedExecutionPolicyMember
    : identifier
      COLON
      distributedExecutionPolicy
      SEMICOLON
    ;


/*
 * Capability declaration:
 *
 *     capability: required_capability;
 */
distributedExecutionCapabilityMember
    : identifier
      COLON
      distributedExecutionCapability
      SEMICOLON
    ;


/*
 * Resource declaration:
 *
 *     resource: resource_requirement;
 */
distributedExecutionResourceMember
    : identifier
      COLON
      distributedExecutionResource
      SEMICOLON
    ;


/*
 * Placement declaration:
 *
 *     placement: placement_policy;
 */
distributedExecutionPlacementMember
    : identifier
      COLON
      distributedExecutionPlacement
      SEMICOLON
    ;


/*
 * Scheduling declaration:
 *
 *     schedule: scheduling_policy;
 */
distributedExecutionScheduleMember
    : identifier
      COLON
      distributedExecutionSchedule
      SEMICOLON
    ;


/*
 * Synchronization declaration:
 *
 *     synchronization: synchronization_policy;
 */
distributedExecutionSynchronizationMember
    : identifier
      COLON
      distributedExecutionSynchronization
      SEMICOLON
    ;


/*
 * Lifecycle declaration:
 *
 *     lifecycle: lifecycle_policy;
 */
distributedExecutionLifecycleMember
    : identifier
      COLON
      distributedExecutionLifecycle
      SEMICOLON
    ;


/*
 * Failure policy:
 *
 *     failure: failure_policy;
 */
distributedExecutionFailureMember
    : identifier
      COLON
      distributedExecutionFailurePolicy
      SEMICOLON
    ;


/*
 * Result aggregation:
 *
 *     aggregation: result_policy;
 */
distributedExecutionAggregationMember
    : identifier
      COLON
      distributedExecutionAggregation
      SEMICOLON
    ;


/*
 * State/checkpoint:
 *
 *     state: checkpoint_policy;
 */
distributedExecutionStateMember
    : identifier
      COLON
      distributedExecutionStatePolicy
      SEMICOLON
    ;


/*
 * Communication:
 *
 *     communication: communication_policy;
 */
distributedExecutionCommunicationMember
    : identifier
      COLON
      distributedExecutionCommunication
      SEMICOLON
    ;


/* ============================================================================
 * 10. CONSTRAINT FORM
 * ========================================================================== */

/*
 * Explicit constraint:
 *
 *     property == expression;
 *     property != expression;
 *     property < expression;
 *     property <= expression;
 *     property > expression;
 *     property >= expression;
 *
 * The grammar does not decide whether a property is a resource, placement,
 * performance, reliability, or other constraint.
 */
distributedExecutionConstraint
    : identifier
      distributedExecutionConstraintOperator
      expression
      SEMICOLON?
    ;


distributedExecutionConstraintOperator
    : EQUALS
    | NOT_EQUALS
    | LESS_THAN
    | LESS_THAN_EQUAL
    | GREATER_THAN
    | GREATER_THAN_EQUAL
    ;


/* ============================================================================
 * 11. LIST-LIKE EXTENSION FORM
 * ========================================================================== */

/*
 * Where a semantic property naturally contains multiple expressions, the
 * expression grammar remains authoritative.
 *
 * This rule is provided as an integration boundary rather than a second list
 * grammar.
 */
distributedExecutionExpressionList
    : expression
      (COMMA expression)*
    ;


/* ============================================================================
 * 12. KEY/VALUE EXTENSION BLOCK
 * ========================================================================== */

/*
 * Explicit property collection:
 *
 *     properties {
 *         key: value;
 *         another_key: another_value;
 *     }
 *
 * This remains source-preserving and open-ended.
 */
distributedExecutionPropertyBlock
    : LBRACE
      distributedExecutionAssignment*
      RBRACE
    ;


/* ============================================================================
 * 13. POLICY BLOCK
 * ========================================================================== */

/*
 * Policy blocks remain generic because policy semantics belong downstream.
 *
 * Example:
 *
 *     policy {
 *         availability: requirement;
 *         reliability: preference;
 *         failure: recovery_policy;
 *     }
 */
distributedExecutionPolicyBlock
    : distributedExecutionPolicyMarker
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


distributedExecutionPolicyMarker
    : identifier
    ;


/* ============================================================================
 * 14. RESOURCE BLOCK
 * ========================================================================== */

/*
 * Resource requirements are referenced rather than redefined.
 */
distributedExecutionResourceBlock
    : distributedExecutionResourceMarker
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


distributedExecutionResourceMarker
    : identifier
    ;


/* ============================================================================
 * 15. PLACEMENT BLOCK
 * ========================================================================== */

distributedExecutionPlacementBlock
    : distributedExecutionPlacementMarker
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


distributedExecutionPlacementMarker
    : identifier
    ;


/* ============================================================================
 * 16. COMMUNICATION BLOCK
 * ========================================================================== */

distributedExecutionCommunicationBlock
    : distributedExecutionCommunicationMarker
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


distributedExecutionCommunicationMarker
    : identifier
    ;


/* ============================================================================
 * 17. LIFECYCLE BLOCK
 * ========================================================================== */

distributedExecutionLifecycleBlock
    : distributedExecutionLifecycleMarker
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


distributedExecutionLifecycleMarker
    : identifier
    ;


/* ============================================================================
 * 18. STATE / CHECKPOINT BLOCK
 * ========================================================================== */

distributedExecutionStateBlock
    : distributedExecutionStateMarker
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


distributedExecutionStateMarker
    : identifier
    ;


/* ============================================================================
 * 19. FAILURE / RECOVERY BLOCK
 * ========================================================================== */

distributedExecutionFailureBlock
    : distributedExecutionFailureMarker
      LBRACE
      distributedExecutionMember*
      RBRACE
    ;


distributedExecutionFailureMarker
    : identifier
    ;


/* ============================================================================
 * 20. EXECUTION OPTIONS
 * ========================================================================== */

/*
 * Open-ended options are represented as ordinary properties.
 *
 * This avoids a permanent closed list of distributed execution features.
 */
distributedExecutionOption
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. EXECUTION EXTENSIONS
 * ========================================================================== */

/*
 * Extension namespace:
 *
 *     vendor::feature
 *     distributed::future::policy
 *     custom::execution::strategy
 *
 * The parser preserves the name.
 *
 * Semantic validation determines whether the extension is permitted.
 */
distributedExecutionExtension
    : qualifiedName
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 22. SCALABLE PARTICIPANT COLLECTION
 * ========================================================================== */

/*
 * Zero or more logical participants.
 *
 * No finite maximum is encoded.
 *
 * IMPORTANT:
 *
 * This represents logical references only.
 *
 * It does not enumerate physical nodes or impose a topology.
 */
distributedExecutionParticipants
    : distributedExecutionParticipant
      (COMMA distributedExecutionParticipant)*
    ;


/* ============================================================================
 * 23. SCALABLE POLICY COLLECTION
 * ========================================================================== */

distributedExecutionPolicies
    : distributedExecutionPolicy
      (COMMA distributedExecutionPolicy)*
    ;


/* ============================================================================
 * 24. SCALABLE CAPABILITY COLLECTION
 * ========================================================================== */

distributedExecutionCapabilities
    : distributedExecutionCapability
      (COMMA distributedExecutionCapability)*
    ;


/* ============================================================================
 * 25. SCALABLE RESOURCE COLLECTION
 * ========================================================================== */

distributedExecutionResources
    : distributedExecutionResource
      (COMMA distributedExecutionResource)*
    ;


/* ============================================================================
 * 26. DISTRIBUTED EXECUTION EXPRESSION
 * ========================================================================== */

/*
 * Generic expression-level representation used by higher execution grammar
 * composition.
 *
 * This deliberately remains expression-based so future computation domains
 * do not require this grammar to be rewritten.
 */
distributedExecutionExpression
    : expression
    ;