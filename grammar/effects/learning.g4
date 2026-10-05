/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/effects/learning.g4
 *
 * Grammar:
 *     LearningEffects
 *
 * Status:
 *     CANONICAL LEARNING-EFFECT DOMAIN INTEGRATION GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime execution.
 *     - No hardware discovery.
 *     - No target selection.
 *     - No resource discovery.
 *     - No randomness.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level integration boundary for the
 * learning effect domain.
 *
 * It connects learning semantics to Zamani's generic effect system without
 * creating a second effect language.
 *
 * Learning is treated as a semantic computational effect.
 *
 * Examples of semantic learning activities that may eventually be represented
 * by this boundary include:
 *
 *     model update
 *     parameter update
 *     knowledge update
 *     evidence incorporation
 *     feedback incorporation
 *     strategy improvement
 *     policy learning
 *     symbolic learning
 *     statistical learning
 *     neural learning
 *     probabilistic learning
 *     reinforcement learning
 *     transfer learning
 *     distributed learning
 *     federated learning
 *     hybrid learning
 *     quantum-assisted learning
 *     adaptive learning
 *     future learning mechanisms
 *
 * The grammar deliberately does NOT enumerate those mechanisms.
 *
 * A new learning algorithm, model family, accelerator, quantum technique,
 * optimization method, or vendor implementation MUST NOT require changing
 * this grammar when its source representation can already be expressed by
 * the generic effect model and ordinary Zamani expressions.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns only learning-domain parser integration boundaries:
 *
 *     learningEffect
 *     learningEffectReference
 *     learningEffectReferenceList
 *     learningEffectSet
 *     learningEffectInvocation
 *     learningEffectOperationUse
 *     learningEffectOperationReference
 *
 * These rules provide a stable syntactic adapter for semantic analysis to
 * recognize constructs that are intended to participate in the learning
 * effect domain.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     effect declarations
 *     effect operation declarations
 *     generic effect references
 *     generic effect sets
 *     generic effect invocation
 *     generic effect operation use
 *     effect handlers
 *     effect polymorphism
 *     effect composition
 *     custom-effect declarations
 *     learn statements
 *     infer statements
 *     deduce statements
 *     reason statements
 *     adapt statements
 *     knowledge statements
 *     query statements
 *     model declarations
 *     tensor syntax
 *     probability syntax
 *     uncertainty syntax
 *     contract syntax
 *     policy syntax
 *     capability syntax
 *     resource syntax
 *     provenance syntax
 *     concurrency syntax
 *     actor syntax
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     networking syntax
 *     distributed syntax
 *     FFI syntax
 *     ABI syntax
 *     runtime execution
 *     scheduling
 *     routing
 *     target selection
 *     physical resource selection.
 *
 * ============================================================================
 * CRITICAL SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The generic effect subsystem remains authoritative.
 *
 * Generic effect syntax is owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effect-types.g4
 *     grammar/effects/effect-polymorphism.g4
 *     grammar/effects/effect-composition.g4
 *     grammar/effects/custom-effects.g4
 *     grammar/effects/effects.g4
 *
 * Therefore this file MUST NOT redefine:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationCall
 *     effectOperationUse
 *     performEffectOperation
 *     handleExpression
 *     handleStatement
 *     effectHandler
 *     effectPolymorphicParameterList
 *     effectComposition
 *     customEffectDeclaration
 *
 * This file only provides learning-domain aliases/adapters around those
 * canonical constructs.
 *
 * ============================================================================
 * LEARNING STATEMENT SEPARATION
 * ============================================================================
 *
 * The source-level:
 *
 *     learn ...
 *
 * statement is owned by:
 *
 *     grammar/statements/learn.g4
 *
 * This file MUST NOT redefine `learnStatement`.
 *
 * The architectural relationship is:
 *
 *     learn statement
 *          |
 *          v
 *     semantic learning operation
 *          |
 *          +--> learning effect
 *          |
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *
 * Therefore:
 *
 *     grammar/statements/learn.g4
 *
 * describes source-level learning intent, while:
 *
 *     grammar/effects/learning.g4
 *
 * provides the effect-domain integration boundary.
 *
 * Neither file replaces the other.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     learning semantics    generic effects
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *             semantic model
 *                    |
 *          +---------+---------+----------------+
 *          |                   |                |
 *          v                   v                v
 *       classical          quantum::ir      HDL/hardware
 *          |                   |                |
 *          +-------------------+----------------+
 *                              |
 *                              v
 *                    canonical semantic IR
 *                              |
 *                              v
 *                    optimization/specialization
 *                              |
 *                              v
 *                    lowering/routing/scheduling
 *                              |
 *                              v
 *                       resilience/recovery
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *
 * This file MUST remain above semantic realization.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     Core
 *     Expressions
 *     EffectOperations
 *     EffectSets
 *
 * Core supplies:
 *
 *     qualifiedName
 *
 * Expressions supplies:
 *
 *     expression
 *     argumentList
 *
 * EffectOperations supplies:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *
 * EffectSets supplies:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public entry point:
 *
 *     learningEffect
 *
 * Publicly reusable domain adapters:
 *
 *     learningEffectReference
 *     learningEffectReferenceList
 *     learningEffectSet
 *     learningEffectInvocation
 *     learningEffectOperationUse
 *     learningEffectOperationReference
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * Potential consumers include:
 *
 *     grammar/ai/
 *     grammar/statements/learn.g4
 *     grammar/expressions/
 *     grammar/effects/effects.g4 consumers
 *     semantic learning analysis
 *     effect inference
 *     effect validation
 *     compiler conformance tests
 *
 * IMPORTANT:
 *
 * grammar/effects/effects.g4 MUST NOT import this file merely to make the
 * generic effect grammar aware of learning.
 *
 * The generic effect grammar remains domain-neutral and open-world.
 *
 * This file is instead consumed by components that need explicit learning
 * effect classification.
 *
 * ============================================================================
 * AST_OWNER
 * ============================================================================
 *
 * The existing domain-neutral frontend AST and effect AST infrastructure.
 *
 * This grammar creates parser contexts only.
 *
 * It MUST NOT introduce a learning-specific backend AST merely because the
 * effect belongs to the learning domain.
 *
 * The frontend must preserve, where applicable:
 *
 *     source span
 *     effect identity
 *     qualified-name segments
 *     operation identity
 *     operation arguments
 *     effect-set membership
 *     source ordering
 *     surrounding semantic context
 *
 * ============================================================================
 * SEMANTIC_OWNER
 * ============================================================================
 *
 * The semantic effect and learning subsystems.
 *
 * Semantic analysis determines:
 *
 *     - whether an effect resolves;
 *     - whether it belongs to the learning domain;
 *     - whether the operation is declared;
 *     - whether operation arguments are type-correct;
 *     - whether learning is permitted in the current context;
 *     - whether required capabilities exist;
 *     - whether resources are sufficient;
 *     - whether contracts are satisfied;
 *     - whether policies authorize the operation;
 *     - whether provenance is required;
 *     - whether reproducibility constraints apply;
 *     - whether deterministic execution is required;
 *     - whether the learning operation is compatible with surrounding effects;
 *     - whether the operation crosses classical/quantum/HDL/distributed
 *       boundaries;
 *     - whether the selected realization is feasible.
 *
 * Parser acceptance MUST NOT imply semantic validity.
 *
 * ============================================================================
 * IR_OWNER
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * There is deliberately no:
 *
 *     LearningEffectIR
 *     TrainingIR
 *     ModelIR
 *     NeuralIR
 *     QMLIR
 *
 * introduced here.
 *
 * The semantic pipeline remains:
 *
 *     learning effect syntax
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
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future domain representation
 *
 * ============================================================================
 * TEST_OWNER
 * ============================================================================
 *
 * Recommended test location:
 *
 *     grammar/tests/effects/learning/
 *
 * Recommended groups:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     cross-domain/
 *     compatibility/
 *     round-trip/
 *
 * ============================================================================
 * SPEC_OWNER
 * ============================================================================
 *
 * Normative human-readable specification:
 *
 *     grammar/specification/
 *
 * Machine-checkable conformance:
 *
 *     grammar/spec/
 *
 * Effect-wide normative behavior:
 *
 *     grammar/spec/effects.md
 *
 * Learning semantics:
 *
 *     grammar/spec/ai.md
 *
 * Learning statement syntax:
 *
 *     grammar/statements/learn.g4
 *
 * This file remains the effect-domain parser boundary.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No lexer rule is defined here.
 *
 * No new keyword is required for this file.
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     LEARNING
 *     TRAIN
 *     TRAINING
 *     MODEL
 *     DATASET
 *     OPTIMIZER
 *     NEURAL
 *     PROBABILISTIC
 *
 * merely to identify a learning effect.
 *
 * Learning effect identity remains an open semantic namespace.
 *
 * ============================================================================
 * OPEN-WORLD LEARNING EFFECT MODEL
 * ============================================================================
 *
 * Learning effect identities are open-world.
 *
 * This grammar MUST NOT enumerate a closed operation catalogue such as:
 *
 *     train
 *     update
 *     fit
 *     optimize
 *     backpropagate
 *     reinforce
 *     classify
 *     predict
 *     embed
 *     cluster
 *     transfer
 *
 * Those are semantic, library, dialect, or application vocabulary.
 *
 * They are not universal grammar primitives.
 *
 * Examples of possible semantic effect identities include:
 *
 *     learning::update
 *     learning::train
 *     learning::infer
 *     learning::feedback
 *     learning::adapt
 *     learning::policy_update
 *     learning::knowledge_update
 *     ai::learning
 *     model::update
 *     probabilistic::learning
 *     quantum::learning
 *     distributed::learning
 *     future::learning::operation
 *     vendor::learning::extension
 *
 * Whether any of these is actually declared and classified as a learning
 * effect is a semantic question.
 *
 * ============================================================================
 * WHY THE GRAMMAR DOES NOT ENUMERATE LEARNING NAMES
 * ============================================================================
 *
 * Zamani must remain extensible without grammar edits for every new learning
 * mechanism.
 *
 * A future learning system might use:
 *
 *     symbolic rules
 *     statistical inference
 *     neural models
 *     probabilistic models
 *     evolutionary computation
 *     reinforcement learning
 *     federated learning
 *     distributed learning
 *     quantum-assisted computation
 *     neuromorphic computation
 *     analog computation
 *     photonic computation
 *     future computational substrates
 *
 * The parser must not need to know which one exists.
 *
 * The semantic registry/declaration system determines meaning.
 *
 * ============================================================================
 * DOMAIN CLASSIFICATION CONTRACT
 * ============================================================================
 *
 * A syntactic `learningEffectReference` is NOT proof that the referenced
 * effect belongs to the learning domain.
 *
 * Semantic resolution must establish:
 *
 *     identity
 *         |
 *         v
 *     declaration
 *         |
 *         v
 *     effect domain
 *         |
 *         v
 *     learning classification
 *
 * This is intentional.
 *
 * It prevents the grammar from becoming a closed catalogue.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * EFFECT
 *     Describes computational behavior or interaction.
 *
 * CAPABILITY
 *     Describes what an execution environment can provide or authorize.
 *
 * RESOURCE
 *     Describes computational resources consumed, required, reserved, or
 *     negotiated by an execution.
 *
 * REQUIREMENT
 *     Describes what must be satisfied for realization.
 *
 * CONSTRAINT
 *     Describes what realization must obey.
 *
 * PREFERENCE
 *     Describes a preferred valid realization.
 *
 * HINT
 *     Provides non-binding realization guidance.
 *
 * POLICY
 *     Governs what is permitted.
 *
 * PROVENANCE
 *     Describes lineage, derivation, evidence, transformation, or decision
 *     history.
 *
 * Learning effects MUST NOT silently become any of these categories.
 *
 * ============================================================================
 * LEARNING EFFECT SEMANTICS
 * ============================================================================
 *
 * A learning effect may semantically represent changes to:
 *
 *     model state
 *     learned parameters
 *     knowledge state
 *     strategy state
 *     policy state
 *     evidence state
 *     learned representations
 *     adaptation state
 *
 * Such changes may also involve other effects, including:
 *
 *     mutation
 *     randomness
 *     IO
 *     network
 *     distributed
 *     measurement
 *     foreign
 *     native
 *     persistence
 *     accelerator computation
 *
 * The effect system determines the complete effect set.
 *
 * This grammar does not silently infer those effects.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Learning is not automatically deterministic or nondeterministic.
 *
 * The semantic layer must determine whether a particular learning operation
 * requires or permits:
 *
 *     randomness
 *     nondeterministic scheduling
 *     distributed reduction
 *     external data
 *     mutable state
 *     time-dependent input
 *     hardware-dependent behavior
 *
 * Reproducibility requirements belong to the semantic/contract/provenance
 * layers.
 *
 * The grammar itself remains deterministic.
 *
 * ============================================================================
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * A learning effect may participate in reproducibility requirements involving:
 *
 *     source identity
 *     data identity
 *     model identity
 *     parameter state
 *     algorithm identity
 *     semantic version
 *     random source
 *     random state
 *     execution policy
 *     target capabilities
 *     resource realization
 *     provenance
 *
 * None of these become mandatory grammar keywords here.
 *
 * They are represented by the appropriate existing semantic systems.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Learning may consume or modify knowledge.
 *
 * Examples:
 *
 *     learning::update
 *     learning::knowledge_update
 *     learning::feedback
 *
 * The knowledge subsystem remains authoritative for:
 *
 *     assertions
 *     retractions
 *     queries
 *     facts
 *     knowledge graphs
 *     provenance
 *
 * This file MUST NOT redefine knowledge syntax.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Learning may consume reasoning results.
 *
 * Examples:
 *
 *     learning::update(reasoning_result)
 *     learning::feedback(inference_result)
 *
 * Reasoning remains owned by its dedicated grammar/semantic subsystem.
 *
 * This file MUST NOT redefine:
 *
 *     infer
 *     deduce
 *     reason
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Learning and adaptation are related but distinct.
 *
 * Learning may produce information that is subsequently consumed by
 * adaptation.
 *
 * The grammar MUST NOT silently equate:
 *
 *     learning
 *
 * with:
 *
 *     adaptation
 *
 * Adaptation remains governed by its own semantic operation, effects,
 * capabilities, resources, contracts, policies, and provenance.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Learning may operate on:
 *
 *     probabilities
 *     distributions
 *     uncertain values
 *     confidence values
 *     beliefs
 *     intervals
 *     symbolic uncertainty
 *
 * These are ordinary semantic/type constructs.
 *
 * This grammar does not impose:
 *
 *     numeric precision
 *     floating-point width
 *     probability cardinality
 *     tensor rank
 *     tensor dimension
 *     model size
 *     dataset size
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Learning may consume evidence.
 *
 * The evidence model may preserve:
 *
 *     claim
 *     source
 *     observation
 *     derivation
 *     confidence
 *     verification
 *     provenance
 *
 * This grammar only preserves the effect identity and its source expressions.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Learning is particularly suitable for provenance tracking.
 *
 * Semantic provenance may record:
 *
 *     source program
 *     source span
 *     learning operation
 *     input data
 *     knowledge source
 *     model identity
 *     parameter state
 *     policy decision
 *     capability decision
 *     resource realization
 *     transformation
 *     verification result
 *     execution realization
 *
 * The grammar does not create provenance records.
 *
 * It merely preserves the syntactic information required to construct them.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Learning operations may be governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by the validation/contract subsystem.
 *
 * This grammar MUST NOT duplicate contract grammar.
 *
 * Semantic analysis may associate contracts with a learning effect.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Learning may be controlled by policies concerning:
 *
 *     data use
 *     privacy
 *     model modification
 *     adaptation authority
 *     reproducibility
 *     security
 *     network access
 *     foreign calls
 *     resource consumption
 *     deployment
 *
 * Policy syntax remains owned by:
 *
 *     grammar/policies/
 *
 * and existing security/policy grammars.
 *
 * This file MUST NOT create a second policy language.
 *
 * ============================================================================
 * SANDBOX INTEGRATION
 * ============================================================================
 *
 * A learning effect may execute under a sandbox.
 *
 * The sandbox may restrict:
 *
 *     IO
 *     network
 *     native execution
 *     foreign execution
 *     reflection
 *     adaptation
 *     persistence
 *     resource consumption
 *
 * The learning grammar does not determine sandbox behavior.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A learning operation may require capabilities such as:
 *
 *     model.compute
 *     tensor.compute
 *     data.read
 *     distributed.compute
 *     accelerator.compute
 *     quantum.compute
 *
 * Capability names remain open-world.
 *
 * This file does not declare capabilities.
 *
 * Capability analysis is downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Learning may consume resources such as:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     energy
 *     accelerator capacity
 *     quantum resources
 *     distributed resources
 *
 * The grammar MUST NOT encode fixed resource capacities.
 *
 * Forbidden examples include:
 *
 *     64GB
 *     24GB
 *     32-bit
 *     1024 parameters
 *     8 devices
 *     16 workers
 *
 * when used as universal language limits.
 *
 * Symbolic resource requirements belong to the resource subsystem.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Learning may participate in quantum-classical hybrid computation.
 *
 * For example, semantic analysis may connect:
 *
 *     learning effect
 *          |
 *          v
 *     quantum computation
 *          |
 *          v
 *     quantum::ir
 *
 * This file MUST NOT define:
 *
 *     qubit identifiers
 *     physical qubits
 *     gate catalogues
 *     coupling maps
 *     calibration
 *     pulse schedules
 *     routing
 *     QEC
 *     decoders
 *     ZQN
 *     HAL
 *
 * If a learning operation contains quantum computation, its quantum portion
 * MUST eventually cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No learning-specific quantum IR is permitted.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Learning may be realized using:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     neuromorphic hardware
 *     photonic hardware
 *     other future substrates
 *
 * This grammar does not select any of them.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Learning may be:
 *
 *     local
 *     parallel
 *     distributed
 *     federated
 *     replicated
 *     heterogeneous
 *
 * The grammar imposes no node, worker, participant, device, memory, or network
 * ceiling.
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *     grammar/concurrency/
 *     grammar/networking/
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Learning effects may eventually cross:
 *
 *     FFI
 *     ABI
 *     external model formats
 *     external data formats
 *     accelerator APIs
 *     quantum representations
 *     HDL representations
 *     vendor dialects
 *
 * Interoperability grammars remain authoritative for those boundaries.
 *
 * This file does not duplicate:
 *
 *     FFI
 *     ABI
 *     JSON
 *     XML
 *     SQL
 *     OpenQASM
 *     QIR
 *     HDL
 *
 * ============================================================================
 * OPEN-WORLD OPERATION ARGUMENTS
 * ============================================================================
 *
 * Operation arguments remain ordinary Zamani expressions.
 *
 * The grammar therefore does not require dedicated syntax for:
 *
 *     model
 *     dataset
 *     objective
 *     optimizer
 *     loss
 *     strategy
 *     evidence
 *     feedback
 *     policy
 *     capability
 *     resource
 *     provenance
 *
 * These can be ordinary expressions resolved semantically.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file contains NO language-level finite limits.
 *
 * It MUST NOT define:
 *
 *     MAX_LEARNING_EFFECTS
 *     MAX_LEARNING_OPERATIONS
 *     MAX_LEARNING_ARGUMENTS
 *     MAX_MODELS
 *     MAX_DATASETS
 *     MAX_PARAMETERS
 *     MAX_TENSORS
 *     MAX_TENSOR_RANK
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * or equivalent language ceilings.
 *
 * Repetition is represented by grammar operators such as:
 *
 *     *
 *     +
 *
 * and by existing recursive grammar structures.
 *
 * Physical scalability is a resource/target question, not a grammar limit.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * Zamani cannot guarantee physically infinite computation.
 *
 * Production scalability instead means:
 *
 *     no artificial language-level ceiling.
 *
 * A source program remains expressible as scale increases until an actual
 * implementation, execution environment, policy, or available resource
 * becomes insufficient.
 *
 * Such failure MUST be represented downstream as a capability/resource/
 * feasibility result rather than as a grammar limitation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     source token stream
 *     grammar version
 *     canonical lexer
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend upon:
 *
 *     hardware availability
 *     model availability
 *     data availability
 *     network state
 *     current time
 *     randomness
 *     runtime state
 *     target selection
 *     scheduler state
 *     compiler host
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a learning effect MUST NOT execute learning.
 *
 * Parsing:
 *
 *     learning::update(model, data)
 *
 * MUST NOT:
 *
 *     load a model
 *     open a dataset
 *     access a file
 *     access a network
 *     allocate a device
 *     execute a kernel
 *     invoke an external service
 *     invoke native code
 *     modify persistent state
 *     modify compiler state
 *     modify runtime state
 *
 * All such behavior belongs downstream after semantic validation and
 * execution planning.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors originate here only when the source shape is malformed.
 *
 * Examples:
 *
 *     learning::
 *     learning::operation(
 *     learning::operation(,)
 *     learning::operation(
 *
 * Semantic errors belong downstream:
 *
 *     unknown effect
 *     unknown operation
 *     effect not classified as learning
 *     incompatible argument types
 *     missing capability
 *     insufficient resources
 *     forbidden policy
 *     invalid contract
 *     unsupported target
 *     unavailable model provider
 *
 * These MUST NOT be converted into parser errors.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Source order must be preserved.
 *
 * This grammar MUST NOT:
 *
 *     sort effect references
 *     deduplicate effect references
 *     normalize namespace aliases
 *     reorder arguments
 *     evaluate expressions
 *
 * Semantic normalization occurs downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST representation must preserve:
 *
 *     effect identity
 *     operation identity
 *     qualified-name segments
 *     operation arguments
 *     effect-set membership
 *     source order
 *     source span
 *     surrounding effect context
 *
 * Conceptually:
 *
 *     LearningEffectReference
 *         -> canonical effect reference representation
 *
 *     LearningEffectOperation
 *         -> canonical effect operation representation
 *
 * No backend-specific node is required.
 *
 * The grammar MUST NOT force:
 *
 *     NeuralModel
 *     GPUModel
 *     QPUModel
 *     PhysicalDataset
 *     DeviceModel
 *     TrainingKernel
 *     VendorModel
 *
 * AST types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must resolve:
 *
 *     learning effect identity
 *     operation declaration
 *     operation signature
 *     effect composition
 *     type compatibility
 *     effect compatibility
 *     capability requirements
 *     resource requirements
 *     contracts
 *     policies
 *     security constraints
 *     provenance
 *     reproducibility
 *     determinism
 *     domain crossings
 *
 * The grammar supplies structure only.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * The grammar creates no IR.
 *
 * The semantic lowering path is:
 *
 *     LearningEffects
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
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> distributed
 *          +--> accelerator
 *          +--> future domains
 *
 * No learning-specific IR may become a competing canonical representation.
 *
 * ============================================================================
 * QUANTUM IR INVARIANT
 * ============================================================================
 *
 * If a learning operation includes quantum computation:
 *
 *     learning
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum semantics
 *         |
 *         v
 *     quantum::ir
 *
 * The file MUST NOT introduce:
 *
 *     LearningQuantumIR
 *     QMLIR
 *     LearningQPUIR
 *
 * ============================================================================
 * EFFECT INFERENCE
 * ============================================================================
 *
 * Semantic effect inference may derive learning effects from source constructs.
 *
 * For example:
 *
 *     learn model from data;
 *
 * may semantically produce a learning effect.
 *
 * However, inference belongs to the semantic effect system.
 *
 * This grammar MUST NOT embed semantic effect inference.
 *
 * ============================================================================
 * EFFECT HANDLING
 * ============================================================================
 *
 * Learning effects may be handled through the generic effect handling system.
 *
 * Handler syntax remains owned by:
 *
 *     grammar/effects/effect-handling.g4
 *
 * This file MUST NOT define a second learning-specific handler language.
 *
 * ============================================================================
 * EFFECT POLYMORPHISM
 * ============================================================================
 *
 * Learning effects may participate in effect polymorphism.
 *
 * Effect variables and effect bounds remain owned by:
 *
 *     grammar/effects/effect-polymorphism.g4
 *
 * This file MUST NOT duplicate them.
 *
 * ============================================================================
 * EFFECT COMPOSITION
 * ============================================================================
 *
 * Learning may compose with:
 *
 *     io
 *     network
 *     randomness
 *     mutation
 *     distributed
 *     native
 *     foreign
 *     quantum
 *     hardware
 *     security
 *     memory
 *     simulation
 *     other future effects
 *
 * Composition is owned by the generic effect system.
 *
 * ============================================================================
 * GENERIC OPERATION USE
 * ============================================================================
 *
 * The canonical generic effect-operation syntax remains authoritative.
 *
 * Therefore this file wraps:
 *
 *     effectOperationUse
 *
 * rather than redefining its syntax.
 *
 * This ensures:
 *
 *     learning effect
 *          |
 *          v
 *     generic effect operation
 *          |
 *          v
 *     semantic classification
 *
 * instead of:
 *
 *     learning effect
 *          |
 *          v
 *     second operation language.
 *
 * ============================================================================
 * DOMAIN-NEUTRAL LEARNING REFERENCE
 * ============================================================================
 *
 * `learningEffectReference` is intentionally based on the canonical
 * `effectReference`.
 *
 * This means that domain identity is resolved semantically.
 *
 * A reference is therefore not accepted because a hard-coded parser catalogue
 * says that a particular learning operation exists.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `learningEffect` is the sole public entry point owned by this grammar.
 *
 * It provides an explicit domain boundary without changing generic effect
 * syntax.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar LearningEffects;

options {
    tokenVocab = ZamaniLexer;
}

import Core,
       Expressions,
       EffectOperations,
       EffectSets;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule provides the single public learning-effect boundary.
 *
 * It deliberately does not enumerate learning algorithms or operation names.
 * ============================================================================
 */

learningEffect
    : learningEffectReference
    | learningEffectReferenceList
    | learningEffectSet
    | learningEffectInvocation
    | learningEffectOperationUse
    | learningEffectOperationReference
    ;


/* ============================================================================
 * 2. LEARNING EFFECT REFERENCE
 * ============================================================================
 *
 * Delegates effect identity to the canonical effect subsystem.
 *
 * Examples of possible semantic identities:
 *
 *     learning::update
 *     learning::feedback
 *     learning::knowledge_update
 *     future::learning::operation
 *
 * Whether the resolved identity actually belongs to the learning domain is a
 * semantic question.
 * ============================================================================
 */

learningEffectReference
    : effectReference
    ;


/* ============================================================================
 * 3. LEARNING EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Delegates list syntax to the canonical effect subsystem.
 *
 * Source ordering is preserved.
 * ============================================================================
 */

learningEffectReferenceList
    : effectReferenceList
    ;


/* ============================================================================
 * 4. LEARNING EFFECT SET
 * ============================================================================
 *
 * Delegates effect-set syntax to the canonical effect subsystem.
 *
 * Examples:
 *
 *     {
 *         learning::update
 *     }
 *
 *     {
 *         learning::update,
 *         randomness::source,
 *         distributed::synchronize
 *     }
 *
 * The grammar does not impose an effect-count limit.
 * ============================================================================
 */

learningEffectSet
    : effectSet
    ;


/* ============================================================================
 * 5. LEARNING EFFECT OPERATION REFERENCE
 * ============================================================================
 *
 * Delegates operation identity to the generic effect-operation subsystem.
 *
 * No operation catalogue is embedded here.
 * ============================================================================
 */

learningEffectOperationReference
    : effectOperationReference
    ;


/* ============================================================================
 * 6. LEARNING EFFECT INVOCATION
 * ============================================================================
 *
 * Delegates invocation syntax to the canonical effect-operation grammar.
 *
 * Conceptual examples:
 *
 *     learning::update(model, data)
 *     learning::feedback(model, observation)
 *     learning::knowledge_update(state, evidence)
 *
 * These examples illustrate source shape only.
 *
 * They do not establish built-in operations.
 * ============================================================================
 */

learningEffectInvocation
    : effectInvocation
    ;


/* ============================================================================
 * 7. LEARNING EFFECT OPERATION USE
 * ============================================================================
 *
 * Delegates operation-use syntax to the canonical generic effect subsystem.
 *
 * This prevents the learning domain from developing a second operation-use
 * language.
 * ============================================================================
 */

learningEffectOperationUse
    : effectOperationUse
    ;


/* ============================================================================
 * 8. INTEGRATION INVARIANTS
 * ============================================================================
 *
 * The following invariants are normative.
 * ============================================================================
 *
 * INVARIANT 1
 *
 *     LearningEffects does not define lexer tokens.
 *
 * INVARIANT 2
 *
 *     LearningEffects does not define the `learn` statement.
 *
 * INVARIANT 3
 *
 *     LearningEffects does not define generic effect declarations.
 *
 * INVARIANT 4
 *
 *     LearningEffects does not define generic effect operation declarations.
 *
 * INVARIANT 5
 *
 *     LearningEffects does not define generic effect sets.
 *
 * INVARIANT 6
 *
 *     LearningEffects does not define generic effect handlers.
 *
 * INVARIANT 7
 *
 *     LearningEffects does not enumerate learning algorithms.
 *
 * INVARIANT 8
 *
 *     LearningEffects does not enumerate model types.
 *
 * INVARIANT 9
 *
 *     LearningEffects does not enumerate datasets.
 *
 * INVARIANT 10
 *
 *     LearningEffects does not select hardware.
 *
 * INVARIANT 11
 *
 *     LearningEffects does not select a runtime.
 *
 * INVARIANT 12
 *
 *     LearningEffects does not allocate resources.
 *
 * INVARIANT 13
 *
 *     LearningEffects does not resolve capabilities.
 *
 * INVARIANT 14
 *
 *     LearningEffects does not authorize execution.
 *
 * INVARIANT 15
 *
 *     LearningEffects does not create an IR.
 *
 * INVARIANT 16
 *
 *     Quantum computation continues through quantum::ir.
 *
 * INVARIANT 17
 *
 *     No universal physical capacity is represented by this grammar.
 *
 * INVARIANT 18
 *
 *     Parsing has no observable execution side effects.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No language-level capacity constants are present.
 *
 * This file MUST remain free of:
 *
 *     MAX_LEARNING_EFFECTS
 *     MAX_LEARNING_OPERATIONS
 *     MAX_LEARNING_ARGUMENTS
 *     MAX_MODELS
 *     MAX_DATASETS
 *     MAX_PARAMETERS
 *     MAX_TENSORS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * No physical identifier is encoded:
 *
 *     CPU_0
 *     GPU_0
 *     FPGA_0
 *     QPU_0
 *     NODE_0
 *     DEVICE_0
 *
 * No fixed hardware property is encoded:
 *
 *     fixed register width
 *     fixed bus width
 *     fixed memory capacity
 *     fixed tensor rank
 *     fixed qubit capacity
 *     fixed node count
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same learning-effect source structure must remain syntactically valid
 * regardless of eventual realization on:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud environments
 *     future computational substrates
 *
 * Target feasibility is evaluated downstream.
 *
 * If a realization cannot satisfy the semantic requirements, the compiler or
 * execution planner reports the appropriate:
 *
 *     capability failure
 *     resource failure
 *     policy failure
 *     contract failure
 *     unsupported-target result
 *
 * The grammar does not change to accommodate a larger or smaller machine.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Learning effects may coexist with:
 *
 *     classical effects
 *     quantum effects
 *     HDL effects
 *     hardware effects
 *     accelerator effects
 *     distributed effects
 *     networking effects
 *     security effects
 *     IO effects
 *     memory effects
 *     randomness effects
 *     mutation effects
 *     native effects
 *     foreign effects
 *     simulation effects
 *     future effect domains
 *
 * No domain requires a second learning grammar.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical learning may consume ordinary expressions and produce semantic
 * state changes.
 *
 * Example source shape:
 *
 *     learning::update(model, data)
 *
 * The grammar does not decide:
 *
 *     algorithm
 *     numeric representation
 *     storage
 *     scheduler
 *     processor
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A learning effect may be combined with quantum effects:
 *
 *     {
 *         learning::update,
 *         quantum::measurement
 *     }
 *
 * The learning grammar only preserves the effect structure.
 *
 * Semantic analysis determines whether the computation crosses the quantum
 * boundary.
 *
 * When it does:
 *
 *     semantic model
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * A learning operation may combine:
 *
 *     classical control
 *     tensor computation
 *     quantum computation
 *     measurement
 *     accelerator computation
 *     distributed computation
 *
 * This grammar imposes no ordering or implementation policy beyond the
 * structure supplied by the generic effect grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Learning may be accelerated or implemented through hardware.
 *
 * Hardware intent remains separate from the learning effect.
 *
 * The compiler may eventually lower semantic learning operations to:
 *
 *     CPU computation
 *     GPU computation
 *     FPGA implementation
 *     ASIC implementation
 *     accelerator implementation
 *     future hardware
 *
 * This file does not choose one.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed learning can compose with:
 *
 *     distributed::*
 *     networking::*
 *     synchronization effects
 *     communication effects
 *     persistence effects
 *
 * No worker or node count is encoded.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Learning effects may occur in concurrent computations.
 *
 * Concurrency semantics remain owned by:
 *
 *     grammar/concurrency/
 *     grammar/execution/
 *
 * This file does not define:
 *
 *     task
 *     actor
 *     channel
 *     scheduler
 *     async
 *     await
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Learning may manipulate valuable or protected computational state.
 *
 * Security analysis may therefore impose:
 *
 *     authorization
 *     sandboxing
 *     trust
 *     data policy
 *     model policy
 *     provenance requirements
 *     adaptation restrictions
 *
 * Security grammars remain authoritative.
 *
 * A syntactically valid learning effect is never proof of authorization.
 *
 * ============================================================================
 * RESOURCE NEGOTIATION
 * ============================================================================
 *
 * Learning effects may result in resource requirements.
 *
 * For example, semantic analysis may determine that a particular declared
 * learning operation requires:
 *
 *     capability("tensor.compute")
 *
 * or:
 *
 *     capability("distributed.compute")
 *
 * or:
 *
 *     capability("quantum.compute")
 *
 * Resource requirements may be symbolic and target-independent.
 *
 * This grammar does not encode the required capacity.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * A learning effect may participate in adaptive execution:
 *
 *     detect
 *     evaluate
 *     select
 *     specialize
 *     fallback
 *     retry
 *     recover
 *     simulate
 *
 * These are execution-planning concepts and remain downstream.
 *
 * Learning syntax remains unchanged when execution realization changes.
 *
 * ============================================================================
 * SIMULATION
 * ============================================================================
 *
 * Learning effects may be simulated.
 *
 * Simulation may be used for:
 *
 *     testing
 *     verification
 *     reproducibility
 *     target absence
 *     fault analysis
 *     performance analysis
 *     planning
 *
 * Simulation semantics belong to:
 *
 *     grammar/execution/
 *
 * This file does not introduce simulation syntax.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Learning effects may eventually map to external systems.
 *
 * Examples include:
 *
 *     model runtimes
 *     accelerator APIs
 *     external data systems
 *     distributed services
 *     foreign functions
 *     vendor dialects
 *     quantum tooling
 *
 * Such mapping belongs downstream under the interoperability/dialect system.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new semantic learning effect identity MUST NOT require a grammar
 * modification when its syntax can already be represented by:
 *
 *     effectReference
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *     effectSet
 *
 * Grammar changes are required only when source syntax itself changes.
 *
 * This keeps semantic extensibility separate from parser evolution.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to malformed source structure.
 *
 * Examples:
 *
 *     malformed qualified name
 *     malformed invocation
 *     malformed argument list
 *     malformed effect set
 *     malformed effect operation use
 *
 * Semantic diagnostics remain downstream:
 *
 *     unresolved learning effect
 *     unresolved operation
 *     non-learning effect used where learning is required
 *     invalid operation signature
 *     type mismatch
 *     unavailable capability
 *     insufficient resource
 *     forbidden policy
 *     invalid contract
 *     unsupported realization
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Positive structural fixtures should include:
 *
 *     learning::update
 *
 *     learning::update(model, data)
 *
 *     learning::feedback(model, observation)
 *
 *     learning::knowledge_update(state, evidence)
 *
 *     {
 *         learning::update
 *     }
 *
 *     {
 *         learning::update,
 *         randomness::source
 *     }
 *
 *     {
 *         learning::update,
 *         distributed::synchronize,
 *         networking::request
 *     }
 *
 *     {
 *         learning::update,
 *         quantum::measurement
 *     }
 *
 *     {
 *         learning::update,
 *         accelerator::compute
 *     }
 *
 *     future::learning::operation
 *
 *     vendor::learning::extension
 *
 * These examples establish syntax shape only.
 *
 * They do NOT establish built-in operations.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Malformed structures must be rejected by the canonical grammar.
 *
 * Examples:
 *
 *     learning::
 *
 *     ::learning::update
 *
 *     learning::update(
 *
 *     learning::update(,)
 *
 *     learning::update(,)
 *
 *     {
 *         learning::update,
 *     malformed
 *
 * Semantic failures must NOT be tested as parser failures.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     zero effect entries where the generic effect-set grammar permits them
 *     one effect
 *     many effects
 *     deeply qualified names
 *     nested expressions
 *     large argument lists
 *     large effect sets
 *     nested semantic contexts
 *     cross-domain effect combinations
 *
 * Test values are fixtures, not language limits.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Vary independently:
 *
 *     source size
 *     effect-set size
 *     qualified-name depth
 *     operation argument count
 *     number of learning operations
 *     number of modules
 *     number of concurrent operations
 *     number of semantic domains
 *
 * The grammar must continue to use structural repetition and recursive
 * composition rather than finite enumerations.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer vocabulary
 *     grammar version
 *     dialect configuration
 *
 * the parser must produce equivalent parse structures.
 *
 * Parsing must not depend on:
 *
 *     model availability
 *     accelerator availability
 *     network availability
 *     machine identity
 *     random state
 *     wall-clock time
 *     runtime state
 *     resource availability
 *
 * ============================================================================
 * ROUND-TRIP TEST CONTRACT
 * ============================================================================
 *
 * Where formatter/printer infrastructure exists:
 *
 *     source
 *       |
 *       v
 *     parse
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     format
 *       |
 *       v
 *     parse
 *
 * must preserve learning-effect semantics.
 *
 * At minimum, preserve:
 *
 *     effect identity
 *     operation identity
 *     operation argument ordering
 *     effect-set membership
 *     qualified-name structure
 *
 * Formatting may change whitespace and equivalent presentation.
 *
 * ============================================================================
 * COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Stable learning-effect syntax must be checked across:
 *
 *     specification
 *     lexer
 *     ANTLR parser
 *     Rust frontend parser
 *     AST
 *     semantic effect analysis
 *     canonical semantic representation
 *
 * A semantic addition that does not alter source syntax should not require a
 * grammar-version change.
 *
 * A source syntax change requires:
 *
 *     version identification
 *     migration information
 *     compatibility fixtures
 *     deprecation policy where applicable
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Consumers must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Safe Rust only.
 *
 * No unsafe implementation is required or permitted.
 *
 * The grammar itself cannot enforce Rust safety, therefore the consuming Rust
 * crate should enforce the repository's existing safe-Rust policy, preferably
 * at the crate boundary where compatible with the repository architecture.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCIES
 * ============================================================================
 *
 * Required dependency direction:
 *
 *     LearningEffects
 *          |
 *          +--> Core
 *          +--> Expressions
 *          +--> EffectOperations
 *          +--> EffectSets
 *
 * Forbidden:
 *
 *     Effects -> LearningEffects -> Effects
 *
 * Forbidden:
 *
 *     LearningEffects -> semantic runtime -> LearningEffects
 *
 * Forbidden:
 *
 *     LearningEffects -> quantum::ir
 *
 * Forbidden:
 *
 *     LearningEffects -> hardware discovery
 *
 * The grammar remains upstream of semantic realization.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 * --------
 *
 * No lexer modification is required solely for this file.
 *
 * The canonical lexer remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * --------------------------------------------------------------------------
 *
 * 2. GENERIC EFFECT SYSTEM
 * ------------------------
 *
 * This file consumes:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *
 * It does not replace them.
 *
 * --------------------------------------------------------------------------
 *
 * 3. LEARN STATEMENT
 * ------------------
 *
 * Source-level learning statements remain owned by:
 *
 *     grammar/statements/learn.g4
 *
 * The semantic layer may associate a `learn` operation with a learning effect.
 *
 * No import from this file back into the statement grammar is required merely
 * to recognize `learn`.
 *
 * --------------------------------------------------------------------------
 *
 * 4. AI SUBSYSTEM
 * ---------------
 *
 * AI grammar components may consume:
 *
 *     learningEffect
 *
 * where an explicit learning-effect boundary is required.
 *
 * The AI subsystem remains responsible for AI-domain semantic constructs.
 *
 * --------------------------------------------------------------------------
 *
 * 5. EFFECT COMPOSITION ROOT
 * --------------------------
 *
 * `grammar/effects/effects.g4` remains generic.
 *
 * It should NOT import LearningEffects merely to enumerate learning.
 *
 * Learning effect identities are already representable through the open-world
 * generic effect machinery.
 *
 * This avoids turning the generic effect root into a domain catalogue.
 *
 * --------------------------------------------------------------------------
 *
 * 6. AST
 * -----
 *
 * The frontend maps the parser contexts into existing generic effect
 * structures.
 *
 * No backend-specific learning node is required merely because the source
 * construct belongs to the learning domain.
 *
 * --------------------------------------------------------------------------
 *
 * 7. SEMANTICS
 * ------------
 *
 * Semantic resolution performs:
 *
 *     effect lookup
 *          |
 *          v
 *     domain classification
 *          |
 *          v
 *     learning validation
 *          |
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource checking
 *          +--> contract checking
 *          +--> policy checking
 *          +--> provenance
 *
 * --------------------------------------------------------------------------
 *
 * 8. IR
 * ----
 *
 * There is no learning-effect IR introduced here.
 *
 * Semantic lowering continues through the repository's canonical semantic
 * representation.
 *
 * If quantum computation is involved:
 *
 *     quantum::ir
 *
 * remains the canonical quantum representation.
 *
 * --------------------------------------------------------------------------
 *
 * 9. EXECUTION
 * ------------
 *
 * Execution planning may select:
 *
 *     local execution
 *     parallel execution
 *     distributed execution
 *     accelerator execution
 *     quantum-assisted execution
 *     simulation
 *     fallback execution
 *     recovery execution
 *
 * The source grammar does not select among them.
 *
 * --------------------------------------------------------------------------
 *
 * 10. PROVENANCE
 * --------------
 *
 * The semantic/provenance layer may record learning lineage.
 *
 * This grammar preserves the source structure necessary for that record.
 *
 * --------------------------------------------------------------------------
 *
 * 11. SECURITY
 * ------------
 *
 * Security/policy layers may restrict learning effects.
 *
 * Parser acceptance is not authorization.
 *
 * --------------------------------------------------------------------------
 *
 * 12. TESTS
 * ---------
 *
 * Required test directory:
 *
 *     grammar/tests/effects/learning/
 *
 * Required categories:
 *
 *     positive
 *     negative
 *     boundary
 *     scalability
 *     determinism
 *     cross-domain
 *     compatibility
 *     round-trip
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] LearningEffects is the grammar identity.
 *
 * [x] The canonical ZamaniLexer vocabulary is consumed.
 *
 * [x] No lexer rules are introduced.
 *
 * [x] No new learning keyword is required.
 *
 * [x] Generic effect references are delegated to EffectSets.
 *
 * [x] Generic effect sets are delegated to EffectSets.
 *
 * [x] Generic operation references are delegated to EffectOperations.
 *
 * [x] Generic invocations are delegated to EffectOperations.
 *
 * [x] Generic operation use is delegated to EffectOperations.
 *
 * [x] The `learn` statement remains owned by statements/learn.g4.
 *
 * [x] No learning algorithm is enumerated.
 *
 * [x] No model architecture is enumerated.
 *
 * [x] No dataset type is enumerated.
 *
 * [x] No tensor limit is encoded.
 *
 * [x] No physical resource limit is encoded.
 *
 * [x] No target is selected.
 *
 * [x] No capability is discovered.
 *
 * [x] No resource is allocated.
 *
 * [x] No runtime action occurs.
 *
 * [x] No security decision occurs during parsing.
 *
 * [x] No provenance record is created during parsing.
 *
 * [x] No IR is introduced.
 *
 * [x] Quantum computation remains downstream through quantum::ir.
 *
 * [x] HDL/hardware realization remains downstream.
 *
 * [x] Distributed realization remains downstream.
 *
 * [x] FFI/ABI remains downstream.
 *
 * [x] Policies remain owned by the policy/security systems.
 *
 * [x] Contracts remain owned by the validation system.
 *
 * [x] Provenance remains owned by provenance infrastructure.
 *
 * [x] Parsing remains deterministic.
 *
 * [x] Source order remains preserved.
 *
 * [x] The grammar is open-world.
 *
 * [x] There is no language-level finite machine capacity.
 *
 * [x] The grammar contains no unsafe implementation.
 *
 * [x] Rust 1.97 / 1.97.1 and Edition 2021 remain the consumer baseline.
 *
 * [x] Integration boundaries are documented.
 *
 * [x] Test ownership is documented.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This grammar answers exactly one question:
 *
 *     "What parser-level structures can participate in the learning-effect
 *      semantic boundary?"
 *
 * It does NOT answer:
 *
 *     "Which learning algorithm runs?"
 *     "Which model is selected?"
 *     "Which dataset is loaded?"
 *     "Which processor executes it?"
 *     "Which accelerator executes it?"
 *     "Which QPU executes it?"
 *     "Which device is selected?"
 *     "Which resource is allocated?"
 *     "Which capability is available?"
 *     "Which policy authorizes it?"
 *     "Which backend is selected?"
 *
 * Those decisions belong downstream.
 *
 * The permanent architectural relationship is:
 *
 *     learning syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     learning semantic model
 *          |
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> distributed
 *          +--> accelerator
 *          +--> future domains
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     target-aware realization
 *
 * This preserves Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 * without converting current physical-machine characteristics into permanent
 * language limits.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */