/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/noise.g4
 *
 * Grammar:
 *     QuantumNoise
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM NOISE GRAMMAR
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
 * This file is the SINGLE SYNTAX OWNER for explicit SOURCE-LEVEL QUANTUM
 * NOISE APPLICATIONS.
 *
 * Noise is represented as an OPEN semantic category.
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     depolarizing
 *     amplitude_damping
 *     phase_damping
 *     bit_flip
 *     phase_flip
 *     bit_phase_flip
 *     thermal
 *     readout
 *     correlated
 *     coherent
 *     stochastic
 *     custom
 *     vendor-specific noise models
 *
 * or any other finite physical noise-model inventory.
 *
 * Those are semantic/backend concepts identified by ordinary source names.
 *
 * Examples of structurally valid noise references include:
 *
 *     depolarizing
 *     amplitude_damping
 *     vendor::noise
 *     custom::channel
 *     future::noise_model
 *
 * Their actual meaning is determined by semantic analysis and the relevant
 * capability/model registry.
 *
 * ============================================================================
 * SOURCE FORM
 * ============================================================================
 *
 * The canonical explicit application form is:
 *
 *     apply noise depolarizing(0.01)(q);
 *
 *     apply noise amplitude_damping(gamma)(q);
 *
 *     apply noise vendor::model(theta, rate)(q0, q1);
 *
 *     apply noise custom::correlated(parameters)(register);
 *
 * The first parenthesized clause belongs to noise parameters.
 *
 * The final parenthesized clause belongs to noise targets.
 *
 * The grammar does not determine:
 *
 *     - the mathematical meaning of the parameters;
 *     - the number of parameters;
 *     - parameter ranges;
 *     - target arity;
 *     - target dimensionality;
 *     - physical realization;
 *     - whether the model is Markovian;
 *     - whether the model is stochastic;
 *     - whether the model is correlated;
 *     - whether the model is CPTP;
 *     - whether the target supports the model.
 *
 * Those are semantic concerns.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantumNoiseStatement
 *     - quantumNoiseApplication
 *     - quantumNoiseReference
 *     - quantumNoiseParameterClause
 *     - quantumNoiseTargetClause
 *     - quantumNoiseTargetList
 *     - quantumNoiseTarget
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - quantum operation syntax generally;
 *     - measurement;
 *     - reset;
 *     - barriers;
 *     - feed-forward;
 *     - dynamic control;
 *     - QEC;
 *     - ZQN implementation;
 *     - noise-model mathematics;
 *     - Kraus operators;
 *     - Choi matrices;
 *     - Lindblad solvers;
 *     - density-matrix simulation;
 *     - stochastic sampling;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - hardware topology;
 *     - device selection;
 *     - physical qubit selection;
 *     - runtime execution;
 *     - quantum::ir implementation.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical grammar owner for explicit source-level
 * noise application syntax.
 *
 * Other grammar files may:
 *
 *     - reference quantumNoiseStatement;
 *     - compose it into larger quantum constructs;
 *     - provide compatibility aliases;
 *     - provide dialect-specific wrappers;
 *
 * but they MUST NOT independently redefine equivalent noise syntax.
 *
 * In particular, no other grammar should independently introduce:
 *
 *     NOISE ...
 *     noise ...
 *     K_NOISE ...
 *     quantumNoise...
 *
 * with a competing interpretation.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexical boundary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * backed by:
 *
 *     grammar/lexer/tokens.g4
 *
 * This grammar therefore consumes:
 *
 *     ZamaniLexer
 *
 * and does NOT declare lexer rules.
 *
 * The existing canonical lexer provides:
 *
 *     NOISE
 *     APPLY
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * and the ordinary identifier/name vocabulary.
 *
 * IMPORTANT:
 *
 * This file intentionally does NOT introduce:
 *
 *     CHANNEL
 *     K_CHANNEL
 *     K_NOISE
 *     NOISE_MODEL
 *     NOISE_CHANNEL
 *
 * merely to make noise parsing possible.
 *
 * `NOISE` is already part of the canonical lexical vocabulary.
 *
 * ============================================================================
 * TOKEN / SEMANTIC DISTINCTION
 * ============================================================================
 *
 * `NOISE` identifies the SOURCE CATEGORY.
 *
 * It does NOT identify a particular noise model.
 *
 * Therefore:
 *
 *     noise depolarizing
 *
 * means:
 *
 *     "apply the semantic noise model named depolarizing"
 *
 * and does NOT mean:
 *
 *     "the grammar knows what depolarizing means."
 *
 * This preserves an OPEN-WORLD language architecture.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *          +--> QuantumNoise
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> parameter typing
 *          +--> target typing
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> noise-model validation
 *          +--> quantum semantic validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> noise analysis
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> routing
 *          +--> scheduling
 *          +--> simulation
 *          +--> hardware lowering
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar MUST NOT bypass the AST/semantic/IR boundary.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Noise syntax expresses:
 *
 *     WHAT noise semantics are requested.
 *
 * It does NOT express:
 *
 *     WHICH QPU
 *     WHICH physical qubit
 *     WHICH device
 *     WHICH calibration record
 *     WHICH coupling map
 *     WHICH control electronics
 *     WHICH simulator
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH memory bank
 *     WHICH physical topology
 *
 * Therefore this grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_NOISE_OPERATIONS
 *     MAX_TARGETS
 *     MAX_PARAMETERS
 *     MAX_KRAUS_OPERATORS
 *     MAX_CHANNEL_DIMENSION
 *     MAX_REGISTER_WIDTH
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * or equivalent artificial limits.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar permits arbitrary syntactic repetition through ordinary ANTLR
 * repetition constructs.
 *
 * For example:
 *
 *     quantumNoiseTargetList
 *         : quantumNoiseTarget (COMMA quantumNoiseTarget)*
 *         ;
 *
 * does not establish a finite source-level target limit.
 *
 * Actual limits arise only from:
 *
 *     - source representation;
 *     - parser/compiler resources;
 *     - semantic validity;
 *     - target capabilities;
 *     - available memory;
 *     - execution resources;
 *     - user/provider policies.
 *
 * Those are NOT grammar limits.
 *
 * ============================================================================
 * PARAMETER MODEL
 * ============================================================================
 *
 * Noise parameters are ordinary Zamani expressions.
 *
 * This deliberately permits:
 *
 *     numeric parameters;
 *     symbolic parameters;
 *     variables;
 *     constants;
 *     generic expressions;
 *     function results;
 *     compile-time values;
 *     runtime values where semantically permitted;
 *     structured expressions supported by the general expression grammar.
 *
 * The grammar does NOT impose:
 *
 *     probability ranges;
 *     floating-point precision;
 *     matrix dimensions;
 *     time ranges;
 *     rate limits;
 *     temperature ranges;
 *     error-rate limits.
 *
 * Semantic analysis owns those checks.
 *
 * ============================================================================
 * TARGET MODEL
 * ============================================================================
 *
 * Noise targets are parsed as general expressions.
 *
 * This deliberately supports future target forms without creating a second
 * quantum-target grammar.
 *
 * Examples may include:
 *
 *     q
 *     q[0]
 *     register
 *     register[index]
 *     logical_qubit
 *     expression_returning_quantum_resource
 *
 * Semantic analysis determines whether the expression denotes a valid quantum
 * noise target.
 *
 * The grammar therefore does NOT hard-code:
 *
 *     q0
 *     q1
 *     physical_q0
 *     physical_q1
 *     QPU0
 *     device0
 *
 * ============================================================================
 * NOISE MODEL SEMANTICS
 * ============================================================================
 *
 * Semantic analysis is responsible for determining whether the referenced
 * noise model represents:
 *
 *     - a valid quantum channel;
 *     - a stochastic process;
 *     - a coherent error;
 *     - a correlated process;
 *     - a measurement/readout model;
 *     - a state-preparation error;
 *     - a composite noise process;
 *     - a custom user-defined model;
 *     - a vendor/dialect-provided model;
 *     - a simulator-only model;
 *     - a target-supported model.
 *
 * If a semantic model claims to represent a quantum channel, the semantic
 * layer may validate the appropriate mathematical invariants, such as the
 * required map properties for the declared channel kind.
 *
 * The grammar MUST NOT perform those validations.
 *
 * ============================================================================
 * OPEN-WORLD NAME RESOLUTION
 * ============================================================================
 *
 * Noise names are source-level names.
 *
 * Examples:
 *
 *     depolarizing
 *     amplitude_damping
 *     correlated::noise
 *     vendor::readout
 *     application::custom_model
 *     future::noise_model
 *
 * Name resolution may find:
 *
 *     - a built-in semantic model;
 *     - an imported model;
 *     - a user-defined model;
 *     - a dialect extension;
 *     - a capability-provided model;
 *     - a target-specific implementation;
 *     - a simulation model.
 *
 * The parser must remain independent of that registry.
 *
 * ============================================================================
 * NO ENUMERATION
 * ============================================================================
 *
 * The following is explicitly forbidden:
 *
 *     quantumNoiseModel
 *         : DEPOLARIZING
 *         | AMPLITUDE_DAMPING
 *         | PHASE_DAMPING
 *         | ...
 *         ;
 *
 * That architecture would require a grammar edit every time a new model is
 * introduced.
 *
 * The production design instead uses:
 *
 *     qualifiedName
 *
 * so the language remains extensible without recompiling the grammar merely
 * because a new semantic noise model exists.
 *
 * ============================================================================
 * GENERIC REFERENCES
 * ============================================================================
 *
 * Generic arguments remain part of the universal name/type system.
 *
 * Example:
 *
 *     apply noise model::<T>(parameter)(target);
 *
 * The exact generic argument semantics are resolved downstream.
 *
 * This grammar does not create a second generic-argument grammar.
 *
 * ============================================================================
 * NOISE PARAMETER / TARGET DISAMBIGUATION
 * ============================================================================
 *
 * The syntax deliberately separates:
 *
 *     noise reference
 *     parameter clause
 *     target clause
 *
 * Example:
 *
 *     apply noise depolarizing(p)(q);
 *
 * parses structurally as:
 *
 *     APPLY
 *     NOISE
 *     reference = depolarizing
 *     parameters = (p)
 *     targets = (q)
 *
 * Example:
 *
 *     apply noise correlated::model(alpha, beta)(q0, q1);
 *
 * parses structurally as:
 *
 *     reference = correlated::model
 *     parameters = (alpha, beta)
 *     targets = (q0, q1)
 *
 * The grammar does not infer semantic arity from syntax.
 *
 * ============================================================================
 * TARGET CARDINALITY
 * ============================================================================
 *
 * A noise model may semantically require:
 *
 *     one target;
 *     multiple targets;
 *     a register;
 *     a tensor-like quantum value;
 *     an abstract quantum resource;
 *     a measurement/readout resource.
 *
 * The grammar accepts a target list of arbitrary source cardinality.
 *
 * Semantic analysis validates model-specific arity.
 *
 * ============================================================================
 * NOISE COMPOSITION
 * ============================================================================
 *
 * Composition of noise models is intentionally NOT represented by a closed
 * list of composition keywords in this file.
 *
 * Composition may be represented by ordinary source-level names and/or the
 * general operation/composition mechanisms of Zamani.
 *
 * For example, a semantic library may provide:
 *
 *     compose(...)
 *     sequence(...)
 *     tensor_product(...)
 *     correlated(...)
 *
 * without requiring new grammar keywords.
 *
 * This prevents noise mathematics from becoming a parser-level API.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM INTERACTION
 * ============================================================================
 *
 * Noise may depend on classical parameters:
 *
 *     apply noise model(rate)(q);
 *
 * or on values computed earlier:
 *
 *     apply noise model(calculated_rate)(q);
 *
 * Whether such a dependency is legal is a semantic/dataflow question.
 *
 * This grammar does not create a separate noise expression language.
 *
 * ============================================================================
 * MID-CIRCUIT / FEED-FORWARD INTEGRATION
 * ============================================================================
 *
 * This grammar does NOT own:
 *
 *     when
 *     measurement
 *     reset
 *     dynamic control
 *     feed-forward
 *
 * Those remain owned by their dedicated grammar components.
 *
 * A noise statement may nevertheless occur inside a quantum block or a
 * feed-forward body when the enclosing grammar permits
 * `quantumNoiseStatement`.
 *
 * ============================================================================
 * QEC INTEGRATION
 * ============================================================================
 *
 * Noise grammar describes requested noise/error semantics.
 *
 * QEC is downstream.
 *
 * The pipeline is:
 *
 *     noise source syntax
 *          |
 *          v
 *     semantic noise model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     QEC analysis / transformation
 *
 * This grammar MUST NOT encode:
 *
 *     surface code;
 *     repetition code;
 *     color code;
 *     code distance;
 *     decoder;
 *     syndrome extraction;
 *     logical-to-physical expansion.
 *
 * Those belong to the QEC semantic/compiler subsystem.
 *
 * ============================================================================
 * ZQN INTEGRATION
 * ============================================================================
 *
 * ZQN is responsible for downstream quantum noise/fault semantics.
 *
 * The grammar supplies source-level intent only.
 *
 * Conceptually:
 *
 *     QuantumNoise
 *          |
 *          v
 *     semantic noise model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     ZQN
 *
 * ZQN may determine:
 *
 *     fault propagation;
 *     stochastic behavior;
 *     correlated behavior;
 *     resilience implications;
 *     execution policies.
 *
 * None of these belong in parser actions.
 *
 * ============================================================================
 * HARDWARE / HAL INTEGRATION
 * ============================================================================
 *
 * The grammar does not identify physical hardware.
 *
 * The HAL may later determine whether a requested noise model is:
 *
 *     - directly supported;
 *     - emulatable;
 *     - approximable;
 *     - decomposable;
 *     - simulator-only;
 *     - unavailable.
 *
 * If unsupported, the semantic/compiler layer must issue a structured
 * diagnostic rather than the grammar changing itself.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for a domain-neutral AST node
 * representing an operation-like semantic construct.
 *
 * Conceptual shape:
 *
 *     NoiseApplication {
 *         reference,
 *         generic_arguments,
 *         parameters,
 *         targets,
 *         source_span
 *     }
 *
 * The exact Rust type is owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT introduce a competing Rust AST hierarchy.
 *
 * In particular, it must not create parser-specific:
 *
 *     NoiseModelEnum
 *     NoiseChannelEnum
 *     PhysicalNoiseNode
 *     QpuNoiseNode
 *
 * merely to enumerate current implementations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *     1. name resolution;
 *     2. generic argument validity;
 *     3. parameter typing;
 *     4. parameter availability;
 *     5. target existence;
 *     6. target quantum compatibility;
 *     7. noise-model arity;
 *     8. model parameter constraints;
 *     9. model mathematical validity where applicable;
 *    10. capability requirements;
 *    11. resource requirements;
 *    12. execution-domain compatibility;
 *    13. dynamic/runtime availability;
 *    14. QEC compatibility;
 *    15. ZQN compatibility.
 *
 * These checks are intentionally outside this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * There is NO `noise::ir`.
 *
 * A validated noise application lowers through the canonical semantic path:
 *
 *     NoiseApplication
 *          |
 *          v
 *     semantic quantum operation/noise effect
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> QEC
 *          +--> ZQN
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> simulation
 *          +--> HAL
 *
 * This preserves the repository's canonical `quantum::ir` boundary.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Source syntax may be accompanied by existing resource/capability constructs
 * such as:
 *
 *     requires capability("quantum.noise")
 *
 *     requires capability("quantum.noise.model")
 *
 *     requires capability("quantum.noise.correlated")
 *
 *     requires capability("quantum.readout")
 *
 *     requires memory >= required_memory
 *
 * These are semantic requirements.
 *
 * They are NOT interpreted by this grammar as hardware limits.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * The same source program must remain syntactically valid across:
 *
 *     simulator;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     hybrid system;
 *     distributed system;
 *     future execution substrate.
 *
 * A target may lower a noise request differently.
 *
 * That does not require changing the source grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for identical:
 *
 *     source;
 *     language version;
 *     grammar version;
 *     lexical vocabulary.
 *
 * Parsing must not depend on:
 *
 *     hardware availability;
 *     network state;
 *     calibration;
 *     runtime state;
 *     random numbers;
 *     wall-clock time;
 *     environment variables.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * It performs no:
 *
 *     filesystem access;
 *     network access;
 *     command execution;
 *     hardware discovery;
 *     credential access;
 *     backend execution.
 *
 * Generated Rust must remain safe Rust under the repository's Rust policy.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser must provide source spans sufficient to identify:
 *
 *     noise keyword;
 *     noise reference;
 *     parameter clause;
 *     target clause.
 *
 * Examples of semantic diagnostics belong downstream:
 *
 *     unknown noise model;
 *     invalid noise parameter;
 *     wrong target type;
 *     unsupported noise capability;
 *     unavailable execution capability;
 *     invalid channel semantics.
 *
 * The parser should report syntax errors only.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This grammar uses direct, non-left-recursive structural productions.
 *
 * It does not recursively enumerate known noise models.
 *
 * This keeps grammar complexity independent of the number of registered noise
 * models.
 *
 * Very large programs remain subject to implementation resource availability,
 * but the grammar itself introduces no artificial source-level machine-size
 * limit.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust target-language actions.
 *
 * Generated Zamani frontend code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and repository safe-Rust policy:
 *
 *     no unsafe
 *
 * This grammar does not require `unsafe`.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 *
 *    `NOISE` and the punctuation tokens are supplied by the canonical
 *    `ZamaniLexer`.
 *
 * 2. EXPRESSIONS
 *
 *    Parameter and target expressions are delegated to the canonical
 *    `Expressions` grammar.
 *
 * 3. NAMES
 *
 *    Noise references use the canonical qualified-name grammar.
 *
 * 4. TYPES
 *
 *    Target/type compatibility is semantic; this grammar does not duplicate
 *    quantum type syntax.
 *
 * 5. QUANTUM ORCHESTRATOR
 *
 *    `grammar/quantum/quantum.g4` should expose:
 *
 *        quantumNoiseStatement
 *
 *    from its quantum element dispatch.
 *
 * 6. OPERATIONS
 *
 *    `grammar/quantum/operations.g4` remains the owner of ordinary:
 *
 *        apply operation(...)
 *
 *    syntax.
 *
 *    It must NOT be made the owner of noise-model enumeration.
 *
 * 7. CHANNELS
 *
 *    A future/companion:
 *
 *        grammar/quantum/channels.g4
 *
 *    may own reusable generic channel specifications.
 *
 *    If it is introduced, `noise.g4` should consume/delegate to that
 *    abstraction rather than duplicate channel semantics.
 *
 * 8. MEASUREMENT
 *
 *    Measurement/readout noise is represented semantically by the referenced
 *    model; measurement syntax remains owned by `measurement.g4`.
 *
 * 9. FEED-FORWARD
 *
 *    `classical-feedforward.g4` remains the sole owner of feed-forward syntax.
 *
 * 10. QEC
 *
 *     `error-correction.g4` remains the owner of QEC source constructs.
 *
 * 11. RESOURCE / CAPABILITY
 *
 *     Existing resource/capability grammars remain authoritative.
 *
 * 12. AST
 *
 *     Map to the existing domain-neutral AST contract.
 *
 * 13. SEMANTICS
 *
 *     Resolve the model and validate its meaning.
 *
 * 14. IR
 *
 *     Lower to the canonical `quantum::ir`.
 *
 * 15. BACKENDS
 *
 *     Let optimization, ZQN, QEC, routing, scheduling and HAL determine
 *     realization.
 *
 * ============================================================================
 * FEATURE COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is considered complete only when all of the following are true:
 *
 *     [x] canonical lexer token vocabulary used;
 *     [x] no new lexer keywords introduced;
 *     [x] no K_* compatibility vocabulary introduced;
 *     [x] no fixed noise-model enumeration;
 *     [x] no fixed hardware limit;
 *     [x] no physical device assumption;
 *     [x] no second expression language;
 *     [x] no second type language;
 *     [x] no second quantum IR;
 *     [x] parameter syntax defined;
 *     [x] target syntax defined;
 *     [x] arbitrary qualified noise names supported;
 *     [x] arbitrary parameter-list cardinality supported;
 *     [x] arbitrary target-list cardinality supported;
 *     [x] source spans preserved by parser architecture;
 *     [x] semantic responsibilities documented;
 *     [x] AST contract documented;
 *     [x] quantum::ir contract documented;
 *     [x] QEC integration documented;
 *     [x] ZQN integration documented;
 *     [x] HAL integration documented;
 *     [x] POCO-REAF documented;
 *     [x] Rust 1.97/1.97.1 compatible design;
 *     [x] no Rust actions;
 *     [x] no unsafe requirement.
 *
 * ============================================================================
 * PUBLIC GRAMMAR
 * ============================================================================
 */

/**
 * Complete source-level quantum noise statement.
 *
 * Canonical form:
 *
 *     apply noise <model>(<parameters>)(<targets>);
 *
 * The parameter clause may be omitted:
 *
 *     apply noise model(q);
 *
 * The target clause remains mandatory because an explicit noise application
 * must identify the semantic object to which the noise is applied.
 */
quantumNoiseStatement
    : quantumNoiseApplication
    ;


/**
 * Noise application.
 *
 * `APPLY NOISE` is intentionally distinct from ordinary operation syntax.
 *
 * This allows the semantic layer to preserve the fact that the source author
 * explicitly requested a noise/error process without requiring the grammar
 * to know the model's implementation.
 */
quantumNoiseApplication
    : APPLY
      NOISE
      quantumNoiseReference
      quantumNoiseParameterClause?
      quantumNoiseTargetClause
      SEMICOLON
    ;


/**
 * Open-world noise model reference.
 *
 * Examples:
 *
 *     depolarizing
 *     amplitude_damping
 *     vendor::noise
 *     custom::correlated
 *     future::model
 *
 * Generic arguments remain optional and are interpreted semantically.
 */
quantumNoiseReference
    : qualifiedName
      genericArgumentSuffix?
    ;


/**
 * Optional parameter clause.
 *
 * Examples:
 *
 *     (0.01)
 *     (gamma)
 *     (alpha, beta)
 *     (temperature, duration, rate)
 *
 * Parameter meaning is semantic.
 */
quantumNoiseParameterClause
    : LPAREN
      argumentList?
      RPAREN
    ;


/**
 * Mandatory target clause.
 *
 * Examples:
 *
 *     (q)
 *     (q0, q1)
 *     (register)
 *     (register[index])
 *
 * Target validity is semantic.
 */
quantumNoiseTargetClause
    : LPAREN
      quantumNoiseTargetList
      RPAREN
    ;


/**
 * Arbitrarily sized target list.
 *
 * No grammar-level target-count limit exists.
 */
quantumNoiseTargetList
    : quantumNoiseTarget
      (COMMA quantumNoiseTarget)*
    ;


/**
 * Noise target.
 *
 * General expressions are intentionally accepted here so that the grammar
 * does not need to predict every future quantum resource reference form.
 *
 * Semantic analysis MUST verify that the expression denotes a valid target
 * for the referenced noise model.
 */
quantumNoiseTarget
    : expression
    ;