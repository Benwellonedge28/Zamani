/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/consistency.g4
 *
 * Grammar:
 *     DistributedConsistency
 *
 * Status:
 *     Production distributed-consistency parser component.
 *
 * Language baseline:
 *     Rust 1.97+
 *     Rust Edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL SYNTAX for distributed consistency
 * intent.
 *
 * Consistency describes semantic guarantees concerning visibility, ordering,
 * freshness, convergence, session behavior, read/write relationships, and
 * other consistency properties.
 *
 * This grammar expresses WHAT a program requires or declares.
 *
 * It does NOT determine HOW that requirement is implemented.
 *
 * It therefore does not select or implement:
 *
 *     - consensus algorithms;
 *     - replication algorithms;
 *     - quorum algorithms;
 *     - locking algorithms;
 *     - conflict-resolution algorithms;
 *     - databases;
 *     - storage engines;
 *     - network transports;
 *     - schedulers;
 *     - placement;
 *     - deployment;
 *     - hardware;
 *     - cloud providers;
 *     - runtime implementations.
 *
 * Those concerns belong to downstream semantic, compiler, runtime, resource,
 * capability, placement, routing, scheduling, resilience, and deployment
 * layers.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     DistributedConsistency parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          +--> distributed consistency semantics
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed execution metadata
 *          |
 *          v
 *     optimization
 *          |
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 * This grammar is upstream of all implementation decisions.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - consistency declarations;
 *     - consistency invocation syntax;
 *     - consistency bodies;
 *     - consistency members;
 *     - consistency properties;
 *     - consistency sections;
 *     - consistency values;
 *     - consistency lists;
 *     - consistency extension calls;
 *     - consistency references as syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer vocabulary;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - types;
 *     - resources;
 *     - capabilities;
 *     - effects;
 *     - policies;
 *     - contracts;
 *     - nodes;
 *     - processes;
 *     - actors;
 *     - channels;
 *     - messages;
 *     - services;
 *     - replication;
 *     - partitioning;
 *     - transactions;
 *     - fault tolerance;
 *     - topology;
 *     - placement;
 *     - networking;
 *     - storage;
 *     - scheduling;
 *     - deployment;
 *     - quantum operations;
 *     - quantum state;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * PUBLIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The stable public parser rule is:
 *
 *     distributedConsistencyConstruct
 *
 * `grammar/distributed/distributed.g4` already consumes this rule.
 *
 * That composition boundary MUST remain stable.
 *
 * The distributed composition root MUST NOT duplicate the rules below.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         -> grammar/lexer/lexer.g4
 *             -> grammar/lexer/tokens.g4
 *                 -> grammar/lexer/keywords.g4
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Expressions:
 *
 *     grammar/expressions/
 *
 * Canonical distributed composition:
 *
 *     grammar/distributed/distributed.g4
 *
 * Related distributed ownership:
 *
 *     grammar/distributed/replication.g4
 *     grammar/distributed/partitioning.g4
 *     grammar/distributed/transactions.g4
 *     grammar/distributed/fault-tolerance.g4
 *     grammar/distributed/topology.g4
 *     grammar/distributed/placement.g4
 *     grammar/distributed/services.g4
 *     grammar/distributed/processes.g4
 *     grammar/distributed/actors.g4
 *     grammar/distributed/channels.g4
 *     grammar/distributed/communication.g4
 *     grammar/distributed/messaging.g4
 *
 * Resource/capability ownership:
 *
 *     grammar/resources/
 *
 * Effect ownership:
 *
 *     grammar/effects/
 *
 * Policy ownership:
 *
 *     grammar/policies/
 *
 * Contract ownership:
 *
 *     grammar/validation/
 *     grammar/distributed/contracts.g4
 *
 * Provenance ownership:
 *
 *     grammar/spec/provenance.md
 *     grammar/specification/
 *
 * ============================================================================
 * LEXER DECISION
 * ============================================================================
 *
 * `consistency` is a language-level distributed semantic construct.
 *
 * Unlike arbitrary consistency-model names, which remain ordinary identifiers,
 * the declaration marker itself is reserved.
 *
 * Therefore the canonical lexer vocabulary must expose:
 *
 *     CONSISTENCY : 'consistency'
 *
 * in:
 *
 *     grammar/lexer/keywords.g4
 *
 * This is intentionally different from reserving:
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
 *     bounded_staleness
 *
 * Those remain semantic names/identifiers.
 *
 * This distinction keeps the grammar open-world.
 *
 * ============================================================================
 * OPEN-WORLD CONSISTENCY MODEL
 * ============================================================================
 *
 * The grammar MUST NOT enumerate every consistency model.
 *
 * Examples of semantic names that remain ordinary identifiers include:
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
 *     conflict_free
 *     application_defined
 *     vendor_model
 *     future_model
 *
 * A new consistency model therefore does not require a parser grammar change.
 *
 * Semantic analysis determines whether a referenced model is:
 *
 *     - standard;
 *     - library-defined;
 *     - dialect-defined;
 *     - vendor-defined;
 *     - experimental;
 *     - deprecated;
 *     - unsupported;
 *     - unknown.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Consistency syntax is target-independent.
 *
 * The same source can express consistency intent for:
 *
 *     - a tiny embedded realization;
 *     - a single CPU;
 *     - multicore execution;
 *     - GPU-assisted execution;
 *     - FPGA execution;
 *     - ASIC execution;
 *     - accelerator execution;
 *     - QPU-assisted execution;
 *     - simulation;
 *     - HPC;
 *     - clusters;
 *     - federated systems;
 *     - cloud systems;
 *     - heterogeneous systems;
 *     - future computational substrates.
 *
 * Source syntax MUST NOT encode a physical machine count.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO grammar-level maximum for:
 *
 *     - consistency declarations;
 *     - consistency members;
 *     - properties;
 *     - nested sections;
 *     - list elements;
 *     - extension arguments;
 *     - expression depth;
 *     - qualified-name depth;
 *     - distributed entities;
 *     - nodes;
 *     - processes;
 *     - actors;
 *     - replicas;
 *     - partitions;
 *     - resources;
 *     - devices;
 *     - machines.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_PROPERTIES
 *     MAX_PARTITIONS
 *     MAX_CONSISTENCY_LEVELS
 *     MAX_CONSISTENCY_RULES
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *
 * or equivalent limits.
 *
 * "Infinity" means that the language imposes no artificial semantic ceiling.
 *
 * Actual execution remains bounded by available:
 *
 *     memory;
 *     storage;
 *     compute;
 *     network capacity;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     time;
 *     deployment resources.
 *
 * Those are not language-level limits.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Consistency values may reference the universal resource/capability/policy
 * system.
 *
 * For example:
 *
 *     consistency state {
 *         requirement: strong;
 *         capability: capability("distributed.consistency");
 *         preference: low_latency;
 *     }
 *
 * or:
 *
 *     consistency state {
 *         requirements {
 *             capability("distributed.consistency");
 *             resources.available("memory");
 *         }
 *     }
 *
 * This grammar merely preserves those expressions.
 *
 * It does NOT evaluate them.
 *
 * Semantic analysis determines whether they are requirements, constraints,
 * preferences, hints, capabilities, effects, policies, or other semantic
 * entities.
 *
 * ============================================================================
 * DECLARATION SHAPE
 * ============================================================================
 *
 * Canonical declaration:
 *
 *     consistency state {
 *         requirement: strong;
 *         ordering: causal;
 *     }
 *
 * The dedicated CONSISTENCY token provides a deterministic declaration
 * boundary.
 *
 * This intentionally replaces the previous ambiguous structural form:
 *
 *     identifier identifier body
 *
 * which could accidentally classify unrelated distributed declarations as
 * consistency declarations.
 *
 * ============================================================================
 * INVOCATION SHAPE
 * ============================================================================
 *
 * Canonical invocation:
 *
 *     consistency::policy(state, strong);
 *
 *     distributed::consistency::require(state, causal);
 *
 * The invocation form remains open-world.
 *
 * The qualified name is semantic data.
 *
 * ============================================================================
 * DECLARATION VS INVOCATION
 * ============================================================================
 *
 * Declaration:
 *
 *     CONSISTENCY identifier body
 *
 * Invocation:
 *
 *     qualifiedName '(' arguments ')' ';'
 *
 * These forms have structurally distinct shapes.
 *
 * The parser therefore does not need semantic predicates.
 *
 * ============================================================================
 * PROPERTY MODEL
 * ============================================================================
 *
 * There is exactly ONE generic property production.
 *
 * Examples:
 *
 *     requirement: strong;
 *     ordering: causal;
 *     visibility: read_your_writes;
 *     freshness: bounded_staleness;
 *     conflict: application_defined;
 *     convergence: eventual;
 *     synchronization: causal;
 *     preference: low_latency;
 *     hint: locality;
 *
 * Property names remain semantic identifiers.
 *
 * The parser preserves repeated properties.
 *
 * Semantic analysis determines whether repetition means:
 *
 *     - composition;
 *     - conjunction;
 *     - override;
 *     - conflict;
 *     - redundancy;
 *     - invalidity.
 *
 * ============================================================================
 * SECTION MODEL
 * ============================================================================
 *
 * Named sections provide structured semantic namespaces without requiring a
 * new grammar production for every future consistency dimension.
 *
 * Example:
 *
 *     consistency state {
 *         read {
 *             guarantee: read_your_writes;
 *             ordering: causal;
 *         }
 *
 *         write {
 *             guarantee: monotonic_write;
 *         }
 *     }
 *
 * ============================================================================
 * VALUE MODEL
 * ============================================================================
 *
 * Consistency values are intentionally open.
 *
 * They may be:
 *
 *     - ordinary Zamani expressions;
 *     - nested consistency objects;
 *     - consistency lists.
 *
 * This allows future semantic models without modifying this grammar.
 *
 * ============================================================================
 * LIST MODEL
 * ============================================================================
 *
 * Lists are structurally unbounded:
 *
 *     [
 *         expression,
 *         expression,
 *         ...
 *     ]
 *
 * No grammar-level element limit exists.
 *
 * The parser preserves list order.
 *
 * ============================================================================
 * EXTENSION MODEL
 * ============================================================================
 *
 * Future/vendor/dialect-specific consistency semantics can be represented
 * without modifying the universal grammar.
 *
 * Example:
 *
 *     vendor::consistency::property(value);
 *
 *     future::consistency::model(argument);
 *
 *     dialect::consistency::guarantee(expression);
 *
 * The semantic layer decides whether the extension is valid.
 *
 * ============================================================================
 * REPLICATION BOUNDARY
 * ============================================================================
 *
 * Replication remains owned by:
 *
 *     grammar/distributed/replication.g4
 *
 * Replication answers questions such as logical multiplicity and replication
 * relationships.
 *
 * Consistency answers questions such as visibility, ordering, freshness, and
 * convergence.
 *
 * This grammar MUST NOT define:
 *
 *     replica;
 *     replication;
 *     replica_factor;
 *     replica_placement;
 *     replica_group;
 *
 * unless those concepts are expressed as ordinary semantic expressions inside
 * a consistency property.
 *
 * ============================================================================
 * PARTITIONING BOUNDARY
 * ============================================================================
 *
 * Partitioning remains owned by:
 *
 *     grammar/distributed/partitioning.g4
 *
 * This grammar does not define:
 *
 *     shard;
 *     partition;
 *     partition count;
 *     shard placement;
 *     partition routing.
 *
 * Such concepts can be referenced semantically through expressions where
 * appropriate.
 *
 * ============================================================================
 * TRANSACTION BOUNDARY
 * ============================================================================
 *
 * Transaction syntax remains owned by:
 *
 *     grammar/distributed/transactions.g4
 *
 * This grammar may express properties semantically associated with
 * transactions, for example:
 *
 *     consistency transaction_scope {
 *         isolation: serializable;
 *     }
 *
 * but it does not define transaction syntax.
 *
 * ============================================================================
 * FAULT-TOLERANCE BOUNDARY
 * ============================================================================
 *
 * Failure and recovery syntax remains owned by:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * This grammar does not define:
 *
 *     retry;
 *     recover;
 *     failover;
 *     restart;
 *     quarantine;
 *     reroute;
 *     reschedule;
 *
 * Consistency properties may constrain post-recovery guarantees through
 * expressions.
 *
 * ============================================================================
 * TOPOLOGY / PLACEMENT BOUNDARY
 * ============================================================================
 *
 * Logical topology remains owned by:
 *
 *     grammar/distributed/topology.g4
 *
 * Placement remains owned by:
 *
 *     grammar/distributed/placement.g4
 *
 * Consistency MUST NOT select:
 *
 *     machine;
 *     node;
 *     rack;
 *     region;
 *     host;
 *     network path;
 *     physical device.
 *
 * Any such requirement belongs downstream to placement/resource/capability
 * analysis.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * This grammar does not select or require a particular transport.
 *
 * It does not define:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     HTTP;
 *     RPC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     vendor transport.
 *
 * Network realization is downstream.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Distributed consistency may apply to classical state associated with:
 *
 *     - quantum control;
 *     - measurement results;
 *     - classical feed-forward;
 *     - orchestration;
 *     - distributed metadata;
 *     - experiment coordination.
 *
 * It MUST NOT define or imply replication of quantum state itself.
 *
 * This grammar does not define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     QuantumGate;
 *     QuantumOperation;
 *     Circuit;
 *     QuantumTopology;
 *     Calibration;
 *     Pulse;
 *     QEC;
 *     ZQN.
 *
 * Quantum semantics remain owned by the quantum subsystem.
 *
 * When a consistency-aware program contains quantum computation, the semantic
 * pipeline remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization / routing / scheduling / QEC / ZQN
 *       |
 *       v
 *     HAL
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Consistency syntax does not define:
 *
 *     register widths;
 *     device counts;
 *     memory sizes;
 *     bus widths;
 *     hardware topology;
 *     clock counts;
 *     physical links.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Consistency declaration syntax itself introduces no runtime effect.
 *
 * Semantic analysis may associate consistency-related operations with effects
 * such as:
 *
 *     distributed
 *     network
 *     io
 *     synchronization
 *     mutation
 *
 * depending on the actual semantic operation.
 *
 * This grammar does not assign those effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * The grammar does not assert that a target has any capability.
 *
 * Semantic analysis may resolve requirements such as:
 *
 *     capability("distributed.consistency")
 *
 *     capability("distributed.strong-consistency")
 *
 *     capability("distributed.causal-ordering")
 *
 * without requiring those capability names to become parser keywords.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions are delegated to the canonical expression/resource
 * system.
 *
 * Examples:
 *
 *     memory >= required_memory
 *
 *     resources.available("distributed.communication")
 *
 *     bandwidth >= required_bandwidth
 *
 * The grammar does not define the units, machine widths, or capacity limits.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Consistency declarations may semantically participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     policy
 *     allow
 *     forbid
 *     prefer
 *     constrain
 *
 * Those constructs remain owned by their canonical grammar/semantic systems.
 *
 * This file only provides expression positions in which they may be
 * referenced when the surrounding language permits them.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Parsing must preserve source structure sufficient for downstream provenance.
 *
 * Semantic provenance may record:
 *
 *     source declaration;
 *     property;
 *     semantic interpretation;
 *     resolved consistency model;
 *     evidence;
 *     selected realization;
 *     transformation;
 *     diagnostic.
 *
 * The grammar does not generate provenance records itself.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must provide enough structure for the domain-neutral AST to
 * represent conceptually:
 *
 *     ConsistencyDeclaration
 *         name
 *         members
 *         source_span
 *
 *     ConsistencyInvocation
 *         qualified_name
 *         arguments
 *         source_span
 *
 *     ConsistencyProperty
 *         name
 *         value
 *         source_span
 *
 *     ConsistencySection
 *         name
 *         members
 *         source_span
 *
 *     ConsistencyList
 *         elements
 *         source_span
 *
 * The exact Rust type names remain owned by the AST subsystem.
 *
 * This grammar MUST NOT require:
 *
 *     RaftAst
 *     PaxosAst
 *     CrdtAst
 *     DatabaseConsistencyAst
 *     PhysicalReplicaAst
 *     HardwareConsistencyAst
 *
 * or other implementation-specific AST nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. resolve the declaration name;
 *     2. resolve qualified names;
 *     3. resolve property names where appropriate;
 *     4. type-check expressions;
 *     5. classify consistency semantics;
 *     6. validate consistency-model compatibility;
 *     7. detect contradictory properties;
 *     8. resolve resource requirements;
 *     9. resolve capability requirements;
 *     10. apply policy constraints;
 *     11. evaluate contract relationships;
 *     12. preserve provenance;
 *     13. produce source-linked diagnostics;
 *     14. determine whether the requested semantic guarantee can be realized.
 *
 * The parser performs NONE of these semantic decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO new IR.
 *
 * There must be no:
 *
 *     ConsistencyIR
 *     DistributedConsistencyIR
 *     ReplicaConsistencyIR
 *     DatabaseConsistencyIR
 *
 * merely for parser convenience.
 *
 * Consistency semantics lower through the repository's canonical semantic
 * representation and established IR architecture.
 *
 * Classical execution continues through the canonical classical pipeline.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * Distributed execution metadata remains compiler/runtime semantic metadata.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler pipeline is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     DistributedConsistency parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> contract checking
 *       +--> policy checking
 *       +--> provenance
 *       |
 *       v
 *     distributed semantic model
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
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
 * No stage after parsing may silently change the declared consistency meaning.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is downstream.
 *
 * The runtime may enforce, monitor, or report consistency guarantees according
 * to the compiled semantic plan.
 *
 * The grammar does not specify:
 *
 *     - runtime data structures;
 *     - synchronization algorithms;
 *     - lock implementations;
 *     - network protocols;
 *     - storage implementations;
 *     - retry mechanisms.
 *
 * Runtime enforcement must preserve the semantic contract established by the
 * compiler.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover structural errors only.
 *
 * Examples:
 *
 *     consistency
 *
 *     consistency state
 *
 *     consistency state {
 *
 *     consistency state {
 *         requirement:
 *     }
 *
 *     distributed::consistency::policy(
 *
 * Semantic diagnostics include:
 *
 *     unknown consistency model;
 *     contradictory consistency requirements;
 *     unsupported capability;
 *     unsatisfied resource requirement;
 *     incompatible policy;
 *     impossible guarantee;
 *     invalid consistency/transaction relationship.
 *
 * Such failures MUST NOT be represented as grammar-level capacity errors.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     - source tokens;
 *     - grammar version;
 *     - lexical configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware availability;
 *     - node count;
 *     - runtime state;
 *     - wall-clock time;
 *     - randomness;
 *     - network state;
 *     - deployment state;
 *     - filesystem state.
 *
 * The grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no runtime callbacks;
 *     - no I/O;
 *     - no network access;
 *     - no hardware access;
 *     - no randomness.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * Downstream AST/tooling must preserve:
 *
 *     - declaration order;
 *     - member order;
 *     - property order;
 *     - list order;
 *     - qualified-name segment order;
 *     - argument order;
 *     - nested section structure;
 *     - source spans.
 *
 * The parser must not:
 *
 *     - sort properties;
 *     - merge properties;
 *     - discard duplicates;
 *     - evaluate expressions;
 *     - normalize semantic values.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This revision intentionally establishes a deterministic production syntax:
 *
 *     consistency <identifier> { ... }
 *
 * rather than retaining the ambiguous:
 *
 *     <identifier> <identifier> { ... }
 *
 * form.
 *
 * The invocation mechanism remains open-world and compatible with the
 * repository's qualified-name expression architecture.
 *
 * The public distributed entry point remains:
 *
 *     distributedConsistencyConstruct
 *
 * Existing distributed.g4 composition therefore remains stable.
 *
 * If historical source used the ambiguous two-identifier declaration form,
 * compatibility/migration tooling must translate it to:
 *
 *     consistency <identifier> { ... }
 *
 * rather than weakening the production grammar.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Public entry point:
 *
 *     distributedConsistencyConstruct
 *
 * The distributed composition grammar consumes exactly that rule.
 *
 * This grammar does not become the repository root.
 *
 * The repository composition remains:
 *
 *     grammar/Zamani.g4
 *         |
 *         v
 *     grammar/antlr/ZamaniParser.g4
 *         |
 *         v
 *     distributed.g4
 *         |
 *         v
 *     consistency.g4
 *
 * ============================================================================
 * GRAMMAR IMPLEMENTATION
 * ============================================================================
 */

parser grammar DistributedConsistency;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
    ;


/*
 * ============================================================================
 * 1. PUBLIC DISTRIBUTED ENTRY POINT
 * ============================================================================
 *
 * Stable rule consumed by:
 *
 *     grammar/distributed/distributed.g4
 *
 * DO NOT rename without a versioned grammar compatibility change.
 */
distributedConsistencyConstruct
    : consistencyConstruct
    ;


/*
 * ============================================================================
 * 2. CONSISTENCY CONSTRUCT
 * ============================================================================
 *
 * Declaration and invocation are structurally distinguishable:
 *
 *     CONSISTENCY identifier body
 *
 * versus:
 *
 *     qualifiedName LPAREN ... RPAREN SEMICOLON
 */
consistencyConstruct
    : consistencyDeclaration
    | consistencyInvocation
    ;


/*
 * ============================================================================
 * 3. CONSISTENCY DECLARATION
 * ============================================================================
 *
 * Canonical:
 *
 *     consistency state {
 *         requirement: strong;
 *         ordering: causal;
 *     }
 *
 * The declaration marker is a reserved language-level token.
 */
consistencyDeclaration
    : CONSISTENCY
      identifier
      consistencyBody
    ;


/*
 * ============================================================================
 * 4. CONSISTENCY INVOCATION
 * ============================================================================
 *
 * Canonical examples:
 *
 *     consistency::policy(state, strong);
 *
 *     distributed::consistency::require(state, causal);
 *
 *     vendor::consistency::property(value);
 *
 * The qualified name is intentionally open-world.
 */
consistencyInvocation
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 5. CONSISTENCY BODY
 * ============================================================================
 */
consistencyBody
    : LBRACE
      consistencyMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. CONSISTENCY MEMBER
 * ============================================================================
 *
 * Members are deliberately divided by syntactic shape:
 *
 *     name : value ;
 *
 *     name { ... }
 *
 *     qualifiedName(...);
 *
 * This allows future semantic properties without requiring new grammar
 * alternatives for every property name.
 */
consistencyMember
    : consistencyProperty
    | consistencySection
    | consistencyExtensionStatement
    ;


/*
 * ============================================================================
 * 7. PROPERTY
 * ============================================================================
 */
consistencyProperty
    : identifier
      COLON
      consistencyValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. NESTED SECTION
 * ============================================================================
 */
consistencySection
    : identifier
      consistencyBody
    ;


/*
 * ============================================================================
 * 9. EXTENSION STATEMENT
 * ============================================================================
 *
 * Examples:
 *
 *     vendor::consistency::property(value);
 *
 *     future::consistency::guarantee(expression);
 *
 *     dialect::consistency::model(argument);
 */
consistencyExtensionStatement
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. VALUE
 * ============================================================================
 *
 * General expressions remain owned by Expressions.
 *
 * Nested objects and lists provide structural data without introducing a
 * second expression language.
 */
consistencyValue
    : expression
    | consistencyObject
    | consistencyList
    ;


/*
 * ============================================================================
 * 11. OBJECT
 * ============================================================================
 */
consistencyObject
    : LBRACE
      consistencyMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. LIST
 * ============================================================================
 *
 * There is no grammar-level list-size limit.
 */
consistencyList
    : LBRACKET
      optionalConsistencyListElements
      RBRACKET
    ;


/*
 * ============================================================================
 * 13. LIST ELEMENTS
 * ============================================================================
 *
 * Elements are separated explicitly by commas.
 *
 * Empty lists are valid.
 *
 * A trailing comma is intentionally not accepted here unless the canonical
 * repository collection grammar later establishes that convention.
 */
optionalConsistencyListElements
    : consistencyListElements?
    ;


consistencyListElements
    : consistencyListElement
      (COMMA consistencyListElement)*
    ;


consistencyListElement
    : expression
    | consistencyObject
    | consistencyList
    ;


/*
 * ============================================================================
 * 14. STABLE SEMANTIC ADAPTER RULES
 * ============================================================================
 *
 * These aliases provide stable integration points for semantic consumers
 * without creating a second expression language.
 */


/*
 * Consistency requirement value.
 */
consistencyRequirement
    : expression
    ;


/*
 * Consistency constraint value.
 */
consistencyConstraint
    : expression
    ;


/*
 * Consistency preference value.
 */
consistencyPreference
    : expression
    ;


/*
 * Consistency guarantee value.
 */
consistencyGuarantee
    : expression
    ;


/*
 * Consistency ordering value.
 */
consistencyOrdering
    : expression
    ;


/*
 * Consistency freshness value.
 */
consistencyFreshness
    : expression
    ;


/*
 * Consistency visibility value.
 */
consistencyVisibility
    : expression
    ;


/*
 * Consistency conflict value.
 */
consistencyConflict
    : expression
    ;


/*
 * Consistency convergence value.
 */
consistencyConvergence
    : expression
    ;


/*
 * Consistency synchronization value.
 */
consistencySynchronization
    : expression
    ;


/*
 * Consistency policy value.
 */
consistencyPolicy
    : expression
    ;


/*
 * Consistency capability requirement.
 */
consistencyCapability
    : expression
    ;


/*
 * Consistency resource requirement.
 */
consistencyResourceRequirement
    : expression
    ;


/*
 * Consistency evidence.
 */
consistencyEvidence
    : expression
    ;


/*
 * Consistency provenance reference.
 */
consistencyProvenance
    : expression
    ;


/*
 * ============================================================================
 * 15. SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parser/frontend must preserve:
 *
 *     - source spans;
 *     - declaration order;
 *     - member order;
 *     - property order;
 *     - section order;
 *     - list order;
 *     - qualified-name segment order;
 *     - argument order;
 *     - expression structure;
 *     - duplicate properties.
 *
 * The grammar itself performs no normalization.
 */


/*
 * ============================================================================
 * 16. NO SEMANTIC EXECUTION
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no embedded Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime callbacks;
 *     - no randomness.
 *
 * ============================================================================
 * 17. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_PARTITIONS
 *     MAX_PROPERTIES
 *     MAX_REQUIREMENTS
 *     MAX_CONSISTENCY_LEVELS
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Also forbidden are implicit fixed physical identities such as:
 *
 *     node_0
 *     replica_0
 *     machine_0
 *     gpu_0
 *     qpu_0
 *
 * Numeric literals appearing inside expressions remain ordinary program
 * semantics and MUST NOT be interpreted as language-wide limits.
 */


/*
 * ============================================================================
 * 18. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     - missing CONSISTENCY marker;
 *     - missing declaration name;
 *     - missing body;
 *     - malformed property;
 *     - malformed section;
 *     - malformed list;
 *     - malformed extension call;
 *     - malformed invocation.
 *
 * Semantic diagnostics:
 *
 *     - unknown consistency model;
 *     - contradictory guarantees;
 *     - unsupported capability;
 *     - unsatisfied resource requirement;
 *     - invalid policy;
 *     - invalid contract relationship;
 *     - unsupported target realization.
 *
 * Resource/capability failure MUST NOT be converted into a grammar-level
 * machine-capacity limit.
 */


/*
 * ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Consistency metadata associated with quantum-classical computation remains
 * classical semantic metadata unless the quantum subsystem explicitly defines
 * a valid relationship.
 *
 * The canonical quantum path remains:
 *
 *     Zamani source
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     quantum semantic analysis
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     QEC / resilience
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *
 * This grammar does not create a quantum IR.
 */


/*
 * ============================================================================
 * 20. REPLICATION INTEGRATION
 * ============================================================================
 *
 * Consistency may semantically apply to replicated state.
 *
 * The relationship is:
 *
 *     replication.g4
 *         -> logical replication intent
 *
 *     consistency.g4
 *         -> visibility/order/freshness/convergence intent
 *
 * Semantic analysis combines the two.
 *
 * Neither grammar owns the other's syntax.
 */


/*
 * ============================================================================
 * 21. TRANSACTION INTEGRATION
 * ============================================================================
 *
 * Transactions remain owned by:
 *
 *     grammar/distributed/transactions.g4
 *
 * Consistency properties may be referenced from transaction semantics through
 * expressions and semantic relations.
 *
 * This grammar does not define transaction operations.
 */


/*
 * ============================================================================
 * 22. FAULT-TOLERANCE INTEGRATION
 * ============================================================================
 *
 * Fault tolerance remains owned by:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * A consistency declaration may specify a guarantee that must remain true
 * after recovery, but the grammar does not implement recovery.
 */


/*
 * ============================================================================
 * 23. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource and capability requirements remain target-independent.
 *
 * Examples of semantic expressions:
 *
 *     capability("distributed.consistency")
 *
 *     capability("distributed.causal-ordering")
 *
 *     memory >= required_memory
 *
 *     bandwidth >= required_bandwidth
 *
 * The grammar preserves these expressions.
 *
 * Semantic analysis determines feasibility.
 */


/*
 * ============================================================================
 * 24. EFFECT INTEGRATION
 * ============================================================================
 *
 * A consistency declaration itself is declarative syntax.
 *
 * Actual operations associated with:
 *
 *     synchronization;
 *     communication;
 *     mutation;
 *     distributed execution;
 *
 * may acquire effects during semantic analysis.
 *
 * This grammar does not assign effects directly.
 */


/*
 * ============================================================================
 * 25. POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may constrain which consistency guarantees are permitted.
 *
 * Examples:
 *
 *     allow consistency model;
 *     forbid consistency model;
 *     prefer consistency model;
 *
 * remain semantic/policy constructs.
 *
 * This grammar does not create a competing policy language.
 */


/*
 * ============================================================================
 * 26. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Consistency declarations and properties must remain traceable to source
 * locations.
 *
 * Downstream provenance may record:
 *
 *     source declaration
 *     selected semantic model
 *     evidence
 *     capability resolution
 *     resource resolution
 *     policy decisions
 *     compiler transformations
 *     final realization
 *
 * The parser only preserves the source structure required to construct those
 * records.
 */


/*
 * ============================================================================
 * 27. CANONICAL IR INTEGRATION
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * There must be one canonical semantic representation for distributed
 * consistency intent rather than a grammar-specific IR.
 *
 * The lowering path is:
 *
 *     Consistency AST
 *         |
 *         v
 *     consistency semantic model
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical IR
 *         +--> quantum::ir where quantum computation is involved
 *         +--> HDL/hardware representation where applicable
 *
 * Optimization, placement, routing, scheduling, resilience, ZQN and HAL
 * remain downstream.
 */


/*
 * ============================================================================
 * 28. RUNTIME INTEGRATION
 * ============================================================================
 *
 * The runtime may enforce, monitor, observe, or report consistency semantics
 * according to the compiled execution plan.
 *
 * Runtime implementations MUST NOT reinterpret source syntax in a way that
 * changes its semantic contract.
 */


/*
 * ============================================================================
 * 29. COMPATIBILITY / MIGRATION
 * ============================================================================
 *
 * The public rule:
 *
 *     distributedConsistencyConstruct
 *
 * remains unchanged.
 *
 * The production declaration syntax is:
 *
 *     consistency <identifier> { ... }
 *
 * The old ambiguous contextual declaration:
 *
 *     <identifier> <identifier> { ... }
 *
 * is intentionally not retained in the production grammar.
 *
 * Historical source using that shape belongs in compatibility/migration
 * tooling rather than the canonical parser.
 *
 * Invocation remains open-world:
 *
 *     qualifiedName(...) ;
 *
 * This permits future consistency APIs without parser expansion.
 */


/*
 * ============================================================================
 * 30. VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * Production validation must include:
 *
 * STRUCTURAL:
 *
 *     - ANTLR generation succeeds;
 *     - imports resolve;
 *     - public rule resolves;
 *     - no unreachable rules;
 *     - no duplicate rules;
 *     - no grammar actions;
 *     - no semantic predicates.
 *
 * POSITIVE:
 *
 *     consistency state {
 *         requirement: strong;
 *     }
 *
 *     consistency state {
 *         requirement: strong;
 *         ordering: causal;
 *         freshness: bounded_staleness;
 *     }
 *
 *     consistency state {
 *         read {
 *             guarantee: read_your_writes;
 *         }
 *
 *         write {
 *             guarantee: monotonic_write;
 *         }
 *     }
 *
 *     consistency state {
 *         requirements {
 *             capability("distributed.consistency");
 *             memory >= required_memory;
 *         }
 *     }
 *
 *     consistency state {
 *         models: [
 *             strong,
 *             causal,
 *             eventual
 *         ];
 *     }
 *
 *     consistency::policy(state, strong);
 *
 *     distributed::consistency::require(
 *         state,
 *         causal
 *     );
 *
 *     vendor::consistency::property(value);
 *
 * NEGATIVE:
 *
 *     consistency
 *
 *     consistency {
 *     }
 *
 *     consistency state
 *
 *     consistency state {
 *         requirement:
 *     }
 *
 *     consistency state {
 *         read {
 *     }
 *
 *     distributed::consistency::require(
 *
 * BOUNDARY:
 *
 *     - empty consistency body;
 *     - one property;
 *     - many properties;
 *     - repeated properties;
 *     - nested sections;
 *     - nested lists;
 *     - deeply qualified names;
 *     - large expression trees;
 *     - large property collections;
 *     - large consistency declarations.
 *
 * SCALABILITY:
 *
 *     - no declaration-count limit;
 *     - no property-count limit;
 *     - no list-size limit;
 *     - no section-count limit;
 *     - no node-count limit;
 *     - no replica-count limit;
 *     - no partition-count limit;
 *     - no resource-count limit.
 *
 * DETERMINISM:
 *
 *     identical source + identical grammar/lexical configuration
 *     -> identical parse structure.
 *
 * COMPATIBILITY:
 *
 *     distributed.g4 continues to consume:
 *
 *         distributedConsistencyConstruct
 *
 *     replication.g4 remains the replication syntax owner;
 *
 *     transactions.g4 remains the transaction syntax owner;
 *
 *     fault-tolerance.g4 remains the fault-tolerance syntax owner;
 *
 *     expressions remain owned by Expressions;
 *
 *     names remain owned by Names.
 */


/*
 * ============================================================================
 * 31. HARD-CODING / PORTABILITY AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no fixed machine count;
 *     no fixed node count;
 *     no fixed replica count;
 *     no fixed partition count;
 *     no fixed memory capacity;
 *     no fixed network capacity;
 *     no fixed thread count;
 *     no fixed device count;
 *     no fixed hardware topology;
 *     no fixed transport;
 *     no fixed database;
 *     no fixed consistency algorithm.
 *
 * Therefore the syntax remains suitable for:
 *
 *     tiny
 *       ->
 *     embedded
 *       ->
 *     workstation
 *       ->
 *     multicore
 *       ->
 *     accelerator
 *       ->
 *     HPC
 *       ->
 *     cluster
 *       ->
 *     federated
 *       ->
 *     cloud
 *       ->
 *     heterogeneous
 *       ->
 *     future computational substrates
 *
 * subject only to actual implementation and resource availability.
 */


/*
 * ============================================================================
 * 32. SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * The consuming compiler/frontend must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust Edition 2021
 *
 * and must use safe Rust only.
 *
 * No unsafe block, unsafe trait, unsafe function, unsafe implementation, or
 * unsafe foreign-memory operation is required by this grammar.
 */


/*
 * ============================================================================
 * 33. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It owns consistency syntax only.
 *
 * [x] It has one stable public distributed entry point.
 *
 * [x] Declaration syntax is deterministic.
 *
 * [x] Invocation syntax is open-world.
 *
 * [x] Consistency model names remain extensible.
 *
 * [x] Properties remain open-world.
 *
 * [x] Nested sections are supported.
 *
 * [x] Structured values are supported.
 *
 * [x] Lists are supported without fixed capacity.
 *
 * [x] General expressions remain owned by Expressions.
 *
 * [x] Names remain owned by Names.
 *
 * [x] Replication remains owned by replication.g4.
 *
 * [x] Partitioning remains owned by partitioning.g4.
 *
 * [x] Transactions remain owned by transactions.g4.
 *
 * [x] Fault tolerance remains owned by fault-tolerance.g4.
 *
 * [x] Topology remains owned by topology.g4.
 *
 * [x] Placement remains owned by placement.g4.
 *
 * [x] Resources remain owned by resources/.
 *
 * [x] Capabilities remain owned by resources/ and semantic analysis.
 *
 * [x] Effects remain owned by effects/.
 *
 * [x] Policies remain owned by policies/.
 *
 * [x] Provenance remains owned by the provenance system.
 *
 * [x] No implementation algorithm is selected.
 *
 * [x] No physical hardware is selected.
 *
 * [x] No transport is selected.
 *
 * [x] No fixed capacity exists.
 *
 * [x] No parser action exists.
 *
 * [x] No semantic predicate exists.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Rust 1.97+ compatibility is defined.
 *
 * [x] Canonical quantum::ir integration is preserved.
 *
 * [x] POCO-REAF semantics are preserved.
 *
 * Repository-level verification must additionally confirm:
 *
 * [ ] `CONSISTENCY` exists in the canonical lexical vocabulary.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] `distributed.g4` imports this grammar successfully.
 *
 * [ ] `distributedConsistencyConstruct` remains reachable.
 *
 * [ ] AST mapping exists for all accepted constructs.
 *
 * [ ] semantic validation exists for accepted consistency properties.
 *
 * [ ] canonical semantic/IR lowering exists.
 *
 * [ ] positive tests pass.
 *
 * [ ] negative tests pass.
 *
 * [ ] boundary tests pass.
 *
 * [ ] scalability tests pass.
 *
 * [ ] determinism tests pass.
 *
 * [ ] compatibility tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The fundamental architectural invariant is:
 *
 *     CONSISTENCY SYNTAX
 *          |
 *          v
 *     PORTABLE SEMANTIC INTENT
 *          |
 *          v
 *     TYPE / EFFECT / CAPABILITY / RESOURCE ANALYSIS
 *          |
 *          v
 *     POLICY / CONTRACT / PROVENANCE ANALYSIS
 *          |
 *          v
 *     CANONICAL SEMANTIC REPRESENTATION
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          |
 *          v
 *     OPTIMIZATION
 *          |
 *          v
 *     PLACEMENT
 *          |
 *          v
 *     ROUTING
 *          |
 *          v
 *     SCHEDULING
 *          |
 *          v
 *     RESILIENCE
 *          |
 *          v
 *     ZQN / HAL / TARGET
 *
 * Source consistency intent MUST remain semantically stable while the
 * realization changes.
 *
 * Therefore the same source program can be considered for increasingly large
 * or different computational environments without changing the consistency
 * meaning merely because the target has more or different resources.
 *
 * ============================================================================
 */