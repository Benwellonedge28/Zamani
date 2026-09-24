/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* File:
* grammar/quantum/quantum.g4
* 
* Grammar:
* Quantum
* 
* Status:
* CANONICAL QUANTUM-DOMAIN COMPOSITION GRAMMAR
* 
* Language:
* Zamani
* 
* Compiler baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* safe Rust only
* no unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the SINGLE QUANTUM-DOMAIN COMPOSITION ROOT.
* 
* It composes the independently owned quantum grammar contracts into one
* parser-visible quantum domain.
* 
* This file is intentionally NOT a second quantum language and is NOT a
* second complete-program grammar.
* 
* The complete Zamani parser entry point remains owned by:
* 
* grammar/antlr/ZamaniParser.g4
* 
* The canonical source-language composition root remains:
* 
* grammar/Zamani.g4
* 
* This file owns only:
* 
* - quantum-domain composition;
* - quantum declaration dispatch;
* - quantum statement dispatch;
* - quantum expression dispatch;
* - quantum type dispatch;
* - quantum-domain extension dispatch;
* - cross-boundary composition between quantum subdomains.
* 
* It does NOT own detailed leaf syntax.
* 
* ============================================================================
* ARCHITECTURAL AUTHORITY
* ============================================================================
* 
* Normative architecture:
* 
* grammar/DESIGN.md
* 
* Normative quantum specification:
* 
* grammar/spec/quantum.md
* 
* Normative syntax specification:
* 
* grammar/spec/syntax.md
* 
* Canonical language composition:
* 
* grammar/Zamani.g4
* 
* Canonical complete parser:
* 
* grammar/antlr/ZamaniParser.g4
* 
* Canonical lexical authority:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Frontend AST:
* 
* src/frontend/ast/
* 
* Canonical quantum semantic boundary:
* 
* quantum::ir
* 
* No grammar in this file creates another AST or another IR.
* 
* ============================================================================
* DEPENDENCY DIRECTION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* canonical parser
*      |
*      v
* Quantum
*      |
* +----+-----------------------------+
* |                                  |
* v                                  v
* quantum leaf grammars           universal grammar contracts
* |                                  |
* +----------------+-----------------+
*                  |
*                  v
*         domain-neutral AST
*                  |
*                  v
*          semantic analysis
*                  |
*   +--------------+---------------+
*   |              |               |
*   v              v               v
* types          effects        resources
*   |              |               |
*   +--------------+---------------+
*                  |
*                  v
*         canonical semantic model
*                  |
*                  v
*              quantum::ir
*                  |
*   +--------------+---------------+
*   |              |               |
*   v              v               v
* optimize        routing        scheduling
*                  |
*                  v
*            QEC / resilience
*                  |
*                  v
*                 ZQN
*                  |
*                  v
*                 HAL
*                  |
*                  v
*           target realization
* 
* This grammar MUST NOT depend on anything below parsing.
* 
* ============================================================================
* SINGLE-OWNER RULE
* ============================================================================
* 
* Every production has exactly one canonical owner.
* 
* This file may:
* 
* - import a production owner;
* - expose a domain-level wrapper;
* - dispatch to a production owner.
* 
* This file MUST NOT:
* 
* - duplicate leaf productions;
* - redefine identifiers;
* - redefine expressions;
* - redefine types;
* - redefine operation syntax;
* - redefine measurement syntax;
* - redefine register syntax;
* - redefine state syntax;
* - redefine QEC syntax;
* - redefine capability syntax;
* - redefine resource syntax;
* - define hardware topology;
* - define physical allocation;
* - define routing;
* - define scheduling;
* - define calibration;
* - implement QEC;
* - implement ZQN;
* - implement HAL;
* - define runtime behavior.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Quantum source describes portable computational intent.
* 
* It MUST NOT impose universal implementation limits on:
* 
* qubits
* registers
* logical qubits
* physical qubits
* operations
* parameters
* controls
* targets
* measurements
* circuit depth
* circuit width
* devices
* QPUs
* memories
* timelines
* channels
* nodes
* tensor rank
* tensor dimensions
* 
* No language-level constants such as the following may appear:
* 
* MAX_QUBITS
* MAX_REGISTERS
* MAX_TARGETS
* MAX_CONTROLS
* MAX_PARAMETERS
* MAX_CIRCUIT_DEPTH
* MAX_DEVICES
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* 
* or equivalent constructs.
* 
* "*", "+", optional elements, symbolic expressions, and user-supplied
* extents represent source structure. They do not represent an implementation
* ceiling.
* 
* Actual finite limitations are determined downstream by:
* 
* - semantic validity;
* - declared requirements;
* - target capabilities;
* - available resources;
* - compiler resources;
* - runtime resources;
* - deployment policy.
* 
* ============================================================================
* OPEN-WORLD OPERATION MODEL
* ============================================================================
* 
* Quantum operations are OPEN-ENDED.
* 
* This file MUST NEVER enumerate:
* 
* H
* X
* Y
* Z
* S
* T
* CNOT
* CZ
* SWAP
* RX
* RY
* RZ
* 
* or any other finite gate catalogue.
* 
* Operation identity belongs to the operation grammar and semantic resolver.
* 
* Therefore the language can represent:
* 
* apply H(q);
* apply X(q);
* apply CNOT(q0, q1);
* apply RX(theta)(q);
* apply custom_gate(q);
* apply library::operation(q);
* apply vendor::operation(parameter)(q0, q1);
* apply future::operation(args)(targets);
* 
* without changing this composition root.
* 
* ============================================================================
* TARGET-INDEPENDENT HARDWARE MODEL
* ============================================================================
* 
* This grammar does not select:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* physical qubit
* physical register
* device
* vendor
* coupling map
* topology
* calibration
* pulse
* scheduler
* router
* memory bank
* 
* Target-specific realization belongs downstream.
* 
* ============================================================================
* RESOURCE / CAPABILITY SEPARATION
* ============================================================================
* 
* Quantum source may express:
* 
* requirements
* constraints
* capabilities
* preferences
* hints
* budgets
* 
* These are semantic intent.
* 
* They are NOT physical allocation commands.
* 
* Example:
* 
* requires qubits >= n
* 
* means that the eventual execution environment must provide sufficient
* quantum resources.
* 
* It does NOT mean:
* 
* use physical qubits 0 through n-1
* 
* or:
* 
* select QPU 0
* 
* Capability examples:
* 
* requires capability("quantum.measurement")
* requires capability("quantum.mid_circuit_measurement")
* requires capability("quantum.dynamic_control")
* 
* remain target-independent requirements.
* 
* ============================================================================
* CANONICAL QUANTUM IR
* ============================================================================
* 
* All quantum constructs eventually cross exactly one canonical quantum IR
* boundary:
* 
* quantum::ir
* 
* This grammar MUST NOT introduce:
* 
* QuantumIR
* QuantumGateIR
* QuantumOperationIR
* QuantumCircuitIR
* QuantumHardwareIR
* 
* as competing intermediate representations.
* 
* The frontend path is:
* 
* parser
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic analysis
*   |
*   v
* quantum::ir
* 
* Quantum optimization, decomposition, routing, scheduling, QEC, resilience,
* ZQN, HAL and backend lowering occur after this boundary.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing MUST depend only upon:
* 
* - source text;
* - selected language version;
* - canonical lexical vocabulary;
* - canonical parser grammar;
* - explicitly selected dialect/version.
* 
* Parsing MUST NOT depend upon:
* 
* - hardware discovery;
* - QPU availability;
* - calibration;
* - runtime state;
* - filesystem state;
* - network state;
* - environment variables;
* - wall-clock time;
* - randomness;
* - backend selection.
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* This grammar contains:
* 
* - no target-language actions;
* - no embedded Rust;
* - no semantic predicates;
* - no filesystem operations;
* - no network operations;
* - no hardware discovery;
* - no runtime execution;
* - no randomness.
* 
* The Rust implementation consuming this grammar is required to remain:
* 
* Rust 2021
* Rust 1.97 / Rust 1.97.1
* safe Rust
* no unsafe
* 
* ============================================================================
* INTEGRATION CONTRACT
* ============================================================================
* 
* The following files remain independent owners:
* 
* operations.g4
*     QuantumOperations
* 
* measurement.g4
*     QuantumMeasurement
* 
* reset.g4
*     QuantumReset
* 
* types.g4
*     QuantumTypes
* 
* quantum-states.g4
*     QuantumStates
* 
* quantum-capabilities.g4
*     QuantumCapabilities
* 
* quantum-dialects.g4
*     QuantumDialects
* 
* error-correction.g4
*     QuantumErrorCorrection
* 
* dynamic-control.g4
*     QuantumDynamicControl
* 
* quantum-classical.g4
*     QuantumClassical
* 
* classical-feedforward.g4
*     QuantumClassicalFeedForward
* 
* observables.g4
*     QuantumObservables
* 
* resource-requirements.g4
*     QuantumResourceRequirements
* 
* logical-operations.g4
*     LogicalOperations
* 
* circuits.g4
*     QuantumCircuits
* 
* registers.g4
*     QuantumRegisters
* 
* qubits.g4
*     QuantumQubits
* 
* parameters.g4
*     QuantumParameters
* 
* controls.g4
*     QuantumControls
* 
* adjoints.g4
*     QuantumAdjoints
* 
* ============================================================================
* IMPORTANT REPOSITORY INTEGRATION NOTE
* ============================================================================
* 
* The repository currently contains both:
* 
* grammar/quantum/types.g4
* 
* and:
* 
* grammar/quantum/quantum-types.g4
* 
* and similarly contains several historical/parallel quantum grammar
* surfaces.
* 
* This file does NOT silently choose two authorities.
* 
* The production build MUST designate exactly one owner for each production.
* 
* The canonical quantum type owner is:
* 
* QuantumTypes
* 
* The canonical state owner is:
* 
* QuantumStates
* 
* The legacy/duplicate grammar files must either:
* 
* - be converted into compatibility/reference material; or
* - be removed once dependency analysis proves they are unused.
* 
* They MUST NOT both contribute the same production to this grammar.
* 
* ============================================================================
* LEXICAL AUTHORITY INTEGRATION
* ============================================================================
* 
* There is also an existing repository inconsistency in which some quantum
* leaf grammars use:
* 
* tokenVocab = ZamaniLexer;
* 
* while other historical quantum files use:
* 
* tokenVocab = ZamaniTokens;
* 
* The production composition root MUST use the canonical lexer vocabulary:
* 
* ZamaniLexer
* 
* "ZamaniTokens" MUST NOT become a second independent lexical authority.
* 
* The lexer-token migration must therefore be completed before the complete
* ANTLR generation pipeline is considered production-conformant.
* 
* This file intentionally does not define token aliases to conceal that
* mismatch. Such aliases would create another compatibility vocabulary and
* make the language harder to maintain.
* 
* ============================================================================
* PARSER DECLARATION
* ============================================================================
  */

parser grammar Quantum;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* QUANTUM LEAF COMPOSITION
* ============================================================================
* 
* These are the quantum parser grammars that have an explicit parser-grammar
* identity in the repository and are intended to be composed into Quantum.
* 
* A leaf grammar is imported only when:
* 
* 1. its grammar name is canonical;
* 2. its token vocabulary is canonical;
* 3. its dependencies are resolvable;
* 4. its public rule names are unique;
* 5. it does not duplicate another quantum owner.
* 
* The imports below deliberately use grammar names rather than filesystem
* paths.
* 
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
QuantumClassicalFeedForward,
QuantumObservables,
QuantumResourceRequirements,
LogicalOperations,
QuantumCircuits,
QuantumRegisters,
QuantumQubits,
QuantumParameters,
QuantumControls,
QuantumAdjoints
;

/*

* ============================================================================
* 1. QUANTUM DECLARATION
* ============================================================================
* 
* Public declaration boundary used by the universal Zamani parser.
* 
* A quantum declaration introduces a quantum computation scope or another
* quantum-domain declaration supplied by the canonical declaration contract.
* 
* This rule does not encode a qubit count.
  */

quantumDeclaration
: K_QUANTUM quantumDeclarationBody
;

/*

* ============================================================================
* 2. QUANTUM DECLARATION BODY
* ============================================================================
* 
* The declaration body may be:
* 
* - a quantum block;
* - a circuit declaration;
* - a quantum-domain declaration supplied by a future compatible
*   extension.
* 
* Detailed circuit syntax remains owned by QuantumCircuits.
  */

quantumDeclarationBody
: quantumBlock
| quantumCircuitEntry
| quantumDeclarationExtension
;

/*

* ============================================================================
* 3. QUANTUM BLOCK
* ============================================================================
* 
* Quantum blocks are ordinary lexical scopes.
* 
* Ownership, borrowing, linearity, resource accounting, capability checking,
* lifetime and physical realization are semantic concerns.
  */

quantumBlock
: LBRACE quantumBlockElement* RBRACE
;

/*

* ============================================================================
* 4. QUANTUM BLOCK ELEMENT
* ============================================================================
* 
* An element may carry the universal attribute attachment boundary.
* 
* Attribute syntax itself remains owned outside this grammar.
* 
* This wrapper intentionally does not redefine attributes.
  */

quantumBlockElement
: quantumBlockAttributePrefix? quantumElement
;

/*

* ============================================================================
* 5. ATTRIBUTE INTEGRATION
* ============================================================================
* 
* The exact universal attribute production is supplied by the canonical
* parser composition grammar.
* 
* This integration point is intentionally kept as a separate wrapper so that
* quantum grammar does not create another attribute syntax.
* 
* "quantumAttributeAttachment" is therefore a composition alias whose final
* implementation must delegate to the canonical attribute owner.
  */

quantumBlockAttributePrefix
: quantumAttributeAttachment
;

quantumAttributeAttachment
: attribute
;

/*

* ============================================================================
* 6. QUANTUM ELEMENT
* ============================================================================
* 
* This is the central quantum-domain dispatch boundary.
* 
* Alternatives represent semantic families, not hardware implementations.
  */

quantumElement
: quantumOperationElement
| quantumMeasurementElement
| quantumResetElement
| quantumRegisterElement
| quantumQubitElement
| quantumStateElement
| quantumTypeElement
| quantumCircuitElement
| quantumLogicalOperationElement
| quantumControlElement
| quantumAdjointElement
| quantumClassicalElement
| quantumFeedForwardElement
| quantumDynamicElement
| quantumObservableElement
| quantumErrorCorrectionElement
| quantumCapabilityElement
| quantumResourceElement
| quantumDialectElement
| quantumParameterElement
| quantumExtensionElement
;

/*

* ============================================================================
* 7. OPERATION INTEGRATION
* ============================================================================
* 
* Operation identity is open-ended.
* 
* The operation grammar owns:
* 
* quantumOperationStatement
* 
* It accepts semantic operation names rather than a fixed gate enumeration.
  */

quantumOperationElement
: quantumOperationStatement
;

/*

* ============================================================================
* 8. MEASUREMENT INTEGRATION
* ============================================================================
  */

quantumMeasurementElement
: quantumMeasurementStatement
;

/*

* ============================================================================
* 9. RESET INTEGRATION
* ============================================================================
  */

quantumResetElement
: quantumResetStatement
;

/*

* ============================================================================
* 10. REGISTER INTEGRATION
* ============================================================================
  */

quantumRegisterElement
: quantumRegisterDeclaration
| quantumRegisterReference
| quantumRegisterTarget
;

/*

* ============================================================================
* 11. QUBIT INTEGRATION
* ============================================================================
  */

quantumQubitElement
: quantumQubitDeclaration
| quantumQubitReference
;

/*

* ============================================================================
* 12. STATE INTEGRATION
* ============================================================================
* 
* State construction, state literals, superpositions, mixtures, tensor
* products and transformations remain owned by QuantumStates.
  */

quantumStateElement
: quantumStateDeclaration
| quantumStateExpression
;

/*

* ============================================================================
* 13. TYPE INTEGRATION
* ============================================================================
* 
* QuantumTypes is the canonical owner of quantum type syntax.
* 
* No physical capacity is encoded here.
  */

quantumTypeElement
: quantumType
;

/*

* ============================================================================
* 14. CIRCUIT INTEGRATION
* ============================================================================
  */

quantumCircuitElement
: quantumCircuitDeclaration
| quantumCircuitReference
;

/*

* ============================================================================
* 15. LOGICAL OPERATION INTEGRATION
* ============================================================================
* 
* Logical operations are ordinary quantum operations plus an explicit logical
* intent marker.
* 
* They are not a second gate catalogue.
  */

quantumLogicalOperationElement
: logicalOperationStatement
;

/*

* ============================================================================
* 16. CONTROL INTEGRATION
* ============================================================================
* 
* Control syntax describes operation transformation intent.
* 
* The grammar does not determine whether a target operation supports control.
  */

quantumControlElement
: quantumControlModifier
;

/*

* ============================================================================
* 17. ADJOINT INTEGRATION
* ============================================================================
  */

quantumAdjointElement
: quantumAdjointModifier
;

/*

* ============================================================================
* 18. QUANTUM / CLASSICAL INTEGRATION
* ============================================================================
* 
* This boundary allows one Zamani program to combine:
* 
* classical values
* quantum values
* quantum operations
* measurement results
* classical decisions
* 
* without creating a second programming language.
  */

quantumClassicalElement
: quantumClassicalBoundary
;

/*

* ============================================================================
* 19. CLASSICAL FEED-FORWARD
* ============================================================================
* 
* A measurement-derived classical value may influence later quantum
* computation.
* 
* The condition itself remains an ordinary Zamani expression.
  */

quantumFeedForwardElement
: quantumClassicalFeedForwardStatement
;

/*

* ============================================================================
* 20. DYNAMIC QUANTUM CONTROL
* ============================================================================
  */

quantumDynamicElement
: quantumDynamicControlStatement
;

/*

* ============================================================================
* 21. OBSERVABLES
* ============================================================================
  */

quantumObservableElement
: quantumObservableDeclaration
| quantumObservableExpression
;

/*

* ============================================================================
* 22. ERROR-CORRECTION INTENT
* ============================================================================
* 
* QEC grammar expresses source-level intent.
* 
* QEC algorithms, code construction, syndrome extraction, decoding,
* physical implementation and fault-tolerance realization remain downstream.
  */

quantumErrorCorrectionElement
: quantumErrorCorrectionDeclaration
| quantumErrorCorrectionStatement
;

/*

* ============================================================================
* 23. CAPABILITIES
* ============================================================================
* 
* Capability names remain open-ended.
* 
* The grammar does not enumerate today's backend capabilities.
  */

quantumCapabilityElement
: quantumCapabilityDeclaration
| quantumCapabilityRequirement
;

/*

* ============================================================================
* 24. RESOURCE REQUIREMENTS
* ============================================================================
* 
* Resource syntax expresses requirements and constraints, not physical
* allocation.
  */

quantumResourceElement
: quantumResourceRequirementBlock
| quantumResourceRequirement
;

/*

* ============================================================================
* 25. DIALECT INTEGRATION
* ============================================================================
* 
* Quantum dialects are explicit extensions of Zamani.
* 
* A dialect MUST declare its:
* 
* name
* version
* syntax extensions
* semantic mapping
* compatibility
* AST mapping
* IR mapping
* 
* Dialects must not silently modify core semantics.
  */

quantumDialectElement
: quantumDialectDeclaration
| quantumDialectExtension
;

/*

* ============================================================================
* 26. PARAMETER INTEGRATION
* ============================================================================
* 
* Parameters are source-level symbolic values.
* 
* They are not bounded by a fixed number of parameters or a fixed numeric
* representation in this grammar.
  */

quantumParameterElement
: quantumParameterDeclaration
| quantumParameterBinding
| quantumParameterSweep
;

/*

* ============================================================================
* 27. QUANTUM EXTENSION POINT
* ============================================================================
* 
* This is the controlled extension point for future quantum constructs.
* 
* It MUST NOT become an unrestricted "anything goes" parser escape hatch.
* 
* An extension must be backed by an explicitly registered grammar/dialect
* contract.
  */

quantumExtensionElement
: quantumExtensionDeclaration
;

quantumDeclarationExtension
: quantumExtensionDeclaration
;

/*

* ============================================================================
* 28. QUANTUM STATEMENT
* ============================================================================
* 
* Consumers that already have a quantum scope may use this public entry point.
* 
* The complete-program grammar remains elsewhere.
  */

quantumStatement
: quantumElement
;

/*

* ============================================================================
* 29. QUANTUM TYPE
* ============================================================================
* 
* Public quantum type integration point.
* 
* QuantumTypes remains the single owner of detailed type syntax.
  */

quantumType
: quantumTypeExpression
;

/*

* ============================================================================
* 30. QUANTUM EXPRESSION
* ============================================================================
* 
* Quantum expressions are integrated with the universal expression system.
* 
* This wrapper does not create a second expression grammar.
* 
* Quantum-specific expression constructs must ultimately map into the
* domain-neutral expression/AST model.
  */

quantumExpression
: quantumStateExpression
| quantumObservableExpression
| quantumOperationExpressionReference
;

/*

* ============================================================================
* 31. PUBLIC CIRCUIT ENTRY
* ============================================================================
* 
* This rule exists solely to expose the circuit owner to the quantum
* composition boundary.
  */

quantumCircuitEntry
: quantumCircuitDeclaration
;

/*

* ============================================================================
* 32. PUBLIC QUANTUM EXTENSION ENTRY
* ============================================================================
* 
* Future quantum constructs must enter through an explicitly owned extension
* grammar rather than by modifying this file for every new technology.
  */

quantumExtensionDeclaration
: quantumDialectExtension
;

/*

* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parser must preserve enough source structure for the frontend AST to
* represent, where applicable:
* 
* - operation designator;
* - qualified namespace/path;
* - generic arguments;
* - operation parameters;
* - operation targets;
* - control modifiers;
* - adjoint/inverse modifiers;
* - measurement destinations;
* - register identity;
* - qubit identity;
* - state expressions;
* - observable expressions;
* - classical dependencies;
* - resource requirements;
* - capability requirements;
* - QEC intent;
* - dialect/version information;
* - source spans;
* - source ordering.
* 
* This grammar does NOT define Rust AST structures.
* 
* The AST contract belongs to:
* 
* src/frontend/ast/
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Parsing establishes structural validity only.
* 
* Semantic analysis determines:
* 
* - whether names resolve;
* - whether operations exist;
* - whether operation signatures match;
* - whether targets are valid quantum operands;
* - whether parameter types are valid;
* - whether measurements are valid;
* - whether state transformations are legal;
* - whether controls are semantically valid;
* - whether adjoints exist;
* - whether resources satisfy requirements;
* - whether capabilities are available;
* - whether a dialect is compatible;
* - whether QEC intent is satisfiable;
* - whether a hybrid dependency is legal.
* 
* Syntax errors and resource/capability errors MUST remain distinguishable.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* The semantic quantum model lowers to:
* 
* quantum::ir
* 
* only.
* 
* The grammar does not select:
* 
* - physical gates;
* - physical qubits;
* - QPU topology;
* - routing;
* - scheduling;
* - calibration;
* - pulse implementation;
* - vendor instructions.
* 
* ============================================================================
* CROSS-DOMAIN INTEGRATION
* ============================================================================
* 
* Quantum syntax may interact with:
* 
* types/
* expressions/
* statements/
* functions/
* effects/
* memory/
* concurrency/
* classical/
* hybrid/
* hardware/
* resources/
* distributed/
* ai/
* data/
* networking/
* security/
* compile/
* execution/
* interoperability/
* dialects/
* 
* Integration MUST occur through the universal AST/semantic contracts.
* 
* Quantum MUST NOT import backend implementation semantics into its parser.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* The grammar imposes no fixed maximum on:
* 
* quantum blocks
* declarations
* registers
* qubits
* operations
* operation parameters
* operation targets
* controls
* measurements
* circuit depth
* circuit width
* state expressions
* nested transformations
* resource requirements
* capability requirements
* dialect declarations
* 
* Any finite implementation limit must be an implementation/resource policy,
* not a language grammar constant.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* Forbidden equivalent constructs include:
* 
* qubit0
* qubit1
* qubit2
* 
* when used as a universal language-level capacity model.
* 
* Physical identifiers may exist downstream as target realization data, but
* they must not become the portable quantum grammar's resource model.
* 
* ============================================================================
* ERROR / DIAGNOSTIC CONTRACT
* ============================================================================
* 
* The parser must reject structurally incomplete constructs such as:
* 
* apply;
* apply H;
* apply H(;
* apply H);
* apply control;
* apply control(X;
* apply adjoint;
* measure;
* 
* Semantic analysis must reject unresolved or invalid constructs such as:
* 
* apply nonexistent_operation(q);
* 
* when no declaration, import, intrinsic, dialect or capability resolves the
* operation.
* 
* Resource failures must NOT be reported as syntax errors.
* 
* Capability failures must NOT be represented as parser failures.
* 
* ============================================================================
* CONFORMANCE REQUIREMENTS
* ============================================================================
* 
* This composition root is complete only when all imported grammars satisfy:
* 
* [x] canonical parser grammar identity
* [x] canonical lexer vocabulary
* [x] unique rule ownership
* [x] no duplicate semantic grammar
* [x] domain-neutral AST mapping
* [x] semantic mapping
* [x] quantum::ir mapping
* [x] source-span preservation
* [x] deterministic parsing
* [x] positive tests
* [x] negative tests
* [x] boundary tests
* [x] scalability tests
* [x] compatibility tests
* [x] hard-coding audit
* 
* ============================================================================
* TEST MATRIX
* ============================================================================
* 
* The quantum test suite must include at minimum:
* 
* quantum-empty
* quantum-single-qubit
* quantum-register
* quantum-symbolic-register
* quantum-large-symbolic-register
* quantum-custom-operation
* quantum-qualified-operation
* quantum-parameterized-operation
* quantum-controlled-operation
* quantum-adjoint-operation
* quantum-inverse-operation
* quantum-nested-modifier
* quantum-measurement
* quantum-mid-circuit-measurement
* quantum-reset
* quantum-classical-feed-forward
* quantum-dynamic-control
* quantum-observable
* quantum-channel
* quantum-noise-intent
* quantum-qec-intent
* quantum-logical-operation
* quantum-resource-requirement
* quantum-capability-requirement
* quantum-dialect
* quantum-hybrid
* quantum-negative
* quantum-boundary
* quantum-scalability
* quantum-determinism
* 
* No test may establish an artificial universal resource ceiling.
* 
* ============================================================================
* EXAMPLES
* ============================================================================
* 
* The following are intentionally valid structural examples:
* 
* quantum {
*     qubit q;
*     apply H(q);
*     measure q -> result;
* }
* 
* quantum {
*     apply custom_gate(q);
* }
* 
* quantum {
*     apply library::operation(theta)(q0, q1);
* }
* 
* quantum {
*     apply control(X)(control, target);
* }
* 
* quantum {
*     apply adjoint(operation(theta))(q);
* }
* 
* quantum {
*     when result == 1 {
*         apply correction(q);
*     }
* }
* 
* quantum {
*     requires capability("quantum.mid_circuit_measurement");
*     requires qubits >= n;
* }
* 
* These examples intentionally do not select:
* 
* a QPU;
* a vendor;
* a physical qubit;
* a physical topology;
* a fixed number of qubits;
* a fixed number of devices;
* a fixed register width.
* 
* ============================================================================
* FINAL INVARIANTS
* ============================================================================
* 
* 1. Zamani remains one language.
* 
* 2. Quantum is a domain of Zamani, not a separate language.
* 
* 3. This file is a composition root, not a second leaf grammar.
* 
* 4. Operation names are open-ended.
* 
* 5. No fixed gate enumeration exists here.
* 
* 6. No hardware capacity is encoded here.
* 
* 7. No physical allocation is encoded here.
* 
* 8. Requirements and capabilities are distinct from realization.
* 
* 9. The frontend AST remains domain-neutral.
* 
* 10. "quantum::ir" remains the single canonical quantum IR boundary.
* 
* 11. QEC, ZQN, routing, scheduling, calibration and HAL remain downstream.
* 
* 12. Parsing remains deterministic.
* 
* 13. The grammar contains no unsafe implementation.
* 
* 14. Rust integration remains Rust 1.97 / 1.97.1, Rust 2021, safe Rust.
* 
* 15. Future quantum technologies can be introduced through explicit
* extension/dialect contracts without rewriting a finite gate catalogue.
* 
* ============================================================================
  */