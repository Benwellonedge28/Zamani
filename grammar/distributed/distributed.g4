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
 *     Production distributed-domain composition root
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97+
 *     Rust Edition 2021
 *
 * Safety:
 *     - Grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe implementation requirement.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No mutable parser-global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE DISTRIBUTED-DOMAIN COMPOSITION ROOT.
 *
 * It does not implement individual distributed features.
 *
 * It composes the independently-owned distributed grammars into stable
 * declaration, statement, expression, and generic-construct boundaries that
 * can be consumed by the canonical Zamani parser.
 *
 * The specialized grammars remain authoritative for their own syntax.
 *
 * This file therefore owns:
 *
 *     - distributed grammar composition;
 *     - public distributed declaration dispatch;
 *     - public distributed statement dispatch;
 *     - public distributed expression dispatch;
 *     - generic distributed construct dispatch;
 *     - distributed list/block composition;
 *     - composition-level ownership boundaries.
 *
 * It does NOT own:
 *
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - type syntax;
 *     - actor syntax;
 *     - task syntax;
 *     - message-schema syntax;
 *     - networking syntax;
 *     - resource syntax;
 *     - effect syntax;
 *     - policy semantics;
 *     - contract semantics;
 *     - placement realization;
 *     - scheduling;
 *     - routing;
 *     - hardware discovery;
 *     - target selection;
 *     - runtime execution;
 *     - IR construction.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Distributed
 *          |
 *          +--> declarations
 *          +--> statements
 *          +--> expressions
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
 *          +--> distributed semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware semantics
 *          +--> distributed execution metadata
 *          |
 *          v
 *     optimization
 *          |
 *          +--> partitioning
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> recovery
 *          |
 *          v
 *     ZQN / HAL / target realization
 *          |
 *          v
 *     runtime
 *
 * The grammar never performs any of the downstream operations itself.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Distributed source describes logical computation and execution intent.
 *
 * It MUST remain valid independently of the eventual realization on:
 *
 *     - one execution resource;
 *     - many CPU cores;
 *     - many processes;
 *     - many machines;
 *     - embedded resources;
 *     - edge resources;
 *     - accelerators;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - quantum simulators;
 *     - HPC systems;
 *     - clusters;
 *     - clouds;
 *     - federated systems;
 *     - heterogeneous systems;
 *     - distributed quantum systems;
 *     - future computational substrates.
 *
 * The source program describes meaning and intent.
 *
 * The compiler/runtime determine realization according to:
 *
 *     - semantic requirements;
 *     - capabilities;
 *     - resources;
 *     - constraints;
 *     - preferences;
 *     - policies;
 *     - topology;
 *     - placement;
 *     - scheduling;
 *     - resilience;
 *     - target availability.
 *
 * A change in physical realization MUST NOT require a source rewrite merely
 * because the deployment has a different scale.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO language-level physical capacity limits.
 *
 * It MUST NOT define or imply:
 *
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_SERVICES
 *     MAX_ACTORS
 *     MAX_TASKS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_REPLICAS
 *     MAX_SHARDS
 *     MAX_PARTITIONS
 *     MAX_REGIONS
 *     MAX_DEVICES
 *     MAX_NETWORKS
 *     MAX_CLUSTERS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_BANDWIDTH
 *
 * It also MUST NOT encode:
 *
 *     - fixed machine counts;
 *     - fixed node IDs;
 *     - fixed worker IDs;
 *     - fixed device IDs;
 *     - physical memory addresses;
 *     - physical network addresses;
 *     - fixed ports;
 *     - provider inventories;
 *     - fixed topology dimensions;
 *     - fixed replica counts;
 *     - fixed shard counts.
 *
 * ANTLR repetition operators describe syntactic cardinality without imposing
 * a semantic maximum.
 *
 * Actual finite limits are implementation, configuration, deployment, or
 * physical-resource constraints and are handled downstream.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Distributed computing is intentionally extensible.
 *
 * The grammar MUST NOT require a new lexer keyword merely because a new
 * distributed semantic abstraction is introduced.
 *
 * Names such as:
 *
 *     federation
 *     region
 *     shard
 *     replica
 *     edge_region
 *     quantum_network
 *     photonic_fabric
 *     neuromorphic_region
 *     future_protocol
 *     future_topology
 *     future_execution_model
 *
 * remain semantic names unless a future source construct genuinely requires
 * reserved syntax.
 *
 * Semantic analysis determines whether a named abstraction is:
 *
 *     - defined;
 *     - supported;
 *     - experimental;
 *     - deprecated;
 *     - vendor-defined;
 *     - dialect-defined;
 *     - extension-defined;
 *     - unknown.
 *
 * Syntax acceptance MUST NOT be interpreted as implementation support.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar defines no lexer rules.
 *
 * All tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The distributed subsystem MUST NOT create:
 *
 *     - duplicate identifier tokens;
 *     - distributed-specific keyword tokens without specification;
 *     - duplicate punctuation;
 *     - duplicate literals.
 *
 * Contextual distributed words may remain ordinary identifiers where the
 * owning grammar permits them.
 *
 * ============================================================================
 * NAME AUTHORITY
 * ============================================================================
 *
 * Name syntax is owned by:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * This file does not redefine:
 *
 *     identifier
 *     qualifiedName
 *     nameSegment
 *     nameList
 *     qualifiedNameList
 *
 * Specialized grammars consume those canonical rules through their own
 * imports.
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Distributed expression adapters may expose a specialized distributed
 * expression boundary, but they MUST delegate actual expression syntax to
 * the canonical expression system.
 *
 * This file does not redefine:
 *
 *     expression
 *     expressionList
 *     operator precedence
 *     arithmetic
 *     logical expressions
 *     calls
 *     indexing
 *     member access
 *     literals.
 *
 * ============================================================================
 * TYPE AUTHORITY
 * ============================================================================
 *
 * Type syntax remains owned by:
 *
 *     grammar/types/types.g4
 *
 * Distributed constructs consume types through their specialized grammars.
 *
 * This file does not define:
 *
 *     typeExpression
 *     generic types
 *     reference types
 *     ownership types
 *     quantum types
 *     tensor types
 *     resource types.
 *
 * ============================================================================
 * SPECIALIZED GRAMMAR OWNERSHIP
 * ============================================================================
 *
 * Public distributed declaration ownership:
 *
 *     Nodes
 *         -> nodeDeclaration
 *
 *     DistributedServices
 *         -> distributedServiceDeclaration
 *
 *     Processes
 *         -> distributedProcessDeclaration
 *
 *     DistributedActors
 *         -> distributedActorConstruct
 *         -> distributedActorDeclaration
 *
 *     DistributedChannels
 *         -> distributedChannelDeclaration
 *
 *     Communication
 *         -> distributedCommunication
 *
 *     DistributedMessages
 *         -> distributedMessageDeclaration
 *         -> distributedMessageValue
 *
 *     DistributedTasks
 *         -> distributedTaskExpression
 *         -> distributedTaskStatement
 *
 *     Collective
 *         -> collectiveDeclaration
 *         -> collectiveInvocation
 *
 *     Replication
 *         -> distributedReplicationDeclaration
 *
 *     DistributedConsistency
 *         -> distributedConsistencyConstruct
 *
 *     DistributedPlacement
 *         -> distributedPlacementDeclaration
 *
 *     DistributedPartitioning
 *         -> distributedPartitioningDeclaration
 *
 *     DistributedTopology
 *         -> distributedTopologyConstruct
 *
 *     RemoteExecution
 *         -> distributedRemoteExecutionDeclaration
 *
 *     DistributedDeployment
 *         -> distributedDeploymentDeclaration
 *
 *     FaultTolerance
 *         -> distributedFaultToleranceDeclaration
 *
 *     DistributedContracts
 *         -> distributedContractDeclaration
 *
 *     DistributedTransactions
 *         -> distributedTransactionDeclaration
 *
 * This file MUST NOT duplicate those implementations.
 *
 * ============================================================================
 * LEGACY MESSAGE OWNERSHIP
 * ============================================================================
 *
 * The historical:
 *
 *     grammar/distributed/messaging.g4
 *
 * grammar is intentionally NOT imported here.
 *
 * The canonical message schema is now owned by:
 *
 *     grammar/networking/messages.g4
 *
 * and distributed qualification is owned by:
 *
 *     grammar/distributed/messages.g4
 *
 * Therefore:
 *
 *     Networking Messages
 *             |
 *             v
 *     DistributedMessages adapter
 *             |
 *             v
 *     Distributed composition
 *
 * There MUST NOT be two independent definitions of:
 *
 *     messageDeclaration
 *     messageField
 *     messageValue
 *     messageTypeReference
 *
 * in the composed parser.
 *
 * `messaging.g4` may remain only as a compatibility/delegation layer until
 * repository-wide migration removes it.
 *
 * ============================================================================
 * TASK OWNERSHIP
 * ============================================================================
 *
 * Distributed task syntax is an adapter over:
 *
 *     grammar/concurrency/tasks.g4
 *
 * The distributed task grammar MUST NOT become a second task language.
 *
 * Therefore:
 *
 *     concurrency/tasks.g4
 *             |
 *             v
 *     DistributedTasks
 *             |
 *             v
 *     Distributed
 *
 * Generic concurrency composition remains owned by:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * This file deliberately does not import DistributedConcurrency because:
 *
 *     DistributedConcurrency
 *             |
 *             v
 *     Distributed
 *
 * already establishes the opposite dependency.
 *
 * Importing it here would create a circular composition dependency.
 *
 * ============================================================================
 * MESSAGE OWNERSHIP
 * ============================================================================
 *
 * Distributed message declarations and values are separated at this boundary.
 *
 * Declaration:
 *
 *     distributedMessageDeclaration
 *
 * belongs to `distributedDeclaration`.
 *
 * Value:
 *
 *     distributedMessageValue
 *
 * belongs to `distributedExpression`.
 *
 * This avoids classifying every message construct as a declaration.
 *
 * ============================================================================
 * STATEMENT / EXPRESSION OWNERSHIP
 * ============================================================================
 *
 * `ZamaniParser.g4` currently consumes:
 *
 *     distributedDeclaration
 *     distributedStatement
 *     distributedExpression
 *
 * Therefore all three public boundaries are provided here.
 *
 * `distributedStatement` owns only distributed constructs that are
 * structurally statements.
 *
 * `distributedExpression` owns only distributed constructs that are
 * structurally expressions.
 *
 * This avoids forcing the universal parser to infer domain classification
 * from a generic distributed union.
 *
 * ============================================================================
 * PUBLIC IMPORT GRAPH
 * ============================================================================
 *
 *     Distributed
 *       |
 *       +--> Nodes
 *       +--> DistributedServices
 *       +--> Processes
 *       +--> DistributedActors
 *       +--> DistributedChannels
 *       +--> Communication
 *       +--> DistributedMessages
 *       +--> DistributedTasks
 *       +--> Collective
 *       +--> Replication
 *       +--> DistributedConsistency
 *       +--> DistributedPlacement
 *       +--> DistributedPartitioning
 *       +--> DistributedTopology
 *       +--> RemoteExecution
 *       +--> DistributedDeployment
 *       +--> FaultTolerance
 *       +--> DistributedContracts
 *       +--> DistributedTransactions
 *
 * There is deliberately no:
 *
 *     Distributed -> DistributedConcurrency
 *
 * edge.
 *
 * The dependency remains:
 *
 *     DistributedConcurrency
 *             |
 *             v
 *         Distributed
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Distributed syntax may be qualified by requirements, capabilities,
 * constraints, preferences, hints, placement intent, policies, and contracts
 * through the owning specialized grammars.
 *
 * This composition root does not duplicate those systems.
 *
 * The semantic pipeline is:
 *
 *     distributed syntax
 *          |
 *          v
 *     AST
 *          |
 *          +--> requirements
 *          +--> capabilities
 *          +--> constraints
 *          +--> preferences
 *          +--> hints
 *          +--> policies
 *          +--> effects
 *          +--> contracts
 *          +--> provenance
 *          |
 *          v
 *     target feasibility
 *
 * Example semantic requirements may express:
 *
 *     capability("distributed.communication")
 *     capability("gpu.compute")
 *     capability("quantum.measurement")
 *     memory >= required_memory
 *     nodes >= required_nodes
 *     topology(required_topology)
 *
 * The grammar does not turn these requirements into physical allocations.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Distributed operations may imply effects such as:
 *
 *     network
 *     distributed
 *     mutation
 *     foreign
 *     native
 *     measurement
 *     randomness
 *     simulation
 *
 * Effect classification is owned by:
 *
 *     grammar/effects/
 *
 * and semantic analysis.
 *
 * This file does not enumerate or assign effects.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Distributed computation may be controlled by policies concerning:
 *
 *     - placement;
 *     - resource selection;
 *     - communication;
 *     - replication;
 *     - consistency;
 *     - deployment;
 *     - security;
 *     - resilience;
 *     - adaptation;
 *     - simulation;
 *     - portability;
 *     - reproducibility.
 *
 * Policy syntax is owned by:
 *
 *     grammar/policies/
 *     grammar/execution/policies.g4
 *     specialized distributed policy clauses.
 *
 * This file only composes those boundaries.
 *
 * ============================================================================
 * CONTRACT / PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Distributed contracts may express:
 *
 *     requires
 *     ensures
 *     invariant
 *     guarantees
 *
 * Evidence and provenance may be attached by their owning grammar/semantic
 * subsystem.
 *
 * This composition root does not implement contract semantics or provenance.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed quantum computation remains a composition of:
 *
 *     distributed semantics
 *             |
 *             v
 *     quantum semantics
 *             |
 *             v
 *     quantum::ir
 *
 * This grammar MUST NOT define:
 *
 *     - QubitId;
 *     - physical qubit allocation;
 *     - native gate sets;
 *     - calibration;
 *     - pulse semantics;
 *     - noise channels;
 *     - QEC algorithms;
 *     - quantum routing;
 *     - ZQN behavior.
 *
 * Those belong downstream.
 *
 * The same distributed source may therefore be realized using:
 *
 *     - a quantum simulator;
 *     - one QPU;
 *     - multiple QPUs;
 *     - heterogeneous quantum resources;
 *     - distributed quantum resources;
 *     - future quantum substrates.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Distributed hardware computation remains target-independent.
 *
 * Hardware realization is determined from:
 *
 *     requirements
 *     capabilities
 *     resources
 *     topology
 *     placement
 *     scheduling
 *     policies
 *
 * This file does not encode a hardware inventory.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Distributed memory remains owned by:
 *
 *     grammar/memory/distributed-memory.g4
 *
 * This file does not duplicate:
 *
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     memory layout;
 *     physical addresses;
 *     NUMA topology;
 *     memory allocation.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Network-specific syntax remains owned by:
 *
 *     grammar/networking/
 *
 * Distributed communication is represented as logical communication intent.
 *
 * This file does not select:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     vendor transports;
 *     physical interfaces;
 *     IP addresses;
 *     ports.
 *
 * Those are target/runtime decisions.
 *
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * Distributed execution may eventually be realized through:
 *
 *     grammar/execution/
 *
 * including:
 *
 *     adaptive execution;
 *     dispatch;
 *     placement;
 *     scheduling;
 *     recovery;
 *     resilience;
 *     simulation;
 *     runtime capabilities;
 *     deployment;
 *     policies.
 *
 * The distributed grammar expresses intent only.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The frontend must map the resulting parse tree into the existing
 * domain-neutral AST.
 *
 * The AST must preserve, where applicable:
 *
 *     - construct kind;
 *     - declaration order;
 *     - statement order;
 *     - expression structure;
 *     - names;
 *     - qualified names;
 *     - arguments;
 *     - parameters;
 *     - nested bodies;
 *     - dependencies;
 *     - relationships;
 *     - clauses;
 *     - attributes;
 *     - modifiers;
 *     - source spans.
 *
 * The AST MUST NOT turn source syntax into physical backend objects such as:
 *
 *     CpuTask
 *     GpuTask
 *     FpgaTask
 *     QpuTask
 *     CloudNode
 *     TcpPacket
 *     PhysicalQubit
 *
 * merely because the syntax originated in this domain.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - name resolution;
 *     - declaration resolution;
 *     - type checking;
 *     - ownership/lifetime checking;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - policy checking;
 *     - contract checking;
 *     - provenance;
 *     - topology validation;
 *     - partitioning validation;
 *     - placement validation;
 *     - replication validation;
 *     - consistency validation;
 *     - transaction validation;
 *     - fault-tolerance validation;
 *     - deployment validation;
 *     - target feasibility.
 *
 * Parser acceptance does not imply semantic validity.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It MUST NOT introduce:
 *
 *     DistributedIR
 *     DistributedNodeIR
 *     DistributedTaskIR
 *     DistributedMessageIR
 *     DistributedTopologyIR
 *
 * as a competing canonical representation.
 *
 * Distributed information lowers through the repository's established
 * semantic/IR pipeline.
 *
 * Classical computation continues toward canonical classical IR.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * HDL and hardware intent continue through their established semantic
 * representations.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Distributed fault tolerance is expressed through:
 *
 *     FaultTolerance
 *     DistributedContracts
 *     DistributedConsistency
 *     DistributedReplication
 *     execution/resilience
 *
 * This grammar does not implement:
 *
 *     retry;
 *     restart;
 *     failover;
 *     reroute;
 *     reschedule;
 *     remap;
 *     checkpointing;
 *     rollback;
 *     recovery;
 *     consensus;
 *     QEC.
 *
 * Those remain downstream responsibilities.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic for a fixed:
 *
 *     - token stream;
 *     - grammar version;
 *     - imported grammar versions.
 *
 * This grammar performs no:
 *
 *     - network access;
 *     - filesystem access;
 *     - environment inspection;
 *     - hardware discovery;
 *     - randomness;
 *     - runtime execution.
 *
 * Therefore parsing cannot depend on:
 *
 *     - machine size;
 *     - CPU availability;
 *     - GPU availability;
 *     - QPU availability;
 *     - node count;
 *     - network state;
 *     - deployment state;
 *     - wall-clock time.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must remain structural.
 *
 * This file must not report:
 *
 *     "GPU unavailable"
 *     "QPU unavailable"
 *     "not enough nodes"
 *     "network unavailable"
 *     "insufficient memory"
 *
 * as syntax errors.
 *
 * Those are semantic/resource/target-feasibility diagnostics.
 *
 * Structural diagnostics may identify:
 *
 *     - malformed distributed declarations;
 *     - malformed distributed statements;
 *     - malformed distributed expressions;
 *     - missing delimiters;
 *     - invalid source structure;
 *     - invalid grammar composition.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * The public rules exported by this composition root are:
 *
 *     distributedDeclaration
 *     distributedStatement
 *     distributedExpression
 *     distributedConstruct
 *     distributedDeclarationList
 *     distributedDeclarationListNonEmpty
 *     distributedStatementList
 *     distributedStatementListNonEmpty
 *     distributedExpressionList
 *     distributedExpressionListNonEmpty
 *     distributedBlockContent
 *     distributedNonEmptyBlockContent
 *
 * These rules form the stable composition interface.
 *
 * Specialized rules remain owned by the imported grammars.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 */

parser grammar Distributed;

options {
    tokenVocab = ZamaniLexer;
}

import
    Nodes,
    DistributedServices,
    Processes,
    DistributedActors,
    DistributedChannels,
    Communication,
    DistributedMessages,
    DistributedTasks,
    Collective,
    Replication,
    DistributedConsistency,
    DistributedPlacement,
    DistributedPartitioning,
    DistributedTopology,
    RemoteExecution,
    DistributedDeployment,
    FaultTolerance,
    DistributedContracts,
    DistributedTransactions
    ;


/*
 * ============================================================================
 * CANONICAL DISTRIBUTED DECLARATION ENTRY POINT
 * ============================================================================
 *
 * This is the sole declaration-level distributed dispatch boundary.
 *
 * Declaration alternatives are deliberately limited to constructs that are
 * structurally declarations.
 *
 * Statement/expression constructs are NOT placed here.
 */
distributedDeclaration
    : nodeDeclaration
    | distributedServiceDeclaration
    | distributedProcessDeclaration
    | distributedActorConstruct
    | distributedChannelDeclaration
    | distributedMessageDeclaration
    | collectiveDeclaration
    | distributedReplicationDeclaration
    | distributedConsistencyConstruct
    | distributedPlacementDeclaration
    | distributedPartitioningDeclaration
    | distributedTopologyConstruct
    | distributedRemoteExecutionDeclaration
    | distributedDeploymentDeclaration
    | distributedFaultToleranceDeclaration
    | distributedContractDeclaration
    | distributedTransactionDeclaration
    ;


/*
 * ============================================================================
 * CANONICAL DISTRIBUTED STATEMENT ENTRY POINT
 * ============================================================================
 *
 * These alternatives are structurally statements.
 *
 * Distributed task statements continue to be owned by the canonical
 * concurrency task grammar through DistributedTasks.
 *
 * Collective invocation is a statement because its canonical syntax is
 * terminated by SEMICOLON.
 *
 * Communication is a statement-level operation.
 *
 * No generic expression is accepted here.
 */
distributedStatement
    : distributedTaskStatement
    | collectiveInvocation
    | distributedCommunication
    ;


/*
 * ============================================================================
 * CANONICAL DISTRIBUTED EXPRESSION ENTRY POINT
 * ============================================================================
 *
 * Distributed expressions remain narrowly scoped.
 *
 * Distributed task expressions are adapters over the canonical task grammar.
 *
 * Distributed message values are adapters over the canonical networking
 * message grammar.
 *
 * No general `expression` alternative is allowed here because that would make
 * every Zamani expression a distributed expression and would collapse domain
 * ownership.
 */
distributedExpression
    : distributedTaskExpression
    | distributedMessageValue
    ;


/*
 * ============================================================================
 * GENERIC DISTRIBUTED CONSTRUCT
 * ============================================================================
 *
 * This is a composition alias, not a competing semantic root.
 *
 * It exists for parent grammars or future domain adapters that need to accept
 * any distributed construct without duplicating the declaration/statement/
 * expression union.
 */
distributedConstruct
    : distributedDeclaration
    | distributedStatement
    | distributedExpression
    ;


/*
 * ============================================================================
 * DECLARATION LISTS
 * ============================================================================
 *
 * No finite declaration count is imposed.
 */

distributedDeclarationList
    : distributedDeclaration*
    ;

distributedDeclarationListNonEmpty
    : distributedDeclaration+
    ;


/*
 * ============================================================================
 * STATEMENT LISTS
 * ============================================================================
 *
 * No finite statement count is imposed.
 */

distributedStatementList
    : distributedStatement*
    ;

distributedStatementListNonEmpty
    : distributedStatement+
    ;


/*
 * ============================================================================
 * EXPRESSION LISTS
 * ============================================================================
 *
 * No finite expression count is imposed.
 *
 * The general expression grammar remains authoritative for ordinary
 * expression lists. These lists are provided only when a distributed owner
 * needs an explicitly distributed expression boundary.
 */

distributedExpressionList
    : distributedExpression*
    ;

distributedExpressionListNonEmpty
    : distributedExpression+
    ;


/*
 * ============================================================================
 * DISTRIBUTED BLOCK CONTENT
 * ============================================================================
 *
 * A distributed block may contain declarations and statements.
 *
 * Expressions are not independently admitted as block members because
 * expression statements are owned by the canonical statement/expression
 * subsystem.
 *
 * If a future distributed feature requires expression statements, that
 * feature must adapt the canonical statement grammar rather than widening
 * this root indiscriminately.
 */

distributedBlockContent
    : (distributedDeclaration | distributedStatement)*
    ;

distributedNonEmptyBlockContent
    : (distributedDeclaration | distributedStatement)+
    ;


/*
 * ============================================================================
 * SOURCE-ORDER / AST CONTRACT
 * ============================================================================
 *
 * The parser preserves source order through the generated parse tree.
 *
 * The frontend AST layer is responsible for preserving:
 *
 *     - declaration order;
 *     - statement order;
 *     - expression nesting;
 *     - source spans;
 *     - attributes;
 *     - modifiers;
 *     - relationships;
 *     - dependencies.
 *
 * No ordering semantics are inferred by this composition root.
 *
 * ============================================================================
 * SCALABILITY / HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     - no finite distributed capacity;
 *     - no fixed node count;
 *     - no fixed process count;
 *     - no fixed task count;
 *     - no fixed worker count;
 *     - no fixed actor count;
 *     - no fixed channel count;
 *     - no fixed message count;
 *     - no fixed replica count;
 *     - no fixed partition count;
 *     - no fixed topology size;
 *     - no physical addresses;
 *     - no device IDs;
 *     - no hardware enumeration;
 *     - no backend selection.
 *
 * All scalable collections use grammar repetition:
 *
 *     *
 *     +
 *     optional rules
 *
 * rather than universal constants.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing public declaration entry point:
 *
 *     distributedDeclaration
 *
 * is preserved.
 *
 * Existing specialized rule names are consumed rather than renamed.
 *
 * New public composition rules:
 *
 *     distributedStatement
 *     distributedExpression
 *     distributedConstruct
 *
 * are additive integration boundaries required by the canonical
 * `ZamaniParser.g4`.
 *
 * The historical `Messaging` grammar is intentionally no longer imported by
 * this root because message-schema ownership has moved to:
 *
 *     Networking Messages
 *         +
 *     DistributedMessages
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/antlr/ZamaniParser.g4
 *
 * The canonical parser consumes:
 *
 *     distributedDeclaration
 *     distributedStatement
 *     distributedExpression
 *
 *
 * DOWNSTREAM / SPECIALIZED:
 *
 *     grammar/distributed/nodes.g4
 *     grammar/distributed/services.g4
 *     grammar/distributed/processes.g4
 *     grammar/distributed/actors.g4
 *     grammar/distributed/channels.g4
 *     grammar/distributed/communication.g4
 *     grammar/distributed/messages.g4
 *     grammar/distributed/tasks.g4
 *     grammar/distributed/collective.g4
 *     grammar/distributed/replication.g4
 *     grammar/distributed/consistency.g4
 *     grammar/distributed/placement.g4
 *     grammar/distributed/partitioning.g4
 *     grammar/distributed/topology.g4
 *     grammar/distributed/remote-execution.g4
 *     grammar/distributed/deployment.g4
 *     grammar/distributed/fault-tolerance.g4
 *     grammar/distributed/contracts.g4
 *     grammar/distributed/transactions.g4
 *
 *
 * CONCURRENCY:
 *
 *     grammar/concurrency/tasks.g4
 *     grammar/concurrency/concurrency.g4
 *     grammar/concurrency/distributed-concurrency.g4
 *
 * `Distributed` may be imported by `DistributedConcurrency`.
 *
 * `Distributed` MUST NOT import `DistributedConcurrency`.
 *
 *
 * NETWORKING:
 *
 *     grammar/networking/messages.g4
 *     grammar/networking/networking.g4
 *
 * `DistributedMessages` adapts the canonical networking message grammar.
 *
 *
 * RESOURCES:
 *
 *     grammar/resources/
 *
 * Distributed resource requirements remain semantic/resource-owned.
 *
 *
 * EFFECTS:
 *
 *     grammar/effects/
 *
 * Distributed effects remain effect-owned.
 *
 *
 * EXECUTION:
 *
 *     grammar/execution/
 *
 * Placement, scheduling, dispatch, resilience, recovery and runtime behavior
 * remain execution-owned.
 *
 *
 * MEMORY:
 *
 *     grammar/memory/distributed-memory.g4
 *
 * Distributed memory semantics remain memory-owned.
 *
 *
 * QUANTUM:
 *
 *     grammar/quantum/
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 *
 * HARDWARE / HDL:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *
 * Target realization remains outside this grammar.
 *
 *
 * AST:
 *
 *     existing domain-neutral frontend AST
 *
 * This grammar must not introduce a distributed-only AST hierarchy.
 *
 *
 * IR:
 *
 *     canonical classical IR
 *     quantum::ir
 *     established HDL/hardware semantic representation
 *
 * No DistributedIR is created here.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Nodes
 *     DistributedServices
 *     Processes
 *     DistributedActors
 *     DistributedChannels
 *     Communication
 *     DistributedMessages
 *     DistributedTasks
 *     Collective
 *     Replication
 *     DistributedConsistency
 *     DistributedPlacement
 *     DistributedPartitioning
 *     DistributedTopology
 *     RemoteExecution
 *     DistributedDeployment
 *     FaultTolerance
 *     DistributedContracts
 *     DistributedTransactions
 *
 * EXPORTS:
 *
 *     distributedDeclaration
 *     distributedStatement
 *     distributedExpression
 *     distributedConstruct
 *     distributedDeclarationList
 *     distributedDeclarationListNonEmpty
 *     distributedStatementList
 *     distributedStatementListNonEmpty
 *     distributedExpressionList
 *     distributedExpressionListNonEmpty
 *     distributedBlockContent
 *     distributedNonEmptyBlockContent
 *
 * CONSUMED_BY:
 *
 *     ZamaniParser
 *     DistributedConcurrency
 *     other explicitly-authorized domain adapters
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     distributed semantic analysis
 *     plus the semantic owner of each specialized construct
 *
 * TYPE_OWNER:
 *
 *     canonical type system
 *
 * EFFECT_OWNER:
 *
 *     canonical effect system
 *
 * RESOURCE_OWNER:
 *
 *     canonical resource/capability system
 *
 * POLICY_OWNER:
 *
 *     canonical policy system
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance system
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR pipeline
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representation
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/syntax.md
 *     distributed-domain specifications
 *     grammar/distributed/README.md
 *
 * TEST_OWNER:
 *
 *     grammar/tests/distributed/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This composition root is complete only when the test suite covers:
 *
 * POSITIVE:
 *
 *     - node declarations;
 *     - service declarations;
 *     - process declarations;
 *     - distributed actors;
 *     - channels;
 *     - messages;
 *     - tasks;
 *     - collectives;
 *     - replication;
 *     - consistency;
 *     - placement;
 *     - partitioning;
 *     - topology;
 *     - remote execution;
 *     - deployment;
 *     - fault tolerance;
 *     - contracts;
 *     - transactions.
 *
 * STATEMENTS:
 *
 *     - distributed task statements;
 *     - collective invocation;
 *     - distributed communication.
 *
 * EXPRESSIONS:
 *
 *     - distributed task expressions;
 *     - distributed message values.
 *
 * NEGATIVE:
 *
 *     - malformed declarations;
 *     - malformed statements;
 *     - malformed expressions;
 *     - invalid nesting;
 *     - missing delimiters;
 *     - malformed qualified names;
 *     - malformed task/message syntax delegated to the owning grammar.
 *
 * BOUNDARY:
 *
 *     - classical + distributed;
 *     - quantum + distributed;
 *     - hybrid + distributed;
 *     - HDL/hardware + distributed;
 *     - AI + distributed;
 *     - networking + distributed;
 *     - resource requirements + distributed;
 *     - effects + distributed;
 *     - policies + distributed;
 *     - contracts + distributed;
 *     - provenance + distributed;
 *     - simulation + distributed;
 *     - adaptive execution + distributed.
 *
 * SCALABILITY:
 *
 *     - arbitrarily many declarations within implementation resources;
 *     - arbitrarily many statements within implementation resources;
 *     - arbitrarily many logical entities;
 *     - arbitrarily many task relationships;
 *     - arbitrarily large topology descriptions;
 *     - arbitrarily large partition/replication descriptions.
 *
 * The tests MUST verify absence of artificial language-level ceilings.
 *
 * DETERMINISM:
 *
 *     identical token stream + identical grammar version
 *         ->
 *     identical parse structure.
 *
 * COMPATIBILITY:
 *
 *     existing distributedDeclaration syntax remains accepted;
 *     new boundaries do not silently reinterpret unrelated syntax.
 *
 * ============================================================================
 * PRODUCTION-READINESS CHECKLIST
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] There is one distributed composition root.
 * [x] No distributed leaf syntax is duplicated here.
 * [x] Declaration/statement/expression boundaries are explicit.
 * [x] `distributedDeclaration` remains the canonical declaration entry point.
 * [x] `distributedStatement` exists for ZamaniParser integration.
 * [x] `distributedExpression` exists for ZamaniParser integration.
 * [x] Distributed task syntax is delegated to DistributedTasks.
 * [x] Distributed message syntax is delegated to DistributedMessages.
 * [x] Canonical networking message ownership is preserved.
 * [x] Legacy duplicate message ownership is not imported.
 * [x] Collective declaration and invocation are separated correctly.
 * [x] DistributedConcurrency is not imported, avoiding a dependency cycle.
 * [x] Resource realization remains downstream.
 * [x] Capability negotiation remains downstream.
 * [x] Placement remains downstream.
 * [x] Routing remains downstream.
 * [x] Scheduling remains downstream.
 * [x] Resilience remains downstream.
 * [x] Quantum realization remains downstream through quantum::ir.
 * [x] HDL/hardware realization remains downstream.
 * [x] No universal capacity constants exist.
 * [x] No fixed machine/device identifiers exist.
 * [x] No target-specific backend logic exists.
 * [x] No parser actions exist.
 * [x] No semantic predicates exist.
 * [x] No runtime calls exist.
 * [x] No unsafe Rust is required.
 * [x] Rust 1.97+ compatibility is preserved at the grammar boundary.
 * [x] Integration contracts are documented.
 * [x] AST ownership is documented.
 * [x] semantic ownership is documented.
 * [x] IR ownership is documented.
 * [x] test ownership is documented.
 * [x] completion criteria are documented.
 *
 * ============================================================================
 * REQUIRED EXTERNAL INTEGRATION FIXES
 * ============================================================================
 *
 * This file deliberately does not conceal defects in specialized grammars.
 *
 * Before the distributed composition can be declared repository-wide
 * production-ready, the following independent contracts must also hold:
 *
 * 1. grammar/distributed/actors.g4
 *
 *    Must import:
 *
 *        Actors
 *
 *    because it delegates to:
 *
 *        actorDeclaration
 *
 *    from grammar/concurrency/actors.g4.
 *
 * 2. grammar/distributed/messages.g4
 *
 *    Must remain the adapter to:
 *
 *        Messages
 *
 *    from grammar/networking/messages.g4.
 *
 * 3. grammar/distributed/messaging.g4
 *
 *    Must not remain an independent implementation of canonical message
 *    schema rules when the networking message grammar is authoritative.
 *
 * 4. grammar/antlr/ZamaniParser.g4
 *
 *    Already expects:
 *
 *        distributedDeclaration
 *        distributedStatement
 *        distributedExpression
 *
 *    This replacement supplies all three public boundaries.
 *
 * 5. grammar/concurrency/distributed-concurrency.g4
 *
 *    May continue importing:
 *
 *        Distributed
 *
 *    and MUST NOT be imported back into Distributed.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * Invariant 1:
 *
 *     Distributed is a composition root, not a second language.
 *
 * Invariant 2:
 *
 *     Every distributed feature has one specialized grammar owner.
 *
 * Invariant 3:
 *
 *     Generic task syntax remains owned by concurrency/tasks.g4.
 *
 * Invariant 4:
 *
 *     Generic message syntax remains owned by networking/messages.g4.
 *
 * Invariant 5:
 *
 *     DistributedMessages is the only distributed message adapter.
 *
 * Invariant 6:
 *
 *     DistributedTasks is the only distributed task adapter.
 *
 * Invariant 7:
 *
 *     DistributedConcurrency depends on Distributed, never the reverse.
 *
 * Invariant 8:
 *
 *     The parser describes logical distributed intent, not physical
 *     realization.
 *
 * Invariant 9:
 *
 *     No finite distributed machine capacity is encoded.
 *
 * Invariant 10:
 *
 *     Resource availability is not a parser concern.
 *
 * Invariant 11:
 *
 *     Capability availability is not a parser concern.
 *
 * Invariant 12:
 *
 *     Network availability is not a parser concern.
 *
 * Invariant 13:
 *
 *     Quantum semantics continue through quantum::ir.
 *
 * Invariant 14:
 *
 *     No DistributedIR is introduced by this grammar.
 *
 * Invariant 15:
 *
 *     The same source-level meaning can be considered for different
 *     realizations without grammar changes.
 *
 * Invariant 16:
 *
 *     Syntax acceptance does not imply semantic or target support.
 *
 * Invariant 17:
 *
 *     Parser errors remain structural errors.
 *
 * Invariant 18:
 *
 *     Rust implementation remains safe Rust.
 *
 * Invariant 19:
 *
 *     Rust 1.97 or later remains supported.
 *
 * Invariant 20:
 *
 *     Future distributed abstractions may be added through specialized
 *     grammars without redesigning this composition root unless they require
 *     a genuinely new syntactic category.
 *
 * ============================================================================
 * END
 * ============================================================================
 */