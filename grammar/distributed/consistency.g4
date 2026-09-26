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
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL SYNTAX for distributed consistency intent.
 *
 * It describes WHAT consistency properties a program requires, declares,
 * prefers, constrains, or references.
 *
 * It does NOT implement a consistency algorithm.
 *
 * It does NOT select:
 *
 *     - Raft;
 *     - Paxos;
 *     - PBFT;
 *     - CRDT implementation;
 *     - quorum algorithm;
 *     - lock implementation;
 *     - database;
 *     - storage engine;
 *     - network transport;
 *     - scheduler;
 *     - placement;
 *     - node;
 *     - replica;
 *     - hardware;
 *     - cloud provider.
 *
 * Those concerns belong to semantic analysis, resource/capability analysis,
 * distributed compilation, scheduling, placement, runtime, and deployment.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - consistency constructs;
 *     - consistency declarations;
 *     - consistency invocations;
 *     - consistency bodies;
 *     - consistency properties;
 *     - consistency property values;
 *     - nested consistency objects;
 *     - consistency lists;
 *     - consistency references;
 *     - consistency extension syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - types;
 *     - resources;
 *     - capabilities;
 *     - nodes;
 *     - replicas;
 *     - replication;
 *     - partitioning;
 *     - transactions;
 *     - consensus;
 *     - fault tolerance;
 *     - resilience;
 *     - placement;
 *     - topology;
 *     - networking;
 *     - messaging;
 *     - storage;
 *     - scheduling;
 *     - deployment;
 *     - quantum operations;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - runtime implementation.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Consistency is semantic intent.
 *
 * The grammar therefore uses an OPEN-WORLD property model.
 *
 * It deliberately does not enumerate every known consistency model as a
 * grammar alternative.
 *
 * Examples of semantic values that may be represented include:
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
 * These are semantic names, not permanently reserved grammar keywords.
 *
 * A future consistency model can therefore be introduced by semantic
 * validation without requiring the parser grammar to become a catalogue of
 * algorithms or databases.
 *
 * ============================================================================
 * CANONICAL STRUCTURE
 * ============================================================================
 *
 * Two source forms are supported:
 *
 *     1. Declarative consistency:
 *
 *        consistency state {
 *            requirement: strong;
 *            ordering: causal;
 *        }
 *
 *     2. Consistency invocation:
 *
 *        consistency.policy(state, strong);
 *
 * The first form expresses named consistency intent.
 *
 * The second form provides an extensible invocation mechanism for semantic
 * libraries, dialects, or higher-level constructs.
 *
 * The grammar intentionally does not require the invocation name to refer to
 * a particular implementation.
 *
 * ============================================================================
 * OPEN WORLD
 * ============================================================================
 *
 * No closed enumeration exists for:
 *
 *     consistency levels
 *     algorithms
 *     protocols
 *     databases
 *     transports
 *     conflict resolvers
 *     convergence mechanisms
 *
 * New semantic models can be introduced without expanding this grammar.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is NO grammar-level maximum for:
 *
 *     - consistency declarations;
 *     - properties;
 *     - nested objects;
 *     - list elements;
 *     - requirements;
 *     - policy expressions;
 *     - consistency scopes;
 *     - distributed entities;
 *     - nodes;
 *     - replicas;
 *     - partitions;
 *     - resources.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_PROPERTIES
 *     MAX_REQUIREMENTS
 *     MAX_PARTITIONS
 *     MAX_CONSISTENCY_LEVELS
 *
 * or equivalent artificial limits.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Consistency syntax must remain portable across target environments.
 *
 * A source program may describe:
 *
 *     what consistency guarantee is required;
 *     what consistency properties are preferred;
 *     what ordering is required;
 *     what freshness is acceptable;
 *     what read/write guarantees are required;
 *     what conflicts must satisfy;
 *
 * It does not directly describe:
 *
 *     which CPU;
 *     which GPU;
 *     which node;
 *     which replica;
 *     which database;
 *     which network;
 *     which storage device.
 *
 * Target realization occurs downstream.
 *
 * ============================================================================
 * RESOURCE SEPARATION
 * ============================================================================
 *
 * Consistency properties are not hardware capacities.
 *
 * A consistency declaration may contain semantic expressions referring to
 * resources or capabilities, but this grammar does not define those resource
 * semantics.
 *
 * For example, resource/capability systems may independently express:
 *
 *     requires capability("distributed.consistency");
 *     requires memory >= required_memory;
 *
 * Such requirements are semantic contracts and are not parser limits.
 *
 * ============================================================================
 * DUPLICATE PROPERTY POLICY
 * ============================================================================
 *
 * Repeated properties are syntactically preserved.
 *
 * Example:
 *
 *     consistency state {
 *         requirement: read_your_writes;
 *         requirement: monotonic_read;
 *     }
 *
 * The parser does not merge, override, reorder, or discard them.
 *
 * Semantic analysis decides whether repeated properties:
 *
 *     - compose;
 *     - conflict;
 *     - override;
 *     - are redundant;
 *     - are invalid.
 *
 * ============================================================================
 * PROPERTY MODEL
 * ============================================================================
 *
 * The grammar intentionally has ONE property production.
 *
 * Do NOT create separate parser productions such as:
 *
 *     consistencyStrong
 *     consistencyCausal
 *     consistencyEventual
 *     consistencyOrdering
 *     consistencyFreshness
 *     ...
 *
 * when their syntax is identical.
 *
 * The property name is semantic data.
 *
 * This avoids parser ambiguity and prevents grammar-level duplication.
 *
 * ============================================================================
 * NESTED VALUES
 * ============================================================================
 *
 * Property values may be:
 *
 *     - general expressions;
 *     - nested objects;
 *     - lists.
 *
 * This permits extensible structures without requiring a new grammar rule
 * every time the semantic consistency model gains another dimension.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Distributed quantum systems may use consistency for:
 *
 *     - classical control state;
 *     - measurement results;
 *     - orchestration metadata;
 *     - distributed protocol state;
 *     - classical feed-forward state.
 *
 * This grammar MUST NOT interpret consistency as replication of quantum state.
 *
 * It MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumOperation
 *     Circuit
 *     QuantumTopology
 *     Calibration
 *     Pulse
 *     QEC
 *     ZQN
 *
 * Quantum semantics remain owned by the quantum subsystem and cross the
 * canonical quantum::ir boundary downstream.
 *
 * ============================================================================
 * NETWORK BOUNDARY
 * ============================================================================
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
 *     vendor transport.
 *
 * Network realization is downstream.
 *
 * ============================================================================
 * REPLICATION BOUNDARY
 * ============================================================================
 *
 * Consistency and replication are distinct concepts.
 *
 * Replication owns:
 *
 *     whether/how state is replicated;
 *
 * consistency owns:
 *
 *     what visibility/order/freshness guarantees apply.
 *
 * A consistency declaration may apply to replicated or non-replicated state.
 *
 * This grammar therefore MUST NOT duplicate replication syntax.
 *
 * ============================================================================
 * TRANSACTION BOUNDARY
 * ============================================================================
 *
 * Transaction semantics remain owned by:
 *
 *     grammar/distributed/transactions.g4
 *
 * This grammar may express consistency properties associated semantically
 * with transactions, but does not define transaction syntax.
 *
 * ============================================================================
 * FAULT-TOLERANCE BOUNDARY
 * ============================================================================
 *
 * Failure handling remains owned by:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * Consistency may constrain the result of recovery, but does not implement
 * recovery.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser must preserve enough structure for the frontend AST to retain:
 *
 *     - source spans;
 *     - declaration order;
 *     - property order;
 *     - property names;
 *     - expressions;
 *     - nested objects;
 *     - lists;
 *     - invocation names;
 *     - invocation arguments.
 *
 * No parser action may resolve semantic policy.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The stable AST contract is conceptually:
 *
 *     ConsistencyDeclaration
 *         name
 *         body
 *
 *     ConsistencyInvocation
 *         qualified_name
 *         arguments
 *
 *     ConsistencyProperty
 *         name
 *         value
 *
 *     ConsistencyValue
 *         expression
 *         object
 *         list
 *
 * The exact Rust AST type names remain owned by the frontend AST subsystem.
 *
 * This grammar must not require vendor-specific AST nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - classify the declaration;
 *     - validate contextual property names;
 *     - resolve names;
 *     - validate expressions;
 *     - detect incompatible properties;
 *     - validate referenced capabilities;
 *     - validate resource requirements;
 *     - determine whether a requested consistency model is implementable;
 *     - preserve diagnostics with source spans.
 *
 * The parser must NOT perform these tasks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO new IR.
 *
 * Consistency intent is lowered by semantic/IR infrastructure into the
 * repository's canonical distributed semantic/IR representation.
 *
 * This file MUST NOT introduce:
 *
 *     ConsistencyIR
 *     DistributedConsistencyIR
 *     ReplicaConsistencyIR
 *
 * merely as a frontend convenience.
 *
 * ============================================================================
 * IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * The grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks;
 *     - no hardware discovery;
 *     - no randomness.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * Generated and handwritten compiler code must use safe Rust only.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is the canonical syntax owner for distributed consistency.
 *
 * grammar/distributed/distributed.g4 MUST:
 *
 *     1. import Consistency;
 *     2. route distributedConsistency to
 *        distributedConsistencyConstruct;
 *     3. remove its duplicate inline consistency invocation syntax.
 *
 * Conceptually:
 *
 *     distributedConstruct
 *         ...
 *         | distributedConsistency
 *         ...
 *         ;
 *
 *     distributedConsistency
 *         : distributedConsistencyConstruct
 *         ;
 *
 * No second consistency grammar should be introduced.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar introduces no lexer tokens.
 *
 * In particular, it does NOT require global keywords for:
 *
 *     consistency
 *     strong
 *     causal
 *     eventual
 *     linearizable
 *     sequential
 *     session
 *     freshness
 *     ordering
 *     visibility
 *     quorum
 *
 * Contextual classification is a semantic responsibility unless the
 * repository-wide lexical specification later deliberately promotes a word
 * to a reserved keyword.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar is deliberately structured so that its primary alternatives
 * have distinguishable shapes:
 *
 *     identifier identifier ...
 *
 * versus:
 *
 *     qualifiedName LPAREN ...
 *
 * Property syntax has exactly one production rather than many identical
 * alternatives.
 *
 * This minimizes ambiguity and makes parser behavior deterministic.
 *
 * ============================================================================
 * VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * Production validation must verify:
 *
 *     - no unreachable rules;
 *     - no duplicate alternatives;
 *     - no ambiguity introduced by imports;
 *     - no left-recursion violations;
 *     - deterministic property parsing;
 *     - source-span preservation;
 *     - AST coverage;
 *     - semantic coverage;
 *     - IR coverage;
 *     - positive cases;
 *     - negative cases;
 *     - boundary cases;
 *     - scalability cases;
 *     - compatibility cases.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no universal capacity constants.
 *
 * Forbidden examples include:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_PROPERTIES
 *     MAX_PARTITIONS
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * No physical identifiers are built into the syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] ownership is explicit
 *     [x] non-ownership is explicit
 *     [x] lexical dependencies are explicit
 *     [x] declaration syntax is defined
 *     [x] invocation syntax is defined
 *     [x] property syntax is unambiguous
 *     [x] nested values are supported
 *     [x] arbitrary expressions are delegated
 *     [x] no fixed consistency enumeration exists
 *     [x] no hardware limits exist
 *     [x] no duplicate consistency property productions exist
 *     [x] AST contract is defined
 *     [x] semantic boundary is defined
 *     [x] IR boundary is defined
 *     [x] distributed integration is defined
 *     [x] quantum boundary is defined
 *     [x] replication boundary is defined
 *     [x] Rust compatibility is defined
 *     [x] safe-Rust requirement is defined
 *     [x] validation requirements are defined
 *     [x] hard-coding audit is defined
 *
 * Repository integration and conformance tests must be completed as
 * repository-level work described below this file.
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
 * Stable entry point consumed by distributed.g4.
 */
distributedConsistencyConstruct
    : consistencyConstruct
    ;


/*
 * ============================================================================
 * CONSISTENCY CONSTRUCT
 * ============================================================================
 *
 * The two forms have intentionally different token shapes:
 *
 *     declaration:
 *         identifier identifier body
 *
 *     invocation:
 *         qualifiedName LPAREN ...
 *
 * This keeps dispatch deterministic without requiring a new lexer keyword.
 */
consistencyConstruct
    : consistencyDeclaration
    | consistencyInvocation
    ;


/*
 * ============================================================================
 * DECLARATION
 * ============================================================================
 *
 * Canonical semantic form:
 *
 *     consistency <name> { ... }
 *
 * The first identifier is contextually classified as "consistency" by
 * semantic analysis.
 *
 * Examples:
 *
 *     consistency state {
 *         requirement: strong;
 *     }
 *
 *     consistency shared_state {
 *         ordering: causal;
 *         freshness: bounded_staleness(10, milliseconds);
 *     }
 *
 * The grammar deliberately does not reserve "consistency" globally.
 */
consistencyDeclaration
    : identifier
      identifier
      consistencyBody
    ;


/*
 * ============================================================================
 * INVOCATION
 * ============================================================================
 *
 * Canonical extensible form:
 *
 *     consistency.policy(state, strong);
 *     distributed.consistency(state, causal);
 *
 * The qualified name remains semantic data.
 */
consistencyInvocation
    : qualifiedName
      LPAREN
      expressionList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 */
consistencyBody
    : LBRACE
      consistencyMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MEMBERS
 * ============================================================================
 *
 * A member is either:
 *
 *     property:
 *         name : value
 *
 * or:
 *
 *     nested declaration:
 *         name { ... }
 *
 * The second form permits semantic namespaces without creating a new grammar
 * rule for every future consistency dimension.
 */
consistencyMember
    : consistencyProperty
    | consistencySection
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * ONE property production intentionally replaces the former collection of
 * syntactically identical productions.
 *
 * Examples:
 *
 *     requirement: strong;
 *     policy: linearizable;
 *     ordering: causal;
 *     visibility: read_after_write;
 *     freshness: bounded_staleness(10, milliseconds);
 *     read: read_your_writes;
 *     write: monotonic;
 *     session: monotonic_reads;
 *     conflict: application_defined;
 *     convergence: eventual;
 *     synchronization: causal;
 *     preference: low_latency;
 *     hint: local_preference;
 *
 * Property names are semantic identifiers.
 */
consistencyProperty
    : identifier
      COLON
      consistencyValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * NESTED SECTION
 * ============================================================================
 *
 * Examples:
 *
 *     consistency state {
 *         read {
 *             guarantee: read_your_writes;
 *             ordering: causal;
 *         }
 *     }
 *
 *     consistency state {
 *         requirements {
 *             availability: expression;
 *             durability: expression;
 *         }
 *     }
 *
 * The parser preserves the section name and structure.
 */
consistencySection
    : identifier
      consistencyBody
    ;


/*
 * ============================================================================
 * VALUE
 * ============================================================================
 *
 * Values delegate normal computation/expression syntax to Expressions.
 */
consistencyValue
    : expression
    | consistencyObject
    | consistencyList
    ;


/*
 * ============================================================================
 * OBJECT
 * ============================================================================
 *
 * Objects intentionally reuse the same property/section structure.
 */
consistencyObject
    : LBRACE
      consistencyMember*
      RBRACE
    ;


/*
 * ============================================================================
 * LIST
 * ============================================================================
 *
 * Lists are unbounded at the grammar level.
 */
consistencyList
    : LBRACKET
      consistencyListElement*
      RBRACKET
    ;


/*
 * ============================================================================
 * LIST ELEMENT
 * ============================================================================
 */
consistencyListElement
    : expression
    | consistencyObject
    | consistencyList
    ;