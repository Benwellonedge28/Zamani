/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/training.g4
 *
 * Role:
 *     Production AI / ML training-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime execution.
 *     - No unsafe implementation.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL SYNTAX for AI/ML TRAINING INTENT.
 *
 * Training syntax describes the semantic intent of a learning computation:
 *
 *     - training declarations;
 *     - training regions;
 *     - training inputs;
 *     - model references;
 *     - dataset references;
 *     - objectives;
 *     - losses;
 *     - optimizers;
 *     - metrics;
 *     - hyperparameters;
 *     - schedules;
 *     - training phases;
 *     - training steps;
 *     - validation;
 *     - evaluation;
 *     - checkpoints;
 *     - reproducibility metadata;
 *     - resource requirements;
 *     - capability requirements;
 *     - constraints;
 *     - preferences;
 *     - distributed-training intent;
 *     - accelerator intent;
 *     - quantum-assisted training intent;
 *     - classical training intent;
 *     - hybrid training intent;
 *     - custom/future training strategies.
 *
 * This grammar does NOT implement training.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Canonical parser
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     Types / Expressions        AI training syntax
 *                                     |
 *                                     v
 *                              Frontend AST
 *                                     |
 *                                     v
 *                            Semantic analysis
 *                                     |
 *                  +------------------+------------------+
 *                  |                  |                  |
 *                  v                  v                  v
 *              AI semantics      Resources          Capabilities
 *                  |                  |                  |
 *                  +------------------+------------------+
 *                                     |
 *                                     v
 *                              Canonical semantic IR
 *                                     |
 *                    +----------------+----------------+
 *                    |                |                |
 *                    v                v                v
 *                Classical        Quantum          Accelerator
 *                  lowering        lowering           lowering
 *                    |                |                |
 *                    +----------------+----------------+
 *                                     |
 *                                     v
 *                                Optimization
 *                                     |
 *                                     v
 *                                 Scheduling
 *                                     |
 *                                     v
 *                              Target realization
 *                                     |
 *                                     v
 *                                  Runtime
 *
 * training.g4 MUST NOT construct IR directly.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - training declaration syntax;
 *     - training region syntax;
 *     - training phase syntax;
 *     - training step syntax;
 *     - training objective boundaries;
 *     - training loss boundaries;
 *     - optimizer boundaries;
 *     - metric boundaries;
 *     - hyperparameter boundaries;
 *     - schedule boundaries;
 *     - validation boundaries;
 *     - checkpoint boundaries;
 *     - reproducibility boundaries;
 *     - training resource/capability/constraint boundaries;
 *     - distributed-training intent boundaries;
 *     - accelerator-training intent boundaries;
 *     - extensible training operation boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - AI keywords;
 *     - identifiers;
 *     - general expressions;
 *     - general statements;
 *     - canonical types;
 *     - tensor types;
 *     - tensor implementation;
 *     - model declarations;
 *     - dataset declarations;
 *     - inference;
 *     - differentiation implementation;
 *     - optimizer implementation;
 *     - numerical kernels;
 *     - accelerator implementation;
 *     - GPU selection;
 *     - CPU selection;
 *     - TPU/NPU selection;
 *     - quantum hardware selection;
 *     - hardware discovery;
 *     - hardware topology;
 *     - resource allocation;
 *     - scheduling;
 *     - compiler optimization;
 *     - canonical IR;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - backend selection;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * DOMAIN KEYWORD POLICY
 * ============================================================================
 *
 * Training vocabulary is NOT added as a collection of lexer keywords.
 *
 * Consequently:
 *
 *     training
 *     model
 *     dataset
 *     objective
 *     loss
 *     optimizer
 *     metric
 *     parameter
 *     phase
 *     step
 *     checkpoint
 *     validation
 *     schedule
 *
 * remain extensible domain identifiers.
 *
 * The canonical NANO_ANNOTATION token provides the syntactic domain boundary.
 *
 * Semantic analysis validates annotation meaning.
 *
 * For example:
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
 *     @validation
 *     @checkpoint
 *
 * are semantic annotations, not new lexer keywords.
 *
 * This preserves library, dialect and future-language extensibility.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Training syntax describes:
 *
 *     WHAT is being optimized;
 *     WHAT data participates;
 *     WHAT objective is intended;
 *     WHAT optimization semantics are requested;
 *     WHAT measurements matter;
 *     WHAT constraints apply;
 *     WHAT capabilities are required;
 *     WHAT resource preferences exist;
 *     WHAT reproducibility guarantees are requested.
 *
 * It MUST NOT silently encode:
 *
 *     - a specific GPU;
 *     - a specific CPU;
 *     - a specific TPU;
 *     - a specific NPU;
 *     - a specific accelerator;
 *     - a specific device ID;
 *     - a fixed device count;
 *     - a fixed worker count;
 *     - a fixed node count;
 *     - a fixed cluster topology;
 *     - a fixed memory capacity;
 *     - a fixed VRAM capacity;
 *     - a fixed SIMD width;
 *     - a fixed accelerator topology.
 *
 * Those concerns belong to:
 *
 *     resources;
 *     capabilities;
 *     targets;
 *     optimization;
 *     scheduling;
 *     deployment;
 *     runtime.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     - training datasets;
 *     - number of models;
 *     - number of objectives;
 *     - number of losses;
 *     - number of metrics;
 *     - number of parameters;
 *     - number of hyperparameters;
 *     - number of phases;
 *     - number of steps;
 *     - number of validation stages;
 *     - number of checkpoints;
 *     - number of workers;
 *     - number of devices;
 *     - number of nodes;
 *     - number of accelerators;
 *     - model size;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - batch size;
 *     - sequence length;
 *     - feature count;
 *     - class count.
 *
 * Repetition is structural.
 *
 * Practical limits are determined by:
 *
 *     - available memory;
 *     - compiler resources;
 *     - runtime resources;
 *     - resource policies;
 *     - target capabilities;
 *     - deployment constraints.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * Canonical types are owned by the shared type grammar.
 *
 * This file reuses:
 *
 *     typeExpression
 *
 * It does NOT define:
 *
 *     TrainingType
 *     ModelType
 *     DatasetType
 *     TensorType
 *     OptimizerType
 *     MetricType
 *
 * as competing type systems.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions are owned by the canonical expression grammar.
 *
 * This file reuses:
 *
 *     expression
 *
 * for:
 *
 *     - model values;
 *     - dataset values;
 *     - loss expressions;
 *     - objective expressions;
 *     - metric expressions;
 *     - optimizer expressions;
 *     - hyperparameters;
 *     - schedules;
 *     - predicates;
 *     - symbolic dimensions;
 *     - tensor expressions;
 *     - quantum expressions;
 *     - hardware-independent resource expressions;
 *     - compile-time expressions;
 *     - runtime expressions where semantically permitted.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Training may consume:
 *
 *     classical values;
 *     tensor values;
 *     model values;
 *     dataset values;
 *     quantum-derived values;
 *     hardware-backed values;
 *     distributed values;
 *     accelerator values;
 *     future domain values.
 *
 * Training syntax does not own those domains.
 *
 * It merely provides semantic boundaries where those values participate in
 * training.
 *
 * ============================================================================
 *
 * DIFFERENTIATION CONTRACT
 * ============================================================================
 *
 * A training declaration may reference differentiated values and operations.
 *
 * This file does NOT define automatic differentiation.
 *
 * Differentiation belongs to:
 *
 *     grammar/ai/differentiation.g4
 *
 * or its eventual canonical semantic owner.
 *
 * ============================================================================
 *
 * DATASET CONTRACT
 * ============================================================================
 *
 * Training may reference dataset values.
 *
 * This file does NOT redefine dataset declarations.
 *
 * Dataset declarations belong to:
 *
 *     grammar/ai/datasets.g4
 *
 * Training receives dataset semantics through ordinary expressions and
 * semantic references.
 *
 * ============================================================================
 *
 * MODEL CONTRACT
 * ============================================================================
 *
 * Training may reference model values.
 *
 * This file does NOT redefine model declarations.
 *
 * Model declarations belong to:
 *
 *     grammar/ai/models.g4
 *
 * ============================================================================
 *
 * TENSOR CONTRACT
 * ============================================================================
 *
 * Training may consume tensor values and tensor types.
 *
 * This file does NOT redefine tensor syntax.
 *
 * Tensor semantics belong to:
 *
 *     grammar/ai/tensors.g4
 *
 * and the canonical tensor/classical type system.
 *
 * ============================================================================
 *
 * DISTRIBUTED TRAINING CONTRACT
 * ============================================================================
 *
 * Distributed training can express intent such as:
 *
 *     partitioning;
 *     replication;
 *     synchronization;
 *     locality;
 *     consistency;
 *     parallel execution;
 *     fault tolerance.
 *
 * It MUST NOT encode:
 *
 *     worker count;
 *     node count;
 *     node identifiers;
 *     cluster topology;
 *     network addresses;
 *     device identifiers.
 *
 * Those are downstream deployment/resource concerns.
 *
 * ============================================================================
 *
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * Reproducibility metadata is explicit semantic information.
 *
 * The grammar must never silently insert:
 *
 *     random seeds;
 *     ordering policies;
 *     initialization algorithms;
 *     deterministic execution modes.
 *
 * If reproducibility is requested, it must be represented explicitly through
 * an expression or annotation and validated downstream.
 *
 * ============================================================================
 *
 * CHECKPOINT CONTRACT
 * ============================================================================
 *
 * A checkpoint declaration expresses checkpoint intent.
 *
 * It does NOT imply a particular:
 *
 *     filesystem;
 *     serialization format;
 *     storage device;
 *     cloud provider;
 *     database;
 *     memory representation.
 *
 * The runtime and interoperability layers determine the realization.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Stable public entry point:
 *
 *     trainingConstruct
 *
 * The canonical AI grammar should delegate training syntax to this rule.
 *
 * This grammar MUST NOT import ai.g4.
 *
 * This grammar SHOULD NOT import models.g4, datasets.g4, or tensors.g4.
 *
 * Those domains are consumed through semantic values and shared
 * expression/type syntax.
 *
 * This avoids:
 *
 *     AI -> Training -> AI
 *
 * and:
 *
 *     Models -> Training -> Models
 *
 * style grammar cycles.
 *
 * ============================================================================
 */

parser grammar AITraining;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Stable public parser boundary for training-domain syntax.
 */
trainingConstruct
    : trainingDeclaration
    | trainingReference
    | trainingExpression
    | trainingRegion
    | trainingAssignment
    ;


/* ============================================================================
 * 2. TRAINING DECLARATION
 * ========================================================================== */

/**
 * Canonical training declaration.
 *
 * Semantic validation MUST require the leading annotation to represent
 * `@training`.
 *
 * Examples:
 *
 *     @training experiment;
 *
 *     @training experiment = train(model, dataset);
 *
 *     @training experiment {
 *         @model model;
 *         @dataset data;
 *         @loss lossFunction;
 *         @optimizer optimizer;
 *     }
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
    : NANO_ANNOTATION
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
 * 3. TRAINING BODY
 * ========================================================================== */

/**
 * Training body is an unbounded ordered sequence of training members.
 */
trainingBody
    : LBRACE
      trainingMember*
      RBRACE
    ;


trainingMember
    : trainingBindingMember
    | trainingReferenceMember
    | trainingObjectiveMember
    | trainingLossMember
    | trainingOptimizerMember
    | trainingMetricMember
    | trainingHyperparameterMember
    | trainingScheduleMember
    | trainingPhase
    | trainingStep
    | trainingValidationMember
    | trainingCheckpointMember
    | trainingReproducibilityMember
    | trainingRequirementMember
    | trainingCapabilityMember
    | trainingConstraintMember
    | trainingPreferenceMember
    | trainingResourceMember
    | trainingDistributedMember
    | trainingAcceleratorMember
    | trainingOperationMember
    | trainingStatementMember
    ;


/* ============================================================================
 * 4. GENERIC TRAINING BINDINGS
 * ========================================================================== */

/**
 * General typed or initialized training-local binding.
 *
 * Semantic analysis determines whether the annotation denotes a model,
 * dataset, tensor, hyperparameter, state, metric, or another training value.
 */
trainingBindingMember
    : NANO_ANNOTATION
      identifier
      trainingBindingType?
      trainingBindingInitializer?
      SEMICOLON
    ;


trainingBindingType
    : COLON
      typeExpression
    ;


trainingBindingInitializer
    : ASSIGN
      expression
    ;


/**
 * A semantic reference without introducing another declaration.
 */
trainingReferenceMember
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 5. MODEL / DATASET REFERENCES
 * ========================================================================== */

/**
 * Training consumes models through expressions.
 *
 * The semantic layer resolves the expression to the model domain.
 */
trainingModelReference
    : expression
    ;


/**
 * Training consumes datasets through expressions.
 *
 * The semantic layer resolves the expression to the dataset domain.
 */
trainingDatasetReference
    : expression
    ;


/**
 * Training consumes tensor values through expressions.
 */
trainingTensorReference
    : expression
    ;


/* ============================================================================
 * 6. OBJECTIVES
 * ========================================================================== */

/**
 * Objective declaration.
 *
 * Semantic validation MUST require the annotation to represent `@objective`.
 */
trainingObjectiveMember
    : NANO_ANNOTATION
      identifier?
      ASSIGN
      expression
      SEMICOLON
    ;


/**
 * Objective may also be represented without an explicit binding.
 *
 * Example:
 *
 *     @objective minimize(error);
 */
trainingObjectiveExpression
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 7. LOSS
 * ========================================================================== */

/**
 * Loss definition.
 *
 * Examples:
 *
 *     @loss loss = cross_entropy;
 *
 *     @loss cross_entropy(output, target);
 */
trainingLossMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


trainingNamedOrUnnamedExpression
    : identifier
      ASSIGN
      expression
    | expression
    ;


/* ============================================================================
 * 8. OPTIMIZER
 * ========================================================================== */

/**
 * Optimizer intent.
 *
 * The grammar intentionally does not enumerate:
 *
 *     SGD
 *     Adam
 *     AdamW
 *     RMSProp
 *     evolutionary methods
 *     quantum optimization
 *     future optimizers
 *
 * They remain semantic/library-level values.
 */
trainingOptimizerMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 9. METRICS
 * ========================================================================== */

/**
 * Training metric.
 *
 * Metric implementations are semantic/library concerns.
 */
trainingMetricMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 10. HYPERPARAMETERS
 * ========================================================================== */

/**
 * Hyperparameters are expressions, not grammar-level constants.
 *
 * This allows:
 *
 *     literals;
 *     symbolic values;
 *     configuration values;
 *     compile-time values;
 *     runtime values;
 *     derived values;
 *     resource-aware values.
 */
trainingHyperparameterMember
    : NANO_ANNOTATION
      identifier
      trainingHyperparameterType?
      ASSIGN
      expression
      SEMICOLON
    ;


trainingHyperparameterType
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 11. SCHEDULES
 * ========================================================================== */

/**
 * A training schedule expresses semantic scheduling intent.
 *
 * This is NOT Zamani compiler scheduling.
 *
 * The runtime/compiler scheduling subsystem remains responsible for actual
 * execution ordering and timing.
 */
trainingScheduleMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 12. TRAINING PHASES
 * ========================================================================== */

/**
 * Named phase containing nested training members.
 *
 * The number of phases is unbounded.
 */
trainingPhase
    : NANO_ANNOTATION
      identifier
      trainingPhaseBody
    ;


trainingPhaseBody
    : LBRACE
      trainingMember*
      RBRACE
    ;


/* ============================================================================
 * 13. TRAINING STEPS
 * ========================================================================== */

/**
 * Named semantic training step.
 *
 * A step does not prescribe:
 *
 *     - one kernel;
 *     - one device;
 *     - one thread;
 *     - one accelerator;
 *     - one scheduling mechanism.
 */
trainingStep
    : NANO_ANNOTATION
      identifier?
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. VALIDATION
 * ========================================================================== */

/**
 * Validation intent.
 *
 * Validation implementation remains outside the grammar.
 */
trainingValidationMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 15. CHECKPOINTS
 * ========================================================================== */

/**
 * Checkpoint intent.
 *
 * The expression determines the semantic checkpoint policy.
 *
 * The grammar does not prescribe storage.
 */
trainingCheckpointMember
    : NANO_ANNOTATION
      trainingCheckpointPayload
      SEMICOLON
    ;


trainingCheckpointPayload
    : identifier
      ASSIGN
      expression
    | expression
    ;


/* ============================================================================
 * 16. REPRODUCIBILITY
 * ========================================================================== */

/**
 * Explicit reproducibility declaration.
 *
 * No implicit seed or ordering is introduced.
 */
trainingReproducibilityMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 17. REQUIREMENTS
 * ========================================================================== */

/**
 * Training requirement.
 *
 * Examples may semantically express:
 *
 *     numerical precision requirements;
 *     memory requirements;
 *     throughput requirements;
 *     latency requirements;
 *     quantum capability requirements;
 *     distributed execution requirements.
 *
 * No physical resource is allocated here.
 */
trainingRequirementMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 18. CAPABILITIES
 * ========================================================================== */

trainingCapabilityMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 19. CONSTRAINTS
 * ========================================================================== */

trainingConstraintMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 20. PREFERENCES
 * ========================================================================== */

trainingPreferenceMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 21. RESOURCE INTENT
 * ========================================================================== */

/**
 * Resource declarations express requirements/preferences, not allocation.
 */
trainingResourceMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 22. DISTRIBUTED TRAINING
 * ========================================================================== */

/**
 * Distributed training is represented as semantic intent.
 *
 * It does NOT encode worker/node/device counts or topology.
 */
trainingDistributedMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 23. ACCELERATOR TRAINING
 * ========================================================================== */

/**
 * Accelerator intent.
 *
 * The grammar does not select GPU/TPU/NPU/FPGA/ASIC hardware.
 */
trainingAcceleratorMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 24. GENERIC TRAINING OPERATIONS
 * ========================================================================== */

/**
 * Extensibility boundary for future training operations.
 *
 * Examples can semantically represent:
 *
 *     federated training;
 *     reinforcement learning;
 *     graph learning;
 *     multimodal learning;
 *     transfer learning;
 *     meta-learning;
 *     continual learning;
 *     self-supervised learning;
 *     quantum-assisted learning;
 *     hybrid learning;
 *     future learning paradigms.
 *
 * The grammar does not need a new production for every future algorithm.
 */
trainingOperationMember
    : NANO_ANNOTATION
      trainingNamedOrUnnamedExpression
      SEMICOLON
    ;


/* ============================================================================
 * 25. ORDINARY ZAMANI STATEMENTS
 * ========================================================================== */

/**
 * Training regions may contain ordinary Zamani statements.
 *
 * This prevents training.g4 from becoming a second general-purpose statement
 * grammar.
 */
trainingStatementMember
    : statement
    ;


/* ============================================================================
 * 26. TRAINING REGIONS
 * ========================================================================== */

/**
 * Anonymous training computation region.
 */
trainingRegion
    : LBRACE
      trainingRegionMember*
      RBRACE
    ;


trainingRegionMember
    : trainingMember
    | statement
    ;


/* ============================================================================
 * 27. TRAINING REFERENCES
 * ========================================================================== */

/**
 * Reference to a training value.
 */
trainingReference
    : identifier
    ;


/**
 * General training expression.
 */
trainingExpression
    : expression
    ;


/* ============================================================================
 * 28. TRAINING ASSIGNMENTS
 * ========================================================================== */

/**
 * Training-local assignment boundary.
 *
 * General assignment semantics remain owned by the expression/statement
 * subsystem.
 */
trainingAssignment
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;