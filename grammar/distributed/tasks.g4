/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/distributed/tasks.g4
 *
 * GRAMMAR
 * -------
 * DistributedTasks
 *
 * STATUS
 * ------
 * PRODUCTION DISTRIBUTED-TASK INTEGRATION GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021 edition
 * Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical PARSER-LEVEL DISTRIBUTED TASK ADAPTER.
 *
 * IMPORTANT:
 *
 * This file does NOT create a second task language.
 *
 * Ordinary task syntax is owned by:
 *
 *     grammar/concurrency/tasks.g4
 *
 * Asynchronous expression syntax is owned by:
 *
 *     grammar/expressions/async.g4
 *
 * Generic distributed computation is owned by:
 *
 *     grammar/distributed/distributed.g4
 *
 * Distributed/concurrency composition is owned by:
 *
 *     grammar/concurrency/distributed-concurrency.g4
 *
 * This file exists to provide a stable semantic/parser boundary for tasks
 * whose execution MAY be realized through distributed resources.
 *
 * A distributed task is a LOGICAL COMPUTATION.
 *
 * It is NOT inherently:
 *
 *     - a process;
 *     - a thread;
 *     - a worker;
 *     - an actor;
 *     - a node;
 *     - a container;
 *     - a machine;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - an accelerator;
 *     - a QPU;
 *     - a network endpoint;
 *     - a cloud instance.
 *
 * Those are possible realization mechanisms determined downstream.
 *
 * ============================================================================
 * CORE ARCHITECTURE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     concurrency/tasks.g4       distributed/tasks.g4
 *          |                               |
 *          |                       distributed adapter
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                  domain-neutral AST
 *                          |
 *                          v
 *                 semantic analysis
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *       effects       capabilities       resources
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                   distributed plan
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *         classical    quantum::ir     HDL/hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                  optimization
 *                          |
 *                 routing / placement
 *                          |
 *                      scheduling
 *                          |
 *                 resilience / recovery
 *                          |
 *                         ZQN
 *                          |
 *                         HAL
 *                          |
 *                    target/runtime
 *
 * This file participates only in the parser/source-structure boundary.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     distributedTask
 *     distributedTaskConstruct
 *     distributedTaskExpression
 *     distributedTaskStatement
 *     distributedTaskConcurrencyExpression
 *     distributedTaskConcurrencyStatement
 *
 * These are DISTRIBUTED-DOMAIN ADAPTERS.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     task syntax;
 *     spawn syntax;
 *     await syntax;
 *     parallel syntax;
 *     async syntax;
 *     futures;
 *     actors;
 *     channels;
 *     messages;
 *     services;
 *     processes;
 *     placement;
 *     topology;
 *     deployment;
 *     resource syntax;
 *     capability syntax;
 *     effect syntax;
 *     contracts;
 *     policies;
 *     networking;
 *     quantum operations;
 *     quantum::ir;
 *     HDL;
 *     hardware realization;
 *     scheduling;
 *     routing;
 *     resilience;
 *     runtime execution.
 *
 * Those responsibilities remain with their canonical owners.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one concrete syntax owner for each construct.
 *
 * Canonical ownership:
 *
 *     taskExpression
 *         -> grammar/concurrency/tasks.g4
 *
 *     taskStatement
 *         -> grammar/concurrency/tasks.g4
 *
 *     spawnExpression
 *         -> grammar/expressions/async.g4
 *
 *     awaitExpression
 *         -> grammar/expressions/async.g4
 *
 *     parallelExpression
 *         -> grammar/expressions/async.g4
 *
 *     distributedTask
 *         -> THIS FILE
 *           as a DISTRIBUTED ADAPTER ONLY
 *
 * Therefore this file MUST NOT redefine:
 *
 *     SPAWN expression
 *     AWAIT expression
 *     PARALLEL expression
 *     task bodies
 *     async expression syntax
 *     future syntax.
 *
 * ============================================================================
 * WHY A DISTRIBUTED TASK ADAPTER EXISTS
 * ============================================================================
 *
 * Distributed computation needs a stable parser boundary for tooling,
 * semantic analysis and composition.
 *
 * However, creating another task syntax would produce competing languages:
 *
 *     task(...)
 *     distributed task(...)
 *     remote task(...)
 *     network task(...)
 *
 * with overlapping semantics.
 *
 * That architecture is prohibited.
 *
 * Instead:
 *
 *     ordinary task syntax
 *             |
 *             v
 *        task semantics
 *             |
 *             v
 *     distributed classification
 *
 * A task may become distributed because of:
 *
 *     - source context;
 *     - capability requirements;
 *     - resource requirements;
 *     - policy;
 *     - placement intent;
 *     - communication dependencies;
 *     - deployment configuration;
 *     - compiler decisions;
 *     - target capabilities.
 *
 * The source task syntax itself does not need to encode physical placement.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Distributed task syntax describes logical computation.
 *
 * The same source must remain usable when realized using:
 *
 *     - a tiny embedded target;
 *     - a single execution context;
 *     - multiple CPU cores;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - QPUs;
 *     - quantum simulators;
 *     - heterogeneous systems;
 *     - multiple processes;
 *     - multiple machines;
 *     - clusters;
 *     - HPC systems;
 *     - cloud infrastructure;
 *     - federated infrastructure;
 *     - future computational substrates.
 *
 * This grammar MUST NOT encode universal capacity limits.
 *
 * In particular, it MUST NOT define:
 *
 *     MAX_TASKS
 *     MAX_DISTRIBUTED_TASKS
 *     MAX_CONCURRENT_TASKS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_DEVICES
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_MEMORY
 *     MAX_BANDWIDTH
 *     MAX_TASK_DEPTH
 *
 * Repetition in ANTLR does not constitute a language capacity.
 *
 * Practical parser/compiler/runtime limits are implementation constraints,
 * not language semantics.
 *
 * ============================================================================
 * NO PHYSICAL RESOURCE ENCODING
 * ============================================================================
 *
 * This file MUST NOT introduce universal syntax such as:
 *
 *     run_on_cpu_0
 *     run_on_gpu_0
 *     run_on_node_0
 *     use_worker_0
 *     use_thread_0
 *     use_device_0
 *
 * Nor may it encode:
 *
 *     fixed worker counts;
 *     fixed machine counts;
 *     fixed topology sizes;
 *     fixed hardware identifiers;
 *     physical memory addresses;
 *     vendor-specific execution units.
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A distributed task may participate in the universal resource model.
 *
 * Conceptually:
 *
 *     requires capability("distributed.communication")
 *
 *     requires capability("parallel.compute")
 *
 *     requires memory >= required_memory
 *
 *     requires topology(required_topology)
 *
 * Such requirements are NOT interpreted by this parser.
 *
 * They are consumed by:
 *
 *     semantic analysis
 *          ->
 *     capability negotiation
 *          ->
 *     resource analysis
 *          ->
 *     placement
 *          ->
 *     scheduling
 *          ->
 *     target realization
 *
 * This file does not duplicate resource grammar.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Distributed tasks may participate in effects including:
 *
 *     distributed
 *     network
 *     mutation
 *     io
 *     randomness
 *     native
 *     foreign
 *     measurement
 *     simulation
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *
 * Effect ownership remains in:
 *
 *     grammar/effects/
 *
 * This file does not define effect syntax.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Distributed task realization may be constrained by:
 *
 *     security policies;
 *     resource policies;
 *     execution policies;
 *     deployment policies;
 *     adaptation policies;
 *     simulation policies.
 *
 * This grammar does not define those policies.
 *
 * Policy ownership remains in the canonical policy/security grammar.
 *
 * A syntactically valid distributed task does not imply authorization.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Contracts remain ordinary Zamani semantic constructs.
 *
 * A distributed task may therefore participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but this file does not redefine those constructs.
 *
 * Contract verification belongs downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * A distributed task may carry semantic provenance through the normal
 * frontend/semantic model.
 *
 * Provenance may record:
 *
 *     source;
 *     derivation;
 *     transformation;
 *     decision;
 *     evidence;
 *     verification;
 *     compilation;
 *     placement;
 *     realization.
 *
 * This grammar does not implement provenance.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * The canonical task grammar is:
 *
 *     grammar/concurrency/tasks.g4
 *
 * Its public task boundary is:
 *
 *     taskExpression
 *     taskStatement
 *
 * This file adapts those constructs into the distributed domain.
 *
 * Dependency:
 *
 *     concurrency/tasks.g4
 *              |
 *              v
 *     distributed/tasks.g4
 *              |
 *              v
 *     distributed-concurrency.g4
 *
 * No reverse dependency is permitted.
 *
 * `concurrency/tasks.g4` MUST NOT import this file.
 *
 * ============================================================================
 * DISTRIBUTED-CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * `grammar/concurrency/distributed-concurrency.g4` already defines the
 * distributed/concurrency composition boundary and expects:
 *
 *     distributedTask
 *
 * This file provides that missing public rule.
 *
 * Therefore the integration becomes:
 *
 *     DistributedConcurrency
 *              |
 *              v
 *       distributedTask
 *              |
 *              v
 *       DistributedTasks
 *              |
 *              v
 *        taskExpression /
 *        taskStatement
 *
 * This resolves the existing composition gap without duplicating task syntax.
 *
 * ============================================================================
 * DISTRIBUTED COMPOSITION INTEGRATION
 * ============================================================================
 *
 * `grammar/distributed/distributed.g4` is the distributed composition root.
 *
 * It owns:
 *
 *     distributedDeclaration
 *
 * and composes specialized distributed grammars.
 *
 * This file must be imported by `Distributed` so that:
 *
 *     distributedTask
 *
 * becomes a valid distributed-domain construct.
 *
 * The distributed root should then contain:
 *
 *     distributedTask
 *
 * in its public declaration alternatives.
 *
 * This file must NOT replace:
 *
 *     distributedDeclaration
 *
 * or create another distributed program root.
 *
 * ============================================================================
 * ACTOR INTEGRATION
 * ============================================================================
 *
 * Actors remain owned by:
 *
 *     grammar/concurrency/actors.g4
 *     grammar/distributed/actors.g4
 *
 * A distributed task may execute inside an actor or be initiated by an actor,
 * but this file does not redefine actor syntax.
 *
 * Actor realization remains independent of task realization.
 *
 * ============================================================================
 * PROCESS INTEGRATION
 * ============================================================================
 *
 * Processes remain owned by:
 *
 *     grammar/distributed/processes.g4
 *
 * A process may host one or more logical tasks.
 *
 * A task is not a process.
 *
 * No process-specific syntax is introduced here.
 *
 * ============================================================================
 * SERVICE INTEGRATION
 * ============================================================================
 *
 * Services remain owned by:
 *
 *     grammar/distributed/services.g4
 *
 * A service may expose operations whose implementation contains tasks.
 *
 * A task may also invoke a service through ordinary expressions/networking
 * semantics.
 *
 * This file does not duplicate service syntax.
 *
 * ============================================================================
 * CHANNEL / MESSAGE INTEGRATION
 * ============================================================================
 *
 * Channels remain owned by:
 *
 *     grammar/distributed/channels.g4
 *     grammar/concurrency/channels.g4
 *
 * Message syntax remains owned by the canonical messaging/networking
 * subsystems.
 *
 * A distributed task may communicate through those abstractions, but this
 * grammar does not create:
 *
 *     distributedTaskMessage
 *     distributedTaskChannel
 *
 * as competing language constructs.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Network realization belongs to:
 *
 *     grammar/networking/
 *
 * A distributed task may require network capabilities or use a networking
 * operation through normal Zamani expressions.
 *
 * This file does not define:
 *
 *     sockets;
 *     ports;
 *     packets;
 *     transport protocols;
 *     physical network interfaces;
 *     network addresses.
 *
 * ============================================================================
 * PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Placement remains owned by:
 *
 *     grammar/distributed/placement.g4
 *
 * A distributed task can therefore be associated semantically with placement
 * intent without embedding placement syntax into the task adapter.
 *
 * This separation is critical for POCO-REAF.
 *
 * The source says:
 *
 *     what computation is required
 *
 * while placement says:
 *
 *     what realization constraints/preferences exist.
 *
 * ============================================================================
 * TOPOLOGY INTEGRATION
 * ============================================================================
 *
 * Logical topology remains owned by:
 *
 *     grammar/distributed/topology.g4
 *
 * A distributed task may depend on topology constraints through the universal
 * resource/capability/placement model.
 *
 * This file does not encode topology.
 *
 * ============================================================================
 * PARTITIONING INTEGRATION
 * ============================================================================
 *
 * Partitioning remains owned by:
 *
 *     grammar/distributed/partitioning.g4
 *
 * A logical task may be partitioned by downstream analysis.
 *
 * Partitioning does not require the task grammar to know:
 *
 *     how many partitions;
 *     how many machines;
 *     how many workers;
 *     which partitioning algorithm;
 *     which physical shard.
 *
 * ============================================================================
 * REPLICATION INTEGRATION
 * ============================================================================
 *
 * Replication remains owned by:
 *
 *     grammar/distributed/replication.g4
 *
 * A task may participate in replicated execution without becoming a different
 * source-level task construct.
 *
 * Replication strategy is semantic/deployment information.
 *
 * ============================================================================
 * FAULT-TOLERANCE INTEGRATION
 * ============================================================================
 *
 * Fault tolerance remains owned by:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * and the broader resilience architecture.
 *
 * A distributed task may be retried, recovered, rerouted, rescheduled,
 * degraded, escalated or rejected by downstream policy and execution
 * machinery.
 *
 * This grammar does not implement those operations.
 *
 * ============================================================================
 * TRANSACTION INTEGRATION
 * ============================================================================
 *
 * Distributed transaction syntax remains owned by:
 *
 *     grammar/distributed/transactions.g4
 *
 * A task may participate in a transaction through semantic composition.
 *
 * This file does not create transaction-specific task syntax.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A distributed task is domain-neutral.
 *
 * Its operand may represent:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     quantum measurement;
 *     quantum-classical feed-forward;
 *     accelerator computation;
 *     HDL/hardware-related computation;
 *     AI computation.
 *
 * If semantic analysis identifies quantum content, the canonical path remains:
 *
 *     frontend AST
 *          ->
 *     quantum semantic analysis
 *          ->
 *     quantum::ir
 *          ->
 *     optimization
 *          ->
 *     routing
 *          ->
 *     scheduling
 *          ->
 *     QEC / resilience
 *          ->
 *     ZQN
 *          ->
 *     HAL
 *
 * This grammar creates NO quantum task IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Distributed tasks may eventually contain or invoke hardware-oriented
 * computation.
 *
 * Hardware realization remains downstream.
 *
 * This file does not define:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     physical device;
 *     register;
 *     physical address.
 *
 * Hardware capabilities are discovered and negotiated downstream.
 *
 * ============================================================================
 * AI / LEARNING / ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Distributed tasks may contain:
 *
 *     reasoning;
 *     inference;
 *     learning;
 *     adaptation;
 *     knowledge operations;
 *     uncertainty;
 *     explainability;
 *     provenance.
 *
 * Those constructs remain owned by their AI/semantic grammars.
 *
 * This file merely provides the distributed task classification boundary.
 *
 * In particular, this file does not create application-specific task
 * keywords.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * A distributed task may be analyzed or executed under simulation.
 *
 * Simulation is an execution strategy, not a different task syntax.
 *
 * The simulation grammar/runtime remains responsible for:
 *
 *     simulation mode;
 *     model selection;
 *     simulation resources;
 *     deterministic simulation;
 *     fault simulation;
 *     performance simulation.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * A distributed task may be adaptively realized.
 *
 * Examples of downstream decisions include:
 *
 *     serialization;
 *     parallel execution;
 *     migration;
 *     replication;
 *     retry;
 *     recovery;
 *     accelerator selection;
 *     heterogeneous placement.
 *
 * Such decisions MUST preserve source semantics.
 *
 * The parser must never select a realization based on currently available
 * hardware.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this file depends only on:
 *
 *     - source tokens;
 *     - grammar composition;
 *     - grammar version.
 *
 * Parsing must not inspect:
 *
 *     - CPU count;
 *     - GPU count;
 *     - node count;
 *     - network state;
 *     - scheduler state;
 *     - runtime state;
 *     - filesystem state;
 *     - wall-clock time;
 *     - randomness;
 *     - deployment state.
 *
 * Identical source and grammar version must produce identical parse structure.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO implementation AST.
 *
 * The parse tree must be lowered into the existing domain-neutral frontend
 * representation.
 *
 * The semantic representation should retain, as applicable:
 *
 *     construct kind;
 *     nested task expression;
 *     nested task statement;
 *     source span;
 *     source ordering;
 *     task classification;
 *     attributes/modifiers;
 *     semantic requirements;
 *     dependencies;
 *     provenance.
 *
 * It MUST NOT require:
 *
 *     NodeId;
 *     WorkerId;
 *     ThreadId;
 *     CpuId;
 *     GpuId;
 *     FpgaId;
 *     QpuId;
 *     DeviceId;
 *     PhysicalAddress.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the underlying task is valid;
 *     - whether it is distributable;
 *     - whether its dependencies permit distribution;
 *     - whether ownership/lifetime rules permit distribution;
 *     - which effects it has;
 *     - which capabilities it requires;
 *     - which resources it requires;
 *     - which policies apply;
 *     - which contracts apply;
 *     - which placement constraints apply;
 *     - whether the target can realize it.
 *
 * Parser classification as a distributed task MUST NOT imply that distribution
 * is physically possible.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The operand/body uses the canonical task grammar and therefore inherits
 * ordinary Zamani type semantics.
 *
 * This file does not introduce:
 *
 *     DistributedTask<T>
 *     Remote<T>
 *     Node<T>
 *     Worker<T>
 *
 * as mandatory universal types.
 *
 * Such types may exist as libraries or semantic abstractions where explicitly
 * defined, but they are not required by this grammar.
 *
 * ============================================================================
 * OWNERSHIP / MEMORY CONTRACT
 * ============================================================================
 *
 * Task captures remain governed by the canonical ownership, borrowing,
 * lifetime, closure and memory systems.
 *
 * Distribution does not silently bypass those rules.
 *
 * Moving a computation between execution contexts is a semantic operation and
 * must be validated downstream.
 *
 * This file does not impose a memory model or capacity.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar generates NO IR.
 *
 * There is no:
 *
 *     DistributedTaskIR
 *     RemoteTaskIR
 *     NodeTaskIR
 *     WorkerTaskIR
 *
 * introduced here.
 *
 * The intended path is:
 *
 *     distributedTask
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic task model
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *      classical            quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *             optimization
 *                    |
 *            placement/routing
 *                    |
 *                scheduling
 *                    |
 *             resilience/QEC
 *                    |
 *                   ZQN
 *                    |
 *                   HAL
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may:
 *
 *     - analyze task dependencies;
 *     - identify distributable computation;
 *     - fuse tasks;
 *     - split tasks;
 *     - serialize tasks;
 *     - parallelize tasks;
 *     - partition work;
 *     - route dependencies;
 *     - place computation;
 *     - specialize for capabilities;
 *     - lower to heterogeneous targets.
 *
 * Transformations must preserve defined program semantics.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns:
 *
 *     scheduling;
 *     dispatch;
 *     communication;
 *     migration;
 *     retry;
 *     recovery;
 *     cancellation;
 *     resource acquisition;
 *     synchronization;
 *     execution.
 *
 * Runtime must not parse this grammar directly.
 *
 * It consumes compiled semantic/IR representations.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Merely classifying a task as distributed does not grant:
 *
 *     network capability;
 *     remote execution capability;
 *     native execution capability;
 *     foreign execution capability;
 *     filesystem capability;
 *     hardware capability.
 *
 * Capabilities and policies remain authoritative.
 *
 * This prevents distributed execution from becoming an implicit security
 * escalation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * It intentionally introduces NO:
 *
 *     TASK
 *     DISTRIBUTED_TASK
 *     REMOTE_TASK
 *     WORKER
 *     NODE
 *     EXECUTOR
 *
 * token.
 *
 * The repository already has:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *     DISTRIBUTED
 *
 * and the canonical task grammar already consumes the asynchronous vocabulary.
 *
 * The distributed task adapter therefore introduces no new lexical dependency.
 *
 * This is intentional:
 *
 *     new distributed execution realization
 *
 * must not require:
 *
 *     new universal keyword
 *
 * unless the language specification establishes an actual new source-level
 * syntactic category.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Tasks
 *
 * from:
 *
 *     grammar/concurrency/tasks.g4
 *
 * It does NOT import:
 *
 *     Distributed
 *
 * because `Distributed` imports this grammar.
 *
 * Importing `Distributed` here would create a cycle:
 *
 *     Distributed
 *         ->
 *     DistributedTasks
 *         ->
 *     Distributed
 *
 * Such a cycle is prohibited.
 *
 * The same reason applies to:
 *
 *     DistributedConcurrency
 *
 * This file must not import the distributed-concurrency composition root.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * `distributedTask` is intentionally the stable public adapter.
 *
 * Consumers should normally use:
 *
 *     distributedTask
 *
 * rather than depending directly on internal adapter rules.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file fills an existing repository composition gap:
 *
 *     grammar/concurrency/distributed-concurrency.g4
 *
 * already references:
 *
 *     distributedTask
 *
 * while the distributed composition root does not currently provide it.
 *
 * Adding this adapter allows existing references to resolve without creating
 * duplicate task syntax.
 *
 * Existing task consumers continue using:
 *
 *     taskExpression
 *     taskStatement
 *
 * from:
 *
 *     grammar/concurrency/tasks.g4
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The distributed task adapter must accept every construct accepted by the
 * canonical task grammar when entered through this adapter:
 *
 *     spawn computation()
 *
 *     spawn value
 *
 *     spawn {
 *         computation()
 *     }
 *
 *     await task
 *
 *     await computation()
 *
 *     parallel computation()
 *
 *     parallel {
 *         computation()
 *     }
 *
 * Nested combinations must remain possible.
 *
 * Examples:
 *
 *     await spawn computation()
 *
 *     spawn {
 *         await computation()
 *     }
 *
 *     parallel {
 *         spawn computation_a();
 *         spawn computation_b();
 *     }
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Test task operands involving:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL-related computation;
 *     AI computation;
 *     accelerator computation;
 *     distributed service invocation;
 *     networking operation.
 *
 * These tests verify parser neutrality.
 *
 * NEGATIVE
 * --------
 *
 * The adapter must reject malformed underlying task syntax according to the
 * canonical task grammar.
 *
 * Examples:
 *
 *     spawn
 *     await
 *     parallel
 *
 * malformed nesting;
 * malformed blocks;
 * unexpected delimiters;
 * incomplete expressions.
 *
 * SECURITY
 * --------
 *
 * Verify that distributed classification does not by itself grant:
 *
 *     network;
 *     native;
 *     foreign;
 *     hardware;
 *     remote-execution;
 *
 * capabilities.
 *
 * SEMANTIC
 * --------
 *
 * Verify downstream diagnostics for:
 *
 *     invalid ownership;
 *     invalid lifetime;
 *     unsatisfied capability;
 *     unavailable resource;
 *     forbidden effect;
 *     invalid placement;
 *     invalid communication dependency.
 *
 * SCALABILITY
 * -----------
 *
 * Test logically increasing task sets and nesting without defining a language
 * maximum.
 *
 * The tests MUST distinguish:
 *
 *     grammar rejection
 *
 * from:
 *
 *     compiler/resource/runtime exhaustion.
 *
 * The grammar must never reject a program because a particular target has
 * insufficient:
 *
 *     processors;
 *     accelerators;
 *     memory;
 *     nodes;
 *     network bandwidth;
 *     QPU capacity.
 *
 * DETERMINISM
 * -----------
 *
 * Identical token streams and grammar version must yield identical parse
 * structure independent of target resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No universal physical or logical capacity is encoded.
 *
 * There are no:
 *
 *     fixed task counts;
 *     fixed worker counts;
 *     fixed node counts;
 *     fixed process counts;
 *     fixed thread counts;
 *     fixed device counts;
 *     fixed memory sizes;
 *     fixed topology sizes;
 *     fixed network sizes;
 *     fixed quantum resource counts.
 *
 * There is no target-specific syntax.
 *
 * There is no vendor-specific syntax.
 *
 * There is no scheduler-specific syntax.
 *
 * There is no runtime-specific syntax.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This is an ANTLR parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime callbacks;
 *     - no mutable parser-global state;
 *     - no randomness.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97+
 *
 * and Rust 2021.
 *
 * This grammar must never require unsafe Rust.
 *
 * FFI/native execution is a downstream explicitly controlled effect/capability
 * boundary and is not created by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [ ] It exists at grammar/distributed/tasks.g4.
 *
 * [ ] Its grammar name is DistributedTasks.
 *
 * [ ] It imports Tasks.
 *
 * [ ] It defines distributedTask.
 *
 * [ ] It defines distributedTaskConstruct.
 *
 * [ ] It defines distributedTaskExpression.
 *
 * [ ] It defines distributedTaskStatement.
 *
 * [ ] It does not redefine spawn syntax.
 *
 * [ ] It does not redefine await syntax.
 *
 * [ ] It does not redefine parallel syntax.
 *
 * [ ] It does not introduce a TASK token.
 *
 * [ ] It does not introduce a remote-task token.
 *
 * [ ] It does not introduce physical resource syntax.
 *
 * [ ] It does not introduce hardware limits.
 *
 * [ ] It does not introduce distributed capacity limits.
 *
 * [ ] It does not create a DistributedTaskIR.
 *
 * [ ] It preserves domain-neutral AST semantics.
 *
 * [ ] It integrates with DistributedConcurrency.
 *
 * [ ] It integrates with Distributed.
 *
 * [ ] It remains independent of runtime scheduling.
 *
 * [ ] It remains independent of placement implementation.
 *
 * [ ] It remains independent of networking implementation.
 *
 * [ ] It remains independent of hardware implementation.
 *
 * [ ] It remains independent of quantum::ir implementation.
 *
 * [ ] It remains deterministic.
 *
 * [ ] It requires no unsafe Rust.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar DistributedTasks;

options {
    tokenVocab = ZamaniLexer;
}

import
    Tasks
;


/*
 * ============================================================================
 * PUBLIC DISTRIBUTED TASK BOUNDARY
 * ============================================================================
 *
 * This is the canonical distributed-domain task entry point.
 *
 * It is deliberately an adapter.
 */
distributedTask
    : distributedTaskConstruct
    ;


/*
 * ============================================================================
 * DISTRIBUTED TASK CONSTRUCT
 * ============================================================================
 *
 * A distributed task is either an expression or a statement originating from
 * the canonical task grammar.
 *
 * No new concrete syntax is introduced here.
 */
distributedTaskConstruct
    : distributedTaskExpression
    | distributedTaskStatement
    ;


/*
 * ============================================================================
 * DISTRIBUTED TASK EXPRESSION
 * ============================================================================
 *
 * The underlying task expression remains owned by concurrency/tasks.g4.
 */
distributedTaskExpression
    : distributedTaskConcurrencyExpression
    ;


/*
 * ============================================================================
 * DISTRIBUTED TASK EXPRESSION ADAPTER
 * ============================================================================
 */
distributedTaskConcurrencyExpression
    : taskExpression
    ;


/*
 * ============================================================================
 * DISTRIBUTED TASK STATEMENT
 * ============================================================================
 *
 * The underlying task statement remains owned by concurrency/tasks.g4.
 */
distributedTaskStatement
    : distributedTaskConcurrencyStatement
    ;


/*
 * ============================================================================
 * DISTRIBUTED TASK STATEMENT ADAPTER
 * ============================================================================
 */
distributedTaskConcurrencyStatement
    : taskStatement
    ;