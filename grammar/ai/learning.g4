/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/learning.g4
 *
 * GRAMMAR
 * -------
 * AILearning
 *
 * STATUS
 * ------
 * CANONICAL AI-DOMAIN LEARNING COMPOSITION ADAPTER
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
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file provides the AI-domain composition boundary for the universal
 * Zamani `learn` operation.
 *
 * IMPORTANT:
 *
 * This file does NOT define the `learn` statement syntax.
 *
 * The canonical source-level syntax is owned by:
 *
 *     grammar/statements/learn.g4
 *
 * whose grammar is:
 *
 *     Learn
 *
 * and whose public rule is:
 *
 *     learnStatement
 *
 * This file therefore exists only to make the canonical learning operation
 * available to the AI composition grammar.
 *
 * The architecture is:
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
 *     Statements
 *          |
 *          v
 *     Learn.learnStatement
 *          |
 *          v
 *     AILearning.aiLearningConstruct
 *          |
 *          v
 *     AI.aiConstruct
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *        types             effects            provenance
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                       semantic learning model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          resources      capabilities       policies
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                     canonical semantic model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *         classical       quantum::ir      other domain IR
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         specialization
 *                              |
 *                           lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                     resilience / recovery
 *                              |
 *                         ZQN / HAL
 *                              |
 *                              v
 *                       target realization
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Learning is a UNIVERSAL COMPUTATIONAL OPERATION.
 *
 * It may be used with:
 *
 *     - machine learning;
 *     - symbolic learning;
 *     - statistical learning;
 *     - probabilistic computation;
 *     - reinforcement learning;
 *     - transfer learning;
 *     - continual learning;
 *     - federated/distributed learning;
 *     - tensor computation;
 *     - scientific computation;
 *     - knowledge systems;
 *     - neural-symbolic computation;
 *     - quantum-assisted learning;
 *     - hybrid quantum-classical computation;
 *     - accelerator computation;
 *     - embedded computation;
 *     - future computational domains.
 *
 * This file does not define those algorithms or domains.
 *
 * A future learning method must not require a new core grammar rule merely
 * because its algorithm has a new name.
 *
 * The semantic model interprets the learning intent.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns EXACTLY ONE parser-level construct:
 *
 *     aiLearningConstruct
 *
 * It is a composition boundary.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - `learn` keyword spelling;
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - statements;
 *     - assignments;
 *     - blocks;
 *     - argument lists;
 *     - learning syntax;
 *     - training declaration syntax;
 *     - model declaration syntax;
 *     - dataset declaration syntax;
 *     - tensor syntax;
 *     - inference syntax;
 *     - reasoning syntax;
 *     - adaptation syntax;
 *     - knowledge syntax;
 *     - query syntax;
 *     - probability syntax;
 *     - uncertainty syntax;
 *     - agent syntax;
 *     - concurrency syntax;
 *     - distributed syntax;
 *     - quantum syntax;
 *     - HDL syntax;
 *     - hardware syntax;
 *     - resource syntax;
 *     - capability syntax;
 *     - effect syntax;
 *     - contract syntax;
 *     - policy syntax;
 *     - provenance syntax;
 *     - AST construction;
 *     - semantic implementation;
 *     - type checking;
 *     - effect checking;
 *     - capability negotiation;
 *     - resource negotiation;
 *     - policy enforcement;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - IR construction;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical source-level `learn` statement.
 *
 * That owner is:
 *
 *     grammar/statements/learn.g4
 *
 * with:
 *
 *     Learn.learnStatement
 *
 * This file MUST NOT define another rule such as:
 *
 *     learningStatement
 *     aiLearningStatement
 *     learnExpression
 *     learningExpression
 *     aiLearnStatement
 *     aiLearningOperation
 *
 * merely to reproduce the same syntax.
 *
 * This prevents grammar divergence.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/learn.g4
 *
 * Grammar:
 *
 *     Learn
 *
 * Public rule:
 *
 *     learnStatement
 *
 * Also depends transitively on the canonical:
 *
 *     ZamaniLexer
 *     Expressions
 *     Names
 *
 * through Learn.
 *
 * This file deliberately does NOT duplicate those dependencies.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * ANTLR imports grammar NAMES rather than filesystem paths.
 *
 * Therefore:
 *
 *     import Learn;
 *
 * is the canonical dependency.
 *
 * The ANTLR grammar build must include:
 *
 *     grammar/statements/
 *
 * in the grammar import/search path.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * EXPORTS
 * -------
 *
 *     aiLearningConstruct
 *
 * This is the ONLY AI-owned public rule in this file.
 *
 * The imported:
 *
 *     learnStatement
 *
 * remains owned by:
 *
 *     Learn
 *
 * ============================================================================
 * CONSUMER CONTRACT
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * AI.g4 should import:
 *
 *     AILearning
 *
 * and include:
 *
 *     aiLearningConstruct
 *
 * as an alternative of:
 *
 *     aiConstruct
 *
 * The intended composition is:
 *
 *     aiConstruct
 *         : ...
 *         | aiLearningConstruct
 *         | ...
 *         ;
 *
 * The AI composition grammar MUST NOT directly reproduce:
 *
 *     LEARN ...
 *
 * syntax.
 *
 * ============================================================================
 * TRAINING RELATIONSHIP
 * ============================================================================
 *
 * Learning and training are related but MUST remain separate grammar
 * ownership boundaries.
 *
 * Existing:
 *
 *     grammar/ai/training.g4
 *
 * owns:
 *
 *     trainingConstruct
 *
 * and the training declaration language.
 *
 * This file owns:
 *
 *     aiLearningConstruct
 *
 * and delegates to:
 *
 *     learnStatement
 *
 * Therefore:
 *
 *     learn model from data;
 *
 * and:
 *
 *     @training Experiment {
 *         ...
 *     }
 *
 * are distinct source constructs.
 *
 * Their semantic systems may share:
 *
 *     models
 *     datasets
 *     objectives
 *     resources
 *     capabilities
 *     policies
 *     provenance
 *
 * without creating duplicate grammar ownership.
 *
 * ============================================================================
 * REASONING RELATIONSHIP
 * ============================================================================
 *
 * Learning may consume results from:
 *
 *     infer
 *     deduce
 *     reason
 *     query
 *     knowledge
 *     evidence
 *     observation
 *
 * This file does not define those operations.
 *
 * Their canonical owners remain elsewhere in the repository.
 *
 * Semantic composition may therefore be:
 *
 *     knowledge
 *          |
 *          v
 *     reasoning
 *          |
 *          v
 *     learning
 *          |
 *          v
 *     model / knowledge / strategy
 *
 * ============================================================================
 * KNOWLEDGE RELATIONSHIP
 * ============================================================================
 *
 * Learning may consume or produce knowledge.
 *
 * The knowledge syntax remains owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * This file does not duplicate:
 *
 *     assert
 *     retract
 *     query
 *     lookup
 *     update
 *
 * ============================================================================
 * INFERENCE RELATIONSHIP
 * ============================================================================
 *
 * Learning and inference are different semantic operations.
 *
 * Existing:
 *
 *     grammar/ai/inference.g4
 *
 * owns AI inference composition.
 *
 * This file does not import or duplicate inference syntax.
 *
 * Semantic pipelines may connect:
 *
 *     learn
 *       |
 *       v
 *     model
 *       |
 *       v
 *     infer
 *
 * ============================================================================
 * ADAPTATION RELATIONSHIP
 * ============================================================================
 *
 * Learning may provide information used by adaptation.
 *
 * Learning itself MUST NOT imply unrestricted self-modification.
 *
 * Adaptation remains governed by:
 *
 *     policy
 *     capability
 *     effects
 *     resources
 *     contracts
 *     authorization
 *     provenance
 *
 * Therefore:
 *
 *     learn
 *
 * does not automatically mean:
 *
 *     modify arbitrary executable code.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * The underlying learning syntax is owned by Learn.
 *
 * Learn consumes the canonical:
 *
 *     expression
 *
 * supplied by:
 *
 *     Expressions
 *
 * Consequently learning operands can remain open-ended and may represent:
 *
 *     model
 *     dataset
 *     knowledge
 *     tensor
 *     parameter
 *     strategy
 *     policy
 *     feedback
 *     observation
 *     measurement
 *     simulation result
 *     query result
 *     reasoning result
 *     distributed result
 *     hardware observation
 *     user-defined value
 *     future-domain value
 *
 * This file does not introduce another expression hierarchy.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Types remain owned by the universal type system.
 *
 * This file creates no:
 *
 *     LearningType
 *     ModelLearningType
 *     TrainingType
 *     NeuralLearningType
 *     QuantumLearningType
 *
 * The semantic learning model may operate on any type supported by the
 * canonical type system, subject to semantic validation.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing has no runtime effects.
 *
 * Semantic analysis may classify learning with effects such as:
 *
 *     learning
 *     mutation
 *     randomness
 *     io
 *     network
 *     distributed
 *     foreign
 *     measurement
 *     simulation
 *
 * according to the actual semantic operation.
 *
 * This grammar does not define a competing effect taxonomy.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Learning may require capabilities such as:
 *
 *     learning
 *     tensor.compute
 *     model.execute
 *     data.read
 *     knowledge.read
 *     distributed.compute
 *     quantum.measurement
 *
 * or future capability identifiers.
 *
 * Capability resolution occurs downstream.
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     node
 *     device
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Learning resource requirements are resolved downstream by the universal
 * resource system.
 *
 * Examples of semantic requirements may include:
 *
 *     requires memory >= required_memory;
 *     requires capability("tensor.compute");
 *     requires capability("distributed.training");
 *     requires capability("quantum.measurement");
 *     requires topology(required_topology);
 *
 * This file imposes NO universal physical limit.
 *
 * It MUST NOT encode:
 *
 *     fixed memory size;
 *     fixed tensor rank;
 *     fixed tensor dimensions;
 *     fixed worker count;
 *     fixed GPU count;
 *     fixed QPU count;
 *     fixed node count;
 *     fixed dataset size;
 *     fixed model size;
 *     fixed parameter count.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Learning may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by the canonical validation/contract system.
 *
 * This file does not duplicate contract grammar.
 *
 * Semantic analysis determines:
 *
 *     - preconditions;
 *     - postconditions;
 *     - invariants;
 *     - assumptions;
 *     - guarantees;
 *     - proof/verification requirements.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Learning may be governed by policies controlling:
 *
 *     - data access;
 *     - model modification;
 *     - adaptation;
 *     - resource use;
 *     - network access;
 *     - external calls;
 *     - randomness;
 *     - privacy/security requirements;
 *     - reproducibility;
 *     - deployment;
 *     - target selection.
 *
 * Policies are semantic controls.
 *
 * This file does not implement policy enforcement.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Learning may change the state of a model, knowledge store, strategy, or
 * other semantic value.
 *
 * Provenance should therefore preserve, where applicable:
 *
 *     source;
 *     data origin;
 *     model identity;
 *     learning operation;
 *     configuration;
 *     evidence;
 *     transformation;
 *     decision;
 *     verification;
 *     language/toolchain version;
 *     reproducibility information.
 *
 * Provenance implementation is downstream.
 *
 * This grammar must preserve source locations through the parser tree so the
 * frontend can associate learning intent with provenance.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * Parsing must not depend on:
 *
 *     hardware availability;
 *     GPU availability;
 *     QPU availability;
 *     network state;
 *     filesystem state;
 *     current resource availability;
 *     randomness;
 *     wall-clock time;
 *     runtime model state.
 *
 * Semantic learning may legitimately be nondeterministic if the program
 * explicitly permits it, but that decision is downstream.
 *
 * ============================================================================
 * QUANTUM / HYBRID CONTRACT
 * ============================================================================
 *
 * Learning may consume quantum-derived values or produce quantum-related
 * computation.
 *
 * This grammar does not define quantum syntax.
 *
 * Quantum semantics must ultimately cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * There must be no:
 *
 *     LearningQuantumIR
 *     AIQuantumIR
 *     QMLGrammarIR
 *
 * created by this grammar.
 *
 * The hybrid semantic path is:
 *
 *     learning intent
 *          |
 *          v
 *     semantic model
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     classical           quantum semantics
 *                               |
 *                               v
 *                          quantum::ir
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL CONTRACT
 * ============================================================================
 *
 * Learning may be realized through:
 *
 *     sequential execution;
 *     parallel execution;
 *     distributed execution;
 *     accelerator execution;
 *     heterogeneous execution;
 *     federated execution;
 *     future execution models.
 *
 * This file does not select the realization.
 *
 * There is no fixed:
 *
 *     worker count;
 *     thread count;
 *     process count;
 *     node count;
 *     accelerator count.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION CONTRACT
 * ============================================================================
 *
 * Learning may participate in adaptive execution.
 *
 * Runtime/compiler layers may select an implementation based on:
 *
 *     available capabilities;
 *     available resources;
 *     policy;
 *     resilience;
 *     performance;
 *     constraints;
 *     preferences.
 *
 * Such selection MUST NOT change the source grammar or source meaning.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file contributes to Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 * by making learning syntax independent of physical realization.
 *
 * The same source-level learning intent may be considered for:
 *
 *     tiny systems;
 *     embedded systems;
 *     CPUs;
 *     multicore CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational targets.
 *
 * Practical feasibility is determined by:
 *
 *     semantic validation;
 *     capabilities;
 *     resources;
 *     policies;
 *     compiler realization;
 *     runtime availability.
 *
 * The grammar itself has no artificial target ceiling.
 *
 * ============================================================================
 * OPEN-WORLD LEARNING MODEL
 * ============================================================================
 *
 * The grammar intentionally does NOT enumerate learning algorithms.
 *
 * It must not contain alternatives for:
 *
 *     gradient_descent
 *     stochastic_gradient_descent
 *     adam
 *     rmsprop
 *     random_forest
 *     decision_tree
 *     svm
 *     transformer
 *     neural_network
 *     reinforcement_algorithm
 *     quantum_algorithm
 *     future_algorithm
 *
 * Such concepts are represented by:
 *
 *     expressions;
 *     model values;
 *     configuration values;
 *     libraries;
 *     dialects;
 *     capabilities;
 *     semantic providers.
 *
 * This keeps the grammar stable as computational science evolves.
 *
 * ============================================================================
 * APPLICATION INDEPENDENCE
 * ============================================================================
 *
 * The grammar does not add application-specific syntax for:
 *
 *     computer vision;
 *     sentiment analysis;
 *     robotics;
 *     blockchain;
 *     payment systems;
 *     legal workflows;
 *     administration;
 *     VR;
 *     AR;
 *     application-specific AI products.
 *
 * Those capabilities belong in libraries, dialects, services, policies, or
 * applications.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST remains domain-neutral.
 *
 * The AST must preserve enough information to represent:
 *
 *     operation kind = learn;
 *     source span;
 *     learning subject;
 *     optional source expression;
 *     optional configuration arguments;
 *     argument order;
 *     named argument names;
 *     argument expressions;
 *     source metadata required for provenance.
 *
 * The AI composition context MUST NOT force an AI-specific AST node when the
 * universal AST already represents the operation.
 *
 * ============================================================================
 * SEMANTIC OWNER
 * ============================================================================
 *
 * Semantic interpretation belongs to the semantic learning subsystem.
 *
 * It determines:
 *
 *     - what is being learned;
 *     - what information is consumed;
 *     - what state may change;
 *     - whether mutation is authorized;
 *     - type compatibility;
 *     - model/data compatibility;
 *     - objective validity;
 *     - policy validity;
 *     - resource requirements;
 *     - capability requirements;
 *     - reproducibility;
 *     - determinism;
 *     - provenance;
 *     - cross-domain validity.
 *
 * Parser acceptance MUST NOT imply semantic validity.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar owns NO IR.
 *
 * Learning semantics lower through the canonical semantic representation.
 *
 * Depending on the computation, the resulting semantic operations may lower
 * toward:
 *
 *     classical representation;
 *     tensor/data representation;
 *     distributed representation;
 *     accelerator representation;
 *     hardware representation;
 *     quantum::ir;
 *     another explicitly defined domain IR.
 *
 * There is no learning-specific universal IR introduced here.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may:
 *
 *     specialize learning;
 *     optimize learning;
 *     fuse operations;
 *     distribute work;
 *     select capabilities;
 *     negotiate resources;
 *     lower tensor computation;
 *     lower quantum computation;
 *     schedule execution;
 *     select accelerators;
 *     generate target-specific code.
 *
 * None of those decisions belong to this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may discover:
 *
 *     available memory;
 *     available compute;
 *     accelerators;
 *     QPUs;
 *     nodes;
 *     storage;
 *     network resources;
 *     model/data providers.
 *
 * Runtime discovery MUST NOT alter parsing or source meaning.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing this grammar must not:
 *
 *     - execute learning;
 *     - load models;
 *     - load datasets;
 *     - access files;
 *     - access networks;
 *     - access credentials;
 *     - access hardware;
 *     - allocate accelerators;
 *     - invoke external frameworks.
 *
 * Such operations belong to explicitly authorized downstream components.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains NO artificial machine-capacity constants.
 *
 * In particular, it must not introduce:
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
 * It also must not encode:
 *
 *     fixed worker counts;
 *     fixed device counts;
 *     fixed model counts;
 *     fixed parameter counts;
 *     fixed tensor dimensions;
 *     fixed dataset sizes;
 *     fixed physical topologies.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical `learnStatement` remains the compatibility authority.
 *
 * This adapter introduces no alternate syntax.
 *
 * Existing valid `learn` syntax therefore remains governed by:
 *
 *     grammar/statements/learn.g4
 *
 * Future changes to learning syntax must occur there.
 *
 * This adapter should require no change merely because the canonical
 * implementation adds another valid learning expression, argument, data
 * source, model type, or learning provider.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics originate from the canonical Learn grammar.
 *
 * This adapter must not duplicate diagnostics for:
 *
 *     missing subject;
 *     malformed from clause;
 *     malformed with clause;
 *     malformed argument;
 *     malformed expression.
 *
 * Semantic diagnostics belong downstream and may report:
 *
 *     invalid learning target;
 *     incompatible data;
 *     unavailable capability;
 *     insufficient resources;
 *     policy violation;
 *     contract violation;
 *     provenance violation;
 *     unsupported semantic realization.
 *
 * Resource failure MUST NOT be reported as a parser failure.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The AI composition test suite must verify at minimum:
 *
 * POSITIVE
 * --------
 *
 *     learn model;
 *
 *     learn model from dataset;
 *
 *     learn model with (objective = objective);
 *
 *     learn model from dataset with (
 *         strategy = strategy,
 *         policy = policy
 *     );
 *
 *     learn parameters from observations;
 *
 *     learn strategy from feedback;
 *
 *     learn model from query(source);
 *
 *     learn model from quantum_measurement;
 *
 *     learn model from simulation_result;
 *
 * The exact validity of semantic targets is checked downstream.
 *
 * NEGATIVE
 * --------
 *
 * The test suite must reject malformed canonical learning syntax, including:
 *
 *     learn;
 *
 *     learn from dataset;
 *
 *     learn model from;
 *
 *     learn model with;
 *
 *     learn model with (;
 *
 *     learn model with (,);
 *
 *     learn model with (a,,b);
 *
 * and other malformed forms according to the canonical Learn grammar.
 *
 * IMPORTANT:
 *
 * These tests belong primarily to:
 *
 *     grammar/tests/statements/learn/
 *
 * and AI composition tests should additionally verify that the same canonical
 * construct is reachable through:
 *
 *     aiLearningConstruct
 *
 * without changing its parse meaning.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required coverage includes:
 *
 *     learning + classical computation
 *     learning + data
 *     learning + knowledge
 *     learning + reasoning
 *     learning + inference
 *     learning + uncertainty
 *     learning + concurrency
 *     learning + distributed computation
 *     learning + accelerator intent
 *     learning + quantum computation
 *     learning + hybrid computation
 *     learning + simulation
 *     learning + contracts
 *     learning + policies
 *     learning + provenance
 *     learning + FFI where semantically authorized
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must remain independent of:
 *
 *     number of learning operations;
 *     number of models;
 *     number of datasets;
 *     number of parameters;
 *     number of phases;
 *     number of training steps;
 *     tensor rank;
 *     tensor dimensions;
 *     worker count;
 *     node count;
 *     accelerator count;
 *     QPU count;
 *     memory capacity;
 *     network size.
 *
 * Scalability tests should stress source size and nesting according to the
 * repository's parser/resource test infrastructure, without turning test
 * values into language limits.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source and lexer/parser configuration must produce equivalent
 * parse structures.
 *
 * Parsing must not depend on:
 *
 *     target hardware;
 *     current resources;
 *     runtime state;
 *     randomness;
 *     time;
 *     network state.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions and requires no unsafe Rust.
 *
 * Generated/consuming Rust code must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *
 * and the repository's safe-Rust policy.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [ ] File grammar name is AILearning.
 *
 * [ ] File is an ANTLR4 parser grammar.
 *
 * [ ] tokenVocab is ZamaniLexer.
 *
 * [ ] Learn is imported from the canonical statement grammar.
 *
 * [ ] Exactly one AI-owned public rule exists:
 *         aiLearningConstruct
 *
 * [ ] aiLearningConstruct delegates to learnStatement.
 *
 * [ ] No learn syntax is duplicated here.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] No identifier rules exist here.
 *
 * [ ] No expression hierarchy exists here.
 *
 * [ ] No type hierarchy exists here.
 *
 * [ ] No training grammar is duplicated here.
 *
 * [ ] No model grammar is duplicated here.
 *
 * [ ] No dataset grammar is duplicated here.
 *
 * [ ] No inference grammar is duplicated here.
 *
 * [ ] No reasoning grammar is duplicated here.
 *
 * [ ] No adaptation grammar is duplicated here.
 *
 * [ ] No knowledge grammar is duplicated here.
 *
 * [ ] No application-specific learning algorithm catalogue exists.
 *
 * [ ] No hardware target is encoded.
 *
 * [ ] No physical resource limit is encoded.
 *
 * [ ] No AI-specific AST is introduced.
 *
 * [ ] No learning-specific universal IR is introduced.
 *
 * [ ] quantum::ir remains the canonical quantum IR boundary.
 *
 * [ ] Resource decisions remain downstream.
 *
 * [ ] Capability decisions remain downstream.
 *
 * [ ] Policy enforcement remains downstream.
 *
 * [ ] Provenance remains downstream.
 *
 * [ ] Runtime execution remains downstream.
 *
 * [ ] Positive composition tests pass.
 *
 * [ ] Negative composition tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Rust 1.97+ safe-Rust integration passes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar AILearning;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL LEARNING GRAMMAR
 * ============================================================================
 *
 * The universal statement-level learning grammar is the sole owner of the
 * source syntax for `learn`.
 *
 * It is located at:
 *
 *     grammar/statements/learn.g4
 *
 * and has grammar identity:
 *
 *     Learn
 *
 * Its public rule is:
 *
 *     learnStatement
 *
 * ============================================================================
 */

import
    Learn
    ;


/*
 * ============================================================================
 * PUBLIC AI LEARNING COMPOSITION BOUNDARY
 * ============================================================================
 *
 * This rule deliberately delegates directly to the universal learning
 * statement.
 *
 * No alternate AI learning syntax is introduced.
 * ============================================================================
 */

aiLearningConstruct
    : learnStatement
    ;


/*
 * ============================================================================
 * END OF PRODUCTION GRAMMAR
 * ============================================================================
 *
 * Ownership summary:
 *
 *     Learn.learnStatement
 *         |
 *         v
 *     AILearning.aiLearningConstruct
 *         |
 *         v
 *     AI.aiConstruct
 *
 * The source syntax remains owned by Learn.
 *
 * The AI subsystem only composes it.
 *
 * ============================================================================
 */