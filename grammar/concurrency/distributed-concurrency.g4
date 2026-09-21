/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/distributed-concurrency.g4
 *
 * GRAMMAR
 * -------
 * DistributedConcurrency
 *
 * STATUS
 * ------
 * Production-target distributed-concurrency composition grammar.
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 * Safe Rust only.
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime callbacks;
 *     - no mutable global parser state;
 *     - no randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the PARSER-LEVEL INTEGRATION BOUNDARY between:
 *
 *     distributed computation
 *
 * and:
 *
 *     concurrency;
 *     tasks;
 *     futures;
 *     parallel computation;
 *     task parallelism.
 *
 * It does NOT create another distributed language.
 *
 * It does NOT create another concurrency language.
 *
 * It composes the canonical grammar owners already present in the repository.
 *
 * The intended architecture is:
 *
 *     distributed/
 *         distributed.g4
 *         communication.g4
 *         messaging.g4
 *         remote-execution.g4
 *         services.g4
 *         ...
 *
 *     concurrency/
 *         tasks.g4
 *         futures.g4
 *         parallel.g4
 *         data-parallel.g4
 *         task-parallel.g4
 *         concurrency.g4
 *         ...
 *
 *     distributed-concurrency.g4
 *         |
 *         +--> composition boundary
 *
 * This file therefore remains deliberately small.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     - distributed/concurrency composition;
 *     - stable parser-facing integration names;
 *     - classification boundaries for tooling;
 *     - adapters connecting distributed syntax to concurrency syntax;
 *     - cross-domain parse structure needed by semantic analysis;
 *     - distributed-concurrency compatibility boundaries.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - ordinary statements;
 *     - blocks;
 *     - spawn syntax;
 *     - await syntax;
 *     - future syntax;
 *     - generic parallel syntax;
 *     - task-parallel syntax;
 *     - data-parallel syntax;
 *     - distributed task declarations;
 *     - distributed operations;
 *     - distributed communication;
 *     - distributed messaging;
 *     - distributed services;
 *     - distributed placement;
 *     - distributed deployment;
 *     - distributed remote execution;
 *     - networking;
 *     - synchronization;
 *     - cancellation;
 *     - scheduling;
 *     - routing;
 *     - resource allocation;
 *     - hardware;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - HAL;
 *     - runtime execution.
 *
 * Those remain owned by their existing repository components.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly ONE syntax owner for every concrete source construct.
 *
 * Distributed syntax:
 *
 *     grammar/distributed/distributed.g4
 *
 * owns:
 *
 *     distributedTask
 *     distributedOperation
 *     distributedStatement
 *     distributedBlock
 *     distributedInvocation
 *     distributedRelationship
 *     distributedDependency
 *     ...
 *
 * Task syntax:
 *
 *     grammar/concurrency/tasks.g4
 *
 * owns task-oriented syntax.
 *
 * Future syntax:
 *
 *     grammar/concurrency/futures.g4
 *
 * owns future/await integration.
 *
 * Generic parallel syntax:
 *
 *     grammar/concurrency/parallel.g4
 *
 * owns parallel syntax.
 *
 * Task-parallel syntax:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * owns:
 *
 *     parallel group { ... }
 *
 * Data-parallel syntax:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * owns:
 *
 *     map
 *     reduce
 *     scan
 *     partition
 *     data-parallel iteration
 *
 * This file MUST NOT redefine any of those concrete productions.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Distributed concurrency expresses LOGICAL COMPUTATIONAL INTENT.
 *
 * It does not encode physical execution.
 *
 * Therefore this file MUST NOT impose language-level limits on:
 *
 *     tasks
 *     concurrent tasks
 *     futures
 *     parallel regions
 *     distributed participants
 *     nodes
 *     processes
 *     workers
 *     threads
 *     cores
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     channels
 *     messages
 *     replicas
 *     partitions
 *     memory
 *     timelines
 *     execution contexts
 *
 * It MUST NOT contain universal constants such as:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_MESSAGES
 *     MAX_CHANNELS
 *     MAX_REPLICAS
 *     MAX_PARTITIONS
 *
 * Logical concurrency is not physical concurrency.
 *
 * A distributed computation may be realized using:
 *
 *     one execution context;
 *     multiple CPU cores;
 *     vector execution;
 *     GPU execution;
 *     FPGA execution;
 *     ASIC execution;
 *     accelerator execution;
 *     quantum/classical execution;
 *     heterogeneous execution;
 *     multiple machines;
 *     a cluster;
 *     HPC infrastructure;
 *     cloud infrastructure;
 *     future computational substrates.
 *
 * The source program remains unchanged when the available realization changes.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * This file MUST NOT confuse:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     placement
 *     implementation decision
 *
 * For example:
 *
 *     logical distributed computation
 *
 * is not:
 *
 *     use node 0
 *
 * and:
 *
 *     requires capability("distributed.communication")
 *
 * is not:
 *
 *     use network device 3
 *
 * Resource discovery and target selection occur downstream.
 *
 * ============================================================================
 * CANONICAL PIPELINE
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
 *          v
 *     DistributedConcurrency
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     name/type/effect analysis
 *          |
 *          v
 *     concurrency analysis
 *          |
 *          v
 *     distributed semantic analysis
 *          |
 *          +----------------------------+
 *          |                            |
 *          v                            v
 *     resource/capability          security analysis
 *          |                            |
 *          +-------------+--------------+
 *                        |
 *                        v
 *              canonical semantic model
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      classical     quantum::ir     HDL/hardware
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                 optimization
 *                        |
 *                 routing / placement
 *                        |
 *                    scheduling
 *                        |
 *                 resilience / QEC
 *                        |
 *                       ZQN
 *                        |
 *                       HAL
 *                        |
 *                 target realization
 *                        |
 *                       runtime
 *
 * This grammar participates ONLY in the parser/source structure portion.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The lexer remains the sole authority for lexical spelling.
 *
 * This file intentionally introduces NO lexer rules.
 *
 * Existing repository vocabulary remains authoritative, including:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *     GROUP
 *     REQUIRES
 *
 * and canonical punctuation such as:
 *
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *     ASSIGN
 *     THIN_ARROW
 *
 * No new distributed-concurrency keyword is required.
 *
 * In particular, this file does NOT introduce:
 *
 *     DISTRIBUTED
 *     DISTRIBUTED_CONCURRENCY
 *     REMOTE
 *     NODE
 *     WORKER
 *     THREAD
 *     CORE
 *     DEVICE
 *     JOIN
 *     DEPENDS
 *     TASK
 *     FUTURE
 *
 * merely to make this grammar appear more expressive.
 *
 * Existing lexical vocabulary is reused.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * `Distributed` is the authoritative distributed-computation grammar.
 *
 * `Tasks` is the authoritative task grammar.
 *
 * `Futures` is the authoritative future grammar.
 *
 * `Parallel` is the authoritative generic parallel grammar.
 *
 * `TaskParallel` is the authoritative structured task-parallel grammar.
 *
 * Therefore:
 *
 *     DistributedConcurrency
 *         imports
 *             Distributed
 *             Tasks
 *             Futures
 *             Parallel
 *             TaskParallel
 *
 * No leaf grammar is reimplemented here.
 *
 * ============================================================================
 */

parser grammar DistributedConcurrency;

options {
    tokenVocab = ZamaniLexer;
}

import
    Distributed,
    Tasks,
    Futures,
    Parallel,
    TaskParallel
;


/*
 * ============================================================================
 * 1. PUBLIC DISTRIBUTED-CONCURRENCY ENTRY POINT
 * ============================================================================
 *
 * This is the stable parser-domain entry point.
 *
 * IMPORTANT:
 *
 * It is an INTEGRATION BOUNDARY.
 *
 * It does not add a new concrete syntax family.
 *
 * The concrete syntax continues to belong to the imported grammar owners.
 *
 * This rule exists so semantic tooling and future parser composition can refer
 * to one stable distributed-concurrency domain boundary.
 */
distributedConcurrencyConstruct
    : distributedConcurrencyTask
    | distributedConcurrencyFuture
    | distributedConcurrencyParallel
    | distributedConcurrencyTaskParallel
    | distributedConcurrencyDistributed
    ;


/*
 * ============================================================================
 * 2. DISTRIBUTED TASK BOUNDARY
 * ============================================================================
 *
 * This adapter connects canonical distributed task syntax with the canonical
 * task-concurrency grammar.
 *
 * No new spawn syntax is introduced.
 *
 * The distributed task declaration itself remains owned by `Distributed`.
 */
distributedConcurrencyTask
    : distributedTask
    ;


/*
 * ============================================================================
 * 3. DISTRIBUTED FUTURE BOUNDARY
 * ============================================================================
 *
 * A future may represent:
 *
 *     local computation;
 *     remote computation;
 *     distributed computation;
 *     accelerator computation;
 *     quantum computation;
 *     hybrid computation;
 *     networking;
 *     storage;
 *     another asynchronous operation.
 *
 * The parser does not decide which.
 *
 * The canonical future grammar remains authoritative.
 */
distributedConcurrencyFuture
    : futureExpression
    ;


/*
 * ============================================================================
 * 4. DISTRIBUTED PARALLEL BOUNDARY
 * ============================================================================
 *
 * The generic parallel construct is owned by `Parallel`.
 *
 * This adapter does not redefine:
 *
 *     parallelExpression
 *
 * or:
 *
 *     parallelStatement
 *
 * The semantic layer determines whether the parallel computation is:
 *
 *     local;
 *     distributed;
 *     heterogeneous;
 *     accelerator-backed;
 *     quantum/classical;
 *     otherwise realized.
 */
distributedConcurrencyParallel
    : parallelConstruct
    ;


/*
 * ============================================================================
 * 5. DISTRIBUTED TASK-PARALLEL BOUNDARY
 * ============================================================================
 *
 * Structured task parallelism remains owned by TaskParallel.
 *
 * Canonical example:
 *
 *     parallel group {
 *         spawn computation_a();
 *         spawn computation_b();
 *     }
 *
 * Whether those logical tasks execute:
 *
 *     sequentially;
 *     concurrently;
 *     locally;
 *     remotely;
 *     heterogeneously;
 *
 * is a downstream semantic/runtime decision.
 */
distributedConcurrencyTaskParallel
    : taskParallelConstruct
    ;


/*
 * ============================================================================
 * 6. DISTRIBUTED DOMAIN BOUNDARY
 * ============================================================================
 *
 * This adapter exposes the canonical distributed grammar to semantic tooling.
 *
 * It does NOT duplicate distributed syntax.
 */
distributedConcurrencyDistributed
    : distributedStatement
    | distributedExpression
    | distributedTask
    | distributedOperation
    ;


/*
 * ============================================================================
 * 7. DISTRIBUTED-CONCURRENCY EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This is the stable expression-facing integration point.
 *
 * It deliberately does not include `distributedStatement`.
 *
 * Statement/expression ownership remains separate.
 */
distributedConcurrencyExpression
    : distributedConcurrencyFuture
    | distributedConcurrencyParallel
    | distributedConcurrencyDistributedExpression
    ;


/*
 * ============================================================================
 * 8. DISTRIBUTED-CONCURRENCY DISTRIBUTED EXPRESSION
 * ============================================================================
 *
 * The canonical distributed grammar currently treats distributed expressions
 * as ordinary Zamani expressions through `distributedExpression`.
 *
 * This adapter preserves that architecture.
 */
distributedConcurrencyDistributedExpression
    : distributedExpression
    ;


/*
 * ============================================================================
 * 9. DISTRIBUTED-CONCURRENCY STATEMENT BOUNDARY
 * ============================================================================
 *
 * This is the stable statement-facing integration point.
 *
 * It does not redefine generic statement syntax.
 */
distributedConcurrencyStatement
    : distributedConcurrencyDistributedStatement
    | distributedConcurrencyParallelStatement
    | distributedConcurrencyTaskStatement
    ;


/*
 * ============================================================================
 * 10. DISTRIBUTED STATEMENT ADAPTER
 * ============================================================================
 *
 * The distributed grammar owns the actual distributed statement syntax.
 */
distributedConcurrencyDistributedStatement
    : distributedStatement
    ;


/*
 * ============================================================================
 * 11. PARALLEL STATEMENT ADAPTER
 * ============================================================================
 *
 * Generic parallel statement syntax remains owned by Parallel.
 *
 * No duplicate `PARALLEL ... SEMICOLON` production is created here.
 */
distributedConcurrencyParallelStatement
    : parallelStatement
    ;


/*
 * ============================================================================
 * 12. TASK STATEMENT ADAPTER
 * ============================================================================
 *
 * Task statement syntax remains owned by Tasks.
 */
distributedConcurrencyTaskStatement
    : taskStatement
    ;


/*
 * ============================================================================
 * 13. DISTRIBUTED-CONCURRENCY DOMAIN
 * ============================================================================
 *
 * Stable semantic classification boundary.
 *
 * This rule intentionally delegates all concrete syntax.
 */
distributedConcurrencyDomain
    : distributedConcurrencyConstruct
    ;


/*
 * ============================================================================
 * 14. DISTRIBUTED ASYNCHRONOUS OPERATION
 * ============================================================================
 *
 * A distributed computation may be represented as an ordinary asynchronous
 * operation.
 *
 * The grammar intentionally does not define:
 *
 *     remote_call
 *     rpc
 *     remote_task
 *     distributed_future
 *
 * as new syntax.
 *
 * Existing asynchronous and distributed syntax remains authoritative.
 */
distributedAsyncOperation
    : futureOperation
    ;


/*
 * ============================================================================
 * 15. DISTRIBUTED PARALLEL OPERATION
 * ============================================================================
 *
 * Generic parallel semantics may eventually be distributed.
 *
 * This is a semantic composition boundary, not a new source syntax.
 */
distributedParallelOperation
    : parallelComputationExpression
    ;


/*
 * ============================================================================
 * 16. DISTRIBUTED TASK-PARALLEL OPERATION
 * ============================================================================
 *
 * Structured task parallelism is represented by the existing TaskParallel
 * grammar.
 */
distributedTaskParallelOperation
    : taskParallelConstruct
    ;


/*
 * ============================================================================
 * 17. DISTRIBUTED COMPUTATION OPERATION
 * ============================================================================
 *
 * Distributed computation remains open-world.
 *
 * `distributedOperation` already accepts a qualified operation name and
 * ordinary Zamani expressions.
 *
 * Therefore this grammar does not create a closed list such as:
 *
 *     distributedSend
 *     distributedReceive
 *     distributedBroadcast
 *     distributedGather
 *     distributedScatter
 *     distributedReduce
 *
 * Those concepts are owned by the distributed/networking grammars where
 * appropriate.
 */
distributedComputationOperation
    : distributedOperation
    ;


/*
 * ============================================================================
 * 18. DISTRIBUTED CONCURRENCY INVOCATION
 * ============================================================================
 *
 * A distributed invocation remains a distributed-domain construct.
 *
 * This adapter is useful for semantic tooling without creating another
 * invocation syntax.
 */
distributedConcurrencyInvocation
    : distributedInvocation
    ;


/*
 * ============================================================================
 * 19. DISTRIBUTED DEPENDENCY BOUNDARY
 * ============================================================================
 *
 * Distributed dependency syntax is owned by Distributed.
 *
 * Concurrency analysis may consume the resulting structure.
 */
distributedConcurrencyDependency
    : distributedDependency
    ;


/*
 * ============================================================================
 * 20. DISTRIBUTED RELATIONSHIP BOUNDARY
 * ============================================================================
 *
 * Distributed relationships are owned by Distributed.
 *
 * Examples already represented by the distributed grammar include relationships
 * expressing dependency, connection, replication, placement, or coordination.
 *
 * This file does not duplicate them.
 */
distributedConcurrencyRelationship
    : distributedRelationship
    ;


/*
 * ============================================================================
 * 21. DISTRIBUTED BLOCK BOUNDARY
 * ============================================================================
 *
 * Distributed blocks remain owned by Distributed.
 */
distributedConcurrencyBlock
    : distributedBlock
    ;


/*
 * ============================================================================
 * 22. DISTRIBUTED TASK + FUTURE COMPOSITION
 * ============================================================================
 *
 * This rule gives semantic tooling a stable name for a common composition:
 *
 *     distributed task
 *     +
 *     future observation
 *
 * The concrete syntax remains owned by the imported grammars.
 */
distributedTaskFutureComposition
    : distributedConcurrencyTask
    | distributedConcurrencyFuture
    ;


/*
 * ============================================================================
 * 23. DISTRIBUTED TASK + PARALLEL COMPOSITION
 * ============================================================================
 *
 * Logical distributed tasks may participate in a parallel region.
 *
 * No physical worker assignment is implied.
 */
distributedTaskParallelComposition
    : distributedConcurrencyTask
    | distributedConcurrencyParallel
    ;


/*
 * ============================================================================
 * 24. DISTRIBUTED TASK + TASK-PARALLEL COMPOSITION
 * ============================================================================
 *
 * Structured task groups remain owned by TaskParallel.
 */
distributedTaskStructuredComposition
    : distributedConcurrencyTaskParallel
    ;


/*
 * ============================================================================
 * 25. DISTRIBUTED CONCURRENCY EXPRESSION
 * ============================================================================
 *
 * Stable expression-oriented alias.
 *
 * This is deliberately an adapter and not a second expression grammar.
 */
distributedConcurrencyComputationExpression
    : distributedConcurrencyExpression
    | distributedParallelOperation
    | distributedTaskParallelOperation
    | distributedAsyncOperation
    ;


/*
 * ============================================================================
 * 26. DISTRIBUTED CONCURRENCY STATEMENT
 * ============================================================================
 *
 * Stable statement-oriented alias.
 */
distributedConcurrencyComputationStatement
    : distributedConcurrencyStatement
    ;


/*
 * ============================================================================
 * 27. OPEN-WORLD EXTENSION BOUNDARY
 * ============================================================================
 *
 * Distributed systems evolve continuously.
 *
 * New distributed execution models must NOT require a new keyword merely
 * because a new execution mechanism appears.
 *
 * Existing qualified-name and expression mechanisms remain available through
 * the canonical distributed grammar.
 *
 * Examples of semantic concepts that may be added downstream include:
 *
 *     distributed::future_protocol
 *     distributed::remote_accelerator
 *     distributed::quantum_federation
 *     distributed::heterogeneous_execution
 *     distributed::edge_execution
 *     distributed::cloud_execution
 *     distributed::fault_domain
 *
 * Their semantic registration belongs outside this grammar.
 */
distributedConcurrencyExtensionPoint
    : distributedOperation
    ;


/*
 * ============================================================================
 * 28. AST CONTRACT
 * ============================================================================
 *
 * This file creates NO AST implementation.
 *
 * Parser output must map into the repository's existing domain-neutral frontend
 * AST.
 *
 * The AST must preserve:
 *
 *     - complete source span;
 *     - child source spans;
 *     - source ordering;
 *     - nesting;
 *     - qualified names;
 *     - expression structure;
 *     - task/future/parallel structure;
 *     - distributed relationships;
 *     - attributes;
 *     - modifiers;
 *     - semantic names.
 *
 * The AST MUST NOT require backend-specific objects such as:
 *
 *     RuntimeTaskId
 *     WorkerId
 *     ThreadId
 *     CoreId
 *     NodeId
 *     GPUId
 *     QPUId
 *     DeviceHandle
 *     NetworkHandle
 *     ExecutorHandle
 *
 * Those are downstream realization concepts.
 *
 * ============================================================================
 * GENERIC OPERATION CONTRACT
 * ============================================================================
 *
 * Where the repository's frontend uses the domain-neutral operation model:
 *
 *     Operation {
 *         name,
 *         namespace,
 *         operands,
 *         parameters,
 *         results,
 *         attributes,
 *         modifiers,
 *         effects,
 *         capabilities,
 *         source
 *     }
 *
 * distributed concurrency constructs should map into that model where
 * appropriate rather than introducing a competing distributed-concurrency
 * operation hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether an operation is distributed;
 *     - whether a task is spawnable;
 *     - whether a value is awaitable;
 *     - whether parallel execution is legal;
 *     - whether dependencies are valid;
 *     - whether data races or ownership conflicts exist;
 *     - whether effects are compatible;
 *     - whether communication is legal;
 *     - whether required capabilities exist;
 *     - whether resource requirements can be satisfied;
 *     - whether placement is possible;
 *     - whether execution is deterministic where required;
 *     - whether fault/recovery semantics are satisfiable;
 *     - whether security requirements are satisfied.
 *
 * Syntax validity MUST NOT imply semantic validity.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP CONTRACT
 * ============================================================================
 *
 * This file does not introduce a distributed ownership model.
 *
 * Ownership, borrowing, references, lifetimes, mutation, sharing, persistence,
 * and distributed memory remain under the canonical memory/type/semantic
 * systems.
 *
 * A task capture across a distributed boundary may therefore require semantic
 * checks for:
 *
 *     ownership;
 *     serialization;
 *     sharing;
 *     lifetime;
 *     capability;
 *     security;
 *     consistency.
 *
 * Those checks occur after parsing.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Distributed concurrency may induce effects such as:
 *
 *     asynchronous execution;
 *     suspension;
 *     communication;
 *     synchronization;
 *     remote execution;
 *     distributed state;
 *     nondeterministic ordering;
 *     failure/recovery;
 *     external resource access.
 *
 * The effect system owns these effects.
 *
 * This grammar only preserves the source structure from which they may be
 * derived.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file does not allocate resources.
 *
 * Resource analysis may derive requirements for:
 *
 *     execution capacity;
 *     memory;
 *     communication;
 *     latency;
 *     bandwidth;
 *     reliability;
 *     accelerator capabilities;
 *     quantum capabilities;
 *     storage;
 *     security domains;
 *     fault domains.
 *
 * The actual realization is downstream.
 *
 * ============================================================================
 * DISTRIBUTED DEPLOYMENT CONTRACT
 * ============================================================================
 *
 * The source grammar MUST remain valid if deployment changes between:
 *
 *     one machine;
 *     multiple machines;
 *     embedded;
 *     edge;
 *     workstation;
 *     server;
 *     HPC;
 *     cluster;
 *     cloud;
 *     heterogeneous system;
 *     quantum/classical infrastructure;
 *     future computational substrate.
 *
 * No source rewrite should be required merely because deployment scale changes.
 *
 * ============================================================================
 * NETWORKING CONTRACT
 * ============================================================================
 *
 * Distributed concurrency does not select transport.
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
 *     vendor-specific transport.
 *
 * Networking and runtime layers select appropriate realizations.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Distributed execution can cross trust boundaries.
 *
 * This grammar does not implement:
 *
 *     authentication;
 *     authorization;
 *     encryption;
 *     key management;
 *     identity;
 *     trust;
 *     attestation.
 *
 * Security requirements are represented by the canonical security/capability
 * system and checked downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Distributed concurrency may coordinate:
 *
 *     quantum computation;
 *     quantum simulation;
 *     hybrid computation;
 *     distributed measurement/control;
 *     remote quantum execution.
 *
 * This grammar MUST NOT introduce:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     GateKind;
 *     quantum topology;
 *     pulse syntax;
 *     calibration;
 *     QEC implementation;
 *     ZQN implementation.
 *
 * Quantum semantics remain under:
 *
 *     quantum::ir
 *
 * followed by:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     QEC/resilience
 *     ZQN
 *     HAL
 *     target realization
 *
 * No distributed-concurrency quantum IR is permitted.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Distributed concurrency may coordinate hardware/software co-design.
 *
 * This file does not define:
 *
 *     wires;
 *     registers;
 *     clocks;
 *     physical pins;
 *     FPGA cells;
 *     ASIC cells;
 *     device IDs;
 *     memory-bank IDs;
 *     physical topology.
 *
 * Hardware and HDL grammars remain authoritative for their respective source
 * domains.
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * Distributed concurrency may surround:
 *
 *     tensor computation;
 *     data pipelines;
 *     training;
 *     inference;
 *     agents;
 *     model execution;
 *     distributed datasets;
 *     data-parallel computation.
 *
 * No AI framework or accelerator API becomes a grammar keyword through this
 * file.
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * It must never introduce:
 *
 *     DistributedConcurrencyIR
 *     DistributedTaskIR
 *     RemoteTaskIR
 *     DistributedQuantumIR
 *     DistributedParallelIR
 *
 * as competing universal representations.
 *
 * Parsed source maps to the canonical frontend/semantic model and then to the
 * existing canonical IR architecture.
 *
 * Quantum source continues through:
 *
 *     quantum::ir
 *
 * Classical source continues through the canonical classical representation.
 *
 * Hardware/HDL source continues through the canonical hardware/HDL semantic
 * representation.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler consumers may use the semantic information derived from this file
 * for:
 *
 *     name resolution;
 *     type checking;
 *     effect checking;
 *     ownership checking;
 *     dependency analysis;
 *     communication analysis;
 *     capability checking;
 *     resource analysis;
 *     optimization;
 *     placement;
 *     routing;
 *     scheduling;
 *     resilience;
 *     target lowering.
 *
 * None of those decisions occur inside this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime realization may use:
 *
 *     local execution;
 *     worker pools;
 *     OS threads;
 *     asynchronous executors;
 *     accelerators;
 *     remote workers;
 *     distributed services;
 *     heterogeneous execution;
 *     quantum processors;
 *     simulators;
 *     future execution mechanisms.
 *
 * The grammar is independent of all such implementation strategies.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source token sequence;
 *     grammar version;
 *     imported grammar versions;
 *     canonical token vocabulary.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU availability;
 *     worker availability;
 *     node count;
 *     GPU availability;
 *     QPU availability;
 *     network availability;
 *     runtime state;
 *     system time;
 *     randomness;
 *     environment variables;
 *     deployment state.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no finite language-level hardware limits.
 *
 * Repetition and nesting are delegated to the canonical grammar owners.
 *
 * Therefore the language can represent source programs containing:
 *
 *     one logical task;
 *     many logical tasks;
 *     nested task groups;
 *     many futures;
 *     many distributed operations;
 *     many communication relationships;
 *     large distributed computation graphs;
 *     arbitrarily complex cross-domain computations;
 *
 * subject only to actual compiler/parser/runtime/resource availability.
 *
 * A practical implementation may impose resource-safety limits externally,
 * but such limits MUST NOT be confused with language semantics.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples of semantic diagnostics include:
 *
 *     non-awaitable value;
 *     invalid distributed task;
 *     invalid ownership capture;
 *     incompatible effects;
 *     unsatisfied capability;
 *     unsatisfied resource requirement;
 *     invalid communication relationship;
 *     illegal placement request;
 *     unsupported execution capability.
 *
 * These MUST NOT be encoded as parser-only special cases.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing filenames remain unchanged.
 *
 * Existing lexer tokens remain unchanged.
 *
 * No token migration is required for this file.
 *
 * Existing concrete syntax remains owned by:
 *
 *     Distributed
 *     Tasks
 *     Futures
 *     Parallel
 *     TaskParallel
 *
 * This file therefore acts as a compatibility-preserving integration layer.
 *
 * ============================================================================
 * INTEGRATION WITH CONCURRENCY.G4
 * ============================================================================
 *
 * `grammar/concurrency/concurrency.g4` is the concurrency composition root.
 *
 * It should import this grammar:
 *
 *     DistributedConcurrency
 *
 * after the distributed-concurrency feature has been integrated into the
 * canonical concurrency composition.
 *
 * IMPORTANT:
 *
 * The parent concurrency grammar SHOULD NOT add another sibling alternative
 * that independently reparses the same distributed constructs.
 *
 * Prefer:
 *
 *     concurrency.g4
 *          |
 *          v
 *     distributedConcurrencyConstruct
 *
 * over:
 *
 *     concurrency.g4
 *          |
 *          +--> distributedTask
 *          +--> distributedOperation
 *          +--> parallelConstruct
 *          +--> taskParallelConstruct
 *
 * because the latter duplicates ownership.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED.G4
 * ============================================================================
 *
 * `grammar/distributed/distributed.g4` remains authoritative for distributed
 * source syntax.
 *
 * It must NOT import this file merely to obtain its own distributed rules.
 *
 * The dependency direction is:
 *
 *     Distributed
 *          ^
 *          |
 *     DistributedConcurrency
 *
 * not:
 *
 *     Distributed <-> DistributedConcurrency
 *
 * This prevents cyclic grammar ownership.
 *
 * ============================================================================
 * INTEGRATION WITH TASKS.G4
 * ============================================================================
 *
 * `tasks.g4` remains authoritative for:
 *
 *     spawn;
 *     await;
 *     task expressions;
 *     task statements;
 *     task-specific composition.
 *
 * This file only consumes those rules.
 *
 * ============================================================================
 * INTEGRATION WITH FUTURES.G4
 * ============================================================================
 *
 * `futures.g4` remains authoritative for:
 *
 *     futureExpression;
 *     futureOperation;
 *     await-based future observation.
 *
 * This file consumes those rules without introducing `FUTURE`.
 *
 * ============================================================================
 * INTEGRATION WITH PARALLEL.G4
 * ============================================================================
 *
 * `parallel.g4` remains authoritative for:
 *
 *     parallelConstruct;
 *     parallelExpression;
 *     parallelStatement;
 *     parallelComputationExpression.
 *
 * This file does not duplicate them.
 *
 * ============================================================================
 * INTEGRATION WITH TASK-PARALLEL.G4
 * ============================================================================
 *
 * `task-parallel.g4` remains authoritative for:
 *
 *     taskParallelConstruct;
 *     taskParallelGroup;
 *     taskParallelItem;
 *     nested task-parallel groups.
 *
 * This file consumes those rules only.
 *
 * ============================================================================
 * INTEGRATION WITH DATA-PARALLEL.G4
 * ============================================================================
 *
 * Data-parallel syntax remains owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * This file does NOT redefine:
 *
 *     dataParallelReduce;
 *     dataParallelMap;
 *     dataParallelScan;
 *     dataParallelPartition;
 *
 * A distributed data-parallel operation is represented by the normal
 * data-parallel syntax and classified semantically as distributed when the
 * semantic/resource model requires it.
 *
 * This prevents duplicate parsing paths.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCE GRAMMARS
 * ============================================================================
 *
 * Resource requirements remain owned by the resource/capability architecture.
 *
 * This file does not create distributed-specific resource syntax merely for
 * parser convenience.
 *
 * ============================================================================
 * INTEGRATION WITH EXECUTION / SCHEDULING
 * ============================================================================
 *
 * Execution and scheduling consume semantic information after parsing.
 *
 * This file does not select:
 *
 *     scheduler;
 *     executor;
 *     worker pool;
 *     placement strategy;
 *     routing algorithm;
 *     retry strategy;
 *     load-balancing strategy.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum constructs encountered inside distributed concurrency continue through
 * the ordinary quantum frontend and:
 *
 *     quantum::ir
 *
 * There is no distributed quantum parser IR.
 *
 * ============================================================================
 * INTEGRATION WITH HDL
 * ============================================================================
 *
 * HDL constructs remain under:
 *
 *     grammar/hdl/
 *
 * Hardware realization remains under:
 *
 *     grammar/hardware/
 *
 * This file only provides the concurrency/distribution composition boundary.
 *
 * ============================================================================
 * INTEGRATION WITH NETWORKING
 * ============================================================================
 *
 * Communication and transport remain under:
 *
 *     grammar/networking/
 *
 * Distributed concurrency expresses logical communication relationships;
 * networking determines implementation.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file is complete only when the integration tests cover the following.
 *
 * ---------------------------------------------------------------------------
 * POSITIVE
 * ---------------------------------------------------------------------------
 *
 * Existing distributed constructs must remain parseable through:
 *
 *     distributedConcurrencyDistributed
 *
 * Existing future constructs must remain parseable through:
 *
 *     distributedConcurrencyFuture
 *
 * Existing parallel constructs must remain parseable through:
 *
 *     distributedConcurrencyParallel
 *
 * Existing task-parallel constructs must remain parseable through:
 *
 *     distributedConcurrencyTaskParallel
 *
 * ---------------------------------------------------------------------------
 * NEGATIVE
 * ---------------------------------------------------------------------------
 *
 * The grammar must reject malformed source inherited from the owning grammar,
 * including incomplete:
 *
 *     distributed operations;
 *     future expressions;
 *     parallel expressions;
 *     task-parallel groups;
 *     distributed statements.
 *
 * The integration grammar must not weaken the owning grammar's diagnostics.
 *
 * ---------------------------------------------------------------------------
 * OWNERSHIP / NON-DUPLICATION
 * ---------------------------------------------------------------------------
 *
 * Tests must verify that:
 *
 *     parallel compute();
 *
 * is owned by Parallel rather than being transformed into a new distributed
 * syntax.
 *
 * Likewise:
 *
 *     parallel group { ... }
 *
 * remains owned by TaskParallel.
 *
 * Distributed operation forms remain owned by Distributed.
 *
 * ---------------------------------------------------------------------------
 * CROSS-DOMAIN
 * ---------------------------------------------------------------------------
 *
 * Test composition with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware intent;
 *     AI/ML computation;
 *     data-parallel computation;
 *     networking;
 *     security;
 *     accelerator intent.
 *
 * The test verifies grammar composition, not target allocation.
 *
 * ---------------------------------------------------------------------------
 * SCALABILITY
 * ---------------------------------------------------------------------------
 *
 * Generate tests containing increasing numbers of logical:
 *
 *     tasks;
 *     futures;
 *     distributed operations;
 *     parallel regions;
 *     nested structured groups;
 *     dependency relationships.
 *
 * No test may establish a fixed universal maximum.
 *
 * ---------------------------------------------------------------------------
 * DETERMINISM
 * ---------------------------------------------------------------------------
 *
 * Identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     grammar composition;
 *
 * must yield equivalent parse structure independent of:
 *
 *     hardware;
 *     node count;
 *     worker count;
 *     runtime state;
 *     environment;
 *     randomness.
 *
 * ---------------------------------------------------------------------------
 * COMPATIBILITY
 * ---------------------------------------------------------------------------
 *
 * Verify that adding this grammar does not change the meaning of existing:
 *
 *     task syntax;
 *     future syntax;
 *     parallel syntax;
 *     task-parallel syntax;
 *     distributed syntax.
 *
 * In particular, no existing construct should gain a second parse path merely
 * because this adapter grammar was imported.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 * This file contains:
 *
 *     NO fixed CPU count;
 *     NO fixed core count;
 *     NO fixed thread count;
 *     NO fixed worker count;
 *     NO fixed GPU count;
 *     NO fixed FPGA count;
 *     NO fixed QPU count;
 *     NO fixed node count;
 *     NO fixed device count;
 *     NO fixed memory size;
 *     NO fixed topology;
 *     NO physical device identifier;
 *     NO physical address;
 *     NO universal deployment limit.
 *
 * The grammar is open-ended through canonical qualified names and imported
 * grammar repetition.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This file must remain an adapter rather than a second implementation of the
 * distributed and concurrency grammars.
 *
 * That keeps:
 *
 *     grammar size;
 *     generated parser size;
 *     ambiguity surface;
 *     maintenance surface;
 *
 * smaller than a duplicated distributed-concurrency grammar.
 *
 * Parser performance optimizations must not change semantic ownership.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * No parser action performs:
 *
 *     network access;
 *     filesystem access;
 *     process execution;
 *     environment inspection;
 *     credential access;
 *     hardware discovery.
 *
 * Distributed execution security is validated downstream.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] Existing filename conventions are preserved.
 * [x] No existing grammar is renamed.
 * [x] No lexer token is unnecessarily added.
 * [x] Canonical distributed grammar remains authoritative.
 * [x] Canonical task grammar remains authoritative.
 * [x] Canonical future grammar remains authoritative.
 * [x] Canonical parallel grammar remains authoritative.
 * [x] Canonical task-parallel grammar remains authoritative.
 * [x] No distributed syntax is duplicated.
 * [x] No spawn syntax is duplicated.
 * [x] No await syntax is duplicated.
 * [x] No generic parallel syntax is duplicated.
 * [x] No task-parallel syntax is duplicated.
 * [x] No data-parallel syntax is duplicated.
 * [x] No new physical-resource vocabulary is required.
 * [x] No machine-size limit is encoded.
 * [x] No hardware topology is encoded.
 * [x] No runtime implementation is embedded.
 * [x] No second quantum IR is created.
 * [x] Quantum integration remains through `quantum::ir`.
 * [x] Distributed networking remains downstream.
 * [x] Scheduling remains downstream.
 * [x] Placement remains downstream.
 * [x] Resource/capability analysis remains downstream.
 * [x] Safe Rust 1.97/1.97.1 compatibility is preserved.
 * [x] No unsafe Rust is required.
 * [x] Positive tests are defined.
 * [x] Negative tests are defined.
 * [x] Boundary tests are defined.
 * [x] Scalability tests are defined.
 * [x] Determinism tests are defined.
 * [x] Compatibility tests are defined.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * The distributed-concurrency grammar is therefore:
 *
 *     SOURCE SYNTAX
 *          |
 *          v
 *     EXISTING CANONICAL GRAMMARS
 *          |
 *          v
 *     DISTRIBUTED-CONCURRENCY ADAPTER
 *          |
 *          v
 *     DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     SEMANTIC ANALYSIS
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     concurrency semantics       distributed semantics
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *              resource/capability analysis
 *                        |
 *                        v
 *                 canonical IR
 *                        |
 *             +----------+----------+
 *             |                     |
 *             v                     v
 *        classical             quantum::ir
 *             |                     |
 *             +----------+----------+
 *                        |
 *                        v
 *                 optimization
 *                        |
 *                 routing/placement
 *                        |
 *                    scheduling
 *                        |
 *                 resilience/QEC/ZQN
 *                        |
 *                       HAL
 *                        |
 *                 target realization
 *
 * The same source therefore remains usable from:
 *
 *     atom
 *       ->
 *     embedded
 *       ->
 *     CPU
 *       ->
 *     multicore
 *       ->
 *     GPU
 *       ->
 *     FPGA
 *       ->
 *     ASIC
 *       ->
 *     QPU / simulator
 *       ->
 *     accelerator
 *       ->
 *     cluster
 *       ->
 *     HPC
 *       ->
 *     cloud
 *       ->
 *     future computational substrates
 *
 * subject to the resources and capabilities actually available.
 *
 * ============================================================================
 * END
 * ============================================================================
 */