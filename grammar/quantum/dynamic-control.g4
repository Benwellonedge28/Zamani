/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/dynamic-control.g4
 *
 * Grammar:
 *     QuantumDynamicControl
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM DYNAMIC-CONTROL SYNTAX COMPONENT
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical owner for QUANTUM-SPECIFIC DYNAMIC CONTROL.
 *
 * Dynamic control expresses quantum computation whose later execution depends
 * on runtime information.
 *
 * Typical dependency:
 *
 *     quantum execution
 *          |
 *          v
 *     measurement / classical computation
 *          |
 *          v
 *     runtime predicate
 *          |
 *          v
 *     dynamic quantum control
 *          |
 *          v
 *     quantum action
 *
 * The grammar describes the SOURCE-LEVEL CONTROL DEPENDENCY.
 *
 * It does not describe how that dependency is physically implemented.
 *
 *
 * Examples:
 *
 *     if result {
 *         apply X(q);
 *     }
 *
 *     if syndrome == expected {
 *         apply correction(data);
 *     } else {
 *         apply recovery(auxiliary);
 *     }
 *
 *     while not_ready {
 *         apply probe(q);
 *     }
 *
 *     if result {
 *         when enabled
 *             apply correction(q);
 *     }
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file sits between:
 *
 *     general expressions
 *             |
 *             v
 *     quantum dynamic-control syntax
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic analysis
 *             |
 *             +--------------------+
 *             |                    |
 *             v                    v
 *     classical semantics    quantum semantics
 *             |                    |
 *             v                    v
 *     classical IR             quantum::ir
 *             |                    |
 *             +---------+----------+
 *                       |
 *                       v
 *                  optimization
 *                       |
 *                       v
 *                  routing
 *                       |
 *                       v
 *                  scheduling
 *                       |
 *                       v
 *                  QEC / ZQN
 *                       |
 *                       v
 *                   resilience
 *                       |
 *                       v
 *                       HAL
 *                       |
 *                       v
 *                 target/runtime
 *
 * This grammar MUST NOT bypass the AST, semantic, or canonical IR layers.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - dynamicQuantumControl
 *     - dynamicQuantumConditionalControl
 *     - dynamicQuantumLoopControl
 *     - dynamicQuantumElseBranch
 *     - dynamicQuantumControlBody
 *     - dynamicQuantumAction
 *
 * This establishes the syntax boundary for:
 *
 *     runtime condition -> quantum execution
 *
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - literals;
 *     - expressions;
 *     - expression precedence;
 *     - general if/else control flow;
 *     - general loops;
 *     - general blocks;
 *     - quantum operation syntax;
 *     - quantum measurement syntax;
 *     - reset syntax;
 *     - barrier syntax;
 *     - classical feed-forward syntax;
 *     - qubit declarations;
 *     - quantum registers;
 *     - controls/modifiers;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - resource allocation;
 *     - capability discovery;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware;
 *     - calibration;
 *     - HAL;
 *     - runtime implementation;
 *     - canonical IR.
 *
 * Existing specialized grammars remain the owners of those constructs.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical owner for each source production.
 *
 * In particular, this file MUST NOT redefine:
 *
 *     quantumOperationStatement
 *     quantumMeasurementStatement
 *     quantumResetOperation
 *     quantumBarrierStatement
 *     quantumClassicalFeedForwardStatement
 *     expression
 *     block
 *     conditionalStatement
 *     loopStatement
 *
 * Those constructs are imported from their canonical owners where required.
 *
 * The existing dynamic-circuits.g4 and mid-circuit-control.g4 files must
 * converge on these productions rather than creating competing definitions.
 *
 * ============================================================================
 * CANONICAL LEXICAL CONTRACT
 * ============================================================================
 *
 * The parser consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Canonical tokens used here are:
 *
 *     IF
 *     ELSE
 *     WHILE
 *     LBRACE
 *     RBRACE
 *
 * No new lexical token is introduced by this grammar.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     K_IF
 *     K_ELSE
 *     K_WHILE
 *     DYNAMIC_IF
 *     DYNAMIC_WHILE
 *     FEEDBACK
 *     FEEDFORWARD
 *     MID_CIRCUIT_TOKEN
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This file imports only grammars whose productions it actually consumes.
 *
 * Expressions:
 *
 *     expression
 *
 * Quantum actions:
 *
 *     QuantumOperations
 *     QuantumMeasurement
 *     QuantumReset
 *     QuantumBarriers
 *     QuantumClassicalFeedForward
 *
 * The imported grammars remain responsible for their own syntax.
 *
 * No lexer grammar is imported directly.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Dynamic-circuits.g4 already contains an older broad dynamic-circuit design.
 *
 * This file provides the production-oriented canonical boundary that separates
 * quantum dynamic-control syntax from general dynamic language control flow.
 *
 * The migration direction is:
 *
 *     existing dynamic-circuits.g4
 *                 |
 *                 v
 *     canonical dynamic-control syntax
 *                 |
 *                 v
 *     this file
 *
 * Existing dynamic-circuit declarations and compatibility constructs may
 * remain in their existing file while they are migrated, but new canonical
 * dynamic quantum control syntax MUST enter through this file.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * There are three related but different concepts:
 *
 * 1. General control flow
 *
 *     if
 *     else
 *     while
 *     for
 *     match
 *
 * This belongs to the general statement/control-flow system.
 *
 *
 * 2. Classical feed-forward
 *
 *     when condition
 *         apply operation(...);
 *
 * This is owned by:
 *
 *     quantum/classical-feedforward.g4
 *
 *
 * 3. Quantum dynamic control
 *
 *     if condition {
 *         quantum actions
 *     }
 *
 *     while condition {
 *         quantum actions
 *     }
 *
 * This file owns the quantum-specific boundary.
 *
 * The same condition may ultimately lower into related semantic control-flow
 * structures, but the source-level ownership remains explicit.
 *
 * ============================================================================
 * OPEN-WORLD CONDITION MODEL
 * ============================================================================
 *
 * Conditions are ordinary Zamani expressions.
 *
 * Examples:
 *
 *     if result {
 *         ...
 *     }
 *
 *     if result == 1 {
 *         ...
 *     }
 *
 *     if syndrome == expected {
 *         ...
 *     }
 *
 *     if ready && enabled {
 *         ...
 *     }
 *
 *     if predicate(measurement, state) {
 *         ...
 *     }
 *
 *     while !done {
 *         ...
 *     }
 *
 * The grammar does not define another boolean-expression language.
 *
 * Semantic analysis determines:
 *
 *     - expression type;
 *     - value availability;
 *     - measurement provenance;
 *     - dependency ordering;
 *     - side effects;
 *     - determinism;
 *     - runtime availability;
 *     - legality as dynamic quantum control.
 *
 * ============================================================================
 * MEASUREMENT DEPENDENCY
 * ============================================================================
 *
 * This grammar does not require a condition to contain a special measurement
 * token.
 *
 * For example:
 *
 *     measure q -> result;
 *
 *     let syndrome = decode(result);
 *
 *     if syndrome == expected {
 *         apply correction(data);
 *     }
 *
 * is structurally represented using the ordinary expression grammar.
 *
 * The semantic layer must determine whether `syndrome` is available and
 * whether its dependency on the measurement is legal.
 *
 * No special measurement-result expression grammar is introduced here.
 *
 * ============================================================================
 * DYNAMIC BODY MODEL
 * ============================================================================
 *
 * A dynamic quantum body contains one or more dynamic quantum actions.
 *
 * Actions may include:
 *
 *     quantum operations;
 *     measurements;
 *     reset;
 *     barriers;
 *     classical feed-forward;
 *     nested dynamic control.
 *
 * Their detailed syntax remains owned by their respective grammars.
 *
 * This permits arbitrarily nested dynamic structures without introducing a
 * second statement language.
 *
 * ============================================================================
 * RECURSION / SCALABILITY
 * ============================================================================
 *
 * Dynamic control is recursively composable:
 *
 *     if a {
 *         if b {
 *             apply X(q);
 *         }
 *     }
 *
 *     while a {
 *         if b {
 *             apply operation(q);
 *         }
 *     }
 *
 * There is no grammar-level maximum for:
 *
 *     nesting depth;
 *     number of branches;
 *     number of actions;
 *     number of loops;
 *     number of conditions;
 *     number of measurements;
 *     number of operations;
 *     number of quantum resources.
 *
 * Actual parser/compiler/runtime stack and memory limits remain implementation
 * and resource-policy concerns rather than language semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Dynamic control describes:
 *
 *     WHAT computation is conditionally or repeatedly performed.
 *
 * It does NOT prescribe:
 *
 *     WHICH CPU;
 *     WHICH GPU;
 *     WHICH FPGA;
 *     WHICH QPU;
 *     WHICH physical qubit;
 *     WHICH control processor;
 *     WHICH memory bank;
 *     WHICH network node;
 *     WHICH topology;
 *     WHICH instruction set;
 *     WHICH pulse;
 *     WHICH calibration;
 *     WHICH scheduler;
 *     WHICH router;
 *     WHICH vendor implementation.
 *
 * The compiler may realize the same source semantics through:
 *
 *     native dynamic-circuit support;
 *     classical control electronics;
 *     host-side control;
 *     FPGA control;
 *     accelerator control;
 *     deferred execution;
 *     program transformation;
 *     simulation;
 *     distributed execution;
 *     another future execution substrate.
 *
 * ============================================================================
 * NO ARTIFICIAL HARDWARE LIMITS
 * ============================================================================
 *
 * This file MUST contain no universal machine ceilings.
 *
 * Forbidden examples include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_DYNAMIC_BRANCHES
 *     MAX_DYNAMIC_DEPTH
 *     MAX_DYNAMIC_OPERATIONS
 *
 * Program values such as:
 *
 *     let depth = 1024;
 *
 * remain valid program semantics.
 *
 * The prohibition concerns compiler/grammar-imposed universal ceilings.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * No physical resource may be selected by this grammar.
 *
 * Expressions such as:
 *
 *     physical_qubit(17)
 *
 * may exist elsewhere only under an explicit target-specific/physical-resource
 * contract.
 *
 * They are not introduced here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST remains domain-neutral.
 *
 * This grammar requires enough source structure for an AST representation
 * equivalent in meaning to:
 *
 *     ConditionalExecution {
 *         condition,
 *         then_body,
 *         else_body?,
 *         source_span
 *     }
 *
 * and:
 *
 *     DynamicLoop {
 *         condition,
 *         body,
 *         source_span
 *     }
 *
 * The AST MUST NOT require a hardware-specific node such as:
 *
 *     QPUConditional
 *     HardwareBranch
 *     PhysicalFeedbackLoop
 *
 * Quantum semantics are attached during semantic analysis.
 *
 * The exact Rust AST type is owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar does not define Rust AST structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the condition is valid;
 *     - whether the condition is classically evaluable;
 *     - whether referenced values are initialized;
 *     - whether measurement-derived values are available;
 *     - whether data dependencies dominate the controlled action;
 *     - whether the action is dynamically executable;
 *     - whether nested dynamic control is legal;
 *     - whether loop termination semantics are valid;
 *     - whether side effects are legal;
 *     - whether the target requires dynamic-control capability;
 *     - whether transformation can preserve source semantics.
 *
 * Semantic analysis must distinguish:
 *
 *     syntax validity;
 *     semantic validity;
 *     resource feasibility;
 *     target capability;
 *     runtime availability.
 *
 * A target lacking dynamic-control capability must not make the source
 * syntactically invalid.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE CONTRACT
 * ============================================================================
 *
 * Capability and resource checks occur downstream.
 *
 * Possible semantic requirements include concepts such as:
 *
 *     quantum.dynamic_control
 *     quantum.mid_circuit_measurement
 *     quantum.classical_feedforward
 *
 * These are capability semantics, not lexer keywords.
 *
 * The grammar must not assume that a target has any particular capability.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * Dynamic quantum control lowers through the canonical semantic pipeline:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * Conditional and loop constructs must reuse the existing canonical IR
 * representation for dynamic/conditional execution where available.
 *
 * If the canonical IR does not yet expose the required construct, that is an
 * IR implementation task; it must NOT be solved by introducing:
 *
 *     dynamic_control::ir
 *     quantum_dynamic::ir
 *     feedforward::ir
 *
 * or another competing quantum IR.
 *
 * ============================================================================
 * OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimization may transform dynamic control only when semantic equivalence
 * is preserved.
 *
 * Permitted downstream transformations may include:
 *
 *     branch simplification;
 *     dead-branch elimination;
 *     condition propagation;
 *     operation fusion;
 *     loop transformation;
 *     deferred measurement;
 *     branch predication;
 *     dynamic-circuit lowering;
 *     host/device partitioning.
 *
 * Such transformations do not belong in this grammar.
 *
 * ============================================================================
 * ROUTING / SCHEDULING CONTRACT
 * ============================================================================
 *
 * Routing determines physical realization.
 *
 * Scheduling determines execution ordering/timing.
 *
 * Dynamic control creates a dependency boundary that downstream scheduling
 * must preserve.
 *
 * This grammar does not encode:
 *
 *     gate duration;
 *     measurement duration;
 *     feedback latency;
 *     clock period;
 *     scheduling slot;
 *     coupling map;
 *     routing path.
 *
 * ============================================================================
 * QEC / ZQN / RESILIENCE CONTRACT
 * ============================================================================
 *
 * QEC owns error-correction implementation.
 *
 * ZQN owns quantum fault/noise semantics.
 *
 * Resilience owns execution adaptation/recovery policy.
 *
 * Dynamic control may consume values produced by these systems only through
 * their canonical semantic interfaces.
 *
 * This grammar does not implement:
 *
 *     syndrome decoding;
 *     recovery;
 *     noise models;
 *     retry;
 *     reroute;
 *     reschedule;
 *     backend failover.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should report structural failures such as:
 *
 *     if
 *     if condition
 *     if condition {
 *     if condition { }
 *     else
 *     while
 *     while condition
 *     while condition {
 *
 * where the required structure is incomplete.
 *
 * Semantic diagnostics should separately report:
 *
 *     unknown condition value;
 *     invalid condition type;
 *     unavailable measurement result;
 *     illegal dynamic dependency;
 *     unsupported dynamic operation;
 *     insufficient target capability;
 *     insufficient resources;
 *     invalid loop semantics.
 *
 * Resource/capability failures must not be reported as syntax errors.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no executable actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no environment inspection;
 *     no hardware discovery;
 *     no secret access;
 *     no runtime execution.
 *
 * Expressions are parsed, never evaluated, by this grammar.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This grammar deliberately avoids:
 *
 *     left-recursive statement-list definitions;
 *     duplicated expression grammars;
 *     duplicated operation grammars;
 *     semantic predicates;
 *     embedded actions;
 *     hardware-dependent decisions.
 *
 * Lists use ANTLR repetition constructs.
 *
 * Nested dynamic control is structurally recursive, as required by the
 * language semantics, but has no artificial language-defined depth.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     canonical lexer;
 *     grammar version;
 *     imported grammar versions;
 *     dialect configuration;
 *
 * parsing must produce equivalent structure and source spans.
 *
 * Parsing MUST NOT depend on:
 *
 *     time;
 *     randomness;
 *     hardware;
 *     filesystem state;
 *     network state;
 *     environment state;
 *     runtime state.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive and target-independent.
 *
 * Existing valid source must retain its established meaning.
 *
 * Existing dynamic-circuit syntax in:
 *
 *     grammar/quantum/dynamic-circuits.g4
 *
 * must be migrated/delegated to these canonical rules rather than duplicated.
 *
 * Existing mid-circuit-control syntax in:
 *
 *     grammar/quantum/mid-circuit-control.g4
 *
 * must either:
 *
 *     - delegate to these rules;
 *     - remain a compatibility-only wrapper;
 *     - or be explicitly deprecated through the compatibility system.
 *
 * `classical-feedforward.g4` remains the canonical owner of `when`.
 *
 * `operations.g4` remains the canonical owner of operation invocation.
 *
 * No existing major filename needs to be renamed.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required test families:
 *
 *     grammar/tests/quantum/dynamic-control/
 *         positive/
 *         negative/
 *         boundary/
 *         scalability/
 *         determinism/
 *         compatibility/
 *         integration/
 *
 * Positive structural cases:
 *
 *     if result {
 *         apply X(q);
 *     }
 *
 *     if result == 1 {
 *         apply X(q);
 *     } else {
 *         apply Z(q);
 *     }
 *
 *     while ready {
 *         apply operation(q);
 *     }
 *
 *     if syndrome == expected {
 *         measure q -> result;
 *     }
 *
 *     if result {
 *         reset q;
 *     }
 *
 *     if result {
 *         barrier q;
 *     }
 *
 *     if result {
 *         when enabled
 *             apply correction(q);
 *     }
 *
 *     if outer {
 *         while inner {
 *             if nested {
 *                 apply operation(target);
 *             }
 *         }
 *     }
 *
 * Negative structural cases:
 *
 *     if
 *
 *     if condition
 *
 *     if condition {
 *
 *     else {
 *     }
 *
 *     while
 *
 *     while condition
 *
 *     while condition {
 *
 *     if condition {
 *         apply
 *     }
 *
 *     if condition {
 *         measure
 *     }
 *
 * Boundary/scalability cases:
 *
 *     one dynamic control;
 *     many actions;
 *     many nested dynamic controls;
 *     many conditions;
 *     many loops;
 *     large expressions;
 *     large operation target lists;
 *     large source units.
 *
 * No test may define a universal maximum.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It contains no:
 *
 *     physical_qubit_0
 *     physical_qubit_1
 *     qpu0
 *     gpu0
 *     cpu0
 *     device0
 *
 * as grammar-level resources.
 *
 * ============================================================================
 * RUST / SAFETY CONTRACT
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     no Rust actions;
 *     no Rust predicates;
 *     no unsafe code;
 *     no runtime implementation.
 *
 * The generated/consuming implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one canonical parser-grammar declaration.
 *     [x] It uses ZamaniLexer.
 *     [x] It uses canonical IF/ELSE/WHILE tokens.
 *     [x] It owns quantum-specific dynamic control only.
 *     [x] It does not redefine general expression syntax.
 *     [x] It does not redefine operation syntax.
 *     [x] It does not redefine measurement syntax.
 *     [x] It does not redefine reset syntax.
 *     [x] It does not redefine barrier syntax.
 *     [x] It does not redefine classical feed-forward syntax.
 *     [x] It supports nested dynamic control.
 *     [x] It has no artificial cardinality limit.
 *     [x] It contains no hardware assumptions.
 *     [x] It contains no vendor gate enumeration.
 *     [x] It contains no semantic actions.
 *     [x] It contains no unsafe Rust.
 *     [x] It has an explicit AST contract.
 *     [x] It has an explicit semantic contract.
 *     [x] It has an explicit IR contract.
 *     [x] It has an explicit compiler/runtime boundary.
 *     [x] It has an explicit QEC/ZQN boundary.
 *     [x] It has an explicit routing/scheduling boundary.
 *     [x] It has positive/negative/boundary/scalability test contracts.
 *     [x] It has deterministic parsing requirements.
 *     [x] It has a hard-coding audit.
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar QuantumDynamicControl;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    QuantumOperations,
    QuantumMeasurement,
    QuantumReset,
    QuantumBarriers,
    QuantumClassicalFeedForward
;


/*
 * ============================================================================
 * 1. DYNAMIC QUANTUM CONTROL ENTRY
 * ============================================================================
 *
 * This is the single exported entry point consumed by quantum.g4 and the
 * canonical parser composition hierarchy.
 */

dynamicQuantumControl
    : dynamicQuantumConditionalControl
    | dynamicQuantumLoopControl
    ;


/*
 * ============================================================================
 * 2. CONDITIONAL DYNAMIC CONTROL
 * ============================================================================
 *
 * Canonical form:
 *
 *     if condition {
 *         quantum action
 *     }
 *
 * Optional else:
 *
 *     if condition {
 *         quantum action
 *     } else {
 *         quantum action
 *     }
 *
 * Braced bodies are intentional. They avoid ambiguity with general statement
 * control flow and establish a stable quantum dynamic-control boundary.
 */

dynamicQuantumConditionalControl
    : IF
      expression
      dynamicQuantumControlBody
      (
          ELSE
          dynamicQuantumElseBranch
      )?
    ;


/*
 * ============================================================================
 * 3. ELSE BRANCH
 * ============================================================================
 *
 * A branch may be:
 *
 *     a quantum body;
 *     another dynamic conditional.
 *
 * This permits:
 *
 *     else if ...
 *
 * without introducing a separate conditional grammar.
 */

dynamicQuantumElseBranch
    : dynamicQuantumControlBody
    | dynamicQuantumConditionalControl
    ;


/*
 * ============================================================================
 * 4. DYNAMIC LOOP
 * ============================================================================
 *
 * Canonical form:
 *
 *     while condition {
 *         quantum action
 *     }
 *
 * No iteration count is encoded by the grammar.
 */

dynamicQuantumLoopControl
    : WHILE
      expression
      dynamicQuantumControlBody
    ;


/*
 * ============================================================================
 * 5. DYNAMIC QUANTUM BODY
 * ============================================================================
 *
 * The body requires at least one action.
 *
 * This prevents an empty quantum dynamic-control construct from being
 * silently accepted as executable quantum behavior.
 */

dynamicQuantumControlBody
    : LBRACE
      dynamicQuantumAction+
      RBRACE
    ;


/*
 * ============================================================================
 * 6. DYNAMIC QUANTUM ACTION
 * ============================================================================
 *
 * Each specialized grammar remains the owner of its syntax.
 *
 * This rule merely composes already-owned actions into a dynamic-control
 * context.
 */

dynamicQuantumAction
    : quantumOperationStatement
    | quantumMeasurementStatement
    | quantumResetOperation
    | quantumBarrierStatement
    | quantumClassicalFeedForwardStatement
    | dynamicQuantumControl
    ;