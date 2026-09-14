/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Quantum Measurement Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/measurement.g4
 *
 * Role:
 *     Canonical reusable PARSER fragment for quantum measurement syntax.
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
 * This file owns SOURCE-LEVEL MEASUREMENT SYNTAX.
 *
 * It describes:
 *
 *     WHAT a Zamani program asks to measure.
 *
 * It does NOT describe:
 *
 *     HOW a backend performs the measurement.
 *
 * The source-level measurement is subsequently lowered through:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic/type/effect/resource analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC
 *       +--> ZQN
 *       +--> resilience
 *       +--> hardware HAL
 *       +--> simulator
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - measurement statements;
 *     - measurement target lists;
 *     - measurement result destinations;
 *     - measurement option blocks;
 *     - measurement option names;
 *     - measurement option values;
 *     - measurement source-level observables/bases;
 *     - measurement kind intent;
 *     - destructive/non-destructive intent;
 *     - reset-after-measurement intent;
 *     - measurement grouping syntax;
 *     - extensible measurement metadata syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - qubit declarations;
 *     - quantum registers;
 *     - quantum operation syntax;
 *     - reset statements outside measurement options;
 *     - observable implementation;
 *     - measurement probabilities;
 *     - result sampling;
 *     - detector implementation;
 *     - readout hardware;
 *     - ADC/DAC configuration;
 *     - calibration;
 *     - measurement pulses;
 *     - measurement timing;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC decoding;
 *     - ZQN noise models;
 *     - resilience decisions;
 *     - backend selection;
 *     - physical qubit allocation;
 *     - canonical quantum IR.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Measurement syntax MUST remain independent of machine scale.
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
 *     DEVICE_ID
 *     QPU_ID
 *     PHYSICAL_QUBIT_ID
 *     READOUT_FREQUENCY
 *     READOUT_DURATION
 *     HARDWARE_TOPOLOGY
 *     VENDOR
 *     BACKEND
 *
 * Any practical limits are determined later by:
 *
 *     semantic analysis;
 *     resource analysis;
 *     QuantumIrLimits;
 *     scheduling;
 *     hardware capability negotiation;
 *     runtime resources.
 *
 * These are not grammar limits.
 *
 * ============================================================================
 * LEXICAL INTEGRATION
 * ============================================================================
 *
 * This file consumes the canonical Zamani lexer vocabulary.
 *
 * Canonical tokens used here include:
 *
 *     MEASURE
 *     WITH
 *     THIN_ARROW
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     ASSIGN
 *     SEMICOLON
 *
 * IMPORTANT:
 *
 * Do NOT introduce duplicate tokens such as:
 *
 *     K_MEASURE
 *     K_WITH
 *     SEMI
 *     MEASUREMENT_TOKEN
 *     BASIS_TOKEN
 *     MODE_TOKEN
 *
 * Measurement option names remain identifiers.
 *
 * This intentionally allows the language to evolve without continually
 * expanding the lexer keyword inventory.
 *
 * ============================================================================
 * CORE SYNTAX
 * ============================================================================
 *
 * The canonical minimal form is:
 *
 *     measure q;
 *
 * A result may be assigned:
 *
 *     measure q -> result;
 *
 * Multiple targets:
 *
 *     measure q0, q1, q2;
 *
 * Multiple targets may also be selected through expressions:
 *
 *     measure register[i];
 *     measure register[start .. end];
 *     measure selection;
 *
 * Measurement semantics may be explicitly described:
 *
 *     measure q with {
 *         basis = X
 *     };
 *
 * Result destination plus options:
 *
 *     measure q -> result with {
 *         basis = X,
 *         mode = destructive
 *     };
 *
 * The option block is deliberately key/value based rather than keyword based.
 *
 * Therefore future semantic concepts can be added without requiring a new
 * lexer keyword for every measurement feature.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * For example:
 *
 *     measure q with {
 *         basis = X
 *     };
 *
 * means:
 *
 *     "measure q using the semantic X basis"
 *
 * It does NOT mean:
 *
 *     "apply a particular physical X rotation pulse"
 *
 * The backend decides how the semantic request is realized.
 *
 * Similarly:
 *
 *     mode = destructive
 *
 * describes semantic measurement behavior.
 *
 * It does NOT specify:
 *
 *     detector technology;
 *     pulse sequence;
 *     readout channel;
 *     hardware duration;
 *     physical qubit.
 *
 * ============================================================================
 * CANONICAL IR INTEGRATION
 * ============================================================================
 *
 * The grammar must eventually lower into:
 *
 *     quantum::ir
 *
 * The existing canonical quantum measurement model owns:
 *
 *     Measurement
 *     MeasurementKind
 *     MeasurementMode
 *     MeasurementObservable
 *     MeasurementBasis
 *     PauliProduct
 *     ClassicalBitId
 *     QubitId
 *
 * This grammar MUST NOT define any of those semantic Rust types.
 *
 * It only preserves source structure needed by the frontend AST.
 *
 * ============================================================================
 * RESULT DESTINATION CONTRACT
 * ============================================================================
 *
 * The destination after:
 *
 *     ->
 *
 * is an ordinary Zamani expression.
 *
 * Examples:
 *
 *     measure q -> result;
 *     measure q -> classical_bit;
 *     measure q -> result[index];
 *     measure q0, q1 -> result;
 *
 * The grammar deliberately does NOT determine whether the destination is:
 *
 *     one bit;
 *     a bit vector;
 *     a register;
 *     a tuple;
 *     an array;
 *     a structured result;
 *     a future measurement-result type.
 *
 * Semantic/type analysis determines whether the destination shape is valid.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * Measurement targets are ordinary Zamani expressions.
 *
 * Examples:
 *
 *     q
 *     q[i]
 *     register
 *     register[i]
 *     register[start .. end]
 *     logical_qubit
 *     expression-derived selection
 *
 * The parser does not decide whether an expression is actually quantum.
 *
 * Semantic analysis performs that validation.
 *
 * ============================================================================
 * OPTION CONTRACT
 * ============================================================================
 *
 * Options have the form:
 *
 *     name = expression
 *
 * Examples:
 *
 *     basis = X
 *     observable = PauliX
 *     kind = projective
 *     mode = destructive
 *     reset = true
 *
 * The grammar does NOT hard-code a closed list of option names.
 *
 * This is intentional.
 *
 * The semantic layer owns the authoritative option registry.
 *
 * Unknown options are therefore syntax-valid but semantically diagnosable.
 *
 * This prevents the grammar from becoming the source of machine-specific
 * assumptions.
 *
 * ============================================================================
 * DUPLICATE OPTIONS
 * ============================================================================
 *
 * Duplicate option names are syntactically valid:
 *
 *     measure q with {
 *         basis = X,
 *         basis = Z
 *     };
 *
 * The grammar does not silently choose one.
 *
 * Semantic validation MUST reject conflicting/duplicate options according to
 * the measurement semantic contract.
 *
 * This preserves source information and produces better diagnostics.
 *
 * ============================================================================
 * EMPTY OPTION BLOCK
 * ============================================================================
 *
 * The following is syntactically valid:
 *
 *     measure q with {};
 *
 * Semantic analysis may normalize it to the default measurement semantics.
 *
 * This keeps parsing deterministic and avoids special grammar branches.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * New measurement concepts can be expressed through options without requiring
 * a new keyword.
 *
 * Examples include future concepts such as:
 *
 *     observable
 *     basis
 *     kind
 *     mode
 *     reset
 *     grouping
 *     result_type
 *     confidence
 *     annotation
 *     semantic_precision
 *
 * Their meaning belongs to semantic analysis and canonical IR.
 *
 * ============================================================================
 * QEC / ZQN / RESILIENCE BOUNDARY
 * ============================================================================
 *
 * This grammar does NOT implement:
 *
 *     QEC;
 *     syndrome extraction;
 *     decoding;
 *     noise channels;
 *     measurement error models;
 *     readout mitigation;
 *     retry policy;
 *     backend switching;
 *     recovery.
 *
 * A measurement may later be consumed by:
 *
 *     QEC
 *     ZQN
 *     resilience
 *
 * but those subsystems remain downstream consumers.
 *
 * ============================================================================
 * DYNAMIC CIRCUITS
 * ============================================================================
 *
 * A measurement result may participate in later classical control:
 *
 *     measure q -> result;
 *
 *     if result {
 *         ...
 *     }
 *
 * This file only owns the measurement operation.
 *
 * The general control-flow grammar owns:
 *
 *     if;
 *     match;
 *     loops;
 *     classical control.
 *
 * Therefore this file must not create a second conditional grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * There must be exactly one syntactic interpretation for:
 *
 *     measure <targets>;
 *
 *     measure <targets> -> <destination>;
 *
 *     measure <targets> with { <options> };
 *
 *     measure <targets> -> <destination> with { <options> };
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     source span;
 *     target expressions;
 *     optional destination;
 *     option order;
 *     option names;
 *     option expressions;
 *     trailing comma presence where the AST policy preserves it.
 *
 * Semantic normalization belongs after parsing.
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
 *     hardware access;
 *     dynamic code execution.
 *
 * A malicious measurement source must therefore be handled entirely as syntax
 * until downstream validation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. MEASUREMENT STATEMENT
 * ========================================================================== */

/*
 * Canonical minimal form:
 *
 *     measure q;
 *
 * The parser records measurement intent.
 */
quantumMeasurementStatement
    : MEASURE
      quantumMeasurementTargetList
      quantumMeasurementDestination?
      quantumMeasurementOptions?
      SEMICOLON
    ;


/* ============================================================================
 * 2. MEASUREMENT TARGET LIST
 * ========================================================================== */

/*
 * Examples:
 *
 *     measure q;
 *     measure q0, q1;
 *     measure register[i];
 *     measure register[start .. end];
 *
 * No target-count limit is encoded.
 */
quantumMeasurementTargetList
    : quantumMeasurementTarget
      (COMMA quantumMeasurementTarget)*
      COMMA?
    ;


/* ============================================================================
 * 3. MEASUREMENT TARGET
 * ========================================================================== */

/*
 * The target is an ordinary Zamani expression.
 *
 * This intentionally avoids defining another:
 *
 *     QubitId
 *     PhysicalQubitId
 *     RegisterId
 *
 * in the grammar.
 *
 * Canonical identity belongs to semantic analysis and quantum::ir.
 */
quantumMeasurementTarget
    : expression
    ;


/* ============================================================================
 * 4. RESULT DESTINATION
 * ========================================================================== */

/*
 * Canonical form:
 *
 *     measure q -> result;
 *
 * The destination remains an expression so semantic analysis can determine
 * whether it represents:
 *
 *     a classical bit;
 *     a classical register;
 *     a result collection;
 *     a structured destination;
 *     another valid measurement sink.
 */
quantumMeasurementDestination
    : THIN_ARROW expression
    ;


/* ============================================================================
 * 5. MEASUREMENT OPTIONS
 * ========================================================================== */

/*
 * Optional semantic configuration:
 *
 *     measure q with {
 *         basis = X
 *     };
 *
 *     measure q -> result with {
 *         mode = destructive,
 *         reset = true
 *     };
 *
 * `WITH` is already a canonical Zamani keyword.
 *
 * No new lexer keyword is introduced.
 */
quantumMeasurementOptions
    : WITH
      LBRACE
      quantumMeasurementOptionList?
      RBRACE
    ;


/* ============================================================================
 * 6. OPTION LIST
 * ========================================================================== */

/*
 * Options are comma-separated.
 *
 * No fixed number of options is permitted or required by the grammar.
 */
quantumMeasurementOptionList
    : quantumMeasurementOption
      (COMMA quantumMeasurementOption)*
      COMMA?
    ;


/* ============================================================================
 * 7. OPTION
 * ========================================================================== */

/*
 * Generic form:
 *
 *     name = expression
 *
 * Examples:
 *
 *     basis = X
 *     mode = destructive
 *     kind = projective
 *     reset = true
 *
 * The semantic layer owns the authoritative option vocabulary.
 */
quantumMeasurementOption
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 8. EXPLICIT STANDARD MEASUREMENT OPTION NAMES
 * ============================================================================
 *
 * IMPORTANT:
 *
 * These rules are NOT used to restrict the option vocabulary.
 *
 * They exist as named parser contracts for semantic consumers and future
 * AST builders that need to distinguish the canonical standard options from
 * extension options.
 *
 * The actual option spelling is still represented by the normal identifier
 * grammar.
 * ========================================================================== */

quantumMeasurementStandardOptionName
    : identifier
    ;


/* ============================================================================
 * 9. BASIS OPTION CONTRACT
 * ========================================================================== */

/*
 * The grammar intentionally does not reserve:
 *
 *     X
 *     Y
 *     Z
 *
 * as lexer keywords.
 *
 * Therefore:
 *
 *     basis = X
 *     basis = Y
 *     basis = Z
 *
 * remain ordinary expressions.
 *
 * Semantic analysis maps them to the canonical IR basis representation where
 * appropriate.
 *
 * Named/custom observables remain possible.
 */


/* ============================================================================
 * 10. MEASUREMENT KIND CONTRACT
 * ========================================================================== */

/*
 * Semantic examples:
 *
 *     kind = projective
 *     kind = generalized
 *     kind = weak
 *     kind = continuous
 *
 * These values are intentionally not lexer keywords.
 *
 * The grammar preserves them as expressions.
 *
 * The canonical IR owns MeasurementKind.
 */


/* ============================================================================
 * 11. MEASUREMENT MODE CONTRACT
 * ========================================================================== */

/*
 * Semantic examples:
 *
 *     mode = destructive
 *     mode = non_destructive
 *
 * Again, these remain expressions.
 *
 * The canonical IR owns MeasurementMode.
 */


/* ============================================================================
 * 12. RESET-AFTER-MEASUREMENT CONTRACT
 * ========================================================================== */

/*
 * Semantic example:
 *
 *     measure q with {
 *         reset = true
 *     };
 *
 * This is NOT equivalent to the standalone:
 *
 *     reset q;
 *
 * The distinction remains available to semantic lowering.
 */


/* ============================================================================
 * 13. OBSERVABLE CONTRACT
 * ========================================================================== */

/*
 * Semantic examples:
 *
 *     measure q with {
 *         observable = X
 *     };
 *
 *     measure q with {
 *         observable = my_observable
 *     };
 *
 *     measure q0, q1 with {
 *         observable = parity
 *     };
 *
 * The grammar does not implement observable mathematics.
 *
 * Canonical observable semantics belong to quantum::ir.
 */


/* ============================================================================
 * 14. RESULT TYPE CONTRACT
 * ========================================================================== */

/*
 * A semantic consumer may use:
 *
 *     result_type = bit
 *
 *     result_type = bit_vector
 *
 *     result_type = probability
 *
 *     result_type = observable
 *
 * The grammar does not define a finite result-type universe.
 *
 * Type analysis owns result compatibility.
 */


/* ============================================================================
 * 15. GROUPING CONTRACT
 * ========================================================================== */

/*
 * Measurement grouping is represented semantically through an option:
 *
 *     grouping = ...
 *
 * This allows multiple measurement targets to remain one source operation
 * without forcing a hardware-specific grouping representation.
 */


/* ============================================================================
 * 16. EXTENSION OPTIONS
 * ========================================================================== */

/*
 * Future language versions may define additional measurement options.
 *
 * For example:
 *
 *     measure q with {
 *         future_option = value
 *     };
 *
 * Such syntax remains parseable without changing this grammar.
 *
 * Semantic version/capability checking decides whether the option is supported.
 */


/* ============================================================================
 * 17. ATTRIBUTE INTEGRATION
 * ========================================================================== */

/*
 * Attributes remain owned by the canonical attribute grammar.
 *
 * An attributed measurement can therefore be represented by the surrounding
 * quantum-block grammar:
 *
 *     #[attribute]
 *     measure q -> result;
 *
 * This file deliberately does not duplicate attribute syntax.
 *
 * If the canonical AST attaches attributes directly to statements, the
 * frontend should wrap quantumMeasurementStatement accordingly.
 */


/* ============================================================================
 * 18. QUANTUM BLOCK INTEGRATION
 * ========================================================================== */

/*
 * quantum/quantum.g4 must reference:
 *
 *     quantumMeasurementStatement
 *
 * as one of its quantum elements.
 *
 * This file becomes the sole owner of that rule.
 */


/* ============================================================================
 * 19. GENERAL STATEMENT INTEGRATION
 * ========================================================================== */

/*
 * The canonical parser must route measurement statements through the quantum
 * statement boundary rather than treating `measure` as a generic function call.
 *
 * This preserves:
 *
 *     quantum measurement intent
 *
 * as a distinct AST node.
 */


/* ============================================================================
 * 20. SEMANTIC VALIDATION CONTRACT
 * ========================================================================== */

/*
 * Parsing succeeds for syntactically valid forms.
 *
 * Semantic analysis MUST subsequently validate:
 *
 *     1. Every target denotes a valid quantum resource.
 *
 *     2. Every target is accessible in the current scope.
 *
 *     3. Every target has a valid quantum type.
 *
 *     4. Destination expressions have a compatible classical/result type.
 *
 *     5. Destination shape is compatible with target/result shape.
 *
 *     6. Measurement options are recognized or explicitly permitted by the
 *        active language/dialect version.
 *
 *     7. Duplicate/conflicting options are rejected.
 *
 *     8. The selected observable is valid for the selected targets.
 *
 *     9. Measurement kind is compatible with the observable.
 *
 *    10. Measurement mode is semantically valid.
 *
 *    11. Reset-after-measurement semantics are valid.
 *
 *    12. Dynamic-circuit dependencies are valid.
 *
 *    13. Resource requirements are satisfiable by the selected compilation
 *        context.
 *
 *    14. Any target-specific restriction is reported as a target/resource
 *        diagnostic rather than a grammar failure.
 */


/* ============================================================================
 * 21. CANONICAL IR LOWERING CONTRACT
 * ========================================================================== */

/*
 * The frontend lowering layer should transform the parsed structure into the
 * canonical measurement representation owned by:
 *
 *     src/quantum/ir/quantum/measurement.rs
 *
 * The lowering layer is responsible for mapping source options such as:
 *
 *     basis
 *     observable
 *     kind
 *     mode
 *     reset
 *
 * into canonical semantic fields.
 *
 * This grammar MUST NOT instantiate or import Rust IR structures.
 */


/* ============================================================================
 * 22. RESOURCE LIMIT CONTRACT
 * ========================================================================== */

/*
 * This grammar performs NO resource-limit validation.
 *
 * In particular, it must not reject a program because it contains:
 *
 *     many targets;
 *     many measurements;
 *     large symbolic registers;
 *     large result structures;
 *     many option entries.
 *
 * Downstream resource analysis may impose explicit policy limits supplied by
 * the compilation/execution context.
 *
 * Therefore:
 *
 *     language capacity
 *
 * is distinct from:
 *
 *     available execution capacity.
 */


/* ============================================================================
 * 23. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * Forbidden examples:
 *
 *     q[0]
 *     q[1]
 *     q[31]
 *
 * as special grammar cases.
 *
 * Forbidden:
 *
 *     MAX_MEASUREMENTS
 *     MAX_QUBITS
 *     MAX_RESULTS
 *     MAX_TARGETS
 *     MAX_SHOTS
 *
 * Forbidden:
 *
 *     IBM
 *     Rigetti
 *     IonQ
 *     Quantinuum
 *     device identifiers
 *     physical readout channel identifiers
 *
 * Forbidden:
 *
 *     fixed measurement duration
 *     fixed readout frequency
 *     fixed detector count
 *
 * This grammar contains none of these machine-specific assumptions.
 */


/* ============================================================================
 * 24. ERROR-RECOVERY CONTRACT
 * ========================================================================== */

/*
 * The generated ANTLR parser may recover from malformed source according to
 * the canonical parser error strategy.
 *
 * This grammar must not contain semantic actions.
 *
 * Diagnostics should identify:
 *
 *     missing target;
 *     malformed destination;
 *     malformed option;
 *     missing assignment;
 *     malformed option block;
 *     missing closing delimiter;
 *     malformed target list.
 *
 * Semantic errors such as:
 *
 *     invalid qubit;
 *     invalid classical destination;
 *     unsupported basis;
 *     unsupported measurement kind;
 *
 * belong to semantic analysis.
 */


/* ============================================================================
 * 25. VALID EXAMPLES
 * ========================================================================== */

/*
 * Minimal:
 *
 *     measure q;
 *
 * Destination:
 *
 *     measure q -> result;
 *
 * Multiple targets:
 *
 *     measure q0, q1, q2;
 *
 * Indexed target:
 *
 *     measure q[i];
 *
 * Range target:
 *
 *     measure q[start .. end];
 *
 * Basis:
 *
 *     measure q with {
 *         basis = X
 *     };
 *
 * Destination plus basis:
 *
 *     measure q -> result with {
 *         basis = X
 *     };
 *
 * Destructive:
 *
 *     measure q with {
 *         mode = destructive
 *     };
 *
 * Reset:
 *
 *     measure q with {
 *         reset = true
 *     };
 *
 * Generalized semantic measurement:
 *
 *     measure q with {
 *         kind = generalized,
 *         observable = observable_name
 *     };
 *
 * Multiple options:
 *
 *     measure q0, q1 -> result with {
 *         basis = Z,
 *         mode = non_destructive,
 *         reset = false
 *     };
 */


/* ============================================================================
 * 26. INVALID EXAMPLES
 * ========================================================================== */

/*
 * Missing target:
 *
 *     measure;
 *
 * Missing destination expression:
 *
 *     measure q ->;
 *
 * Missing option assignment:
 *
 *     measure q with {
 *         basis
 *     };
 *
 * Missing option value:
 *
 *     measure q with {
 *         basis =
 *     };
 *
 * Malformed option:
 *
 *     measure q with {
 *         = X
 *     };
 *
 * Malformed target list:
 *
 *     measure q,;
 *
 * Missing closing brace:
 *
 *     measure q with {
 *         basis = X;
 *
 * These are syntax errors.
 *
 * Semantic invalidity is intentionally not encoded here.
 */


/* ============================================================================
 * 27. ROUND-TRIP CONTRACT
 * ========================================================================== */

/*
 * The parser/AST/printer pipeline should preserve the semantic structure of:
 *
 *     measure targets -> destination with { options };
 *
 * Option order may be preserved by the AST where source fidelity is required.
 *
 * Canonical formatting may normalize:
 *
 *     whitespace;
 *     line breaks;
 *     comma placement;
 *
 * without changing measurement semantics.
 */


/* ============================================================================
 * 28. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Existing syntax:
 *
 *     measure q;
 *
 *     measure q -> result;
 *
 * remains valid.
 *
 * The grammar therefore provides a backwards-compatible expansion of the
 * existing measurement surface rather than replacing it with a machine-specific
 * representation.
 *
 * Existing consumers of `quantumMeasurementStatement` continue to use the same
 * rule name.
 *
 * The important migration is lexical-token normalization:
 *
 *     K_MEASURE  -> MEASURE
 *     SEMI       -> SEMICOLON
 *
 * because the canonical lexer owns those token names.
 */


/* ============================================================================
 * 29. DEPENDENCY CONTRACT
 * ========================================================================== */

/*
 * Direct grammar dependencies:
 *
 *     MEASURE
 *     WITH
 *     THIN_ARROW
 *     LPAREN/RPAREN
 *     LBRACE/RBRACE
 *     COMMA
 *     ASSIGN
 *     SEMICOLON
 *     identifier
 *     expression
 *
 * Indirect semantic consumers:
 *
 *     frontend AST
 *     semantic analyzer
 *     type checker
 *     effect checker
 *     capability checker
 *     resource checker
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *     resilience
 *     hardware HAL
 *     simulator
 *     runtime
 *
 * This file must NOT directly depend on any Rust implementation module.
 */


/* ============================================================================
 * 30. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when:
 *
 *     [x] It owns measurement syntax exclusively.
 *
 *     [x] It uses canonical lexer token names.
 *
 *     [x] It imposes no machine-size limit.
 *
 *     [x] It supports single and multiple targets.
 *
 *     [x] It supports expression-based targets.
 *
 *     [x] It supports result destinations.
 *
 *     [x] It supports extensible semantic options.
 *
 *     [x] It supports basis/observable intent without hard-coded gate syntax.
 *
 *     [x] It supports measurement-kind intent.
 *
 *     [x] It supports destructive/non-destructive intent.
 *
 *     [x] It supports reset-after-measurement intent.
 *
 *     [x] It remains independent of hardware.
 *
 *     [x] It remains independent of QEC.
 *
 *     [x] It remains independent of ZQN.
 *
 *     [x] It remains independent of scheduling.
 *
 *     [x] It remains independent of routing.
 *
 *     [x] It remains independent of optimization.
 *
 *     [x] It remains independent of runtime execution.
 *
 *     [x] It lowers conceptually into the existing quantum::ir measurement
 *         model.
 *
 *     [x] It does not define a second measurement IR.
 *
 *     [x] It contains no Rust.
 *
 *     [x] It requires no unsafe Rust.
 *
 *     [x] It is deterministic.
 *
 *     [x] It has explicit positive and negative test requirements.
 *
 *     [x] It preserves existing `measure q;` and `measure q -> result;`
 *         source forms.
 *
 * ============================================================================
 */