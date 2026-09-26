/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/inference.g4
 *
 * Grammar:
 *     Inference
 *
 * Status:
 *     CANONICAL AI INFERENCE LEAF GRAMMAR
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of AI/ML inference intent.
 *
 * It describes:
 *
 *     - inference declarations;
 *     - inference invocations;
 *     - model references;
 *     - inference inputs;
 *     - inference outputs;
 *     - preprocessing;
 *     - postprocessing;
 *     - decoding;
 *     - inference configuration;
 *     - inference execution policy;
 *     - resource requirements;
 *     - capability requirements;
 *     - constraints;
 *     - preferences;
 *     - inference-local regions;
 *     - ordinary Zamani statements inside inference regions.
 *
 * It does NOT implement:
 *
 *     - model execution;
 *     - model storage;
 *     - tensor storage;
 *     - training;
 *     - optimization algorithms;
 *     - automatic differentiation;
 *     - scheduling;
 *     - placement;
 *     - accelerator selection;
 *     - hardware discovery;
 *     - deployment;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
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
 *     AI composition
 *          |
 *          v
 *     Inference
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     AI semantic model          resource/capability model
 *          |                              |
 *          +--------------+---------------+
 *                         |
 *                         v
 *                canonical semantic IR
 *                         |
 *                 optimization
 *                         |
 *              routing/scheduling
 *                         |
 *                 target lowering
 *                         |
 *                      runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     inferenceConstruct
 *     inferenceDeclaration
 *     inferenceInvocation
 *     inferenceBody
 *     inferenceMember
 *     inferenceDirective
 *     inference resource/capability/constraint/preference boundaries
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     paths
 *     literals
 *     general expressions
 *     general types
 *     statements
 *     models
 *     datasets
 *     tensors
 *     training
 *     agents
 *     hardware
 *     networking
 *     distributed placement
 *     deployment
 *     IR
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns:
 *
 *     AT : '@'
 *
 * This grammar therefore MUST NOT use NANO_ANNOTATION.
 *
 * Annotation names remain composed from:
 *
 *     AT
 *     identifier
 *
 * Reserved language keywords used for resource contracts are consumed through
 * their canonical lexer tokens:
 *
 *     REQUIRES
 *     CAPABILITY
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *
 * No new lexer token is introduced here.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * General type syntax is owned by:
 *
 *     Types
 *
 * This file consumes:
 *
 *     typeExpression
 *
 * It MUST NOT define an inference-specific competing type system.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     Expressions
 *
 * This file consumes:
 *
 *     expression
 *     argumentList
 *
 * Model references, dataset references, tensor expressions, quantum-derived
 * values and hardware-backed values therefore remain ordinary semantic values.
 *
 * This prevents dependency cycles such as:
 *
 *     Inference -> Models -> Inference
 *     Inference -> Datasets -> Inference
 *
 * ============================================================================
 * STATEMENT CONTRACT
 * ============================================================================
 *
 * Inference regions may contain ordinary Zamani statements.
 *
 * The canonical statement composition grammar is therefore imported.
 *
 * This permits:
 *
 *     inference
 *         -> classical computation
 *         -> quantum computation
 *         -> measurement
 *         -> classical decision
 *         -> inference
 *
 * without introducing an AI-specific statement language.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar imposes NO universal limit on:
 *
 *     models
 *     inputs
 *     outputs
 *     tensors
 *     tensor rank
 *     tensor dimensions
 *     batch size
 *     sequence length
 *     inference calls
 *     inference stages
 *     workers
 *     devices
 *     accelerators
 *     nodes
 *     replicas
 *     streams
 *     generated values
 *
 * There are no:
 *
 *     MAX_MODELS
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_BATCH_SIZE
 *     MAX_SEQUENCE_LENGTH
 *     MAX_TENSOR_RANK
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_WORKERS
 *     MAX_NODES
 *
 * or equivalent grammar-level ceilings.
 *
 * Any practical limitation belongs to compiler, runtime, target capability,
 * resource availability, deployment policy or operating environment.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * The source language distinguishes:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *
 * A requirement such as:
 *
 *     requires capability("tensor.compute")
 *
 * does not select a physical accelerator.
 *
 * The grammar MUST NOT encode:
 *
 *     GPU 0
 *     CPU 3
 *     device 17
 *     QPU 2
 *
 * as universal inference semantics.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Inference syntax is target-independent.
 *
 * The same inference source may ultimately be lowered to:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     NPU
 *     TPU
 *     QPU-assisted execution
 *     distributed systems
 *     heterogeneous systems
 *     future execution targets
 *
 * without changing the source grammar merely because the hardware changed.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Inference may consume quantum-derived values through ordinary expressions.
 *
 * This grammar does NOT define:
 *
 *     qubits
 *     gates
 *     physical qubits
 *     quantum topology
 *     routing
 *     QEC
 *     ZQN
 *
 * Quantum semantics remain owned by the quantum subsystem and ultimately:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Inference may require capabilities provided by hardware or HDL realizations.
 *
 * It does not define:
 *
 *     register widths
 *     bus widths
 *     memory-bank counts
 *     FPGA resource counts
 *     accelerator counts
 *     physical wiring
 *     clock topology
 *
 * Those belong to:
 *
 *     hardware/
 *     hdl/
 *     resources/
 *     compile/
 *     execution/
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every inference declaration MUST preserve:
 *
 *     - source span;
 *     - annotation/name;
 *     - declaration name;
 *     - declared type, if present;
 *     - initializer, if present;
 *     - ordered members;
 *     - directive names;
 *     - directive targets;
 *     - directive arguments;
 *     - nested regions;
 *     - expressions;
 *     - statements;
 *     - source provenance.
 *
 * The AST MUST remain domain-neutral.
 *
 * This grammar does NOT require a parallel inference-specific IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - validating @inference / @infer;
 *     - resolving model references;
 *     - resolving dataset references;
 *     - resolving tensor types;
 *     - checking input/output compatibility;
 *     - checking inference configuration;
 *     - checking duplicate names;
 *     - checking directive applicability;
 *     - validating requirements;
 *     - validating capabilities;
 *     - validating constraints;
 *     - distinguishing preferences from requirements;
 *     - checking portability;
 *     - checking determinism;
 *     - checking stochastic execution policy;
 *     - checking resource availability;
 *     - checking target capability compatibility.
 *
 * The parser performs none of those semantic checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Semantic inference operations must map into the repository's canonical
 * semantic representation.
 *
 * If inference interacts with quantum computation:
 *
 *     inference syntax
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * There MUST NOT be an AI-specific second quantum IR.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a fixed:
 *
 *     source
 *     lexer
 *     grammar
 *     parser configuration
 *
 * Runtime sampling, stochastic inference, randomness and model nondeterminism
 * are semantic/runtime concerns and MUST NOT affect parsing.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no target-specific actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no hardware discovery;
 *     - no unsafe implementation.
 *
 * Generated/consuming Rust code must remain compatible with:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 */

parser grammar Inference;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Complete inference-domain construct.
 *
 * The two public forms are deliberately structurally distinct:
 *
 *     @inference Name { ... }
 *
 * and:
 *
 *     @infer Model(args...);
 *
 * This avoids treating arbitrary expressions as inference constructs.
 */
inferenceConstruct
    : inferenceDeclaration
    | inferenceInvocation
    ;


/* ============================================================================
 * 2. DECLARATION
 * ========================================================================== */

/**
 * Canonical inference declaration.
 *
 * Examples:
 *
 *     @inference classify {
 *         ...
 *     }
 *
 *     @inference classify: Result {
 *         ...
 *     }
 *
 *     @inference classify = model {
 *         ...
 *     }
 *
 * The semantic layer validates the annotation spelling.
 */
inferenceDeclaration
    : AT
      inferenceDeclarationAnnotation
      identifier
      inferenceDeclarationType?
      inferenceInitializer?
      inferenceBody
    ;


inferenceDeclarationAnnotation
    : identifier
    ;


inferenceDeclarationType
    : COLON
      typeExpression
    ;


inferenceInitializer
    : ASSIGN
      expression
    ;


inferenceBody
    : LBRACE
      inferenceMember*
      RBRACE
    ;


/* ============================================================================
 * 3. INVOCATION
 * ========================================================================== */

/**
 * Inference invocation.
 *
 * Examples:
 *
 *     @infer(model, input);
 *
 *     @infer classifier(input, context);
 *
 *     @infer pipeline(input);
 *
 * The invocation target and arguments are expressions/semantic references.
 */
inferenceInvocation
    : AT
      inferenceInvocationAnnotation
      LPAREN
      argumentList?
      RPAREN
      SEMI?
    ;


inferenceInvocationAnnotation
    : identifier
    ;


/* ============================================================================
 * 4. INFERENCE MEMBERS
 * ========================================================================== */

/**
 * Inference bodies may contain:
 *
 *     inference directives;
 *     resource contracts;
 *     ordinary Zamani statements.
 *
 * This is the primary AI/classical/hybrid integration boundary.
 */
inferenceMember
    : inferenceDirective
    | inferenceResourceContract
    | inferenceCapabilityContract
    | inferenceConstraintContract
    | inferencePreferenceContract
    | inferenceHintContract
    | statement
    ;


/* ============================================================================
 * 5. GENERIC INFERENCE DIRECTIVE
 * ========================================================================== */

/**
 * Extensible inference-local directive.
 *
 * Examples:
 *
 *     @model classifier;
 *     @input features: Tensor<Float>;
 *     @output prediction: Tensor<Float>;
 *     @preprocess normalize;
 *     @postprocess decode;
 *     @decode decoder;
 *     @batch batch_size;
 *     @stream input;
 *     @context context;
 *     @configuration config;
 *
 * The directive name is intentionally not converted into a lexer keyword.
 *
 * Semantic analysis owns the registered directive vocabulary.
 */
inferenceDirective
    : AT
      inferenceDirectiveName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceDirectiveName
    : identifier
    ;


inferenceDirectivePayload
    : inferenceDirectiveCall
    | inferenceDirectiveBinding
    | inferenceDirectiveTypedBinding
    | inferenceDirectiveAssignment
    | inferenceDirectiveTarget
    | inferenceDirectiveRegion
    ;


inferenceDirectiveCall
    : LPAREN
      argumentList?
      RPAREN
    ;


inferenceDirectiveBinding
    : identifier
      inferenceDirectiveCall?
    ;


inferenceDirectiveTypedBinding
    : identifier
      COLON
      typeExpression
      inferenceDirectiveCall?
    ;


inferenceDirectiveAssignment
    : identifier?
      ASSIGN
      expression
    ;


inferenceDirectiveTarget
    : expression
    ;


inferenceDirectiveRegion
    : LBRACE
      inferenceMember*
      RBRACE
    ;


/* ============================================================================
 * 6. RESOURCE CONTRACT
 * ========================================================================== */

/**
 * Hard resource requirement.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *     requires throughput >= required_throughput;
 *     requires latency <= latency_budget;
 *
 * The expression is semantic data.
 *
 * No physical resource is selected here.
 */
inferenceResourceContract
    : REQUIRES
      expression
      SEMI?
    ;


/* ============================================================================
 * 7. CAPABILITY CONTRACT
 * ========================================================================== */

/**
 * Capability requirement.
 *
 * Examples:
 *
 *     capability("tensor.compute");
 *     capability("streaming");
 *     capability("quantum.measurement");
 *
 * Capability satisfaction is determined downstream.
 */
inferenceCapabilityContract
    : CAPABILITY
      expression
      SEMI?
    ;


/* ============================================================================
 * 8. CONSTRAINT CONTRACT
 * ========================================================================== */

/**
 * Hard semantic constraint.
 */
inferenceConstraintContract
    : CONSTRAINT
      expression
      SEMI?
    ;


/* ============================================================================
 * 9. PREFERENCE CONTRACT
 * ========================================================================== */

/**
 * Non-binding optimization preference.
 *
 * A preference MUST NOT silently become a hard requirement.
 */
inferencePreferenceContract
    : PREFER
      expression
      SEMI?
    ;


/* ============================================================================
 * 10. HINT CONTRACT
 * ========================================================================== */

/**
 * Non-semantic implementation hint.
 *
 * Hints may influence optimization or lowering but cannot redefine program
 * meaning.
 */
inferenceHintContract
    : HINT
      expression
      SEMI?
    ;


/* ============================================================================
 * 11. COMMON EXPLICIT INFERENCE ROLES
 * ========================================================================== */

/**
 * These rules provide stable semantic anchors for tooling and AST mapping.
 *
 * They intentionally reuse the generic directive representation.
 *
 * A future inference dialect can extend the directive registry without
 * changing the lexer.
 */

inferenceModelDirective
    : AT
      inferenceModelName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceModelName
    : identifier
    ;


inferenceInputDirective
    : AT
      inferenceInputName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceInputName
    : identifier
    ;


inferenceOutputDirective
    : AT
      inferenceOutputName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceOutputName
    : identifier
    ;


/* ============================================================================
 * 12. PREPROCESSING
 * ========================================================================== */

/**
 * Preprocessing is an inference directive rather than a fixed algorithm list.
 *
 * This allows:
 *
 *     normalize
 *     tokenize
 *     encode
 *     resize
 *     transform
 *     custom preprocessing
 *
 * without making every operation a language keyword.
 */
inferencePreprocessDirective
    : AT
      inferencePreprocessName
      inferenceDirectivePayload?
      SEMI?
    ;


inferencePreprocessName
    : identifier
    ;


/* ============================================================================
 * 13. POSTPROCESSING
 * ========================================================================== */

inferencePostprocessDirective
    : AT
      inferencePostprocessName
      inferenceDirectivePayload?
      SEMI?
    ;


inferencePostprocessName
    : identifier
    ;


/* ============================================================================
 * 14. DECODING
 * ========================================================================== */

inferenceDecodeDirective
    : AT
      inferenceDecodeName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceDecodeName
    : identifier
    ;


/* ============================================================================
 * 15. EXECUTION POLICY
 * ========================================================================== */

/**
 * Execution policies remain semantic directives.
 *
 * Possible registered names include:
 *
 *     deterministic
 *     stochastic
 *     streaming
 *     batched
 *     adaptive
 *     online
 *     offline
 *     synchronous
 *     asynchronous
 *
 * No finite enumeration is embedded in the grammar.
 */
inferenceExecutionDirective
    : AT
      inferenceExecutionName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceExecutionName
    : identifier
    ;


/* ============================================================================
 * 16. NUMERICAL POLICY
 * ========================================================================== */

inferenceNumericalDirective
    : AT
      inferenceNumericalName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceNumericalName
    : identifier
    ;


/* ============================================================================
 * 17. QUALITY / ACCEPTANCE
 * ========================================================================== */

inferenceQualityDirective
    : AT
      inferenceQualityName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceQualityName
    : identifier
    ;


/* ============================================================================
 * 18. CONTEXT / STATE
 * ========================================================================== */

inferenceContextDirective
    : AT
      inferenceContextName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceContextName
    : identifier
    ;


/* ============================================================================
 * 19. DISTRIBUTED / ADAPTIVE EXECUTION
 * ========================================================================== */

inferenceDistributedDirective
    : AT
      inferenceDistributedName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceDistributedName
    : identifier
    ;


inferenceAdaptiveDirective
    : AT
      inferenceAdaptiveName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceAdaptiveName
    : identifier
    ;


/* ============================================================================
 * 20. INTEROPERABILITY
 * ========================================================================== */

/**
 * Interoperability is expression-based.
 *
 * AI inference may consume:
 *
 *     classical values;
 *     dataset values;
 *     tensor values;
 *     quantum-derived values;
 *     hardware-backed values;
 *     distributed values.
 *
 * No domain-specific IR is introduced here.
 */
inferenceInteroperabilityDirective
    : AT
      inferenceInteroperabilityName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceInteroperabilityName
    : identifier
    ;


/* ============================================================================
 * 21. EXTENSION / DIALECT BOUNDARY
 * ========================================================================== */

/**
 * Future AI dialects may register additional directives.
 *
 * The parser preserves their structure.
 *
 * Semantic/dialect validation determines whether a directive is legal.
 */
inferenceExtensionDirective
    : AT
      inferenceExtensionName
      inferenceDirectivePayload?
      SEMI?
    ;


inferenceExtensionName
    : identifier
    ;