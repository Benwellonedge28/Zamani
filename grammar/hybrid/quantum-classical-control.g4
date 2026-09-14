/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/quantum-classical-control.g4
 *
 * Status:
 *     Production hybrid-domain quantum/classical control composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the HYBRID COMPOSITION LAYER for quantum/classical control.
 *
 * It does NOT redefine quantum control semantics.
 *
 * Existing ownership is deliberately preserved:
 *
 *     grammar/quantum/mid-circuit-control.g4
 *         owns explicit quantum mid-circuit control syntax.
 *
 *     grammar/quantum/quantum-classical.g4
 *         owns the canonical quantum/classical source boundary.
 *
 *     grammar/quantum/quantum.g4
 *         composes the quantum grammar.
 *
 *     grammar/hybrid/hybrid.g4
 *         composes hybrid computational domains.
 *
 * This file provides the hybrid-domain integration point that allows those
 * existing constructs to participate in a hybrid program without creating
 * duplicate quantum-control syntax.
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
 *     canonical parser / grammar composition
 *          |
 *          v
 *     this hybrid control boundary
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     classical semantics        quantum semantics
 *          |                          |
 *          v                          v
 *     canonical classical        quantum::ir
 *          representation             |
 *          |                          |
 *          +------------+-------------+
 *                       |
 *                       v
 *                 canonical semantic
 *                       IR
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      optimization   routing      scheduling
 *                                      |
 *                                      v
 *                              ZQN / QEC / resilience
 *                                      |
 *                                      v
 *                                 hardware HAL
 *                                      |
 *                                      v
 *                               target lowering
 *                                      |
 *                                      v
 *                                   runtime
 *
 * This grammar performs no lowering and creates no IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hybrid-domain composition of quantum/classical control;
 *     - the stable hybrid grammar entry point for quantum/classical control;
 *     - grouping of existing quantum-control constructs inside hybrid syntax;
 *     - hybrid control-region composition;
 *     - hybrid control dependency composition;
 *     - source-level composition of quantum control with canonical statements;
 *     - integration boundaries for future hybrid control extensions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - keywords;
 *     - identifiers;
 *     - literals;
 *     - expression syntax;
 *     - expression precedence;
 *     - type syntax;
 *     - general IF/ELSE syntax;
 *     - general loops;
 *     - quantum operation syntax;
 *     - quantum gate syntax;
 *     - quantum target syntax;
 *     - measurement syntax;
 *     - quantum/classical conversion semantics;
 *     - quantum mid-circuit control syntax itself;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - topology;
 *     - calibration;
 *     - device selection;
 *     - resource allocation;
 *     - canonical classical IR;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * The following existing components remain authoritative:
 *
 *     quantum/mid-circuit-control.g4
 *         explicit `control (condition) quantum-operation` syntax.
 *
 *     quantum/quantum-classical.g4
 *         explicit quantum/classical boundary syntax.
 *
 *     quantum/operations.g4
 *         quantum operation invocation syntax.
 *
 *     quantum/measurement.g4
 *         measurement syntax.
 *
 *     quantum/dynamic-circuits.g4
 *         dynamic-circuit syntax.
 *
 *     statements/*
 *         ordinary classical statement/control-flow syntax.
 *
 * This file MUST NOT copy any of those productions.
 *
 * ============================================================================
 * CANONICAL LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical lexer is responsible for lexical identity.
 *
 * In particular this file uses the canonical vocabulary:
 *
 *     CONTROL
 *     APPLY
 *     QUANTUM
 *     IF
 *     ELSE
 *     WHEN
 *     MEASURE
 *     OBSERVE
 *     LET
 *     REQUIRES
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *
 * This file MUST NOT introduce:
 *
 *     K_CONTROL
 *     K_APPLY
 *     K_QUANTUM
 *     K_IF
 *     K_ELSE
 *     K_WHEN
 *     K_MEASURE
 *     K_OBSERVE
 *     SEMI
 *
 * or any replacement lexical vocabulary.
 *
 * ============================================================================
 * NO CLOSED QUANTUM OPERATION INVENTORY
 * ============================================================================
 *
 * This grammar never enumerates:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     SWAP
 *     U
 *
 * or any other finite gate catalogue.
 *
 * Operation identity remains owned by the quantum operation grammar and
 * semantic resolution.
 *
 * Therefore future, vendor, logical, calibrated, synthesized or user-defined
 * operations do not require this file to be rewritten merely because a new
 * operation is introduced.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Hybrid quantum/classical control expresses:
 *
 *     computation
 *     dependency
 *     data flow
 *     control flow
 *     quantum intent
 *     classical intent
 *     semantic requirements
 *     capabilities
 *     constraints
 *
 * It does NOT express:
 *
 *     physical device identifiers
 *     physical qubit identifiers
 *     CPU identifiers
 *     GPU identifiers
 *     accelerator identifiers
 *     topology
 *     coupling maps
 *     pulse schedules
 *     calibration constants
 *     feedback latency
 *     clock frequency
 *     fixed resource counts
 *
 * The same source-level dependency may therefore be implemented using:
 *
 *     dynamic QPU control
 *     host-side classical control
 *     FPGA control logic
 *     simulator branching
 *     deferred execution
 *     compiler transformation
 *     future execution mechanisms
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file intentionally contains no finite source-language resource limits.
 *
 * It does NOT impose limits on:
 *
 *     qubits
 *     classical values
 *     measurement results
 *     control predicates
 *     operations
 *     control nesting
 *     circuit depth
 *     parameters
 *     devices
 *     nodes
 *     memory
 *     threads
 *     accelerators
 *
 * Repetition is represented structurally through recursive grammar rules and
 * zero-or-more / one-or-more constructs.
 *
 * Actual limits belong to:
 *
 *     parser implementation resources
 *     semantic analysis policy
 *     compiler resources
 *     target capabilities
 *     scheduling constraints
 *     runtime resources
 *     deployment configuration
 *
 * They MUST NOT become grammar semantics.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *     - whether a condition is Boolean-compatible;
 *     - whether a condition is classical;
 *     - whether a value originated from measurement;
 *     - whether the value is available at the control point;
 *     - whether the controlled operation is valid;
 *     - whether the operation has quantum effects;
 *     - whether the control has classical effects;
 *     - whether a synchronization dependency exists;
 *     - whether the target supports the dependency;
 *     - whether lowering preserves semantic meaning.
 *
 * The grammar does not answer those questions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - source spans;
 *     - hybrid-domain boundary;
 *     - control dependency;
 *     - condition expression;
 *     - controlled quantum statement;
 *     - nesting;
 *     - source ordering;
 *     - attached annotations/metadata.
 *
 * The AST MUST NOT contain target-specific allocation such as:
 *
 *     PhysicalQubitId
 *     DeviceId
 *     BackendId
 *     PulseId
 *     ScheduleSlot
 *     HardwareAddress
 *
 * unless those belong to an explicitly downstream representation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar defines no IR types.
 *
 * The semantic/lowering layer maps the parsed structure into:
 *
 *     canonical classical representation
 *     quantum::ir
 *     canonical cross-domain control/data dependencies
 *
 * according to the repository's semantic IR architecture.
 *
 * The grammar must never create a second quantum IR.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * Quantum semantics remain owned by:
 *
 *     quantum::ir
 *
 * This grammar therefore never defines:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumRegister
 *     MeasurementResult
 *     QuantumCircuit
 *
 * as semantic data structures.
 *
 * ============================================================================
 * QEC CONTRACT
 * ============================================================================
 *
 * This grammar may represent a control dependency such as:
 *
 *     control (syndrome) apply correction to data;
 *
 * but it does NOT implement:
 *
 *     syndrome extraction
 *     stabilizer construction
 *     decoding
 *     correction algorithms
 *     code distance
 *     logical-error analysis
 *
 * QEC remains downstream.
 *
 * ============================================================================
 * ZQN CONTRACT
 * ============================================================================
 *
 * This grammar does not represent:
 *
 *     noise models
 *     fault classes
 *     correlated faults
 *     leakage
 *     loss
 *     erasure
 *     measurement-error models
 *
 * ZQN remains responsible for fault/noise semantics.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * This grammar does not decide:
 *
 *     retry
 *     restart
 *     resume
 *     rollback
 *     reroute
 *     reschedule
 *     recompile
 *     reoptimize
 *     switch backend
 *     change QEC
 *     mitigate
 *     quarantine
 *     abort
 *
 * Resilience consumes downstream execution and fault information.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * The grammar expresses dependency.
 *
 * Scheduling determines:
 *
 *     ordering
 *     timing
 *     synchronization
 *     feedback realization
 *     latency handling
 *     alignment
 *     delays
 *     dynamic-control feasibility
 *
 * No timing value is encoded here.
 *
 * ============================================================================
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * This file never selects:
 *
 *     a QPU
 *     a simulator
 *     a CPU
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     a node
 *     a network endpoint
 *
 * Hardware capabilities are discovered and evaluated downstream.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     canonical lexer
 *           |
 *           v
 *     lower-level parser fragments
 *           |
 *           v
 *     quantum control grammar
 *           |
 *           v
 *     THIS FILE
 *           |
 *           v
 *     hybrid grammar
 *           |
 *           v
 *     canonical parser
 *           |
 *           v
 *     frontend AST
 *           |
 *           v
 *     semantic analysis
 *           |
 *           v
 *     canonical IR
 *
 * This file must never be imported by:
 *
 *     lexer
 *     expressions
 *     types
 *     quantum operation definitions
 *     quantum measurement definitions
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     hardware
 *     runtime
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar, not a combined grammar.
 *
 * It consumes ZamaniLexer.
 *
 * It should be imported by the hybrid parser composition layer rather than
 * becoming the canonical top-level parser itself.
 *
 * The lower-level quantum grammar is responsible for exposing:
 *
 *     quantumMidCircuitControlStatement
 *
 * to parser composition.
 *
 * ============================================================================
 */

parser grammar QuantumClassicalControl;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. HYBRID QUANTUM/CLASSICAL CONTROL ENTRY POINT
 * ============================================================================
 *
 * This is the public rule exported by this file.
 *
 * The actual `control (...) ...` syntax remains owned by
 * quantum/mid-circuit-control.g4.
 *
 * This wrapper exists so hybrid.g4 can consume quantum/classical control
 * through a stable hybrid-specific rule without duplicating quantum syntax.
 */
hybridQuantumClassicalControl
    : quantumMidCircuitControlStatement
    ;


/*
 * ============================================================================
 * 2. HYBRID CONTROL SEQUENCE
 * ============================================================================
 *
 * A hybrid region may contain any number of quantum/classical control
 * constructs.
 *
 * No finite count is imposed.
 */
hybridQuantumClassicalControlSequence
    : hybridQuantumClassicalControl*
    ;


/*
 * ============================================================================
 * 3. HYBRID CONTROL ELEMENT
 * ============================================================================
 *
 * This rule deliberately delegates ordinary statements to the canonical
 * statement grammar.
 *
 * The hybrid layer does not create another classical control-flow language.
 */
hybridQuantumClassicalControlElement
    : hybridQuantumClassicalControl
    | statement
    ;


/*
 * ============================================================================
 * 4. HYBRID CONTROL REGION
 * ============================================================================
 *
 * This is a syntactic grouping construct for a mixed-domain control region.
 *
 * It does not imply:
 *
 *     - a hardware execution region;
 *     - a scheduling region;
 *     - a device boundary;
 *     - a thread;
 *     - a process;
 *     - a physical controller.
 *
 * Semantic analysis determines its meaning.
 */
hybridQuantumClassicalControlRegion
    : LBRACE
      hybridQuantumClassicalControlElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. CONTROL DEPENDENCY
 * ============================================================================
 *
 * This rule provides a stable AST-facing name for the semantic relationship
 * represented by the existing quantum mid-circuit control grammar.
 *
 * It deliberately does not duplicate the condition syntax.
 */
hybridQuantumClassicalControlDependency
    : quantumMidCircuitControlStatement
    ;


/*
 * ============================================================================
 * 6. MEASUREMENT-TO-CONTROL COMPOSITION
 * ============================================================================
 *
 * The measurement itself remains owned by the quantum measurement grammar.
 *
 * This rule provides the hybrid source composition point:
 *
 *     measurement
 *         |
 *         v
 *     classical dependency
 *         |
 *         v
 *     quantum control
 *
 * The actual validity of the dependency is semantic.
 *
 * The grammar intentionally does NOT require the measurement to occur
 * immediately before the control statement.
 *
 * This allows:
 *
 *     measure q -> result;
 *
 *     classical computation;
 *
 *     control (result) apply operation to target;
 *
 * without imposing a false temporal grammar constraint.
 */
hybridMeasurementControlBoundary
    : hybridQuantumClassicalControl
    ;


/*
 * ============================================================================
 * 7. NESTED HYBRID CONTROL
 * ============================================================================
 *
 * Nested control is inherited through the canonical quantum control grammar.
 *
 * This wrapper exists as a named integration point for AST/tooling consumers.
 *
 * No maximum nesting depth is encoded.
 */
hybridNestedQuantumClassicalControl
    : hybridQuantumClassicalControl
    ;


/*
 * ============================================================================
 * 8. CONTROLLED HYBRID REGION
 * ============================================================================
 *
 * A classical control dependency may semantically encompass a mixed-domain
 * region. The actual condition and quantum-operation syntax remain owned by
 * their respective grammars.
 *
 * This rule is intentionally structural.
 */
hybridControlledRegion
    : hybridQuantumClassicalControlRegion
    ;


/*
 * ============================================================================
 * 9. CONTROL + CLASSICAL STATEMENT COMPOSITION
 * ============================================================================
 *
 * Ordinary classical statements remain owned by the statement grammar.
 *
 * This rule exists only as a composition point for hybrid AST construction.
 */
hybridControlStatement
    : hybridQuantumClassicalControl
    | statement
    ;


/*
 * ============================================================================
 * 10. CONTROL DEPENDENCY SEQUENCE
 * ============================================================================
 *
 * Structural sequence with no artificial count.
 */
hybridControlDependencySequence
    : hybridControlStatement*
    ;


/*
 * ============================================================================
 * 11. PUBLIC HYBRID DISPATCH RULE
 * ============================================================================
 *
 * Hybrid grammar consumers should use this rule rather than depending on
 * implementation-specific quantum-control rule names where possible.
 */
hybridQuantumClassicalControlConstruct
    : hybridQuantumClassicalControl
    | hybridQuantumClassicalControlRegion
    | hybridControlDependencySequence
    ;


/*
 * ============================================================================
 * 12. SEMANTIC ANCHOR
 * ============================================================================
 *
 * This rule intentionally remains a parser-level alias.
 *
 * It gives the frontend a stable source boundary:
 *
 *     classical predicate
 *            |
 *            v
 *     quantum operation
 *
 * The semantic layer determines whether the predicate is:
 *
 *     measurement-derived
 *     ordinary classical
 *     compile-time
 *     runtime
 *     distributed
 *     symbolic
 *     otherwise supported
 *
 * No semantic assumption is encoded here.
 */
quantumClassicalControlBoundary
    : hybridQuantumClassicalControl
    ;


/*
 * ============================================================================
 * 13. EXTENSIBILITY POINT
 * ============================================================================
 *
 * Future hybrid control constructs MUST be added through dedicated grammar
 * ownership rather than by weakening this rule into arbitrary expressions.
 *
 * Examples of future constructs may include:
 *
 *     classical feedback regions
 *     distributed quantum/classical control
 *     accelerator feedback
 *     adaptive quantum execution
 *     heterogeneous control
 *
 * Such constructs must acquire their own grammar ownership before being added
 * here.
 *
 * ============================================================================
 * 14. HARD-CODING AUDIT
 * ============================================================================
 *
 * No fixed machine or resource quantity occurs in this grammar.
 *
 * Forbidden examples include:
 *
 *     MAX_QUBITS
 *     MAX_BITS
 *     MAX_CONTROLS
 *     MAX_MEASUREMENTS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_DEPTH
 *     DEVICE_0
 *     QUBIT_0
 *     QUBIT_1
 *
 * None are present.
 *
 * ============================================================================
 * 15. SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     dynamic code execution
 *     hardware access
 *     backend communication
 *     environment inspection
 *     mutable global state
 *
 * It is pure syntax.
 *
 * ============================================================================
 * 16. DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for:
 *
 *     control (condition) apply operation to target;
 *
 * and nested forms represented by the canonical quantum-control grammar.
 *
 * Invalid forms must fail deterministically.
 *
 * ============================================================================
 * 17. COMPATIBILITY
 * ============================================================================
 *
 * This file must remain compatible with:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/hybrid/hybrid.g4
 *     grammar/quantum/quantum.g4
 *     grammar/quantum/mid-circuit-control.g4
 *     grammar/quantum/quantum-classical.g4
 *     grammar/quantum/operations.g4
 *     grammar/statements/*
 *
 * It must not require modification when a new quantum gate, hardware target,
 * QPU topology, CPU architecture, accelerator, or backend is introduced.
 *
 * ============================================================================
 * 18. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     1. It compiles under ANTLR4 with the canonical Zamani lexer.
 *
 *     2. It introduces no duplicate ownership of quantum mid-circuit control.
 *
 *     3. It exposes a stable hybrid-specific control entry point.
 *
 *     4. It composes with hybrid.g4 without circular grammar dependencies.
 *
 *     5. It accepts arbitrary structurally valid control nesting.
 *
 *     6. It imposes no finite machine/resource limits.
 *
 *     7. It creates no IR.
 *
 *     8. It introduces no hardware assumptions.
 *
 *     9. It preserves quantum::ir as the quantum semantic boundary.
 *
 *    10. It preserves the existing quantum/mid-circuit-control.g4 ownership.
 *
 *    11. It uses canonical lexer token names.
 *
 *    12. It contains no Rust actions and therefore no unsafe Rust.
 *
 *    13. Positive, negative, boundary, cross-domain, determinism and
 *        compatibility tests pass.
 *
 * ============================================================================
 */