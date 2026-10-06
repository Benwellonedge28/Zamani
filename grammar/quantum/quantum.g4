/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum.g4
 *
 * Grammar:
 *     Quantum
 *
 * Status:
 *     CANONICAL QUANTUM-DOMAIN COMPOSITION / ORCHESTRATION GRAMMAR
 *
 * Purpose:
 *     This file is the single parser-level orchestration boundary for the
 *     complete quantum domain of Zamani.
 *
 * This file does NOT implement quantum leaf syntax.
 *
 * It composes independently owned quantum parser grammars and exposes stable
 * public dispatch rules to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Architecture:
 *
 *     grammar/DESIGN.md
 *
 * Quantum specification:
 *
 *     grammar/spec/quantum.md
 *
 * Syntax specification:
 *
 *     grammar/spec/syntax.md
 *
 * Complete language composition:
 *
 *     grammar/Zamani.g4
 *
 * Canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Frontend AST:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * CORE RESPONSIBILITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantum-domain composition;
 *     - quantum-domain dispatch;
 *     - quantum declaration dispatch;
 *     - quantum statement dispatch;
 *     - quantum expression dispatch;
 *     - quantum type dispatch;
 *     - quantum extension dispatch;
 *     - stable public quantum parser entry points;
 *     - composition between quantum subdomains.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - names;
 *     - expressions;
 *     - general statements;
 *     - general declarations;
 *     - quantum operation syntax;
 *     - quantum measurement syntax;
 *     - qubit syntax;
 *     - register syntax;
 *     - circuit syntax;
 *     - state syntax;
 *     - parameter syntax;
 *     - control syntax;
 *     - adjoint syntax;
 *     - QEC syntax;
 *     - noise syntax;
 *     - channel syntax;
 *     - resource syntax;
 *     - capability syntax;
 *     - dialect syntax;
 *     - learning syntax;
 *     - inference syntax;
 *     - uncertainty syntax;
 *     - provenance syntax;
 *     - explanation syntax;
 *     - adaptive execution syntax;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - calibration;
 *     - target selection;
 *     - physical allocation;
 *     - ZQN implementation;
 *     - HAL implementation;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Every production must have exactly one canonical owner.
 *
 * This file may:
 *
 *     import a canonical parser grammar;
 *     expose a wrapper around an imported public rule;
 *     compose imported public rules.
 *
 * This file MUST NOT copy a production from another grammar merely to make
 * the build succeed.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Quantum source describes computational intent.
 *
 * This file imposes no universal limits on:
 *
 *     qubits
 *     logical qubits
 *     registers
 *     register extents
 *     operations
 *     parameters
 *     controls
 *     targets
 *     measurements
 *     circuits
 *     circuit depth
 *     circuit width
 *     states
 *     channels
 *     observables
 *     QEC resources
 *     devices
 *     QPUs
 *     nodes
 *     memory
 *     timelines
 *     capabilities
 *     requirements
 *
 * No grammar-level capacity constants are permitted.
 *
 * Resource feasibility belongs downstream to:
 *
 *     semantic analysis
 *     resource analysis
 *     capability negotiation
 *     compilation
 *     routing
 *     scheduling
 *     resilience
 *     runtime
 *     deployment
 *
 * ============================================================================
 * OPEN-WORLD QUANTUM MODEL
 * ============================================================================
 *
 * This orchestrator MUST NOT enumerate individual quantum operations.
 *
 * There is deliberately no:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * catalogue here.
 *
 * Operation identity belongs to QuantumOperations and its semantic resolver.
 *
 * Consequently new operations, library operations, vendor operations,
 * parameterized operations, logical operations, future operations and dialect
 * operations do not require modification of this orchestration root.
 *
 * ============================================================================
 * CANONICAL PIPELINE
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
 *     Quantum
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +---- types
 *          +---- effects
 *          +---- capabilities
 *          +---- resources
 *          +---- contracts
 *          +---- policies
 *          +---- provenance
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar never selects:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     vendor
 *     physical qubit
 *     physical register
 *     physical topology
 *     coupling map
 *     calibration
 *     pulse schedule
 *     scheduler
 *     router
 *     memory bank
 *
 * These are downstream realization concerns.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Source-level quantum intent may express:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     budgets
 *
 * These remain declarative.
 *
 * They do not perform physical allocation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     source text
 *     canonical lexical vocabulary
 *     parser grammar
 *     selected language version
 *     explicitly selected dialect/version
 *
 * Parsing MUST NOT depend upon:
 *
 *     hardware discovery
 *     QPU availability
 *     calibration
 *     runtime state
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     backend selection
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no target-language actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no randomness.
 *
 * Rust consumers remain:
 *
 *     Rust 2021
 *     Rust 1.97+
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 */

parser grammar Quantum;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL QUANTUM PARSER IMPORTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * ANTLR parser imports are grammar-name imports.
 *
 * Every imported item below MUST itself be a parser grammar.
 *
 * Leaf files that are currently grammar fragments rather than parser grammars
 * MUST NOT be falsely imported under invented grammar names.
 *
 * Those files require an independent parser-grammar promotion step.
 *
 * ============================================================================
 *
 * Current canonical parser grammar families:
 *
 *     QuantumOperations
 *     QuantumMeasurement
 *     QuantumReset
 *     QuantumTypes
 *     QuantumStates
 *     QuantumCapabilities
 *     QuantumDialects
 *     QuantumErrorCorrection
 *     QuantumDynamicControl
 *     QuantumClassical
 *     QuantumClassicalFeedForward
 *     QuantumObservables
 *     QuantumResourceRequirements
 *     LogicalOperations
 *     QuantumParameters
 *     QuantumControls
 *     QuantumAdjoints
 *
 * Additional quantum families should become parser grammars with stable
 * ownership before being added to this import list.
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
    QuantumParameters,
    QuantumControls,
    QuantumAdjoints
;


/*
 * ============================================================================
 * PUBLIC QUANTUM DOMAIN ENTRY
 * ============================================================================
 *
 * This is the primary entry point consumed by ZamaniParser.
 *
 * The quantum token itself is owned by ZamaniLexer.
 *
 * The current canonical lexical spelling is:
 *
 *     quantum
 *
 * represented by:
 *
 *     QUANTUM
 *
 * No private token alias is introduced here.
 *
 * ============================================================================
 */

quantumDeclaration
    : QUANTUM quantumDeclarationBody
    ;


/*
 * ============================================================================
 * QUANTUM DECLARATION BODY
 * ============================================================================
 *
 * Quantum declarations may contain the canonical quantum computation body.
 *
 * Detailed constructs are delegated to their owners.
 * ============================================================================
 */

quantumDeclarationBody
    : quantumBlock
    | quantumDeclarationEntry
    ;


/*
 * ============================================================================
 * QUANTUM BLOCK
 * ============================================================================
 *
 * The block itself is structural.
 *
 * Resource ownership, lifetimes, linearity, capability resolution and
 * execution semantics belong downstream.
 * ============================================================================
 */

quantumBlock
    : LBRACE quantumBlockElement* RBRACE
    ;


/*
 * ============================================================================
 * QUANTUM BLOCK ELEMENT
 * ============================================================================
 *
 * No quantum leaf syntax is duplicated here.
 *
 * The element dispatcher delegates to the canonical quantum-domain owners.
 * ============================================================================
 */

quantumBlockElement
    : quantumDeclarationElement
    | quantumStatementElement
    | quantumExpressionElement
    ;


/*
 * ============================================================================
 * QUANTUM DECLARATION ELEMENT
 * ============================================================================
 *
 * Only declaration productions actually owned by imported parser grammars
 * are exposed here.
 * ============================================================================
 */

quantumDeclarationElement
    : quantumCapabilityDeclaration
    | quantumDialectDeclaration
    | quantumErrorCorrectionDeclaration
    ;


/*
 * ============================================================================
 * QUANTUM STATEMENT ELEMENT
 * ============================================================================
 *
 * This is the central statement-family dispatcher.
 *
 * Individual semantics remain in their leaf owners.
 * ============================================================================
 */

quantumStatementElement
    : quantumOperationStatement
    | quantumMeasurementStatement
    | quantumResetOperation
    | logicalOperationStatement
    | quantumClassicalConstruct
    | quantumClassicalFeedForwardStatement
    | dynamicQuantumControl
    | quantumObservationStatement
    | quantumErrorCorrectionStatement
    | quantumCapabilityRequirement
    | quantumResourceRequirement
    ;


/*
 * ============================================================================
 * QUANTUM EXPRESSION ELEMENT
 * ============================================================================
 */

quantumExpressionElement
    : quantumExpression
    ;


/*
 * ============================================================================
 * QUANTUM EXPRESSION
 * ============================================================================
 *
 * Quantum expressions remain open-ended and are ultimately part of the
 * domain-neutral expression model.
 *
 * This wrapper intentionally exposes only expressions that actually have
 * canonical imported owners.
 * ============================================================================
 */

quantumExpression
    : quantumStateExpression
    | quantumObservableExpression
    | quantumOperationExpressionReference
    | quantumParameterExpression
    | quantumCapabilityExpression
    ;


/*
 * ============================================================================
 * QUANTUM OPERATION
 * ============================================================================
 *
 * QuantumOperations is the sole owner of operation syntax.
 *
 * This root deliberately does not enumerate operations.
 * ============================================================================
 */

quantumOperation
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * QUANTUM MEASUREMENT
 * ============================================================================
 */

quantumMeasurement
    : quantumMeasurementStatement
    ;


/*
 * ============================================================================
 * QUANTUM RESET
 * ============================================================================
 */

quantumReset
    : quantumResetOperation
    ;


/*
 * ============================================================================
 * QUANTUM TYPES
 * ============================================================================
 */

quantumTypeEntry
    : quantumType
    ;


/*
 * ============================================================================
 * QUANTUM STATES
 * ============================================================================
 */

quantumState
    : quantumStateExpression
    ;


/*
 * ============================================================================
 * QUANTUM CAPABILITIES
 * ============================================================================
 *
 * Capability identity is open-ended.
 *
 * The grammar does not enumerate today's hardware capabilities.
 * ============================================================================
 */

quantumCapability
    : quantumCapabilityReference
    | quantumCapabilityRequirement
    | quantumCapabilityDeclaration
    ;


/*
 * ============================================================================
 * QUANTUM RESOURCES
 * ============================================================================
 *
 * Requirements are declarations of intent.
 *
 * They do not perform allocation.
 * ============================================================================
 */

quantumResource
    : quantumResourceRequirement
    | quantumResourceRequirementBlock
    ;


/*
 * ============================================================================
 * QUANTUM DIALECTS
 * ============================================================================
 */

quantumDialect
    : quantumDialectDeclaration
    | quantumDialectExtension
    ;


/*
 * ============================================================================
 * QUANTUM QEC
 * ============================================================================
 *
 * This is QEC intent syntax only.
 *
 * Encoding, syndrome extraction, decoding, recovery and physical
 * fault-tolerance are downstream concerns.
 * ============================================================================
 */

quantumQec
    : quantumErrorCorrectionDeclaration
    | quantumErrorCorrectionStatement
    ;


/*
 * ============================================================================
 * QUANTUM DYNAMIC CONTROL
 * ============================================================================
 *
 * Runtime behavior is not implemented by the grammar.
 *
 * The grammar only represents source-level control structure.
 * ============================================================================
 */

quantumDynamicExecution
    : dynamicQuantumControl
    ;


/*
 * ============================================================================
 * QUANTUM / CLASSICAL HYBRID
 * ============================================================================
 */

quantumHybrid
    : quantumClassicalConstruct
    | hybridComputationRegion
    | quantumClassicalBoundaryStatement
    | classicalControlOfQuantum
    | quantumConditionalExecution
    ;


/*
 * ============================================================================
 * QUANTUM FEED-FORWARD
 * ============================================================================
 */

quantumFeedForward
    : quantumClassicalFeedForwardStatement
    ;


/*
 * ============================================================================
 * QUANTUM OBSERVABLES
 * ============================================================================
 */

quantumObservable
    : quantumObservableExpression
    | quantumObservationStatement
    | quantumObservableDeclaration
    ;


/*
 * ============================================================================
 * QUANTUM PARAMETERS
 * ============================================================================
 */

quantumParameter
    : quantumParameterExpression
    | quantumParameterBinding
    | quantumParameterSweep
    ;


/*
 * ============================================================================
 * QUANTUM CONTROLS
 * ============================================================================
 */

quantumControl
    : quantumControlModifier
    | quantumControlSpecification
    ;


/*
 * ============================================================================
 * QUANTUM ADJOINTS
 * ============================================================================
 */

quantumAdjoint
    : quantumAdjointModifier
    | quantumAdjointOperationReference
    ;


/*
 * ============================================================================
 * QUANTUM LOGICAL OPERATIONS
 * ============================================================================
 */

quantumLogicalOperation
    : logicalOperation
    | logicalOperationStatement
    ;


/*
 * ============================================================================
 * QUANTUM DECLARATION ENTRY
 * ============================================================================
 *
 * This rule is intentionally extensible through canonical owners.
 *
 * It does NOT provide an unrestricted arbitrary-token escape hatch.
 * ============================================================================
 */

quantumDeclarationEntry
    : quantumCapabilityDeclaration
    | quantumDialectDeclaration
    | quantumErrorCorrectionDeclaration
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM STATEMENT
 * ============================================================================
 *
 * Useful for tools that already know they are inside a quantum context.
 * ============================================================================
 */

quantumStatement
    : quantumStatementElement
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM EXPRESSION
 * ============================================================================
 */

quantumExpressionEntry
    : quantumExpression
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM TYPE
 * ============================================================================
 */

quantumTypeExpression
    : quantumType
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM RESOURCE CONTRACT
 * ============================================================================
 */

quantumResourceContract
    : quantumResource
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM CAPABILITY CONTRACT
 * ============================================================================
 */

quantumCapabilityContract
    : quantumCapability
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM DIALECT CONTRACT
 * ============================================================================
 */

quantumDialectContract
    : quantumDialect
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM QEC CONTRACT
 * ============================================================================
 */

quantumErrorCorrectionContract
    : quantumQec
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM HYBRID CONTRACT
 * ============================================================================
 */

quantumHybridContract
    : quantumHybrid
    ;


/*
 * ============================================================================
 * PUBLIC QUANTUM DYNAMIC EXECUTION CONTRACT
 * ============================================================================
 */

quantumDynamicExecutionContract
    : quantumDynamicExecution
    ;


/*
 * ============================================================================
 * DOMAIN-NEUTRAL AST CONTRACT
 * ============================================================================
 *
 * This grammar preserves enough syntax for the frontend AST to represent:
 *
 *     operation identity
 *     operation namespace
 *     operation parameters
 *     operation targets
 *     controls
 *     adjoints
 *     measurement destinations
 *     state expressions
 *     observable expressions
 *     resource requirements
 *     capability requirements
 *     QEC intent
 *     dialect identity
 *     hybrid dependencies
 *     dynamic control
 *     source ordering
 *     source locations
 *
 * The AST is owned outside this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural validity only.
 *
 * Semantic analysis determines:
 *
 *     name resolution
 *     type correctness
 *     operation validity
 *     operand validity
 *     parameter validity
 *     measurement validity
 *     state validity
 *     control validity
 *     adjoint validity
 *     capability satisfaction
 *     resource satisfaction
 *     dialect compatibility
 *     QEC feasibility
 *     hybrid legality
 *     effect legality
 *     contract validity
 *     policy validity
 *
 * A semantic/resource/capability failure MUST NOT be disguised as a syntax
 * failure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Quantum constructs may participate in the universal effect system.
 *
 * Examples include:
 *
 *     measurement
 *     randomness
 *     simulation
 *     learning
 *     adaptation
 *     foreign interaction
 *     distributed execution
 *
 * Effects are not implemented here.
 *
 * They are attached by semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Quantum capability requirements are open-ended.
 *
 * Examples include:
 *
 *     quantum::measurement
 *     quantum::mid_circuit_measurement
 *     quantum::dynamic_control
 *     quantum::fault_tolerant_execution
 *
 * No closed hardware capability catalogue is encoded here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Quantum source may require symbolic resources:
 *
 *     qubits >= required_qubits
 *     memory >= required_memory
 *     capability("quantum.measurement")
 *     topology(required_topology)
 *
 * This grammar does not resolve those expressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Quantum execution may be constrained by universal policies governing:
 *
 *     resource use
 *     execution
 *     security
 *     adaptation
 *     simulation
 *     deployment
 *     resilience
 *
 * Policy ownership remains outside this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Quantum constructs must preserve source provenance sufficiently for:
 *
 *     compilation tracing
 *     scientific reproducibility
 *     transformation auditing
 *     explanation
 *     debugging
 *     verification
 *
 * Provenance generation is downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * All semantically valid quantum computation eventually crosses:
 *
 *     quantum::ir
 *
 * There is no second quantum frontend IR introduced by this grammar.
 *
 * This file does not create:
 *
 *     QuantumIR
 *     QuantumGateIR
 *     QuantumCircuitIR
 *     QuantumHardwareIR
 *
 * or equivalent competing representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DOWNSTREAM QUANTUM PIPELINE
 * ============================================================================
 *
 * After semantic analysis:
 *
 *     quantum::ir
 *          |
 *          +--> optimization
 *          |
 *          +--> decomposition
 *          |
 *          +--> routing
 *          |
 *          +--> scheduling
 *          |
 *          +--> resilience
 *          |
 *          +--> QEC
 *          |
 *          +--> ZQN
 *          |
 *          +--> HAL
 *          |
 *          +--> target realization
 *
 * This grammar must never move those concerns upstream into syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable public wrapper names should not be casually renamed.
 *
 * New quantum features should normally be added by:
 *
 *     1. creating or promoting a canonical leaf parser grammar;
 *     2. defining its public rule;
 *     3. adding that parser grammar to this import list;
 *     4. adding one dispatch alternative here;
 *     5. adding specification;
 *     6. adding AST/semantic/IR mapping;
 *     7. adding tests.
 *
 * A new operation must NOT require changing this file if it is already
 * representable through QuantumOperations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately uses unbounded parser repetition.
 *
 * No language-level finite limit exists for:
 *
 *     quantum declarations
 *     quantum statements
 *     operations
 *     parameters
 *     targets
 *     controls
 *     measurements
 *     circuits
 *     state terms
 *     resource requirements
 *     capability requirements
 *     dialect features
 *
 * Practical limits imposed by:
 *
 *     compiler memory
 *     parser implementation
 *     operating system
 *     runtime
 *     target hardware
 *     deployment environment
 *
 * are not language limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain universal capacity constants such as:
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
 * It also MUST NOT encode a finite gate catalogue.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural parser errors include malformed source such as:
 *
 *     quantum {
 *     quantum {
 *     quantum
 *     quantum { ...
 *
 * Semantic errors include:
 *
 *     unresolved operation
 *     invalid quantum operand
 *     invalid parameter type
 *     unavailable capability
 *     unsatisfied resource requirement
 *     incompatible dialect
 *     invalid QEC intent
 *
 * Resource and capability failures remain semantic diagnostics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required quantum conformance families:
 *
 *     quantum-minimal
 *     quantum-operation
 *     quantum-custom-operation
 *     quantum-qualified-operation
 *     quantum-parameterized-operation
 *     quantum-measurement
 *     quantum-reset
 *     quantum-state
 *     quantum-observable
 *     quantum-control
 *     quantum-adjoint
 *     quantum-logical-operation
 *     quantum-resource
 *     quantum-capability
 *     quantum-dialect
 *     quantum-dynamic-control
 *     quantum-hybrid
 *     quantum-feed-forward
 *     quantum-qec
 *     quantum-uncertainty
 *     quantum-provenance
 *     quantum-adaptive-execution
 *     quantum-learning
 *     quantum-inference
 *     quantum-channel
 *     quantum-noise
 *     quantum-barrier
 *     quantum-kernel
 *     quantum-scalability
 *     quantum-determinism
 *     quantum-negative
 *     quantum-boundary
 *
 * Every family requires:
 *
 *     positive tests
 *     negative tests
 *     boundary tests
 *     compatibility tests
 *     determinism tests
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IMPORTANT REPOSITORY INTEGRATION REQUIREMENTS
 * ============================================================================
 *
 * The following existing files currently contain quantum syntax that must
 * remain independently owned rather than being duplicated here:
 *
 *     grammar/quantum/circuits.g4
 *     grammar/quantum/qubits.g4
 *
 * These files currently behave as grammar fragments rather than canonical
 * parser grammars.
 *
 * Therefore they MUST be promoted independently to:
 *
 *     parser grammar QuantumCircuits;
 *     parser grammar QuantumQubits;
 *
 * before they can legally appear in the import list above.
 *
 * Their existing productions should remain in those files.
 *
 * Do NOT copy their productions into this file.
 *
 * The same promotion rule applies to any other .g4 file under quantum/
 * that lacks a parser-grammar declaration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM DIRECTORY ORCHESTRATION CONTRACT
 * ============================================================================
 *
 * The intended directory architecture is:
 *
 *     quantum/
 *       |
 *       +-- quantum.g4                  <-- THIS FILE
 *       |
 *       +-- operations.g4
 *       +-- measurement.g4
 *       +-- reset.g4
 *       +-- types.g4
 *       +-- states / quantum-states.g4
 *       +-- registers.g4
 *       +-- qubits.g4
 *       +-- circuits.g4
 *       +-- parameters.g4
 *       +-- controls.g4
 *       +-- adjoints.g4
 *       +-- logical-operations.g4
 *       +-- quantum-capabilities.g4
 *       +-- resource-requirements.g4
 *       +-- quantum-dialects.g4
 *       +-- dynamic-control.g4
 *       +-- quantum-classical.g4
 *       +-- classical-feedforward.g4
 *       +-- observables.g4
 *       +-- error-correction.g4
 *       |
 *       +-- adaptive-quantum-execution.g4
 *       +-- quantum-learning.g4
 *       +-- quantum-inference.g4
 *       +-- uncertainty.g4
 *       +-- provenance.g4
 *       +-- explanations.g4
 *       +-- channels.g4
 *       +-- noise.g4
 *       +-- barriers.g4
 *       +-- kernels.g4
 *       +-- dynamic-circuits.g4
 *       +-- controlled-operations.g4
 *       +-- mid-circuit-control.g4
 *       +-- quantum-classical-control.g4
 *       +-- pulse-intent.g4
 *       +-- logical-qubits.g4
 *       +-- physical-qubits.g4
 *       +-- ...
 *
 * The important rule is:
 *
 *     leaf owner
 *          |
 *          v
 *     canonical quantum parser grammar
 *          |
 *          v
 *     Quantum
 *          |
 *          v
 *     ZamaniParser
 *
 * Quantum.g4 therefore remains stable while quantum technology evolves.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * FINAL INVARIANTS
 * ============================================================================
 *
 * 1. Quantum is a domain of Zamani, not another language.
 *
 * 2. Quantum.g4 is the quantum composition root.
 *
 * 3. Leaf syntax has one owner.
 *
 * 4. This file contains no gate catalogue.
 *
 * 5. Operation identity remains open-ended.
 *
 * 6. No physical hardware capacity is encoded.
 *
 * 7. No physical allocation is encoded.
 *
 * 8. No routing is encoded.
 *
 * 9. No scheduling is encoded.
 *
 * 10. No calibration is encoded.
 *
 * 11. No backend selection is encoded.
 *
 * 12. Resource requirements are distinct from allocation.
 *
 * 13. Capabilities are distinct from target identity.
 *
 * 14. Quantum/classical hybrid computation remains one semantic system.
 *
 * 15. Dynamic execution remains source intent rather than runtime behavior.
 *
 * 16. QEC remains source intent rather than physical implementation.
 *
 * 17. Dialects are explicit and versioned.
 *
 * 18. Parsing remains deterministic.
 *
 * 19. The AST remains domain-neutral.
 *
 * 20. quantum::ir remains the canonical quantum semantic boundary.
 *
 * 21. Future quantum operations do not require modification of this file.
 *
 * 22. No artificial universal capacity limit exists.
 *
 * 23. Rust integration remains Rust 1.97+, Rust 2021 and safe Rust only.
 *
 * ============================================================================
 */