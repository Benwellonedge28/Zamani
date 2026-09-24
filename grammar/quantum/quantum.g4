/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum.g4
 *
 * Grammar:
 *     Quantum
 *
 * Status:
 *     CANONICAL QUANTUM-DOMAIN PARSER COMPOSITION GRAMMAR
 *
 * Purpose:
 *     Compose the quantum-domain grammar into the canonical Zamani parser
 *     without duplicating leaf grammar ownership.
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
 * AUTHORITY
 * ============================================================================
 *
 * Normative language specification:
 *
 *     grammar/specification/
 *
 * Quantum specification:
 *
 *     grammar/spec/quantum.md
 *
 * Architecture:
 *
 *     grammar/DESIGN.md
 *
 * Canonical root:
 *
 *     grammar/Zamani.g4
 *
 * Canonical parser composition root:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Frontend:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * ROLE
 * ============================================================================
 *
 * This file is the QUANTUM DOMAIN COMPOSITION ROOT.
 *
 * It owns:
 *
 *     - the Quantum parser grammar;
 *     - composition of quantum parser grammars;
 *     - the quantum declaration entry point;
 *     - the quantum element dispatch boundary;
 *     - the quantum statement boundary;
 *     - the quantum expression/type integration boundary;
 *     - the quantum/classical integration boundary;
 *     - the quantum-domain extension boundary.
 *
 * It does NOT own the detailed syntax of:
 *
 *     - qubits;
 *     - registers;
 *     - logical qubits;
 *     - physical qubits;
 *     - states;
 *     - operations;
 *     - controlled operations;
 *     - parameterized operations;
 *     - circuits;
 *     - measurements;
 *     - reset;
 *     - observables;
 *     - channels;
 *     - QEC;
 *     - noise;
 *     - resources;
 *     - capabilities;
 *     - dialects;
 *     - dynamic circuits;
 *     - classical feed-forward;
 *     - pulse intent;
 *     - kernels.
 *
 * Those constructs belong to their existing dedicated grammar files.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Every parser production MUST have exactly one canonical owner.
 *
 * This file therefore MUST NOT reimplement a production merely because the
 * production is needed by the quantum domain.
 *
 * Integration wrappers are permitted.
 *
 * An integration wrapper:
 *
 *     - has no independent semantic meaning;
 *     - delegates to an owning grammar production;
 *     - exists only to compose domains;
 *     - must not transform or reinterpret the delegated syntax.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         ZamaniParser
 *                              |
 *                              v
 *                            Quantum
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        quantum syntax   quantum types   quantum expressions
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                       frontend AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *       types               effects            resources
 *                              |
 *                              v
 *                   canonical semantic model
 *                              |
 *                              v
 *                         quantum::ir
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      optimize             routing            scheduling
 *                              |
 *                              v
 *                    QEC / resilience / ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *                              |
 *                              v
 *                            runtime
 *
 * The grammar MUST NOT depend on any stage below semantic parsing.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani quantum syntax represents portable computational intent.
 *
 * The grammar MUST NOT encode an implementation ceiling for:
 *
 *     qubits
 *     registers
 *     controls
 *     targets
 *     operations
 *     parameters
 *     circuit depth
 *     circuit width
 *     measurements
 *     devices
 *     QPUs
 *     nodes
 *     memories
 *     timelines
 *     channels
 *     states
 *     tensor rank
 *     tensor dimensions
 *
 * In particular, this grammar MUST NOT contain:
 *
 *     MAX_QUBITS
 *     MAX_REGISTERS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_PARAMETERS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * or equivalent universal limits.
 *
 * ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * represent grammatically unbounded source structure.
 *
 * Actual finite limits are determined by:
 *
 *     - semantic validity;
 *     - target capabilities;
 *     - resource availability;
 *     - compiler resources;
 *     - runtime resources;
 *     - deployment policy.
 *
 * ============================================================================
 * OPEN-WORLD QUANTUM OPERATIONS
 * ============================================================================
 *
 * The language MUST NOT maintain a closed grammar-level gate list.
 *
 * INVALID ARCHITECTURE:
 *
 *     quantumGate
 *         : H
 *         | X
 *         | Y
 *         | Z
 *         | CNOT
 *         | SWAP
 *         | ...
 *         ;
 *
 * That design makes the language depend on a finite list of operations.
 *
 * The canonical operation grammar instead accepts an open operation
 * designator.
 *
 * Examples:
 *
 *     apply H(q);
 *
 *     apply X(q);
 *
 *     apply CNOT(q0, q1);
 *
 *     apply RX(theta)(q);
 *
 *     apply custom_gate(q);
 *
 *     apply vendor::operation(q);
 *
 *     apply future::operation(parameter)(q0, q1);
 *
 * The semantic layer resolves the meaning.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Source-level quantum syntax may express:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     hints
 *     budgets
 *
 * These are NOT hardware allocation commands.
 *
 * For example:
 *
 *     requires quantum::mid_circuit_measurement;
 *
 * describes a semantic requirement.
 *
 * It does NOT mean:
 *
 *     use physical qubit 0;
 *
 * or:
 *
 *     use QPU 1;
 *
 * or:
 *
 *     select vendor X.
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * QUANTUM IR INVARIANT
 * ============================================================================
 *
 * There is exactly one canonical quantum IR boundary:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumIR
 *     QuantumGateIR
 *     QuantumCircuitIR
 *     QuantumOperationIR
 *     QuantumHardwareIR
 *
 * as competing intermediate representations.
 *
 * The source-to-target path is:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> decomposition
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC
 *       +--> resilience
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target lowering
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - source text;
 *     - selected grammar version;
 *     - canonical lexer;
 *     - canonical parser;
 *     - explicitly selected language dialect/version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware;
 *     - QPU availability;
 *     - runtime state;
 *     - network state;
 *     - filesystem state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - calibration data;
 *     - backend selection.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * It performs no:
 *
 *     - filesystem access;
 *     - network access;
 *     - hardware discovery;
 *     - runtime execution;
 *     - random selection;
 *     - environment inspection.
 *
 * Generated Rust must remain:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust
 *     no unsafe
 *
 * The grammar itself requires no unsafe implementation.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical parser composition root imports this grammar by the grammar
 * name:
 *
 *     Quantum
 *
 * Therefore the grammar declaration below is mandatory.
 *
 * The leaf grammar files that currently exist under grammar/quantum/ are
 * progressively promoted to parser grammars with stable grammar names.
 *
 * This file must then import only those parser grammars.
 *
 * The build system MUST make grammar/quantum available in the ANTLR grammar
 * source path.
 *
 * ============================================================================
 * CURRENT LEAF OWNERSHIP
 * ============================================================================
 *
 * Existing repository ownership is retained.
 *
 * Representative ownership:
 *
 *     operations.g4
 *         -> QuantumOperations
 *
 *     measurement.g4
 *         -> QuantumMeasurement
 *
 *     reset.g4
 *         -> QuantumReset
 *
 *     quantum-types.g4
 *         -> QuantumTypes
 *
 *     quantum-states.g4
 *         -> QuantumStates
 *
 *     quantum-capabilities.g4
 *         -> QuantumCapabilities
 *
 *     quantum-dialects.g4
 *         -> QuantumDialects
 *
 *     error-correction.g4
 *         -> QuantumErrorCorrection
 *
 *     dynamic-control.g4
 *         -> QuantumDynamicControl
 *
 *     quantum-classical.g4
 *         -> QuantumClassical
 *
 *     observables.g4
 *         -> QuantumObservables
 *
 * Files that are currently fragment-style rather than valid independently
 * importable parser grammars MUST retain their existing filenames and
 * ownership. They must be promoted to parser grammars before being imported
 * here.
 *
 * Do NOT create duplicate files merely to work around this.
 *
 * ============================================================================
 * IMPORTANT: NO GATE GRAMMAR
 * ============================================================================
 *
 * grammar/quantum/gates.g4 may contain compatibility or semantic gate-related
 * material, but this composition root MUST NOT turn it into a closed
 * enumeration of primitive gate names.
 *
 * Gate names are ordinary operation names.
 *
 * The semantic system determines whether:
 *
 *     H
 *     X
 *     CNOT
 *     RX
 *     custom_gate
 *     vendor::operation
 *
 * is:
 *
 *     a primitive;
 *     a library operation;
 *     a user-defined operation;
 *     a dialect operation;
 *     an intrinsic;
 *     an imported operation;
 *     a future operation;
 *     or an unresolved name.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The canonical parser must be able to enter quantum syntax through:
 *
 *     quantumDeclaration
 *     quantumStatement
 *     quantumType
 *     quantumExpression
 *
 * depending on the enclosing universal grammar.
 *
 * The complete program entry remains:
 *
 *     ZamaniParser.program
 *
 * This grammar MUST NOT introduce a competing complete-program entry point.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar Quantum;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * QUANTUM LEAF GRAMMAR COMPOSITION
 * ============================================================================
 *
 * Only independently declared parser grammars are imported here.
 *
 * The remaining existing quantum fragments are integrated once their grammar
 * headers are promoted without changing their filenames or ownership.
 * ============================================================================
 */

import
    QuantumOperations,
    QuantumMeasurement,
    QuantumReset,
    QuantumTypes,
    QuantumStates,
    QuantumCapabilities,
    QuantumDialects,
    QuantumErrorCorrection,
    QuantumDynamicControl,
    QuantumClassical,
    QuantumObservables
;


/*
 * ============================================================================
 * 1. QUANTUM DECLARATION
 * ============================================================================
 *
 * This is the public quantum-domain declaration boundary.
 *
 * The keyword `quantum` belongs to the canonical lexical vocabulary.
 *
 * The body delegates to quantum-domain constructs.
 *
 * No hardware/resource count is encoded here.
 * ============================================================================
 */

quantumDeclaration
    : K_QUANTUM quantumDeclarationBody
    ;


/*
 * ============================================================================
 * 2. QUANTUM DECLARATION BODY
 * ============================================================================
 *
 * A quantum declaration may introduce a circuit or a quantum computation
 * block.
 *
 * Circuit declaration syntax is owned by the circuit grammar once that
 * existing fragment is promoted to an importable parser grammar.
 *
 * The block form remains the universal scope form.
 * ============================================================================
 */

quantumDeclarationBody
    : quantumBlock
    | quantumCircuitEntry
    ;


/*
 * ============================================================================
 * 3. QUANTUM BLOCK
 * ============================================================================
 *
 * Quantum blocks are ordinary lexical scopes.
 *
 * Ownership, linearity, lifetime, resource accounting and semantic validity
 * are downstream concerns.
 * ============================================================================
 */

quantumBlock
    : LBRACE quantumBlockElement* RBRACE
    ;


/*
 * ============================================================================
 * 4. QUANTUM BLOCK ELEMENT
 * ============================================================================
 *
 * Attributes are attached at the integration boundary.
 *
 * Detailed construct syntax belongs to the owning grammar.
 * ============================================================================
 */

quantumBlockElement
    : quantumAttributeAttachment?
      quantumElement
    ;


/*
 * ============================================================================
 * 5. QUANTUM ELEMENT
 * ============================================================================
 *
 * This is the central quantum-domain dispatch boundary.
 *
 * The alternatives are semantic categories, not hardware categories.
 * ============================================================================
 */

quantumElement
    : quantumOperationElement
    | quantumMeasurementElement
    | quantumResetElement
    | quantumStateElement
    | quantumTypeElement
    | quantumCapabilityElement
    | quantumErrorCorrectionElement
    | quantumDialectElement
    | quantumClassicalElement
    | quantumDynamicElement
    | quantumObservableElement
    | quantumExtensionElement
    ;


/*
 * ============================================================================
 * 6. OPERATION ELEMENT
 * ============================================================================
 *
 * QuantumOperations owns the detailed operation syntax.
 * ============================================================================
 */

quantumOperationElement
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * 7. MEASUREMENT ELEMENT
 * ============================================================================
 */

quantumMeasurementElement
    : quantumMeasurementStatement
    ;


/*
 * ============================================================================
 * 8. RESET ELEMENT
 * ============================================================================
 */

quantumResetElement
    : quantumResetStatement
    ;


/*
 * ============================================================================
 * 9. STATE ELEMENT
 * ============================================================================
 *
 * State syntax is owned by QuantumStates.
 *
 * This grammar does not decide how states are represented physically.
 * ============================================================================
 */

quantumStateElement
    : quantumStateExpression
    ;


/*
 * ============================================================================
 * 10. TYPE ELEMENT
 * ============================================================================
 *
 * QuantumTypes owns the actual quantum type syntax.
 *
 * A quantum type can express source-level semantic properties such as:
 *
 *     qubit
 *     qubit[n]
 *     logical qubit
 *     quantum resource
 *     domain-specific quantum type
 *
 * Physical realization remains outside the grammar.
 * ============================================================================
 */

quantumTypeElement
    : quantumType
    ;


/*
 * ============================================================================
 * 11. CAPABILITY ELEMENT
 * ============================================================================
 */

quantumCapabilityElement
    : quantumCapabilityDeclaration
    | quantumCapabilityRequirement
    ;


/*
 * ============================================================================
 * 12. ERROR-CORRECTION ELEMENT
 * ============================================================================
 *
 * QEC syntax expresses intent only.
 *
 * It does not implement:
 *
 *     encoding;
 *     syndrome extraction;
 *     decoding;
 *     recovery;
 *     code selection.
 *
 * Those belong downstream.
 * ============================================================================
 */

quantumErrorCorrectionElement
    : quantumErrorCorrectionDeclaration
    ;


/*
 * ============================================================================
 * 13. DIALECT ELEMENT
 * ============================================================================
 */

quantumDialectElement
    : quantumDialectDeclaration
    ;


/*
 * ============================================================================
 * 14. QUANTUM / CLASSICAL ELEMENT
 * ============================================================================
 *
 * This is the boundary between the quantum domain and the universal
 * classical computation domain.
 *
 * Measurement results may participate in ordinary classical expressions and
 * control flow according to semantic rules.
 * ============================================================================
 */

quantumClassicalElement
    : quantumClassicalBoundary
    ;


/*
 * ============================================================================
 * 15. DYNAMIC QUANTUM ELEMENT
 * ============================================================================
 *
 * Dynamic-control grammar owns measurement-dependent and runtime-dependent
 * quantum control syntax.
 * ============================================================================
 */

quantumDynamicElement
    : quantumDynamicControl
    ;


/*
 * ============================================================================
 * 16. OBSERVABLE ELEMENT
 * ============================================================================
 */

quantumObservableElement
    : quantumObservationStatement
    ;


/*
 * ============================================================================
 * 17. CIRCUIT INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Circuit syntax remains owned by circuits.g4.
 *
 * The existing circuits.g4 currently contains fragment-style material and
 * therefore MUST be promoted to an importable parser grammar before this
 * delegation is enabled in the generated parser.
 *
 * `quantumCircuitEntry` is deliberately isolated so that the ownership
 * contract remains stable while that promotion is performed.
 * ============================================================================
 */

quantumCircuitEntry
    : circuitDeclaration
    ;


/*
 * ============================================================================
 * 18. QUANTUM ATTRIBUTE ATTACHMENT
 * ============================================================================
 *
 * Attributes remain owned by the canonical core attribute grammar.
 *
 * This rule is only an integration wrapper.
 * ============================================================================
 */

quantumAttributeAttachment
    : attributes
    ;


/*
 * ============================================================================
 * 19. QUANTUM STATEMENT ENTRY
 * ============================================================================
 *
 * This entry point is used when universal statement composition has already
 * established a quantum context.
 * ============================================================================
 */

quantumStatement
    : quantumElement
    ;


/*
 * ============================================================================
 * 20. QUANTUM TYPE ENTRY
 * ============================================================================
 *
 * Public integration point for universal type composition.
 * ============================================================================
 */

quantumType
    : quantumTypeExpression
    ;


/*
 * ============================================================================
 * 21. QUANTUM EXPRESSION ENTRY
 * ============================================================================
 *
 * Quantum expressions must ultimately use the universal expression system.
 *
 * This grammar MUST NOT introduce a second expression language.
 * ============================================================================
 */

quantumExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. QUANTUM OPERATION EXTENSION
 * ============================================================================
 *
 * The operation grammar remains open-world.
 *
 * There is deliberately no rule such as:
 *
 *     quantumGate : H | X | Y | Z | ...
 *
 * Future operations therefore do not require this composition grammar to be
 * modified.
 * ============================================================================
 */

quantumExtensionElement
    : quantumExtensionDeclaration
    ;


/*
 * ============================================================================
 * 23. QUANTUM EXTENSION DECLARATION
 * ============================================================================
 *
 * Extension syntax is intentionally delegated to the dialect mechanism.
 *
 * The rule exists as an integration boundary rather than as a new extension
 * language.
 * ============================================================================
 */

quantumExtensionDeclaration
    : quantumDialectDeclaration
    ;


/*
 * ============================================================================
 * 24. QUANTUM RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements are intentionally NOT duplicated here.
 *
 * The universal resource grammar remains authoritative.
 *
 * Once the quantum resource fragment is promoted to an independently
 * importable parser grammar, this composition root should delegate to its
 * canonical production rather than creating a duplicate resource grammar.
 *
 * This boundary is intentionally left out of `quantumElement` until the
 * existing resource fragment has a valid parser-grammar identity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. QUANTUM REGISTER / QUBIT INTEGRATION
 * ============================================================================
 *
 * Qubit and register syntax remains owned by:
 *
 *     grammar/quantum/qubits.g4
 *     grammar/quantum/quantum-registers.g4
 *     grammar/quantum/logical-qubits.g4
 *     grammar/quantum/physical-qubits.g4
 *
 * These files currently contain fragment-style grammar material.
 *
 * They MUST be promoted to parser grammars before being imported here.
 *
 * Their filenames and ownership MUST NOT be changed merely to satisfy ANTLR.
 *
 * Once promoted, this dispatcher will delegate:
 *
 *     quantumQubitDeclaration
 *     quantumRegisterDeclaration
 *     logicalQubitDeclaration
 *     physicalQubitDeclaration
 *
 * without copying their implementations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. NO PHYSICAL TARGET SYNTAX
 * ============================================================================
 *
 * This grammar intentionally does not contain productions for:
 *
 *     physical qubit number
 *     QPU identifier
 *     coupling-map index
 *     vendor backend
 *     hardware address
 *     device topology
 *     calibration slot
 *     pulse duration
 *
 * A source program may express target-independent requirements, but physical
 * realization is downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. NO RESOURCE LIMITS
 * ============================================================================
 *
 * There are deliberately no finite grammar repetitions such as:
 *
 *     (x x x x)?
 *
 * to represent a resource limit.
 *
 * All source cardinality is represented through:
 *
 *     *
 *     +
 *
 * or through expressions evaluated semantically.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. NO HARD-CODED OPERATION INVENTORY
 * ============================================================================
 *
 * The following are examples of valid semantic operation names, not grammar
 * alternatives:
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
 *     custom_gate
 *     vendor::operation
 *     library::operation
 *     future::operation
 *
 * The parser accepts names through QuantumOperations.
 *
 * Semantic resolution determines their meaning.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. QUANTUM / CLASSICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * A quantum program is not a second programming language.
 *
 * It shares the universal Zamani:
 *
 *     types
 *     expressions
 *     statements
 *     functions
 *     modules
 *     effects
 *     memory
 *     concurrency
 *     resources
 *     capabilities
 *     diagnostics
 *
 * Quantum-specific syntax is therefore an extension of the universal
 * language, not an isolated parser universe.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. FRONTEND AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * It MUST map through the existing domain-neutral AST.
 *
 * Required conceptual mappings include:
 *
 *     quantumOperationStatement
 *         -> generic/domain-neutral operation representation
 *
 *     quantumStateExpression
 *         -> domain-neutral expression/state representation
 *
 *     quantumType
 *         -> existing TypeExpr/type representation
 *
 *     quantumMeasurementStatement
 *         -> generic/domain-neutral measurement operation
 *
 *     quantumCapabilityRequirement
 *         -> requirement/capability representation
 *
 *     quantumErrorCorrectionDeclaration
 *         -> quantum semantic declaration/intent
 *
 * The exact AST node names are owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT create a second quantum AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every construct entering the AST must retain sufficient source information
 * for diagnostics.
 *
 * The grammar MUST therefore avoid semantic transformations that destroy:
 *
 *     start token
 *     stop token
 *     source ordering
 *     nesting information
 *     operation name
 *     parameters
 *     targets
 *     modifiers
 *     attributes
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis must subsequently determine:
 *
 *     - whether an operation exists;
 *     - whether its parameters are valid;
 *     - whether target types are valid;
 *     - whether controls are valid;
 *     - whether targets overlap illegally;
 *     - whether a state is valid;
 *     - whether a measurement is legal;
 *     - whether dynamic control is supported;
 *     - whether capabilities are satisfied;
 *     - whether resources are sufficient;
 *     - whether effects are legal;
 *     - whether the quantum computation is semantically valid.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource analysis occurs after parsing.
 *
 * Examples of valid source intent:
 *
 *     requires qubits >= n
 *
 *     requires memory >= required_memory
 *
 *     requires capability("quantum.measurement")
 *
 *     requires capability("quantum.mid_circuit_measurement")
 *
 *     requires topology(...)
 *
 * These requirements are not parser-level hardware allocations.
 *
 * The grammar must never convert today's available hardware capacity into a
 * language maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. ROUTING CONTRACT
 * ============================================================================
 *
 * Routing is downstream.
 *
 * The grammar MUST NOT determine:
 *
 *     logical -> physical qubit mapping
 *     coupling-map traversal
 *     swap insertion
 *     topology embedding
 *
 * The semantic representation remains portable until routing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling is downstream.
 *
 * The grammar MUST NOT determine:
 *
 *     gate duration
 *     pulse duration
 *     clock cycle
 *     instruction issue slot
 *     physical resource reservation
 *
 * Timing intent, where semantically required, is expressed through the
 * appropriate timing/resource contract and resolved later.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. QEC CONTRACT
 * ============================================================================
 *
 * Error-correction syntax describes intent.
 *
 * It does not implement:
 *
 *     encoders
 *     decoders
 *     syndrome extraction
 *     recovery
 *     code-distance algorithms
 *     physical layouts
 *
 * Those remain downstream responsibilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. ZQN CONTRACT
 * ============================================================================
 *
 * ZQN remains responsible for fault/noise semantics downstream.
 *
 * This grammar does not:
 *
 *     simulate noise;
 *     select noise models;
 *     inject faults;
 *     determine physical error rates.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. RESILIENCE CONTRACT
 * ============================================================================
 *
 * Quantum execution may eventually expose resilience policies downstream.
 *
 * The established vocabulary remains:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * and outcomes:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * This grammar does not implement state transitions or recovery.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. OPEN QUANTUM COMPUTATION MODEL
 * ============================================================================
 *
 * The composition boundary remains capable of representing future quantum
 * computational models without changing the core grammar merely because the
 * underlying physical technology changes.
 *
 * Examples include:
 *
 *     superconducting
 *     trapped-ion
 *     neutral-atom
 *     photonic
 *     spin
 *     topological
 *     measurement-based
 *     analog
 *     quantum annealing
 *     tensor-network simulation
 *     distributed quantum computing
 *     quantum networking
 *     quantum sensing
 *     future computational substrates
 *
 * These are semantic/dialect/backend concerns unless they introduce genuinely
 * new source-level syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. OPEN-WORLD OPERATION SEMANTICS
 * ============================================================================
 *
 * An operation is conceptually:
 *
 *     name
 *     namespace
 *     parameters
 *     operands
 *     results
 *     modifiers
 *     attributes
 *     effects
 *     capabilities
 *     source
 *
 * This semantic shape belongs downstream.
 *
 * QuantumOperations owns only the source syntax needed to preserve these
 * components.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. PORTABILITY CONTRACT
 * ============================================================================
 *
 * The same source-level quantum program must be capable of being lowered to
 * any target that satisfies its semantic requirements.
 *
 * Potential targets include:
 *
 *     embedded quantum controllers
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     quantum simulators
 *     accelerators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future targets
 *
 * This grammar does not guarantee that every target can execute every
 * program. It guarantees that the language does not unnecessarily encode a
 * particular target into the source syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing filenames MUST remain stable.
 *
 * Existing public syntax should remain accepted where it does not violate the
 * production architecture.
 *
 * Compatibility forms must be handled by the owning grammar and marked in
 * the compatibility specification.
 *
 * This composition grammar must not silently introduce incompatible aliases.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. DIALECT CONTRACT
 * ============================================================================
 *
 * Quantum dialects are explicitly versioned/declared extensions.
 *
 * A dialect may add:
 *
 *     syntax
 *     semantic declarations
 *     operation namespaces
 *     interoperability forms
 *
 * A dialect must not:
 *
 *     redefine core syntax ambiguously;
 *     introduce hardware limits;
 *     create another quantum IR;
 *     bypass semantic validation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * OpenQASM, QIR and other quantum formats are interoperability representations.
 *
 * They are NOT the canonical Zamani semantic model.
 *
 * The intended direction is:
 *
 *     Zamani
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> OpenQASM import/export
 *       +--> QIR lowering/import/export
 *       +--> backend-specific lowering
 *
 * No interoperability format becomes a second Zamani grammar authority.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. FEATURE COMPLETION CONTRACT
 * ============================================================================
 *
 * A quantum feature is complete only when:
 *
 *     specification
 *         |
 *         v
 *     lexical contract
 *         |
 *         v
 *     grammar
 *         |
 *         v
 *     parser implementation
 *         |
 *         v
 *     AST mapping
 *         |
 *         v
 *     source spans
 *         |
 *         v
 *     semantic validation
 *         |
 *         v
 *     canonical IR mapping
 *         |
 *         v
 *     compiler consumer
 *         |
 *         v
 *     runtime consumer
 *         |
 *         v
 *     diagnostics
 *         |
 *         v
 *     positive tests
 *         |
 *         v
 *     negative tests
 *         |
 *         v
 *     boundary tests
 *         |
 *         v
 *     scalability tests
 *         |
 *         v
 *     determinism tests
 *         |
 *         v
 *     compatibility tests
 *         |
 *         v
 *     hard-coding audit
 *
 * Only then may the feature be marked STABLE.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the architectural hard-coding audit because it contains
 * no universal hardware/resource constants.
 *
 * Specifically, it does not define:
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
 * It also does not enumerate:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     fixed QPU IDs
 *     fixed topology
 *     fixed coupling maps
 *     fixed vendor devices
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. SCALABILITY AUDIT
 * ============================================================================
 *
 * Source cardinality uses:
 *
 *     *
 *     +
 *
 * and semantic expressions.
 *
 * There is no grammar-level ceiling on:
 *
 *     quantum declarations
 *     operations
 *     parameters
 *     targets
 *     measurements
 *     controls
 *     circuit elements
 *     nesting
 *
 * Practical execution remains limited only by the resources available to the
 * parser, compiler, runtime and target.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. DIAGNOSTIC AUDIT
 * ============================================================================
 *
 * The parser should report structural errors at the narrowest owning grammar
 * boundary.
 *
 * Semantic errors must not be manufactured as parser errors.
 *
 * Examples:
 *
 *     unknown operation
 *         -> semantic error
 *
 *     unsupported operation on selected target
 *         -> capability/target error
 *
 *     insufficient qubits
 *         -> resource error
 *
 *     malformed operation syntax
 *         -> parser error
 *
 *     malformed state literal
 *         -> lexer/parser error
 *
 *     invalid quantum type
 *         -> semantic/type error
 *
 * This separation preserves deterministic and useful diagnostics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. SECURITY AUDIT
 * ============================================================================
 *
 * This grammar:
 *
 *     - executes no code;
 *     - accesses no files;
 *     - accesses no network;
 *     - accesses no hardware;
 *     - reads no secrets;
 *     - performs no environment inspection;
 *     - performs no dynamic loading.
 *
 * It therefore introduces no unsafe Rust requirement.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. FINAL OWNERSHIP STATEMENT
 * ============================================================================
 *
 * This file is the QUANTUM COMPOSITION ROOT.
 *
 * It does not become a second implementation of:
 *
 *     qubits.g4
 *     operations.g4
 *     measurement.g4
 *     reset.g4
 *     circuits.g4
 *     states.g4
 *     quantum-types.g4
 *     QEC
 *     routing
 *     scheduling
 *     ZQN
 *     HAL
 *
 * It composes them.
 *
 * The canonical semantic boundary remains:
 *
 *     quantum::ir
 *
 * The canonical complete-program boundary remains:
 *
 *     ZamaniParser.program
 *
 * The canonical source-to-target architecture remains:
 *
 *     Program
 *       -> Lexer
 *       -> Parser
 *       -> AST
 *       -> Semantic Analysis
 *       -> Canonical IR
 *       -> quantum::ir
 *       -> Optimization
 *       -> Routing
 *       -> Scheduling
 *       -> QEC / Resilience / ZQN
 *       -> HAL
 *       -> Target
 *
 * This is the grammar-level foundation required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * without converting current hardware limitations into permanent language
 * limitations.
 *
 * ============================================================================
 */