/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/execution.g4
 *
 * Grammar:
 *     Execution
 *
 * Status:
 *     Production execution-domain composition root
 *
 * Language:
 *     Zamani
 *
 * Toolchain:
 *     ANTLR parser grammar
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the CANONICAL COMPOSITION ROOT for the source-level execution
 * domain.
 *
 * It owns the stable boundary between:
 *
 *     "this computation is intended to execute"
 *
 * and the specialized execution-intent grammars that describe:
 *
 *     runtime intent
 *     entry-point intent
 *     scheduling intent
 *     placement intent
 *     recovery intent
 *     checkpoint intent
 *     tracing intent
 *     profiling intent
 *     lifecycle intent
 *     dispatch intent
 *     deployment intent
 *     synchronization intent
 *
 * This file describes SOURCE SYNTAX ONLY.
 *
 * It does NOT execute anything.
 *
 * It does NOT:
 *
 *     discover hardware
 *     allocate hardware
 *     select devices
 *     route operations
 *     schedule operations
 *     perform QEC
 *     perform ZQN processing
 *     perform optimization
 *     perform calibration
 *     perform recovery
 *     perform deployment
 *     invoke runtime APIs
 *     invoke vendor APIs
 *     create a runtime IR
 *     create an execution IR
 *     create a second quantum IR
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                        ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                    domain-neutral AST
 *                              |
 *                              v
 *                    semantic analysis
 *                              |
 *            +-----------------+------------------+
 *            |                 |                  |
 *            v                 v                  v
 *        capabilities       resources          effects
 *            |                 |                  |
 *            +-----------------+------------------+
 *                              |
 *                              v
 *                   canonical semantic model
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *         classical IR      quantum::ir    HDL/hardware
 *              |               |               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *          placement       scheduling       routing
 *              |               |               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                    resilience / recovery
 *                              |
 *                              v
 *                             QEC
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                    target realization
 *                              |
 *                              v
 *                           runtime
 *
 * Execution grammar is therefore an INTENT boundary, not an implementation
 * boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Execution syntax MUST remain target-independent.
 *
 * A source program may execute:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL computation
 *     hardware/software co-designed computation
 *     distributed computation
 *     AI computation
 *     tensor/data computation
 *     accelerator computation
 *     scientific computation
 *     embedded computation
 *     future computation
 *
 * without changing the execution grammar merely because the realization
 * changes.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define universal limits such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_TIMELINES
 *     MAX_TASKS
 *     MAX_JOBS
 *     MAX_EXECUTIONS
 *     MAX_CONTEXT_ENTRIES
 *     MAX_RETRIES
 *     MAX_CHECKPOINTS
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *
 * The grammar also MUST NOT encode physical identities such as:
 *
 *     cpu(0)
 *     gpu(0)
 *     qpu(0)
 *     fpga(0)
 *     node(0)
 *     qubit(0)
 *
 * as the portable execution model.
 *
 * A concrete value in source is program semantics.
 *
 * A compiler/runtime capacity is NOT a language capacity.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Execution intent must preserve the distinction between:
 *
 *     requirement
 *     constraint
 *     capability
 *     resource
 *     preference
 *     hint
 *
 * Examples:
 *
 *     requires capability("quantum.measurement")
 *
 *     requires qubits >= n
 *
 *     requires memory >= required_memory
 *
 *     prefer capability("gpu.compute")
 *
 *     hint scheduling.policy
 *
 * These do not select a physical machine.
 *
 * Semantic analysis and target realization determine whether and how the
 * request can be satisfied.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Execution may execute a quantum computation.
 *
 * Execution MUST NOT define quantum operations.
 *
 * Quantum semantics remain owned by grammar/quantum/.
 *
 * Quantum lowering remains:
 *
 *     quantum source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum model
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
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *
 * Execution MUST NEVER introduce:
 *
 *     ExecutionQuantumIR
 *     RuntimeQuantumIR
 *     PhysicalQuantumIR
 *
 * or any other competing quantum representation.
 *
 * `quantum::ir` remains the canonical quantum IR boundary.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * executionSubject is deliberately generic.
 *
 * It does not enumerate:
 *
 *     classical
 *     quantum
 *     GPU
 *     FPGA
 *     QPU
 *     AI
 *     tensor
 *     distributed
 *     HDL
 *     vendor
 *
 * as a closed execution vocabulary.
 *
 * The subject is an existing Zamani expression/block computation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     executionDeclaration
 *     executionRequest
 *     executionSubject
 *     executionContextAttachment
 *     executionStatement
 *     executionExpression
 *     executionDomainDeclaration
 *     executionDomainStatement
 *     executionDomainExpression
 *     composition of independent execution leaf grammars
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical rules
 *     identifiers
 *     expressions
 *     types
 *     blocks
 *     scheduling rules
 *     placement rules
 *     recovery rules
 *     checkpoint rules
 *     tracing rules
 *     profiling rules
 *     lifecycle rules
 *     dispatch rules
 *     deployment rules
 *     synchronization rules
 *     runtime implementation
 *     hardware realization
 *     resource discovery
 *     target discovery
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     quantum::ir
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Canonical direction:
 *
 *     lexer
 *       |
 *       v
 *     foundational parser grammars
 *       |
 *       v
 *     execution leaf grammars
 *       |
 *       v
 *     Execution
 *       |
 *       v
 *     ZamaniParser
 *
 * Execution leaf grammars MUST NOT depend on Execution merely to obtain
 * execution concepts.
 *
 * This prevents:
 *
 *     Execution -> Runtime -> Execution
 *
 * and similar cycles.
 *
 * ============================================================================
 * CURRENT REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The following independent execution grammars are composed here:
 *
 *     ExecutionContext
 *     ExecutionEntryPoints
 *     Runtime
 *     Scheduling
 *     ExecutionPlacement
 *     Recovery
 *     Checkpointing
 *     Tracing
 *     Profiling
 *     Lifecycle
 *     Dispatch
 *     Deployment
 *     ExecutionSynchronization
 *
 * These are deliberately composed through their PUBLIC entry points.
 *
 * ============================================================================
 * KNOWN BOUNDARIES THAT MUST NOT BE MASKED HERE
 * ============================================================================
 *
 * observability.g4
 * ----------------
 *
 * The current repository version imports Execution itself.
 *
 * Therefore:
 *
 *     Execution -> Observability -> Execution
 *
 * would create a circular parser dependency.
 *
 * Execution MUST NOT duplicate observability syntax to work around this.
 *
 * Observability must first be made dependency-direction-safe by making it
 * depend only on foundational grammars. After that correction it can be
 * imported here without changing the execution ownership model.
 *
 *
 * resilience.g4
 * -------------
 *
 * The current repository version contains duplicate resilience rule
 * definitions.
 *
 * Execution MUST NOT copy resilience syntax to compensate.
 *
 * Resilience remains a separate authority and becomes composable here after
 * its own duplicate-rule defect is corrected.
 *
 *
 * environments.g4
 * ---------------
 *
 * The current repository version documents the intended environment grammar
 * but does not currently expose a normal parser-grammar declaration.
 *
 * Execution therefore MUST NOT invent a second environment grammar.
 *
 * Once environments.g4 exposes its canonical parser grammar, it may be added
 * to this composition root.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT create local keyword tokens.
 *
 * Contextual vocabulary belongs to the relevant semantic grammar and/or
 * canonical lexer specification.
 *
 * ============================================================================
 * CORE DEPENDENCY
 * ============================================================================
 *
 * Core remains the authority for:
 *
 *     expression
 *     blockExpression
 *     identifier
 *     qualifiedName
 *     argumentList
 *     common source structures
 *
 * Execution MUST NOT create replacement versions of these constructs.
 *
 * ============================================================================
 * PUBLIC COMPOSITION
 * ============================================================================
 *
 * The root parser imports Execution.
 *
 * Therefore the following public execution rules are intentionally exposed:
 *
 *     executionDeclaration
 *     executionStatement
 *     executionExpression
 *
 * The specialized grammars remain reachable through:
 *
 *     executionDomainDeclaration
 *     executionDomainStatement
 *     executionDomainExpression
 *
 * ============================================================================
 */

parser grammar Execution;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    ExecutionContext,
    ExecutionEntryPoints,
    Runtime,
    Scheduling,
    ExecutionPlacement,
    Recovery,
    Checkpointing,
    Tracing,
    Profiling,
    Lifecycle,
    Dispatch,
    Deployment,
    ExecutionSynchronization
;


/*
 * ============================================================================
 * 1. PRIMARY EXECUTION DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     execute <computation>;
 *
 * Optional context:
 *
 *     execute <computation> with {
 *         ...
 *     };
 *
 * The subject is semantic computation, not a physical target.
 *
 * ============================================================================
 */

executionDeclaration
    : EXECUTE executionRequest executionTerminator?
    ;


/*
 * ============================================================================
 * 2. EXECUTION REQUEST
 * ============================================================================
 */

executionRequest
    : executionSubject executionContextAttachment?
    ;


/*
 * ============================================================================
 * 3. EXECUTION SUBJECT
 * ============================================================================
 *
 * The canonical subject is an existing expression.
 *
 * A block expression is accepted explicitly so execution can represent
 * inline computations without inventing a second block language.
 *
 * ============================================================================
 */

executionSubject
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 4. EXECUTION CONTEXT ATTACHMENT
 * ============================================================================
 *
 * ExecutionContext owns all context internals.
 *
 * Execution does not duplicate:
 *
 *     context entries
 *     context keys
 *     context values
 *     context objects
 *     context comparisons
 *     context lists
 *
 * ============================================================================
 */

executionContextAttachment
    : WITH executionContext
    ;


/*
 * ============================================================================
 * 5. TERMINATOR
 * ============================================================================
 */

executionTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * 6. EXECUTION-DOMAIN DECLARATIONS
 * ============================================================================
 *
 * These alternatives expose the public declaration contracts of the
 * independent execution grammars.
 *
 * No leaf syntax is duplicated here.
 * ============================================================================
 */

executionDomainDeclaration
    : entryPointDeclaration
    | runtimeDeclaration
    | recoveryDeclaration
    | checkpointingDeclaration
    | lifecycleDeclaration
    | dispatchDeclaration
    | deploymentDeclaration
    | synchronizationDeclaration
    | schedulingDeclaration
    | executionPlacement
    | tracingConstruct
    | profilingConstruct
    ;


/*
 * ============================================================================
 * 7. EXECUTION-DOMAIN STATEMENTS
 * ============================================================================
 *
 * Each leaf grammar owns its own statement syntax.
 *
 * ============================================================================
 */

executionDomainStatement
    : runtimeStatement
    | recoveryStatement
    | checkpointingStatement
    | lifecycleStatement
    | synchronizationDeclaration
    ;


/*
 * ============================================================================
 * 8. EXECUTION-DOMAIN EXPRESSIONS
 * ============================================================================
 *
 * These are semantic execution expressions supplied by independent execution
 * grammars.
 *
 * No execution-specific expression language is created here.
 * ============================================================================
 */

executionDomainExpression
    : runtimeExpression
    | recoveryExpression
    | checkpointingExpression
    ;


/*
 * ============================================================================
 * 9. EXECUTION STATEMENT
 * ============================================================================
 *
 * The execution domain is intentionally additive.
 *
 * The generic `execute` form is kept separate from specialized execution
 * declarations.
 *
 * ============================================================================
 */

executionStatement
    : executionDomainStatement
    ;


/*
 * ============================================================================
 * 10. EXECUTION EXPRESSION
 * ============================================================================
 */

executionExpression
    : executionDomainExpression
    ;


/*
 * ============================================================================
 * 11. PUBLIC EXECUTION DOMAIN ENTRY
 * ============================================================================
 *
 * This rule gives tooling a single execution-domain entry point without
 * creating another program root.
 *
 * ============================================================================
 */

executionDomainElement
    : executionDeclaration
    | executionDomainDeclaration
    | executionDomainStatement
    | executionDomainExpression
    ;


/*
 * ============================================================================
 * 12. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling is an independent authority.
 *
 * execution.g4 only exposes its public declaration.
 *
 * Scheduling semantics are resolved downstream:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic scheduling intent
 *       |
 *       v
 *     scheduler
 *
 * The grammar does not calculate schedules.
 *
 * ============================================================================
 */

executionScheduling
    : executionSchedule
    ;


/*
 * ============================================================================
 * 13. PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Placement remains semantic intent.
 *
 * The grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     node
 *     physical qubit
 *     memory bank
 *
 * ============================================================================
 */

executionPlacementIntent
    : executionPlacement
    ;


/*
 * ============================================================================
 * 14. RECOVERY INTEGRATION
 * ============================================================================
 */

executionRecovery
    : recoveryDeclaration
    ;


/*
 * ============================================================================
 * 15. CHECKPOINT INTEGRATION
 * ============================================================================
 */

executionCheckpointing
    : checkpointingDeclaration
    ;


/*
 * ============================================================================
 * 16. TRACING INTEGRATION
 * ============================================================================
 */

executionTracing
    : tracingConstruct
    ;


/*
 * ============================================================================
 * 17. PROFILING INTEGRATION
 * ============================================================================
 */

executionProfiling
    : profilingConstruct
    ;


/*
 * ============================================================================
 * 18. LIFECYCLE INTEGRATION
 * ============================================================================
 */

executionLifecycle
    : lifecycleDeclaration
    ;


/*
 * ============================================================================
 * 19. DISPATCH INTEGRATION
 * ============================================================================
 */

executionDispatch
    : dispatchDeclaration
    ;


/*
 * ============================================================================
 * 20. DEPLOYMENT INTEGRATION
 * ============================================================================
 */

executionDeployment
    : deploymentDeclaration
    ;


/*
 * ============================================================================
 * 21. SYNCHRONIZATION INTEGRATION
 * ============================================================================
 */

executionSynchronization
    : synchronizationDeclaration
    ;


/*
 * ============================================================================
 * 22. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime remains a separate authority.
 *
 * The runtime grammar provides:
 *
 *     runtimeDeclaration
 *     runtimeStatement
 *     runtimeExpression
 *
 * Execution merely exposes those contracts.
 *
 * Runtime implementation occurs after semantic analysis.
 *
 * ============================================================================
 */

executionRuntimeDeclaration
    : runtimeDeclaration
    ;

executionRuntimeStatement
    : runtimeStatement
    ;

executionRuntimeExpression
    : runtimeExpression
    ;


/*
 * ============================================================================
 * 23. ENTRY-POINT INTEGRATION
 * ============================================================================
 */

executionEntryPoint
    : entryPointDeclaration
    ;


/*
 * ============================================================================
 * 24. CONTEXT INTEGRATION
 * ============================================================================
 *
 * This explicit wrapper is useful to downstream tooling and keeps
 * ExecutionContext independently testable.
 * ============================================================================
 */

executionContext
    : optionalExecutionContext
    ;


/*
 * ============================================================================
 * 25. SEMANTIC CONTRACT
 * ============================================================================
 *
 * The syntax represented by this grammar maps conceptually to:
 *
 *     ExecutionDeclaration {
 *         subject: Computation,
 *         context: Option<ExecutionContext>,
 *         source_span: SourceSpan
 *     }
 *
 * Specialized declarations map to domain-neutral AST nodes such as:
 *
 *     EntryPointDecl
 *     RuntimeIntent
 *     ScheduleIntent
 *     PlacementIntent
 *     RecoveryIntent
 *     CheckpointIntent
 *     TraceIntent
 *     ProfileIntent
 *     LifecycleIntent
 *     DispatchIntent
 *     DeploymentIntent
 *     SynchronizationIntent
 *
 * Exact Rust AST types belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT require:
 *
 *     CpuId
 *     GpuId
 *     FpgaId
 *     QpuId
 *     PhysicalQubitId
 *     PhysicalMemoryAddress
 *     HardwareTopology
 *
 * as universal AST requirements.
 *
 * ============================================================================
 * 26. RESOURCE AND CAPABILITY SEMANTICS
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     preferences
 *     hints
 *
 * against:
 *
 *     program semantics
 *     effects
 *     target capabilities
 *     available resources
 *     portability requirements
 *     execution policy
 *
 * The parser does none of this.
 *
 * ============================================================================
 * 27. COMPILATION CONTRACT
 * ============================================================================
 *
 * Execution syntax may identify the computation that is eventually executed.
 *
 * It does not define:
 *
 *     optimization
 *     lowering
 *     code generation
 *     instruction selection
 *     target-specific compilation
 *     binary layout
 *
 * Those remain compiler concerns.
 *
 * ============================================================================
 * 28. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling grammar describes scheduling intent.
 *
 * The scheduler determines realization.
 *
 * This grammar does not calculate:
 *
 *     start time
 *     end time
 *     critical path
 *     resource occupancy
 *     ordering
 *     temporal placement
 *
 * ============================================================================
 * 29. PLACEMENT / ROUTING CONTRACT
 * ============================================================================
 *
 * Placement grammar describes intent.
 *
 * Routing remains downstream.
 *
 * Execution does not select physical:
 *
 *     cores
 *     accelerators
 *     FPGA regions
 *     QPU qubits
 *     nodes
 *     memory banks
 *     network links
 *
 * ============================================================================
 * 30. RECOVERY / RESILIENCE CONTRACT
 * ============================================================================
 *
 * Recovery syntax represents recovery intent.
 *
 * Resilience semantics remain a separate concern.
 *
 * The runtime/compiler may ultimately use:
 *
 *     retry
 *     restart
 *     resume
 *     rollback
 *     migration
 *     compensation
 *     checkpoint restoration
 *     escalation
 *     degraded acceptance
 *
 * without the parser implementing any of these algorithms.
 *
 * ============================================================================
 * 31. CHECKPOINT CONTRACT
 * ============================================================================
 *
 * Checkpointing is source-level intent.
 *
 * It does not define:
 *
 *     storage backend
 *     physical storage address
 *     serialization implementation
 *     replication algorithm
 *     recovery algorithm
 *
 * Those belong downstream.
 *
 * ============================================================================
 * 32. OBSERVABILITY CONTRACT
 * ============================================================================
 *
 * Tracing and profiling remain portable source intent.
 *
 * This grammar does not select:
 *
 *     OpenTelemetry
 *     Prometheus
 *     vendor profiler
 *     tracing SDK
 *     storage backend
 *
 * Runtime realization chooses the implementation.
 *
 * ============================================================================
 * 33. LIFECYCLE CONTRACT
 * ============================================================================
 *
 * Lifecycle is a semantic state model.
 *
 * Execution does not enumerate lifecycle states.
 *
 * Therefore future states can be represented without changing this grammar.
 *
 * ============================================================================
 * 34. DISPATCH CONTRACT
 * ============================================================================
 *
 * Dispatch intent describes a handoff boundary.
 *
 * It does not define:
 *
 *     process spawning implementation
 *     OS syscall
 *     queue implementation
 *     device API
 *     network transport
 *
 * ============================================================================
 * 35. DEPLOYMENT CONTRACT
 * ============================================================================
 *
 * Deployment intent remains target-independent.
 *
 * Execution does not define:
 *
 *     cloud provider
 *     container engine
 *     cluster scheduler
 *     node allocator
 *     service mesh
 *     deployment API
 *
 * ============================================================================
 * 36. SYNCHRONIZATION CONTRACT
 * ============================================================================
 *
 * Synchronization describes semantic ordering and dependency intent.
 *
 * It does not mandate:
 *
 *     mutexes
 *     CPU fences
 *     GPU fences
 *     barriers
 *     semaphores
 *     vendor synchronization primitives
 *
 * The realization is downstream.
 *
 * ============================================================================
 * 37. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic actions
 *     no predicates
 *     no runtime calls
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no randomness
 *     no mutable global state
 *
 * Equivalent canonical token streams must produce equivalent parse structures.
 *
 * Runtime nondeterminism is a semantic/runtime property and does not alter
 * parser determinism.
 *
 * ============================================================================
 * 38. SOURCE SPANS
 * ============================================================================
 *
 * Every execution AST node MUST preserve source-span information.
 *
 * This grammar does not implement source spans itself.
 *
 * The parser/frontend infrastructure must preserve:
 *
 *     start offset
 *     end offset
 *     line
 *     column
 *     source identity
 *
 * sufficiently for diagnostics, tooling, formatting and provenance.
 *
 * ============================================================================
 * 39. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax errors:
 *
 *     parser
 *
 * Semantic errors:
 *
 *     semantic analysis
 *
 * Resource errors:
 *
 *     resource analysis
 *
 * Capability errors:
 *
 *     capability analysis
 *
 * Target errors:
 *
 *     target resolution
 *
 * Scheduling errors:
 *
 *     scheduler
 *
 * Runtime failures:
 *
 *     runtime
 *
 * Execution.g4 MUST NOT collapse these error classes into syntax errors.
 *
 * ============================================================================
 * 40. SCALABILITY
 * ============================================================================
 *
 * Repetition is structural.
 *
 * This grammar places no finite language-level ceiling on:
 *
 *     execution declarations
 *     execution subjects
 *     context entries
 *     context nesting
 *     scheduling properties
 *     placement properties
 *     recovery policies
 *     checkpoints
 *     traces
 *     profiling entries
 *     lifecycle states
 *     lifecycle transitions
 *     synchronization dependencies
 *     distributed resources
 *     quantum resources
 *     classical resources
 *     accelerator resources
 *
 * Actual limits arise only from:
 *
 *     parser implementation resources
 *     compiler resources
 *     runtime resources
 *     available target resources
 *     operating-environment limits
 *     explicit program requirements
 *
 * Those are not grammar capacities.
 *
 * ============================================================================
 * 41. FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * A future computing technology should first attempt integration through:
 *
 *     expressions
 *     types
 *     attributes
 *     capabilities
 *     resources
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     dialects
 *
 * rather than adding a new execution keyword.
 *
 * This allows execution.g4 to remain stable as the set of computational
 * technologies expands.
 *
 * ============================================================================
 * 42. SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Therefore it requires no unsafe Rust.
 *
 * The generated Zamani frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No execution grammar feature may require `unsafe`.
 *
 * ============================================================================
 * 43. COMPLETION CRITERIA
 * ============================================================================
 *
 * execution.g4 is complete when:
 *
 * [x] It is the sole execution composition root.
 * [x] It preserves the existing filename.
 * [x] It uses the canonical ZamaniLexer vocabulary.
 * [x] It reuses Core syntax.
 * [x] It reuses ExecutionContext.
 * [x] It exposes the independent execution leaf grammars.
 * [x] It does not duplicate leaf grammar ownership.
 * [x] It does not create an execution IR.
 * [x] It preserves quantum::ir as the canonical quantum boundary.
 * [x] It contains no hardware capacity limits.
 * [x] It contains no physical-device selection.
 * [x] It contains no runtime implementation.
 * [x] It contains no scheduling implementation.
 * [x] It contains no routing implementation.
 * [x] It contains no QEC implementation.
 * [x] It contains no ZQN implementation.
 * [x] It contains no semantic actions.
 * [x] It requires no unsafe Rust.
 * [x] It remains domain-neutral.
 * [x] It supports POCO-REAF.
 *
 * Repository integration gates that remain external to this file:
 *
 * [ ] observability.g4 must break its Execution dependency before composition.
 * [ ] resilience.g4 must remove duplicate rule definitions before composition.
 * [ ] environments.g4 must expose a canonical parser grammar before
 *     composition.
 *
 * Those are intentionally NOT duplicated or hidden here.
 *
 * ============================================================================
 * END OF EXECUTION.G4
 * ============================================================================
 */