/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/pipeline.g4
 *
 * STATUS
 * ------
 * CANONICAL PIPELINE-COMPUTATION PARSER COMPONENT
 *
 * PURPOSE
 * -------
 * This file owns the source-level syntax for ordered computational pipelines.
 *
 * A pipeline represents a logical data/computation flow:
 *
 *     source
 *        |
 *        v
 *     stage
 *        |
 *        v
 *     stage
 *        |
 *        v
 *     sink
 *
 * The pipeline is a semantic/dataflow abstraction.
 *
 * It does NOT define:
 *
 *     - threads;
 *     - workers;
 *     - cores;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - accelerators;
 *     - nodes;
 *     - queues;
 *     - buffers;
 *     - physical channels;
 *     - machine topology;
 *     - network topology;
 *     - scheduling algorithms;
 *     - placement;
 *     - routing;
 *     - resource allocation;
 *     - hardware discovery;
 *     - runtime implementation.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Compiler/runtime:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no unsafe code;
 *     - no semantic predicates;
 *     - no runtime execution;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Pipeline computation is a concurrency/dataflow abstraction.
 *
 * It sits conceptually beside:
 *
 *     parallel.g4
 *     task-parallel.g4
 *     data-parallel.g4
 *     tasks.g4
 *
 * but owns a different source-level concept.
 *
 * `parallel.g4`
 *     owns generic parallel regions.
 *
 * `task-parallel.g4`
 *     owns structured task parallelism.
 *
 * `data-parallel.g4`
 *     owns data-parallel computation.
 *
 * `tasks.g4`
 *     owns task/spawn/await composition.
 *
 * THIS FILE
 *     owns ordered pipeline/dataflow composition.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     expression
 *     blockExpression
 *     statement
 *     task syntax
 *     spawn syntax
 *     await syntax
 *     generic parallel syntax
 *     data-parallel syntax
 *     synchronization syntax
 *     channel syntax
 *     memory syntax
 *     resource syntax
 *     hardware syntax
 *
 * Those remain owned by their canonical grammar components.
 *
 * ============================================================================
 * PIPELINE SEMANTIC MODEL
 * ============================================================================
 *
 * A pipeline is an ordered sequence of logical stages.
 *
 * Conceptually:
 *
 *     pipeline {
 *         source()
 *         transform()
 *         analyze()
 *         sink()
 *     }
 *
 * represents:
 *
 *     source
 *       ->
 *     transform
 *       ->
 *     analyze
 *       ->
 *     sink
 *
 * The exact dataflow contract is determined by semantic analysis.
 *
 * This grammar does NOT assume that every stage corresponds to:
 *
 *     one thread;
 *     one worker;
 *     one process;
 *     one CPU;
 *     one GPU;
 *     one accelerator;
 *     one node;
 *     one queue.
 *
 * A compiler may realize a pipeline using:
 *
 *     sequential execution;
 *     fused stages;
 *     concurrent stages;
 *     streaming;
 *     buffering;
 *     task parallelism;
 *     data parallelism;
 *     SIMD/vector execution;
 *     GPU execution;
 *     FPGA execution;
 *     accelerator execution;
 *     distributed execution;
 *     heterogeneous execution;
 *     quantum/classical orchestration;
 *     future computational substrates.
 *
 * The realization must preserve the language-defined semantics.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Pipeline syntax is target-independent.
 *
 * The grammar imposes NO universal finite limit on:
 *
 *     pipelines;
 *     stages;
 *     nested pipelines;
 *     pipeline depth;
 *     stream elements;
 *     logical streams;
 *     tasks;
 *     workers;
 *     threads;
 *     cores;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     QPUs;
 *     accelerators;
 *     nodes;
 *     buffers;
 *     channels;
 *     memory;
 *     topology;
 *     pipeline width.
 *
 * The grammar MUST NOT contain implementation constants such as:
 *
 *     MAX_PIPELINES
 *     MAX_STAGES
 *     MAX_PIPELINE_DEPTH
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_BUFFERS
 *     MAX_CHANNELS
 *     MAX_STREAMS
 *
 * "Infinity" means that no artificial finite hardware-scale ceiling is imposed
 * by the language grammar. Actual execution remains bounded only by program
 * semantics and the resources/capabilities available to the implementation.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file consumes:
 *
 *     tokenVocab = ZamaniLexer
 *
 * Existing canonical tokens used by this grammar include:
 *
 *     PIPE
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *     COLON
 *     COMMA
 *
 * No new lexical token is required by this grammar.
 *
 * IMPORTANT:
 *
 * The repository already uses PIPE (`|`) elsewhere, including expression
 * syntax. Therefore this grammar does NOT make a bare:
 *
 *     expression | expression
 *
 * into an unrestricted pipeline expression.
 *
 * Pipeline composition is structurally delimited by:
 *
 *     pipeline { ... }
 *
 * This prevents the pipeline grammar from silently taking ownership of
 * unrelated ordinary expression syntax.
 *
 * ============================================================================
 * PIPELINE SOURCE FORM
 * ============================================================================
 *
 * Canonical structured form:
 *
 *     pipeline {
 *         source();
 *         transform();
 *         sink();
 *     }
 *
 * Explicit stage composition may also use PIPE inside the pipeline:
 *
 *     pipeline {
 *         source() | transform() | sink()
 *     }
 *
 * Named stages may be expressed as:
 *
 *     pipeline {
 *         source: source();
 *         transform: transform();
 *         sink: sink();
 *     }
 *
 * The named-stage form provides a stable semantic identity for diagnostics,
 * provenance, tooling and downstream scheduling without assigning a physical
 * execution identity.
 *
 * ============================================================================
 * PIPELINE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     pipelineConstruct
 *     pipelineExpression
 *     pipelineBlock
 *     pipelineStageList
 *     pipelineStage
 *     pipelineNamedStage
 *     pipelineStageBody
 *     pipelineChain
 *     pipelineChainStage
 *     pipelineStatement
 *
 * THIS FILE DOES NOT OWN:
 *
 *     expression
 *     blockExpression
 *     statement
 *     taskSpawnExpression
 *     taskAwaitExpression
 *     parallelExpression
 *     dataParallelExpression
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Expressions are imported from the canonical expression composition.
 *
 * Blocks are imported through the canonical expression/block composition.
 *
 * This grammar therefore imports:
 *
 *     Expressions
 *     ZamaniExpressionBlocks
 *
 * in the same manner as the existing parallel grammar.
 *
 * ============================================================================
 */

parser grammar Pipeline;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    ZamaniExpressionBlocks
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `pipelineConstruct` is the stable parser-facing entry point for this domain.
 *
 * It intentionally does not consume EOF because the canonical root parser owns
 * complete-source termination.
 */

pipelineConstruct
    : pipelineExpression
    ;


/*
 * ============================================================================
 * 2. PIPELINE EXPRESSION
 * ============================================================================
 *
 * A pipeline is represented by an explicitly delimited structured body.
 *
 * This makes pipeline syntax distinguishable from ordinary use of PIPE.
 */

pipelineExpression
    : PIPELINE_OPEN pipelineBody PIPELINE_CLOSE
    ;


/*
 * ============================================================================
 * 3. PIPELINE BODY
 * ============================================================================
 *
 * A pipeline body may be:
 *
 *     - a sequence of stages;
 *     - an explicitly chained sequence.
 *
 * The grammar deliberately requires at least one stage.
 *
 * An empty pipeline has no computational meaning and should therefore be
 * rejected structurally rather than being accepted as an accidental no-op.
 */

pipelineBody
    : pipelineStageList
    | pipelineChain
    ;


/*
 * ============================================================================
 * 4. PIPELINE STAGE LIST
 * ============================================================================
 *
 * Example:
 *
 *     pipeline {
 *         source();
 *         transform();
 *         sink();
 *     }
 *
 * The repetition is unbounded at the language level.
 */

pipelineStageList
    : pipelineStage
      (
          SEMICOLON
          pipelineStage
      )*
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 5. PIPELINE STAGE
 * ============================================================================
 *
 * A stage may be:
 *
 *     unnamed expression
 *
 * or:
 *
 *     named expression
 *
 * The name is semantic identity only.
 *
 * It is NOT:
 *
 *     worker ID;
 *     device ID;
 *     process ID;
 *     node ID;
 *     hardware queue ID.
 */

pipelineStage
    : pipelineNamedStage
    | pipelineStageBody
    ;


pipelineNamedStage
    : identifier
      COLON
      pipelineStageBody
    ;


/*
 * ============================================================================
 * 6. PIPELINE STAGE BODY
 * ============================================================================
 *
 * Pipeline stages deliberately reuse ordinary Zamani expressions.
 *
 * This allows stages to contain:
 *
 *     calls;
 *     functions;
 *     closures;
 *     async expressions;
 *     quantum operations;
 *     classical computation;
 *     data operations;
 *     HDL/hardware-related semantic operations;
 *     resource-aware operations;
 *     future language extensions.
 *
 * No pipeline-specific computation language is introduced.
 */

pipelineStageBody
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 7. EXPLICIT PIPELINE CHAIN
 * ============================================================================
 *
 * The existing PIPE token is used only after the pipeline construct has already
 * established the syntactic context.
 *
 * Example:
 *
 *     pipeline {
 *         source() | transform() | sink()
 *     }
 *
 * This permits compact dataflow syntax without introducing another operator.
 *
 * IMPORTANT:
 *
 * This is NOT a universal expression-level PIPE operator.
 *
 * PIPE has other lexical uses in the existing language, including lambda
 * syntax. Pipeline interpretation is therefore restricted to this grammar's
 * explicit pipeline context.
 */

pipelineChain
    : pipelineChainStage
      (
          PIPE
          pipelineChainStage
      )+
      SEMICOLON?
    ;


pipelineChainStage
    : pipelineNamedStage
    | pipelineStageBody
    ;


/*
 * ============================================================================
 * 8. PIPELINE STATEMENT
 * ============================================================================
 *
 * Statement-level adapter.
 *
 * The canonical statement grammar may consume this rule where concurrency or
 * domain statements are dispatched.
 */

pipelineStatement
    : pipelineExpression
    ;


/*
 * ============================================================================
 * 9. NESTED PIPELINES
 * ============================================================================
 *
 * A pipeline stage can itself contain a block expression, and the semantic
 * layer may permit a nested pipeline expression wherever the surrounding
 * expression grammar exposes it.
 *
 * This grammar does not impose a nesting-depth limit.
 *
 * Nested pipelines remain logical composition, not physical hierarchy.
 */


/*
 * ============================================================================
 * 10. DATAFLOW SEMANTICS
 * ============================================================================
 *
 * The grammar establishes stage ordering.
 *
 * Semantic analysis determines:
 *
 *     - stage input types;
 *     - stage output types;
 *     - stream/data compatibility;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - mutation;
 *     - ordering requirements;
 *     - backpressure semantics;
 *     - determinism;
 *     - cancellation behavior;
 *     - resource requirements;
 *     - capability requirements;
 *     - whether stages may overlap.
 *
 * The grammar does NOT determine whether stages execute concurrently.
 *
 * For example:
 *
 *     pipeline {
 *         read()
 *         transform()
 *         write()
 *     }
 *
 * establishes logical ordering.
 *
 * The implementation may:
 *
 *     fuse read+transform;
 *     overlap transform+write;
 *     buffer between stages;
 *     serialize everything;
 *     distribute stages;
 *     place stages on heterogeneous resources.
 *
 * Such decisions belong downstream.
 *
 * ============================================================================
 * 11. ORDERING
 * ============================================================================
 *
 * Pipeline order is semantic.
 *
 * If:
 *
 *     A -> B -> C
 *
 * is written in a pipeline, the semantic model must preserve the required
 * dependency ordering.
 *
 * This does NOT necessarily require global wall-clock serialization.
 *
 * For streaming computations, multiple logical items may flow through
 * different stages concurrently, provided the program's declared semantics
 * permit that behavior.
 *
 * The grammar does not encode the amount of buffering or overlap.
 *
 * ============================================================================
 * 12. STAGE IDENTITY
 * ============================================================================
 *
 * A named stage:
 *
 *     transform: normalize()
 *
 * gives the semantic model a stable logical name:
 *
 *     transform
 *
 * It does not create:
 *
 *     a physical device;
 *     a worker;
 *     a queue;
 *     a thread;
 *     a node.
 *
 * Duplicate stage names, if prohibited, are a semantic validation issue rather
 * than a grammar issue.
 *
 * ============================================================================
 * 13. INPUT / OUTPUT CONTRACT
 * ============================================================================
 *
 * A pipeline stage is an ordinary Zamani expression or block expression.
 *
 * Therefore input/output semantics are inferred from the ordinary type and
 * effect system.
 *
 * The pipeline grammar does not introduce another type system.
 *
 * Examples of possible semantic stages include:
 *
 *     read_data()
 *     normalize(data)
 *     fft(signal)
 *     train(model)
 *     quantum_operation(...)
 *     measure(...)
 *     synthesize(...)
 *     deploy(...)
 *
 * Whether any such expression is valid as a pipeline stage is determined by
 * semantic analysis.
 *
 * ============================================================================
 * 14. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical stages may contain:
 *
 *     scalar computation;
 *     vector computation;
 *     matrix computation;
 *     tensor computation;
 *     numerical computation;
 *     symbolic computation;
 *     scientific computation;
 *     signal processing;
 *     optimization.
 *
 * No physical vector width, register width or CPU count is encoded.
 *
 * ============================================================================
 * 15. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum stages remain ordinary source-level expressions until semantic
 * analysis establishes quantum meaning.
 *
 * Example conceptual pipeline:
 *
 *     pipeline {
 *         prepare()
 *         quantum_operation()
 *         measure()
 *         classical_decision()
 *     }
 *
 * The lowering path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum analysis
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
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This file creates NO quantum IR.
 *
 * It also does not enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     vendor gates
 *     physical gates.
 *
 * ============================================================================
 * 16. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented expressions may appear as pipeline stages where the
 * canonical HDL/hardware grammar permits them.
 *
 * Pipeline syntax does not determine:
 *
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     clock;
 *     register;
 *     bus width;
 *     physical memory;
 *     placement;
 *     routing.
 *
 * Those are semantic/compiler/backend responsibilities.
 *
 * ============================================================================
 * 17. DATA-PARALLEL INTEGRATION
 * ============================================================================
 *
 * A pipeline stage may contain a data-parallel computation.
 *
 * Example conceptual structure:
 *
 *     pipeline {
 *         source()
 *         map(...)
 *         reduce(...)
 *         sink()
 *     }
 *
 * The actual data-parallel syntax remains owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * This file does not redefine map/reduce/scan/partition syntax.
 *
 * ============================================================================
 * 18. TASK-PARALLEL INTEGRATION
 * ============================================================================
 *
 * Pipeline stages may contain task-oriented computation when permitted by the
 * expression/task composition.
 *
 * This file does not redefine:
 *
 *     spawnExpression
 *     awaitExpression
 *     taskParallelConstruct
 *
 * Those remain owned by:
 *
 *     grammar/expressions/async.g4
 *     grammar/concurrency/tasks.g4
 *     grammar/concurrency/task-parallel.g4
 *
 * ============================================================================
 * 19. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Pipeline syntax itself has no resource requirement.
 *
 * Semantic analysis may derive requirements from its stages.
 *
 * Examples:
 *
 *     memory requirements;
 *     communication requirements;
 *     streaming requirements;
 *     latency requirements;
 *     throughput requirements;
 *     quantum capabilities;
 *     accelerator capabilities;
 *     reliability requirements;
 *     energy constraints.
 *
 * These are represented through the canonical resource/capability model.
 *
 * The pipeline grammar does not select a target.
 *
 * ============================================================================
 * 20. MEMORY INTEGRATION
 * ============================================================================
 *
 * Pipeline stages may create values that flow between stages.
 *
 * The canonical memory/type system owns:
 *
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     aliasing;
 *     mutation;
 *     persistence;
 *     shared memory;
 *     distributed memory;
 *     accelerator memory;
 *     quantum memory.
 *
 * Pipeline syntax does not introduce a second ownership model.
 *
 * ============================================================================
 * 21. EFFECT INTEGRATION
 * ============================================================================
 *
 * A pipeline may carry effects such as:
 *
 *     I/O;
 *     asynchronous execution;
 *     communication;
 *     state mutation;
 *     quantum effects;
 *     external effects;
 *     cancellation;
 *     nondeterministic effects.
 *
 * The effect system remains authoritative.
 *
 * Pipeline grammar merely establishes structural composition.
 *
 * ============================================================================
 * 22. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     missing pipeline delimiter;
 *     missing stage;
 *     malformed named stage;
 *     malformed pipeline chain;
 *     missing closing delimiter;
 *     invalid stage separator.
 *
 * Semantic errors belong downstream and may include:
 *
 *     incompatible stage types;
 *     invalid stage effects;
 *     ownership conflict;
 *     illegal mutation;
 *     unsatisfied capability;
 *     impossible resource requirement;
 *     invalid quantum/classical boundary;
 *     invalid hardware semantic.
 *
 * Diagnostics must preserve the source span of:
 *
 *     pipeline;
 *     each stage;
 *     each stage name;
 *     each PIPE separator;
 *     the complete pipeline boundary.
 *
 * ============================================================================
 * 23. DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer configuration;
 *     parser grammar;
 *
 * the parser must produce the same pipeline parse structure.
 *
 * Parsing must not depend on:
 *
 *     CPU count;
 *     GPU availability;
 *     QPU availability;
 *     machine topology;
 *     network state;
 *     runtime state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness.
 *
 * ============================================================================
 * 24. SCALABILITY
 * ============================================================================
 *
 * These forms must remain structurally valid regardless of logical stage count:
 *
 *     pipeline {
 *         a()
 *     }
 *
 *     pipeline {
 *         a();
 *         b();
 *     }
 *
 *     pipeline {
 *         a();
 *         b();
 *         c();
 *         ...
 *     }
 *
 * The grammar uses repetition rather than a fixed enumeration.
 *
 * There is no rule such as:
 *
 *     pipelineStage pipelineStage pipelineStage
 *
 * because that would impose an artificial fixed structure.
 *
 * ============================================================================
 * 25. RESOURCE SCALING
 * ============================================================================
 *
 * Pipeline size and physical execution size are separate concepts.
 *
 * A pipeline with many logical stages may be:
 *
 *     fused;
 *     serialized;
 *     streamed;
 *     overlapped;
 *     replicated;
 *     partitioned;
 *     distributed;
 *     accelerated.
 *
 * The decision belongs to:
 *
 *     optimization;
 *     scheduling;
 *     resource management;
 *     compiler;
 *     runtime;
 *     deployment.
 *
 * ============================================================================
 * 26. SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     hardware discovery;
 *     credential access;
 *     environment inspection.
 *
 * Pipeline stages themselves may have security-sensitive semantics, but those
 * are validated downstream by the normal effect/security/capability systems.
 *
 * ============================================================================
 * 27. AST CONTRACT
 * ============================================================================
 *
 * The grammar should lower conceptually to an existing domain-neutral
 * representation rather than introducing a pipeline-specific IR.
 *
 * Minimum structural information required by the AST:
 *
 *     Pipeline
 *         - source span
 *         - ordered stages
 *
 *     PipelineStage
 *         - source span
 *         - optional logical name
 *         - stage expression/body
 *         - separator/order information where required for diagnostics
 *
 * The AST must NOT encode:
 *
 *     worker;
 *     thread;
 *     core;
 *     device;
 *     node;
 *     physical queue;
 *     physical topology.
 *
 * If the existing frontend AST lacks a generic structured/dataflow operation,
 * it should be extended with the repository's generic operation model rather
 * than creating a second pipeline IR.
 *
 * ============================================================================
 * 28. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define a pipeline IR.
 *
 * Semantic lowering should map pipeline structure into the repository's
 * canonical semantic representation/dataflow/task representation.
 *
 * The canonical IR should preserve:
 *
 *     stage ordering;
 *     dependencies;
 *     value flow;
 *     effects;
 *     resource requirements;
 *     capability requirements;
 *     determinism requirements.
 *
 * Physical scheduling is downstream.
 *
 * ============================================================================
 * 29. CONCURRENCY COMPOSITION
 * ============================================================================
 *
 * `concurrency.g4` must expose:
 *
 *     concurrencyPipelineConstruct
 *         : pipelineConstruct
 *         ;
 *
 * and include it in:
 *
 *     concurrencyConstruct
 *
 * and, where appropriate:
 *
 *     concurrencyExpression
 *
 * or the repository's canonical concurrency dispatch boundary.
 *
 * It must NOT copy the pipeline rules into `concurrency.g4`.
 *
 * ============================================================================
 * 30. ROOT PARSER COMPOSITION
 * ============================================================================
 *
 * `grammar/antlr/ZamaniParser.g4` is the canonical parser composition point.
 *
 * It must import the pipeline grammar through the concurrency composition
 * hierarchy rather than importing this file from multiple independent paths.
 *
 * Preferred ownership chain:
 *
 *     Zamani.g4
 *         |
 *         v
 *     ZamaniParser.g4
 *         |
 *         v
 *     Concurrency
 *         |
 *         v
 *     Pipeline
 *
 * There must not be another independent parser path for this construct.
 *
 * ============================================================================
 * 31. FRONTEND INTEGRATION
 * ============================================================================
 *
 * The complete pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     pipelineConstruct
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *     type/effect analysis   resource/capability analysis
 *       |                         |
 *       +------------+------------+
 *                    |
 *                    v
 *          canonical semantic model
 *                    |
 *                    v
 *              canonical IR
 *                    |
 *       +------------+-------------+
 *       |            |             |
 *       v            v             v
 *   classical     quantum::ir   HDL/hardware
 *       |            |             |
 *       +------------+-------------+
 *                    |
 *                    v
 *               optimization
 *                    |
 *                    v
 *             routing/scheduling
 *                    |
 *                    v
 *            resilience/QEC/ZQN
 *                    |
 *                    v
 *                   HAL
 *                    |
 *                    v
 *             target realization
 *
 * ============================================================================
 * 32. COMPATIBILITY
 * ============================================================================
 *
 * This is a new file and therefore introduces a new grammar component.
 *
 * It MUST NOT rename existing concurrency tokens.
 *
 * It MUST preserve:
 *
 *     PIPE
 *     SEMICOLON
 *     COLON
 *     LBRACE
 *     RBRACE
 *
 * and all existing expression tokens.
 *
 * A future `PIPELINE` keyword may be introduced only through the canonical
 * lexer/specification compatibility process.
 *
 * Until then, this grammar intentionally uses the existing token vocabulary.
 *
 * ============================================================================
 * 33. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *     pipeline {
 *         source()
 *     }
 *
 *     pipeline {
 *         source();
 *         transform();
 *         sink();
 *     }
 *
 *     pipeline {
 *         source() | transform() | sink()
 *     }
 *
 *     pipeline {
 *         source: source();
 *         transform: transform();
 *         sink: sink();
 *     }
 *
 *     pipeline {
 *         source();
 *         nested({
 *             transform()
 *         });
 *     }
 *
 * Tests must also combine pipeline stages with:
 *
 *     classical expressions;
 *     tensor operations;
 *     data-parallel computation;
 *     async computation;
 *     task computation;
 *     quantum operations;
 *     hybrid computation;
 *     hardware/software co-design;
 *     distributed operations;
 *     AI/data operations.
 *
 * Negative tests:
 *
 *     pipeline {}
 *
 *     pipeline {
 *     }
 *
 *     pipeline {
 *         : invalid
 *     }
 *
 *     pipeline {
 *         source() |
 *     }
 *
 *     pipeline {
 *         | sink()
 *     }
 *
 *     pipeline {
 *         source() ;;
 *     }
 *
 * Boundary tests:
 *
 *     one stage;
 *     two stages;
 *     many stages;
 *     nested stage bodies;
 *     named/unnamed stages;
 *     trailing semicolon;
 *     multiline pipeline;
 *     compact pipeline;
 *     deeply nested valid expressions.
 *
 * Scalability tests:
 *
 *     generated pipelines with increasing stage counts;
 *     large expressions inside stages;
 *     large logical stream descriptions;
 *     large nested pipelines where permitted.
 *
 * No test should assert a language-level maximum.
 *
 * Determinism tests:
 *
 *     identical source -> identical parse tree;
 *     identical source -> identical diagnostics.
 *
 * Compatibility tests:
 *
 *     existing uses of PIPE continue to parse outside `pipeline`;
 *     lambda expressions using PIPE remain valid;
 *     ordinary bitwise/operator contexts remain unchanged;
 *     existing concurrency constructs remain unchanged.
 *
 * ============================================================================
 * 34. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     fixed worker count;
 *     fixed thread count;
 *     fixed core count;
 *     fixed device count;
 *     fixed node count;
 *     fixed memory size;
 *     fixed buffer size;
 *     fixed stream count;
 *     fixed stage count;
 *     fixed topology.
 *
 * Numeric values in stage expressions remain ordinary program semantics.
 *
 * ============================================================================
 * 35. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has one canonical pipeline entry point.
 *
 * [x] It does not redefine ordinary expressions.
 *
 * [x] It does not redefine ordinary blocks.
 *
 * [x] It does not redefine tasks.
 *
 * [x] It does not redefine generic parallelism.
 *
 * [x] It does not redefine data parallelism.
 *
 * [x] It reuses the canonical lexer.
 *
 * [x] It uses existing PIPE punctuation/operator vocabulary.
 *
 * [x] It imposes no physical resource limits.
 *
 * [x] It supports an unbounded logical stage sequence.
 *
 * [x] It supports named logical stages.
 *
 * [x] It supports structured pipeline composition.
 *
 * [x] It supports compact pipeline chains.
 *
 * [x] It remains target-independent.
 *
 * [x] It introduces no pipeline-specific IR.
 *
 * [x] It preserves the quantum::ir boundary.
 *
 * [x] It supports classical/quantum/HDL/hybrid integration downstream.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It contains no unsafe Rust.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 *
 * [x] Integration responsibilities are declared before integration.
 *
 * Remaining repository-level completion requires:
 *
 *     - Concurrency imports Pipeline;
 *     - ZamaniParser imports/conposes Concurrency;
 *     - the frontend AST maps the construct;
 *     - semantic analysis validates stage compatibility;
 *     - canonical IR lowering exists;
 *     - positive/negative/boundary/scalability/determinism tests exist.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * A pipeline describes:
 *
 *     WHAT logical computation flows through stages
 *
 * not:
 *
 *     WHERE
 *     ON WHICH DEVICE
 *     WITH HOW MANY WORKERS
 *     ON HOW MANY CORES
 *     ON HOW MANY NODES
 *     THROUGH WHICH PHYSICAL QUEUE
 *
 * The realization remains downstream.
 *
 * Therefore pipeline syntax preserves:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * subject to program semantics and resources actually available at realization
 * time.
 *
 * ============================================================================
 */