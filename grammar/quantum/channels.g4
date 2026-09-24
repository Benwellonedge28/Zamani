/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/channels.g4
 *
 * Grammar:
 *     QuantumChannels
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM-CHANNEL GRAMMAR COMPONENT
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
 * This file is the SINGLE GRAMMAR OWNER for source-level quantum-channel
 * specification syntax.
 *
 * A quantum channel is treated as a semantic quantum process.
 *
 * It is NOT defined here as:
 *
 *     - a Kraus matrix;
 *     - a Choi matrix;
 *     - a superoperator;
 *     - a Pauli-transfer matrix;
 *     - a Lindblad generator;
 *     - a stochastic table;
 *     - a simulator implementation;
 *     - a hardware instruction;
 *     - a vendor noise model.
 *
 * Those are representations or implementations owned downstream by ZQN.
 *
 * The grammar describes portable SOURCE INTENT.
 *
 * Canonical pipeline:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     QuantumChannels
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical quantum::ir
 *          |
 *          v
 *     ZQN channel semantics
 *          |
 *          +--> representation selection
 *          +--> noise analysis
 *          +--> simulation
 *          +--> QEC
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * The repository already has:
 *
 *     grammar/quantum/operations.g4
 *
 * for generic quantum operation invocation.
 *
 * It also has ZQN channel infrastructure under:
 *
 *     src/quantum/zqn/channel/
 *
 * That subsystem already distinguishes channel semantics from concrete
 * representations such as:
 *
 *     Kraus
 *     Choi
 *     ProcessMatrix
 *     PauliTransfer
 *     Stochastic
 *     Lindblad
 *     Superoperator
 *     Liouville
 *     Tensor
 *     Symbolic
 *     Sampled
 *     Extension
 *
 * This grammar therefore does NOT reproduce that mathematical type hierarchy.
 *
 * Instead it supplies a stable source-language contract for describing:
 *
 *     channel identity
 *     channel parameters
 *     channel operands/resources
 *     channel properties
 *     representation intent
 *     accuracy intent
 *     physicality intent
 *     composition intent
 *     application intent
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantumChannelSpecification
 *     - quantumChannelReference
 *     - quantumChannelArgumentClause
 *     - quantumChannelArgumentList
 *     - quantumChannelArgument
 *     - quantumChannelPropertyBlock
 *     - quantumChannelPropertyList
 *     - quantumChannelProperty
 *     - quantumChannelPropertyValue
 *     - quantumChannelRepresentationSpecification
 *     - quantumChannelAccuracySpecification
 *     - quantumChannelPhysicalitySpecification
 *     - quantumChannelCompositionSpecification
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - quantum operation syntax;
 *     - quantum targets;
 *     - measurements;
 *     - resets;
 *     - observables;
 *     - dynamic control;
 *     - noise-model policy;
 *     - Kraus mathematics;
 *     - Choi mathematics;
 *     - Lindblad mathematics;
 *     - stochastic probability validation;
 *     - density matrices;
 *     - state-vector representation;
 *     - simulation;
 *     - QEC;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - hardware topology;
 *     - vendor APIs;
 *     - runtime execution;
 *     - canonical quantum::ir implementation.
 *
 * Every production has exactly one canonical owner.
 *
 * ============================================================================
 * IMPORTANT: NO NEW CHANNEL KEYWORD
 * ============================================================================
 *
 * The current canonical lexer vocabulary does not define a stable CHANNEL
 * token.
 *
 * This file therefore MUST NOT introduce:
 *
 *     CHANNEL
 *     K_CHANNEL
 *     QUANTUM_CHANNEL
 *     K_QUANTUM_CHANNEL
 *
 * merely to make this grammar appear self-contained.
 *
 * Channel identities remain ordinary qualified names.
 *
 * Examples:
 *
 *     depolarizing
 *     amplitude_damping
 *     library::channel
 *     vendor::channel
 *     custom::process
 *     future::quantum_channel
 *
 * Whether a referenced name denotes a channel is a semantic-resolution
 * question.
 *
 * This preserves the open-world operation/channel model required for
 * POCO-REAF.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * The production lexer boundary is:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file introduces NO lexer rules.
 *
 * Canonical tokens consumed here are existing language tokens such as:
 *
 *     APPLY
 *     NOISE
 *     WITH
 *     ASSIGN
 *     COMMA
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *
 * where supplied by the canonical lexer.
 *
 * Channel names and representation names are NOT lexer keywords.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names + Expressions + Types
 *          |
 *          v
 *     QuantumOperations
 *          |
 *          v
 *     QuantumChannels
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     ZQN
 *
 * This grammar MUST NOT import:
 *
 *     ZQN implementation
 *     simulation
 *     QEC implementation
 *     routing
 *     scheduling
 *     hardware
 *     HAL
 *     runtime
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Channel syntax MUST remain target-independent.
 *
 * It MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CHANNELS
 *     MAX_CHANNEL_ARITY
 *     MAX_KRAUS_OPERATORS
 *     MAX_MATRIX_SIZE
 *     MAX_CHANNEL_DIMENSION
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_SIZE
 *     MAX_OPERATION_COUNT
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *     MAX_QPUS
 *
 * It MUST NOT encode universal physical identities such as:
 *
 *     physical_qubit_0
 *     qpu0
 *     device0
 *     readout_channel0
 *
 * It MUST NOT encode vendor-specific implementations.
 *
 * A channel may therefore scale from:
 *
 *     one resource
 *
 * to:
 *
 *     many resources
 *
 * to:
 *
 *     symbolic resource collections
 *
 * to:
 *
 *     distributed quantum systems
 *
 * subject only to semantic validity and actual compiler/runtime/target
 * resources.
 *
 * "Unbounded" means no artificial language ceiling.
 *
 * It does NOT mean that physical execution has infinite resources.
 *
 * ============================================================================
 * CHANNEL SEMANTIC MODEL
 * ============================================================================
 *
 * A channel is a semantic process:
 *
 *     input quantum state
 *             |
 *             v
 *       quantum channel
 *             |
 *             v
 *     output quantum state
 *
 * The channel may be:
 *
 *     unit-preserving;
 *     non-unitary;
 *     trace-preserving;
 *     trace-nonincreasing;
 *     deterministic;
 *     stochastic;
 *     continuous-time;
 *     symbolic;
 *     sampled;
 *     approximate;
 *     representation-independent.
 *
 * The grammar does not decide which mathematical properties hold.
 *
 * Semantic validation does.
 *
 * ============================================================================
 * OPEN-WORLD CHANNEL IDENTITIES
 * ============================================================================
 *
 * Channel names remain open-ended.
 *
 * Examples:
 *
 *     depolarizing
 *     amplitude_damping
 *     phase_damping
 *     custom_channel
 *     vendor::channel
 *     future::channel
 *
 * These names are semantic references.
 *
 * The grammar does not enumerate:
 *
 *     DepolarizingChannel
 *     AmplitudeDampingChannel
 *     BitFlipChannel
 *     PhaseFlipChannel
 *     PauliChannel
 *     ThermalChannel
 *
 * as closed grammar alternatives.
 *
 * A new channel therefore does not require a lexer change merely because
 * a new physical or mathematical channel is introduced.
 *
 * ============================================================================
 * CHANNEL REFERENCE
 * ============================================================================
 *
 * A channel reference is an ordinary qualified semantic name.
 *
 * Examples:
 *
 *     channel
 *     library::channel
 *     vendor::channel
 *     future::quantum::channel
 *
 * Semantic analysis resolves whether the referenced entity is:
 *
 *     - a channel;
 *     - a channel family;
 *     - a user-defined channel;
 *     - a dialect-provided channel;
 *     - a library channel;
 *     - a provider-specific channel;
 *     - an unresolved reference.
 *
 * ============================================================================
 * CHANNEL PARAMETERS
 * ============================================================================
 *
 * Channel parameters are expressions.
 *
 * This permits:
 *
 *     symbolic parameters;
 *     constants;
 *     runtime values where legal;
 *     typed parameters;
 *     expressions;
 *     data-derived parameters;
 *     future parameter kinds.
 *
 * No fixed parameter count is imposed.
 *
 * ============================================================================
 * CHANNEL PROPERTIES
 * ============================================================================
 *
 * Channel properties use open key/value syntax.
 *
 * Examples:
 *
 *     representation = kraus
 *     accuracy = exact
 *     tolerance = epsilon
 *     physicality = cptp
 *     mode = symbolic
 *     provenance = characterization
 *
 * Property names are identifiers.
 *
 * The grammar intentionally does NOT enumerate the complete property
 * vocabulary.
 *
 * Semantic registries own the authoritative meaning of property names.
 *
 * This allows future ZQN/channel properties without continually expanding the
 * lexical keyword inventory.
 *
 * ============================================================================
 * UNKNOWN PROPERTIES
 * ============================================================================
 *
 * Unknown properties remain syntactically representable.
 *
 * Example:
 *
 *     future_property = value
 *
 * Semantic analysis decides whether the property is:
 *
 *     stable;
 *     experimental;
 *     dialect-defined;
 *     deprecated;
 *     unsupported;
 *     capability-dependent;
 *     version-dependent.
 *
 * The parser must preserve the property rather than silently discarding it.
 *
 * ============================================================================
 * DUPLICATE PROPERTIES
 * ============================================================================
 *
 * Duplicate properties remain syntactically representable:
 *
 *     representation = kraus,
 *     representation = choi
 *
 * The parser preserves source order.
 *
 * Semantic analysis determines whether:
 *
 *     - duplicates are forbidden;
 *     - duplicate values are equivalent;
 *     - later values override earlier values;
 *     - multiple values form a collection;
 *     - a dialect changes the rule.
 *
 * The grammar must not silently select one value.
 *
 * ============================================================================
 * REPRESENTATION INDEPENDENCE
 * ============================================================================
 *
 * Channel representation is semantic metadata.
 *
 * The grammar may preserve an explicit representation request:
 *
 *     representation = kraus
 *     representation = choi
 *     representation = lindblad
 *     representation = symbolic
 *     representation = custom::representation
 *
 * But the grammar does not construct the representation.
 *
 * ZQN owns representation validation and conversion.
 *
 * The source program therefore remains independent from a particular matrix
 * layout or numerical storage strategy.
 *
 * ============================================================================
 * ACCURACY
 * ============================================================================
 *
 * Accuracy is expressed semantically.
 *
 * Examples:
 *
 *     accuracy = exact
 *     accuracy = approximate
 *     accuracy = bounded
 *     accuracy = statistical
 *     accuracy = unknown
 *
 * A tolerance may be supplied as an expression:
 *
 *     tolerance = epsilon
 *     tolerance = 1e-8
 *     tolerance = requested_precision
 *
 * No universal tolerance is embedded in the grammar.
 *
 * ============================================================================
 * PHYSICALITY
 * ============================================================================
 *
 * Physical validity is not established by parsing.
 *
 * A channel may carry a semantic physicality requirement:
 *
 *     physicality = cptp
 *     physicality = trace_preserving
 *     physicality = trace_non_increasing
 *     physicality = unvalidated
 *
 * These values remain semantic identifiers.
 *
 * The grammar does not implement:
 *
 *     complete positivity checks;
 *     trace-preservation checks;
 *     positivity checks;
 *     matrix validation;
 *     numerical validation.
 *
 * Those belong to ZQN/channel validation.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Channels may be composed semantically.
 *
 * Composition intent may be represented using open properties:
 *
 *     composition = sequential
 *     composition = tensor
 *     composition = parallel
 *     composition = conditional
 *     composition = custom::composition
 *
 * The grammar does not implement channel algebra.
 *
 * Mathematical composition belongs to the ZQN channel subsystem.
 *
 * ============================================================================
 * RESOURCE TARGETING
 * ============================================================================
 *
 * Channel targets are deliberately reused from the canonical quantum operation
 * grammar where channel application is integrated with operation application.
 *
 * This prevents:
 *
 *     QuantumChannelTarget
 *
 * from becoming a second quantum-resource identity system.
 *
 * The canonical operation grammar remains responsible for target structure.
 *
 * Semantic analysis determines whether a channel's input/output arity and
 * resource types are compatible with the supplied targets.
 *
 * ============================================================================
 * APPLICATION BOUNDARY
 * ============================================================================
 *
 * The current language already owns generic quantum application through:
 *
 *     grammar/quantum/operations.g4
 *
 * Therefore this file MUST NOT duplicate:
 *
 *     quantumOperationStatement
 *     quantumOperationInvocation
 *     quantumOperationTargetClause
 *
 * A channel application is semantically a quantum process application.
 *
 * Where the source explicitly uses the existing `noise` vocabulary, this file
 * provides a channel-specific source form:
 *
 *     apply noise channel(parameters)(targets);
 *
 * This does NOT create a new CHANNEL keyword.
 *
 * It also does not replace ordinary operation syntax.
 *
 * A future general channel declaration/application syntax must be introduced
 * through the language specification and then composed here, rather than
 * silently creating another competing application grammar.
 *
 * ============================================================================
 * CHANNEL SPECIFICATION FRAGMENT
 * ============================================================================
 *
 * The core reusable channel specification is intentionally independent from
 * any particular application statement.
 *
 * It can therefore be consumed by:
 *
 *     noise grammar;
 *     dynamic-control grammar;
 *     quantum/classical boundary grammar;
 *     dialects;
 *     interoperability adapters;
 *     future channel declarations.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted channel construct must preserve:
 *
 *     source span;
 *     channel reference;
 *     ordered arguments;
 *     ordered properties;
 *     property names;
 *     property values;
 *     representation intent;
 *     accuracy intent;
 *     physicality intent;
 *     composition intent;
 *     application targets where present.
 *
 * The AST remains domain-neutral.
 *
 * Preferred conceptual representation:
 *
 *     Operation / Process
 *       kind = QuantumChannel
 *       reference
 *       arguments
 *       properties
 *       targets
 *       source_span
 *
 * The parser MUST NOT create:
 *
 *     KrausIR
 *     ChoiIR
 *     LindbladIR
 *     VendorChannelIR
 *     HardwareChannelIR
 *
 * The canonical semantic path remains:
 *
 *     AST
 *       |
 *       v
 *     semantic quantum process
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     ZQN channel representation
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - channel classification;
 *     - parameter typing;
 *     - target compatibility;
 *     - channel arity;
 *     - input/output resource typing;
 *     - physicality validation;
 *     - representation validation;
 *     - accuracy validation;
 *     - composition validity;
 *     - capability requirements;
 *     - resource requirements;
 *     - effect analysis;
 *     - measurement/channel interaction;
 *     - dynamic-control interaction;
 *     - QEC interaction;
 *     - ZQN compatibility.
 *
 * The parser only establishes structure.
 *
 * ============================================================================
 * CHANNEL/NOISE SEPARATION
 * ============================================================================
 *
 * A quantum channel is broader than a noise model.
 *
 * Therefore:
 *
 *     channel semantics
 *         !=
 *     noise-model semantics
 *
 * ZQN owns the distinction.
 *
 * Examples of semantic channel uses include:
 *
 *     noise;
 *     measurement instruments;
 *     state transformations;
 *     conditional processes;
 *     open-system evolution;
 *     characterization models;
 *     sampled processes.
 *
 * The grammar does not collapse all channels into "noise".
 *
 * ============================================================================
 * QEC INTEGRATION
 * ============================================================================
 *
 * A channel may participate in error-correction analysis.
 *
 * This file does NOT own:
 *
 *     syndrome extraction;
 *     stabilizer construction;
 *     decoding;
 *     recovery;
 *     code distance;
 *     logical encoding.
 *
 * Those remain in:
 *
 *     grammar/quantum/error-correction.g4
 *     src/quantum/qec/
 *
 * Channel semantics may provide input to QEC analysis downstream.
 *
 * ============================================================================
 * ZQN INTEGRATION
 * ============================================================================
 *
 * ZQN owns the mathematical channel subsystem.
 *
 * The source grammar supplies semantic intent.
 *
 * Conceptually:
 *
 *     source channel specification
 *             |
 *             v
 *       frontend AST
 *             |
 *             v
 *       quantum::ir
 *             |
 *             v
 *            ZQN
 *             |
 *       +-----+-----------------------+
 *       |                             |
 *       v                             v
 * representation                 noise semantics
 *       |
 *       +--> Kraus
 *       +--> Choi
 *       +--> ProcessMatrix
 *       +--> PauliTransfer
 *       +--> Stochastic
 *       +--> Lindblad
 *       +--> Superoperator
 *       +--> Liouville
 *       +--> Tensor
 *       +--> Symbolic
 *       +--> Sampled
 *       +--> Extension
 *
 * The grammar MUST NOT reproduce these implementations.
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * There is no:
 *
 *     quantum_channel::ir
 *
 * and no:
 *
 *     channel::ir
 *
 * The canonical semantic quantum boundary remains:
 *
 *     quantum::ir
 *
 * Channel metadata/process intent must lower into the existing canonical
 * semantic model and then be consumed by ZQN.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compilation may:
 *
 *     - resolve channel references;
 *     - specialize symbolic parameters;
 *     - select a compatible representation;
 *     - transform equivalent representations;
 *     - propagate channels through an optimized computation;
 *     - combine compatible channels;
 *     - defer channel materialization;
 *     - preserve symbolic channels;
 *     - route channel-associated resources;
 *     - schedule channel application;
 *     - adapt to target capabilities.
 *
 * The compiler MUST NOT silently:
 *
 *     - drop a channel;
 *     - replace a channel with identity;
 *     - replace an exact channel with an approximation;
 *     - change physicality semantics;
 *     - change declared error/tolerance semantics;
 *     - select a backend merely because it is convenient.
 *
 * Any such transformation requires an explicit semantic equivalence or
 * permitted approximation contract.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime realization may depend on:
 *
 *     target capabilities;
 *     numerical resources;
 *     execution policy;
 *     calibration;
 *     noise characterization;
 *     simulator capabilities;
 *     distributed resources.
 *
 * Runtime limitations are not language limitations.
 *
 * A valid source program may therefore fail at execution because a concrete
 * target cannot satisfy its requirements.
 *
 * Such a failure must be reported as a resource/capability/feasibility
 * failure, not as invalid source syntax.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Channel capabilities are not hard-coded here.
 *
 * Semantic analysis may derive requirements such as:
 *
 *     capability("quantum.channel")
 *     capability("quantum.dynamic_channel")
 *     capability("quantum.open_system")
 *     capability("quantum.noise_model")
 *     capability("quantum.channel_representation.kraus")
 *
 * The capability registry remains outside this grammar.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis may derive:
 *
 *     channel dimension;
 *     operator count;
 *     tensor size;
 *     numerical precision;
 *     memory requirements;
 *     execution cost;
 *     communication requirements.
 *
 * These are analysis results, not grammar constants.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CHANNELS
 *     MAX_CHANNEL_ARITY
 *     MAX_KRAUS_OPERATORS
 *     MAX_DIMENSION
 *     MAX_CHANNEL_DIMENSION
 *     MAX_TENSOR_SIZE
 *     MAX_REPRESENTATION_SIZE
 *     MAX_NOISE_MODELS
 *     QUBIT_0
 *     QUBIT_1
 *     QPU_0
 *     DEVICE_0
 *
 * Also forbidden:
 *
 *     IBM_CHANNEL
 *     IONQ_CHANNEL
 *     RIGETTI_CHANNEL
 *     NVIDIA_CHANNEL
 *     AMD_CHANNEL
 *
 * as universal grammar semantics.
 *
 * Provider-specific channel names may exist as ordinary semantic identifiers
 * or dialect extensions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Structural repetition is used for:
 *
 *     channel arguments;
 *     properties;
 *     target collections;
 *
 * No finite alternative list is used to establish a language-level maximum.
 *
 * This permits:
 *
 *     one parameter;
 *     many parameters;
 *     symbolic parameters;
 *     large property collections;
 *     large resource collections;
 *     arbitrarily large source programs;
 *
 * subject to actual implementation resources.
 *
 * The grammar must remain structurally recursive only where recursion is
 * semantically necessary and must avoid unnecessary left recursion.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for equivalent source.
 *
 * The grammar:
 *
 *     contains no semantic actions;
 *     contains no runtime calls;
 *     contains no random state;
 *     contains no device discovery;
 *     contains no network access;
 *     contains no time-dependent behavior;
 *     contains no global mutable state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Channel syntax is data.
 *
 * Parsing a channel specification MUST NOT:
 *
 *     - instantiate a channel;
 *     - allocate a matrix;
 *     - allocate a tensor;
 *     - contact a QPU;
 *     - contact a vendor service;
 *     - load executable code;
 *     - invoke a simulator;
 *     - execute a channel;
 *     - evaluate arbitrary source expressions.
 *
 * Semantic and runtime layers must enforce explicit resource admission before
 * materializing potentially enormous channel representations.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics must identify:
 *
 *     missing channel reference;
 *     malformed channel argument list;
 *     malformed property;
 *     missing property value;
 *     malformed representation specification;
 *     malformed channel application;
 *     missing target clause where the application form requires one;
 *     malformed delimiter structure.
 *
 * Semantic diagnostics must separately identify:
 *
 *     unknown channel;
 *     invalid parameter;
 *     invalid target;
 *     incompatible channel arity;
 *     unsupported representation;
 *     invalid physicality;
 *     invalid accuracy contract;
 *     unsupported capability;
 *     insufficient resources.
 *
 * Syntax and resource/capability failures MUST NOT be conflated.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar is a new canonical ownership component.
 *
 * It does not rename:
 *
 *     operations.g4
 *     measurement.g4
 *     dynamic-circuits.g4
 *     dynamic-control.g4
 *     quantum-classical.g4
 *     error-correction.g4
 *
 * Existing source forms remain governed by their existing owners.
 *
 * A future dedicated `channel` keyword, if ever desired, must be introduced
 * through:
 *
 *     specification
 *       ->
 *     lexical contract
 *       ->
 *     lexer
 *       ->
 *     grammar
 *       ->
 *     AST
 *       ->
 *     semantic model
 *       ->
 *     IR
 *       ->
 *     conformance tests
 *
 * It must not be introduced ad hoc in this file.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive structural tests:
 *
 *     apply noise depolarizing()(q);
 *     apply noise amplitude_damping(gamma)(q);
 *     apply noise vendor::channel(parameter)(q);
 *
 * Channel specification fragments:
 *
 *     channel_reference
 *     channel_reference(parameter)
 *
 *     channel_reference with {
 *         representation = kraus
 *     }
 *
 *     channel_reference(parameter) with {
 *         representation = choi,
 *         accuracy = exact
 *     }
 *
 *     channel_reference(parameter) with {
 *         physicality = cptp,
 *         tolerance = epsilon
 *     }
 *
 * The exact enclosing source construct is supplied by the consuming grammar.
 *
 * Negative syntax tests:
 *
 *     apply noise;
 *     apply noise (q);
 *     apply noise channel(;
 *     apply noise channel(parameter;
 *     apply noise channel(parameter)(;
 *     channel with { = value };
 *     channel with { property = };
 *
 * Semantic-negative tests:
 *
 *     unknown channel;
 *     invalid parameter type;
 *     incompatible target;
 *     invalid channel arity;
 *     unsupported representation;
 *     impossible physicality contract.
 *
 * Boundary tests:
 *
 *     one argument;
 *     many arguments;
 *     one property;
 *     many properties;
 *     nested expressions;
 *     qualified names;
 *     deeply qualified channel names;
 *     symbolic parameters;
 *     large target lists;
 *     empty optional property block where the consuming syntax permits it.
 *
 * Scalability tests:
 *
 *     no fixed argument count;
 *     no fixed property count;
 *     no fixed target count;
 *     no fixed channel count;
 *     no fixed representation count;
 *     no fixed dimension;
 *     no fixed number of composed channels.
 *
 * Determinism tests:
 *
 *     identical source -> identical parse structure.
 *
 * Compatibility tests:
 *
 *     existing operation syntax remains owned by operations.g4;
 *     existing measurement syntax remains owned by measurement.g4;
 *     existing dynamic-control syntax remains owned by dynamic-control.g4;
 *     existing quantum/classical syntax remains owned by quantum-classical.g4.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This `.g4` file contains no Rust actions.
 *
 * Generated/native Zamani parser integration must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * The Rust implementation should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * where applicable to the relevant crate/module boundary.
 *
 * Parser generation and runtime integration must not require unsafe Rust.
 *
 * ============================================================================
 * DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] canonical path established
 * [x] canonical parser grammar
 * [x] canonical ZamaniLexer vocabulary
 * [x] no invented CHANNEL token
 * [x] open-world channel names
 * [x] reusable channel specification
 * [x] open parameter model
 * [x] open property model
 * [x] representation independence
 * [x] accuracy contract
 * [x] physicality contract
 * [x] composition contract
 * [x] POCO-REAF compliance
 * [x] no hardware limits
 * [x] no fixed channel count
 * [x] no fixed channel arity
 * [x] no second quantum IR
 * [x] AST contract
 * [x] semantic contract
 * [x] compiler contract
 * [x] runtime contract
 * [x] ZQN integration
 * [x] diagnostics contract
 * [x] security contract
 * [x] determinism contract
 * [x] scalability contract
 * [x] compatibility contract
 * [x] test contract
 * [x] Rust 1.97/1.97.1 safe-Rust contract
 *
 * The file must additionally pass repository-wide:
 *
 *     grammar validation;
 *     import-cycle validation;
 *     lexer/parser conformance;
 *     AST coverage;
 *     semantic coverage;
 *     IR coverage;
 *     scalability tests;
 *     hard-coding audit.
 *
 * ============================================================================
 */

parser grammar QuantumChannels;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions, Types, QuantumOperations;


/* ============================================================================
 * 1. REUSABLE CHANNEL SPECIFICATION
 * ========================================================================== */

/*
 * A channel specification is deliberately reusable.
 *
 * It can be consumed by noise, dynamic-circuit, dialect, interoperability,
 * or future dedicated channel syntax without duplicating the channel contract.
 */
quantumChannelSpecification
    : quantumChannelReference
      quantumChannelArgumentClause?
      quantumChannelPropertyBlock?
    ;


/* ============================================================================
 * 2. CHANNEL REFERENCE
 * ========================================================================== */

/*
 * Channel identity is an ordinary qualified name.
 *
 * Examples:
 *
 *     depolarizing
 *     library::depolarizing
 *     vendor::channel
 *     future::open_system::process
 *
 * Semantic analysis determines whether the name resolves to a channel.
 */
quantumChannelReference
    : qualifiedName
    ;


/* ============================================================================
 * 3. CHANNEL ARGUMENTS
 * ========================================================================== */

quantumChannelArgumentClause
    : LPAREN quantumChannelArgumentList? RPAREN
    ;


quantumChannelArgumentList
    : quantumChannelArgument
      (COMMA quantumChannelArgument)*
      COMMA?
    ;


quantumChannelArgument
    : expression
    ;


/* ============================================================================
 * 4. CHANNEL PROPERTY BLOCK
 * ========================================================================== */

quantumChannelPropertyBlock
    : WITH LBRACE quantumChannelPropertyList? RBRACE
    ;


quantumChannelPropertyList
    : quantumChannelProperty
      (COMMA quantumChannelProperty)*
      COMMA?
    ;


quantumChannelProperty
    : identifier ASSIGN quantumChannelPropertyValue
    ;


quantumChannelPropertyValue
    : expression
    ;


/* ============================================================================
 * 5. REPRESENTATION SPECIFICATION
 * ========================================================================== */

/*
 * This is a semantic helper rule.
 *
 * Representation names remain expressions/identifiers rather than a closed
 * parser-level enumeration.
 */
quantumChannelRepresentationSpecification
    : identifier ASSIGN expression
    ;


/* ============================================================================
 * 6. ACCURACY SPECIFICATION
 * ========================================================================== */

quantumChannelAccuracySpecification
    : identifier ASSIGN expression
    ;


/* ============================================================================
 * 7. PHYSICALITY SPECIFICATION
 * ========================================================================== */

quantumChannelPhysicalitySpecification
    : identifier ASSIGN expression
    ;


/* ============================================================================
 * 8. COMPOSITION SPECIFICATION
 * ========================================================================== */

quantumChannelCompositionSpecification
    : identifier ASSIGN expression
    ;


/* ============================================================================
 * 9. NOISE-CHANNEL APPLICATION
 * ========================================================================== */

/*
 * The existing lexer already provides NOISE.
 *
 * This is intentionally a narrow source-level integration point for applying
 * a channel as a noise/process effect without inventing a CHANNEL keyword.
 *
 * Generic quantum operation syntax remains owned by operations.g4.
 *
 * Example:
 *
 *     apply noise depolarizing()(q);
 *
 *     apply noise amplitude_damping(gamma)(q);
 *
 *     apply noise vendor::channel(parameter)(q0, q1);
 *
 * The semantic layer determines whether the referenced process is actually
 * a valid channel and whether it is appropriate as noise.
 */
quantumNoiseChannelApplication
    : APPLY NOISE quantumChannelReference
      quantumChannelArgumentClause?
      quantumOperationTargetClause
      quantumChannelPropertyBlock?
      SEMICOLON
    ;


/* ============================================================================
 * 10. EMBEDDABLE CHANNEL SPECIFICATION
 * ========================================================================== */

/*
 * This rule is intentionally free of statement terminators.
 *
 * It is the integration point for future grammars that need a channel
 * descriptor without owning a complete statement.
 */
quantumChannelDescriptor
    : quantumChannelSpecification
    ;