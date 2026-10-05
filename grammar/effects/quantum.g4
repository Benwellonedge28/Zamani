/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/quantum.g4
 *
 * Grammar:
 *     QuantumEffects
 *
 * Status:
 *     CANONICAL MODULAR PRODUCTION PARSER GRAMMAR
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust.
 *     - No semantic predicates.
 *     - No parser actions.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for explicitly qualified quantum
 * effect references.
 *
 * Canonical examples:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::reset
 *     quantum::mid_circuit_measurement
 *     quantum::dynamic_control
 *     quantum::error_correction
 *     quantum::logical_state
 *     quantum::photonic::interaction
 *     quantum::future::operation
 *     quantum::vendor::extension
 *
 * The namespace after `quantum::` is OPEN-WORLD.
 *
 * This file therefore deliberately does NOT enumerate individual quantum
 * effects.
 *
 * Adding a new semantic quantum effect MUST NOT require modifying this file
 * merely to add its name.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser grammars
 *          |
 *          v
 *     QuantumEffects                    <-- THIS FILE
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic effect model
 *          |
 *     +----+-------------+-------------+
 *     |                  |             |
 *     v                  v             v
 *   effects          capabilities   resources
 *     |                  |             |
 *     +------------------+-------------+
 *                        |
 *                        v
 *              canonical semantic model
 *                        |
 *                        v
 *                    quantum::ir
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      optimize       route        schedule
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                 resilience / QEC / ZQN
 *                        |
 *                        v
 *                       HAL
 *                        |
 *                        v
 *                  target/runtime
 *
 * This grammar stops at SOURCE SYNTAX.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns exactly these parser-level concepts:
 *
 *     quantumEffectReference
 *     quantumEffectPath
 *     quantumEffectPathSegment
 *     quantumEffectReferenceList
 *
 * `quantumEffectReference` is the canonical public entry point.
 *
 * `quantumEffectPath` represents the open-world path following `quantum::`.
 *
 * `quantumEffectPathSegment` provides a stable named boundary for one path
 * segment.
 *
 * `quantumEffectReferenceList` provides a reusable non-empty source list.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - general effect references;
 *     - effect sets;
 *     - effect declarations;
 *     - effect operations;
 *     - effect invocation;
 *     - effect handlers;
 *     - effect polymorphism;
 *     - effect composition;
 *     - capability declarations;
 *     - capability requirements;
 *     - resource requirements;
 *     - resource quantities;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - policies;
 *     - quantum operations;
 *     - quantum gates;
 *     - qubits;
 *     - quantum registers;
 *     - circuits;
 *     - measurement syntax;
 *     - reset syntax;
 *     - observables;
 *     - quantum channels;
 *     - noise models;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - physical qubits;
 *     - QPU selection;
 *     - hardware discovery;
 *     - target selection;
 *     - quantum::ir;
 *     - runtime dispatch.
 *
 * Those responsibilities belong to their existing repository owners.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * General effect syntax remains owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * In particular, this file MUST NOT redefine:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     optionalEffectSet
 *
 * A general effect such as:
 *
 *     effects {
 *         quantum::measurement
 *     }
 *
 * is already structurally representable through the generic qualified-name
 * effect-reference grammar.
 *
 * This file exists only when a grammar consumer explicitly needs the
 * additional invariant:
 *
 *     the reference begins with the `quantum` namespace.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical public lexer boundary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume that vocabulary.
 *
 * Therefore this file MUST use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * ZamaniTokens is an internal lexical composition layer, not the public
 * parser-facing lexer vocabulary.
 *
 * ============================================================================
 * REQUIRED TOKENS
 * ============================================================================
 *
 * This grammar consumes only lexical tokens owned elsewhere:
 *
 *     K_QUANTUM
 *     DOUBLE_COLON
 *     COMMA
 *
 * The lexical ownership remains in grammar/lexer/.
 *
 * This file MUST NOT define any lexer rule.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Canonical identifier syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * This grammar imports Names and reuses:
 *
 *     identifier
 *
 * It therefore does not create a second identifier grammar.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This file imports:
 *
 *     Names
 *
 * only.
 *
 * It intentionally does NOT import:
 *
 *     Effects
 *     EffectSets
 *     EffectDeclarations
 *     EffectOperations
 *     EffectHandling
 *     Capabilities
 *     Resources
 *     Quantum
 *     QuantumTypes
 *     Hardware
 *     QEC
 *     ZQN
 *
 * This keeps the grammar leaf independent and prevents cyclic dependencies.
 *
 * Dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *        Names
 *          |
 *          v
 *     QuantumEffects
 *          |
 *          v
 *     higher-level grammar consumers
 *
 * ============================================================================
 * OPEN-WORLD SEMANTICS
 * ============================================================================
 *
 * The grammar intentionally accepts:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::reset
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::error_correction
 *     quantum::logical_qubit
 *     quantum::logical_state
 *     quantum::photonic::interaction
 *     quantum::ion::interaction
 *     quantum::future::operation
 *     quantum::custom::effect
 *
 * It also accepts future names that do not exist today.
 *
 * For example:
 *
 *     quantum::future::new_architecture::operation
 *
 * is syntactically valid.
 *
 * Whether such an effect exists, is imported, is authorized, is supported,
 * or can be realized is a semantic/capability question.
 *
 * ============================================================================
 * NO CLOSED QUANTUM EFFECT CATALOGUE
 * ============================================================================
 *
 * This file MUST NOT contain a grammar such as:
 *
 *     quantumEffect
 *         : MEASUREMENT
 *         | RESET
 *         | READOUT
 *         | ...
 *         ;
 *
 * That architecture would require grammar changes whenever the quantum
 * semantic vocabulary grows.
 *
 * Instead:
 *
 *     quantumEffectReference
 *         : K_QUANTUM DOUBLE_COLON quantumEffectPath
 *         ;
 *
 * makes the language extensible without making the grammar infinite or
 * requiring continual modification.
 *
 * ============================================================================
 * NAMESPACE SEMANTICS
 * ============================================================================
 *
 * The first namespace component is syntactically fixed to:
 *
 *     quantum
 *
 * represented by:
 *
 *     K_QUANTUM
 *
 * Everything after:
 *
 *     quantum::
 *
 * is an open-world identifier path.
 *
 * The grammar does not decide whether:
 *
 *     quantum::measurement
 *
 * means a built-in operation, declared effect, imported effect, dialect
 * effect, library effect, vendor extension, or future semantic construct.
 *
 * Semantic name resolution decides that.
 *
 * ============================================================================
 * EFFECT VS OPERATION
 * ============================================================================
 *
 * A quantum effect is NOT a quantum operation.
 *
 * For example:
 *
 *     quantum::measurement
 *
 * is an effect reference.
 *
 * A source operation such as:
 *
 *     measure q
 *
 * belongs to the quantum computational grammar.
 *
 * Likewise:
 *
 *     H
 *     X
 *     CNOT
 *
 * are operation identifiers or semantic operation descriptions, not effect
 * names owned by this grammar.
 *
 * This separation is essential:
 *
 *     effect
 *         describes computational interaction/behavior
 *
 *     operation
 *         describes a computation
 *
 *     capability
 *         describes what a realization can provide
 *
 *     resource
 *         describes required/provided quantities
 *
 *     target
 *         identifies a realization context
 *
 * ============================================================================
 * EFFECT VS CAPABILITY
 * ============================================================================
 *
 * These are intentionally independent concepts.
 *
 * Example:
 *
 *     quantum::measurement
 *
 * may semantically require a capability such as:
 *
 *     quantum.measurement
 *
 * but the grammar MUST NOT equate the two.
 *
 * The capability subsystem determines:
 *
 *     effect
 *         |
 *         v
 *     required capabilities
 *
 * This file only parses the effect identity.
 *
 * ============================================================================
 * EFFECT VS RESOURCE
 * ============================================================================
 *
 * This grammar MUST NOT parse resource requirements as part of the effect
 * name.
 *
 * Do NOT create syntax such as:
 *
 *     quantum::measurement requires 8 qubits
 *
 * inside this grammar.
 *
 * Resource syntax belongs to the resource/requirement subsystem.
 *
 * A semantic model may associate:
 *
 *     quantum::measurement
 *         |
 *         +--> resource requirements
 *
 * but that association occurs after parsing.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The following are NOT grammar concerns:
 *
 *     QPU identity
 *     physical qubit identity
 *     topology
 *     coupling graph
 *     calibration
 *     pulse channel
 *     gate duration
 *     hardware generation
 *     vendor backend
 *     device count
 *
 * Therefore source such as:
 *
 *     quantum::measurement
 *
 * does not select:
 *
 *     a particular QPU;
 *     a particular simulator;
 *     a particular processor;
 *     a physical qubit;
 *     a physical topology.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same source-level quantum effect reference must remain valid independent
 * of whether the semantic operation is eventually realized by:
 *
 *     - a tiny system;
 *     - an embedded system;
 *     - a CPU;
 *     - a multicore CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - an accelerator;
 *     - a quantum processor;
 *     - a quantum simulator;
 *     - an HPC system;
 *     - a cluster;
 *     - a distributed system;
 *     - a cloud environment;
 *     - a future computational substrate.
 *
 * Source portability does not imply that every target can satisfy every
 * semantic requirement.
 *
 * A target that cannot satisfy the requirements must fail capability/resource
 * analysis explicitly rather than requiring source-level rewriting.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite language-level limit is encoded for:
 *
 *     - effect path depth;
 *     - number of path segments;
 *     - number of effect references in a list;
 *     - number of effects in a program;
 *     - number of quantum operations;
 *     - number of qubits;
 *     - number of quantum registers;
 *     - number of QPUs;
 *     - number of devices;
 *     - number of processors;
 *     - number of nodes;
 *     - memory;
 *     - tensor rank;
 *     - network size.
 *
 * Repetition is expressed through ANTLR repetition operators.
 *
 * No constants such as:
 *
 *     MAX_QUANTUM_EFFECTS
 *     MAX_EFFECT_DEPTH
 *     MAX_QUBITS
 *     MAX_QPU_COUNT
 *     MAX_DEVICES
 *
 * may be introduced here.
 *
 * Practical implementation limits remain implementation/resource-policy
 * concerns rather than language semantics.
 *
 * ============================================================================
 * SYNTAX / SEMANTICS BOUNDARY
 * ============================================================================
 *
 * The parser establishes only:
 *
 *     1. the namespace is `quantum`;
 *     2. `::` follows the namespace;
 *     3. at least one identifier follows;
 *     4. additional namespace/path segments are structurally valid.
 *
 * Semantic analysis determines:
 *
 *     - whether the effect exists;
 *     - whether the effect is imported;
 *     - whether the effect is visible;
 *     - whether the effect is declared;
 *     - whether the effect is deprecated;
 *     - whether the effect is compatible;
 *     - whether the effect is permitted;
 *     - which capabilities it requires;
 *     - which resources it requires;
 *     - which effects it induces;
 *     - which quantum semantics it contributes;
 *     - whether it can cross into quantum::ir;
 *     - whether a target can realize it.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST should preserve at least:
 *
 *     namespace = quantum
 *     path segments
 *     source span
 *     original source spelling where required
 *     source ordering
 *
 * Conceptually:
 *
 *     QuantumEffectReference
 *     {
 *         namespace,
 *         path,
 *         source_span
 *     }
 *
 * The exact Rust AST representation remains owned by the frontend AST
 * subsystem.
 *
 * This grammar MUST NOT define Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A successful parse MUST NOT be interpreted as proof that the effect exists
 * or is executable.
 *
 * For example:
 *
 *     quantum::future::unknown_effect
 *
 * is syntactically valid.
 *
 * Semantic analysis may reject it because no semantic declaration,
 * registration, import, dialect, library, or intrinsic resolves the name.
 *
 * This separation permits future extension without grammar modification.
 *
 * ============================================================================
 * EFFECT ANALYSIS CONTRACT
 * ============================================================================
 *
 * After AST construction:
 *
 *     QuantumEffectReference
 *              |
 *              v
 *        name resolution
 *              |
 *              v
 *       effect definition
 *              |
 *       +------+------+
 *       |             |
 *       v             v
 *   capabilities   resources
 *       |             |
 *       +------+------+
 *              |
 *              v
 *      effect checking
 *              |
 *              v
 *    canonical semantic model
 *
 * Effect checking must remain independent of parser success.
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * This file MUST NOT construct quantum::ir.
 *
 * The canonical quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * A quantum effect may contribute metadata to the semantic model and
 * subsequently to quantum::ir, but this grammar has no direct dependency on
 * the IR implementation.
 *
 * ============================================================================
 * QEC CONTRACT
 * ============================================================================
 *
 * QEC is downstream.
 *
 * A source effect such as:
 *
 *     quantum::error_correction
 *
 * does not specify:
 *
 *     - a code family;
 *     - a code distance;
 *     - a decoder;
 *     - syndrome extraction;
 *     - physical qubit placement;
 *     - correction schedule;
 *     - logical-qubit encoding.
 *
 * Those decisions belong to semantic lowering and the QEC subsystem.
 *
 * ============================================================================
 * ZQN CONTRACT
 * ============================================================================
 *
 * ZQN is downstream.
 *
 * This grammar does not define:
 *
 *     noise;
 *     faults;
 *     leakage;
 *     loss;
 *     erasure;
 *     fault probabilities;
 *     calibration;
 *     fault locations.
 *
 * A semantic effect may interact with ZQN, but this file does not parse ZQN
 * semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     - actions;
 *     - predicates;
 *     - runtime calls;
 *     - I/O;
 *     - randomness;
 *     - environment inspection;
 *     - hardware inspection;
 *     - target selection.
 *
 * Given the same token stream and grammar configuration, parsing is
 * deterministic.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser context provides the structural information necessary for the
 * frontend to preserve:
 *
 *     - segment order;
 *     - source spans;
 *     - namespace spelling;
 *     - original source positions.
 *
 * Semantic normalization must not destroy source information required by
 * diagnostics, formatting, IDE tooling, provenance, or compatibility
 * reporting.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Structural parser errors include:
 *
 *     quantum
 *     quantum::
 *     quantum:::
 *     quantum::
 *     quantum::.
 *
 * depending on the lexical interpretation of the input.
 *
 * The following are syntactically valid and therefore MUST NOT be rejected
 * merely by this grammar:
 *
 *     quantum::unknown
 *     quantum::future::operation
 *     quantum::vendor::extension
 *
 * Unknown names are semantic diagnostics.
 *
 * Capability failures are semantic/capability diagnostics.
 *
 * Resource failures are resource diagnostics.
 *
 * Target realization failures are backend/runtime diagnostics.
 *
 * These categories MUST remain distinguishable.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical source spelling:
 *
 *     quantum::name
 *
 * Existing source forms such as:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::reset
 *
 * remain structurally compatible.
 *
 * Adding a new final or nested identifier segment does not require a grammar
 * revision.
 *
 * Compatibility migrations for historical spellings belong to:
 *
 *     grammar/compatibility/
 *
 * and MUST NOT be encoded as duplicate grammar alternatives here unless the
 * language specification explicitly requires them.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Quantum dialects may define additional semantic effect names beneath the
 * quantum namespace.
 *
 * For example:
 *
 *     quantum::dialect_name::effect
 *
 * The dialect registry/semantic subsystem determines whether the name is
 * valid.
 *
 * This grammar does not dynamically load dialects and does not inspect a
 * dialect registry.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Test ownership:
 *
 *     grammar/tests/effects/quantum/
 *
 * Required positive tests:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::reset
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::error_correction
 *     quantum::logical_state
 *     quantum::future::operation
 *     quantum::vendor::extension
 *     quantum::a::b::c
 *
 * Required list tests:
 *
 *     quantum::measurement, quantum::readout
 *     quantum::measurement, quantum::readout,
 *
 * Required negative structural tests:
 *
 *     quantum
 *     quantum::
 *     quantum::
 *     quantum:::
 *
 * Required semantic-negative tests:
 *
 *     quantum::unknown_effect
 *     quantum::future::unknown
 *
 * These must be rejected by semantic resolution where no corresponding
 * definition exists, not by the parser solely because the name is unfamiliar.
 *
 * Required scalability tests:
 *
 *     increasing path depth;
 *     increasing reference-list cardinality;
 *     large generated source units;
 *     long but valid semantic names;
 *     large cross-domain programs.
 *
 * No test may establish a universal maximum.
 *
 * Required determinism tests:
 *
 *     identical source parsed repeatedly;
 *     equivalent source under permitted formatting changes;
 *     identical source under identical language configuration.
 *
 * Required compatibility tests:
 *
 *     current canonical namespace spelling;
 *     supported historical aliases, if any;
 *     rejection of unsupported legacy forms.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *
 * EXPORTS:
 *
 *     quantumEffectReference
 *     quantumEffectPath
 *     quantumEffectPathSegment
 *     quantumEffectReferenceList
 *
 * CONSUMED_BY:
 *
 *     quantum-specific effect-aware grammar contexts;
 *     quantum effect semantic analysis;
 *     grammar/effects/effects.g4 integration when explicitly required;
 *     quantum/effect-aware composition grammars.
 *
 * AST_OWNER:
 *
 *     src/frontend/ast/
 *
 * SEMANTIC_OWNER:
 *
 *     effect semantic analysis +
 *     quantum semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic model;
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/quantum/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/spec/quantum.md
 *     grammar/specification/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * INTEGRATION WITH GENERAL EFFECTS
 * ============================================================================
 *
 * `grammar/effects/effect-sets.g4` remains the owner of generic effect
 * references.
 *
 * Therefore this grammar does not replace:
 *
 *     effectReference
 *
 * and does not redefine:
 *
 *     effectSet
 *
 * A general effect collection can continue to parse:
 *
 *     effects {
 *         quantum::measurement,
 *         quantum::readout,
 *     }
 *
 * through the generic qualified-name mechanism.
 *
 * If a higher-level grammar needs to distinguish a quantum effect
 * syntactically, it should consume:
 *
 *     quantumEffectReference
 *
 * from this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT DECLARATIONS
 * ============================================================================
 *
 * Effect declarations remain owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * This file does not declare effects.
 *
 * A declaration whose semantic identity belongs to the quantum namespace is
 * resolved by semantic/name-resolution machinery.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT HANDLING
 * ============================================================================
 *
 * Handler syntax remains owned by:
 *
 *     grammar/effects/effect-handling.g4
 *
 * A handler may semantically handle a quantum effect, but handler syntax is
 * not duplicated here.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT OPERATIONS
 * ============================================================================
 *
 * Operation invocation remains owned by:
 *
 *     grammar/effects/effect-operations.g4
 *
 * This grammar contributes only the effect identity where a consumer needs
 * it.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM GRAMMAR
 * ============================================================================
 *
 * Quantum computational syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * That subsystem owns concepts such as:
 *
 *     qubits;
 *     operations;
 *     measurements;
 *     circuits;
 *     states;
 *     observables;
 *     channels;
 *     dynamic control.
 *
 * This file MUST NOT import the quantum computational grammar.
 *
 * The relationship is:
 *
 *     QuantumEffects
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     quantum semantic domain
 *
 * not:
 *
 *     QuantumEffects
 *          |
 *          v
 *     Quantum
 *          |
 *          v
 *     QuantumEffects
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * A quantum effect may produce semantic resource requirements, but this
 * grammar does not parse those requirements.
 *
 * Example semantic relationship:
 *
 *     quantum::measurement
 *          |
 *          +--> capability requirement
 *          |
 *          +--> resource requirement
 *          |
 *          +--> effect semantics
 *
 * ============================================================================
 * INTEGRATION WITH CAPABILITIES
 * ============================================================================
 *
 * Capability syntax remains owned by the capability/resource/security
 * subsystem.
 *
 * This grammar does not assume that:
 *
 *     quantum::measurement
 *
 * is itself a capability.
 *
 * The semantic model determines the required capability set.
 *
 * ============================================================================
 * INTEGRATION WITH POLICIES
 * ============================================================================
 *
 * Policies remain owned by the policy subsystem.
 *
 * A policy may:
 *
 *     permit
 *     forbid
 *     constrain
 *     prefer
 *     require
 *     audit
 *
 * a quantum effect.
 *
 * This grammar does not encode policy semantics.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * The parsed reference must remain source-locatable so provenance can record:
 *
 *     source occurrence
 *     resolved effect
 *     semantic transformation
 *     verification
 *     lowering
 *
 * This grammar does not create provenance records.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
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
 * Also forbidden:
 *
 *     fixed physical qubit identifiers;
 *     vendor backend names;
 *     fixed topology names;
 *     fixed calibration identifiers;
 *     finite quantum-effect catalogues;
 *     target-specific parser branches.
 *
 * ============================================================================
 * PRODUCTION QUALITY RULE
 * ============================================================================
 *
 * This file intentionally has a SMALL public rule surface.
 *
 * The following anti-pattern is prohibited:
 *
 *     one alias rule per semantic context.
 *
 * For example, the following must NOT be recreated:
 *
 *     quantumEffectDeclarationReference
 *     quantumEffectHandlerReference
 *     quantumEffectInvocationTarget
 *     quantumEffectMetadataTarget
 *     quantumEffectSymbol
 *     quantumEffectReferenceAlias
 *
 * when they all consume exactly the same syntax.
 *
 * Their distinctions belong to the consuming grammar or semantic model.
 *
 * Keeping one canonical reference rule prevents parse-tree fragmentation and
 * reduces downstream AST maintenance.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] The grammar identity is QuantumEffects.
 *
 * [x] The public lexer vocabulary is ZamaniLexer.
 *
 * [x] Identifier syntax comes from Names.
 *
 * [x] `quantum::` is the canonical namespace boundary.
 *
 * [x] The quantum effect path is open-world.
 *
 * [x] There is no finite quantum-effect catalogue.
 *
 * [x] There are no machine-capacity constants.
 *
 * [x] There is no physical-hardware selection.
 *
 * [x] There is no QEC implementation.
 *
 * [x] There is no ZQN implementation.
 *
 * [x] There is no routing.
 *
 * [x] There is no scheduling.
 *
 * [x] There is no quantum::ir dependency.
 *
 * [x] There is no Rust action.
 *
 * [x] There is no unsafe Rust requirement.
 *
 * [x] General effect references remain owned by EffectSets.
 *
 * [x] Effect declarations remain owned by EffectDeclarations.
 *
 * [x] Effect operations remain owned by EffectOperations.
 *
 * [x] Effect handlers remain owned by EffectHandling.
 *
 * [x] Resource semantics remain downstream.
 *
 * [x] Capability semantics remain downstream.
 *
 * [x] Target realization remains downstream.
 *
 * [x] Source structure can be preserved for AST construction.
 *
 * [x] The grammar can accept future quantum effect names without modification.
 *
 * [x] The grammar has explicit integration and dependency contracts.
 *
 * [ ] Generated parser conformance tests pass.
 *
 * [ ] Rust frontend AST lowering tests pass.
 *
 * [ ] Semantic effect-resolution tests pass.
 *
 * [ ] Cross-domain quantum/effect tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * The final unchecked items are repository/build verification gates, not
 * additional syntax that belongs in this file.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar QuantumEffects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. CANONICAL QUANTUM EFFECT REFERENCE
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::reset
 *     quantum::dynamic_control
 *
 * The first component is always the reserved quantum namespace.
 *
 * The remainder is an open-world qualified effect path.
 */
quantumEffectReference
    : K_QUANTUM
      DOUBLE_COLON
      quantumEffectPath
    ;


/*
 * ============================================================================
 * 2. OPEN-WORLD QUANTUM EFFECT PATH
 * ============================================================================
 *
 * Examples:
 *
 *     measurement
 *     readout
 *     future::operation
 *     photonic::interaction
 *     vendor::extension::operation
 *
 * There is deliberately no fixed maximum path depth.
 */
quantumEffectPath
    : quantumEffectPathSegment
      (
          DOUBLE_COLON
          quantumEffectPathSegment
      )*
    ;


/*
 * ============================================================================
 * 3. PATH SEGMENT
 * ============================================================================
 *
 * A path segment is an ordinary Zamani identifier.
 *
 * Semantic analysis determines whether the resulting path identifies:
 *
 *     - a built-in effect;
 *     - a declared effect;
 *     - an imported effect;
 *     - a dialect extension;
 *     - a library-provided effect;
 *     - a future effect;
 *     - an unresolved name.
 */
quantumEffectPathSegment
    : identifier
    ;


/*
 * ============================================================================
 * 4. QUANTUM EFFECT REFERENCE LIST
 * ============================================================================
 *
 * This is a reusable non-empty list.
 *
 * Example:
 *
 *     quantum::measurement,
 *     quantum::readout,
 *     quantum::reset,
 *
 * A trailing comma is intentionally accepted.
 *
 * This rule does not replace the general effect-set grammar.
 */
quantumEffectReferenceList
    : quantumEffectReference
      (
          COMMA
          quantumEffectReference
      )*
      COMMA?
    ;