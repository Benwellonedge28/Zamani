/*
 * ============================================================================
 * Zamani Programming Language
 * Deterministic Parallelism Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/deterministic-parallelism.g4
 *
 * Status:
 *     Production parser component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL COMPOSITION BOUNDARY for deterministic
 * parallel computation.
 *
 * It does NOT reimplement:
 *
 *     parallel.g4
 *     data-parallel.g4
 *     task-parallel.g4
 *
 * Those files remain the syntax owners for their respective constructs.
 *
 * This file adds one orthogonal semantic modifier:
 *
 *     deterministic
 *
 * to an existing parallel computation.
 *
 * Canonical forms include:
 *
 *     deterministic parallel {
 *         first();
 *         second();
 *     }
 *
 *     deterministic parallel compute();
 *
 *     deterministic parallel for item in values {
 *         compute(item);
 *     }
 *
 *     deterministic vectorized parallel for item in values {
 *         compute(item);
 *     }
 *
 *     deterministic parallel::reduce(values, combine);
 *
 *     deterministic parallel group {
 *         spawn first();
 *         spawn second();
 *     }
 *
 * The exact semantic meaning of deterministic execution is established
 * downstream. The grammar preserves the user's intent; it does not implement
 * determinism.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the deterministic-parallel modifier;
 *     - the deterministic-parallel composition boundary;
 *     - deterministic composition of existing parallel domains;
 *     - stable parser entry points for deterministic parallel tooling;
 *     - syntactic preservation of deterministic intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic parallel syntax;
 *     - data-parallel syntax;
 *     - task-parallel syntax;
 *     - task creation;
 *     - await;
 *     - futures;
 *     - synchronization;
 *     - reductions;
 *     - map;
 *     - scan;
 *     - partition;
 *     - ordinary expressions;
 *     - ordinary statements;
 *     - block syntax;
 *     - expression precedence;
 *     - scheduling;
 *     - resource allocation;
 *     - worker allocation;
 *     - thread allocation;
 *     - CPU/GPU/FPGA/QPU selection;
 *     - topology;
 *     - placement;
 *     - routing;
 *     - runtime implementation;
 *     - compiler optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL/hardware IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Determinism is an orthogonal semantic property.
 *
 * It must not be confused with:
 *
 *     sequential execution
 *     single-thread execution
 *     single-core execution
 *     fixed worker count
 *     fixed scheduling algorithm
 *     fixed hardware
 *
 * A deterministic computation may execute:
 *
 *     sequentially;
 *     concurrently;
 *     using SIMD;
 *     on GPUs;
 *     on FPGAs;
 *     on accelerators;
 *     across distributed resources;
 *     as hybrid classical/quantum computation;
 *     on future computational substrates.
 *
 * Determinism describes the required observable semantics, not the physical
 * execution mechanism.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The deterministic modifier MUST NOT impose any physical execution model.
 *
 * It MUST NOT imply:
 *
 *     one worker;
 *     one thread;
 *     one core;
 *     one CPU;
 *     one GPU;
 *     one FPGA;
 *     one QPU;
 *     one node;
 *     one execution queue.
 *
 * It also MUST NOT encode:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_PARALLELISM
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_DIMENSION
 *     MAX_MEMORY
 *
 * There is no language-level upper bound on:
 *
 *     parallel regions;
 *     task count;
 *     data elements;
 *     partitions;
 *     reduction operands;
 *     nesting;
 *     distributed participants;
 *     logical execution contexts.
 *
 * Actual execution remains constrained by:
 *
 *     program semantics;
 *     representational limits;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     explicitly declared requirements.
 *
 * ============================================================================
 * DETERMINISM SEMANTICS
 * ============================================================================
 *
 * The semantic contract is:
 *
 *     deterministic X
 *
 * means that observable behavior required by the language's determinism model
 * must not be changed merely because X is parallelized or physically realized
 * differently.
 *
 * Determinism may concern:
 *
 *     - observable result values;
 *     - ordering where ordering is semantically observable;
 *     - reduction semantics;
 *     - side-effect ordering;
 *     - externally visible events;
 *     - reproducibility;
 *     - dependency ordering;
 *     - canonical serialization;
 *     - provenance;
 *     - deterministic failure behavior where specified.
 *
 * The grammar does NOT decide which of these properties is required for a
 * particular construct.
 *
 * Semantic analysis owns that decision according to:
 *
 *     language specification;
 *     operation semantics;
 *     effects;
 *     types;
 *     capabilities;
 *     resource contracts;
 *     determinism policy.
 *
 * ============================================================================
 * IMPORTANT: DETERMINISTIC != ORDERED
 * ============================================================================
 *
 * Deterministic execution does not automatically mean that every operation
 * must execute in source order.
 *
 * For example:
 *
 *     deterministic parallel {
 *         independent_a();
 *         independent_b();
 *     }
 *
 * may execute both operations concurrently if the semantic model guarantees
 * that all observable behavior remains deterministic.
 *
 * Likewise, a deterministic reduction may use a legal parallel reduction
 * strategy when the reduction operation's semantic contract permits it.
 *
 * The scheduler, optimizer and reduction semantics determine the valid
 * implementation strategy.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The existing concurrency vocabulary is preserved.
 *
 * Existing tokens consumed through imported grammars include:
 *
 *     PARALLEL
 *     FOR
 *     IN
 *     WHEN
 *     VECTORIZED
 *     GROUP
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *     COMMA
 *
 * This file requires one new canonical lexical token:
 *
 *     DETERMINISTIC
 *
 * Canonical spelling:
 *
 *     deterministic
 *
 * The token MUST be added to the canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and to the corresponding Rust lexer keyword mapping:
 *
 *     src/lexer.rs
 *
 * if that implementation maintains an explicit keyword registry.
 *
 * No other lexer token is required.
 *
 * Do NOT add:
 *
 *     DETERMINISTIC_PARALLEL
 *     DETERMINISTIC_TASK
 *     DETERMINISTIC_DATA_PARALLEL
 *     DETERMINISTIC_REDUCE
 *     ORDERED_PARALLEL
 *     REPRODUCIBLE_PARALLEL
 *
 * The single modifier is deliberately compositional.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Generic parallel syntax is owned by:
 *
 *     grammar/concurrency/parallel.g4
 *
 * Data-parallel syntax is owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * Task-parallel syntax is owned by:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * These grammars are imported rather than duplicated.
 *
 * ============================================================================
 */

parser grammar DeterministicParallelism;

options {
    tokenVocab = ZamaniLexer;
}

import
    Parallel,
    DataParallel,
    TaskParallel
;


/*
 * ============================================================================
 * 1. PUBLIC DETERMINISTIC-PARALLEL ENTRY POINT
 * ============================================================================
 *
 * This is the primary public parser rule owned by this file.
 *
 * Every deterministic parallel construct begins with:
 *
 *     deterministic
 *
 * followed by one of the existing parallel-domain constructs.
 *
 * This makes the deterministic modifier structurally unambiguous with the
 * existing:
 *
 *     parallel ...
 *
 * syntax.
 */
deterministicParallelConstruct
    : deterministicParallelExpression
    ;


/*
 * ============================================================================
 * 2. DETERMINISTIC PARALLEL EXPRESSION
 * ============================================================================
 *
 * The deterministic modifier precedes an existing parallel computation.
 *
 * No new parallel-body syntax is introduced here.
 */
deterministicParallelExpression
    : DETERMINISTIC
      deterministicParallelBody
    ;


/*
 * ============================================================================
 * 3. DETERMINISTIC PARALLEL BODY
 * ============================================================================
 *
 * Existing domain grammars remain the owners of their concrete syntax.
 *
 * Generic parallel:
 *
 *     deterministic parallel { ... }
 *     deterministic parallel compute()
 *
 * Data parallel:
 *
 *     deterministic parallel for item in values { ... }
 *     deterministic vectorized parallel for item in values { ... }
 *     deterministic parallel::reduce(values, combine)
 *
 * Task parallel:
 *
 *     deterministic parallel group {
 *         spawn a();
 *         spawn b();
 *     }
 *
 * No domain-specific production is duplicated here.
 */
deterministicParallelBody
    : parallelConstruct
    | dataParallelConstruct
    | taskParallelConstruct
    ;


/*
 * ============================================================================
 * 4. GENERIC DETERMINISTIC PARALLEL
 * ============================================================================
 *
 * Stable semantic/tooling adapter.
 *
 * This rule does not define another syntax family.
 *
 * It simply classifies the generic `parallel` family after the deterministic
 * modifier has been consumed.
 */
deterministicGenericParallel
    : DETERMINISTIC
      parallelConstruct
    ;


/*
 * ============================================================================
 * 5. DETERMINISTIC DATA PARALLEL
 * ============================================================================
 *
 * Data-parallel concrete syntax remains entirely owned by DataParallel.
 *
 * This adapter exists so semantic tooling can distinguish:
 *
 *     deterministic data parallelism
 *
 * without inspecting the raw parse tree for the modifier.
 */
deterministicDataParallel
    : DETERMINISTIC
      dataParallelConstruct
    ;


/*
 * ============================================================================
 * 6. DETERMINISTIC TASK PARALLEL
 * ============================================================================
 *
 * Task-parallel syntax remains entirely owned by TaskParallel.
 */
deterministicTaskParallel
    : DETERMINISTIC
      taskParallelConstruct
    ;


/*
 * ============================================================================
 * 7. STABLE COMPATIBILITY ALIAS
 * ============================================================================
 *
 * Tooling may use this name when it needs a deterministic-parallel domain
 * boundary without caring which specialized parallel family was selected.
 *
 * This is a pure adapter and introduces no syntax.
 */
deterministicParallel
    : deterministicParallelConstruct
    ;


/*
 * ============================================================================
 * 8. DETERMINISTIC REGION
 * ============================================================================
 *
 * Stable semantic classification boundary.
 *
 * This does not introduce another region syntax.
 */
deterministicParallelRegion
    : deterministicParallelConstruct
    ;


/*
 * ============================================================================
 * 9. DETERMINISTIC NESTING
 * ============================================================================
 *
 * No special recursion is required here.
 *
 * Existing block/statement composition can encounter another:
 *
 *     deterministicParallelConstruct
 *
 * inside the body of a parallel computation.
 *
 * Therefore nested deterministic parallelism remains possible without a
 * hard-coded nesting limit.
 */
deterministicParallelNested
    : deterministicParallelConstruct
    ;


/*
 * ============================================================================
 * 10. DETERMINISTIC COMPOSITION
 * ============================================================================
 *
 * Stable adapter for semantic tooling that needs to treat all deterministic
 * parallel domains uniformly.
 */
deterministicParallelComposition
    : deterministicParallelBody
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define Rust AST structures.
 *
 * The parser/frontend must preserve:
 *
 *     modifier = deterministic
 *     underlying construct
 *     source span
 *     nested source structure
 *     source ordering
 *
 * Recommended conceptual representation:
 *
 *     Operation / ConcurrencyConstruct
 *         modifiers
 *             deterministic
 *         body
 *             existing parallel construct
 *
 * The exact AST type belongs to:
 *
 *     src/frontend/ast/
 *
 * The preferred architecture remains domain-neutral.
 *
 * Do NOT create:
 *
 *     DeterministicParallelIR
 *     DeterministicTaskIR
 *     DeterministicQuantumParallelIR
 *
 * merely because this grammar introduces a deterministic modifier.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the enclosed operation is actually deterministic;
 *     - which observable properties are covered by determinism;
 *     - whether effects permit deterministic parallel execution;
 *     - whether memory accesses preserve deterministic semantics;
 *     - whether aliases create observable races;
 *     - whether reductions are deterministic;
 *     - whether communication ordering is deterministic;
 *     - whether random/stateful operations require explicit reproducibility;
 *     - whether quantum measurement semantics permit the requested guarantee;
 *     - whether distributed execution preserves the required semantics;
 *     - whether the target can satisfy the requested guarantee.
 *
 * Parsing MUST NOT perform these checks.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Deterministic parallelism interacts with effects.
 *
 * Examples of effects requiring semantic analysis include:
 *
 *     mutation
 *     I/O
 *     external communication
 *     randomness
 *     timing
 *     synchronization
 *     distributed communication
 *     quantum measurement
 *     hardware interaction
 *
 * The grammar does not attempt to classify those effects.
 *
 * ============================================================================
 * DATA-PARALLEL / REDUCTION INTEGRATION
 * ============================================================================
 *
 * `data-parallel.g4` remains the sole owner of:
 *
 *     parallel::map
 *     parallel::reduce
 *     parallel::scan
 *     parallel::partition
 *     parallel for
 *     vectorized parallel for
 *
 * Therefore this file MUST NOT define:
 *
 *     deterministicReduce
 *     deterministicMap
 *     deterministicScan
 *     deterministicPartition
 *
 * as independent operation grammars.
 *
 * The deterministic modifier is applied to the existing operation:
 *
 *     deterministic parallel::reduce(values, combine)
 *
 * Semantic analysis determines whether:
 *
 *     - the reducer is associative;
 *     - the reducer is commutative;
 *     - ordering is observable;
 *     - an identity exists;
 *     - a deterministic tree/reduction strategy is required;
 *     - numerical reproducibility requirements apply.
 *
 * ============================================================================
 * TASK-PARALLEL INTEGRATION
 * ============================================================================
 *
 * `task-parallel.g4` remains the owner of:
 *
 *     parallel group {
 *         ...
 *     }
 *
 * Therefore:
 *
 *     deterministic parallel group {
 *         spawn a();
 *         spawn b();
 *     }
 *
 * is represented by:
 *
 *     DETERMINISTIC
 *         +
 *     taskParallelConstruct
 *
 * rather than a duplicated deterministic task-group grammar.
 *
 * Task dependencies remain expressed by the existing task/await grammar.
 *
 * No DEPENDS token is introduced.
 *
 * ============================================================================
 * GENERIC PARALLEL INTEGRATION
 * ============================================================================
 *
 * `parallel.g4` remains the owner of:
 *
 *     parallel { ... }
 *     parallel expression
 *     parallel(callable)
 *
 * Therefore:
 *
 *     deterministic parallel { ... }
 *
 * is simply:
 *
 *     DETERMINISTIC
 *         +
 *     parallelConstruct
 *
 * The generic parallel grammar remains unchanged.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A deterministic parallel body may contain quantum computation where the
 * enclosing semantic context permits it.
 *
 * Example:
 *
 *     deterministic parallel {
 *         quantum_work();
 *     }
 *
 * This grammar does NOT define:
 *
 *     quantum gates;
 *     qubits;
 *     physical qubits;
 *     circuits;
 *     QEC;
 *     noise;
 *     calibration;
 *     routing;
 *     scheduling.
 *
 * The canonical quantum path remains:
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
 *       +--> optimization
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC
 *       +--> resilience
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target realization
 *
 * No second quantum IR is created.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Deterministic parallelism may surround:
 *
 *     arithmetic;
 *     numerical computation;
 *     symbolic computation;
 *     tensor operations;
 *     signal processing;
 *     scientific computation;
 *     AI/ML operations;
 *     data processing.
 *
 * Whether those computations are deterministic is a semantic property.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A deterministic parallel construct may contain hardware/software co-design
 * intent.
 *
 * The grammar does not select:
 *
 *     FPGA;
 *     ASIC;
 *     GPU;
 *     accelerator;
 *     clock;
 *     pipeline width;
 *     physical resource;
 *     hardware address.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Deterministic parallelism may eventually be distributed across:
 *
 *     one execution context;
 *     multiple execution contexts;
 *     one machine;
 *     multiple machines;
 *     clusters;
 *     cloud resources;
 *     heterogeneous resources;
 *     future distributed substrates.
 *
 * The grammar contains no node count or topology.
 *
 * Distributed determinism belongs to:
 *
 *     communication semantics;
 *     consistency semantics;
 *     ordering semantics;
 *     scheduling;
 *     provenance;
 *     runtime;
 *
 * not this grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Determinism is a semantic requirement, not a hardware resource.
 *
 * The compiler may need to determine whether the target provides capabilities
 * necessary to preserve deterministic semantics.
 *
 * Examples:
 *
 *     deterministic communication
 *     deterministic reduction
 *     reproducible execution
 *     stable ordering
 *     deterministic scheduling support
 *
 * Such capabilities must be represented by the existing resource/capability
 * system rather than by hardware-specific grammar rules.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * The scheduler is responsible for choosing a valid physical execution order.
 *
 * A deterministic program does NOT require one fixed scheduler.
 *
 * Possible implementations include:
 *
 *     static scheduling;
 *     dependency scheduling;
 *     work stealing with deterministic coordination;
 *     distributed deterministic scheduling;
 *     accelerator scheduling;
 *     quantum scheduling;
 *     serialized execution.
 *
 * The chosen implementation must preserve the semantic guarantee.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file introduces NO IR.
 *
 * The deterministic modifier becomes semantic metadata/intent in the existing
 * canonical representation.
 *
 * There must not be a:
 *
 *     DeterministicParallelIR
 *
 * created by this grammar.
 *
 * The canonical downstream representation owns determinism according to the
 * repository's existing semantic/IR architecture.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * Classical computation continues through the existing classical semantic/IR
 * path.
 *
 * HDL/hardware computation continues through the existing HDL/hardware path.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler consumes deterministic semantic information for:
 *
 *     dependency analysis;
 *     effect analysis;
 *     alias analysis;
 *     reduction legality;
 *     optimization legality;
 *     scheduling constraints;
 *     reproducibility;
 *     target capability checking;
 *     resource feasibility.
 *
 * The compiler may transform execution strategy only when semantic
 * determinism is preserved.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime may use any safe realization that satisfies the semantic
 * contract.
 *
 * It may:
 *
 *     serialize;
 *     parallelize;
 *     distribute;
 *     batch;
 *     pipeline;
 *     vectorize;
 *     use accelerators;
 *     use heterogeneous execution.
 *
 * The runtime MUST NOT silently weaken an explicitly required deterministic
 * semantic guarantee.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Examples:
 *
 *     deterministic
 *     deterministic parallel
 *     deterministic parallel {
 *     deterministic parallel group
 *
 * are syntactically incomplete.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     nondeterministic operation inside deterministic region;
 *     unordered externally visible effects;
 *     non-reproducible random source;
 *     nondeterministic reduction;
 *     unsupported deterministic communication;
 *
 * must be semantic diagnostics, not parser alternatives.
 *
 * Diagnostics should preserve:
 *
 *     source span;
 *     deterministic modifier span;
 *     underlying construct span;
 *     relevant operand spans.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing itself is deterministic.
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     language version;
 *
 * the parser must produce equivalent parse structures.
 *
 * Runtime determinism is a separate semantic guarantee.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     performs no I/O;
 *     performs no network access;
 *     performs no filesystem access;
 *     performs no hardware discovery;
 *     executes no user code;
 *     contains no target-language actions;
 *     contains no semantic predicates requiring runtime state;
 *     contains no unsafe Rust.
 *
 * The generated parser is an input-processing component and must therefore be
 * exercised against malformed, deeply nested and adversarial source inputs by
 * the parser/validation test suite.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     hardware IDs;
 *     physical resource IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     node IDs;
 *     worker IDs;
 *     thread IDs;
 *     fixed vector widths;
 *     fixed tensor dimensions;
 *     fixed memory sizes;
 *     topology;
 *     placement;
 *     machine-specific limits.
 *
 * The only finite lexical concept introduced is the keyword:
 *
 *     deterministic
 *
 * That is language vocabulary, not a hardware limit.
 *
 * ============================================================================
 * REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 * Generic parallel:
 *
 *     deterministic parallel {}
 *
 *     deterministic parallel {
 *         first();
 *         second();
 *     }
 *
 *     deterministic parallel compute();
 *
 *     deterministic parallel(callable);
 *
 * Data parallel:
 *
 *     deterministic parallel for item in values {
 *         compute(item);
 *     }
 *
 *     deterministic parallel for item in values
 *         when predicate(item) {
 *         compute(item);
 *     }
 *
 *     deterministic vectorized parallel for item in values {
 *         compute(item);
 *     }
 *
 *     deterministic parallel::map(values, compute);
 *
 *     deterministic parallel::reduce(values, combine);
 *
 *     deterministic parallel::scan(values, combine);
 *
 *     deterministic parallel::partition(values, partitioner);
 *
 * Task parallel:
 *
 *     deterministic parallel group {
 *         spawn first();
 *         spawn second();
 *     }
 *
 *     deterministic parallel group {
 *         spawn first();
 *         spawn second();
 *         await first_result;
 *     }
 *
 * Nested:
 *
 *     deterministic parallel {
 *         deterministic parallel {
 *             compute();
 *         }
 *     }
 *
 * Cross-domain:
 *
 *     deterministic parallel {
 *         classical_work();
 *     }
 *
 *     deterministic parallel {
 *         quantum_work();
 *     }
 *
 *     deterministic parallel {
 *         accelerator_work();
 *     }
 *
 * ============================================================================
 * REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 *     deterministic
 *
 *     deterministic parallel
 *
 *     deterministic parallel group
 *
 *     deterministic parallel for
 *
 *     deterministic parallel for item
 *
 *     deterministic parallel for item in
 *
 *     deterministic parallel::reduce
 *
 *     deterministic parallel::reduce()
 *
 *     deterministic parallel (
 *
 *     deterministic parallel [
 *
 *     deterministic parallel group (
 *
 *     deterministic vectorized
 *
 * These must fail structurally rather than being silently reinterpreted as
 * another construct.
 *
 * ============================================================================
 * REQUIRED BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     empty deterministic parallel region;
 *     one logical operation;
 *     many logical operations;
 *     nested deterministic regions;
 *     deterministic region containing ordinary parallel regions;
 *     deterministic data-parallel region;
 *     deterministic task-parallel region;
 *     dynamically sized collections;
 *     symbolic collection sizes;
 *     large expressions;
 *     large nested programs.
 *
 * No boundary test may define a language-level maximum.
 *
 * ============================================================================
 * REQUIRED SCALABILITY TESTS
 * ============================================================================
 *
 * The test suite must vary logical workload size without changing grammar
 * semantics.
 *
 * Required dimensions:
 *
 *     logical task count;
 *     data-element count;
 *     reduction size;
 *     nesting depth;
 *     number of independent parallel regions;
 *     distributed participant count;
 *     quantum logical operation count;
 *     tensor/data dimensions.
 *
 * The tests must NOT establish:
 *
 *     maximum workers;
 *     maximum threads;
 *     maximum cores;
 *     maximum GPUs;
 *     maximum QPUs;
 *     maximum nodes.
 *
 * ============================================================================
 * REQUIRED DETERMINISM TESTS
 * ============================================================================
 *
 * 1. Same source -> equivalent parse structure.
 *
 * 2. Same source with different available resource counts -> same semantic
 *    determinism requirement.
 *
 * 3. Generic parallel and deterministic parallel remain distinguishable in the
 *    AST/semantic metadata.
 *
 * 4. Deterministic data-parallel reduction preserves its deterministic
 *    requirement downstream.
 *
 * 5. Deterministic task-parallel groups preserve dependency structure.
 *
 * 6. Nested deterministic constructs preserve nesting.
 *
 * 7. Parser diagnostics for malformed deterministic constructs are stable.
 *
 * ============================================================================
 * REQUIRED COMPATIBILITY TESTS
 * ============================================================================
 *
 * Existing syntax must continue to parse:
 *
 *     parallel {}
 *
 *     parallel for item in values {
 *         compute(item);
 *     }
 *
 *     parallel::reduce(values, combine);
 *
 *     parallel group {
 *         spawn work();
 *     }
 *
 * The addition of deterministic parallelism must not change the meaning of
 * existing non-deterministic source forms.
 *
 * ============================================================================
 * INTEGRATION WITH concurrency.g4
 * ============================================================================
 *
 * `grammar/concurrency/concurrency.g4` is the concurrency composition root.
 *
 * It should import this grammar as:
 *
 *     DeterministicParallelism
 *
 * alongside its existing concurrency-domain components.
 *
 * IMPORTANT:
 *
 * Do NOT add:
 *
 *     | deterministicParallelConstruct
 *
 * alongside:
 *
 *     | concurrencyParallelConstruct
 *     | concurrencyDataParallelConstruct
 *     | concurrencyTaskParallelConstruct
 *
 * if those alternatives are already reachable from the same parser context.
 *
 * That would create competing parse paths.
 *
 * Instead, the canonical composition should expose deterministic parallelism
 * through one dedicated semantic/domain entry point at the appropriate parser
 * dispatch location.
 *
 * If the root parser already dispatches all concurrency constructs through
 * `concurrencyConstruct`, the deterministic alternative should be integrated
 * there exactly once.
 *
 * ============================================================================
 * INTEGRATION WITH parallel.g4
 * ============================================================================
 *
 * NO modification to parallel.g4 is required for its existing syntax.
 *
 * It remains the owner of:
 *
 *     parallelConstruct
 *     parallelExpression
 *     parallelBody
 *     parallelStatement
 *
 * DeterministicParallelism consumes `parallelConstruct`.
 *
 * ============================================================================
 * INTEGRATION WITH data-parallel.g4
 * ============================================================================
 *
 * NO syntax duplication is permitted.
 *
 * DataParallel remains the owner of:
 *
 *     dataParallelConstruct
 *     dataParallelIteration
 *     dataParallelMap
 *     dataParallelReduce
 *     dataParallelScan
 *     dataParallelPartition
 *
 * DeterministicParallelism merely prefixes those constructs with:
 *
 *     DETERMINISTIC
 *
 * ============================================================================
 * INTEGRATION WITH task-parallel.g4
 * ============================================================================
 *
 * TaskParallel remains the owner of:
 *
 *     taskParallelConstruct
 *     taskParallelGroup
 *     taskParallelItem
 *     taskParallelTask
 *     taskParallelDependency
 *     taskParallelNestedGroup
 *
 * DeterministicParallelism does not redefine any of them.
 *
 * ============================================================================
 * INTEGRATION WITH tasks.g4
 * ============================================================================
 *
 * This file does not redefine:
 *
 *     spawn;
 *     await;
 *     task creation;
 *     task handles;
 *     task scopes.
 *
 * Those remain owned by Tasks.
 *
 * ============================================================================
 * INTEGRATION WITH reduction.g4
 * ============================================================================
 *
 * Reduction syntax remains owned by:
 *
 *     data-parallel.g4
 *
 * and any reduction-specific semantic adapters remain owned by:
 *
 *     reduction.g4
 *
 * DeterministicParallelism must not create a competing reduction grammar.
 *
 * The semantic pipeline becomes:
 *
 *     deterministic
 *          +
 *     dataParallelReduce
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     reduction semantics
 *          +
 *     determinism semantics
 *          |
 *          v
 *     canonical IR
 *
 * ============================================================================
 * INTEGRATION WITH AST
 * ============================================================================
 *
 * The frontend should preserve deterministic intent as metadata/modifier state
 * on the existing concurrency/operation representation.
 *
 * Preferred conceptual model:
 *
 *     Operation
 *         modifiers = {
 *             deterministic
 *         }
 *
 * rather than:
 *
 *     DeterministicParallelOperation
 *
 * unless the existing AST architecture independently requires a specialized
 * node for semantic reasons.
 *
 * ============================================================================
 * INTEGRATION WITH CANONICAL IR
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Deterministic intent must reach the canonical semantic/IR layer through the
 * existing frontend lowering pipeline.
 *
 * Quantum constructs continue through:
 *
 *     quantum::ir
 *
 * Classical constructs continue through the existing classical representation.
 *
 * Hardware/HDL constructs continue through their canonical representation.
 *
 * ============================================================================
 * INTEGRATION WITH RUST
 * ============================================================================
 *
 * This grammar is target-language independent.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * No grammar action may require:
 *
 *     unsafe
 *     unsafe blocks
 *     unsafe traits
 *     target-specific runtime hooks.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Existing concurrency grammar ownership is preserved.
 *
 * [x] Generic parallel syntax is delegated to Parallel.
 *
 * [x] Data-parallel syntax is delegated to DataParallel.
 *
 * [x] Task-parallel syntax is delegated to TaskParallel.
 *
 * [x] No reduction grammar is duplicated.
 *
 * [x] No task/spawn/await grammar is duplicated.
 *
 * [x] No expression grammar is duplicated.
 *
 * [x] No block grammar is duplicated.
 *
 * [x] No scheduler is encoded.
 *
 * [x] No resource allocator is encoded.
 *
 * [x] No hardware topology is encoded.
 *
 * [x] No fixed execution-resource count exists.
 *
 * [x] No fixed data/vector/tensor size exists.
 *
 * [x] No quantum gate list exists.
 *
 * [x] No second quantum IR exists.
 *
 * [x] Deterministic intent has a single syntactic modifier.
 *
 * [x] Generic/data/task parallelism can all carry the modifier.
 *
 * [x] Nested deterministic parallelism is structurally possible.
 *
 * [x] Dynamic workload size is unrestricted by grammar constants.
 *
 * [x] Parser determinism is explicitly specified.
 *
 * [x] Semantic determinism is explicitly downstream.
 *
 * [x] AST integration is predefined.
 *
 * [x] IR integration is predefined.
 *
 * [x] Compiler integration is predefined.
 *
 * [x] Runtime integration is predefined.
 *
 * [x] Quantum integration is predefined.
 *
 * [x] HDL/hardware integration is predefined.
 *
 * [x] Distributed integration is predefined.
 *
 * [x] Positive tests are specified.
 *
 * [x] Negative tests are specified.
 *
 * [x] Boundary tests are specified.
 *
 * [x] Scalability tests are specified.
 *
 * [x] Determinism tests are specified.
 *
 * [x] Compatibility tests are specified.
 *
 * [x] Safe Rust requirement is explicit.
 *
 * [x] No Rust actions are used.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file expresses:
 *
 *     DETERMINISTIC PARALLEL INTENT
 *
 * It does NOT express:
 *
 *     HOW MANY RESOURCES
 *     WHICH RESOURCES
 *     WHERE RESOURCES ARE
 *     WHICH HARDWARE IS USED
 *     HOW TASKS ARE SCHEDULED
 *     HOW DATA IS PARTITIONED PHYSICALLY
 *     HOW QUANTUM OPERATIONS ARE ROUTED
 *     HOW QEC IS IMPLEMENTED
 *     HOW ZQN IS REALIZED
 *     HOW HDL IS SYNTHESIZED
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     deterministic parallel syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic determinism analysis
 *       |
 *       +--> effects
 *       +--> ownership
 *       +--> dependencies
 *       +--> resources
 *       +--> capabilities
 *       +--> reproducibility
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL/hardware
 *       +--> distributed
 *       +--> accelerator
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     actual target
 *
 * This is the required deterministic-parallel contribution to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */