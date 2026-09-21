/*
 * ============================================================================
 * Zamani — Concurrency Grammar Composition Boundary
 * ============================================================================
 *
 * File:
 *   grammar/concurrency/concurrency.g4
 *
 * Purpose:
 *   Canonical composition boundary for all Zamani concurrency constructs.
 *
 * Status:
 *   PRODUCTION
 *
 * Architectural role:
 *   This grammar composes independently owned concurrency grammars into one
 *   stable concurrency semantic category.
 *
 * This file is NOT:
 *   - a second concurrency language;
 *   - a task implementation;
 *   - a scheduler;
 *   - a runtime;
 *   - a resource manager;
 *   - a distributed runtime;
 *   - a hardware topology description;
 *   - a quantum IR;
 *   - an optimization pass;
 *   - a QEC implementation;
 *   - a ZQN implementation;
 *   - a HAL implementation.
 *
 * OWNERSHIP
 * ---------------------------------------------------------------------------
 *
 * This file owns:
 *   - concurrency grammar composition;
 *   - the public concurrency construct boundary;
 *   - stable semantic grouping of concurrency constructs;
 *   - integration of independently owned concurrency domains;
 *   - the relationship between expression-level and statement-level
 *     concurrency syntax.
 *
 * This file does NOT own:
 *   - async expression syntax;
 *   - await expression syntax;
 *   - spawn syntax;
 *   - task syntax;
 *   - future syntax;
 *   - generic parallel syntax;
 *   - data-parallel syntax;
 *   - task-parallel syntax;
 *   - actor syntax;
 *   - channel syntax;
 *   - cancellation syntax;
 *   - synchronization syntax;
 *   - reduction syntax;
 *   - distributed placement;
 *   - scheduling;
 *   - hardware resources.
 *
 * Those responsibilities remain in their dedicated grammar files.
 *
 * PORTABILITY / POCO-REAF
 * ---------------------------------------------------------------------------
 *
 * Concurrency syntax describes computation semantics and synchronization
 * intent, not the machine on which the computation happens.
 *
 * This grammar MUST NOT encode universal limits such as:
 *
 *   MAX_THREADS
 *   MAX_TASKS
 *   MAX_CORES
 *   MAX_NODES
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_QPUS
 *   MAX_CHANNELS
 *   MAX_ACTORS
 *   MAX_FUTURES
 *   MAX_PARALLELISM
 *
 * Nor may it encode physical identifiers such as:
 *
 *   cpu0
 *   gpu0
 *   node0
 *   core0
 *   thread0
 *   physical_device_0
 *
 * unless such identifiers are part of a target-specific downstream
 * representation rather than universal Zamani syntax.
 *
 * A Zamani program expresses:
 *
 *   - what may execute concurrently;
 *   - ordering requirements;
 *   - synchronization requirements;
 *   - communication semantics;
 *   - cancellation semantics;
 *   - task/future relationships;
 *   - parallel decomposition;
 *   - deterministic behavior requirements;
 *   - resource/capability requirements.
 *
 * Resource realization is resolved later by semantic analysis, compilation,
 * scheduling, runtime, deployment, and target backends.
 *
 * PIPELINE
 * ---------------------------------------------------------------------------
 *
 *   source
 *     |
 *     v
 *   lexer
 *     |
 *     v
 *   parser
 *     |
 *     v
 *   domain-neutral AST
 *     |
 *     v
 *   semantic analysis
 *     |
 *     +--> ownership / effects / capabilities
 *     |
 *     +--> resource requirements
 *     |
 *     +--> determinism / ordering
 *     |
 *     v
 *   canonical semantic IR
 *     |
 *     v
 *   optimization
 *     |
 *     v
 *   scheduling / placement / resilience
 *     |
 *     v
 *   runtime / HAL / target realization
 *
 * No runtime or scheduler behavior is implemented here.
 *
 * AST CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Concrete grammar constructs must lower to the repository's existing
 * domain-neutral frontend representation.
 *
 * The preferred operation shape is conceptually:
 *
 *   Operation {
 *       name,
 *       namespace,
 *       operands,
 *       parameters,
 *       results,
 *       attributes,
 *       modifiers,
 *       effects,
 *       capabilities,
 *       source,
 *   }
 *
 * This grammar MUST NOT introduce a concurrency-specific IR merely because
 * a new syntax category exists.
 *
 * SEMANTIC CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Semantic analysis is responsible for:
 *
 *   - determining task/future dependencies;
 *   - validating await relationships;
 *   - validating synchronization;
 *   - validating channel communication;
 *   - validating actor communication;
 *   - determining ordering requirements;
 *   - detecting invalid data races where the language semantics require it;
 *   - validating cancellation relationships;
 *   - checking deterministic-concurrency requirements;
 *   - resolving resource/capability requirements;
 *   - checking distributed execution requirements;
 *   - determining whether an operation can be lowered to a target.
 *
 * This grammar only establishes syntactic structure.
 *
 * RESOURCE CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Resource requirements belong to the resource/capability system.
 *
 * Do not add grammar such as:
 *
 *   run_on_8_threads
 *   use_gpu_0
 *   spawn_1024_threads
 *
 * as universal concurrency constructs.
 *
 * Instead, resource semantics should use the repository's resource and
 * capability mechanisms, for example conceptually:
 *
 *   requires capability("parallel.compute")
 *   requires capability("task.parallelism")
 *
 * The concrete resource grammar remains owned by grammar/resources/.
 *
 * DETERMINISM
 * ---------------------------------------------------------------------------
 *
 * The grammar itself must remain deterministic.
 *
 * Semantic determinism requirements are handled downstream.
 *
 * The parser must not choose a different interpretation merely because a
 * target machine has a different number of processors, accelerators, nodes,
 * channels, or other resources.
 *
 * ERROR / DIAGNOSTIC CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Errors caused by:
 *
 *   - malformed concurrency syntax
 *   - missing required operands
 *   - malformed task/future constructs
 *   - malformed synchronization constructs
 *   - malformed channel/actor constructs
 *
 * belong to parsing diagnostics.
 *
 * Errors caused by:
 *
 *   - unavailable resources
 *   - unsatisfied capabilities
 *   - impossible placement
 *   - invalid synchronization semantics
 *   - unsupported target realization
 *
 * belong to semantic analysis, compilation, or runtime diagnostics.
 *
 * SECURITY
 * ---------------------------------------------------------------------------
 *
 * This grammar introduces no executable actions.
 *
 * It must not:
 *
 *   - execute user code;
 *   - perform I/O;
 *   - access the filesystem;
 *   - access the network;
 *   - inspect hardware;
 *   - invoke schedulers;
 *   - invoke runtimes;
 *   - perform unsafe operations.
 *
 * Generated Rust consumers must remain compatible with Rust 1.97 / 1.97.1
 * and must not require unsafe Rust.
 *
 * PERFORMANCE
 * ---------------------------------------------------------------------------
 *
 * This file is intentionally a composition grammar.
 *
 * It must not duplicate large leaf grammars or introduce unnecessary
 * left-recursive/ambiguous alternatives.
 *
 * Each concrete concurrency construct has one owning grammar.
 *
 * TEST CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Required conformance categories:
 *
 *   positive:
 *     - async expressions
 *     - await
 *     - spawn
 *     - tasks
 *     - futures
 *     - parallel constructs
 *     - data parallelism
 *     - task parallelism
 *     - actors
 *     - channels
 *     - cancellation
 *     - synchronization
 *
 *   negative:
 *     - malformed constructs
 *     - missing operands
 *     - malformed channel operations
 *     - malformed synchronization
 *     - malformed task/future composition
 *
 *   boundary:
 *     - one task
 *     - one future
 *     - one actor
 *     - one channel
 *     - one parallel region
 *     - arbitrarily large program structure represented by repetition,
 *       subject only to parser/runtime resource availability
 *
 *   scalability:
 *     - no grammar-level maximum task count
 *     - no grammar-level maximum future count
 *     - no grammar-level maximum actor count
 *     - no grammar-level maximum channel count
 *     - no grammar-level maximum parallel regions
 *     - no grammar-level maximum nesting depth beyond implementation/runtime
 *       resource exhaustion
 *
 *   determinism:
 *     - identical source produces identical parse structure
 *
 *   compatibility:
 *     - existing canonical concurrency tokens remain valid
 *     - no obsolete token spelling is reintroduced
 *     - no duplicate syntax ownership is introduced
 *
 * HARD-CODING AUDIT
 * ---------------------------------------------------------------------------
 *
 * This file contains:
 *
 *   - no hardware counts;
 *   - no processor counts;
 *   - no fixed task counts;
 *   - no fixed thread counts;
 *   - no fixed node counts;
 *   - no fixed device identifiers;
 *   - no fixed memory sizes;
 *   - no fixed topology;
 *   - no fixed accelerator count.
 *
 * COMPLETION CRITERIA
 * ---------------------------------------------------------------------------
 *
 * This file is complete when:
 *
 *   [x] It is the canonical concurrency composition boundary.
 *   [x] Concrete syntax remains owned by specialized grammars.
 *   [x] Async expression ownership remains outside this file.
 *   [x] No stale task/future/data-parallel/task-parallel statement rules
 *       are referenced.
 *   [x] No duplicate concurrency syntax is introduced.
 *   [x] No hardware/resource limits are encoded.
 *   [x] No scheduler/runtime behavior is encoded.
 *   [x] No second concurrency IR is introduced.
 *   [x] Existing canonical tokens remain authoritative.
 *   [x] The grammar is parser-only and contains no target-language actions.
 *   [x] The public rule `concurrencyConstruct` is stable for downstream
 *       grammar composition.
 *
 * ============================================================================
 */

parser grammar Concurrency;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Each imported grammar owns a distinct concurrency concern.
 *
 * IMPORTANT:
 *
 * Do not replace these imports by copying their rules into this file.
 *
 * Doing so would create competing ownership and make future maintenance
 * require synchronized edits across multiple grammar files.
 * ============================================================================
 */

import
    AsyncExpressions,
    Tasks,
    Futures,
    Parallel,
    DataParallel,
    TaskParallel,
    Actors,
    Channels,
    Cancellation,
    Synchronization
;

/*
 * ============================================================================
 * PUBLIC COMPOSITION BOUNDARY
 * ============================================================================
 *
 * `concurrencyConstruct` is the only universal concurrency composition rule
 * that downstream grammar should normally consume.
 *
 * It intentionally groups expression-level and construct-level concurrency
 * syntax without redefining any of those constructs.
 *
 * Concrete ownership remains in the imported grammars.
 * ============================================================================
 */

concurrencyConstruct
    : concurrencyExpression
    | concurrencyStatementConstruct
    ;

/*
 * ============================================================================
 * EXPRESSION-LEVEL CONCURRENCY
 * ============================================================================
 *
 * Async expression syntax is owned by expressions/async.g4.
 *
 * This adapter deliberately references the canonical expression-level
 * constructs instead of recreating await/spawn/parallel syntax here.
 * ============================================================================
 */

concurrencyExpression
    : awaitExpression
    | spawnExpression
    | parallelExpression
    | futureExpression
    ;

/*
 * ============================================================================
 * CONSTRUCT-LEVEL CONCURRENCY
 * ============================================================================
 *
 * These rules are semantic composition adapters.
 *
 * They do not introduce new concrete syntax.
 * ============================================================================
 */

concurrencyStatementConstruct
    : taskConstruct
    | futureObservationRoot
    | futureResultRoot
    | parallelConstruct
    | dataParallelConstruct
    | taskParallelConstruct
    | actorConstruct
    | channelConstruct
    | cancellationConstruct
    | synchronizationConstruct
    ;

/*
 * ============================================================================
 * STABLE DOMAIN ADAPTERS
 * ============================================================================
 *
 * These names provide explicit semantic categories for downstream tooling
 * without requiring consumers to know every concrete concurrency grammar.
 *
 * They are aliases only; they do not own concrete syntax.
 * ============================================================================
 */

taskConcurrencyConstruct
    : taskConstruct
    ;

futureConcurrencyConstruct
    : futureExpression
    | futureObservationRoot
    | futureResultRoot
    ;

parallelConcurrencyConstruct
    : parallelConstruct
    ;

dataParallelConcurrencyConstruct
    : dataParallelConstruct
    ;

taskParallelConcurrencyConstruct
    : taskParallelConstruct
    ;

actorConcurrencyConstruct
    : actorConstruct
    ;

channelConcurrencyConstruct
    : channelConstruct
    ;

cancellationConcurrencyConstruct
    : cancellationConstruct
    ;

synchronizationConcurrencyConstruct
    : synchronizationConstruct
    ;

/*
 * ============================================================================
 * SEMANTIC CONCURRENCY CATEGORY
 * ============================================================================
 *
 * Tooling which needs to classify a parsed construct as "concurrency" should
 * normally consume this rule rather than depending on individual leaf rules.
 *
 * This makes the composition boundary stable while allowing individual
 * concurrency features to evolve independently.
 * ============================================================================
 */

concurrencySemanticConstruct
    : concurrencyConstruct
    ;

/*
 * ============================================================================
 * EXPRESSION CATEGORY
 * ============================================================================
 *
 * This adapter is useful to semantic analysis and AST construction when it
 * needs to know that an expression belongs to the concurrency domain.
 *
 * It intentionally does not introduce another expression grammar.
 * ============================================================================
 */

concurrencySemanticExpression
    : concurrencyExpression
    ;

/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. ROOT PARSER
 *
 *    ZamaniParser.g4
 *      |
 *      +--> Statements
 *      |
 *      +--> Concurrency
 *
 *    The root parser should consume `concurrencyConstruct` through the
 *    established statement/expression composition path.
 *
 *
 * 2. UNIVERSAL STATEMENTS
 *
 *    grammar/statements/statements.g4 remains the owner of universal
 *    statement dispatch.
 *
 *    grammar/statements/concurrency.g4, where present, is only an adapter.
 *
 *    It must not reproduce:
 *
 *      taskConstruct
 *      futureExpression
 *      parallelConstruct
 *      dataParallelConstruct
 *      taskParallelConstruct
 *      actorConstruct
 *      channelConstruct
 *      cancellationConstruct
 *      synchronizationConstruct
 *
 *    Those remain owned by their leaf grammars.
 *
 *
 * 3. ASYNC EXPRESSIONS
 *
 *    grammar/expressions/async.g4 owns:
 *
 *      awaitExpression
 *      spawnExpression
 *      parallelExpression
 *
 *    This file consumes those rules but does not redefine them.
 *
 *
 * 4. TASKS
 *
 *    grammar/concurrency/tasks.g4 owns task/spawn/task-specific concrete
 *    syntax.
 *
 *    This file only composes `taskConstruct`.
 *
 *
 * 5. FUTURES
 *
 *    grammar/concurrency/futures.g4 owns future-specific syntax.
 *
 *    Existing future rule names such as:
 *
 *      futureExpression
 *      futureObservationRoot
 *      futureResultRoot
 *
 *    are consumed here.
 *
 *    Do NOT reintroduce stale names such as:
 *
 *      futureConstruct
 *      futureStatement
 *
 *    unless those names are explicitly restored by the owning futures
 *    grammar. They are not invented here.
 *
 *
 * 6. PARALLELISM
 *
 *    grammar/concurrency/parallel.g4 owns generic parallel syntax.
 *
 *    grammar/concurrency/data-parallel.g4 owns data-parallel syntax.
 *
 *    grammar/concurrency/task-parallel.g4 owns task-parallel syntax.
 *
 *    This file only composes their public roots.
 *
 *
 * 7. ACTORS
 *
 *    grammar/concurrency/actors.g4 owns actor syntax.
 *
 *    This file exposes it only through `actorConcurrencyConstruct`.
 *
 *
 * 8. CHANNELS
 *
 *    grammar/concurrency/channels.g4 owns channel syntax.
 *
 *    Canonical channel vocabulary must be supplied by the central lexer.
 *
 *    Expected canonical vocabulary includes concepts represented by tokens
 *    such as:
 *
 *      CHANNEL
 *      SEND
 *      RECEIVE
 *      CLOSE
 *      SELECT
 *      DEFAULT
 *
 *    Token ownership belongs to the lexer, not this parser grammar.
 *
 *
 * 9. CANCELLATION
 *
 *    grammar/concurrency/cancellation.g4 owns cancellation syntax.
 *
 *
 * 10. SYNCHRONIZATION
 *
 *     grammar/concurrency/synchronization.g4 owns synchronization syntax.
 *
 *
 * 11. REDUCTION
 *
 *     Reduction remains owned by data-parallel/reduction composition.
 *
 *     This file deliberately does not add:
 *
 *       reductionConstruct
 *
 *     as a duplicate sibling syntax path.
 *
 *     A reduction which is already a data-parallel operation remains a
 *     data-parallel construct.
 *
 *
 * 12. RESOURCES
 *
 *     Resource requirements are not owned here.
 *
 *     grammar/resources/ owns:
 *
 *       requirements
 *       capabilities
 *       constraints
 *       preferences
 *       hints
 *       placement
 *       scaling
 *
 *     Therefore this file does not invent a second `REQUIRES` grammar.
 *
 *
 * 13. DISTRIBUTED CONCURRENCY
 *
 *     Distributed concurrency remains a composition of distributed semantics
 *     and concurrency semantics.
 *
 *     Physical placement, node selection, topology, replication, deployment,
 *     and communication realization belong downstream.
 *
 *     This file deliberately avoids fixed distributed topology.
 *
 *
 * 14. QUANTUM
 *
 *     Quantum operations remain owned by the quantum grammar and lower to
 *     the canonical `quantum::ir`.
 *
 *     Concurrency may describe when quantum computation occurs, but this
 *     grammar must never introduce a second quantum IR.
 *
 *
 * 15. HDL / HARDWARE
 *
 *     Hardware realization remains outside this grammar.
 *
 *     No CPU/core/thread/GPU/FPGA/QPU count or physical topology is encoded.
 *
 *
 * 16. AST
 *
 *     All constructs are lowered into the existing domain-neutral AST.
 *
 *     The concurrency domain may attach:
 *
 *       effects
 *       dependencies
 *       capabilities
 *       attributes
 *       ordering metadata
 *       source spans
 *
 *     without creating a parser-specific concurrency IR.
 *
 *
 * 17. SEMANTIC ANALYSIS
 *
 *     Semantic analysis determines:
 *
 *       dependency validity
 *       ownership
 *       synchronization correctness
 *       cancellation semantics
 *       communication validity
 *       determinism
 *       resource requirements
 *       capability satisfaction
 *       portability
 *
 *
 * 18. COMPILER / RUNTIME
 *
 *     Compiler and runtime components determine actual:
 *
 *       task scheduling
 *       worker allocation
 *       process placement
 *       accelerator use
 *       distributed placement
 *       communication implementation
 *       synchronization implementation
 *
 *     None of those decisions belong in this grammar.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * COMPATIBILITY ALIASES
 * ============================================================================
 *
 * These aliases provide stable semantic names for tooling.
 *
 * They intentionally contain no punctuation, token, or hardware assumptions.
 * ============================================================================
 */

concurrencyOperation
    : concurrencySemanticConstruct
    ;

concurrencyExpressionRoot
    : concurrencySemanticExpression
    ;

concurrencyRoot
    : concurrencyConstruct
    ;