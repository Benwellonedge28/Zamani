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
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Grammar contains no embedded Rust, semantic actions, predicates,
 *     filesystem access, network access, hardware discovery, or runtime calls.
 *
 *     The Zamani compiler/runtime implementation MUST use safe Rust only.
 *     `unsafe` Rust is not part of this contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL SCHEDULING INTENT.
 *
 * It describes constraints, requirements, preferences, hints, ordering,
 * temporal intent, resource intent, and scheduler-policy intent that may
 * influence later scheduling.
 *
 * It does NOT implement a scheduler.
 *
 * The complete architectural pipeline is:
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
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural analysis
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> dependency analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
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
 *     scheduling
 *          |
 *          v
 *     placement
 *          |
 *          v
 *     resilience / QEC / ZQN where applicable
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the `schedule` construct;
 *     - scheduling subjects;
 *     - scheduling property/intent entries;
 *     - scheduling requirement intent;
 *     - scheduling constraint intent;
 *     - scheduling preference intent;
 *     - scheduling hint intent;
 *     - scheduling policy references;
 *     - scheduling objective references;
 *     - ordering/dependency intent;
 *     - temporal intent;
 *     - alignment intent;
 *     - priority intent;
 *     - resource intent;
 *     - concurrency intent;
 *     - latency intent;
 *     - deadline intent;
 *     - extensible scheduling properties;
 *     - nested scheduling property blocks.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - expression precedence;
 *     - types;
 *     - resource-expression semantics;
 *     - capability discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - placement;
 *     - routing;
 *     - scheduling algorithms;
 *     - queue management;
 *     - operating-system scheduling;
 *     - runtime dispatch;
 *     - quantum operations;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - optimization algorithms.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT create a second:
 *
 *     expression grammar;
 *     name grammar;
 *     resource grammar;
 *     capability grammar;
 *     type grammar;
 *     scheduling algorithm;
 *     IR.
 *
 * Canonical expression syntax is imported from:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical name syntax is imported from:
 *
 *     grammar/core/names.g4
 *
 * Resource/capability meaning remains owned by their respective semantic
 * systems.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the canonical lexical composition.
 *
 * This grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Required scheduling-specific lexical token:
 *
 *     SCHEDULE
 *
 * The current repository's canonical keyword vocabulary does not yet expose
 * SCHEDULE. Therefore adding:
 *
 *     SCHEDULE : 'schedule' ;
 *
 * to grammar/lexer/keywords.g4 is the ONE required lexical integration
 * change for this grammar.
 *
 * No policy/objective/dependency/etc. tokens are required.
 *
 * Scheduling property names remain ordinary identifiers/qualified names.
 *
 * This deliberately avoids a closed scheduler keyword registry.
 *
 * ============================================================================
 * WHY ONLY `SCHEDULE` IS A KEYWORD
 * ============================================================================
 *
 * `schedule` introduces a distinct language construct and therefore benefits
 * from stable lexical recognition.
 *
 * Names such as:
 *
 *     asap
 *     critical_path
 *     latency
 *     throughput
 *     energy
 *     fidelity
 *     custom_policy
 *     vendor::policy
 *     future::scheduler
 *
 * do NOT need to become keywords.
 *
 * They are semantic names.
 *
 * This permits future scheduler policies, domain-specific scheduling
 * properties, and dialect extensions without repeatedly changing the lexer.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Scheduling syntax MUST preserve:
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
 * Scheduling source code describes portable execution intent.
 *
 * It MUST NOT require a particular:
 *
 *     CPU;
 *     core;
 *     thread;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     physical qubit;
 *     node;
 *     device;
 *     memory bank;
 *     accelerator;
 *     network topology;
 *     clock implementation;
 *     scheduler implementation.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level finite limits on:
 *
 *     - number of schedule entries;
 *     - number of nested property entries;
 *     - number of scheduling properties;
 *     - number of dependencies;
 *     - number of tasks;
 *     - number of operations;
 *     - number of resources;
 *     - number of machines;
 *     - number of nodes;
 *     - number of processors;
 *     - number of accelerators;
 *     - number of qubits;
 *     - schedule depth;
 *     - schedule duration;
 *     - concurrency;
 *     - target size.
 *
 * ANTLR repetition operators provide unbounded language structure:
 *
 *     *
 *     +
 *
 * Actual limits belong to implementation/resource policy.
 *
 * Examples:
 *
 *     compiler memory;
 *     compiler execution time;
 *     target capacity;
 *     runtime capacity;
 *     operating-system constraints;
 *     explicitly requested program constraints.
 *
 * These are NOT language-level scheduling limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_OPERATIONS
 *     MAX_STAGES
 *     MAX_RESOURCES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CONCURRENCY
 *     MAX_SCHEDULE_LENGTH
 *     MAX_DURATION
 *     MAX_TIMELINES
 *
 * It also MUST NOT encode:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical qubit 0
 *     fixed topology
 *     fixed clock frequency
 *     fixed register width
 *     fixed memory capacity.
 *
 * A numeric scheduling value is allowed when it is PROGRAM SEMANTICS.
 *
 * For example:
 *
 *     concurrency: available;
 *
 * or:
 *
 *     concurrency: requested_parallelism;
 *
 * or:
 *
 *     deadline: application_deadline;
 *
 * The grammar does not interpret the value as a machine limit.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts are intentionally represented as distinct structural forms.
 *
 * REQUIREMENT:
 *
 *     A condition necessary for a valid realization.
 *
 * CONSTRAINT:
 *
 *     A mandatory restriction on acceptable realization.
 *
 * PREFERENCE:
 *
 *     Advisory optimization intent.
 *
 * HINT:
 *
 *     Weaker advisory information.
 *
 * The semantic layer MUST preserve these distinctions.
 *
 * A preference MUST NOT silently become a requirement.
 *
 * A hint MUST NOT silently become a constraint.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - has no semantic actions;
 *     - has no predicates;
 *     - has no runtime calls;
 *     - has no hardware discovery;
 *     - has no resource discovery;
 *     - has no filesystem access;
 *     - has no network access;
 *     - has no randomness;
 *     - has no mutable global parser state.
 *
 * Given the same token stream and grammar version, parsing is deterministic.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Scheduling syntax is declarative.
 *
 * Parsing MUST NOT:
 *
 *     - start a process;
 *     - contact a scheduler;
 *     - contact a cluster;
 *     - query hardware;
 *     - inspect credentials;
 *     - allocate resources;
 *     - access a device;
 *     - execute a program;
 *     - invoke a quantum processor;
 *     - invoke an HDL simulator.
 *
 * ============================================================================
 * RUST COMPATIBILITY
 * ============================================================================
 *
 * This grammar is intentionally target-independent.
 *
 * Generated Zamani parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * No embedded Rust code is used.
 *
 * The compiler implementation MUST use safe Rust.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve scheduling syntax as a domain-neutral
 * scheduling-intent structure.
 *
 * Conceptually:
 *
 *     ExecutionSchedule {
 *         subject: Option<Expression>,
 *         entries: Vec<ScheduleEntry>,
 *         span: SourceSpan
 *     }
 *
 * Each entry should preserve at least:
 *
 *     kind;
 *     key/name where applicable;
 *     value;
 *     source span;
 *     source ordering.
 *
 * Recommended semantic categories:
 *
 *     Policy
 *     Objective
 *     Order
 *     Dependency
 *     Timing
 *     Alignment
 *     Priority
 *     Resource
 *     Concurrency
 *     Latency
 *     Deadline
 *     Requirement
 *     Constraint
 *     Preference
 *     Hint
 *     Property
 *
 * The exact Rust AST names belong to the existing frontend AST architecture.
 *
 * This grammar MUST NOT define those Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing determines structural validity only.
 *
 * Semantic analysis determines:
 *
 *     - whether the schedule subject is schedulable;
 *     - whether a property is valid;
 *     - whether a policy exists;
 *     - whether a policy is compatible with the program;
 *     - whether dependencies are meaningful;
 *     - whether timing values have valid temporal types;
 *     - whether constraints are satisfiable;
 *     - whether resources can satisfy the requirements;
 *     - whether capabilities exist;
 *     - whether preferences are feasible;
 *     - whether hints can be applied;
 *     - whether concurrent execution is semantically legal;
 *     - whether effects/ownership impose ordering;
 *     - whether quantum scheduling requirements are compatible with quantum::ir;
 *     - whether distributed scheduling requirements are realizable;
 *     - whether HDL timing intent is compatible with hardware semantics.
 *
 * The parser MUST NOT perform these checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct IR implementation.
 *
 * Scheduling intent is attached to the canonical semantic model.
 *
 * Later compiler stages consume that model when creating an executable
 * schedule.
 *
 * For quantum programs:
 *
 *     quantum source
 *          |
 *          v
 *     quantum semantic model
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
 *
 * This file MUST NOT create:
 *
 *     QuantumScheduleIR
 *     QuantumSchedulingIR
 *     PhysicalQubitScheduleIR
 *
 * as competing quantum IRs.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Scheduling intent may apply to:
 *
 *     functions;
 *     calls;
 *     loops;
 *     tasks;
 *     pipelines;
 *     asynchronous computation;
 *     accelerator work;
 *     data-parallel work;
 *     distributed work.
 *
 * The grammar does not enumerate these as a finite set of machine classes.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum scheduling may ultimately consider:
 *
 *     operation dependencies;
 *     resource conflicts;
 *     logical operation ordering;
 *     measurement dependencies;
 *     classical feed-forward;
 *     timing constraints;
 *     capability requirements;
 *     resilience requirements.
 *
 * This grammar does NOT:
 *
 *     - select physical qubits;
 *     - insert SWAP operations;
 *     - choose a coupling topology;
 *     - select calibration;
 *     - select a QPU;
 *     - schedule pulses;
 *     - perform QEC.
 *
 * Those belong to downstream subsystems.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Scheduling intent may be consumed for:
 *
 *     pipeline ordering;
 *     timing requirements;
 *     latency requirements;
 *     resource conflicts;
 *     synchronization;
 *     throughput goals.
 *
 * It MUST NOT encode a universal hardware clock, register width, FPGA size,
 * ASIC structure, or physical routing decision.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Scheduling may express:
 *
 *     ordering;
 *     dependency;
 *     latency;
 *     synchronization;
 *     locality-related intent;
 *     resource requirements;
 *     throughput objectives.
 *
 * Actual node placement remains owned by placement/deployment/resource
 * subsystems.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Scheduling values use the canonical expression language.
 *
 * This allows semantic expressions such as:
 *
 *     available_parallelism
 *     workload_size
 *     required_memory
 *     latency_budget
 *     problem_size
 *
 * without imposing a language-level resource ceiling.
 *
 * The scheduler/resource subsystem determines actual availability.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability references may be expressed through ordinary expressions and
 * qualified names.
 *
 * Examples:
 *
 *     capability::parallel
 *     quantum::dynamic_control
 *     accelerator::tensor_compute
 *
 * Whether a capability exists is decided downstream.
 *
 * ============================================================================
 * PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Scheduling and placement remain separate:
 *
 *     scheduling
 *         =
 *     WHEN / ORDER / UNDER WHAT EXECUTION CONSTRAINTS
 *
 *     placement
 *         =
 *     WHERE / WITH WHICH ALLOWED REALIZATION CONTEXT
 *
 * The scheduler may consume placement information, but this grammar does not
 * duplicate grammar/execution/placement.g4.
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Scheduling does not implement:
 *
 *     retry;
 *     recovery;
 *     failover;
 *     quarantine;
 *     escalation;
 *     degradation.
 *
 * Those remain resilience/runtime concerns.
 *
 * A scheduling property may reference a resilience policy by name/value, but
 * semantic execution behavior belongs downstream.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Scheduling properties are open-ended:
 *
 *     standard_property: value;
 *
 *     custom::property: value;
 *
 *     future::scheduler::property: value;
 *
 * This means future scheduling capabilities can be introduced without
 * requiring every scheduler name to become a lexer keyword.
 *
 * Semantic validation determines whether a property is known, supported,
 * deprecated, experimental, or supplied by a dialect.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER DEFINITION
 * ============================================================================
 */

parser grammar Scheduling;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC SCHEDULE ENTRY POINT
 * ============================================================================
 *
 * Canonical:
 *
 *     schedule {
 *         ...
 *     }
 *
 * Subject-associated:
 *
 *     schedule computation {
 *         ...
 *     }
 *
 * The subject is an expression so the language does not need a closed list of
 * schedulable entities.
 *
 * ============================================================================
 */

executionSchedule
    : SCHEDULE executionScheduleSubject? executionScheduleBody executionScheduleTerminator?
    ;


/*
 * ============================================================================
 * 2. SCHEDULE SUBJECT
 * ============================================================================
 *
 * The subject may be:
 *
 *     a function reference;
 *     a call;
 *     a pipeline;
 *     a computation;
 *     a task;
 *     a quantum computation;
 *     a classical computation;
 *     a hybrid computation;
 *     a distributed computation;
 *     an accelerator computation;
 *     another semantic execution object.
 *
 * Semantic analysis decides whether it is schedulable.
 * ============================================================================
 */

executionScheduleSubject
    : expression
    ;


/*
 * ============================================================================
 * 3. SCHEDULE BODY
 * ============================================================================
 *
 * Arbitrary entry count.
 *
 * No fixed schedule-property count exists.
 * ============================================================================
 */

executionScheduleBody
    : LBRACE executionScheduleEntry* RBRACE
    ;


/*
 * ============================================================================
 * 4. SCHEDULE ENTRY
 * ============================================================================
 *
 * The first alternatives provide stable semantic categories for the common
 * scheduling intents.
 *
 * The final property alternative permits extensible scheduling concepts.
 * ============================================================================
 */

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
    | executionScheduleRequirement
    | executionScheduleConstraint
    | executionSchedulePreference
    | executionScheduleHint
    | executionScheduleProperty
    ;


/*
 * ============================================================================
 * 5. COMMON KEY/VALUE FORM
 * ============================================================================
 *
 * Scheduling properties deliberately use:
 *
 *     name : expression
 *
 * rather than requiring every future scheduling concept to become a lexer
 * keyword.
 *
 * This also makes dialect and future-extension integration possible.
 * ============================================================================
 */

executionScheduleNamedValue
    : qualifiedName COLON executionScheduleValue executionScheduleEntryTerminator
    ;


/*
 * ============================================================================
 * 6. POLICY
 * ============================================================================
 *
 * Examples:
 *
 *     policy: asap;
 *     policy: critical_path;
 *     policy: custom::policy;
 *
 * The policy name/value is semantic data.
 *
 * The grammar does NOT enumerate scheduler algorithms.
 * ============================================================================
 */

executionSchedulePolicy
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 7. OBJECTIVE
 * ============================================================================
 *
 * Examples:
 *
 *     objective: latency;
 *     objective: throughput;
 *     objective: energy;
 *     objective: fidelity;
 *
 * Semantic analysis validates the objective.
 * ============================================================================
 */

executionScheduleObjective
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 8. ORDER
 * ============================================================================
 *
 * Ordering intent is represented by an expression.
 *
 * This permits future dependency/order representations without introducing a
 * second dependency language.
 * ============================================================================
 */

executionScheduleOrder
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 9. DEPENDENCY
 * ============================================================================
 *
 * Examples:
 *
 *     dependency: dependency_graph;
 *
 *     dependency: depends_on(a, b);
 *
 *     dependency: ordered(stage_a, stage_b);
 *
 * The actual dependency graph is constructed downstream.
 * ============================================================================
 */

executionScheduleDependency
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 10. TIMING
 * ============================================================================
 *
 * Examples:
 *
 *     timing: timing_window;
 *     timing: bounded;
 *     timing: duration;
 *
 * No physical clock resolution is established here.
 * ============================================================================
 */

executionScheduleTiming
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 11. ALIGNMENT
 * ============================================================================
 *
 * Alignment is semantic intent.
 *
 * Physical alignment is resolved by downstream target-aware compilation.
 * ============================================================================
 */

executionScheduleAlignment
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 12. PRIORITY
 * ============================================================================
 */

executionSchedulePriority
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 13. RESOURCE
 * ============================================================================
 *
 * Resource values remain expressions.
 *
 * No resource is allocated by parsing.
 * ============================================================================
 */

executionScheduleResource
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 14. CONCURRENCY
 * ============================================================================
 *
 * Examples:
 *
 *     concurrency: available;
 *     concurrency: requested_parallelism;
 *     concurrency: workload_parallelism;
 *
 * A numeric value is a program-level semantic constraint if the surrounding
 * semantic model defines it that way. It is never a compiler-wide maximum.
 * ============================================================================
 */

executionScheduleConcurrency
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 15. LATENCY
 * ============================================================================
 */

executionScheduleLatency
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 16. DEADLINE
 * ============================================================================
 */

executionScheduleDeadline
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 17. REQUIREMENT
 * ============================================================================
 *
 * Requirement syntax is structurally separated from preference/hint syntax.
 *
 * The value remains an expression so the canonical requirement semantics can
 * evolve independently.
 * ============================================================================
 */

executionScheduleRequirement
    : REQUIRES executionScheduleValue executionScheduleEntryTerminator
    ;


/*
 * ============================================================================
 * 18. CONSTRAINT
 * ============================================================================
 *
 * `constraint:` remains an extensible named constraint property rather than a
 * parser implementation of constraint solving.
 *
 * Example:
 *
 *     constraint: no_overlap;
 *
 * ============================================================================
 */

executionScheduleConstraint
    : executionScheduleNamedValue
    ;


/*
 * ============================================================================
 * 19. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * The semantic layer MUST NOT promote them to mandatory constraints silently.
 * ============================================================================
 */

executionSchedulePreference
    : PREFER executionScheduleValue executionScheduleEntryTerminator
    ;


/*
 * ============================================================================
 * 20. HINT
 * ============================================================================
 *
 * Hints are weaker than requirements and constraints.
 * ============================================================================
 */

executionScheduleHint
    : HINT executionScheduleValue executionScheduleEntryTerminator
    ;


/*
 * ============================================================================
 * 21. EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * This is the generic extension mechanism.
 *
 * Examples:
 *
 *     custom::scheduler: custom::policy;
 *
 *     quantum::timing: symbolic_window;
 *
 *     distributed::ordering: causal;
 *
 *     future::property: value;
 *
 * Property meaning is resolved semantically.
 * ============================================================================
 */

executionScheduleProperty
    : qualifiedName executionSchedulePropertyAssignment executionScheduleEntryTerminator
    ;


/*
 * ============================================================================
 * 22. PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * Both `:` and `=` are supported for property assignment.
 *
 * Comparison remains part of the canonical expression grammar.
 * ============================================================================
 */

executionSchedulePropertyAssignment
    : COLON executionScheduleValue
    | ASSIGN executionScheduleValue
    ;


/*
 * ============================================================================
 * 23. GENERIC SCHEDULE VALUE
 * ============================================================================
 *
 * Canonical expression syntax is reused.
 *
 * This allows values to be:
 *
 *     literals;
 *     identifiers;
 *     qualified names;
 *     calls;
 *     arithmetic;
 *     comparisons;
 *     symbolic quantities;
 *     resource expressions;
 *     capability expressions;
 *     temporal expressions;
 *     future expression forms.
 *
 * No second scheduling value language is created.
 * ============================================================================
 */

executionScheduleValue
    : expression
    | executionSchedulePropertyBlock
    ;


/*
 * ============================================================================
 * 24. NESTED PROPERTY BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     timing: {
 *         start: earliest;
 *         finish: bounded;
 *     };
 *
 * Nested property depth is not artificially limited.
 * ============================================================================
 */

executionSchedulePropertyBlock
    : LBRACE executionSchedulePropertyEntry* RBRACE
    ;


executionSchedulePropertyEntry
    : qualifiedName executionSchedulePropertyAssignment executionScheduleEntryTerminator
    ;


/*
 * ============================================================================
 * 25. ENTRY TERMINATOR
 * ============================================================================
 *
 * Semicolon is the canonical statement/property terminator.
 *
 * Comma is NOT accepted here.
 *
 * This corrects the previous grammar's unnecessary ambiguity between:
 *
 *     property lists
 *
 * and:
 *
 *     expression lists.
 *
 * Generated/configuration serialization should use the surrounding canonical
 * language syntax rather than inventing a second comma-separated scheduling
 * statement language.
 * ============================================================================
 */

executionScheduleEntryTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 26. OUTER TERMINATOR
 * ============================================================================
 *
 * The enclosing statement/declaration composition may own the final
 * terminator, so it remains optional here.
 * ============================================================================
 */

executionScheduleTerminator
    : SEMICOLON
    ;