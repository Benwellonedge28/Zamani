/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/effects/randomness.g4
 *
 * Grammar:
 *     RandomnessEffects
 *
 * Status:
 *     CANONICAL MODULAR RANDOMNESS-EFFECT INTEGRATION GRAMMAR
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no entropy acquisition;
 *     - no random-number generation;
 *     - no target selection.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level integration boundary for the
 * RANDOMNESS EFFECT DOMAIN.
 *
 * It provides thin, reusable wrappers around Zamani's generic effect
 * machinery so that semantic tooling can identify constructs that are intended
 * to participate in randomness-related effect analysis.
 *
 * This file does NOT implement randomness.
 *
 * It does NOT generate random values.
 *
 * It does NOT select an RNG.
 *
 * It does NOT select an entropy source.
 *
 * It does NOT select a hardware device.
 *
 * It does NOT define a cryptographic algorithm.
 *
 * It does NOT define a pseudorandom algorithm.
 *
 * It does NOT define a probability distribution.
 *
 * It does NOT define a seed width.
 *
 * It does NOT define an entropy-pool size.
 *
 * It does NOT define a random-state representation.
 *
 * Those concerns belong to semantic analysis, libraries, security policy,
 * execution, resources, capabilities, and target realization.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY randomness-domain parser integration wrappers:
 *
 *     randomnessOperationReference
 *     randomnessOperationInvocation
 *     randomnessOperationUse
 *     randomnessEffectReference
 *     randomnessEffectReferenceList
 *     randomnessEffectSet
 *
 * These wrappers provide stable domain boundaries for:
 *
 *     - parser listeners;
 *     - AST conversion;
 *     - semantic classification;
 *     - conformance tooling;
 *     - effect analysis;
 *     - documentation;
 *     - future domain-specific extensions.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectInvocation
 *     effectOperationUse
 *     perform syntax
 *     effect handlers
 *     expressions
 *     statements
 *     identifiers
 *     qualified names
 *     types
 *     probabilities
 *     distributions
 *     AI semantics
 *     quantum operations
 *     quantum measurement
 *     cryptographic algorithms
 *     entropy collection
 *     operating-system randomness APIs
 *     hardware RNGs
 *     QPU randomness
 *     physical noise
 *     scheduling
 *     resource allocation
 *     capability discovery
 *     security enforcement
 *     policy enforcement
 *     runtime execution
 *     target selection
 *     backend selection
 *     canonical IR implementation
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The production pipeline is:
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
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> effect classification
 *          +--> randomness analysis
 *          +--> determinism analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> policy analysis
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> other domain representations
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     execution planning
 *          |
 *          v
 *     target realization
 *
 * This grammar remains entirely above semantic realization.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Generic effect syntax is owned by:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effects.g4
 *
 * Therefore this file MUST NOT redefine generic effect rules.
 *
 * In particular, this file MUST NOT redefine:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectInvocationArguments
 *     effectOperationCall
 *     effectOperationUse
 *     performEffectOperation
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectDeclaration
 *     effectHandler
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports only generic grammar boundaries that it actually
 * reuses.
 *
 * Dependency direction:
 *
 *     RandomnessEffects
 *          |
 *          +--> Core
 *          +--> Expressions
 *          +--> EffectOperations
 *          +--> EffectSets
 *
 * No generic effect grammar may import this grammar.
 *
 * The dependency graph therefore remains acyclic.
 *
 * Canonical architecture:
 *
 *     Effects
 *        |
 *        +--> generic effect grammars
 *
 *     RandomnessEffects
 *        |
 *        +--> generic effect grammars
 *
 * There MUST NOT be:
 *
 *     Effects -> RandomnessEffects -> Effects
 *
 * ============================================================================
 * OPEN-WORLD RANDOMNESS MODEL
 * ============================================================================
 *
 * Randomness-related operations MUST NOT be enumerated here.
 *
 * The grammar therefore MUST NOT contain a finite catalogue such as:
 *
 *     random
 *     rand
 *     randomInt
 *     randomFloat
 *     randomBytes
 *     seed
 *     entropy
 *     secureRandom
 *     nondeterministic
 *     pseudoRandom
 *
 * Those concepts may exist as:
 *
 *     library operations
 *     effect declarations
 *     capabilities
 *     policies
 *     dialects
 *     semantic operations
 *     vendor extensions
 *     future domain extensions
 *
 * They are not a closed universal grammar vocabulary.
 *
 * Examples of open-world operation identities include:
 *
 *     randomness::sample
 *     randomness::draw
 *     randomness::entropy
 *     randomness::seed
 *     randomness::stream::next
 *     randomness::source::acquire
 *     cryptography::entropy
 *     simulation::random
 *     quantum::measurement
 *     vendor::randomness::operation
 *     future::randomness::operation
 *
 * The parser accepts qualified identities.
 *
 * Semantic analysis determines what those identities actually mean.
 *
 * ============================================================================
 * RANDOMNESS IS NOT ONE THING
 * ============================================================================
 *
 * Semantic analysis MUST distinguish at least the relevant categories required
 * by the language specification and implementation:
 *
 *     pseudorandom generation
 *     externally sourced entropy
 *     physical randomness
 *     cryptographic randomness
 *     deterministic seeded generation
 *     nondeterministic execution
 *     stochastic simulation
 *     probabilistic computation
 *     quantum measurement outcomes
 *     scheduler nondeterminism
 *     environmental nondeterminism
 *
 * These concepts MUST NOT be collapsed merely because they all involve the
 * word "randomness".
 *
 * In particular:
 *
 *     randomness
 *
 * MUST NOT automatically mean:
 *
 *     nondeterminism
 *
 * and:
 *
 *     nondeterminism
 *
 * MUST NOT automatically mean:
 *
 *     cryptographic randomness.
 *
 * Their semantic relationships are established downstream.
 *
 * ============================================================================
 * DETERMINISM BOUNDARY
 * ============================================================================
 *
 * Randomness has a direct relationship with reproducibility.
 *
 * The grammar itself MUST remain deterministic.
 *
 * Parsing MUST NOT:
 *
 *     generate random values;
 *     inspect an entropy source;
 *     inspect the operating system;
 *     inspect hardware;
 *     query a random device;
 *     inspect scheduler state;
 *     inspect wall-clock time;
 *     choose a seed;
 *     mutate global random state.
 *
 * A source program containing a randomness effect is still parsed entirely
 * from source text.
 *
 * Whether its execution is:
 *
 *     deterministic
 *     reproducible
 *     stochastic
 *     nondeterministic
 *     cryptographically unpredictable
 *
 * is a semantic/execution property.
 *
 * ============================================================================
 * SEEDING MODEL
 * ============================================================================
 *
 * Seed syntax is NOT defined as a special universal grammar construct here.
 *
 * A seed may be represented by:
 *
 *     an ordinary value;
 *     an operation argument;
 *     a configuration value;
 *     a capability;
 *     a policy;
 *     a resource;
 *     an execution parameter;
 *     a library abstraction.
 *
 * This prevents the language from imposing a fixed seed representation or
 * width.
 *
 * For example, the semantic system may support a reproducible operation such
 * as:
 *
 *     randomness::sample(seed, distribution)
 *
 * without this grammar having to know:
 *
 *     seed width;
 *     RNG algorithm;
 *     internal state size;
 *     number of generated values;
 *     hardware implementation.
 *
 * ============================================================================
 * CRYPTOGRAPHIC RANDOMNESS BOUNDARY
 * ============================================================================
 *
 * Cryptographic randomness is NOT equivalent to ordinary stochastic
 * computation.
 *
 * If an operation requires cryptographic entropy, semantic analysis and
 * security policy MUST be able to distinguish it from ordinary pseudorandom
 * generation.
 *
 * This grammar does NOT define:
 *
 *     a cryptographic RNG;
 *     a cryptographic algorithm;
 *     an entropy quality threshold;
 *     a security level;
 *     a hardware entropy device;
 *     an operating-system API.
 *
 * Such requirements belong to:
 *
 *     security
 *     capabilities
 *     policies
 *     resources
 *     semantic validation
 *     execution
 *
 * ============================================================================
 * QUANTUM RANDOMNESS BOUNDARY
 * ============================================================================
 *
 * Quantum measurement can produce probabilistic outcomes, but this grammar
 * does NOT redefine quantum measurement.
 *
 * Quantum syntax remains owned by the quantum grammar subsystem.
 *
 * If semantic analysis determines that:
 *
 *     quantum::measurement
 *
 * produces an outcome whose distribution is physically stochastic, the
 * resulting semantic model may interact with randomness analysis.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * This file MUST NOT create:
 *
 *     RandomQuantumIR
 *     QuantumRandomIR
 *     RandomnessIR
 *     RandomQubitIR
 *
 * or any competing quantum representation.
 *
 * The pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
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
 *
 * ============================================================================
 * SIMULATION BOUNDARY
 * ============================================================================
 *
 * Randomness used by simulation is not automatically equivalent to physical
 * randomness.
 *
 * A simulator may use:
 *
 *     deterministic seeded generation;
 *     reproducible streams;
 *     stochastic models;
 *     controlled perturbations;
 *     probabilistic sampling;
 *     externally supplied entropy.
 *
 * Those choices belong to execution semantics.
 *
 * The grammar only identifies the relevant effect-domain boundary.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT
 * ------
 *
 * Describes that a computation may consume, produce, depend upon, or expose
 * randomness-related behavior.
 *
 * CAPABILITY
 * ----------
 *
 * Describes what the realization is capable of providing.
 *
 * Possible semantic capabilities include, without being a closed list:
 *
 *     randomness
 *     reproducible-randomness
 *     entropy
 *     cryptographic-entropy
 *     quantum-randomness
 *     stochastic-simulation
 *
 * Capability names remain open-world.
 *
 * RESOURCE
 * --------
 *
 * Describes computational or environmental resources involved in realization.
 *
 * Examples may include:
 *
 *     entropy availability
 *     randomness throughput
 *     execution state
 *     memory
 *     storage
 *     accelerator capability
 *
 * No fixed numeric limits belong in this grammar.
 *
 * REQUIREMENT
 * -----------
 *
 * Specifies what must be available for realization.
 *
 * CONSTRAINT
 * ----------
 *
 * Specifies what a valid realization must obey.
 *
 * PREFERENCE
 * ----------
 *
 * Specifies which valid realization is preferred.
 *
 * POLICY
 * ------
 *
 * Governs permitted randomness sources and execution behavior.
 *
 * The grammar does not define these constructs.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A randomness effect expresses computational intent.
 *
 * It MUST NOT encode a physical implementation.
 *
 * The same source-level semantic request may be realized through:
 *
 *     deterministic simulation;
 *     pseudorandom generation;
 *     hardware entropy;
 *     operating-system entropy;
 *     distributed entropy;
 *     accelerator facilities;
 *     quantum measurement;
 *     future computational mechanisms.
 *
 * The realization is selected only after:
 *
 *     semantic analysis
 *     capability negotiation
 *     resource analysis
 *     security analysis
 *     policy analysis
 *     execution planning
 *
 * Therefore adding a new:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     entropy source
 *     operating system
 *     runtime
 *     cloud platform
 *     distributed platform
 *
 * MUST NOT require this grammar to change merely because the realization
 * mechanism changed.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO artificial capacity limits.
 *
 * It MUST NOT define:
 *
 *     MAX_RANDOM_VALUES
 *     MAX_RANDOM_OPERATIONS
 *     MAX_RANDOM_STREAMS
 *     MAX_ENTROPY
 *     MAX_RANDOMNESS_SOURCES
 *     MAX_SEED_SIZE
 *     MAX_RNG_STATE
 *     MAX_DISTRIBUTIONS
 *     MAX_SAMPLES
 *     MAX_PROBABILITY_STATES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * or equivalent artificial limits.
 *
 * There is no language-level fixed limit on:
 *
 *     number of randomness operations;
 *     number of randomness domains;
 *     number of effect references;
 *     number of effect-set entries;
 *     number of operation arguments;
 *     qualified-name depth;
 *     number of stochastic computations;
 *     number of samples;
 *     number of random streams;
 *     program size;
 *     machine size;
 *     distributed scale.
 *
 * "Infinity" means that the language grammar introduces no artificial ceiling.
 *
 * It does NOT claim that physical hardware, memory, storage, compiler
 * resources, or execution time are physically infinite.
 *
 * ============================================================================
 * DOMAIN EXTENSIBILITY
 * ============================================================================
 *
 * New randomness mechanisms MUST normally integrate through:
 *
 *     qualified names;
 *     effect declarations;
 *     capabilities;
 *     resources;
 *     requirements;
 *     constraints;
 *     policies;
 *     dialects;
 *     libraries;
 *     semantic registrations.
 *
 * Examples:
 *
 *     randomness::sample
 *     randomness::stream::next
 *     cryptography::entropy
 *     simulation::stochastic_step
 *     quantum::measurement
 *     vendor::entropy::acquire
 *     future::randomness::operation
 *
 * No modification to this grammar is required merely because a new semantic
 * randomness mechanism is introduced.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The domain-neutral AST MUST preserve enough information for downstream
 * semantic analysis, including as applicable:
 *
 *     source span;
 *     source ordering;
 *     qualified operation identity;
 *     operation arguments;
 *     effect identity;
 *     effect-set membership;
 *     source metadata.
 *
 * The AST MUST NOT require:
 *
 *     RNG implementation objects;
 *     entropy handles;
 *     operating-system handles;
 *     hardware RNG IDs;
 *     device IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     backend IDs;
 *     machine IDs.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for:
 *
 *     1. name resolution;
 *     2. operation resolution;
 *     3. randomness-effect classification;
 *     4. argument/type checking;
 *     5. effect checking;
 *     6. determinism analysis;
 *     7. reproducibility analysis;
 *     8. capability checking;
 *     9. resource analysis;
 *    10. requirement checking;
 *    11. constraint checking;
 *    12. security analysis;
 *    13. policy checking;
 *    14. provenance construction;
 *    15. target-independent validation;
 *    16. canonical semantic lowering.
 *
 * The parser MUST NOT perform these operations.
 *
 * ============================================================================
 * DETERMINISM AND REPRODUCIBILITY
 * ============================================================================
 *
 * Randomness-aware semantic analysis MUST be able to distinguish:
 *
 *     reproducible randomness
 *
 * from:
 *
 *     externally sourced randomness
 *
 * and:
 *
 *     nondeterministic execution.
 *
 * A reproducible stochastic computation may depend on explicit source-level
 * inputs or execution parameters while remaining reproducible under an
 * appropriate execution contract.
 *
 * This distinction belongs downstream.
 *
 * The grammar MUST NOT secretly introduce:
 *
 *     implicit global seed;
 *     hidden RNG;
 *     hidden entropy source;
 *     hidden timestamp;
 *     hidden scheduler state.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * When randomness participates in a semantic computation, downstream
 * provenance may need to preserve information such as:
 *
 *     randomness source;
 *     declared operation;
 *     execution mode;
 *     reproducibility contract;
 *     seed provenance where applicable;
 *     policy;
 *     capability;
 *     semantic transformation;
 *     verification information.
 *
 * The grammar does not construct provenance objects.
 *
 * It only preserves the syntax required for downstream analysis.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a randomness construct MUST NEVER acquire entropy or execute a
 * randomness operation.
 *
 * In particular, parsing:
 *
 *     randomness::sample(value)
 *
 * MUST NOT:
 *
 *     access an operating-system entropy device;
 *     invoke a hardware RNG;
 *     invoke a cryptographic provider;
 *     access a QPU;
 *     inspect system state;
 *     generate a random number;
 *     modify random state.
 *
 * Security-sensitive randomness requirements are validated downstream.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * An invalid randomness operation is not represented by a fixed parser
 * catalogue.
 *
 * Semantic analysis may report:
 *
 *     unresolved operation;
 *     invalid operation declaration;
 *     incompatible argument type;
 *     missing capability;
 *     unavailable entropy source;
 *     prohibited randomness source;
 *     insufficient resource;
 *     incompatible reproducibility contract;
 *     incompatible security policy;
 *     unsupported target realization.
 *
 * These are semantic or execution errors, not keyword-enumeration errors.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Randomness operations may require capabilities.
 *
 * Capability syntax remains owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Requirement syntax remains owned by:
 *
 *     grammar/core/requirements.g4
 *
 * Resource-specific semantics remain owned by:
 *
 *     grammar/resources/
 *
 * This grammar does not redefine any of them.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Randomness may have resource implications.
 *
 * Examples include:
 *
 *     entropy availability;
 *     computation;
 *     memory;
 *     storage;
 *     throughput;
 *     latency;
 *     energy;
 *     accelerator availability.
 *
 * These are semantic resource properties.
 *
 * This grammar MUST NOT encode fixed resource quantities.
 *
 * It MUST NOT prescribe universal values for:
 *
 *     seed size;
 *     entropy size;
 *     state size;
 *     sample count;
 *     throughput;
 *     memory;
 *     storage.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may govern randomness behavior, including:
 *
 *     permitted source classes;
 *     cryptographic requirements;
 *     reproducibility requirements;
 *     simulation restrictions;
 *     provenance requirements;
 *     external-entropy restrictions;
 *     execution restrictions.
 *
 * Policy syntax is owned elsewhere.
 *
 * This file provides only the randomness effect boundary.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Randomness may participate in:
 *
 *     classical computation;
 *     probabilistic computation;
 *     AI/ML;
 *     simulation;
 *     quantum computation;
 *     hybrid computation;
 *     distributed computation;
 *     cryptography;
 *     security;
 *     hardware/software co-design;
 *     HDL simulation;
 *     accelerator computation;
 *     future computational domains.
 *
 * The generic effect model allows those domains to coexist.
 *
 * Example semantic composition:
 *
 *     randomness
 *         +
 *     classical
 *
 * or:
 *
 *     randomness
 *         +
 *     quantum::measurement
 *         +
 *     classical control
 *
 * or:
 *
 *     randomness
 *         +
 *     simulation
 *         +
 *     reproducibility policy
 *
 * No domain-specific second effect language is introduced.
 *
 * ============================================================================
 * ANTLR GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar RandomnessEffects;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    EffectOperations,
    EffectSets
;


/*
 * ============================================================================
 * RANDOMNESS OPERATION REFERENCE
 * ============================================================================
 *
 * Reuses the canonical generic effect-operation reference.
 *
 * Examples:
 *
 *     randomness::sample
 *     randomness::draw
 *     randomness::entropy
 *     randomness::stream::next
 *     cryptography::entropy
 *     simulation::random
 *     vendor::randomness::operation
 *     future::randomness::operation
 *
 * The parser does not determine whether a resolved operation is actually a
 * randomness operation.
 *
 * Semantic analysis performs domain classification.
 */

randomnessOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================
 * RANDOMNESS OPERATION INVOCATION
 * ============================================================================
 *
 * Reuses canonical effect invocation syntax.
 *
 * Examples:
 *
 *     randomness::sample(value)
 *     randomness::draw(distribution)
 *     randomness::stream::next(stream)
 *     cryptography::entropy(request)
 *
 * Argument syntax remains owned by the expression subsystem.
 */

randomnessOperationInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * RANDOMNESS OPERATION USE
 * ============================================================================
 *
 * Stable domain wrapper around the canonical effect-operation-use construct.
 *
 * The wrapper exists for semantic tooling and AST conversion without creating
 * a competing operation language.
 */

randomnessOperationUse
    : effectOperationUse
    ;


/*
 * ============================================================================
 * RANDOMNESS EFFECT REFERENCE
 * ============================================================================
 *
 * Reuses the canonical effect-reference syntax.
 *
 * Examples:
 *
 *     randomness
 *     randomness::stochastic
 *     randomness::reproducible
 *     cryptography::entropy
 *
 * Semantic analysis determines the actual effect classification.
 */

randomnessEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * RANDOMNESS EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Reuses the canonical unbounded effect-reference list.
 *
 * Examples:
 *
 *     randomness
 *
 *     randomness, simulation
 *
 *     randomness::reproducible, data::transform
 *
 *     quantum::measurement, randomness::stochastic
 *
 * No fixed number of entries exists.
 */

randomnessEffectReferenceList
    : effectReferenceList
    ;


/*
 * ============================================================================
 * RANDOMNESS EFFECT SET
 * ============================================================================
 *
 * Reuses the canonical effect-set representation.
 *
 * Example:
 *
 *     {
 *         randomness::sample,
 *         simulation::stochastic_step
 *     }
 *
 * The entries receive their semantic meaning only during downstream
 * analysis.
 */

randomnessEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. The canonical lexer is ZamaniLexer.
 *
 * 2. Core owns:
 *
 *        identifier
 *        qualifiedName
 *        paths
 *        punctuation
 *        common source constructs
 *
 * 3. Expressions owns:
 *
 *        expression
 *        argument syntax
 *
 * 4. EffectOperations owns:
 *
 *        effectOperationReference
 *        effectInvocation
 *        effectOperationUse
 *
 * 5. EffectSets owns:
 *
 *        effectReference
 *        effectReferenceList
 *        effectSet
 *
 * 6. This file owns only randomness-prefixed integration wrappers.
 *
 * 7. The generic effect composition root remains:
 *
 *        grammar/effects/effects.g4
 *
 * 8. The generic effect composition root MUST remain domain-neutral.
 *
 * 9. This grammar MUST NOT be imported by the generic effect composition root
 *    if doing so would create a dependency cycle.
 *
 * 10. Statement-level effect syntax remains owned by:
 *
 *        grammar/statements/effects.g4
 *
 * 11. Expression-level effect syntax remains owned by:
 *
 *        grammar/expressions/effects.g4
 *
 * 12. Randomness classification occurs in semantic analysis.
 *
 * 13. Capability/resource/policy/security analysis occurs downstream.
 *
 * 14. Canonical IR lowering occurs downstream.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     Core
 *     Expressions
 *     EffectOperations
 *     EffectSets
 *
 * EXPORTS:
 *
 *     randomnessOperationReference
 *     randomnessOperationInvocation
 *     randomnessOperationUse
 *     randomnessEffectReference
 *     randomnessEffectReferenceList
 *     randomnessEffectSet
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     semantic effect analysis
 *     randomness/determinism analysis
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/spec/determinism.md
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *     classical IR where applicable
 *     quantum::ir where quantum semantics are involved
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/determinism/
 *
 * CONSUMED_BY:
 *
 *     semantic analysis
 *     conformance tooling
 *     AST conversion
 *     effect analysis
 *     determinism analysis
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It is an ANTLR4 parser grammar.
 *
 * [x] It uses ZamaniLexer.
 *
 * [x] It contains no lexer rules.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It contains no unsafe implementation requirement.
 *
 * [x] It does not execute randomness.
 *
 * [x] It does not acquire entropy.
 *
 * [x] It does not inspect hardware.
 *
 * [x] It does not inspect the operating system.
 *
 * [x] It does not select a runtime.
 *
 * [x] It does not select a target.
 *
 * [x] It does not redefine generic effect syntax.
 *
 * [x] It does not enumerate random operations.
 *
 * [x] It does not impose a fixed RNG model.
 *
 * [x] It does not impose a fixed entropy model.
 *
 * [x] It does not impose a fixed seed representation.
 *
 * [x] It does not impose fixed probability/distribution limits.
 *
 * [x] It distinguishes grammar from semantic randomness behavior.
 *
 * [x] It preserves the determinism boundary.
 *
 * [x] It preserves the quantum::ir boundary.
 *
 * [x] It remains open-world.
 *
 * [x] It introduces no machine-capacity constants.
 *
 * [x] It can participate in classical, quantum, hybrid, HDL, AI, distributed,
 *     networking, cryptographic, accelerator, embedded, and future-domain
 *     programs without requiring a new randomness grammar for every target.
 *
 * ============================================================================
 * REQUIRED POSITIVE CONFORMANCE TESTS
 * ============================================================================
 *
 * Operation references:
 *
 *     randomness::sample
 *     randomness::draw
 *     randomness::entropy
 *     randomness::stream::next
 *     cryptography::entropy
 *     simulation::random
 *     vendor::randomness::operation
 *     future::randomness::operation
 *
 * Operation invocations:
 *
 *     randomness::sample(value)
 *     randomness::draw(distribution)
 *     randomness::entropy(request)
 *     randomness::stream::next(stream)
 *     cryptography::entropy(request)
 *
 * Effect references:
 *
 *     randomness
 *     randomness::stochastic
 *     randomness::reproducible
 *
 * Effect sets:
 *
 *     {
 *         randomness::sample,
 *         simulation::stochastic_step
 *     }
 *
 * Cross-domain references:
 *
 *     {
 *         randomness::sample,
 *         quantum::measurement
 *     }
 *
 *     {
 *         randomness::sample,
 *         ai::inference,
 *         data::transform
 *     }
 *
 * ============================================================================
 * REQUIRED NEGATIVE / SEMANTIC TESTS
 * ============================================================================
 *
 * The following MUST be tested semantically rather than by adding parser
 * keyword alternatives:
 *
 *     unresolved randomness operation;
 *     invalid effect identity;
 *     incompatible argument type;
 *     missing randomness capability;
 *     unavailable entropy capability;
 *     prohibited randomness policy;
 *     cryptographic-randomness requirement not satisfied;
 *     reproducibility requirement not satisfied;
 *     invalid effect context;
 *     invalid operation declaration.
 *
 * The parser MUST remain open-world.
 *
 * ============================================================================
 * REQUIRED DETERMINISM TESTS
 * ============================================================================
 *
 * Verify that parsing is independent of:
 *
 *     operating-system entropy;
 *     hardware RNG state;
 *     system time;
 *     scheduler state;
 *     process identity;
 *     thread identity;
 *     environment state;
 *     network state.
 *
 * Repeated parsing of identical source MUST produce equivalent parser
 * structure under the same grammar/toolchain version.
 *
 * ============================================================================
 * REQUIRED SCALABILITY TESTS
 * ============================================================================
 *
 * Tests MUST cover generated inputs containing:
 *
 *     deeply qualified operation names;
 *     large effect sets;
 *     large argument lists;
 *     many randomness operations;
 *     many independent stochastic computations;
 *     nested effect constructs;
 *     mixed randomness and classical computation;
 *     mixed randomness and quantum computation;
 *     mixed randomness and simulation;
 *     distributed stochastic execution;
 *     large source programs.
 *
 * Tests MUST scale input size through generated data.
 *
 * They MUST NOT introduce grammar constants that establish a universal limit.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     fixed random-operation catalogues;
 *     fixed RNG algorithms;
 *     fixed entropy providers;
 *     fixed seed widths;
 *     fixed RNG state widths;
 *     fixed distribution counts;
 *     fixed sample counts;
 *     fixed entropy-pool sizes;
 *     fixed hardware RNG counts;
 *     fixed QPU counts;
 *     fixed device counts;
 *     fixed machine capacities.
 *
 * Also forbidden:
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
 * No equivalent artificial capacity constant may be introduced under another
 * name.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file establishes randomness as an OPEN semantic effect domain.
 *
 * The resulting architecture is:
 *
 *     source
 *       |
 *       v
 *     generic effect syntax
 *       |
 *       v
 *     randomness-domain classification
 *       |
 *       +--> determinism analysis
 *       +--> reproducibility analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> security analysis
 *       +--> policy analysis
 *       +--> provenance
 *       |
 *       v
 *     target-independent semantic representation
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical             quantum::ir           HDL/hardware
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                              v
 *                         execution plan
 *                              |
 *                              v
 *                         target realization
 *
 * A new randomness algorithm, entropy source, simulator, accelerator, QPU,
 * classical processor, distributed platform, or future computational
 * mechanism therefore does not require changing this grammar merely because
 * the realization has changed.
 *
 * That is the required POCO-REAF property.
 *
 * ============================================================================
 */