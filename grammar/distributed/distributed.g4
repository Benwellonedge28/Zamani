/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/distributed.g4
 *
 * Grammar:
 *     Distributed
 *
 * Status:
 *     Production distributed-computing domain grammar.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No mutable compiler-global state.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL SYNTAX for distributed computation
 * in Zamani.
 *
 * It describes distributed COMPUTATIONAL INTENT.
 *
 * It does not implement:
 *
 *     - distributed execution;
 *     - network transport;
 *     - service discovery;
 *     - node discovery;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - replication algorithms;
 *     - consensus algorithms;
 *     - consistency algorithms;
 *     - fault-tolerance algorithms;
 *     - distributed storage;
 *     - runtime communication;
 *     - deployment;
 *     - cloud-provider integration;
 *     - hardware discovery;
 *     - quantum routing;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * Those concerns belong to downstream semantic, compiler, runtime, resource,
 * networking, scheduling, routing, and resilience subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     UTF-8 source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser / Distributed parser component
 *          |
 *          v
 *     Distributed AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> distributed semantic validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed execution metadata
 *          +--> resource requirements
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     placement / routing / scheduling
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime / distributed environment
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NEVER construct or redefine quantum::ir.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed syntax must support:
 *
 *     Program Once
 *         ->
 *     Compile Once
 *         ->
 *     Run Everywhere
 *         ->
 *     Run Anywhere
 *         ->
 *     Run Forever
 *
 * A distributed program describes WHAT distributed behavior is required,
 * not WHICH particular collection of machines must provide it.
 *
 * Therefore this grammar does NOT encode:
 *
 *     - fixed node counts;
 *     - fixed process counts;
 *     - fixed service counts;
 *     - fixed cluster sizes;
 *     - fixed topology;
 *     - fixed network addresses;
 *     - fixed ports;
 *     - fixed machine IDs;
 *     - fixed device IDs;
 *     - fixed CPU counts;
 *     - fixed GPU counts;
 *     - fixed QPU counts;
 *     - fixed memory capacities;
 *     - fixed bandwidth;
 *     - fixed latency;
 *     - fixed deployment regions;
 *     - fixed cloud providers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar deliberately imposes no language-level finite limit on:
 *
 *     - distributed declarations;
 *     - nodes;
 *     - services;
 *     - processes;
 *     - workers;
 *     - actors;
 *     - endpoints;
 *     - channels;
 *     - messages;
 *     - partitions;
 *     - replicas;
 *     - shards;
 *     - tasks;
 *     - dependencies;
 *     - regions;
 *     - placement alternatives;
 *     - consistency clauses;
 *     - replication clauses;
 *     - communication clauses;
 *     - nested distributed scopes;
 *     - qualified-name depth.
 *
 * Repetition is represented using:
 *
 *     *
 *     +
 *
 * rather than artificial finite cardinalities.
 *
 * Practical limits imposed by:
 *
 *     - parser memory;
 *     - compiler memory;
 *     - operating-system resources;
 *     - runtime resources;
 *     - network capacity;
 *     - available hardware;
 *     - deployment policy
 *
 * are NOT grammar limits.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Distributed concepts are intentionally represented using canonical names
 * rather than an exhaustive keyword inventory.
 *
 * Examples of valid semantic names include:
 *
 *     distributed::node
 *     distributed::service
 *     distributed::worker
 *     distributed::actor
 *     distributed::channel
 *     distributed::message
 *     distributed::replication
 *     distributed::consistency
 *     distributed::partition
 *     distributed::placement
 *     distributed::remote_execution
 *     distributed::migration
 *     distributed::coordination
 *     distributed::consensus
 *     distributed::future::operation
 *     distributed::vendor::extension
 *     distributed::future::capability
 *
 * Future distributed abstractions can therefore be introduced without
 * changing the lexical vocabulary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed-domain declaration syntax;
 *     - distributed computation scopes;
 *     - distributed entity declarations;
 *     - distributed process/task declarations;
 *     - distributed service declarations;
 *     - distributed actor declarations;
 *     - distributed channel declarations;
 *     - distributed communication declarations;
 *     - distributed remote-execution intent;
 *     - distributed replication intent;
 *     - distributed consistency intent;
 *     - distributed partitioning intent;
 *     - distributed placement requirements;
 *     - distributed deployment intent;
 *     - distributed coordination intent;
 *     - distributed semantic options;
 *     - distributed dependency relationships.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - lexical tokens;
 *     - generic effects;
 *     - networking protocols;
 *     - resources;
 *     - capabilities;
 *     - hardware;
 *     - target selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - memory ownership;
 *     - concurrency primitives;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     ZamaniLexer
 *          |
 *          +--> Names
 *          |
 *          +--> Expressions
 *          |
 *          v
 *     Distributed
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
 *
 * This grammar MUST NOT duplicate those rules.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * This parser grammar uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and imports:
 *
 *     Names
 *     Expressions
 *
 * The canonical ANTLR build MUST make:
 *
 *     grammar/antlr/
 *     grammar/core/
 *     grammar/expressions/
 *     grammar/distributed/
 *
 * available on the ANTLR grammar source/import path.
 *
 * This file is therefore independent at the semantic-ownership level while
 * remaining correctly integrated into the repository's canonical grammar
 * composition system.
 *
 * ============================================================================
 * IMPORTANT: NO DUPLICATED LEXER KEYWORDS
 * ============================================================================
 *
 * This file deliberately does NOT introduce tokens such as:
 *
 *     NODE
 *     SERVICE
 *     WORKER
 *     ACTOR
 *     CHANNEL
 *     MESSAGE
 *     REPLICA
 *     SHARD
 *     CLUSTER
 *     REGION
 *     CONSENSUS
 *
 * Such a closed keyword inventory would make future distributed extensions
 * require grammar and lexer changes.
 *
 * Instead, the grammar uses canonical identifiers and qualified names.
 *
 * Semantic analysis determines whether a name represents a distributed
 * construct.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser answers:
 *
 *     "Is this distributed construct structurally valid?"
 *
 * It does NOT answer:
 *
 *     "Can the requested deployment actually be realized?"
 *
 *     "Which nodes will execute it?"
 *
 *     "Which network will carry it?"
 *
 *     "Which transport protocol is used?"
 *
 *     "Which consistency algorithm is selected?"
 *
 *     "How many replicas are available?"
 *
 *     "Is the requested placement possible?"
 *
 * Those questions belong downstream.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * A distributed construct may express resource requirements through the
 * generic resource/capability systems.
 *
 * This grammar does NOT define:
 *
 *     maxNodes
 *     maxWorkers
 *     cpuCount
 *     gpuCount
 *     memorySize
 *     bandwidth
 *     topology
 *     deviceId
 *     address
 *
 * or equivalent fixed machine properties.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * Distributed communication intent is distinct from network implementation.
 *
 * For example:
 *
 *     distributed::send
 *
 * does not imply:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transport
 *
 * The networking subsystem chooses a valid realization.
 *
 * ============================================================================
 * CONSISTENCY BOUNDARY
 * ============================================================================
 *
 * The grammar records consistency intent.
 *
 * It does not implement a consistency algorithm.
 *
 * A source program may express an abstract consistency requirement such as:
 *
 *     distributed::consistency
 *
 * or a named policy:
 *
 *     distributed::consistency::required
 *
 * The semantic layer determines whether the requested property is:
 *
 *     supported
 *     unsupported
 *     conditionally supported
 *     unknown
 *
 * ============================================================================
 * REPLICATION BOUNDARY
 * ============================================================================
 *
 * Replication is an intent.
 *
 * The grammar does not require a finite number of physical copies.
 *
 * Any quantity expression, when permitted by the language, remains a semantic
 * requirement and is evaluated against available resources downstream.
 *
 * ============================================================================
 * PLACEMENT BOUNDARY
 * ============================================================================
 *
 * Placement describes constraints/preferences.
 *
 * It does NOT directly select a physical machine.
 *
 * A source-level placement expression must remain portable.
 *
 * ============================================================================
 * REMOTE EXECUTION BOUNDARY
 * ============================================================================
 *
 * Remote execution describes execution intent.
 *
 * It does not encode:
 *
 *     host;
 *     IP address;
 *     port;
 *     provider;
 *     device;
 *     cluster;
 *     physical node.
 *
 * Runtime dispatch and deployment resolve those details.
 *
 * ============================================================================
 * DISTRIBUTED STATE
 * ============================================================================
 *
 * Distributed state may be declared syntactically, but:
 *
 *     ownership;
 *     consistency;
 *     replication;
 *     persistence;
 *     serialization;
 *     synchronization;
 *     failure semantics
 *
 * are semantic/runtime concerns.
 *
 * ============================================================================
 * FAILURE BOUNDARY
 * ============================================================================
 *
 * This grammar does not implement distributed fault tolerance.
 *
 * Failure policy belongs to the appropriate runtime/resilience subsystems.
 *
 * The grammar may record source-level requirements concerning:
 *
 *     distributed::fault_tolerance
 *     distributed::availability
 *     distributed::recovery
 *
 * without implementing them.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed quantum programs may use this grammar to express:
 *
 *     distributed execution;
 *     communication;
 *     logical state placement;
 *     distributed coordination;
 *     remote execution intent.
 *
 * However:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic representation.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     pulse semantics
 *     QEC algorithms
 *     ZQN noise models.
 *
 * Distributed quantum routing is downstream.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A distributed declaration may coexist with HDL/hardware constructs.
 *
 * This grammar does not define:
 *
 *     wires;
 *     clocks;
 *     physical ports;
 *     FPGA resources;
 *     ASIC cells;
 *     hardware addresses.
 *
 * Hardware realization belongs to the HDL/hardware subsystems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - has no semantic predicates;
 *     - has no actions;
 *     - has no I/O;
 *     - has no network calls;
 *     - has no hardware discovery;
 *     - has no runtime calls;
 *     - has no randomness.
 *
 * Therefore parsing is determined solely by the token stream.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * Frontend AST construction must preserve:
 *
 *     - declaration ordering;
 *     - entity names;
 *     - qualified names;
 *     - expression structure;
 *     - option ordering;
 *     - communication ordering;
 *     - dependency ordering;
 *     - nested scopes;
 *     - source spans.
 *
 * Semantic canonicalization occurs downstream.
 *
 * ============================================================================
 */

parser grammar Distributed;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * These are the stable integration rules for higher-level parser composition.
 *
 * The grammar does not require the complete source file to be distributed-only.
 */

distributedDeclaration
    : distributedProgram
    | distributedScope
    | distributedEntityDeclaration
    | distributedTaskDeclaration
    | distributedServiceDeclaration
    | distributedActorDeclaration
    | distributedChannelDeclaration
    | distributedStateDeclaration
    | distributedCommunicationDeclaration
    | distributedReplicationDeclaration
    | distributedConsistencyDeclaration
    | distributedPlacementDeclaration
    | distributedRemoteExecutionDeclaration
    | distributedDeploymentDeclaration
    | distributedCoordinationDeclaration
    ;


/* ============================================================================
 * 2. DISTRIBUTED PROGRAM
 * ============================================================================
 *
 * Canonical semantic shape:
 *
 *     distributed <name> { ... }
 *
 * `distributed` is intentionally parsed as an identifier rather than a
 * dedicated lexer keyword.
 *
 * Semantic analysis MUST verify the contextual spelling/classification.
 */

distributedProgram
    : identifier
      identifier
      LBRACE
      distributedMember*
      RBRACE
    ;


/*
 * Distributed member declarations are deliberately open-ended.
 *
 * This allows future domain-specific distributed declarations to coexist
 * without requiring a new lexer token for every future abstraction.
 */

distributedMember
    : distributedEntityDeclaration
    | distributedTaskDeclaration
    | distributedServiceDeclaration
    | distributedActorDeclaration
    | distributedChannelDeclaration
    | distributedStateDeclaration
    | distributedCommunicationDeclaration
    | distributedReplicationDeclaration
    | distributedConsistencyDeclaration
    | distributedPlacementDeclaration
    | distributedRemoteExecutionDeclaration
    | distributedDeploymentDeclaration
    | distributedCoordinationDeclaration
    | distributedDependencyDeclaration
    | distributedPolicyDeclaration
    | distributedRequirementDeclaration
    ;


/* ============================================================================
 * 3. DISTRIBUTED SCOPE
 * ============================================================================
 *
 * A scope groups distributed intent without requiring a particular deployment
 * topology.
 */

distributedScope
    : identifier
      LBRACE
      distributedMember*
      RBRACE
    ;


/* ============================================================================
 * 4. COMMON DECLARATION PREFIX
 * ============================================================================
 *
 * Distributed declarations use source-level names.
 *
 * Semantic validation determines whether the name is legal in context.
 */

distributedNamedDeclaration
    : identifier
      identifier
    ;


/* ============================================================================
 * 5. GENERIC DISTRIBUTED ENTITY
 * ============================================================================
 *
 * Examples of semantic entities represented by this structure:
 *
 *     distributed node
 *     distributed worker
 *     distributed process
 *     distributed partition
 *     distributed shard
 *     distributed region
 *
 * The grammar does not close the entity vocabulary.
 */

distributedEntityDeclaration
    : identifier
      identifier
      distributedDeclarationBody?
    ;

distributedDeclarationBody
    : LBRACE
      distributedMember*
      RBRACE
    ;


/* ============================================================================
 * 6. TASK
 * ============================================================================
 *
 * A distributed task describes a unit of computation.
 *
 * It does not determine where the task executes.
 */

distributedTaskDeclaration
    : identifier
      identifier
      distributedTaskSignature?
      distributedDeclarationBody?
    ;

distributedTaskSignature
    : LPAREN distributedParameterList? RPAREN
      distributedTaskReturnType?
    ;

distributedTaskReturnType
    : THIN_ARROW expression
    ;

distributedParameterList
    : distributedParameter
      (COMMA distributedParameter)*
      COMMA?
    ;

distributedParameter
    : identifier
      (COLON expression)?
      (ASSIGN expression)?
    ;


/* ============================================================================
 * 7. SERVICE
 * ============================================================================
 *
 * A service is a source-level distributed callable abstraction.
 *
 * It does not imply:
 *
 *     - server count;
 *     - process count;
 *     - machine count;
 *     - network protocol;
 *     - provider;
 *     - address.
 */

distributedServiceDeclaration
    : identifier
      identifier
      distributedServiceSignature?
      distributedDeclarationBody?
    ;

distributedServiceSignature
    : LPAREN distributedParameterList? RPAREN
      distributedServiceReturnType?
    ;

distributedServiceReturnType
    : THIN_ARROW expression
    ;


/* ============================================================================
 * 8. ACTOR
 * ============================================================================
 *
 * Actor syntax describes a logical actor abstraction.
 *
 * Actor scheduling, placement, isolation and mailbox implementation are
 * downstream concerns.
 */

distributedActorDeclaration
    : identifier
      identifier
      distributedActorBody?
    ;

distributedActorBody
    : LBRACE
      distributedActorMember*
      RBRACE
    ;

distributedActorMember
    : distributedStateDeclaration
    | distributedTaskDeclaration
    | distributedServiceDeclaration
    | distributedCommunicationDeclaration
    | distributedDependencyDeclaration
    | distributedPolicyDeclaration
    ;


/* ============================================================================
 * 9. CHANNEL
 * ============================================================================
 *
 * A channel expresses communication intent.
 *
 * Transport remains outside the grammar.
 */

distributedChannelDeclaration
    : identifier
      identifier
      distributedChannelBody?
    ;

distributedChannelBody
    : LBRACE
      distributedChannelMember*
      RBRACE
    ;

distributedChannelMember
    : distributedEndpointClause
    | distributedMessageClause
    | distributedPolicyDeclaration
    | distributedRequirementDeclaration
    | distributedAttributeClause
    ;

distributedEndpointClause
    : identifier
      distributedReferenceList
      SEMICOLON
    ;

distributedMessageClause
    : identifier
      distributedTypeOrExpression
      SEMICOLON
    ;


/* ============================================================================
 * 10. STATE
 * ============================================================================
 *
 * Distributed state is a semantic abstraction.
 */

distributedStateDeclaration
    : identifier
      identifier
      distributedStateBody?
    ;

distributedStateBody
    : LBRACE
      distributedStateMember*
      RBRACE
    ;

distributedStateMember
    : distributedInitializationClause
    | distributedReplicationDeclaration
    | distributedConsistencyDeclaration
    | distributedRequirementDeclaration
    | distributedPolicyDeclaration
    | distributedAttributeClause
    ;

distributedInitializationClause
    : identifier
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. COMMUNICATION
 * ============================================================================
 *
 * Generic communication construct.
 *
 * This remains intentionally transport-neutral.
 */

distributedCommunicationDeclaration
    : identifier
      distributedCommunicationOperation
      distributedCommunicationArguments?
      SEMICOLON
    ;

distributedCommunicationOperation
    : identifier
    ;

distributedCommunicationArguments
    : LPAREN
      distributedArgumentList?
      RPAREN
    ;

distributedArgumentList
    : distributedArgument
      (COMMA distributedArgument)*
      COMMA?
    ;

distributedArgument
    : expression
    | qualifiedName
    ;


/* ============================================================================
 * 12. SEND / RECEIVE-STYLE INTENT
 * ============================================================================
 *
 * These wrappers do not enumerate the operation names.
 *
 * Semantic analysis may classify:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::gather
 *     distributed::scatter
 *     distributed::reduce
 *     distributed::publish
 *     distributed::subscribe
 *
 * or future operations.
 */

distributedSend
    : identifier
      distributedArgumentList?
      SEMICOLON
    ;

distributedReceive
    : identifier
      distributedArgumentList?
      SEMICOLON
    ;


/* ============================================================================
 * 13. REPLICATION
 * ============================================================================
 *
 * Replication intent is represented without imposing a fixed replica count.
 */

distributedReplicationDeclaration
    : identifier
      distributedReplicationBody?
    ;

distributedReplicationBody
    : LBRACE
      distributedReplicationMember*
      RBRACE
    ;

distributedReplicationMember
    : distributedReplicaRequirement
    | distributedReplicationPolicy
    | distributedConsistencyDeclaration
    | distributedPlacementDeclaration
    | distributedRequirementDeclaration
    | distributedAttributeClause
    ;

distributedReplicaRequirement
    : identifier
      expression
      SEMICOLON
    ;

distributedReplicationPolicy
    : identifier
      expression?
      SEMICOLON
    ;


/* ============================================================================
 * 14. CONSISTENCY
 * ============================================================================
 *
 * Consistency names remain open-ended.
 */

distributedConsistencyDeclaration
    : identifier
      distributedConsistencyBody?
    ;

distributedConsistencyBody
    : LBRACE
      distributedConsistencyMember*
      RBRACE
    ;

distributedConsistencyMember
    : distributedConsistencyPolicy
    | distributedRequirementDeclaration
    | distributedDependencyDeclaration
    | distributedAttributeClause
    ;

distributedConsistencyPolicy
    : identifier
      expression?
      SEMICOLON
    ;


/* ============================================================================
 * 15. PARTITIONING
 * ============================================================================
 *
 * Partitioning describes logical data/work partitioning.
 *
 * It does not select physical machines.
 */

distributedPartitionDeclaration
    : identifier
      identifier
      distributedPartitionBody?
    ;

distributedPartitionBody
    : LBRACE
      distributedPartitionMember*
      RBRACE
    ;

distributedPartitionMember
    : distributedPartitionKey
    | distributedPartitionPolicy
    | distributedRequirementDeclaration
    | distributedAttributeClause
    ;

distributedPartitionKey
    : identifier
      expression
      SEMICOLON
    ;

distributedPartitionPolicy
    : identifier
      expression?
      SEMICOLON
    ;


/* ============================================================================
 * 16. PLACEMENT
 * ============================================================================
 *
 * Placement expresses constraints/preferences.
 *
 * It does not directly select a physical target.
 */

distributedPlacementDeclaration
    : identifier
      distributedPlacementBody?
    ;

distributedPlacementBody
    : LBRACE
      distributedPlacementMember*
      RBRACE
    ;

distributedPlacementMember
    : distributedPlacementConstraint
    | distributedPlacementPreference
    | distributedRequirementDeclaration
    | distributedAttributeClause
    ;

distributedPlacementConstraint
    : identifier
      expression
      SEMICOLON
    ;

distributedPlacementPreference
    : identifier
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 17. REMOTE EXECUTION
 * ============================================================================
 *
 * Remote execution is an intent, not a host/address declaration.
 */

distributedRemoteExecutionDeclaration
    : identifier
      distributedRemoteExecutionBody?
    ;

distributedRemoteExecutionBody
    : LBRACE
      distributedRemoteExecutionMember*
      RBRACE
    ;

distributedRemoteExecutionMember
    : distributedExecutionTarget
    | distributedExecutionArguments
    | distributedRequirementDeclaration
    | distributedPlacementDeclaration
    | distributedPolicyDeclaration
    | distributedAttributeClause
    ;

distributedExecutionTarget
    : identifier
      qualifiedName
      SEMICOLON
    ;

distributedExecutionArguments
    : identifier
      distributedArgumentList?
      SEMICOLON
    ;


/* ============================================================================
 * 18. DEPLOYMENT
 * ============================================================================
 *
 * Deployment describes desired realization properties.
 *
 * It does not encode a fixed cluster.
 */

distributedDeploymentDeclaration
    : identifier
      distributedDeploymentBody?
    ;

distributedDeploymentBody
    : LBRACE
      distributedDeploymentMember*
      RBRACE
    ;

distributedDeploymentMember
    : distributedPlacementDeclaration
    | distributedRequirementDeclaration
    | distributedPolicyDeclaration
    | distributedDependencyDeclaration
    | distributedAttributeClause
    ;


/* ============================================================================
 * 19. COORDINATION
 * ============================================================================
 */

distributedCoordinationDeclaration
    : identifier
      distributedCoordinationBody?
    ;

distributedCoordinationBody
    : LBRACE
      distributedCoordinationMember*
      RBRACE
    ;

distributedCoordinationMember
    : distributedDependencyDeclaration
    | distributedCommunicationDeclaration
    | distributedConsistencyDeclaration
    | distributedRequirementDeclaration
    | distributedPolicyDeclaration
    | distributedAttributeClause
    ;


/* ============================================================================
 * 20. DEPENDENCIES
 * ============================================================================
 *
 * Dependency structure is semantic dependency information.
 *
 * It does not imply a particular scheduler.
 */

distributedDependencyDeclaration
    : identifier
      distributedReferenceList
      SEMICOLON
    ;

distributedReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;


/* ============================================================================
 * 21. REQUIREMENTS
 * ============================================================================
 *
 * Requirements express what a valid realization needs.
 *
 * They do not allocate resources.
 */

distributedRequirementDeclaration
    : identifier
      distributedRequirementExpression
      SEMICOLON
    ;

distributedRequirementExpression
    : expression
    ;


/* ============================================================================
 * 22. POLICIES
 * ============================================================================
 *
 * Policy syntax remains open-ended.
 *
 * Policy interpretation belongs to semantic/compiler/runtime policy systems.
 */

distributedPolicyDeclaration
    : identifier
      distributedPolicyValue?
      SEMICOLON
    ;

distributedPolicyValue
    : expression
    | qualifiedName
    ;


/* ============================================================================
 * 23. ATTRIBUTES
 * ============================================================================
 *
 * Domain-specific attributes are represented structurally.
 */

distributedAttributeClause
    : identifier
      (LPAREN distributedArgumentList? RPAREN)?
      SEMICOLON
    ;


/* ============================================================================
 * 24. GENERIC DISTRIBUTED REFERENCES
 * ============================================================================
 *
 * Qualified names are owned by Names.
 *
 * This grammar does not recreate qualified-name syntax.
 */

distributedReference
    : qualifiedName
    ;

distributedQualifiedReference
    : qualifiedName
    ;

distributedQualifiedReferenceList
    : distributedQualifiedReference
      (COMMA distributedQualifiedReference)*
      COMMA?
    ;


/* ============================================================================
 * 25. TYPE / EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Distributed grammar must not create a second type grammar.
 *
 * Where a distributed construct needs a value/type-like expression, the
 * canonical expression grammar is consumed.
 */

distributedTypeOrExpression
    : expression
    ;


/* ============================================================================
 * 26. DISTRIBUTED INVOCATION
 * ============================================================================
 *
 * Invocation remains target-independent.
 */

distributedInvocation
    : qualifiedName
      LPAREN
      distributedArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 27. DISTRIBUTED FUTURES
 * ============================================================================
 *
 * A future is a semantic asynchronous result abstraction.
 *
 * It does not prescribe a particular runtime implementation.
 */

distributedFuture
    : identifier
      distributedInvocation
    ;


/* ============================================================================
 * 28. DISTRIBUTED TASK DEPENDENCY
 * ============================================================================
 *
 * A task may depend on an arbitrary number of other logical tasks.
 */

distributedTaskDependency
    : identifier
      distributedReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 29. DISTRIBUTED DATA FLOW
 * ============================================================================
 */

distributedDataFlow
    : qualifiedName
      identifier
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 30. DISTRIBUTED PIPELINE
 * ============================================================================
 */

distributedPipeline
    : identifier
      identifier
      LBRACE
      distributedPipelineStage*
      RBRACE
    ;

distributedPipelineStage
    : identifier
      distributedReferenceList?
      SEMICOLON
    ;


/* ============================================================================
 * 31. DISTRIBUTED PARTITION / SHARD / REPLICA REFERENCES
 * ============================================================================
 *
 * These remain symbolic.
 */

distributedPartitionReference
    : qualifiedName
    ;

distributedShardReference
    : qualifiedName
    ;

distributedReplicaReference
    : qualifiedName
    ;


/* ============================================================================
 * 32. DISTRIBUTED NAMESPACE REFERENCE
 * ============================================================================
 *
 * Semantic validation may classify the first qualified-name component as:
 *
 *     distributed
 *
 * without introducing a dedicated lexer token.
 */

distributedNamespaceReference
    : qualifiedName
    ;


/* ============================================================================
 * 33. DISTRIBUTED DOMAIN PATH
 * ============================================================================
 *
 * Examples:
 *
 *     distributed::node
 *     distributed::service
 *     distributed::communication
 *     distributed::future::operation
 *
 * The canonical qualified-name syntax remains owned by Names.
 */

distributedDomainReference
    : qualifiedName
    ;


/* ============================================================================
 * 34. OPTIONAL DISTRIBUTED EXPRESSION
 * ============================================================================
 */

optionalDistributedExpression
    : expression?
    ;


/* ============================================================================
 * 35. OPTIONAL DISTRIBUTED REFERENCE
 * ============================================================================
 */

optionalDistributedReference
    : qualifiedName?
    ;


/* ============================================================================
 * 36. DISTRIBUTED LISTS
 * ============================================================================
 *
 * All lists are unbounded by language design.
 */

distributedExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;

distributedNameList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;


/* ============================================================================
 * 37. DISTRIBUTED SCOPE BODY
 * ============================================================================
 */

distributedBody
    : LBRACE
      distributedMember*
      RBRACE
    ;


/* ============================================================================
 * 38. DISTRIBUTED DECLARATION WITH OPTIONS
 * ============================================================================
 *
 * Generic extensibility boundary.
 */

distributedConfigurableDeclaration
    : identifier
      identifier
      distributedOptionList?
      distributedBody?
    ;

distributedOptionList
    : distributedOption
      (COMMA distributedOption)*
      COMMA?
    ;

distributedOption
    : identifier
      (ASSIGN expression)?
    ;


/* ============================================================================
 * 39. DISTRIBUTED RESOURCE REQUIREMENT REFERENCE
 * ============================================================================
 *
 * This is intentionally a symbolic reference.
 *
 * Actual resource quantities/capabilities belong to the canonical resource
 * subsystem.
 */

distributedResourceReference
    : qualifiedName
    ;


/* ============================================================================
 * 40. DISTRIBUTED CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability discovery and validation are semantic operations.
 */

distributedCapabilityReference
    : qualifiedName
    ;


/* ============================================================================
 * 41. DISTRIBUTED EFFECT REFERENCE
 * ============================================================================
 *
 * Generic effect classification remains compatible with:
 *
 *     grammar/effects/distributed.g4
 *
 * This grammar does not redefine the generic effect system.
 */

distributedEffectReference
    : qualifiedName
    ;


/* ============================================================================
 * 42. DISTRIBUTED SECURITY REFERENCE
 * ============================================================================
 *
 * Security authorization is not granted by this grammar.
 */

distributedSecurityReference
    : qualifiedName
    ;


/* ============================================================================
 * 43. DISTRIBUTED FAILURE / RECOVERY INTENT
 * ============================================================================
 *
 * These are source-level semantic references only.
 */

distributedFailurePolicyReference
    : qualifiedName
    ;

distributedRecoveryPolicyReference
    : qualifiedName
    ;


/* ============================================================================
 * 44. DISTRIBUTED OBSERVABILITY REFERENCE
 * ============================================================================
 */

distributedTelemetryReference
    : qualifiedName
    ;


/* ============================================================================
 * 45. DISTRIBUTED VERSIONED REFERENCE
 * ============================================================================
 *
 * Version semantics belong to the canonical version/compatibility subsystem.
 */

distributedVersionedReference
    : qualifiedName
    ;


/* ============================================================================
 * 46. FUTURE-PROOF EXTENSION POINT
 * ============================================================================
 *
 * A future distributed construct may be represented by a qualified semantic
 * name without requiring a grammar keyword.
 */

distributedExtensionReference
    : qualifiedName
    ;


/* ============================================================================
 * 47. COMPOSITION CONTRACT
 * ============================================================================
 *
 * Higher-level parser composition may consume:
 *
 *     distributedDeclaration
 *
 * directly.
 *
 * The canonical Zamani parser remains responsible for deciding where a
 * distributed declaration is legal in a complete source program.
 *
 * This file therefore does not redefine:
 *
 *     program
 *     sourceElement
 *     item
 *     declaration
 *     statement
 *
 * ============================================================================
 * 48. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve the syntactic categories represented here.
 *
 * Conceptually:
 *
 *     DistributedProgram
 *     DistributedScope
 *     DistributedEntity
 *     DistributedTask
 *     DistributedService
 *     DistributedActor
 *     DistributedChannel
 *     DistributedState
 *     DistributedCommunication
 *     DistributedReplication
 *     DistributedConsistency
 *     DistributedPartition
 *     DistributedPlacement
 *     DistributedRemoteExecution
 *     DistributedDeployment
 *     DistributedCoordination
 *     DistributedDependency
 *
 * The exact Rust AST types are NOT defined by this grammar.
 *
 * ============================================================================
 * 49. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether a declaration is actually distributed;
 *     - whether names are valid;
 *     - whether referenced entities exist;
 *     - whether dependencies are valid;
 *     - whether communication is type-safe;
 *     - whether consistency requirements are coherent;
 *     - whether replication requirements are satisfiable;
 *     - whether placement requirements are satisfiable;
 *     - whether resource requirements are satisfiable;
 *     - whether capability requirements are available;
 *     - whether security requirements are authorized;
 *     - whether quantum/classical boundaries are legal;
 *     - whether a realization preserves program semantics.
 *
 * None of those checks occur in this grammar.
 *
 * ============================================================================
 * 50. COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler flow:
 *
 *     Distributed AST
 *          |
 *          v
 *     semantic distributed model
 *          |
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> communication analysis
 *          +--> consistency analysis
 *          +--> placement analysis
 *          +--> security analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> hardware/HDL representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     runtime realization
 *
 * ============================================================================
 * 51. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may consume the semantic representation to:
 *
 *     - discover available execution resources;
 *     - negotiate capabilities;
 *     - select valid placements;
 *     - establish communication;
 *     - dispatch tasks;
 *     - manage distributed state;
 *     - monitor execution;
 *     - report failures.
 *
 * Runtime MUST NOT depend directly on parser-specific implementation details.
 *
 * ============================================================================
 * 52. NETWORK INTEGRATION
 * ============================================================================
 *
 * Network subsystem consumes semantic communication intent.
 *
 * This grammar does not choose:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     custom transport
 *
 * or any future transport.
 *
 * ============================================================================
 * 53. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis consumes symbolic resource/capability references.
 *
 * Physical availability is evaluated downstream.
 *
 * There is no grammar-level:
 *
 *     MAX_NODES
 *     MAX_SERVICES
 *     MAX_WORKERS
 *     MAX_REPLICAS
 *
 * ============================================================================
 * 54. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * This grammar may express:
 *
 *     dependency;
 *     task relationship;
 *     ordering intent;
 *     coordination intent.
 *
 * It does not construct a schedule.
 *
 * The scheduling subsystem remains responsible for:
 *
 *     ordering;
 *     timing;
 *     resource conflicts;
 *     synchronization;
 *     placement-aware execution.
 *
 * ============================================================================
 * 55. ROUTING INTEGRATION
 * ============================================================================
 *
 * This grammar does not select communication routes.
 *
 * Distributed routing remains a downstream realization problem.
 *
 * ============================================================================
 * 56. RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Distributed failure/recovery intent may be preserved in the semantic model.
 *
 * Resilience decides:
 *
 *     retry;
 *     restart;
 *     resume;
 *     rollback;
 *     reroute;
 *     remap;
 *     reschedule;
 *     switch backend;
 *     quarantine;
 *     abort.
 *
 * This grammar does not implement those decisions.
 *
 * ============================================================================
 * 57. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed quantum computation follows:
 *
 *     Zamani source
 *          |
 *          v
 *     distributed syntax
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     quantum semantic lowering
 *          |
 *          v
 *     quantum::ir
 *
 * No quantum gate, qubit, topology, QEC or ZQN representation is duplicated
 * here.
 *
 * ============================================================================
 * 58. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given the same token stream, this grammar produces the same parse result.
 *
 * No:
 *
 *     actions
 *     predicates
 *     randomness
 *     environment queries
 *     filesystem access
 *     network access
 *     hardware queries
 *
 * are permitted.
 *
 * ============================================================================
 * 59. SECURITY CONTRACT
 * ============================================================================
 *
 * A distributed declaration does not grant:
 *
 *     network permission;
 *     process permission;
 *     node permission;
 *     deployment permission;
 *     cloud permission;
 *     hardware permission.
 *
 * Authorization remains a security/runtime concern.
 *
 * ============================================================================
 * 60. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It uses the canonical ZamaniLexer.
 *     [x] It consumes canonical Names.
 *     [x] It consumes canonical Expressions.
 *     [x] It does not duplicate lexical rules.
 *     [x] It does not duplicate expression syntax.
 *     [x] It does not define machine limits.
 *     [x] It does not define node limits.
 *     [x] It does not define topology.
 *     [x] It does not define network transport.
 *     [x] It does not define runtime behavior.
 *     [x] It does not define quantum::ir.
 *     [x] It does not define QEC.
 *     [x] It does not define ZQN.
 *     [x] It remains open to future distributed abstractions.
 *     [x] It supports arbitrary distributed declaration nesting.
 *     [x] It preserves semantic intent for downstream compilation.
 *     [x] It contains no embedded unsafe Rust.
 *     [x] It contains no embedded Rust at all.
 *
 * Required external validation:
 *
 *     - ANTLR grammar generation;
 *     - parser integration;
 *     - AST construction;
 *     - positive distributed syntax tests;
 *     - negative syntax tests;
 *     - scalability tests;
 *     - cross-domain tests;
 *     - quantum/distributed integration tests;
 *     - classical/distributed integration tests;
 *     - HDL/distributed integration tests;
 *     - resource/capability integration tests;
 *     - deterministic parse tests.
 *
 * ============================================================================
 */