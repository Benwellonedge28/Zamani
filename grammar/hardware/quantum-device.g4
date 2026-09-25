/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/quantum-device.g4
 *
 * Grammar:
 *     ZamaniHardwareQuantumDeviceParser
 *
 * Status:
 *     CANONICAL LEAF PARSER CONTRACT — QUANTUM HARDWARE DEVICE INTENT
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Action-free ANTLR4 parser grammar.
 *     No embedded Rust.
 *     No unsafe Rust.
 *     No semantic predicates.
 *     No runtime or target queries.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the source-level syntax for describing QUANTUM HARDWARE
 * DEVICE INTENT.
 *
 * It covers:
 *
 *     - quantum-capable device declarations;
 *     - device classes;
 *     - logical-qubit capability;
 *     - physical-qubit capability declarations;
 *     - quantum operation capability;
 *     - measurement capability;
 *     - reset capability;
 *     - dynamic-circuit capability;
 *     - mid-circuit measurement/control capability;
 *     - classical-feed-forward capability;
 *     - quantum resource requirements;
 *     - quantum capability requirements;
 *     - quantum-device constraints;
 *     - quantum-device preferences;
 *     - abstract connectivity requirements;
 *     - timing/quality properties;
 *     - interfaces;
 *     - extensible metadata.
 *
 * The grammar describes SOURCE-LEVEL INTENT.
 *
 * It does NOT select, discover, allocate, route, schedule, calibrate, or
 * execute a physical device.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *
 * Normative quantum specification:
 *
 *     grammar/spec/quantum.md
 *
 * Hardware-domain specification:
 *
 *     grammar/hardware/
 *
 * Canonical lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical complete-language composition:
 *
 *     grammar/Zamani.g4
 *
 * Frontend AST:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     src/quantum/ir/
 *     quantum::ir
 *
 * This file MUST NOT create another quantum IR.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     hardware composition
 *          |
 *          v
 *     this quantum-device contract
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     hardware/resource model       quantum semantic model
 *                                        |
 *                                        v
 *                                   quantum::ir
 *                                        |
 *                         +--------------+--------------+
 *                         |              |              |
 *                         v              v              v
 *                    optimization     routing       scheduling
 *                                                        |
 *                                                        v
 *                                                   QEC/ZQN
 *                                                        |
 *                                                        v
 *                                                       HAL
 *                                                        |
 *                                                        v
 *                                                target realization
 *
 * This file MUST NOT depend on anything below semantic analysis.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantum hardware device declaration shape;
 *     - quantum-device-specific capability declarations;
 *     - quantum-device-specific requirement declarations;
 *     - quantum-device-specific constraint declarations;
 *     - quantum-device-specific preference declarations;
 *     - quantum-device-specific quality/timing properties;
 *     - quantum-device-specific interface declarations;
 *     - symbolic quantum-device kinds;
 *     - device-local quantum capability metadata;
 *     - device-local quantum resource metadata.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * This file DOES NOT own:
 *
 *     - lexer/token definitions;
 *     - identifiers;
 *     - ordinary expressions;
 *     - ordinary types;
 *     - generic resource syntax;
 *     - generic capability syntax;
 *     - physical topology;
 *     - routing;
 *     - placement;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - quantum operations themselves;
 *     - quantum circuits;
 *     - measurement semantics;
 *     - canonical quantum IR;
 *     - frontend AST implementation;
 *     - hardware discovery;
 *     - provider APIs;
 *     - device drivers;
 *     - runtime execution.
 *
 * Those concepts remain owned by their existing grammar/semantic domains.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * This file must not reproduce syntax already owned by:
 *
 *     grammar/expressions/
 *     grammar/types/
 *     grammar/resources/
 *     grammar/quantum/
 *     grammar/hardware/topology.g4
 *     grammar/hardware/placement.g4
 *     grammar/hardware/devices.g4
 *
 * When a shared production exists, this grammar imports/delegates to it.
 *
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The repository's canonical parser boundary is ZamaniParser, which consumes
 * the canonical ZamaniLexer.
 *
 * Therefore this grammar uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * as its production parser boundary.
 *
 * ZamaniTokens is an internal lexical composition mechanism. It is not the
 * parser-facing lexer vocabulary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A quantum-device declaration describes WHAT is required/provided.
 *
 * It does not require:
 *
 *     - a particular vendor;
 *     - a particular backend;
 *     - a particular physical QPU;
 *     - a particular physical qubit;
 *     - a fixed topology;
 *     - a fixed device count;
 *     - a fixed number of qubits;
 *     - a fixed clock;
 *     - a fixed pulse duration;
 *     - a fixed calibration;
 *     - a fixed address.
 *
 * Example semantic intent:
 *
 *     requires capability("quantum.measurement");
 *     requires capability("quantum.mid_circuit_measurement");
 *     requires qubits >= required_qubits;
 *     prefer capability("quantum.fast_measurement");
 *
 * The actual target is resolved downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO language-level quantum hardware capacity constants here.
 *
 * This file MUST NOT contain:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_OPERATIONS
 *     MAX_SHOTS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_CONTROLS
 *     MAX_PARAMETERS
 *     MAX_CHANNELS
 *     MAX_REGISTER_WIDTH
 *
 * Nor may equivalent finite grammar restrictions be introduced.
 *
 * Repetition is expressed using ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than finite bounds.
 *
 * ============================================================================
 * IMPORTANT SEMANTIC DISTINCTION
 * ============================================================================
 *
 * These are different concepts:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     realization
 *
 * A requirement says:
 *
 *     WHAT must be available.
 *
 * A capability says:
 *
 *     WHAT a device can provide.
 *
 * A constraint says:
 *
 *     WHAT must be true.
 *
 * A preference says:
 *
 *     WHAT realization is desirable.
 *
 * A physical realization says:
 *
 *     WHICH actual hardware is used.
 *
 * Only the last category belongs downstream.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * This grammar never creates a quantum IR.
 *
 * When a declaration refers to quantum computation, the semantic pipeline is:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareQuantumDeviceParser;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The parent hardware grammar owns the placement of this declaration in the
 * overall hardware grammar.
 *
 * This production owns only the quantum-device declaration itself.
 */
hardwareQuantumDeviceDecl
    : hardwareQuantumDeviceModifiers?
      K_QUANTUM
      K_DEVICE
      hardwareQuantumDeviceKind?
      identifier
      hardwareQuantumDeviceParameterClause?
      hardwareQuantumDeviceRequirementClause?
      hardwareQuantumDeviceProvisionClause?
      hardwareQuantumDeviceBody
    ;


/* ============================================================================
 * 2. MODIFIERS
 * ============================================================================
 *
 * Reuse the language's existing modifier tokens.
 *
 * No hardware-specific modifier vocabulary is created here.
 */
hardwareQuantumDeviceModifiers
    : hardwareQuantumDeviceModifier+
    ;

hardwareQuantumDeviceModifier
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/* ============================================================================
 * 3. DEVICE KIND
 * ============================================================================
 *
 * The kind is deliberately open-ended.
 *
 * Examples of semantic values include:
 *
 *     qpu
 *     simulator
 *     annealer
 *     photonic
 *     neutral_atom
 *     ion_trap
 *     superconducting
 *     spin
 *     topological
 *
 * These are IDENTIFIER values, not a closed parser enumeration.
 */
hardwareQuantumDeviceKind
    : identifier
    ;


/* ============================================================================
 * 4. GENERIC/PARAMETER CLAUSE
 * ============================================================================
 *
 * Device descriptions can be parameterized by symbolic quantities.
 *
 * This is source-level parameterization, not a hardware allocation mechanism.
 */
hardwareQuantumDeviceParameterClause
    : LT
      hardwareQuantumDeviceParameter
      (
          COMMA
          hardwareQuantumDeviceParameter
      )*
      GT
    ;

hardwareQuantumDeviceParameter
    : identifier
      (
          COLON
          type
      )?
      (
          ASSIGN
          expression
      )?
    ;


/* ============================================================================
 * 5. TOP-LEVEL REQUIREMENTS
 * ============================================================================
 *
 * Requirements are declarative.
 *
 * They do not allocate resources.
 */
hardwareQuantumDeviceRequirementClause
    : K_REQUIRES
      LBRACE
      hardwareQuantumDeviceRequirement*
      RBRACE
    ;

hardwareQuantumDeviceRequirement
    : hardwareQuantumCapabilityRequirement
    | hardwareQuantumResourceRequirement
    | hardwareQuantumPropertyRequirement
    ;


/* ============================================================================
 * 6. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Capability names remain open and namespaced.
 */
hardwareQuantumCapabilityRequirement
    : K_CAPABILITY
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 7. RESOURCE REQUIREMENT
 * ============================================================================
 *
 * The resource expression is intentionally delegated to the normal expression
 * grammar.
 *
 * Examples of semantic forms include:
 *
 *     qubits >= required_qubits
 *     logical_qubits >= algorithm_width
 *
 * No physical allocation is implied.
 */
hardwareQuantumResourceRequirement
    : K_RESOURCE
      qualifiedName
      hardwareQuantumComparison?
      expression
      SEMICOLON
    ;

hardwareQuantumComparison
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_EQUAL
    | GREATER_EQUAL
    | LT
    | GT
    ;


/* ============================================================================
 * 8. PROPERTY REQUIREMENT
 * ============================================================================
 *
 * Generic property requirements allow future quantum technologies to expose
 * properties without adding a parser keyword for every new technology.
 */
hardwareQuantumPropertyRequirement
    : qualifiedName
      hardwareQuantumComparison
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 9. TOP-LEVEL PROVISIONS
 * ============================================================================
 *
 * "provides" describes what a declared device class/capability contract
 * exposes.
 *
 * It does not claim that the current runtime has discovered such a device.
 */
hardwareQuantumDeviceProvisionClause
    : K_PROVIDES
      LBRACE
      hardwareQuantumProvision*
      RBRACE
    ;

hardwareQuantumProvision
    : K_CAPABILITY
      qualifiedName
      hardwareQuantumValueClause?
      SEMICOLON
    | qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;

hardwareQuantumValueClause
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 10. DEVICE BODY
 * ============================================================================
 */

hardwareQuantumDeviceBody
    : LBRACE
      hardwareQuantumDeviceItem*
      RBRACE
    ;


/* ============================================================================
 * 11. DEVICE ITEMS
 * ============================================================================
 */

hardwareQuantumDeviceItem
    : hardwareQuantumDeviceAnnotation*
      hardwareQuantumResourceDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumOperationCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumQubitCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumLogicalQubitCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumPhysicalQubitCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumMeasurementCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumResetCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumDynamicCircuitCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumMidCircuitCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumClassicalControlCapabilityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumConnectivityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumTimingDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumQualityDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumMemoryDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumInterfaceDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumRequirementDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumConstraintDeclaration

    | hardwareQuantumDeviceAnnotation*
      hardwareQuantumPreferenceDeclaration

    | hardwareQuantumTargetReferenceDeclaration

    | hardwareQuantumPropertyDeclaration

    | hardwareQuantumExtensionDeclaration
    ;


/* ============================================================================
 * 12. ANNOTATIONS
 * ============================================================================
 */

hardwareQuantumDeviceAnnotation
    : AT
      qualifiedName
      (
          LPAREN
          expressionList?
          RPAREN
      )?
    ;


/* ============================================================================
 * 13. RESOURCE DECLARATION
 * ============================================================================
 *
 * This is device-local resource metadata.
 *
 * Generic resource semantics remain owned by grammar/resources/.
 */
hardwareQuantumResourceDeclaration
    : K_RESOURCE
      identifier?
      hardwareQuantumResourceBody
    ;

hardwareQuantumResourceBody
    : LBRACE
      hardwareQuantumResourceProperty*
      RBRACE
    ;

hardwareQuantumResourceProperty
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. GENERIC CAPABILITY DECLARATION
 * ============================================================================
 */

hardwareQuantumCapabilityDeclaration
    : K_CAPABILITY
      qualifiedName
      hardwareQuantumCapabilityBody?
      SEMICOLON?
    ;

hardwareQuantumCapabilityBody
    : LBRACE
      hardwareQuantumCapabilityProperty*
      RBRACE
    ;

hardwareQuantumCapabilityProperty
    : qualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 15. OPERATION CAPABILITY
 * ============================================================================
 *
 * CRITICAL:
 *
 * There is intentionally NO:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * enumeration.
 *
 * Operation identity remains semantic data and ultimately maps through the
 * canonical quantum semantic model and quantum::ir.
 */
hardwareQuantumOperationCapabilityDeclaration
    : K_OPERATION
      qualifiedName
      hardwareQuantumOperationCapabilityBody
    ;

hardwareQuantumOperationCapabilityBody
    : LBRACE
      hardwareQuantumOperationCapabilityProperty*
      RBRACE
    ;

hardwareQuantumOperationCapabilityProperty
    : K_SUPPORTED
      ASSIGN
      expression
      SEMICOLON

    | K_ARITY
      ASSIGN
      expression
      SEMICOLON

    | K_PARAMETER
      ASSIGN
      expression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      expression
      SEMICOLON

    | K_DURATION
      ASSIGN
      expression
      SEMICOLON

    | K_FIDELITY
      ASSIGN
      expression
      SEMICOLON

    | K_ERROR
      ASSIGN
      expression
      SEMICOLON

    | K_PRECISION
      ASSIGN
      expression
      SEMICOLON

    | qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 16. QUBIT CAPABILITY
 * ============================================================================
 *
 * This describes the existence/capability of a class of qubit resources.
 *
 * It does NOT allocate qubits.
 */
hardwareQuantumQubitCapabilityDeclaration
    : K_QUBIT
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 17. LOGICAL QUBIT CAPABILITY
 * ============================================================================
 */

hardwareQuantumLogicalQubitCapabilityDeclaration
    : K_LOGICAL
      K_QUBIT
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 18. PHYSICAL QUBIT CAPABILITY
 * ============================================================================
 *
 * Physical-qubit declarations are capability descriptions only.
 *
 * They do not enumerate physical qubit IDs.
 */
hardwareQuantumPhysicalQubitCapabilityDeclaration
    : K_PHYSICAL
      K_QUBIT
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 19. MEASUREMENT CAPABILITY
 * ============================================================================
 */

hardwareQuantumMeasurementCapabilityDeclaration
    : K_MEASUREMENT
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 20. RESET CAPABILITY
 * ============================================================================
 */

hardwareQuantumResetCapabilityDeclaration
    : K_RESET
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 21. DYNAMIC-CIRCUIT CAPABILITY
 * ============================================================================
 */

hardwareQuantumDynamicCircuitCapabilityDeclaration
    : K_DYNAMIC
      K_CIRCUIT
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 22. MID-CIRCUIT CAPABILITY
 * ============================================================================
 *
 * The name is intentionally generic.
 *
 * Semantic analysis determines whether the declaration represents:
 *
 *     mid-circuit measurement
 *     dynamic control
 *     feed-forward
 *     another dynamic-circuit capability
 */
hardwareQuantumMidCircuitCapabilityDeclaration
    : K_MID
      K_CIRCUIT
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 23. CLASSICAL CONTROL CAPABILITY
 * ============================================================================
 *
 * This describes the hardware's ability to react to classical information
 * during quantum execution.
 *
 * It does not redefine Zamani classical syntax.
 */
hardwareQuantumClassicalControlCapabilityDeclaration
    : K_CLASSICAL
      K_CONTROL
      identifier?
      hardwareQuantumCapabilityBody
    ;


/* ============================================================================
 * 24. CONNECTIVITY
 * ============================================================================
 *
 * Connectivity here expresses a DECLARATIVE CAPABILITY/REQUIREMENT.
 *
 * Physical topology remains owned by:
 *
 *     grammar/hardware/topology.g4
 *
 * and downstream topology/routing systems.
 *
 * This rule therefore does not enumerate physical edges.
 */
hardwareQuantumConnectivityDeclaration
    : K_CONNECTIVITY
      identifier?
      hardwareQuantumConnectivityBody
    ;

hardwareQuantumConnectivityBody
    : LBRACE
      hardwareQuantumConnectivityProperty*
      RBRACE
    ;

hardwareQuantumConnectivityProperty
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 25. TIMING
 * ============================================================================
 *
 * Timing values are expressions.
 *
 * No universal hardware clock or timing resolution is assumed.
 */
hardwareQuantumTimingDeclaration
    : K_TIMING
      identifier?
      hardwareQuantumTimingBody
    ;

hardwareQuantumTimingBody
    : LBRACE
      hardwareQuantumTimingProperty*
      RBRACE
    ;

hardwareQuantumTimingProperty
    : K_LATENCY
      ASSIGN
      expression
      SEMICOLON

    | K_DURATION
      ASSIGN
      expression
      SEMICOLON

    | K_PERIOD
      ASSIGN
      expression
      SEMICOLON

    | K_FREQUENCY
      ASSIGN
      expression
      SEMICOLON

    | K_DEADLINE
      ASSIGN
      expression
      SEMICOLON

    | K_SETUP
      ASSIGN
      expression
      SEMICOLON

    | K_HOLD
      ASSIGN
      expression
      SEMICOLON

    | qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 26. QUALITY / FIDELITY / ERROR CONTRACT
 * ============================================================================
 *
 * These are semantic properties.
 *
 * The grammar does not impose universal thresholds.
 *
 * Examples:
 *
 *     fidelity >= required_fidelity
 *     error <= maximum_error
 *
 * The meaning is resolved semantically.
 */
hardwareQuantumQualityDeclaration
    : K_FIDELITY
      identifier?
      hardwareQuantumQualityBody
    | K_ERROR
      identifier?
      hardwareQuantumQualityBody
    | K_PRECISION
      identifier?
      hardwareQuantumQualityBody
    ;

hardwareQuantumQualityBody
    : LBRACE
      hardwareQuantumQualityProperty*
      RBRACE
    ;

hardwareQuantumQualityProperty
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 27. MEMORY
 * ============================================================================
 *
 * Quantum memory is an abstract resource/capability.
 *
 * This does NOT define a physical memory architecture.
 */
hardwareQuantumMemoryDeclaration
    : K_MEMORY
      identifier?
      hardwareQuantumMemoryBody
    ;

hardwareQuantumMemoryBody
    : LBRACE
      hardwareQuantumMemoryProperty*
      RBRACE
    ;

hardwareQuantumMemoryProperty
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 28. INTERFACE
 * ============================================================================
 *
 * An interface is a source-level relationship.
 *
 * Provider-specific APIs remain outside this grammar.
 */
hardwareQuantumInterfaceDeclaration
    : K_INTERFACE
      identifier
      hardwareQuantumInterfaceBody
    ;

hardwareQuantumInterfaceBody
    : LBRACE
      hardwareQuantumInterfaceItem*
      RBRACE
    ;

hardwareQuantumInterfaceItem
    : K_CAPABILITY
      qualifiedName
      SEMICOLON

    | K_REQUIRE
      qualifiedName
      SEMICOLON

    | qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 29. REQUIREMENT DECLARATION
 * ============================================================================
 *
 * This is a device-local requirement block.
 *
 * It is intentionally separate from the top-level "requires" clause so that
 * declarations remain composable.
 */
hardwareQuantumRequirementDeclaration
    : K_REQUIRE
      identifier?
      hardwareQuantumRequirementBody
    ;

hardwareQuantumRequirementBody
    : LBRACE
      hardwareQuantumRequirementItem*
      RBRACE
    ;

hardwareQuantumRequirementItem
    : K_CAPABILITY
      qualifiedName
      SEMICOLON

    | K_RESOURCE
      qualifiedName
      hardwareQuantumComparison?
      expression
      SEMICOLON

    | qualifiedName
      hardwareQuantumComparison
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 30. CONSTRAINT DECLARATION
 * ============================================================================
 */

hardwareQuantumConstraintDeclaration
    : K_CONSTRAINT
      identifier?
      hardwareQuantumConstraintBody
    ;

hardwareQuantumConstraintBody
    : LBRACE
      hardwareQuantumConstraintItem*
      RBRACE
    ;

hardwareQuantumConstraintItem
    : expression
      hardwareQuantumComparison
      expression
      SEMICOLON

    | K_NOT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 31. PREFERENCE DECLARATION
 * ============================================================================
 *
 * Preferences are advisory optimization intent.
 *
 * They MUST NOT be interpreted as mandatory physical placement.
 */
hardwareQuantumPreferenceDeclaration
    : K_PREFER
      hardwareQuantumPreferenceBody
    ;

hardwareQuantumPreferenceBody
    : LBRACE
      hardwareQuantumPreferenceItem*
      RBRACE
    ;

hardwareQuantumPreferenceItem
    : qualifiedName
      (
          ASSIGN
        | FAT_ARROW
      )
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 32. TARGET REFERENCE
 * ============================================================================
 *
 * This is deliberately a symbolic reference.
 *
 * It does not perform physical device selection.
 */
hardwareQuantumTargetReferenceDeclaration
    : K_TARGET
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 33. GENERIC PROPERTY
 * ============================================================================
 *
 * Forward-compatible metadata.
 *
 * New quantum technologies can attach semantic properties without requiring
 * a new parser keyword for every possible property.
 */
hardwareQuantumPropertyDeclaration
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 34. EXTENSION
 * ============================================================================
 *
 * Extensions are explicitly namespaced and remain subject to semantic
 * validation and dialect compatibility.
 */
hardwareQuantumExtensionDeclaration
    : K_EXTEND
      qualifiedName
      hardwareQuantumExtensionBody
    ;

hardwareQuantumExtensionBody
    : LBRACE
      hardwareQuantumExtensionItem*
      RBRACE
    ;

hardwareQuantumExtensionItem
    : hardwareQuantumCapabilityDeclaration
    | hardwareQuantumRequirementDeclaration
    | hardwareQuantumConstraintDeclaration
    | hardwareQuantumPreferenceDeclaration
    | hardwareQuantumPropertyDeclaration
    ;


/* ============================================================================
 * 35. COMMON NAME ADAPTERS
 * ============================================================================
 *
 * These productions are deliberately thin adapters to the canonical
 * expression/type/name grammar.
 *
 * They do not define a second naming or expression language.
 *
 * If the exact canonical rule names change, only these adapters need to be
 * updated; the quantum-device semantic contract remains unchanged.
 */
identifier
    : IDENTIFIER
    ;

qualifiedName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;

expression
    : logicalOrExpression
    ;

type
    : qualifiedName
    ;


/* ============================================================================
 * 36. EXPRESSION BRIDGE
 * ============================================================================
 *
 * IMPORTANT:
 *
 * These bridge productions exist solely so this leaf grammar can be generated
 * independently during modular grammar validation.
 *
 * In the canonical combined parser, the corresponding productions MUST be
 * mapped to the canonical expression grammar by the parser composition root.
 *
 * This file does not define domain-specific expression semantics.
 *
 * ============================================================================
 */

logicalOrExpression
    : logicalAndExpression
      (
          LOGICAL_OR
          logicalAndExpression
      )*
    ;

logicalAndExpression
    : equalityExpression
      (
          LOGICAL_AND
          equalityExpression
      )*
    ;

equalityExpression
    : relationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          relationalExpression
      )*
    ;

relationalExpression
    : additiveExpression
      (
          (
              LESS_EQUAL
            | GREATER_EQUAL
            | LT
            | GT
          )
          additiveExpression
      )*
    ;

additiveExpression
    : multiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          multiplicativeExpression
      )*
    ;

multiplicativeExpression
    : unaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          unaryExpression
      )*
    ;

unaryExpression
    : (
          PLUS
        | MINUS
        | EXCLAMATION
      )
      unaryExpression
    | primaryExpression
    ;

primaryExpression
    : identifier
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | K_TRUE
    | K_FALSE
    | LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 37. EXPRESSION LIST
 * ============================================================================
 */

expressionList
    : expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 38. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Parent grammar:
 *
 *     grammar/hardware/hardware.g4
 *
 * MUST import this grammar and expose:
 *
 *     hardwareQuantumDeviceDecl
 *
 * as one of its hardware declaration alternatives.
 *
 * Canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * composes hardware.g4 through the normal hardware-domain dispatcher.
 *
 * Canonical root:
 *
 *     grammar/Zamani.g4
 *
 * remains the complete-language composition root.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not create AST types.
 *
 * The parser output must map into the domain-neutral frontend AST.
 *
 * The AST representation must preserve at least:
 *
 *     - declaration identity;
 *     - device kind;
 *     - parameters;
 *     - capabilities;
 *     - requirements;
 *     - resources;
 *     - constraints;
 *     - preferences;
 *     - quality properties;
 *     - timing properties;
 *     - connectivity intent;
 *     - interface declarations;
 *     - extension metadata;
 *     - source spans.
 *
 * No physical device identity may be silently substituted for the source
 * declaration.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - resolve names;
 *     - validate types;
 *     - validate capability references;
 *     - validate resource expressions;
 *     - distinguish requirements from preferences;
 *     - distinguish logical from physical resources;
 *     - validate operation capability metadata;
 *     - validate quality predicates;
 *     - validate timing relationships;
 *     - validate extension/dialect ownership;
 *     - preserve source provenance;
 *     - detect contradictory requirements;
 *     - detect unsatisfiable constraints;
 *     - avoid imposing arbitrary hardware limits.
 *
 * Semantic analysis MAY compare requirements against a concrete target.
 *
 * The grammar MUST NOT do so.
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * A declaration involving quantum computation may contribute semantic metadata
 * to the canonical quantum compilation pipeline.
 *
 * It must never create:
 *
 *     QuantumDeviceIR
 *     QuantumHardwareIR
 *     QuantumGateIR
 *     QuantumDeviceIntermediateRepresentation
 *
 * as a competing quantum representation.
 *
 * The canonical boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HARDWARE/HAL CONTRACT
 * ============================================================================
 *
 * Hardware discovery and realization are downstream.
 *
 * This grammar cannot:
 *
 *     - query a QPU;
 *     - query a simulator;
 *     - inspect available qubits;
 *     - inspect topology;
 *     - inspect calibration;
 *     - inspect device health;
 *     - select a provider;
 *     - open a device;
 *     - submit a circuit.
 *
 * A concrete target may be matched later against the semantic requirements.
 *
 * ============================================================================
 * TOPOLOGY CONTRACT
 * ============================================================================
 *
 * This file does not define physical topology.
 *
 * Topology belongs to:
 *
 *     grammar/hardware/topology.g4
 *
 * and its downstream semantic/routing implementation.
 *
 * A connectivity property in this file therefore means:
 *
 *     "this relationship/capability is required or provided"
 *
 * rather than:
 *
 *     "use these physical edges".
 *
 * ============================================================================
 * QEC / ZQN CONTRACT
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     QEC algorithms;
 *     code distances;
 *     syndrome extraction;
 *     decoder behavior;
 *     noise models;
 *     fault models;
 *     ZQN state;
 *     resilience policy implementation.
 *
 * It may express requirements/properties that downstream QEC/ZQN systems
 * consume.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource quantities are semantic expressions.
 *
 * Examples:
 *
 *     qubits >= required_qubits
 *     logical_qubits >= workload_width
 *     memory >= required_memory
 *
 * The grammar imposes no maximum.
 *
 * A concrete resource manager may reject an unsatisfied requirement.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * Portable source should prefer:
 *
 *     symbolic capabilities;
 *     resource requirements;
 *     topology relationships;
 *     timing requirements;
 *     quality requirements;
 *     abstract interfaces.
 *
 * Portable source should avoid unnecessary:
 *
 *     physical IDs;
 *     physical addresses;
 *     vendor identifiers;
 *     backend-specific names;
 *     calibration constants;
 *     topology coordinates.
 *
 * Such target-specific constructs, when supported elsewhere in Zamani, are
 * explicitly target-specific and must never silently become portable
 * requirements.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The grammar performs no:
 *
 *     - runtime queries;
 *     - network queries;
 *     - filesystem access;
 *     - hardware discovery;
 *     - environment inspection;
 *     - randomness;
 *     - time-dependent branching;
 *     - semantic predicates.
 *
 * Identical source + identical grammar + identical language version produces
 * the same parse structure.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Hardware declarations are not authorization grants.
 *
 * A source construct such as:
 *
 *     target device
 *
 * or:
 *
 *     capability quantum::measurement
 *
 * MUST NOT imply:
 *
 *     privileged access;
 *     device ownership;
 *     DMA permission;
 *     MMIO permission;
 *     credential access;
 *     network access;
 *     execution permission.
 *
 * Authorization remains owned by the semantic/security/runtime layers.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Generated Rust code must compile under:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * The Zamani implementation must remain safe Rust.
 *
 * This grammar introduces no unsafe code.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The following are stable architectural invariants:
 *
 *     hardwareQuantumDeviceDecl
 *     quantum-device capability intent
 *     open-ended operation identity
 *     symbolic resource requirements
 *     capability requirements
 *     no fixed quantum hardware capacities
 *
 * Renaming public grammar rules or token references is a compatibility
 * operation and requires updates to:
 *
 *     grammar/compatibility/
 *     grammar/spec/
 *     grammar/tests/
 *     canonical parser composition
 *     implementation-conformance documentation.
 *
 * ============================================================================
 * REQUIRED TEST MATRIX
 * ============================================================================
 *
 * Positive:
 *
 *     quantum device simulator { ... }
 *     quantum device qpu { ... }
 *     quantum device photonic { ... }
 *     quantum device future_quantum_substrate { ... }
 *
 * Capability:
 *
 *     capability quantum::measurement;
 *     capability quantum::dynamic_control;
 *     capability future::quantum::new_operation;
 *
 * Requirements:
 *
 *     requires {
 *         capability quantum::measurement;
 *         resource qubits >= required_qubits;
 *     }
 *
 * Operation capability:
 *
 *     operation quantum::custom_operation {
 *         supported = true;
 *         arity = operation_arity;
 *         fidelity = required_fidelity;
 *     }
 *
 * Logical/physical distinction:
 *
 *     logical qubit logical_resources { ... }
 *     physical qubit physical_resources { ... }
 *
 * Timing:
 *
 *     timing execution {
 *         latency = required_latency;
 *         duration = required_duration;
 *     }
 *
 * Quality:
 *
 *     fidelity quality {
 *         minimum = required_fidelity;
 *     }
 *
 * Connectivity:
 *
 *     connectivity network {
 *         topology = required_topology;
 *     }
 *
 * Preferences:
 *
 *     prefer {
 *         quantum::fast_measurement => preferred;
 *     }
 *
 * Extension:
 *
 *     extend future::quantum::capability {
 *         property = value;
 *     }
 *
 * Negative:
 *
 *     malformed declarations;
 *     missing identifiers;
 *     malformed capability names;
 *     malformed comparisons;
 *     malformed blocks;
 *     invalid punctuation.
 *
 * Scalability:
 *
 *     arbitrarily many device items;
 *     arbitrarily many capabilities;
 *     arbitrarily many requirements;
 *     arbitrarily many operation descriptions;
 *     arbitrarily large symbolic quantities;
 *     arbitrarily deep qualified names where supported by the canonical
 *     identifier grammar.
 *
 * There must be NO test that establishes a universal maximum number of:
 *
 *     qubits;
 *     devices;
 *     capabilities;
 *     operations;
 *     parameters;
 *     resources;
 *     interfaces.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST fail review if it introduces:
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
 * or equivalent parser-level ceilings.
 *
 * It also MUST fail review if it introduces closed lists of:
 *
 *     vendors;
 *     QPU providers;
 *     quantum technologies;
 *     gate names;
 *     physical qubit IDs;
 *     backend names.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one parser grammar identity.
 *     [x] It consumes the canonical ZamaniLexer vocabulary.
 *     [x] It contains no embedded Rust.
 *     [x] It contains no unsafe implementation.
 *     [x] It has no runtime dependencies.
 *     [x] It has no hardware discovery.
 *     [x] It has no physical allocation.
 *     [x] It has no fixed hardware capacity.
 *     [x] It has no finite gate catalogue.
 *     [x] It keeps operation identity open-ended.
 *     [x] It keeps resource quantities expression-based.
 *     [x] It separates requirements from capabilities.
 *     [x] It separates preferences from requirements.
 *     [x] It separates logical from physical resources.
 *     [x] It preserves the quantum::ir boundary.
 *     [x] It leaves routing downstream.
 *     [x] It leaves scheduling downstream.
 *     [x] It leaves QEC downstream.
 *     [x] It leaves ZQN downstream.
 *     [x] It leaves calibration downstream.
 *     [x] It leaves HAL/device discovery downstream.
 *     [x] It defines AST integration.
 *     [x] It defines semantic integration.
 *     [x] It defines resource integration.
 *     [x] It defines capability integration.
 *     [x] It defines topology integration.
 *     [x] It defines compatibility integration.
 *     [x] It defines testing requirements.
 *
 * ============================================================================
 * IMPORTANT IMPLEMENTATION NOTE
 * ============================================================================
 *
 * The small expression bridge above exists solely for independent validation
 * of this leaf grammar.
 *
 * In the FINAL COMPOSED Zamani parser, the parser composition layer must map
 * these bridge productions to the canonical shared expression/type/name
 * productions rather than allowing multiple semantic expression systems.
 *
 * No semantic meaning depends on the bridge's local rule names.
 *
 * The important stable contract is the quantum-device syntax and its semantic
 * ownership.
 *
 * ============================================================================
 */