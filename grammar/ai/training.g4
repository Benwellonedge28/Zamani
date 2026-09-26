/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/training.g4
 *
 * Grammar:
 *     AITraining
 *
 * Status:
 *     CANONICAL AI TRAINING SOURCE-SYNTAX COMPONENT
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific implementation.
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
 * This grammar owns SOURCE-LEVEL TRAINING INTENT.
 *
 * It describes:
 *
 *     - training declarations;
 *     - training phases;
 *     - training steps;
 *     - model references;
 *     - dataset references;
 *     - objectives;
 *     - losses;
 *     - optimizers;
 *     - metrics;
 *     - hyperparameters;
 *     - schedules;
 *     - validation;
 *     - evaluation;
 *     - checkpoints;
 *     - reproducibility;
 *     - resource requirements;
 *     - capability requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - distributed-training intent;
 *     - accelerator intent;
 *     - hybrid classical/quantum training intent;
 *     - extensible future training operations.
 *
 * This grammar describes intent.
 *
 * It does NOT implement:
 *
 *     - gradient descent;
 *     - automatic differentiation;
 *     - optimizer algorithms;
 *     - numerical kernels;
 *     - tensor execution;
 *     - model execution;
 *     - dataset execution;
 *     - scheduling;
 *     - resource allocation;
 *     - hardware discovery;
 *     - device selection;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
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
 *          v
 *     AITraining
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+----------+-----------+
 *     |               |           |
 *     v               v           v
 *     AI semantics  Resources  Capabilities
 *     |               |           |
 *     +---------------+-----------+
 *                     |
 *                     v
 *            canonical semantic model
 *                     |
 *                     v
 *             canonical compiler IR
 *                     |
 *        +------------+-------------+
 *        |            |             |
 *        v            v             v
 *    Classical     quantum::ir   HDL/accelerator
 *    lowering       lowering      lowering
 *        |            |             |
 *        +------------+-------------+
 *                     |
 *                     v
 *                 optimization
 *                     |
 *                 scheduling
 *                     |
 *                 resilience
 *                     |
 *                    ZQN
 *                     |
 *                    HAL
 *                     |
 *              target realization
 *
 * `training.g4` MUST NOT construct or define IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - training declaration syntax;
 *     - training body boundaries;
 *     - training phase syntax;
 *     - training step syntax;
 *     - training objective syntax;
 *     - training loss syntax;
 *     - optimizer intent syntax;
 *     - metric syntax;
 *     - hyperparameter syntax;
 *     - schedule syntax;
 *     - validation/evaluation syntax;
 *     - checkpoint syntax;
 *     - reproducibility syntax;
 *     - resource/capability/constraint/preference/hint syntax;
 *     - distributed-training intent syntax;
 *     - accelerator-training intent syntax;
 *     - generic training-operation extension syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - annotation marker lexing;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - statements;
 *     - model declarations;
 *     - dataset declarations;
 *     - tensor declarations;
 *     - inference;
 *     - differentiation;
 *     - optimizer implementations;
 *     - framework APIs;
 *     - hardware;
 *     - resource discovery;
 *     - scheduling;
 *     - runtime;
 *     - IR.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical annotation marker is:
 *
 *     AT
 *
 * which is lexically:
 *
 *     '@'
 *
 * This file MUST NOT use the obsolete/non-canonical:
 *
 *     NANO_ANNOTATION
 *
 * Training annotations therefore begin structurally as:
 *
 *     AT IDENTIFIER
 *
 * Examples:
 *
 *     @training
 *     @model
 *     @dataset
 *     @objective
 *     @loss
 *     @optimizer
 *     @metric
 *     @phase
 *     @step
 *     @checkpoint
 *
 * Annotation meaning is resolved semantically.
 *
 * This grammar does not create one lexer token per AI concept.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Types belong to the canonical type grammar.
 *
 * This file consumes:
 *
 *     typeExpression
 *
 * It MUST NOT define:
 *
 *     TrainingType
 *     ModelType
 *     DatasetType
 *     TensorType
 *     OptimizerType
 *
 * as competing type systems.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Expressions belong to the canonical expression grammar.
 *
 * This file consumes:
 *
 *     expression
 *     argumentList
 *
 * Expressions may represent:
 *
 *     - models;
 *     - datasets;
 *     - tensors;
 *     - objectives;
 *     - losses;
 *     - metrics;
 *     - optimizer values;
 *     - hyperparameters;
 *     - schedules;
 *     - predicates;
 *     - symbolic dimensions;
 *     - resource quantities;
 *     - capabilities;
 *     - quantum-derived values;
 *     - classical values;
 *     - distributed values.
 *
 * This grammar MUST NOT redefine expression syntax.
 *
 * ============================================================================
 * STATEMENT CONTRACT
 * ============================================================================
 *
 * Ordinary Zamani statements remain owned by the canonical statement grammar.
 *
 * Training bodies may contain ordinary statements.
 *
 * This grammar MUST NOT become a second general-purpose statement grammar.
 *
 * ============================================================================
 * MODEL CONTRACT
 * ============================================================================
 *
 * Models are semantic values.
 *
 * Model declarations remain owned by:
 *
 *     grammar/ai/model.g4
 *     grammar/ai/models.g4
 *
 * depending on the repository's canonical model composition path.
 *
 * Training references models through:
 *
 *     expression
 *
 * Training does not redefine model declaration syntax.
 *
 * ============================================================================
 * DATASET CONTRACT
 * ============================================================================
 *
 * Datasets are semantic values.
 *
 * Dataset declarations remain owned by:
 *
 *     grammar/ai/datasets.g4
 *
 * and the broader:
 *
 *     grammar/data/
 *
 * subsystem.
 *
 * Training references datasets through expressions.
 *
 * Training MUST NOT duplicate dataset declaration syntax.
 *
 * ============================================================================
 * TENSOR CONTRACT
 * ============================================================================
 *
 * Tensor types and tensor-domain syntax remain owned by the canonical AI/data
 * and type systems.
 *
 * Training only consumes tensor values/types through existing contracts.
 *
 * ============================================================================
 * DIFFERENTIATION CONTRACT
 * ============================================================================
 *
 * Automatic differentiation is NOT implemented here.
 *
 * Training may consume differentiated values and operations through ordinary
 * expressions.
 *
 * Differentiation remains owned by the canonical differentiation subsystem.
 *
 * ============================================================================
 * AI FRAMEWORK INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     SGD
 *     Adam
 *     AdamW
 *     RMSProp
 *     Adagrad
 *     L-BFGS
 *     XGBoost
 *     PyTorch
 *     TensorFlow
 *     JAX
 *     ONNX
 *     CUDA
 *     ROCm
 *     TPU
 *     NPU
 *
 * or any future framework/algorithm merely because it exists.
 *
 * Such names remain ordinary semantic/library identifiers.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The training language describes:
 *
 *     WHAT is being trained;
 *     WHAT data is used;
 *     WHAT objective is optimized;
 *     WHAT measurements matter;
 *     WHAT correctness constraints apply;
 *     WHAT capabilities are required;
 *     WHAT resources are required or preferred.
 *
 * It does NOT prescribe:
 *
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - TPU;
 *     - NPU;
 *     - QPU;
 *     - physical device identifier;
 *     - node identifier;
 *     - worker identifier;
 *     - memory bank;
 *     - fixed memory capacity;
 *     - fixed accelerator count;
 *     - fixed worker count;
 *     - fixed node count;
 *     - fixed topology.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     - models;
 *     - datasets;
 *     - parameters;
 *     - hyperparameters;
 *     - objectives;
 *     - losses;
 *     - metrics;
 *     - phases;
 *     - steps;
 *     - checkpoints;
 *     - validation stages;
 *     - workers;
 *     - devices;
 *     - nodes;
 *     - accelerators;
 *     - tensor dimensions;
 *     - tensor rank;
 *     - batch size;
 *     - sequence length;
 *     - feature count;
 *     - class count.
 *
 * Repetition is structural.
 *
 * Practical limitations arise only from genuine:
 *
 *     - compiler resources;
 *     - runtime resources;
 *     - target capabilities;
 *     - explicitly declared requirements;
 *     - deployment constraints;
 *     - physical resources.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_PARAMETERS
 *     MAX_HYPERPARAMETERS
 *     MAX_PHASES
 *     MAX_STEPS
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *
 * Numeric values in source programs remain ordinary program semantics.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * Training distinguishes:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *
 * Examples:
 *
 *     requires capability("tensor.compute")
 *     requires capability("distributed.training")
 *     requires capability("quantum.measurement")
 *
 * are portable semantic requirements.
 *
 * They are NOT hardware selection commands.
 *
 * A source program MUST NOT require:
 *
 *     GPU 0
 *     QPU 7
 *     node 12
 *
 * merely to express portable training intent.
 *
 * ============================================================================
 * DISTRIBUTED TRAINING
 * ============================================================================
 *
 * Distributed training may express:
 *
 *     partitioning;
 *     replication;
 *     synchronization;
 *     consistency;
 *     locality;
 *     fault tolerance;
 *     parallelism.
 *
 * It MUST NOT encode a universal machine topology.
 *
 * ============================================================================
 * QUANTUM / HYBRID TRAINING
 * ============================================================================
 *
 * Training expressions may consume quantum-derived values.
 *
 * Example semantic pattern:
 *
 *     quantum computation
 *          |
 *          v
 *     measurement/value
 *          |
 *          v
 *     training expression
 *
 * Quantum semantics remain owned by the quantum subsystem.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * Training MUST NOT define:
 *
 *     QubitId
 *     physical qubits
 *     quantum gates
 *     coupling maps
 *     routing
 *     QEC
 *     ZQN
 *
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility is explicit source intent.
 *
 * The grammar does not silently insert:
 *
 *     seeds;
 *     deterministic ordering;
 *     initialization policy;
 *     execution policy.
 *
 * Those must be expressed explicitly when required and validated downstream.
 *
 * ============================================================================
 * CHECKPOINTS
 * ============================================================================
 *
 * Checkpoints describe logical persistence/recovery intent.
 *
 * They do not select:
 *
 *     - filesystem;
 *     - object store;
 *     - database;
 *     - cloud provider;
 *     - device memory;
 *     - storage device.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Public entry point:
 *
 *     trainingConstruct
 *
 * Required parser imports:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * This grammar MUST NOT import:
 *
 *     AI
 *     Models
 *     Datasets
 *     Tensors
 *
 * because those imports create unnecessary domain cycles.
 *
 * AITraining is a leaf/domain grammar.
 *
 * The AI composition grammar is responsible for composing it.
 *
 * ============================================================================
 */

parser grammar AITraining;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `trainingConstruct` intentionally does NOT accept:
 *
 *     expression
 *     identifier
 *     assignment
 *     arbitrary block
 *
 * by themselves.
 *
 * This prevents the training grammar from swallowing ordinary Zamani syntax.
 */
trainingConstruct
    : trainingDeclaration
    ;


/* ============================================================================
 * TRAINING DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     @training Experiment
 *     {
 *         ...
 *     }
 *
 * Optional type and initializer are supported:
 *
 *     @training Experiment : TrainingSpec = config;
 *
 * The exact semantic validity of the annotation spelling `training` is
 * checked after parsing.
 */
trainingDeclaration
    : trainingAnnotation
      identifier
      trainingTypeClause?
      trainingInitializer?
      trainingBody?
      SEMICOLON?
    ;


trainingAnnotation
    : AT
      trainingAnnotationName
    ;


trainingAnnotationName
    : IDENTIFIER
    ;


trainingTypeClause
    : COLON
      typeExpression
    ;


trainingInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * TRAINING BODY
 * ============================================================================
 */

trainingBody
    : LBRACE
      trainingMember*
      RBRACE
    ;


trainingMember
    : trainingAnnotatedMember
    | trainingRequirementMember
    | trainingCapabilityMember
    | trainingConstraintMember
    | trainingPreferenceMember
    | trainingHintMember
    | trainingPhase
    | trainingStep
    | trainingStatement
    ;


/* ============================================================================
 * ANNOTATED TRAINING MEMBERS
 * ============================================================================
 *
 * The annotation is structurally generic.
 *
 * Semantic analysis determines whether it denotes:
 *
 *     model
 *     dataset
 *     objective
 *     loss
 *     optimizer
 *     metric
 *     hyperparameter
 *     schedule
 *     validation
 *     evaluation
 *     checkpoint
 *     reproducibility
 *     distributed training
 *     accelerator intent
 *     another registered training concept.
 *
 * This avoids a growing lexer vocabulary.
 */
trainingAnnotatedMember
    : trainingMemberAnnotation
      trainingMemberPayload
      SEMICOLON?
    ;


trainingMemberAnnotation
    : AT
      trainingAnnotationName
    ;


trainingMemberPayload
    : trainingNamedBinding
    | trainingExpressionPayload
    | trainingBlockPayload
    | trainingCallPayload
    ;


trainingNamedBinding
    : identifier
      trainingTypeClause?
      ASSIGN
      expression
    ;


trainingExpressionPayload
    : expression
    ;


trainingBlockPayload
    : LBRACE
      trainingMember*
      RBRACE
    ;


trainingCallPayload
    : identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * EXPLICIT RESOURCE / CAPABILITY CONTRACTS
 * ============================================================================
 *
 * These are deliberately separate from annotations because they have language
 * semantics that must remain distinguishable:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *
 * They remain target-independent.
 */
trainingRequirementMember
    : REQUIRES
      expression
      SEMICOLON
    ;


trainingCapabilityMember
    : CAPABILITY
      expression
      SEMICOLON
    ;


trainingConstraintMember
    : CONSTRAINT
      expression
      SEMICOLON
    ;


trainingPreferenceMember
    : PREFER
      expression
      SEMICOLON
    ;


trainingHintMember
    : HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * TRAINING PHASE
 * ============================================================================
 *
 * Phases are structural and unbounded.
 *
 * Examples:
 *
 *     @phase warmup {
 *         ...
 *     }
 *
 *     @phase optimization {
 *         ...
 *     }
 */
trainingPhase
    : AT
      trainingPhaseName
      LBRACE
      trainingMember*
      RBRACE
    ;


trainingPhaseName
    : IDENTIFIER
    ;


/* ============================================================================
 * TRAINING STEP
 * ============================================================================
 *
 * A training step is a semantic unit of training work.
 *
 * It does not correspond to:
 *
 *     one CPU instruction;
 *     one GPU kernel;
 *     one thread;
 *     one accelerator;
 *     one device.
 */
trainingStep
    : AT
      trainingStepName
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE trainingMember* RBRACE
      )
      SEMICOLON?
    ;


trainingStepName
    : IDENTIFIER
    ;


/* ============================================================================
 * ORDINARY ZAMANI STATEMENTS
 * ============================================================================
 *
 * Training code may contain ordinary language statements.
 *
 * The canonical statement grammar owns their syntax.
 */
trainingStatement
    : statement
    ;


/* ============================================================================
 * NAMED TRAINING VALUE HELPERS
 * ============================================================================
 *
 * These semantic helper rules deliberately do not enumerate algorithms.
 *
 * For example:
 *
 *     @loss loss = cross_entropy(output, target);
 *     @optimizer optimizer = my_optimizer(config);
 *     @metric accuracy = accuracy(predictions, labels);
 *
 * all use the same structural representation.
 */
trainingNamedValue
    : identifier
      ASSIGN
      expression
    ;


trainingNamedValueWithType
    : identifier
      COLON
      typeExpression
      ASSIGN
      expression
    ;


/* ============================================================================
 * TRAINING OPERATION EXTENSION
 * ============================================================================
 *
 * Future training paradigms remain expressible without adding a new grammar
 * production for every algorithm.
 *
 * Examples of semantic operations include:
 *
 *     transfer learning
 *     continual learning
 *     meta learning
 *     reinforcement learning
 *     self-supervised learning
 *     federated learning
 *     graph learning
 *     multimodal learning
 *     probabilistic learning
 *     symbolic learning
 *     quantum-assisted learning
 *     hybrid learning
 *     future learning systems
 *
 * The operation remains an annotation + expression.
 */
trainingOperation
    : AT
      trainingAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * RESOURCE-AWARE TRAINING EXPRESSIONS
 * ============================================================================
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *     requires capability("tensor.compute");
 *     requires capability("quantum.measurement");
 *     requires capability("distributed.training");
 *
 * The expressions themselves are owned by the canonical expression grammar.
 */
trainingResourceExpression
    : expression
    ;


/* ============================================================================
 * DISTRIBUTED TRAINING INTENT
 * ============================================================================
 *
 * These forms remain semantic rather than physical.
 */
trainingDistributedIntent
    : AT
      trainingAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * ACCELERATOR INTENT
 * ============================================================================
 *
 * Accelerator intent may be expressed without naming a physical device.
 */
trainingAcceleratorIntent
    : AT
      trainingAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * HYBRID / QUANTUM TRAINING
 * ============================================================================
 *
 * Quantum-derived values are ordinary expressions from the training grammar's
 * perspective.
 *
 * No quantum-specific syntax is duplicated here.
 */
trainingHybridIntent
    : AT
      trainingAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Semantic registration may add future training annotations without changing
 * the lexer.
 *
 * The parser remains structurally stable.
 *
 * Examples:
 *
 *     @training
 *     @model
 *     @dataset
 *     @objective
 *     @loss
 *     @optimizer
 *     @metric
 *     @schedule
 *     @validation
 *     @evaluation
 *     @checkpoint
 *     @reproducibility
 *     @distributed
 *     @accelerator
 *     @hybrid
 *     @custom_training_operation
 *
 * are all represented by:
 *
 *     AT IDENTIFIER ...
 *
 * where their exact semantic role is determined downstream.
 */


/* ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser layer must preserve:
 *
 *     - annotation name;
 *     - declared training name;
 *     - declared type;
 *     - initializer;
 *     - member order;
 *     - member source spans;
 *     - expressions;
 *     - resource/capability clauses;
 *     - nested phases;
 *     - nested steps;
 *     - ordinary statements.
 *
 * The grammar does NOT mandate vendor/framework AST types.
 *
 * The frontend AST should remain domain-neutral where practical.
 *
 * A semantic training representation may be produced after AST construction.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate at least:
 *
 *     - training annotation meaning;
 *     - identifier binding;
 *     - duplicate declarations;
 *     - type compatibility;
 *     - model reference validity;
 *     - dataset reference validity;
 *     - objective validity;
 *     - loss validity;
 *     - optimizer compatibility;
 *     - metric compatibility;
 *     - hyperparameter validity;
 *     - schedule validity;
 *     - phase/step relationships;
 *     - resource requirements;
 *     - capability requirements;
 *     - constraint consistency;
 *     - preference legality;
 *     - reproducibility semantics;
 *     - checkpoint semantics;
 *     - distributed semantics;
 *     - quantum/classical boundary validity.
 *
 * None of these decisions belong in this parser grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The semantic model is lowered into the repository's canonical compiler IR.
 *
 * AI training must not create:
 *
 *     TrainingIR
 *     MLIR
 *     FrameworkIR
 *     VendorTrainingIR
 *
 * merely as a frontend grammar artifact.
 *
 * If a training-specific intermediate representation is required internally,
 * it must have an explicitly documented ownership and lowering contract and
 * must not compete with the repository's canonical semantic/IR boundary.
 *
 * Quantum-derived operations ultimately follow:
 *
 *     training semantics
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * where the computation is genuinely quantum.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages consume semantic information, not parser-specific
 * implementation details.
 *
 * The compiler may:
 *
 *     - specialize;
 *     - fuse operations;
 *     - lower tensor computations;
 *     - distribute work;
 *     - select available capabilities;
 *     - optimize;
 *     - schedule;
 *     - route quantum work;
 *     - lower to hardware.
 *
 * None of these decisions modify training grammar semantics.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may discover:
 *
 *     - available memory;
 *     - accelerators;
 *     - devices;
 *     - nodes;
 *     - storage;
 *     - network resources;
 *     - quantum capabilities.
 *
 * Such discovery MUST NOT alter parsing or source meaning.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Diagnostics must preserve source spans for:
 *
 *     - invalid training declaration;
 *     - invalid annotation;
 *     - duplicate name;
 *     - invalid objective;
 *     - invalid loss;
 *     - invalid optimizer;
 *     - invalid metric;
 *     - invalid requirement;
 *     - invalid capability;
 *     - invalid constraint;
 *     - invalid preference;
 *     - invalid phase;
 *     - invalid step;
 *     - invalid expression;
 *     - invalid type.
 *
 * Parser diagnostics must remain syntax-focused.
 *
 * Semantic diagnostics must explain semantic failures.
 *
 * Resource diagnostics must distinguish:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *
 * rather than presenting all of them as "hardware errors".
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing training source must not:
 *
 *     - execute training;
 *     - load a model;
 *     - load a dataset;
 *     - access a network;
 *     - access credentials;
 *     - access hardware;
 *     - allocate devices;
 *     - invoke frameworks.
 *
 * Such operations belong to explicitly authorized downstream compiler/runtime
 * components.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source;
 *     - lexer version;
 *     - parser grammar;
 *     - explicitly selected dialect configuration.
 *
 * It must not depend on:
 *
 *     - hardware availability;
 *     - GPU count;
 *     - QPU availability;
 *     - network state;
 *     - filesystem state;
 *     - randomness;
 *     - wall-clock time.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests include:
 *
 *     @training experiment;
 *
 *     @training experiment {
 *         @model model;
 *         @dataset data;
 *         @objective objective = loss;
 *         @loss loss = compute_loss(output, target);
 *         @optimizer optimizer = optimizer_config;
 *         @metric accuracy = metric_fn;
 *     }
 *
 *     @training experiment {
 *         requires capability("tensor.compute");
 *         requires capability("distributed.training");
 *         requires capability("quantum.measurement");
 *         prefer capability("accelerator.compute");
 *         hint optimization_policy;
 *     }
 *
 *     @training experiment {
 *         @phase warmup {
 *             @step initialize = initialize_model();
 *         }
 *
 *         @phase optimization {
 *             @step update = update_model();
 *         }
 *     }
 *
 *     @training experiment {
 *         @distributed distribution_policy;
 *         @accelerator accelerator_policy;
 *         @hybrid quantum_training_value;
 *     }
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The grammar/test suite must reject:
 *
 *     - missing training name;
 *     - missing annotation name;
 *     - malformed type clause;
 *     - malformed initializer;
 *     - malformed member;
 *     - malformed requirement;
 *     - malformed capability;
 *     - malformed phase;
 *     - malformed step;
 *     - incomplete expression.
 *
 * It must also ensure that arbitrary ordinary expressions are NOT accepted
 * merely because `trainingConstruct` was selected.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests must demonstrate that the grammar accepts structurally unbounded:
 *
 *     - phases;
 *     - steps;
 *     - objectives;
 *     - metrics;
 *     - hyperparameters;
 *     - checkpoints;
 *     - nested training members;
 *     - resource requirements;
 *     - capability requirements;
 *     - expression complexity.
 *
 * Tests must NOT establish artificial universal limits.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Required integration coverage:
 *
 *     classical + AI
 *     quantum + AI
 *     classical + quantum + AI
 *     AI + data
 *     AI + distributed
 *     AI + hardware intent
 *     AI + networking
 *     AI + security
 *     AI + HDL/accelerator intent
 *
 * ============================================================================
 * POCO-REAF TEST
 * ============================================================================
 *
 * The same semantic training source must remain valid when the target changes
 * between:
 *
 *     embedded CPU
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     cluster
 *     heterogeneous system
 *     future target
 *
 * without modifying this grammar to accommodate the target's capacity.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Search this file for and reject universal machine limits such as:
 *
 *     MAX_*
 *     GPU_0
 *     CPU_0
 *     QPU_0
 *     NODE_0
 *     DEVICE_0
 *     fixed worker counts
 *     fixed node counts
 *     fixed tensor dimensions
 *     fixed tensor rank
 *     fixed memory capacities
 *
 * A numeric literal in a test/example is acceptable when it is program data,
 * not a compiler limit.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * The grammar itself is target-independent ANTLR4.
 *
 * The generated/consuming Rust implementation must remain compatible with:
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
 * This file is complete when:
 *
 * [ ] AT is used instead of obsolete NANO_ANNOTATION.
 *
 * [ ] SEMICOLON matches the canonical lexer vocabulary.
 *
 * [ ] Types come only from Types.
 *
 * [ ] Expressions come only from Expressions.
 *
 * [ ] Statements come only from Statements.
 *
 * [ ] No arbitrary expression is a public training construct.
 *
 * [ ] Training declarations have a clear syntactic boundary.
 *
 * [ ] Resource requirements are distinct from capabilities.
 *
 * [ ] Capabilities are distinct from constraints.
 *
 * [ ] Constraints are distinct from preferences.
 *
 * [ ] Preferences are distinct from hints.
 *
 * [ ] Models are not redefined.
 *
 * [ ] Datasets are not redefined.
 *
 * [ ] Tensors are not redefined.
 *
 * [ ] Differentiation is not redefined.
 *
 * [ ] No AI framework is hard-coded.
 *
 * [ ] No hardware target is hard-coded.
 *
 * [ ] No physical resource limit is hard-coded.
 *
 * [ ] No quantum gate list is hard-coded.
 *
 * [ ] quantum::ir remains the canonical quantum boundary.
 *
 * [ ] No second AI/quantum IR is introduced by this grammar.
 *
 * [ ] Source spans are preserved downstream.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97/1.97.1 integration passes.
 *
 * [ ] Safe-Rust policy remains satisfied.
 *
 * ============================================================================
 */