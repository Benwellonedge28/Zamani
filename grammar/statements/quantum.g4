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
 * CANONICAL QUANTUM STATEMENT COMPOSITION BOUNDARY
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
 * This file is the SINGLE STATEMENT-LAYER OWNER for admitting quantum
 * statements into the language-wide `statement` grammar.
 *
 * It is intentionally a COMPOSITION GRAMMAR.
 *
 * It does NOT reimplement quantum syntax.
 *
 * Detailed quantum syntax remains owned by the appropriate files under:
 *
 *     grammar/quantum/
 *
 * In particular:
 *
 *     operations.g4
 *         quantum operation invocation
 *
 *     parameterized-operations.g4
 *         quantum operation parameter syntax
 *
 *     controlled-operations.g4
 *         controlled-operation syntax
 *
 *     measurement.g4
 *         measurement syntax
 *
 *     reset.g4
 *         reset syntax
 *
 *     gates.g4
 *         legacy/compatibility gate syntax and barrier ownership
 *
 *     observables.g4
 *         observable/observation syntax
 *
 *     mid-circuit-control.g4
 *         explicit quantum/classical mid-circuit dependency syntax
 *
 *     dynamic-circuits.g4
 *         dynamic quantum control-flow syntax
 *
 *     quantum-classical.g4
 *         quantum/classical boundary syntax
 *
 * This file MUST NOT duplicate any of those productions.
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
 *          +--> statement
 *                    |
 *                    +--> quantumStatement
 *                              |
 *                              +--> quantumOperationStatement
 *                              +--> quantumMeasurementStatement
 *                              +--> quantumResetStatement
 *                              +--> quantumBarrierStatement
 *                              +--> quantumObservationStatement
 *                              +--> quantumMidCircuitControlStatement
 *                              +--> quantumDynamicCircuitStatement
 *                              +--> quantumClassicalConstruct
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> quantum semantic validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
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
 * This file MUST remain upstream of all implementation decisions.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     - `quantumStatement`;
 *     - statement-level quantum family dispatch;
 *     - the relationship between the language-wide statement grammar and the
 *       detailed quantum grammar;
 *     - the canonical statement-layer integration boundary for quantum
 *       computation.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexer rules;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - declarations;
 *     - functions;
 *     - modules;
 *     - quantum operation syntax;
 *     - operation parameters;
 *     - operation targets;
 *     - gate definitions;
 *     - measurement syntax;
 *     - reset syntax;
 *     - observable syntax;
 *     - dynamic-circuit syntax;
 *     - mid-circuit-control syntax;
 *     - quantum/classical conversion syntax;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware;
 *     - resource discovery;
 *     - capability discovery;
 *     - physical allocation;
 *     - calibration;
 *     - backend selection;
 *     - runtime execution;
 *     - quantum::ir.
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH RULE
 * ============================================================================
 *
 * There MUST be exactly ONE effective `quantumStatement` rule in the
 * production parser.
 *
 * This file owns that rule at the statement-layer composition boundary.
 *
 * No detailed quantum grammar may redefine `quantumStatement`.
 *
 * `grammar/quantum/quantum.g4` MUST NOT provide a competing effective
 * statement dispatcher in the final assembled parser.
 *
 * Its role remains quantum-domain orchestration/declaration composition.
 *
 * ============================================================================
 * CRITICAL DESIGN RULE
 * ============================================================================
 *
 * This file contains NO compatibility aliases of the form:
 *
 *     quantumObservationStatement
 *         -> quantumObservationStatementOwned
 *         -> quantumObservationStatementCanonical
 *         -> quantumObservationStatement
 *
 * or equivalent chains.
 *
 * Such aliases create recursive grammar cycles and do not provide a real
 * integration boundary.
 *
 * The detailed owner is referenced DIRECTLY.
 *
 * ============================================================================
 * QUANTUM OPERATION MODEL
 * ============================================================================
 *
 * Quantum operations remain OPEN-WORLD semantic names.
 *
 * This file does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     U
 *
 * or any vendor/future operation.
 *
 * Examples such as:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(control, target);
 *     apply RX(theta)(q);
 *     apply custom.operation(q);
 *     apply vendor.operation(parameters)(targets);
 *
 * are admitted through the canonical:
 *
 *     quantumOperationStatement
 *
 * rule owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * Operation identity and meaning are resolved by semantic analysis.
 *
 * This is essential for POCO-REAF and future extensibility.
 *
 * ============================================================================
 * NO SECOND QUANTUM IR
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     QuantumGate
 *     QuantumOperationIR
 *     QuantumStatementIR
 *     QuantumInstructionIR
 *     PhysicalQuantumOperation
 *
 * or any other intermediate representation.
 *
 * Quantum syntax ultimately lowers through the existing canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * The frontend AST remains domain-neutral.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file expresses no machine-size assumptions.
 *
 * It contains no language-level limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     registers
 *     operations
 *     parameters
 *     targets
 *     controls
 *     measurements
 *     circuit depth
 *     dynamic branches
 *     loop iterations
 *     devices
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     nodes
 *     memory
 *     topology
 *     timelines
 *
 * It also contains no:
 *
 *     physical qubit IDs
 *     device IDs
 *     backend IDs
 *     vendor IDs
 *     coupling maps
 *     hardware addresses
 *     pulse durations
 *     calibration values
 *     scheduling slots
 *     native gate inventories
 *
 * A program may contain arbitrary semantic cardinality supported by the
 * language and represented by the source program.
 *
 * Actual limits are determined downstream by:
 *
 *     parser resources
 *     compiler resources
 *     semantic/resource policies
 *     target capabilities
 *     available resources
 *     runtime policies
 *     deployment constraints
 *
 * Those limits MUST NOT be converted into grammar constants.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Quantum computation syntax is separate from resource and capability intent.
 *
 * For example:
 *
 *     apply operation(q);
 *
 * describes computation.
 *
 * A requirement such as:
 *
 *     requires capability("quantum.measurement")
 *
 * is a capability/resource concern owned by the appropriate resource,
 * hardware, or requirement grammar.
 *
 * This file MUST NOT introduce:
 *
 *     use_qpu_0
 *     physical_qubit_17
 *     gpu_0
 *     cpu_0
 *
 * as language-level quantum realization semantics.
 *
 * Logical-to-physical mapping belongs downstream.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file consumes the repository's canonical lexer vocabulary indirectly
 * through the detailed quantum grammar owners.
 *
 * It MUST NOT declare lexer rules.
 *
 * It MUST NOT introduce:
 *
 *     K_APPLY
 *     K_MEASURE
 *     K_RESET
 *     K_BARRIER
 *     K_CONTROL
 *     K_OBSERVE
 *
 * or duplicate punctuation/token definitions.
 *
 * Token ownership remains in the canonical lexer/token layer.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * This file does not define `expression`.
 *
 * Quantum operation arguments and targets are handled by:
 *
 *     grammar/quantum/operations.g4
 *
 * Measurement targets/options are handled by:
 *
 *     grammar/quantum/measurement.g4
 *
 * Observable expressions are handled by:
 *
 *     grammar/quantum/observables.g4
 *
 * Dynamic conditions are handled by:
 *
 *     grammar/quantum/dynamic-circuits.g4
 *     grammar/quantum/mid-circuit-control.g4
 *
 * Quantum/classical expressions are handled by:
 *
 *     grammar/quantum/quantum-classical.g4
 *
 * This prevents the statement layer from creating a second expression
 * language.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file does not define quantum types.
 *
 * Quantum type syntax remains owned by:
 *
 *     grammar/quantum/quantum-types.g4
 *     grammar/types/
 *
 * Examples include:
 *
 *     Qubit
 *     Qubit[n]
 *     quantum registers
 *     logical quantum resources
 *
 * Type validity is a semantic concern.
 *
 * ============================================================================
 * STATEMENT FAMILIES
 * ============================================================================
 *
 * The canonical quantum statement dispatcher admits the following families.
 *
 * 1. Quantum operation
 *
 *     quantumOperationStatement
 *
 * 2. Measurement
 *
 *     quantumMeasurementStatement
 *
 * 3. Reset
 *
 *     quantumResetStatement
 *
 *     provided by the statement-level adapter below, delegating to the
 *     canonical reset owner.
 *
 * 4. Barrier
 *
 *     quantumBarrierStatement
 *
 *     provided by the statement-level adapter below, delegating to the
 *     existing barrier owner.
 *
 * 5. Observation
 *
 *     quantumObservationStatement
 *
 * 6. Mid-circuit control
 *
 *     quantumMidCircuitControlStatement
 *
 * 7. Dynamic quantum control flow
 *
 *     quantumDynamicCircuitStatement
 *
 * 8. Quantum/classical boundary
 *
 *     quantumClassicalConstruct
 *
 * The detailed syntax remains owned by the corresponding quantum grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CANONICAL QUANTUM STATEMENT DISPATCH
 * ============================================================================
 *
 * This is the only rule in this file that represents the complete
 * statement-level quantum family.
 *
 * Ordering is intentionally explicit.
 *
 * More specific quantum constructs are admitted before generic expression
 * statements by the language-wide statement dispatcher.
 *
 * This prevents keyword-led constructs such as:
 *
 *     apply
 *     measure
 *     reset
 *     observe
 *
 * from being accidentally treated as ordinary expressions.
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
    | quantumClassicalConstruct
    ;


/*
 * ============================================================================
 * RESET STATEMENT-LAYER ADAPTER
 * ============================================================================
 *
 * OWNERSHIP
 * ---------
 *
 * `grammar/quantum/reset.g4` is the detailed owner of:
 *
 *     quantumResetOperation
 *
 * This file owns only the statement-layer name:
 *
 *     quantumResetStatement
 *
 * The adapter contains no reset syntax of its own.
 *
 * Canonical lowering:
 *
 *     quantumResetStatement
 *          |
 *          v
 *     quantumResetOperation
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 */

quantumResetStatement
    : quantumResetOperation
    ;


/*
 * ============================================================================
 * BARRIER STATEMENT-LAYER ADAPTER
 * ============================================================================
 *
 * OWNERSHIP
 * ---------
 *
 * The repository's existing `grammar/quantum/gates.g4` owns:
 *
 *     barrierStatement
 *
 * This file does not duplicate its concrete syntax.
 *
 * The adapter creates the statement-layer quantum family name:
 *
 *     quantumBarrierStatement
 *
 * A barrier remains semantic ordering/synchronization intent.
 *
 * It does not encode:
 *
 *     hardware clock cycles
 *     pulse duration
 *     physical synchronization
 *     timing slots
 *     target topology
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 */

quantumBarrierStatement
    : barrierStatement
    ;


/*
 * ============================================================================
 * OPERATION INTEGRATION
 * ============================================================================
 *
 * The canonical operation rule is consumed directly.
 *
 * Owner:
 *
 *     grammar/quantum/operations.g4
 *
 * This file MUST NOT define another operation production.
 *
 * In particular, do not add:
 *
 *     quantumOperationStatement
 *         : APPLY ...
 *
 * here.
 *
 * Doing so would create two owners.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MEASUREMENT INTEGRATION
 * ============================================================================
 *
 * The canonical measurement rule is consumed directly.
 *
 * Owner:
 *
 *     grammar/quantum/measurement.g4
 *
 * This file MUST NOT duplicate:
 *
 *     measure
 *     measurement targets
 *     destinations
 *     measurement options
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * OBSERVATION INTEGRATION
 * ============================================================================
 *
 * The canonical observable grammar already owns:
 *
 *     quantumObservationStatement
 *
 * Therefore this file references it DIRECTLY.
 *
 * There is intentionally no:
 *
 *     quantumObservationStatementOwned
 *
 *     quantumObservationStatementCanonical
 *
 * compatibility chain.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MID-CIRCUIT CONTROL INTEGRATION
 * ============================================================================
 *
 * The canonical mid-circuit-control grammar already owns:
 *
 *     quantumMidCircuitControlStatement
 *
 * Therefore this file references that production directly.
 *
 * This file does not duplicate:
 *
 *     condition syntax
 *     control syntax
 *     measurement dependencies
 *     feedback syntax
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DYNAMIC-CIRCUIT INTEGRATION
 * ============================================================================
 *
 * The canonical dynamic-circuit grammar already owns:
 *
 *     quantumDynamicCircuitStatement
 *
 * Therefore this file references that production directly.
 *
 * This file does not duplicate:
 *
 *     if
 *     else
 *     dynamic loops
 *     dynamic blocks
 *     runtime conditions
 *     conditioned quantum operations
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM / CLASSICAL INTEGRATION
 * ============================================================================
 *
 * `grammar/quantum/quantum-classical.g4` owns the detailed syntax of the
 * quantum/classical boundary.
 *
 * Its canonical family entry point is:
 *
 *     quantumClassicalConstruct
 *
 * This file admits that family as a statement.
 *
 * The construct may represent:
 *
 *     - quantum invocation from classical code;
 *     - classical binding from quantum results;
 *     - classical control of quantum computation;
 *     - quantum parameter binding;
 *     - hybrid value binding;
 *     - explicit quantum/classical conversion;
 *     - synchronization;
 *     - hybrid requirements/capabilities.
 *
 * No hardware implementation is selected here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DOMAIN-NEUTRAL INTEGRATION
 * ============================================================================
 *
 * Quantum statements are only one statement family in Zamani.
 *
 * The language-wide statement dispatcher in:
 *
 *     grammar/statements/statements.g4
 *
 * remains the owner of:
 *
 *     statement
 *
 * It should admit:
 *
 *     quantumStatement
 *
 * as one domain family.
 *
 * Conceptually:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | controlFlowStatement
 *         | assertionStatement
 *         | quantumStatement
 *         | ...
 *         ;
 *
 * `statements.g4` remains responsible for deciding when a quantum statement
 * is legal in ordinary statement position.
 *
 * This file does not redefine `statement`.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DECLARATION SEPARATION
 * ============================================================================
 *
 * Quantum declarations are NOT statements owned here.
 *
 * Examples:
 *
 *     qubit declarations
 *     quantum register declarations
 *     circuit declarations
 *     observable declarations
 *     quantum resource declarations
 *     quantum capability declarations
 *
 * remain owned by their declaration/domain grammar.
 *
 * The language-wide declaration dispatcher is responsible for admitting them
 * where declarations are legal.
 *
 * This distinction prevents declaration syntax from being duplicated inside
 * the statement grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GENERAL CONTROL-FLOW SEPARATION
 * ============================================================================
 *
 * Zamani already has general control-flow grammar.
 *
 * Therefore ordinary:
 *
 *     if
 *     else
 *     while
 *     for
 *     match
 *     break
 *     continue
 *     return
 *
 * remain owned by the corresponding general statement grammar.
 *
 * Quantum-specific dynamic dependencies remain available through:
 *
 *     quantumMidCircuitControlStatement
 *     quantumDynamicCircuitStatement
 *
 * This prevents the quantum grammar from becoming a second general-purpose
 * control-flow language.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARDWARE / RESOURCE SEPARATION
 * ============================================================================
 *
 * This file does not select a target.
 *
 * It does not contain:
 *
 *     cpu
 *     gpu
 *     fpga
 *     asic
 *     qpu
 *     simulator
 *     cluster
 *     device
 *
 * as universal quantum statement semantics.
 *
 * Hardware intent remains owned by:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *
 * Compilation and deployment intent remains owned by:
 *
 *     grammar/compile/
 *     grammar/execution/
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM::IR INTEGRATION
 * ============================================================================
 *
 * The parser does not construct quantum::ir.
 *
 * The required path is:
 *
 *     quantum statement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *
 * For operation statements, the AST must preserve at least:
 *
 *     operation name/designator
 *     generic arguments where present
 *     parameter/value arguments
 *     target expressions
 *     operation modifiers
 *     source span
 *     attributes/modifiers relevant to semantics
 *
 * For measurement:
 *
 *     measurement target
 *     destination
 *     options
 *     source span
 *
 * For reset:
 *
 *     reset targets
 *     source span
 *
 * For barrier:
 *
 *     barrier targets
 *     source span
 *
 * For observation:
 *
 *     observable expression
 *     destination where applicable
 *     source span
 *
 * For dynamic control:
 *
 *     condition
 *     controlled body
 *     nested dynamic structure
 *     source span
 *
 * For hybrid constructs:
 *
 *     quantum/classical boundary
 *     values
 *     conditions
 *     synchronization structure
 *     source span
 *
 * No physical resource identity is injected merely because parsing succeeded.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GENERIC OPERATION / GATE PRINCIPLE
 * ============================================================================
 *
 * The parser remains open-world.
 *
 * These are examples, not grammar alternatives:
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
 *     custom_gate
 *     library::operation
 *     vendor::operation
 *     future::operation
 *
 * The semantic layer determines:
 *
 *     whether the operation exists;
 *     its arity;
 *     its parameter contract;
 *     its target contract;
 *     its capabilities;
 *     its resource requirements;
 *     its canonical quantum semantics.
 *
 * This is the required open-world model for:
 *
 *     standard operations;
 *     user operations;
 *     library operations;
 *     dialect operations;
 *     vendor operations;
 *     future operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CONTROL / MODIFIER PRINCIPLE
 * ============================================================================
 *
 * Controlled, adjoint, inverse, and other operation modifiers are NOT
 * reimplemented here.
 *
 * They are owned by the detailed quantum operation grammar.
 *
 * This file only admits the resulting operation statement through:
 *
 *     quantumOperationStatement
 *
 * Therefore there is no fixed:
 *
 *     maximum number of controls;
 *     maximum modifier nesting depth;
 *     maximum target count.
 *
 * Semantic validation determines whether a particular composition is valid.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MEASUREMENT PRINCIPLE
 * ============================================================================
 *
 * Measurement remains an explicit quantum statement family.
 *
 * The measurement grammar supports:
 *
 *     measurement targets;
 *     optional destinations;
 *     semantic options.
 *
 * This file does not impose:
 *
 *     bit width;
 *     register width;
 *     target count;
 *     measurement count;
 *     observable size;
 *     result count;
 *     readout technology;
 *     readout latency.
 *
 * Those are semantic/target concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESET PRINCIPLE
 * ============================================================================
 *
 * Reset remains an explicit quantum statement family.
 *
 * The reset grammar determines source syntax.
 *
 * Semantic analysis determines:
 *
 *     target validity;
 *     reset-state semantics;
 *     resource requirements;
 *     whether reset is supported by a target;
 *     how reset is lowered.
 *
 * This grammar does not select:
 *
 *     active reset;
 *     passive reset;
 *     measurement/reset sequence;
 *     pulse reset;
 *     simulator state replacement;
 *     fault-tolerant reset implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * BARRIER PRINCIPLE
 * ============================================================================
 *
 * A source barrier represents a semantic ordering/synchronization boundary.
 *
 * It does not mean:
 *
 *     one hardware clock;
 *     one pulse boundary;
 *     one scheduler slot;
 *     one physical synchronization event.
 *
 * Those interpretations belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DYNAMIC QUANTUM COMPUTATION
 * ============================================================================
 *
 * The grammar supports source programs in which quantum execution depends on
 * values produced during execution.
 *
 * Examples include:
 *
 *     measure q -> result;
 *
 *     control (result) apply correction(target);
 *
 *     dynamic conditional computation;
 *
 *     measurement-dependent quantum operations.
 *
 * The grammar does not decide whether the target supports:
 *
 *     native dynamic circuits;
 *     host feedback;
 *     controller feedback;
 *     deferred execution;
 *     simulation branching.
 *
 * That is target lowering/runtime policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HYBRID COMPUTATION
 * ============================================================================
 *
 * Quantum/classical integration remains first-class.
 *
 * A Zamani program may combine:
 *
 *     classical computation
 *     quantum computation
 *     measurement
 *     classical feedback
 *     dynamic control
 *     data movement
 *     resource requirements
 *
 * without becoming two separate languages.
 *
 * The quantum statement boundary remains compatible with:
 *
 *     classical IR
 *     quantum::ir
 *
 * through semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE CO-DESIGN
 * ============================================================================
 *
 * Quantum statements may participate in a larger hardware/software
 * co-design program.
 *
 * This file nevertheless does not own:
 *
 *     wires;
 *     buses;
 *     fixed widths;
 *     clocks;
 *     physical addresses;
 *     FPGA resources;
 *     ASIC resources;
 *     physical placement;
 *     hardware topology.
 *
 * Those belong to:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * Quantum computation remains a semantic computation domain.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QEC / ZQN / RESILIENCE
 * ============================================================================
 *
 * Quantum error correction, fault/noise semantics, and resilience remain
 * downstream.
 *
 * This file does not encode:
 *
 *     code distance;
 *     decoder implementation;
 *     syndrome layout;
 *     physical error rates;
 *     noise probabilities;
 *     calibration values;
 *     recovery algorithms;
 *     retry policies;
 *     device-specific fault models.
 *
 * The semantic/IR pipeline supplies the information required by:
 *
 *     QEC
 *     ZQN
 *     resilience
 *
 * after quantum semantics have been established.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ROUTING / SCHEDULING
 * ============================================================================
 *
 * Source-level ordering is semantic.
 *
 * Physical ordering is not.
 *
 * This file never selects:
 *
 *     physical qubit 0;
 *     physical qubit 1;
 *     coupling edge;
 *     SWAP insertion;
 *     machine cycle;
 *     pulse slot;
 *     hardware queue.
 *
 * Routing and scheduling consume canonical semantic/IR information.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no filesystem access;
 *     - no networking;
 *     - no process execution;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no mutable global state.
 *
 * Given identical:
 *
 *     source tokens;
 *     lexer vocabulary;
 *     grammar version;
 *     parser configuration;
 *
 * the structural parse must be deterministic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This file is responsible for the statement-family boundary only.
 *
 * Structural syntax errors belong to the detailed grammar owners.
 *
 * Examples:
 *
 *     malformed operation
 *     malformed measurement
 *     malformed reset
 *     malformed barrier
 *     malformed observation
 *     malformed dynamic control
 *     malformed hybrid boundary
 *
 * Semantic diagnostics remain downstream.
 *
 * Examples:
 *
 *     unknown operation
 *     invalid operation arity
 *     invalid target type
 *     invalid quantum resource
 *     unsupported measurement basis
 *     unsupported modifier
 *     unavailable capability
 *     insufficient resources
 *     invalid dynamic dependency
 *     invalid quantum/classical conversion
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file deliberately preserves the existing detailed quantum rule names
 * wherever they already represent the canonical owner.
 *
 * No unnecessary renames are introduced.
 *
 * Existing names retained include:
 *
 *     quantumOperationStatement
 *     quantumMeasurementStatement
 *     quantumResetOperation
 *     barrierStatement
 *     quantumObservationStatement
 *     quantumMidCircuitControlStatement
 *     quantumDynamicCircuitStatement
 *     quantumClassicalConstruct
 *
 * The only new statement-layer adapters are:
 *
 *     quantumResetStatement
 *     quantumBarrierStatement
 *
 * because the existing detailed owners expose:
 *
 *     quantumResetOperation
 *     barrierStatement
 *
 * respectively.
 *
 * No alias is introduced where the canonical owner already has the desired
 * statement name.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUIRED INTEGRATION WITH OTHER FILES
 * ============================================================================
 *
 * This file is complete independently as a statement composition boundary,
 * provided the following repository contracts are respected.
 *
 * --------------------------------------------------------------------------
 * 1. grammar/statements/statements.g4
 * --------------------------------------------------------------------------
 *
 * The language-wide statement dispatcher must admit:
 *
 *     quantumStatement
 *
 * exactly once.
 *
 * It must NOT copy the alternatives from this file.
 *
 *
 * --------------------------------------------------------------------------
 * 2. grammar/quantum/operations.g4
 * --------------------------------------------------------------------------
 *
 * Must remain the sole detailed owner of:
 *
 *     quantumOperationStatement
 *
 * No gate catalogue is added here.
 *
 *
 * --------------------------------------------------------------------------
 * 3. grammar/quantum/measurement.g4
 * --------------------------------------------------------------------------
 *
 * Must remain the sole detailed owner of:
 *
 *     quantumMeasurementStatement
 *
 *
 * --------------------------------------------------------------------------
 * 4. grammar/quantum/reset.g4
 * --------------------------------------------------------------------------
 *
 * Must remain the sole detailed owner of:
 *
 *     quantumResetOperation
 *
 * This file adapts that rule to:
 *
 *     quantumResetStatement
 *
 *
 * --------------------------------------------------------------------------
 * 5. grammar/quantum/gates.g4
 * --------------------------------------------------------------------------
 *
 * Must remain the existing owner of:
 *
 *     barrierStatement
 *
 * This file adapts it to:
 *
 *     quantumBarrierStatement
 *
 * The legacy fixed-gate grammar MUST NOT become the canonical operation
 * vocabulary.
 *
 *
 * --------------------------------------------------------------------------
 * 6. grammar/quantum/observables.g4
 * --------------------------------------------------------------------------
 *
 * Must remain the owner of:
 *
 *     quantumObservationStatement
 *
 * No compatibility recursion is permitted.
 *
 *
 * --------------------------------------------------------------------------
 * 7. grammar/quantum/mid-circuit-control.g4
 * --------------------------------------------------------------------------
 *
 * Must remain the owner of:
 *
 *     quantumMidCircuitControlStatement
 *
 * This file references it directly.
 *
 *
 * --------------------------------------------------------------------------
 * 8. grammar/quantum/dynamic-circuits.g4
 * --------------------------------------------------------------------------
 *
 * Must remain the owner of:
 *
 *     quantumDynamicCircuitStatement
 *
 * This file references it directly.
 *
 *
 * --------------------------------------------------------------------------
 * 9. grammar/quantum/quantum-classical.g4
 * --------------------------------------------------------------------------
 *
 * Must expose:
 *
 *     quantumClassicalConstruct
 *
 * as the canonical quantum/classical statement-family entry point.
 *
 * This file does not duplicate its detailed constructs.
 *
 *
 * --------------------------------------------------------------------------
 * 10. grammar/quantum/quantum.g4
 * --------------------------------------------------------------------------
 *
 * It may continue to orchestrate the quantum domain and quantum declarations.
 *
 * It MUST NOT introduce another effective:
 *
 *     quantumStatement
 *
 * rule in the production parser.
 *
 * This avoids the previous competing-authority problem.
 *
 *
 * --------------------------------------------------------------------------
 * 11. grammar/quantum/parameterized-operations.g4
 * --------------------------------------------------------------------------
 *
 * Remains the owner of parameter declaration/binding syntax used by operation
 * grammars.
 *
 * This file does not duplicate parameter syntax.
 *
 *
 * --------------------------------------------------------------------------
 * 12. grammar/quantum/controlled-operations.g4
 * --------------------------------------------------------------------------
 *
 * Remains the owner of explicit controlled-operation syntax where that syntax
 * is consumed by the operation grammar.
 *
 * This file does not duplicate control syntax.
 *
 *
 * --------------------------------------------------------------------------
 * 13. grammar/quantum/quantum-types.g4
 * --------------------------------------------------------------------------
 *
 * Remains the quantum type owner.
 *
 * No Qubit/Qubit[n]/register type syntax is introduced here.
 *
 *
 * --------------------------------------------------------------------------
 * 14. src/frontend/ast/
 * --------------------------------------------------------------------------
 *
 * Parsed quantum statements must lower into the existing domain-neutral AST.
 *
 * Generic operation semantics must remain compatible with the established
 * generic Operation model:
 *
 *     name
 *     namespace
 *     operands
 *     parameters
 *     results
 *     attributes
 *     modifiers
 *     effects
 *     capabilities
 *     source
 *
 * The grammar must not require a `QuantumGate` enum.
 *
 *
 * --------------------------------------------------------------------------
 * 15. src/quantum/ir/
 * --------------------------------------------------------------------------
 *
 * Quantum semantic lowering must terminate at the existing canonical
 * quantum::ir boundary.
 *
 * This grammar must never create a second quantum IR.
 *
 *
 * --------------------------------------------------------------------------
 * 16. optimization / routing / scheduling / QEC / ZQN / HAL
 * --------------------------------------------------------------------------
 *
 * These systems consume downstream semantic/IR representations.
 *
 * No dependency from this grammar into those implementations is permitted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar deliberately contains no finite language-level limits on:
 *
 *     quantum statements
 *     operations
 *     parameters
 *     targets
 *     controls
 *     measurements
 *     dynamic branches
 *     nesting
 *     circuit depth
 *     program size
 *
 * Repetition is structural.
 *
 * The implementation may impose resource safeguards against pathological input
 * for parser/compiler safety, but such safeguards are implementation policies,
 * not language semantics.
 *
 * In particular, this file must never acquire:
 *
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_PARAMETERS
 *     MAX_MEASUREMENTS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *     MAX_QPU_SIZE
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust code.
 *
 * The generated parser/frontend integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * The source language's own `unsafe` constructs, if any, do not authorize
 * unsafe implementation code in the compiler.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file's repository-level tests must verify composition rather than
 * duplicate detailed quantum syntax tests.
 *
 * POSITIVE
 * --------
 *
 * The assembled parser must accept at least:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(control, target);
 *     apply RX(theta)(q);
 *     apply custom.operation(q);
 *     apply vendor.operation(theta)(targets);
 *
 *     measure q;
 *     measure q -> result;
 *
 *     reset q;
 *
 *     barrier q;
 *
 *     observe observable;
 *
 *     control (result) apply operation(target);
 *
 *     dynamic quantum control;
 *
 *     quantum/classical boundary constructs.
 *
 * These examples are semantic test cases, not gate inventories.
 *
 *
 * NEGATIVE
 * --------
 *
 * The assembled parser must reject structurally malformed forms such as:
 *
 *     apply;
 *     apply operation;
 *     apply operation(;
 *     measure;
 *     reset;
 *     barrier;
 *     observe;
 *     malformed control syntax;
 *     malformed dynamic syntax;
 *     malformed hybrid syntax.
 *
 *
 * BOUNDARY
 * --------
 *
 * Tests must cover:
 *
 *     one target;
 *     many targets;
 *     symbolic targets;
 *     indexed targets;
 *     ranges;
 *     parameterized operations;
 *     nested operation modifiers;
 *     multiple measurements;
 *     nested dynamic control;
 *     large statement sequences.
 *
 *
 * SCALABILITY
 * -----------
 *
 * Test sizes are implementation test inputs only.
 *
 * They must NOT establish language limits.
 *
 * The tests must demonstrate that the grammar itself contains no artificial
 * machine-size ceiling.
 *
 *
 * DETERMINISM
 * -----------
 *
 * The same source and parser configuration must produce the same structural
 * parse.
 *
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing valid quantum syntax owned by the detailed grammars must remain
 * accepted unless deliberately deprecated by the language specification.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the POCO-REAF hard-coding audit because it contains no:
 *
 *     MAX_QUBITS
 *     MAX_REGISTERS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_PARAMETERS
 *     MAX_MEASUREMENTS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *     MAX_QPU_SIZE
 *
 * and no target-specific universal resource identifiers.
 *
 * Program-level constants remain valid because they are source semantics and
 * are not compiler-imposed machine limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DEFINITION OF DONE
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 *     [x] `quantumStatement` is the sole statement-layer dispatcher;
 *     [x] operation syntax is delegated to operations.g4;
 *     [x] measurement syntax is delegated to measurement.g4;
 *     [x] reset syntax is delegated to reset.g4;
 *     [x] barrier syntax is delegated to gates.g4;
 *     [x] observation syntax is delegated to observables.g4;
 *     [x] mid-circuit control is delegated to mid-circuit-control.g4;
 *     [x] dynamic circuits are delegated to dynamic-circuits.g4;
 *     [x] hybrid syntax is delegated to quantum-classical.g4;
 *     [x] no recursive compatibility aliases exist;
 *     [x] no fixed gate inventory exists here;
 *     [x] no fixed resource limits exist here;
 *     [x] no hardware topology exists here;
 *     [x] no physical qubit mapping exists here;
 *     [x] no QEC implementation exists here;
 *     [x] no ZQN implementation exists here;
 *     [x] no routing exists here;
 *     [x] no scheduling exists here;
 *     [x] no quantum IR is defined here;
 *     [x] POCO-REAF is preserved;
 *     [x] the grammar remains target-independent;
 *     [x] the grammar remains safe-Rust compatible;
 *     [x] existing filenames remain unchanged.
 *
 * REPOSITORY-LEVEL COMPLETION ALSO REQUIRES:
 *
 *     [ ] statements.g4 admits `quantumStatement` exactly once;
 *     [ ] quantum.g4 does not provide a competing effective dispatcher;
 *     [ ] all referenced detailed rules are present in the assembled grammar;
 *     [ ] ANTLR assembly produces no duplicate-rule errors;
 *     [ ] ANTLR assembly produces no recursive alias cycle;
 *     [ ] Rust lexer/parser conformance passes;
 *     [ ] frontend AST mapping passes;
 *     [ ] semantic analysis passes;
 *     [ ] quantum::ir lowering passes;
 *     [ ] positive tests pass;
 *     [ ] negative tests pass;
 *     [ ] boundary tests pass;
 *     [ ] scalability tests pass;
 *     [ ] determinism tests pass;
 *     [ ] compatibility tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is deliberately boring.
 *
 * That is a production property.
 *
 * It is a statement composition boundary, not a second quantum language.
 *
 * The detailed quantum language remains distributed across its existing
 * ownership files, while this file provides exactly one stable bridge into
 * the universal Zamani statement system.
 *
 * The resulting architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     statement
 *          |
 *          v
 *     quantumStatement
 *          |
 *          +--> operations
 *          +--> measurement
 *          +--> reset
 *          +--> barrier
 *          +--> observation
 *          +--> mid-circuit control
 *          +--> dynamic circuit
 *          +--> quantum/classical
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
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
 * This preserves Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever:
 *
 * source semantics remain independent of the eventual machine, while
 * implementation, resource, topology, scheduling, routing, resilience and
 * hardware decisions are resolved downstream.
 *
 * ============================================================================
 */