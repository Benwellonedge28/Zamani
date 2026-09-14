/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/consistency.g4
 *
 * Grammar:
 *     Consistency
 *
 * Status:
 *     Production distributed-consistency parser component.
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL SYNTAX for distributed consistency
 * requirements and policies.
 *
 * It expresses WHAT consistency semantics a program requires or prefers.
 *
 * It does NOT implement:
 *
 *     - consensus;
 *     - replication;
 *     - transaction protocols;
 *     - distributed locking;
 *     - quorum algorithms;
 *     - leader election;
 *     - failure detection;
 *     - network transport;
 *     - storage engines;
 *     - scheduling;
 *     - placement;
 *     - routing;
 *     - deployment;
 *     - runtime synchronization;
 *     - distributed recovery;
 *     - resilience;
 *     - hardware discovery;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir.
 *
 * Those responsibilities belong to downstream semantic, compiler, runtime,
 * networking, distributed, resource, scheduling, routing, and resilience
 * subsystems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - consistency declaration syntax;
 *     - consistency requirement syntax;
 *     - consistency policy syntax;
 *     - consistency scope syntax;
 *     - consistency property syntax;
 *     - consistency ordering intent;
 *     - consistency visibility intent;
 *     - consistency freshness intent;
 *     - consistency read/write guarantees;
 *     - consistency dependency relationships;
 *     - consistency policy expressions;
 *     - consistency extension syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier spelling;
 *     - qualified-name construction;
 *     - general expressions;
 *     - types;
 *     * replication;
 *     - nodes;
 *     - services;
 *     - communication;
 *     - networking;
 *     - messaging;
 *     - transactions;
 *     - consensus algorithms;
 *     - storage;
 *     - scheduling;
 *     - placement;
 *     - routing;
 *     - resource discovery;
 *     - hardware discovery;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime implementation.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Consistency is a SEMANTIC REQUIREMENT.
 *
 * It is not a machine property.
 *
 * Therefore:
 *
 *     consistency requirement
 *
 * is distinct from:
 *
 *     physical topology
 *     node count
 *     replica count
 *     network transport
 *     hardware identity
 *     provider identity
 *
 * A consistency declaration MUST remain valid when the same Zamani program
 * moves between different execution environments.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar supports:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Consistency syntax therefore describes semantic guarantees rather than
 * selecting a particular implementation.
 *
 * For example, a source program may request a named consistency guarantee,
 * but the grammar does NOT imply that the implementation must use:
 *
 *     Raft
 *     Paxos
 *     PBFT
 *     CRDT
 *     quorum replication
 *     locks
 *     transactions
 *     a particular database
 *     a particular transport
 *     a particular cloud provider.
 *
 * Those choices belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO finite language-level limit on:
 *
 *     - consistency declarations;
 *     - consistency properties;
 *     - consistency scopes;
 *     - policy expressions;
 *     - dependency expressions;
 *     - nested policy structures;
 *     - qualified names;
 *     - number of consistency requirements;
 *     - number of distributed entities;
 *     - number of replicas;
 *     - number of nodes.
 *
 * ANTLR repetition operators are used instead of artificial limits.
 *
 * There is deliberately NO:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_PROPERTIES
 *     MAX_REQUIREMENTS
 *     MAX_CONSISTENCY_LEVELS
 *
 * Practical resource limits belong outside the grammar.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * The grammar deliberately does not create a closed list of consistency
 * keywords.
 *
 * This permits semantic systems to evolve beyond currently known models.
 *
 * Examples of semantic names that may be represented include:
 *
 *     strong
 *     linearizable
 *     sequential
 *     causal
 *     eventual
 *     session
 *     monotonic_read
 *     monotonic_write
 *     read_your_writes
 *     consistent_prefix
 *     bounded_staleness
 *     custom
 *
 * These names are semantic identifiers.
 *
 * They are NOT permanently hard-coded lexer keywords.
 *
 * Future consistency models can therefore be introduced without necessarily
 * changing the lexical grammar.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser answers:
 *
 *     "Is the consistency construct structurally valid?"
 *
 * It does NOT answer:
 *
 *     "Can this consistency guarantee actually be implemented?"
 *
 *     "Which algorithm implements it?"
 *
 *     "Which nodes participate?"
 *
 *     "Which replicas participate?"
 *
 *     "Which transport is used?"
 *
 *     "How much latency will it have?"
 *
 *     "Does the target hardware support it?"
 *
 * Those questions belong downstream.
 *
 * ============================================================================
 * CONSISTENCY DIMENSIONS
 * ============================================================================
 *
 * A consistency declaration may express independent dimensions such as:
 *
 *     - ordering;
 *     - visibility;
 *     - freshness;
 *     - read guarantees;
 *     - write guarantees;
 *     - session guarantees;
 *     - synchronization requirements;
 *     - durability-related requirements;
 *     - conflict semantics;
 *     - convergence requirements;
 *     - custom policy properties.
 *
 * The grammar preserves these dimensions independently.
 *
 * Semantic analysis determines whether their combination is meaningful.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * Consistency is NOT replication.
 *
 * A consistency requirement may apply to:
 *
 *     - replicated state;
 *     - shared state;
 *     - distributed computation;
 *     - messages;
 *     - services;
 *     - data;
 *     - execution results;
 *     - logical state.
 *
 * It does not require that replication be present.
 *
 * Likewise:
 *
 * replication does not imply a particular consistency model.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Distributed quantum computation may use consistency requirements for:
 *
 *     - classical control state;
 *     - distributed measurement results;
 *     - orchestration metadata;
 *     - execution coordination;
 *     - distributed classical state;
 *     - logical protocol state.
 *
 * This grammar MUST NOT interpret consistency as permission to duplicate
 * arbitrary quantum states.
 *
 * It MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     Gate
 *     Circuit
 *     QuantumOperation
 *     quantum topology
 *     calibration
 *     pulse semantics
 *     QEC
 *     ZQN
 *
 * Quantum semantics remain owned by the quantum subsystem and ultimately
 * cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * NETWORK BOUNDARY
 * ============================================================================
 *
 * A consistency requirement does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     RPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transport
 *
 * Network realization is downstream.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * This grammar does not encode:
 *
 *     - CPU count;
 *     - GPU count;
 *     - QPU count;
 *     - node count;
 *     - memory capacity;
 *     - network bandwidth;
 *     - network latency;
 *     - topology;
 *     - device identifier;
 *     - machine identifier;
 *     - physical address.
 *
 * Such information belongs to capabilities, resources, targets, deployment,
 * scheduling, or runtime systems.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * This component consumes:
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
 * This grammar MUST NOT redefine those rules.
 *
 * ============================================================================
 * AGGREGATE INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/distributed/distributed.g4 MUST import this grammar component.
 *
 * The aggregate distributed grammar MUST route its consistency declaration
 * branch to:
 *
 *     distributedConsistencyDeclaration
 *
 * It MUST NOT create a second grammar rule for the same responsibility.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file introduces NO lexer rules.
 *
 * No global tokens are introduced for:
 *
 *     consistency
 *     strong
 *     eventual
 *     causal
 *     linearizable
 *     sequential
 *     session
 *     quorum
 *     freshness
 *     ordering
 *     visibility
 *
 * These remain semantic names.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - declaration ordering;
 *     - source spans;
 *     - target names;
 *     - property ordering;
 *     - expression structure;
 *     - policy names;
 *     - requirement names;
 *     - nested scope structure.
 *
 * The parser MUST NOT:
 *
 *     - reorder properties;
 *     - resolve policies;
 *     - select algorithms;
 *     - select replicas;
 *     - select nodes;
 *     - select transports.
 *
 * ============================================================================
 * DUPLICATE PROPERTY POLICY
 * ============================================================================
 *
 * Repeated properties are syntactically accepted.
 *
 * Example:
 *
 *     consistency state {
 *         requirement: read_your_writes;
 *         requirement: monotonic_read;
 *     }
 *
 * The grammar preserves both.
 *
 * Semantic analysis determines whether they:
 *
 *     - compose;
 *     - conflict;
 *     - override;
 *     - are redundant;
 *     - are invalid.
 *
 * The parser must not silently discard information.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no Rust code;
 *     - no filesystem operations;
 *     - no networking;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no hardware discovery.
 *
 * Parsing therefore depends only on the input token stream.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The grammar contains no embedded Rust.
 *
 * Generated frontend/runtime integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * Generated parser integration must use safe Rust.
 *
 * This grammar itself introduces no unsafe code.
 *
 * ============================================================================
 */

parser grammar Consistency;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable rule consumed by:
 *
 *     grammar/distributed/distributed.g4
 */
distributedConsistencyDeclaration
    : consistencyDeclaration
    ;


/*
 * ============================================================================
 * CONSISTENCY DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     consistency <target> <body>
 *
 * The words "consistency" and the target are identifiers rather than global
 * lexer keywords. Semantic analysis performs contextual classification.
 */
consistencyDeclaration
    : identifier
      identifier
      consistencyBody
    ;


/*
 * ============================================================================
 * CONSISTENCY BODY
 * ============================================================================
 *
 * A consistency declaration may be empty at syntax level. Semantic validation
 * decides whether an empty declaration is meaningful.
 */
consistencyBody
    : LBRACE
      consistencyMember*
      RBRACE
    ;


/*
 * ============================================================================
 * CONSISTENCY MEMBERS
 * ============================================================================
 */
consistencyMember
    : consistencyRequirement
    | consistencyPolicy
    | consistencyOrdering
    | consistencyVisibility
    | consistencyFreshness
    | consistencyReadGuarantee
    | consistencyWriteGuarantee
    | consistencySessionGuarantee
    | consistencyConflictPolicy
    | consistencyConvergence
    | consistencySynchronization
    | consistencyDependency
    | consistencyConstraint
    | consistencyPreference
    | consistencyHint
    | consistencyExtension
    ;


/*
 * ============================================================================
 * GENERIC NAMED REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     requirement: strong;
 *     requirement: causal;
 *     requirement: read_your_writes;
 *
 * The value is an expression so future policy systems are not constrained to
 * a fixed enumeration.
 */
consistencyRequirement
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Examples:
 *
 *     policy: linearizable;
 *     policy: eventual;
 *     policy: custom_policy();
 */
consistencyPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Represents ordering intent without selecting the implementation algorithm.
 *
 * Examples:
 *
 *     ordering: causal;
 *     ordering: sequential;
 *     ordering: total;
 */
consistencyOrdering
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Represents when writes/updates become observable.
 */
consistencyVisibility
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * FRESHNESS
 * ============================================================================
 *
 * Freshness may be represented as an arbitrary expression so units and
 * semantic validation remain downstream responsibilities.
 *
 * Examples:
 *
 *     freshness: policy;
 *     freshness: max_staleness;
 *     freshness: duration_expression;
 */
consistencyFreshness
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * READ GUARANTEE
 * ============================================================================
 *
 * Examples:
 *
 *     read: read_your_writes;
 *     read: monotonic;
 *     read: consistent_prefix;
 */
consistencyReadGuarantee
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * WRITE GUARANTEE
 * ============================================================================
 *
 * Examples:
 *
 *     write: monotonic;
 *     write: ordered;
 *     write: durable;
 */
consistencyWriteGuarantee
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SESSION GUARANTEE
 * ============================================================================
 *
 * Session-level guarantees are syntactically separate from general read/write
 * guarantees so the AST can preserve their semantic dimension.
 */
consistencySessionGuarantee
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONFLICT POLICY
 * ============================================================================
 *
 * Conflict resolution is expressed as semantic intent.
 *
 * The grammar does not implement:
 *
 *     CRDT
 *     merge algorithms
 *     last-write-wins
 *     application-specific conflict resolution
 *
 * unless those are subsequently defined by semantic/runtime components.
 */
consistencyConflictPolicy
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONVERGENCE
 * ============================================================================
 *
 * Represents convergence requirements without requiring a particular
 * distributed algorithm.
 */
consistencyConvergence
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SYNCHRONIZATION
 * ============================================================================
 *
 * Represents synchronization requirements without selecting locks, barriers,
 * consensus, or transport mechanisms.
 */
consistencySynchronization
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * DEPENDENCY
 * ============================================================================
 *
 * A consistency policy may depend on another semantic entity.
 *
 * Examples:
 *
 *     depends_on: state.policy;
 *     depends_on: distributed.consistency;
 */
consistencyDependency
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Constraints remain semantic constraints rather than physical machine
 * declarations.
 */
consistencyConstraint
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences are intentionally weaker than requirements.
 *
 * Semantic analysis decides whether a preference can be honored.
 */
consistencyPreference
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Hints do not become mandatory semantic requirements merely because they are
 * syntactically present.
 */
consistencyHint
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * EXTENSION
 * ============================================================================
 *
 * Open-world extension point.
 *
 * A dialect/provider-independent semantic extension can be represented using
 * an identifier and expression without modifying this grammar for every
 * future distributed consistency concept.
 */
consistencyExtension
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * NESTED CONSISTENCY SCOPE
 * ============================================================================
 *
 * This rule is provided for future aggregate integration where a consistency
 * policy needs an explicitly nested semantic scope.
 *
 * It does not introduce a machine-specific hierarchy.
 */
consistencyScope
    : identifier
      LBRACE
      consistencyMember*
      RBRACE
    ;