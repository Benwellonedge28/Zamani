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
 *     Production parser-composition root
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *
 * Safety:
 *     - Grammar-only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No mutable parser-global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE DISTRIBUTED-DOMAIN PARSER COMPOSITION ROOT.
 *
 * It does NOT implement the individual distributed language features.
 *
 * Individual source-level distributed constructs remain owned by their
 * existing specialized grammar files under:
 *
 *     grammar/distributed/
 *
 * This file composes those grammars into one stable public distributed
 * language boundary.
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * The architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     Distributed parser composition
 *          |
 *          +--> Nodes
 *          +--> Services
 *          +--> Processes
 *          +--> Actors
 *          +--> Channels
 *          +--> Communication
 *          +--> Messaging
 *          +--> Collective computation
 *          +--> Replication
 *          +--> Consistency
 *          +--> Placement
 *          +--> Partitioning
 *          +--> Topology
 *          +--> Remote execution
 *          +--> Deployment
 *          +--> Fault tolerance
 *          +--> Contracts
 *          +--> Transactions
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> ownership/lifetime analysis
 *          +--> security analysis
 *          +--> distributed semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical computation / IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed execution metadata
 *          +--> resource requirements
 *          |
 *          v
 *     optimization
 *          |
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> deployment
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar MUST remain upstream of all implementation decisions.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed syntax participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A distributed program describes logical computation and execution intent.
 *
 * It MUST NOT require source rewriting merely because the same program is
 * realized using:
 *
 *     - one execution resource;
 *     - many CPU cores;
 *     - many processes;
 *     - many machines;
 *     - embedded resources;
 *     - edge resources;
 *     - HPC systems;
 *     - clusters;
 *     - supercomputers;
 *     - cloud resources;
 *     - federated resources;
 *     - heterogeneous CPU/GPU/FPGA/ASIC resources;
 *     - quantum-classical resources;
 *     - distributed quantum resources;
 *     - future computational substrates.
 *
 * This grammar therefore describes WHAT distributed computation means,
 * while downstream systems determine HOW and WHERE it is realized.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * NO LANGUAGE-LEVEL HARDWARE OR DEPLOYMENT CAPACITY IS ENCODED HERE.
 *
 * This file MUST NOT impose:
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
 *     - fixed CPU IDs;
 *     - fixed GPU IDs;
 *     - fixed FPGA IDs;
 *     - fixed QPU IDs;
 *     - fixed physical memory addresses;
 *     - fixed network addresses;
 *     - fixed ports;
 *     - fixed cloud-provider inventories;
 *     - fixed topology sizes.
 *
 * Repetition in the composed grammars is represented by normal ANTLR
 * repetition constructs and therefore does not establish a language-level
 * capacity.
 *
 * Practical limits are implementation/deployment policy, not language
 * semantics.
 *
 * ============================================================================
 * REQUIREMENT / REALIZATION SEPARATION
 * ============================================================================
 *
 * Distributed source may express:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     hints
 *     logical relationships
 *     logical topology
 *     logical placement intent
 *     execution policies
 *
 * It MUST NOT silently convert those concepts into:
 *
 *     physical node selection;
 *     machine selection;
 *     transport selection;
 *     routing;
 *     scheduling;
 *     hardware allocation;
 *     cloud-provider selection.
 *
 * For example, a semantic requirement equivalent to:
 *
 *     requires capability("distributed.communication")
 *
 * is not equivalent to:
 *
 *     use_machine(0)
 *
 * and:
 *
 *     requires nodes >= n
 *
 * is not a grammar-level maximum of n nodes.
 *
 * Resource/capability interpretation belongs downstream.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Distributed computing must remain extensible.
 *
 * The distributed grammar MUST NOT require a new lexer keyword whenever a new
 * distributed abstraction is introduced.
 *
 * Existing specialized grammars already provide structured syntax for the
 * currently defined distributed domains.
 *
 * Future semantic concepts may be represented through the language's existing
 * identifier/qualified-name and expression mechanisms where their owning
 * grammar explicitly permits extension.
 *
 * Examples of semantic names that must not require new lexer tokens merely
 * because they are new:
 *
 *     distributed::federation
 *     distributed::region
 *     distributed::shard
 *     distributed::replica
 *     distributed::quantum_network
 *     distributed::edge_region
 *     distributed::photonic_fabric
 *     distributed::neuromorphic_region
 *     distributed::future_architecture
 *     distributed::future_protocol
 *
 * Syntactic acceptance does not imply semantic support.
 *
 * Semantic analysis determines whether a construct is:
 *
 *     defined
 *     supported
 *     experimental
 *     deprecated
 *     vendor-specific
 *     dialect-defined
 *     extension-defined
 *     unknown
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar does NOT define lexer rules.
 *
 * All token spelling and token identity come from the canonical Zamani lexer:
 *
 *     ZamaniLexer
 *
 * This grammar therefore MUST NOT define:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     keywords
 *     operators
 *     punctuation
 *     whitespace
 *     comments
 *
 * Domain-specific concepts such as:
 *
 *     node
 *     worker
 *     service
 *     actor
 *     shard
 *     replica
 *     federation
 *
 * MUST NOT be added to the lexer merely because they are distributed concepts.
 *
 * ============================================================================
 * NAME AUTHORITY
 * ============================================================================
 *
 * Canonical name syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * and its canonical parser grammar:
 *
 *     Names
 *
 * This composition root consumes:
 *
 *     identifier
 *     qualifiedName
 *
 * through imported grammar dependencies where required.
 *
 * It MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     nameSegment
 *     nameList
 *     qualifiedNameList
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * General expressions remain owned by the canonical expression grammar.
 *
 * Distributed grammars consume:
 *
 *     expression
 *     expressionList
 *     optionalExpressionList
 *
 * where their individual contracts require them.
 *
 * This file MUST NOT redefine:
 *
 *     expression
 *     expressionList
 *     operator precedence
 *     arithmetic
 *     logical expressions
 *     indexing
 *     calls
 *     member access
 *
 * ============================================================================
 * TYPE AUTHORITY
 * ============================================================================
 *
 * Type syntax remains owned by the canonical type system.
 *
 * Distributed grammars may consume type expressions through their existing
 * imports.
 *
 * This file MUST NOT redefine:
 *
 *     typeExpression
 *     generic type syntax
 *     ownership types
 *     reference types
 *     quantum types
 *     tensor types
 *     resource types
 *
 * ============================================================================
 * SPECIALIZED GRAMMAR OWNERSHIP
 * ============================================================================
 *
 * The following existing files remain the authoritative source-level owners
 * of their respective distributed constructs:
 *
 *     nodes.g4
 *         -> Nodes
 *
 *     services.g4
 *         -> DistributedServices
 *
 *     processes.g4
 *         -> Processes
 *
 *     actors.g4
 *         -> DistributedActors
 *
 *     channels.g4
 *         -> DistributedChannels
 *
 *     communication.g4
 *         -> Communication
 *
 *     messaging.g4
 *         -> Messaging
 *
 *     collective.g4
 *         -> Collective
 *
 *     replication.g4
 *         -> Replication
 *
 *     consistency.g4
 *         -> DistributedConsistency
 *
 *     placement.g4
 *         -> DistributedPlacement
 *
 *     partitioning.g4
 *         -> DistributedPartitioning
 *
 *     topology.g4
 *         -> DistributedTopology
 *
 *     remote-execution.g4
 *         -> RemoteExecution
 *
 *     deployment.g4
 *         -> DistributedDeployment
 *
 *     fault-tolerance.g4
 *         -> FaultTolerance
 *
 *     contracts.g4
 *         -> DistributedContracts
 *
 *     transactions.g4
 *         -> DistributedTransactions
 *
 * This file MUST NOT duplicate their internal rules.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * Canonical composition:
 *
 *     Distributed
 *       |
 *       +--> Nodes
 *       +--> DistributedServices
 *       +--> Processes
 *       +--> DistributedActors
 *       +--> DistributedChannels
 *       +--> Communication
 *       +--> Messaging
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
 * Each specialized grammar remains responsible for importing its own
 * foundational dependencies.
 *
 * The composition root therefore does not duplicate:
 *
 *     Names
 *     Expressions
 *     Types
 *     Parameters
 *     Resources
 *     Concurrency
 *     Networking
 *     Hardware
 *
 * ============================================================================
 * CRITICAL COMPOSITION RULE
 * ============================================================================
 *
 * There is ONE canonical public distributed entry point:
 *
 *     distributedDeclaration
 *
 * This rule is the stable boundary consumed by the higher-level Zamani
 * composition grammar.
 *
 * Do NOT create competing root concepts such as:
 *
 *     distributedProgram
 *     distributedCompilationUnit
 *     distributedCompleteScope
 *     distributedCompleteMember
 *
 * as alternative versions of the same public entry point.
 *
 * If a future parent grammar needs a contextual wrapper, it should wrap:
 *
 *     distributedDeclaration
 *
 * rather than create another distributed language root.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO AST implementation.
 *
 * The parser tree produced by this composition root must be mapped by the
 * frontend to the domain-neutral AST under:
 *
 *     src/frontend/ast/
 *
 * The AST must preserve, as applicable:
 *
 *     - source ordering;
 *     - declaration ordering;
 *     - construct kind;
 *     - source names;
 *     - qualified-name segments;
 *     - expression structure;
 *     - type structure;
 *     - argument ordering;
 *     - nested bodies;
 *     - relationships;
 *     - dependencies;
 *     - clauses;
 *     - properties;
 *     - source spans;
 *     - source-level attributes/modifiers.
 *
 * The distributed grammar MUST NOT force backend-specific AST nodes merely
 * because the source construct is distributed.
 *
 * A logical distributed task must not automatically become:
 *
 *     CpuTask
 *     GpuTask
 *     FpgaTask
 *     QpuTask
 *     CloudTask
 *
 * merely because of parser classification.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for:
 *
 *     - name resolution;
 *     - declaration resolution;
 *     - type checking;
 *     - effect checking;
 *     - capability checking;
 *     - resource requirement analysis;
 *     - ownership/lifetime analysis;
 *     - security analysis;
 *     - distributed relationship validation;
 *     - topology validation;
 *     - placement intent validation;
 *     - replication validation;
 *     - consistency validation;
 *     - fault-tolerance validation;
 *     - transaction validation;
 *     - deployment validation;
 *     - portability analysis.
 *
 * The parser does NOT determine whether the requested computation can
 * actually be realized on a particular deployment.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * There must be no automatic:
 *
 *     DistributedIR
 *     DistributedTopologyIR
 *     DistributedNodeIR
 *     DistributedTaskIR
 *
 * introduced merely because syntax originated in this directory.
 *
 * Distributed semantic information must lower through the repository's
 * established canonical semantic/IR architecture.
 *
 * Classical computation follows the canonical classical pipeline.
 *
 * Quantum computation MUST continue through:
 *
 *     quantum::ir
 *
 * and MUST NOT create a competing frontend-specific quantum IR.
 *
 * Hardware/HDL constructs continue through their established hardware/HDL
 * representation.
 *
 * Distributed execution metadata remains semantic/compiler/runtime metadata
 * rather than a parser-owned runtime representation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed quantum computation is supported by composition, not by
 * embedding quantum implementation into this grammar.
 *
 * The boundary is:
 *
 *     distributed source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     native gate sets
 *     quantum topology
 *     calibration
 *     pulse semantics
 *     noise channels
 *     QEC algorithms
 *
 * ============================================================================
 * QEC / ZQN / RESILIENCE BOUNDARY
 * ============================================================================
 *
 * Distributed fault-tolerance syntax may express source-level intent through
 * the existing specialized grammar:
 *
 *     fault-tolerance.g4
 *
 * It does not implement:
 *
 *     retry;
 *     restart;
 *     reroute;
 *     remap;
 *     reschedule;
 *     recompile;
 *     quarantine;
 *     abort;
 *     syndrome extraction;
 *     decoding;
 *     correction;
 *     noise simulation.
 *
 * Those remain downstream responsibilities.
 *
 * The established resilience vocabulary remains semantic/runtime data:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * and outcomes:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * They MUST NOT become parser-level implementation decisions.
 *
 * ============================================================================
 * MEMORY / CONCURRENCY / NETWORKING BOUNDARIES
 * ============================================================================
 *
 * Distributed memory remains owned by:
 *
 *     grammar/memory/distributed-memory.g4
 *
 * Generic concurrency remains owned by:
 *
 *     grammar/concurrency/
 *
 * Networking remains owned by:
 *
 *     grammar/networking/
 *
 * This grammar does not duplicate:
 *
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     generic channels;
 *     generic futures;
 *     transport protocols;
 *     network addresses;
 *     sockets;
 *     packet formats.
 *
 * Distributed grammars consume those concepts where their existing contracts
 * explicitly require them.
 *
 * ============================================================================
 * HARDWARE / HDL BOUNDARY
 * ============================================================================
 *
 * Distributed computation may target:
 *
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     heterogeneous systems
 *     future systems
 *
 * but this grammar does not select or enumerate physical hardware.
 *
 * Hardware intent remains owned by:
 *
 *     grammar/hardware/
 *
 * HDL remains owned by:
 *
 *     grammar/hdl/
 *
 * A distributed declaration therefore describes logical computation rather
 * than a physical machine layout.
 *
 * ============================================================================
 * NETWORK BOUNDARY
 * ============================================================================
 *
 * Distributed communication is not transport selection.
 *
 * For example, communication syntax may express:
 *
 *     send
 *     receive
 *     broadcast
 *     scatter
 *     gather
 *     reduce
 *
 * but this grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor-specific transport
 *
 * Networking/runtime layers determine the realization.
 *
 * ============================================================================
 * TOPOLOGY / PLACEMENT BOUNDARY
 * ============================================================================
 *
 * Logical topology and placement intent are distinct from physical realization.
 *
 *     topology
 *         -> logical relationships
 *
 *     placement
 *         -> requirements/preferences/constraints
 *
 *     routing
 *         -> physical/logical path realization
 *
 *     scheduling
 *         -> execution ordering/resource assignment
 *
 *     target realization
 *         -> actual deployment
 *
 * This grammar only composes the source-level topology and placement syntax
 * already owned by:
 *
 *     topology.g4
 *     placement.g4
 *
 * ============================================================================
 * DISTRIBUTED DATA / PARTITIONING BOUNDARY
 * ============================================================================
 *
 * Partitioning remains owned by:
 *
 *     partitioning.g4
 *
 * It describes logical partitioning intent.
 *
 * It does not decide:
 *
 *     - storage engine;
 *     - hash implementation;
 *     - physical shard placement;
 *     - machine assignment;
 *     - network route;
 *     - replication mechanism.
 *
 * ============================================================================
 * DEPLOYMENT BOUNDARY
 * ============================================================================
 *
 * Deployment syntax remains owned by:
 *
 *     deployment.g4
 *
 * This composition root only exposes it through the canonical distributed
 * declaration boundary.
 *
 * It does not perform:
 *
 *     - provider selection;
 *     - resource provisioning;
 *     - machine creation;
 *     - container creation;
 *     - process launch;
 *     - service discovery;
 *     - network configuration.
 *
 * ============================================================================
 * TRANSACTION BOUNDARY
 * ============================================================================
 *
 * Distributed transactions remain owned by:
 *
 *     transactions.g4
 *
 * Transaction algorithms and storage semantics remain downstream.
 *
 * ============================================================================
 * CONTRACT BOUNDARY
 * ============================================================================
 *
 * Distributed contracts remain owned by:
 *
 *     contracts.g4
 *
 * Contract expressions remain ordinary Zamani expressions.
 *
 * The parser does not prove contracts.
 *
 * ============================================================================
 * SOURCE COMPATIBILITY
 * ============================================================================
 *
 * This replacement intentionally preserves the existing public rule:
 *
 *     distributedDeclaration
 *
 * and the existing specialized grammar filenames.
 *
 * It removes duplicated composition-root rules rather than renaming the
 * repository's established distributed files.
 *
 * Existing consumers should migrate only their composition point if they were
 * incorrectly consuming one of the old duplicate root rules.
 *
 * Specialized grammar rules remain available through their owning grammars.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * Generated ANTLR Rust parser artifacts must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * The Zamani compiler/frontend must remain safe Rust.
 *
 * This grammar imposes no requirement for:
 *
 *     unsafe
 *
 * and no downstream implementation may use this grammar as justification for
 * introducing unsafe Rust.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no randomness;
 *     - no environment inspection;
 *     - no hardware inspection;
 *     - no network access;
 *     - no runtime callbacks.
 *
 * Given the same token stream and grammar version, parsing must be
 * deterministic.
 *
 * Distributed source must therefore parse independently of:
 *
 *     - CPU availability;
 *     - GPU availability;
 *     - FPGA availability;
 *     - QPU availability;
 *     - node count;
 *     - network state;
 *     - deployment topology;
 *     - cloud provider;
 *     - runtime state;
 *     - wall-clock time.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve source locations for every composed construct
 * required by diagnostics and tooling.
 *
 * At minimum this includes:
 *
 *     - declaration start/end;
 *     - names;
 *     - qualified names;
 *     - operation names;
 *     - arguments;
 *     - clauses;
 *     - nested bodies;
 *     - relationships;
 *     - dependency edges.
 *
 * This grammar itself does not manufacture source spans; the parser/frontend
 * layer records them from the ANTLR parse tree/token stream.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser errors must remain structural diagnostics.
 *
 * This grammar must not:
 *
 *     - query hardware to decide validity;
 *     - query resource availability;
 *     - contact a network service;
 *     - perform service discovery;
 *     - perform deployment;
 *     - perform semantic capability checks.
 *
 * A syntactically valid declaration may still be semantically invalid or
 * unrealizable.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * All imported grammars use the repository's canonical lexer vocabulary.
 *
 * This root therefore imports only parser grammars.
 *
 * No lexer grammar is defined here.
 *
 * No token vocabulary is redefined here.
 *
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
    Messaging,
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
 * CANONICAL PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Every distributed construct enters through this rule when the parent
 * grammar expects a distributed declaration/construct.
 *
 * The alternatives deliberately delegate ownership to the specialized
 * grammars. No distributed construct is reimplemented here.
 *
 * The ordering follows the repository's domain decomposition rather than
 * hardware scale or implementation preference.
 */
distributedDeclaration
    : nodeDeclaration
    | distributedServiceDeclaration
    | distributedProcessDeclaration
    | distributedActorConstruct
    | distributedChannelDeclaration
    | distributedCommunication
    | messagingConstruct
    | collectiveConstruct
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
 * CANONICAL COMPOSITION ALIAS
 * ============================================================================
 *
 * Internal parent grammars that need a generic "distributed construct"
 * spelling may use this rule.
 *
 * It is intentionally an alias of the single public boundary and does not
 * create another semantic root.
 */
distributedConstruct
    : distributedDeclaration
    ;


/*
 * ============================================================================
 * DISTRIBUTED DECLARATION LIST
 * ============================================================================
 *
 * This is a structural helper, not a competing root.
 *
 * No finite number of declarations is imposed.
 */
distributedDeclarationList
    : distributedDeclaration*
    ;


/*
 * ============================================================================
 * NON-EMPTY DISTRIBUTED DECLARATION LIST
 * ============================================================================
 */
distributedDeclarationListNonEmpty
    : distributedDeclaration+
    ;


/*
 * ============================================================================
 * DISTRIBUTED BLOCK CONTENT
 * ============================================================================
 *
 * This helper is deliberately expressed in terms of the canonical declaration
 * boundary.
 *
 * It does not define a second distributed-member union.
 */
distributedBlockContent
    : distributedDeclaration*
    ;


/*
 * ============================================================================
 * DISTRIBUTED NON-EMPTY BLOCK CONTENT
 * ============================================================================
 */
distributedNonEmptyBlockContent
    : distributedDeclaration+
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This composition root is complete when all of the following remain true:
 *
 * [1] `distributedDeclaration` is the sole canonical distributed entry point.
 *
 * [2] Every distributed feature has exactly one owning specialized grammar.
 *
 * [3] This file contains no duplicate implementation of specialized rules.
 *
 * [4] All specialized grammars consume the canonical Zamani lexer vocabulary.
 *
 * [5] General identifiers and qualified names remain owned by Names.
 *
 * [6] General expressions remain owned by Expressions.
 *
 * [7] General type syntax remains owned by the canonical type grammar.
 *
 * [8] Distributed memory remains owned by memory/distributed-memory.g4.
 *
 * [9] Generic concurrency remains owned by grammar/concurrency/.
 *
 * [10] Networking remains owned by grammar/networking/.
 *
 * [11] Hardware remains owned by grammar/hardware/.
 *
 * [12] HDL remains owned by grammar/hdl/.
 *
 * [13] Quantum semantics continue through `quantum::ir`.
 *
 * [14] No distributed-specific competing IR is created by this grammar.
 *
 * [15] No fixed machine/resource limits are encoded.
 *
 * [16] No physical hardware identifiers are required by the grammar.
 *
 * [17] No network transport is selected by the parser.
 *
 * [18] No placement algorithm is implemented by the parser.
 *
 * [19] No scheduler is implemented by the parser.
 *
 * [20] No deployment operation is implemented by the parser.
 *
 * [21] No QEC implementation is embedded.
 *
 * [22] No ZQN implementation is embedded.
 *
 * [23] No resilience implementation is embedded.
 *
 * [24] No semantic predicates exist.
 *
 * [25] No embedded Rust actions exist.
 *
 * [26] No unsafe Rust is required.
 *
 * [27] The parser is deterministic.
 *
 * [28] Source ordering and source spans can be preserved.
 *
 * [29] Existing specialized grammar ownership remains intact.
 *
 * [30] Future distributed concepts can be introduced through existing
 *      extensibility mechanisms without automatically expanding the lexer.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file is intentionally small in semantic responsibility.
 *
 * It is a COMPOSITION ROOT, not a distributed-computing implementation.
 *
 * Therefore:
 *
 *     distributed/distributed.g4
 *             =
 *     composition
 *
 * while:
 *
 *     distributed/nodes.g4
 *             =
 *     node syntax
 *
 *     distributed/services.g4
 *             =
 *     service syntax
 *
 *     distributed/processes.g4
 *             =
 *     process syntax
 *
 *     distributed/actors.g4
 *             =
 *     actor syntax
 *
 *     distributed/channels.g4
 *             =
 *     channel syntax
 *
 *     distributed/communication.g4
 *             =
 *     communication syntax
 *
 *     distributed/messaging.g4
 *             =
 *     messaging syntax
 *
 *     distributed/collective.g4
 *             =
 *     collective syntax
 *
 *     distributed/replication.g4
 *             =
 *     replication syntax
 *
 *     distributed/consistency.g4
 *             =
 *     consistency syntax
 *
 *     distributed/placement.g4
 *             =
 *     placement syntax
 *
 *     distributed/partitioning.g4
 *             =
 *     partitioning syntax
 *
 *     distributed/topology.g4
 *             =
 *     topology syntax
 *
 *     distributed/remote-execution.g4
 *             =
 *     remote execution syntax
 *
 *     distributed/deployment.g4
 *             =
 *     distributed deployment composition
 *
 *     distributed/fault-tolerance.g4
 *             =
 *     fault-tolerance syntax
 *
 *     distributed/contracts.g4
 *             =
 *     distributed contract syntax
 *
 *     distributed/transactions.g4
 *             =
 *     distributed transaction syntax
 *
 * The whole distributed domain then remains one source-level language
 * surface, while semantic realization is performed downstream.
 *
 * ============================================================================
 * POCO-REAF FINAL FORM
 * ============================================================================
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Semantic / capability / resource analysis
 *          |
 *          v
 *     Target-independent lowering
 *          |
 *          +--> local
 *          +--> multicore
 *          +--> many-core
 *          +--> distributed
 *          +--> HPC
 *          +--> cloud
 *          +--> edge
 *          +--> CPU/GPU/FPGA/ASIC
 *          +--> quantum-classical
 *          +--> distributed quantum
 *          +--> future substrates
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Run Forever
 *
 * subject to the actual program semantics, implementation support and
 * resources available at realization time.
 *
 * ============================================================================
 */