/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/replication.g4
 *
 * Grammar:
 *     Replication
 *
 * Status:
 *     Production distributed-replication parser component.
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
 * This file defines the SOURCE-LEVEL SYNTAX for distributed replication
 * intent in Zamani.
 *
 * Replication describes the logical requirement that a computation, service,
 * value, state abstraction, data abstraction, execution unit, or other
 * replication-capable entity may have multiple logical realizations.
 *
 * This grammar does NOT implement replication.
 *
 * It records syntax that downstream semantic and execution layers may
 * interpret into:
 *
 *     - replication requirements;
 *     - replication policies;
 *     - replica relationships;
 *     - replication factors;
 *     - consistency requirements;
 *     - availability requirements;
 *     - durability requirements;
 *     - synchronization requirements;
 *     - placement requirements;
 *     - resource requirements;
 *     - replication dependencies.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - replication declaration syntax;
 *     - logical replication names;
 *     - replication bodies;
 *     - replication properties;
 *     - logical replica declarations;
 *     - replica groups;
 *     - replication dependencies;
 *     - replication policy expressions;
 *     - replication factor expressions;
 *     - replication target expressions;
 *     - replication extension syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifier spelling;
 *     - qualified-name spelling;
 *     - general expressions;
 *     - general types;
 *     - memory management;
 *     - networking;
 *     - transport protocols;
 *     - node discovery;
 *     - service discovery;
 *     - hardware discovery;
 *     - resource discovery;
 *     - placement algorithms;
 *     - scheduling;
 *     - routing;
 *     - consensus;
 *     - consistency algorithms;
 *     - replication protocols;
 *     - storage engines;
 *     - serialization;
 *     - checkpoint implementation;
 *     - retry/recovery;
 *     - resilience;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HDL IR;
 *     - runtime execution;
 *     - provider-specific APIs.
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
 *     ZamaniLexer
 *          |
 *          v
 *     Replication parser component
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     name resolution
 *          |
 *          v
 *     type/effect/capability analysis
 *          |
 *          v
 *     distributed semantic analysis
 *          |
 *          v
 *     resource/constraint analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          |
 *          +--> quantum::ir
 *          |
 *          +--> HDL/hardware representation
 *          |
 *          +--> distributed execution metadata
 *          |
 *          v
 *     optimization / routing / scheduling
 *          |
 *          v
 *     deployment / runtime
 *
 * Grammar is therefore upstream of execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani follows:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Replication syntax must describe logical intent rather than temporary
 * physical deployment facts.
 *
 * This grammar therefore MUST NOT encode:
 *
 *     - maximum replicas;
 *     - maximum nodes;
 *     - maximum processes;
 *     - maximum workers;
 *     - maximum devices;
 *     - fixed cluster sizes;
 *     - fixed topology;
 *     - fixed hostnames;
 *     - fixed IP addresses;
 *     - fixed ports;
 *     - fixed machine identifiers;
 *     - fixed hardware identifiers;
 *     - fixed CPU counts;
 *     - fixed GPU counts;
 *     - fixed QPU counts;
 *     - fixed memory capacities;
 *     - fixed network capacities.
 *
 * A source program may explicitly request a semantic replication quantity,
 * but that quantity is a PROGRAM REQUIREMENT, not a statement about the
 * physical machine.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level finite limit is imposed on:
 *
 *     - replication declarations;
 *     - replica declarations;
 *     - replica groups;
 *     - replication properties;
 *     - dependencies;
 *     - nested policy blocks;
 *     - expression complexity;
 *     - qualified-name depth;
 *     - logical replica count;
 *     - replication factor expressions.
 *
 * ANTLR repetition operators are used instead of artificial finite bounds.
 *
 * Practical limits are external:
 *
 *     - parser memory;
 *     - compiler memory;
 *     - operating-system limits;
 *     - runtime resources;
 *     - deployment capacity;
 *     - available hardware;
 *     - network capacity;
 *     - provider policy.
 *
 * Such limits MUST NOT be encoded as language grammar limits.
 *
 * ============================================================================
 * REPLICATION IS NOT PLACEMENT
 * ============================================================================
 *
 * Replication describes logical multiplicity.
 *
 * Placement determines where a realization is physically located.
 *
 * Therefore:
 *
 *     replica primary
 *
 * does NOT mean:
 *
 *     machine 0
 *
 * and:
 *
 *     replica secondary
 *
 * does NOT mean:
 *
 *     machine 1
 *
 * No implicit physical mapping is permitted.
 *
 * ============================================================================
 * REPLICATION IS NOT CONSISTENCY
 * ============================================================================
 *
 * Replication and consistency are separate semantic dimensions.
 *
 * This grammar may carry:
 *
 *     consistency: policy;
 *
 * but does not implement:
 *
 *     - linearizability;
 *     - sequential consistency;
 *     - causal consistency;
 *     - eventual consistency;
 *     - quorum algorithms;
 *     - consensus;
 *     - transactional protocols.
 *
 * Those are downstream semantic/runtime responsibilities.
 *
 * ============================================================================
 * REPLICATION IS NOT FAULT TOLERANCE
 * ============================================================================
 *
 * Replication may contribute to fault tolerance, but this grammar does not
 * implement:
 *
 *     - failure detection;
 *     - failover;
 *     - retry;
 *     - restart;
 *     - recovery;
 *     - leader election;
 *     - replica repair;
 *     - checkpoint restoration.
 *
 * Those concerns belong to distributed runtime and resilience layers.
 *
 * ============================================================================
 * REPLICATION IS NOT BACKUP
 * ============================================================================
 *
 * A replication declaration MUST NOT implicitly mean:
 *
 *     - backup;
 *     - snapshot;
 *     - archival storage;
 *     - persistence;
 *     - cache;
 *     - checkpoint.
 *
 * Such semantics require explicit downstream interpretation.
 *
 * ============================================================================
 * REPLICATION FACTOR
 * ============================================================================
 *
 * A replication factor is represented by an expression.
 *
 * Examples:
 *
 *     factor: desired_replicas;
 *     factor: resources.replication_capacity;
 *     factor: policy.replica_count;
 *     factor: compute_factor();
 *
 * This permits the semantic/resource system to determine the actual value.
 *
 * The grammar does NOT impose:
 *
 *     factor <= N
 *
 * for any hard-coded N.
 *
 * The semantic layer is responsible for:
 *
 *     - type checking;
 *     - unit checking;
 *     - validity;
 *     - feasibility;
 *     - resource availability;
 *     - policy compliance.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Replication syntax may occur around distributed quantum computation.
 *
 * However this grammar MUST NOT define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     Gate;
 *     Circuit;
 *     QuantumOperation;
 *     topology;
 *     calibration;
 *     pulse;
 *     QEC;
 *     ZQN.
 *
 * Quantum semantics remain owned by the quantum subsystem and ultimately
 * cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * In particular, a replication declaration MUST NOT be interpreted by the
 * grammar as permission to copy an arbitrary unknown quantum state.
 *
 * Quantum no-cloning and valid logical-state handling are semantic concerns.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Replication can describe logical replication of hardware-related computation
 * or execution units, but this grammar does not define:
 *
 *     - wires;
 *     - clocks;
 *     - FPGA resources;
 *     - ASIC cells;
 *     - physical ports;
 *     - hardware addresses;
 *     - physical device identifiers.
 *
 * Hardware realization belongs to the HDL and hardware subsystems.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Replication properties may reference the universal resource system.
 *
 * The following concepts remain distinct:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *
 * This file records syntax only.
 *
 * It does not decide whether a resource condition can be satisfied.
 *
 * ============================================================================
 * NETWORK BOUNDARY
 * ============================================================================
 *
 * Replication does not select a transport.
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
 *     vendor-specific transport.
 *
 * Network realization belongs to the networking, deployment, and runtime
 * layers.
 *
 * ============================================================================
 * MEMORY BOUNDARY
 * ============================================================================
 *
 * Distributed-memory operation syntax belongs to:
 *
 *     memory/distributed-memory.g4
 *
 * This file does not redefine:
 *
 *     - allocation;
 *     - deallocation;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - memory locations.
 *
 * A replicated object may reference memory semantics through shared canonical
 * abstractions.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * Replication policies are intentionally represented primarily through
 * identifiers and expressions.
 *
 * This avoids a closed enumeration such as:
 *
 *     SYNCHRONOUS
 *     ASYNCHRONOUS
 *     ACTIVE_ACTIVE
 *     ACTIVE_PASSIVE
 *
 * becoming a permanent language limitation.
 *
 * Future policies may therefore be represented through semantic names such as:
 *
 *     distributed::replication::synchronous
 *     distributed::replication::asynchronous
 *     distributed::replication::active_active
 *     distributed::replication::erasure
 *     distributed::replication::custom
 *
 * The semantic layer determines whether a policy exists and whether it is
 * supported by a particular compilation/execution environment.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     ZamaniLexer
 *     Names
 *     Expressions
 *
 * Names owns:
 *
 *     identifier
 *     qualified names
 *     name references
 *
 * Expressions owns:
 *
 *     expression
 *     expression lists
 *     calls
 *     operators
 *     indexing
 *     member access
 *
 * This grammar MUST NOT redefine those concepts.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file introduces NO lexer rules.
 *
 * In particular it does not add global tokens for:
 *
 *     replication
 *     replica
 *     replicas
 *     factor
 *     consistency
 *     durability
 *     availability
 *     placement
 *     synchronization
 *
 * These remain contextual semantic names.
 *
 * This keeps the language extensible without continuously expanding the global
 * keyword vocabulary.
 *
 * ============================================================================
 * IMPORTANT CONTEXTUAL-NAME CONTRACT
 * ============================================================================
 *
 * Because the declaration marker is represented through the canonical
 * identifier rule, this parser component alone does not determine that an
 * arbitrary identifier is semantically the word:
 *
 *     replication
 *
 * The frontend semantic layer MUST perform contextual classification.
 *
 * This is deliberate.
 *
 * It prevents domain grammar from becoming a second lexical authority.
 *
 * ============================================================================
 * PUBLIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The stable public entry point is:
 *
 *     distributedReplicationDeclaration
 *
 * The aggregate:
 *
 *     grammar/distributed/distributed.g4
 *
 * MUST import this grammar component and route its replication declaration
 * branch to:
 *
 *     distributedReplicationDeclaration
 *
 * It MUST NOT define a second rule with the same responsibility.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST representation for this grammar must preserve:
 *
 *     - source span of declaration;
 *     - logical replication name;
 *     - body presence/absence;
 *     - member ordering;
 *     - property names;
 *     - property expressions;
 *     - replica names;
 *     - replica-body structure;
 *     - dependency expressions;
 *     - extension expressions;
 *     - nested block structure.
 *
 * The AST MUST NOT resolve:
 *
 *     - physical nodes;
 *     - devices;
 *     - addresses;
 *     - transport;
 *     - topology;
 *     - runtime placement.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the declaration marker is valid;
 *     - whether the logical target is replicable;
 *     - whether the factor expression is valid;
 *     - whether a policy exists;
 *     - whether policies conflict;
 *     - whether requested resources are satisfiable;
 *     - whether placement constraints are satisfiable;
 *     - whether consistency requirements are realizable;
 *     - whether durability requirements are realizable;
 *     - whether quantum-related replication is semantically legal.
 *
 * Syntax alone MUST NOT imply any of those properties.
 *
 * ============================================================================
 * DUPLICATE PROPERTY POLICY
 * ============================================================================
 *
 * This grammar permits repeated properties.
 *
 * Example:
 *
 *     replication workload {
 *         preference: low_latency;
 *         preference: low_energy;
 *     }
 *
 * The grammar does not silently select one.
 *
 * Semantic analysis MUST determine whether:
 *
 *     - repeated properties are cumulative;
 *     - repeated properties conflict;
 *     - later properties override earlier properties;
 *     - duplicates are forbidden.
 *
 * This preserves source information and avoids hidden grammar semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem operations;
 *     - no networking;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no hardware discovery.
 *
 * Parsing therefore depends only on the token stream and grammar.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * AST construction must preserve source ordering and spans.
 *
 * Semantic canonicalization may happen later.
 *
 * The parser MUST NOT reorder:
 *
 *     - replicas;
 *     - properties;
 *     - dependencies;
 *     - nested blocks.
 *
 * ============================================================================
 * DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser errors include:
 *
 *     - malformed declaration;
 *     - malformed body;
 *     - malformed property;
 *     - malformed replica declaration;
 *     - malformed dependency;
 *     - malformed argument list;
 *     - unbalanced delimiters.
 *
 * Semantic errors include:
 *
 *     - invalid replication target;
 *     - invalid factor;
 *     - unsupported policy;
 *     - impossible resource requirement;
 *     - contradictory constraints;
 *     - invalid quantum replication;
 *     - invalid consistency combination.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Therefore generated/parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * The implementation must use safe Rust only.
 *
 * No unsafe blocks or unsafe abstractions are required by this grammar.
 *
 * ============================================================================
 */

parser grammar Replication;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Complete replication declaration.
 *
 * Canonical source shape:
 *
 *     replication workload;
 *
 * or:
 *
 *     replication workload {
 *         target: computation;
 *         factor: desired_replicas;
 *     }
 *
 * The first identifier is contextually classified as `replication`.
 */
distributedReplicationDeclaration
    : replicationDeclarationMarker
      identifier
      distributedReplicationBody?
      SEMICOLON?
    ;


/* ============================================================================
 * DECLARATION MARKER
 * ========================================================================== */

/*
 * Contextual marker.
 *
 * No new lexer keyword is introduced.
 */
replicationDeclarationMarker
    : identifier
    ;


/* ============================================================================
 * REPLICATION BODY
 * ========================================================================== */

distributedReplicationBody
    : LBRACE
      distributedReplicationMember*
      RBRACE
    ;


/* ============================================================================
 * REPLICATION MEMBERS
 * ========================================================================== */

distributedReplicationMember
    : distributedReplicationProperty
    | distributedReplicationReplica
    | distributedReplicationDependency
    | distributedReplicationGroup
    | distributedReplicationBlock
    | distributedReplicationExpressionStatement
    ;


/* ============================================================================
 * PROPERTY
 * ========================================================================== */

/*
 * Generic property form:
 *
 *     target: workload;
 *     factor: desired_replicas;
 *     mode: distributed::replication::synchronous;
 *     consistency: consistency_policy;
 *     placement: placement_policy;
 *     availability: availability_policy;
 *     durability: durability_policy;
 *     synchronization: synchronization_policy;
 *     requirement: resource_requirement;
 *     constraint: resource_constraint;
 *     preference: resource_preference;
 *     hint: resource_hint;
 *     policy: replication_policy;
 *
 * The property name is intentionally an identifier.
 *
 * Semantic analysis owns the property vocabulary.
 */
distributedReplicationProperty
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * LOGICAL REPLICA
 * ========================================================================== */

/*
 * A replica is a logical entity.
 *
 * It does not identify:
 *
 *     - a node;
 *     - a process;
 *     - a machine;
 *     - a device;
 *     - an address.
 */
distributedReplicationReplica
    : replicaDeclarationMarker
      identifier
      distributedReplicationReplicaBody?
      SEMICOLON?
    ;


replicaDeclarationMarker
    : identifier
    ;


distributedReplicationReplicaBody
    : LBRACE
      distributedReplicationReplicaMember*
      RBRACE
    ;


distributedReplicationReplicaMember
    : distributedReplicationProperty
    | distributedReplicationBlock
    | distributedReplicationExpressionStatement
    ;


/* ============================================================================
 * REPLICA GROUP
 * ========================================================================== */

/*
 * A logical replica group permits a program to describe related replicas
 * without tying them to physical deployment.
 *
 * Example:
 *
 *     group workers {
 *         members: worker_set;
 *         policy: replication_policy;
 *     }
 */
distributedReplicationGroup
    : groupDeclarationMarker
      identifier
      distributedReplicationGroupBody?
      SEMICOLON?
    ;


groupDeclarationMarker
    : identifier
    ;


distributedReplicationGroupBody
    : LBRACE
      distributedReplicationGroupMember*
      RBRACE
    ;


distributedReplicationGroupMember
    : distributedReplicationProperty
    | distributedReplicationReplica
    | distributedReplicationDependency
    | distributedReplicationBlock
    | distributedReplicationExpressionStatement
    ;


/* ============================================================================
 * DEPENDENCY
 * ========================================================================== */

/*
 * Example:
 *
 *     depends_on: base_replication;
 *
 * This describes a logical semantic dependency.
 *
 * It does not describe:
 *
 *     - network routing;
 *     - machine dependency;
 *     - hardware dependency.
 */
distributedReplicationDependency
    : dependencyDeclarationMarker
      COLON
      expression
      SEMICOLON
    ;


dependencyDeclarationMarker
    : identifier
    ;


/* ============================================================================
 * NESTED POLICY / CONSTRAINT BLOCK
 * ========================================================================== */

/*
 * Generic nested block.
 *
 * Examples:
 *
 *     policy {
 *         mode: synchronous;
 *     }
 *
 *     consistency {
 *         requirement: causal;
 *     }
 *
 *     placement {
 *         preference: locality;
 *     }
 *
 *     resources {
 *         requirement: resources.replication_capacity;
 *     }
 *
 * The block name remains contextual.
 */
distributedReplicationBlock
    : identifier
      LBRACE
      distributedReplicationMember*
      RBRACE
    ;


/* ============================================================================
 * EXPRESSION STATEMENT
 * ========================================================================== */

/*
 * Allows a semantic expression to appear in a replication body.
 *
 * Example:
 *
 *     validate_replication_policy();
 *
 * The expression itself belongs to the canonical expression grammar.
 *
 * This rule does not define what such an expression means.
 */
distributedReplicationExpressionStatement
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * TARGET
 * ========================================================================== */

/*
 * Semantic target expression.
 *
 * Examples:
 *
 *     computation
 *     service
 *     state
 *     data
 *     workload
 *     module::operation()
 */
distributedReplicationTarget
    : expression
    ;


/* ============================================================================
 * FACTOR
 * ========================================================================== */

/*
 * Replication factor is an expression rather than a grammar-level integer.
 *
 * Valid examples include:
 *
 *     3
 *     desired_replicas
 *     policy.replica_count
 *     resources.replication_capacity
 *     compute_factor()
 *
 * The grammar does not impose a maximum.
 */
distributedReplicationFactor
    : expression
    ;


/* ============================================================================
 * MODE
 * ========================================================================== */

distributedReplicationMode
    : expression
    ;


/* ============================================================================
 * POLICY
 * ========================================================================== */

distributedReplicationPolicy
    : expression
    ;


/* ============================================================================
 * CONSISTENCY
 * ========================================================================== */

distributedReplicationConsistency
    : expression
    ;


/* ============================================================================
 * PLACEMENT
 * ========================================================================== */

distributedReplicationPlacement
    : expression
    ;


/* ============================================================================
 * AVAILABILITY
 * ========================================================================== */

distributedReplicationAvailability
    : expression
    ;


/* ============================================================================
 * DURABILITY
 * ========================================================================== */

distributedReplicationDurability
    : expression
    ;


/* ============================================================================
 * SYNCHRONIZATION
 * ========================================================================== */

distributedReplicationSynchronization
    : expression
    ;


/* ============================================================================
 * RESOURCE REQUIREMENT
 * ========================================================================== */

distributedReplicationRequirement
    : expression
    ;


/* ============================================================================
 * RESOURCE CONSTRAINT
 * ========================================================================== */

distributedReplicationConstraint
    : expression
    ;


/* ============================================================================
 * RESOURCE PREFERENCE
 * ========================================================================== */

distributedReplicationPreference
    : expression
    ;


/* ============================================================================
 * RESOURCE HINT
 * ========================================================================== */

distributedReplicationHint
    : expression
    ;


/* ============================================================================
 * EXPRESSION ARGUMENTS
 * ========================================================================== */

/*
 * Reusable comma-separated expression list.
 *
 * There is no fixed argument count.
 */
distributedReplicationArguments
    : expression
      (COMMA expression)*
    ;


/* ============================================================================
 * QUALIFIED EXTENSION
 * ========================================================================== */

/*
 * Open-world extension syntax.
 *
 * Example:
 *
 *     distributed::replication::custom_policy(arg1, arg2)
 *
 * This does not make the policy valid.
 *
 * Semantic registration/validation remains downstream.
 */
distributedReplicationExtension
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
      (
          LPAREN
          distributedReplicationArguments?
          RPAREN
      )?
    ;


/* ============================================================================
 * SEMANTIC PROPERTY SHAPES
 * ========================================================================== */

/*
 * The following rules provide stable parser-level contracts for downstream
 * grammar composition without duplicating expression syntax.
 *
 * They are intentionally aliases over `expression`.
 *
 * No physical resource is selected by these rules.
 */


/*
 * Logical target.
 */
distributedReplicationTargetProperty
    : distributedReplicationTarget
    ;


/*
 * Desired replication quantity.
 */
distributedReplicationFactorProperty
    : distributedReplicationFactor
    ;


/*
 * Replication policy.
 */
distributedReplicationPolicyProperty
    : distributedReplicationPolicy
    ;


/*
 * Consistency requirement.
 */
distributedReplicationConsistencyProperty
    : distributedReplicationConsistency
    ;


/*
 * Placement requirement/preference.
 */
distributedReplicationPlacementProperty
    : distributedReplicationPlacement
    ;


/*
 * Availability requirement.
 */
distributedReplicationAvailabilityProperty
    : distributedReplicationAvailability
    ;


/*
 * Durability requirement.
 */
distributedReplicationDurabilityProperty
    : distributedReplicationDurability
    ;


/*
 * Synchronization requirement.
 */
distributedReplicationSynchronizationProperty
    : distributedReplicationSynchronization
    ;


/* ============================================================================
 * SEMANTIC INTEGRATION GUARANTEES
 * ========================================================================== */

/*
 * GUARANTEE 1: NO PHYSICAL IDENTITY
 *
 * A logical replica identifier MUST NOT be interpreted as a physical identity.
 *
 * The following mappings are forbidden unless explicitly supplied by a
 * separate deployment/placement system:
 *
 *     primary   -> machine 0
 *     secondary -> machine 1
 *     replica_a -> device 0
 *
 *
 * GUARANTEE 2: NO RESOURCE INVENTORY
 *
 * The grammar does not inspect or encode the available number of:
 *
 *     nodes;
 *     processes;
 *     CPUs;
 *     GPUs;
 *     QPUs;
 *     FPGAs;
 *     devices;
 *     memory units;
 *     network links.
 *
 *
 * GUARANTEE 3: NO TRANSPORT
 *
 * Replication does not select a networking protocol.
 *
 *
 * GUARANTEE 4: NO PLACEMENT
 *
 * Replication describes logical multiplicity.
 *
 * Physical realization belongs to placement/resource/deployment systems.
 *
 *
 * GUARANTEE 5: NO SCHEDULING
 *
 * Replication does not determine execution order or timing.
 *
 * Scheduling owns those concerns.
 *
 *
 * GUARANTEE 6: NO ROUTING
 *
 * Replication does not determine physical communication paths.
 *
 * Routing/networking own those concerns.
 *
 *
 * GUARANTEE 7: NO CONSENSUS IMPLEMENTATION
 *
 * A consistency property is data consumed by semantic/runtime systems.
 *
 *
 * GUARANTEE 8: NO RESILIENCE IMPLEMENTATION
 *
 * Replication syntax does not perform retry, restart, recovery, or failover.
 *
 *
 * GUARANTEE 9: NO QUANTUM STATE COPYING ASSUMPTION
 *
 * Replication of a quantum-related computation does not imply copying an
 * arbitrary unknown quantum state.
 *
 *
 * GUARANTEE 10: NO IR DUPLICATION
 *
 * The grammar never creates a second:
 *
 *     quantum::ir
 *
 * representation.
 *
 * Quantum semantics lower through the canonical quantum IR boundary.
 */


/* ============================================================================
 * POCO-REAF RESOURCE RESOLUTION
 * ========================================================================== */

/*
 * The following conceptual source:
 *
 *     replication workload {
 *         factor: resources.replication_capacity;
 *     }
 *
 * means:
 *
 *     "Use the resource system's replication capacity when determining the
 *      desired replication factor."
 *
 * It does NOT mean:
 *
 *     "The machine has a fixed number of replicas."
 *
 * Similarly:
 *
 *     replication workload {
 *         requirement: resources.distributed_replication;
 *     }
 *
 * describes a requirement.
 *
 * It does not select a particular provider, node, machine, or device.
 */


/* ============================================================================
 * QUANTUM SAFETY
 * ========================================================================== */

/*
 * A construct such as:
 *
 *     replication quantum_workload {
 *         target: quantum_program;
 *         factor: desired_parallel_realizations;
 *     }
 *
 * remains syntactically valid.
 *
 * Whether the requested semantic replication is physically or mathematically
 * valid is determined by the quantum semantic layer.
 *
 * In particular, the compiler MUST distinguish:
 *
 *     - replicated program descriptions;
 *     - replicated classical control;
 *     - independent executions;
 *     - replicated logical information;
 *     - QEC-supported logical-state handling;
 *     - measurement-derived information;
 *     - provider-supported state mechanisms;
 *     - invalid attempts to copy arbitrary unknown quantum state.
 *
 * This grammar does not implement those distinctions.
 */


/* ============================================================================
 * COMPILER INTEGRATION
 * ========================================================================== */

/*
 * Frontend:
 *
 *     ReplicationContext
 *          |
 *          v
 *     distributed replication AST node
 *
 * Semantic analysis:
 *
 *     AST
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> consistency validation
 *       +--> placement validation
 *       +--> quantum semantic validation
 *       |
 *       v
 *     canonical semantic representation
 *
 * Backend:
 *
 *     canonical semantics
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> hardware/resource realization
 *          +--> runtime dispatch
 */


/* ============================================================================
 * RUNTIME INTEGRATION
 * ========================================================================== */

/*
 * Runtime components may consume semantic replication metadata such as:
 *
 *     target
 *     desired factor
 *     policy
 *     consistency
 *     availability
 *     durability
 *     placement constraints
 *
 * Runtime MUST NOT interpret this grammar directly as a physical deployment
 * specification.
 *
 * Runtime resolution occurs through:
 *
 *     resource discovery
 *     capability negotiation
 *     placement
 *     scheduling
 *     deployment
 *     networking
 *     resilience
 */


/* ============================================================================
 * RESOURCE INTEGRATION
 * ========================================================================== */

/*
 * Resource analysis owns feasibility.
 *
 * For example:
 *
 *     factor: desired_replicas;
 *
 * may become:
 *
 *     requested replication = evaluate(desired_replicas)
 *
 * followed by:
 *
 *     available capacity = resource system
 *
 * followed by:
 *
 *     placement = placement system
 *
 * followed by:
 *
 *     execution = runtime
 *
 * No grammar rule is modified based on the discovered resource count.
 */


/* ============================================================================
 * CONSISTENCY INTEGRATION
 * ========================================================================== */

/*
 * The consistency subsystem consumes semantic expressions such as:
 *
 *     consistency: policy;
 *
 * It determines whether the requested consistency model is:
 *
 *     supported;
 *     unsupported;
 *     conditionally supported;
 *     unknown;
 *     conflicting.
 *
 * This grammar remains independent of the implementation algorithm.
 */


/* ============================================================================
 * RESILIENCE INTEGRATION
 * ========================================================================== */

/*
 * Resilience may consume replication metadata when deciding how to recover
 * from distributed failures.
 *
 * It may determine:
 *
 *     retry;
 *     restart;
 *     reroute;
 *     reschedule;
 *     remap;
 *     switch backend;
 *     quarantine;
 *     recover;
 *     abort.
 *
 * None of those actions are implemented here.
 */


/* ============================================================================
 * SOURCE COMPATIBILITY
 * ========================================================================== */

/*
 * The canonical public rule:
 *
 *     distributedReplicationDeclaration
 *
 * MUST remain stable.
 *
 * If older frontend code already references this rule, the aggregate grammar
 * should continue exposing it.
 *
 * Internal helper-rule names may evolve without changing the public entry
 * point, provided AST semantics remain compatible.
 */


/* ============================================================================
 * HARD-CODING AUDIT
 * ========================================================================== */

/*
 * This file contains no:
 *
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_SHARDS
 *     MAX_PARTITIONS
 *     MAX_WORKERS
 *     MAX_PROCESSES
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_CPUS
 *
 * It contains no:
 *
 *     fixed node IDs;
 *     fixed device IDs;
 *     fixed network addresses;
 *     fixed topology;
 *     fixed deployment;
 *     fixed transport;
 *     fixed provider.
 *
 * Numeric literals, when supplied through `expression`, are PROGRAM VALUES,
 * not machine-capacity declarations.
 *
 * This distinction must be preserved by semantic analysis.
 */


/* ============================================================================
 * TEST CONTRACT
 * ========================================================================== */

/*
 * Positive tests MUST include:
 *
 *     replication workload;
 *
 *     replication workload {
 *         target: computation;
 *     }
 *
 *     replication workload {
 *         factor: desired_replicas;
 *     }
 *
 *     replication workload {
 *         factor: resources.replication_capacity;
 *         consistency: policy;
 *         placement: locality;
 *     }
 *
 *     replication workload {
 *         replica primary;
 *         replica secondary;
 *     }
 *
 *     replication workload {
 *         group workers {
 *             members: worker_set;
 *         }
 *     }
 *
 *     replication workload {
 *         policy {
 *             mode: distributed::replication::synchronous;
 *         }
 *     }
 *
 * Negative tests MUST include:
 *
 *     replication;
 *
 *     replication workload {
 *         factor:
 *     }
 *
 *     replication workload {
 *         target:
 *     }
 *
 *     replication workload {
 *         replica;
 *     }
 *
 *     replication workload {
 *         {
 *     }
 *
 * Boundary tests MUST include:
 *
 *     - one replication;
 *     - many replication properties;
 *     - many replicas;
 *     - deeply nested policy blocks;
 *     - deeply qualified policy names;
 *     - large expression trees;
 *     - large argument lists.
 *
 * Scalability tests MUST verify that the grammar contains no source-level
 * machine-size ceiling.
 *
 * Cross-domain tests MUST include combinations with:
 *
 *     classical;
 *     quantum;
 *     HDL;
 *     hardware;
 *     AI;
 *     accelerator;
 *     networking;
 *     memory;
 *     distributed execution.
 *
 * Quantum tests MUST verify that parsing does not imply physical quantum-state
 * copying semantics.
 *
 * Determinism tests MUST verify that identical token streams produce identical
 * parse structures.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is COMPLETE only when all of the following are true:
 *
 * [ ] `Replication` is the parser grammar name.
 *
 * [ ] `ZamaniLexer` is the canonical token vocabulary.
 *
 * [ ] `Names` supplies identifier/name infrastructure.
 *
 * [ ] `Expressions` supplies expression infrastructure.
 *
 * [ ] No duplicate identifier grammar exists here.
 *
 * [ ] No duplicate expression grammar exists here.
 *
 * [ ] `distributedReplicationDeclaration` is the stable public entry point.
 *
 * [ ] `distributed.g4` imports/consumes this rule exactly once.
 *
 * [ ] No competing replication declaration rule exists elsewhere in the
 *     distributed aggregate.
 *
 * [ ] No lexer keyword is introduced here.
 *
 * [ ] No physical node/device identity is encoded.
 *
 * [ ] No fixed replication limit exists.
 *
 * [ ] No fixed machine capacity exists.
 *
 * [ ] No transport protocol is selected.
 *
 * [ ] No placement algorithm is implemented.
 *
 * [ ] No scheduling is implemented.
 *
 * [ ] No routing is implemented.
 *
 * [ ] No consistency algorithm is implemented.
 *
 * [ ] No resilience algorithm is implemented.
 *
 * [ ] No QEC semantics are duplicated.
 *
 * [ ] No ZQN semantics are duplicated.
 *
 * [ ] No quantum::ir representation is duplicated.
 *
 * [ ] AST source ordering and source spans can be preserved.
 *
 * [ ] Semantic validation owns replication feasibility.
 *
 * [ ] Resource analysis owns physical capacity.
 *
 * [ ] Placement owns physical realization.
 *
 * [ ] Runtime owns execution.
 *
 * [ ] Rust integration remains Rust 1.97/1.97.1 compatible.
 *
 * [ ] No unsafe implementation is required.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] Documentation identifies this file as the authoritative owner of
 *     distributed replication syntax.
 */


/* ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ========================================================================== */

/*
 * This grammar expresses:
 *
 *     WHAT should be replicated
 *     HOW replication is semantically constrained
 *     WHICH logical relationships exist
 *
 * It does NOT permanently encode:
 *
 *     WHERE replication occurs
 *     WHICH machine performs it
 *     WHICH device performs it
 *     HOW many machines exist
 *     WHICH topology exists
 *     WHICH transport is used
 *     WHICH runtime performs it
 *
 * Therefore the same Zamani source semantics can remain valid while the
 * available execution environment changes from:
 *
 *     tiny embedded system
 *          ->
 *     single machine
 *          ->
 *     multicore system
 *          ->
 *     accelerator
 *          ->
 *     quantum system
 *          ->
 *     heterogeneous machine
 *          ->
 *     cluster
 *          ->
 *     supercomputer
 *          ->
 *     cloud
 *          ->
 *     future computing architecture.
 *
 * This is the required grammar-level foundation for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and:
 *
 *     Zamani — From Atom to Everywhere.
 */