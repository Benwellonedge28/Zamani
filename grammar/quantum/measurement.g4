/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/measurement.g4
 *
 * Grammar:
 *     QuantumMeasurement
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM MEASUREMENT GRAMMAR
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
 * This file is the SINGLE GRAMMAR OWNER for source-level quantum
 * measurement syntax.
 *
 * It describes:
 *
 *     WHAT a Zamani program requests to measure.
 *
 * It does NOT describe:
 *
 *     HOW the measurement is physically or computationally realized.
 *
 * Canonical pipeline:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     QuantumMeasurement
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic/type/effect/resource analysis
 *          |
 *          v
 *     canonical semantic representation
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
 *          +--> simulator
 *          |
 *          v
 *     target realization
 *
 * The grammar never bypasses the AST, semantic, or canonical IR boundaries.
 *
 * ============================================================================
 * GRAMMAR OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantumMeasurementStatement
 *     - quantumMeasurementTargetList
 *     - quantumMeasurementTarget
 *     - quantumMeasurementDestination
 *     - quantumMeasurementOptions
 *     - quantumMeasurementOptionList
 *     - quantumMeasurementOption
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - qubit declarations;
 *     - quantum registers;
 *     - quantum operations;
 *     - operation parameters;
 *     - controls;
 *     - adjoints;
 *     - reset statements;
 *     - observables;
 *     - dynamic classical control;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware topology;
 *     - physical qubit allocation;
 *     - backend selection;
 *     - runtime execution;
 *     - canonical quantum::ir types.
 *
 * Every production owned here must have exactly one canonical owner.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names + Expressions
 *          |
 *          v
 *     QuantumMeasurement
 *          |
 *          v
 *     quantum.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar MUST NOT import Operations, Controls, Reset, Observables,
 * Hardware, Resources, QEC, ZQN, or Runtime grammars merely to validate
 * semantic properties.
 *
 * Keeping this dependency direction prevents grammar cycles.
 *
 * ============================================================================
 * CANONICAL ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore this file consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT consume:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * ZamaniTokens is a lexical composition vocabulary. ZamaniLexer is the
 * production lexer boundary consumed by parser grammars.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Names supplies:
 *
 *     identifier
 *     qualified-name infrastructure where applicable
 *
 * Expressions supplies:
 *
 *     expression
 *
 * No quantum-specific target grammar is imported here.
 *
 * A measurement target is deliberately represented as an ordinary expression.
 *
 * Semantic analysis determines whether that expression denotes a valid
 * measurable quantum resource.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar consumes canonical lexical tokens.
 *
 * Required tokens:
 *
 *     MEASURE
 *     WITH
 *     THIN_ARROW
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     ASSIGN
 *     SEMICOLON
 *
 * This file introduces NO lexer tokens.
 *
 * In particular, it does not introduce:
 *
 *     K_MEASURE
 *     K_WITH
 *     MEASUREMENT_TOKEN
 *     BASIS_TOKEN
 *     OBSERVABLE_TOKEN
 *     MODE_TOKEN
 *     KIND_TOKEN
 *     RESULT_TOKEN
 *
 * Measurement option names remain ordinary identifiers.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Measurement syntax is target-independent.
 *
 * This file MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CLASSICAL_BITS
 *     MAX_MEASUREMENTS
 *     MAX_RESULTS
 *     MAX_SHOTS
 *     MAX_REGISTER_WIDTH
 *     MAX_OBSERVABLE_SIZE
 *     MAX_PAULI_WEIGHT
 *     MAX_CIRCUIT_DEPTH
 *     MAX_RESULT_BUFFER
 *     MAX_DEVICES
 *     MAX_READOUT_CHANNELS
 *
 * It MUST NOT encode:
 *
 *     qpu0
 *     qpu1
 *     physical_qubit_0
 *     physical_qubit_1
 *     readout_channel_0
 *     device_0
 *
 * It MUST NOT encode:
 *
 *     READOUT_FREQUENCY
 *     READOUT_DURATION
 *     HARDWARE_TOPOLOGY
 *     VENDOR
 *     BACKEND
 *
 * A measurement program may therefore scale from a single quantum resource
 * to arbitrarily large source-level resource collections, subject only to
 * actual compiler, runtime, semantic, and target resources.
 *
 * "Unbounded by grammar" does not mean physically infinite.
 *
 * It means the language does not establish an artificial universal ceiling.
 *
 * ============================================================================
 * CORE SOURCE FORMS
 * ============================================================================
 *
 * Minimal:
 *
 *     measure q;
 *
 * Multiple targets:
 *
 *     measure q0, q1, q2;
 *
 * Indexed target:
 *
 *     measure register[i];
 *
 * Slice/range target where supported by the expression grammar:
 *
 *     measure register[start .. end];
 *
 * Result destination:
 *
 *     measure q -> result;
 *
 * Multiple targets to one semantic destination:
 *
 *     measure q0, q1 -> result;
 *
 * Semantic options:
 *
 *     measure q with {
 *         basis = X
 *     };
 *
 * Destination plus options:
 *
 *     measure q -> result with {
 *         basis = X,
 *         mode = destructive
 *     };
 *
 * Empty options:
 *
 *     measure q with {};
 *
 * No option vocabulary is hard-coded into the grammar.
 *
 * ============================================================================
 * MEASUREMENT SEMANTICS
 * ============================================================================
 *
 * This grammar represents measurement INTENT.
 *
 * For example:
 *
 *     measure q with {
 *         basis = X
 *     };
 *
 * means that the program requests an X-basis measurement semantically.
 *
 * It does NOT specify:
 *
 *     - a physical pulse;
 *     - a detector;
 *     - an ADC;
 *     - a readout channel;
 *     - a physical qubit;
 *     - a calibration;
 *     - a sampling device;
 *     - a vendor instruction.
 *
 * Such realization belongs downstream.
 *
 * ============================================================================
 * MEASUREMENT TARGET CONTRACT
 * ============================================================================
 *
 * A target is an ordinary Zamani expression.
 *
 * Examples:
 *
 *     q
 *     q[i]
 *     register
 *     register[i]
 *     register[start .. end]
 *     logical_qubit
 *     selected
 *     selection[index]
 *     quantum_expression
 *
 * The parser does NOT decide whether the expression is quantum.
 *
 * Semantic analysis determines:
 *
 *     - whether the target resolves;
 *     - whether it has a measurable quantum type;
 *     - whether it is initialized;
 *     - whether its ownership/lifetime is valid;
 *     - whether measurement is permitted in the current effect context;
 *     - whether target overlap is legal;
 *     - whether the target is compatible with the measurement semantics.
 *
 * ============================================================================
 * TARGET CARDINALITY
 * ============================================================================
 *
 * Target cardinality is structural.
 *
 * This:
 *
 *     quantumMeasurementTargetList
 *
 * uses repetition rather than a fixed-size alternative.
 *
 * There is intentionally no:
 *
 *     measurement2
 *     measurement4
 *     measurement8
 *     measurement32
 *
 * and no finite target-count ceiling.
 *
 * ============================================================================
 * RESULT DESTINATION CONTRACT
 * ============================================================================
 *
 * The optional destination is an ordinary expression:
 *
 *     measure q -> result;
 *
 *     measure q -> result[index];
 *
 *     measure q -> register;
 *
 *     measure q -> destination;
 *
 * The grammar does not determine the destination's type or cardinality.
 *
 * Semantic/type analysis determines whether the destination can receive the
 * measurement result.
 *
 * This preserves portability across:
 *
 *     single-bit results;
 *     bit collections;
 *     registers;
 *     tuples;
 *     arrays;
 *     structured result objects;
 *     observable results;
 *     future result representations.
 *
 * ============================================================================
 * OPTION CONTRACT
 * ============================================================================
 *
 * Measurement options use an open key/value form:
 *
 *     identifier = expression
 *
 * Examples:
 *
 *     basis = X
 *     observable = PauliX
 *     kind = projective
 *     mode = destructive
 *     reset = true
 *     grouping = group
 *     result_type = bit
 *
 * The grammar intentionally does NOT enumerate these names.
 *
 * The semantic layer owns the authoritative measurement-option registry.
 *
 * This permits future measurement semantics without requiring a new lexer
 * keyword or grammar production for every new concept.
 *
 * ============================================================================
 * UNKNOWN OPTIONS
 * ============================================================================
 *
 * Unknown options are syntactically valid:
 *
 *     measure q with {
 *         future_measurement_feature = value
 *     };
 *
 * Semantic analysis decides whether the option:
 *
 *     - is stable;
 *     - is experimental;
 *     - belongs to an active dialect;
 *     - is deprecated;
 *     - is unsupported;
 *     - requires a capability;
 *     - requires a language version.
 *
 * The parser must not silently discard the option.
 *
 * ============================================================================
 * DUPLICATE OPTIONS
 * ============================================================================
 *
 * Duplicate option names remain syntactically representable:
 *
 *     measure q with {
 *         basis = X,
 *         basis = Z
 *     };
 *
 * The parser preserves both options.
 *
 * Semantic validation MUST determine whether:
 *
 *     - duplicates are forbidden;
 *     - duplicates are mergeable;
 *     - later values override earlier values;
 *     - a dialect defines a legal repeated option.
 *
 * The grammar must not silently choose one.
 *
 * ============================================================================
 * OPTION ORDER
 * ============================================================================
 *
 * Option order is source information.
 *
 * The AST should preserve option order until semantic normalization.
 *
 * This supports:
 *
 *     diagnostics;
 *     formatting;
 *     source mapping;
 *     provenance;
 *     IDE tooling;
 *     compatibility diagnostics.
 *
 * Semantic normalization may later canonicalize equivalent option sets.
 *
 * ============================================================================
 * TRAILING COMMAS
 * ============================================================================
 *
 * A trailing comma is accepted in:
 *
 *     quantumMeasurementTargetList
 *     quantumMeasurementOptionList
 *
 * Examples:
 *
 *     measure q0, q1,;
 *
 *     measure q with {
 *         basis = X,
 *     };
 *
 * The policy is intentionally explicit and deterministic.
 *
 * If the repository-wide syntax policy later forbids trailing commas, that
 * change must be made through the canonical syntax/compatibility process,
 * rather than independently changing this file.
 *
 * ============================================================================
 * EMPTY OPTION BLOCK
 * ============================================================================
 *
 * This is syntactically valid:
 *
 *     measure q with {};
 *
 * Semantic analysis may normalize it to default measurement semantics.
 *
 * ============================================================================
 * MEASUREMENT BASIS
 * ============================================================================
 *
 * Basis values are expressions rather than lexer-level keywords.
 *
 * Therefore all of the following can be represented structurally:
 *
 *     basis = X
 *     basis = Y
 *     basis = Z
 *     basis = my_basis
 *     basis = custom_basis
 *     basis = basis_expression
 *
 * The grammar does not define:
 *
 *     XBasis
 *     YBasis
 *     ZBasis
 *     PauliBasis
 *
 * as special syntax.
 *
 * Semantic analysis maps recognized values into the canonical quantum
 * semantic model.
 *
 * ============================================================================
 * OBSERVABLES
 * ============================================================================
 *
 * Measurement may reference an observable through an option:
 *
 *     measure q with {
 *         observable = my_observable
 *     };
 *
 * This file does NOT own observable declaration syntax.
 *
 * Observable declarations remain owned by:
 *
 *     grammar/quantum/observables.g4
 *
 * This file only preserves the source-level reference/expression.
 *
 * ============================================================================
 * MEASUREMENT KIND
 * ============================================================================
 *
 * Measurement kind is semantic data.
 *
 * Examples may include:
 *
 *     kind = projective
 *     kind = generalized
 *     kind = weak
 *     kind = continuous
 *
 * The grammar does not restrict the universe of future kinds.
 *
 * Semantic analysis determines which kinds are supported and what they mean.
 *
 * ============================================================================
 * MEASUREMENT MODE
 * ============================================================================
 *
 * Mode is semantic data.
 *
 * Examples:
 *
 *     mode = destructive
 *     mode = non_destructive
 *
 * The grammar does not specify how a backend implements the mode.
 *
 * ============================================================================
 * RESET-AFTER-MEASUREMENT
 * ============================================================================
 *
 * A measurement option may express semantic reset intent:
 *
 *     measure q with {
 *         reset = true
 *     };
 *
 * This does NOT redefine the standalone reset syntax.
 *
 * Standalone reset remains owned by:
 *
 *     grammar/quantum/reset.g4
 *
 * Semantic analysis determines whether the measurement-plus-reset request
 * lowers to one semantic operation, a measurement followed by reset, or
 * another valid canonical representation.
 *
 * ============================================================================
 * DYNAMIC CIRCUITS
 * ============================================================================
 *
 * A measurement result may feed subsequent classical control:
 *
 *     measure q -> result;
 *
 *     if result {
 *         ...
 *     }
 *
 * This file owns only:
 *
 *     measurement
 *
 * It does not own:
 *
 *     if
 *     match
 *     loops
 *     branching
 *     classical control flow
 *
 * Dynamic-circuit semantics remain downstream and in their dedicated grammar
 * components.
 *
 * ============================================================================
 * QUANTUM/CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Measurement creates a semantic boundary between quantum information and
 * classical information.
 *
 * The grammar preserves that boundary structurally through:
 *
 *     quantumMeasurementTarget
 *     quantumMeasurementDestination
 *
 * Semantic analysis determines:
 *
 *     quantum input type
 *     classical result type
 *     ownership/lifetime effects
 *     synchronization requirements
 *     feed-forward requirements
 *     effect annotations
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must map into the existing domain-neutral frontend AST.
 *
 * The parser must preserve at least:
 *
 *     - source span;
 *     - measurement keyword span where supported;
 *     - ordered target expressions;
 *     - optional destination expression;
 *     - ordered measurement options;
 *     - option-name source span;
 *     - option-value expression;
 *     - delimiters/spans needed for diagnostics and tooling.
 *
 * It MUST NOT introduce:
 *
 *     QuantumMeasurementIr
 *     QuantumMeasurementNodeWithHardware
 *     PhysicalMeasurementNode
 *     BackendMeasurementNode
 *
 * unless such a representation is already part of the canonical domain-neutral
 * AST contract.
 *
 * The preferred semantic flow is:
 *
 *     syntax
 *       |
 *       v
 *     generic/domain-neutral AST
 *       |
 *       v
 *     semantic quantum measurement
 *       |
 *       v
 *     quantum::ir
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - target resolution;
 *     - target quantum typing;
 *     - result destination typing;
 *     - option resolution;
 *     - option duplication rules;
 *     - basis validation;
 *     - observable validation;
 *     - measurement-kind validation;
 *     - measurement-mode validation;
 *     - reset semantics;
 *     - effect checking;
 *     - ownership/lifetime checking;
 *     - resource requirements;
 *     - capability requirements;
 *     - dynamic-circuit legality;
 *     - measurement ordering constraints;
 *     - language-version compatibility.
 *
 * Syntax-valid source is not necessarily semantically valid source.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar does not decide resource availability.
 *
 * Semantic/resource analysis may derive requirements such as:
 *
 *     capability("quantum.measurement")
 *
 *     capability("quantum.mid_circuit_measurement")
 *
 *     capability("quantum.observable_measurement")
 *
 *     capability("quantum.non_destructive_measurement")
 *
 *     requires memory >= required_memory
 *
 *     requires capability("classical.feedforward")
 *
 * The grammar does not contain resource limits.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar must not define a second quantum IR.
 *
 * Semantic lowering may construct the existing canonical measurement
 * representation containing whatever the repository's quantum::ir contract
 * requires, such as:
 *
 *     measurement kind
 *     measurement mode
 *     observable/basis
 *     logical operands
 *     result destination
 *     effects
 *     resource requirements
 *     source provenance
 *
 * Exact Rust types belong to the canonical quantum::ir implementation,
 * not this grammar.
 *
 * ============================================================================
 * QEC / ZQN / RESILIENCE
 * ============================================================================
 *
 * Measurement may be consumed by:
 *
 *     QEC
 *     ZQN
 *     resilience
 *
 * but none of those systems are implemented here.
 *
 * This file does NOT define:
 *
 *     syndrome extraction
 *     decoding
 *     readout error models
 *     noise channels
 *     mitigation algorithms
 *     retry policies
 *     recovery policies
 *     backend switching
 *
 * Those systems consume semantic/IR information downstream.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     physical readout channel
 *     ADC
 *     detector
 *     topology
 *     vendor
 *     backend
 *
 * Hardware realization occurs only after semantic analysis and canonical IR.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The parser must not inspect:
 *
 *     hardware;
 *     available QPUs;
 *     CPU count;
 *     GPU count;
 *     memory capacity;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     randomness;
 *     wall-clock time.
 *
 * Identical source token streams under the same language/dialect configuration
 * must produce equivalent parse structures.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no process execution;
 *     no hardware access;
 *     no runtime calls;
 *     no dynamic code execution.
 *
 * Safe Rust requirements therefore remain downstream implementation
 * requirements:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     no unsafe Rust
 *
 * ============================================================================
 * PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * The grammar uses structural repetition:
 *
 *     *
 *     +
 *
 * rather than fixed-size alternatives.
 *
 * The language therefore has no grammar-level maximum for:
 *
 *     measurement targets;
 *     measurement options;
 *     source-level measurement statements.
 *
 * Practical parser/compiler limits are implementation/resource limits, not
 * language semantics.
 *
 * Generated source may therefore represent very large target collections
 * provided the implementation has sufficient resources.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural syntax errors include:
 *
 *     measure
 *     measure ;
 *     measure ,
 *     measure q ->
 *     measure q with
 *     measure q with {
 *     measure q with { basis }
 *     measure q with { = X }
 *     measure q with { basis = }
 *
 * Semantic errors are NOT parser errors.
 *
 * Examples:
 *
 *     measure classical_value;
 *     measure unresolved_name;
 *     measure non_quantum_expression;
 *
 * may be syntactically valid and must reach semantic analysis.
 *
 * Semantic diagnostics may include:
 *
 *     invalid measurement target;
 *     invalid result destination;
 *     unsupported measurement kind;
 *     unsupported measurement mode;
 *     invalid basis;
 *     invalid observable;
 *     conflicting options;
 *     unavailable capability;
 *     insufficient resources;
 *     invalid dynamic-circuit context.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical source form:
 *
 *     measure <targets>;
 *
 * remains supported.
 *
 * Existing result form:
 *
 *     measure <targets> -> <destination>;
 *
 * remains supported.
 *
 * Existing option form:
 *
 *     measure <targets> with { ... };
 *
 * remains supported.
 *
 * This file does not rename the existing measurement grammar.
 *
 * Compatibility-affecting syntax changes must go through:
 *
 *     grammar/compatibility/
 *
 *     grammar/spec/compatibility.md
 *
 *     grammar/grammar.md
 *
 * and the language-version policy.
 *
 * ============================================================================
 * INTEGRATION WITH quantum.g4
 * ============================================================================
 *
 * quantum.g4 is the quantum composition/orchestration layer.
 *
 * It must import this parser grammar and route:
 *
 *     quantumMeasurementStatement
 *
 * through:
 *
 *     quantumMeasurementElement
 *
 * as already established by the quantum grammar architecture.
 *
 * quantum.g4 MUST NOT redefine:
 *
 *     quantumMeasurementStatement
 *     quantumMeasurementTargetList
 *     quantumMeasurementTarget
 *     quantumMeasurementDestination
 *     quantumMeasurementOptions
 *     quantumMeasurementOptionList
 *     quantumMeasurementOption
 *
 * ============================================================================
 * INTEGRATION WITH reset.g4
 * ============================================================================
 *
 * reset.g4 owns standalone reset syntax.
 *
 * It must not import this file merely to implement reset.
 *
 * Measurement reset intent is represented only as a measurement option.
 *
 * ============================================================================
 * INTEGRATION WITH observables.g4
 * ============================================================================
 *
 * observables.g4 owns observable declarations and observable-specific syntax.
 *
 * This file accepts an observable expression through:
 *
 *     measurement option
 *
 * and leaves observable resolution to semantic analysis.
 *
 * ============================================================================
 * INTEGRATION WITH operations.g4
 * ============================================================================
 *
 * operations.g4 owns operation invocation.
 *
 * A measurement is NOT represented as:
 *
 *     apply measure(...)
 *
 * in the canonical grammar.
 *
 * `MEASURE` has dedicated measurement-statement syntax because measurement
 * has a semantic quantum/classical boundary.
 *
 * ============================================================================
 * INTEGRATION WITH controls.g4
 * ============================================================================
 *
 * controls.g4 owns quantum operation control modifiers.
 *
 * Measurement does not import controls.g4.
 *
 * If a future measurement feature needs control semantics, it must be added
 * through a separate semantic contract or a deliberately specified extension,
 * not by creating a circular grammar dependency.
 *
 * ============================================================================
 * INTEGRATION WITH adjoints.g4
 * ============================================================================
 *
 * Measurement is generally non-unitary and therefore is not an ordinary
 * adjointable quantum operation.
 *
 * This grammar does not encode that semantic restriction.
 *
 * If source syntax attempts to place measurement under an adjoint modifier,
 * the appropriate semantic layer must reject it unless the language formally
 * defines a reversible measurement construct.
 *
 * ============================================================================
 * INTEGRATION WITH quantum types
 * ============================================================================
 *
 * This file does not define:
 *
 *     Qubit
 *     Qubit[n]
 *     LogicalQubit
 *     QuantumRegister
 *     MeasurementResult
 *
 * Type syntax remains owned by the canonical type grammars.
 *
 * ============================================================================
 * INTEGRATION WITH HYBRID COMPUTATION
 * ============================================================================
 *
 * A measurement can produce classical information consumed by subsequent
 * classical computation.
 *
 * Example:
 *
 *     measure q -> result;
 *
 *     if result {
 *         ...
 *     }
 *
 * The measurement grammar only creates the measurement boundary.
 *
 * Hybrid semantics are handled downstream.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Measurement resource requirements are semantic facts.
 *
 * They may be derived from:
 *
 *     target cardinality;
 *     measurement kind;
 *     measurement mode;
 *     observable;
 *     result shape;
 *     dynamic-circuit requirements.
 *
 * No resource limit is encoded here.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE
 * ============================================================================
 *
 * Hardware-specific measurement realization is downstream.
 *
 * Examples include:
 *
 *     readout method;
 *     physical channel;
 *     timing;
 *     calibration;
 *     native instruction;
 *     topology;
 *     detector configuration.
 *
 * None belong in this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH INTEROPERABILITY
 * ============================================================================
 *
 * OpenQASM, QIR, vendor formats, simulator formats, and other external
 * representations may map to the canonical measurement semantic model.
 *
 * They are interoperability formats.
 *
 * They are not alternative canonical Zamani measurement grammars.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive syntax tests:
 *
 *     measure q;
 *
 *     measure q0, q1;
 *
 *     measure q0, q1, q2;
 *
 *     measure register[i];
 *
 *     measure register[start .. end];
 *
 *     measure q -> result;
 *
 *     measure q -> result[index];
 *
 *     measure q0, q1 -> result;
 *
 *     measure q with {};
 *
 *     measure q with {
 *         basis = X
 *     };
 *
 *     measure q with {
 *         basis = X,
 *         mode = destructive
 *     };
 *
 *     measure q -> result with {
 *         basis = X,
 *         reset = true
 *     };
 *
 *     measure q with {
 *         observable = my_observable
 *     };
 *
 *     measure q with {
 *         future_option = future_value
 *     };
 *
 * Required boundary tests:
 *
 *     one target;
 *     multiple targets;
 *     symbolic/indexed target;
 *     sliced target;
 *     one option;
 *     many options;
 *     empty option block;
 *     trailing target comma;
 *     trailing option comma;
 *     destination;
 *     destination plus options;
 *     nested expression target;
 *     nested expression option value.
 *
 * Required negative parser tests:
 *
 *     measure;
 *     measure ;
 *     measure ,
 *     measure q ,
 *     measure q ->
 *     measure q with
 *     measure q with {
 *     measure q with }
 *     measure q with { basis }
 *     measure q with { = X }
 *     measure q with { basis = }
 *     measure q -> ;
 *
 * Required semantic tests:
 *
 *     classical target;
 *     unresolved target;
 *     invalid target type;
 *     invalid result destination;
 *     unknown semantic option;
 *     duplicate/conflicting option;
 *     unsupported measurement kind;
 *     unsupported measurement mode;
 *     unsupported basis;
 *     invalid observable;
 *     insufficient capability;
 *     insufficient resources.
 *
 * Required scalability tests:
 *
 *     generated target lists of increasing cardinality;
 *     generated option lists of increasing cardinality;
 *     large source files;
 *     symbolic target collections;
 *     large-but-valid expressions.
 *
 * No scalability test may define a universal maximum.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete as the independent measurement grammar contract when:
 *
 * [x] It has a parser grammar declaration.
 *
 * [x] It consumes the canonical ZamaniLexer vocabulary.
 *
 * [x] It imports reusable Names/Expressions grammar components.
 *
 * [x] It has a single ownership boundary.
 *
 * [x] It owns measurement statement syntax.
 *
 * [x] It owns measurement target-list syntax.
 *
 * [x] It owns measurement destination syntax.
 *
 * [x] It owns measurement option syntax.
 *
 * [x] It does not own general expression syntax.
 *
 * [x] It does not own identifier syntax.
 *
 * [x] It does not own quantum type syntax.
 *
 * [x] It does not own operation syntax.
 *
 * [x] It does not own reset syntax.
 *
 * [x] It does not own observable declarations.
 *
 * [x] It does not own dynamic control-flow syntax.
 *
 * [x] It does not own hardware realization.
 *
 * [x] It does not own resource availability.
 *
 * [x] It does not own QEC.
 *
 * [x] It does not own ZQN.
 *
 * [x] It does not own resilience.
 *
 * [x] It does not own routing.
 *
 * [x] It does not own scheduling.
 *
 * [x] It does not define a second quantum IR.
 *
 * [x] It contains no hardware-size constants.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It supports arbitrary source-level target cardinality.
 *
 * [x] It supports arbitrary source-level option cardinality.
 *
 * [x] It preserves source structure for AST construction.
 *
 * [x] It preserves option ordering.
 *
 * [x] It keeps semantic validation downstream.
 *
 * [x] It preserves the quantum::ir boundary.
 *
 * [x] It has explicit integration contracts.
 *
 * [x] It has explicit positive/negative/boundary/scalability tests.
 *
 * [ ] quantum.g4 imports this grammar in the canonical composition build.
 *
 * [ ] The generated ANTLR parser accepts the positive corpus.
 *
 * [ ] The generated ANTLR parser rejects the structural negative corpus.
 *
 * [ ] Rust lexer tokenization conforms to the canonical lexer.
 *
 * [ ] Frontend AST lowering conforms to the AST contract.
 *
 * [ ] Semantic measurement validation conforms to the semantic contract.
 *
 * [ ] canonical quantum::ir lowering conforms to the IR contract.
 *
 * [ ] Cross-domain/hybrid tests pass.
 *
 * ============================================================================
 */


/* ============================================================================
 * PARSER DECLARATION
 * ========================================================================== */

parser grammar QuantumMeasurement;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * 1. MEASUREMENT STATEMENT
 * ========================================================================== */

/*
 * Canonical forms:
 *
 *     measure q;
 *     measure q -> result;
 *     measure q with { basis = X };
 *     measure q -> result with { basis = X };
 */
quantumMeasurementStatement
    : MEASURE
      quantumMeasurementTargetList
      quantumMeasurementDestination?
      quantumMeasurementOptions?
      SEMICOLON
    ;


/* ============================================================================
 * 2. TARGET LIST
 * ========================================================================== */

/*
 * Target cardinality is intentionally unbounded by grammar.
 *
 * The optional trailing comma is part of the explicit source syntax policy.
 */
quantumMeasurementTargetList
    : quantumMeasurementTarget
      (COMMA quantumMeasurementTarget)*
      COMMA?
    ;


/* ============================================================================
 * 3. TARGET
 * ========================================================================== */

/*
 * A target is an ordinary expression.
 *
 * Semantic analysis determines whether it is measurable.
 */
quantumMeasurementTarget
    : expression
    ;


/* ============================================================================
 * 4. RESULT DESTINATION
 * ========================================================================== */

quantumMeasurementDestination
    : THIN_ARROW expression
    ;


/* ============================================================================
 * 5. OPTIONS
 * ========================================================================== */

quantumMeasurementOptions
    : WITH
      LBRACE
      quantumMeasurementOptionList?
      RBRACE
    ;


/* ============================================================================
 * 6. OPTION LIST
 * ========================================================================== */

quantumMeasurementOptionList
    : quantumMeasurementOption
      (COMMA quantumMeasurementOption)*
      COMMA?
    ;


/* ============================================================================
 * 7. OPTION
 * ========================================================================== */

/*
 * The option namespace is deliberately open.
 *
 * Semantic analysis owns the authoritative option registry.
 */
quantumMeasurementOption
    : identifier
      ASSIGN
      expression
    ;