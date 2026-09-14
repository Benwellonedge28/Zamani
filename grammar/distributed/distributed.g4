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
 *     Production distributed-computing parser grammar.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No mutable global parser state.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL DISTRIBUTED COMPUTATION SYNTAX.
 *
 * It describes distributed computational intent and provides a stable syntax
 * boundary between Zamani source code and downstream semantic analysis.
 *
 * It does NOT implement:
 *
 *     - distributed execution;
 *     - process scheduling;
 *     - service discovery;
 *     - node discovery;
 *     - network transport;
 *     - routing;
 *     - placement;
 *     - deployment;
 *     - replication algorithms;
 *     - consensus algorithms;
 *     - consistency algorithms;
 *     - fault-tolerance algorithms;
 *     - distributed storage;
 *     - cloud-provider integration;
 *     - hardware discovery;
 *     - quantum routing;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * Those concerns belong to their respective repository subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 *     SOURCE
 *        |
 *        v
 *     LEXER
 *        |
 *        v
 *     PARSER
 *        |
 *        v
 *     FRONTEND AST
 *        |
 *        +--> name resolution
 *        +--> type checking
 *        +--> effect checking
 *        +--> capability checking
 *        +--> resource analysis
 *        +--> security analysis
 *        +--> distributed semantic validation
 *        |
 *        v
 *     CANONICAL SEMANTIC REPRESENTATION
 *        |
 *        +--> classical IR
 *        +--> quantum::ir
 *        +--> HDL/hardware representation
 *        +--> distributed execution metadata
 *        +--> resource requirements
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     placement / routing / scheduling
 *        |
 *        v
 *     target realization
 *        |
 *        v
 *     runtime
 *
 * The grammar never directly constructs or modifies an IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed syntax follows:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * Source code describes semantic intent.
 *
 * Source code MUST NOT require rewriting merely because execution moves
 * between:
 *
 *     - one machine;
 *     - many machines;
 *     - embedded systems;
 *     - clusters;
 *     - clouds;
 *     - HPC systems;
 *     - heterogeneous systems;
 *     - quantum/classical systems;
 *     - future execution architectures.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Distributed domain concepts are represented primarily through qualified
 * names rather than a closed lexer keyword inventory.
 *
 * Examples:
 *
 *     distributed::node
 *     distributed::service
 *     distributed::worker
 *     distributed::actor
 *     distributed::task
 *     distributed::channel
 *     distributed::message
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::scatter
 *     distributed::gather
 *     distributed::reduce
 *     distributed::replication
 *     distributed::consistency
 *     distributed::partition
 *     distributed::placement
 *     distributed::remote_execution
 *     distributed::migration
 *     distributed::coordination
 *     distributed::consensus
 *     distributed::fault_tolerance
 *     distributed::recovery
 *     distributed::availability
 *
 * A future extension such as:
 *
 *     distributed::future_protocol
 *
 * remains syntactically representable without adding a lexer keyword.
 *
 * Semantic validation determines whether a particular name is defined,
 * supported, deprecated, experimental, vendor-specific, or unknown.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed source-level containers;
 *     - distributed named entities;
 *     - distributed executable operations;
 *     - distributed bindings;
 *     - distributed relationships;
 *     - distributed dependencies;
 *     - distributed clauses;
 *     - distributed blocks;
 *     - distributed argument lists;
 *     - distributed semantic attributes;
 *     - distributed source-level composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - general expression syntax;
 *     - general type syntax;
 *     - memory ownership;
 *     - generic concurrency primitives;
 *     - network protocol syntax;
 *     - hardware topology;
 *     - physical resources;
 *     - device identifiers;
 *     - resource allocation;
 *     - target selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
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
 *          |      |
 *          |      +--> identifier
 *          |      +--> qualifiedName
 *          |
 *          +--> Expressions
 *                 |
 *                 +--> expression
 *                 +--> expressionList
 *                 +--> optionalExpressionList
 *          |
 *          v
 *     Distributed
 *
 * This grammar MUST NOT redefine:
 *
 *     IDENTIFIER
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     operators
 *     punctuation
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite machine/resource limits are encoded.
 *
 * There is deliberately no:
 *
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_SERVICES
 *     MAX_ACTORS
 *     MAX_TASKS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_REPLICAS
 *     MAX_SHARDS
 *     MAX_REGIONS
 *     MAX_DEVICES
 *     MAX_NETWORKS
 *     MAX_CLUSTER_SIZE
 *
 * Nor does this grammar encode:
 *
 *     CPU counts
 *     GPU counts
 *     QPU counts
 *     memory sizes
 *     network bandwidth
 *     network addresses
 *     ports
 *     topology sizes
 *     machine IDs
 *     provider IDs
 *     device IDs
 *
 * Repetition is represented by ANTLR's `*` and `+`.
 *
 * Practical resource limits are owned by:
 *
 *     parser resource policy;
 *     compiler resource policy;
 *     semantic validation;
 *     resource manager;
 *     scheduler;
 *     deployment;
 *     runtime.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser determines:
 *
 *     "Is the source structurally valid distributed syntax?"
 *
 * It does not determine:
 *
 *     "Can this deployment actually be realized?"
 *
 *     "Which node executes this task?"
 *
 *     "Which transport is used?"
 *
 *     "How many replicas exist?"
 *
 *     "Which consistency algorithm is selected?"
 *
 *     "Which scheduler is selected?"
 *
 *     "Which hardware is selected?"
 *
 *     "Which cloud provider is selected?"
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed quantum programs may use this grammar to express:
 *
 *     distributed execution;
 *     logical-state placement;
 *     communication intent;
 *     coordination;
 *     remote operations;
 *     distributed measurement/control intent.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     pulse semantics
 *     QEC algorithms
 *     ZQN noise models
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Distributed syntax may coexist with:
 *
 *     classical computation;
 *     quantum computation;
 *     HDL;
 *     hardware descriptions;
 *     accelerator descriptions.
 *
 * Hardware-specific realization remains outside this grammar.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Distributed communication expresses intent.
 *
 * For example:
 *
 *     distributed::send(channel, value, destination);
 *
 * does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor-specific transport
 *
 * The networking subsystem chooses a valid realization.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no environment queries;
 *     - no hardware queries.
 *
 * Parsing therefore depends only on the supplied token stream.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * Frontend AST construction must preserve:
 *
 *     - source ordering;
 *     - declaration ordering;
 *     - qualified-name segment ordering;
 *     - expression structure;
 *     - argument ordering;
 *     - clause ordering;
 *     - nested block structure;
 *     - source spans.
 *
 * Semantic canonicalization occurs after parsing.
 *
 * ============================================================================
 */

parser grammar Distributed;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule is the stable entry point used by the higher-level Zamani parser
 * when a distributed declaration or distributed statement is expected.
 *
 * No finite number of distributed declarations is imposed.
 */
distributedDeclaration
    : distributedContainerDeclaration
    | distributedNamedDeclaration
    | distributedOperation
    | distributedBinding
    | distributedRelationship
    | distributedDependency
    | distributedBlock
    ;


/*
 * ============================================================================
 * 2. DISTRIBUTED CONTAINER
 * ============================================================================
 *
 * General form:
 *
 *     distributed::node worker {
 *         ...
 *     }
 *
 *     distributed::service api {
 *         ...
 *     }
 *
 *     distributed::actor processor {
 *         ...
 *     }
 *
 *     distributed::cluster logical_group {
 *         ...
 *     }
 *
 * The first qualified name is a semantic kind.
 *
 * The parser intentionally does not enumerate all possible kinds.
 *
 * This is what allows future distributed abstractions to be introduced
 * without changing the lexical vocabulary.
 */
distributedContainerDeclaration
    : distributedKind distributedEntityName distributedBlock
    ;


/*
 * Semantic kind.
 *
 * Examples:
 *
 *     distributed::node
 *     distributed::service
 *     distributed::task
 *     distributed::worker
 *     distributed::actor
 *     distributed::channel
 *     distributed::state
 *     distributed::partition
 *     distributed::replication
 *     distributed::deployment
 *
 * Semantic analysis owns classification.
 */
distributedKind
    : qualifiedName
    ;


/*
 * Entity name.
 *
 * Entity names are ordinary Zamani identifiers.
 *
 * This grammar does not create NodeName, ServiceName, WorkerName, etc.
 */
distributedEntityName
    : identifier
    ;


/*
 * ============================================================================
 * 3. GENERIC NAMED DISTRIBUTED DECLARATION
 * ============================================================================
 *
 * General form:
 *
 *     distributed::task compute;
 *
 *     distributed::service api;
 *
 *     distributed::worker worker;
 *
 *     distributed::future::entity object;
 *
 * The semantic layer determines the meaning of the qualified kind.
 */
distributedNamedDeclaration
    : distributedKind distributedEntityName SEMICOLON
    ;


/*
 * ============================================================================
 * 4. DISTRIBUTED OPERATION
 * ============================================================================
 *
 * General form:
 *
 *     distributed::send(channel, value, destination);
 *
 *     distributed::receive(channel);
 *
 *     distributed::broadcast(message, group);
 *
 *     distributed::migrate(task, target);
 *
 *     distributed::checkpoint(state);
 *
 *     distributed::recover(state);
 *
 *     distributed::coordinate(group);
 *
 *     distributed::consensus(proposal);
 *
 * The operation name is open-world.
 *
 * Future distributed operations therefore do not require grammar changes.
 */
distributedOperation
    : distributedOperationName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


distributedOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. DISTRIBUTED BINDING
 * ============================================================================
 *
 * General form:
 *
 *     distributed::endpoint endpoint = expression;
 *
 *     distributed::state state = expression;
 *
 *     distributed::channel channel = expression;
 *
 *     distributed::future result = expression;
 *
 * The type/meaning of the left-hand qualified kind belongs to semantic
 * analysis.
 */
distributedBinding
    : distributedKind
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. DISTRIBUTED RELATIONSHIPS
 * ============================================================================
 *
 * Relationships express source-level relationships between named entities.
 *
 * Examples:
 *
 *     distributed::depends_on(task_a, task_b);
 *
 *     distributed::connect(channel, producer, consumer);
 *
 *     distributed::replica_of(copy, original);
 *
 *     distributed::placed_within(worker, region);
 *
 *     distributed::coordinates(controller, group);
 *
 * The actual semantics are downstream.
 */
distributedRelationship
    : distributedRelationshipName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


distributedRelationshipName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. DISTRIBUTED DEPENDENCIES
 * ============================================================================
 *
 * Explicit dependency syntax:
 *
 *     distributed::task_a -> distributed::task_b;
 *
 *     distributed::stage_a -> distributed::stage_b;
 *
 * No machine topology is implied.
 *
 * The arrow means semantic dependency/ordering intent.
 *
 * It does NOT mean:
 *
 *     network route;
 *     physical link;
 *     hardware connection;
 *     scheduling decision.
 */
distributedDependency
    : qualifiedName
      THIN_ARROW
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. DISTRIBUTED BLOCK
 * ============================================================================
 *
 * General form:
 *
 *     distributed::scope {
 *         ...
 *     }
 *
 * The block is intentionally recursive.
 *
 * This permits arbitrarily deep semantic composition subject only to
 * implementation/resource limits rather than a language-level fixed depth.
 */
distributedBlock
    : distributedBlockHeader
      LBRACE
      distributedMember*
      RBRACE
    ;


distributedBlockHeader
    : qualifiedName
    ;


/*
 * ============================================================================
 * 9. DISTRIBUTED MEMBER
 * ============================================================================
 *
 * The alternatives are deliberately structured around distinct syntactic
 * shapes.
 *
 * The grammar does NOT maintain separate closed alternatives such as:
 *
 *     nodeDeclaration
 *     serviceDeclaration
 *     workerDeclaration
 *     actorDeclaration
 *     ...
 *
 * because doing so would turn every future distributed abstraction into a
 * grammar-maintenance operation.
 *
 * Instead:
 *
 *     semantic kind + structural form
 *
 * is parsed here and classified later.
 */
distributedMember
    : distributedContainerDeclaration
    | distributedNamedDeclaration
    | distributedOperation
    | distributedBinding
    | distributedRelationship
    | distributedDependency
    | distributedBlock
    ;


/*
 * ============================================================================
 * 10. ARGUMENT LIST
 * ============================================================================
 *
 * This rule is provided as a stable distributed-domain wrapper.
 *
 * It delegates expression syntax to the canonical expression grammar.
 */
distributedArgumentList
    : expressionList
    ;


optionalDistributedArgumentList
    : distributedArgumentList?
    ;


/*
 * ============================================================================
 * 11. DISTRIBUTED ENTITY REFERENCE
 * ============================================================================
 *
 * A distributed entity reference is syntactically a normal qualified name.
 *
 * Examples:
 *
 *     worker
 *     group::worker
 *     distributed::worker
 *     distributed::group::worker
 *
 * The semantic layer determines whether the reference denotes:
 *
 *     node
 *     service
 *     task
 *     actor
 *     channel
 *     state
 *     partition
 *     resource
 *     future
 *     deployment
 *     or another distributed object.
 */
distributedEntityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 12. DISTRIBUTED NAME LIST
 * ============================================================================
 *
 * There is no finite cardinality.
 */
distributedEntityReferenceList
    : distributedEntityReference
      (COMMA distributedEntityReference)*
      COMMA?
    ;


optionalDistributedEntityReferenceList
    : distributedEntityReferenceList?
    ;


/*
 * ============================================================================
 * 13. DISTRIBUTED SEMANTIC OPTION
 * ============================================================================
 *
 * General form:
 *
 *     distributed::consistency = expression;
 *
 *     distributed::placement = expression;
 *
 *     distributed::reliability = expression;
 *
 *     distributed::availability = expression;
 *
 *     distributed::latency = expression;
 *
 * The grammar records structure only.
 *
 * It does not interpret the requested property.
 */
distributedOption
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. DISTRIBUTED OPTION BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     distributed::service api {
 *         distributed::consistency = policy;
 *         distributed::availability = requirement;
 *         distributed::placement = preference;
 *     }
 *
 * Options remain semantic declarations rather than deployment commands.
 */
distributedOptionBlock
    : distributedKind
      distributedEntityName
      LBRACE
      distributedOption*
      RBRACE
    ;


/*
 * ============================================================================
 * 15. REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * Resource requirements belong to the generic resource/capability subsystem.
 *
 * This grammar only provides the distributed syntactic wrapper.
 *
 * Examples:
 *
 *     distributed::requires(resource_expression);
 *
 *     distributed::capability(required_capability);
 *
 *     distributed::constraint(constraint_expression);
 *
 * There is no special syntax for:
 *
 *     CPU count
 *     GPU count
 *     QPU count
 *     node count
 *     memory size
 *     bandwidth
 *
 * unless such concepts are explicitly represented by the repository's
 * resource/capability semantic system.
 */
distributedRequirement
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. DISTRIBUTED POLICY
 * ============================================================================
 *
 * Policies remain declarative.
 *
 * Examples:
 *
 *     distributed::consistency(policy);
 *     distributed::replication(policy);
 *     distributed::failure_policy(policy);
 *     distributed::recovery(policy);
 *
 * No algorithm is implemented by the grammar.
 */
distributedPolicy
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. DISTRIBUTED COMMUNICATION
 * ============================================================================
 *
 * Communication is represented as an intent operation.
 *
 * Examples:
 *
 *     distributed::send(channel, value, destination);
 *     distributed::receive(channel);
 *     distributed::broadcast(value, group);
 *     distributed::scatter(value, group);
 *     distributed::gather(group);
 *     distributed::reduce(value, operation, group);
 *
 * Transport selection belongs to networking/runtime layers.
 */
distributedCommunication
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. DISTRIBUTED REPLICATION
 * ============================================================================
 *
 * Replication intent is represented structurally.
 *
 * Examples:
 *
 *     distributed::replication(state);
 *     distributed::replicate(state, policy);
 *
 * The grammar never requires a finite number of physical copies.
 */
distributedReplication
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. DISTRIBUTED CONSISTENCY
 * ============================================================================
 *
 * Consistency intent is declarative.
 *
 * Examples:
 *
 *     distributed::consistency(state, policy);
 *
 * The grammar does not implement:
 *
 *     Raft
 *     Paxos
 *     PBFT
 *     CRDT
 *     linearizability
 *     eventual consistency
 *     serializability
 *
 * as runtime algorithms.
 *
 * Those are semantic/runtime concerns.
 */
distributedConsistency
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. DISTRIBUTED PLACEMENT
 * ============================================================================
 *
 * Placement is an abstract requirement/preference.
 *
 * Examples:
 *
 *     distributed::placement(worker, preference);
 *     distributed::place(task, constraint);
 *
 * It MUST NOT directly identify a physical machine.
 */
distributedPlacement
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. REMOTE EXECUTION
 * ============================================================================
 *
 * Remote execution is represented as an operation.
 *
 * Examples:
 *
 *     distributed::remote_execution(task, requirement);
 *     distributed::invoke(service, arguments);
 *
 * The grammar does not contain:
 *
 *     host;
 *     IP address;
 *     port;
 *     cloud provider;
 *     machine ID;
 *     device ID.
 */
distributedRemoteExecution
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. DISTRIBUTED DEPLOYMENT
 * ============================================================================
 *
 * Deployment intent is represented without selecting a physical deployment.
 *
 * Examples:
 *
 *     distributed::deploy(service, requirement);
 *     distributed::deployment(application, policy);
 *
 * Target realization belongs to compilation/deployment/runtime layers.
 */
distributedDeployment
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. DISTRIBUTED COORDINATION
 * ============================================================================
 *
 * Coordination is semantic intent.
 *
 * Examples:
 *
 *     distributed::coordinate(group);
 *     distributed::barrier(group);
 *     distributed::consensus(proposal);
 *
 * No synchronization algorithm is encoded here.
 */
distributedCoordination
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. DISTRIBUTED FAILURE / RECOVERY INTENT
 * ============================================================================
 *
 * Failure and recovery are source-level intent only.
 *
 * Examples:
 *
 *     distributed::fault_tolerance(policy);
 *     distributed::recovery(state, policy);
 *     distributed::availability(requirement);
 *
 * Resilience owns actual recovery decisions and execution.
 */
distributedFailureIntent
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. DISTRIBUTED STATE
 * ============================================================================
 *
 * State syntax remains semantic-neutral.
 *
 * Examples:
 *
 *     distributed::state state = initial_value;
 *
 *     distributed::state state {
 *         ...
 *     }
 *
 * The grammar does not decide:
 *
 *     ownership;
 *     persistence;
 *     replication;
 *     consistency;
 *     serialization;
 *     synchronization;
 *     physical storage.
 */
distributedState
    : distributedKind
      identifier
      (
          ASSIGN expression SEMICOLON
        | distributedBlock
      )
    ;


/*
 * ============================================================================
 * 26. DISTRIBUTED TASK
 * ============================================================================
 *
 * Task syntax is represented using the open semantic-kind form.
 *
 * Examples:
 *
 *     distributed::task compute {
 *         ...
 *     }
 *
 *     distributed::task compute = callable;
 *
 * The semantic layer determines execution characteristics.
 */
distributedTask
    : distributedKind
      identifier
      (
          distributedBlock
        | ASSIGN expression SEMICOLON
      )
    ;


/*
 * ============================================================================
 * 27. DISTRIBUTED SERVICE
 * ============================================================================
 *
 * Examples:
 *
 *     distributed::service api {
 *         ...
 *     }
 *
 *     distributed::service worker;
 *
 * No service-discovery mechanism is encoded here.
 */
distributedService
    : distributedKind
      identifier
      (
          distributedBlock
        | SEMICOLON
      )
    ;


/*
 * ============================================================================
 * 28. DISTRIBUTED ACTOR
 * ============================================================================
 *
 * Actor semantics are delegated to the concurrency/runtime layers.
 */
distributedActor
    : distributedKind
      identifier
      distributedBlock
    ;


/*
 * ============================================================================
 * 29. DISTRIBUTED CHANNEL
 * ============================================================================
 *
 * Channel syntax is semantic intent.
 *
 * The grammar does not decide:
 *
 *     queue size;
 *     transport;
 *     memory location;
 *     network implementation;
 *     synchronization mechanism.
 */
distributedChannel
    : distributedKind
      identifier
      (
          distributedBlock
        | ASSIGN expression SEMICOLON
        | SEMICOLON
      )
    ;


/*
 * ============================================================================
 * 30. DISTRIBUTED MEMBER EXPANSION
 * ============================================================================
 *
 * This rule is the canonical recursive composition boundary.
 *
 * Domain-specific semantic classifiers can recognize:
 *
 *     node
 *     worker
 *     service
 *     actor
 *     task
 *     channel
 *     state
 *     partition
 *     shard
 *     replica
 *     deployment
 *     placement
 *     coordination
 *     recovery
 *     future constructs
 *
 * without requiring this grammar to maintain an ever-growing closed list of
 * keywords.
 */
distributedMemberExpanded
    : distributedContainerDeclaration
    | distributedNamedDeclaration
    | distributedOperation
    | distributedBinding
    | distributedRelationship
    | distributedDependency
    | distributedBlock
    ;


/*
 * ============================================================================
 * 31. TOP-LEVEL DISTRIBUTED SCOPE
 * ============================================================================
 *
 * This is the recommended source-level grouping form.
 *
 * Example:
 *
 *     distributed::program application {
 *         ...
 *     }
 *
 * The exact semantic interpretation of `distributed::program` belongs to
 * semantic analysis.
 */
distributedScope
    : distributedKind
      distributedEntityName
      distributedBlock
    ;


/*
 * ============================================================================
 * 32. DISTRIBUTED PROGRAM
 * ============================================================================
 *
 * Program syntax remains machine independent.
 *
 * Example:
 *
 *     distributed::program application {
 *         distributed::task compute {
 *             ...
 *         }
 *
 *         distributed::service api {
 *             ...
 *         }
 *     }
 */
distributedProgram
    : distributedKind
      distributedEntityName
      distributedBlock
    ;


/*
 * ============================================================================
 * 33. DISTRIBUTED DOMAIN EXPRESSION
 * ============================================================================
 *
 * Distributed-specific expressions remain ordinary Zamani expressions.
 *
 * This rule exists as an integration boundary so future semantic analysis can
 * identify distributed expressions without creating a second expression AST.
 */
distributedExpression
    : expression
    ;


/*
 * ============================================================================
 * 34. DISTRIBUTED EXPRESSION LIST
 * ============================================================================
 */
distributedExpressionList
    : expressionList
    ;


optionalDistributedExpressionList
    : optionalExpressionList
    ;


/*
 * ============================================================================
 * 35. DISTRIBUTED QUALIFIED KIND
 * ============================================================================
 *
 * This is intentionally an alias boundary rather than a duplicate name
 * implementation.
 */
distributedQualifiedKind
    : qualifiedName
    ;


/*
 * ============================================================================
 * 36. DISTRIBUTED IDENTIFIER
 * ============================================================================
 *
 * Distributed entities use the canonical Zamani identifier rule.
 */
distributedIdentifier
    : identifier
    ;


/*
 * ============================================================================
 * 37. DISTRIBUTED MEMBER LIST
 * ============================================================================
 *
 * No finite cardinality.
 */
distributedMemberList
    : distributedMember*
    ;


/*
 * ============================================================================
 * 38. DISTRIBUTED NON-EMPTY MEMBER LIST
 * ============================================================================
 */
distributedNonEmptyMemberList
    : distributedMember+
    ;


/*
 * ============================================================================
 * 39. DISTRIBUTED DECLARATION BODY
 * ============================================================================
 */
distributedDeclarationBody
    : LBRACE
      distributedMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 40. DISTRIBUTED ARGUMENT BODY
 * ============================================================================
 */
distributedArgumentBody
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 41. DISTRIBUTED INVOCATION
 * ============================================================================
 *
 * Explicitly named for higher-level parser composition.
 */
distributedInvocation
    : qualifiedName
      distributedArgumentBody
      SEMICOLON
    ;


/*
 * ============================================================================
 * 42. DISTRIBUTED REFERENCE
 * ============================================================================
 */
distributedReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 43. DISTRIBUTED REFERENCE ASSIGNMENT
 * ============================================================================
 */
distributedReferenceAssignment
    : distributedReference
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 44. DISTRIBUTED EDGE
 * ============================================================================
 *
 * Represents semantic dependency/order intent.
 */
distributedEdge
    : distributedReference
      THIN_ARROW
      distributedReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 45. DISTRIBUTED NESTED SCOPE
 * ============================================================================
 */
distributedNestedScope
    : distributedReference
      distributedDeclarationBody
    ;


/*
 * ============================================================================
 * 46. GENERIC DISTRIBUTED STATEMENT
 * ============================================================================
 *
 * This is the stable composition point for future statement-level integration.
 */
distributedStatement
    : distributedInvocation
    | distributedReferenceAssignment
    | distributedEdge
    | distributedNestedScope
    ;


/*
 * ============================================================================
 * 47. COMPLETE DISTRIBUTED MEMBER
 * ============================================================================
 *
 * This rule intentionally exposes the canonical union to parent grammars.
 */
distributedCompleteMember
    : distributedContainerDeclaration
    | distributedNamedDeclaration
    | distributedOperation
    | distributedBinding
    | distributedRelationship
    | distributedDependency
    | distributedBlock
    | distributedState
    | distributedTask
    | distributedService
    | distributedActor
    | distributedChannel
    | distributedInvocation
    | distributedReferenceAssignment
    | distributedEdge
    ;


/*
 * ============================================================================
 * 48. COMPLETE DISTRIBUTED SCOPE
 * ============================================================================
 */
distributedCompleteScope
    : distributedKind
      distributedEntityName
      LBRACE
      distributedCompleteMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 49. COMPLETE DISTRIBUTED PROGRAM
 * ============================================================================
 *
 * Stable top-level rule for a distributed-only compilation unit.
 */
distributedCompilationUnit
    : distributedCompleteScope
    ;


/*
 * ============================================================================
 * 50. SEMANTIC EXTENSION POINT
 * ============================================================================
 *
 * The grammar intentionally permits future distributed constructs through
 * qualified names and structural forms.
 *
 * Examples that require NO lexer modification:
 *
 *     distributed::mesh_group group { ... }
 *     distributed::federation federation { ... }
 *     distributed::quantum_network qnet { ... }
 *     distributed::entanglement_domain domain { ... }
 *     distributed::edge_region region { ... }
 *     distributed::neuromorphic_cluster cluster { ... }
 *     distributed::future_architecture system { ... }
 *
 * The semantic registry/capability layer determines whether such constructs
 * are defined.
 *
 * ============================================================================
 * END OF GRAMMAR
 * ============================================================================
 */