/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/measurement.g4
 *
 * Grammar:
 *     MeasurementEffects
 *
 * Status:
 *     CANONICAL / PRODUCTION MODULAR EFFECT GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, no semantic predicates,
 *     no runtime calls, no filesystem access, no network access, no hardware
 *     discovery, and no unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the canonical grammar boundary for EFFECT-LEVEL
 * MEASUREMENT REFERENCES.
 *
 * It does NOT own source-level measurement syntax.
 *
 * Source-level measurement syntax is owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * For example:
 *
 *     measure q;
 *     measure q -> result;
 *     measure q with {
 *         basis = X
 *     };
 *
 * remains owned by QuantumMeasurement.
 *
 * This file instead provides the effect-system boundary for measurement
 * semantics.
 *
 * Examples of effect identities that may be represented by the general
 * effect system include:
 *
 *     quantum::measurement
 *     quantum::measurement::readout
 *     quantum::measurement::projective
 *     quantum::measurement::observable
 *     quantum::measurement::dynamic
 *     quantum::measurement::custom::future_effect
 *
 * The grammar intentionally does NOT establish a closed list of measurement
 * effects.
 *
 * New measurement-related effects must therefore be representable without
 * changing this grammar merely because a new effect name is introduced.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * There are three different concepts:
 *
 *     1. MEASUREMENT SOURCE SYNTAX
 *
 *        Example:
 *
 *            measure q -> result;
 *
 *        Owner:
 *
 *            grammar/quantum/measurement.g4
 *
 *
 *     2. MEASUREMENT EFFECT IDENTITY
 *
 *        Example:
 *
 *            quantum::measurement
 *
 *        Owner:
 *
 *            general effect system
 *
 *        Specialized boundary:
 *
 *            THIS FILE
 *
 *
 *     3. MEASUREMENT SEMANTICS
 *
 *        Examples:
 *
 *            projective measurement
 *            generalized measurement
 *            destructive measurement
 *            non-destructive measurement
 *            observable measurement
 *            mid-circuit measurement
 *            adaptive measurement
 *            measurement with classical feed-forward
 *
 *        Owner:
 *
 *            semantic quantum/effect analysis
 *
 *            and eventually:
 *
 *            quantum::ir
 *
 * This file MUST NOT collapse those three layers into one grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - measurement-effect reference composition;
 *     - a stable measurement-effect parser boundary;
 *     - measurement-effect collection composition where explicitly consumed;
 *     - source-level classification of an effect reference as a measurement
 *       effect boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the `measure` statement;
 *     - measurement expressions;
 *     - measurement targets;
 *     - measurement destinations;
 *     - measurement options;
 *     - basis syntax;
 *     - observable declarations;
 *     - quantum operations;
 *     - qubits;
 *     - quantum registers;
 *     - quantum states;
 *     - dynamic control;
 *     - classical control flow;
 *     - effect declarations;
 *     - effect operation declarations;
 *     - effect handlers;
 *     - generic effect sets;
 *     - generic effect references;
 *     - capabilities;
 *     - resources;
 *     - requirements;
 *     - policies;
 *     - contracts;
 *     - provenance;
 *     - hardware;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - target selection;
 *     - backend selection;
 *     - runtime execution;
 *     - quantum::ir.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * The following ownership is mandatory:
 *
 *     grammar/quantum/measurement.g4
 *         -> measurement source syntax
 *
 *     grammar/effects/effect-sets.g4
 *         -> generic effect references and effect sets
 *
 *     grammar/effects/effect-operations.g4
 *         -> generic effect operation use
 *
 *     grammar/effects/effect-declarations.g4
 *         -> effect declarations
 *
 *     grammar/effects/quantum.g4
 *         -> quantum effect namespace qualification
 *
 *     THIS FILE
 *         -> measurement-effect classification boundary
 *
 * No file may redefine another file's owned syntax.
 *
 * ============================================================================
 * WHY THIS FILE IS NECESSARY
 * ============================================================================
 *
 * A measurement is simultaneously:
 *
 *     - a quantum computational operation;
 *     - a source-level semantic event;
 *     - an observable effect;
 *     - potentially a source of classical information;
 *     - potentially stochastic;
 *     - potentially destructive;
 *     - potentially involved in dynamic control;
 *     - potentially subject to capability and resource requirements.
 *
 * The quantum measurement grammar describes the operation itself.
 *
 * The effect subsystem must separately be able to identify that a computation
 * has a measurement-related effect.
 *
 * This file establishes that boundary without creating another measurement
 * language.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Measurement effects are OPEN-WORLD.
 *
 * The grammar MUST NOT enumerate:
 *
 *     measurement
 *     readout
 *     projective
 *     generalized
 *     weak
 *     continuous
 *     destructive
 *     non_destructive
 *     mid_circuit
 *     adaptive
 *     observable
 *
 * as an exhaustive grammar catalogue.
 *
 * Those names may exist as semantic effect identities, dialect-defined names,
 * implementation-defined names, or future names.
 *
 * The parser must preserve their identity without requiring a grammar change.
 *
 * This permits future measurement models, quantum architectures, simulation
 * mechanisms, and vendor-independent abstractions.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar expresses portable semantic intent.
 *
 * It MUST NOT encode physical realization.
 *
 * Therefore this file MUST NOT encode:
 *
 *     physical qubit identifiers;
 *     QPU identifiers;
 *     detector identifiers;
 *     readout-channel identifiers;
 *     ADC identifiers;
 *     pulse identifiers;
 *     backend identifiers;
 *     vendor identifiers;
 *     calibration identifiers;
 *     topology identifiers;
 *     fixed measurement widths;
 *     fixed result-buffer sizes;
 *     fixed device counts;
 *     fixed processor counts;
 *     fixed node counts.
 *
 * It MUST NOT establish language-level constants such as:
 *
 *     MAX_MEASUREMENTS
 *     MAX_QUBITS
 *     MAX_RESULTS
 *     MAX_SHOTS
 *     MAX_READOUT_CHANNELS
 *     MAX_DEVICES
 *     MAX_RESULT_WIDTH
 *     MAX_OBSERVABLE_SIZE
 *
 * or equivalent limits.
 *
 * A program may contain arbitrarily many measurement-effect references as
 * permitted by the actual compiler, runtime, and target resources.
 *
 * The grammar establishes no artificial universal ceiling.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * This grammar MUST NOT create another identifier or qualified-name system.
 *
 * Canonical naming remains owned by the existing core name grammar.
 *
 * Measurement-effect references are therefore represented using the existing
 * qualified-name structure.
 *
 * This preserves:
 *
 *     namespace portability;
 *     arbitrary qualification depth;
 *     future domain extension;
 *     vendor-independent naming;
 *     dialect extensibility.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO lexer tokens.
 *
 * It does not introduce:
 *
 *     MEASUREMENT_EFFECT
 *     MEASUREMENT_NAMESPACE
 *     READOUT_EFFECT
 *     PROJECTIVE_EFFECT
 *     DESTRUCTIVE_EFFECT
 *     BASIS_EFFECT
 *     OBSERVABLE_EFFECT
 *
 * Effect identities remain names.
 *
 * The production parser consumes the repository's canonical lexer vocabulary.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/core/names.g4
 *
 *     grammar/effects/effect-sets.g4
 *
 * The first supplies canonical naming.
 *
 * The second remains the canonical owner of generic effect references and
 * effect sets.
 *
 * IMPORTANT:
 *
 * This file does not need to import EffectSets merely to redefine
 * effectReference. Where an enclosing grammar already imports EffectSets, it
 * should consume the canonical effectReference rule directly.
 *
 * This file provides a specialized adapter boundary rather than a competing
 * generic effect grammar.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public parser rules exported by this grammar:
 *
 *     measurementEffectReference
 *     measurementEffect
 *     measurementEffectReferenceList
 *     measurementEffectGroup
 *     nonEmptyMeasurementEffectGroup
 *
 * The public entry point for one measurement effect is:
 *
 *     measurementEffect
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Potential consumers include:
 *
 *     grammar/effects/effects.g4
 *
 *     grammar/effects/effect-operations.g4
 *
 *     grammar/effects/effect-types.g4
 *
 *     grammar/effects/effect-polymorphism.g4
 *
 *     grammar/effects/effect-diagnostics.md
 *
 *     semantic effect analysis
 *
 *     quantum semantic analysis
 *
 *     effect conformance tests
 *
 * A consumer MUST NOT redefine the rules exported here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser structure only.
 *
 * The downstream frontend AST must preserve:
 *
 *     - source span;
 *     - complete qualified-name spelling;
 *     - namespace segments;
 *     - source ordering;
 *     - collection ordering where applicable.
 *
 * The grammar MUST NOT create:
 *
 *     MeasurementEffectIR
 *     QuantumMeasurementIR
 *     PhysicalMeasurementEffect
 *     QPUMeasurementEffect
 *     BackendMeasurementEffect
 *
 * or any target-specific AST structure.
 *
 * The preferred path is:
 *
 *     parser context
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic measurement-effect model
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax establishes only that a qualified effect identity was written in a
 * measurement-effect grammar context.
 *
 * Semantic analysis determines:
 *
 *     - whether the effect exists;
 *     - whether it is visible;
 *     - whether it is imported;
 *     - whether it is deprecated;
 *     - whether its version is compatible;
 *     - whether it is legal in the current context;
 *     - whether it is quantum;
 *     - whether it is measurement-related;
 *     - what operation or operations it describes;
 *     - what capabilities it requires;
 *     - what resources it may require;
 *     - what effects it composes with;
 *     - whether it introduces stochastic behavior;
 *     - whether it changes quantum state;
 *     - whether it produces classical information;
 *     - whether it introduces synchronization requirements.
 *
 * Syntax-valid input is therefore not necessarily semantically valid.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar does not own types.
 *
 * Measurement result types remain owned by the existing type system and
 * semantic quantum model.
 *
 * Examples of possible semantic result forms include:
 *
 *     bit
 *     bit collection
 *     tuple
 *     array
 *     register
 *     structured measurement result
 *     observable result
 *     probabilistic result
 *     future result representation
 *
 * The grammar MUST NOT enumerate these result types.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This file exists specifically at the effect boundary.
 *
 * A measurement operation may semantically induce effects such as:
 *
 *     quantum::measurement
 *     quantum::readout
 *     randomness::stochastic
 *     quantum::dynamic_control
 *
 * However, this grammar MUST NOT automatically infer those effects from a
 * source-level spelling.
 *
 * Effect inference is semantic.
 *
 * For example:
 *
 *     measure q;
 *
 * may induce a canonical measurement effect.
 *
 * Whether it also induces:
 *
 *     randomness
 *     mutation
 *     synchronization
 *     communication
 *
 * depends on semantic context and implementation-independent language rules.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are not defined here.
 *
 * A measurement effect may require capabilities such as:
 *
 *     quantum.measurement
 *     quantum.readout
 *     quantum.observable
 *     quantum.dynamic_control
 *
 * but those are semantic capability identities.
 *
 * The grammar must not enumerate or require them.
 *
 * Capability resolution belongs to the resource/capability analysis layer.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file does not allocate resources.
 *
 * A measurement effect may have semantic resource consequences involving:
 *
 *     quantum resources;
 *     classical result storage;
 *     communication;
 *     synchronization;
 *     simulator memory;
 *     execution time;
 *     energy;
 *     reliability;
 *     readout availability.
 *
 * These are evaluated downstream.
 *
 * The grammar MUST NOT introduce fixed resource quantities.
 *
 * ============================================================================
 * REQUIREMENT CONTRACT
 * ============================================================================
 *
 * Measurement requirements remain separate from effect identity.
 *
 * Conceptually:
 *
 *     effect:
 *         quantum::measurement
 *
 * may result in semantic requirements such as:
 *
 *     requires capability("quantum.measurement");
 *
 * or:
 *
 *     requires capability("quantum.readout");
 *
 * or other target-independent requirements.
 *
 * The grammar does not decide which requirement applies.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain measurement effects.
 *
 * Examples include policies concerning:
 *
 *     measurement authorization;
 *     data handling;
 *     result retention;
 *     reproducibility;
 *     stochastic execution;
 *     privacy;
 *     simulation;
 *     hardware selection;
 *     adaptive execution.
 *
 * Policy syntax remains owned by the policy/security subsystems.
 *
 * This grammar does not authorize a measurement.
 *
 * ============================================================================
 * CONTRACT / VALIDATION CONTRACT
 * ============================================================================
 *
 * Measurement effects may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * through the existing validation/contract architecture.
 *
 * This grammar does not redefine those constructs.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The source-level effect identity must remain traceable through:
 *
 *     source
 *       |
 *       v
 *     parser context
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic effect
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     lowering / execution
 *
 * Provenance must preserve sufficient information to explain:
 *
 *     where the measurement effect originated;
 *     which semantic rule resolved it;
 *     which capability was selected;
 *     which resource analysis was performed;
 *     which lowering decisions consumed it.
 *
 * The grammar itself only preserves source structure.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This file does NOT own quantum measurement syntax.
 *
 * The canonical source syntax remains:
 *
 *     grammar/quantum/measurement.g4
 *
 * That grammar owns:
 *
 *     quantumMeasurementStatement
 *     quantumMeasurementTargetList
 *     quantumMeasurementTarget
 *     quantumMeasurementDestination
 *     quantumMeasurementOptions
 *     quantumMeasurementOptionList
 *     quantumMeasurementOption
 *
 * This file may identify the corresponding semantic effect, but must never
 * duplicate those rules.
 *
 * The quantum semantic path remains:
 *
 *     measurement source syntax
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic quantum measurement
 *         |
 *         v
 *     quantum::ir
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * `quantum::ir` remains the canonical quantum IR boundary.
 *
 * This grammar does not create or define quantum IR.
 *
 * A measurement effect may ultimately constrain or annotate canonical quantum
 * operations in quantum::ir.
 *
 * The transformation is downstream:
 *
 *     MeasurementEffects
 *         |
 *         v
 *     AST
 *         |
 *         v
 *     semantic effect model
 *         |
 *         v
 *     quantum::ir
 *
 * Never:
 *
 *     grammar
 *         |
 *         v
 *     vendor IR
 *
 * and never:
 *
 *     grammar
 *         |
 *         v
 *     physical qubit map
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Measurement effects may eventually participate in hybrid or HDL-aware
 * execution.
 *
 * This grammar does not define:
 *
 *     signals;
 *     wires;
 *     ports;
 *     clocks;
 *     physical readout channels;
 *     ADCs;
 *     timing constraints;
 *     synthesis primitives.
 *
 * Those belong to HDL/hardware semantics.
 *
 * A measurement effect may cross the HDL boundary only after semantic
 * lowering.
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Measurement can produce classical information.
 *
 * This file does not define classical result syntax.
 *
 * The result destination remains owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * and its destination expression remains an ordinary Zamani expression.
 *
 * Semantic analysis determines:
 *
 *     quantum input
 *         |
 *         v
 *     measurement
 *         |
 *         v
 *     classical result
 *
 * This is especially important for hybrid and dynamic computation.
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Measurement results may participate in:
 *
 *     inference;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     evidence;
 *     decision making.
 *
 * None of those constructs are defined here.
 *
 * They consume the semantic result through the normal expression, data,
 * effect, provenance, and policy systems.
 *
 * This prevents measurement from becoming coupled to a particular AI model.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar must be deterministic.
 *
 * It contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no runtime calls;
 *     no randomness;
 *     no hardware inspection;
 *     no environment inspection;
 *     no target-dependent branching.
 *
 * Given the same token stream and grammar version, parsing must produce the
 * same parse-tree structure.
 *
 * Measurement randomness is a semantic/runtime concern and does not belong in
 * the parser.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses structural repetition:
 *
 *     *
 *     +
 *
 * and never finite alternatives representing hardware capacity.
 *
 * It therefore imposes no source-level ceiling on:
 *
 *     effect-path depth;
 *     number of references;
 *     number of measurement effects;
 *     number of effect groups;
 *     number of source declarations;
 *     number of measurements;
 *     number of qubits;
 *     number of devices;
 *     number of processors;
 *     number of nodes.
 *
 * Practical limits belong to:
 *
 *     lexer resources;
 *     parser resources;
 *     semantic-analysis budgets;
 *     compiler configuration;
 *     resource negotiation;
 *     target capabilities;
 *     runtime resources.
 *
 * Those limits MUST NOT become grammar semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no finite measurement-effect catalogue.
 *
 * It MUST NOT introduce:
 *
 *     MEASUREMENT_0
 *     MEASUREMENT_1
 *     QUBIT_0
 *     QUBIT_1
 *     DEVICE_0
 *     QPU_0
 *     READOUT_CHANNEL_0
 *
 * and MUST NOT introduce capacity constants.
 *
 * No vendor, backend, topology, device, or physical resource is embedded in
 * the grammar.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing general effect references remain valid.
 *
 * Examples:
 *
 *     effects {
 *         quantum::measurement
 *     }
 *
 *     effects {
 *         quantum::measurement,
 *         quantum::readout
 *     }
 *
 *     effects {
 *         quantum::measurement::future
 *     }
 *
 * No new measurement effect name requires a grammar change.
 *
 * Existing source-level measurement syntax remains owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * Therefore this file must not change the meaning of:
 *
 *     measure q;
 *
 *     measure q -> result;
 *
 *     measure q with { ... };
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Adding a new measurement effect identity does NOT require a grammar version
 * change.
 *
 * A grammar version change is required only when the structural syntax of the
 * measurement-effect boundary changes.
 *
 * Semantic additions such as:
 *
 *     new measurement models;
 *     new measurement effects;
 *     new observable semantics;
 *     new readout semantics;
 *     new target capabilities;
 *     new hardware realization;
 *
 * do not by themselves require editing this file.
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
 *     device discovery;
 *     backend communication;
 *     foreign-function invocation;
 *     generated-code execution;
 *     secret access.
 *
 * Measurement authorization is a downstream policy/security concern.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following are representative structural tests.
 *
 * They are examples, not a closed vocabulary.
 *
 * ---------------------------------------------------------------------------
 * POSITIVE
 * ---------------------------------------------------------------------------
 *
 *     quantum::measurement
 *
 *     quantum::measurement::readout
 *
 *     quantum::measurement::observable
 *
 *     quantum::measurement::dynamic
 *
 *     quantum::measurement::custom::future
 *
 *     quantum::measurement::vendor::extension
 *
 *     future::measurement::effect
 *
 * The final example demonstrates that this grammar does not silently turn
 * arbitrary future namespaces into invalid syntax. Semantic classification
 * remains authoritative.
 *
 * ---------------------------------------------------------------------------
 * COLLECTION POSITIVE
 * ---------------------------------------------------------------------------
 *
 *     {
 *         quantum::measurement
 *     }
 *
 *     {
 *         quantum::measurement,
 *         quantum::measurement::readout,
 *     }
 *
 * Empty collections are also structurally representable where this grammar's
 * collection rule is used.
 *
 * ---------------------------------------------------------------------------
 * NEGATIVE STRUCTURAL CASES
 * ---------------------------------------------------------------------------
 *
 * The following must be rejected by the qualified-name grammar:
 *
 *     quantum::
 *
 *     quantum:::measurement
 *
 *     quantum::::measurement
 *
 *     ::quantum::measurement
 *
 *     quantum::measurement::
 *
 *     quantum::measurement:::
 *
 * Exact lexical rejection behavior is governed by the canonical lexer and
 * names grammar.
 *
 * ---------------------------------------------------------------------------
 * SEMANTICALLY INVALID BUT SYNTACTICALLY VALID
 * ---------------------------------------------------------------------------
 *
 * A source such as:
 *
 *     quantum::unknown_measurement_effect
 *
 * may be syntactically valid.
 *
 * Semantic analysis may reject it when no effect definition or active dialect
 * provides the referenced identity.
 *
 * This distinction is mandatory.
 *
 * ---------------------------------------------------------------------------
 * BOUNDARY
 * ---------------------------------------------------------------------------
 *
 * Test:
 *
 *     quantum::measurement::a::b::c::d::e
 *
 * and equivalent arbitrarily deep qualified names.
 *
 * No grammar rewrite should be necessary to support additional depth.
 *
 * ---------------------------------------------------------------------------
 * SCALABILITY
 * ---------------------------------------------------------------------------
 *
 * Generate effect collections containing:
 *
 *     many measurement-effect references;
 *     deeply qualified names;
 *     mixed domain references;
 *     large source units.
 *
 * The grammar must remain unchanged as test scale increases.
 *
 * ---------------------------------------------------------------------------
 * CROSS-DOMAIN
 * ---------------------------------------------------------------------------
 *
 * Measurement effects must coexist with:
 *
 *     classical effects;
 *     AI effects;
 *     learning effects;
 *     adaptation effects;
 *     distributed effects;
 *     networking effects;
 *     security effects;
 *     foreign effects;
 *     simulation effects;
 *     HDL effects.
 *
 * Example:
 *
 *     effects {
 *         quantum::measurement,
 *         learning::update,
 *         distributed::communication,
 *         security::audit
 *     }
 *
 * The effect set remains owned by EffectSets.
 *
 * ---------------------------------------------------------------------------
 * DETERMINISM
 * ---------------------------------------------------------------------------
 *
 * Parse identical measurement-effect references repeatedly and verify:
 *
 *     equivalent parse trees;
 *     equivalent source spans;
 *     equivalent qualified-name segmentation.
 *
 * ---------------------------------------------------------------------------
 * COMPATIBILITY
 * ---------------------------------------------------------------------------
 *
 * Verify that:
 *
 *     effects { quantum::measurement }
 *
 * continues to parse through the generic effect-set grammar.
 *
 * Verify independently that:
 *
 *     measure q;
 *
 * continues to parse through QuantumMeasurement.
 *
 * The two paths must not be accidentally coupled.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The complete architecture is:
 *
 *     source
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     measure q;                  effects {
 *       |                           quantum::measurement
 *       v                         }
 *     QuantumMeasurement               |
 *       |                              v
 *       +---------------+--------------+
 *                       |
 *                       v
 *                domain-neutral AST
 *                       |
 *                       v
 *                semantic analysis
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *        types       effects     capabilities
 *                       |
 *                       v
 *                 resources/policies
 *                       |
 *                       v
 *              canonical semantic model
 *                       |
 *                       v
 *                  quantum::ir
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      optimization  routing      scheduling
 *          |            |             |
 *          +------------+-------------+
 *                       |
 *                       v
 *                    resilience
 *                       |
 *                       v
 *                       ZQN
 *                       |
 *                       v
 *                      HAL
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *         QPU        simulator      future target
 *
 * The grammar layer never selects the final target.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/effect-sets.g4
 * ============================================================================
 *
 * `EffectSets` remains the canonical generic effect-set grammar.
 *
 * It already supports qualified effect identities through:
 *
 *     effectReference
 *         : qualifiedName
 *         ;
 *
 * Therefore this file MUST NOT redefine `effectReference`.
 *
 * A normal effect set such as:
 *
 *     effects {
 *         quantum::measurement
 *     }
 *
 * continues to parse through EffectSets.
 *
 * This file exists only when a grammar consumer needs an explicit
 * measurement-effect parser boundary.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/quantum.g4
 * ============================================================================
 *
 * QuantumEffects owns quantum namespace qualification.
 *
 * It already provides a canonical boundary for:
 *
 *     quantum::measurement
 *
 * This file must not create a competing `quantum` namespace grammar.
 *
 * Where a consumer needs strict quantum namespace validation, it should use
 * QuantumEffects.
 *
 * Where a consumer needs measurement-effect classification, it may use this
 * file.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/quantum/measurement.g4
 * ============================================================================
 *
 * QuantumMeasurement owns:
 *
 *     measure
 *     targets
 *     destinations
 *     options
 *
 * This file must not import QuantumMeasurement.
 *
 * Doing so would reverse dependency direction:
 *
 *     effect grammar
 *         -> quantum source grammar
 *
 * and could create unnecessary grammar coupling.
 *
 * The semantic layer joins the two concepts after parsing.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/effects.g4
 * ============================================================================
 *
 * The aggregate effect grammar remains responsible for composing:
 *
 *     declarations
 *     sets
 *     operations
 *     effect-system components
 *
 * It should not create a second measurement-effect implementation.
 *
 * If measurement-specific dispatch is required, it should consume:
 *
 *     measurementEffect
 *
 * from this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/effect-operations.g4
 * ============================================================================
 *
 * Generic effect operation invocation remains owned by EffectOperations.
 *
 * This file must not introduce:
 *
 *     measurement(...)
 *
 *     measureEffect(...)
 *
 *     invokeMeasurement(...)
 *
 * as competing operation invocation syntax.
 *
 * A measurement effect operation, if represented as an effect operation,
 * remains structurally subject to the generic effect operation model.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/effect-declarations.g4
 * ============================================================================
 *
 * Effect declarations remain owned by EffectDeclarations.
 *
 * This file does not declare:
 *
 *     effect measurement;
 *
 * or any other effect.
 *
 * The semantic registry determines whether an effect identity has a
 * declaration.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/capabilities.g4
 * ============================================================================
 *
 * Capability syntax remains owned by the capability grammar.
 *
 * This file supplies no capability syntax.
 *
 * Semantic analysis may associate a measurement effect with capability
 * requirements.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/resources/
 * ============================================================================
 *
 * Resource syntax remains owned by:
 *
 *     grammar/resources/
 *
 * Measurement resource feasibility is evaluated after parsing.
 *
 * Examples may include:
 *
 *     quantum measurement capability;
 *     result storage;
 *     readout capability;
 *     communication;
 *     synchronization;
 *     simulator memory.
 *
 * No fixed quantity is introduced here.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/validation/
 * ============================================================================
 *
 * Validation may determine whether a measurement effect is:
 *
 *     permitted;
 *     satisfiable;
 *     compatible;
 *     supported;
 *     policy-compliant.
 *
 * Validation does not belong in this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/security/
 * ============================================================================
 *
 * Security may constrain measurement-related operations through:
 *
 *     capabilities;
 *     policies;
 *     sandbox rules;
 *     authorization;
 *     audit;
 *     provenance.
 *
 * This grammar does not enforce those controls.
 *
 * ============================================================================
 * INTEGRATION WITH AI / REASONING
 * ============================================================================
 *
 * Measurement results can feed:
 *
 *     reasoning;
 *     inference;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     evidence;
 *     explainability;
 *     decisions.
 *
 * Those systems consume semantic measurement results.
 *
 * They must not import this grammar to implement their own measurement
 * syntax.
 *
 * ============================================================================
 * INTEGRATION WITH HYBRID COMPUTATION
 * ============================================================================
 *
 * Measurement may form:
 *
 *     quantum -> classical
 *
 * data/control boundary.
 *
 * Hybrid grammars own control composition.
 *
 * This file contributes only effect identity.
 *
 * ============================================================================
 * INTEGRATION WITH HDL/HARDWARE
 * ============================================================================
 *
 * Hardware and HDL systems may eventually realize measurement effects through:
 *
 *     readout circuitry;
 *     controller logic;
 *     accelerators;
 *     simulation;
 *     other execution mechanisms.
 *
 * This grammar remains independent of those implementations.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED EXECUTION
 * ============================================================================
 *
 * Measurement may occur in distributed quantum execution or simulation.
 *
 * Distribution remains represented through distributed effects, resources,
 * topology, communication, and execution semantics.
 *
 * This grammar does not introduce:
 *
 *     node counts;
 *     cluster sizes;
 *     device identifiers;
 *     network topology.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * A measurement-effect reference should remain traceable to:
 *
 *     source span;
 *     semantic effect;
 *     capability decision;
 *     resource decision;
 *     policy decision;
 *     IR operation;
 *     execution realization.
 *
 * This is essential for reproducibility and explanation.
 *
 * ============================================================================
 * FILE-LEVEL COMPLETION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     canonical names grammar;
 *     canonical lexer vocabulary.
 *
 * EXPORTS:
 *
 *     measurementEffect;
 *     measurementEffectReference;
 *     measurementEffectReferenceList;
 *     measurementEffectGroup;
 *     nonEmptyMeasurementEffectGroup.
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     effect semantic analysis + quantum semantic analysis.
 *
 * IR_OWNER:
 *
 *     canonical semantic IR;
 *     quantum::ir for quantum realization.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/measurement/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/spec/quantum.md
 *
 * SOURCE_SYNTAX_OWNER:
 *
 *     grammar/quantum/measurement.g4
 *
 * GENERIC_EFFECT_OWNER:
 *
 *     grammar/effects/effect-sets.g4
 *
 * QUANTUM_EFFECT_NAMESPACE_OWNER:
 *
 *     grammar/effects/quantum.g4
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It compiles as an ANTLR4 parser grammar.
 *
 *     [ ] It consumes the canonical Zamani lexer vocabulary.
 *
 *     [ ] It introduces no lexer tokens.
 *
 *     [ ] It introduces no embedded Rust.
 *
 *     [ ] It introduces no unsafe code.
 *
 *     [ ] It does not duplicate QuantumMeasurement.
 *
 *     [ ] It does not duplicate EffectSets.
 *
 *     [ ] It does not duplicate QuantumEffects.
 *
 *     [ ] It does not define generic effect operations.
 *
 *     [ ] It does not define effect declarations.
 *
 *     [ ] It does not define effect handlers.
 *
 *     [ ] It does not define capabilities.
 *
 *     [ ] It does not define resources.
 *
 *     [ ] It does not define policies.
 *
 *     [ ] It does not define contracts.
 *
 *     [ ] It does not define hardware.
 *
 *     [ ] It does not define QEC.
 *
 *     [ ] It does not define ZQN.
 *
 *     [ ] It does not define scheduling.
 *
 *     [ ] It does not define routing.
 *
 *     [ ] It does not define target selection.
 *
 *     [ ] It does not define runtime behavior.
 *
 *     [ ] It contains no finite measurement-effect catalogue.
 *
 *     [ ] It contains no machine-capacity constants.
 *
 *     [ ] It supports arbitrary qualified-name depth.
 *
 *     [ ] It supports arbitrary collection size through repetition.
 *
 *     [ ] It preserves source ordering.
 *
 *     [ ] It preserves the AST/semantic/IR boundary.
 *
 *     [ ] It preserves quantum::ir as the canonical quantum IR boundary.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Rust 1.97 integration succeeds.
 *
 *     [ ] Rust 1.97.1 integration succeeds.
 *
 *     [ ] The consuming Rust implementation remains safe.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file deliberately establishes:
 *
 *     ONE MEASUREMENT-EFFECT BOUNDARY
 *                  |
 *                  v
 *          GENERIC EFFECT SYSTEM
 *                  |
 *                  v
 *        DOMAIN-NEUTRAL AST
 *                  |
 *                  v
 *         SEMANTIC ANALYSIS
 *                  |
 *          +-------+-------+
 *          |               |
 *          v               v
 *       EFFECTS       QUANTUM SEMANTICS
 *                          |
 *                          v
 *                      quantum::ir
 *                          |
 *                          v
 *                 target-independent
 *                      realization
 *
 * Measurement source syntax remains independently owned by
 * `grammar/quantum/measurement.g4`.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar MeasurementEffects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. MEASUREMENT EFFECT
 * ============================================================================
 *
 * Public entry point for a measurement-effect reference.
 *
 * The rule deliberately delegates to the canonical qualified-name structure.
 *
 * No measurement effect names are enumerated here.
 */
measurementEffect
    : measurementEffectReference
    ;


/*
 * ============================================================================
 * 2. MEASUREMENT EFFECT REFERENCE
 * ============================================================================
 *
 * A measurement-effect reference is represented as a canonical qualified
 * name.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::measurement::readout
 *     quantum::measurement::observable
 *     quantum::measurement::future
 *
 * The grammar does not determine whether the name actually denotes a
 * measurement effect.
 *
 * Semantic analysis performs that classification.
 *
 * This is intentional: effect identities remain open-world.
 */
measurementEffectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. MEASUREMENT EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Arbitrarily many measurement-effect references may be represented.
 *
 * A trailing comma is accepted consistently with the existing effect-set
 * grammar.
 */
measurementEffectReferenceList
    : measurementEffectReference
      (
          COMMA
          measurementEffectReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. MEASUREMENT EFFECT GROUP
 * ============================================================================
 *
 * Structural grouping for consumers that require a collection boundary.
 *
 * This rule does not change generic effect-set semantics.
 */
measurementEffectGroup
    : LBRACE
      measurementEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 5. NON-EMPTY MEASUREMENT EFFECT GROUP
 * ============================================================================
 *
 * Structural variant for consumers that require at least one reference.
 */
nonEmptyMeasurementEffectGroup
    : LBRACE
      measurementEffectReferenceList
      RBRACE
    ;