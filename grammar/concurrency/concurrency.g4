/*
 * ============================================================================
 * Zamani Programming Language
 * Canonical Concurrency-Domain Composition Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/concurrency.g4
 *
 * Role:
 *     Stable parser-domain composition boundary for Zamani concurrency.
 *
 * This file is NOT a second task grammar.
 *
 * It coordinates the independently owned concurrency grammar components:
 *
 *     tasks.g4
 *     futures.g4
 *     parallel.g4
 *     data-parallel.g4
 *     task-parallel.g4
 *
 * Future independently owned concurrency domains may be added here only after
 * their lexical, syntactic, AST, semantic and compatibility contracts exist.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     This grammar contains no target-language actions.
 *     No unsafe Rust is required or permitted by this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the concurrency-domain parser entry point;
 *   - composition of concurrency grammar domains;
 *   - concurrency-domain classification;
 *   - the stable integration boundary between the main parser and the
 *     independently owned concurrency grammars;
 *   - concurrency-level specification clauses that are genuinely syntax-level
 *     and already supported by the canonical lexer/specification.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - qualified names;
 *   - literals;
 *   - expressions;
 *   - types;
 *   - patterns;
 *   - blocks;
 *   - statements generally;
 *   - function declarations generally;
 *   - task syntax;
 *   - future syntax;
 *   - parallel syntax;
 *   - actor syntax;
 *   - channel syntax;
 *   - synchronization algorithms;
 *   - memory semantics;
 *   - resource allocation;
 *   - scheduling;
 *   - routing;
 *   - hardware;
 *   - quantum IR;
 *   - classical IR;
 *   - QEC;
 *   - ZQN;
 *   - runtime implementation;
 *   - executor implementation;
 *   - thread creation;
 *   - worker pools;
 *   - CPU/GPU/QPU topology;
 *   - device selection;
 *   - deployment.
 *
 * Those concerns belong to their canonical owners.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Concurrency syntax expresses portable computational intent.
 *
 * It MUST NOT encode:
 *
 *   MAX_TASKS
 *   MAX_THREADS
 *   MAX_WORKERS
 *   MAX_CORES
 *   MAX_NODES
 *   MAX_DEVICES
 *   MAX_CHANNELS
 *   MAX_ACTORS
 *   MAX_PARALLELISM
 *
 * It MUST NOT assume:
 *
 *   task == thread
 *   thread == core
 *   actor == process
 *   channel == queue
 *   node == machine
 *
 * Runtime and target lowering decide how concurrency intent is realized.
 *
 * Therefore the same semantic program may be lowered to:
 *
 *   - one execution context;
 *   - multiple CPU cores;
 *   - SIMD/data parallel execution;
 *   - GPU execution;
 *   - FPGA/ASIC acceleration;
 *   - distributed execution;
 *   - quantum/classical orchestration;
 *   - heterogeneous execution;
 *   - future computational substrates.
 *
 * No finite machine-size limit is expressed by this grammar.
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
 *     concurrencyConstruct
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     canonical AST                 other AST domains
 *          |
 *          v
 *     name/type/effect/resource analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +------------+-------------+----------------+
 *          |            |             |                |
 *          v            v             v                v
 *      classical     quantum      distributed      hardware
 *          |            |             |                |
 *          +------------+-------------+----------------+
 *                               |
 *                               v
 *                optimization / routing / scheduling
 *                               |
 *                               v
 *                         resilience/runtime
 *
 * This file never lowers directly to a runtime primitive.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns lexical spelling.
 *
 * Concurrency vocabulary currently established by the repository includes:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *     REQUIRES
 *
 * This grammar does not create lexer rules.
 *
 * Do NOT add tokens here for:
 *
 *     THREAD
 *     WORKER
 *     ACTOR
 *     CHANNEL
 *     MUTEX
 *     LOCK
 *     JOIN
 *     CANCEL
 *     TASK_SCOPE
 *     CONCURRENT
 *
 * unless those words first become canonical language vocabulary through the
 * lexical/specification authority process.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * The following canonical parser rules are external to this file:
 *
 *     expression
 *     statement
 *     blockExpression
 *     identifier
 *     qualifiedName
 *
 * This file MUST NOT redefine them.
 *
 * Domain-specific task/parallel productions are owned by their corresponding
 * concurrency grammar files.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * `concurrency.g4` is a COMPOSITION ROOT.
 *
 * It must not duplicate:
 *
 *     tasks.g4
 *         -> spawn / await / task-oriented concurrency
 *
 *     parallel.g4
 *         -> parallel-specific syntax
 *
 *     futures.g4
 *         -> future-specific syntax
 *
 *     data-parallel.g4
 *         -> data-parallel syntax
 *
 *     task-parallel.g4
 *         -> task-parallel syntax
 *
 * If another file owns a production, this file references that production
 * through the composition contract instead of redefining it.
 *
 * ============================================================================
 */

parser grammar Concurrency;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC CONCURRENCY ENTRY POINT
 * ============================================================================
 *
 * This is the ONLY general-purpose entry point owned by this file.
 *
 * The canonical parser/composition layer should invoke this rule when the
 * current syntactic context begins a concurrency construct.
 *
 * The alternatives are deliberately explicit.
 *
 * This prevents arbitrary identifiers from becoming accidental concurrency
 * keywords and prevents unconstrained extension productions from swallowing
 * ordinary Zamani expressions.
 */
concurrencyConstruct
    : concurrencyTaskConstruct
    | concurrencyParallelConstruct
    | concurrencyFutureConstruct
    | concurrencyDataParallelConstruct
    | concurrencyTaskParallelConstruct
    ;


/*
 * ============================================================================
 * 2. TASK CONCURRENCY
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/concurrency/tasks.g4
 *
 * Expected public task-domain entry point:
 *
 *     taskConcurrencyExpression
 *     taskStatement
 *
 * The composition grammar must not duplicate their implementation.
 *
 * The exact production exposed by tasks.g4 is therefore isolated behind this
 * adapter. If the canonical composition parser chooses statement/expression
 * dispatch itself, it can use the appropriate task production directly.
 */
concurrencyTaskConstruct
    : taskConcurrencyExpression
    | taskStatement
    ;


/*
 * ============================================================================
 * 3. PARALLEL CONCURRENCY
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/concurrency/parallel.g4
 *
 * Parallelism is an intent/semantic property.
 *
 * The grammar does not specify:
 *
 *     worker count;
 *     thread count;
 *     core count;
 *     device count;
 *     execution width;
 *     placement;
 *     topology.
 */
concurrencyParallelConstruct
    : parallelConstruct
    ;


/*
 * ============================================================================
 * 4. FUTURE CONCURRENCY
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/concurrency/futures.g4
 *
 * Future semantics remain target-independent.
 *
 * A future may be implemented using:
 *
 *     local execution;
 *     remote execution;
 *     distributed execution;
 *     accelerator execution;
 *     quantum/classical execution;
 *     another future execution mechanism.
 */
concurrencyFutureConstruct
    : futureConstruct
    ;


/*
 * ============================================================================
 * 5. DATA PARALLEL CONCURRENCY
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * Data-parallel syntax must remain independent of:
 *
 *     SIMD width;
 *     GPU warp size;
 *     vector register width;
 *     accelerator count;
 *     machine topology.
 */
concurrencyDataParallelConstruct
    : dataParallelConstruct
    ;


/*
 * ============================================================================
 * 6. TASK PARALLEL CONCURRENCY
 * ============================================================================
 *
 * Ownership:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * Task parallelism must describe dependencies and parallel intent, not the
 * eventual number of execution resources.
 */
concurrencyTaskParallelConstruct
    : taskParallelConstruct
    ;


/*
 * ============================================================================
 * 7. CONCURRENCY EXPRESSION ADAPTER
 * ============================================================================
 *
 * The expression parser may use this rule at the appropriate expression
 * precedence/integration point.
 *
 * This rule exists only to provide a stable composition name.
 *
 * It must not recursively reference `expression`.
 */
concurrencyExpression
    : taskConcurrencyExpression
    | parallelConstruct
    | futureConstruct
    | dataParallelConstruct
    | taskParallelConstruct
    ;


/*
 * ============================================================================
 * 8. CONCURRENCY STATEMENT ADAPTER
 * ============================================================================
 *
 * Statement composition uses the independently owned concurrency statement
 * productions.
 *
 * This rule does not redefine generic statement syntax.
 */
concurrencyStatement
    : taskStatement
    | parallelStatement
    | futureStatement
    | dataParallelStatement
    | taskParallelStatement
    ;


/*
 * ============================================================================
 * 9. CONCURRENCY SPECIFICATION
 * ============================================================================
 *
 * Requirements are semantic declarations, not machine selections.
 *
 * Example conceptual meaning:
 *
 *     requires asynchronous
 *
 * does NOT mean:
 *
 *     use N threads
 *     use N cores
 *     use device X
 *
 * The semantic/resource layers interpret the requirement.
 *
 * IMPORTANT:
 *
 * This production is intentionally limited to vocabulary that already has a
 * canonical lexical contract.
 *
 * `REQUIRES` is therefore the only reserved keyword used here for this
 * purpose.
 */
concurrencyRequirementClause
    : REQUIRES
      concurrencyRequirement
      (
          COMMA
          concurrencyRequirement
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. CONCURRENCY REQUIREMENT
 * ============================================================================
 *
 * Requirement identity is represented structurally rather than by a closed
 * list of hardware names.
 *
 * This keeps the language open to future computational substrates.
 *
 * The semantic layer decides whether a referenced capability/requirement is:
 *
 *     known;
 *     supported;
 *     satisfiable;
 *     portable;
 *     target-specific;
 *     incompatible with another requirement.
 *
 * No physical resource is selected here.
 */
concurrencyRequirement
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. CONCURRENCY REQUIREMENT LIST
 * ============================================================================
 *
 * This named rule exists so diagnostics and semantic tooling have a stable
 * syntactic boundary.
 */
concurrencyRequirementList
    : concurrencyRequirement
      (
          COMMA
          concurrencyRequirement
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. CONCURRENCY DOMAIN COMPOSITION
 * ============================================================================
 *
 * This production is useful to consumers that need to distinguish a
 * concurrency-domain node from an ordinary expression/statement without
 * depending on the individual domain files.
 */
concurrencyDomain
    : concurrencyConstruct
    ;


/*
 * ============================================================================
 * 13. AST CONTRACT
 * ============================================================================
 *
 * Every accepted production must map into the canonical frontend AST.
 *
 * This grammar itself does not construct AST nodes.
 *
 * Minimum semantic classifications expected downstream:
 *
 *     TaskConcurrency
 *     ParallelConcurrency
 *     FutureConcurrency
 *     DataParallelConcurrency
 *     TaskParallelConcurrency
 *     ConcurrencyRequirement
 *
 * The AST must preserve:
 *
 *     source span;
 *     source ordering;
 *     construct kind;
 *     child expressions;
 *     child statements;
 *     semantic names;
 *     requirements;
 *     source-level attributes handled by the canonical attribute system.
 *
 * No AST node may contain an implicit:
 *
 *     thread count;
 *     worker count;
 *     core count;
 *     device count;
 *     machine identifier;
 *     hardware topology.
 *
 * Those belong downstream.
 */


/*
 * ============================================================================
 * 14. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     task validity;
 *     awaitability;
 *     parallel legality;
 *     data dependency analysis;
 *     task dependency analysis;
 *     effect checking;
 *     ownership/lifetime checking;
 *     synchronization correctness;
 *     resource requirements;
 *     capability requirements;
 *     determinism analysis;
 *     cancellation semantics;
 *     distributed execution legality.
 *
 * Syntax acceptance MUST NOT imply semantic validity.
 *
 * For example:
 *
 *     await value
 *
 * may parse successfully while semantic analysis determines that `value` is
 * not awaitable.
 *
 * Likewise:
 *
 *     parallel computation
 *
 * may parse successfully while semantic analysis determines that the
 * computation cannot legally be parallelized.
 */


/*
 * ============================================================================
 * 15. MEMORY INTEGRATION
 * ============================================================================
 *
 * Concurrency does not own memory semantics.
 *
 * The memory/type system remains authoritative for:
 *
 *     ownership;
 *     borrowing;
 *     lifetimes;
 *     aliasing;
 *     mutation;
 *     shared access.
 *
 * A concurrent boundary may cause additional semantic requirements, but those
 * requirements are derived downstream.
 *
 * No concurrency grammar rule may introduce a second ownership model.
 */


/*
 * ============================================================================
 * 16. EFFECT INTEGRATION
 * ============================================================================
 *
 * Concurrency may introduce effects such as:
 *
 *     asynchronous execution;
 *     synchronization;
 *     communication;
 *     nondeterministic ordering;
 *     distributed interaction;
 *     cancellation;
 *     external execution.
 *
 * The effect system owns their representation.
 *
 * This grammar merely identifies the source construct that can produce those
 * effects.
 */


/*
 * ============================================================================
 * 17. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis is downstream.
 *
 * It may derive requirements concerning:
 *
 *     execution capacity;
 *     memory;
 *     communication;
 *     latency;
 *     energy;
 *     reliability;
 *     accelerator capability;
 *     quantum/classical capability;
 *     distributed placement.
 *
 * This file imposes none of those limits.
 *
 * In particular, no grammar production may introduce:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_NODES
 *     MAX_DEVICES
 */


/*
 * ============================================================================
 * 18. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling consumes the dependency/semantic representation produced after
 * parsing.
 *
 * This grammar does not select:
 *
 *     ASAP;
 *     ALAP;
 *     list scheduling;
 *     critical-path scheduling;
 *     RCPSP;
 *     resource allocation;
 *     timing slots;
 *     worker assignment;
 *     hardware placement.
 *
 * Those remain scheduling-layer decisions.
 */


/*
 * ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Concurrency may surround or coordinate quantum computation.
 *
 * It MUST NOT define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     quantum gates;
 *     circuits;
 *     quantum operations;
 *     QEC;
 *     ZQN;
 *     quantum routing;
 *     quantum scheduling;
 *     pulse semantics.
 *
 * Quantum syntax remains owned by the quantum grammar and is lowered through
 * the canonical `quantum::ir` semantic boundary.
 *
 * A concurrency construct may therefore contain quantum computation through
 * the ordinary expression/domain composition model without making concurrency
 * responsible for quantum representation.
 */


/*
 * ============================================================================
 * 20. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical computation remains an ordinary consumer of concurrency intent.
 *
 * This grammar imposes no distinction between:
 *
 *     scalar work;
 *     vector work;
 *     matrix work;
 *     tensor work;
 *     accelerator work;
 *
 * when that distinction is not syntactically necessary.
 *
 * The classical semantic/IR layers determine the actual representation.
 */


/*
 * ============================================================================
 * 21. HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Concurrency syntax must remain usable around hardware/HDL-related
 * computations without selecting a physical implementation.
 *
 * This grammar does not define:
 *
 *     clocks;
 *     wires;
 *     ports;
 *     FPGA resources;
 *     ASIC cells;
 *     hardware threads;
 *     device addresses.
 *
 * Those concepts remain owned by the HDL/hardware domains.
 */


/*
 * ============================================================================
 * 22. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Concurrency and distribution are related but not identical.
 *
 * A concurrent computation may execute entirely locally.
 *
 * A distributed computation may execute across multiple nodes.
 *
 * The same source-level concurrency semantics may be lowered differently
 * according to target capabilities.
 *
 * No node count or network topology is represented here.
 */


/*
 * ============================================================================
 * 23. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime realization may map concurrency semantics to:
 *
 *     coroutines;
 *     event loops;
 *     work stealing;
 *     thread pools;
 *     OS threads;
 *     accelerator queues;
 *     distributed execution;
 *     heterogeneous execution;
 *     future execution mechanisms.
 *
 * None of these are grammar-level concepts.
 *
 * The runtime consumes canonical semantic/IR representations rather than
 * reparsing source grammar.
 */


/*
 * ============================================================================
 * 24. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Given:
 *
 *     identical source;
 *     identical language version;
 *     identical grammar version;
 *
 * the parser must produce the same syntactic result.
 *
 * Runtime nondeterminism is not parser nondeterminism.
 *
 * The grammar must not depend on:
 *
 *     hash-map iteration order;
 *     runtime scheduling;
 *     hardware availability;
 *     thread count;
 *     device discovery.
 */


/*
 * ============================================================================
 * 25. SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite language-level limit is encoded for:
 *
 *     concurrency constructs;
 *     task count;
 *     parallel regions;
 *     nested expressions;
 *     distributed participants;
 *     accelerator count;
 *     quantum resources;
 *     hardware resources.
 *
 * "Infinity" here means:
 *
 *     no artificial grammar-imposed finite maximum.
 *
 * Actual compilation/execution remains bounded by available:
 *
 *     memory;
 *     parser resources;
 *     compiler resources;
 *     runtime resources;
 *     target resources;
 *     deployment policy.
 *
 * Those limitations must be reported by the appropriate subsystem rather than
 * masquerading as syntax restrictions.
 */


/*
 * ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must remain free of:
 *
 *     fixed machine sizes;
 *     fixed worker counts;
 *     fixed thread counts;
 *     fixed core counts;
 *     fixed node counts;
 *     fixed device identifiers;
 *     fixed topology;
 *     fixed queue capacity;
 *     fixed stack size;
 *     fixed memory size;
 *     fixed accelerator width;
 *     fixed quantum width.
 *
 * Any number appearing in this file must be grammar punctuation/version
 * machinery, not a hidden machine capacity.
 */


/*
 * ============================================================================
 * 27. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical constructs:
 *
 *     async
 *     await
 *     spawn
 *     parallel
 *
 * must retain their established meaning.
 *
 * A new concurrency construct requires coordinated updates to:
 *
 *     specification;
 *     lexical vocabulary, if necessary;
 *     parser ownership;
 *     AST;
 *     semantic analysis;
 *     diagnostics;
 *     IR lowering;
 *     compatibility policy;
 *     tests;
 *     documentation.
 *
 * This file must never silently create a new keyword merely by accepting an
 * arbitrary identifier.
 */


/*
 * ============================================================================
 * 28. NEGATIVE-AMBIGUITY CONTRACT
 * ============================================================================
 *
 * The following anti-patterns are explicitly forbidden:
 *
 *     identifier blockExpression
 *
 * as a generic concurrency construct.
 *
 * This prevents ordinary language constructs such as:
 *
 *     foo { ... }
 *
 * from accidentally becoming concurrency syntax.
 *
 * Likewise, generic:
 *
 *     qualifiedName(...)
 *
 * forms must not be classified as concurrency merely because their name
 * happens to look concurrency-related.
 *
 * Such extension mechanisms require explicit dialect/keyword contracts.
 */


/*
 * ============================================================================
 * 29. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *     async task programs;
 *     await expressions;
 *     spawn expressions;
 *     parallel expressions;
 *     data-parallel constructs;
 *     task-parallel constructs;
 *     future constructs;
 *     valid requirement clauses.
 *
 * Negative tests:
 *
 *     incomplete async constructs;
 *     await without operand;
 *     spawn without operand;
 *     parallel without operand;
 *     malformed requirement clauses;
 *     malformed nested concurrency constructs;
 *     accidental identifier-as-concurrency constructs.
 *
 * Boundary tests:
 *
 *     deeply nested concurrency;
 *     large task graphs;
 *     large parallel regions;
 *     large requirement lists;
 *     large expressions;
 *     large source units.
 *
 * Cross-domain tests:
 *
 *     classical + concurrency;
 *     quantum + concurrency;
 *     quantum + classical + concurrency;
 *     HDL + concurrency;
 *     hardware + concurrency;
 *     distributed + concurrency;
 *     AI + concurrency;
 *     accelerator + concurrency;
 *     resilience + concurrency.
 *
 * Scalability tests:
 *
 *     no syntax-level maximum on task count;
 *     no syntax-level maximum on parallel regions;
 *     no syntax-level maximum on participants;
 *     no machine-topology dependency.
 *
 * Determinism tests:
 *
 *     same source -> same token/parse structure.
 *
 * Round-trip tests:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> canonical printer
 *       -> parser
 *
 * must preserve intended concurrency semantics.
 */


/*
 * ============================================================================
 * 30. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] It is the sole composition root for grammar/concurrency/.
 *
 * [ ] It does not duplicate task syntax owned by tasks.g4.
 *
 * [ ] It does not duplicate parallel syntax owned by parallel.g4.
 *
 * [ ] It does not duplicate future syntax owned by futures.g4.
 *
 * [ ] It does not duplicate data-parallel syntax.
 *
 * [ ] It does not duplicate task-parallel syntax.
 *
 * [ ] It contains no lexer rules.
 *
 * [ ] It contains no runtime implementation.
 *
 * [ ] It contains no scheduler implementation.
 *
 * [ ] It contains no hardware assumptions.
 *
 * [ ] It contains no quantum IR.
 *
 * [ ] It contains no QEC or ZQN semantics.
 *
 * [ ] It contains no fixed machine/resource maximum.
 *
 * [ ] It does not treat arbitrary identifiers as concurrency keywords.
 *
 * [ ] Every referenced domain rule has exactly one canonical owner.
 *
 * [ ] Every referenced domain rule has a defined AST destination.
 *
 * [ ] Semantic analysis consumes the resulting AST.
 *
 * [ ] Resource analysis remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] Runtime remains downstream.
 *
 * [ ] Rust implementation remains safe Rust 1.97/1.97.1.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Round-trip tests exist.
 *
 * [ ] POCO-REAF scalability tests exist.
 *
 * ============================================================================
 */