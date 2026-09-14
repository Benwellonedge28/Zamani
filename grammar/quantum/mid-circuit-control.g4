/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Quantum Mid-Circuit Control Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/mid-circuit-control.g4
 *
 * Role:
 *     Canonical reusable PARSER fragment for quantum mid-circuit control.
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
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX needed to express quantum work whose
 * execution depends on classical information that may have become available
 * during quantum execution.
 *
 * Typical source-level examples include:
 *
 *     measure q -> result;
 *
 *     if result {
 *         apply X to target;
 *     }
 *
 *     control (result) apply X to target;
 *
 *     control (result == 1) apply Z to target;
 *
 * The important distinction is:
 *
 *     measurement
 *          |
 *          v
 *     classical result
 *          |
 *          v
 *     condition
 *          |
 *          v
 *     quantum operation
 *
 * This file describes the SOURCE RELATIONSHIP.
 *
 * It does NOT implement:
 *
 *     - dynamic-circuit execution;
 *     - measurement;
 *     - quantum operations;
 *     - classical expressions;
 *     - classical control-flow in general;
 *     - scheduling;
 *     - routing;
 *     - hardware control;
 *     - pulse programming;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - backend selection;
 *     - runtime dispatch;
 *     - quantum::ir.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - explicit quantum mid-circuit control syntax;
 *     - the syntactic association between a classical condition and quantum
 *       work;
 *     - optional explicit measurement-dependency annotations;
 *     - quantum-specific control predicates;
 *     - quantum-controlled statement wrappers;
 *     - source-level control dependency structure;
 *     - source-level dynamic-control extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - IF/ELSE as a general language construct;
 *     - WHILE/FOR/LOOP;
 *     - blocks;
 *     - expressions;
 *     - identifiers;
 *     - measurement syntax;
 *     - reset syntax;
 *     - quantum operation syntax;
 *     - qubit identity;
 *     - classical bit identity;
 *     - classical register identity;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - hardware capabilities;
 *     - hardware topology;
 *     - backend selection;
 *     - execution;
 *     - canonical IR.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *          +---- this grammar fragment
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     name/type/effect/resource/capability analysis
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +---- quantum::ir
 *          |
 *          +---- optimization
 *          |
 *          +---- routing
 *          |
 *          +---- scheduling
 *          |
 *          +---- QEC
 *          |
 *          +---- ZQN
 *          |
 *          +---- resilience
 *          |
 *          +---- hardware HAL
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Mid-circuit control MUST describe semantic dependency, not hardware
 * realization.
 *
 * This grammar MUST NOT encode:
 *
 *     - a maximum number of measurements;
 *     - a maximum number of control predicates;
 *     - a maximum number of controlled operations;
 *     - a maximum circuit depth;
 *     - a fixed number of classical bits;
 *     - a fixed number of qubits;
 *     - a fixed number of control qubits;
 *     - a fixed number of measurement results;
 *     - a fixed feedback latency;
 *     - a fixed measurement duration;
 *     - a fixed gate duration;
 *     - a fixed device;
 *     - a fixed topology;
 *     - a fixed connectivity graph;
 *     - a fixed backend;
 *     - a fixed QPU;
 *     - a physical qubit identifier;
 *     - a hardware address.
 *
 * Therefore:
 *
 *     source control dependency
 *          !=
 *     physical feedback implementation
 *
 * A backend may implement a source-level dependency through:
 *
 *     - dynamic circuit execution;
 *     - deferred classical control;
 *     - compiler transformation;
 *     - measurement deferral;
 *     - conditional gates;
 *     - classical host control;
 *     - FPGA/control electronics;
 *     - simulator branching;
 *     - another valid target realization.
 *
 * ============================================================================
 * CANONICAL LEXICAL INTEGRATION
 * ============================================================================
 *
 * This file consumes the canonical Zamani lexer.
 *
 * Relevant existing tokens include:
 *
 *     IF
 *     ELSE
 *     CONTROL
 *     LPAREN
 *     RPAREN
 *     SEMICOLON
 *
 * Quantum operation names remain semantic identifiers rather than a fixed
 * hardware gate vocabulary.
 *
 * DO NOT introduce:
 *
 *     K_IF
 *     K_ELSE
 *     K_CONTROL
 *     SEMI
 *     MID_CIRCUIT_TOKEN
 *     CONDITION_TOKEN
 *     FEEDBACK_TOKEN
 *
 * The canonical lexer already owns lexical identity.
 *
 * ============================================================================
 * GENERAL CONTROL-FLOW BOUNDARY
 * ============================================================================
 *
 * IMPORTANT:
 *
 * General:
 *
 *     if
 *     else
 *     while
 *     for
 *     match
 *     blocks
 *
 * belong to the general statement/control-flow grammar.
 *
 * This file therefore does NOT redefine:
 *
 *     conditionalStatement
 *     loopStatement
 *     block
 *     expression
 *
 * Instead, this file introduces the explicit quantum-specific:
 *
 *     control (...)
 *
 * form.
 *
 * This avoids creating a second `if` grammar that competes with the general
 * language grammar.
 *
 * A normal:
 *
 *     if result {
 *         apply X to q;
 *     }
 *
 * remains a normal Zamani conditional inside a quantum region.
 *
 * Semantic analysis determines whether `result` is a mid-circuit measurement
 * dependency.
 *
 * ============================================================================
 * EXPLICIT MID-CIRCUIT CONTROL
 * ============================================================================
 *
 * Canonical explicit form:
 *
 *     control (result) apply X to target;
 *
 * More expressive conditions:
 *
 *     control (result == 1) apply X to target;
 *
 *     control (result && ready) apply Z to target;
 *
 * The condition is an ordinary Zamani expression.
 *
 * The grammar does not determine whether it is:
 *
 *     - compile-time;
 *     - runtime classical;
 *     - measurement-derived;
 *     - distributed;
 *     - temporal;
 *     - probabilistic;
 *     - hardware-supported.
 *
 * Semantic analysis determines this.
 *
 * ============================================================================
 * WHY EXPLICIT `control` EXISTS
 * ============================================================================
 *
 * The explicit form provides an unambiguous quantum-specific syntax without
 * duplicating the general:
 *
 *     if (...) { ... }
 *
 * grammar.
 *
 * This gives tools a precise source-level representation for:
 *
 *     classical predicate
 *             |
 *             v
 *     quantum operation
 *
 * while retaining ordinary `if` for general language control flow.
 *
 * ============================================================================
 * CONDITION CONTRACT
 * ============================================================================
 *
 * A control predicate is an ordinary Zamani expression.
 *
 * Examples:
 *
 *     result
 *     result == 1
 *     result[0]
 *     syndrome == expected
 *     flag && ready
 *     classical_state.value
 *
 * The grammar imposes no fixed width.
 *
 * It is therefore invalid for this grammar to assume:
 *
 *     one-bit condition
 *     32-bit condition
 *     64-bit condition
 *
 * The semantic/type system determines whether the expression can be converted
 * to a valid control predicate.
 *
 * ============================================================================
 * CONTROLLED BODY CONTRACT
 * ============================================================================
 *
 * The controlled body is intentionally an existing quantum operation statement.
 *
 * This file does NOT redefine:
 *
 *     quantumOperationStatement
 *
 * The quantum operation grammar remains the single owner of quantum operation
 * syntax.
 *
 * Therefore:
 *
 *     control (condition) apply X to q;
 *
 * becomes structurally:
 *
 *     control
 *       |
 *       +-- condition
 *       |
 *       +-- quantumOperationStatement
 *
 * ============================================================================
 * MULTIPLE CONTROL DEPENDENCIES
 * ============================================================================
 *
 * Nested control is syntactically supported:
 *
 *     control (a)
 *         control (b)
 *             apply X to q;
 *
 * No fixed nesting depth is encoded.
 *
 * Semantic analysis may reject invalid combinations according to the language
 * type/effect/resource model.
 *
 * ============================================================================
 * CONTROL EXPRESSION COMPOSITION
 * ============================================================================
 *
 * Conditions may contain arbitrary expressions:
 *
 *     control (a == b) apply X to q;
 *
 *     control (result && enabled) apply Z to q;
 *
 *     control (syndrome.value == expected) apply correction to data;
 *
 * This file does not define another boolean expression grammar.
 *
 * ============================================================================
 * MEASUREMENT DEPENDENCY
 * ============================================================================
 *
 * A condition may depend on a measurement result:
 *
 *     measure q -> result;
 *
 *     control (result) apply X to target;
 *
 * The grammar does NOT require that the measurement appear immediately before
 * the control statement.
 *
 * Dependency validity is semantic.
 *
 * The semantic layer determines whether the referenced value:
 *
 *     - exists;
 *     - is initialized;
 *     - is classical;
 *     - originates from measurement;
 *     - is available at the control point;
 *     - is legally usable for quantum control.
 *
 * ============================================================================
 * NO TEMPORAL/HARDWARE ASSUMPTIONS
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     feedback within N ns
 *     measurement latency
 *     maximum feedback depth
 *     maximum measurement distance
 *     maximum classical-control latency
 *
 * Those are target/resource/scheduling properties.
 *
 * ============================================================================
 * DYNAMIC CIRCUIT BOUNDARY
 * ============================================================================
 *
 * A dynamic circuit is a semantic property of a program execution graph.
 *
 * This file only contributes one possible source representation:
 *
 *     measurement result
 *          |
 *          v
 *     runtime predicate
 *          |
 *          v
 *     quantum operation
 *
 * The dynamic-circuit compiler/runtime machinery belongs outside grammar.
 *
 * ============================================================================
 * MID-CIRCUIT CONTROL VS QEC
 * ============================================================================
 *
 * This grammar may express:
 *
 *     control (syndrome) apply correction to data;
 *
 * but it does NOT implement:
 *
 *     - syndrome extraction;
 *     - stabilizer measurement;
 *     - decoding;
 *     - correction algorithms;
 *     - code-distance calculation;
 *     - logical error correction.
 *
 * QEC remains the owner of those semantics.
 *
 * ============================================================================
 * MID-CIRCUIT CONTROL VS ZQN
 * ============================================================================
 *
 * This grammar does NOT define:
 *
 *     - noise;
 *     - fault classes;
 *     - correlated faults;
 *     - leakage;
 *     - loss;
 *     - erasure;
 *     - measurement-error models.
 *
 * ZQN remains the owner of fault/noise semantics.
 *
 * ============================================================================
 * MID-CIRCUIT CONTROL VS RESILIENCE
 * ============================================================================
 *
 * This grammar does NOT decide:
 *
 *     retry;
 *     rollback;
 *     reroute;
 *     reschedule;
 *     switch backend;
 *     change QEC;
 *     mitigate;
 *     recover;
 *     abort.
 *
 * Resilience consumes downstream execution/telemetry/fault information.
 *
 * ============================================================================
 * MID-CIRCUIT CONTROL VS SCHEDULING
 * ============================================================================
 *
 * The grammar describes dependency.
 *
 * Scheduling decides:
 *
 *     when the measurement occurs;
 *     when the result becomes available;
 *     when the controlled operation may execute;
 *     whether alignment is necessary;
 *     whether delays are required;
 *     whether dynamic control is target-supported.
 *
 * ============================================================================
 * MID-CIRCUIT CONTROL VS ROUTING
 * ============================================================================
 *
 * This file never names:
 *
 *     physical qubit;
 *     coupling edge;
 *     topology;
 *     device;
 *     placement.
 *
 * Routing remains downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - source span;
 *     - condition expression;
 *     - controlled-body source node;
 *     - nesting;
 *     - source ordering;
 *     - annotations attached to the control statement.
 *
 * The AST MUST NOT directly contain:
 *
 *     PhysicalQubitId
 *     BackendId
 *     DeviceId
 *     PulseId
 *     ScheduleSlot
 *
 * unless those belong to a separate downstream representation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not define IR types.
 *
 * During semantic lowering, the source structure should become a canonical
 * semantic dependency representation consumed by:
 *
 *     quantum::ir
 *
 * or the canonical cross-domain IR if the dependency is represented at a
 * language-wide control/data-flow level.
 *
 * The correct IR representation is determined by the existing frontend/IR
 * contract, not invented in this grammar file.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar must produce deterministic parsing for:
 *
 *     control (condition) apply operation to target;
 *
 * Nested forms:
 *
 *     control (a) control (b) apply operation to target;
 *
 * Malformed forms must fail deterministically.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     backend communication;
 *     hardware communication;
 *     dynamic evaluation.
 *
 * Expressions are parsed structurally only.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no source-level finite resource limits in this file.
 *
 * Scalability is bounded only by:
 *
 *     - available compiler memory;
 *     - parser/runtime implementation resources;
 *     - semantic/resource constraints;
 *     - target capabilities;
 *     - execution resources.
 *
 * The grammar itself imposes no arbitrary machine-size ceiling.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. EXPLICIT MID-CIRCUIT CONTROL
 * ========================================================================== */

/*
 * Explicit quantum control:
 *
 *     control (condition) apply X to q;
 *
 * The body is deliberately an existing quantum operation statement.
 *
 * This rule therefore does not redefine operation syntax.
 */
quantumMidCircuitControlStatement
    : CONTROL
      LPAREN
      quantumMidCircuitControlCondition
      RPAREN
      quantumMidCircuitControlledStatement
    ;


/* ============================================================================
 * 2. CONTROL CONDITION
 * ========================================================================== */

/*
 * A control condition is an ordinary Zamani expression.
 *
 * Examples:
 *
 *     result
 *     result == 1
 *     result[0]
 *     syndrome == expected
 *     flag && ready
 *
 * No fixed bit width is imposed.
 */
quantumMidCircuitControlCondition
    : expression
    ;


/* ============================================================================
 * 3. CONTROLLED STATEMENT
 * ========================================================================== */

/*
 * The controlled body is an existing quantum operation.
 *
 * Keeping this as a reference rather than copying the operation grammar is
 * essential: quantum/operations.g4 remains the single owner of operation
 * syntax.
 */
quantumMidCircuitControlledStatement
    : quantumOperationStatement
    | quantumMidCircuitControlStatement
    ;


/* ============================================================================
 * 4. NESTED MID-CIRCUIT CONTROL
 * ========================================================================== */

/*
 * Nested control is intentionally recursive.
 *
 * Example:
 *
 *     control (outer)
 *         control (inner)
 *             apply X to q;
 *
 * No fixed nesting depth is encoded.
 *
 * Semantic analysis determines whether nested conditions are legal.
 */
quantumNestedMidCircuitControl
    : quantumMidCircuitControlStatement
    ;


/* ============================================================================
 * 5. CONDITIONED QUANTUM OPERATION ALIAS
 * ========================================================================== */

/*
 * Compatibility-facing alias.
 *
 * This gives semantic/front-end integration a stable quantum-specific name
 * without creating another operation grammar.
 *
 * It MUST resolve to the same AST representation as:
 *
 *     quantumMidCircuitControlStatement
 */
quantumConditionedQuantumOperation
    : quantumMidCircuitControlStatement
    ;


/* ============================================================================
 * 6. MEASUREMENT-DEPENDENT CONTROL
 * ========================================================================== */

/*
 * This rule represents the source relationship:
 *
 *     measurement-derived expression
 *             |
 *             v
 *     quantum control
 *
 * The grammar deliberately does not require a particular measurement syntax
 * immediately before the control statement.
 *
 * Example:
 *
 *     measure q -> result;
 *     control (result) apply X to target;
 *
 * Semantic analysis establishes the dependency.
 */
quantumMeasurementDependentControl
    : quantumMidCircuitControlStatement
    ;


/* ============================================================================
 * 7. QUANTUM FEEDBACK CONTROL
 * ========================================================================== */

/*
 * Feedback is intentionally an alias over the canonical explicit control form.
 *
 * This prevents multiple competing syntaxes for the same AST concept.
 */
quantumFeedbackControl
    : quantumMidCircuitControlStatement
    ;


/* ============================================================================
 * 8. CONTROLLED QUANTUM STATEMENT
 * ========================================================================== */

/*
 * Canonical integration entry point for quantum statement aggregators.
 *
 * IMPORTANT:
 *
 * This rule should be referenced by quantum/quantum.g4 or the canonical
 * quantum statement aggregator.
 */
quantumMidCircuitControl
    : quantumMidCircuitControlStatement
    ;