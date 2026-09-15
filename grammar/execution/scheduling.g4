/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/scheduling.g4
 *
 * Grammar:
 *     Scheduling
 *
 * Status:
 *     Production scheduling-intent parser grammar
 *
 * Purpose:
 *     Defines source-level scheduling intent for Zamani execution.
 *
 * Architectural position:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> semantic analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling subsystem
 *          |
 *          +--> scheduling model
 *          +--> dependency analysis
 *          +--> resource constraints
 *          +--> temporal constraints
 *          +--> planner
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 *
 * CORE PRINCIPLE
 * ============================================================================
 *
 * This grammar describes:
 *
 *     WHAT scheduling behavior is requested or permitted.
 *
 * It does NOT describe:
 *
 *     HOW a scheduler computes a schedule.
 *
 * Therefore this grammar MUST NOT implement:
 *
 *     - ASAP scheduling;
 *     - ALAP scheduling;
 *     - list scheduling;
 *     - critical-path scheduling;
 *     - RCPSP;
 *     - resource allocation;
 *     - dependency analysis;
 *     - routing;
 *     - pulse scheduling;
 *     - calibration;
 *     - hardware discovery;
 *     - device selection;
 *     - queue management;
 *     - runtime scheduling;
 *     - operating-system scheduling.
 *
 * Those are semantic/compiler/runtime responsibilities.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the `schedule` source construct;
 *     - scheduling intent;
 *     - scheduling policy references;
 *     - scheduling objectives;
 *     - ordering intent;
 *     - dependency intent;
 *     - temporal constraints;
 *     - resource-aware scheduling intent;
 *     - alignment intent;
 *     - latency intent;
 *     - deadline intent;
 *     - priority intent;
 *     - concurrency limits expressed semantically;
 *     - scheduling preferences;
 *     - scheduling constraints;
 *     - scheduling hints;
 *     - schedule composition;
 *     - schedule metadata/property blocks.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - canonical IR;
 *     - quantum operations;
 *     - physical qubit mapping;
 *     - routing;
 *     - hardware discovery;
 *     - resource discovery;
 *     - scheduling algorithms;
 *     - runtime queues;
 *     - device selection;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - optimization algorithms.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Scheduling syntax must preserve:
 *
 *     Program_Once
 *          |
 *          v
 *     Compile_Once
 *          |
 *          v
 *     Run_Everywhere
 *          |
 *          v
 *     Run_Anywhere
 *          |
 *          v
 *     Run_Forever
 *
 * A source-level schedule therefore describes semantic requirements,
 * constraints, preferences and hints rather than a concrete machine schedule.
 *
 * For example, source syntax may express:
 *
 *     schedule {
 *         policy: asap;
 *         priority: critical;
 *         alignment: required;
 *         latency: bounded;
 *     }
 *
 * without meaning:
 *
 *     use device X;
 *     use N cores;
 *     use N qubits;
 *     use topology Y;
 *     use a fixed clock;
 *     use a fixed pulse duration.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately no grammar-level finite scheduling limits.
 *
 * This grammar contains no:
 *
 *     MAX_OPERATIONS
 *     MAX_STAGES
 *     MAX_RESOURCES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_DEPENDENCIES
 *     MAX_SCHEDULE_LENGTH
 *     MAX_DURATION
 *     MAX_CONCURRENCY
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * Actual limits belong to:
 *
 *     - semantic validation;
 *     - compiler policy;
 *     - resource management;
 *     - target capabilities;
 *     - runtime policy;
 *     - operating-system limits;
 *     - explicitly requested program constraints.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no embedded Rust;
 *     - no runtime calls;
 *     - no random behavior;
 *     - no device discovery;
 *     - no filesystem access;
 *     - no network access;
 *     - no mutable global state.
 *
 * Parsing therefore depends only on the canonical token stream.
 *
 * ============================================================================
 *
 * RUST COMPATIBILITY
 * ============================================================================
 *
 * Generated parser/runtime integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The grammar itself contains no Rust implementation code.
 *
 * The Zamani implementation requires safe Rust.
 *
 * No `unsafe` Rust is required or permitted by this grammar.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar requires the canonical lexer to expose:
 *
 *     SCHEDULE : 'schedule'
 *
 * as a reserved scheduling construct.
 *
 * This is intentionally a single stable syntactic keyword.
 *
 * Scheduling policies, objectives, resources, capabilities, algorithms and
 * vendor extensions remain identifiers/qualified names rather than an
 * ever-growing closed keyword list.
 *
 * ============================================================================
 *
 * CORE GRAMMAR CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical core rules:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *
 * It MUST NOT redefine them.
 *
 * ============================================================================
 *
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser produces syntax.
 *
 * Semantic analysis determines:
 *
 *     - whether the requested schedule is meaningful;
 *     - whether a referenced policy exists;
 *     - whether a dependency is valid;
 *     - whether timing constraints are satisfiable;
 *     - whether resource constraints are satisfiable;
 *     - whether capabilities support the request;
 *     - whether the requested schedule conflicts with other constraints.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum scheduling consumes canonical quantum semantics.
 *
 * The grammar does not:
 *
 *     - schedule gates;
 *     - choose physical qubits;
 *     - insert SWAP operations;
 *     - choose pulse timings;
 *     - select calibration data;
 *     - construct hardware topology.
 *
 * The intended pipeline is:
 *
 *     quantum source
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     hardware realization
 *
 * ============================================================================
 *
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Scheduling can apply to:
 *
 *     - classical functions;
 *     - tasks;
 *     - loops;
 *     - pipelines;
 *     - accelerator work;
 *     - asynchronous operations;
 *     - distributed computation.
 *
 * The grammar does not distinguish those by fixed machine assumptions.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware scheduling may consume:
 *
 *     - timing constraints;
 *     - clock relationships;
 *     - latency requirements;
 *     - pipeline constraints;
 *     - resource conflicts;
 *     - ordering requirements.
 *
 * This grammar does not define physical clocks, frequencies, wires,
 * registers, devices or topology.
 *
 * Those belong to HDL/hardware semantic layers.
 *
 * ============================================================================
 *
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Scheduling may express:
 *
 *     - ordering;
 *     - dependency;
 *     - locality;
 *     - latency;
 *     - synchronization;
 *     - priority;
 *     - resource intent.
 *
 * Node placement and distributed execution remain downstream concerns.
 *
 * ============================================================================
 *
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * A schedule may contain retry/recovery-compatible constraints or hints,
 * but this grammar does not implement recovery.
 *
 * Resilience remains responsible for adapting execution after failures.
 *
 * ============================================================================
 *
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource references are semantic expressions.
 *
 * The grammar does not discover or allocate resources.
 *
 * For example:
 *
 *     resources: available
 *
 * may be interpreted downstream according to the resource model.
 *
 * A source program must not need rewriting merely because the available
 * resource quantity changes.
 *
 * ============================================================================
 *
 * EXTENSIBILITY
 * ============================================================================
 *
 * Scheduling policies and properties intentionally use qualified names.
 *
 * This permits:
 *
 *     standard policies;
 *     future policies;
 *     domain-specific policies;
 *     vendor-neutral extensions;
 *     dialect-specific policies;
 *
 * without requiring the grammar to enumerate every future scheduler.
 *
 * Semantic validation remains responsible for determining whether a name is
 * known and supported.
 *
 * ============================================================================
 */

parser grammar Scheduling;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. TOP-LEVEL SCHEDULING CONSTRUCT
 * ============================================================================
 *
 * Canonical form:
 *
 *     schedule {
 *         ...
 *     }
 *
 * A schedule may also be associated with an explicit subject:
 *
 *     schedule target {
 *         ...
 *     }
 *
 * The subject is an expression because Zamani must not maintain a closed
 * inventory of schedulable entities.
 * ========================================================================== */

executionSchedule
    : SCHEDULE executionScheduleSubject? executionScheduleBody executionScheduleTerminator?
    ;


/* ============================================================================
 * 2. SCHEDULE SUBJECT
 * ============================================================================
 *
 * Examples:
 *
 *     schedule circuit {
 *         ...
 *     }
 *
 *     schedule pipeline {
 *         ...
 *     }
 *
 *     schedule expression {
 *         ...
 *     }
 *
 * Semantic analysis determines whether the expression denotes a schedulable
 * entity.
 * ========================================================================== */

executionScheduleSubject
    : expression
    ;


/* ============================================================================
 * 3. SCHEDULE BODY
 * ============================================================================
 *
 * An empty schedule body is intentionally legal.
 *
 * This permits tooling, macros and incremental compilation to construct an
 * initially empty scheduling declaration.
 * ========================================================================== */

executionScheduleBody
    : LBRACE executionScheduleEntry* RBRACE
    ;


/* ============================================================================
 * 4. SCHEDULE ENTRIES
 * ============================================================================
 *
 * Entries are categorized structurally so downstream AST construction can
 * distinguish semantic intent without requiring string inspection.
 * ========================================================================== */

executionScheduleEntry
    : executionSchedulePolicy
    | executionScheduleObjective
    | executionScheduleOrder
    | executionScheduleDependency
    | executionScheduleTiming
    | executionScheduleAlignment
    | executionSchedulePriority
    | executionScheduleResource
    | executionScheduleConcurrency
    | executionScheduleLatency
    | executionScheduleDeadline
    | executionSchedulePreference
    | executionScheduleConstraint
    | executionScheduleHint
    | executionScheduleProperty
    ;


/* ============================================================================
 * 5. POLICY
 * ============================================================================
 *
 * Examples:
 *
 *     policy: asap;
 *     policy: critical_path;
 *     policy: custom::policy;
 *
 * The grammar does not enumerate algorithms.
 *
 * Therefore adding a new scheduler does not require changing the language
 * grammar.
 * ========================================================================== */

executionSchedulePolicy
    : SCHEDULE_POLICY_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/*
 * The canonical lexer may later reserve `policy`.
 *
 * Until that keyword exists, this rule is intentionally isolated behind a
 * dedicated token so the lexer/parser contract is explicit.
 *
 * See integration note below.
 */


/* ============================================================================
 * 6. OBJECTIVE
 * ============================================================================
 *
 * Examples:
 *
 *     objective: latency;
 *     objective: throughput;
 *     objective: energy;
 *     objective: fidelity;
 *
 * The semantic layer determines the objective's validity.
 * ========================================================================== */

executionScheduleObjective
    : SCHEDULE_OBJECTIVE_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 7. ORDERING
 * ============================================================================
 *
 * Ordering expresses precedence intent.
 *
 * It does not construct the dependency graph.
 * ========================================================================== */

executionScheduleOrder
    : SCHEDULE_ORDER_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 8. DEPENDENCY INTENT
 * ============================================================================
 *
 * Generic dependency expressions allow:
 *
 *     dependency: a before b;
 *
 *     dependency: stage_a -> stage_b;
 *
 * depending on the expression grammar available to the enclosing parser.
 *
 * The scheduler is responsible for validating and constructing the actual
 * dependency DAG.
 * ========================================================================== */

executionScheduleDependency
    : SCHEDULE_DEPENDENCY_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 9. TIMING
 * ============================================================================
 *
 * Timing is represented as an expression.
 *
 * This allows duration/temporal types to evolve independently of the grammar.
 *
 * Examples:
 *
 *     timing: bounded;
 *     timing: duration;
 *     timing: window;
 *
 * The grammar does not establish physical clock precision.
 * ========================================================================== */

executionScheduleTiming
    : SCHEDULE_TIMING_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 10. ALIGNMENT
 * ============================================================================
 *
 * Alignment expresses semantic alignment requirements.
 *
 * Hardware-specific alignment units are resolved later.
 * ========================================================================== */

executionScheduleAlignment
    : SCHEDULE_ALIGNMENT_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 11. PRIORITY
 * ============================================================================
 *
 * Priority is an execution preference/constraint, not a scheduler algorithm.
 * ========================================================================== */

executionSchedulePriority
    : SCHEDULE_PRIORITY_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 12. RESOURCE INTENT
 * ============================================================================
 *
 * Resources are semantic expressions.
 *
 * No finite resource count is encoded here.
 * ========================================================================== */

executionScheduleResource
    : SCHEDULE_RESOURCE_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 13. CONCURRENCY
 * ============================================================================
 *
 * Concurrency may be expressed as:
 *
 *     concurrency: available;
 *     concurrency: bounded;
 *     concurrency: requirement;
 *
 * A numeric value, if supplied, remains a program-level constraint rather
 * than a machine-wide grammar limit.
 * ========================================================================== */

executionScheduleConcurrency
    : SCHEDULE_CONCURRENCY_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 14. LATENCY
 * ============================================================================
 *
 * Latency is expressed semantically.
 *
 * Actual measured latency belongs to runtime/target analysis.
 * ========================================================================== */

executionScheduleLatency
    : SCHEDULE_LATENCY_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 15. DEADLINE
 * ============================================================================
 *
 * Deadline syntax expresses a requirement or preference.
 *
 * Whether it is achievable is a semantic/runtime question.
 * ========================================================================== */

executionScheduleDeadline
    : SCHEDULE_DEADLINE_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 16. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory unless semantic analysis explicitly determines
 * otherwise from the surrounding construct.
 * ========================================================================== */

executionSchedulePreference
    : SCHEDULE_PREFERENCE_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 17. CONSTRAINT
 * ============================================================================
 *
 * Constraints restrict legal schedules.
 *
 * They do not select a specific physical realization.
 * ========================================================================== */

executionScheduleConstraint
    : SCHEDULE_CONSTRAINT_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 18. HINT
 * ============================================================================
 *
 * Hints are advisory.
 *
 * A backend MUST NOT silently promote a hint to a semantic requirement.
 * ========================================================================== */

executionScheduleHint
    : SCHEDULE_HINT_KEY executionScheduleValue executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 19. EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * Future scheduling concepts can be represented without changing the
 * scheduler grammar when the semantic property name is not a core language
 * keyword.
 *
 * Example:
 *
 *     custom::scheduler_property: value;
 *
 * This rule is intentionally last in the alternatives so well-known
 * scheduling properties remain structurally identifiable.
 * ========================================================================== */

executionScheduleProperty
    : qualifiedName executionSchedulePropertyAssignment executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 20. PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * Both `:` and `=` are supported because they represent property assignment
 * rather than comparison.
 *
 * Comparison semantics remain inside expressions.
 * ========================================================================== */

executionSchedulePropertyAssignment
    : COLON executionScheduleValue
    | ASSIGN executionScheduleValue
    ;


/* ============================================================================
 * 21. GENERIC SCHEDULE VALUE
 * ============================================================================
 *
 * The value is delegated to the canonical expression grammar.
 *
 * This is critical for scalability:
 *
 *     numbers
 *     durations
 *     identifiers
 *     qualified names
 *     function calls
 *     resource expressions
 *     capability expressions
 *     compile-time expressions
 *     future expression forms
 *
 * can all evolve without creating a second value language here.
 * ========================================================================== */

executionScheduleValue
    : expression
    | executionSchedulePropertyBlock
    ;


/* ============================================================================
 * 22. NESTED PROPERTY BLOCK
 * ============================================================================
 *
 * Nested blocks allow structured scheduling metadata without requiring a
 * separate scheduler-specific object model in the grammar.
 *
 * Example:
 *
 *     timing: {
 *         start: earliest;
 *         finish: bounded;
 *     }
 *
 * Semantic analysis determines whether each property is valid.
 * ========================================================================== */

executionSchedulePropertyBlock
    : LBRACE executionSchedulePropertyEntry* RBRACE
    ;


executionSchedulePropertyEntry
    : qualifiedName executionSchedulePropertyAssignment executionScheduleEntryTerminator
    ;


/* ============================================================================
 * 23. ENTRY TERMINATORS
 * ============================================================================
 *
 * Semicolon is the canonical separator.
 *
 * Comma is accepted to support compact generated/configuration-oriented
 * syntax where the surrounding language permits it.
 *
 * A terminator is required for every property entry.
 *
 * This deliberate requirement prevents ambiguous concatenation of scheduling
 * properties.
 * ========================================================================== */

executionScheduleEntryTerminator
    : SEMI
    | COMMA
    ;


/* ============================================================================
 * 24. OUTER SCHEDULE TERMINATOR
 * ============================================================================
 *
 * The enclosing execution construct may also own the final statement
 * terminator. This rule therefore remains optional.
 * ========================================================================== */

executionScheduleTerminator
    : SEMI
    ;


/* ============================================================================
 * 25. SCHEDULING KEYWORD CONTRACT
 * ============================================================================
 *
 * These symbolic names document the lexer contract required by this parser.
 *
 * IMPORTANT:
 *
 * In a parser grammar using tokenVocab, these tokens MUST be defined by the
 * canonical lexer. They are NOT parser-local token declarations.
 *
 * Required canonical lexer vocabulary:
 *
 *     SCHEDULE
 *     POLICY
 *     OBJECTIVE
 *     ORDER
 *     DEPENDENCY
 *     TIMING
 *     ALIGNMENT
 *     PRIORITY
 *     RESOURCE
 *     CONCURRENCY
 *     LATENCY
 *     DEADLINE
 *     PREFERENCE
 *     CONSTRAINT
 *     HINT
 *
 * The repository's lexer should add these only if they are not already
 * present. They should remain a small stable scheduling vocabulary.
 *
 * Future scheduler names MUST remain identifiers/qualified names.
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR NOTE
 * ============================================================================
 *
 * The aliases below are intentionally written as parser rule names rather
 * than lexer token declarations so this file remains a pure parser grammar.
 *
 * They must be mapped to the corresponding canonical lexer tokens by the
 * lexer/token vocabulary integration.
 * ============================================================================
 */

executionSchedulePolicyKey
    : POLICY
    ;

executionScheduleObjectiveKey
    : OBJECTIVE
    ;

executionScheduleOrderKey
    : ORDER
    ;

executionScheduleDependencyKey
    : DEPENDENCY
    ;

executionScheduleTimingKey
    : TIMING
    ;

executionScheduleAlignmentKey
    : ALIGNMENT
    ;

executionSchedulePriorityKey
    : PRIORITY
    ;

executionScheduleResourceKey
    : RESOURCE
    ;

executionScheduleConcurrencyKey
    : CONCURRENCY
    ;

executionScheduleLatencyKey
    : LATENCY
    ;

executionScheduleDeadlineKey
    : DEADLINE
    ;

executionSchedulePreferenceKey
    : PREFERENCE
    ;

executionScheduleConstraintKey
    : CONSTRAINT
    ;

executionScheduleHintKey
    : HINT
    ;


/*
 * ============================================================================
 * CANONICAL KEY RULE ALIASES
 * ============================================================================
 *
 * These aliases are intentionally separate from the semantic rule names so
 * future lexer vocabulary changes remain localized to the parser composition
 * layer.
 * ============================================================================
 */

executionSchedulePolicyKeyCanonical
    : executionSchedulePolicyKey
    ;

executionScheduleObjectiveKeyCanonical
    : executionScheduleObjectiveKey
    ;

executionScheduleOrderKeyCanonical
    : executionScheduleOrderKey
    ;

executionScheduleDependencyKeyCanonical
    : executionScheduleDependencyKey
    ;

executionScheduleTimingKeyCanonical
    : executionScheduleTimingKey
    ;

executionScheduleAlignmentKeyCanonical
    : executionScheduleAlignmentKey
    ;

executionSchedulePriorityKeyCanonical
    : executionSchedulePriorityKey
    ;

executionScheduleResourceKeyCanonical
    : executionScheduleResourceKey
    ;

executionScheduleConcurrencyKeyCanonical
    : executionScheduleConcurrencyKey
    ;

executionScheduleLatencyKeyCanonical
    : executionScheduleLatencyKey
    ;

executionScheduleDeadlineKeyCanonical
    : executionScheduleDeadlineKey
    ;

executionSchedulePreferenceKeyCanonical
    : executionSchedulePreferenceKey
    ;

executionScheduleConstraintKeyCanonical
    : executionScheduleConstraintKey
    ;

executionScheduleHintKeyCanonical
    : executionScheduleHintKey
    ;