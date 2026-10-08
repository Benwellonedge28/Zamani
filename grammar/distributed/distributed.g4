/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/distributed/distributed.g4
 *
 * GRAMMAR
 * -------
 * Distributed
 *
 * ROLE
 * ----
 * Canonical distributed-domain composition root.
 *
 * This file is the ONLY orchestrator of the grammars contained in:
 *
 *     grammar/distributed/
 *
 * It owns composition and public dispatch boundaries.
 *
 * It does NOT own the concrete syntax of distributed features.
 *
 * ============================================================================
 * PRODUCTION CONTRACT
 * ============================================================================
 *
 * Rust baseline:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * Safety:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no unsafe implementation requirement;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no randomness;
 *     - no mutable parser-global state.
 *
 * The generated parser remains a pure syntactic component.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * The distributed subsystem follows:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Distributed
 *          |
 *          +--> specialized distributed grammars
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          +--> topology
 *          +--> placement
 *          +--> partitioning
 *          +--> replication
 *          +--> consistency
 *          +--> resilience
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> Classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
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
 *
 * This grammar participates ONLY in the syntactic stage.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Distributed source expresses logical computation and execution intent.
 *
 * It MUST NOT encode the physical size of the eventual realization.
 *
 * The same source meaning must be representable on:
 *
 *     - one execution resource;
 *     - multiple cores;
 *     - multiple processes;
 *     - multiple machines;
 *     - embedded systems;
 *     - edge systems;
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
 * Physical realization is determined downstream from:
 *
 *     requirements
 *     capabilities
 *     resources
 *     constraints
 *     preferences
 *     hints
 *     policies
 *     topology
 *     placement
 *     scheduling
 *     resilience
 *     target availability
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO universal physical capacity.
 *
 * It MUST NOT define:
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
 * It MUST NOT encode:
 *
 *     - fixed machine counts;
 *     - fixed process counts;
 *     - fixed worker counts;
 *     - fixed replica counts;
 *     - fixed shard counts;
 *     - fixed topology dimensions;
 *     - physical addresses;
 *     - physical device identifiers;
 *     - provider inventories;
 *     - fixed network ports;
 *     - hardware-specific limits.
 *
 * `*`, `+`, and optional grammar constructs describe syntax only.
 *
 * They do NOT constitute physical infinity.
 *
 * Actual finite limits belong to:
 *
 *     compiler;
 *     deployment;
 *     runtime;
 *     target capabilities;
 *     resource availability;
 *     physical hardware.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Distributed computing is an open semantic space.
 *
 * New distributed abstractions MUST NOT require a new lexer keyword merely
 * because a new technology or execution model is introduced.
 *
 * Examples of names that remain ordinary Zamani names:
 *
 *     federation
 *     region
 *     shard
 *     replica
 *     worker
 *     swarm
 *     quantum_network
 *     photonic_fabric
 *     future_protocol
 *     future_topology
 *
 * Semantic analysis determines whether such names are:
 *
 *     defined;
 *     supported;
 *     experimental;
 *     deprecated;
 *     vendor-defined;
 *     dialect-defined;
 *     extension-defined;
 *     unknown.
 *
 * Parsing MUST NOT imply implementation support.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * No lexer rules are defined here.
 *
 * All lexical ownership belongs to:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar MUST NOT introduce:
 *
 *     - duplicate identifiers;
 *     - duplicate punctuation;
 *     - duplicate literals;
 *     - distributed-specific lexer rules;
 *     - physical-target keywords.
 *
 * Contextual distributed concepts should remain ordinary identifiers unless
 * the language specification explicitly requires reserved lexical syntax.
 *
 * ============================================================================
 * NAME AUTHORITY
 * ============================================================================
 *
 * Name syntax belongs to:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * This grammar does not redefine:
 *
 *     identifier
 *     qualifiedName
 *     nameSegment
 *     nameList
 *     qualifiedNameList
 *
 * ============================================================================
 * TYPE AUTHORITY
 * ============================================================================
 *
 * Type syntax remains owned by the canonical type subsystem.
 *
 * Distributed grammars consume types through their leaf grammars.
 *
 * This file does not define:
 *
 *     typeExpression
 *     genericType
 *     referenceType
 *     ownershipType
 *     quantumType
 *     tensorType
 *     resourceType
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * General expression syntax belongs to:
 *
 *     grammar/expressions/
 *
 * This composition root does not redefine:
 *
 *     expression
 *     expressionList
 *     calls
 *     indexing
 *     member access
 *     operators
 *     literals
 *
 * Distributed expression boundaries are adapters only.
 *
 * ============================================================================
 * CONTRACT / POLICY / RESOURCE AUTHORITY
 * ============================================================================
 *
 * This root does not duplicate:
 *
 *     requires
 *     ensures
 *     invariant
 *     guarantee
 *     capability
 *     resource
 *     constraint
 *     preference
 *     policy
 *     provenance
 *     evidence
 *     effect
 *
 * Specialized grammars may expose those concepts through their own public
 * clauses.
 *
 * Their semantic meaning remains owned by the corresponding universal
 * subsystem.
 *
 * ============================================================================
 * DISTRIBUTED LEAF OWNERSHIP
 * ============================================================================
 *
 * The following files are orchestrated here.
 *
 *     actors.g4
 *         DistributedActors
 *
 *     channels.g4
 *         DistributedChannels
 *
 *     collective.g4
 *         Collective
 *
 *     communication.g4
 *         Communication
 *
 *     consistency.g4
 *         DistributedConsistency
 *
 *     contracts.g4
 *         DistributedContracts
 *
 *     deployment.g4
 *         DistributedDeployment
 *
 *     fault-tolerance.g4
 *         FaultTolerance
 *
 *     messages.g4
 *         DistributedMessages
 *
 *     nodes.g4
 *         Nodes
 *
 *     partitioning.g4
 *         DistributedPartitioning
 *
 *     placement.g4
 *         DistributedPlacement
 *
 *     processes.g4
 *         Processes
 *
 *     remote-execution.g4
 *         RemoteExecution
 *
 *     replication.g4
 *         Replication
 *
 *     services.g4
 *         DistributedServices
 *
 *     tasks.g4
 *         DistributedTasks
 *
 *     topology.g4
 *         DistributedTopology
 *
 *     transactions.g4
 *         DistributedTransactions
 *
 * The historical:
 *
 *     messaging.g4
 *
 * is deliberately NOT imported.
 *
 * Canonical message-schema ownership is:
 *
 *     grammar/networking/messages.g4
 *             |
 *             v
 *     DistributedMessages
 *             |
 *             v
 *     Distributed
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
 * PUBLIC DECLARATION BOUNDARY
 * ============================================================================
 *
 * This is the sole distributed declaration dispatcher.
 *
 * Concrete declaration syntax remains owned by the imported leaf grammar.
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
 * PUBLIC STATEMENT BOUNDARY
 * ============================================================================
 *
 * Only constructs whose owning syntax is statement-shaped belong here.
 *
 * Generic Zamani statements remain owned by grammar/statements/.
 */
distributedStatement
    : distributedTaskStatement
    | collectiveInvocation
    | distributedCommunication
    ;


/*
 * ============================================================================
 * PUBLIC EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Only distributed-owned expression adapters belong here.
 *
 * General expressions remain owned by grammar/expressions/.
 */
distributedExpression
    : distributedTaskExpression
    | distributedMessageValue
    ;


/*
 * ============================================================================
 * PUBLIC DISTRIBUTED CONSTRUCT
 * ============================================================================
 *
 * This is the complete domain-level composition boundary.
 *
 * It is an alias over the three structural categories:
 *
 *     declaration
 *     statement
 *     expression
 *
 * It creates no new semantic construct.
 */
distributedConstruct
    : distributedDeclaration
    | distributedStatement
    | distributedExpression
    ;


/*
 * ============================================================================
 * DISTRIBUTED TASK COMPATIBILITY BOUNDARY
 * ============================================================================
 *
 * `DistributedTasks` owns the task adapter.
 *
 * The public `distributedTask` rule is intentionally exposed here through
 * the imported grammar so that:
 *
 *     grammar/concurrency/distributed-concurrency.g4
 *
 * can consume the same task construct without creating a second task grammar.
 *
 * DO NOT redefine distributedTask here.
 */


/*
 * ============================================================================
 * DISTRIBUTED OPERATION COMPATIBILITY ALIAS
 * ============================================================================
 *
 * Historical/concurrency integration refers to:
 *
 *     distributedOperation
 *
 * No independent distributed-operation syntax is created.
 *
 * The alias intentionally delegates to the canonical distributed construct
 * boundary.
 *
 * This preserves compatibility while preventing:
 *
 *     distributedOperation
 *     distributedCommunication
 *     distributedTask
 *     distributedInvocation
 *
 * from becoming competing semantic languages.
 *
 * Semantic classification occurs downstream.
 */
distributedOperation
    : distributedConstruct
    ;


/*
 * ============================================================================
 * DECLARATION COLLECTIONS
 * ============================================================================
 *
 * These rules impose no semantic cardinality.
 */

distributedDeclarationList
    : distributedDeclaration*
    ;

distributedDeclarationListNonEmpty
    : distributedDeclaration+
    ;


/*
 * ============================================================================
 * STATEMENT COLLECTIONS
 * ============================================================================
 */

distributedStatementList
    : distributedStatement*
    ;

distributedStatementListNonEmpty
    : distributedStatement+
    ;


/*
 * ============================================================================
 * EXPRESSION COLLECTIONS
 * ============================================================================
 *
 * These lists are provided only to distributed-owned consumers.
 *
 * The universal expression-list grammar remains authoritative for ordinary
 * Zamani expressions.
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
 * A distributed block may contain distributed declarations and distributed
 * statements.
 *
 * Ordinary expression statements remain owned by the canonical statement
 * subsystem.
 */
distributedBlockContent
    : (distributedDeclaration | distributedStatement)*
    ;

distributedNonEmptyBlockContent
    : (distributedDeclaration | distributedStatement)+
    ;


/*
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * The generated parse tree preserves source order.
 *
 * The AST layer is responsible for preserving:
 *
 *     - declaration order;
 *     - statement order;
 *     - nested expression structure;
 *     - source spans;
 *     - attributes;
 *     - modifiers;
 *     - dependencies;
 *     - relationships.
 *
 * This grammar does not infer execution ordering.
 */


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parser acceptance does not imply semantic validity.
 *
 * Downstream analysis owns:
 *
 *     name resolution
 *     type checking
 *     effect checking
 *     capability checking
 *     resource checking
 *     requirement checking
 *     constraint checking
 *     policy checking
 *     contract checking
 *     provenance
 *     topology validation
 *     placement validation
 *     partition validation
 *     replication validation
 *     consistency validation
 *     transaction validation
 *     fault-tolerance validation
 *     deployment validation
 *     target feasibility
 *
 * The parser MUST NOT diagnose physical infeasibility as syntax failure.
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Distributed source may ultimately be qualified by universal resource
 * semantics such as:
 *
 *     requires capability("distributed.communication");
 *     requires capability("gpu.compute");
 *     requires capability("quantum.measurement");
 *     requires memory >= required_memory;
 *     requires topology(required_topology);
 *
 * The distributed grammar does not resolve those requirements.
 *
 * The semantic pipeline is:
 *
 *     syntax
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     resource analysis
 *       |
 *       v
 *     capability negotiation
 *       |
 *       v
 *     execution planning
 *       |
 *       v
 *     target realization
 *
 * No physical resource is selected here.
 */


/*
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Distributed constructs may carry semantic effects including:
 *
 *     network
 *     distributed
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     measurement
 *     simulation
 *
 * Effect ownership remains in grammar/effects/ and semantic analysis.
 *
 * This root does not assign effects merely because a rule is distributed.
 */


/*
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Distributed execution may be constrained by policies for:
 *
 *     placement
 *     communication
 *     replication
 *     consistency
 *     deployment
 *     resilience
 *     security
 *     adaptation
 *     simulation
 *     portability
 *     reproducibility
 *
 * Policy syntax remains owned by the policy/execution/security subsystems and
 * specialized leaf grammars.
 */


/*
 * ============================================================================
 * CONTRACT / PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Distributed contracts are parsed by DistributedContracts.
 *
 * Universal contract semantics remain owned by validation.
 *
 * Provenance remains a universal semantic facility and may preserve:
 *
 *     source
 *     derivation
 *     transformation
 *     evidence
 *     decision
 *     verification
 *     version
 *
 * This composition root does not create a distributed provenance model.
 */


/*
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Canonical direction:
 *
 *     Distributed
 *          ^
 *          |
 *     DistributedConcurrency
 *
 * Therefore:
 *
 *     Distributed
 *
 * MUST NOT import:
 *
 *     DistributedConcurrency
 *
 * `DistributedConcurrency` may import this grammar and consume:
 *
 *     distributedTask
 *     distributedOperation
 *     distributedStatement
 *     distributedExpression
 *
 * Generic task syntax remains owned by:
 *
 *     grammar/concurrency/tasks.g4
 *
 * Generic actor syntax remains owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * Generic channels remain owned by:
 *
 *     grammar/concurrency/channels.g4
 */


/*
 * ============================================================================
 * MESSAGING INTEGRATION
 * ============================================================================
 *
 * Canonical message-schema ownership:
 *
 *     grammar/networking/messages.g4
 *
 * Distributed adapter:
 *
 *     grammar/distributed/messages.g4
 *
 * Legacy:
 *
 *     grammar/distributed/messaging.g4
 *
 * is not imported.
 *
 * Therefore there is exactly one active distributed message adapter and one
 * canonical generic message-schema owner.
 */


/*
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Distributed communication expresses logical communication intent.
 *
 * This grammar does not choose:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     vendor transports
 *     network interface
 *     IP address
 *     port
 *
 * Networking realization remains downstream.
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed quantum computation is represented by the combination of:
 *
 *     distributed semantics
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar does NOT define:
 *
 *     qubit allocation
 *     physical qubit identifiers
 *     native gates
 *     calibration
 *     pulse schedules
 *     QEC implementation
 *     quantum routing
 *     QPU selection
 *
 * The same distributed source may therefore target:
 *
 *     quantum simulation
 *     one QPU
 *     multiple QPUs
 *     heterogeneous quantum resources
 *     future quantum substrates
 */


/*
 * ============================================================================
 * CLASSICAL / AI / DATA / HDL INTEGRATION
 * ============================================================================
 *
 * Distributed computation may contain or invoke:
 *
 *     classical computation
 *     tensor computation
 *     AI computation
 *     knowledge operations
 *     reasoning
 *     learning
 *     adaptive execution
 *     data processing
 *     HDL intent
 *     hardware intent
 *
 * Those domains remain independently owned.
 *
 * Distributed.g4 orchestrates only their distributed composition boundary.
 *
 * No AI-specific application syntax is introduced here.
 */


/*
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware realization remains capability/resource driven.
 *
 * The grammar MUST NOT encode:
 *
 *     CPU inventory
 *     GPU inventory
 *     FPGA inventory
 *     ASIC inventory
 *     QPU inventory
 *     machine addresses
 *     physical device identifiers
 *     fixed accelerator counts
 *
 * Logical distributed intent is lowered downstream.
 */


/*
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Distributed memory semantics remain owned by the memory subsystem.
 *
 * This grammar does not define:
 *
 *     physical addresses
 *     NUMA topology
 *     memory capacities
 *     ownership mechanics
 *     borrowing
 *     lifetimes
 *     allocation algorithms
 */


/*
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * Distributed execution may eventually consume:
 *
 *     execution/placement
 *     execution/scheduling
 *     execution/adaptive
 *     execution/recovery
 *     execution/resilience
 *     execution/simulation
 *     execution/runtime-capabilities
 *     execution/policies
 *     execution/deployment
 *
 * The parser describes intent.
 *
 * It does not:
 *
 *     discover targets;
 *     select targets;
 *     schedule work;
 *     route traffic;
 *     recover failures;
 *     allocate resources.
 */


/*
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Distributed fault tolerance is composed from:
 *
 *     FaultTolerance
 *     DistributedContracts
 *     DistributedConsistency
 *     Replication
 *     execution/resilience
 *     execution/recovery
 *
 * The parser does not execute:
 *
 *     retry
 *     restart
 *     failover
 *     reroute
 *     reschedule
 *     remap
 *     checkpoint
 *     rollback
 *     recovery
 *
 * Those are semantic/runtime responsibilities.
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parse structure only.
 *
 * The domain-neutral AST must preserve, where applicable:
 *
 *     construct kind
 *     source order
 *     names
 *     qualified names
 *     parameters
 *     arguments
 *     nested bodies
 *     dependencies
 *     relationships
 *     attributes
 *     modifiers
 *     clauses
 *     source spans
 *
 * The AST MUST NOT automatically become:
 *
 *     CpuTask
 *     GpuTask
 *     FpgaTask
 *     QpuTask
 *     CloudNode
 *     TcpPacket
 *     PhysicalQubit
 *
 * merely because a construct originated in this grammar.
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * In particular, it does not create:
 *
 *     DistributedIR
 *     DistributedNodeIR
 *     DistributedTaskIR
 *     DistributedMessageIR
 *     DistributedTopologyIR
 *
 * Distributed semantic information flows into the repository's canonical
 * semantic representation.
 *
 * Classical computation:
 *
 *     -> Classical IR
 *
 * Quantum computation:
 *
 *     -> quantum::ir
 *
 * HDL/hardware:
 *
 *     -> established HDL/hardware semantic representation
 *
 * The downstream compiler performs:
 *
 *     optimization
 *     lowering
 *     partitioning
 *     placement
 *     routing
 *     scheduling
 *     resilience
 *     ZQN generation
 *     HAL realization
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For a fixed:
 *
 *     token stream
 *     grammar version
 *     imported grammar versions
 *
 * parsing must be deterministic.
 *
 * Parsing MUST NOT depend upon:
 *
 *     hardware
 *     CPU count
 *     memory size
 *     node count
 *     network state
 *     runtime state
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     target availability.
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural parser errors include:
 *
 *     malformed declaration
 *     malformed statement
 *     malformed expression
 *     missing delimiter
 *     invalid structural nesting
 *     malformed delegated construct
 *     unexpected EOF
 *
 * The parser MUST NOT report physical feasibility as syntax errors.
 *
 * Therefore diagnostics such as:
 *
 *     GPU unavailable
 *     QPU unavailable
 *     insufficient memory
 *     insufficient nodes
 *     unavailable network
 *     unsupported transport
 *
 * belong downstream.
 */


/*
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *
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
 *     distributedOperation
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
 *     explicitly-authorized domain adapters
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     distributed semantic layer
 *     plus each specialized construct's semantic owner
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
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *     grammar/distributed/README.md
 *
 * TEST_OWNER:
 *
 *     grammar/tests/distributed/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/resources/
 *     grammar/tests/effects/
 *     grammar/tests/policies/
 *     grammar/tests/provenance/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     node
 *     service
 *     process
 *     actor
 *     channel
 *     message
 *     task
 *     collective
 *     replication
 *     consistency
 *     placement
 *     partitioning
 *     topology
 *     remote execution
 *     deployment
 *     fault tolerance
 *     contract
 *     transaction
 *
 * STATEMENTS:
 *
 *     distributed task statement
 *     collective invocation
 *     communication
 *
 * EXPRESSIONS:
 *
 *     distributed task expression
 *     distributed message value
 *
 * COMPATIBILITY:
 *
 *     distributedOperation
 *         -> distributedConstruct
 *
 * must remain an alias rather than an independent syntax family.
 *
 * NEGATIVE:
 *
 *     malformed declarations
 *     malformed statements
 *     malformed expressions
 *     malformed delegated constructs
 *     invalid nesting
 *     missing delimiters
 *     unexpected EOF
 *
 * CROSS-DOMAIN:
 *
 *     classical + distributed
 *     quantum + distributed
 *     hybrid + distributed
 *     HDL + distributed
 *     hardware + distributed
 *     AI + distributed
 *     data + distributed
 *     networking + distributed
 *     resources + distributed
 *     effects + distributed
 *     contracts + distributed
 *     policies + distributed
 *     provenance + distributed
 *     simulation + distributed
 *     adaptive execution + distributed
 *
 * SCALABILITY:
 *
 *     arbitrarily many declarations subject only to implementation resources
 *     arbitrarily many statements subject only to implementation resources
 *     arbitrarily many logical entities
 *     arbitrarily large topology descriptions
 *     arbitrarily large partition descriptions
 *     arbitrarily large replication descriptions
 *
 * DETERMINISM:
 *
 *     identical source
 *         ->
 *     identical token stream
 *         ->
 *     equivalent parse structure
 *
 * HARDWARE-INDEPENDENCE:
 *
 *     parsing remains unchanged when target hardware changes.
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [x] no machine-count constants;
 *     [x] no node-count constants;
 *     [x] no process-count constants;
 *     [x] no task-count constants;
 *     [x] no actor-count constants;
 *     [x] no channel-count constants;
 *     [x] no message-count constants;
 *     [x] no replica-count constants;
 *     [x] no partition-count constants;
 *     [x] no topology-size constants;
 *     [x] no memory-capacity constants;
 *     [x] no bandwidth constants;
 *     [x] no hardware inventory;
 *     [x] no physical addresses;
 *     [x] no provider-specific targets;
 *     [x] no backend selection;
 *     [x] no scheduler selection;
 *     [x] no routing algorithm selection;
 *     [x] no parser-side resource discovery;
 *     [x] no parser-side capability negotiation.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] one Distributed composition root exists;
 *     [x] every active distributed leaf grammar is imported exactly here;
 *     [x] no leaf syntax is duplicated;
 *     [x] declarations have one dispatch boundary;
 *     [x] statements have one dispatch boundary;
 *     [x] expressions have one dispatch boundary;
 *     [x] distributedConstruct is the complete composition alias;
 *     [x] distributedOperation is compatibility-only;
 *     [x] distributedOperation creates no second operation language;
 *     [x] generic tasks remain owned by concurrency/tasks.g4;
 *     [x] generic actors remain owned by concurrency/actors.g4;
 *     [x] generic message schemas remain owned by networking/messages.g4;
 *     [x] legacy messaging.g4 is not imported;
 *     [x] DistributedConcurrency is not imported;
 *     [x] no circular dependency is introduced here;
 *     [x] resource realization remains downstream;
 *     [x] capability negotiation remains downstream;
 *     [x] placement remains downstream;
 *     [x] routing remains downstream;
 *     [x] scheduling remains downstream;
 *     [x] resilience remains downstream;
 *     [x] quantum lowering remains downstream through quantum::ir;
 *     [x] HDL/hardware realization remains downstream;
 *     [x] no universal physical capacity is encoded;
 *     [x] no target-specific syntax is encoded;
 *     [x] no parser actions exist;
 *     [x] no semantic predicates exist;
 *     [x] no unsafe Rust is required;
 *     [x] Rust 1.97+ compatibility is preserved;
 *     [x] AST ownership is explicit;
 *     [x] semantic ownership is explicit;
 *     [x] IR ownership is explicit;
 *     [x] resource ownership is explicit;
 *     [x] policy ownership is explicit;
 *     [x] provenance ownership is explicit;
 *     [x] test ownership is explicit;
 *     [x] scalability requirements are explicit;
 *     [x] compatibility requirements are explicit.
 *
 * ============================================================================
 * REQUIRED COMPANION INTEGRATION
 * ============================================================================
 *
 * This composition root is complete only if the following independent
 * dependency defects are corrected as part of their owning files.
 *
 * 1. grammar/distributed/actors.g4
 *
 *    Its current `distributedActorDeclaration` consumes:
 *
 *        actorDeclaration
 *
 *    from:
 *
 *        grammar/concurrency/actors.g4
 *
 *    Therefore DistributedActors MUST import:
 *
 *        Actors
 *
 *    Its dependency must be:
 *
 *        Actors
 *          |
 *          v
 *        DistributedActors
 *          |
 *          v
 *        Distributed
 *
 *    Do NOT duplicate actorDeclaration inside DistributedActors.
 *
 * 2. grammar/concurrency/distributed-concurrency.g4
 *
 *    It consumes:
 *
 *        distributedTask
 *        distributedOperation
 *
 *    `distributedTask` is supplied by DistributedTasks.
 *
 *    `distributedOperation` is supplied by this file as a compatibility alias.
 *
 *    Therefore no second distributed operation grammar is necessary.
 *
 * 3. grammar/distributed/messaging.g4
 *
 *    It remains outside this composition root.
 *
 *    It must be treated as a compatibility/deprecation layer and MUST NOT
 *    become another active message-schema owner.
 *
 * 4. grammar/networking/messages.g4
 *
 *    Remains the canonical generic message-schema owner.
 *
 * 5. grammar/antlr/ZamaniParser.g4
 *
 *    Continues consuming:
 *
 *        distributedDeclaration
 *        distributedStatement
 *        distributedExpression
 *
 *    No change to the universal parser contract is required merely because
 *    this composition root is replaced.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Correct:
 *
 *     lexer
 *       |
 *       v
 *     universal grammar
 *       |
 *       v
 *     domain grammar
 *       |
 *       v
 *     distributed composition root
 *       |
 *       +--> distributed leaf grammars
 *
 * More precisely:
 *
 *     ZamaniParser
 *          |
 *          v
 *     Distributed
 *          |
 *          +--> Nodes
 *          +--> Services
 *          +--> Processes
 *          +--> Actors
 *          +--> Channels
 *          +--> Communication
 *          +--> Messages
 *          +--> Tasks
 *          +--> Collective
 *          +--> Replication
 *          +--> Consistency
 *          +--> Placement
 *          +--> Partitioning
 *          +--> Topology
 *          +--> RemoteExecution
 *          +--> Deployment
 *          +--> FaultTolerance
 *          +--> Contracts
 *          +--> Transactions
 *
 * And:
 *
 *     DistributedConcurrency
 *          |
 *          v
 *     Distributed
 *
 * NOT:
 *
 *     Distributed
 *          |
 *          v
 *     DistributedConcurrency
 *
 * ============================================================================
 * FINAL INVARIANTS
 * ============================================================================
 *
 * 1. Distributed is one composition root.
 *
 * 2. Each distributed feature has one concrete syntax owner.
 *
 * 3. This file owns only composition and dispatch.
 *
 * 4. Generic task syntax remains owned by Tasks.
 *
 * 5. Generic actor syntax remains owned by Actors.
 *
 * 6. Generic message syntax remains owned by networking Messages.
 *
 * 7. DistributedMessages is the only active distributed message adapter.
 *
 * 8. DistributedTasks is the only active distributed task adapter.
 *
 * 9. DistributedConcurrency depends on Distributed, never the reverse.
 *
 * 10. distributedOperation is a compatibility alias, not a semantic language.
 *
 * 11. The parser describes logical intent, not physical realization.
 *
 * 12. No finite distributed capacity is encoded.
 *
 * 13. Resource availability is not a parser concern.
 *
 * 14. Capability availability is not a parser concern.
 *
 * 15. Network availability is not a parser concern.
 *
 * 16. Quantum semantics remain downstream through quantum::ir.
 *
 * 17. No DistributedIR is introduced.
 *
 * 18. Syntax acceptance does not imply semantic or target support.
 *
 * 19. Parser diagnostics remain structural.
 *
 * 20. Safe Rust remains sufficient for the implementation.
 *
 * 21. Rust 1.97+ remains the implementation baseline.
 *
 * 22. Future distributed abstractions can be introduced through leaf grammars
 *     without redesigning this root unless a genuinely new syntactic category
 *     is required.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */