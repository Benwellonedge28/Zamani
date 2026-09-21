/*
 * ============================================================================
 * Zamani Universal Computing Language
 * Statement-Layer Concurrency Integration Grammar
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/concurrency.g4
 *
 * STATUS
 * ------
 * PRODUCTION STATEMENT-INTEGRATION ADAPTER
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * COMPILER BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021 edition
 * Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the STATEMENT-LAYER integration boundary for concurrency.
 *
 * It does NOT own the concurrency language.
 *
 * The concurrency domain itself is owned by:
 *
 *     grammar/concurrency/
 *
 * and its canonical composition boundary:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * This file exists because the universal statement grammar needs a stable,
 * statement-context entry point through which concurrency constructs can enter
 * the language.
 *
 * The architectural relationship is:
 *
 *
 *     grammar/antlr/ZamaniLexer.g4
 *                    |
 *                    v
 *     grammar/antlr/ZamaniParser.g4
 *                    |
 *          +---------+----------+
 *          |                    |
 *          v                    v
 *     Statements            Concurrency
 *          |                    |
 *          |                    v
 *          |        grammar/concurrency/
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *        statement-level adapter
 *                    |
 *                    v
 *          frontend/domain-neutral AST
 *
 *
 * This file therefore MUST remain thin.
 *
 * ============================================================================
 * PRIMARY OWNERSHIP RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - statement-context admission of concurrency;
 *     - the statement-layer adapter name;
 *     - statement-level integration with the universal Statements grammar;
 *     - documentation of the concurrency/statement boundary;
 *     - stable integration naming for tooling.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - concurrency domain syntax;
 *     - task syntax;
 *     - async syntax;
 *     - await syntax;
 *     - spawn syntax;
 *     - parallel syntax;
 *     - data-parallel syntax;
 *     - task-parallel syntax;
 *     - actor syntax;
 *     - channel syntax;
 *     - cancellation syntax;
 *     - synchronization syntax;
 *     - future syntax;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - blocks;
 *     - declarations;
 *     - effects;
 *     - memory;
 *     - resources;
 *     - capabilities;
 *     - scheduling;
 *     - placement;
 *     - routing;
 *     - optimization;
 *     - runtime execution;
 *     - hardware selection;
 *     - topology;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * Those responsibilities belong to their canonical owners.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Zamani deliberately separates:
 *
 *     DOMAIN OWNERSHIP
 *
 * from:
 *
 *     CONTEXT INTEGRATION.
 *
 * `grammar/concurrency/concurrency.g4` answers:
 *
 *     "What belongs to the concurrency domain?"
 *
 * This file answers:
 *
 *     "How does an already-defined concurrency construct enter statement
 *      context?"
 *
 * That distinction prevents the statement grammar from becoming a second
 * concurrency language.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one owner for each concurrency production.
 *
 * For example:
 *
 *     taskSpawnStatement
 *         -> grammar/concurrency/tasks.g4
 *
 *     parallelStatement
 *         -> grammar/concurrency/parallel.g4
 *
 *     channelStatement
 *         -> grammar/concurrency/channels.g4
 *
 *     synchronizationStatement
 *         -> grammar/concurrency/synchronization.g4
 *
 * This file MUST NOT redefine any of them.
 *
 * The concurrency-domain composition grammar remains responsible for composing
 * those independently owned constructs.
 *
 * ============================================================================
 * ANTLR COMPOSITION MODEL
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical token vocabulary is supplied by:
 *
 *     ZamaniLexer
 *
 * This file contains NO lexer rules.
 *
 * This file also contains NO parser actions, semantic predicates, embedded
 * Rust, target-language code, filesystem operations, network operations,
 * hardware discovery, or runtime behavior.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The imported `Concurrency` grammar is the domain-level concurrency
 * composition authority.
 *
 * Therefore:
 *
 *     Statements/ConcurrencyStatements
 *                |
 *                v
 *            Concurrency
 *                |
 *        +-------+--------+
 *        |       |        |
 *       Tasks  Parallel  ...
 *
 * This adapter must never import every concurrency leaf directly.
 *
 * Doing so would create a second domain composition hierarchy.
 *
 * ============================================================================
 * IMPORTANT: GRAMMAR NAME
 * ============================================================================
 *
 * The grammar name is intentionally:
 *
 *     ConcurrencyStatements
 *
 * rather than:
 *
 *     Concurrency
 *
 * because `Concurrency` is already owned by:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * and the canonical parser already imports that domain grammar.
 *
 * The distinct grammar name prevents an accidental duplicate ANTLR grammar
 * authority.
 *
 * ============================================================================
 */

parser grammar ConcurrencyStatements;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * DOMAIN COMPOSITION IMPORT
 * ============================================================================
 *
 * The complete concurrency-domain composition is imported once.
 *
 * This file does NOT import:
 *
 *     Tasks
 *     Futures
 *     Parallel
 *     DataParallel
 *     TaskParallel
 *     Actors
 *     Channels
 *     Cancellation
 *     Synchronization
 *
 * individually.
 *
 * Those are already composed by the canonical `Concurrency` grammar.
 *
 * This gives the repository the following ownership graph:
 *
 *
 *     ZamaniParser
 *          |
 *          +--> Statements
 *          |       |
 *          |       +--> ConcurrencyStatements
 *          |               |
 *          |               +--> Concurrency
 *          |
 *          +--> Concurrency
 *
 *
 * The important invariant is that both paths ultimately refer to the SAME
 * concurrency-domain grammar rather than defining independent syntax.
 *
 * ============================================================================
 */

import Concurrency;


/*
 * ============================================================================
 * 1. PUBLIC STATEMENT-LAYER ADAPTER
 * ============================================================================
 *
 * This is the ONLY production owned by this file that the universal statement
 * grammar needs to consume.
 *
 * It deliberately has a different name from `concurrencyStatement`.
 *
 * Reason:
 *
 *     `concurrencyStatement`
 *
 * belongs to the concurrency domain.
 *
 *     `concurrencyStatementAdapter`
 *
 * belongs to the statement-context integration layer.
 *
 * This distinction prevents a second generic concurrency statement authority.
 *
 * ============================================================================
 */

concurrencyStatementAdapter
    : concurrencyStatement
    ;


/*
 * ============================================================================
 * 2. STATEMENT-CONTEXT CLASSIFICATION BOUNDARY
 * ============================================================================
 *
 * Tooling may need to identify that a statement originated from the
 * concurrency domain without inspecting individual task/parallel/channel/
 * actor/etc. productions.
 *
 * This rule provides that classification boundary.
 *
 * It does NOT create a second syntax model.
 *
 * ============================================================================
 */

concurrencyStatementDomain
    : concurrencyStatementAdapter
    ;


/*
 * ============================================================================
 * 3. CONCURRENCY STATEMENT INTEGRATION CONTRACT
 * ============================================================================
 *
 * A successful match means only:
 *
 *     "the source has the structural shape of a concurrency statement."
 *
 * It does NOT mean:
 *
 *     - the statement is semantically valid;
 *     - its operands have valid types;
 *     - its effects are legal;
 *     - its resources are available;
 *     - its capabilities are supported;
 *     - its dependencies are satisfiable;
 *     - its parallelism can be realized;
 *     - its cancellation policy is executable;
 *     - its synchronization is race-free;
 *     - its target is capable of executing it.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 4. AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar creates NO AST implementation type.
 *
 * The parser/frontend layer must lower the resulting parse structure into the
 * repository's existing domain-neutral AST.
 *
 * The AST must preserve:
 *
 *     - source span;
 *     - source ordering;
 *     - concurrency construct identity;
 *     - nested expressions;
 *     - nested statements;
 *     - declarations where applicable;
 *     - attributes/modifiers;
 *     - names/references;
 *     - dependency relationships;
 *     - syntactic operands.
 *
 * The AST MUST NOT gain machine-specific information merely because a source
 * statement is concurrent.
 *
 * In particular, this adapter must never imply:
 *
 *     ThreadId
 *     WorkerId
 *     CoreId
 *     CpuId
 *     GpuId
 *     FpgaId
 *     QpuId
 *     NodeId
 *     DeviceId
 *     PhysicalQueueId
 *     PhysicalAddress
 *
 * Those belong to downstream target-specific realization where explicitly
 * required.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 5. EXPRESSION INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file does NOT define:
 *
 *     expression
 *     assignment
 *     call
 *     await
 *     spawn
 *     indexing
 *     member access
 *     operators
 *     literals
 *
 * Those constructs remain owned by the canonical expression and concurrency
 * grammars.
 *
 * A concurrency statement may contain expressions because its owning domain
 * grammar consumes the canonical expression boundary.
 *
 * Therefore the dependency direction remains:
 *
 *
 *     canonical expressions
 *             ^
 *             |
 *     concurrency leaf
 *             ^
 *             |
 *     Concurrency
 *             ^
 *             |
 *     ConcurrencyStatements
 *
 *
 * There is no expression implementation in this file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 6. BLOCK INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file does not define blocks.
 *
 * A concurrency construct requiring a body must use the canonical block/body
 * production owned by the appropriate concurrency or core grammar.
 *
 * This prevents:
 *
 *     statements/concurrency.g4
 *
 * from becoming a second block grammar.
 *
 * Nested concurrency remains legal whenever the owning concurrency construct
 * permits nested statements according to its semantic contract.
 *
 * No nesting depth is hard-coded here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 7. DECLARATION SEPARATION
 * ============================================================================
 *
 * Concurrency declarations are NOT automatically statements.
 *
 * Examples include domain-specific declarations such as:
 *
 *     actor declarations;
 *     channel declarations;
 *     synchronization declarations;
 *     cancellation declarations;
 *
 * Their legality in declaration context is determined by:
 *
 *     grammar/declarations/
 *     grammar/concurrency/
 *
 * and the canonical parser composition.
 *
 * This file admits only the statement-level concurrency boundary.
 *
 * This prevents declarations from accidentally becoming legal wherever a
 * generic statement is accepted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 8. EXPRESSION-STATEMENT SEPARATION
 * ============================================================================
 *
 * Some concurrency constructs may have both expression and statement forms.
 *
 * Examples include concepts such as:
 *
 *     spawn
 *     await
 *     send
 *     receive
 *     cancellation observation
 *
 * This adapter does not attempt to decide whether an expression should be
 * interpreted as a statement.
 *
 * The owning concurrency grammar determines the syntactic form.
 *
 * The canonical statement grammar remains responsible for ordinary:
 *
 *     expression SEMICOLON
 *
 * integration.
 *
 * This avoids introducing precedence or ambiguity hacks into this adapter.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. TASK INTEGRATION
 * ============================================================================
 *
 * Task syntax remains owned by:
 *
 *     grammar/concurrency/tasks.g4
 *
 * This file therefore does NOT contain:
 *
 *     taskSpawnStatement
 *     taskAwaitStatement
 *     taskParallelStatement
 *     taskStatement
 *
 * implementations.
 *
 * The dependency is:
 *
 *     tasks.g4
 *          |
 *          v
 *     Concurrency
 *          |
 *          v
 *     concurrencyStatement
 *          |
 *          v
 *     concurrencyStatementAdapter
 *
 * The same task syntax can therefore participate in other contexts without
 * copying its grammar into statements/concurrency.g4.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. PARALLEL INTEGRATION
 * ============================================================================
 *
 * General parallel syntax remains owned by:
 *
 *     grammar/concurrency/parallel.g4
 *
 * This adapter does not define:
 *
 *     parallel
 *     parallel regions
 *     parallel work items
 *     parallel iteration
 *     parallel reduction
 *     parallel dependencies
 *
 * A parallel statement expresses semantic opportunity/intent.
 *
 * It does NOT specify:
 *
 *     worker count;
 *     thread count;
 *     CPU count;
 *     GPU count;
 *     accelerator count;
 *     QPU count;
 *     execution width;
 *     physical placement.
 *
 * Those are downstream scheduling/resource decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. DATA-PARALLEL INTEGRATION
 * ============================================================================
 *
 * Data-parallel syntax remains owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * This adapter does not define:
 *
 *     SIMD width;
 *     vector width;
 *     warp width;
 *     block size;
 *     GPU count;
 *     accelerator count;
 *     partition count;
 *     physical data placement.
 *
 * Logical data domains remain independent of target realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. TASK-PARALLEL INTEGRATION
 * ============================================================================
 *
 * Task-parallel syntax remains owned by:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * This adapter does not define:
 *
 *     task count limits;
 *     worker counts;
 *     dependency storage;
 *     scheduler implementation;
 *     DAG representation;
 *     queue representation.
 *
 * Logical task dependencies belong to the semantic/task-parallel layer.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. FUTURE / ASYNC INTEGRATION
 * ============================================================================
 *
 * Future and asynchronous syntax remains owned by:
 *
 *     grammar/concurrency/futures.g4
 *     grammar/concurrency/tasks.g4
 *
 * This adapter does not introduce a FUTURE token and does not define a
 * future implementation.
 *
 * A future-like computation may ultimately be realized:
 *
 *     locally;
 *     asynchronously;
 *     remotely;
 *     distributively;
 *     on an accelerator;
 *     through quantum/classical orchestration;
 *     on another computational substrate.
 *
 * The source statement remains target-independent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. ACTOR INTEGRATION
 * ============================================================================
 *
 * Actor syntax remains owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * This file does not define:
 *
 *     actor declarations;
 *     actor spawning;
 *     actor messaging;
 *     actor lifecycle;
 *     supervision;
 *     mailbox semantics.
 *
 * In particular:
 *
 *     actor != thread
 *     actor != process
 *     actor != node
 *     actor != machine
 *
 * These are possible implementation realizations, not parser semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. CHANNEL INTEGRATION
 * ============================================================================
 *
 * Channel syntax remains owned by:
 *
 *     grammar/concurrency/channels.g4
 *
 * This adapter does not define:
 *
 *     send;
 *     receive;
 *     select;
 *     close;
 *     buffering;
 *     queue implementation;
 *     transport.
 *
 * A channel may ultimately be realized using:
 *
 *     local communication;
 *     shared memory;
 *     accelerator communication;
 *     hardware FIFOs;
 *     interconnects;
 *     networking;
 *     distributed transport;
 *     another communication substrate.
 *
 * None of those implementation choices belong here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. CANCELLATION INTEGRATION
 * ============================================================================
 *
 * Cancellation syntax remains owned by:
 *
 *     grammar/concurrency/cancellation.g4
 *
 * This adapter does not implement cancellation propagation.
 *
 * It does not define:
 *
 *     OS signals;
 *     thread interruption;
 *     process termination;
 *     device cancellation;
 *     rollback;
 *     retry;
 *     recovery.
 *
 * Those are semantic/runtime/resilience concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. SYNCHRONIZATION INTEGRATION
 * ============================================================================
 *
 * Synchronization syntax remains owned by:
 *
 *     grammar/concurrency/synchronization.g4
 *
 * This adapter does not define:
 *
 *     mutex algorithms;
 *     futexes;
 *     spinlocks;
 *     hardware atomics;
 *     lock implementations;
 *     scheduler behavior;
 *     fairness algorithms.
 *
 * It merely admits an already-defined synchronization statement into the
 * concurrency statement domain.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Concurrency may surround classical computation.
 *
 * This file does not select:
 *
 *     CPU;
 *     core;
 *     SIMD width;
 *     vector register width;
 *     thread;
 *     worker pool.
 *
 * Classical semantic/IR lowering remains downstream.
 *
 * A resource-poor target may serialize logically independent computation.
 *
 * A resource-rich target may exploit available parallelism.
 *
 * Both are compatible with POCO-REAF when semantic guarantees are preserved.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Concurrency may surround or coordinate quantum computation.
 *
 * This file does NOT define:
 *
 *     qubits;
 *     quantum gates;
 *     quantum operations;
 *     physical qubit IDs;
 *     logical-to-physical mappings;
 *     quantum circuits;
 *     QEC;
 *     ZQN;
 *     routing;
 *     pulse scheduling;
 *     calibration.
 *
 * Quantum syntax continues through the canonical path:
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
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This file creates no quantum-specific IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Concurrency statements may contain or coordinate computations whose eventual
 * realization involves HDL or hardware/software co-design.
 *
 * This file does not define:
 *
 *     clocks;
 *     signals;
 *     wires;
 *     registers;
 *     hardware processes;
 *     pipelines;
 *     hardware modules.
 *
 * Those remain owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * A parallel source construct does not imply a fixed hardware replication
 * factor.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Concurrency does not imply distribution.
 *
 * A concurrency construct may remain:
 *
 *     local;
 *     process-local;
 *     accelerator-local;
 *     distributed;
 *     heterogeneous.
 *
 * Distribution remains owned by:
 *
 *     grammar/distributed/
 *
 * Placement and topology are downstream concerns.
 *
 * No node count or network topology is encoded here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Concurrency may surround:
 *
 *     tensor operations;
 *     data pipelines;
 *     model training;
 *     model inference;
 *     streaming computation;
 *     distributed learning.
 *
 * AI/data semantics remain owned by:
 *
 *     grammar/ai/
 *     grammar/data/
 *
 * This file does not introduce framework-specific syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. NETWORKING INTEGRATION
 * ============================================================================
 *
 * Concurrency may participate in networked computation.
 *
 * Networking syntax remains owned by:
 *
 *     grammar/networking/
 *
 * This file does not interpret a concurrent operation as:
 *
 *     socket;
 *     packet;
 *     connection;
 *     network node.
 *
 * Such interpretations belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Concurrency does not own `requires` syntax.
 *
 * Requirements and capabilities remain owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * The concurrency semantic layer may derive requirements from source
 * constructs.
 *
 * For example, a parallel construct might cause semantic analysis to derive:
 *
 *     requirement: parallel_execution
 *
 * or:
 *
 *     capability: concurrent_execution
 *
 * without the statement grammar selecting a physical implementation.
 *
 * This distinction is fundamental:
 *
 *
 *     requirement
 *         !=
 *     physical resource selection
 *
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. EFFECT INTEGRATION
 * ============================================================================
 *
 * Concurrency constructs may introduce effects such as:
 *
 *     asynchronous;
 *     concurrent;
 *     parallel;
 *     communication;
 *     synchronization;
 *     cancellation;
 *     distributed interaction.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * and the semantic effect system.
 *
 * This file does not encode effect implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. MEMORY / OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * Concurrency must use the canonical ownership, borrowing, aliasing and
 * lifetime model.
 *
 * This file does not create a second memory model.
 *
 * Semantic analysis remains responsible for determining whether a concurrent
 * access is legal.
 *
 * For example, syntactic validity does not guarantee that two concurrent
 * mutations are semantically race-free.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. DETERMINISM
 * ============================================================================
 *
 * This grammar is parser-deterministic.
 *
 * Its result depends only upon:
 *
 *     source tokens;
 *     grammar version;
 *     parser configuration;
 *     explicitly selected dialect configuration.
 *
 * It MUST NOT depend upon:
 *
 *     hardware availability;
 *     scheduler state;
 *     runtime timing;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     environment state;
 *     device discovery.
 *
 * Runtime scheduling nondeterminism is separate from parser determinism.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. POCO-REAF SCALABILITY
 * ============================================================================
 *
 * This adapter introduces NO grammar-level finite machine limits.
 *
 * In particular, it does not define:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CHANNELS
 *     MAX_ACTORS
 *     MAX_PARALLELISM
 *     MAX_DATA_ITEMS
 *     MAX_DEPENDENCIES
 *     MAX_NESTING
 *     MAX_MEMORY
 *     MAX_STORAGE
 *
 * Repetition and nesting are represented by the underlying grammar structure.
 *
 * "Infinity" means:
 *
 *     no artificial language-level machine ceiling.
 *
 * Actual execution remains bounded by available:
 *
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     deployment policy;
 *     explicit program constraints.
 *
 * Those limits must never be silently promoted into grammar constants.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. NUMERIC-LITERAL RULE
 * ============================================================================
 *
 * Numeric values occurring in source code are program semantics.
 *
 * This file must never reinterpret a numeric value as an implementation limit.
 *
 * Therefore:
 *
 *     1024
 *
 * may be a perfectly valid source value.
 *
 * What is prohibited is a grammar-level rule such as:
 *
 *     concurrencyWidth : [1..1024]
 *
 * when 1024 is merely an implementation limitation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. NO HARDWARE BINDING
 * ============================================================================
 *
 * This adapter must never introduce syntax equivalent to:
 *
 *     run_on_core(...)
 *     run_on_gpu(...)
 *     run_on_qpu(...)
 *     run_on_node(...)
 *     use_thread(...)
 *     use_worker(...)
 *     use_device(...)
 *
 * unless such constructs are independently established as explicit,
 * target-specific language features by the resource/hardware/dialect
 * specification.
 *
 * Even then, those features must not be silently introduced through this file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. NO RESOURCE OWNERSHIP
 * ============================================================================
 *
 * This file does not allocate resources.
 *
 * It does not:
 *
 *     create threads;
 *     create workers;
 *     allocate CPUs;
 *     allocate GPUs;
 *     allocate QPUs;
 *     allocate nodes;
 *     allocate channels;
 *     reserve memory;
 *     select devices.
 *
 * It only provides syntax composition.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. NO SCHEDULING
 * ============================================================================
 *
 * A concurrency statement can expose logical parallelism.
 *
 * It does not establish physical execution order.
 *
 * The downstream scheduler determines a legal realization based upon:
 *
 *     dependencies;
 *     effects;
 *     resource requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     correctness requirements;
 *     target characteristics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. NO RUNTIME DEPENDENCY
 * ============================================================================
 *
 * The grammar must be usable without:
 *
 *     a runtime;
 *     an executor;
 *     an operating system;
 *     a scheduler;
 *     a physical device;
 *     a network;
 *     hardware discovery.
 *
 * This is necessary for:
 *
 *     parsing;
 *     formatting;
 *     static analysis;
 *     language servers;
 *     documentation generation;
 *     source transformation;
 *     deterministic compilation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. ERROR BOUNDARY
 * ============================================================================
 *
 * This file introduces no custom error actions.
 *
 * Structural errors are reported by the canonical parser/diagnostic layer.
 *
 * Examples of syntax failures include:
 *
 *     malformed concurrency statement;
 *     missing required concurrency operand;
 *     malformed concurrency body;
 *     invalid statement-level structure.
 *
 * Errors such as:
 *
 *     insufficient workers;
 *     insufficient memory;
 *     unavailable GPU;
 *     unavailable QPU;
 *     unsupported topology;
 *     resource exhaustion;
 *     invalid synchronization semantics;
 *
 * are NOT parser errors.
 *
 * They belong to semantic/resource/target/runtime layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. AST / SEMANTIC / IR PIPELINE
 * ============================================================================
 *
 * The complete path remains:
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
 *     statement adapter
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     name/type/effect analysis
 *          |
 *          v
 *     concurrency/dependency analysis
 *          |
 *          v
 *     resource/capability analysis
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical representation      quantum::ir
 *          |                             |
 *          +--------------+--------------+
 *                         |
 *                         v
 *                    optimization
 *                         |
 *                    routing/scheduling
 *                         |
 *                     resilience
 *                         |
 *                      ZQN/HAL
 *                         |
 *                         v
 *                  target realization
 *
 * This file participates only in the parser stage.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. CROSS-DOMAIN INVARIANT
 * ============================================================================
 *
 * Concurrency is orthogonal to computational domain.
 *
 * The same concurrency syntax may surround:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware co-design;
 *     AI computation;
 *     data processing;
 *     networking;
 *     distributed computation;
 *     accelerator computation;
 *     future computational domains.
 *
 * This file therefore must not create domain-specific copies such as:
 *
 *     quantumConcurrencyStatement
 *     gpuConcurrencyStatement
 *     cpuConcurrencyStatement
 *     hdlConcurrencyStatement
 *
 * unless a separate language-domain specification explicitly establishes such
 * a syntax boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. COMPATIBILITY
 * ============================================================================
 *
 * This adapter is intentionally small so that concurrency-domain evolution
 * does not require repeated edits here.
 *
 * When a new concurrency feature is added:
 *
 *     1. define it in its owning concurrency grammar;
 *     2. expose it through the canonical `Concurrency` composition grammar;
 *     3. preserve the `concurrencyStatement` domain boundary;
 *     4. this adapter continues to consume that boundary unchanged.
 *
 * Therefore:
 *
 *     new concurrency feature
 *             |
 *             v
 *       owning grammar
 *             |
 *             v
 *       Concurrency
 *             |
 *             v
 *   concurrencyStatement
 *             |
 *             v
 * concurrencyStatementAdapter
 *
 * This is specifically designed to satisfy the requirement that this file
 * should not need to be reopened merely because another concurrency leaf was
 * expanded.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. DIALECT INTEGRATION
 * ============================================================================
 *
 * Dialects must not bypass this boundary.
 *
 * A concurrency dialect may extend the concurrency domain through the
 * repository's dialect mechanism, but the resulting construct must still
 * enter the canonical parser/AST/semantic pipeline.
 *
 * A dialect must not:
 *
 *     inject runtime behavior;
 *     bypass type checking;
 *     bypass resource checking;
 *     create a private concurrency IR;
 *     select hardware directly;
 *     introduce unsafe Rust requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. MACRO / METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Macro expansion may produce concurrency syntax.
 *
 * After expansion, generated syntax must pass through the same canonical
 * grammar/AST/semantic validation pipeline.
 *
 * Macros must not use this adapter as an escape hatch from:
 *
 *     type checking;
 *     effect checking;
 *     resource checking;
 *     capability checking;
 *     portability checking.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. SECURITY INVARIANT
 * ============================================================================
 *
 * Parsing concurrency syntax must not provide:
 *
 *     filesystem access;
 *     network access;
 *     secret access;
 *     hardware access;
 *     runtime execution;
 *     arbitrary code execution.
 *
 * The grammar is declarative and side-effect free.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. SAFE-RUST INVARIANT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The repository implementation consuming this grammar is required to support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021 edition
 *
 * without requiring:
 *
 *     unsafe Rust.
 *
 * No grammar construct here requires unsafe operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. TEST CONTRACT
 * ============================================================================
 *
 * Tests for this file must test INTEGRATION, not re-test every concurrency
 * leaf implementation.
 *
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 * At minimum, statement-level integration must accept representative valid
 * forms for:
 *
 *     task concurrency;
 *     async/task computation;
 *     await;
 *     parallel computation;
 *     data-parallel computation;
 *     task-parallel computation;
 *     actor operations;
 *     channel operations;
 *     cancellation;
 *     synchronization.
 *
 * The concrete examples belong in the corresponding domain test directories.
 *
 * ============================================================================
 *
 * NEGATIVE TESTS
 * --------------
 *
 * The adapter must reject malformed statement-level structures through the
 * canonical parser.
 *
 * It must not convert semantic failures into parser acceptance.
 *
 * ============================================================================
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Test:
 *
 *     one concurrency statement;
 *     many concurrency statements;
 *     nested concurrency;
 *     concurrency inside ordinary blocks;
 *     ordinary statements around concurrency;
 *     deeply nested valid structures where supported.
 *
 * ============================================================================
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Test increasing source sizes without introducing grammar-level resource
 * ceilings.
 *
 * No test may assert a universal maximum number of:
 *
 *     tasks;
 *     parallel regions;
 *     channels;
 *     actors;
 *     workers;
 *     devices;
 *     nodes;
 *     threads;
 *     qubits.
 *
 * ============================================================================
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Given identical:
 *
 *     source;
 *     grammar version;
 *     lexer configuration;
 *     dialect configuration;
 *
 * repeated parsing must produce equivalent structural results.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * The statement layer must be able to compose concurrency with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     resources.
 *
 * ============================================================================
 *
 * ROUND-TRIP TESTS
 * ----------------
 *
 * Where formatter support exists:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve the meaning of concurrency constructs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CHANNELS
 *     MAX_ACTORS
 *     MAX_PARALLELISM
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TIMELINES
 *
 * It contains no physical identifiers.
 *
 * It contains no fixed topology.
 *
 * It contains no target-specific scheduling rule.
 *
 * It contains no implementation-specific worker count.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [ ] Grammar name is unique.
 *
 *     [ ] It does not compete with `Concurrency`.
 *
 *     [ ] It imports the canonical concurrency composition boundary.
 *
 *     [ ] It exposes exactly one statement-layer adapter.
 *
 *     [ ] It does not redefine `statement`.
 *
 *     [ ] It does not redefine `concurrencyStatement`.
 *
 *     [ ] It does not redefine any concurrency leaf syntax.
 *
 *     [ ] It does not define lexer rules.
 *
 *     [ ] It does not define expressions.
 *
 *     [ ] It does not define types.
 *
 *     [ ] It does not define blocks.
 *
 *     [ ] It does not define declarations.
 *
 *     [ ] It does not define resources.
 *
 *     [ ] It does not define capabilities.
 *
 *     [ ] It does not define effects.
 *
 *     [ ] It does not define scheduling.
 *
 *     [ ] It does not define placement.
 *
 *     [ ] It does not define routing.
 *
 *     [ ] It does not define runtime behavior.
 *
 *     [ ] It does not define hardware topology.
 *
 *     [ ] It does not create a quantum IR.
 *
 *     [ ] It does not create a concurrency IR.
 *
 *     [ ] It contains no target-language actions.
 *
 *     [ ] It contains no semantic predicates.
 *
 *     [ ] It requires no unsafe Rust.
 *
 *     [ ] It supports Rust 1.97 / 1.97.1 through the repository frontend.
 *
 *     [ ] The canonical `Concurrency` grammar is valid and exposes
 *         `concurrencyStatement`.
 *
 *     [ ] `Statements` imports this grammar.
 *
 *     [ ] `Statements.statement` admits `concurrencyStatementAdapter`.
 *
 *     [ ] `ZamaniParser` receives the resulting integrated statement rule
 *         through `Statements`.
 *
 *     [ ] The standalone `Concurrency` import remains the domain authority.
 *
 *     [ ] No duplicate concurrency statement authority remains.
 *
 *     [ ] Positive integration tests pass.
 *
 *     [ ] Negative integration tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. FINAL ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 * ----------
 * `grammar/concurrency/` owns concurrency syntax.
 *
 * INVARIANT 2
 * ----------
 * `grammar/concurrency/concurrency.g4` owns concurrency-domain composition.
 *
 * INVARIANT 3
 * ----------
 * `grammar/statements/statements.g4` owns `statement`.
 *
 * INVARIANT 4
 * ----------
 * This file owns only the statement-layer adapter.
 *
 * INVARIANT 5
 * ----------
 * No leaf concurrency grammar is duplicated here.
 *
 * INVARIANT 6
 * ----------
 * No lexer rule is duplicated here.
 *
 * INVARIANT 7
 * ----------
 * No type system is duplicated here.
 *
 * INVARIANT 8
 * ----------
 * No semantic analysis is performed here.
 *
 * INVARIANT 9
 * ----------
 * No resource is selected here.
 *
 * INVARIANT 10
 * -----------
 * No scheduler decision is made here.
 *
 * INVARIANT 11
 * -----------
 * No runtime behavior is implemented here.
 *
 * INVARIANT 12
 * -----------
 * Quantum computation continues through the canonical `quantum::ir` boundary.
 *
 * INVARIANT 13
 * -----------
 * Hardware limits never become language limits.
 *
 * INVARIANT 14
 * -----------
 * Logical concurrency remains independent of physical execution width.
 *
 * INVARIANT 15
 * -----------
 * The same source-level concurrency semantics can be realized on different
 * available resources without rewriting this grammar.
 *
 * INVARIANT 16
 * -----------
 * The grammar contains no unsafe Rust and requires no unsafe Rust.
 *
 * INVARIANT 17
 * -----------
 * POCO-REAF is preserved:
 *
 *     Program Once
 *       -> Compile Once
 *       -> Run Everywhere
 *       -> Anywhere
 *       -> Forever
 *
 * ============================================================================
 * END OF grammar/statements/concurrency.g4
 * ============================================================================
 */