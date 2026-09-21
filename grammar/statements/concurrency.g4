/*
 * ============================================================================
 * Zamani Universal Computing Language
 * Canonical Concurrency-Domain Composition Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/concurrency.g4
 *
 * Status:
 *     PRODUCTION CONCURRENCY COMPOSITION ROOT
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION OWNER for Zamani concurrency syntax.
 *
 * It does not implement individual concurrency constructs.
 *
 * It composes the independently owned concurrency grammars:
 *
 *     tasks.g4
 *     futures.g4
 *     parallel.g4
 *     data-parallel.g4
 *     task-parallel.g4
 *     actors.g4
 *     channels.g4
 *     cancellation.g4
 *     synchronization.g4
 *
 * The individual files remain responsible for their own syntax.
 *
 * This file provides stable parser-level integration points for:
 *
 *     expressions
 *     statements
 *     declarations
 *     concurrency-domain tooling
 *
 * ============================================================================
 * IMPORTANT PATH / OWNERSHIP RULE
 * ============================================================================
 *
 * This file is located at:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * It MUST NOT be duplicated as:
 *
 *     grammar/statements/concurrency.g4
 *
 * `grammar/statements/statements.g4` remains the universal statement
 * composition owner.
 *
 * `grammar/concurrency/concurrency.g4` owns only the concurrency-domain
 * composition boundary.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - concurrency-domain composition;
 *     - concurrency expression composition;
 *     - concurrency statement composition;
 *     - concurrency declaration composition;
 *     - concurrency integration adapters;
 *     - stable public concurrency parser entry points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - expression precedence;
 *     - ordinary statements;
 *     - blocks;
 *     - types;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - resource semantics;
 *     - capability evaluation;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - optimization;
 *     - runtime execution;
 *     - worker creation;
 *     - thread creation;
 *     - CPU selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - QPU selection;
 *     - node selection;
 *     - physical topology;
 *     - quantum operations;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * Those concerns belong to their canonical downstream owners.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one effective concurrency-domain composition grammar.
 *
 * This file is that owner.
 *
 * Leaf grammars MUST NOT redefine:
 *
 *     concurrencyExpression
 *     concurrencyStatement
 *     concurrencyDeclaration
 *     concurrencyConstruct
 *
 * The leaf grammars own only their specific constructs.
 *
 * ============================================================================
 * ANTLR COMPOSITION MODEL
 * ============================================================================
 *
 * ANTLR parser grammar imports behave as grammar composition/inheritance:
 *
 *     Concurrency
 *          |
 *          +--> Tasks
 *          +--> Futures
 *          +--> Parallel
 *          +--> DataParallel
 *          +--> TaskParallel
 *          +--> Actors
 *          +--> Channels
 *          +--> Cancellation
 *          +--> Synchronization
 *
 * The resulting effective parser sees the imported rules as part of the
 * composed grammar.
 *
 * Therefore this file must not copy the rules from those grammars.
 *
 * ============================================================================
 * REQUIRED DELEGATE CONTRACT
 * ============================================================================
 *
 * Every imported grammar MUST be a valid ANTLR parser grammar.
 *
 * In particular:
 *
 *     parallel.g4
 *     task-parallel.g4
 *
 * currently require their parser-grammar headers and canonical token-vocabulary
 * declarations to be repaired before this composition can be generated.
 *
 * That repair belongs to those files.
 *
 * This file deliberately does not duplicate their syntax as a workaround.
 *
 * ============================================================================
 * CANONICAL IMPORTS
 * ============================================================================
 */

parser grammar Concurrency;

options {
    tokenVocab = ZamaniLexer;
}

import
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
 * 1. CONCURRENCY EXPRESSION COMPOSITION
 * ============================================================================
 *
 * This is the concurrency-domain expression boundary.
 *
 * It is intended to be consumed by the canonical expression composition layer.
 *
 * Important:
 *
 *     - ordinary `await` and `spawn` remain owned by Tasks/Async;
 *     - ordinary parallel syntax remains owned by Parallel;
 *     - data parallel syntax remains owned by DataParallel;
 *     - structured task parallelism remains owned by TaskParallel;
 *     - actor/channel/cancellation/synchronization expressions remain owned by
 *       their respective files.
 *
 * No implementation detail is encoded here.
 */

concurrencyExpression
    : concurrencyTaskExpression
    | concurrencyParallelExpression
    | concurrencyDataParallelExpression
    | concurrencyTaskParallelExpression
    | concurrencyActorExpression
    | concurrencyChannelExpression
    | concurrencyCancellationExpression
    | concurrencySynchronizationExpression
    ;


/*
 * ============================================================================
 * 2. TASK EXPRESSION ADAPTER
 * ============================================================================
 *
 * Tasks own spawn/await syntax.
 *
 * `taskConcurrencyExpression` currently contains the task-level expression
 * forms defined by tasks.g4.
 *
 * This adapter deliberately does not recreate:
 *
 *     spawn
 *     await
 *     async
 *
 * syntax.
 */

concurrencyTaskExpression
    : taskConcurrencyExpression
    ;


/*
 * ============================================================================
 * 3. PARALLEL EXPRESSION ADAPTER
 * ============================================================================
 *
 * Parallel computation syntax belongs to parallel.g4.
 *
 * This adapter provides one stable concurrency-domain name.
 */

concurrencyParallelExpression
    : parallelConstruct
    ;


/*
 * ============================================================================
 * 4. DATA-PARALLEL EXPRESSION ADAPTER
 * ============================================================================
 *
 * Data-parallel syntax remains owned by data-parallel.g4.
 */

concurrencyDataParallelExpression
    : dataParallelConstruct
    ;


/*
 * ============================================================================
 * 5. TASK-PARALLEL EXPRESSION ADAPTER
 * ============================================================================
 *
 * Structured task parallelism remains owned by task-parallel.g4.
 *
 * `taskParallelRootExpression` is the stable expression-level entry point.
 */

concurrencyTaskParallelExpression
    : taskParallelRootExpression
    ;


/*
 * ============================================================================
 * 6. ACTOR EXPRESSION ADAPTER
 * ============================================================================
 *
 * Actor semantics remain owned by actors.g4.
 *
 * This file does not define actor lifecycle, messaging, supervision or
 * restart semantics.
 */

concurrencyActorExpression
    : actorExpressionRoot
    ;


/*
 * ============================================================================
 * 7. CHANNEL EXPRESSION ADAPTER
 * ============================================================================
 *
 * Channel syntax remains owned by channels.g4.
 *
 * This file does not define:
 *
 *     channel type semantics;
 *     buffering;
 *     ownership;
 *     send/receive behavior;
 *     close semantics;
 *     select semantics.
 */

concurrencyChannelExpression
    : channelRootExpression
    ;


/*
 * ============================================================================
 * 8. CANCELLATION EXPRESSION ADAPTER
 * ============================================================================
 *
 * Cancellation syntax remains owned by cancellation.g4.
 *
 * Cancellation propagation and runtime behavior are semantic/runtime concerns.
 */

concurrencyCancellationExpression
    : cancellationExpressionRoot
    ;


/*
 * ============================================================================
 * 9. SYNCHRONIZATION EXPRESSION ADAPTER
 * ============================================================================
 *
 * Synchronization syntax remains owned by synchronization.g4.
 */

concurrencySynchronizationExpression
    : synchronizationExpression
    ;


/*
 * ============================================================================
 * 10. CONCURRENCY STATEMENT COMPOSITION
 * ============================================================================
 *
 * This is the concurrency-domain statement boundary.
 *
 * It is intended to be consumed by:
 *
 *     grammar/statements/statements.g4
 *
 * That file remains the authoritative universal `statement` owner.
 *
 * This file MUST NOT redefine `statement`.
 */

concurrencyStatement
    : concurrencyTaskStatement
    | concurrencyParallelStatement
    | concurrencyDataParallelStatement
    | concurrencyTaskParallelStatement
    | concurrencyActorStatement
    | concurrencyChannelStatement
    | concurrencyCancellationStatement
    | concurrencySynchronizationStatement
    ;


/*
 * ============================================================================
 * 11. TASK STATEMENT ADAPTER
 * ============================================================================
 *
 * Task statement syntax remains owned by tasks.g4.
 */

concurrencyTaskStatement
    : taskStatement
    ;


/*
 * ============================================================================
 * 12. PARALLEL STATEMENT ADAPTER
 * ============================================================================
 */

concurrencyParallelStatement
    : parallelStatement
    ;


/*
 * ============================================================================
 * 13. DATA-PARALLEL STATEMENT ADAPTER
 * ============================================================================
 */

concurrencyDataParallelStatement
    : dataParallelStatement
    ;


/*
 * ============================================================================
 * 14. TASK-PARALLEL STATEMENT ADAPTER
 * ============================================================================
 *
 * The canonical task-parallel grammar exposes a root statement boundary.
 */

concurrencyTaskParallelStatement
    : taskParallelRootStatement
    ;


/*
 * ============================================================================
 * 15. ACTOR STATEMENT ADAPTER
 * ============================================================================
 */

concurrencyActorStatement
    : actorStatementRoot
    ;


/*
 * ============================================================================
 * 16. CHANNEL STATEMENT ADAPTER
 * ============================================================================
 */

concurrencyChannelStatement
    : channelRootStatement
    ;


/*
 * ============================================================================
 * 17. CANCELLATION STATEMENT ADAPTER
 * ============================================================================
 */

concurrencyCancellationStatement
    : cancellationStatement
    ;


/*
 * ============================================================================
 * 18. SYNCHRONIZATION STATEMENT ADAPTER
 * ============================================================================
 */

concurrencySynchronizationStatement
    : synchronizationStatement
    ;


/*
 * ============================================================================
 * 19. CONCURRENCY DECLARATION COMPOSITION
 * ============================================================================
 *
 * Concurrency declarations are intentionally kept separate from statements.
 *
 * This prevents declaration syntax from becoming an accidental statement
 * alternative and allows the universal declarations dispatcher to decide
 * where declarations are legal.
 */

concurrencyDeclaration
    : concurrencyActorDeclaration
    | concurrencyChannelDeclaration
    | concurrencyCancellationDeclaration
    | concurrencySynchronizationDeclaration
    ;


/*
 * ============================================================================
 * 20. ACTOR DECLARATION
 * ============================================================================
 */

concurrencyActorDeclaration
    : actorDeclarationRoot
    ;


/*
 * ============================================================================
 * 21. CHANNEL DECLARATION
 * ============================================================================
 */

concurrencyChannelDeclaration
    : channelDeclaration
    ;


/*
 * ============================================================================
 * 22. CANCELLATION DECLARATION
 * ============================================================================
 */

concurrencyCancellationDeclaration
    : cancellationDeclaration
    ;


/*
 * ============================================================================
 * 23. SYNCHRONIZATION DECLARATION
 * ============================================================================
 */

concurrencySynchronizationDeclaration
    : synchronizationDeclaration
    ;


/*
 * ============================================================================
 * 24. CONCURRENCY CONSTRUCT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * A single rule mixing expression and statement forms is intentionally NOT
 * used here.
 *
 * For example:
 *
 *     spawn work();
 *
 * can be structurally valid both as an expression followed by a terminator
 * and as a statement-level task construct.
 *
 * Mixing both contexts into one root creates avoidable prediction ambiguity.
 *
 * Therefore the production API is context-specific:
 *
 *     concurrencyExpression
 *     concurrencyStatement
 *     concurrencyDeclaration
 *
 * `concurrencyConstruct` is retained as an expression-oriented compatibility
 * boundary for existing tooling.
 */

concurrencyConstruct
    : concurrencyExpression
    ;


/*
 * ============================================================================
 * 25. FUTURE SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Futures are intentionally NOT added as another competing syntax alternative.
 *
 * In the current language design:
 *
 *     futures.g4
 *         -> awaitExpression
 *         -> canonical async grammar
 *
 * Therefore a future is a semantic property of an asynchronous computation,
 * not a requirement for a separate `future` keyword.
 *
 * The following boundary is provided for tooling that needs to identify the
 * future-specific parser representation.
 */

concurrencyFutureExpression
    : futureExpression
    ;


/*
 * ============================================================================
 * 26. FUTURE OBSERVATION BOUNDARY
 * ============================================================================
 *
 * Await remains the syntactic operation.
 *
 * Whether its operand denotes a Future-like semantic value is decided by
 * semantic/type analysis.
 */

concurrencyFutureObservation
    : futureObservationRoot
    ;


/*
 * ============================================================================
 * 27. CONCURRENCY DOMAIN
 * ============================================================================
 *
 * A domain-level consumer should use the context-specific roots rather than
 * this rule when it knows whether it is parsing an expression, statement or
 * declaration.
 *
 * This rule is retained as a structural classification boundary for tooling.
 */

concurrencyDomainExpression
    : concurrencyExpression
    ;

concurrencyDomainStatement
    : concurrencyStatement
    ;

concurrencyDomainDeclaration
    : concurrencyDeclaration
    ;


/*
 * ============================================================================
 * 28. RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Concurrency MUST NOT own `requires` syntax.
 *
 * Resource requirements belong to the resource subsystem.
 *
 * For example, source-level requirements such as:
 *
 *     requires capability(...)
 *     requires memory(...)
 *     requires communication(...)
 *
 * are parsed through the canonical resource/capability grammar.
 *
 * This concurrency grammar only supplies the concurrency construct that causes
 * semantic/resource analysis to derive concurrency-related requirements.
 *
 * Therefore there is intentionally NO:
 *
 *     concurrencyRequirementClause
 *
 * here.
 *
 * This prevents duplicate ownership of REQUIRES.
 */


/*
 * ============================================================================
 * 29. TYPE INTEGRATION
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     Task<T>
 *     Future<T>
 *     Promise<T>
 *     Executor<T>
 *     Worker<T>
 *     Channel<T>
 *
 * as runtime-specific language types.
 *
 * Type syntax belongs to grammar/types/.
 *
 * Semantic analysis determines whether an expression is:
 *
 *     awaitable;
 *     spawnable;
 *     parallelizable;
 *     cancellable;
 *     sendable;
 *     receivable;
 *     synchronizable.
 *
 * The parser only establishes source structure.
 */


/*
 * ============================================================================
 * 30. EFFECT INTEGRATION
 * ============================================================================
 *
 * Concurrency constructs may introduce effects such as:
 *
 *     asynchronous execution
 *     suspension
 *     communication
 *     synchronization
 *     cancellation
 *     nondeterministic ordering
 *     distributed interaction
 *
 * Effects are owned by grammar/effects/ and semantic effect analysis.
 *
 * No effect implementation belongs here.
 */


/*
 * ============================================================================
 * 31. OWNERSHIP / MEMORY INTEGRATION
 * ============================================================================
 *
 * Concurrency does not create a second ownership model.
 *
 * Ownership, borrowing, aliasing and lifetime rules remain owned by the
 * canonical memory/type/semantic systems.
 *
 * A concurrent operation may impose additional semantic restrictions, but
 * those restrictions are checked downstream.
 */


/*
 * ============================================================================
 * 32. DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Parsing may depend only on:
 *
 *     source token stream;
 *     selected grammar/language version;
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time;
 *     randomness;
 *     environment state;
 *     filesystem state;
 *     network state;
 *     available hardware;
 *     runtime state;
 *     scheduler state.
 *
 * No semantic predicate or target-dependent parser action is used here.
 */


/*
 * ============================================================================
 * 33. POCO-REAF
 * ============================================================================
 *
 * Concurrency expresses LOGICAL COMPUTATIONAL INTENT.
 *
 * It MUST NOT encode universal implementation limits.
 *
 * In particular, this grammar does not establish maximum values for:
 *
 *     tasks
 *     concurrent operations
 *     workers
 *     threads
 *     cores
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     nodes
 *     devices
 *     channels
 *     actors
 *     parallel regions
 *     task groups
 *     data elements
 *
 * Repetition and nesting are represented structurally by grammar operators
 * and delegated grammar rules.
 *
 * Any actual finite limit comes from:
 *
 *     source data;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     deployment policy;
 *     explicit program requirements.
 *
 * None of those becomes a universal grammar maximum.
 */


/*
 * ============================================================================
 * 34. RESOURCE-SCALABLE EXECUTION
 * ============================================================================
 *
 * The same logical program may be lowered differently according to resources.
 *
 * Example semantic intent:
 *
 *     parallel {
 *         a()
 *         b()
 *         c()
 *     }
 *
 * may be realized as:
 *
 *     one execution context;
 *     cooperative concurrency;
 *     multiple CPU execution contexts;
 *     SIMD/data-parallel execution;
 *     GPU execution;
 *     FPGA/ASIC execution;
 *     distributed execution;
 *     heterogeneous execution;
 *     quantum/classical orchestration;
 *     future computational substrates.
 *
 * The grammar does not choose among these realizations.
 *
 * A resource-poor target may serialize independent work while preserving
 * semantics.
 *
 * A resource-rich target may exploit additional available parallelism.
 *
 * This is a compiler/scheduler/runtime decision.
 */


/*
 * ============================================================================
 * 35. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Concurrency may surround and coordinate quantum computation.
 *
 * This grammar does NOT define:
 *
 *     quantum operations;
 *     gates;
 *     physical qubit identifiers;
 *     logical-to-physical mappings;
 *     QEC;
 *     QZN/ZQN;
 *     routing;
 *     pulse scheduling;
 *     calibration.
 *
 * If a concurrency construct contains quantum computation, the downstream
 * pipeline remains:
 *
 *     Zamani source
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
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * There is no concurrency-specific quantum IR here.
 */


/*
 * ============================================================================
 * 36. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical concurrency lowers through the canonical classical semantic/IR
 * path.
 *
 * This grammar does not select:
 *
 *     CPU;
 *     core;
 *     SIMD width;
 *     vector register;
 *     worker pool;
 *     OS thread.
 */


/*
 * ============================================================================
 * 37. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Concurrency syntax may participate in software/hardware co-design.
 *
 * The grammar does not interpret:
 *
 *     parallel
 *
 * as a fixed hardware replication factor.
 *
 * HDL semantics determine clocking, processes, pipeline behavior and hardware
 * realization downstream.
 */


/*
 * ============================================================================
 * 38. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A concurrency construct does not imply locality.
 *
 * The same logical computation may eventually execute:
 *
 *     locally;
 *     in another process;
 *     on another machine;
 *     on an accelerator;
 *     across a cluster;
 *     in a cloud deployment;
 *     on a future computational substrate.
 *
 * Placement, communication and topology remain downstream concerns.
 */


/*
 * ============================================================================
 * 39. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Concurrency can surround:
 *
 *     tensor computation;
 *     model inference;
 *     training;
 *     data pipelines;
 *     streaming;
 *     distributed learning.
 *
 * This grammar does not introduce framework-specific syntax.
 *
 * Tensor/model/data semantics remain owned by their canonical domains.
 */


/*
 * ============================================================================
 * 40. SECURITY INTEGRATION
 * ============================================================================
 *
 * Concurrency constructs may cross security boundaries.
 *
 * Authentication, authorization, capability enforcement, isolation and secure
 * execution remain downstream semantic/runtime concerns.
 *
 * Parser behavior must remain deterministic and side-effect free.
 */


/*
 * ============================================================================
 * 41. AST CONTRACT
 * ============================================================================
 *
 * This grammar constructs NO Rust AST directly.
 *
 * Every accepted concurrency construct must map into the repository's existing
 * domain-neutral frontend AST.
 *
 * Existing canonical frontend expression representations include:
 *
 *     ExpressionKind::Await
 *     ExpressionKind::Spawn
 *     ExpressionKind::Async
 *
 * Therefore this grammar MUST NOT create a second concurrency expression enum.
 *
 * The AST must preserve at minimum:
 *
 *     source span;
 *     source ordering;
 *     construct category;
 *     nested operands;
 *     nested statements;
 *     declaration structure;
 *     modifiers/attributes owned by canonical grammar;
 *     semantic names;
 *     dependency structure.
 *
 * It MUST NOT embed:
 *
 *     WorkerId
 *     ThreadId
 *     CoreId
 *     DeviceId
 *     GPUId
 *     QPUId
 *     PhysicalQubitId
 *     runtime queue ID
 *
 * merely because a source construct is concurrent.
 */


/*
 * ============================================================================
 * 42. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural validity only.
 *
 * Semantic analysis owns:
 *
 *     name resolution;
 *     type checking;
 *     ownership;
 *     borrowing;
 *     lifetime validation;
 *     effect checking;
 *     dependency analysis;
 *     data-race analysis;
 *     synchronization correctness;
 *     cancellation correctness;
 *     determinism;
 *     capability requirements;
 *     resource requirements;
 *     portability;
 *     distributed legality;
 *     quantum/classical legality;
 *     HDL/software co-design legality.
 *
 * A syntactically valid concurrent construct is therefore not automatically
 * semantically valid.
 */


/*
 * ============================================================================
 * 43. IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The intended pipeline is:
 *
 *     grammar
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic model
 *         |
 *         v
 *     canonical IR
 *         |
 *         +------------------+
 *         |                  |
 *         v                  v
 *     classical          quantum::ir
 *         |                  |
 *         +--------+---------+
 *                  |
 *                  v
 *       optimization / lowering
 *                  |
 *          routing / scheduling
 *                  |
 *             resilience
 *                  |
 *               ZQN/HAL
 *                  |
 *             target runtime
 *
 * Concurrency does not create a second universal IR.
 */


/*
 * ============================================================================
 * 44. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling is downstream.
 *
 * This grammar does not select:
 *
 *     worker;
 *     thread;
 *     CPU;
 *     GPU;
 *     QPU;
 *     node;
 *     queue;
 *     time slot;
 *     placement;
 *     topology.
 *
 * The scheduler receives semantic dependencies and resource/capability
 * information and determines an executable plan.
 */


/*
 * ============================================================================
 * 45. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime code MUST consume compiled semantic/IR representations.
 *
 * Runtime code MUST NOT parse:
 *
 *     grammar/concurrency/*.g4
 *
 * directly.
 *
 * The grammar therefore has no dependency on:
 *
 *     executor implementation;
 *     thread pool implementation;
 *     async runtime;
 *     OS scheduler;
 *     device runtime;
 *     distributed runtime.
 */


/*
 * ============================================================================
 * 46. SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no embedded Rust;
 *     no filesystem access;
 *     no networking;
 *     no environment inspection;
 *     no hardware discovery;
 *     no randomness;
 *     no runtime execution.
 *
 * The compiler/runtime implementation must use:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only.
 *
 * No unsafe Rust is required or permitted.
 */


/*
 * ============================================================================
 * 47. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level implementation assumptions include:
 *
 *     fixed worker counts;
 *     fixed thread counts;
 *     fixed core counts;
 *     fixed CPU counts;
 *     fixed GPU counts;
 *     fixed FPGA counts;
 *     fixed QPU counts;
 *     fixed node counts;
 *     fixed device counts;
 *     fixed channel capacities;
 *     fixed task counts;
 *     fixed parallelism widths;
 *     fixed topology;
 *     physical device identifiers.
 *
 * Numeric literals appearing in ordinary Zamani expressions remain ordinary
 * program values.
 *
 * The prohibition applies to implementation limits, not program semantics.
 */


/*
 * ============================================================================
 * 48. DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Grammar-level diagnostics should identify structural failures only.
 *
 * Examples:
 *
 *     malformed concurrency construct;
 *     missing operand;
 *     malformed actor construct;
 *     malformed channel construct;
 *     malformed cancellation construct;
 *     malformed synchronization construct;
 *     malformed parallel construct.
 *
 * Diagnostics about:
 *
 *     insufficient CPUs;
 *     insufficient GPUs;
 *     insufficient QPUs;
 *     insufficient memory;
 *     unsupported topology;
 *     unavailable workers;
 *
 * are NOT syntax diagnostics.
 *
 * They belong to semantic/resource/target analysis.
 */


/*
 * ============================================================================
 * 49. DETERMINISTIC RECOVERY
 * ============================================================================
 *
 * Error recovery belongs to the canonical parser/frontend infrastructure.
 *
 * This grammar deliberately contains no recovery actions.
 *
 * Recovery must:
 *
 *     make progress;
 *     preserve source spans;
 *     avoid fabricated valid concurrency constructs;
 *     avoid infinite loops;
 *     remain deterministic.
 */


/*
 * ============================================================================
 * 50. COMPATIBILITY
 * ============================================================================
 *
 * Stable concurrency source syntax must remain compatible across language
 * versions unless a documented language-version rule explicitly changes it.
 *
 * A newly introduced concurrency capability MUST NOT silently reinterpret an
 * existing valid program.
 *
 * Lexical reservation changes belong to the canonical lexer compatibility
 * process, not this file.
 */


/*
 * ============================================================================
 * 51. TEST CONTRACT
 * ============================================================================
 *
 * This composition root requires tests at the domain level.
 *
 * Positive:
 *
 *     spawn
 *     await
 *     parallel
 *     data-parallel
 *     task-parallel
 *     actors
 *     channels
 *     cancellation
 *     synchronization
 *
 * Negative:
 *
 *     missing operands;
 *     malformed blocks;
 *     malformed dependencies;
 *     malformed declarations;
 *     malformed synchronization;
 *     malformed channel operations;
 *     malformed cancellation.
 *
 * Boundary:
 *
 *     one task;
 *     many tasks;
 *     nested concurrency;
 *     nested parallel regions;
 *     empty/near-empty legal structures where owned by leaf grammars;
 *     deep dependency graphs;
 *     broad dependency graphs.
 *
 * Scalability:
 *
 *     source size;
 *     task count;
 *     dependency count;
 *     nesting depth;
 *     data-domain size;
 *     distributed logical participants.
 *
 * No test may establish a universal implementation maximum.
 *
 * Determinism:
 *
 *     identical source + identical grammar configuration
 *         => identical parse structure.
 *
 * Cross-domain:
 *
 *     classical + concurrency;
 *     quantum + concurrency;
 *     hybrid + concurrency;
 *     HDL + concurrency;
 *     AI + concurrency;
 *     data + concurrency;
 *     distributed + concurrency;
 *     networking + concurrency;
 *     hardware + concurrency.
 */


/*
 * ============================================================================
 * 52. FEATURE COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete only when the following integration chain exists:
 *
 *     specification
 *         |
 *         v
 *     canonical lexer
 *         |
 *         v
 *     canonical concurrency parser composition
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     resource/capability analysis
 *         |
 *         v
 *     canonical IR
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     runtime/target realization
 *
 * Every concurrency leaf must have:
 *
 *     syntax contract;
 *     AST mapping;
 *     semantic mapping;
 *     effect mapping;
 *     resource/capability mapping;
 *     IR destination;
 *     diagnostics;
 *     positive tests;
 *     negative tests;
 *     boundary tests;
 *     scalability tests;
 *     determinism tests;
 *     compatibility tests.
 */


/*
 * ============================================================================
 * 53. FINAL INVARIANTS
 * ============================================================================
 *
 * Invariant 1:
 *
 *     This file composes concurrency.
 *
 * Invariant 2:
 *
 *     Leaf files own leaf syntax.
 *
 * Invariant 3:
 *
 *     statements/statements.g4 owns `statement`.
 *
 * Invariant 4:
 *
 *     expressions/expressions.g4 owns `expression`.
 *
 * Invariant 5:
 *
 *     lexer files own tokens.
 *
 * Invariant 6:
 *
 *     resources/ owns requirements and capabilities.
 *
 * Invariant 7:
 *
 *     semantic analysis owns concurrency validity.
 *
 * Invariant 8:
 *
 *     scheduling owns physical execution ordering.
 *
 * Invariant 9:
 *
 *     runtime owns execution.
 *
 * Invariant 10:
 *
 *     quantum semantics ultimately cross the canonical `quantum::ir`
 *     boundary.
 *
 * Invariant 11:
 *
 *     no physical machine limit becomes a language limit.
 *
 * Invariant 12:
 *
 *     no Rust `unsafe` is required.
 *
 * Invariant 13:
 *
 *     no target-specific implementation is embedded in the grammar.
 *
 * Invariant 14:
 *
 *     POCO-REAF remains the portability contract.
 *
 * ============================================================================
 */