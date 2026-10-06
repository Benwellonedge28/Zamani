/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-classical-control.g4
 *
 * Grammar:
 *     QuantumClassicalControl
 *
 * Status:
 *     Canonical quantum/classical control composition boundary.
 *
 * Technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical COMPOSITION boundary for quantum/classical
 * control inside the quantum grammar.
 *
 * It does NOT introduce a second quantum-control language.
 *
 * It does NOT duplicate:
 *
 *     - quantum operation syntax;
 *     - measurement syntax;
 *     - general classical control-flow;
 *     - quantum/classical value conversion syntax;
 *     - dynamic-circuit syntax;
 *     - feed-forward syntax;
 *     - synchronization syntax;
 *     - quantum mid-circuit control syntax.
 *
 * Those constructs already have owners elsewhere in the repository.
 *
 * This file provides one stable public entry point through which the quantum
 * grammar can expose quantum/classical control to the canonical parser.
 *
 * ============================================================================
 * ARCHITECTURAL MODEL
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
 *          v
 *     QuantumClassicalControl
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical semantics           quantum semantics
 *          |                             |
 *          |                             v
 *          |                         quantum::ir
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 canonical semantic
 *                     representation
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      optimization    routing     scheduling
 *                                      |
 *                                      v
 *                               QEC / ZQN /
 *                               resilience
 *                                      |
 *                                      v
 *                                  HAL / target
 *
 * This grammar performs no semantic lowering.
 *
 * ============================================================================
 * SINGLE-OWNER ARCHITECTURE
 * ============================================================================
 *
 * Existing ownership is preserved.
 *
 * --------------------------------------------------------------------------
 * Quantum mid-circuit control
 * --------------------------------------------------------------------------
 *
 * Owner:
 *
 *     grammar/quantum/mid-circuit-control.g4
 *
 * It owns explicit source forms such as:
 *
 *     control (condition) quantum-operation
 *
 * This file MUST NOT reproduce those productions.
 *
 * --------------------------------------------------------------------------
 * Quantum/classical interoperability
 * --------------------------------------------------------------------------
 *
 * Owner:
 *
 *     grammar/quantum/quantum-classical.g4
 *
 * It owns:
 *
 *     - quantum invocation from classical code;
 *     - quantum-result bindings;
 *     - classical control of quantum computation;
 *     - classical-controlled quantum operations;
 *     - measurement-controlled quantum operations;
 *     - quantum parameter bindings;
 *     - hybrid values;
 *     - quantum/classical conversion;
 *     - synchronization;
 *     - feedback;
 *     - iterative hybrid computation;
 *     - measurement-driven regions;
 *     - hybrid requirements/capabilities;
 *     - hybrid effects;
 *     - related source-level boundaries.
 *
 * This file delegates to those constructs.
 *
 * --------------------------------------------------------------------------
 * Quantum-domain composition
 * --------------------------------------------------------------------------
 *
 * Owner:
 *
 *     grammar/quantum/quantum.g4
 *
 * This file must eventually be consumed by the quantum composition root.
 *
 * --------------------------------------------------------------------------
 * Quantum operations
 * --------------------------------------------------------------------------
 *
 * Owner:
 *
 *     grammar/quantum/operations.g4
 *
 * No gate or operation catalogue belongs here.
 *
 * --------------------------------------------------------------------------
 * Measurement
 * --------------------------------------------------------------------------
 *
 * Owner:
 *
 *     grammar/quantum/measurement.g4
 *
 * No measurement grammar belongs here.
 *
 * --------------------------------------------------------------------------
 * Dynamic circuits
 * --------------------------------------------------------------------------
 *
 * Owner:
 *
 *     grammar/quantum/dynamic-circuits.g4
 *
 * This file composes dynamic-control semantics only through the established
 * quantum/classical boundary.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file describes semantic relationships, not physical realization.
 *
 * Therefore it MUST NOT encode:
 *
 *     - maximum qubit count;
 *     - maximum classical value width;
 *     - maximum measurement count;
 *     - maximum control count;
 *     - maximum operation count;
 *     - maximum nesting depth;
 *     - maximum circuit depth;
 *     - maximum number of devices;
 *     - maximum number of CPUs;
 *     - maximum number of GPUs;
 *     - maximum number of FPGAs;
 *     - maximum number of nodes;
 *     - maximum memory;
 *     - maximum number of threads;
 *     - maximum register width;
 *     - maximum tensor rank;
 *     - maximum network size;
 *     - maximum feedback distance;
 *     - fixed feedback latency;
 *     - fixed device topology;
 *     - fixed QPU topology;
 *     - fixed hardware vendor;
 *     - physical qubit identifiers;
 *     - hardware addresses;
 *     - pulse timing;
 *     - calibration values.
 *
 * The same source-level control dependency may therefore be realized by:
 *
 *     - a QPU dynamic circuit;
 *     - host-side classical control;
 *     - FPGA/control logic;
 *     - an accelerator;
 *     - a simulator;
 *     - distributed execution;
 *     - deferred execution;
 *     - compiler transformation;
 *     - future execution technology.
 *
 * The grammar does not choose among these realizations.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The language has no grammar-level finite machine capacity.
 *
 * Repetition is represented structurally.
 *
 * Examples include:
 *
 *     many controls;
 *     many measurements;
 *     many classical dependencies;
 *     many quantum operations;
 *     deeply nested hybrid computation;
 *     large parameter sets;
 *     large symbolic expressions;
 *     large hybrid regions.
 *
 * Any practical limit belongs to implementation resources or an explicit
 * semantic/resource policy.
 *
 * Such limits MUST NOT become language semantics.
 *
 * ============================================================================
 * DOMAIN-NEUTRAL AST CONTRACT
 * ============================================================================
 *
 * This grammar preserves source structure required by the frontend AST.
 *
 * The AST should be able to preserve:
 *
 *     - source span;
 *     - control construct kind;
 *     - classical expression;
 *     - measurement dependency;
 *     - quantum operation dependency;
 *     - value-flow direction;
 *     - synchronization dependency;
 *     - feedback relationship;
 *     - hybrid region;
 *     - nested structure;
 *     - source ordering;
 *     - attributes/annotations where supported.
 *
 * The AST MUST remain domain-neutral.
 *
 * This grammar MUST NOT introduce:
 *
 *     PhysicalQubitId
 *     PhysicalDeviceId
 *     BackendId
 *     PulseId
 *     ScheduleSlot
 *     HardwareAddress
 *     VendorInstruction
 *
 * as frontend AST concepts.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural validity only.
 *
 * Semantic analysis determines:
 *
 *     - whether a classical value is valid for a quantum operation;
 *     - whether a condition is valid;
 *     - whether a condition is Boolean-compatible;
 *     - whether a condition is measurement-derived;
 *     - whether a measurement result is available;
 *     - whether a quantum operation accepts the supplied parameters;
 *     - whether a quantum result can be consumed classically;
 *     - whether a conversion is permitted;
 *     - whether synchronization is required;
 *     - whether the operation has quantum effects;
 *     - whether the operation has classical effects;
 *     - whether required capabilities exist;
 *     - whether required resources exist;
 *     - whether a policy permits the execution;
 *     - whether the construct can be lowered.
 *
 * Syntax failures and semantic/resource/capability failures MUST remain
 * separate diagnostic classes.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file does not define a second type system.
 *
 * Types remain owned by the universal type system and the quantum type system
 * where appropriate.
 *
 * Classical values may be used as:
 *
 *     - quantum operation parameters;
 *     - control predicates;
 *     - iteration values;
 *     - feedback values;
 *     - synchronization conditions.
 *
 * Quantum-derived values may become:
 *
 *     - measurement results;
 *     - observations;
 *     - execution results;
 *     - classical bindings;
 *     - classical control dependencies.
 *
 * Exact legality is determined by semantic/type analysis.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This file does not define effects.
 *
 * Quantum/classical control may eventually participate in effects such as:
 *
 *     quantum
 *     measurement
 *     IO
 *     mutation
 *     network
 *     distributed
 *     synchronization
 *     randomness
 *     foreign
 *
 * Effect analysis remains downstream.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements remain separate from grammar ownership.
 *
 * A program may semantically require capabilities such as:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::feed_forward
 *     quantum::hybrid_execution
 *     classical::control
 *
 * Capability resolution is downstream.
 *
 * This file does not enumerate physical providers.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain symbolic.
 *
 * Examples include:
 *
 *     requires qubits >= required_qubits;
 *     requires memory >= required_memory;
 *     requires capability("quantum.dynamic_control");
 *
 * This grammar does not resolve those requirements.
 *
 * It does not allocate:
 *
 *     qubits;
 *     processors;
 *     devices;
 *     memory;
 *     network links;
 *     accelerators.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policy is evaluated downstream.
 *
 * A policy may constrain:
 *
 *     - whether dynamic control is permitted;
 *     - whether host-side feedback is permitted;
 *     - whether network-mediated control is permitted;
 *     - whether adaptation is permitted;
 *     - whether simulation is permitted;
 *     - whether a target is acceptable.
 *
 * No policy decision occurs during parsing.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend must preserve sufficient source information for provenance.
 *
 * Relevant provenance may include:
 *
 *     source span;
 *     source construct;
 *     transformation;
 *     semantic dependency;
 *     selected lowering;
 *     capability decision;
 *     resource decision;
 *     execution realization.
 *
 * This grammar itself records no runtime provenance.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * Quantum semantics eventually cross the canonical boundary:
 *
 *     quantum::ir
 *
 * There is no second quantum IR owned by this file.
 *
 * The source relationship:
 *
 *     classical value
 *            |
 *            v
 *     quantum control
 *
 * is represented in the domain-neutral frontend and then lowered into the
 * canonical semantic representation.
 *
 * Routing, scheduling, decomposition, QEC, ZQN, resilience and HAL remain
 * downstream.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     hardware access;
 *     backend discovery;
 *     dynamic code execution;
 *     environment inspection.
 *
 * Generated Rust integration remains safe Rust.
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only upon:
 *
 *     - token stream;
 *     - selected grammar;
 *     - parser configuration;
 *     - explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend upon:
 *
 *     - hardware availability;
 *     - QPU state;
 *     - calibration;
 *     - network state;
 *     - filesystem state;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime state;
 *     - scheduler state.
 *
 * Identical source under identical grammar configuration must produce
 * equivalent parser structure.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file intentionally provides a stable composition API rather than
 * exposing every internal rule from every quantum/classical grammar.
 *
 * Consumers should depend on:
 *
 *     quantumClassicalControl
 *
 * or:
 *
 *     quantumClassicalControlConstruct
 *
 * rather than reaching through this file to individual implementation rules.
 *
 * New quantum operations, devices, targets, vendors, topologies or execution
 * mechanisms MUST NOT require changes here merely because they are new.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This file imports the canonical quantum/classical parser composition
 * grammar.
 *
 * It does not import:
 *
 *     hardware
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     resilience
 *     runtime
 *
 * Those are downstream semantic consumers.
 *
 * ============================================================================
 */

parser grammar QuantumClassicalControl;

options {
    tokenVocab = ZamaniLexer;
}

import
    QuantumClassical
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the single public entry point exported by this file.
 *
 * It intentionally delegates to existing ownership.
 *
 * ============================================================================
 */

quantumClassicalControl
    : quantumClassicalControlConstruct
    ;


/*
 * ============================================================================
 * CONTROL CONSTRUCT DISPATCH
 * ============================================================================
 *
 * These alternatives are existing quantum/classical constructs.
 *
 * No new implementation of their syntax is introduced here.
 *
 * ============================================================================
 */

quantumClassicalControlConstruct
    : classicalControlOfQuantum
    | classicalControlledQuantumOperation
    | measurementControlledQuantumOperation
    | quantumParameterBinding
    | hybridValueBinding
    | quantumResult
    | quantumClassicalConversion
    | quantumClassicalSynchronization
    | quantumClassicalFeedback
    | hybridIteration
    | measurementDrivenQuantumRegion
    | quantumConditionalExecution
    | hybridRequirement
    | hybridCapability
    | hybridResourceRequirement
    | hybridConstraint
    | hybridPreference
    | hybridHint
    | hybridEffectBoundary
    | quantumParameterUpdate
    | quantumComputationFromClassical
    | classicalComputationFromQuantum
    | hybridPipeline
    | hybridDomainValue
    | hybridExtensionStatement
    ;


/*
 * ============================================================================
 * CLASSICAL -> QUANTUM CONTROL
 * ============================================================================
 *
 * Stable named integration point.
 *
 * The actual classical-control syntax remains owned by QuantumClassical.
 *
 * ============================================================================
 */

quantumClassicalControlStatement
    : classicalControlOfQuantum
    | classicalControlledQuantumOperation
    | measurementControlledQuantumOperation
    | quantumConditionalExecution
    ;


/*
 * ============================================================================
 * MEASUREMENT -> CLASSICAL -> QUANTUM CONTROL
 * ============================================================================
 *
 * Measurement syntax remains owned by the quantum measurement grammar.
 *
 * This file only exposes the semantic composition boundary.
 *
 * ============================================================================
 */

quantumMeasurementClassicalControl
    : measurementControlledQuantumOperation
    | measurementDrivenQuantumRegion
    | quantumClassicalFeedback
    ;


/*
 * ============================================================================
 * CLASSICAL VALUE -> QUANTUM PARAMETER
 * ============================================================================
 *
 * The expression and parameter grammars remain authoritative.
 *
 * ============================================================================
 */

quantumClassicalParameterControl
    : quantumParameterBinding
    | quantumParameterUpdate
    | quantumComputationFromClassical
    ;


/*
 * ============================================================================
 * QUANTUM RESULT -> CLASSICAL COMPUTATION
 * ============================================================================
 *
 * The source-level result syntax remains owned by QuantumClassical.
 *
 * ============================================================================
 */

quantumResultClassicalControl
    : classicalComputationFromQuantum
    | quantumComputationFromClassical
    | quantumResult
    | hybridValueBinding
    ;


/*
 * ============================================================================
 * HYBRID SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization syntax is not duplicated.
 *
 * ============================================================================
 */

quantumClassicalControlSynchronization
    : quantumClassicalSynchronization
    | hybridPipeline
    ;


/*
 * ============================================================================
 * HYBRID CONTROL REGION
 * ============================================================================
 *
 * A region is structural composition.
 *
 * It does not imply:
 *
 *     - a physical device;
 *     - a process;
 *     - a thread;
 *     - a hardware controller;
 *     - a scheduling domain.
 *
 * ============================================================================
 */

quantumClassicalControlRegion
    : LBRACE
      quantumClassicalControlRegionElement*
      RBRACE
    ;


quantumClassicalControlRegionElement
    : quantumClassicalControlConstruct
    | statement
    ;


/*
 * ============================================================================
 * CONTROL DEPENDENCY SEQUENCE
 * ============================================================================
 *
 * No finite number of dependencies is encoded.
 *
 * ============================================================================
 */

quantumClassicalControlSequence
    : quantumClassicalControlConstruct*
    ;


/*
 * ============================================================================
 * NESTED CONTROL
 * ============================================================================
 *
 * Nested control is structurally allowed.
 *
 * Semantic validation determines whether a particular nesting is valid.
 *
 * No maximum nesting depth is encoded.
 *
 * ============================================================================
 */

quantumClassicalNestedControl
    : quantumClassicalControlRegion
    | quantumClassicalControlSequence
    | quantumClassicalControlStatement
    ;


/*
 * ============================================================================
 * HYBRID CONTROL DISPATCH
 * ============================================================================
 *
 * This rule is intended for hybrid/quantum composition layers that need a
 * stable name without depending upon implementation details of
 * QuantumClassical.g4.
 *
 * ============================================================================
 */

quantumClassicalControlBoundary
    : quantumClassicalControl
    ;


/*
 * ============================================================================
 * AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * The frontend AST mapper should map:
 *
 *     quantumClassicalControl
 *         ->
 *     domain-neutral control/dependency node
 *
 * The AST mapper must preserve:
 *
 *     source span;
 *     condition;
 *     value-flow direction;
 *     quantum dependency;
 *     nesting;
 *     annotations;
 *     source order.
 *
 * It must not create target-specific allocation information.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes:
 *
 *     quantumClassicalControl
 *
 * and determines:
 *
 *     condition type;
 *     measurement dependency;
 *     value availability;
 *     effect set;
 *     capability requirements;
 *     resource requirements;
 *     policy constraints;
 *     provenance;
 *     lowering feasibility.
 *
 * ============================================================================
 * LOWERING CONTRACT
 * ============================================================================
 *
 * The intended path is:
 *
 *     source
 *       |
 *       v
 *     quantumClassicalControl
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic control/data dependency
 *       |
 *       +----------------------+
 *       |                      |
 *       v                      v
 * classical representation   quantum::ir
 *       |                      |
 *       +----------+-----------+
 *                  |
 *                  v
 *             optimization
 *                  |
 *                  v
 *               lowering
 *                  |
 *                  v
 *               routing
 *                  |
 *                  v
 *              scheduling
 *                  |
 *                  v
 *              QEC / ZQN
 *                  |
 *                  v
 *              resilience
 *                  |
 *                  v
 *                  HAL
 *                  |
 *                  v
 *             target/runtime
 *
 * This file participates only in the first stage.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * PARSER DIAGNOSTICS belong to this grammar/composed grammar:
 *
 *     missing expression;
 *     malformed control construct;
 *     malformed parameter binding;
 *     malformed synchronization construct;
 *     malformed region;
 *     malformed sequence.
 *
 * SEMANTIC DIAGNOSTICS belong downstream:
 *
 *     unknown identifier;
 *     invalid type;
 *     invalid measurement dependency;
 *     invalid quantum operation;
 *     unavailable capability;
 *     insufficient resources;
 *     policy violation;
 *     unsupported lowering;
 *     target infeasibility.
 *
 * A resource or capability failure MUST NOT become a parser error.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 * It contains no:
 *
 *     fixed qubit identifier;
 *     fixed device identifier;
 *     vendor gate list;
 *     physical topology;
 *     physical allocation;
 *     timing constant;
 *     feedback-latency constant.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Adding a new:
 *
 *     quantum operation;
 *     quantum dialect;
 *     quantum provider;
 *     QPU;
 *     simulator;
 *     accelerator;
 *     hardware target;
 *     classical processor;
 *     distributed execution model;
 *     control realization
 *
 * must not require this file to be modified merely because the new target or
 * operation exists.
 *
 * New SOURCE SYNTAX requires its own grammar owner and must then be composed
 * through an explicit integration contract.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file requires tests at the repository test layer.
 *
 * --------------------------------------------------------------------------
 * Positive
 * --------------------------------------------------------------------------
 *
 * Required semantic categories include:
 *
 *     classical condition -> quantum computation
 *     measurement result -> classical condition -> quantum computation
 *     classical parameter -> quantum operation
 *     quantum result -> classical computation
 *     quantum/classical synchronization
 *     hybrid feedback
 *     hybrid iteration
 *     nested hybrid control
 *     hybrid pipeline
 *
 * --------------------------------------------------------------------------
 * Negative
 * --------------------------------------------------------------------------
 *
 * Required categories include:
 *
 *     malformed control expression
 *     malformed region
 *     malformed parameter binding
 *     malformed synchronization
 *     malformed feedback
 *
 * Semantic-negative tests must separately cover:
 *
 *     invalid control type
 *     invalid measurement dependency
 *     invalid quantum operation
 *     unavailable capability
 *     insufficient resource
 *     forbidden policy
 *
 * --------------------------------------------------------------------------
 * Boundary
 * --------------------------------------------------------------------------
 *
 * Verify:
 *
 *     quantum control vs general control-flow;
 *     measurement vs classical result;
 *     classical value vs quantum parameter;
 *     hybrid control vs quantum operation;
 *     source syntax vs physical execution;
 *     capability requirement vs physical allocation.
 *
 * --------------------------------------------------------------------------
 * Scalability
 * --------------------------------------------------------------------------
 *
 * Tests must cover structurally large:
 *
 *     control sequences;
 *     hybrid regions;
 *     nested control;
 *     symbolic parameter expressions;
 *     measurement dependencies;
 *     hybrid pipelines.
 *
 * Tests MUST NOT define an artificial machine-size ceiling.
 *
 * --------------------------------------------------------------------------
 * Determinism
 * --------------------------------------------------------------------------
 *
 * Identical token streams under identical parser configuration must produce
 * equivalent parse structure.
 *
 * --------------------------------------------------------------------------
 * Compatibility
 * --------------------------------------------------------------------------
 *
 * Existing valid quantum/classical source forms must retain their meaning.
 *
 * ============================================================================
 * REQUIRED INTEGRATION
 * ============================================================================
 *
 * This file is intentionally independent from backend implementation.
 *
 * Upstream:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/quantum/quantum-classical.g4
 *     grammar/expressions/*
 *     grammar/types/*
 *     grammar/statements/*
 *
 * Immediate grammar dependency:
 *
 *     QuantumClassical
 *
 * Downstream:
 *
 *     grammar/quantum/quantum.g4
 *     grammar/hybrid/hybrid.g4
 *     grammar/statements/quantum.g4
 *     frontend AST
 *     semantic analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract/policy analysis
 *     provenance
 *     classical representation
 *     quantum::ir
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     resilience
 *     HAL
 *
 * ============================================================================
 * REQUIRED QUANTUM COMPOSITION CHANGE
 * ============================================================================
 *
 * The quantum composition root must expose this file through one stable
 * delegation point.
 *
 * The intended ownership is:
 *
 *     quantum/quantum-classical-control.g4
 *         |
 *         v
 *     quantumClassicalControl
 *         |
 *         v
 *     quantum.g4
 *         |
 *         v
 *     canonical Zamani parser
 *
 * The quantum composition root must NOT copy the rules from this file.
 *
 * It should delegate to:
 *
 *     quantumClassicalControl
 *
 * rather than reproducing:
 *
 *     classicalControlOfQuantum
 *     classicalControlledQuantumOperation
 *     measurementControlledQuantumOperation
 *     quantumClassicalFeedback
 *     etc.
 *
 * ============================================================================
 * REQUIRED HYBRID COMPOSITION CHANGE
 * ============================================================================
 *
 * The hybrid composition root should consume:
 *
 *     quantumClassicalControl
 *
 * where a stable hybrid quantum/classical-control entry point is required.
 *
 * It must not duplicate the source syntax.
 *
 * ============================================================================
 * REQUIRED STATEMENT COMPOSITION CHANGE
 * ============================================================================
 *
 * The statement composition layer should have exactly one public quantum
 * statement dispatch boundary.
 *
 * This file must remain a leaf/composition dependency and must not become a
 * second complete-program statement grammar.
 *
 * ============================================================================
 * ANTLR COMPOSITION RULE
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It must be generated together with its imported parser grammars using the
 * repository's ANTLR build configuration.
 *
 * Generated parser code is not part of this file.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Consequently it requires:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust
 *     no unsafe Rust
 *
 * Rust-side semantic handling must occur outside this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It has one parser grammar identity.
 *
 *     [x] It uses the canonical Zamani lexer.
 *
 *     [x] It has one explicit dependency: QuantumClassical.
 *
 *     [x] It does not define a second quantum-control syntax.
 *
 *     [x] It does not enumerate quantum gates.
 *
 *     [x] It does not enumerate hardware.
 *
 *     [x] It does not allocate resources.
 *
 *     [x] It does not select targets.
 *
 *     [x] It does not define an IR.
 *
 *     [x] It preserves quantum::ir as the canonical quantum semantic boundary.
 *
 *     [x] It delegates value/control syntax to its existing owner.
 *
 *     [x] It provides stable composition names.
 *
 *     [x] It supports structurally unbounded repetition.
 *
 *     [x] It imposes no artificial machine-size ceiling.
 *
 *     [x] It requires no unsafe Rust.
 *
 *     [x] It separates parser errors from semantic/resource errors.
 *
 *     [x] It defines AST integration.
 *
 *     [x] It defines semantic integration.
 *
 *     [x] It defines IR integration.
 *
 *     [x] It defines downstream integration.
 *
 *     [x] It defines scalability tests.
 *
 *     [x] It defines determinism tests.
 *
 *     [x] It defines compatibility tests.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How is quantum/classical control exposed as a stable quantum-domain
 *      grammar boundary?"
 *
 * It does NOT answer:
 *
 *     "Which machine executes it?"
 *
 *     "Which QPU executes it?"
 *
 *     "Which physical qubit is used?"
 *
 *     "Which CPU executes the classical part?"
 *
 *     "Which GPU or accelerator is selected?"
 *
 *     "How is the dependency scheduled?"
 *
 *     "How is the operation routed?"
 *
 *     "Which QEC implementation is selected?"
 *
 *     "Which noise model is used?"
 *
 *     "Which HAL realizes the operation?"
 *
 * Those decisions remain downstream.
 *
 * Therefore this file preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * by expressing quantum/classical control as portable program meaning rather
 * than as a description of today's physical machine.
 *
 * ============================================================================
 */