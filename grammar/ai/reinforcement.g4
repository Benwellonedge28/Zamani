/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/reinforcement.g4
 *
 * GRAMMAR
 * -------
 * AIReinforcement
 *
 * STATUS
 * ------
 * CANONICAL AI-DOMAIN REINFORCEMENT-LEARNING COMPOSITION GRAMMAR
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97 or later
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the source-level structural vocabulary required to express
 * reinforcement-learning intent without embedding a particular reinforcement
 * algorithm, runtime, machine, accelerator, simulator, quantum processor,
 * hardware topology, or numerical representation.
 *
 * Reinforcement learning is treated as a semantic computational pattern:
 *
 *     observation/state
 *          |
 *          v
 *     decision/policy
 *          |
 *          v
 *       action
 *          |
 *          v
 *     environment/system
 *          |
 *          v
 *       outcome
 *          |
 *          v
 *       reward
 *          |
 *          v
 *       feedback
 *          |
 *          v
 *       learning
 *
 * The grammar describes this intent.
 *
 * It does NOT implement the learning algorithm.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
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
 *     AIReinforcement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------+-------------------+-------------------+
 *          |                   |                   |                   |
 *          v                   v                   v                   v
 *        types             effects            resources           capabilities
 *          |                   |                   |                   |
 *          +-------------------+-------------------+-------------------+
 *                              |
 *                              v
 *                         contracts
 *                              |
 *                              v
 *                           policies
 *                              |
 *                              v
 *                         provenance
 *                              |
 *                              v
 *                     reinforcement semantic model
 *                              |
 *                 +------------+-------------+
 *                 |                          |
 *                 v                          v
 *            learning                    decision
 *                 |                          |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+----------------+
 *              |                                |
 *              v                                v
 *         classical semantics             quantum semantics
 *                                                   |
 *                                                   v
 *                                              quantum::ir
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         specialization
 *                              |
 *                           lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / recovery
 *                              |
 *                         ZQN / HAL
 *                              |
 *                              v
 *                       target realization
 *
 * ============================================================================
 * CORE PRINCIPLES
 * ============================================================================
 *
 * 1. OPEN WORLD
 *
 * New reinforcement algorithms, environments, policy representations,
 * value-function representations, exploration strategies, reward models,
 * simulation technologies, hardware targets, and learning systems must not
 * require changes to this grammar merely because they acquire new names.
 *
 *
 * 2. INTENT, NOT IMPLEMENTATION
 *
 * The grammar describes what the program means.
 *
 * It does not prescribe:
 *
 *     - a specific optimizer;
 *     - a specific policy representation;
 *     - a specific value representation;
 *     - a specific neural architecture;
 *     - a specific sampling algorithm;
 *     - a specific exploration algorithm;
 *     - a specific simulator;
 *     - a specific accelerator;
 *     - a specific CPU;
 *     - a specific GPU;
 *     - a specific FPGA;
 *     - a specific ASIC;
 *     - a specific QPU.
 *
 *
 * 3. NO PHYSICAL LIMITS
 *
 * This grammar contains no universal constants for:
 *
 *     episodes
 *     steps
 *     states
 *     actions
 *     observations
 *     agents
 *     environments
 *     parameters
 *     model size
 *     workers
 *     threads
 *     processes
 *     nodes
 *     devices
 *     accelerators
 *     GPUs
 *     QPUs
 *     qubits
 *     memory
 *     tensor rank
 *     tensor dimensions
 *
 * Repetition is expressed with grammar repetition operators or ordinary
 * expressions.
 *
 *
 * 4. SEMANTIC EXTENSIBILITY
 *
 * Algorithm names and implementation details are semantic values.
 *
 * Examples include:
 *
 *     algorithm
 *     optimizer
 *     policy representation
 *     exploration strategy
 *     value estimator
 *     reward shaping
 *     replay strategy
 *     sampling strategy
 *     update rule
 *     environment implementation
 *
 * These are not closed grammar enumerations.
 *
 *
 * 5. SHARED OWNERSHIP
 *
 * This file composes existing universal systems.
 *
 * It does not create competing versions of:
 *
 *     learning
 *     training
 *     feedback
 *     decisions
 *     policies
 *     contracts
 *     resources
 *     capabilities
 *     effects
 *     provenance
 *     expressions
 *     types
 *     concurrency
 *     actors
 *     quantum operations
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     reinforcementConstruct
 *     reinforcementDeclaration
 *     reinforcementItem
 *     reinforcementStateItem
 *     reinforcementActionItem
 *     reinforcementObservationItem
 *     reinforcementRewardItem
 *     reinforcementTransitionItem
 *     reinforcementEpisodeItem
 *     reinforcementPolicyItem
 *     reinforcementValueItem
 *     reinforcementObjectiveItem
 *     reinforcementEnvironmentItem
 *     reinforcementExplorationItem
 *     reinforcementEvaluationItem
 *     reinforcementLearningItem
 *     reinforcementFeedbackItem
 *     reinforcementDecisionItem
 *     reinforcementConstraintItem
 *     reinforcementResourceItem
 *     reinforcementCapabilityItem
 *     reinforcementEffectItem
 *     reinforcementContractItem
 *     reinforcementProvenanceItem
 *     reinforcementAttributeItem
 *     reinforcementMetadataItem
 *     reinforcementNamedItem
 *     reinforcementStatement
 *     reinforcementExpression
 *     reinforcementOperation
 *     reinforcementArgumentList
 *     reinforcementArgument
 *     reinforcementNamedArgument
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - literals;
 *     - general expressions;
 *     - expression precedence;
 *     - types;
 *     - functions;
 *     - declarations;
 *     - modules;
 *     - general statements;
 *     - `learn` statement syntax;
 *     - `train` syntax;
 *     - feedback syntax;
 *     - decision syntax;
 *     - policy syntax;
 *     - contract syntax;
 *     - resource syntax;
 *     - capability syntax;
 *     - effect syntax;
 *     - provenance syntax;
 *     - actor syntax;
 *     - concurrency syntax;
 *     - distributed syntax;
 *     - quantum operation syntax;
 *     - HDL syntax;
 *     - hardware realization;
 *     - target selection;
 *     - optimization;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Universal expression and naming infrastructure:
 *
 *     ../expressions/expressions
 *     ../types/types
 *     ../core/names
 *     ../core/attributes
 *     ../core/modifiers
 *     ../core/metadata
 *
 * Existing AI composition:
 *
 *     ../ai/learning
 *     ../ai/training
 *     ../ai/feedback
 *     ../ai/decisions
 *     ../ai/provenance
 *
 * Universal semantic systems:
 *
 *     ../resources/*
 *     ../effects/*
 *     ../validation/*
 *     ../policies/*
 *
 * IMPORTANT:
 *
 * The grammar does not need to import every semantic subsystem merely to
 * reference it conceptually. Shared semantic ownership is resolved by the
 * compiler's semantic model.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PUBLIC:
 *
 *     reinforcementConstruct
 *     reinforcementDeclaration
 *     reinforcementStatement
 *     reinforcementExpression
 *
 * All remaining rules are implementation details unless explicitly consumed
 * by the AI composition grammar.
 *
 * ============================================================================
 * CONSUMER CONTRACT
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * AI.g4 should import AIReinforcement and admit:
 *
 *     reinforcementConstruct
 *
 * as one AI-domain alternative.
 *
 * AI.g4 MUST NOT reproduce the rules in this file.
 *
 * ============================================================================
 * RELATIONSHIP TO LEARNING
 * ============================================================================
 *
 * Reinforcement learning is a specialization of the universal learning
 * semantic operation.
 *
 * This file therefore MUST NOT define another `learn` statement.
 *
 * The semantic relationship is:
 *
 *     reinforcement declaration
 *             |
 *             v
 *     learning intent
 *             |
 *             v
 *     learning semantic operation
 *
 * The canonical source-level learn statement remains owned by:
 *
 *     grammar/statements/learn.g4
 *
 * The AI composition adapter remains:
 *
 *     grammar/ai/learning.g4
 *
 * ============================================================================
 * RELATIONSHIP TO TRAINING
 * ============================================================================
 *
 * Training describes execution of a learning process.
 *
 * Reinforcement learning describes the interaction structure that supplies
 * observations, actions, outcomes, rewards, and updates.
 *
 * Existing training grammar remains authoritative for training declarations.
 *
 * This file may reference training through ordinary expressions or named
 * semantic fields, but must not duplicate training grammar.
 *
 * ============================================================================
 * RELATIONSHIP TO FEEDBACK
 * ============================================================================
 *
 * Reward and outcome information may be represented as feedback.
 *
 * Existing:
 *
 *     grammar/ai/feedback.g4
 *
 * owns feedback syntax.
 *
 * This file therefore represents reinforcement feedback as expressions and
 * semantic fields rather than creating a competing feedback grammar.
 *
 * Semantic path:
 *
 *     environment
 *          |
 *          v
 *       outcome
 *          |
 *          v
 *       reward
 *          |
 *          v
 *       feedback
 *          |
 *          v
 *       learning
 *
 * ============================================================================
 * RELATIONSHIP TO DECISIONS
 * ============================================================================
 *
 * Selecting an action is a decision.
 *
 * Existing:
 *
 *     grammar/ai/decisions.g4
 *
 * owns general decision syntax.
 *
 * Reinforcement learning may therefore reference a decision or policy as an
 * expression.
 *
 * This file does not create a second decision system.
 *
 * ============================================================================
 * RELATIONSHIP TO POLICIES
 * ============================================================================
 *
 * A reinforcement policy is a semantic object representing how actions are
 * selected from relevant information.
 *
 * The word `policy` therefore has semantic significance here, but policy
 * enforcement remains owned by the universal policy subsystem.
 *
 * A reinforcement policy may be:
 *
 *     - deterministic;
 *     - stochastic;
 *     - learned;
 *     - supplied externally;
 *     - derived;
 *     - composed;
 *     - constrained;
 *     - adaptive;
 *     - user-defined;
 *     - provided by a dialect.
 *
 * The grammar deliberately does not enumerate these as closed alternatives.
 *
 * ============================================================================
 * RELATIONSHIP TO ADAPTATION
 * ============================================================================
 *
 * Reinforcement learning can produce information used by controlled
 * adaptation.
 *
 * It must not imply unrestricted executable-code mutation.
 *
 * The semantic path is:
 *
 *     observation
 *        |
 *        v
 *     decision
 *        |
 *        v
 *      action
 *        |
 *        v
 *      reward
 *        |
 *        v
 *     learning
 *        |
 *        v
 *     candidate update
 *        |
 *        v
 *     policy / contract / capability / authorization checks
 *        |
 *        v
 *     controlled adaptation
 *
 * ============================================================================
 * STATE CONTRACT
 * ============================================================================
 *
 * A state is an ordinary expression.
 *
 * The grammar does not define:
 *
 *     StateType
 *     FixedState
 *     FiniteStateCount
 *     StateIndexWidth
 *
 * A state may be:
 *
 *     scalar
 *     tuple
 *     record
 *     graph
 *     tensor
 *     symbolic structure
 *     probabilistic structure
 *     quantum-derived value
 *     hybrid value
 *     distributed observation
 *     user-defined type
 *
 * ============================================================================
 * ACTION CONTRACT
 * ============================================================================
 *
 * An action is an ordinary expression.
 *
 * Actions may represent:
 *
 *     values
 *     function calls
 *     operations
 *     messages
 *     resource choices
 *     quantum operations
 *     hardware operations
 *     control decisions
 *     user-defined actions
 *
 * The grammar does not enumerate action kinds.
 *
 * ============================================================================
 * OBSERVATION CONTRACT
 * ============================================================================
 *
 * Observations are ordinary expressions.
 *
 * They may come from:
 *
 *     computation
 *     data
 *     simulation
 *     sensors
 *     hardware
 *     measurement
 *     quantum execution
 *     distributed systems
 *     another agent
 *     environment state
 *
 * ============================================================================
 * REWARD CONTRACT
 * ============================================================================
 *
 * Reward is deliberately represented by an expression.
 *
 * It may be:
 *
 *     scalar
 *     vector
 *     structured value
 *     distribution
 *     uncertain value
 *     symbolic value
 *     tensor
 *     quantum-derived measurement
 *     user-defined value
 *
 * The semantic layer determines whether the selected learning formulation
 * accepts that representation.
 *
 * No universal numeric width or precision is imposed.
 *
 * ============================================================================
 * TRANSITION CONTRACT
 * ============================================================================
 *
 * A transition represents semantic movement from one relevant configuration
 * to another.
 *
 * The grammar permits:
 *
 *     state
 *     action
 *     next_state
 *     reward
 *     observation
 *     terminal
 *
 * without requiring a particular mathematical representation.
 *
 * ============================================================================
 * EPISODE CONTRACT
 * ============================================================================
 *
 * An episode is a semantic grouping of interactions.
 *
 * It is not required to have a fixed number of steps.
 *
 * The grammar therefore uses zero-or-more items and ordinary expressions.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * The policy expression identifies the strategy used to select actions.
 *
 * No algorithm names are reserved.
 *
 * Therefore all of the following may remain ordinary semantic identifiers:
 *
 *     a user-defined policy
 *     a library policy
 *     a vendor policy
 *     a learned policy
 *     a symbolic policy
 *     a neural policy
 *     a probabilistic policy
 *     a quantum-assisted policy
 *     a hybrid policy
 *
 * ============================================================================
 * VALUE CONTRACT
 * ============================================================================
 *
 * A value expression can represent:
 *
 *     expected return
 *     utility
 *     state value
 *     action value
 *     advantage
 *     risk
 *     preference
 *     user-defined value representation
 *
 * The grammar does not impose mathematical definitions.
 *
 * ============================================================================
 * OBJECTIVE CONTRACT
 * ============================================================================
 *
 * The objective is an expression.
 *
 * It may encode:
 *
 *     return
 *     utility
 *     reward
 *     risk
 *     constraint satisfaction
 *     multi-objective optimization
 *     resource efficiency
 *     robustness
 *     reliability
 *     user-defined criteria
 *
 * The semantic layer validates compatibility.
 *
 * ============================================================================
 * EXPLORATION CONTRACT
 * ============================================================================
 *
 * Exploration configuration is an expression.
 *
 * This avoids grammar-level enumeration of exploration algorithms.
 *
 * No fixed exploration schedule, probability, temperature, decay constant,
 * action count, or step count is embedded in the grammar.
 *
 * ============================================================================
 * EVALUATION CONTRACT
 * ============================================================================
 *
 * Evaluation may reference:
 *
 *     objective
 *     reward
 *     return
 *     value
 *     confidence
 *     uncertainty
 *     evidence
 *     provenance
 *     policy
 *     constraints
 *
 * Evaluation semantics are downstream.
 *
 * ============================================================================
 * MULTI-AGENT CONTRACT
 * ============================================================================
 *
 * Multi-agent reinforcement learning must integrate with the existing actor
 * and concurrency systems.
 *
 * This file does NOT create:
 *
 *     ReinforcementActor
 *     ReinforcementChannel
 *     ReinforcementScheduler
 *
 * An agent may instead be an ordinary expression referencing the existing
 * actor/agent semantic model.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Reinforcement learning may be realized sequentially, concurrently,
 * distributed, federated, heterogeneous, or through future execution models.
 *
 * No worker/node/device count is embedded.
 *
 * Resource selection remains downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Reinforcement learning may consume quantum-derived observations, use
 * quantum operations, or participate in hybrid computation.
 *
 * This grammar does not define quantum operations.
 *
 * Quantum semantics MUST eventually use:
 *
 *     quantum::ir
 *
 * There must be no:
 *
 *     ReinforcementQuantumIR
 *     RLQuantumIR
 *     ReinforcementQMLIR
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Reinforcement learning may control or optimize hardware/HDL computation.
 *
 * Hardware realization remains outside this grammar.
 *
 * Any physical resource requirement is represented semantically through the
 * universal resource/capability systems.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource information may be supplied using ordinary expressions or the
 * universal resource syntax.
 *
 * Examples of semantic intent include:
 *
 *     resource: training_resources;
 *     capability: required_capability;
 *     constraint: resource_constraint;
 *
 * This grammar never encodes:
 *
 *     fixed worker count
 *     fixed thread count
 *     fixed memory size
 *     fixed accelerator count
 *     fixed node count
 *     fixed device count
 *     fixed QPU count
 *     fixed qubit count
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities remain open-world.
 *
 * Examples:
 *
 *     capability("learning")
 *     capability("model.execute")
 *     capability("tensor.compute")
 *     capability("distributed.compute")
 *     capability("quantum.measurement")
 *
 * are semantic capability identifiers, not grammar-level enumerations.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Reinforcement learning may cause semantic effects including:
 *
 *     learning
 *     mutation
 *     randomness
 *     io
 *     network
 *     distributed
 *     measurement
 *     simulation
 *     native
 *     foreign
 *     adaptation
 *
 * Effects are resolved by the existing effect system.
 *
 * This grammar creates no competing effect taxonomy.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Reinforcement constructs may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by the validation subsystem.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may govern:
 *
 *     action eligibility
 *     reward handling
 *     data access
 *     model updates
 *     exploration
 *     adaptation
 *     resource use
 *     network access
 *     external calls
 *     simulation
 *     deployment
 *     reproducibility
 *     security
 *
 * Policy enforcement remains outside this grammar.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Reinforcement execution can produce decisions and learned state.
 *
 * Provenance should therefore preserve, where applicable:
 *
 *     source
 *     observation origin
 *     action
 *     reward origin
 *     transition
 *     policy
 *     model
 *     learning operation
 *     evidence
 *     evaluation
 *     resource realization
 *     capability realization
 *     semantic transformation
 *     execution result
 *     adaptation result
 *     toolchain/language version
 *     reproducibility metadata
 *
 * This grammar only preserves parser structure/source locations.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The parser must not inspect:
 *
 *     hardware
 *     runtime resources
 *     random sources
 *     network state
 *     current model state
 *     wall-clock time
 *     environment state
 *
 * Runtime stochasticity is a semantic property and must be explicitly
 * represented by the semantic/effect system where required.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar is unbounded in the language-design sense.
 *
 * There are no grammar-level finite limits on:
 *
 *     states
 *     actions
 *     observations
 *     transitions
 *     episodes
 *     agents
 *     environments
 *     objectives
 *     constraints
 *     policies
 *     models
 *     parameters
 *     resources
 *     capabilities
 *
 * The practical limit is imposed by available compiler/runtime resources,
 * representation limits, and target feasibility, not by an artificial
 * language constant.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A reinforcement-learning source program describes portable computational
 * intent.
 *
 * Target realization may differ across:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future targets
 *
 * The source grammar does not select the target.
 *
 * The compiler resolves realization through:
 *
 *     semantics
 *       |
 *       v
 *     capabilities
 *       |
 *       v
 *     resources
 *       |
 *       v
 *     policies
 *       |
 *       v
 *     execution planning
 *       |
 *       v
 *     lowering
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * `REINFORCEMENT` is already part of the canonical lexical vocabulary.
 *
 * No new reinforcement-specific keyword is required by this grammar.
 *
 * Existing ordinary identifiers remain available for future algorithms and
 * representations.
 *
 * This is intentional: algorithm names must not become reserved words.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics are limited to malformed syntax.
 *
 * Semantic diagnostics belong downstream and include, where applicable:
 *
 *     invalid state representation
 *     invalid action representation
 *     invalid observation representation
 *     invalid reward representation
 *     invalid transition
 *     invalid policy
 *     incompatible objective
 *     invalid value relation
 *     invalid feedback relation
 *     unsatisfied capability
 *     insufficient resources
 *     forbidden effect
 *     violated contract
 *     violated policy
 *     invalid provenance
 *     invalid adaptation authorization
 *     invalid quantum semantic composition
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral AST must preserve:
 *
 *     construct kind
 *     optional identity
 *     ordered items
 *     state expression
 *     observation expression
 *     action expression
 *     reward expression
 *     transition expression
 *     episode information
 *     policy expression
 *     value expression
 *     objective expression
 *     environment expression
 *     exploration expression
 *     evaluation expression
 *     learning expression
 *     feedback expression
 *     decision expression/reference
 *     constraints
 *     resources
 *     capabilities
 *     effects
 *     contracts
 *     policies
 *     provenance references
 *     attributes
 *     metadata
 *     arbitrary named semantic extensions
 *     source span
 *
 * The AST MUST NOT contain:
 *
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     ASIC IDs
 *     QPU IDs
 *     physical qubit mappings
 *     hardware coupling maps
 *     pulse schedules
 *     calibration data
 *     backend-specific instructions
 *
 * unless such information is explicitly represented by a separate downstream
 * target description rather than the language AST.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * There must be no:
 *
 *     ReinforcementIR
 *     ReinforcementLearningIR
 *     RLIR
 *     NeuralRLIR
 *
 * Learning semantics lower through the canonical semantic representation.
 *
 * If classical computation is required, the classical IR owns its realization.
 *
 * If quantum computation is required, the quantum semantic portion lowers
 * through:
 *
 *     quantum::ir
 *
 * The reinforcement grammar never bypasses that boundary.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backend realization may choose:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     distributed system
 *     heterogeneous target
 *     future target
 *
 * based on semantic requirements and negotiated capabilities.
 *
 * This grammar never selects one.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required test owner:
 *
 *     grammar/tests/ai/reinforcement/
 *
 * Recommended structure:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     cross-domain/
 *     determinism/
 *     compatibility/
 *     round-trip/
 *     composition/
 *
 * Minimum semantic integration cases:
 *
 *     state + action
 *     observation + action + reward
 *     transition
 *     episode
 *     policy
 *     objective
 *     exploration
 *     evaluation
 *     learning
 *     feedback
 *     decision
 *     resource
 *     capability
 *     effect
 *     contract
 *     policy
 *     provenance
 *     multi-agent
 *     distributed
 *     quantum/hybrid
 *     simulation
 *     hardware/resource negotiation
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 *
 * The following rules intentionally use ordinary expressions wherever possible.
 *
 * This prevents a closed algorithm catalog and preserves POCO-REAF.
 *
 * ============================================================================
 */

grammar AIReinforcement {

    import
        ../expressions/expressions,
        ../types/types,
        ../core/names,
        ../core/attributes,
        ../core/modifiers,
        ../core/metadata,
        ../ai/learning,
        ../ai/training,
        ../ai/feedback,
        ../ai/decisions,
        ../ai/provenance
    ;

    /*
     * ========================================================================
     * PUBLIC COMPOSITION ENTRY
     * ========================================================================
     *
     * This is the single AI-domain entry point exported by this file.
     */
    reinforcementConstruct
        : reinforcementDeclaration
        | reinforcementStatement
        | reinforcementExpression
        ;


    /*
     * ========================================================================
     * DECLARATION
     * ========================================================================
     *
     * Canonical structural form:
     *
     *     reinforcement policy {
     *         state: state_value;
     *         action: action_value;
     *         reward: reward_value;
     *     }
     *
     * The identity and body are semantic; the grammar remains open-ended.
     */
    reinforcementDeclaration
        : REINFORCEMENT
          reinforcementName?
          LBRACE
          reinforcementItem*
          RBRACE
        ;


    /*
     * ========================================================================
     * NAME
     * ========================================================================
     */
    reinforcementName
        : qualifiedName
        ;


    /*
     * ========================================================================
     * DECLARATION ITEMS
     * ========================================================================
     *
     * Ordered repetition preserves source ordering.
     *
     * No item-count limit exists.
     */
    reinforcementItem
        : reinforcementStateItem
        | reinforcementActionItem
        | reinforcementObservationItem
        | reinforcementRewardItem
        | reinforcementTransitionItem
        | reinforcementEpisodeItem
        | reinforcementPolicyItem
        | reinforcementValueItem
        | reinforcementObjectiveItem
        | reinforcementEnvironmentItem
        | reinforcementExplorationItem
        | reinforcementEvaluationItem
        | reinforcementLearningItem
        | reinforcementFeedbackItem
        | reinforcementDecisionItem
        | reinforcementConstraintItem
        | reinforcementResourceItem
        | reinforcementCapabilityItem
        | reinforcementEffectItem
        | reinforcementContractItem
        | reinforcementProvenanceItem
        | reinforcementAttributeItem
        | reinforcementMetadataItem
        | reinforcementNamedItem
        ;


    /*
     * ========================================================================
     * STATE
     * ========================================================================
     *
     * The value is deliberately an expression.
     */
    reinforcementStateItem
        : STATE COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * ACTION
     * ========================================================================
     */
    reinforcementActionItem
        : ACTION COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * OBSERVATION
     * ========================================================================
     */
    reinforcementObservationItem
        : OBSERVATION COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * REWARD
     * ========================================================================
     *
     * REWARD is represented through a named semantic field rather than a
     * closed numerical grammar.
     *
     * The lexer does not currently require a universal REWARD token.
     *
     * Therefore the field is accepted through the open named-item mechanism
     * below when the canonical lexer treats `reward` as an identifier.
     *
     * If a future canonical REWARD token is introduced, the semantic owner
     * may add a dedicated adapter without changing the reinforcement model.
     *
     * To keep this file compatible with the current lexical vocabulary, the
     * canonical dedicated rule is intentionally identifier-based.
     */
    reinforcementRewardItem
        : reinforcementNamedSemanticItem
        ;


    /*
     * ========================================================================
     * TRANSITION
     * ========================================================================
     */
    reinforcementTransitionItem
        : TRANSITION COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * EPISODE
     * ========================================================================
     */
    reinforcementEpisodeItem
        : EPISODE COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * POLICY
     * ========================================================================
     *
     * The expression identifies the policy semantic object.
     *
     * Policy implementation/enforcement remains elsewhere.
     */
    reinforcementPolicyItem
        : POLICY COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * VALUE
     * ========================================================================
     *
     * `value` is represented as an open semantic field so the grammar does
     * not require a universal VALUE keyword.
     */
    reinforcementValueItem
        : reinforcementNamedSemanticItem
        ;


    /*
     * ========================================================================
     * OBJECTIVE
     * ========================================================================
     */
    reinforcementObjectiveItem
        : OBJECTIVE COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * ENVIRONMENT
     * ========================================================================
     *
     * The environment is an ordinary semantic expression.
     */
    reinforcementEnvironmentItem
        : ENVIRONMENT COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * EXPLORATION
     * ========================================================================
     *
     * Exploration is deliberately open-world.
     *
     * An algorithm/strategy name is an expression rather than a grammar
     * enumeration.
     */
    reinforcementExplorationItem
        : EXPLORATION COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * EVALUATION
     * ========================================================================
     */
    reinforcementEvaluationItem
        : EVALUATE COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * LEARNING
     * ========================================================================
     *
     * The value is a reference/expression associated with the canonical
     * learning subsystem.
     *
     * This does NOT duplicate `learnStatement`.
     */
    reinforcementLearningItem
        : LEARNING COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * FEEDBACK
     * ========================================================================
     */
    reinforcementFeedbackItem
        : FEEDBACK COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * DECISION
     * ========================================================================
     *
     * The expression may refer to a decision owned by decisions.g4.
     */
    reinforcementDecisionItem
        : DECISION COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * CONSTRAINT
     * ========================================================================
     */
    reinforcementConstraintItem
        : CONSTRAINT COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * RESOURCE
     * ========================================================================
     *
     * This is an expression-level reference into the universal resource
     * subsystem. It does not duplicate resource grammar.
     */
    reinforcementResourceItem
        : RESOURCE COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * CAPABILITY
     * ========================================================================
     */
    reinforcementCapabilityItem
        : CAPABILITY COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * EFFECT
     * ========================================================================
     */
    reinforcementEffectItem
        : EFFECT COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * CONTRACT
     * ========================================================================
     *
     * The validation subsystem owns contract semantics.
     */
    reinforcementContractItem
        : CONTRACT COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * PROVENANCE
     * ========================================================================
     */
    reinforcementProvenanceItem
        : PROVENANCE COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * ATTRIBUTES
     * ========================================================================
     */
    reinforcementAttributeItem
        : attributeBlock
        ;


    /*
     * ========================================================================
     * METADATA
     * ========================================================================
     */
    reinforcementMetadataItem
        : metadataBlock
        ;


    /*
     * ========================================================================
     * OPEN-WORLD NAMED ITEM
     * ========================================================================
     *
     * This is the primary extensibility mechanism.
     *
     * Future semantic properties do not need new reserved words.
     *
     * Examples:
     *
     *     algorithm: algorithm;
     *     optimizer: optimizer;
     *     representation: representation;
     *     return: return_expression;
     *     discount: discount_expression;
     *     horizon: horizon_expression;
     *     risk: risk_expression;
     *     seed: seed_expression;
     *     sampler: sampler_expression;
     *     replay: replay_expression;
     *     update: update_expression;
     *
     * These remain identifiers unless the language later gives one of them
     * genuine universal lexical status.
     */
    reinforcementNamedItem
        : reinforcementFieldName COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * DEDICATED SEMANTIC NAME ADAPTER
     * ========================================================================
     *
     * This permits important RL concepts to remain semantic fields without
     * reserving additional keywords.
     *
     * Examples:
     *
     *     reward: reward_expression;
     *     value: value_expression;
     *     exploration: exploration_expression;
     *
     * The semantic layer interprets their names.
     */
    reinforcementNamedSemanticItem
        : reinforcementFieldName COLON expression SEMICOLON?
        ;


    /*
     * ========================================================================
     * FIELD NAME
     * ========================================================================
     *
     * Qualified names allow namespace-based semantic extension.
     */
    reinforcementFieldName
        : identifier
        | qualifiedName
        ;


    /*
     * ========================================================================
     * GENERIC REINFORCEMENT STATEMENT
     * ========================================================================
     *
     * Generic operations remain open-world.
     *
     * Examples:
     *
     *     reinforcement update(model, transition);
     *     reinforcement evaluate(policy, environment);
     *     reinforcement collect(environment);
     *     reinforcement reset(environment);
     *
     * The operation name is semantic, not an algorithm catalog.
     */
    reinforcementStatement
        : REINFORCEMENT
          reinforcementOperation
          reinforcementArgumentList?
          SEMICOLON
        ;


    /*
     * ========================================================================
     * OPERATION
     * ========================================================================
     */
    reinforcementOperation
        : qualifiedName
        ;


    /*
     * ========================================================================
     * ARGUMENT LIST
     * ========================================================================
     *
     * No fixed argument count.
     *
     * A trailing comma is intentionally not accepted here so invocation
     * syntax remains consistent with the generic operation grammar.
     */
    reinforcementArgumentList
        : LPAREN
          reinforcementArgument
          (
              COMMA
              reinforcementArgument
          )*
          RPAREN
        ;


    /*
     * ========================================================================
     * ARGUMENT
     * ========================================================================
     */
    reinforcementArgument
        : reinforcementNamedArgument
        | expression
        ;


    /*
     * ========================================================================
     * NAMED ARGUMENT
     * ========================================================================
     */
    reinforcementNamedArgument
        : identifier ASSIGN expression
        ;


    /*
     * ========================================================================
     * EXPRESSION
     * ========================================================================
     *
     * This allows reinforcement semantics to participate in larger Zamani
     * expressions without defining another expression hierarchy.
     */
    reinforcementExpression
        : REINFORCEMENT
          reinforcementOperation
          reinforcementArgumentList?
        ;
}