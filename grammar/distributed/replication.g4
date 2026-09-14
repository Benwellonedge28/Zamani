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
 * Purpose:
 *     Production source grammar for distributed replication intent.
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *     No unsafe code
 *
 * ============================================================================
 *
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL SYNTAX for distributed replication.
 *
 * Replication describes the semantic intent that one logical computation,
 * value, service, state object, data object, execution unit, or other
 * replication-capable entity may have multiple logical realizations.
 *
 * This file does NOT implement replication.
 *
 * The grammar is upstream of:
 *
 *     - semantic analysis;
 *     - type checking;
 *     - capability checking;
 *     - resource analysis;
 *     - consistency analysis;
 *     - placement;
 *     - scheduling;
 *     - routing;
 *     - networking;
 *     - storage;
 *     - hardware selection;
 *     - deployment;
 *     - runtime execution;
 *     - resilience/recovery.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed replication declaration syntax;
 *     - replication intent syntax;
 *     - logical replica identity syntax;
 *     - replica-set/group syntax;
 *     - replication policy references;
 *     - replication factor expressions;
 *     - replication mode expressions;
 *     - replication consistency intent;
 *     - replication placement intent;
 *     - replication durability intent;
 *     - replication availability intent;
 *     - replication ordering intent;
 *     - replication synchronization intent;
 *     - replication dependency syntax;
 *     - replication options;
 *     - replication extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - lexical tokens;
 *     - resources;
 *     - capabilities;
 *     - physical devices;
 *     - nodes;
 *     - network endpoints;
 *     - network transport;
 *     - placement algorithms;
 *     - scheduling algorithms;
 *     - routing algorithms;
 *     - storage implementation;
 *     - consensus algorithms;
 *     - consistency implementation;
 *     - replication protocols;
 *     - serialization implementation;
 *     - fault detection;
 *     - fault recovery;
 *     - resilience;
 *     - hardware discovery;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Replication syntax describes WHAT replication semantics are desired.
 *
 * It must not encode temporary physical deployment facts.
 *
 * Therefore this grammar does NOT impose or encode:
 *
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_SHARDS
 *     MAX_PARTITIONS
 *     MAX_DEVICES
 *     MAX_WORKERS
 *     MAX_PROCESSES
 *
 * and does not require:
 *
 *     node 0;
 *     node 1;
 *     device 0;
 *     replica 0;
 *     replica 1;
 *
 * A replication factor, when explicitly supplied, is an expression:
 *
 *     factor: replicas;
 *
 *     factor: desired_replicas;
 *
 *     factor: resource.replica_capacity;
 *
 * The semantic/resource layers determine whether that expression can be
 * satisfied.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level limit is imposed on:
 *
 *     - replication declarations;
 *     - replica sets;
 *     - replica members;
 *     - replication options;
 *     - dependencies;
 *     - policy nesting;
 *     - expression complexity;
 *     - qualified-name depth;
 *     - replication groups;
 *     - logical replicas.
 *
 * Repetition uses ANTLR repetition operators.
 *
 * Practical limits such as:
 *
 *     - parser memory;
 *     - compiler memory;
 *     - available execution resources;
 *     - deployment limits;
 *     - resource availability;
 *     - provider constraints;
 *
 * are NOT language-level grammar limits.
 *
 * ============================================================================
 *
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * Replication is not synonymous with:
 *
 *     placement;
 *     redundancy;
 *     backup;
 *     caching;
 *     sharding;
 *     partitioning;
 *     consensus;
 *     failover;
 *     retry.
 *
 * Those concepts may interact with replication but remain independently owned.
 *
 * The semantic layer must preserve these distinctions.
 *
 * ============================================================================
 *
 * REPLICATION FACTOR
 * ============================================================================
 *
 * A replication factor is a semantic quantity.
 *
 * The grammar therefore accepts an expression rather than an integer literal
 * restricted by a grammar-defined maximum.
 *
 * Examples:
 *
 *     factor: desired_replicas;
 *
 *     factor: available_replicas;
 *
 *     factor: policy.replication_factor;
 *
 *     factor: min(required, capacity);
 *
 * The semantic layer determines:
 *
 *     - type;
 *     - unit;
 *     - validity;
 *     - availability;
 *     - feasibility;
 *     - policy compliance.
 *
 * The grammar does not determine any of these.
 *
 * ============================================================================
 *
 * REPLICA IDENTITY
 * ============================================================================
 *
 * A logical replica name is a source-level identifier.
 *
 * It is NOT:
 *
 *     - a machine identifier;
 *     - a process identifier;
 *     - a hardware identifier;
 *     - a network address;
 *     - a device address;
 *     - a physical replica location.
 *
 * Physical identity is resolved downstream.
 *
 * ============================================================================
 *
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Replication may express requirements, constraints, preferences, and hints.
 *
 * These are deliberately distinct.
 *
 * REQUIREMENT:
 *
 *     mandatory semantic condition.
 *
 * CONSTRAINT:
 *
 *     restriction on legal realizations.
 *
 * PREFERENCE:
 *
 *     desirable realization property.
 *
 * HINT:
 *
 *     non-binding implementation guidance.
 *
 * This grammar records their syntax only.
 *
 * It does not decide whether a requested property can be satisfied.
 *
 * ============================================================================
 *
 * CONSISTENCY BOUNDARY
 * ============================================================================
 *
 * Replication and consistency are related but separate semantic concepts.
 *
 * This grammar can record a consistency policy reference.
 *
 * It does not implement:
 *
 *     - linearizability;
 *     - sequential consistency;
 *     - causal consistency;
 *     - eventual consistency;
 *     - transactional consistency;
 *     - consensus;
 *     - quorum algorithms.
 *
 * Those semantics belong to the distributed semantic/runtime layers.
 *
 * ============================================================================
 *
 * PLACEMENT BOUNDARY
 * ============================================================================
 *
 * Replication may express placement intent such as:
 *
 *     locality;
 *     affinity;
 *     anti_affinity;
 *     diversity;
 *     topology_aware;
 *     region_aware;
 *     energy_aware.
 *
 * Such names are semantic values.
 *
 * They do NOT identify physical nodes.
 *
 * The hardware/resource/deployment subsystem resolves physical realization.
 *
 * ============================================================================
 *
 * FAILURE / RESILIENCE BOUNDARY
 * ============================================================================
 *
 * Replication may improve availability or fault tolerance, but replication
 * syntax does not implement recovery.
 *
 * The grammar does not:
 *
 *     retry;
 *     restart;
 *     fail over;
 *     elect leaders;
 *     restore checkpoints;
 *     repair replicas;
 *     detect faults.
 *
 * Resilience decides how to respond to execution failures.
 *
 * ============================================================================
 *
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Distributed replication syntax may appear around quantum-related
 * computation, state, services, or execution metadata where the semantics
 * permit it.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     Gate;
 *     Circuit;
 *     quantum topology;
 *     calibration;
 *     pulse semantics;
 *     QEC;
 *     ZQN.
 *
 * Quantum semantic lowering remains through:
 *
 *     quantum::ir
 *
 * Replication of arbitrary unknown quantum state must never be assumed to
 * mean that the state can simply be copied or serialized.
 *
 * Semantic analysis must reject or constrain replication requests that violate
 * the applicable quantum semantics.
 *
 * ============================================================================
 *
 * MEMORY BOUNDARY
 * ============================================================================
 *
 * `memory/distributed-memory.g4` owns distributed-memory operation syntax.
 *
 * This file owns general distributed replication intent.
 *
 * A memory replication operation may reference this semantic domain, but this
 * file must not redefine:
 *
 *     memoryPlace;
 *     memoryQualifiedName;
 *     memory ownership;
 *     borrowing;
 *     lifetimes;
 *     allocation;
 *     deallocation.
 *
 * ============================================================================
 *
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * Replication does not select a transport protocol.
 *
 * It does not imply:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     HTTP;
 *     RPC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     vendor-specific transport.
 *
 * Network realization is owned by networking/runtime/deployment layers.
 *
 * ============================================================================
 *
 * STORAGE BOUNDARY
 * ============================================================================
 *
 * Replication does not imply:
 *
 *     persistent storage;
 *     database replication;
 *     filesystem replication;
 *     object storage;
 *     cache replication.
 *
 * If persistence is semantically required, that requirement must be represented
 * by the appropriate storage/data/resource abstractions.
 *
 * ============================================================================
 *
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Replication modes and policies are represented primarily through identifiers
 * and qualified names rather than a closed grammar enumeration.
 *
 * This allows future technologies to be represented without repeatedly
 * changing the global lexer.
 *
 * Examples:
 *
 *     distributed::replication
 *     distributed::replication::active_active
 *     distributed::replication::active_passive
 *     distributed::replication::synchronous
 *     distributed::replication::asynchronous
 *     distributed::replication::quorum
 *     distributed::replication::erasure
 *     distributed::replication::custom_policy
 *
 * Whether any such policy is supported is a semantic/backend question.
 *
 * ============================================================================
 *
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          +--> Names
 *          |
 *          +--> Expressions
 *          |
 *          +--> Replication
 *          |
 *          v
 *     Distributed aggregate grammar
 *
 * This file consumes:
 *
 *     identifier
 *     qualifiedName / name-reference infrastructure
 *     expression
 *
 * from the canonical shared grammar.
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 *
 * BUILD CONTRACT
 * ============================================================================
 *
 * This is a parser grammar component.
 *
 * It intentionally contains:
 *
 *     - no lexer grammar;
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem operations;
 *     - no networking;
 *     - no runtime calls;
 *     - no hardware discovery;
 *     - no mutable global state;
 *     - no randomness.
 *
 * Rust 1.97 / 1.97.1 compatibility is therefore enforced at the frontend and
 * generated-parser integration layer rather than through Rust-specific grammar
 * actions.
 *
 * ============================================================================
 *
 * PUBLIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The distributed aggregate grammar already exposes:
 *
 *     distributedReplicationDeclaration
 *
 * as a distributed declaration branch.
 *
 * This file is the authoritative owner of that rule.
 *
 * The aggregate grammar MUST import this grammar component and MUST NOT define
 * another competing `distributedReplicationDeclaration`.
 *
 * ============================================================================
 *
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * AST construction must preserve:
 *
 *     - declaration order;
 *     - target expression;
 *     - replication factor expression;
 *     - replica names;
 *     - policy expressions;
 *     - option order;
 *     - dependency order;
 *     - nested blocks;
 *     - source spans;
 *     - syntactic distinctions that affect diagnostics.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 *
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser is responsible for structural syntax errors.
 *
 * Semantic analysis is responsible for errors such as:
 *
 *     - non-replicable target;
 *     - invalid replication factor type;
 *     - unsupported policy;
 *     - insufficient resources;
 *     - impossible consistency requirement;
 *     - impossible placement requirement;
 *     - illegal quantum-state replication;
 *     - conflicting replication policies.
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
 * Complete distributed replication declaration.
 *
 * Canonical contextual form:
 *
 *     replication name;
 *
 * or:
 *
 *     replication name {
 *         target: value;
 *         factor: replicas;
 *     }
 *
 * `replication` remains an identifier at lexical level.
 *
 * Semantic analysis is responsible for contextual classification.
 */
distributedReplicationDeclaration
    : replicationMarker
      identifier
      distributedReplicationBody?
      SEMICOLON?
    ;


/* ============================================================================
 * CONTEXTUAL MARKER
 * ========================================================================== */

/*
 * The declaration marker is intentionally contextual.
 *
 * This avoids forcing a new globally reserved lexer keyword solely for
 * distributed replication.
 */
replicationMarker
    : identifier
    ;


/* ============================================================================
 * BODY
 * ========================================================================== */

/*
 * Zero or more replication members.
 *
 * There is no fixed number of options or declarations.
 */
distributedReplicationBody
    : LBRACE
      distributedReplicationMember*
      RBRACE
    ;


/* ============================================================================
 * MEMBERS
 * ========================================================================== */

distributedReplicationMember
    : distributedReplicationAssignment
    | distributedReplicationExpression
    | distributedReplicationBlock
    | distributedReplicationReplica
    | distributedReplicationDependency
    ;


/* ============================================================================
 * GENERIC ASSIGNMENT
 * ========================================================================== */

/*
 * Generic replication property:
 *
 *     target: workload;
 *     factor: desired_replicas;
 *     mode: distributed::replication::synchronous;
 *     policy: policy_name;
 *     consistency: consistency_policy;
 *
 * Semantic analysis determines the meaning of the key.
 */
distributedReplicationAssignment
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * DIRECT EXPRESSION
 * ========================================================================== */

/*
 * Allows an expression to occur as a replication member when the surrounding
 * semantic context permits it.
 */
distributedReplicationExpression
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * NESTED BLOCK
 * ========================================================================== */

/*
 * Generic policy/constraint/metadata block.
 *
 * Examples:
 *
 *     policy {
 *         mode: synchronous;
 *     }
 *
 *     placement {
 *         preference: locality;
 *     }
 *
 *     consistency {
 *         policy: causal;
 *     }
 *
 * The block name remains an identifier so future extensions do not require
 * global lexer changes.
 */
distributedReplicationBlock
    : identifier
      LBRACE
      distributedReplicationMember*
      RBRACE
    ;


/* ============================================================================
 * EXPLICIT REPLICA
 * ========================================================================== */

/*
 * A logical replica declaration.
 *
 * Example:
 *
 *     replica primary;
 *
 *     replica backup {
 *         role: secondary;
 *     }
 *
 * The name is a LOGICAL replica name.
 *
 * It is never implicitly a machine, node, device, process, or network address.
 */
distributedReplicationReplica
    : replicaMarker
      identifier
      distributedReplicationReplicaBody?
      SEMICOLON?
    ;


replicaMarker
    : identifier
    ;


distributedReplicationReplicaBody
    : LBRACE
      distributedReplicationReplicaMember*
      RBRACE
    ;


distributedReplicationReplicaMember
    : distributedReplicationAssignment
    | distributedReplicationExpression
    | distributedReplicationBlock
    ;


/* ============================================================================
 * DEPENDENCY
 * ========================================================================== */

/*
 * Replication dependencies describe semantic ordering/relationship between
 * logical replication declarations.
 *
 * Example:
 *
 *     depends_on: base_state;
 *
 * The dependency is semantic.
 *
 * It does not specify a network path or machine dependency.
 */
distributedReplicationDependency
    : dependencyMarker
      COLON
      expression
      SEMICOLON
    ;


dependencyMarker
    : identifier
    ;


/* ============================================================================
 * SEMANTIC SPECIALIZATIONS
 * ========================================================================== */

/*
 * The following rules provide stable semantic entry points for downstream
 * parser composition while still delegating values to the common expression
 * grammar.
 *
 * They intentionally do not enumerate a finite set of policies.
 */


/*
 * Logical replication target.
 *
 * The target is represented by an expression because the target may be:
 *
 *     a value;
 *     a service;
 *     a computation;
 *     a state object;
 *     a data object;
 *     a resource-backed abstraction;
 *     a symbolic reference.
 */
distributedReplicationTarget
    : expression
    ;


/*
 * Replication factor.
 *
 * IMPORTANT:
 *
 * This is an expression, not a grammar-level integer.
 *
 * Therefore the grammar does not impose:
 *
 *     minimum replica count;
 *     maximum replica count;
 *     machine count;
 *     cluster count.
 */
distributedReplicationFactor
    : expression
    ;


/*
 * Replication mode/policy.
 */
distributedReplicationMode
    : expression
    ;


/*
 * Consistency policy reference.
 */
distributedReplicationConsistency
    : expression
    ;


/*
 * Placement policy reference.
 */
distributedReplicationPlacement
    : expression
    ;


/*
 * Availability policy reference.
 */
distributedReplicationAvailability
    : expression
    ;


/*
 * Durability policy reference.
 */
distributedReplicationDurability
    : expression
    ;


/*
 * Synchronization policy reference.
 */
distributedReplicationSynchronization
    : expression
    ;


/*
 * Resource requirement reference.
 *
 * The expression is interpreted by the universal resource subsystem.
 */
distributedReplicationRequirement
    : expression
    ;


/*
 * Resource constraint reference.
 */
distributedReplicationConstraint
    : expression
    ;


/*
 * Resource preference reference.
 */
distributedReplicationPreference
    : expression
    ;


/*
 * Resource hint reference.
 */
distributedReplicationHint
    : expression
    ;


/* ============================================================================
 * STRUCTURED OPTION LIST
 * ========================================================================== */

/*
 * A reusable option list for semantic consumers.
 *
 * Example conceptual form:
 *
 *     replication_options(
 *         factor,
 *         consistency,
 *         placement
 *     )
 *
 * This grammar does not attach semantics to positional arguments.
 */
distributedReplicationArguments
    : distributedReplicationArgument
      (COMMA distributedReplicationArgument)*
    ;


distributedReplicationArgument
    : expression
    ;


/* ============================================================================
 * QUALIFIED POLICY REFERENCE
 * ========================================================================== */

/*
 * A policy/reference is represented through the shared name/expression system.
 *
 * This intentionally avoids a second replication-policy identifier grammar.
 */
distributedReplicationPolicyReference
    : expression
    ;


/* ============================================================================
 * EXTENSION POINT
 * ========================================================================== */

/*
 * Open-world extension point.
 *
 * Future replication technologies can use a qualified semantic name and
 * expression arguments without requiring this grammar to enumerate every
 * future replication algorithm.
 */
distributedReplicationExtension
    : identifier
      (DCOLON identifier)*
      (
          LPAREN
          distributedReplicationArguments?
          RPAREN
      )?
    ;


/* ============================================================================
 * SEMANTIC INTEGRATION NOTES
 * ========================================================================== */

/*
 * The following conceptual source forms are intentionally represented by the
 * generic assignment rule rather than by independent hard-coded grammar
 * keywords:
 *
 *     target: workload;
 *     factor: replicas;
 *     mode: synchronous;
 *     consistency: causal;
 *     placement: locality;
 *     availability: high;
 *     durability: durable;
 *     synchronization: automatic;
 *     requirement: resource.requirement;
 *     constraint: resource.constraint;
 *     preference: resource.preference;
 *     hint: resource.hint;
 *     policy: replication_policy;
 *
 * This preserves the distinction between syntax and semantic vocabulary.
 *
 * Semantic analysis SHOULD normalize these keys into the canonical replication
 * model rather than requiring every future key to become a grammar rule.
 */


/* ============================================================================
 * NON-PHYSICAL IDENTITY GUARANTEE
 * ========================================================================== */

/*
 * The grammar deliberately permits logical names such as:
 *
 *     primary
 *     secondary
 *     replica_a
 *     replica_b
 *
 * but these names have NO physical meaning.
 *
 * The compiler/runtime must never infer:
 *
 *     primary -> machine 0
 *     secondary -> machine 1
 *
 * or any equivalent fixed mapping.
 */


/* ============================================================================
 * QUANTITY / RESOURCE GUARANTEE
 * ========================================================================== */

/*
 * A source expression such as:
 *
 *     factor: 3;
 *
 * is a program-level semantic request for a replication factor of three.
 *
 * It is NOT a declaration that the machine has three nodes.
 *
 * Conversely:
 *
 *     factor: available_replicas;
 *
 * permits the semantic/resource layers to derive a suitable value from the
 * execution environment.
 *
 * The grammar does not evaluate either expression.
 */


/* ============================================================================
 * DYNAMIC RESOURCE GUARANTEE
 * ========================================================================== */

/*
 * Replication may depend on runtime/resource information without embedding
 * that information in grammar structure.
 *
 * For example, semantic systems may interpret:
 *
 *     factor: resources.replication_capacity;
 *
 * or:
 *
 *     factor: policy.desired_factor;
 *
 * according to the canonical resource model.
 *
 * This is essential for POCO-REAF.
 */


/* ============================================================================
 * QUANTUM SAFETY GUARANTEE
 * ========================================================================== */

/*
 * The existence of a replication declaration does NOT authorize copying an
 * arbitrary quantum state.
 *
 * For quantum-related targets, semantic validation must distinguish between:
 *
 *     - replicated classical control/data;
 *     - replicated circuit/program descriptions;
 *     - replicated logical information under an appropriate protocol;
 *     - QEC-supported logical state handling;
 *     - measurement-derived state;
 *     - provider-supported state;
 *     - physically impossible or semantically invalid copying.
 *
 * No quantum no-cloning semantics are implemented here; they belong to the
 * quantum semantic layer.
 */


/* ============================================================================
 * CONSISTENCY / QUORUM SAFETY GUARANTEE
 * ========================================================================== */

/*
 * A replication factor does not automatically imply a quorum.
 *
 * The grammar therefore does not calculate:
 *
 *     quorum = factor / 2 + 1
 *
 * or any equivalent formula.
 *
 * Such policy belongs to the semantic/runtime implementation of the selected
 * consistency model.
 */


/* ============================================================================
 * NO AUTOMATIC FAILOVER GUARANTEE
 * ========================================================================== */

/*
 * Replication does not automatically mean failover.
 *
 * A resilient runtime may use replicas for recovery, but the resilience layer
 * decides whether:
 *
 *     retry;
 *     reroute;
 *     restart;
 *     resume;
 *     switch replica;
 *     rollback;
 *     abort
 *
 * is valid.
 *
 * This grammar remains unaware of those decisions.
 */


/* ============================================================================
 * NO AUTOMATIC PLACEMENT GUARANTEE
 * ========================================================================== */

/*
 * A placement expression such as:
 *
 *     placement: locality;
 *
 * is an abstract semantic preference/requirement.
 *
 * It does not imply:
 *
 *     region X;
 *     rack Y;
 *     node Z;
 *     device N;
 *     topology T.
 *
 * Physical realization belongs downstream.
 */


/* ============================================================================
 * DETERMINISM GUARANTEE
 * ========================================================================== */

/*
 * This grammar has:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no I/O;
 *     - no randomness;
 *     - no runtime callbacks;
 *     - no resource discovery;
 *     - no network calls;
 *     - no hardware calls.
 *
 * Therefore parsing depends only on the supplied token stream.
 */


/* ============================================================================
 * SOURCE ORDER GUARANTEE
 * ========================================================================== */

/*
 * Grammar repetition preserves source order in the parse tree.
 *
 * AST construction must preserve that order until semantic canonicalization
 * explicitly establishes that ordering is irrelevant.
 *
 * This matters for:
 *
 *     diagnostics;
 *     source mapping;
 *     deterministic serialization;
 *     tooling;
 *     round-trip printing.
 */


/* ============================================================================
 * HARD-CODING AUDIT
 * ========================================================================== */

/*
 * This file intentionally contains no:
 *
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_PARTITIONS
 *     MAX_SHARDS
 *     MAX_DEVICES
 *     MAX_WORKERS
 *     MAX_PROCESSES
 *
 * It also contains no:
 *
 *     fixed node ID;
 *     fixed device ID;
 *     fixed address;
 *     fixed port;
 *     fixed topology;
 *     fixed provider;
 *     fixed region;
 *     fixed cluster size;
 *     fixed hardware size.
 *
 * Any finite parser/runtime limit encountered during implementation must be
 * classified separately as:
 *
 *     1. language semantic requirement;
 *     2. parser implementation limit;
 *     3. resource constraint;
 *     4. runtime/deployment constraint;
 *     5. test-only limitation;
 *     6. accidental hard-coding.
 *
 * Accidental hard-coding must be removed.
 */


/* ============================================================================
 * INTEGRATION REQUIREMENTS
 * ========================================================================== */

/*
 * REQUIRED AGGREGATE INTEGRATION
 *
 * grammar/distributed/distributed.g4
 *
 * MUST:
 *
 *     - import Replication;
 *     - retain distributedReplicationDeclaration as the public branch;
 *     - remove any duplicate local implementation of that rule;
 *     - preserve the existing distributedDeclaration/member dispatch.
 *
 * REQUIRED SHARED DEPENDENCIES
 *
 * grammar/core/names.g4
 * grammar/expressions/expressions.g4
 *
 * MUST remain the owners of names and expressions.
 *
 * REQUIRED RESOURCE INTEGRATION
 *
 * grammar/resources/*
 *
 * remains the owner of resource/capability semantics.
 *
 * REQUIRED MEMORY INTEGRATION
 *
 * grammar/memory/distributed-memory.g4
 *
 * remains the owner of distributed-memory-specific syntax.
 *
 * REQUIRED NETWORK INTEGRATION
 *
 * grammar/networking/*
 *
 * remains the owner of network transport and endpoint semantics.
 *
 * REQUIRED HARDWARE INTEGRATION
 *
 * grammar/hardware/*
 *
 * remains the owner of physical hardware semantics.
 *
 * REQUIRED QUANTUM INTEGRATION
 *
 * grammar/quantum/*
 *
 * remains the owner of quantum semantics.
 *
 * REQUIRED RESILIENCE INTEGRATION
 *
 * src/quantum/resilience/*
 * or the repository's canonical resilience subsystem
 *
 * remains the owner of recovery/failure decisions.
 */


/* ============================================================================
 * AST CONTRACT
 * ========================================================================== */

/*
 * The frontend AST should represent this grammar with a dedicated distributed
 * replication node or equivalent canonical distributed-domain node.
 *
 * The AST SHOULD preserve at minimum:
 *
 *     declaration name;
 *     target expression;
 *     factor expression;
 *     policy/mode expressions;
 *     options;
 *     logical replica declarations;
 *     dependencies;
 *     nested scopes;
 *     source spans.
 *
 * The AST MUST NOT introduce:
 *
 *     PhysicalNodeId;
 *     DeviceId;
 *     QubitId;
 *     PhysicalQubitId;
 *     NetworkAddress;
 *     TransportHandle;
 *
 * merely because replication syntax exists.
 *
 * Physical realization belongs downstream.
 */


/* ============================================================================
 * IR CONTRACT
 * ========================================================================== */

/*
 * This grammar does NOT define an IR.
 *
 * After parsing and semantic analysis, the replication intent should be
 * lowered into the repository's canonical distributed semantic representation.
 *
 * If the replicated target is quantum:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * If the target is classical:
 *
 *     the canonical classical representation remains authoritative.
 *
 * If the target is HDL/hardware:
 *
 *     the canonical HDL/hardware representation remains authoritative.
 *
 * Replication metadata may accompany these representations through the
 * repository's canonical execution/resource metadata rather than creating a
 * second domain-specific IR.
 */


/* ============================================================================
 * COMPILER CONTRACT
 * ========================================================================== */

/*
 * Compiler stages downstream of this grammar are responsible for determining:
 *
 *     - whether replication is semantically valid;
 *     - whether the target is replicable;
 *     - required resources;
 *     - feasible replication factor;
 *     - consistency compatibility;
 *     - placement feasibility;
 *     - scheduling;
 *     - lowering;
 *     - target realization.
 *
 * Compilation must not silently convert a logical replication request into a
 * fixed physical deployment.
 */


/* ============================================================================
 * RUNTIME CONTRACT
 * ========================================================================== */

/*
 * Runtime systems may resolve:
 *
 *     available resources;
 *     replica placement;
 *     transport;
 *     synchronization;
 *     lifecycle;
 *     failure handling;
 *     dynamic scaling.
 *
 * Runtime behavior must remain constrained by the compiled semantic contract.
 *
 * A runtime must not silently change the program's semantic replication
 * requirement merely because a particular backend is convenient.
 */


/* ============================================================================
 * TOOLING CONTRACT
 * ========================================================================== */

/*
 * Tooling must be able to:
 *
 *     - syntax-highlight replication declarations;
 *     - locate replication declarations;
 *     - inspect logical replica names;
 *     - inspect replication expressions;
 *     - provide structural diagnostics;
 *     - preserve source spans;
 *     - format without changing semantics;
 *     - support future policy names without requiring every policy to be a
 *       lexer keyword.
 *
 * Language servers should rely on the AST rather than reparsing semantic
 * meaning from raw text.
 */


/* ============================================================================
 * TEST CONTRACT
 * ========================================================================== */

/*
 * REQUIRED POSITIVE TEST CATEGORIES
 *
 * 1. Minimal declaration
 *
 *     replication workload;
 *
 * 2. Target
 *
 *     replication workload {
 *         target: computation;
 *     }
 *
 * 3. Expression factor
 *
 *     replication workload {
 *         factor: desired_replicas;
 *     }
 *
 * 4. Literal factor
 *
 *     replication workload {
 *         factor: 3;
 *     }
 *
 * 5. Policy
 *
 *     replication workload {
 *         policy: distributed::replication::policy;
 *     }
 *
 * 6. Multiple options
 *
 * 7. Nested policy blocks
 *
 * 8. Logical replica declarations
 *
 * 9. Dependencies
 *
 * 10. Qualified policy names
 *
 * 11. Cross-domain target expressions
 *
 * 12. Large numbers of members
 *
 * 13. Deeply nested semantic blocks within implementation limits
 *
 * 14. Classical + replication
 *
 * 15. Quantum + replication metadata
 *
 * 16. HDL/hardware + replication metadata
 *
 * 17. Distributed + AI/data replication
 *
 * ============================================================================
 *
 * NEGATIVE TESTS
 *
 * Must reject structurally malformed forms including:
 *
 *     - missing declaration name;
 *     - missing `{`;
 *     - missing `}`;
 *     - missing `:`;
 *     - missing expression;
 *     - malformed argument list;
 *     - malformed replica declaration;
 *     - malformed dependency;
 *     - malformed qualified extension.
 *
 * Semantic-negative tests belong to semantic analysis rather than this parser.
 *
 * ============================================================================
 *
 * BOUNDARY TESTS
 *
 * Test:
 *
 *     zero optional members;
 *     one member;
 *     many members;
 *     deeply nested blocks;
 *     large expression trees;
 *     long qualified names;
 *     many logical replica declarations;
 *     large replication-factor expressions.
 *
 * No test may establish a false language maximum.
 *
 * ============================================================================
 *
 * SCALABILITY TESTS
 *
 * Verify that the grammar itself contains no artificial limit for:
 *
 *     replicas;
 *     nodes;
 *     machines;
 *     devices;
 *     resources;
 *     workers;
 *     program size.
 *
 * ============================================================================
 *
 * DETERMINISM TESTS
 *
 * The same token stream must produce equivalent parse structures across
 * repeated parser invocations.
 *
 * ============================================================================
 *
 * ROUND-TRIP TESTS
 *
 * Where the frontend provides a printer:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * must preserve intended replication semantics.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is COMPLETE only when:
 *
 * [ ] It compiles under the repository's canonical ANTLR build.
 *
 * [ ] Its imported grammar names match the actual repository grammar names.
 *
 * [ ] Its token names match the canonical Zamani lexer.
 *
 * [ ] `distributedReplicationDeclaration` is owned here exactly once.
 *
 * [ ] `distributed/distributed.g4` imports this component rather than
 *     redefining its replication rule.
 *
 * [ ] No duplicate identifier/name/expression grammar exists here.
 *
 * [ ] No physical resource identifiers are hard-coded.
 *
 * [ ] No finite replication maximum is encoded.
 *
 * [ ] Replication quantities are expression-based.
 *
 * [ ] Logical replica identity is separated from physical identity.
 *
 * [ ] Resource requirement/constraint/preference/hint semantics remain
 *     downstream.
 *
 * [ ] Consistency implementation remains downstream.
 *
 * [ ] Placement remains downstream.
 *
 * [ ] Networking remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] Hardware selection remains downstream.
 *
 * [ ] Quantum semantics remain downstream through `quantum::ir`.
 *
 * [ ] QEC and ZQN remain outside this grammar.
 *
 * [ ] Resilience remains outside this grammar.
 *
 * [ ] AST source-order and source-span preservation is defined.
 *
 * [ ] Positive parser tests exist.
 *
 * [ ] Negative parser tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] Documentation agrees with the grammar.
 *
 * [ ] No later grammar component needs to reopen this file merely to add a
 *     new replication policy or physical resource.
 *
 * ============================================================================
 *
 * END OF FILE
 * ============================================================================
 */