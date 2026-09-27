/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/lifecycle.g4
 *
 * Grammar:
 *     Lifecycle
 *
 * Status:
 *     Production execution-lifecycle grammar component
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL LIFECYCLE INTENT.
 *
 * A lifecycle describes the semantic progression and permitted transitions of
 * an executable Zamani entity without prescribing how those transitions are
 * physically implemented.
 *
 * It supports lifecycle intent for:
 *
 *     - programs;
 *     - computations;
 *     - functions;
 *     - tasks;
 *     - jobs;
 *     - services;
 *     - agents;
 *     - pipelines;
 *     - classical computations;
 *     - quantum computations;
 *     - hybrid computations;
 *     - HDL/hardware computations;
 *     - accelerator workloads;
 *     - distributed workloads;
 *     - AI workloads;
 *     - future execution domains.
 *
 * The grammar describes WHAT lifecycle semantics are requested.
 *
 * It does NOT describe HOW the runtime implements those semantics.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
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
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> lifecycle validation
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     runtime lifecycle realization
 *
 * Lifecycle grammar MUST NOT bypass this pipeline.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - lifecycle declaration syntax;
 *     - lifecycle target syntax;
 *     - lifecycle body structure;
 *     - lifecycle properties;
 *     - lifecycle state references;
 *     - lifecycle transition syntax;
 *     - lifecycle transition conditions;
 *     - lifecycle transition metadata;
 *     - lifecycle event hooks;
 *     - lifecycle nested policy blocks;
 *     - lifecycle extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - declarations outside lifecycle intent;
 *     - task semantics;
 *     - process semantics;
 *     - scheduling algorithms;
 *     - placement algorithms;
 *     - dispatch algorithms;
 *     - recovery algorithms;
 *     - resilience algorithms;
 *     - checkpoint implementation;
 *     - tracing;
 *     - profiling;
 *     - observability implementation;
 *     - deployment implementation;
 *     - hardware discovery;
 *     - resource allocation;
 *     - quantum operation semantics;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - vendor APIs;
 *     - runtime implementation;
 *     - lifecycle execution.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It imports only foundational parser grammars:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * It does NOT import:
 *
 *     Execution
 *     Runtime
 *     Scheduling
 *     Placement
 *     Dispatch
 *     Recovery
 *     Resilience
 *     Checkpointing
 *     Observability
 *     Tracing
 *     Profiling
 *
 * Those grammars may consume this grammar's public entry points.
 *
 * This dependency direction prevents:
 *
 *     Lifecycle -> Execution -> Lifecycle
 *
 * and similar circular dependencies.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The current canonical keyword vocabulary does not need to reserve every
 * lifecycle-specific word.
 *
 * Therefore this grammar deliberately does NOT invent a new LIFECYCLE lexer
 * token.
 *
 * The lifecycle declaration introducer is represented as an identifier and
 * MUST be validated semantically to have the canonical spelling:
 *
 *     lifecycle
 *
 * This keeps this file independently compatible with the existing lexer and
 * avoids silently creating a second lexical authority.
 *
 * If the language specification later promotes `lifecycle` to a reserved
 * keyword, the lexer may introduce LIFECYCLE as a compatibility-preserving
 * lexical refinement. This parser's semantic model need not change.
 *
 * Likewise, lifecycle states, event names, policy names, transition labels,
 * and future lifecycle concepts remain semantic names rather than a closed
 * lexer enumeration.
 *
 * ============================================================================
 * OPEN-ENDED VOCABULARY
 * ============================================================================
 *
 * DO NOT create lexer keywords for every lifecycle state.
 *
 * For example, the grammar MUST NOT enumerate:
 *
 *     CREATED
 *     INITIALIZING
 *     RUNNING
 *     PAUSED
 *     COMPLETED
 *     FAILED
 *     TERMINATED
 *
 * as the only legal states.
 *
 * These are valid semantic values:
 *
 *     created
 *     running
 *     paused
 *     completed
 *     failed
 *
 * but future programs may define:
 *
 *     waiting_for_calibration
 *     thermal_degraded
 *     quantum_recovery
 *     replicated
 *     checkpoint_pending
 *     vendor_defined_state
 *     future::state
 *
 * without changing the parser.
 *
 * Semantic analysis determines which state names have valid meaning for the
 * active lifecycle profile.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Lifecycle syntax is target independent.
 *
 * It MUST NOT encode:
 *
 *     CPU identity
 *     CPU count
 *     core identity
 *     core count
 *     thread count
 *     GPU identity
 *     GPU count
 *     FPGA identity
 *     FPGA count
 *     ASIC identity
 *     QPU identity
 *     QPU count
 *     physical qubit identity
 *     physical memory bank
 *     physical address
 *     network node identity
 *     fixed topology
 *     fixed queue capacity
 *     fixed device count
 *
 * The lifecycle describes semantic execution state, not physical realization.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * The grammar MUST NOT define:
 *
 *     MAX_LIFECYCLE_STATES
 *     MAX_TRANSITIONS
 *     MAX_EVENTS
 *     MAX_HOOKS
 *     MAX_RETRIES
 *     MAX_RECOVERY_BRANCHES
 *     MAX_TASKS
 *     MAX_JOBS
 *     MAX_PROCESSES
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * Repetition is structurally represented by `*`.
 *
 * Any concrete value appearing in source is program semantics, not a compiler
 * capacity.
 *
 * For example:
 *
 *     retry_limit: 3;
 *
 * is a valid program property.
 *
 * It does NOT establish:
 *
 *     MAX_RETRIES = 3
 *
 * for Zamani.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / HINT SEPARATION
 * ============================================================================
 *
 * Lifecycle properties may express:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *
 * These concepts MUST remain semantically distinct.
 *
 * Example:
 *
 *     requires: capability("quantum.measurement");
 *
 * is a capability requirement.
 *
 *     prefer: accelerator("quantum");
 *
 * is a preference.
 *
 *     hint: recovery.strategy;
 *
 * is an implementation hint.
 *
 * None of these selects a physical device.
 *
 * ============================================================================
 * STATE MODEL
 * ============================================================================
 *
 * Lifecycle states are semantic names.
 *
 * A lifecycle may therefore contain any number of states subject only to
 * implementation resources.
 *
 * Conceptual example:
 *
 *     lifecycle computation {
 *         initial: created;
 *
 *         created -> running;
 *         running -> completed;
 *         running -> failed;
 *         failed -> recovering;
 *         recovering -> running;
 *         completed -> retired;
 *
 *         terminal: completed;
 *         terminal: retired;
 *     }
 *
 * The grammar does not decide whether these states are legal.
 *
 * Semantic analysis validates:
 *
 *     - state existence;
 *     - duplicate state declarations;
 *     - initial-state rules;
 *     - terminal-state rules;
 *     - transition validity;
 *     - reachability;
 *     - cycles;
 *     - determinism;
 *     - effect compatibility;
 *     - resource compatibility;
 *     - capability compatibility.
 *
 * ============================================================================
 * TRANSITIONS
 * ============================================================================
 *
 * A transition has the generic form:
 *
 *     from_state -> to_state;
 *
 * It may optionally carry:
 *
 *     when <expression>
 *
 * and/or:
 *
 *     with { ... }
 *
 * Example:
 *
 *     running -> recovering
 *         when fault_detected
 *         with {
 *             requires: capability("recovery");
 *         };
 *
 * The grammar does not decide whether the transition can actually occur.
 *
 * ============================================================================
 * EVENTS
 * ============================================================================
 *
 * Lifecycle events are semantic expressions rather than a closed event list.
 *
 * Examples:
 *
 *     on: cancellation;
 *     on: timeout;
 *     on: resource_exhaustion;
 *     on: measurement_failure;
 *     on: hardware_degradation;
 *     on: external_signal;
 *
 * Future event classes can be introduced without modifying this grammar.
 *
 * ============================================================================
 * HOOKS
 * ============================================================================
 *
 * Named lifecycle blocks support semantic hooks such as:
 *
 *     entry { ... }
 *     exit { ... }
 *     on_failure { ... }
 *     on_recovery { ... }
 *     before_start { ... }
 *     after_stop { ... }
 *
 * These names remain open-ended.
 *
 * The lifecycle grammar does not execute the contents.
 *
 * Semantic/runtime layers determine the permitted hook behavior.
 *
 * ============================================================================
 * NESTED POLICY BLOCKS
 * ============================================================================
 *
 * Lifecycle properties may contain nested blocks:
 *
 *     lifecycle computation {
 *         recovery {
 *             strategy: retry;
 *             checkpoint: enabled;
 *         }
 *     }
 *
 * This avoids creating a new parser rule for every future lifecycle policy.
 *
 * ============================================================================
 * EXPRESSION OWNERSHIP
 * ============================================================================
 *
 * Lifecycle values are expressions.
 *
 * Therefore this grammar does NOT duplicate:
 *
 *     numeric literals
 *     strings
 *     arrays
 *     maps
 *     calls
 *     arithmetic
 *     comparisons
 *     logical expressions
 *     quantum expressions
 *     tensor expressions
 *     resource expressions
 *
 * Those remain owned by Expressions and the relevant domain grammars.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar is designed to map into a domain-neutral AST equivalent to:
 *
 *     LifecycleDeclaration {
 *         target: Option<Expression>,
 *         members: Vec<LifecycleMember>,
 *         source_span: SourceSpan,
 *     }
 *
 *     LifecycleMember:
 *         Property
 *         Transition
 *         NamedBlock
 *
 *     LifecycleProperty {
 *         key: QualifiedName,
 *         value: Expression,
 *         source_span: SourceSpan,
 *     }
 *
 *     LifecycleTransition {
 *         from: QualifiedName,
 *         to: QualifiedName,
 *         condition: Option<Expression>,
 *         body: Option<Vec<LifecycleMember>>,
 *         source_span: SourceSpan,
 *     }
 *
 *     LifecycleNamedBlock {
 *         name: QualifiedName,
 *         members: Vec<LifecycleMember>,
 *         source_span: SourceSpan,
 *     }
 *
 * The exact Rust types belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT require:
 *
 *     PhysicalDeviceId
 *     CpuId
 *     GpuId
 *     FpgaId
 *     QpuId
 *     PhysicalQubitId
 *     HardwareTopology
 *
 * in the frontend AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - canonical lifecycle keyword validation;
 *     - target resolution;
 *     - state declaration interpretation;
 *     - transition graph construction;
 *     - state reachability;
 *     - initial-state validation;
 *     - terminal-state validation;
 *     - transition guard validation;
 *     - lifecycle determinism;
 *     - lifecycle/effect compatibility;
 *     - lifecycle/resource compatibility;
 *     - lifecycle/capability compatibility;
 *     - lifecycle/recovery integration;
 *     - lifecycle/resilience integration;
 *     - lifecycle/checkpoint integration;
 *     - lifecycle/runtime integration.
 *
 * Parsing MUST NOT perform these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT create a lifecycle IR.
 *
 * The lowering path is:
 *
 *     LifecycleDeclaration
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic lifecycle model
 *             |
 *             v
 *     canonical semantic/execution representation
 *             |
 *       +-----+-----+------------------+
 *       |           |                  |
 *       v           v                  v
 *   classical    quantum::ir       HDL/hardware
 *       |           |                  |
 *       +-----------+------------------+
 *                   |
 *                   v
 *          scheduling / routing /
 *          resilience / QEC / ZQN
 *                   |
 *                   v
 *                  HAL
 *                   |
 *                   v
 *                runtime
 *
 * Quantum lifecycle metadata may accompany a computation that ultimately
 * lowers through quantum::ir.
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * `execution.g4` is the execution composition root.
 *
 * It should import this grammar and expose:
 *
 *     lifecycleDeclaration
 *     lifecycleStatement
 *
 * through its public execution dispatch.
 *
 * `runtime.g4` owns runtime-specific execution-control syntax.
 *
 * Runtime MUST NOT copy this grammar.
 *
 * Runtime may consume the semantic Lifecycle model produced after parsing.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime lifecycle operations such as:
 *
 *     start
 *     stop
 *     pause
 *     resume
 *     cancel
 *     wait
 *     yield
 *
 * remain runtime concerns.
 *
 * Lifecycle grammar describes the state model governing such operations.
 *
 * Therefore:
 *
 *     lifecycle.g4
 *         -> lifecycle declaration
 *
 *     runtime.g4
 *         -> runtime operation
 *
 *     semantic analysis
 *         -> validates relationship
 *
 *     runtime
 *         -> realizes transition
 *
 * Neither grammar should absorb the other.
 *
 * ============================================================================
 * RECOVERY / RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Recovery and resilience remain separate authorities.
 *
 * Lifecycle may contain semantic properties referring to:
 *
 *     recovery
 *     retry
 *     degradation
 *     escalation
 *     failover
 *     checkpoint
 *     restoration
 *
 * Example:
 *
 *     failed -> recovering with {
 *         strategy: recover;
 *     };
 *
 * `recovery.g4` and `resilience.g4` remain responsible for their own syntax.
 *
 * The lifecycle grammar MUST NOT reproduce their complete grammars.
 *
 * ============================================================================
 * CHECKPOINTING INTEGRATION
 * ============================================================================
 *
 * Lifecycle can identify checkpoint-related policy through generic properties
 * or nested blocks.
 *
 * Example:
 *
 *     running {
 *         checkpoint {
 *             enabled: true;
 *         }
 *     }
 *
 * Checkpoint creation, persistence, storage, and restoration are owned by
 * checkpointing/runtime infrastructure.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * A lifecycle transition may have semantic scheduling metadata:
 *
 *     running -> paused with {
 *         schedule: deferred;
 *     };
 *
 * The lifecycle grammar stores the metadata structurally.
 *
 * It does not calculate:
 *
 *     start times;
 *     end times;
 *     resource occupancy;
 *     critical paths;
 *     queue order;
 *     schedules.
 *
 * Those belong to scheduling.
 *
 * ============================================================================
 * PLACEMENT / ROUTING INTEGRATION
 * ============================================================================
 *
 * Lifecycle MUST NOT map:
 *
 *     state -> CPU
 *     state -> GPU
 *     state -> QPU
 *     state -> physical qubit
 *     state -> FPGA region
 *
 * Placement and routing remain downstream.
 *
 * ============================================================================
 * OBSERVABILITY / TRACING / PROFILING
 * ============================================================================
 *
 * Lifecycle semantics may be observed by:
 *
 *     observability
 *     tracing
 *     profiling
 *
 * but lifecycle.g4 does not duplicate their syntax.
 *
 * For example:
 *
 *     running
 *
 * may be emitted as a runtime lifecycle event.
 *
 * The observation subsystem determines how that event is represented,
 * transported, sampled, correlated, or stored.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * The lifecycle target is intentionally domain neutral.
 *
 * It may refer to:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL computation
 *     hardware intent
 *     distributed computation
 *     AI workload
 *     tensor/data pipeline
 *     networking operation
 *     cryptographic computation
 *     embedded computation
 *     future domain
 *
 * No domain-specific lifecycle grammar is required merely to support a new
 * execution domain.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Lifecycle may apply to:
 *
 *     quantum circuit
 *     quantum kernel
 *     quantum job
 *     logical computation
 *     QEC-protected computation
 *     hybrid quantum/classical computation
 *
 * It does not define:
 *
 *     gates
 *     qubits
 *     quantum states
 *     measurements
 *     physical topology
 *     pulses
 *     calibration
 *     QEC algorithms
 *     ZQN models
 *
 * Quantum computation continues to use:
 *
 *     quantum::ir
 *
 * as the canonical quantum IR boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Lifecycle may apply to hardware/co-design entities:
 *
 *     synthesis
 *     simulation
 *     deployment
 *     activation
 *     verification
 *     degradation
 *     retirement
 *
 * It does not define physical implementation details.
 *
 * No fixed:
 *
 *     bit width
 *     register width
 *     FPGA region count
 *     pipeline depth
 *     memory capacity
 *
 * is imposed here.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Lifecycle may apply to distributed computations:
 *
 *     created
 *     scheduled
 *     running
 *     degraded
 *     migrating
 *     recovering
 *     completed
 *     retired
 *
 * The grammar does not define:
 *
 *     node count
 *     replica count
 *     physical node identity
 *     network topology
 *     transport protocol
 *
 * Those belong to distributed execution, networking, placement, and runtime
 * subsystems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no runtime calls;
 *     - no hardware access;
 *     - no network access;
 *     - no filesystem access;
 *     - no randomness;
 *     - no mutable global state.
 *
 * Source order is preserved by the parser.
 *
 * Semantic analysis decides whether the resulting lifecycle is deterministic.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover syntax failures such as:
 *
 *     missing lifecycle body
 *     missing transition destination
 *     malformed property
 *     malformed transition
 *     missing separator
 *
 * Semantic diagnostics cover:
 *
 *     invalid lifecycle target
 *     duplicate initial state
 *     unknown state
 *     unreachable state
 *     invalid terminal state
 *     transition to unknown state
 *     impossible transition guard
 *     conflicting lifecycle properties
 *     duplicate singleton property
 *     invalid lifecycle extension
 *     unsupported lifecycle capability
 *     unsatisfied lifecycle resource requirement
 *     incompatible recovery policy
 *     invalid lifecycle effect
 *
 * All diagnostics must preserve source spans.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Lifecycle grammar cannot:
 *
 *     - execute code;
 *     - execute shell commands;
 *     - access hardware;
 *     - access files;
 *     - access networks;
 *     - bypass authorization;
 *     - bypass capability validation;
 *     - bypass semantic validation;
 *     - invoke runtime services during parsing.
 *
 * Lifecycle bodies are syntax, not immediate execution.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar imposes no language-level limits on:
 *
 *     lifecycle declarations
 *     lifecycle members
 *     states
 *     transitions
 *     hooks
 *     events
 *     nested policy blocks
 *     properties
 *
 * Repetition is represented structurally.
 *
 * Implementation resource exhaustion is distinct from language invalidity.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical stable form is:
 *
 *     lifecycle <target> {
 *         <members>
 *     }
 *
 * `lifecycle` remains semantically validated as a contextual identifier until
 * the lexical specification promotes it to a reserved keyword.
 *
 * Lifecycle property names remain open-ended.
 *
 * This permits future lifecycle features without changing the AST model.
 *
 * Any future stable syntax must follow:
 *
 *     proposal
 *       ->
 *     semantic design
 *       ->
 *     AST contract
 *       ->
 *     grammar
 *       ->
 *     semantic implementation
 *       ->
 *     IR contract
 *       ->
 *     tests
 *       ->
 *     stable
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * lifecycleDeclaration
 *
 *     Standalone lifecycle declaration.
 *
 * lifecycleStatement
 *
 *     Statement-compatible wrapper for execution composition.
 *
 * lifecycleBody
 *
 *     Reusable lifecycle body.
 *
 * lifecycleMember
 *
 *     Reusable lifecycle member.
 *
 * lifecycleTransition
 *
 *     Reusable transition rule.
 *
 * ============================================================================
 * EXAMPLES
 * ============================================================================
 *
 * Basic lifecycle:
 *
 *     lifecycle computation {
 *         initial: created;
 *         terminal: completed;
 *
 *         created -> running;
 *         running -> completed;
 *     }
 *
 * Conditional transition:
 *
 *     lifecycle computation {
 *         initial: created;
 *
 *         created -> running
 *             when ready;
 *
 *         running -> failed
 *             when failure_detected;
 *     }
 *
 * Transition policy:
 *
 *     lifecycle computation {
 *         running -> recovering
 *             when fault_detected
 *             with {
 *                 strategy: recover;
 *                 requires: capability("recovery");
 *             };
 *     }
 *
 * Nested lifecycle policy:
 *
 *     lifecycle distributed::service {
 *         initial: created;
 *
 *         running {
 *             checkpoint {
 *                 enabled: true;
 *             }
 *         }
 *     }
 *
 * Quantum lifecycle:
 *
 *     lifecycle quantum::kernel {
 *         initial: created;
 *
 *         created -> running
 *             with {
 *                 requires: capability("quantum.compute");
 *             };
 *
 *         running -> completed
 *             when measurement_complete;
 *     }
 *
 * HDL lifecycle:
 *
 *     lifecycle hardware::accelerator {
 *         initial: synthesized;
 *
 *         synthesized -> verified;
 *         verified -> deployed;
 *         deployed -> retired;
 *     }
 *
 * ============================================================================
 * IMPORTANT SEMANTIC RULE
 * ============================================================================
 *
 * The grammar accepts lifecycle structure.
 *
 * It does NOT declare that a lifecycle state exists merely because it appears
 * syntactically.
 *
 * State existence, state identity, transition validity, reachability, and
 * execution behavior are semantic concerns.
 *
 * ============================================================================
 */

parser grammar Lifecycle;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ============================================================================
 */

/*
 * Standalone lifecycle declaration.
 *
 * Canonical form:
 *
 *     lifecycle <target> {
 *         ...
 *     }
 */
lifecycleDeclaration
    : lifecycleKeyword lifecycleTarget? lifecycleBody
    ;

/*
 * Statement-compatible wrapper.
 *
 * The optional terminator allows composition with statement systems that
 * already own semicolon handling while still permitting lifecycle declarations
 * inside execution-oriented bodies.
 */
lifecycleStatement
    : lifecycleDeclaration SEMI?
    ;


/*
 * ============================================================================
 * 2. CONTEXTUAL LIFECYCLE KEYWORD
 * ============================================================================
 *
 * `lifecycle` is intentionally contextual at this stage because the current
 * canonical lexer does not require a dedicated LIFECYCLE token.
 *
 * Semantic analysis MUST require the exact canonical spelling.
 */
lifecycleKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 3. TARGET
 * ============================================================================
 *
 * The target is an expression so lifecycle intent can attach to:
 *
 *     named computations
 *     qualified computations
 *     function calls
 *     pipelines
 *     expressions
 *     domain-neutral execution subjects
 *
 * Physical target selection is prohibited at this layer.
 */
lifecycleTarget
    : expression
    ;


/*
 * ============================================================================
 * 4. BODY
 * ============================================================================
 */

lifecycleBody
    : LBRACE lifecycleMember* RBRACE
    ;


/*
 * ============================================================================
 * 5. MEMBER DISPATCH
 * ============================================================================
 *
 * Members are deliberately structural rather than keyword-enumerated.
 *
 * This permits lifecycle evolution without continually expanding the lexer.
 */
lifecycleMember
    : lifecycleTransition
    | lifecycleProperty
    | lifecycleNamedBlock
    ;


/*
 * ============================================================================
 * 6. PROPERTIES
 * ============================================================================
 *
 * Canonical form:
 *
 *     property: expression;
 *
 * Assignment syntax is retained as a compatibility form:
 *
 *     property = expression;
 *
 * The semantic specification decides whether assignment syntax is enabled
 * for the active language profile.
 */
lifecycleProperty
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;


/*
 * ============================================================================
 * 7. TRANSITIONS
 * ============================================================================
 *
 * Canonical form:
 *
 *     from -> to;
 *
 * Extended form:
 *
 *     from -> to when expression;
 *
 *     from -> to with {
 *         ...
 *     };
 *
 *     from -> to when expression with {
 *         ...
 *     };
 *
 * State names remain semantic qualified names.
 */
lifecycleTransition
    : lifecycleStateReference
      ARROW
      lifecycleStateReference
      lifecycleTransitionCondition?
      lifecycleTransitionBody?
      SEMI
    ;


/*
 * ============================================================================
 * 8. STATE REFERENCE
 * ============================================================================
 *
 * A state reference is symbolic.
 *
 * It is NOT a physical device/resource reference.
 */
lifecycleStateReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 9. TRANSITION CONDITION
 * ============================================================================
 *
 * `when` is an existing canonical lexical token.
 *
 * The expression grammar remains authoritative for the condition.
 */
lifecycleTransitionCondition
    : WHEN expression
    ;


/*
 * ============================================================================
 * 10. TRANSITION BODY
 * ============================================================================
 *
 * `with { ... }` attaches semantic metadata/policy to a transition.
 *
 * It does not execute the body.
 */
lifecycleTransitionBody
    : WITH lifecycleBody
    ;


/*
 * ============================================================================
 * 11. NAMED LIFECYCLE BLOCK
 * ============================================================================
 *
 * Named blocks provide open-ended lifecycle extension points.
 *
 * Examples:
 *
 *     entry { ... }
 *     exit { ... }
 *     recovery { ... }
 *     checkpoint { ... }
 *     retry { ... }
 *     observability { ... }
 *
 * The semantic registry determines which names have defined meaning.
 */
lifecycleNamedBlock
    : qualifiedName lifecycleBody
    ;


/*
 * ============================================================================
 * 12. REUSABLE VALUES
 * ============================================================================
 *
 * These aliases provide stable integration points without duplicating the
 * expression grammar.
 */
lifecycleValue
    : expression
    ;

lifecycleCondition
    : expression
    ;

lifecyclePolicyValue
    : expression
    ;

lifecycleRequirementValue
    : expression
    ;

lifecycleCapabilityValue
    : expression
    ;

lifecycleResourceValue
    : expression
    ;

lifecyclePreferenceValue
    : expression
    ;

lifecycleHintValue
    : expression
    ;


/*
 * ============================================================================
 * 13. REUSABLE TRANSITION COMPONENTS
 * ============================================================================
 *
 * These rules are intentionally public enough for future execution grammar
 * composition without requiring those grammars to duplicate transition
 * structure.
 */
lifecycleTransitionFrom
    : lifecycleStateReference
    ;

lifecycleTransitionTo
    : lifecycleStateReference
    ;

lifecycleTransitionGuard
    : lifecycleTransitionCondition
    ;

lifecycleTransitionMetadata
    : lifecycleTransitionBody
    ;


/*
 * ============================================================================
 * 14. SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * These rules do not reserve new keywords.
 *
 * They are structural aliases for semantic consumers and future feature
 * manifests.
 *
 * The semantic registry determines the exact meaning of the property key.
 */
lifecycleInitialState
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;

lifecycleTerminalState
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;

lifecycleEntryPolicy
    : qualifiedName lifecycleBody
    ;

lifecycleExitPolicy
    : qualifiedName lifecycleBody
    ;

lifecycleEventPolicy
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;

lifecycleRecoveryPolicy
    : qualifiedName lifecycleBody
    ;

lifecycleCheckpointPolicy
    : qualifiedName lifecycleBody
    ;

lifecycleRetryPolicy
    : qualifiedName lifecycleBody
    ;

lifecycleCapabilityPolicy
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;

lifecycleResourcePolicy
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;

lifecyclePortabilityPolicy
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;

lifecycleDeterminismPolicy
    : qualifiedName (COLON | ASSIGN) expression SEMI
    ;


/*
 * ============================================================================
 * 15. EXTENSION BLOCK
 * ============================================================================
 *
 * This is deliberately structural.
 *
 * Future lifecycle dialects may attach additional semantic blocks without
 * changing the core parser shape.
 */
lifecycleExtension
    : qualifiedName lifecycleBody
    ;


/*
 * ============================================================================
 * 16. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] ownership is explicit;
 * [x] non-ownership is explicit;
 * [x] canonical lexer boundary is explicit;
 * [x] no competing lexer is introduced;
 * [x] no lifecycle-specific machine limits exist;
 * [x] lifecycle state vocabulary is open-ended;
 * [x] transition count is unbounded by language semantics;
 * [x] event vocabulary is open-ended;
 * [x] policy vocabulary is open-ended;
 * [x] expressions are delegated to Expressions;
 * [x] types are delegated to Types;
 * [x] identifiers/names are delegated to Core;
 * [x] lifecycle does not implement runtime behavior;
 * [x] lifecycle does not implement scheduling;
 * [x] lifecycle does not implement placement;
 * [x] lifecycle does not implement routing;
 * [x] lifecycle does not implement recovery;
 * [x] lifecycle does not implement resilience;
 * [x] lifecycle does not implement QEC;
 * [x] lifecycle does not implement ZQN;
 * [x] lifecycle does not implement HAL;
 * [x] lifecycle does not introduce a second IR;
 * [x] quantum computation remains compatible with quantum::ir;
 * [x] classical computation remains domain neutral;
 * [x] HDL/hardware computation remains target independent;
 * [x] distributed computation remains topology independent;
 * [x] no Rust code is embedded;
 * [x] no unsafe code is required;
 * [x] parsing is deterministic;
 * [x] source structure is preserved for semantic analysis.
 *
 * Repository integration still requires the corresponding composition and
 * conformance tests described in the integration contract.
 *
 * ============================================================================
 */