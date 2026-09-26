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
 * Rust integration:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *     No embedded unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL STRUCTURAL SYNTAX for logical
 * distributed replication intent.
 *
 * Replication expresses logical multiplicity and relationships between
 * realizations of a computational entity.
 *
 * It does NOT implement:
 *
 *     - replica placement;
 *     - node discovery;
 *     - process scheduling;
 *     - networking;
 *     - transport;
 *     - consensus;
 *     - consistency algorithms;
 *     - fault tolerance;
 *     - recovery;
 *     - storage;
 *     - serialization;
 *     - hardware selection;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Replication
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       +--> name resolution
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> distributed semantic analysis
 *       +--> security analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical representation / IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> distributed execution metadata
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
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *
 * Name authority:
 *
 *     grammar/core/names.g4
 *
 * Expression authority:
 *
 *     grammar/expressions/
 *
 * Resource authority:
 *
 *     grammar/resources/
 *
 * Placement authority:
 *
 *     grammar/resources/placement.g4
 *     grammar/distributed/placement.g4
 *
 * Consistency authority:
 *
 *     grammar/distributed/consistency.g4
 *
 * Fault-tolerance authority:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * Quantum authority:
 *
 *     grammar/quantum/
 *
 * Canonical quantum IR:
 *
 *     quantum::ir
 *
 * This file must never become a second authority for any of those systems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Replication syntax describes PROGRAM INTENT.
 *
 * It must not encode physical deployment assumptions.
 *
 * Therefore this grammar contains no:
 *
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NETWORK_SIZE
 *
 * Nor does it encode:
 *
 *     machine_0
 *     node_0
 *     gpu_0
 *     qpu_0
 *     physical_qubit_0
 *     fixed_hostname
 *     fixed_ip
 *     fixed_port
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All potentially unbounded collections use ANTLR repetition:
 *
 *     *
 *     +
 *
 * There is no grammar-level ceiling on:
 *
 *     - replication declarations;
 *     - replicas;
 *     - replica groups;
 *     - properties;
 *     - dependencies;
 *     - policy expressions;
 *     - expression depth;
 *     - qualified-name depth;
 *     - logical replication factors;
 *     - extension properties.
 *
 * Practical limitations belong to:
 *
 *     - compiler resources;
 *     - parser resources;
 *     - operating-system resources;
 *     - target resources;
 *     - runtime resources;
 *     - deployment resources.
 *
 * Those limitations must never become source-language grammar limits.
 *
 * ============================================================================
 * IMPORTANT CONTEXTUAL-KEYWORD DECISION
 * ============================================================================
 *
 * The current Zamani lexical vocabulary does not define a dedicated
 * REPLICATION token.
 *
 * Therefore this grammar deliberately does NOT invent one.
 *
 * The declaration marker is represented structurally and classified by
 * semantic analysis.
 *
 * The higher-level parser/composition layer MUST invoke
 * `distributedReplicationDeclaration` only in a context where a replication
 * declaration is expected.
 *
 * This preserves the repository's open-world naming architecture and avoids
 * introducing an unnecessary global keyword solely for this domain.
 *
 * If `replication` is eventually promoted to a reserved keyword, that is a
 * separate lexical/compatibility change. This grammar should then replace
 * the contextual marker with the canonical lexer token.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - replication declaration framing;
 *     - logical replica declarations;
 *     - replica groups;
 *     - replication properties;
 *     - replication dependencies;
 *     - replication extension expressions;
 *     - replication member composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - types;
 *     - resource definitions;
 *     - resource discovery;
 *     - placement algorithms;
 *     - consistency algorithms;
 *     - fault-tolerance algorithms;
 *     - network protocols;
 *     - topology;
 *     - scheduling;
 *     - routing;
 *     - quantum operations;
 *     - quantum state representation;
 *     - HDL;
 *     - hardware realization;
 *     - IR definitions;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 */

parser grammar Replication;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable public rule consumed by the distributed grammar composition layer.
 *
 * Canonical structural forms:
 *
 *     replication workload;
 *
 *     replication workload {
 *         target: computation;
 *         factor: replicas;
 *     }
 *
 *     replication workload {
 *         policy: distributed::replication::policy;
 *         consistency: consistency_policy;
 *     }
 *
 * The semantic layer determines whether the first name is actually the
 * contextual replication declaration marker.
 */
distributedReplicationDeclaration
    : replicationDeclarationMarker
      identifier
      distributedReplicationBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. CONTEXTUAL DECLARATION MARKER
 * ============================================================================
 *
 * This remains a structural identifier because the current lexer does not
 * expose REPLICATION as a dedicated token.
 *
 * Semantic analysis MUST classify the marker.
 *
 * The parser must not create a second keyword authority.
 */
replicationDeclarationMarker
    : identifier
    ;


/*
 * ============================================================================
 * 3. REPLICATION BODY
 * ============================================================================
 */

distributedReplicationBody
    : LBRACE
      distributedReplicationMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. REPLICATION MEMBER
 * ============================================================================
 *
 * A member has one structural responsibility.
 *
 * Property names remain open-world identifiers.
 */
distributedReplicationMember
    : distributedReplicationProperty
    | distributedReplicationReplica
    | distributedReplicationGroup
    | distributedReplicationDependency
    | distributedReplicationExtensionStatement
    ;


/*
 * ============================================================================
 * 5. PROPERTY
 * ============================================================================
 *
 * Generic property form:
 *
 *     target: workload;
 *     factor: replica_count;
 *     policy: replication_policy;
 *     consistency: consistency_policy;
 *     placement: placement_requirement;
 *     availability: availability_requirement;
 *     durability: durability_requirement;
 *     synchronization: synchronization_policy;
 *     requirement: resource_requirement;
 *     constraint: resource_constraint;
 *     preference: resource_preference;
 *     hint: resource_hint;
 *
 * The property name is intentionally not enumerated.
 *
 * This permits future replication semantics without repeatedly modifying the
 * parser whenever a new semantic property is introduced.
 *
 * Property meaning belongs to semantic analysis.
 */
distributedReplicationProperty
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. LOGICAL REPLICA
 * ============================================================================
 *
 * A replica name is a LOGICAL NAME.
 *
 * It is never a physical machine/device/node identifier by implication.
 *
 * Examples:
 *
 *     replica primary;
 *
 *     replica secondary;
 *
 *     replica worker_a {
 *         preference: locality;
 *     }
 *
 * The word `replica` is contextually classified by semantic analysis.
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
    | distributedReplicationDependency
    | distributedReplicationExtensionStatement
    ;


/*
 * ============================================================================
 * 7. REPLICA GROUP
 * ============================================================================
 *
 * A group describes a logical collection of replicas.
 *
 * It does not identify a physical cluster, machine group, availability zone,
 * rack, host, or network segment.
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
    | distributedReplicationExtensionStatement
    ;


/*
 * ============================================================================
 * 8. DEPENDENCY
 * ============================================================================
 *
 * Example:
 *
 *     depends_on: base_replication;
 *
 * Dependency meaning remains semantic.
 *
 * It does not mean:
 *
 *     network dependency;
 *     machine dependency;
 *     hardware dependency;
 *     process dependency.
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


/*
 * ============================================================================
 * 9. OPEN-WORLD EXTENSION
 * ============================================================================
 *
 * This provides a structured extension point without creating a closed list
 * of replication algorithms or policies.
 *
 * Examples:
 *
 *     distributed::replication::policy(argument);
 *
 *     vendor::replication::extension(argument);
 *
 *     future::replication::mechanism(value);
 *
 * The semantic layer determines whether the referenced extension exists.
 */
distributedReplicationExtensionStatement
    : distributedReplicationExtension
      SEMICOLON
    ;


distributedReplicationExtension
    : qualifiedName
      distributedReplicationCallArguments?
    ;


distributedReplicationCallArguments
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 10. STABLE SEMANTIC PROPERTY SHAPES
 * ============================================================================
 *
 * These rules provide explicit integration points for semantic consumers.
 *
 * They are transparent aliases over the canonical expression grammar.
 *
 * They do not create separate expression languages.
 *
 * They do not create separate IR types.
 */


/*
 * Logical replication target.
 */
distributedReplicationTarget
    : expression
    ;


/*
 * Desired replication quantity.
 *
 * This is an expression rather than a grammar-level integer.
 *
 * Valid examples include:
 *
 *     3
 *     desired_replicas
 *     policy.replica_count
 *     resources.replication_capacity
 *     compute_factor()
 *
 * The grammar imposes no upper bound.
 */
distributedReplicationFactor
    : expression
    ;


/*
 * Replication policy.
 */
distributedReplicationPolicy
    : expression
    ;


/*
 * Consistency requirement.
 */
distributedReplicationConsistency
    : expression
    ;


/*
 * Placement intent.
 */
distributedReplicationPlacement
    : expression
    ;


/*
 * Availability requirement.
 */
distributedReplicationAvailability
    : expression
    ;


/*
 * Durability requirement.
 */
distributedReplicationDurability
    : expression
    ;


/*
 * Synchronization requirement.
 */
distributedReplicationSynchronization
    : expression
    ;


/*
 * Generic resource requirement.
 */
distributedReplicationRequirement
    : expression
    ;


/*
 * Generic resource constraint.
 */
distributedReplicationConstraint
    : expression
    ;


/*
 * Generic resource preference.
 */
distributedReplicationPreference
    : expression
    ;


/*
 * Generic resource hint.
 */
distributedReplicationHint
    : expression
    ;


/*
 * ============================================================================
 * 11. EXPRESSION ARGUMENTS
 * ============================================================================
 *
 * Canonical expression infrastructure is reused.
 *
 * No fixed argument count exists.
 */
distributedReplicationArguments
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 12. EXPLICIT SEMANTIC PROPERTY ADAPTERS
 * ============================================================================
 *
 * These are parser-composition adapters.
 *
 * They do not introduce new syntax.
 *
 * A semantic frontend may use these names when mapping known property
 * categories to the domain-neutral AST.
 */


/*
 *     target: expression;
 */
distributedReplicationTargetProperty
    : distributedReplicationTarget
    ;


/*
 *     factor: expression;
 */
distributedReplicationFactorProperty
    : distributedReplicationFactor
    ;


/*
 *     policy: expression;
 */
distributedReplicationPolicyProperty
    : distributedReplicationPolicy
    ;


/*
 *     consistency: expression;
 */
distributedReplicationConsistencyProperty
    : distributedReplicationConsistency
    ;


/*
 *     placement: expression;
 */
distributedReplicationPlacementProperty
    : distributedReplicationPlacement
    ;


/*
 *     availability: expression;
 */
distributedReplicationAvailabilityProperty
    : distributedReplicationAvailability
    ;


/*
 *     durability: expression;
 */
distributedReplicationDurabilityProperty
    : distributedReplicationDurability
    ;


/*
 *     synchronization: expression;
 */
distributedReplicationSynchronizationProperty
    : distributedReplicationSynchronization
    ;


/*
 * ============================================================================
 * 13. SEMANTIC OWNERSHIP CONTRACT
 * ============================================================================
 *
 * The parser preserves source structure.
 *
 * Semantic analysis is responsible for interpreting property identifiers.
 *
 * For example:
 *
 *     replication workload {
 *         factor: desired_replicas;
 *         consistency: consistency_policy;
 *         placement: placement_policy;
 *     }
 *
 * becomes conceptually:
 *
 *     ReplicationDeclaration {
 *         name: workload,
 *         properties: [
 *             Property(factor, desired_replicas),
 *             Property(consistency, consistency_policy),
 *             Property(placement, placement_policy)
 *         ]
 *     }
 *
 * The grammar MUST NOT directly produce:
 *
 *     PhysicalNodeId
 *     PhysicalDeviceId
 *     TransportId
 *     QubitId
 *     CpuId
 *     GpuId
 *     FpgaId
 *
 * as replication semantics.
 *
 * ============================================================================
 * 14. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts must remain semantically distinct even though their syntax
 * uses the same expression mechanism.
 *
 * REQUIREMENT:
 *
 *     must be satisfiable.
 *
 * CONSTRAINT:
 *
 *     limits legal realization.
 *
 * PREFERENCE:
 *
 *     expresses a desired realization but is not necessarily mandatory.
 *
 * HINT:
 *
 *     supplies implementation guidance without defining program correctness.
 *
 * This distinction belongs to semantic analysis and resource planning.
 *
 * ============================================================================
 * 15. REPLICATION FACTOR CONTRACT
 * ============================================================================
 *
 * A factor is a PROGRAM VALUE.
 *
 * Examples:
 *
 *     factor: 1;
 *     factor: desired_replicas;
 *     factor: resources.replication_capacity;
 *     factor: compute_factor();
 *
 * The grammar does not impose:
 *
 *     factor <= 2
 *     factor <= 8
 *     factor <= 1024
 *
 * or any other machine-dependent limit.
 *
 * Semantic analysis determines:
 *
 *     - type;
 *     - validity;
 *     - units;
 *     - feasibility;
 *     - resource availability;
 *     - policy compliance.
 *
 * ============================================================================
 * 16. REPLICATION IS NOT PLACEMENT
 * ============================================================================
 *
 * This:
 *
 *     replica primary;
 *
 * means a logical replica named `primary`.
 *
 * It does NOT mean:
 *
 *     machine 0
 *     node 0
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * 17. REPLICATION IS NOT CONSISTENCY
 * ============================================================================
 *
 * A property such as:
 *
 *     consistency: policy;
 *
 * records semantic intent.
 *
 * It does not implement:
 *
 *     linearizability;
 *     sequential consistency;
 *     causal consistency;
 *     eventual consistency;
 *     quorum algorithms;
 *     consensus;
 *     CRDT algorithms;
 *     transactional protocols.
 *
 * Those belong to their respective semantic/runtime systems.
 *
 * ============================================================================
 * 18. REPLICATION IS NOT FAULT TOLERANCE
 * ============================================================================
 *
 * Replication may contribute to resilience, but this grammar does not
 * implement:
 *
 *     retry;
 *     restart;
 *     failover;
 *     repair;
 *     leader election;
 *     checkpoint restoration;
 *     failure detection;
 *     recovery.
 *
 * Those concerns remain owned by fault-tolerance/resilience/runtime systems.
 *
 * ============================================================================
 * 19. QUANTUM SAFETY
 * ============================================================================
 *
 * Replication syntax may surround quantum-related computation.
 *
 * Example:
 *
 *     replication quantum_workload {
 *         target: quantum_program;
 *         factor: executions;
 *     }
 *
 * This does NOT imply copying an arbitrary unknown quantum state.
 *
 * Semantic analysis must distinguish:
 *
 *     - repeated independent execution;
 *     - replicated classical control;
 *     - replicated logical computation;
 *     - measurement-derived information;
 *     - QEC-supported logical state handling;
 *     - invalid quantum-state copying.
 *
 * No quantum-specific IR is created here.
 *
 * All actual quantum semantics continue through:
 *
 *     quantum::ir
 *
 * ============================================================================
 * 20. HDL / HARDWARE SAFETY
 * ============================================================================
 *
 * Replication syntax may describe replicated hardware/software intent.
 *
 * It does not define:
 *
 *     wires;
 *     physical ports;
 *     clocks;
 *     FPGA resources;
 *     ASIC cells;
 *     physical addresses;
 *     device identifiers.
 *
 * HDL and hardware subsystems own those concerns.
 *
 * ============================================================================
 * 21. NETWORKING SAFETY
 * ============================================================================
 *
 * Replication does not select:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     HTTP;
 *     RPC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     vendor-specific transports.
 *
 * Network realization belongs to networking, placement, scheduling, and
 * runtime layers.
 *
 * ============================================================================
 * 22. MEMORY SAFETY
 * ============================================================================
 *
 * This grammar does not redefine:
 *
 *     allocation;
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     address spaces;
 *     distributed memory;
 *     persistence.
 *
 * Memory semantics remain owned by the memory subsystem.
 *
 * ============================================================================
 * 23. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no network access;
 *     - no filesystem access;
 *     - no hardware queries;
 *     - no runtime callbacks;
 *     - no randomness.
 *
 * Identical token streams therefore produce structurally equivalent parser
 * results.
 *
 * ============================================================================
 * 24. SOURCE PRESERVATION
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - declaration source span;
 *     - declaration name;
 *     - property ordering;
 *     - property source spans;
 *     - replica ordering;
 *     - replica names;
 *     - group ordering;
 *     - dependency ordering;
 *     - extension ordering;
 *     - nested structure.
 *
 * Semantic canonicalization occurs after parsing.
 *
 * The parser must not reorder source constructs.
 *
 * ============================================================================
 * 25. DUPLICATE PROPERTY POLICY
 * ============================================================================
 *
 * This grammar intentionally permits repeated property names.
 *
 * Example:
 *
 *     replication workload {
 *         preference: low_latency;
 *         preference: low_energy;
 *     }
 *
 * The parser preserves both occurrences.
 *
 * Semantic analysis decides whether the properties are:
 *
 *     - cumulative;
 *     - overriding;
 *     - conflicting;
 *     - invalid.
 *
 * The parser must not silently discard either occurrence.
 *
 * ============================================================================
 * 26. OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * Qualified extension names are accepted structurally:
 *
 *     vendor::replication::policy(...)
 *
 *     future::replication::mechanism(...)
 *
 *     domain::custom::replication(...)
 *
 * The parser does not need modification merely because a new semantic
 * extension is introduced.
 *
 * Semantic registration determines:
 *
 *     - existence;
 *     - version;
 *     - compatibility;
 *     - capability requirements;
 *     - implementation support.
 *
 * ============================================================================
 * 27. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must provide a domain-neutral representation containing
 * at least:
 *
 *     ReplicationDeclaration
 *         name
 *         members
 *         source_span
 *
 *     ReplicationProperty
 *         name
 *         value
 *         source_span
 *
 *     ReplicationReplica
 *         name
 *         members
 *         source_span
 *
 *     ReplicationGroup
 *         name
 *         members
 *         source_span
 *
 *     ReplicationDependency
 *         value
 *         source_span
 *
 *     ReplicationExtension
 *         qualified_name
 *         arguments
 *         source_span
 *
 * The grammar itself does not define Rust AST structures.
 *
 * ============================================================================
 * 28. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *     - contextual declaration classification;
 *     - logical target validity;
 *     - factor type and validity;
 *     - policy existence;
 *     - property compatibility;
 *     - dependency validity;
 *     - resource requirements;
 *     - placement compatibility;
 *     - consistency compatibility;
 *     - durability requirements;
 *     - availability requirements;
 *     - synchronization requirements;
 *     - quantum semantic legality;
 *     - extension registration;
 *     - capability requirements.
 *
 * ============================================================================
 * 29. RESOURCE CONTRACT
 * ============================================================================
 *
 * Physical feasibility belongs to the resource system.
 *
 * For example:
 *
 *     replication workload {
 *         factor: desired_replicas;
 *         requirement: resources.replication_capacity;
 *     }
 *
 * means that the semantic/resource systems must determine whether the desired
 * realization is feasible.
 *
 * The grammar does not query the machine.
 *
 * The grammar does not know how many nodes exist.
 *
 * The grammar does not change according to hardware availability.
 *
 * ============================================================================
 * 30. PLACEMENT CONTRACT
 * ============================================================================
 *
 * Placement remains a downstream concern.
 *
 * The dependency direction is:
 *
 *     replication intent
 *          |
 *          v
 *     semantic representation
 *          |
 *          v
 *     resource analysis
 *          |
 *          v
 *     placement
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     deployment/runtime
 *
 * This file must never perform physical placement.
 *
 * ============================================================================
 * 31. CONSISTENCY CONTRACT
 * ============================================================================
 *
 * Replication may reference consistency intent through a property:
 *
 *     consistency: policy;
 *
 * The consistency subsystem determines whether the policy is:
 *
 *     supported;
 *     unsupported;
 *     conditional;
 *     conflicting;
 *     unknown.
 *
 * The replication grammar does not enumerate or implement consistency
 * algorithms.
 *
 * ============================================================================
 * 32. RESILIENCE CONTRACT
 * ============================================================================
 *
 * Resilience/fault-tolerance systems may consume replication metadata.
 *
 * Possible downstream outcomes include:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * Those outcomes are not parser decisions.
 *
 * ============================================================================
 * 33. COMPILER CONTRACT
 * ============================================================================
 *
 * The frontend lowers replication syntax into the canonical semantic model.
 *
 * Conceptually:
 *
 *     parser context
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic replication model
 *          |
 *          +--> resource requirements
 *          +--> capability requirements
 *          +--> placement intent
 *          +--> consistency intent
 *          +--> resilience metadata
 *          |
 *          v
 *     compiler planning
 *
 * The replication grammar must never introduce a second distributed IR.
 *
 * ============================================================================
 * 34. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If a replicated computation contains quantum semantics:
 *
 *     replication
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * There must not be:
 *
 *     replication::QuantumIR
 *
 * or:
 *
 *     distributed::QuantumIR
 *
 * competing with `quantum::ir`.
 *
 * ============================================================================
 * 35. RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded target-language implementation.
 *
 * Generated parser integration must therefore remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * Hand-written Zamani compiler code must use safe Rust.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * 36. VALIDATION CONTRACT
 * ============================================================================
 *
 * Grammar validation must verify:
 *
 *     - grammar name is Replication;
 *     - token vocabulary is ZamaniLexer;
 *     - Names is imported;
 *     - Expressions is imported;
 *     - no identifier grammar is duplicated;
 *     - no expression grammar is duplicated;
 *     - no fixed replica count exists;
 *     - no physical resource capacity exists;
 *     - no physical device identifier is encoded;
 *     - no transport protocol is selected;
 *     - no placement algorithm is implemented;
 *     - no semantic predicate exists;
 *     - no embedded action exists;
 *     - repetition remains unbounded by language constants;
 *     - public entry point remains stable;
 *     - parser remains deterministic.
 *
 * ============================================================================
 * 37. TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax:
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
 *         policy: distributed::replication::policy;
 *     }
 *
 *     replication workload {
 *         consistency: consistency_policy;
 *         availability: availability_policy;
 *         durability: durability_policy;
 *     }
 *
 *     replication workload {
 *         replica primary;
 *         replica secondary;
 *     }
 *
 *     replication workload {
 *         group workers {
 *             replica worker_a;
 *             replica worker_b;
 *         }
 *     }
 *
 *     replication workload {
 *         depends_on: base_workload;
 *     }
 *
 *     replication workload {
 *         vendor::replication::policy(argument);
 *     }
 *
 * Boundary tests:
 *
 *     - empty replication body;
 *     - one property;
 *     - many properties;
 *     - one replica;
 *     - many replicas;
 *     - many groups;
 *     - nested groups;
 *     - deeply qualified extension names;
 *     - large expression lists;
 *     - symbolic replication factors.
 *
 * Negative syntax tests:
 *
 *     - missing declaration name;
 *     - malformed property;
 *     - missing colon;
 *     - missing semicolon;
 *     - malformed replica;
 *     - malformed group;
 *     - malformed dependency;
 *     - malformed extension arguments;
 *     - unbalanced braces;
 *     - unbalanced parentheses.
 *
 * Scalability tests must verify that no artificial machine-size limit exists.
 *
 * Cross-domain tests must include:
 *
 *     classical;
 *     quantum;
 *     HDL;
 *     hardware;
 *     AI;
 *     networking;
 *     memory;
 *     distributed execution.
 *
 * ============================================================================
 * 38. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Also forbidden:
 *
 *     replica_0
 *     node_0
 *     gpu_0
 *     qpu_0
 *     physical_qubit_0
 *
 * as grammar-defined physical identities.
 *
 * Numeric expressions are permitted because they are program values:
 *
 *     factor: 8;
 *
 * is valid source intent.
 *
 * It does NOT establish:
 *
 *     MAX_REPLICAS = 8
 *
 * ============================================================================
 * 39. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable public entry point:
 *
 *     distributedReplicationDeclaration
 *
 * Stable semantic helper names:
 *
 *     distributedReplicationTarget
 *     distributedReplicationFactor
 *     distributedReplicationPolicy
 *     distributedReplicationConsistency
 *     distributedReplicationPlacement
 *     distributedReplicationAvailability
 *     distributedReplicationDurability
 *     distributedReplicationSynchronization
 *     distributedReplicationRequirement
 *     distributedReplicationConstraint
 *     distributedReplicationPreference
 *     distributedReplicationHint
 *
 * Internal implementation rules may evolve provided the public entry points
 * and AST/semantic contracts remain compatible.
 *
 * ============================================================================
 * 40. INTEGRATION CONTRACT
 * ============================================================================
 *
 * `grammar/distributed/distributed.g4` must import:
 *
 *     Replication
 *
 * when the specialized replication declaration is exposed through the
 * distributed composition layer.
 *
 * IMPORTANT:
 *
 * The current Distributed grammar already contains:
 *
 *     distributedReplication
 *
 * which owns generic call-form syntax:
 *
 *     qualifiedName(expressionList);
 *
 * That rule should remain the generic operation form.
 *
 * It must not be copied into this file.
 *
 * The specialized declaration form owned here is:
 *
 *     distributedReplicationDeclaration
 *
 * The aggregate must route that form to this rule only in a syntactic context
 * where the replication declaration is expected.
 *
 * Because `replication` is currently contextual rather than a lexer token,
 * the aggregate must NOT create an ambiguous alternative that competes
 * blindly with `distributedNamedDeclaration`.
 *
 * If the root composition later promotes `replication` to a reserved token,
 * the integration can become a deterministic token-based dispatch without
 * changing the semantic ownership of this file.
 *
 * ============================================================================
 * 41. EXISTING-FILE INTEGRATION
 * ============================================================================
 *
 * Do NOT create another replication grammar such as:
 *
 *     distributed/replicas.g4
 *     distributed/replication-policy.g4
 *     distributed/distributed-replication.g4
 *
 * unless a separate semantic ownership boundary is demonstrated.
 *
 * Keep this filename:
 *
 *     grammar/distributed/replication.g4
 *
 * Existing:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * remains responsible for fault-tolerance intent.
 *
 * Existing:
 *
 *     grammar/distributed/consistency.g4
 *
 * remains responsible for consistency intent.
 *
 * Existing:
 *
 *     grammar/distributed/placement.g4
 *
 * remains responsible for distributed placement association.
 *
 * Existing:
 *
 *     grammar/resources/placement.g4
 *
 * remains responsible for generic placement intent.
 *
 * Existing:
 *
 *     grammar/distributed/partitioning.g4
 *
 * remains responsible for partitioning.
 *
 * Existing:
 *
 *     grammar/distributed/messaging.g4
 *
 * remains responsible for its existing compatibility surface until it is
 * reconciled with the canonical networking message grammar.
 *
 * This replication grammar must not duplicate any of those responsibilities.
 *
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] One canonical Replication grammar identity exists.
 *
 * [x] ZamaniLexer is the token vocabulary.
 *
 * [x] Names is the canonical name dependency.
 *
 * [x] Expressions is the canonical expression dependency.
 *
 * [x] No identifier syntax is duplicated.
 *
 * [x] No expression syntax is duplicated.
 *
 * [x] No fixed replica limit exists.
 *
 * [x] No fixed node limit exists.
 *
 * [x] No fixed machine capacity exists.
 *
 * [x] No physical device identity is encoded.
 *
 * [x] No transport is selected.
 *
 * [x] No placement algorithm is implemented.
 *
 * [x] No consistency algorithm is implemented.
 *
 * [x] No fault-tolerance algorithm is implemented.
 *
 * [x] No quantum IR is duplicated.
 *
 * [x] The canonical quantum::ir boundary is preserved.
 *
 * [x] Resource feasibility remains downstream.
 *
 * [x] Source ordering can be preserved.
 *
 * [x] Source spans can be preserved.
 *
 * [x] Duplicate properties are preserved for semantic analysis.
 *
 * [x] Future qualified extensions remain syntactically representable.
 *
 * [x] No semantic predicates are required.
 *
 * [x] No embedded Rust actions are required.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * [x] Safe Rust integration is sufficient.
 *
 * [x] Scalability is expressed through unbounded grammar repetition.
 *
 * [x] Positive tests are defined.
 *
 * [x] Negative tests are defined.
 *
 * [x] Boundary tests are defined.
 *
 * [x] Cross-domain tests are defined.
 *
 * [x] Hard-coding audit is defined.
 *
 * [x] Integration ownership is explicitly defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What logical replication intent does the program express?"
 *
 * It does NOT answer:
 *
 *     "Which machine executes the replica?"
 *     "Which node hosts the replica?"
 *     "Which CPU/GPU/QPU executes it?"
 *     "Which network transports it?"
 *     "Which topology is selected?"
 *     "Which scheduler runs it?"
 *     "Which consistency algorithm implements it?"
 *     "Which fault-tolerance algorithm implements it?"
 *     "Which QEC strategy implements it?"
 *     "Which physical hardware realizes it?"
 *
 * Those decisions remain downstream.
 *
 * Therefore replication syntax remains portable across:
 *
 *     embedded systems
 *     single machines
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     quantum processors
 *     accelerators
 *     clusters
 *     HPC systems
 *     clouds
 *     heterogeneous systems
 *     future computational architectures
 *
 * This is the required grammar-level foundation for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */