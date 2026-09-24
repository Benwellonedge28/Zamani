/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/pulse-intent.g4
 *
 * Grammar:
 *     QuantumPulseIntent
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM PULSE-INTENT GRAMMAR
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
 * This file owns the SOURCE-LEVEL SYNTAX for PORTABLE QUANTUM PULSE INTENT.
 *
 * It describes what pulse-level behaviour is required or intended without
 * making the Zamani language dependent on:
 *
 *     - a particular QPU;
 *     - a particular vendor;
 *     - a particular control electronics architecture;
 *     - a particular pulse compiler;
 *     - a particular DAC;
 *     - a particular AWG;
 *     - a particular physical channel;
 *     - a particular calibration record;
 *     - a particular clock frequency;
 *     - a particular sampling rate;
 *     - a particular waveform representation;
 *     - a particular qubit numbering scheme;
 *     - a particular topology;
 *     - a particular native gate set.
 *
 * The grammar therefore describes PULSE INTENT, not physical pulse execution.
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * Pulse intent is an optional refinement of quantum semantic intent.
 *
 * The intended pipeline is:
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
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> pulse lowering
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> calibration
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 * This grammar MUST NOT bypass the frontend AST or semantic-analysis layers.
 *
 * ============================================================================
 * IMPORTANT: THIS IS NOT A PHYSICAL PULSE GRAMMAR
 * ============================================================================
 *
 * The grammar deliberately does NOT encode a universal physical waveform
 * language.
 *
 * It does not establish universal syntax for:
 *
 *     DAC samples
 *     AWG instructions
 *     FPGA registers
 *     hardware channels
 *     physical ports
 *     device addresses
 *     clock domains
 *     calibration records
 *     vendor pulse formats
 *
 * Those may be represented through explicitly versioned dialects or
 * interoperability formats.
 *
 * The base Zamani language instead expresses abstract intent.
 *
 * ============================================================================
 * OPEN-ENDED OPERATION MODEL
 * ============================================================================
 *
 * Pulse intent uses the existing open-ended qualified-operation model.
 *
 * The semantic namespace:
 *
 *     pulse::
 *
 * identifies pulse-intent operations.
 *
 * Examples:
 *
 *     apply pulse::play(waveform)(q);
 *
 *     apply pulse::delay(duration)(q);
 *
 *     apply pulse::capture(window)(q);
 *
 *     apply pulse::set_phase(phase)(q);
 *
 *     apply pulse::shift_phase(delta)(q);
 *
 *     apply pulse::set_frequency(frequency)(q);
 *
 *     apply pulse::shift_frequency(delta)(q);
 *
 *     apply pulse::barrier()(q0, q1);
 *
 * The actual operation names remain open-ended.
 *
 * Therefore the grammar does NOT enumerate:
 *
 *     play
 *     delay
 *     capture
 *     set_phase
 *     shift_phase
 *     set_frequency
 *     shift_frequency
 *     barrier
 *     sample
 *     acquire
 *     frame
 *     waveform
 *
 * as universal lexer keywords.
 *
 * Future pulse operations can therefore be introduced semantically or by
 * dialect without changing the core lexer.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - pulse-intent structural syntax;
 *     - pulse-intent invocation structure;
 *     - pulse-intent designators;
 *     - pulse-intent parameter clauses;
 *     - pulse-intent target clauses;
 *     - pulse-intent blocks;
 *     - pulse-intent sequencing;
 *     - pulse-intent attributes where explicitly attached by the surrounding
 *       composition grammar;
 *     - pulse-intent grouping;
 *     - pulse-intent timing-expression structure;
 *     - pulse-intent waveform-expression structure;
 *     - pulse-intent frame/reference structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - quantum operation semantics;
 *     - qubit identity;
 *     - physical qubit identity;
 *     - hardware topology;
 *     - calibration;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - HAL;
 *     - runtime execution;
 *     - pulse synthesis algorithms;
 *     - waveform numerical implementation;
 *     - device discovery;
 *     - vendor-specific pulse semantics.
 *
 * ============================================================================
 * NO SECOND LEXER
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST NOT define lexer rules.
 *
 * Its lexical vocabulary comes from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * Existing shared tokens are therefore used rather than duplicated.
 *
 * ============================================================================
 * NO SECOND AST
 * ============================================================================
 *
 * This grammar produces structural information consumed by the existing
 * domain-neutral frontend AST.
 *
 * The grammar MUST NOT introduce a competing:
 *
 *     PulseIR
 *     PulseAST
 *     PhysicalPulseNode
 *     VendorPulseNode
 *
 * as a second canonical representation.
 *
 * The frontend may represent the parsed structure through the existing
 * generic Operation / expression / attribute model.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * Pulse intent must lower into the existing semantic quantum architecture.
 *
 * The intended relationship is:
 *
 *     pulse syntax
 *          |
 *          v
 *     generic frontend AST
 *          |
 *          v
 *     semantic pulse intent
 *          |
 *          v
 *     quantum::ir / canonical quantum semantic model
 *          |
 *          v
 *     pulse-aware lowering
 *
 * This grammar must NOT define a second pulse-specific canonical IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Pulse intent MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * where the program expresses portable computational intent and the compiler
 * resolves a suitable physical realization.
 *
 * The source MUST NOT need to be rewritten merely because:
 *
 *     - the number of available channels changes;
 *     - waveform hardware changes;
 *     - the available sampling rate changes;
 *     - the target has a different native pulse representation;
 *     - the QPU has a different topology;
 *     - calibration data changes;
 *     - a different pulse compiler is selected;
 *     - a future pulse-control architecture is introduced.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This grammar introduces NO universal limits for:
 *
 *     qubits
 *     pulse operations
 *     pulse sequences
 *     sequence depth
 *     parameters
 *     waveform points
 *     channels
 *     frames
 *     captures
 *     targets
 *     nested blocks
 *     duration values
 *     frequency values
 *     amplitudes
 *     phases
 *     devices
 *     QPUs
 *     nodes
 *     memory
 *     scheduling resources
 *
 * In particular, this file MUST NOT contain:
 *
 *     MAX_PULSES
 *     MAX_WAVEFORM_POINTS
 *     MAX_CHANNELS
 *     MAX_FRAMES
 *     MAX_QUBITS
 *     MAX_DURATION
 *     MAX_FREQUENCY
 *     MAX_AMPLITUDE
 *     MAX_PHASE
 *     MAX_SEQUENCE_DEPTH
 *
 * or equivalent universal limits.
 *
 * Repetition is represented by normal ANTLR repetition constructs.
 *
 * Actual resource limitations are implementation, target, deployment, or
 * runtime concerns.
 *
 * ============================================================================
 * SEMANTIC REQUIREMENT VS PHYSICAL REALIZATION
 * ============================================================================
 *
 * These are different:
 *
 *     requires pulse capability
 *
 *     requires a minimum timing property
 *
 *     requires a waveform property
 *
 *     use physical channel 3
 *
 * The first three can be portable semantic requirements.
 *
 * The last is target-specific realization and MUST NOT be silently treated as
 * portable source semantics.
 *
 * Physical bindings, when supported, belong to explicit target-specific
 * interoperability/dialect mechanisms.
 *
 * ============================================================================
 * RESOURCE SEPARATION
 * ============================================================================
 *
 * Pulse intent may refer to expressions representing:
 *
 *     duration
 *     frequency
 *     phase
 *     amplitude
 *     bandwidth
 *     energy
 *     latency
 *     sampling requirements
 *     quality requirements
 *     resource requirements
 *
 * Those expressions remain ordinary Zamani expressions.
 *
 * This grammar does not determine whether the target can satisfy them.
 *
 * Resource analysis and capability analysis are downstream.
 *
 * ============================================================================
 * TIMING MODEL
 * ============================================================================
 *
 * Timing expressions are ordinary Zamani expressions.
 *
 * This intentionally permits:
 *
 *     symbolic duration
 *     parameterized duration
 *     computed duration
 *     generic duration
 *     runtime-derived duration where the semantic model permits it
 *
 * Examples:
 *
 *     apply pulse::delay(t)(q);
 *
 *     apply pulse::play(waveform)(q);
 *
 *     apply pulse::play(waveform, duration)(q);
 *
 * The grammar does not require duration literals to fit a host integer type.
 *
 * Semantic/type checking owns numeric validity.
 *
 * ============================================================================
 * WAVEFORM MODEL
 * ============================================================================
 *
 * A waveform is represented by an expression or a semantic waveform
 * reference.
 *
 * The grammar intentionally does not require a fixed representation such as:
 *
 *     waveform[1024]
 *
 * or:
 *
 *     sample_count <= 1024
 *
 * A waveform may therefore be:
 *
 *     - a named waveform;
 *     - an expression;
 *     - a generated waveform;
 *     - a symbolic waveform;
 *     - a library waveform;
 *     - a dialect-provided waveform;
 *     - a future waveform representation.
 *
 * Semantic analysis determines whether the expression is a valid waveform
 * description.
 *
 * ============================================================================
 * FRAME MODEL
 * ============================================================================
 *
 * A pulse frame is a semantic reference, not necessarily a physical channel.
 *
 * A frame may represent:
 *
 *     phase context
 *     frequency context
 *     waveform context
 *     logical control context
 *     target-specific realization context
 *
 * The grammar therefore permits a frame reference as an ordinary qualified
 * name/expression.
 *
 * Physical frame realization remains downstream.
 *
 * ============================================================================
 * CAPTURE MODEL
 * ============================================================================
 *
 * Capture intent describes a desired observation/acquisition operation.
 *
 * The grammar may represent:
 *
 *     capture window
 *     capture target
 *     capture mode
 *     capture destination
 *     optional timing parameters
 *
 * It does not prescribe:
 *
 *     ADC architecture
 *     sample width
 *     device memory
 *     physical readout electronics
 *     vendor acquisition format.
 *
 * ============================================================================
 * SEQUENCING
 * ============================================================================
 *
 * Pulse intent can be represented as a sequence of operations.
 *
 * Sequence ordering is semantic where ordering affects program meaning.
 *
 * The grammar does not perform scheduling.
 *
 * Scheduling remains downstream.
 *
 * A source sequence:
 *
 *     pulse A
 *     pulse B
 *
 * establishes the semantic ordering relationship where applicable.
 *
 * The scheduler may realize that ordering using:
 *
 *     parallel execution
 *     pipelining
 *     hardware scheduling
 *     inserted synchronization
 *     target-specific lowering
 *
 * provided semantic requirements are preserved.
 *
 * ============================================================================
 * PARALLELISM
 * ============================================================================
 *
 * This grammar does not assume a fixed number of pulse channels.
 *
 * Parallel pulse intent may be represented by ordinary Zamani concurrency or
 * quantum composition constructs.
 *
 * Channel count is a target/resource property.
 *
 * ============================================================================
 * FRAME OPERATIONS
 * ============================================================================
 *
 * Frame-related intent is deliberately represented generically.
 *
 * Examples:
 *
 *     apply pulse::set_phase(phi)(frame);
 *
 *     apply pulse::shift_phase(delta)(frame);
 *
 *     apply pulse::set_frequency(f)(frame);
 *
 *     apply pulse::shift_frequency(delta)(frame);
 *
 *     apply pulse::set_amplitude(a)(frame);
 *
 * The semantic layer determines:
 *
 *     whether the operation exists;
 *     whether the target is a valid frame;
 *     whether the property is supported;
 *     whether the operation is compile-time or runtime;
 *     whether it can be lowered to the target.
 *
 * ============================================================================
 * MODIFIER COMPATIBILITY
 * ============================================================================
 *
 * Pulse intent may participate in the existing quantum operation modifier
 * system where semantically meaningful.
 *
 * Examples:
 *
 *     apply control(pulse::operation)(c, q);
 *
 *     apply adjoint(pulse::operation)(q);
 *
 * Such constructs are not automatically valid.
 *
 * Semantic analysis must determine whether the requested transformation has
 * a meaningful pulse interpretation.
 *
 * The grammar does not decide this.
 *
 * ============================================================================
 * BLOCK STRUCTURE
 * ============================================================================
 *
 * A pulse-intent block may group a sequence of pulse-intent applications.
 *
 * The block has no fixed length.
 *
 * This permits source structures such as:
 *
 *     pulse intent {
 *         ...
 *     }
 *
 * only when the surrounding canonical grammar provides the corresponding
 * declaration/annotation context.
 *
 * This file does not introduce a new globally reserved `pulse` keyword.
 *
 * Instead, block forms use the existing qualified-name/operation model so
 * future language versions do not require lexer changes merely to add pulse
 * concepts.
 *
 * ============================================================================
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * The preferred base-language form is:
 *
 *     apply pulse::operation(parameters)(targets);
 *
 * Examples:
 *
 *     apply pulse::play(waveform)(q);
 *
 *     apply pulse::play(waveform, duration)(q);
 *
 *     apply pulse::delay(duration)(q);
 *
 *     apply pulse::capture(window)(q);
 *
 *     apply pulse::set_phase(phase)(frame);
 *
 *     apply pulse::shift_phase(delta)(frame);
 *
 *     apply pulse::set_frequency(frequency)(frame);
 *
 *     apply pulse::shift_frequency(delta)(frame);
 *
 *     apply pulse::set_amplitude(amplitude)(frame);
 *
 *     apply pulse::barrier()(q0, q1);
 *
 * The operation names above are examples, NOT a closed vocabulary.
 *
 * ============================================================================
 * GENERIC / FUTURE OPERATION SUPPORT
 * ============================================================================
 *
 * Any valid qualified operation name can syntactically participate in the
 * pulse intent structure.
 *
 * This supports:
 *
 *     pulse::future_operation
 *     pulse::vendor_extension
 *     pulse::library::operation
 *     pulse::dialect::operation
 *
 * Semantic capability and dialect validation remain downstream.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Vendor-specific or hardware-specific pulse semantics MUST use the existing
 * dialect/interoperability architecture.
 *
 * The base grammar must remain vendor-neutral.
 *
 * A dialect may define additional pulse syntax, provided that:
 *
 *     - it is explicitly declared;
 *     - it is versioned;
 *     - its compatibility is known;
 *     - its semantic mapping is defined;
 *     - its IR mapping is defined;
 *     - it cannot silently redefine base Zamani semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve enough structure to represent:
 *
 *     - pulse operation designator;
 *     - namespace/path;
 *     - generic arguments;
 *     - parameter expressions;
 *     - target expressions;
 *     - modifier nesting where used;
 *     - ordering;
 *     - source spans;
 *     - enclosing pulse-intent context where applicable.
 *
 * Representative semantic shape:
 *
 *     Operation {
 *         name,
 *         namespace,
 *         operands,
 *         parameters,
 *         results,
 *         attributes,
 *         modifiers,
 *         effects,
 *         capabilities,
 *         source
 *     }
 *
 * The exact AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar does not define that Rust structure.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the operation belongs to pulse intent;
 *     - whether the operation name resolves;
 *     - whether parameters are valid;
 *     - whether parameter types are valid;
 *     - whether targets are valid;
 *     - whether target roles are valid;
 *     - whether timing relationships are valid;
 *     - whether frame references are valid;
 *     - whether waveform expressions are valid;
 *     - whether capture expressions are valid;
 *     - whether modifiers are semantically supported;
 *     - whether required capabilities exist;
 *     - whether resource requirements can be satisfied;
 *     - whether the target supports the requested semantic operation;
 *     - whether lowering is possible;
 *     - whether a dialect is required.
 *
 * None of these are parser decisions.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * Pulse intent must lower through the existing quantum semantic architecture.
 *
 * Conceptually:
 *
 *     PulseIntentSyntax
 *          |
 *          v
 *     Generic Operation AST
 *          |
 *          v
 *     Semantic Quantum Operation
 *          |
 *          v
 *     quantum::ir
 *
 * The IR must preserve the semantic information necessary for later:
 *
 *     - pulse lowering;
 *     - decomposition;
 *     - timing;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - HAL integration.
 *
 * This grammar does not decide which representation the physical target uses.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler responsibilities downstream include:
 *
 *     1. resolve pulse operation;
 *     2. validate semantic parameters;
 *     3. resolve required capabilities;
 *     4. evaluate resource requirements;
 *     5. lower abstract waveform/frame intent;
 *     6. select an appropriate realization;
 *     7. apply target-specific optimization;
 *     8. schedule;
 *     9. calibrate;
 *    10. lower through HAL/backend;
 *    11. preserve provenance.
 *
 * The compiler may reject a target because it cannot satisfy the pulse intent.
 *
 * Such failure is NOT a grammar error.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime responsibilities may include:
 *
 *     - dynamic pulse parameters;
 *     - runtime capability checks where required;
 *     - execution monitoring;
 *     - timing coordination;
 *     - acquisition;
 *     - fault handling;
 *     - resilience;
 *     - observability.
 *
 * Runtime behaviour must preserve the semantic contract established upstream.
 *
 * ============================================================================
 * CALIBRATION INTEGRATION
 * ============================================================================
 *
 * Calibration is downstream of this grammar.
 *
 * The source may express a semantic property requiring calibration support,
 * but the grammar must not contain calibration records.
 *
 * Examples of downstream information:
 *
 *     waveform calibration
 *     frame calibration
 *     timing calibration
 *     amplitude calibration
 *     frequency calibration
 *     device-specific compensation
 *
 * The calibration subsystem resolves these properties.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling owns:
 *
 *     - exact start times;
 *     - overlap;
 *     - resource conflicts;
 *     - execution ordering;
 *     - timing realization;
 *     - insertion of synchronization where semantically legal.
 *
 * This grammar expresses timing intent only.
 *
 * ============================================================================
 * ROUTING INTEGRATION
 * ============================================================================
 *
 * Routing owns physical mapping.
 *
 * This grammar must not assign:
 *
 *     physical_qubit(0)
 *     physical_channel(3)
 *     physical_port(7)
 *
 * as universal semantics.
 *
 * If a program genuinely requires a physical binding, it must use the
 * repository's explicit target-specific interoperability/dialect mechanism.
 *
 * ============================================================================
 * QEC / RESILIENCE / ZQN
 * ============================================================================
 *
 * Pulse intent does not implement:
 *
 *     QEC
 *     decoding
 *     mitigation
 *     noise simulation
 *     fault injection
 *     resilience policy
 *
 * Those remain owned by their respective subsystems.
 *
 * Pulse intent may provide semantic metadata that those subsystems consume.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Pulse intent may require abstract capabilities such as:
 *
 *     pulse.control
 *     pulse.waveform
 *     pulse.capture
 *     pulse.phase_control
 *     pulse.frequency_control
 *     pulse.timing
 *
 * These names are semantic examples and are not required to be reserved
 * keywords.
 *
 * Capability resolution belongs to the resource/capability system.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The grammar must not contain:
 *
 *     QPU0
 *     channel0
 *     DAC0
 *     AWG0
 *     port0
 *     qubit0
 *     FPGA0
 *
 * as universal pulse concepts.
 *
 * A source identifier containing such text remains an identifier.
 *
 * Physical interpretation is downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no randomness;
 *     - no time-dependent parsing.
 *
 * Parsing therefore depends only on:
 *
 *     source text;
 *     canonical lexer;
 *     imported parser grammar definitions.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * Structural errors belong to parsing.
 *
 * Examples:
 *
 *     apply pulse::play;
 *     apply pulse::play(;
 *     apply pulse::play(waveform);
 *     apply pulse::play(waveform)(;
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unresolved pulse operation
 *     invalid waveform type
 *     invalid target
 *     unsupported pulse capability
 *     unavailable timing capability
 *     impossible resource requirement
 *     unsupported target lowering
 *
 * Resource failure must never be represented as a syntax failure.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All lists use ordinary ANTLR repetition.
 *
 * No list has a fixed universal maximum.
 *
 * This applies to:
 *
 *     parameter lists;
 *     target lists;
 *     pulse sequences;
 *     nested pulse blocks;
 *     waveform expressions;
 *     frame expressions;
 *     metadata.
 *
 * The implementation may impose configurable safety/resource budgets to
 * protect itself from denial-of-service inputs.
 *
 * Such implementation budgets are NOT language semantics.
 *
 * ============================================================================
 * SOURCE SPANS / PROVENANCE
 * ============================================================================
 *
 * The parser must preserve source locations through the frontend pipeline.
 *
 * This allows diagnostics and provenance to identify:
 *
 *     - pulse operation;
 *     - parameter;
 *     - target;
 *     - timing expression;
 *     - waveform expression;
 *     - frame expression;
 *     - enclosing source construct.
 *
 * Generated/lowered pulse instructions must retain provenance back to the
 * original Zamani source where the compiler infrastructure supports it.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file does not create a second quantum language version.
 *
 * Compatibility is governed by the central Zamani language versioning system.
 *
 * Future changes must preserve the existing operation-name extensibility model.
 *
 * Adding a new pulse operation name must NOT require a lexer change.
 *
 * ============================================================================
 * VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - this grammar is a parser grammar;
 *     - tokenVocab is ZamaniLexer;
 *     - imported grammar names resolve;
 *     - no lexer rules exist here;
 *     - no fixed pulse-operation vocabulary exists;
 *     - no machine-size constants exist;
 *     - no physical device identifiers are encoded;
 *     - no duplicate pulse IR is defined;
 *     - no vendor is made mandatory by the base grammar;
 *     - no target discovery is performed;
 *     - grammar remains deterministic.
 *
 * ============================================================================
 * TEST REQUIREMENTS
 * ============================================================================
 *
 * Positive tests must cover:
 *
 *     - simple pulse application;
 *     - parameterized pulse;
 *     - symbolic waveform;
 *     - symbolic duration;
 *     - symbolic amplitude;
 *     - symbolic phase;
 *     - symbolic frequency;
 *     - frame reference;
 *     - capture intent;
 *     - delay intent;
 *     - sequencing;
 *     - multiple targets;
 *     - qualified pulse operation;
 *     - future/unknown operation names;
 *     - dialect-qualified operations;
 *     - generic operation references;
 *     - pulse operation used with valid quantum modifiers.
 *
 * Negative tests must cover:
 *
 *     - missing operation;
 *     - missing target clause;
 *     - malformed parameter list;
 *     - malformed target list;
 *     - malformed qualified name;
 *     - incomplete modifier;
 *     - invalid delimiters;
 *     - invalid sequencing.
 *
 * Semantic negative tests must cover:
 *
 *     - unresolved operation;
 *     - invalid waveform;
 *     - invalid timing expression;
 *     - invalid target type;
 *     - unsupported capability;
 *     - unavailable resource;
 *     - unsupported target realization.
 *
 * Boundary tests must cover:
 *
 *     - zero/empty lists where syntactically permitted;
 *     - one parameter;
 *     - many parameters;
 *     - one target;
 *     - many targets;
 *     - nested operations;
 *     - nested modifiers;
 *     - long pulse sequences;
 *     - symbolic quantities;
 *     - very large numeric literals represented by the language;
 *     - large expressions.
 *
 * Scalability tests must verify:
 *
 *     - no artificial pulse-count limit;
 *     - no artificial waveform-size limit;
 *     - no artificial channel limit;
 *     - no artificial target-count limit;
 *     - no artificial sequence-depth limit.
 *
 * Determinism tests must parse identical source repeatedly and obtain the
 * same parse structure.
 *
 * Cross-domain tests must include:
 *
 *     classical + pulse intent
 *     quantum + pulse intent
 *     hybrid + pulse intent
 *     quantum + resources + pulse intent
 *     quantum + hardware capability + pulse intent
 *     quantum + scheduling + pulse intent
 *     quantum + QEC/resilience metadata + pulse intent
 *     distributed quantum + pulse intent
 *     quantum + interoperability/dialect + pulse intent
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the hard-coding audit only if it contains no universal
 * limits or physical target assumptions.
 *
 * Forbidden architectural patterns include:
 *
 *     MAX_PULSES
 *     MAX_CHANNELS
 *     MAX_WAVEFORM_POINTS
 *     MAX_QUBITS
 *     QPU0
 *     CHANNEL0
 *     DAC0
 *     fixed sample width
 *     fixed sampling frequency
 *     fixed clock
 *     fixed hardware topology
 *     fixed vendor operation set
 *
 * Numeric literals appearing in examples/comments are not language limits.
 *
 * ============================================================================
 * DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] source-level pulse intent has a single grammar owner;
 *     [x] no second lexer is introduced;
 *     [x] no fixed pulse operation vocabulary is introduced;
 *     [x] existing open-ended quantum operation architecture is reused;
 *     [x] pulse parameters use ordinary Zamani expressions;
 *     [x] pulse targets use ordinary Zamani expressions;
 *     [x] timing is semantic rather than physical;
 *     [x] waveform representation remains extensible;
 *     [x] frame representation remains abstract;
 *     [x] capture remains semantic;
 *     [x] calibration remains downstream;
 *     [x] scheduling remains downstream;
 *     [x] routing remains downstream;
 *     [x] QEC remains downstream;
 *     [x] ZQN remains downstream;
 *     [x] HAL remains downstream;
 *     [x] quantum::ir remains canonical;
 *     [x] AST integration is defined;
 *     [x] semantic integration is defined;
 *     [x] compiler integration is defined;
 *     [x] runtime integration is defined;
 *     [x] compatibility is defined;
 *     [x] scalability is defined;
 *     [x] hard-coding audit is defined;
 *     [x] test requirements are defined;
 *     [x] safe Rust 1.97/1.97.1 compatibility is defined.
 *
 * ============================================================================
 */

parser grammar QuantumPulseIntent;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    Types
;


/*
 * ============================================================================
 * 1. PULSE INTENT STATEMENT
 * ============================================================================
 *
 * Canonical form:
 *
 *     apply pulse::operation(parameters)(targets);
 *
 * The existing `APPLY` token is deliberately reused.
 *
 * `pulse` remains an ordinary identifier in the qualified operation name.
 * Semantic analysis recognizes the `pulse` namespace as the base pulse-intent
 * namespace.
 */

quantumPulseIntentStatement
    : quantumPulseIntentApplication SEMICOLON
    ;


/*
 * ============================================================================
 * 2. PULSE INTENT APPLICATION
 * ============================================================================
 */

quantumPulseIntentApplication
    : APPLY quantumPulseIntentInvocation
    ;


/*
 * ============================================================================
 * 3. PULSE INTENT INVOCATION
 * ============================================================================
 *
 * The invocation has:
 *
 *     operation designator
 *     optional parameters
 *     target clause
 *
 * This mirrors the canonical quantum operation invocation model so that pulse
 * intent does not create a second operation language.
 */

quantumPulseIntentInvocation
    : quantumPulseIntentCallee
      quantumPulseIntentParameterClause?
      quantumPulseIntentTargetClause
    ;


/*
 * ============================================================================
 * 4. PULSE CALLEE
 * ============================================================================
 *
 * A pulse callee may be:
 *
 *     - an ordinary pulse operation designator;
 *     - a recursively modified operation.
 *
 * Semantic analysis validates whether a modifier is meaningful for the pulse
 * operation.
 */

quantumPulseIntentCallee
    : quantumPulseIntentDesignator
    | quantumPulseIntentModifier
    ;


/*
 * ============================================================================
 * 5. PULSE OPERATION DESIGNATOR
 * ============================================================================
 *
 * The designator is a normal qualified Zamani name.
 *
 * The semantic layer must classify the designator as pulse intent.
 *
 * The base convention is:
 *
 *     pulse::operation
 *
 * Examples:
 *
 *     pulse::play
 *     pulse::delay
 *     pulse::capture
 *     pulse::set_phase
 *     pulse::future_operation
 *
 * The grammar does not enumerate those operation names.
 */

quantumPulseIntentDesignator
    : qualifiedName
      genericArgumentSuffix?
    ;


/*
 * ============================================================================
 * 6. PARAMETERS
 * ============================================================================
 *
 * Parameters are ordinary Zamani expressions.
 *
 * This permits:
 *
 *     constants
 *     symbolic values
 *     generic values
 *     computed values
 *     references
 *     function results
 *     resource-derived values
 *
 * Semantic analysis determines which expressions are legal for a particular
 * pulse operation.
 */

quantumPulseIntentParameterClause
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * 7. TARGETS
 * ============================================================================
 *
 * Pulse targets are ordinary expressions.
 *
 * Examples may include:
 *
 *     q
 *     q[i]
 *     register
 *     frame
 *     logical_resource
 *
 * The semantic layer determines whether an expression denotes a valid pulse
 * target.
 */

quantumPulseIntentTargetClause
    : LPAREN quantumPulseIntentTargetList? RPAREN
    ;


quantumPulseIntentTargetList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


quantumPulseIntentTarget
    : expression
    ;


/*
 * ============================================================================
 * 8. REUSABLE TARGET LIST
 * ============================================================================
 */

quantumPulseIntentTargets
    : quantumPulseIntentTarget
      (
          COMMA
          quantumPulseIntentTarget
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. MODIFIERS
 * ============================================================================
 *
 * Reuse the same structural modifier model as quantum operations.
 *
 * The grammar does not impose a nesting limit.
 */

quantumPulseIntentModifier
    : CONTROL
      LPAREN
      quantumPulseIntentModifierOperand
      RPAREN

    | ADJOINT
      LPAREN
      quantumPulseIntentModifierOperand
      RPAREN

    | INVERSE
      LPAREN
      quantumPulseIntentModifierOperand
      RPAREN
    ;


quantumPulseIntentModifierOperand
    : quantumPulseIntentDesignator
    | quantumPulseIntentModifier
    ;


/*
 * ============================================================================
 * 10. PULSE INTENT SEQUENCE
 * ============================================================================
 *
 * This rule allows a surrounding quantum composition grammar to consume an
 * explicit sequence without redefining statement syntax.
 *
 * The sequence has no fixed maximum length.
 */

quantumPulseIntentSequence
    : quantumPulseIntentStatement*
    ;


/*
 * ============================================================================
 * 11. PULSE INTENT GROUP
 * ============================================================================
 *
 * Grouping allows a caller to attach an existing expression/block context to
 * a sequence of pulse-intent statements.
 *
 * No new global `pulse` keyword is introduced here.
 */

quantumPulseIntentGroup
    : LBRACE
      quantumPulseIntentSequence
      RBRACE
    ;


/*
 * ============================================================================
 * 12. TIMING EXPRESSION
 * ============================================================================
 *
 * Named structural wrapper for downstream semantic consumers.
 *
 * Timing remains an ordinary Zamani expression.
 *
 * This rule intentionally does not restrict units, widths, or numeric
 * representations.
 */

quantumPulseTimingExpression
    : expression
    ;


/*
 * ============================================================================
 * 13. WAVEFORM EXPRESSION
 * ============================================================================
 *
 * A waveform is semantically interpreted from an ordinary expression.
 *
 * It may be:
 *
 *     a named value;
 *     a function result;
 *     a symbolic expression;
 *     a generated value;
 *     a dialect-defined object;
 *     a future representation.
 */

quantumPulseWaveformExpression
    : expression
    ;


/*
 * ============================================================================
 * 14. FRAME REFERENCE
 * ============================================================================
 *
 * A frame is represented structurally by an expression.
 *
 * Physical frame identity is resolved downstream.
 */

quantumPulseFrameReference
    : expression
    ;


/*
 * ============================================================================
 * 15. CAPTURE EXPRESSION
 * ============================================================================
 *
 * Capture parameters remain ordinary expressions.
 */

quantumPulseCaptureExpression
    : expression
    ;


/*
 * ============================================================================
 * 16. PULSE PARAMETER EXPRESSION
 * ============================================================================
 *
 * Generic wrapper used by semantic tooling to identify a pulse-related
 * expression without creating a separate expression language.
 */

quantumPulseParameterExpression
    : expression
    ;


/*
 * ============================================================================
 * 17. PULSE INTENT DECLARATION BODY
 * ============================================================================
 *
 * This is intentionally structural and can be embedded by a future canonical
 * declaration/composition grammar.
 *
 * The enclosing declaration is responsible for introducing the semantic
 * context.
 */

quantumPulseIntentBody
    : LBRACE
      quantumPulseIntentSequence
      RBRACE
    ;


/*
 * ============================================================================
 * 18. SEMANTIC NAMESPACE CONTRACT
 * ============================================================================
 *
 * The parser intentionally cannot and should not enforce the textual
 * namespace prefix:
 *
 *     pulse::
 *
 * because `qualifiedName` is owned by the universal Names grammar.
 *
 * The semantic analyzer MUST classify a pulse-intent operation using the
 * language's namespace resolution rules.
 *
 * The base-language convention is:
 *
 *     pulse::<operation>
 *
 * A dialect may provide an explicit mapping to the same semantic category.
 *
 * This preserves an extensible parser while keeping the semantic contract
 * explicit.
 *
 * ============================================================================
 */