/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effects.g4
 *
 * Grammar identity:
 *     Effects
 *
 * Role:
 *     CANONICAL EFFECT-SUBSYSTEM ORCHESTRATOR
 *
 * Status:
 *     PRODUCTION ARCHITECTURE
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - parser grammar only;
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no randomness;
 *     - no target probing.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE CANONICAL ORCHESTRATOR for the effect subsystem.
 *
 * It is intentionally NOT a monolithic effect grammar.
 *
 * Its responsibilities are:
 *
 *     1. establish the Effects parser grammar;
 *     2. consume the canonical lexer vocabulary;
 *     3. compose the generic effect-language modules;
 *     4. expose stable public integration rules;
 *     5. provide cross-domain effect dispatch;
 *     6. preserve open-world effect identity;
 *     7. keep domain-specific effect families separate from generic syntax;
 *     8. prevent dependency cycles;
 *     9. provide one parser boundary to ZamaniParser;
 *    10. preserve the POCO-REAF architecture.
 *
 * It MUST NOT:
 *
 *     - define lexer rules;
 *     - enumerate effect kinds;
 *     - enumerate effect operations;
 *     - select hardware;
 *     - allocate resources;
 *     - discover capabilities;
 *     - perform effect inference;
 *     - perform effect checking;
 *     - perform security authorization;
 *     - perform runtime dispatch;
 *     - define quantum IR;
 *     - define physical qubits;
 *     - define routing;
 *     - define scheduling;
 *     - define QEC;
 *     - define ZQN;
 *     - define HAL behavior.
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * OWNS
 * -----
 *
 *     Effects parser identity
 *     generic effect composition
 *     generic effect dispatch
 *     effect-subsystem integration boundaries
 *     cross-module effect orchestration
 *     public effect entry points
 *
 * DOES NOT OWN
 * -------------
 *
 *     lexical tokens
 *     identifiers
 *     qualified names
 *     types
 *     expressions
 *     declarations outside effects
 *     capabilities
 *     resources
 *     policies
 *     contracts
 *     runtime semantics
 *     IR
 *     target realization
 *
 * PUBLIC ORCHESTRATOR RULES
 * -------------------------
 *
 *     effectConstruct
 *     effectDeclarationConstruct
 *     effectOperationConstruct
 *     effectHandlingConstruct
 *     effectTypeConstruct
 *     effectPolymorphismConstruct
 *     effectCollectionConstruct
 *     effectExtensionConstruct
 *     effectDomainConstruct
 *     effectReferenceConstruct
 *     effectSetConstruct
 *     effectInvocationConstruct
 *
 * These rules are stable parser integration points.
 *
 * ============================================================================
 * AUTHORITY CHAIN
 * ============================================================================
 *
 *     grammar/specification/
 *             |
 *             v
 *     grammar/spec/
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *     grammar/effects/effects.g4
 *             |
 *       +-----+------------------------------+
 *       |                                    |
 *       v                                    v
 * generic effect modules             specialized effect adapters
 *       |                                    |
 *       +----------------+-------------------+
 *                        |
 *                        v
 *                domain-neutral AST
 *                        |
 *                        v
 *                structural validation
 *                        |
 *                        v
 *                semantic effect analysis
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      capability     resource      policy
 *       analysis      analysis      analysis
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *              canonical semantic model
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      classical    quantum::ir      HDL/hardware
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                  optimization
 *                        |
 *                 routing/scheduling
 *                        |
 *                 resilience/QEC/ZQN
 *                        |
 *                        v
 *                       HAL
 *                        |
 *                        v
 *                 target realization
 *
 * ============================================================================
 * POCO-REAF INVARIANT
 * ============================================================================
 *
 * The effect grammar describes SOURCE SEMANTICS, not MACHINE REALIZATION.
 *
 * Therefore the grammar has no language-level constants for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     processes
 *     memory
 *     storage
 *     tensor rank
 *     network size
 *     topology size
 *     effect count
 *     operation count
 *     handler count
 *
 * No constructs such as:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_OPERATIONS
 *     MAX_HANDLER_ARMS
 *     MAX_EFFECT_SET_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * are permitted.
 *
 * Any implementation resource limitation belongs to the compiler, parser
 * runtime, operating environment, or target environment. It MUST NOT become
 * language semantics.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * An effect is identified by source-level syntax.
 *
 * Examples:
 *
 *     IO
 *     io::read
 *     quantum::measurement
 *     distributed::consensus
 *     accelerator::tensor
 *     hardware::signal
 *     networking::request
 *     security::authorize
 *     learning::update
 *     adaptation::strategy
 *     simulation::model
 *     foreign::call
 *     vendor::future::effect
 *
 * The grammar does not decide whether these names exist.
 *
 * Name existence, declaration compatibility and domain classification are
 * semantic concerns.
 *
 * A new effect domain therefore does not require modifying this file.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT
 *     Describes computational behavior or observable interaction.
 *
 * CAPABILITY
 *     Describes what a realization environment can provide.
 *
 * RESOURCE
 *     Describes computational resources involved in a realization.
 *
 * REQUIREMENT
 *     Describes a condition necessary for a valid realization.
 *
 * CONSTRAINT
 *     Describes a condition a realization must satisfy.
 *
 * PREFERENCE
 *     Describes a preferred valid realization.
 *
 * HINT
 *     Provides non-binding implementation guidance.
 *
 * POLICY
 *     Governs what realizations or behaviors are permitted.
 *
 * TARGET
 *     Describes a realization environment.
 *
 * HANDLER
 *     Describes source-level handling of effect behavior.
 *
 * These concepts MUST NOT be collapsed into one another.
 *
 * For example:
 *
 *     with effect { quantum::measurement }
 *
 * does not mean:
 *
 *     use QPU X
 *     use physical qubit Y
 *     use topology Z
 *     use calibration C
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar contains NO lexer rules.
 *
 * The production parser hierarchy uses the canonical Zamani lexer vocabulary.
 *
 * The intended chain is:
 *
 *     grammar/lexer/
 *             |
 *             v
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     ZamaniLexer
 *             |
 *             v
 *     parser grammars
 *
 * This file MUST NOT define effect-specific lexer tokens.
 *
 * In particular, it must not introduce local alternatives for:
 *
 *     EFFECT
 *     EFFECTS
 *     PERFORM
 *     HANDLE
 *     RESUME
 *     ABORT
 *
 * or domain names.
 *
 * ============================================================================
 * TOKEN-VOCABULARY INTEGRATION
 * ============================================================================
 *
 * `ZamaniLexer` is the canonical production vocabulary expected by the
 * canonical parser composition root.
 *
 * Individual effect grammars MUST converge on that vocabulary.
 *
 * Legacy grammars that still declare:
 *
 *     tokenVocab = ZamaniTokens
 *
 * are compatibility debt and must be migrated to the canonical vocabulary
 * under the repository's lexer/ANTLR migration plan.
 *
 * This orchestrator deliberately does not introduce a second token vocabulary
 * merely to accommodate legacy children.
 *
 * ============================================================================
 * SHARED SYNTAX CONTRACT
 * ============================================================================
 *
 * The following remain owned by shared grammars:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     argumentList
 *     attributes
 *     modifiers
 *     genericParameters
 *     visibility
 *     module syntax
 *     source-unit syntax
 *
 * This file MUST NOT redefine them.
 *
 * ============================================================================
 * GENERIC EFFECT MODULES
 * ============================================================================
 *
 * The canonical generic effect language is divided into:
 *
 *     EffectDeclarations
 *     EffectSets
 *     EffectOperations
 *     EffectHandling
 *     EffectTypes
 *     EffectPolymorphism
 *     EffectComposition
 *     CustomEffects
 *
 * Ownership:
 *
 *     effect-declarations.g4
 *         effect declaration syntax
 *
 *     effect-sets.g4
 *         effect references and sets
 *
 *     effect-operations.g4
 *         effect operation references and invocation
 *
 *     effect-handling.g4
 *         effect handlers and handling constructs
 *
 *     effect-types.g4
 *         effect-qualified type syntax
 *
 *     effect-polymorphism.g4
 *         effect variables, bounds and substitutions
 *
 *     effect-composition.g4
 *         effect composition integration boundary
 *
 *     custom-effects.g4
 *         extension, mapping, adaptation and wrapping relationships
 *
 * This file composes those modules. It does not replace them.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * The dependency graph MUST remain acyclic.
 *
 * Canonical direction:
 *
 *     Effects
 *       |
 *       +--> EffectDeclarations
 *       |
 *       +--> EffectSets
 *       |
 *       +--> EffectOperations
 *       |
 *       +--> EffectHandling
 *       |
 *       +--> EffectTypes
 *       |
 *       +--> EffectPolymorphism
 *       |
 *       +--> EffectComposition
 *       |
 *       +--> CustomEffects
 *
 * Child grammars may depend on lower-level common grammars.
 *
 * They MUST NOT import Effects.
 *
 * In particular, this forbidden cycle MUST never exist:
 *
 *     Effects
 *       |
 *       v
 *     SpecializedEffect
 *       |
 *       v
 *     Effects
 *
 * The same rule applies to:
 *
 *     quantum
 *     hardware
 *     networking
 *     distributed
 *     learning
 *     adaptation
 *     simulation
 *     security
 *     foreign
 *     IO
 *     reflection
 *     native
 *     randomness
 *     mutation
 *     measurement
 *
 * ============================================================================
 * SPECIALIZED EFFECT FAMILIES
 * ============================================================================
 *
 * The effects directory contains specialized effect grammars.
 *
 * These are NOT separate effect languages.
 *
 * They are domain adapters/classifiers around the generic effect model.
 *
 * Existing families include, where present:
 *
 *     adaptation
 *     capabilities
 *     code generation
 *     distributed
 *     foreign
 *     hardware
 *     IO
 *     learning
 *     measurement
 *     mutation
 *     native
 *     networking
 *     quantum
 *     randomness
 *     reflection
 *     security
 *     simulation
 *
 * Their semantic purpose is to provide domain-specific parser boundaries
 * without closing the generic effect universe.
 *
 * IMPORTANT:
 *
 * A specialized grammar MUST NOT be imported here merely because its domain
 * name is interesting.
 *
 * It is imported only when:
 *
 *     1. its parser grammar is acyclic;
 *     2. it consumes canonical shared syntax;
 *     3. it does not redefine generic effect rules;
 *     4. it does not import Effects;
 *     5. its public rules have stable ownership;
 *     6. its AST and semantic contracts exist.
 *
 * Until those conditions are true, the generic open-world rules remain the
 * canonical source syntax.
 *
 * ============================================================================
 * GENERIC DOMAIN EXTENSIBILITY
 * ============================================================================
 *
 * The universal effect system already permits:
 *
 *     future::domain::effect
 *
 * without changing this grammar.
 *
 * Consequently:
 *
 *     quantum::measurement
 *
 * does not require a quantum-specific alternative here.
 *
 * Nor does:
 *
 *     hardware::signal
 *     distributed::consensus
 *     ai::inference
 *     learning::update
 *     adaptation::policy
 *     vendor::future::operation
 *
 * This is essential for long-term scalability.
 *
 * ============================================================================
 * EFFECT DECLARATIONS
 * ============================================================================
 *
 * Owned by:
 *
 *     EffectDeclarations
 *
 * This orchestrator exposes declaration syntax without redefining it.
 *
 * ============================================================================
 * EFFECT SETS
 * ============================================================================
 *
 * Owned by:
 *
 *     EffectSets
 *
 * Effect sets remain open-ended.
 *
 * Examples:
 *
 *     with effects { io::read }
 *
 *     with effects {
 *         io::read,
 *         data::transform,
 *         quantum::measurement,
 *     }
 *
 * The grammar must not impose a fixed cardinality.
 *
 * ============================================================================
 * EFFECT OPERATIONS
 * ============================================================================
 *
 * Owned by:
 *
 *     EffectOperations
 *
 * Operation names remain open-world.
 *
 * The generic syntax can therefore represent:
 *
 *     quantum::measurement(q)
 *     distributed::consensus(value)
 *     networking::request(request)
 *     learning::update(model)
 *     adaptation::select(strategy)
 *
 * without adding grammar alternatives for every future operation.
 *
 * ============================================================================
 * EFFECT HANDLING
 * ============================================================================
 *
 * Owned by:
 *
 *     EffectHandling
 *
 * Handlers describe how source-level effect behavior is handled.
 *
 * They do not select:
 *
 *     hardware
 *     backend
 *     device
 *     topology
 *     scheduler
 *     QEC implementation
 *     runtime transport
 *
 * Those remain downstream concerns.
 *
 * ============================================================================
 * EFFECT TYPES
 * ============================================================================
 *
 * Owned by:
 *
 *     EffectTypes
 *
 * The type system owns the base type.
 *
 * EffectTypes owns only the effect qualification boundary.
 *
 * This prevents a Types <-> Effects circular grammar relationship.
 *
 * ============================================================================
 * EFFECT POLYMORPHISM
 * ============================================================================
 *
 * Owned by:
 *
 *     EffectPolymorphism
 *
 * Effect variables represent source-level abstraction over effect behavior.
 *
 * They do not represent:
 *
 *     device variables
 *     CPU variables
 *     QPU variables
 *     qubit identifiers
 *     resource handles
 *     physical topology
 *
 * ============================================================================
 * EFFECT COMPOSITION
 * ============================================================================
 *
 * Owned by:
 *
 *     EffectComposition
 *
 * This file MUST expose composition through the imported rule rather than
 * recreating another effect-set grammar.
 *
 * ============================================================================
 * CUSTOM EFFECTS
 * ============================================================================
 *
 * Owned by:
 *
 *     CustomEffects
 *
 * Custom effects extend relationships between semantic effects.
 *
 * They do not become a second declaration system.
 *
 * ============================================================================
 * EFFECT DOMAIN ADAPTER CONTRACT
 * ============================================================================
 *
 * A specialized effect grammar must obey this contract:
 *
 *     DOMAIN_EFFECT
 *         |
 *         v
 *     canonical effect reference
 *         |
 *         v
 *     generic effect semantic model
 *
 * It MUST NOT create:
 *
 *     DomainEffectIR
 *     DomainEffectRuntime
 *     DomainHardwareEffect
 *
 * merely because it is domain-specific.
 *
 * The canonical semantic layer determines downstream representation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum effects are represented as ordinary open-world effect identities.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::reset
 *     quantum::readout
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::logical_operation
 *
 * This grammar does NOT define:
 *
 *     physical qubits
 *     qubit allocation
 *     coupling maps
 *     topology
 *     calibration
 *     pulse schedules
 *     noise models
 *     QEC codes
 *     decoders
 *     routing
 *     scheduling
 *
 * Quantum semantic lowering remains:
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
 *     QEC/resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Effects may represent hardware/software interaction through open-world
 * names such as:
 *
 *     hardware::compute
 *     hardware::memory
 *     hardware::signal
 *     accelerator::compute
 *     hdl::event
 *
 * The grammar does not determine:
 *
 *     device identity
 *     physical address
 *     register width
 *     bus width
 *     memory capacity
 *     FPGA resource count
 *     ASIC structure
 *     placement
 *     routing
 *     clock implementation
 *
 * Those are target-realization concerns.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA INTEGRATION
 * ============================================================================
 *
 * The effect model is equally applicable to:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     data processing
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     inference
 *     provenance
 *     simulation
 *
 * These are represented as effect identities or declared effect operations,
 * not as a closed list of language-level effects.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Distributed and networking effects participate in the same generic model.
 *
 * Examples:
 *
 *     distributed::consensus
 *     distributed::replication
 *     networking::send
 *     networking::receive
 *     networking::request
 *
 * The grammar does not determine:
 *
 *     node count
 *     topology
 *     transport
 *     endpoint allocation
 *     network device
 *
 * ============================================================================
 * LEARNING / ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Learning and adaptation are effect semantics, not parser-level machine
 * decisions.
 *
 * For example:
 *
 *     learning::update
 *     adaptation::select
 *
 * may be represented by the generic effect model.
 *
 * Semantic analysis determines:
 *
 *     model changes
 *     state changes
 *     authorization
 *     policy
 *     provenance
 *     resource implications
 *     reproducibility requirements
 *     capability requirements
 *
 * The grammar itself remains target-neutral.
 *
 * ============================================================================
 * FOREIGN / NATIVE INTEGRATION
 * ============================================================================
 *
 * Foreign/native effects are allowed to express boundaries to external
 * execution environments.
 *
 * They MUST participate in the effect system.
 *
 * They do not grant automatic permission to:
 *
 *     execute native code
 *     bypass safety
 *     bypass capabilities
 *     bypass policy
 *     bypass resource checking
 *
 * Those decisions remain semantic/security/runtime concerns.
 *
 * ============================================================================
 * REFLECTION / CODE GENERATION INTEGRATION
 * ============================================================================
 *
 * Reflection and code-generation effects may be represented by open-world
 * effect names.
 *
 * Example:
 *
 *     reflection::inspect
 *     codegen::generate
 *
 * Such effects remain subject to:
 *
 *     capability analysis
 *     policy analysis
 *     provenance
 *     reproducibility
 *     compatibility
 *
 * No target-specific behavior is embedded here.
 *
 * ============================================================================
 * EFFECTS AND CAPABILITIES
 * ============================================================================
 *
 * An effect MAY semantically imply a capability requirement.
 *
 * For example:
 *
 *     quantum::measurement
 *
 * might cause semantic analysis to require a capability such as:
 *
 *     quantum.measurement
 *
 * But this grammar does NOT perform that mapping.
 *
 * The separation is:
 *
 *     EFFECT
 *       |
 *       v
 * semantic analysis
 *       |
 *       v
 * CAPABILITY REQUIREMENT
 *       |
 *       v
 * capability negotiation
 *       |
 *       v
 * target realization
 *
 * ============================================================================
 * EFFECTS AND RESOURCES
 * ============================================================================
 *
 * An effect MAY imply resource requirements.
 *
 * The grammar does not encode physical resource quantities.
 *
 * For example, semantic analysis may derive:
 *
 *     requires memory >= required_memory
 *
 * or:
 *
 *     requires capability("quantum.measurement")
 *
 * or:
 *
 *     requires topology(required_topology)
 *
 * without changing the effect grammar.
 *
 * ============================================================================
 * EFFECTS AND POLICIES
 * ============================================================================
 *
 * Effects may be constrained by policies.
 *
 * The policy subsystem remains the owner of:
 *
 *     permissions
 *     prohibitions
 *     constraints
 *     preferences
 *     fallbacks
 *     adaptation policy
 *     security policy
 *
 * This grammar does not duplicate policy syntax.
 *
 * ============================================================================
 * EFFECTS AND CONTRACTS
 * ============================================================================
 *
 * Effects may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The contract subsystem owns contract syntax.
 *
 * The effect system merely provides semantic effect information to contract
 * analysis.
 *
 * ============================================================================
 * EFFECTS AND PROVENANCE
 * ============================================================================
 *
 * Effect analysis may contribute provenance records describing:
 *
 *     source effect
 *     inferred effect
 *     transformed effect
 *     resolved operation
 *     semantic decision
 *     capability derivation
 *     resource derivation
 *
 * Provenance is downstream of parsing.
 *
 * The grammar preserves source structure needed for provenance.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates parser contexts only.
 *
 * The frontend AST must remain domain-neutral.
 *
 * It must preserve, where applicable:
 *
 *     source span
 *     declaration identity
 *     qualified-name segments
 *     effect references
 *     effect-set membership
 *     effect operation identity
 *     operation arguments
 *     handler structure
 *     effect variables
 *     effect substitutions
 *     custom-effect relationships
 *     composition structure
 *
 * It MUST NOT require backend-specific nodes such as:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     GPUInstruction
 *     CPUInstruction
 *     FPGAPrimitive
 *     DeviceHandle
 *     BackendId
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     declaration consistency
 *     effect inference
 *     effect normalization
 *     effect compatibility
 *     effect polymorphism
 *     handler validation
 *     capability derivation
 *     resource derivation
 *     policy validation
 *     security validation
 *     domain interpretation
 *     target-independent validity
 *
 * The parser MUST NOT perform any of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * The effect pipeline is:
 *
 *     effect syntax
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     effect semantic model
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +--> classical IR
 *         |
 *         +--> quantum::ir
 *         |
 *         +--> HDL/hardware representation
 *         |
 *         +--> distributed representation
 *         |
 *         +--> future domain representation
 *
 * No effect grammar may introduce a competing quantum IR.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors belong here.
 *
 * Examples:
 *
 *     malformed effect declaration
 *     malformed effect set
 *     malformed effect reference
 *     malformed invocation
 *     malformed handler
 *     malformed polymorphic construct
 *     malformed custom-effect construct
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unknown effect
 *     unknown operation
 *     incompatible effect
 *     invalid substitution
 *     invalid handler
 *     unavailable capability
 *     insufficient resources
 *     forbidden policy
 *     unsupported target
 *
 * Capability/resource failures MUST NOT be reported as parser syntax errors.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     token stream
 *     selected grammar
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware
 *     target availability
 *     resource availability
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *
 * Identical input under identical grammar configuration must produce equivalent
 * parser structure.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limits on:
 *
 *     effect declarations
 *     effect operations
 *     effect references
 *     effect-set members
 *     handler arms
 *     nested handlers
 *     effect variables
 *     generic parameters
 *     qualified-name depth
 *     namespaces
 *     domains
 *     modules
 *     program size
 *     machine size
 *
 * Any practical finite limit belongs to implementation resources and MUST NOT
 * alter source-language semantics.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing generic effect concepts remain represented:
 *
 *     declarations
 *     sets
 *     operations
 *     handlers
 *     effect-qualified types
 *     effect polymorphism
 *     composition
 *     custom effects
 *
 * Compatibility is preserved by keeping their dedicated grammar ownership.
 *
 * Legacy grammar files may remain during migration, but they MUST NOT become
 * competing authorities.
 *
 * The canonical promotion path is:
 *
 *     proposal
 *         ->
 *     specification
 *         ->
 *     AST contract
 *         ->
 *     canonical grammar
 *         ->
 *     semantic implementation
 *         ->
 *     IR contract
 *         ->
 *     conformance tests
 *         ->
 *     stable feature
 *
 * ============================================================================
 * PUBLIC DISPATCH MODEL
 * ============================================================================
 *
 * `effectConstruct` is the universal effect-subsystem dispatch rule.
 *
 * It is intentionally composed from canonical child rules.
 *
 * No child rule is reimplemented here.
 *
 * ============================================================================
 */

parser grammar Effects;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL GENERIC EFFECT MODULES
 * ============================================================================
 *
 * These are the generic language components.
 *
 * Domain-specific effect grammars are intentionally not imported here merely
 * to enumerate their vocabulary. Open-world qualified names already provide
 * the universal mechanism required for new domains.
 *
 * A specialized grammar may be independently composed by the parser/build
 * architecture after it satisfies the non-cyclic module contract.
 *
 * ============================================================================
 */

import
    Core,
    Types,
    Expressions,
    EffectDeclarations,
    EffectSets,
    EffectOperations,
    EffectHandling,
    EffectTypes,
    EffectPolymorphism,
    EffectComposition,
    CustomEffects
;

/*
 * ============================================================================
 * EFFECT SUBSYSTEM ENTRY POINT
 * ============================================================================
 *
 * This is the principal public dispatch rule.
 *
 * Every generic effect construct enters through this boundary.
 *
 * ============================================================================
 */

effectConstruct
    : effectDeclaration
    | effectOperationDeclaration
    | effectInvocation
    | effectOperationUse
    | performEffectOperation
    | handleExpression
    | handleStatement
    | effectTypeQualification
    | effectPolymorphicConstruct
    | effectComposition
    | customEffectDeclaration
    | customEffectDefinition
    | customEffectReferenceDefinition
    | customEffectCompositionDefinition
    | customEffectAdapterDefinition
    | customEffectMapping
    | customEffectRefinementDefinition
    | customEffectExtensionDeclaration
    | customEffectWrapperDeclaration
    ;

/*
 * ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 */

effectDeclarationConstruct
    : effectDeclaration
    ;

effectOperationDeclarationConstruct
    : effectOperationDeclaration
    ;

/*
 * ============================================================================
 * OPERATION INTEGRATION
 * ============================================================================
 */

effectOperationConstruct
    : effectOperationDeclaration
    | effectInvocation
    | effectOperationUse
    | performEffectOperation
    ;

effectInvocationConstruct
    : effectInvocation
    | effectOperationUse
    ;

effectOperationReferenceConstruct
    : effectOperationReference
    ;

/*
 * ============================================================================
 * HANDLING INTEGRATION
 * ============================================================================
 */

effectHandlingConstruct
    : handleExpression
    | handleStatement
    | effectHandlingConstruct
    ;

/*
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 */

effectTypeConstruct
    : effectTypeQualification
    | effectTypeClause
    | effectTypeQualifier
    ;

/*
 * ============================================================================
 * POLYMORPHISM INTEGRATION
 * ============================================================================
 */

effectPolymorphismConstruct
    : effectPolymorphicConstruct
    | effectPolymorphicApplication
    | effectPolymorphicSignature
    | effectPolymorphicScope
    ;

/*
 * ============================================================================
 * COLLECTION INTEGRATION
 * ============================================================================
 */

effectCollectionConstruct
    : effectReference
    | effectSet
    | effectSetComposition
    | effectComposition
    ;

effectReferenceConstruct
    : effectReference
    ;

effectSetConstruct
    : effectSet
    ;

/*
 * ============================================================================
 * EXTENSION INTEGRATION
 * ============================================================================
 *
 * Custom effects remain relationships around canonical effects.
 * They do not replace the ordinary effect declaration/set system.
 *
 * ============================================================================
 */

effectExtensionConstruct
    : customEffectDeclaration
    | customEffectDefinition
    | customEffectReferenceDefinition
    | customEffectCompositionDefinition
    | customEffectAdapterDefinition
    | customEffectMapping
    | customEffectRefinementDefinition
    | customEffectExtensionDeclaration
    | customEffectWrapperDeclaration
    ;

/*
 * ============================================================================
 * DOMAIN INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Specialized effect domains remain open-world semantic classifications.
 *
 * A generic effect reference such as:
 *
 *     quantum::measurement
 *
 * is already valid through `effectReference`.
 *
 * Therefore the universal parser does not need:
 *
 *     quantumEffect
 *     hardwareEffect
 *     learningEffect
 *     networkEffect
 *
 * alternatives merely to make those names legal.
 *
 * Specialized grammars may expose additional structured syntax where a domain
 * genuinely requires syntax beyond an ordinary effect reference, but those
 * grammars must remain separate from this generic composition root until their
 * import graphs are proven acyclic.
 *
 * ============================================================================
 */

effectDomainConstruct
    : effectReference
    | effectSet
    | effectInvocation
    ;

/*
 * ============================================================================
 * EFFECT CONTEXT
 * ============================================================================
 *
 * This rule gives consuming grammars one stable noun-like integration point.
 *
 * It does not introduce another effect-set syntax.
 * ============================================================================
 */

effectContext
    : effectSet
    ;

/*
 * ============================================================================
 * OPTIONAL EFFECT CONTEXT
 * ============================================================================
 *
 * Optionality belongs to the consuming context rather than to a second effect
 * collection grammar.
 * ============================================================================
 */

optionalEffectContext
    : effectContext?
    ;

/*
 * ============================================================================
 * EFFECT USE
 * ============================================================================
 *
 * Stable integration point for statements, expressions, functions, contracts,
 * policies and other consumers that need to refer to an effectful construct.
 * ============================================================================
 */

effectUse
    : effectInvocation
    | effectOperationUse
    | performEffectOperation
    ;

/*
 * ============================================================================
 * EFFECT HANDLER ENTRY POINT
 * ============================================================================
 */

effectHandlerConstruct
    : handleExpression
    | handleStatement
    ;

/*
 * ============================================================================
 * EFFECT QUALIFICATION ENTRY POINT
 * ============================================================================
 */

effectQualificationConstruct
    : effectTypeQualification
    ;

/*
 * ============================================================================
 * EFFECT POLYMORPHIC ENTRY POINT
 * ============================================================================
 */

effectGenericConstruct
    : effectPolymorphicConstruct
    ;

/*
 * ============================================================================
 * EFFECT COMPOSITION ENTRY POINT
 * ============================================================================
 */

effectCompositionConstruct
    : effectComposition
    ;

/*
 * ============================================================================
 * CUSTOM EFFECT ENTRY POINT
 * ============================================================================
 */

customEffectConstruct
    : customEffectDeclaration
    | customEffectDefinition
    | customEffectReferenceDefinition
    | customEffectCompositionDefinition
    | customEffectAdapterDefinition
    | customEffectMapping
    | customEffectRefinementDefinition
    | customEffectExtensionDeclaration
    | customEffectWrapperDeclaration
    ;

/*
 * ============================================================================
 * ORCHESTRATOR COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] Effects is the sole generic effect-subsystem composition root.
 *
 * [ ] ZamaniParser imports Effects rather than importing every effect leaf.
 *
 * [ ] This file consumes the canonical production lexer vocabulary.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] No local effect token vocabulary exists here.
 *
 * [ ] Effect declarations have exactly one grammar owner.
 *
 * [ ] Effect sets have exactly one grammar owner.
 *
 * [ ] Effect operations have exactly one grammar owner.
 *
 * [ ] Effect handlers have exactly one grammar owner.
 *
 * [ ] Effect-qualified types have exactly one grammar owner.
 *
 * [ ] Effect polymorphism has exactly one grammar owner.
 *
 * [ ] Effect composition has exactly one grammar owner.
 *
 * [ ] Custom-effect extension syntax has exactly one grammar owner.
 *
 * [ ] No child grammar imports Effects.
 *
 * [ ] No circular grammar dependency exists.
 *
 * [ ] Specialized domains are not enumerated as a closed effect catalogue.
 *
 * [ ] New effect namespaces can be introduced without editing this file.
 *
 * [ ] Quantum effect identity remains open-world.
 *
 * [ ] Hardware effect identity remains open-world.
 *
 * [ ] Distributed effect identity remains open-world.
 *
 * [ ] Networking effect identity remains open-world.
 *
 * [ ] Learning/adaptation effect identity remains open-world.
 *
 * [ ] Foreign/native effect identity remains open-world.
 *
 * [ ] No machine-size limit is encoded.
 *
 * [ ] No effect-count limit is encoded.
 *
 * [ ] No handler-count limit is encoded.
 *
 * [ ] No resource capacity is encoded.
 *
 * [ ] No target selection is encoded.
 *
 * [ ] No physical hardware identity is encoded.
 *
 * [ ] No quantum physical realization is encoded.
 *
 * [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 * [ ] QEC remains downstream.
 *
 * [ ] ZQN remains downstream.
 *
 * [ ] Routing remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] HAL remains downstream.
 *
 * [ ] Capability resolution remains downstream.
 *
 * [ ] Resource resolution remains downstream.
 *
 * [ ] Policy resolution remains downstream.
 *
 * [ ] Provenance remains downstream.
 *
 * [ ] AST mapping exists for every public construct.
 *
 * [ ] Semantic mapping exists for every public construct.
 *
 * [ ] IR destination exists for every executable effect construct.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Generated parser remains compatible with Rust 1.97 / 1.97.1.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/specification/
 *     grammar/spec/
 *     grammar/lexer/
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/antlr/ZamaniParser.g4
 *
 * Immediate dependencies:
 *
 *     Core
 *     Types
 *     Expressions
 *     EffectDeclarations
 *     EffectSets
 *     EffectOperations
 *     EffectHandling
 *     EffectTypes
 *     EffectPolymorphism
 *     EffectComposition
 *     CustomEffects
 *
 * Downstream:
 *
 *     domain-neutral AST
 *     structural validation
 *     semantic effect analysis
 *     type/effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     security analysis
 *     provenance
 *     canonical semantic model
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representation
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     target realization
 *
 * Consumers MUST depend on public rule names rather than generated ANTLR token
 * numbers or generated implementation details.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * The effect subsystem describes:
 *
 *     WHAT computational behavior is observable or requested.
 *
 * It does not describe:
 *
 *     WHERE it executes.
 *     WHICH machine executes it.
 *     WHICH device executes it.
 *     WHICH physical qubit is used.
 *     WHICH CPU executes it.
 *     WHICH GPU executes it.
 *     WHICH FPGA resource implements it.
 *     WHICH network topology is selected.
 *     WHICH scheduler realizes it.
 *     WHICH routing algorithm realizes it.
 *     WHICH QEC implementation realizes it.
 *     WHICH HAL realizes it.
 *
 * Therefore:
 *
 *     Zamani source
 *         ->
 *     parse effect intent
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic effect analysis
 *         ->
 *     capability/resource/policy resolution
 *         ->
 *     canonical semantic representation
 *         ->
 *     classical IR / quantum::ir / HDL representation
 *         ->
 *     optimization
 *         ->
 *     lowering
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     resilience/QEC/ZQN
 *         ->
 *     HAL
 *         ->
 *     available target realization
 *
 * remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * without making today's hardware the definition of tomorrow's language.
 *
 * ============================================================================
 */