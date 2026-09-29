/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/learning.g4
 *
 * Grammar:
 *     Learning
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Domain:
 *     Memory / Sankofa / Knowledge / Learning
 *
 * Status:
 *     CANONICAL LEARNING SYNTAX COMPONENT
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     Safe Rust only.
 *
 *     This grammar contains no embedded Rust actions and requires no
 *     `unsafe` Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax for Zamani's `learn` construct.
 *
 * `learn` expresses learning intent.
 *
 * It does NOT specify or implement a particular:
 *
 *     learning algorithm
 *     neural network
 *     optimizer
 *     gradient method
 *     statistical model
 *     symbolic learner
 *     probabilistic learner
 *     AI framework
 *     dataset engine
 *     accelerator
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     runtime
 *     storage engine
 *     distributed protocol
 *
 * Learning semantics are resolved after parsing.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The canonical path is:
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
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     resource / capability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / scheduling / placement
 *          |
 *          v
 *     target realization
 *
 * This grammar is only the syntax layer.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     learnStatement
 *     learnExpression
 *     learnFromClause
 *     learnWithClause
 *     learnArgumentList
 *     learnArgument
 *     learnNamedArgument
 *
 * It owns the syntactic relationship between:
 *
 *     LEARN
 *     learning subject/expression
 *     optional FROM source expression
 *     optional WITH argument list
 *     statement terminator
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     remember
 *     recall
 *     infer
 *     wisdom
 *     history
 *     provenance
 *     consensus
 *     MTS
 *     zamani
 *     sasa
 *     memory allocation
 *     ownership
 *     borrowing
 *     lifetimes
 *     persistence
 *     distributed-memory implementation
 *     accelerator selection
 *     quantum computation
 *     QEC
 *     ZQN
 *     HAL
 *     model training implementation
 *     optimizer implementation
 *     dataset implementation
 *     inference implementation
 *     storage implementation
 *     scheduling
 *     routing
 *     hardware discovery
 *     resource discovery
 *     capability discovery
 *     runtime execution
 *
 * Those responsibilities belong to their respective grammar, semantic,
 * compiler, runtime, or backend layers.
 *
 * ============================================================================
 * LEAF-GRAMMAR RULE
 * ============================================================================
 *
 * This is a leaf/domain grammar.
 *
 * It MUST NOT import:
 *
 *     Sankofa
 *     Memory
 *     Recall
 *     History
 *     Wisdom
 *
 * Those higher-level components may compose this grammar.
 *
 * Dependency direction:
 *
 *     Learning
 *        |
 *        +--> Names
 *        |
 *        +--> Expressions
 *
 * and:
 *
 *     Sankofa
 *        |
 *        +--> Learning
 *
 * NOT:
 *
 *     Learning
 *        |
 *        +--> Sankofa
 *
 * This prevents circular grammar dependencies.
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is the only lexical authority.
 *
 * This grammar consumes:
 *
 *     LEARN
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     ASSIGN
 *     SEMICOLON
 *
 * together with identifiers and expression tokens supplied by the imported
 * grammars and ZamaniLexer vocabulary.
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT invent:
 *
 *     LEARNING
 *     DATASET
 *     MODEL
 *     TRAIN
 *     EPOCH
 *     GRADIENT
 *     OPTIMIZER
 *     NEURAL
 *
 * as mandatory parser-level keywords merely to support future learning
 * technologies.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Learning must remain extensible.
 *
 * The grammar therefore does NOT enumerate learning algorithms.
 *
 * Prohibited design:
 *
 *     learningAlgorithm
 *         : gradientDescent
 *         | reinforcementLearning
 *         | neuralNetwork
 *         | symbolicLearning
 *         | ...
 *
 * as a universal closed list.
 *
 * Instead, the learning subject, source, and options are represented by
 * ordinary Zamani expressions and identifiers.
 *
 * This permits semantic systems to add support for:
 *
 *     symbolic learning
 *     statistical learning
 *     neural learning
 *     probabilistic learning
 *     reinforcement learning
 *     differentiable computation
 *     evolutionary methods
 *     quantum-assisted learning
 *     distributed learning
 *     future learning models
 *
 * without changing this grammar merely because a new implementation exists.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The same source-level `learn` construct must remain portable across:
 *
 *     tiny embedded targets
 *     CPUs
 *     multicore systems
 *     GPUs
 *     accelerators
 *     FPGAs
 *     ASICs
 *     QPUs
 *     quantum simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * Program source describes learning intent.
 *
 * The compiler and runtime determine how that intent is realized.
 *
 * Therefore the grammar must not contain target-specific syntax such as:
 *
 *     train_on_gpu_0
 *     use_8_cores
 *     use_device_3
 *     use_qpu_0
 *
 * as universal language constructs.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is intentionally NO universal upper bound for:
 *
 *     learning operations
 *     learning expressions
 *     learning arguments
 *     datasets
 *     samples
 *     features
 *     parameters
 *     models
 *     tensors
 *     dimensions
 *     training steps
 *     iterations
 *     epochs
 *     agents
 *     workers
 *     devices
 *     nodes
 *     accelerators
 *     memory
 *     timelines
 *     histories
 *     provenance records
 *
 * The grammar MUST NOT define:
 *
 *     MAX_LEARNING_STEPS
 *     MAX_EPOCHS
 *     MAX_SAMPLES
 *     MAX_FEATURES
 *     MAX_PARAMETERS
 *     MAX_MODELS
 *     MAX_TENSORS
 *     MAX_TENSOR_RANK
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_QPUS
 *
 * It must also not encode:
 *
 *     fixed tensor rank
 *     fixed model size
 *     fixed dataset size
 *     fixed number of workers
 *     fixed number of accelerators
 *     fixed memory capacity
 *
 * as language-level restrictions.
 *
 * ============================================================================
 * PROGRAM VALUE VS IMPLEMENTATION LIMIT
 * ============================================================================
 *
 * A numeric value appearing inside a learning expression is ordinary program
 * data.
 *
 * For example:
 *
 *     learn model with (steps = n);
 *
 * or:
 *
 *     learn model with (batch = 1024);
 *
 * does not establish a universal learning limit.
 *
 * A compiler, runtime, target, or execution profile may have resource limits.
 *
 * Those limits are resource/capability facts, not grammar restrictions.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Learning may require resources or capabilities.
 *
 * Examples of semantic intent include:
 *
 *     requires capability("learning")
 *     requires capability("tensor.compute")
 *     requires capability("distributed.compute")
 *     requires capability("persistent.memory")
 *
 * These expressions are interpreted by the resource/capability subsystem.
 *
 * This grammar does not determine whether a capability exists.
 *
 * It does not select a physical device.
 *
 * ============================================================================
 * LEARN STATEMENT
 * ============================================================================
 *
 * The statement form is:
 *
 *     learn <expression>;
 *
 * optionally:
 *
 *     learn <expression> from <expression>;
 *
 * or:
 *
 *     learn <expression> with (<arguments>);
 *
 * or:
 *
 *     learn <expression> from <expression>
 *         with (<arguments>);
 *
 * Examples:
 *
 *     learn model;
 *
 *     learn model from dataset;
 *
 *     learn model with (policy = p);
 *
 *     learn model from dataset with (
 *         policy = p,
 *         objective = objective
 *     );
 *
 * The subject and source are expressions rather than a closed list of
 * model/dataset types.
 *
 * ============================================================================
 * LEARN EXPRESSION
 * ============================================================================
 *
 * The expression form is identical except that it has no statement
 * terminator.
 *
 * Example:
 *
 *     let result = learn model from dataset;
 *
 * Whether `learn` may occur in a particular expression position is ultimately
 * controlled by the canonical expression/statement composition and semantic
 * rules.
 *
 * ============================================================================
 * FROM CLAUSE
 * ============================================================================
 *
 * `from` identifies the semantic source of learning information.
 *
 * The source is an ordinary expression.
 *
 * It may therefore represent:
 *
 *     a dataset
 *     a memory object
 *     a stream
 *     a query
 *     a previous model
 *     a knowledge source
 *     a temporal source
 *     a distributed source
 *     a computed value
 *     a user-defined abstraction
 *
 * The grammar does not enumerate those possibilities.
 *
 * ============================================================================
 * WITH CLAUSE
 * ============================================================================
 *
 * `with (...)` carries optional learning parameters.
 *
 * The argument namespace is deliberately open.
 *
 * Examples:
 *
 *     learn model with (policy = p);
 *
 *     learn model with (
 *         objective = objective,
 *         strategy = strategy
 *     );
 *
 *     learn model from dataset with (
 *         policy = policy,
 *         resources = requirements,
 *         capability = capability
 *     );
 *
 * The grammar does not define a fixed property list.
 *
 * Therefore future semantic options do not require a new parser keyword.
 *
 * ============================================================================
 * ARGUMENT MODEL
 * ============================================================================
 *
 * Arguments may be:
 *
 *     positional expressions
 *
 * or:
 *
 *     named arguments
 *
 * Examples:
 *
 *     learn model with (dataset);
 *
 *     learn model with (dataset, policy);
 *
 *     learn model with (
 *         dataset = data,
 *         policy = p
 *     );
 *
 * Named arguments use ordinary Zamani identifiers.
 *
 * Semantic analysis determines whether a given argument is meaningful.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is accepted:
 *
 *     learn model with (
 *         policy = p,
 *     );
 *
 * This is a syntactic convenience.
 *
 * It does not impose an argument-count limit.
 *
 * ============================================================================
 * EMPTY WITH
 * ============================================================================
 *
 * The syntax:
 *
 *     learn model with ();
 *
 * is accepted by this grammar.
 *
 * This is intentional because an empty option set is structurally distinct
 * from an absent `with` clause and permits syntax-preserving tooling.
 *
 * Semantic analysis may determine whether an empty option set has any effect.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The domain-neutral frontend AST must preserve sufficient information for:
 *
 *     learn operation identity
 *     source span
 *     subject expression
 *     optional source expression
 *     positional arguments
 *     named arguments
 *     argument ordering where relevant
 *     source metadata
 *
 * Conceptual mapping:
 *
 *     learnExpression
 *         ->
 *     existing generic/domain-neutral operation AST
 *
 *     learnStatement
 *         ->
 *     statement-level wrapper around the learning operation
 *
 * No backend-specific AST node is required.
 *
 * Do NOT introduce nodes such as:
 *
 *     NeuralNetworkLearningNode
 *     CUDATrainingNode
 *     GPUTrainingNode
 *     QPULearningNode
 *     TensorFlowLearningNode
 *     PyTorchLearningNode
 *
 * merely because a backend exists.
 *
 * ============================================================================
 * AST COMPATIBILITY
 * ============================================================================
 *
 * Existing frontend AST compatibility must be checked before lowering.
 *
 * If the current AST represents learning with fewer fields than this grammar
 * exposes, the frontend must evolve to preserve the additional information
 * rather than silently discarding:
 *
 *     from
 *     with
 *     positional arguments
 *     named arguments
 *
 * The grammar itself must not introduce a competing AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     what is being learned;
 *     what source provides learning information;
 *     whether source and target are type-compatible;
 *     whether the learning operation is valid;
 *     whether supplied arguments are valid;
 *     whether a model or knowledge object exists;
 *     whether required capabilities exist;
 *     whether resources are sufficient;
 *     whether ownership/lifetime rules are satisfied;
 *     whether security policies permit the operation;
 *     whether provenance requirements are satisfied;
 *     whether temporal constraints are valid;
 *     whether distributed execution is valid;
 *     whether the operation is deterministic where required.
 *
 * None of these checks belong in ANTLR grammar actions.
 *
 * ============================================================================
 * LEARNING / AI INTEGRATION
 * ============================================================================
 *
 * This grammar is compatible with the AI domain without making AI frameworks
 * part of the language syntax.
 *
 * A semantic implementation may map learning intent to:
 *
 *     symbolic computation
 *     statistical computation
 *     tensor computation
 *     neural computation
 *     probabilistic computation
 *     agent computation
 *     distributed computation
 *     accelerator computation
 *
 * The language syntax remains the same.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Learning commonly consumes or produces memory-resident information.
 *
 * This grammar therefore permits ordinary expressions as learning subjects
 * and sources.
 *
 * Memory semantics remain owned by:
 *
 *     grammar/memory/memory.g4
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/persistence.g4
 *     grammar/memory/memory-capabilities.g4
 *
 * Learning does not create a second memory system.
 *
 * ============================================================================
 * RECALL INTEGRATION
 * ============================================================================
 *
 * `learn` may semantically consume a value obtained through `recall`.
 *
 * Recall syntax remains owned by:
 *
 *     grammar/memory/recall.g4
 *
 * This grammar does not duplicate recall rules.
 *
 * Conceptually:
 *
 *     recall source
 *          |
 *          v
 *     expression/value
 *          |
 *          v
 *     learn target
 *
 * Such composition is resolved by the canonical expression/semantic system.
 *
 * ============================================================================
 * HISTORY / PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Learning may use historical or provenance-aware information.
 *
 * This grammar does not redefine:
 *
 *     history
 *     provenance
 *     lineage
 *     temporal execution
 *
 * Those concepts remain owned by their respective grammar components.
 *
 * A source expression may semantically denote any valid history/provenance
 * abstraction.
 *
 * ============================================================================
 * TEMPORAL INTEGRATION
 * ============================================================================
 *
 * Learning may participate in temporal computation.
 *
 * This grammar does not define:
 *
 *     timeline
 *     branch
 *     rewind
 *     fork
 *     merge
 *     timestamp
 *
 * as learning-specific syntax.
 *
 * Temporal semantics remain downstream and are integrated through:
 *
 *     grammar/types/temporal.g4
 *     grammar/memory/temporal.g4
 *     grammar/memory/history.g4
 *     execution/ temporal components
 *
 * where applicable.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Learning may participate in hybrid quantum-classical computation.
 *
 * Examples of semantic arrangements include:
 *
 *     classical data
 *         ->
 *     learning
 *         ->
 *     quantum computation
 *
 * or:
 *
 *     quantum measurement
 *         ->
 *     classical value
 *         ->
 *     learning
 *
 * This grammar does not define:
 *
 *     quantum gates
 *     qubits
 *     quantum states
 *     measurement
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *     physical qubits
 *
 * If a learned value participates in quantum computation, the established
 * architecture remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing / scheduling / resilience
 *       ->
 *     QEC / ZQN
 *       ->
 *     HAL
 *
 * No learning-specific quantum IR is introduced.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Learning may consume and produce ordinary classical values.
 *
 * The grammar therefore deliberately relies on the canonical expression and
 * type systems rather than creating a second classical learning expression
 * language.
 *
 * ============================================================================
 * TENSOR / DATA INTEGRATION
 * ============================================================================
 *
 * Tensor and dataset semantics belong to:
 *
 *     grammar/data/
 *     grammar/classical/
 *     grammar/ai/
 *     grammar/types/
 *
 * as appropriate.
 *
 * This grammar accepts expressions and therefore does not need to know whether
 * a source is:
 *
 *     scalar
 *     vector
 *     matrix
 *     tensor
 *     stream
 *     collection
 *     dataset
 *     user-defined structure
 *
 * Type and semantic analysis decide that.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Learning may be distributed.
 *
 * The grammar does not encode:
 *
 *     worker count
 *     node count
 *     shard count
 *     replication count
 *     topology
 *     device placement
 *
 * Distributed realization belongs to:
 *
 *     grammar/distributed/
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * and downstream compiler/runtime systems.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Learning intent must remain target-independent.
 *
 * Valid semantic requirements may include:
 *
 *     requires capability("tensor.compute")
 *     requires capability("distributed.compute")
 *     requires capability("accelerator.compute")
 *
 * These are capabilities, not device selections.
 *
 * The grammar MUST NOT turn:
 *
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     CPU
 *
 * into mandatory implementation choices for a learning expression.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements belong to the resource system.
 *
 * Examples:
 *
 *     requires memory >= required_memory
 *     requires capability("learning")
 *     requires capability("tensor.compute")
 *
 * The grammar must carry resource intent without imposing universal capacity
 * limits.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * `learn` does not grant access to its source.
 *
 * For example:
 *
 *     learn model from secret;
 *
 * does not by itself authorize access to `secret`.
 *
 * Authorization, confidentiality, integrity, provenance, trust, and policy
 * enforcement belong downstream.
 *
 * The grammar must not embed:
 *
 *     credentials
 *     access tokens
 *     encryption keys
 *     secrets
 *     physical storage identifiers
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Given the same token stream and parser configuration, the grammar must
 * produce the same parse-tree structure.
 *
 * Parsing must not inspect:
 *
 *     datasets
 *     models
 *     memory
 *     filesystem state
 *     network state
 *     hardware state
 *     current time
 *     runtime state
 *     randomness
 *
 * Learning itself may be nondeterministic at semantic/runtime level.
 *
 * That is not a parsing concern.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should identify structural errors such as:
 *
 *     missing learning subject
 *     missing source expression
 *     malformed from clause
 *     malformed with clause
 *     malformed argument
 *     missing assignment expression
 *     missing closing parenthesis
 *     missing statement terminator
 *
 * Semantic diagnostics must remain separate.
 *
 * Examples:
 *
 *     unknown model
 *     unknown dataset
 *     incompatible types
 *     unavailable capability
 *     insufficient resources
 *     invalid policy
 *     invalid provenance
 *     unauthorized source
 *
 * These must not be converted into parser errors.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical forms are:
 *
 *     learn expression;
 *
 *     learn expression from expression;
 *
 *     learn expression with (...);
 *
 *     learn expression from expression with (...);
 *
 * Existing Sankofa syntax:
 *
 *     learn expression;
 *     learn expression from expression;
 *
 * remains valid.
 *
 * The `from` form must therefore remain source-compatible.
 *
 * Additional `with` syntax is an extension of the learning component and
 * should be documented in grammar/grammar.md with its implementation status
 * until the frontend fully supports it.
 *
 * Feature evolution follows:
 *
 *     proposal
 *       ->
 *     specification
 *       ->
 *     grammar
 *       ->
 *     AST
 *       ->
 *     semantic implementation
 *       ->
 *     canonical IR
 *       ->
 *     compiler
 *       ->
 *     tests
 *       ->
 *     compatibility review
 *       ->
 *     stable
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where formatter support exists:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve the semantic meaning of:
 *
 *     learning subject
 *     from source
 *     with arguments
 *
 * The formatter must not silently remove optional clauses that carry semantic
 * information.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This grammar must remain parser-only and structurally simple.
 *
 * It must not use:
 *
 *     embedded actions
 *     semantic predicates
 *     model loading
 *     dataset loading
 *     network requests
 *     filesystem inspection
 *     hardware discovery
 *     runtime callbacks
 *
 * during parsing.
 *
 * The grammar is intentionally linear:
 *
 *     LEARN
 *       expression
 *       optional FROM expression
 *       optional WITH argument-list
 *       optional statement terminator
 *
 * ============================================================================
 * SCALABILITY / HARD-CODING AUDIT
 * ============================================================================
 *
 * No universal language limit is encoded here.
 *
 * In particular, this file contains no:
 *
 *     MAX_LEARNING_STEPS
 *     MAX_EPOCHS
 *     MAX_SAMPLES
 *     MAX_FEATURES
 *     MAX_PARAMETERS
 *     MAX_MODELS
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * It also contains no fixed:
 *
 *     dataset size
 *     model size
 *     tensor dimension
 *     worker count
 *     node count
 *     accelerator count
 *     memory capacity
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive examples:
 *
 *     learn model;
 *
 *     learn target from dataset;
 *
 *     learn target with ();
 *
 *     learn target with (policy = p);
 *
 *     learn target from dataset with (policy = p);
 *
 *     learn target with (dataset);
 *
 *     learn target with (dataset, policy);
 *
 *     learn target with (
 *         dataset = data,
 *         policy = policy
 *     );
 *
 *     learn qualified::model from qualified::dataset;
 *
 *     learn object.member from source[index];
 *
 *     learn model from query(argument);
 *
 *     learn model with (
 *         objective = expression,
 *         strategy = expression,
 *     );
 *
 * Negative examples:
 *
 *     learn;
 *
 *     learn from dataset;
 *
 *     learn target from;
 *
 *     learn target with;
 *
 *     learn target with (;
 *
 *     learn target with );
 *
 *     learn target with (= value);
 *
 *     learn target with (policy = );
 *
 *     learn target from dataset with (policy = );
 *
 *     learn target from dataset extra;
 *
 * where `extra` is not valid according to the surrounding grammar.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Verify:
 *
 *     empty with list
 *     one argument
 *     many arguments
 *     positional arguments
 *     named arguments
 *     mixed arguments
 *     trailing comma
 *     nested expressions
 *     qualified names
 *     member expressions
 *     indexed expressions
 *     function calls
 *     complex source expressions
 *     large expressions
 *
 * These tests must verify syntax behavior, not establish artificial limits.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Verify that the same syntax remains valid when semantic values represent:
 *
 *     one sample
 *     many samples
 *     one feature
 *     many features
 *     one parameter
 *     many parameters
 *     one model
 *     many models
 *     one learning step
 *     many learning steps
 *     one worker
 *     many workers
 *     one node
 *     many nodes
 *     one accelerator
 *     many accelerators
 *
 * No test may establish a maximum as a language rule.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Learning syntax must be tested in programs containing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     distributed computation
 *     AI/ML
 *     tensors
 *     persistent memory
 *     provenance
 *     temporal computation
 *     security policies
 *     resource requirements
 *     compile intent
 *     execution intent
 *
 * Semantic validity is checked downstream.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is composed by higher-level grammars.
 *
 * Required direct dependencies:
 *
 *     Names
 *     Expressions
 *
 * Required lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * Higher-level integration:
 *
 *     grammar/memory/sankofa.g4
 *
 * must import `Learning` and delegate its learning production to:
 *
 *     learnStatement
 *
 * The Sankofa grammar must not maintain a second independent copy of the
 * `learn` syntax.
 *
 * ============================================================================
 * SANKOFA INTEGRATION
 * ============================================================================
 *
 * Existing Sankofa ownership currently includes:
 *
 *     sankofaLearn
 *
 * The canonical composition should become:
 *
 *     sankofaLearn
 *         : learnStatement
 *         ;
 *
 * This preserves the existing Sankofa rule name while transferring ownership
 * of the actual learning syntax to this file.
 *
 * Existing Sankofa rules for:
 *
 *     remember
 *     recall
 *     infer
 *     wisdom
 *     zamani
 *     sasa
 *
 * remain owned by their appropriate grammar components.
 *
 * ============================================================================
 * RECALL / HISTORY / WISDOM SEPARATION
 * ============================================================================
 *
 * Do not copy:
 *
 *     recallStatement
 *     recallExpression
 *
 * from grammar/memory/recall.g4.
 *
 * Do not copy:
 *
 *     history*
 *
 * from grammar/memory/history.g4.
 *
 * Do not copy:
 *
 *     wisdom*
 *
 * from grammar/memory/wisdom.g4.
 *
 * Each feature has one syntax owner.
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This file does NOT define a LearningIR.
 *
 * Learning must lower through the existing domain-neutral semantic pipeline:
 *
 *     learning syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic learning model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     canonical IR
 *
 * Backend-specific lowering occurs later.
 *
 * For hybrid quantum/classical computation, quantum portions continue through:
 *
 *     quantum::ir
 *
 * without introducing a learning-specific quantum IR.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * Generated parser/frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] Existing filename is preserved.
 * [x] New grammar has one independent responsibility.
 * [x] Canonical LEARN token is reused.
 * [x] Canonical expression grammar is reused.
 * [x] Canonical identifier grammar is reused.
 * [x] Statement and expression forms are separated.
 * [x] FROM source syntax is supported.
 * [x] WITH options are supported.
 * [x] Positional arguments are supported.
 * [x] Named arguments are supported.
 * [x] Empty WITH lists are supported.
 * [x] Trailing commas are supported.
 * [x] Argument names remain open-ended.
 * [x] Learning algorithms are not enumerated.
 * [x] AI frameworks are not embedded.
 * [x] Hardware targets are not embedded.
 * [x] Resource capacities are not hard-coded.
 * [x] No universal scaling limits are defined.
 * [x] No parser actions are required.
 * [x] No semantic predicates are required.
 * [x] No runtime dependency exists.
 * [x] No second AST is introduced.
 * [x] No second IR is introduced.
 * [x] Quantum semantics remain outside this grammar.
 * [x] QEC remains outside this grammar.
 * [x] ZQN remains outside this grammar.
 * [x] HAL remains outside this grammar.
 * [x] Sankofa can compose this component without circular dependency.
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar Learning;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
    ;


/* ============================================================================
 * PUBLIC STATEMENT ENTRY
 * ========================================================================== */

learnStatement
    : LEARN
      expression
      learnFromClause?
      learnWithClause?
      SEMICOLON
    ;


/* ============================================================================
 * PUBLIC EXPRESSION ENTRY
 * ========================================================================== */

learnExpression
    : LEARN
      expression
      learnFromClause?
      learnWithClause?
    ;


/* ============================================================================
 * OPTIONAL SOURCE CLAUSE
 * ========================================================================== */

learnFromClause
    : FROM
      expression
    ;


/* ============================================================================
 * OPTIONAL PARAMETERS
 * ========================================================================== */

learnWithClause
    : WITH
      LPAREN
      learnArgumentList?
      RPAREN
    ;


/* ============================================================================
 * ARGUMENT LIST
 * ========================================================================== */

learnArgumentList
    : learnArgument
      (COMMA learnArgument)*
      COMMA?
    ;


/* ============================================================================
 * ARGUMENT
 * ========================================================================== */

learnArgument
    : learnNamedArgument
    | expression
    ;


/* ============================================================================
 * NAMED ARGUMENT
 * ========================================================================== */

learnNamedArgument
    : identifier
      ASSIGN
      expression
    ;