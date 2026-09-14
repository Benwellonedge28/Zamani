/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum.g4
 *
 * Status:
 *     Production quantum grammar orchestration fragment.
 *
 * Role:
 *     Canonical composition boundary for the modular quantum grammar.
 *
 * Language:
 *     Zamani
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file is the QUANTUM GRAMMAR ORCHESTRATOR.
 *
 * It owns:
 *
 *     - quantum source entry points;
 *     - quantum declaration composition;
 *     - quantum block composition;
 *     - quantum statement composition;
 *     - quantum/classical boundary composition;
 *     - integration of specialized quantum grammar fragments;
 *     - grammar-level ownership boundaries;
 *     - the syntax-level quantum extension point.
 *
 * It does NOT own the detailed syntax of:
 *
 *     - qubit declarations;
 *     - register declarations;
 *     - logical qubits;
 *     - physical qubits;
 *     - quantum states;
 *     - operations;
 *     - parameterized operations;
 *     - controlled operations;
 *     - measurements;
 *     - reset;
 *     - observables;
 *     - QEC declarations;
 *     - resource declarations;
 *     - capability declarations;
 *     - dynamic-circuit constructs;
 *     - quantum/classical control details;
 *     - dialect details.
 *
 * Those belong to their dedicated grammar files.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * There MUST be exactly one owner for every production rule.
 *
 * This file MUST NOT redefine a rule already owned by:
 *
 *     grammar/quantum/qubits.g4
 *     grammar/quantum/quantum-registers.g4
 *     grammar/quantum/quantum-states.g4
 *     grammar/quantum/operations.g4
 *     grammar/quantum/controlled-operations.g4
 *     grammar/quantum/parameterized-operations.g4
 *     grammar/quantum/circuits.g4
 *     grammar/quantum/measurement.g4
 *     grammar/quantum/reset.g4
 *     grammar/quantum/observables.g4
 *     grammar/quantum/dynamic-circuits.g4
 *     grammar/quantum/mid-circuit-control.g4
 *     grammar/quantum/quantum-classical.g4
 *     grammar/quantum/logical-qubits.g4
 *     grammar/quantum/physical-qubits.g4
 *     grammar/quantum/error-correction.g4
 *     grammar/quantum/quantum-resources.g4
 *     grammar/quantum/quantum-capabilities.g4
 *     grammar/quantum/quantum-dialects.g4
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexical grammar
 *           |
 *           v
 *     core names / types / expressions
 *           |
 *           v
 *     specialized quantum grammar fragments
 *           |
 *           v
 *     THIS FILE
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
 *     canonical semantic IR
 *           |
 *           +---- quantum::ir
 *           |
 *           +---- optimization
 *           |
 *           +---- routing
 *           |
 *           +---- scheduling
 *           |
 *           +---- QEC
 *           |
 *           +---- ZQN
 *           |
 *           +---- resilience
 *           |
 *           +---- hardware HAL
 *           |
 *           v
 *     target lowering
 *           |
 *           v
 *     runtime
 *
 * This grammar MUST NOT depend on:
 *
 *     quantum::ir
 *     QEC implementation
 *     ZQN implementation
 *     scheduling implementation
 *     routing implementation
 *     optimization implementation
 *     hardware discovery
 *     calibration
 *     runtime dispatch
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Zamani quantum syntax expresses:
 *
 *     computation
 *     intent
 *     semantic structure
 *     resource requirements
 *     capabilities
 *     constraints
 *
 * It MUST NOT encode accidental properties of one machine.
 *
 * Therefore this file contains NO:
 *
 *     MAX_QUBITS
 *     MAX_REGISTERS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_PARAMETERS
 *     MAX_CLASSICAL_BITS
 *     device IDs
 *     QPU IDs
 *     vendor IDs
 *     topology IDs
 *     coupling maps
 *     physical addresses
 *     fixed processor sizes
 *     simulator dimensions
 *     backend names
 *     pulse durations
 *     native gate sets
 *
 * A program may therefore describe a computation requiring an arbitrary
 * number of quantum resources, subject only to semantic validity and the
 * resources available to the compiler/runtime/target.
 *
 * ============================================================================
 * IMPORTANT SCALABILITY DISTINCTION
 * ============================================================================
 *
 * "Unlimited by the grammar" does NOT mean "physically infinite".
 *
 * The grammar imposes no artificial finite machine ceiling.
 *
 * Actual execution remains bounded by:
 *
 *     - available memory;
 *     - parser/compiler resources;
 *     - target capabilities;
 *     - physical resources;
 *     - runtime resources;
 *     - execution policy;
 *     - user/provider constraints.
 *
 * Those limits MUST remain outside the source grammar.
 *
 * ============================================================================
 * CANONICAL LEXICAL CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical lexical vocabulary.
 *
 * It MUST NOT declare lexer rules.
 *
 * In particular, it MUST NOT introduce a second quantum keyword vocabulary.
 *
 * Operation names such as:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     SWAP
 *     U
 *     custom_operation
 *     vendor_operation
 *     future_operation
 *
 * remain semantic names rather than a closed grammar-level gate inventory.
 *
 * ============================================================================
 * SPECIALIZED FILE CONTRACT
 * ============================================================================
 *
 * qubits.g4
 *     Owns source-level qubit declarations and references.
 *
 * quantum-registers.g4
 *     Owns quantum register declarations.
 *
 * quantum-states.g4
 *     Owns quantum state expressions.
 *
 * operations.g4
 *     Owns operation invocation and operation composition.
 *
 * controlled-operations.g4
 *     Owns controlled-operation-specific syntax.
 *
 * parameterized-operations.g4
 *     Owns parameterized operation syntax.
 *
 * circuits.g4
 *     Owns circuit declarations and circuit-specific syntax.
 *
 * measurement.g4
 *     Owns measurement syntax.
 *
 * reset.g4
 *     Owns reset syntax.
 *
 * observables.g4
 *     Owns observable syntax.
 *
 * dynamic-circuits.g4
 *     Owns dynamic-circuit syntax.
 *
 * mid-circuit-control.g4
 *     Owns measurement-dependent control syntax.
 *
 * quantum-classical.g4
 *     Owns explicit quantum/classical boundary constructs.
 *
 * logical-qubits.g4
 *     Owns logical-qubit syntax.
 *
 * physical-qubits.g4
 *     Owns explicit physical-qubit intent syntax.
 *
 * error-correction.g4
 *     Owns QEC source declarations/references only.
 *
 * quantum-resources.g4
 *     Owns quantum resource requirements and resource expressions.
 *
 * quantum-capabilities.g4
 *     Owns capability requirements.
 *
 * quantum-dialects.g4
 *     Owns dialect-extension syntax.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this valid Zamani quantum syntax?"
 *
 * Semantic analysis answers:
 *
 *     "What does this quantum program mean?"
 *
 * Resource analysis answers:
 *
 *     "Can the requested computation be supported by the available resources?"
 *
 * Lowering answers:
 *
 *     "How can the semantic program be realized on this target?"
 *
 * Routing answers:
 *
 *     "How are logical resources mapped to target resources?"
 *
 * Scheduling answers:
 *
 *     "In what order and timing are operations realized?"
 *
 * Optimization answers:
 *
 *     "What equivalent implementation is preferable?"
 *
 * QEC answers:
 *
 *     "How are quantum errors detected/corrected?"
 *
 * ZQN answers:
 *
 *     "What fault/noise semantics apply?"
 *
 * Resilience answers:
 *
 *     "How should execution adapt when conditions change or failures occur?"
 *
 * Hardware HAL answers:
 *
 *     "What can this target actually provide?"
 *
 * Runtime answers:
 *
 *     "How is the compiled computation executed?"
 *
 * This grammar MUST NOT collapse these responsibilities.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. QUANTUM SOURCE ENTRY POINT
 * ========================================================================== */

/*
 * A quantum declaration introduces a quantum computation region.
 *
 * Supported forms are delegated to the circuit grammar and the quantum
 * block grammar.
 *
 * Examples:
 *
 *     quantum {
 *         ...
 *     }
 *
 *     quantum circuit Bell {
 *         ...
 *     }
 *
 * The grammar does not specify how many qubits the declaration contains.
 */
quantumDeclaration
    : K_QUANTUM quantumDeclarationBody
    ;


quantumDeclarationBody
    : quantumCircuitDeclaration
    | quantumBlock
    ;


/* ============================================================================
 * 2. QUANTUM BLOCK
 * ========================================================================== */

/*
 * Quantum blocks are scopes.
 *
 * Scope semantics, ownership, linearity, lifetime and resource accounting
 * belong to semantic analysis.
 */
quantumBlock
    : LBRACE quantumBlockElement* RBRACE
    ;


quantumBlockElement
    : quantumAttributes?
      quantumElement
    ;


/* ============================================================================
 * 3. QUANTUM ELEMENT
 * ========================================================================== */

/*
 * This is the central composition point.
 *
 * Specialized grammar files own the individual constructs.
 *
 * No detailed quantum production is duplicated here.
 */
quantumElement
    : quantumDeclarationElement
    | quantumOperationElement
    | quantumMeasurementElement
    | quantumStateElement
    | quantumControlElement
    | quantumResourceElement
    | quantumCapabilityElement
    | quantumErrorCorrectionElement
    | quantumDialectElement
    | quantumClassicalElement
    | quantumDynamicElement
    | quantumObservableElement
    | quantumStatementExtension
    ;


/* ============================================================================
 * 4. DECLARATION ELEMENTS
 * ========================================================================== */

/*
 * Declaration composition.
 *
 * qubits.g4 and quantum-registers.g4 remain the owners of the detailed
 * declaration syntax.
 */
quantumDeclarationElement
    : quantumQubitDeclaration
    | quantumRegisterDeclarationFamily
    | logicalQubitDeclarationFamily
    | physicalQubitDeclarationStatement
    | quantumCircuitDeclaration
    ;


/* ============================================================================
 * 5. OPERATION ELEMENTS
 * ========================================================================== */

/*
 * operations.g4 is the owner of ordinary operation invocation.
 *
 * controlled-operations.g4 and parameterized-operations.g4 refine operation
 * forms through their own productions.
 *
 * quantum.g4 does not redefine quantumOperationStatement.
 */
quantumOperationElement
    : quantumOperationStatement
    ;


/* ============================================================================
 * 6. MEASUREMENT ELEMENTS
 * ========================================================================== */

/*
 * measurement.g4 owns measurement syntax.
 */
quantumMeasurementElement
    : quantumMeasurementStatement
    ;


/* ============================================================================
 * 7. STATE ELEMENTS
 * ========================================================================== */

/*
 * quantum-states.g4 owns state-expression syntax.
 *
 * A state expression can appear as part of a declaration, initializer,
 * operation argument, measurement-related expression, or another semantic
 * context according to the type system.
 */
quantumStateElement
    : quantumStateExpression
    ;


/* ============================================================================
 * 8. CONTROL ELEMENTS
 * ========================================================================== */

/*
 * Quantum control is intentionally composed from specialized grammar layers.
 *
 * This avoids creating a second control-flow language inside quantum.g4.
 */
quantumControlElement
    : quantumClassicalControlElement
    | quantumDynamicControlElement
    ;


/*
 * Explicit integration boundary for quantum/classical control.
 *
 * quantum-classical.g4 owns the detailed productions.
 */
quantumClassicalControlElement
    : conditionalQuantumOperation
    ;


/*
 * Dynamic circuit control is owned by dynamic-circuits.g4 and
 * mid-circuit-control.g4.
 */
quantumDynamicControlElement
    : dynamicQuantumControl
    | midCircuitQuantumControl
    ;


/* ============================================================================
 * 9. RESOURCE ELEMENTS
 * ========================================================================== */

/*
 * Quantum resource syntax remains declarative.
 *
 * It does not allocate a physical QPU resource.
 */
quantumResourceElement
    : quantumResourceDeclaration
    ;


/* ============================================================================
 * 10. CAPABILITY ELEMENTS
 * ========================================================================== */

/*
 * Capability declarations are semantic requirements.
 *
 * They do not select a particular backend.
 */
quantumCapabilityElement
    : quantumCapabilityDeclaration
    ;


/* ============================================================================
 * 11. ERROR-CORRECTION ELEMENTS
 * ========================================================================== */

/*
 * QEC grammar expresses source-level intent only.
 *
 * This file MUST NOT contain:
 *
 *     stabilizer algorithms
 *     decoders
 *     syndrome extraction algorithms
 *     code implementations
 *     recovery matrices
 *
 * Those belong to the QEC subsystem.
 */
quantumErrorCorrectionElement
    : quantumErrorCorrectionDeclaration
    ;


/* ============================================================================
 * 12. DIALECT ELEMENTS
 * ========================================================================== */

/*
 * Dialects extend the language without forcing this core grammar to enumerate
 * every future quantum technology.
 */
quantumDialectElement
    : quantumDialectDeclaration
    ;


/* ============================================================================
 * 13. QUANTUM/CLASSICAL ELEMENTS
 * ========================================================================== */

/*
 * Explicit quantum/classical boundary constructs are delegated to
 * quantum-classical.g4.
 */
quantumClassicalElement
    : quantumClassicalBoundary
    ;


/* ============================================================================
 * 14. DYNAMIC QUANTUM ELEMENTS
 * ========================================================================== */

/*
 * Dynamic circuit constructs remain separate from ordinary static operation
 * syntax.
 */
quantumDynamicElement
    : dynamicCircuitStatement
    ;


/* ============================================================================
 * 15. OBSERVABLE ELEMENTS
 * ========================================================================== */

/*
 * observables.g4 owns observable syntax.
 */
quantumObservableElement
    : quantumObservationStatement
    ;


/* ============================================================================
 * 16. GENERAL QUANTUM STATEMENT EXTENSION
 * ========================================================================== */

/*
 * This is the controlled extension point for future quantum constructs.
 *
 * IMPORTANT:
 *
 * This is intentionally an explicit rule rather than:
 *
 *     quantumElement : expression ;
 *
 * because accepting every expression here would make malformed quantum syntax
 * silently valid and would weaken diagnostics.
 *
 * New quantum language features must therefore register a dedicated grammar
 * production and add it to this integration point.
 */
quantumStatementExtension
    : quantumBarrierStatement
    | quantumResetStatement
    ;


/* ============================================================================
 * 17. QUANTUM ATTRIBUTES
 * ========================================================================== */

/*
 * Attributes remain owned by the canonical attribute grammar.
 *
 * This wrapper exists only to define the attachment point for quantum
 * constructs.
 */
quantumAttributes
    : attributes
    ;


/* ============================================================================
 * 18. QUANTUM STATEMENT ENTRY POINT
 * ========================================================================== */

/*
 * Canonical parser consumers can use quantumStatement when they are already
 * inside a quantum scope.
 *
 * Declarations are included because quantum resources can have block scope.
 */
quantumStatement
    : quantumElement
    ;


/* ============================================================================
 * 19. QUANTUM PROGRAM COMPOSITION
 * ========================================================================== */

/*
 * A quantum program is structurally a sequence of quantum elements.
 *
 * No maximum element count is imposed.
 */
quantumProgram
    : quantumDeclaration+
    ;


/* ============================================================================
 * 20. QUANTUM SEMANTIC EXTENSION BOUNDARY
 * ========================================================================== */

/*
 * Future quantum domains must enter through this boundary rather than
 * modifying unrelated quantum grammar rules.
 *
 * Examples of future extensions include:
 *
 *     photonic computation
 *     trapped-ion computation
 *     neutral-atom computation
 *     superconducting computation
 *     spin computation
 *     topological computation
 *     measurement-based computation
 *     tensor-network computation
 *     analog quantum computation
 *     distributed quantum computation
 *     quantum networking
 *     quantum sensing
 *     quantum simulation
 *     future quantum substrates
 *
 * Such extensions must describe source semantics rather than a specific
 * physical vendor.
 */
quantumExtensionElement
    : quantumDialectDeclaration
    ;


/* ============================================================================
 * 21. RESOURCE-INDEPENDENT TARGETING
 * ========================================================================== */

/*
 * This grammar intentionally has no target-selection rule such as:
 *
 *     quantum on IBM...
 *     quantum on device...
 *     quantum on qpu...
 *
 * Target selection belongs to compilation/deployment configuration.
 *
 * A source program may express requirements and capabilities, but target
 * realization is downstream.
 */


/* ============================================================================
 * 22. MACHINE-INDEPENDENT RESOURCE CARDINALITY
 * ========================================================================== */

/*
 * No rule in this file contains a finite machine-size cardinality.
 *
 * Correct:
 *
 *     zero or more quantum elements
 *     one or more targets
 *     arbitrary expression-based extents
 *
 * Incorrect:
 *
 *     qubit[32]
 *     qubit[64]
 *     maximumTargets
 *     maximumControls
 *     maximumDepth
 *
 * Resource feasibility belongs outside the grammar.
 */


/* ============================================================================
 * 23. PHYSICAL RESOURCE SEPARATION
 * ========================================================================== */

/*
 * A physical-qubit construct, when explicitly present in the language, is
 * delegated to physical-qubits.g4.
 *
 * This file does not turn a symbolic qubit into a physical allocation.
 *
 * Therefore:
 *
 *     logical qubit
 *
 * and:
 *
 *     physical resource
 *
 * remain different semantic concepts.
 */


/* ============================================================================
 * 24. QEC SEPARATION
 * ========================================================================== */

/*
 * QEC syntax can express intent such as:
 *
 *     logical
 *     code
 *     parity
 *     surface
 *
 * but this file does not define how correction is performed.
 *
 * Downstream:
 *
 *     grammar
 *         ->
 *     AST
 *         ->
 *     semantic QEC intent
 *         ->
 *     QEC subsystem
 *
 * The grammar MUST NOT import Rust QEC implementations.
 */


/* ============================================================================
 * 25. ZQN SEPARATION
 * ========================================================================== */

/*
 * Quantum noise syntax, when present, is an intent/reference boundary.
 *
 * ZQN owns:
 *
 *     fault classes
 *     correlated faults
 *     leakage
 *     loss
 *     erasure
 *     channels
 *     calibration effects
 *     fault semantics
 *
 * quantum.g4 MUST NOT define noise-channel mathematics.
 */


/* ============================================================================
 * 26. IR SEPARATION
 * ========================================================================== */

/*
 * This grammar never constructs quantum::ir directly.
 *
 * The required pipeline is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * Therefore a grammar rule MUST NOT assume a particular IR node layout.
 */


/* ============================================================================
 * 27. OPTIMIZATION SEPARATION
 * ========================================================================== */

/*
 * No optimization decision belongs here.
 *
 * The grammar must accept semantically valid source regardless of whether
 * downstream optimization:
 *
 *     cancels operations;
 *     decomposes operations;
 *     fuses operations;
 *     changes implementation;
 *     changes target realization.
 *
 * Semantic equivalence is verified downstream.
 */


/* ============================================================================
 * 28. ROUTING SEPARATION
 * ========================================================================== */

/*
 * Logical target relationships remain source semantics.
 *
 * Physical connectivity and mapping belong to routing/hardware lowering.
 *
 * No coupling map or topology appears here.
 */


/* ============================================================================
 * 29. SCHEDULING SEPARATION
 * ========================================================================== */

/*
 * The grammar does not encode:
 *
 *     gate duration
 *     pulse duration
 *     clock period
 *     alignment grid
 *     ASAP
 *     ALAP
 *     resource reservation
 *     timing slots
 *
 * Scheduling receives semantic operations and resource constraints later.
 */


/* ============================================================================
 * 30. HARDWARE SEPARATION
 * ========================================================================== */

/*
 * Hardware grammar and HAL own:
 *
 *     devices
 *     capabilities
 *     resources
 *     topology
 *     placement
 *     target descriptions
 *     hardware-specific constraints
 *
 * quantum.g4 only establishes the language-level quantum boundary.
 */


/* ============================================================================
 * 31. DETERMINISM
 * ========================================================================== */

/*
 * This grammar must be deterministic with respect to the canonical lexer and
 * parser configuration.
 *
 * There must be:
 *
 *     no semantic actions;
 *     no filesystem access;
 *     no network access;
 *     no environment inspection;
 *     no hardware discovery;
 *     no runtime calls;
 *     no random decisions.
 *
 * The same source and same grammar version must produce the same parse tree.
 */


/* ============================================================================
 * 32. ERROR-DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * This file intentionally rejects arbitrary expressions as a fallback quantum
 * statement.
 *
 * That is important for production diagnostics.
 *
 * A malformed construct should fail close to its actual source location
 * rather than being swallowed by a generic expression alternative.
 */


/* ============================================================================
 * 33. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Grammar evolution MUST preserve:
 *
 *     existing valid source
 *     existing semantic meaning
 *     existing AST contracts
 *
 * unless a language-version migration explicitly changes them.
 *
 * New quantum constructs should normally be added through:
 *
 *     dedicated specialized grammar file
 *         |
 *         v
 *     quantumElement integration
 *
 * rather than by rewriting unrelated productions.
 */


/* ============================================================================
 * 34. POCO-REAF CONTRACT
 * ========================================================================== */

/*
 * The complete source-to-execution contract is:
 *
 *     Zamani source
 *          |
 *          v
 *     quantum syntax
 *          |
 *          v
 *     semantic meaning
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     target-independent transformations
 *          |
 *          v
 *     target capabilities/resources
 *          |
 *          v
 *     routing / scheduling / optimization
 *          |
 *          v
 *     hardware/runtime realization
 *
 * Therefore:
 *
 *     ONE SOURCE PROGRAM
 *
 * may produce:
 *
 *     MANY VALID TARGET REALIZATIONS
 *
 * without changing the source-level semantic program.
 */


/* ============================================================================
 * 35. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is COMPLETE only when all of the following are true:
 *
 * [x] It owns only quantum grammar composition.
 * [x] It does not duplicate specialized quantum rules.
 * [x] It does not define lexer tokens.
 * [x] It contains no Rust.
 * [x] It contains no unsafe code.
 * [x] It contains no machine-size limit.
 * [x] It contains no fixed qubit count.
 * [x] It contains no fixed register count.
 * [x] It contains no fixed target count.
 * [x] It contains no hardware topology.
 * [x] It contains no vendor-specific operation catalogue.
 * [x] It does not construct IR.
 * [x] It does not implement QEC.
 * [x] It does not implement ZQN.
 * [x] It does not implement routing.
 * [x] It does not implement scheduling.
 * [x] It does not implement optimization.
 * [x] It does not perform hardware discovery.
 * [x] It does not perform runtime dispatch.
 * [x] It provides explicit extension boundaries.
 * [x] It preserves specialized-file ownership.
 * [x] It supports arbitrary source-scale growth.
 *
 * Repository integration still requires the grammar aggregation layer to
 * include the specialized fragments exactly once and requires those fragments
 * to use the same canonical token vocabulary.
 */


/* ============================================================================
 * 36. INTEGRATION MAP
 * ========================================================================== */

/*
 * Required integration:
 *
 *     grammar/lexer/tokens.g4
 *             |
 *             v
 *     grammar/core/*
 *             |
 *             v
 *     grammar/types/*
 *             |
 *             v
 *     grammar/expressions/*
 *             |
 *             +-----------------------------+
 *             |                             |
 *             v                             v
 *     grammar/quantum/qubits.g4     grammar/quantum/quantum-states.g4
 *             |                             |
 *             +-------------+---------------+
 *                           |
 *                           v
 *                  grammar/quantum/quantum.g4
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *        operations    measurement      circuits
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                     frontend AST
 *                           |
 *                           v
 *                   semantic analysis
 *                           |
 *                           v
 *                     quantum::ir
 *                           |
 *        +------------------+-------------------+
 *        |                  |                   |
 *        v                  v                   v
 *   optimization        routing            scheduling
 *        |                  |                   |
 *        +------------------+-------------------+
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *            QEC           ZQN       resilience
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                      hardware HAL
 *                           |
 *                           v
 *                       runtime
 *
 * No dependency may point from these downstream systems back into this
 * grammar.
 */