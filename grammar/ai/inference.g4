/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/inference.g4
 *
 * Status:
 *     Production-ready AI / ML inference-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - Parser grammar only.
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
 * This file owns the SOURCE-LEVEL SYNTACTIC BOUNDARY for AI/ML inference.
 *
 * It expresses inference intent without encoding:
 *
 *     - a particular model framework;
 *     - a particular inference engine;
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular TPU;
 *     - a particular NPU;
 *     - a particular accelerator;
 *     - a particular device;
 *     - a fixed batch size;
 *     - a fixed sequence length;
 *     - a fixed tensor rank;
 *     - a fixed tensor dimension;
 *     - a fixed memory capacity;
 *     - a fixed number of workers;
 *     - a fixed deployment topology;
 *     - a fixed network topology.
 *
 * The grammar describes inference intent.
 *
 * Semantic analysis determines what the inference operation means.
 *
 * Compilation, optimization, scheduling, resource management, hardware
 * realization, deployment and runtime determine HOW that intent is realized.
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
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Types / Expressions           AI inference
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                  Frontend AST
 *                        |
 *                        v
 *              Semantic / Type Analysis
 *                        |
 *                        v
 *                 AI Semantic Model
 *                        |
 *          +-------------+-------------+
 *          |                           |
 *          v                           v
 *      Classical                   Quantum
 *       semantics                  semantics
 *          |                           |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                Canonical semantic IR
 *                        |
 *                 Optimization
 *                        |
 *                   Scheduling
 *                        |
 *                 Target lowering
 *                        |
 *                     Runtime
 *
 * inference.g4 MUST NOT construct IR.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - inference declaration syntax;
 *     - inference invocation syntax;
 *     - inference region syntax;
 *     - inference input/output boundaries;
 *     - inference model-reference boundaries;
 *     - inference configuration boundaries;
 *     - inference preprocessing/postprocessing boundaries;
 *     - inference decoding boundaries;
 *     - inference validation/evaluation hooks;
 *     - inference resource/capability/constraint boundaries;
 *     - inference execution-intent boundaries;
 *     - inference-local annotations;
 *     - inference-local composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - literals;
 *     - general expressions;
 *     - general types;
 *     - model definitions;
 *     - tensor definitions;
 *     - dataset definitions;
 *     - training algorithms;
 *     - optimizer algorithms;
 *     - automatic differentiation;
 *     - numerical kernels;
 *     - AI runtime implementation;
 *     - compiler optimization;
 *     - scheduling;
 *     - resource discovery;
 *     - resource allocation;
 *     - hardware discovery;
 *     - hardware topology;
 *     - accelerator selection;
 *     - deployment;
 *     - networking;
 *     - canonical IR;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - hardware calibration.
 *
 * ============================================================================
 *
 * LEXER POLICY
 * ============================================================================
 *
 * This grammar deliberately does NOT introduce new AI inference keywords.
 *
 * Inference vocabulary such as:
 *
 *     @inference
 *     @infer
 *     @model
 *     @input
 *     @output
 *     @decode
 *     @preprocess
 *     @postprocess
 *     @batch
 *     @stream
 *     @deterministic
 *     @stochastic
 *     @temperature
 *     @sampling
 *     @resource
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *     @target
 *     @placement
 *
 * is represented through the repository's generic NANO_ANNOTATION mechanism.
 *
 * Semantic analysis owns the validation of annotation names and meanings.
 *
 * This prevents the lexer from becoming a closed list of AI frameworks,
 * algorithms, vendors or temporary implementation concepts.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * An inference declaration describes WHAT inference means.
 *
 * It may describe:
 *
 *     - which model/value is used;
 *     - which inputs are supplied;
 *     - which outputs are required;
 *     - preprocessing;
 *     - postprocessing;
 *     - decoding;
 *     - execution semantics;
 *     - determinism requirements;
 *     - numerical requirements;
 *     - quality requirements;
 *     - latency requirements;
 *     - resource requirements;
 *     - capability requirements;
 *     - portability constraints;
 *     - execution preferences.
 *
 * It must not silently mean:
 *
 *     "run on GPU 0"
 *
 * or:
 *
 *     "use exactly N devices"
 *
 * or:
 *
 *     "use exactly N cores"
 *
 * or:
 *
 *     "require exactly X GB memory".
 *
 * Such details belong to resource/target/deployment/runtime semantics.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     - inference declarations;
 *     - models;
 *     - inputs;
 *     - outputs;
 *     - tensors;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - sequence length;
 *     - batch size;
 *     - generated outputs;
 *     - inference stages;
 *     - preprocessing stages;
 *     - postprocessing stages;
 *     - decoding stages;
 *     - execution workers;
 *     - devices;
 *     - accelerators;
 *     - nodes;
 *     - replicas;
 *     - streams.
 *
 * Repetition is structural.
 *
 * Practical limits belong to:
 *
 *     - compiler configuration;
 *     - resource availability;
 *     - scheduling;
 *     - target capabilities;
 *     - runtime policy;
 *     - deployment policy.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar reuses:
 *
 *     typeExpression
 *
 * from the canonical type grammar.
 *
 * inference.g4 MUST NOT define another type system.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions are owned by the canonical expression grammar.
 *
 * This grammar therefore reuses:
 *
 *     expression
 *
 * for:
 *
 *     - model references;
 *     - input expressions;
 *     - output expressions;
 *     - configuration;
 *     - decoding;
 *     - preprocessing;
 *     - postprocessing;
 *     - constraints;
 *     - resource requirements;
 *     - capabilities;
 *     - preferences;
 *     - execution conditions;
 *     - arbitrary future inference semantics.
 *
 * ============================================================================
 *
 * MODEL INTEGRATION
 * ============================================================================
 *
 * inference.g4 does NOT import models.g4.
 *
 * Model declarations are owned by:
 *
 *     grammar/ai/models.g4
 *
 * Inference consumes model VALUES/REFERENCES through expressions.
 *
 * This prevents:
 *
 *     inference -> models -> inference
 *
 * dependency cycles.
 *
 * The semantic resolver connects an inference model reference to its model
 * declaration.
 *
 * ============================================================================
 *
 * DATASET INTEGRATION
 * ============================================================================
 *
 * inference.g4 does NOT import datasets.g4.
 *
 * Dataset declarations are owned by:
 *
 *     grammar/ai/datasets.g4
 *
 * Inference may consume dataset-derived values through ordinary expressions.
 *
 * Dataset storage, streaming, sharding and placement remain outside this file.
 *
 * ============================================================================
 *
 * TENSOR INTEGRATION
 * ============================================================================
 *
 * inference.g4 does NOT define a competing tensor grammar.
 *
 * Tensor types and tensor expressions are represented through:
 *
 *     typeExpression
 *     expression
 *
 * Tensor semantics are resolved downstream.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An inference computation may consume or produce values originating from
 * quantum computation.
 *
 * This grammar does NOT import quantum grammar.
 *
 * The semantic layer may resolve expressions to:
 *
 *     classical values;
 *     quantum-derived values;
 *     hybrid values;
 *     hardware-backed values.
 *
 * If an inference operation contains quantum computation, that computation
 * remains owned by the quantum language/IR boundary.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * HARDWARE / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * This grammar may express:
 *
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *     @resource
 *
 * but does not select hardware.
 *
 * For example, the grammar permits semantic declarations equivalent to:
 *
 *     @requires tensor_compute;
 *     @capability low_latency;
 *     @preference energy_efficiency;
 *
 * without requiring:
 *
 *     GPU0
 *     TPU7
 *     NPU2
 *     device_17
 *
 * Hardware realization is owned downstream.
 *
 * ============================================================================
 *
 * SEMANTIC VALIDATION CONTRACT
 * ============================================================================
 *
 * The grammar MUST accept syntax.
 *
 * Semantic analysis MUST validate:
 *
 *     - annotation names;
 *     - declaration identity;
 *     - duplicate inference declarations;
 *     - model references;
 *     - input references;
 *     - output references;
 *     - type compatibility;
 *     - input/output compatibility;
 *     - configuration validity;
 *     - resource requirement validity;
 *     - capability compatibility;
 *     - constraint consistency;
 *     - unsupported execution combinations;
 *     - portability violations;
 *     - invalid inference modes.
 *
 * The parser must not attempt these checks.
 *
 * ============================================================================
 *
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic for the same:
 *
 *     source;
 *     grammar version;
 *     lexer version;
 *     parser configuration.
 *
 * Runtime stochasticity such as sampling is NOT a parsing concern.
 *
 * A semantic declaration such as:
 *
 *     @deterministic
 *
 * or:
 *
 *     @stochastic
 *
 * is interpreted downstream.
 *
 * ============================================================================
 */

parser grammar Inference;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Public parser boundary for inference-domain syntax.
 *
 * The semantic layer determines whether the annotation represents:
 *
 *     @inference
 *     @infer
 *     or another registered inference construct.
 */
inferenceConstruct
    : inferenceDeclaration
    | inferenceInvocation
    ;


/* ============================================================================
 * 2. INFERENCE DECLARATION
 * ========================================================================== */

/**
 * Canonical inference declaration.
 *
 * Conceptual form:
 *
 *     @inference RunModel {
 *         ...
 *     }
 *
 * The grammar intentionally does not require the annotation text to be a
 * particular hard-coded keyword.
 *
 * Semantic validation must establish that the annotation is registered as an
 * inference declaration annotation.
 */
inferenceDeclaration
    : NANO_ANNOTATION
      identifier
      inferenceTypeClause?
      inferenceInitializer?
      inferenceBody
    ;


/**
 * Optional declared result/type boundary.
 */
inferenceTypeClause
    : COLON
      typeExpression
    ;


/**
 * Optional declaration initializer.
 *
 * Example semantic form:
 *
 *     @inference Run = someInferenceExpression {
 *         ...
 *     }
 */
inferenceInitializer
    : ASSIGN
      expression
    ;


/**
 * Inference body.
 */
inferenceBody
    : LBRACE
      inferenceMember*
      RBRACE
    ;


/* ============================================================================
 * 3. INFERENCE MEMBERS
 * ========================================================================== */

/**
 * An inference body can contain inference-specific annotated clauses or
 * ordinary Zamani statements.
 *
 * This is important for hybrid programs because inference orchestration may
 * contain ordinary classical control flow.
 */
inferenceMember
    : inferenceClause
    | statement
    ;


/* ============================================================================
 * 4. GENERIC INFERENCE CLAUSE
 * ========================================================================== */

/**
 * Generic extensible inference clause.
 *
 * Supported semantic forms include, but are not limited to:
 *
 *     @model model;
 *     @input x: Tensor<Float>;
 *     @output y: Tensor<Float>;
 *     @preprocess normalize { ... }
 *     @postprocess decode { ... }
 *     @decode result = decoder(...);
 *     @resource requirement = ...;
 *     @requires capability;
 *     @constraint condition;
 *     @preference preference;
 *     @batch size;
 *     @stream source;
 *     @deterministic;
 *     @stochastic;
 *
 * The grammar intentionally does not enumerate every future inference concept.
 *
 * Annotation semantics are versioned outside the lexer.
 */
inferenceClause
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceTypeClause?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/**
 * Optional semantic target of a clause.
 *
 * Examples:
 *
 *     @model model
 *     @input features
 *     @output prediction
 *     @decoder decoder
 */
inferenceClauseTarget
    : identifier
    ;


/**
 * A clause either contains a nested body or terminates as a declaration /
 * expression-style statement.
 */
inferenceBodyOrTerminator
    : inferenceBody
    | SEMICOLON
    ;


/* ============================================================================
 * 5. INFERENCE INVOCATION
 * ========================================================================== */

/**
 * Invocation boundary.
 *
 * The actual invocation semantics remain ordinary Zamani expression semantics.
 *
 * Conceptual forms include:
 *
 *     @infer model(input);
 *
 *     @infer model(input, context);
 *
 *     @infer pipeline(input);
 *
 *     @infer expression;
 *
 * This allows future inference invocation forms without changing the lexer.
 */
inferenceInvocation
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 6. MODEL REFERENCE
 * ========================================================================== */

/**
 * Model references are expressions rather than grammar-level model objects.
 *
 * Semantic analysis resolves the expression to a model value.
 */
inferenceModelReference
    : expression
    ;


/* ============================================================================
 * 7. INPUT BOUNDARY
 * ========================================================================== */

/**
 * Input declaration/reference boundary.
 *
 * Inputs may originate from:
 *
 *     - scalar values;
 *     - vectors;
 *     - matrices;
 *     - tensors;
 *     - streams;
 *     - datasets;
 *     - classical computation;
 *     - quantum-derived measurements;
 *     - hardware interfaces;
 *     - distributed sources.
 *
 * The source type is determined semantically.
 */
inferenceInput
    : NANO_ANNOTATION
      identifier
      inferenceTypeClause?
      inferenceInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 8. OUTPUT BOUNDARY
 * ========================================================================== */

/**
 * Output declaration/reference boundary.
 */
inferenceOutput
    : NANO_ANNOTATION
      identifier
      inferenceTypeClause?
      inferenceInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 9. PREPROCESSING
 * ========================================================================== */

/**
 * Preprocessing is intentionally represented as an expression or nested
 * semantic region.
 *
 * It may include:
 *
 *     normalization;
 *     tokenization;
 *     resizing;
 *     feature construction;
 *     encoding;
 *     filtering;
 *     classical computation;
 *     hardware preprocessing;
 *     future transformations.
 */
inferencePreprocess
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 10. POSTPROCESSING
 * ========================================================================== */

/**
 * Postprocessing may include:
 *
 *     decoding;
 *     normalization;
 *     projection;
 *     filtering;
 *     aggregation;
 *     conversion;
 *     classical computation;
 *     quantum-derived result processing.
 */
inferencePostprocess
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 11. DECODING
 * ========================================================================== */

/**
 * Decoding semantics are implementation-independent.
 *
 * The grammar does not hard-code:
 *
 *     greedy decoding;
 *     beam width;
 *     sampling count;
 *     temperature;
 *     top-k;
 *     top-p;
 *     any particular algorithm.
 *
 * Those can be represented as ordinary expressions/configuration values.
 */
inferenceDecoding
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 12. EXECUTION MODE
 * ========================================================================== */

/**
 * Execution-mode declarations are intentionally annotation-based.
 *
 * Possible semantic modes include:
 *
 *     deterministic;
 *     stochastic;
 *     streaming;
 *     batched;
 *     interactive;
 *     online;
 *     offline;
 *     speculative;
 *     adaptive;
 *     distributed;
 *     asynchronous;
 *     synchronous;
 *     quantum-assisted;
 *     hybrid.
 *
 * The semantic registry owns the actual mode vocabulary.
 */
inferenceExecutionMode
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 13. RESOURCE REQUIREMENTS
 * ========================================================================== */

/**
 * Resource requirements are declarations, not allocations.
 *
 * Examples of semantic categories include:
 *
 *     compute;
 *     memory;
 *     storage;
 *     bandwidth;
 *     latency;
 *     throughput;
 *     energy;
 *     reliability;
 *     accelerator capability.
 *
 * No physical capacity is encoded here.
 */
inferenceResourceRequirement
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceTypeClause?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 14. CAPABILITY REQUIREMENTS
 * ========================================================================== */

/**
 * Capability requirements describe WHAT the execution environment must
 * support, without naming a concrete device.
 */
inferenceCapabilityRequirement
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 15. CONSTRAINTS
 * ========================================================================== */

/**
 * Constraints describe semantic restrictions.
 *
 * Examples:
 *
 *     quality;
 *     latency;
 *     precision;
 *     numerical stability;
 *     privacy;
 *     reliability;
 *     portability;
 *     determinism.
 *
 * Constraint interpretation belongs to semantic/resource analysis.
 */
inferenceConstraint
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 16. PREFERENCES
 * ========================================================================== */

/**
 * Preferences are non-mandatory optimization hints.
 *
 * A preference MUST NOT be interpreted as a hard requirement unless semantic
 * analysis explicitly says so.
 */
inferencePreference
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 17. QUALITY / ACCEPTANCE CONTRACT
 * ========================================================================== */

/**
 * Quality declarations can express semantic acceptance conditions.
 *
 * The actual metric and threshold types are expressions.
 *
 * No fixed metric or fixed threshold is encoded into the grammar.
 */
inferenceQuality
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 18. FAILURE / FALLBACK CONTRACT
 * ========================================================================== */

/**
 * Inference may describe fallback intent.
 *
 * Actual resilience decisions remain owned by the resilience subsystem.
 *
 * This grammar merely provides a syntactic boundary for semantic declarations.
 */
inferenceFallback
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 19. STATE / CONTEXT
 * ========================================================================== */

/**
 * Inference context can represent semantic state required by an inference
 * computation.
 *
 * It does not imply serialization of arbitrary machine or quantum state.
 */
inferenceContext
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceTypeClause?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 20. BATCHING / STREAMING
 * ========================================================================== */

/**
 * Batch and stream configuration remain expressions.
 *
 * This deliberately permits:
 *
 *     static batching;
 *     dynamic batching;
 *     adaptive batching;
 *     streaming;
 *     micro-batching;
 *     provider-specific future strategies.
 *
 * No finite batch-size limit is encoded.
 */
inferenceBatching
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


inferenceStreaming
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 21. PRECISION / NUMERICAL POLICY
 * ========================================================================== */

/**
 * Numerical policy may describe semantic precision intent.
 *
 * The grammar does not enumerate hardware formats.
 *
 * Semantic analysis may resolve:
 *
 *     precision;
 *     numerical stability;
 *     mixed precision;
 *     exactness;
 *     approximation;
 *     quantization intent.
 */
inferenceNumericalPolicy
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceTypeClause?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 22. DETERMINISM / STOCHASTICITY
 * ========================================================================== */

/**
 * Determinism and stochasticity are semantic properties.
 */
inferenceReproducibility
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 23. DISTRIBUTED INFERENCE
 * ========================================================================== */

/**
 * Distributed inference is expressed without hard-coding:
 *
 *     node count;
 *     worker count;
 *     replica count;
 *     topology;
 *     device IDs;
 *     network addresses.
 *
 * Placement and realization belong downstream.
 */
inferenceDistributed
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 24. ADAPTIVE / DYNAMIC INFERENCE
 * ========================================================================== */

/**
 * Supports semantic declarations for inference that may adapt according to:
 *
 *     input;
 *     observed quality;
 *     resource availability;
 *     runtime capabilities;
 *     model state;
 *     execution context.
 *
 * Actual adaptation belongs to the semantic/runtime layers.
 */
inferenceAdaptive
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 25. INTEROPERABILITY
 * ========================================================================== */

/**
 * Inference may interoperate with:
 *
 *     classical computation;
 *     quantum computation;
 *     HDL/hardware;
 *     accelerators;
 *     distributed services;
 *     external systems.
 *
 * This rule provides the syntactic boundary only.
 */
inferenceInteroperability
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;


/* ============================================================================
 * 26. EXTENSION / DIALECT BOUNDARY
 * ========================================================================== */

/**
 * Future inference dialects may introduce new annotations without modifying
 * this grammar.
 *
 * Dialect validation belongs to the dialect/semantic subsystem.
 */
inferenceExtension
    : NANO_ANNOTATION
      inferenceClauseTarget?
      inferenceTypeClause?
      inferenceInitializer?
      inferenceBodyOrTerminator
    ;