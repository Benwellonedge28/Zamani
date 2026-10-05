/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/effects/distributed.g4
 *
 * GRAMMAR
 * -------
 * DistributedEffects
 *
 * STATUS
 * ------
 * PRODUCTION DISTRIBUTED-EFFECT DOMAIN BOUNDARY
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * ANTLR
 * -----
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar:
 *
 *     - contains no embedded Rust;
 *     - contains no semantic predicates;
 *     - contains no actions;
 *     - contains no unsafe code;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no runtime execution;
 *     - performs no randomness;
 *     - performs no target selection.
 *
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file provides the parser-level boundary for identifying a distributed
 * effect reference without creating a second effect language.
 *
 * The generic effect subsystem already owns generic effect references through:
 *
 *     grammar/effects/effect-sets.g4
 *
 * whose canonical rule is:
 *
 *     effectReference
 *         : qualifiedName
 *         ;
 *
 * Therefore this file MUST NOT redefine:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectOperationReference
 *     effectInvocation
 *     effectDeclaration
 *
 * Instead, this grammar provides an explicit distributed-domain view of an
 * already-valid symbolic qualified name.
 *
 *
 * ============================================================================
 * 2. ARCHITECTURAL ROLE
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical Zamani lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     qualifiedName
 *       |
 *       +------------------------------+
 *       |                              |
 *       v                              v
 * generic effect semantics       distributed-effect
 *                                 classification
 *       |                              |
 *       +---------------+--------------+
 *                       |
 *                       v
 *                domain-neutral AST
 *                       |
 *                       v
 *                semantic analysis
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      effects     capabilities    resources
 *                       |
 *                       v
 *              distributed semantics
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      concurrency   networking    distributed
 *                                  execution
 *                       |
 *                       v
 *               canonical semantic
 *                  representation
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      classical    quantum::ir     HDL/hardware
 *                       |
 *                       v
 *              optimization/lowering
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *       routing     scheduling    resilience
 *                       |
 *                       v
 *                     HAL
 *                       |
 *                       v
 *                target realization
 *
 *
 * ============================================================================
 * 3. THIS FILE OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     distributedEffectReference
 *     distributedEffectQualifiedReference
 *     distributedEffectNamespace
 *     distributedEffectMemberPath
 *     distributedEffectMember
 *
 * These rules provide an explicit parser-tree boundary for downstream
 * consumers that need to distinguish a distributed effect from an arbitrary
 * qualified name.
 *
 *
 * ============================================================================
 * 4. THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identifiers
 *     qualified names
 *     effect references
 *     effect sets
 *     effect declarations
 *     effect operations
 *     effect invocation
 *     effect handling
 *     effect polymorphism
 *     effect composition
 *
 *     nodes
 *     processes
 *     services
 *     actors
 *     channels
 *     messages
 *     communication
 *     collectives
 *     replication
 *     consistency
 *     placement
 *     partitioning
 *     topology
 *     remote execution
 *     deployment
 *     fault tolerance
 *     transactions
 *
 *     capabilities
 *     resources
 *     requirements
 *     constraints
 *     preferences
 *     policies
 *     scheduling
 *     routing
 *
 *     network protocols
 *     transport selection
 *     addresses
 *     ports
 *
 *     quantum operations
 *     qubits
 *     physical qubits
 *     topology
 *     calibration
 *     noise
 *     QEC
 *     ZQN
 *
 *     hardware selection
 *     backend selection
 *     runtime execution
 *
 * Those concerns remain owned by their respective repository subsystems.
 *
 *
 * ============================================================================
 * 5. CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * Generic effect references are already defined by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * Specifically:
 *
 *     effectReference
 *         : qualifiedName
 *         ;
 *
 * Therefore:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::consensus
 *     distributed::replication
 *     distributed::migration
 *
 * are already syntactically valid generic effect references.
 *
 * This file MUST NOT replace that rule.
 *
 * It also MUST NOT create a second generic effect-set implementation.
 *
 * In particular, do NOT add:
 *
 *     distributedEffectSet
 *     distributedEffectReferenceList
 *     distributedEffectOperation
 *
 * merely to duplicate generic effect facilities.
 *
 *
 * ============================================================================
 * 6. OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Distributed effect names are open-world symbolic names.
 *
 * Examples:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::collective
 *     distributed::consensus
 *     distributed::replication
 *     distributed::coordination
 *     distributed::migration
 *     distributed::remote_execution
 *     distributed::federation
 *     distributed::region
 *     distributed::shard
 *     distributed::replica
 *     distributed::quantum_network
 *     distributed::edge_region
 *     distributed::future_protocol
 *     distributed::vendor::extension
 *
 * The grammar MUST NOT enumerate these names.
 *
 * Adding a new distributed effect identity therefore does not require editing
 * this grammar.
 *
 * For example, the following must remain possible without grammar changes:
 *
 *     distributed::new_transport
 *     distributed::future_fabric
 *     distributed::photonic_link
 *     distributed::neuromorphic_region
 *     distributed::federated_consensus
 *
 * Whether such an identity is known, supported, experimental, deprecated,
 * vendor-defined, or invalid is a semantic concern.
 *
 *
 * ============================================================================
 * 7. NAMESPACE POLICY
 * ============================================================================
 *
 * The distributed namespace is represented structurally as an identifier.
 *
 * The grammar intentionally does NOT introduce a DISTRIBUTED lexer token.
 *
 * The semantic layer determines whether the first segment has the canonical
 * spelling:
 *
 *     distributed
 *
 * This preserves the open-world namespace architecture and prevents the
 * distributed subsystem from becoming dependent on a growing keyword list.
 *
 * Therefore:
 *
 *     distributed::send
 *
 * is structurally recognized as a qualified symbolic reference whose first
 * segment is classified semantically as the distributed namespace.
 *
 * The parser does not need to hard-code a finite list of distributed effects.
 *
 *
 * ============================================================================
 * 8. NAME AUTHORITY
 * ============================================================================
 *
 * Canonical name syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * Grammar:
 *
 *     Names
 *
 * This file therefore imports and consumes:
 *
 *     identifier
 *     qualifiedName
 *
 * rather than defining another identifier system.
 *
 * This prevents divergence between:
 *
 *     distributed names
 *     quantum names
 *     hardware names
 *     AI names
 *     networking names
 *     effect names
 *     module names
 *     resource names
 *
 *
 * ============================================================================
 * 9. LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore:
 *
 *     - defines no lexer rules;
 *     - defines no keywords;
 *     - defines no identifiers;
 *     - defines no punctuation;
 *     - defines no operators;
 *     - defines no literals.
 *
 * All tokens are consumed from:
 *
 *     ZamaniLexer
 *
 *
 * ============================================================================
 * 10. EFFECT AUTHORITY
 * ============================================================================
 *
 * Generic effect syntax remains owned by:
 *
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effect-types.g4
 *     grammar/effects/effect-polymorphism.g4
 *     grammar/effects/effect-composition.g4
 *     grammar/effects/custom-effects.g4
 *
 * This file consumes the naming model required to classify a distributed
 * effect but does not replace any of those authorities.
 *
 *
 * ============================================================================
 * 11. DISTRIBUTED DOMAIN AUTHORITY
 * ============================================================================
 *
 * Distributed computation itself remains owned by:
 *
 *     grammar/distributed/
 *
 * including, where applicable:
 *
 *     distributed.g4
 *     nodes.g4
 *     services.g4
 *     processes.g4
 *     actors.g4
 *     channels.g4
 *     communication.g4
 *     messaging.g4
 *     collective.g4
 *     replication.g4
 *     consistency.g4
 *     placement.g4
 *     partitioning.g4
 *     topology.g4
 *     remote-execution.g4
 *     deployment.g4
 *     fault-tolerance.g4
 *     contracts.g4
 *     transactions.g4
 *
 * This file MUST NOT duplicate those constructs.
 *
 *
 * ============================================================================
 * 12. RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * A distributed effect describes computational intent.
 *
 * It does not select the resources that realize the intent.
 *
 * For example:
 *
 *     distributed::send
 *
 * does NOT identify:
 *
 *     a particular machine;
 *     a particular node;
 *     a particular process;
 *     a particular network interface;
 *     a particular address;
 *     a particular port;
 *     a particular transport;
 *     a particular cloud provider;
 *     a particular topology.
 *
 * A semantic effect may subsequently imply requirements or capabilities such
 * as:
 *
 *     capability("distributed.communication")
 *
 * or:
 *
 *     capability("network.message")
 *
 * but this grammar does not define those mappings.
 *
 * Capability semantics remain owned by the capability subsystem.
 *
 * Resource semantics remain owned by the resource subsystem.
 *
 *
 * ============================================================================
 * 13. EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * These concepts MUST remain distinct:
 *
 *     effect
 *         =
 *     computational interaction expressed by the program
 *
 *     capability
 *         =
 *     ability offered by a realization
 *
 *     resource
 *         =
 *     consumable/allocatable/available realization property
 *
 *     requirement
 *         =
 *     condition the program requires
 *
 *     constraint
 *         =
 *     condition limiting valid realizations
 *
 *     preference
 *         =
 *     non-mandatory realization preference
 *
 * The grammar must not collapse these concepts.
 *
 *
 * ============================================================================
 * 14. POCO-REAF CONTRACT
 * ============================================================================
 *
 * A distributed effect is portable source intent.
 *
 * The same source must be capable of participating in realizations involving:
 *
 *     one execution resource;
 *     many cores;
 *     many threads;
 *     many processes;
 *     many nodes;
 *     clusters;
 *     HPC systems;
 *     supercomputers;
 *     edge systems;
 *     cloud systems;
 *     federated systems;
 *     heterogeneous CPU/GPU/FPGA/ASIC systems;
 *     accelerator systems;
 *     quantum-classical systems;
 *     distributed quantum systems;
 *     future computational substrates.
 *
 * Source syntax must not need to change merely because the realization scale
 * changes.
 *
 *
 * ============================================================================
 * 15. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO language-level capacity limit in this grammar.
 *
 * In particular, this file MUST NOT define or depend on:
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
 *     fixed node IDs;
 *     fixed process counts;
 *     fixed device counts;
 *     fixed machine counts;
 *     fixed addresses;
 *     fixed ports;
 *     fixed topology sizes;
 *     fixed namespace depth;
 *     fixed effect count.
 *
 * Repetition is expressed using ANTLR repetition operators.
 *
 * Practical limits imposed by:
 *
 *     parser memory;
 *     compiler memory;
 *     source storage;
 *     token-stream size;
 *     recursion depth;
 *     execution time;
 *     deployment capacity;
 *
 * are implementation/resource constraints and are not language semantics.
 *
 *
 * ============================================================================
 * 16. DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only upon:
 *
 *     source text;
 *     lexer specification;
 *     parser specification;
 *     selected language/grammar configuration.
 *
 * Parsing MUST NOT depend upon:
 *
 *     hardware availability;
 *     node count;
 *     network state;
 *     runtime state;
 *     scheduler state;
 *     filesystem state;
 *     environment state;
 *     wall-clock time;
 *     random state;
 *     backend enumeration.
 *
 *
 * ============================================================================
 * 17. AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no Rust AST implementation.
 *
 * A consumer may preserve:
 *
 *     namespace;
 *     member path;
 *     source spelling;
 *     source span;
 *     qualified-name ordering.
 *
 * The semantic AST must remain domain-neutral.
 *
 * A distributed effect must NOT automatically become a backend-specific node
 * such as:
 *
 *     CpuSend
 *     GpuSend
 *     QpuSend
 *     CloudSend
 *     TcpSend
 *     RdmaSend
 *
 * merely because it is distributed.
 *
 *
 * ============================================================================
 * 18. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     namespace classification;
 *     name resolution;
 *     effect declaration lookup;
 *     effect existence;
 *     effect compatibility;
 *     effect inference;
 *     capability mapping;
 *     resource consequences;
 *     policy validation;
 *     security validation;
 *     portability analysis;
 *     target realization analysis.
 *
 * The semantic layer may classify:
 *
 *     distributed::send
 *
 * as a distributed effect.
 *
 * This file itself does not determine whether the effect is:
 *
 *     known;
 *     implemented;
 *     authorized;
 *     available;
 *     realizable;
 *     executable.
 *
 *
 * ============================================================================
 * 19. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * It MUST NOT create:
 *
 *     DistributedIR
 *     DistributedEffectIR
 *     DistributedNodeIR
 *     DistributedNetworkIR
 *
 * merely because a source name begins with:
 *
 *     distributed::
 *
 * Distributed semantics are lowered through the repository's established
 * canonical semantic/IR architecture.
 *
 * Classical computation continues through the canonical classical path.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * HDL/hardware computation continues through its established representation.
 *
 *
 * ============================================================================
 * 20. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A distributed effect may ultimately participate in distributed quantum
 * computation.
 *
 * This grammar MUST NOT encode:
 *
 *     physical qubits;
 *     logical qubits;
 *     QPU identifiers;
 *     gate sets;
 *     coupling maps;
 *     calibration;
 *     pulse schedules;
 *     noise channels;
 *     QEC codes;
 *     routing decisions.
 *
 * The downstream path remains:
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
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> resilience
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 *
 * ============================================================================
 * 21. CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Distributed effects may interact with:
 *
 *     tasks;
 *     futures;
 *     actors;
 *     channels;
 *     synchronization;
 *     asynchronous execution;
 *     parallel execution.
 *
 * However, this file does not own any of those syntaxes.
 *
 * Generic concurrency remains under:
 *
 *     grammar/concurrency/
 *
 * Distributed execution remains under:
 *
 *     grammar/distributed/
 *
 * The semantic layer joins them.
 *
 *
 * ============================================================================
 * 22. NETWORKING INTEGRATION
 * ============================================================================
 *
 * A distributed effect may imply network-related semantics.
 *
 * For example:
 *
 *     distributed::send
 *
 * may ultimately require a network capability.
 *
 * This grammar does not define:
 *
 *     protocol;
 *     endpoint;
 *     address;
 *     port;
 *     transport;
 *     packet;
 *     socket;
 *     link.
 *
 * Those belong to:
 *
 *     grammar/networking/
 *
 * and the downstream networking implementation.
 *
 *
 * ============================================================================
 * 23. SECURITY INTEGRATION
 * ============================================================================
 *
 * Distributed effects may be subject to:
 *
 *     capabilities;
 *     authorization;
 *     trust;
 *     sandbox policies;
 *     data policies;
 *     network policies;
 *     provenance;
 *     audit requirements.
 *
 * Security syntax remains owned by:
 *
 *     grammar/security/
 *
 * This grammar does not embed security policy semantics.
 *
 *
 * ============================================================================
 * 24. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Semantic consumers may attach provenance to distributed effects.
 *
 * Provenance may record:
 *
 *     source;
 *     declaration;
 *     derivation;
 *     transformation;
 *     evidence;
 *     policy;
 *     decision;
 *     compilation stage.
 *
 * This grammar preserves the symbolic identity and source location needed by
 * those downstream systems.
 *
 * It does not define the provenance model itself.
 *
 *
 * ============================================================================
 * 25. POLICY INTEGRATION
 * ============================================================================
 *
 * A distributed effect may be constrained by policies such as:
 *
 *     placement policy;
 *     communication policy;
 *     security policy;
 *     resource policy;
 *     resilience policy;
 *     deployment policy;
 *     adaptation policy.
 *
 * Policy interpretation remains downstream.
 *
 * The grammar must not encode provider-specific or machine-specific policy
 * choices.
 *
 *
 * ============================================================================
 * 26. RULE DEPENDENCY CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     grammar/core/names.g4
 *             |
 *             v
 *     DistributedEffects
 *
 *
 * DIRECT IMPORTS
 * --------------
 *
 *     Names
 *
 *
 * PUBLIC RULES
 * ------------
 *
 *     distributedEffectReference
 *     distributedEffectQualifiedReference
 *     distributedEffectNamespace
 *     distributedEffectMemberPath
 *     distributedEffectMember
 *
 *
 * INTERNAL RULES
 * --------------
 *
 * None.
 *
 * All rules are intentionally explicit because downstream parser composition
 * may require a stable named boundary.
 *
 *
 * ============================================================================
 * 27. DOWNSTREAM CONSUMERS
 * ============================================================================
 *
 * Potential consumers include:
 *
 *     grammar/distributed/
 *     grammar/concurrency/
 *     grammar/networking/
 *     grammar/security/
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/spec/
 *     compiler semantic analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     distributed planning
 *     provenance tooling
 *     diagnostics tooling
 *
 * A consumer MUST import this grammar only when it actually needs the
 * distributed-effect-specific parse boundary.
 *
 *
 * ============================================================================
 * 28. IMPORTANT NON-CYCLIC INTEGRATION RULE
 * ============================================================================
 *
 * `grammar/effects/effects.g4` is the generic effect composition root.
 *
 * It MUST NOT import this file merely to make distributed effects legal.
 *
 * Distributed effects are already legal through:
 *
 *     EffectSets
 *         |
 *         v
 *     effectReference
 *         |
 *         v
 *     qualifiedName
 *
 * Therefore:
 *
 *     effects { distributed::consensus }
 *
 * requires no special generic-effect grammar branch.
 *
 * This prevents:
 *
 *     Effects
 *        |
 *        v
 * DistributedEffects
 *        |
 *        v
 * Effects
 *
 * style dependency cycles.
 *
 *
 * ============================================================================
 * 29. EXISTING EFFECT INTEGRATION
 * ============================================================================
 *
 * The generic effect grammar remains:
 *
 *     grammar/effects/effects.g4
 *
 * Its existing composition remains authoritative.
 *
 * The generic effect-set rule remains:
 *
 *     effectReference
 *         : qualifiedName
 *         ;
 *
 * Therefore:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::consensus
 *     distributed::replication
 *
 * are already accepted as generic effect identities.
 *
 * This file adds an explicit distributed-domain boundary without changing
 * generic effect ownership.
 *
 *
 * ============================================================================
 * 30. EXISTING DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed computation remains owned by:
 *
 *     grammar/distributed/distributed.g4
 *
 * That composition root owns the distributed declaration boundary.
 *
 * It must continue to compose:
 *
 *     Nodes
 *     Services
 *     Processes
 *     Actors
 *     Channels
 *     Communication
 *     Messaging
 *     Collective
 *     Replication
 *     Consistency
 *     Placement
 *     Partitioning
 *     Topology
 *     RemoteExecution
 *     Deployment
 *     FaultTolerance
 *     Contracts
 *     Transactions
 *
 * This file MUST NOT import the distributed composition root merely to obtain
 * distributed names.
 *
 * That would reverse ownership.
 *
 *
 * ============================================================================
 * 31. NO CROSS-DOMAIN KEYWORD EXPLOSION
 * ============================================================================
 *
 * This grammar intentionally does not introduce keywords for:
 *
 *     send;
 *     receive;
 *     broadcast;
 *     consensus;
 *     replication;
 *     migration;
 *     federation;
 *     shard;
 *     replica;
 *     node;
 *     region;
 *     topology;
 *     cluster;
 *     cloud;
 *     edge;
 *     QPU;
 *     GPU;
 *     FPGA.
 *
 * These are semantic names unless another authoritative grammar explicitly
 * requires them as language syntax.
 *
 *
 * ============================================================================
 * 32. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics may report:
 *
 *     missing distributed namespace segment;
 *     missing qualification separator;
 *     missing effect member;
 *     malformed qualified reference.
 *
 * Semantic diagnostics belong downstream and may distinguish:
 *
 *     unknown distributed effect;
 *     unresolved distributed effect;
 *     inaccessible distributed effect;
 *     unsupported distributed effect;
 *     unavailable capability;
 *     insufficient resources;
 *     policy violation;
 *     target realization failure.
 *
 * These must not be collapsed into parser errors.
 *
 *
 * ============================================================================
 * 33. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite should accept:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::consensus
 *     distributed::replication
 *     distributed::coordination
 *     distributed::migration
 *     distributed::remote_execution
 *     distributed::federation
 *     distributed::region
 *     distributed::shard
 *     distributed::replica
 *     distributed::quantum_network
 *     distributed::future::operation
 *     distributed::vendor::extension
 *
 * Deep qualification should remain valid:
 *
 *     distributed::domain::subdomain::operation
 *
 * without a grammar-defined maximum depth.
 *
 *
 * ============================================================================
 * 34. NEGATIVE / SEMANTIC TEST CONTRACT
 * ============================================================================
 *
 * The parser-level tests should reject malformed structures such as:
 *
 *     ::
 *     distributed::
 *     ::send
 *     distributed:::send
 *     distributed::::send
 *
 * A structurally valid qualified name whose first segment is not:
 *
 *     distributed
 *
 * is NOT necessarily a parser error.
 *
 * For example:
 *
 *     networking::send
 *
 * is a valid qualified name.
 *
 * It simply is not classified as a distributed effect by the semantic layer.
 *
 * This distinction is mandatory.
 *
 *
 * ============================================================================
 * 35. BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     one member;
 *     two members;
 *     deeply qualified members;
 *     long identifiers;
 *     Unicode identifiers where supported;
 *     vendor namespaces;
 *     extension namespaces;
 *     generated names;
 *     macro-generated names;
 *     imported names;
 *     aliases resolved downstream.
 *
 * No test may establish a universal maximum.
 *
 *
 * ============================================================================
 * 36. SCALABILITY TESTS
 * ============================================================================
 *
 * The grammar must remain correct as source programs contain increasingly:
 *
 *     many distributed effects;
 *     many effect-set entries;
 *     many namespaces;
 *     many qualified segments;
 *     many modules;
 *     many distributed declarations;
 *     many nodes;
 *     many processes;
 *     many actors;
 *     many channels;
 *     many services.
 *
 * The test suite must measure implementation behavior rather than turn a
 * fixture size into a language limit.
 *
 *
 * ============================================================================
 * 37. DETERMINISM TESTS
 * ============================================================================
 *
 * Identical source and identical grammar configuration must produce:
 *
 *     identical token interpretation;
 *     identical parse structure;
 *     identical source spans.
 *
 * Results must not depend on:
 *
 *     node availability;
 *     CPU count;
 *     GPU count;
 *     network state;
 *     scheduler state;
 *     target selection.
 *
 *
 * ============================================================================
 * 38. POCO-REAF TEST
 * ============================================================================
 *
 * A source-level effect such as:
 *
 *     distributed::send
 *
 * must remain unchanged when the eventual realization changes between:
 *
 *     single-process;
 *     multicore;
 *     multiprocess;
 *     cluster;
 *     HPC;
 *     cloud;
 *     edge;
 *     heterogeneous;
 *     quantum-classical;
 *     future target.
 *
 * Only downstream semantic/resource/capability/target decisions may vary.
 *
 *
 * ============================================================================
 * 39. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 * No:
 *
 *     machine enumeration;
 *     provider enumeration;
 *     topology enumeration;
 *     node enumeration;
 *     transport enumeration;
 *     device enumeration;
 *     capacity constants;
 *     fixed namespace depth;
 *     fixed effect count;
 *     physical identifiers;
 *     backend selection.
 *
 * In particular, this file must never contain:
 *
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 *
 * ============================================================================
 * 40. RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no Rust implementation code.
 *
 * Generated parser/frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The implementation must use safe Rust.
 *
 * No:
 *
 *     unsafe blocks;
 *     unsafe functions;
 *     unsafe traits;
 *     foreign execution;
 *
 * are required by this grammar.
 *
 *
 * ============================================================================
 * 41. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] canonical ZamaniLexer is the token vocabulary;
 *     [x] canonical Names grammar owns identifier syntax;
 *     [x] generic effect references remain owned by EffectSets;
 *     [x] distributed effect classification has a named parse boundary;
 *     [x] no distributed keyword is required;
 *     [x] distributed effect names remain open-world;
 *     [x] arbitrary qualified-name depth remains supported;
 *     [x] no machine capacity is encoded;
 *     [x] no topology is encoded;
 *     [x] no physical node is encoded;
 *     [x] no network endpoint is encoded;
 *     [x] no transport is encoded;
 *     [x] no hardware is selected;
 *     [x] no quantum hardware is selected;
 *     [x] no QEC implementation is selected;
 *     [x] no ZQN implementation is selected;
 *     [x] no IR is created;
 *     [x] quantum::ir remains the quantum semantic boundary;
 *     [x] distributed/distributed.g4 remains the distributed domain root;
 *     [x] effects/effects.g4 remains the generic effect root;
 *     [x] no grammar dependency cycle is introduced;
 *     [x] parser and semantic responsibilities remain separated;
 *     [x] diagnostics remain downstream;
 *     [x] provenance remains downstream;
 *     [x] resource analysis remains downstream;
 *     [x] capability analysis remains downstream;
 *     [x] deterministic parsing is preserved;
 *     [x] Rust 1.97/1.97.1 compatibility is preserved;
 *     [x] no unsafe Rust is required.
 *
 *
 * ============================================================================
 * 42. FINAL INVARIANT
 * ============================================================================
 *
 * This grammar expresses:
 *
 *     WHAT KIND OF EFFECT IS BEING REFERENCED
 *
 * It does NOT express:
 *
 *     WHERE IT RUNS
 *     HOW IT RUNS
 *     WHICH MACHINE RUNS IT
 *     WHICH NODE RUNS IT
 *     WHICH NETWORK CARRIES IT
 *     WHICH DEVICE REALIZES IT
 *     WHICH QPU REALIZES IT
 *     WHICH CPU REALIZES IT
 *     WHICH GPU REALIZES IT
 *     WHICH FPGA REALIZES IT
 *     WHICH ASIC REALIZES IT
 *     WHICH TOPOLOGY IS USED
 *     WHICH ROUTE IS USED
 *     WHICH SCHEDULE IS USED
 *     WHICH QEC CODE IS USED
 *     WHICH CALIBRATION IS USED
 *
 * The separation is mandatory for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */

parser grammar DistributedEffects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 43. DISTRIBUTED EFFECT REFERENCE
 * ============================================================================
 *
 * This is the primary public rule.
 *
 * It is intentionally based on the canonical qualified-name syntax rather
 * than duplicating that syntax.
 *
 * Examples:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::consensus
 *     distributed::consensus::proposal
 *
 * Semantic analysis establishes whether the first name segment is the
 * canonical distributed namespace.
 */
distributedEffectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 44. DISTRIBUTED QUALIFIED REFERENCE
 * ============================================================================
 *
 * Explicit parser boundary for consumers that need the distributed namespace
 * and member path as separate parse-tree concepts.
 *
 * This rule is intentionally equivalent in accepted structure to a canonical
 * qualified name with at least two segments.
 *
 * The semantic layer validates the namespace identity.
 */
distributedEffectQualifiedReference
    : distributedEffectNamespace
      DOUBLE_COLON
      distributedEffectMemberPath
    ;


/*
 * ============================================================================
 * 45. DISTRIBUTED NAMESPACE
 * ============================================================================
 *
 * Canonical semantic spelling:
 *
 *     distributed
 *
 * The grammar does not encode that spelling as a lexer keyword.
 *
 * Semantic analysis compares the resolved canonical name.
 */
distributedEffectNamespace
    : identifier
    ;


/*
 * ============================================================================
 * 46. DISTRIBUTED EFFECT MEMBER PATH
 * ============================================================================
 *
 * One or more canonical identifier segments after the namespace.
 *
 * Examples:
 *
 *     send
 *     consensus
 *     consensus::proposal
 *     vendor::extension::operation
 *
 * There is deliberately no finite depth.
 */
distributedEffectMemberPath
    : distributedEffectMember
      (
          DOUBLE_COLON
          distributedEffectMember
      )*
    ;


/*
 * ============================================================================
 * 47. DISTRIBUTED EFFECT MEMBER
 * ============================================================================
 *
 * Member names are ordinary language identifiers.
 *
 * No distributed-effect catalog is encoded here.
 */
distributedEffectMember
    : identifier
    ;


/*
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */