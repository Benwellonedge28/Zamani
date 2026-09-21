/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/quantum.g4
 *
 * STATUS
 * ------
 * CANONICAL QUANTUM STATEMENT-LAYER COMPOSITION / INTEGRATION GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser fragment
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the statement-layer integration boundary for quantum
 * computation.
 *
 * It does NOT implement a second quantum language.
 *
 * It does NOT define quantum operations, gates, qubits, measurements,
 * observables, reset semantics, dynamic-circuit semantics, routing,
 * scheduling, QEC, ZQN, resilience, calibration, hardware topology,
 * backend selection, or quantum::ir.
 *
 * Instead, it composes the existing quantum grammar owners into the
 * language-wide statement architecture.
 *
 * The intended direction is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       +--> grammar/statements/quantum.g4       <-- THIS FILE
 *       |
 *       +--> grammar/quantum/*.g4
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical quantum semantic representation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> resource analysis
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC
 *       +--> ZQN
 *       +--> resilience
 *       +--> HAL
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This file describes SOURCE-LEVEL STATEMENT STRUCTURE.
 *
 * It must never encode a particular machine.
 *
 * Therefore it contains no language-level limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     registers
 *     operations
 *     controls
 *     parameters
 *     circuit depth
 *     measurements
 *     timelines
 *     devices
 *     processors
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     topology
 *
 * It also contains no:
 *
 *     device IDs
 *     physical qubit IDs
 *     coupling maps
 *     vendor gate sets
 *     pulse durations
 *     calibration values
 *     scheduling slots
 *     backend IDs
 *     hardware addresses
 *
 * Program constants remain valid source semantics.
 *
 * For example:
 *
 *     qubit[1024]
 *
 * may be valid program meaning.
 *
 * It must never be interpreted as:
 *
 *     "Zamani supports at most 1024 qubits."
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     - statement-level admission of quantum constructs;
 *     - statement-level quantum dispatch;
 *     - compatibility adapters between the statement layer and existing
 *       quantum grammar owners;
 *     - the canonical statement-layer entry point for quantum statements;
 *     - composition of quantum-specific statement families;
 *     - explicit separation between quantum statements and ordinary
 *       language statements.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexical tokens;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - declarations;
 *     - quantum operation syntax;
 *     - operation parameter syntax;
 *     - operation target syntax;
 *     - measurement syntax;
 *     - reset syntax;
 *     - observable syntax;
 *     - barrier syntax;
 *     - controlled-operation syntax;
 *     - dynamic-circuit syntax;
 *     - circuit declaration syntax;
 *     - QEC syntax/semantics;
 *     - ZQN syntax/semantics;
 *     - resource discovery;
 *     - capability discovery;
 *     - physical allocation;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - backend selection;
 *     - runtime execution;
 *     - quantum::ir.
 *
 * ============================================================================
 * EXISTING REPOSITORY OWNERS
 * ============================================================================
 *
 * The following files remain the detailed owners of their respective syntax:
 *
 *     grammar/quantum/operations.g4
 *         quantum operation invocation
 *
 *     grammar/quantum/parameterized-operations.g4
 *         parameter and argument extensions
 *
 *     grammar/quantum/controlled-operations.g4
 *         explicit controlled-operation structures
 *
 *     grammar/quantum/measurement.g4
 *         measurement syntax
 *
 *     grammar/quantum/reset.g4
 *         reset syntax
 *
 *     grammar/quantum/gates.g4
 *         legacy/gate-oriented compatibility surface
 *
 *     grammar/quantum/observables.g4
 *         observable/observation syntax
 *
 *     grammar/quantum/mid-circuit-control.g4
 *         explicit mid-circuit control
 *
 *     grammar/quantum/dynamic-circuits.g4
 *         dynamic-circuit syntax
 *
 *     grammar/quantum/circuits.g4
 *         circuit declarations and circuit composition
 *
 *     grammar/quantum/quantum-classical.g4
 *         hybrid quantum/classical boundaries
 *
 *     grammar/quantum/quantum.g4
 *         quantum-domain aggregation/declarations
 *
 * This file MUST NOT copy their concrete productions.
 *
 * ============================================================================
 * IMPORTANT AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one effective statement-layer quantum dispatcher.
 *
 * The desired production architecture is:
 *
 *     grammar/statements/statements.g4
 *                 |
 *                 +--> quantumStatement
 *                         |
 *                         +--> quantumOperationStatement
 *                         +--> quantumMeasurementStatement
 *                         +--> quantumResetStatement
 *                         +--> quantumBarrierStatement
 *                         +--> quantumObservationStatement
 *                         +--> quantumMidCircuitControlStatement
 *                         +--> quantumDynamicCircuitStatement
 *                         +--> quantumClassicalStatement
 *
 * The detailed quantum grammar remains owned by grammar/quantum/.
 *
 * `grammar/quantum/quantum.g4` must therefore not remain a competing
 * statement dispatcher after this file becomes authoritative.
 *
 * Its quantum-domain aggregation role should be retained for declarations
 * and quantum-domain elements, but the language-wide statement composition
 * belongs here.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical lexer vocabulary.
 *
 * It must NOT create lexer rules.
 *
 * Existing canonical vocabulary includes quantum-related tokens such as:
 *
 *     APPLY
 *     MEASURE
 *     RESET
 *     BARRIER
 *     CONTROL
 *     ADJOINT
 *     INVERSE
 *     OBSERVE
 *     QUANTUM
 *
 * Punctuation is likewise owned by the canonical lexer.
 *
 * This file therefore does NOT introduce:
 *
 *     K_APPLY
 *     K_MEASURE
 *     K_RESET
 *     K_BARRIER
 *     K_CONTROL
 *     K_OBSERVE
 *     SEMI
 *
 * merely to support quantum statements.
 *
 * Existing quantum files that still use legacy `K_*` names or literal
 * spellings must be reconciled at their own ownership boundary.
 *
 * This file must not reproduce those lexical inconsistencies.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All conditions, arguments, targets, parameters, resource expressions,
 * observable expressions, and measurement predicates are delegated to the
 * canonical expression grammar.
 *
 * This file does NOT define:
 *
 *     expression
 *     assignmentExpression
 *     logicalExpression
 *     comparisonExpression
 *     indexing
 *     ranges
 *     calls
 *     literals
 *
 * Quantum semantic validity is determined after parsing.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file does not define quantum types.
 *
 * Quantum types remain owned by:
 *
 *     grammar/types/
 *     grammar/quantum/quantum-types.g4
 *
 * Examples such as:
 *
 *     Qubit
 *     Qubit[n]
 *     QuantumRegister<T>
 *     logical quantum resources
 *
 * are type-system concerns rather than statement-layer concerns.
 *
 * ============================================================================
 * CORE QUANTUM STATEMENT ENTRY POINT
 * ============================================================================
 *
 * `quantumStatement` is the canonical statement-layer entry point.
 *
 * It deliberately contains only statement families.
 *
 * It does not include quantum declarations because declarations have a
 * separate ownership path.
 *
 * It also does not admit arbitrary `expression` values. This is intentional:
 * accepting every expression here would make malformed quantum syntax
 * accidentally valid and would weaken diagnostics.
 *
 * ============================================================================
 */

quantumStatement
    : quantumOperationStatement
    | quantumMeasurementStatement
    | quantumResetStatement
    | quantumBarrierStatement
    | quantumObservationStatement
    | quantumMidCircuitControlStatement
    | quantumDynamicCircuitStatement
    | quantumClassicalStatement
    ;


/*
 * ============================================================================
 * QUANTUM OPERATION STATEMENT
 * ============================================================================
 *
 * Detailed syntax remains exclusively owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * That grammar already provides the generic operation model:
 *
 *     apply operation(targets);
 *
 * and parameterized forms such as:
 *
 *     apply operation(parameters)(targets);
 *
 * Operation names remain data/identifiers rather than a fixed universal gate
 * enumeration.
 *
 * Therefore this file must NOT contain:
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
 *
 * as grammar alternatives.
 *
 * User/vendor/future operations remain open-world names.
 *
 * The semantic layer determines whether an operation exists and what it means.
 *
 * Canonical lowering:
 *
 *     quantumOperationStatement
 *             |
 *             v
 *     frontend AST Operation
 *             |
 *             v
 *     semantic quantum operation
 *             |
 *             v
 *     quantum::ir
 */
quantumStatementOperation
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * MEASUREMENT STATEMENT
 * ============================================================================
 *
 * Detailed syntax remains owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * The statement-layer adapter exists so measurement has one stable name at
 * the statement integration boundary.
 *
 * No measurement width, result count, observable size, or target count is
 * restricted here.
 */
quantumStatementMeasurement
    : quantumMeasurementStatement
    ;


/*
 * ============================================================================
 * RESET STATEMENT
 * ============================================================================
 *
 * Existing repository ownership:
 *
 *     grammar/quantum/reset.g4
 *
 * The current reset grammar owns:
 *
 *     quantumResetOperation
 *
 * This adapter establishes the statement-layer name without duplicating reset
 * syntax.
 *
 * This also provides the compatibility bridge required by the existing
 * quantum aggregation grammar, which currently refers to
 * `quantumResetStatement`.
 */
quantumResetStatement
    : quantumResetOperation
    ;


/*
 * ============================================================================
 * BARRIER STATEMENT
 * ============================================================================
 *
 * Existing repository ownership:
 *
 *     grammar/quantum/gates.g4
 *
 * The existing gate grammar owns:
 *
 *     barrierStatement
 *
 * This adapter provides the canonical quantum statement-layer name.
 *
 * A barrier expresses a source-level ordering/synchronization intent.
 *
 * It does NOT specify:
 *
 *     hardware clock cycles
 *     pulse durations
 *     physical synchronization hardware
 *     target timing
 *     device topology
 *
 * Scheduling and lowering remain downstream.
 */
quantumBarrierStatement
    : barrierStatement
    ;


/*
 * ============================================================================
 * OBSERVATION / OBSERVABLE STATEMENT
 * ============================================================================
 *
 * Detailed observable syntax remains owned by:
 *
 *     grammar/quantum/observables.g4
 *
 * This file merely admits it as a quantum statement.
 *
 * Observable semantics are resolved after parsing.
 *
 * No fixed:
 *
 *     observable width
 *     Pauli weight
 *     term count
 *     target count
 *     result precision
 *
 * is encoded here.
 */
quantumObservationStatement
    : quantumObservationStatementOwned
    ;


/*
 * ============================================================================
 * OBSERVATION OWNERSHIP ADAPTER
 * ============================================================================
 *
 * `quantumObservationStatementOwned` is an explicit integration seam.
 *
 * The actual observables grammar should expose this alias when its historical
 * rule name differs.
 *
 * Production integration may map:
 *
 *     quantumObservationStatementOwned
 *         -> quantumObservationStatement
 *
 * from grammar/quantum/observables.g4.
 *
 * The important architectural rule is that this file does not reproduce
 * observable syntax.
 *
 * ============================================================================
 *
 * NOTE:
 *
 * If the canonical observable grammar already exposes
 * `quantumObservationStatement` directly, the adapter above should be
 * collapsed during grammar assembly so that only ONE effective rule name is
 * present.
 *
 * No second observable grammar is permitted.
 */
quantumObservationStatementOwned
    : quantumObservationStatementCanonical
    ;


/*
 * ============================================================================
 * CANONICAL OBSERVATION INTEGRATION SEAM
 * ============================================================================
 *
 * This name is deliberately isolated because the current repository contains
 * historical naming differences in observable grammar.
 *
 * The canonical assembled grammar must bind this rule to the actual owner in:
 *
 *     grammar/quantum/observables.g4
 *
 * It must never duplicate the observable production here.
 */
quantumObservationStatementCanonical
    : quantumObservationStatement
    ;


/*
 * ============================================================================
 * MID-CIRCUIT CONTROL
 * ============================================================================
 *
 * Detailed explicit mid-circuit control remains owned by:
 *
 *     grammar/quantum/mid-circuit-control.g4
 *
 * Canonical source shape:
 *
 *     control (condition) apply operation(targets);
 *
 * The condition is a normal Zamani expression.
 *
 * No fixed:
 *
 *     condition width
 *     measurement count
 *     control count
 *     feedback latency
 *     circuit depth
 *
 * is encoded here.
 *
 * Nested control remains possible because the underlying grammar is recursive.
 */
quantumMidCircuitControlStatement
    : quantumMidCircuitControlOwned
    ;


/*
 * ============================================================================
 * MID-CIRCUIT CONTROL OWNERSHIP SEAM
 * ============================================================================
 *
 * The current detailed owner exposes:
 *
 *     quantumMidCircuitControlStatement
 *
 * The statement-layer composition must ultimately reference that exact
 * production once the grammar assembler has one effective namespace.
 *
 * This adapter name exists to make the ownership direction explicit.
 *
 * It must not create a second semantic representation.
 */
quantumMidCircuitControlOwned
    : quantumMidCircuitControlCanonical
    ;


quantumMidCircuitControlCanonical
    : quantumMidCircuitControlStatement
    ;


/*
 * ============================================================================
 * DYNAMIC CIRCUIT STATEMENT
 * ============================================================================
 *
 * Detailed dynamic-circuit syntax remains owned by:
 *
 *     grammar/quantum/dynamic-circuits.g4
 *
 * This file does not define:
 *
 *     if
 *     while
 *     for
 *     dynamic condition expressions
 *     dynamic circuit bodies
 *
 * itself.
 *
 * General control flow remains owned by grammar/statements/.
 */
quantumDynamicCircuitStatement
    : quantumDynamicCircuitStatementOwned
    ;


quantumDynamicCircuitStatementOwned
    : quantumDynamicCircuitStatementCanonical
    ;


quantumDynamicCircuitStatementCanonical
    : quantumDynamicCircuitStatement
    ;


/*
 * ============================================================================
 * QUANTUM/CLASSICAL STATEMENT BOUNDARY
 * ============================================================================
 *
 * Quantum/classical interaction remains owned by:
 *
 *     grammar/quantum/quantum-classical.g4
 *
 * The statement layer only admits the already-defined quantum/classical
 * statement family.
 *
 * It must not duplicate:
 *
 *     classical control
 *     result binding
 *     measurement conversion
 *     hybrid invocation
 *     synchronization
 *     classical feedback
 *
 * Those remain owned by the quantum-classical grammar and the general
 * expression/type systems.
 */
quantumClassicalStatement
    : quantumClassicalStatementOwned
    ;


quantumClassicalStatementOwned
    : quantumClassicalStatementCanonical
    ;


quantumClassicalStatementCanonical
    : quantumClassicalConstruct
    ;


/*
 * ============================================================================
 * CANONICAL OPERATION INTEGRATION ALIAS
 * ============================================================================
 *
 * This alias is useful to statement aggregators that need a stable
 * statement-layer name without importing the detailed operation vocabulary.
 *
 * It intentionally delegates completely.
 */
quantumOperationStatementCanonical
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * CANONICAL MEASUREMENT INTEGRATION ALIAS
 * ============================================================================
 */
quantumMeasurementStatementCanonical
    : quantumMeasurementStatement
    ;


/*
 * ============================================================================
 * QUANTUM STATEMENT SEQUENCE
 * ============================================================================
 *
 * A quantum region may contain an arbitrary sequence of quantum statements.
 *
 * There is intentionally no fixed:
 *
 *     operation count
 *     measurement count
 *     depth
 *     statement count
 *     register count
 *
 * The actual implementation is bounded only by available compiler resources
 * and explicit semantic/resource constraints.
 */
quantumStatementSequence
    : quantumStatement*
    ;


/*
 * ============================================================================
 * QUANTUM STATEMENT BLOCK
 * ============================================================================
 *
 * This is a statement-layer adapter, NOT a second block grammar.
 *
 * General block syntax remains owned by:
 *
 *     grammar/statements/blocks.g4
 *
 * The canonical statement grammar should use its existing `block` rule where
 * a normal Zamani block is required.
 *
 * This rule is provided only for quantum-domain consumers that need a named
 * quantum statement sequence.
 */
quantumStatementBlock
    : LBRACE
      quantumStatementSequence
      RBRACE
    ;


/*
 * ============================================================================
 * ATTRIBUTED QUANTUM STATEMENT
 * ============================================================================
 *
 * Attributes remain owned by grammar/core/attributes.g4.
 *
 * This file only defines the attachment point.
 *
 * Attributes are metadata/intent and do not directly select a hardware
 * backend.
 */
quantumAttributedStatement
    : attributes*
      quantumStatement
    ;


/*
 * ============================================================================
 * QUANTUM STATEMENT DISPATCH ADAPTER
 * ============================================================================
 *
 * This is the rule that language-wide statement composition should reference.
 *
 * It exists separately from `quantumStatement` so that future statement
 * composition can attach attributes or other language-wide wrappers without
 * modifying each quantum grammar owner.
 */
quantumStatementDispatch
    : quantumAttributedStatement
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parse tree produced by this file has NO direct knowledge of:
 *
 *     PhysicalQubitId
 *     BackendId
 *     DeviceId
 *     ScheduleSlot
 *     PulseId
 *     CouplingEdge
 *     CalibrationId
 *
 * Such concepts belong downstream.
 *
 * The semantic pipeline is:
 *
 *     quantum statement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          +--> names
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resource requirements
 *          +--> ownership
 *          |
 *          v
 *     canonical quantum semantics
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 * The grammar never constructs or modifies quantum::ir.
 *
 * ============================================================================
 * GENERIC OPERATION PRINCIPLE
 * ============================================================================
 *
 * A valid operation statement is structurally open-world:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(c, t);
 *     apply RX(theta)(q);
 *     apply custom.operation(q);
 *     apply vendor.operation(q);
 *     apply future.operation(parameters)(targets);
 *
 * No one of these names is privileged by this file.
 *
 * The semantic layer determines whether a referenced operation is:
 *
 *     standard
 *     user-defined
 *     library-defined
 *     dialect-defined
 *     vendor-defined
 *     future-defined
 *
 * This is required for POCO-REAF.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file MUST remain independent of:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     simulator
 *     cluster
 *     cloud
 *     topology
 *     physical allocation
 *
 * The same source-level quantum statement may ultimately be lowered to
 * different computational substrates.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A quantum statement expresses computation.
 *
 * Resource and capability requirements are separate semantic concerns.
 *
 * For example, a program may require:
 *
 *     capability("quantum.measurement")
 *
 * or:
 *
 *     capability("quantum.mid_circuit_control")
 *
 * without selecting:
 *
 *     device 0
 *     physical qubit 17
 *     backend X
 *
 * Resource/capability syntax belongs to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/quantum/
 *
 * as appropriate.
 *
 * This file must not duplicate those grammars.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM INTEGRATION
 * ============================================================================
 *
 * General Zamani statements remain valid around quantum statements.
 *
 * Example:
 *
 *     let angle = compute_angle();
 *
 *     apply RX(angle)(q);
 *
 *     measure q;
 *
 *     if result {
 *         apply correction(target);
 *     }
 *
 * The quantum statement layer does not create a second control-flow language.
 *
 * Ordinary:
 *
 *     if
 *     else
 *     while
 *     for
 *     match
 *
 * remain owned by the general statement grammar.
 *
 * Quantum-specific dynamic dependency remains owned by
 * `mid-circuit-control.g4` and `dynamic-circuits.g4`.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Quantum statements may participate in hardware/software co-design.
 *
 * However, this file does not introduce:
 *
 *     physical wires
 *     fixed register widths
 *     fixed buses
 *     physical addresses
 *     clock numbers
 *     fixed timing
 *     fixed hardware resources
 *
 * HDL intent belongs to:
 *
 *     grammar/hdl/
 *
 * Hardware capability/resource intent belongs to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *
 * Quantum statements remain semantic computation.
 *
 * ============================================================================
 * QEC INTEGRATION
 * ============================================================================
 *
 * QEC is downstream.
 *
 * A quantum statement may eventually participate in:
 *
 *     logical computation
 *     syndrome extraction
 *     correction
 *     fault-tolerant execution
 *
 * but this file does not implement:
 *
 *     code distance
 *     decoder algorithms
 *     syndrome processing
 *     physical correction
 *     logical error rates
 *
 * QEC analysis consumes canonical quantum semantics/IR.
 *
 * ============================================================================
 * ZQN INTEGRATION
 * ============================================================================
 *
 * ZQN owns fault/noise semantics.
 *
 * This file does not encode:
 *
 *     noise probabilities
 *     leakage
 *     loss
 *     erasure
 *     correlated faults
 *     calibration noise
 *     measurement-error models
 *
 * Those remain downstream semantic/runtime concerns.
 *
 * ============================================================================
 * ROUTING INTEGRATION
 * ============================================================================
 *
 * Quantum targets are source-level semantic resources.
 *
 * This file never chooses:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     coupling edge
 *     topology
 *     swap insertion
 *
 * Routing is performed after canonical quantum semantic lowering.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Statement ordering establishes semantic dependencies.
 *
 * This file does not specify:
 *
 *     pulse time
 *     gate duration
 *     machine cycle
 *     hardware slot
 *     queue position
 *
 * Scheduling is downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no I/O;
 *     - no filesystem access;
 *     - no networking;
 *     - no process execution;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no mutable global state.
 *
 * Identical token streams and identical grammar/lexer versions must produce
 * deterministic structural parsing.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * This file is responsible only for structural statement recognition.
 *
 * Syntax diagnostics include:
 *
 *     malformed quantum operation statement
 *     malformed measurement statement
 *     malformed reset statement
 *     malformed barrier statement
 *     malformed observation statement
 *     malformed mid-circuit control
 *     malformed dynamic quantum statement
 *     malformed quantum/classical statement
 *
 * Semantic diagnostics remain downstream:
 *
 *     unknown operation
 *     invalid operation arity
 *     invalid quantum operand
 *     invalid type
 *     invalid measurement target
 *     invalid control dependency
 *     unsupported capability
 *     insufficient resources
 *     invalid QEC requirement
 *     unavailable target capability
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted quantum statement must preserve enough source structure for
 * the domain-neutral AST to retain:
 *
 *     source span
 *     statement kind
 *     operation/reference identity
 *     argument expressions
 *     target expressions
 *     modifiers
 *     measurement information
 *     reset information
 *     control condition
 *     dynamic structure
 *     attributes
 *     source ordering
 *
 * The AST must NOT acquire physical hardware identity merely because a
 * statement was parsed.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This grammar does not define quantum::ir.
 *
 * The required lowering direction is:
 *
 *     parse tree
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic quantum operation
 *       |
 *       v
 *     quantum::ir
 *
 * There must be no second statement-specific quantum IR.
 *
 * In particular, this file must not introduce:
 *
 *     QuantumStatementIR
 *     QuantumGateIR
 *     QuantumOperationIR
 *     PhysicalQuantumStatementIR
 *
 * merely to bridge grammar and the existing canonical IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The source program describes semantic computation once.
 *
 * The same source-level quantum statements must be capable of being compiled
 * toward:
 *
 *     tiny quantum-capable systems
 *     larger systems
 *     simulators
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     heterogeneous systems
 *     distributed systems
 *     future computational substrates
 *
 * subject only to the actual semantic requirements and resources available.
 *
 * No source grammar rewrite should be necessary merely because the target
 * machine becomes larger or smaller.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no artificial finite limits on:
 *
 *     statements
 *     operations
 *     targets
 *     parameters
 *     measurements
 *     controls
 *     nesting
 *     circuit depth
 *     program size
 *
 * Recursive grammar constructs are intentionally open-ended.
 *
 * Actual implementation limits belong to:
 *
 *     parser resource policy
 *     compiler resource policy
 *     semantic resource policy
 *     target capability
 *     runtime resources
 *
 * and must not be confused with language validity.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no Rust implementation actions.
 *
 * The generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * The language keyword `unsafe`, elsewhere in Zamani, does not authorize
 * unsafe Rust inside the compiler implementation.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is intended to provide a stable statement-layer boundary while
 * the repository's older quantum grammar names are reconciled.
 *
 * Existing detailed owners must remain the semantic owners.
 *
 * Compatibility adapters are permitted.
 *
 * Duplicate semantic grammars are not.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENTS
 * ============================================================================
 *
 * The following integration must be performed when assembling the production
 * grammar:
 *
 * 1. grammar/statements/statements.g4
 *
 *    Add `quantumStatement` as one statement-family alternative.
 *
 *    The canonical composition becomes conceptually:
 *
 *        statement
 *            : ...
 *            | quantumStatement
 *            ;
 *
 *
 * 2. grammar/quantum/quantum.g4
 *
 *    Retain quantum-domain declaration/element ownership.
 *
 *    Remove its competing statement-level ownership from the authoritative
 *    assembled grammar.
 *
 *    It must not provide a second effective `quantumStatement`.
 *
 *
 * 3. grammar/quantum/reset.g4
 *
 *    Continue to own:
 *
 *        quantumResetOperation
 *
 *    This file provides the statement-layer adapter:
 *
 *        quantumResetStatement
 *
 *
 * 4. grammar/quantum/gates.g4
 *
 *    Continue to own:
 *
 *        barrierStatement
 *
 *    This file provides:
 *
 *        quantumBarrierStatement
 *
 *
 * 5. grammar/quantum/operations.g4
 *
 *    Continue to own:
 *
 *        quantumOperationStatement
 *
 *    No operation syntax is duplicated here.
 *
 *
 * 6. grammar/quantum/measurement.g4
 *
 *    Continue to own:
 *
 *        quantumMeasurementStatement
 *
 *
 * 7. grammar/quantum/mid-circuit-control.g4
 *
 *    Continue to own:
 *
 *        quantumMidCircuitControlStatement
 *
 *
 * 8. grammar/quantum/dynamic-circuits.g4
 *
 *    Continue to own:
 *
 *        quantumDynamicCircuitStatement
 *
 *
 * 9. grammar/quantum/observables.g4
 *
 *    Must expose one canonical observation-statement production.
 *
 *    Legacy `K_*`/plain-token discrepancies must be resolved at the lexer/
 *    observable grammar boundary rather than duplicated here.
 *
 *
 * 10. grammar/quantum/quantum-classical.g4
 *
 *     Must expose the canonical quantum/classical statement family through
 *     one stable production.
 *
 *
 * 11. grammar/spec/quantum.md
 *
 *     This file conforms to the existing specification principle:
 *
 *         grammar
 *             -> AST
 *             -> semantic analysis
 *             -> quantum::ir
 *
 *     It does not introduce a second quantum IR.
 *
 * ============================================================================
 * IMPORTANT ASSEMBLY NOTE
 * ============================================================================
 *
 * The aliases in this file are intentionally integration boundaries.
 *
 * During final ANTLR assembly, the repository must ensure that aliases do not
 * recursively reference themselves through duplicate imported rule names.
 *
 * In particular, the following names must each have exactly one canonical
 * underlying owner:
 *
 *     quantumObservationStatement
 *     quantumMidCircuitControlStatement
 *     quantumDynamicCircuitStatement
 *     quantumOperationStatement
 *     quantumMeasurementStatement
 *
 * If the detailed owner already uses the exact canonical name, the
 * corresponding `...Owned` / `...Canonical` compatibility layers should be
 * collapsed by the grammar assembler rather than generating duplicate parser
 * rules.
 *
 * The architectural requirement is ONE semantic owner, not multiple copies.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file requires repository-level tests for:
 *
 * POSITIVE
 * --------
 *
 *     apply H(q);
 *     apply custom.operation(q);
 *     apply operation(theta)(q);
 *     measure q;
 *     reset q;
 *     barrier q;
 *     observe observable;
 *     control (result) apply X(q);
 *     nested control;
 *     dynamic quantum statement;
 *     quantum/classical statement.
 *
 * NEGATIVE
 * --------
 *
 *     malformed operation;
 *     missing target;
 *     malformed measurement;
 *     malformed reset;
 *     malformed barrier;
 *     malformed observation;
 *     malformed control;
 *     malformed dynamic statement;
 *     invalid statement termination.
 *
 * BOUNDARY
 * --------
 *
 *     one quantum statement;
 *     empty quantum sequence;
 *     deeply nested quantum control;
 *     large target lists;
 *     large parameter lists;
 *     symbolic target expressions;
 *     symbolic resource expressions;
 *     large statement sequences.
 *
 * SCALABILITY
 * -----------
 *
 * Tests must demonstrate that the grammar does not establish arbitrary limits
 * on:
 *
 *     qubits;
 *     targets;
 *     controls;
 *     operations;
 *     parameters;
 *     statements;
 *     circuit depth.
 *
 * The benchmark sizes used by tests are implementation test values only and
 * must never become grammar limits.
 *
 * DETERMINISM
 * -----------
 *
 * Identical source must produce identical parse structure.
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing valid quantum syntax must remain accepted unless deliberately
 * deprecated by the language specification.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the architectural hard-coding rule only if it contains no
 * language-level constants representing:
 *
 *     MAX_QUBITS
 *     MAX_REGISTER_SIZE
 *     MAX_OPERATION_COUNT
 *     MAX_CONTROL_COUNT
 *     MAX_PARAMETER_COUNT
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICE_COUNT
 *     MAX_QPU_SIZE
 *
 * and no target-specific universal identifiers such as:
 *
 *     physical_qubit_0
 *     physical_qubit_1
 *     gpu0
 *     qpu0
 *     cpu0
 *
 * as language semantics.
 *
 * Ordinary user identifiers containing similar text remain ordinary source
 * identifiers and are not language-imposed resources.
 *
 * ============================================================================
 * DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [x] file has one clear ownership boundary
 *     [x] no lexer rules are duplicated
 *     [x] no expression grammar is duplicated
 *     [x] no type grammar is duplicated
 *     [x] no operation grammar is duplicated
 *     [x] no measurement grammar is duplicated
 *     [x] no reset grammar is duplicated
 *     [x] no observable grammar is duplicated
 *     [x] no dynamic-control grammar is duplicated
 *     [x] no hardware semantics are embedded
 *     [x] no physical resource limits are embedded
 *     [x] no quantum::ir is defined here
 *     [x] POCO-REAF is preserved
 *     [x] arbitrary semantic scale is preserved
 *     [x] deterministic parsing is preserved
 *     [x] safe-Rust implementation remains possible
 *     [x] integration contracts are explicit
 *     [x] downstream ownership is explicit
 *
 * Repository-level completion additionally requires:
 *
 *     [ ] statements.g4 includes quantumStatement
 *     [ ] quantum.g4 no longer competes for statement ownership
 *     [ ] observable naming/token mismatch is reconciled
 *     [ ] all aliases resolve to one effective production
 *     [ ] generated ANTLR parser has no duplicate-rule conflict
 *     [ ] Rust lexer/parser conformance passes
 *     [ ] AST mapping passes
 *     [ ] semantic mapping passes
 *     [ ] quantum::ir lowering passes
 *     [ ] positive tests pass
 *     [ ] negative tests pass
 *     [ ] boundary tests pass
 *     [ ] scalability tests pass
 *     [ ] determinism tests pass
 *     [ ] compatibility tests pass
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This file is a STATEMENT COMPOSITION BOUNDARY.
 *
 * It is not a gate catalogue.
 *
 * It is not a quantum simulator.
 *
 * It is not a QPU description.
 *
 * It is not a hardware description.
 *
 * It is not a routing system.
 *
 * It is not a scheduler.
 *
 * It is not QEC.
 *
 * It is not ZQN.
 *
 * It is not a runtime.
 *
 * It is the syntactic bridge that allows the language-wide statement system
 * to admit quantum computation while preserving the canonical architecture:
 *
 *     Zamani source
 *         ->
 *     AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing / scheduling
 *         ->
 *     QEC / ZQN / resilience
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * This separation is what permits the same Zamani source program to scale
 * from the smallest available computational substrate toward arbitrarily
 * larger systems whenever the semantic requirements can be satisfied by the
 * available resources and target capabilities.
 *
 * ============================================================================
 */

