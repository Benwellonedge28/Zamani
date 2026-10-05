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
 *     CANONICAL MODULAR RANDOMNESS-EFFECT DOMAIN ADAPTER
 *
 * Technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains:
 *       - no embedded Rust;
 *       - no semantic predicates;
 *       - no executable actions;
 *       - no filesystem access;
 *       - no network access;
 *       - no hardware discovery;
 *       - no runtime execution;
 *       - no entropy acquisition;
 *       - no random-value generation;
 *       - no target selection;
 *       - no unsafe Rust requirement.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file provides the parser-level boundary for the randomness effect
 * domain.
 *
 * It is deliberately a THIN DOMAIN ADAPTER over Zamani's generic effect
 * machinery.
 *
 * It does not create a second randomness language.
 *
 * It does not enumerate randomness algorithms, providers, distributions,
 * entropy sources, generators, devices, or implementation strategies.
 *
 *
 * OWNS
 * ----
 *
 * This file owns only the following domain-labelled parser boundaries:
 *
 *     randomnessOperationReference
 *     randomnessOperationInvocation
 *     randomnessOperationUse
 *     randomnessEffectReference
 *     randomnessEffectReferenceList
 *     randomnessEffectSet
 *
 * These rules are aliases/adapters over canonical generic effect syntax.
 *
 *
 * DOES NOT OWN
 * -------------
 *
 * This file does not own:
 *
 *     effect declarations
 *     effect operation declarations
 *     generic effect references
 *     generic effect sets
 *     generic effect invocation
 *     perform syntax
 *     effect handlers
 *     expressions
 *     statements
 *     identifiers
 *     qualified names
 *     types
 *     probability semantics
 *     distribution semantics
 *     RNG algorithms
 *     pseudorandom algorithms
 *     entropy collection
 *     cryptographic algorithms
 *     security enforcement
 *     policies
 *     capabilities
 *     resources
 *     deterministic replay
 *     scheduler semantics
 *     quantum measurement
 *     quantum IR
 *     HDL semantics
 *     hardware discovery
 *     runtime dispatch
 *     target selection
 *     backend selection
 *     physical realization.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The language pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> effect classification
 *       +--> randomness classification
 *       +--> type analysis
 *       +--> determinism analysis
 *       +--> reproducibility analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> security analysis
 *       +--> policy analysis
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> other domain representation
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       v
 *     execution planning
 *       |
 *       v
 *     target realization
 *
 * This grammar participates only above semantic realization.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Generic effect syntax is owned by the existing effect subsystem:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effects.g4
 *
 * Therefore this file MUST NOT redefine any generic effect rule.
 *
 * In particular, it MUST NOT redefine:
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
 * This file only adds domain-labelled aliases.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     EffectOperations
 *     EffectSets
 *
 * DIRECT DEPENDENCIES ONLY.
 *
 * Core and Expressions are intentionally NOT imported directly here because
 * they are already dependencies of the generic grammars consumed above.
 *
 * This keeps the dependency surface minimal and prevents unnecessary
 * coupling between domain adapters and universal syntax.
 *
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
 *
 * CONSUMED_BY:
 *
 *     randomness-domain semantic classification
 *     AST conversion tooling
 *     grammar conformance tooling
 *     randomness-specific parser tests
 *     semantic effect tests
 *     determinism/reproducibility tests
 *
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 *
 * SEMANTIC_OWNER:
 *
 *     semantic effect analysis
 *     randomness classification
 *     determinism analysis
 *     reproducibility analysis
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/spec/determinism.md
 *     grammar/spec/resources.md
 *     grammar/spec/policies.md
 *     relevant specification/effects documentation
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     classical IR where applicable
 *     quantum::ir where quantum semantics are involved
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/
 *     grammar/tests/semantic/
 *     grammar/tests/determinism/
 *     grammar/tests/scalability/
 *     grammar/tests/quantum/
 *     grammar/tests/hybrid/
 *
 * ============================================================================
 * ANTLR CONFIGURATION
 * ============================================================================
 *
 * The canonical lexical vocabulary for parser grammars is:
 *
 *     ZamaniTokens
 *
 * This matches the existing generic effect-operation grammar and avoids
 * introducing a second token-vocabulary identity.
 *
 * The canonical chain is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     parser grammars
 *
 * `ZamaniLexer` remains the public lexer grammar.
 *
 * `ZamaniTokens` remains the canonical token vocabulary.
 *
 * ============================================================================
 */

parser grammar RandomnessEffects;

options {
    tokenVocab = ZamaniTokens;
}

import
    EffectOperations,
    EffectSets
;


/*
 * ============================================================================
 * RANDOMNESS OPERATION REFERENCE
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provides a stable parser-tree boundary for an operation that will be
 * classified semantically as randomness-related.
 *
 * The operation identity itself remains a normal Zamani qualified name through
 * the canonical EffectOperations grammar.
 *
 *
 * IMPORTANT
 * ---------
 *
 * This rule does NOT restrict the operation to a literal `randomness`
 * namespace.
 *
 * That is intentional.
 *
 * Randomness semantics may arise from:
 *
 *     randomness::...
 *     cryptography::...
 *     simulation::...
 *     quantum::...
 *     vendor::...
 *     future::...
 *
 * or another declared namespace.
 *
 * Domain classification is semantic, not lexical.
 */

randomnessOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================
 * RANDOMNESS OPERATION INVOCATION
 * ============================================================================
 *
 * Reuses the canonical effect invocation grammar.
 *
 * Examples of source-level identities that MAY subsequently be classified
 * semantically as randomness-related include:
 *
 *     randomness::sample(value)
 *     randomness::stream::next(stream)
 *     cryptography::entropy(request)
 *     simulation::stochastic_step(state)
 *     vendor::randomness::operation(value)
 *
 * These are examples of semantic identities, not a closed grammar catalogue.
 */

randomnessOperationInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * RANDOMNESS OPERATION USE
 * ============================================================================
 *
 * Stable adapter boundary for tools that need to identify a randomness-domain
 * operation use without creating a new operation-use language.
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
 * Semantic analysis determines whether the resolved effect belongs to the
 * randomness domain.
 *
 * This permits open-world effect namespaces and future extensions.
 */

randomnessEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * RANDOMNESS EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Reuses the canonical effect-reference list.
 *
 * No fixed cardinality is encoded.
 *
 * The generic EffectSets grammar owns:
 *
 *     list structure
 *     comma handling
 *     trailing comma behavior
 *     qualified-name syntax.
 */

randomnessEffectReferenceList
    : effectReferenceList
    ;


/*
 * ============================================================================
 * RANDOMNESS EFFECT SET
 * ============================================================================
 *
 * Reuses the canonical effect-set grammar.
 *
 * The contents are classified semantically.
 *
 * This prevents the randomness domain from creating a second effect-set
 * language.
 */

randomnessEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser recognizes structure only.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the referenced operation resolves;
 *     - whether the resolved operation is randomness-related;
 *     - whether it is pseudorandom or externally sourced;
 *     - whether it is cryptographically security-sensitive;
 *     - whether it is deterministic/reproducible;
 *     - whether it introduces execution nondeterminism;
 *     - whether it is simulation-related;
 *     - whether it is associated with quantum measurement;
 *     - whether required capabilities exist;
 *     - whether required resources exist;
 *     - whether policy permits the requested behavior;
 *     - whether provenance must be recorded;
 *     - whether the operation is legal in its effect context;
 *     - how the operation lowers into the canonical semantic representation.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * RANDOMNESS SEMANTIC CLASSES
 * ============================================================================
 *
 * The semantic model SHOULD be capable of distinguishing, where applicable:
 *
 *     deterministic pseudorandom generation
 *     reproducible stochastic computation
 *     externally sourced entropy
 *     physical entropy
 *     cryptographic randomness
 *     stochastic simulation
 *     probabilistic computation
 *     quantum measurement outcomes
 *     environmental nondeterminism
 *     scheduler nondeterminism
 *
 * These are semantic properties.
 *
 * They are NOT grammar alternatives.
 *
 * In particular:
 *
 *     randomness != automatically nondeterminism
 *
 *     nondeterminism != automatically cryptographic randomness
 *
 *     stochastic simulation != automatically physical randomness
 *
 *     quantum measurement != automatically a generic RNG operation
 *
 * ============================================================================
 * DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * This grammar itself is deterministic.
 *
 * Parsing MUST NOT:
 *
 *     generate random values;
 *     obtain entropy;
 *     select a seed;
 *     inspect hardware;
 *     inspect operating-system randomness;
 *     inspect scheduler state;
 *     inspect wall-clock time;
 *     access environment state;
 *     access network state.
 *
 * Reproducibility is established by downstream execution semantics and
 * contracts.
 *
 * A seed, when applicable, remains an ordinary semantic value or execution
 * parameter. This grammar does not define:
 *
 *     seed width;
 *     seed encoding;
 *     generator state size;
 *     algorithm;
 *     stream representation.
 *
 * ============================================================================
 * CRYPTOGRAPHIC BOUNDARY
 * ============================================================================
 *
 * Cryptographically suitable randomness is a semantic/security requirement.
 *
 * This grammar does not define:
 *
 *     cryptographic algorithms;
 *     security levels;
 *     entropy thresholds;
 *     entropy devices;
 *     operating-system APIs;
 *     hardware RNG implementations.
 *
 * Security policy, capabilities, resources, semantic validation, and execution
 * infrastructure determine whether a requested cryptographic randomness
 * operation can be realized.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum measurement remains owned by the quantum subsystem.
 *
 * This grammar MUST NOT define quantum measurement syntax merely because
 * measurement outcomes can be probabilistic.
 *
 * The semantic relationship is:
 *
 *     quantum source
 *          |
 *          v
 *     domain-neutral semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * If a quantum operation has stochastic semantics, randomness analysis may
 * consume that semantic information.
 *
 * It MUST NOT create:
 *
 *     RandomnessIR
 *     QuantumRandomIR
 *     RandomQuantumIR
 *     RandomQubitIR
 *
 * or another quantum representation.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * SIMULATION BOUNDARY
 * ============================================================================
 *
 * Simulation may require stochastic behavior without requiring physical
 * entropy.
 *
 * The execution subsystem may therefore choose an implementation such as:
 *
 *     deterministic seeded simulation
 *     reproducible stochastic simulation
 *     externally seeded simulation
 *     physical-entropy-backed simulation
 *
 * according to the semantic contract, capabilities, resources and policy.
 *
 * This grammar does not select the implementation.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT
 * ------
 *
 * Describes observable or semantically relevant computational behavior.
 *
 * CAPABILITY
 * ----------
 *
 * Describes what a realization environment can provide.
 *
 * RESOURCE
 * --------
 *
 * Describes resources involved in realizing the computation.
 *
 * REQUIREMENT
 * -----------
 *
 * Describes what a realization must satisfy.
 *
 * CONSTRAINT
 * ----------
 *
 * Describes what a valid realization must obey.
 *
 * POLICY
 * ------
 *
 * Describes permitted/restricted behavior.
 *
 * PROVENANCE
 * ----------
 *
 * Describes traceability of relevant semantic decisions and derived artifacts.
 *
 * This grammar defines none of these semantic systems.
 *
 * It only supplies parser boundaries that downstream systems can consume.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * New randomness algorithms, distributions, entropy sources, generators,
 * accelerators, simulators, quantum technologies, or vendor facilities MUST
 * NOT require a modification to this grammar merely because a new semantic
 * entity has been introduced.
 *
 * New semantic entities should normally be introduced through:
 *
 *     libraries
 *     effect declarations
 *     semantic registrations
 *     capabilities
 *     resource descriptions
 *     policies
 *     dialects
 *     interoperability contracts
 *
 * The grammar remains stable because operation identity is name-based.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no artificial limits on:
 *
 *     operation count;
 *     effect count;
 *     effect-set size;
 *     effect-reference count;
 *     argument count;
 *     qualified-name depth;
 *     stochastic computation count;
 *     random streams;
 *     samples;
 *     distributions;
 *     source-program size;
 *     machine size;
 *     execution scale.
 *
 * It MUST NOT encode implementation ceilings for:
 *
 *     random values;
 *     entropy;
 *     streams;
 *     generator state;
 *     samples;
 *     distributions;
 *     memory;
 *     processors;
 *     accelerators;
 *     devices;
 *     nodes;
 *     quantum resources;
 *     network resources.
 *
 * Practical parser/compiler/runtime limits remain implementation resource
 * policies. They are not language semantics.
 *
 * The same semantic program can therefore be considered for a wide range of
 * realizations, subject to capability/resource/policy feasibility.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no semantic AST object.
 *
 * The frontend AST must preserve the source information needed to represent:
 *
 *     operation identity;
 *     effect identity;
 *     argument expressions;
 *     source ordering;
 *     source spans;
 *     enclosing effect context.
 *
 * The AST MUST remain domain-neutral.
 *
 * It MUST NOT require:
 *
 *     RNG objects;
 *     entropy handles;
 *     hardware RNG IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     physical-device IDs;
 *     backend IDs.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The downstream path is:
 *
 *     parser structure
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic effect model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> future domain representation
 *
 * When quantum semantics are involved, the canonical quantum destination
 * remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource and capability syntax is owned elsewhere.
 *
 * Conceptually, semantic analysis may derive requirements such as:
 *
 *     required capability: randomness
 *
 *     required capability: cryptographic entropy
 *
 *     required capability: reproducible stochastic execution
 *
 *     required capability: quantum measurement
 *
 * or arbitrary future capabilities.
 *
 * It may also derive resource requirements based on the operation and its
 * semantic contract.
 *
 * This file does not encode those requirements as grammar-level constants.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Security and execution policies may constrain:
 *
 *     permitted randomness sources;
 *     reproducibility;
 *     cryptographic suitability;
 *     external entropy;
 *     simulation;
 *     provenance;
 *     replay;
 *     execution environments.
 *
 * Policies are resolved downstream.
 *
 * A policy failure is therefore not a parser failure.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Depending on semantic requirements, downstream provenance may preserve:
 *
 *     declared operation;
 *     resolved operation;
 *     source location;
 *     semantic classification;
 *     execution mode;
 *     reproducibility contract;
 *     source of entropy;
 *     seed provenance where applicable;
 *     policy decisions;
 *     capability decisions;
 *     transformations;
 *     verification information.
 *
 * This grammar preserves only the source structure required to construct that
 * information.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are restricted to malformed source structure.
 *
 * Examples:
 *
 *     malformed qualified operation;
 *     malformed invocation;
 *     malformed effect reference;
 *     malformed effect set.
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown operation;
 *     invalid operation signature;
 *     operation not classified as randomness;
 *     unavailable capability;
 *     unavailable resource;
 *     prohibited source;
 *     incompatible reproducibility contract;
 *     invalid security requirement;
 *     unsupported realization.
 *
 * Resource/capability/security/policy failures MUST NOT be converted into
 * artificial parser restrictions.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing generic effect syntax remains authoritative.
 *
 * This adapter introduces no new reserved words.
 *
 * Therefore existing operation names and effect names remain compatible with
 * the open-world effect model.
 *
 * Historical or vendor-specific names are handled through the repository's
 * compatibility/dialect architecture rather than through an ever-growing
 * randomness keyword catalogue.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following tests belong to the integration suite.
 *
 *
 * POSITIVE OPERATION REFERENCES
 * ------------------------------
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
 *
 * POSITIVE INVOCATIONS
 * --------------------
 *
 *     randomness::sample(value)
 *     randomness::draw(distribution)
 *     randomness::entropy(request)
 *     randomness::stream::next(stream)
 *     cryptography::entropy(request)
 *
 *
 * POSITIVE EFFECT REFERENCES
 * --------------------------
 *
 *     randomness
 *     randomness::stochastic
 *     randomness::reproducible
 *
 *
 * POSITIVE EFFECT SETS
 * --------------------
 *
 *     {
 *         randomness::sample,
 *         simulation::stochastic_step
 *     }
 *
 *     {
 *         quantum::measurement,
 *         randomness::stochastic
 *     }
 *
 *     {
 *         randomness::sample,
 *         ai::inference,
 *         data::transform
 *     }
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Verify composition with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI/ML
 *     distributed
 *     networking
 *     cryptography/security
 *     simulation
 *     accelerators
 *
 *
 * SEMANTIC NEGATIVE TESTS
 * -----------------------
 *
 * The parser remains open-world.
 *
 * Therefore the following are semantic tests rather than hard-coded parser
 * exclusions:
 *
 *     unresolved operation;
 *     operation with invalid signature;
 *     operation requiring unavailable capability;
 *     operation requiring unavailable resource;
 *     operation prohibited by policy;
 *     operation incompatible with reproducibility requirements;
 *     operation incompatible with security requirements.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parsing must produce equivalent parser structure for identical source under
 * identical grammar/toolchain configuration regardless of:
 *
 *     operating-system entropy state;
 *     hardware RNG state;
 *     scheduler state;
 *     system time;
 *     process identity;
 *     thread identity;
 *     network state;
 *     environment state.
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Tests should generate increasingly large valid programs and verify:
 *
 *     deeply qualified operation names;
 *     large effect sets;
 *     large invocation argument lists;
 *     many independent randomness operations;
 *     mixed stochastic and deterministic computation;
 *     mixed classical/quantum computation;
 *     distributed stochastic execution;
 *     simulation workloads;
 *     large source units.
 *
 * Test scaling is determined by the test harness and available resources,
 * not by grammar-level constants.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no fixed randomness operation catalogue;
 *     no fixed RNG algorithm list;
 *     no fixed entropy provider list;
 *     no fixed seed representation;
 *     no fixed generator-state representation;
 *     no fixed distribution catalogue;
 *     no fixed sample limit;
 *     no fixed stream limit;
 *     no hardware-capacity assumption;
 *     no target-selection rule.
 *
 * It also introduces no artificial universal resource ceilings.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING EFFECT ROOT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `grammar/effects/effects.g4` remains the generic effect composition root.
 *
 * It MUST NOT import RandomnessEffects merely to expose this adapter.
 *
 * The correct dependency direction is:
 *
 *     RandomnessEffects
 *          |
 *          +--> EffectOperations
 *          +--> EffectSets
 *
 * while:
 *
 *     Effects
 *          |
 *          +--> generic effect declarations
 *          +--> generic effect sets
 *          +--> generic effect operations
 *          +--> generic handlers
 *
 * The generic effect system therefore remains domain-neutral.
 *
 * Randomness classification happens through semantic/domain integration.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT OPERATIONS
 * ============================================================================
 *
 * `grammar/effects/effect-operations.g4` remains the sole owner of:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *
 * This file delegates to those rules.
 *
 * Therefore any future correction to generic invocation syntax automatically
 * propagates through this adapter without requiring a second implementation.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT SETS
 * ============================================================================
 *
 * `grammar/effects/effect-sets.g4` remains the sole owner of:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * This file delegates to those rules.
 *
 * Therefore changes to generic effect-set syntax remain centralized.
 *
 * ============================================================================
 * INTEGRATION WITH AI / LEARNING / ADAPTATION
 * ============================================================================
 *
 * Randomness may participate in:
 *
 *     learning;
 *     adaptation;
 *     inference;
 *     probabilistic computation;
 *     stochastic optimization;
 *     simulation;
 *     agent behavior.
 *
 * These domains remain owners of their own syntax.
 *
 * The randomness adapter does not introduce AI-specific randomness constructs.
 *
 * Example semantic relationship:
 *
 *     AI operation
 *          |
 *          +--> randomness effect
 *          |
 *          v
 *     effect analysis
 *
 * ============================================================================
 * INTEGRATION WITH DETERMINISTIC EXECUTION
 * ============================================================================
 *
 * Reproducibility is a cross-cutting execution property.
 *
 * It should integrate with:
 *
 *     execution/
 *     validation/
 *     provenance/
 *     policies/
 *
 * rather than being implemented as randomness-specific parser keywords.
 *
 * A deterministic execution contract can therefore govern a randomness
 * operation without changing this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH SECURITY
 * ============================================================================
 *
 * Security-sensitive randomness belongs at the intersection of:
 *
 *     effects
 *     capabilities
 *     resources
 *     security
 *     policies
 *     provenance
 *
 * This file does not authorize or deny such operations.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum measurement remains owned by:
 *
 *     grammar/quantum/
 *
 * Its semantic consequences may be consumed by randomness/determinism
 * analysis.
 *
 * No separate quantum-randomness grammar or IR is introduced.
 *
 * ============================================================================
 * INTEGRATION WITH HDL / HARDWARE
 * ============================================================================
 *
 * Hardware entropy sources, physical noise sources, accelerators and hardware
 * randomness facilities are represented by target capabilities and semantic
 * operation declarations.
 *
 * This grammar does not encode:
 *
 *     device IDs;
 *     physical addresses;
 *     hardware resource counts;
 *     register widths;
 *     bus widths;
 *     topology;
 *     placement;
 *     routing;
 *     clock implementation.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED COMPUTATION
 * ============================================================================
 *
 * Distributed stochastic computation is represented through composition of:
 *
 *     randomness effects
 *     concurrency semantics
 *     distributed semantics
 *     networking semantics
 *     resource/capability constraints
 *     reproducibility policy.
 *
 * This file does not encode process, node, worker, stream, or device counts.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * ANTLR generation MUST resolve:
 *
 *     RandomnessEffects
 *          |
 *          +--> EffectOperations
 *          +--> EffectSets
 *
 * and their transitive dependencies.
 *
 * The build system is responsible for supplying the canonical grammar library
 * path.
 *
 * No embedded build commands or target-specific actions belong here.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It is a parser grammar.
 *
 * [x] Its grammar identity is RandomnessEffects.
 *
 * [x] It uses the canonical ZamaniTokens vocabulary.
 *
 * [x] It imports only the generic effect grammars it directly consumes.
 *
 * [x] It does not directly import Core or Expressions unnecessarily.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not define semantic predicates.
 *
 * [x] It does not embed Rust.
 *
 * [x] It does not require unsafe Rust.
 *
 * [x] It does not execute randomness.
 *
 * [x] It does not acquire entropy.
 *
 * [x] It does not inspect hardware.
 *
 * [x] It does not inspect runtime state.
 *
 * [x] It does not select a target.
 *
 * [x] It does not enumerate randomness operations.
 *
 * [x] It does not enumerate RNG algorithms.
 *
 * [x] It does not enumerate distributions.
 *
 * [x] It does not define a seed width.
 *
 * [x] It does not define generator-state size.
 *
 * [x] It does not define entropy-pool size.
 *
 * [x] It does not define machine-capacity limits.
 *
 * [x] It preserves the generic effect ownership model.
 *
 * [x] It preserves the open-world effect model.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * [x] It preserves the deterministic parser boundary.
 *
 * [x] It preserves source-level portability.
 *
 * [x] It remains suitable for future randomness mechanisms without grammar
 *     modification.
 *
 * Repository-level verification still required:
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] All transitive imports resolve.
 *
 * [ ] No imported-rule collisions occur.
 *
 * [ ] Parser integration succeeds.
 *
 * [ ] Rust frontend integration succeeds on Rust 1.97 / 1.97.1.
 *
 * [ ] Positive conformance tests pass.
 *
 * [ ] Negative semantic tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass within available implementation resources.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Randomness is a semantic effect domain, not a closed grammar catalogue.
 *
 * The stable architecture is:
 *
 *     generic effect syntax
 *          |
 *          v
 *     randomness adapter
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic classification
 *          |
 *          +--> determinism
 *          +--> reproducibility
 *          +--> security
 *          +--> capabilities
 *          +--> resources
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     target realization
 *
 * A new randomness algorithm, distribution, entropy source, simulator,
 * accelerator, processor, QPU, distributed platform, or future computational
 * mechanism therefore does not require changing this grammar merely because
 * the implementation has changed.
 *
 * This is the required scalable, target-independent effect boundary.
 *
 * ============================================================================
 */