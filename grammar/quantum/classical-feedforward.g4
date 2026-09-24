/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/classical-feedforward.g4
 *
 * Grammar:
 *     QuantumClassicalFeedForward
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM CLASSICAL-FEED-FORWARD GRAMMAR
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
 * This file is the SINGLE SYNTAX OWNER for explicit quantum/classical
 * feed-forward relationships.
 *
 * Classical feed-forward represents the source-level dependency:
 *
 *     quantum execution
 *          |
 *          v
 *     classical value / measurement result
 *          |
 *          v
 *     classical predicate or computed control value
 *          |
 *          v
 *     later quantum operation
 *
 * The grammar expresses that dependency.
 *
 * It does NOT specify how the dependency is physically realized.
 *
 *
 * Examples:
 *
 *     measure q -> result;
 *
 *     when result
 *         apply X(q);
 *
 *     when result == 1
 *         apply X(q);
 *
 *     when syndrome == expected
 *         apply correction(data);
 *
 *     when result && enabled {
 *         apply correction_a(data);
 *         apply correction_b(data);
 *     }
 *
 * The condition is an ordinary Zamani expression.
 *
 * Measurement-derived status is established by semantic analysis rather than
 * by a second expression language.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Classical feed-forward is a DATA/CONTROL DEPENDENCY.
 *
 * It is NOT:
 *
 *     a physical feedback wire;
 *     a fixed latency;
 *     a QPU-specific instruction;
 *     a host/device API;
 *     a pulse-level operation;
 *     a scheduler command;
 *     a routing command;
 *     a QEC decoder;
 *     a ZQN noise operation.
 *
 * The same source dependency may be realized by:
 *
 *     dynamic-circuit hardware;
 *     classical control electronics;
 *     host-side execution;
 *     accelerator control;
 *     FPGA control logic;
 *     compiler transformation;
 *     deferred execution;
 *     simulation;
 *     distributed execution;
 *     another future execution substrate.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantumClassicalFeedForwardStatement
 *     - quantumClassicalFeedForwardCondition
 *     - quantumClassicalFeedForwardBody
 *     - quantumClassicalFeedForwardBlock
 *     - quantumClassicalFeedForwardOperation
 *     - quantumClassicalFeedForwardExtension
 *
 * This file therefore owns the syntax connecting:
 *
 *     classical condition
 *             ->
 *     quantum action
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - keywords;
 *     - identifiers;
 *     - literals;
 *     - expressions;
 *     - expression precedence;
 *     - general if/else syntax;
 *     - loops;
 *     - blocks generally;
 *     - quantum measurements;
 *     - quantum operations;
 *     - reset;
 *     - barriers;
 *     - controls;
 *     - adjoints;
 *     - qubit declarations;
 *     - classical declarations;
 *     - classical types;
 *     - resource requirements;
 *     - capability declarations;
 *     - hardware;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime;
 *     - canonical IR.
 *
 * The specialized grammars remain the owners of their own constructs.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical owner for feed-forward syntax.
 *
 * In particular, no other quantum grammar may independently define another
 * production equivalent to:
 *
 *     WHEN expression quantumOperationStatement
 *
 * once this grammar has been integrated.
 *
 * Existing compatibility rules may delegate to this grammar, but must not
 * redefine its syntax.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     canonical lexer
 *           |
 *           v
 *     core names / expressions
 *           |
 *           v
 *     QuantumClassicalFeedForward
 *           |
 *           +-------------------+
 *           |                   |
 *           v                   v
 *     quantum operations   quantum statements
 *           |                   |
 *           +---------+---------+
 *                     |
 *                     v
 *              frontend AST
 *                     |
 *                     v
 *             semantic analysis
 *                     |
 *          +----------+----------+
 *          |                     |
 *          v                     v
 *     classical semantics   quantum semantics
 *          |                     |
 *          v                     v
 *     classical IR          quantum::ir
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *                optimization
 *                     |
 *                     v
 *                 routing
 *                     |
 *                     v
 *                scheduling
 *                     |
 *                     v
 *                 QEC / ZQN
 *                     |
 *                     v
 *                resilience
 *                     |
 *                     v
 *                hardware HAL
 *                     |
 *                     v
 *             target realization
 *                     |
 *                     v
 *                  runtime
 *
 * This grammar MUST NOT reverse this dependency direction.
 *
 * ============================================================================
 * CANONICAL LEXICAL CONTRACT
 * ============================================================================
 *
 * The production parser consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The canonical lexical spelling used by this grammar is:
 *
 *     when
 *
 * represented by the canonical:
 *
 *     WHEN
 *
 * token.
 *
 * The quantum operation keyword remains:
 *
 *     apply
 *
 * represented by:
 *
 *     APPLY
 *
 * when consumed indirectly by quantum operation grammar.
 *
 * IMPORTANT:
 *
 * This file MUST NOT introduce:
 *
 *     K_WHEN
 *     K_APPLY
 *     FEEDBACK
 *     FEEDFORWARD
 *     FEEDBACK_TOKEN
 *     FEEDFORWARD_TOKEN
 *
 * merely to implement this feature.
 *
 * The current lexical architecture already provides the required structural
 * vocabulary.
 *
 * "feed-forward" is a semantic category, not necessarily a new keyword.
 *
 * ============================================================================
 * WHY `WHEN` IS USED
 * ============================================================================
 *
 * Zamani already reserves:
 *
 *     WHEN : 'when'
 *
 * in the canonical keyword vocabulary.
 *
 * Reusing it prevents unnecessary lexical expansion.
 *
 * The source form:
 *
 *     when condition
 *         apply operation(target);
 *
 * is therefore a syntactic quantum feed-forward construct because the
 * following quantum action is explicitly owned by this grammar boundary.
 *
 * The meaning of the condition is determined downstream.
 *
 * ============================================================================
 * OPEN-WORLD CONDITION MODEL
 * ============================================================================
 *
 * A feed-forward condition is:
 *
 *     expression
 *
 * and not a special "measurement bit" grammar.
 *
 * Therefore all of the following may be structurally represented:
 *
 *     when result
 *
 *     when result == 1
 *
 *     when result[0]
 *
 *     when syndrome == expected
 *
 *     when parity == correction_needed
 *
 *     when ready && valid
 *
 *     when classical_state.value > threshold
 *
 *     when f(measurement_result)
 *
 *     when predicate(result, state)
 *
 * Semantic analysis determines whether the expression is legally usable as
 * feed-forward control.
 *
 * ============================================================================
 * MEASUREMENT DEPENDENCY
 * ============================================================================
 *
 * This grammar does NOT require the condition to contain a special token such
 * as:
 *
 *     measurement_result
 *
 * nor does it require a measurement to occur immediately before the
 * feed-forward statement.
 *
 * For example:
 *
 *     measure q -> result;
 *
 *     classical_value = process(result);
 *
 *     when classical_value == expected
 *         apply correction(data);
 *
 * is structurally valid.
 *
 * Semantic analysis must determine:
 *
 *     - whether result exists;
 *     - whether result is initialized;
 *     - whether it originated from a measurement;
 *     - whether classical_value depends on that result;
 *     - whether the value is available at this point;
 *     - whether the expression has an appropriate type;
 *     - whether the dependency is legal for quantum control.
 *
 * ============================================================================
 * NO DUPLICATE EXPRESSION LANGUAGE
 * ============================================================================
 *
 * This grammar delegates conditions to:
 *
 *     expression
 *
 * It does NOT define:
 *
 *     feedForwardBooleanExpression
 *     feedForwardComparison
 *     feedForwardLogicalExpression
 *     feedForwardArithmeticExpression
 *
 * Such duplication would create a second expression language.
 *
 * ============================================================================
 * QUANTUM ACTION OWNERSHIP
 * ============================================================================
 *
 * The feed-forward body delegates the actual quantum operation to:
 *
 *     quantumOperationStatement
 *
 * owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * Therefore this file does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     vendor.operation
 *     custom_operation
 *     future_operation
 *
 * All operation names remain open semantic names.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Feed-forward MUST NOT identify:
 *
 *     physical qubits;
 *     CPU cores;
 *     GPU devices;
 *     FPGA regions;
 *     QPU identifiers;
 *     classical control processors;
 *     memory banks;
 *     network nodes;
 *     hardware addresses;
 *     vendor-specific control channels.
 *
 * The condition describes a dependency.
 *
 * Physical realization is a downstream compiler/runtime concern.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A program written with this grammar describes:
 *
 *     WHAT should happen when a classical condition becomes satisfied.
 *
 * It does not prescribe:
 *
 *     WHERE it happens;
 *     WHICH processor evaluates the condition;
 *     WHICH device executes the operation;
 *     WHICH physical qubits are involved;
 *     HOW the result is transported;
 *     HOW quickly feedback is implemented;
 *     WHICH hardware instruction is generated.
 *
 * Consequently the same program can be lowered to:
 *
 *     tiny systems;
 *     embedded controllers;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     simulators;
 *     accelerators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     future computational substrates.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally has NO finite language-level capacity for:
 *
 *     conditions;
 *     feed-forward statements;
 *     nested feed-forward blocks;
 *     operations;
 *     targets;
 *     measurements;
 *     classical values;
 *     quantum resources;
 *     circuit depth;
 *     execution stages;
 *     devices;
 *     nodes;
 *     threads;
 *     memory;
 *     timelines.
 *
 * There are no:
 *
 *     MAX_QUBITS
 *     MAX_CLASSICAL_BITS
 *     MAX_FEEDFORWARD
 *     MAX_FEEDBACK
 *     MAX_BRANCHES
 *     MAX_OPERATIONS
 *     MAX_TARGETS
 *     MAX_MEASUREMENTS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *
 * or equivalent grammar-level limits.
 *
 * Repetition is represented structurally using ANTLR `*`, `+`, and recursive
 * composition where required.
 *
 * "Infinity" therefore means:
 *
 *     no artificial finite limit is introduced by this grammar;
 *
 * not:
 *
 *     physically infinite execution.
 *
 * Actual limits remain resource/compiler/runtime/backend concerns.
 *
 * ============================================================================
 * SYNTAX CONTRACT
 * ============================================================================
 *
 * Canonical single-action form:
 *
 *     when condition
 *         quantum-operation;
 *
 * Canonical block form:
 *
 *     when condition {
 *         quantum-operation;
 *         quantum-operation;
 *     }
 *
 * The body contains quantum operation statements rather than arbitrary
 * statements. This keeps the feature specifically about classical
 * feed-forward into quantum computation and prevents it from becoming a
 * second general-purpose `if` statement.
 *
 * General classical control remains owned by the general statement grammar.
 *
 * ============================================================================
 * ANTLR GRAMMAR
 * ============================================================================
 */

parser grammar QuantumClassicalFeedForward;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. CANONICAL FEED-FORWARD STATEMENT
 * ============================================================================
 *
 * Source-level semantic shape:
 *
 *     WHEN <classical-condition> <quantum-body>
 *
 * ============================================================================
 */

quantumClassicalFeedForwardStatement
    : WHEN
      quantumClassicalFeedForwardCondition
      quantumClassicalFeedForwardBody
    ;


/*
 * ============================================================================
 * 2. CONDITION
 * ============================================================================
 *
 * The condition is the canonical Zamani expression language.
 *
 * ============================================================================
 */

quantumClassicalFeedForwardCondition
    : expression
    ;


/*
 * ============================================================================
 * 3. BODY
 * ============================================================================
 *
 * A feed-forward body is either:
 *
 *     one quantum operation
 *
 * or:
 *
 *     a quantum-operation block.
 *
 * The actual operation grammar remains the owner of operation syntax.
 *
 * ============================================================================
 */

quantumClassicalFeedForwardBody
    : quantumClassicalFeedForwardOperation
    | quantumClassicalFeedForwardBlock
    ;


/*
 * ============================================================================
 * 4. SINGLE QUANTUM ACTION
 * ============================================================================
 *
 * This is deliberately an integration wrapper rather than a new operation
 * grammar.
 *
 * `quantumOperationStatement` remains owned by operations.g4.
 *
 * ============================================================================
 */

quantumClassicalFeedForwardOperation
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * 5. MULTI-ACTION FEED-FORWARD BLOCK
 * ============================================================================
 *
 * The block permits multiple quantum operations to share one classical
 * predicate.
 *
 * Example:
 *
 *     when syndrome == expected {
 *         apply correction_a(data);
 *         apply correction_b(auxiliary);
 *     }
 *
 * No maximum number of operations is encoded.
 *
 * ============================================================================
 */

quantumClassicalFeedForwardBlock
    : LBRACE
      quantumClassicalFeedForwardOperation*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future feed-forward syntax should compose through this boundary rather than
 * modifying the canonical condition language or creating new operation
 * vocabularies.
 *
 * Possible future semantic extensions include:
 *
 *     measurement-derived predicates;
 *     syndrome-dependent operations;
 *     classical computed predicates;
 *     deferred predicates;
 *     distributed classical predicates;
 *     asynchronous result dependencies;
 *     hybrid control regions.
 *
 * Such extensions must preserve the rule:
 *
 *     syntax describes dependency;
 *     semantics determine legality;
 *     compiler determines realization.
 *
 * ============================================================================
 */

quantumClassicalFeedForwardExtension
    : quantumClassicalFeedForwardStatement
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes:
 *
 *     condition
 *       |
 *       v
 *     quantum body
 *
 * Semantic analysis establishes:
 *
 *     whether the condition is classical;
 *     whether the condition is available;
 *     whether it depends on measurement;
 *     whether it may control quantum execution;
 *     whether the controlled operation is valid;
 *     whether the operation has the necessary capability;
 *     whether the required resources exist;
 *     whether the target supports the dependency;
 *     whether transformation/decomposition is required.
 *
 * The grammar MUST NOT perform these checks.
 *
 * ============================================================================
 * MEASUREMENT-TO-FEED-FORWARD CONTRACT
 * ============================================================================
 *
 * A canonical semantic dependency may be:
 *
 *     measure
 *       |
 *       v
 *     classical result
 *       |
 *       v
 *     classical computation
 *       |
 *       v
 *     feed-forward condition
 *       |
 *       v
 *     quantum operation
 *
 * The grammar only represents the final source-level dependency.
 *
 * Measurement syntax remains owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * Measurement-result semantics remain owned by semantic analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve a domain-neutral representation containing:
 *
 *     condition;
 *     body;
 *     source span;
 *     nested source spans;
 *     source ordering;
 *     attached attributes/metadata when supported by the enclosing grammar.
 *
 * Conceptually:
 *
 *     FeedForward {
 *         condition,
 *         body,
 *         source_span
 *     }
 *
 * The exact Rust AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT create:
 *
 *     QuantumFeedForwardNode
 *     PhysicalFeedForwardNode
 *     QPUFeedForwardNode
 *     HardwareFeedbackNode
 *
 * as a competing AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC MODEL CONTRACT
 * ============================================================================
 *
 * Semantic analysis should resolve:
 *
 *     condition
 *         ->
 *     classical dependency expression
 *
 * and:
 *
 *     body
 *         ->
 *     semantic quantum operation(s)
 *
 * The semantic model should preserve the dependency edge:
 *
 *     condition
 *         |
 *         v
 *     quantum operation
 *
 * without assigning physical placement.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * After semantic analysis:
 *
 *     condition
 *          |
 *          v
 *     canonical dependency/control representation
 *          |
 *          v
 *     quantum::ir
 *
 * Quantum operations continue to cross the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * If the repository's canonical cross-domain control/data-flow representation
 * carries the predicate outside quantum::ir, the semantic dependency may be
 * represented there while the quantum operation remains represented through
 * quantum::ir.
 *
 * This grammar must not create:
 *
 *     FeedForwardIR
 *     QuantumFeedForwardIR
 *     DynamicCircuitIR
 *
 * as another IR layer.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may transform the source dependency into:
 *
 *     conditional quantum operations;
 *     dynamic circuit control;
 *     host/device control;
 *     control-electronics operations;
 *     deferred execution;
 *     predication;
 *     branch specialization;
 *     simulation branching;
 *     another semantically equivalent representation.
 *
 * The selected implementation must preserve source semantics.
 *
 * The compiler, not this grammar, determines the realization.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * A feed-forward construct may require capabilities such as:
 *
 *     capability("quantum.dynamic_control")
 *     capability("quantum.mid_circuit_measurement")
 *     capability("quantum.classical_feedforward")
 *
 * if those capabilities exist in the semantic capability registry.
 *
 * This grammar does NOT require any particular capability token.
 *
 * Resource analysis determines whether the target can satisfy the semantic
 * requirements.
 *
 * A program must not be rejected by this grammar merely because a current
 * target lacks a capability.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling owns:
 *
 *     measurement ordering;
 *     dependency ordering;
 *     availability timing;
 *     synchronization;
 *     latency;
 *     overlap;
 *     placement;
 *     execution order;
 *     target-specific timing.
 *
 * This grammar owns none of those properties.
 *
 * In particular, it MUST NOT encode:
 *
 *     feedback_after 10ns
 *     feedback_within 100cycles
 *     measurement_latency 20
 *
 * unless such constructs are independently standardized as portable semantic
 * requirements elsewhere.
 *
 * Even then, the numeric value is program semantics, not a machine maximum.
 *
 * ============================================================================
 * ROUTING CONTRACT
 * ============================================================================
 *
 * Routing determines physical realization.
 *
 * Feed-forward grammar MUST NOT encode:
 *
 *     physical qubit;
 *     physical control line;
 *     coupling edge;
 *     device address;
 *     topology;
 *     control processor.
 *
 * ============================================================================
 * QEC CONTRACT
 * ============================================================================
 *
 * Feed-forward may be used to express source-level correction after a
 * measurement-derived syndrome.
 *
 * Example:
 *
 *     when syndrome == expected {
 *         apply correction(data);
 *     }
 *
 * This grammar does NOT implement:
 *
 *     syndrome extraction;
 *     stabilizer codes;
 *     decoders;
 *     recovery algorithms;
 *     code distance;
 *     logical error rates.
 *
 * QEC remains downstream.
 *
 * ============================================================================
 * ZQN / NOISE CONTRACT
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     measurement noise;
 *     readout error;
 *     leakage;
 *     erasure;
 *     correlated faults;
 *     fault probabilities.
 *
 * ZQN remains the canonical fault/noise semantic subsystem.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * This grammar does not decide:
 *
 *     retry;
 *     recover;
 *     reroute;
 *     reschedule;
 *     switch backend;
 *     degrade;
 *     reject.
 *
 * Resilience consumes downstream execution and fault information.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no external calls;
 *     no environment inspection;
 *     no hardware discovery;
 *     no randomness.
 *
 * Given the same source, lexer vocabulary, imported grammar versions and
 * language version, parsing is deterministic.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural errors such as:
 *
 *     when
 *     when {
 *     when condition
 *     when condition {
 *     when condition apply
 *     when condition apply operation(
 *     when condition apply operation(target
 *
 * Semantic diagnostics should separately identify:
 *
 *     condition is not a valid classical predicate;
 *     condition references an unknown value;
 *     condition references an unavailable measurement result;
 *     condition has an invalid type;
 *     operation is not valid under the condition;
 *     operation requires unavailable capability;
 *     operation requires unavailable resources;
 *     target does not support dynamic control.
 *
 * These are deliberately not parser errors.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     dynamic evaluation;
 *     hardware communication;
 *     runtime calls.
 *
 * User expressions are parsed structurally.
 *
 * No source expression is executed during parsing.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     canonical expression parsing;
 *     ordinary alternatives;
 *     iterative block contents;
 *     no semantic predicates;
 *     no embedded actions.
 *
 * It introduces no artificial source-size ceiling.
 *
 * Implementation resource limits remain compiler/parser policy rather than
 * language semantics.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source syntax in:
 *
 *     grammar/quantum/quantum-classical.g4
 *
 * currently contains feed-forward-like constructs including:
 *
 *     classicalControlledQuantumOperation
 *     conditionalQuantumOperation
 *     measurementControlledQuantumOperation
 *     quantumClassicalFeedback
 *
 * Integration MUST converge those constructs on this file where their
 * semantics are classical-condition -> quantum-action feed-forward.
 *
 * No new syntax is required for compatibility.
 *
 * Existing `when <expression> apply <operation> <targets>;` source should
 * continue to parse after migration, subject to the canonical operation
 * grammar's exact invocation form.
 *
 * ============================================================================
 * REQUIRED INTEGRATION
 * ============================================================================
 *
 * 1. quantum.g4
 *
 * Add this grammar to the quantum composition hierarchy.
 *
 * It should expose:
 *
 *     quantumClassicalFeedForwardStatement
 *
 * through the existing quantum control/dynamic element boundary.
 *
 * quantum.g4 remains the composition owner and MUST NOT duplicate the rule.
 *
 *
 * 2. quantum-classical.g4
 *
 * Existing feed-forward-like rules must be reconciled.
 *
 * Specifically:
 *
 *     quantumClassicalFeedback
 *
 * must become an adapter/reference to the canonical feed-forward construct,
 * or be deprecated if its spelling is not required for compatibility.
 *
 * Its current use of:
 *
 *     K_FEEDBACK
 *     K_WHEN
 *     K_APPLY
 *
 * must NOT remain as an independent lexical contract because the canonical
 * lexer currently owns:
 *
 *     WHEN
 *     APPLY
 *
 * and does not establish a canonical FEEDBACK token.
 *
 *
 * 3. mid-circuit-control.g4
 *
 * The explicit:
 *
 *     control (expression) quantumOperationStatement
 *
 * form remains distinct.
 *
 * It must NOT be duplicated here.
 *
 * Conceptual distinction:
 *
 *     control(condition) operation
 *         = explicit quantum control modifier
 *
 *     when condition operation
 *         = classical feed-forward dependency
 *
 * Both may lower to related semantic control structures, but their source
 * syntax remains distinct.
 *
 *
 * 4. dynamic-circuits.g4
 *
 * Dynamic `if` constructs remain owned there/general statement grammar.
 *
 * It must not duplicate the `when` feed-forward rule.
 *
 *
 * 5. measurement.g4
 *
 * No changes to measurement ownership.
 *
 * Measurement results become feed-forward inputs through semantic name/data
 * flow.
 *
 *
 * 6. operations.g4
 *
 * No operation syntax should be copied into this file.
 *
 * `quantumOperationStatement` remains the operation owner.
 *
 *
 * 7. expressions/
 *
 * `Expressions` remains the condition owner.
 *
 * No feed-forward expression grammar should be added.
 *
 *
 * 8. lexer/keywords.g4
 *
 * No new keyword is required.
 *
 * The existing:
 *
 *     WHEN : 'when'
 *
 * token is sufficient.
 *
 * Do NOT add:
 *
 *     FEEDBACK
 *     FEEDFORWARD
 *
 * solely for this grammar.
 *
 *
 * 9. AST
 *
 * Reuse the existing domain-neutral conditional/control/dependency AST
 * representation where possible.
 *
 * If a dedicated semantic node is required, it must be a domain-neutral
 * conditional/dependency node rather than a hardware-specific feed-forward
 * node.
 *
 *
 * 10. semantic analysis
 *
 * Resolve:
 *
 *     expression dependencies;
 *     measurement provenance;
 *     classical typing;
 *     availability;
 *     side effects;
 *     quantum-control legality;
 *     capability requirements;
 *     resource requirements.
 *
 *
 * 11. quantum::ir
 *
 * Lower quantum actions through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * Do not create a second feed-forward quantum IR.
 *
 *
 * 12. compiler/runtime
 *
 * The compiler/runtime may choose any semantically valid realization available
 * on the target.
 *
 *
 * 13. tests
 *
 * Add conformance tests under:
 *
 *     grammar/tests/quantum/classical-feedforward/
 *
 * with:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     compatibility/
 *     integration/
 *
 * ============================================================================
 * REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 * The following forms should be accepted once the surrounding operation
 * grammar is composed:
 *
 *     when result
 *         apply X(q);
 *
 *     when result == 1
 *         apply X(q);
 *
 *     when syndrome == expected
 *         apply correction(data);
 *
 *     when ready && enabled
 *         apply operation(target);
 *
 *     when predicate(result, state) {
 *         apply correction_a(data);
 *         apply correction_b(auxiliary);
 *     }
 *
 *     when measurement_state.value > threshold
 *         apply vendor.operation(target);
 *
 * Operation names in these examples are semantic identifiers, not reserved
 * gate vocabulary.
 *
 * ============================================================================
 * REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * Reject structurally incomplete forms:
 *
 *     when
 *
 *     when {
 *
 *     when condition
 *
 *     when condition {
 *
 *     when condition apply
 *
 *     when condition apply operation(
 *
 *     when condition apply operation(target
 *
 *     when condition {
 *         }
 *
 *     when condition {
 *         apply
 *     }
 *
 * Semantic analysis, rather than parsing, should reject:
 *
 *     when unknown_value
 *         apply operation(target);
 *
 * if `unknown_value` is not declared.
 *
 * Likewise:
 *
 *     when quantum_state
 *         apply operation(target);
 *
 * may be semantically invalid if `quantum_state` cannot be used as a classical
 * predicate.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test at least:
 *
 *     one condition;
 *     one quantum action;
 *     multiple quantum actions;
 *     empty block;
 *     nested expressions;
 *     qualified operation names;
 *     parameterized operations;
 *     indexed classical values;
 *     large symbolic expressions;
 *     deeply nested feed-forward blocks within available parser resources.
 *
 * An empty block is syntactically representable by this grammar but may be
 * rejected semantically if the language requires at least one executable
 * action.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Verify parsing for increasing source sizes:
 *
 *     one feed-forward statement;
 *     many statements;
 *     many conditions;
 *     many operations in one block;
 *     nested blocks;
 *     large expressions;
 *     large symbolic indices;
 *     large source units.
 *
 * No test may establish:
 *
 *     "N is the maximum supported number of feed-forward statements".
 *
 * Tests should instead verify that no grammar-level ceiling exists.
 *
 * Practical resource exhaustion is an implementation/resource-policy result,
 * not a language semantic failure.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * For identical:
 *
 *     source;
 *     language version;
 *     dialect configuration;
 *     canonical lexer;
 *     imported grammar versions;
 *
 * repeated parsing must produce equivalent parse structures and source spans.
 *
 * No environment, hardware, network, or runtime state may affect parsing.
 *
 * ============================================================================
 * COMPATIBILITY TESTS
 * ============================================================================
 *
 * Verify compatibility with:
 *
 *     quantum-classical.g4
 *     mid-circuit-control.g4
 *     dynamic-circuits.g4
 *     measurement.g4
 *     operations.g4
 *     quantum.g4
 *     canonical ZamaniLexer
 *
 * In particular verify that:
 *
 *     WHEN
 *
 * remains the sole lexical spelling used by this construct.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO hardware capacity constants;
 *     NO physical device identifiers;
 *     NO fixed qubit counts;
 *     NO fixed classical widths;
 *     NO fixed branch counts;
 *     NO fixed feedback latency;
 *     NO fixed operation counts;
 *     NO fixed node counts;
 *     NO fixed memory sizes;
 *     NO fixed register widths;
 *     NO fixed topology;
 *     NO vendor-specific operation enumeration.
 *
 * Specifically prohibited:
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
 *     no runtime dependencies.
 *
 * The generated/consuming implementation MUST remain compatible with:
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
 * This file is COMPLETE when:
 *
 *   [x] It is the sole grammar owner of `when` feed-forward syntax.
 *   [x] It uses canonical `ZamaniLexer`.
 *   [x] It uses canonical `WHEN`.
 *   [x] It does not introduce a FEEDBACK/FEEDFORWARD token.
 *   [x] Conditions reuse canonical `expression`.
 *   [x] Quantum operations reuse `quantumOperationStatement`.
 *   [x] No quantum gate enumeration exists.
 *   [x] No physical target syntax exists.
 *   [x] No hardware limits exist.
 *   [x] No semantic actions exist.
 *   [x] No embedded Rust exists.
 *   [x] No second quantum IR exists.
 *   [x] AST contract is defined.
 *   [x] Semantic contract is defined.
 *   [x] IR contract is defined.
 *   [x] Compiler contract is defined.
 *   [x] Runtime contract is defined.
 *   [x] Scheduling boundary is defined.
 *   [x] Routing boundary is defined.
 *   [x] QEC boundary is defined.
 *   [x] ZQN boundary is defined.
 *   [x] Resource/capability boundary is defined.
 *   [x] Diagnostic contract is defined.
 *   [x] Security contract is defined.
 *   [x] Performance contract is defined.
 *   [x] Scalability contract is defined.
 *   [x] Determinism contract is defined.
 *   [x] Compatibility contract is defined.
 *   [x] Test contract is defined.
 *   [x] Hard-coding audit is defined.
 *
 * Repository integration is complete only after the existing
 * quantum-classical/mid-circuit/dynamic grammar fragments are reconciled with
 * this owner.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Classical feed-forward is:
 *
 *     SOURCE-LEVEL DEPENDENCY
 *
 * not:
 *
 *     hardware feedback protocol;
 *     pulse instruction;
 *     physical control wire;
 *     scheduler instruction;
 *     QEC decoder;
 *     ZQN operation;
 *     runtime API;
 *     separate quantum IR.
 *
 * Final semantic path:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     classical feed-forward syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic dependency analysis
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 * classical semantics      quantum semantics
 *       |                       |
 *       v                       v
 * classical IR              quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *              optimization
 *                   |
 *                   v
 *              routing/scheduling
 *                   |
 *                   v
 *               QEC/ZQN
 *                   |
 *                   v
 *                  HAL
 *                   |
 *                   v
 *             target/runtime
 *
 * This is the required architecture for POCO-REAF.
 *
 * ============================================================================
 */