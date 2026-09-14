/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/ai.g4
 *
 * Status:
 *     Production-ready AI / ML domain composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no target-specific implementation;
 *       - no filesystem access;
 *       - no network access;
 *       - no runtime execution;
 *       - no unsafe implementation.
 *
 * Rust consumers MUST use safe Rust only.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file defines the AUTHORITATIVE PARSER BOUNDARY for AI / machine-
 * learning-domain syntax in Zamani.
 *
 * It provides stable syntactic integration points for:
 *
 *     - machine-learning computations;
 *     - models;
 *     - model values;
 *     - datasets;
 *     - tensors;
 *     - layers;
 *     - training;
 *     - inference;
 *     - evaluation;
 *     - optimization;
 *     - differentiation;
 *     - model pipelines;
 *     - agents;
 *     - reinforcement learning;
 *     - graph learning;
 *     - time-series learning;
 *     - generative computation;
 *     - multimodal computation;
 *     - AI/classical interoperability;
 *     - AI/quantum interoperability;
 *     - AI/hardware interoperability;
 *     - AI/distributed interoperability;
 *     - AI accelerator interoperability;
 *     - AI resource requirements;
 *     - AI capabilities;
 *     - AI execution regions.
 *
 * The grammar deliberately describes COMPUTATIONAL INTENT rather than a
 * particular ML framework, accelerator, model implementation, device, or
 * execution environment.
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
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     Types / Expressions             AI domain
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                    Frontend AST
 *                          |
 *                          v
 *              Semantic / type analysis
 *                          |
 *          +---------------+----------------+
 *          |                                |
 *          v                                v
 *    AI semantic model                Resource metadata
 *          |                                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                 Canonical semantic IR
 *                          |
 *              +-----------+-----------+
 *              |           |           |
 *              v           v           v
 *          Classical    Quantum    Accelerator
 *             IR          IR        lowering
 *              |           |           |
 *              +-----------+-----------+
 *                          |
 *                          v
 *                    Optimization
 *                          |
 *                          v
 *                     Scheduling
 *                          |
 *                          v
 *                  Target realization
 *                          |
 *                          v
 *                       Runtime
 *
 * AI.g4 MUST NOT construct IR directly.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI-domain parser composition;
 *     - AI-domain semantic boundary rules;
 *     - AI-domain computation regions;
 *     - AI-domain model/value boundaries;
 *     - AI-domain dataset/value boundaries;
 *     - AI-domain tensor/value boundaries;
 *     - AI-domain training/inference/evaluation boundaries;
 *     - AI-domain pipeline boundaries;
 *     - AI-domain agent boundaries;
 *     - AI-domain differentiation boundaries;
 *     - AI-domain optimization boundaries;
 *     - AI-domain resource/capability boundaries;
 *     - AI-domain interoperability boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - keyword definitions;
 *     - identifiers;
 *     - literals;
 *     - general expressions;
 *     - general statements;
 *     - general declarations;
 *     - general types;
 *     - tensor storage;
 *     - tensor implementation;
 *     - numerical kernels;
 *     - automatic differentiation implementation;
 *     - optimizer implementation;
 *     - model implementation;
 *     - dataset storage;
 *     - training algorithms;
 *     - inference algorithms;
 *     - neural-network execution;
 *     - accelerator implementation;
 *     - GPU selection;
 *     - TPU selection;
 *     - NPU selection;
 *     - CPU selection;
 *     - device discovery;
 *     - distributed placement;
 *     - scheduling;
 *     - runtime execution;
 *     - canonical IR;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - hardware calibration;
 *     - backend selection.
 *
 * ============================================================================
 *
 * DOMAIN KEYWORD POLICY
 * ============================================================================
 *
 * AI operations are deliberately NOT introduced as reserved lexer keywords.
 *
 * Therefore words such as:
 *
 *     model
 *     dataset
 *     tensor
 *     train
 *     infer
 *     inference
 *     layer
 *     optimizer
 *     gradient
 *     loss
 *     agent
 *     policy
 *     reward
 *     attention
 *     embedding
 *     transformer
 *     convolution
 *     classifier
 *
 * remain ordinary identifiers unless another language specification explicitly
 * promotes one to a reserved keyword.
 *
 * This is intentional.
 *
 * It allows:
 *
 *     user-defined AI libraries;
 *     future algorithms;
 *     vendor-independent dialects;
 *     experimental AI constructs;
 *     domain-specific model APIs;
 *     library evolution;
 *
 * without requiring lexical grammar changes.
 *
 * Semantic analysis determines whether an identifier denotes a canonical
 * AI operation, library operation, user-defined operation, dialect operation,
 * or another callable value.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * AI syntax describes:
 *
 *     - computation;
 *     - model intent;
 *     - data relationships;
 *     - transformations;
 *     - training intent;
 *     - inference intent;
 *     - evaluation intent;
 *     - optimization intent;
 *     - differentiation intent;
 *     - resource requirements;
 *     - capability requirements;
 *     - portability constraints.
 *
 * It MUST NOT implicitly describe:
 *
 *     - a particular GPU;
 *     - a particular CPU;
 *     - a particular accelerator;
 *     - a particular TPU/NPU;
 *     - a particular device ID;
 *     - a particular memory capacity;
 *     - a particular VRAM size;
 *     - a particular number of devices;
 *     - a particular number of cores;
 *     - a particular number of threads;
 *     - a particular SIMD width;
 *     - a particular cluster size;
 *     - a particular node topology;
 *     - a particular network topology;
 *     - a particular accelerator vendor.
 *
 * Those concerns belong to semantic/resource analysis, optimization,
 * scheduling, target lowering, deployment and runtime.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level limits for:
 *
 *     - model size;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - parameter count;
 *     - layer count;
 *     - dataset size;
 *     - batch size;
 *     - sequence length;
 *     - feature count;
 *     - class count;
 *     - number of models;
 *     - number of pipeline stages;
 *     - number of agents;
 *     - number of training steps;
 *     - number of devices;
 *     - number of accelerators;
 *     - number of nodes;
 *     - number of workers;
 *     - number of distributed partitions.
 *
 * Repetition is structural.
 *
 * Practical limits are resource-policy/compiler/runtime concerns and MUST NOT
 * become language-semantic limits.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * AI.g4 reuses the canonical type grammar.
 *
 * Canonical source-level types are owned by:
 *
 *     grammar/antlr/Types.g4
 *
 * AI.g4 MUST NOT define another type system.
 *
 * AI values may semantically resolve to:
 *
 *     - scalar values;
 *     - vectors;
 *     - matrices;
 *     - tensors;
 *     - model values;
 *     - dataset values;
 *     - layer values;
 *     - optimizer values;
 *     - agent values;
 *     - distributed values;
 *     - quantum-derived values;
 *     - hardware-backed values;
 *     - user-defined values.
 *
 * Their actual semantic representation belongs downstream.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * AI.g4 therefore reuses:
 *
 *     expression
 *     typeExpression
 *
 * rather than redefining:
 *
 *     arithmetic;
 *     calls;
 *     indexing;
 *     member access;
 *     assignment;
 *     literals;
 *     lambdas;
 *     ranges;
 *     operators;
 *     comprehensions.
 *
 * ============================================================================
 *
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * AI.g4 is intentionally parser-composable.
 *
 * The canonical parser may import it and expose:
 *
 *     aiConstruct
 *
 * to semantic analysis.
 *
 * More specialized AI grammars can depend on this stable boundary:
 *
 *     grammar/ai/models.g4
 *     grammar/ai/tensors.g4
 *     grammar/ai/datasets.g4
 *     grammar/ai/training.g4
 *     grammar/ai/inference.g4
 *     grammar/ai/agents.g4
 *     grammar/ai/pipelines.g4
 *     grammar/ai/differentiation.g4
 *     grammar/ai/ai-accelerators.g4
 *
 * without requiring changes to the lexical layer.
 *
 * Specialized grammars MUST NOT redefine the ownership of the canonical
 * `expression` or `typeExpression` rules.
 *
 * ============================================================================
 */

parser grammar AI;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC AI ENTRY POINT
 * ========================================================================== */

/**
 * Stable public parser boundary for AI-domain syntax.
 *
 * AI semantic classification occurs downstream.
 */
aiConstruct
    : aiExpression
    | aiValue
    | aiType
    | aiComputationRegion
    | aiModelBoundary
    | aiDatasetBoundary
    | aiTensorBoundary
    | aiTrainingBoundary
    | aiInferenceBoundary
    | aiEvaluationBoundary
    | aiOptimizationBoundary
    | aiDifferentiationBoundary
    | aiPipelineBoundary
    | aiAgentBoundary
    | aiResourceBoundary
    | aiCapabilityBoundary
    | aiInteroperabilityBoundary
    ;


/* ============================================================================
 * 2. GENERAL AI EXPRESSION
 * ========================================================================== */

/**
 * AI classification of an ordinary Zamani expression.
 *
 * No separate AI expression language is introduced.
 */
aiExpression
    : expression
    ;


/**
 * AI value boundary.
 */
aiValue
    : expression
    ;


/**
 * AI type boundary.
 */
aiType
    : typeExpression
    ;


/* ============================================================================
 * 3. AI COMPUTATION REGION
 * ========================================================================== */

/**
 * A computation region containing ordinary Zamani statements.
 *
 * Statement semantics remain owned by the canonical statement grammar.
 */
aiComputationRegion
    : LBRACE aiStatementSequence RBRACE
    ;


aiStatementSequence
    : statement*
    ;


/* ============================================================================
 * 4. MODEL BOUNDARY
 * ========================================================================== */

/**
 * Model references are ordinary Zamani values.
 *
 * Model identity, model architecture, parameter representation, and model
 * execution semantics are semantic concerns.
 */
aiModelBoundary
    : aiModelReference
    | aiModelExpression
    | aiModelCall
    ;


aiModelReference
    : identifier
    ;


aiModelExpression
    : expression
    ;


aiModelCall
    : expression
    ;


/* ============================================================================
 * 5. DATASET BOUNDARY
 * ========================================================================== */

/**
 * Dataset values remain ordinary Zamani expressions.
 *
 * Dataset storage, streaming, sharding, persistence and distributed
 * placement are not grammar responsibilities.
 */
aiDatasetBoundary
    : aiDatasetReference
    | aiDatasetExpression
    | aiDatasetTransformation
    ;


aiDatasetReference
    : identifier
    ;


aiDatasetExpression
    : expression
    ;


aiDatasetTransformation
    : expression
    ;


/* ============================================================================
 * 6. TENSOR BOUNDARY
 * ========================================================================== */

/**
 * Tensor syntax is already supported by the classical/tensor domain.
 *
 * AI.g4 deliberately does not define a competing tensor grammar.
 */
aiTensorBoundary
    : aiTensorExpression
    | aiTensorType
    | aiTensorReference
    ;


aiTensorExpression
    : expression
    ;


aiTensorType
    : typeExpression
    ;


aiTensorReference
    : identifier
    ;


/* ============================================================================
 * 7. TRAINING BOUNDARY
 * ========================================================================== */

/**
 * Training intent is represented through ordinary Zamani expressions and
 * computation regions.
 *
 * The grammar does not hard-code a training algorithm.
 *
 * Therefore SGD, Adam, evolutionary optimization, quantum-assisted training,
 * federated training, distributed training, reinforcement learning, or a
 * future optimization method can all be represented without changing this
 * grammar.
 */
aiTrainingBoundary
    : aiTrainingExpression
    | aiTrainingRegion
    ;


aiTrainingExpression
    : expression
    ;


aiTrainingRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 8. INFERENCE BOUNDARY
 * ========================================================================== */

/**
 * Inference is a semantic execution mode, not a machine-specific syntax.
 */
aiInferenceBoundary
    : aiInferenceExpression
    | aiInferenceRegion
    ;


aiInferenceExpression
    : expression
    ;


aiInferenceRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 9. EVALUATION BOUNDARY
 * ========================================================================== */

/**
 * Evaluation may include:
 *
 *     - metrics;
 *     - validation;
 *     - testing;
 *     - benchmarking;
 *     - statistical analysis;
 *     - model comparison;
 *     - uncertainty analysis.
 *
 * Their concrete meaning is semantic/library-defined.
 */
aiEvaluationBoundary
    : aiEvaluationExpression
    | aiEvaluationRegion
    ;


aiEvaluationExpression
    : expression
    ;


aiEvaluationRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 10. OPTIMIZATION BOUNDARY
 * ========================================================================== */

/**
 * AI optimization is intentionally distinct from Zamani compiler
 * optimization.
 *
 * This boundary can represent:
 *
 *     model optimization;
 *     parameter optimization;
 *     architecture search;
 *     hyperparameter search;
 *     quantization intent;
 *     pruning intent;
 *     distillation intent;
 *     compilation-aware optimization.
 *
 * It does not select a compiler optimization pass.
 */
aiOptimizationBoundary
    : aiOptimizationExpression
    | aiOptimizationRegion
    ;


aiOptimizationExpression
    : expression
    ;


aiOptimizationRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 11. DIFFERENTIATION BOUNDARY
 * ========================================================================== */

/**
 * Differentiation includes semantic concepts such as:
 *
 *     gradients;
 *     Jacobians;
 *     Hessians;
 *     automatic differentiation;
 *     symbolic differentiation;
 *     numerical differentiation;
 *     reverse-mode differentiation;
 *     forward-mode differentiation.
 *
 * The grammar does not select an implementation strategy.
 */
aiDifferentiationBoundary
    : aiDifferentiationExpression
    | aiDifferentiationRegion
    ;


aiDifferentiationExpression
    : expression
    ;


aiDifferentiationRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 12. PIPELINE BOUNDARY
 * ========================================================================== */

/**
 * AI pipelines may represent arbitrary compositions of:
 *
 *     data loading;
 *     preprocessing;
 *     transformation;
 *     feature extraction;
 *     training;
 *     validation;
 *     inference;
 *     postprocessing;
 *     evaluation;
 *     deployment preparation.
 *
 * Pipeline cardinality is unbounded by this grammar.
 */
aiPipelineBoundary
    : aiPipelineExpression
    | aiPipelineRegion
    ;


aiPipelineExpression
    : expression
    ;


aiPipelineRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 13. AGENT BOUNDARY
 * ========================================================================== */

/**
 * Agent syntax is intentionally semantic-domain neutral.
 *
 * An agent may combine:
 *
 *     perception;
 *     state;
 *     planning;
 *     memory;
 *     policy;
 *     action;
 *     feedback;
 *     tools;
 *     learning;
 *     reasoning.
 *
 * The grammar does not prescribe a particular agent architecture.
 */
aiAgentBoundary
    : aiAgentExpression
    | aiAgentRegion
    ;


aiAgentExpression
    : expression
    ;


aiAgentRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 14. AI RESOURCE BOUNDARY
 * ========================================================================== */

/**
 * Resource requirements are expressed using ordinary Zamani expressions.
 *
 * The semantic/resource system determines whether an expression describes:
 *
 *     memory;
 *     compute;
 *     accelerator capacity;
 *     storage;
 *     bandwidth;
 *     latency;
 *     energy;
 *     throughput;
 *     reliability;
 *     scalability.
 *
 * This grammar does not contain resource constants.
 */
aiResourceBoundary
    : aiResourceExpression
    ;


aiResourceExpression
    : expression
    ;


/* ============================================================================
 * 15. AI CAPABILITY BOUNDARY
 * ========================================================================== */

/**
 * Capability requirements are distinct from concrete hardware selection.
 *
 * For example, semantic analysis may distinguish:
 *
 *     requires tensor acceleration
 *
 * from:
 *
 *     use accelerator X
 *
 * AI.g4 only provides the syntactic domain boundary.
 */
aiCapabilityBoundary
    : aiCapabilityExpression
    ;


aiCapabilityExpression
    : expression
    ;


/* ============================================================================
 * 16. AI INTEROPERABILITY BOUNDARY
 * ========================================================================== */

/**
 * AI may interoperate with:
 *
 *     classical computation;
 *     quantum computation;
 *     HDL/hardware;
 *     distributed computation;
 *     networking;
 *     data systems;
 *     external functions;
 *     future computational domains.
 *
 * No domain-specific implementation is selected here.
 */
aiInteroperabilityBoundary
    : aiInteroperabilityExpression
    | aiInteroperabilityRegion
    ;


aiInteroperabilityExpression
    : expression
    ;


aiInteroperabilityRegion
    : LBRACE aiStatementSequence RBRACE
    ;


/* ============================================================================
 * 17. CLASSICAL / AI BOUNDARY
 * ========================================================================== */

/**
 * Classical values may feed AI computations and AI results may feed classical
 * computation.
 *
 * The actual type compatibility is semantic.
 */
aiClassicalBoundary
    : expression
    ;


/* ============================================================================
 * 18. QUANTUM / AI BOUNDARY
 * ========================================================================== */

/**
 * AI/quantum interoperability is represented without creating a second
 * quantum grammar.
 *
 * Quantum syntax remains owned by the quantum domain.
 *
 * Quantum semantic lowering remains owned by the canonical quantum IR
 * pipeline, including quantum::ir.
 */
aiQuantumBoundary
    : expression
    ;


/* ============================================================================
 * 19. HARDWARE / AI BOUNDARY
 * ========================================================================== */

/**
 * Hardware realization is downstream.
 *
 * This rule does NOT encode:
 *
 *     GPU;
 *     TPU;
 *     NPU;
 *     FPGA;
 *     ASIC;
 *     CPU;
 *     device IDs;
 *     memory capacities;
 *     accelerator counts.
 */
aiHardwareBoundary
    : expression
    ;


/* ============================================================================
 * 20. DISTRIBUTED / AI BOUNDARY
 * ========================================================================== */

/**
 * Distributed AI is represented semantically.
 *
 * No node count, worker count, cluster size or topology is encoded.
 */
aiDistributedBoundary
    : expression
    ;


/* ============================================================================
 * 21. DATA / AI BOUNDARY
 * ========================================================================== */

/**
 * Data-domain ownership remains with grammar/data/.
 *
 * AI.g4 only establishes a semantic bridge.
 */
aiDataBoundary
    : expression
    ;


/* ============================================================================
 * 22. AI VALUE WITH TYPE
 * ========================================================================== */

/**
 * Stable syntax boundary for consumers that need to associate an AI value
 * with a canonical Zamani type.
 *
 * Declaration ownership remains outside this file.
 */
aiTypedValue
    : aiValue
    | typeExpression
    ;


/* ============================================================================
 * 23. AI ARGUMENT BOUNDARY
 * ========================================================================== */

/**
 * AI operations use canonical expressions as arguments.
 *
 * No AI-specific argument grammar is introduced.
 */
aiArgument
    : expression
    ;


aiArgumentList
    : aiArgument
      (COMMA aiArgument)*
      COMMA?
    ;


/* ============================================================================
 * 24. AI NAME BOUNDARY
 * ========================================================================== */

/**
 * AI names remain ordinary canonical identifiers.
 *
 * This is critical for extensibility.
 */
aiName
    : identifier
    ;


/* ============================================================================
 * 25. AI QUALIFIED NAME BOUNDARY
 * ========================================================================== */

/**
 * Qualified names are semantic references.
 *
 * The canonical identifier/path grammar remains authoritative.
 */
aiQualifiedName
    : identifier
    ;


/* ============================================================================
 * 26. AI COMPUTATION VALUE
 * ========================================================================== */

/**
 * A computation value may be any canonical expression.
 */
aiComputationValue
    : expression
    ;


/* ============================================================================
 * 27. AI CALL BOUNDARY
 * ========================================================================== */

/**
 * Calls remain owned by expressions/calls.g4.
 *
 * AI semantic analysis decides whether the call represents:
 *
 *     model invocation;
 *     dataset transformation;
 *     training;
 *     inference;
 *     evaluation;
 *     optimizer execution;
 *     agent execution;
 *     tensor operation;
 *     user-defined functionality;
 *     library functionality.
 */
aiCallBoundary
    : expression
    ;


/* ============================================================================
 * 28. AI INDEXING BOUNDARY
 * ========================================================================== */

/**
 * Indexing remains owned by expressions/indexing.g4.
 *
 * This boundary covers semantic indexing of:
 *
 *     tensors;
 *     datasets;
 *     model parameters;
 *     sequences;
 *     feature collections;
 *     arbitrary AI-domain values.
 */
aiIndexBoundary
    : expression
    ;


/* ============================================================================
 * 29. AI MEMBER BOUNDARY
 * ========================================================================== */

/**
 * Member access remains owned by the canonical expression grammar.
 */
aiMemberBoundary
    : expression
    ;


/* ============================================================================
 * 30. AI LAMBDA / FUNCTION BOUNDARY
 * ========================================================================== */

/**
 * Functions and lambdas remain canonical Zamani constructs.
 *
 * AI.g4 does not create a second callable type system.
 */
aiCallableBoundary
    : expression
    ;


/* ============================================================================
 * 31. AI CONTROL BOUNDARY
 * ========================================================================== */

/**
 * AI computations may use normal Zamani control flow.
 *
 * This includes:
 *
 *     conditions;
 *     loops;
 *     matching;
 *     functions;
 *     concurrency;
 *     cancellation;
 *     exceptions;
 *     effects.
 *
 * Their syntax remains owned by the corresponding canonical grammars.
 */
aiControlBoundary
    : expression
    ;


/* ============================================================================
 * 32. AI EFFECT BOUNDARY
 * ========================================================================== */

/**
 * AI effects are interpreted by the canonical effect system.
 *
 * Examples may include:
 *
 *     data access;
 *     accelerator access;
 *     distributed execution;
 *     external service access;
 *     model loading;
 *     persistent state;
 *     nondeterministic sampling.
 *
 * This grammar does not define those effects.
 */
aiEffectBoundary
    : expression
    ;


/* ============================================================================
 * 33. AI DETERMINISM BOUNDARY
 * ========================================================================== */

/**
 * Determinism is a semantic/runtime property.
 *
 * The grammar does not force a particular random-number generator,
 * accelerator execution order, distributed reduction order, or model backend.
 */
aiDeterminismBoundary
    : expression
    ;


/* ============================================================================
 * 34. AI PORTABILITY BOUNDARY
 * ========================================================================== */

/**
 * AI programs remain portable across execution targets.
 *
 * Hardware-specific realization belongs downstream.
 */
aiPortabilityBoundary
    : expression
    ;


/* ============================================================================
 * 35. AI SCALABILITY BOUNDARY
 * ========================================================================== */

/**
 * Scalability requirements are semantic/resource properties.
 *
 * No finite AI resource count is encoded here.
 */
aiScalabilityBoundary
    : expression
    ;


/* ============================================================================
 * 36. AI FUTURE-EXTENSION BOUNDARY
 * ========================================================================== */

/**
 * Future AI paradigms must be representable without changing this grammar
 * merely because a new algorithm, architecture, model family, accelerator,
 * training strategy or deployment strategy appears.
 *
 * New semantics should preferably be introduced through:
 *
 *     libraries;
 *     semantic registries;
 *     dialects;
 *     capabilities;
 *     traits;
 *     generic abstractions;
 *     future domain grammars.
 */
aiExtensionBoundary
    : expression
    | aiComputationRegion
    ;


/* ============================================================================
 * 37. AI DOMAIN COMPOSITION
 * ========================================================================== */

/**
 * This rule is the preferred composition point for future specialized AI
 * grammars.
 *
 * Specialized grammars can classify their constructs beneath this boundary
 * without changing the canonical lexical layer.
 */
aiDomainComposition
    : aiModelBoundary
    | aiDatasetBoundary
    | aiTensorBoundary
    | aiTrainingBoundary
    | aiInferenceBoundary
    | aiEvaluationBoundary
    | aiOptimizationBoundary
    | aiDifferentiationBoundary
    | aiPipelineBoundary
    | aiAgentBoundary
    | aiResourceBoundary
    | aiCapabilityBoundary
    | aiInteroperabilityBoundary
    | aiExtensionBoundary
    ;


/* ============================================================================
 * 38. AI DOMAIN PROGRAM
 * ========================================================================== */

/**
 * A complete AI computation region is a sequence of canonical statements.
 *
 * This is NOT the canonical Zamani program entry point.
 *
 * The canonical parser remains responsible for complete source programs.
 */
aiProgram
    : aiDomainElement* EOF
    ;


aiDomainElement
    : aiDomainComposition
    | aiExpression
    | aiComputationRegion
    ;


/* ============================================================================
 * 39. AI SEMANTIC PRESERVATION CONTRACT
 * ========================================================================== */

/**
 * Parsing this grammar MUST NOT:
 *
 *     - execute a model;
 *     - load a model;
 *     - access a dataset;
 *     - allocate a tensor;
 *     - allocate accelerator memory;
 *     - access a GPU;
 *     - access a QPU;
 *     - access the network;
 *     - access the filesystem;
 *     - select a backend;
 *     - mutate compiler-global state.
 *
 * The parser produces syntax only.
 *
 * Semantic analysis is responsible for determining what an AI construct means.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. AI RESOURCE SEMANTICS
 * ========================================================================== */

/**
 * Resource semantics are deliberately represented as ordinary expressions.
 *
 * This avoids embedding physical resource limits into the grammar.
 *
 * Examples of semantic interpretations include:
 *
 *     memory requirement;
 *     compute requirement;
 *     tensor accelerator capability;
 *     distributed execution requirement;
 *     latency preference;
 *     energy preference;
 *     reliability requirement;
 *     portability requirement.
 */
aiResourceRequirement
    : expression
    ;


aiResourceConstraint
    : expression
    ;


aiResourcePreference
    : expression
    ;


aiResourceHint
    : expression
    ;


/* ============================================================================
 * 41. AI CAPABILITY SEMANTICS
 * ========================================================================== */

/**
 * Capability declarations are semantic concepts.
 *
 * They must not imply physical device selection.
 */
aiCapabilityRequirement
    : expression
    ;


aiCapabilityPreference
    : expression
    ;


/* ============================================================================
 * 42. AI MODEL PARAMETERS
 * ========================================================================== */

/**
 * Model parameters are ordinary AI-domain values.
 *
 * Their representation may be:
 *
 *     scalar;
 *     vector;
 *     matrix;
 *     tensor;
 *     sparse value;
 *     distributed value;
 *     quantum-derived value;
 *     future numerical representation.
 *
 * The grammar imposes no fixed parameter count.
 */
aiParameter
    : expression
    ;


aiParameterList
    : aiParameter
      (COMMA aiParameter)*
      COMMA?
    ;


/* ============================================================================
 * 43. AI FEATURE BOUNDARY
 * ========================================================================== */

/**
 * Features may be represented by any canonical Zamani expression.
 *
 * Feature count is unrestricted by grammar.
 */
aiFeature
    : expression
    ;


aiFeatureList
    : aiFeature
      (COMMA aiFeature)*
      COMMA?
    ;


/* ============================================================================
 * 44. AI SAMPLE BOUNDARY
 * ========================================================================== */

/**
 * Dataset samples are semantic values.
 */
aiSample
    : expression
    ;


aiSampleList
    : aiSample
      (COMMA aiSample)*
      COMMA?
    ;


/* ============================================================================
 * 45. AI MODEL INPUT / OUTPUT
 * ========================================================================== */

/**
 * Model inputs and outputs remain ordinary canonical values.
 */
aiInput
    : expression
    ;


aiOutput
    : expression
    ;


aiInputList
    : aiInput
      (COMMA aiInput)*
      COMMA?
    ;


aiOutputList
    : aiOutput
      (COMMA aiOutput)*
      COMMA?
    ;


/* ============================================================================
 * 46. AI LOSS / OBJECTIVE BOUNDARY
 * ========================================================================== */

/**
 * Loss functions and objectives are semantic expressions.
 *
 * The grammar does not reserve names such as:
 *
 *     mse
 *     cross_entropy
 *     hinge
 *     kl_divergence
 *     custom_loss
 *
 * This allows future and user-defined objectives.
 */
aiObjective
    : expression
    ;


aiLoss
    : expression
    ;


/* ============================================================================
 * 47. AI METRIC BOUNDARY
 * ========================================================================== */

/**
 * Metrics remain semantic/library-defined.
 */
aiMetric
    : expression
    ;


aiMetricList
    : aiMetric
      (COMMA aiMetric)*
      COMMA?
    ;


/* ============================================================================
 * 48. AI STATE BOUNDARY
 * ========================================================================== */

/**
 * AI state may represent:
 *
 *     model state;
 *     optimizer state;
 *     agent state;
 *     recurrent state;
 *     environment state;
 *     distributed state.
 *
 * No serialization or storage implementation is implied.
 */
aiState
    : expression
    ;


/* ============================================================================
 * 49. AI POLICY BOUNDARY
 * ========================================================================== */

aiPolicy
    : expression
    ;


/* ============================================================================
 * 50. AI REWARD BOUNDARY
 * ========================================================================== */

aiReward
    : expression
    ;


/* ============================================================================
 * 51. AI ACTION BOUNDARY
 * ========================================================================== */

aiAction
    : expression
    ;


/* ============================================================================
 * 52. AI ENVIRONMENT BOUNDARY
 * ========================================================================== */

aiEnvironment
    : expression
    ;


/* ============================================================================
 * 53. AI SEMANTIC EXTENSIBILITY
 * ========================================================================== */

/**
 * Future AI concepts can be represented without modifying the lexical layer.
 *
 * This is a deliberate POCO-REAF property:
 *
 *     source semantics
 *          |
 *          v
 *     domain interpretation
 *          |
 *          v
 *     target-independent representation
 *          |
 *          v
 *     target realization
 *
 * New hardware must not require new source semantics merely because its
 * physical characteristics differ.
 */
aiFutureValue
    : expression
    ;