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
 *     CANONICAL PRODUCTION AI-INFERENCE LEAF GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *     Safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the SOURCE-LEVEL STRUCTURE of portable computational
 * inference intent.
 *
 * Inference is treated as a general computation capability rather than as
 * a vocabulary of particular machine-learning algorithms.
 *
 * This grammar supports:
 *
 *     - inference declarations;
 *     - inference invocations;
 *     - model references;
 *     - input/output bindings;
 *     - typed bindings;
 *     - inference-local directives;
 *     - preprocessing intent;
 *     - postprocessing intent;
 *     - decoding intent;
 *     - execution intent;
 *     - numerical intent;
 *     - quality/acceptance intent;
 *     - context/state intent;
 *     - distributed/adaptive intent;
 *     - interoperability intent;
 *     - extensibility;
 *     - ordinary Zamani statements;
 *     - canonical resource statements;
 *     - classical/quantum/hybrid composition through ordinary expressions
 *       and statements.
 *
 * The grammar expresses COMPUTATIONAL INTENT.
 *
 * It does not prescribe a particular:
 *
 *     model format
 *     framework
 *     optimizer
 *     accelerator
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     runtime
 *     deployment system
 *     storage engine
 *     network
 *     vendor
 *     hardware topology
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexical system
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     AI
 *          |
 *          v
 *     Inference                         <-- THIS FILE
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------------------+
 *          |               |               |
 *          v               v               v
 *        types          effects        provenance
 *          |               |               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                  semantic inference model
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *         resources    capabilities    policies
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                 canonical semantic model
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        classical     quantum::ir   HDL/hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                     optimization
 *                          |
 *                    specialization
 *                          |
 *                       lowering
 *                          |
 *                 routing/scheduling
 *                          |
 *                 resilience/recovery
 *                          |
 *                    ZQN where needed
 *                          |
 *                    HAL where needed
 *                          |
 *                  target realization
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns exactly these parser-level concepts:
 *
 *     inferenceConstruct
 *     inferenceDeclaration
 *     inferenceInvocation
 *     inferenceBody
 *     inferenceMember
 *     inferenceDirective
 *     inferenceDirectivePayload
 *     inferenceBinding
 *     inferenceTypedBinding
 *     inferenceNamedBinding
 *     inferenceRegion
 *     inferenceExtension
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     keyword spelling
 *     identifiers
 *     qualified names
 *     general expressions
 *     expression precedence
 *     argument-list syntax
 *     general types
 *     generic types
 *     statements
 *     assertions
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     effects
 *     policies
 *     provenance
 *     contracts
 *     models
 *     datasets
 *     tensors
 *     training
 *     agents
 *     probabilistic computation
 *     knowledge
 *     reasoning
 *     adaptation semantics
 *     quantum operations
 *     HDL syntax
 *     hardware realization
 *     resource allocation
 *     scheduling
 *     routing
 *     optimization
 *     IR
 *     runtime execution
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General language constructs remain owned by their existing canonical
 * grammars.
 *
 * In particular:
 *
 *     lexical vocabulary
 *         -> grammar/lexer/
 *
 *     canonical executable lexer
 *         -> grammar/antlr/ZamaniLexer.g4
 *
 *     names
 *         -> grammar/core/names.g4
 *
 *     types
 *         -> grammar/types/types.g4
 *
 *     expressions
 *         -> grammar/expressions/expressions.g4
 *
 *     statements
 *         -> grammar/statements/statements.g4
 *
 *     resource statements
 *         -> grammar/statements/resource.g4
 *         -> grammar/resources/
 *
 *     models
 *         -> grammar/ai/models.g4
 *
 *     datasets
 *         -> grammar/ai/datasets.g4
 *
 *     tensors
 *         -> grammar/ai/tensors.g4
 *
 *     training
 *         -> grammar/ai/training.g4
 *
 *     agents
 *         -> grammar/ai/agents.g4
 *
 *     probabilistic computation
 *         -> grammar/ai/probabilistic.g4
 *
 *     quantum computation
 *         -> grammar/quantum/
 *
 *     hybrid computation
 *         -> grammar/hybrid/
 *
 *     hardware
 *         -> grammar/hardware/
 *
 *     HDL
 *         -> grammar/hdl/
 *
 * This file MUST NOT redefine any of those authorities.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     ZamaniLexer
 *     Types
 *     Expressions
 *     Statements
 *
 *
 * EXPORTS
 * -------
 *
 *     inferenceConstruct
 *     inferenceDeclaration
 *     inferenceInvocation
 *
 * Secondary reusable parser rules are intentionally kept local to this grammar.
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/ai/ai.g4
 *
 * through:
 *
 *     inferenceConstruct
 *
 *
 * AST_OWNER
 * ---------
 *
 * Existing domain-neutral frontend AST.
 *
 * The AST must preserve:
 *
 *     source span
 *     declaration/invocation form
 *     inference identity
 *     ordered members
 *     directive identity
 *     directive payload
 *     binding names
 *     declared types
 *     expressions
 *     nested regions
 *     statement structure
 *     source provenance
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * AI semantic analysis together with:
 *
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     provenance
 *     portability analysis
 *
 *
 * IR_OWNER
 * --------
 *
 * This grammar owns NO IR.
 *
 * Inference semantics lower into the repository's canonical semantic model.
 *
 * Classical computation may lower to the canonical classical representation.
 *
 * Quantum computation MUST cross:
 *
 *     quantum::ir
 *
 * rather than creating an inference-specific quantum representation.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/ai/inference/
 *
 * Recommended test groups:
 *
 *     declarations/
 *     invocation/
 *     bindings/
 *     directives/
 *     resources/
 *     interoperability/
 *     quantum/
 *     hybrid/
 *     scalability/
 *     negative/
 *     compatibility/
 *     determinism/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/ai.md
 *     grammar/specification/
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * The canonical lexical authority ultimately composes:
 *
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/operators.g4
 *     grammar/lexer/punctuation.g4
 *     grammar/lexer/identifiers.g4
 *     grammar/lexer/literals.g4
 *     ...
 *
 * This grammar defines NO lexer rules.
 *
 *
 * ============================================================================
 * IMPORTANT LEXICAL CORRECTION
 * ============================================================================
 *
 * The canonical Zamani vocabulary already provides:
 *
 *     INFER
 *     AT
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *
 * Therefore this grammar MUST use those canonical token names.
 *
 * In particular:
 *
 *     SEMICOLON
 *
 * is canonical.
 *
 * A legacy:
 *
 *     SEMI
 *
 * token MUST NOT be referenced here.
 *
 *
 * ============================================================================
 * INFERENCE ENTRY-POINT POLICY
 * ============================================================================
 *
 * The reserved:
 *
 *     INFER
 *
 * keyword is the unambiguous inference operation entry point.
 *
 * Canonical supported forms are:
 *
 *     infer Model(input);
 *
 *     infer Model(input, context);
 *
 *     infer result from source;
 *
 * and annotation-compatible forms:
 *
 *     @infer Model(input);
 *
 *     @infer Model(input, context);
 *
 *     @infer name {
 *         ...
 *     }
 *
 * The declaration form deliberately uses:
 *
 *     INFER
 *
 * rather than an arbitrary:
 *
 *     @identifier name { ... }
 *
 * because a generic annotation-based declaration would collide structurally
 * with:
 *
 *     @model
 *     @agent
 *     @pipeline
 *     @training
 *     @dataset
 *     @deployment
 *
 * and other AI constructs.
 *
 * This is a critical parser-disambiguation rule.
 *
 *
 * ============================================================================
 * ANNOTATION POLICY
 * ============================================================================
 *
 * Generic inference directives use:
 *
 *     AT IDENTIFIER
 *
 * rather than one lexer keyword per directive.
 *
 * Therefore names such as:
 *
 *     @model
 *     @input
 *     @output
 *     @preprocess
 *     @postprocess
 *     @decode
 *     @context
 *     @configuration
 *     @deterministic
 *     @stream
 *     @batch
 *     @quality
 *     @fallback
 *     @custom_extension
 *
 * remain open-ended source names.
 *
 * Semantic validation determines whether a particular directive is registered,
 * valid, deprecated, experimental, or dialect-specific.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax belongs to:
 *
 *     Types
 *
 * This grammar consumes:
 *
 *     typeExpression
 *
 * It MUST NOT define:
 *
 *     InferenceType
 *     ModelType
 *     InputType
 *     OutputType
 *     AIType
 *
 * as competing type systems.
 *
 * An inference input can therefore have any valid Zamani type, including
 * future domain types, tensor types, classical types, quantum-derived value
 * types, hardware-independent data types, and user-defined types.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Expression syntax belongs to:
 *
 *     Expressions
 *
 * This grammar consumes:
 *
 *     expression
 *     argumentList
 *
 * An expression may represent:
 *
 *     a model
 *     a tensor
 *     a dataset
 *     a scalar
 *     a collection
 *     a stream
 *     a quantum-derived measurement
 *     a classical value
 *     an HDL-derived value
 *     a hardware-independent resource quantity
 *     a distributed value
 *     an AI result
 *     a future domain value
 *
 * No inference-specific expression hierarchy is introduced.
 *
 *
 * ============================================================================
 * STATEMENT CONTRACT
 * ============================================================================
 *
 * Ordinary statements remain owned by:
 *
 *     Statements
 *
 * Inference bodies may therefore contain ordinary Zamani computation.
 *
 * This is essential for hybrid execution:
 *
 *     inference
 *         ->
 *     classical computation
 *         ->
 *     quantum computation
 *         ->
 *     measurement
 *         ->
 *     classical control
 *         ->
 *     inference
 *
 * without creating an AI-only programming language.
 *
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource syntax is NOT duplicated here.
 *
 * Inference bodies consume the canonical:
 *
 *     statement
 *
 * production.
 *
 * Therefore forms such as:
 *
 *     requires capability("tensor.compute");
 *     requires memory >= required_memory;
 *     requires qubits >= required_qubits;
 *     requires topology(required_topology);
 *     capability tensor::compute;
 *     constraint latency <= latency_goal;
 *     prefer throughput >= desired_throughput;
 *     hint execution::low_latency;
 *
 * are parsed through the existing statement/resource architecture.
 *
 * This prevents inference.g4 from becoming a second resource grammar.
 *
 * A change to:
 *
 *     grammar/resources/
 *
 * therefore does not require duplicating the changed resource syntax here.
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Inference itself does not define an effect grammar.
 *
 * Semantic analysis determines effects such as:
 *
 *     randomness
 *     learning
 *     adaptation
 *     model_access
 *     external_data
 *     network
 *     foreign
 *     measurement
 *     distributed
 *
 * where applicable.
 *
 * The inference grammar merely preserves the source structure required for
 * those semantic analyses.
 *
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability declarations are not physically resolved here.
 *
 * A program may express:
 *
 *     requires capability("tensor.compute");
 *
 * or:
 *
 *     requires quantum::measurement;
 *
 * through the canonical resource/requirement grammar.
 *
 * Capability satisfaction occurs after parsing.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * This file does NOT define:
 *
 *     qubit
 *     gate
 *     physical qubit
 *     topology
 *     routing
 *     calibration
 *     error correction
 *     ZQN
 *
 * Quantum-derived values may simply appear as ordinary expressions:
 *
 *     infer classifier(measurement);
 *
 *     infer classifier(quantum_result);
 *
 *     infer decision(context = measured_state);
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * and eventually:
 *
 *     quantum::ir
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Inference can consume values produced by:
 *
 *     HDL
 *     hardware
 *     accelerators
 *     embedded computation
 *     distributed computation
 *
 * through ordinary expressions and generic capabilities/resources.
 *
 * This file does NOT encode:
 *
 *     register widths
 *     bus widths
 *     physical addresses
 *     FPGA capacity
 *     accelerator count
 *     CPU count
 *     GPU count
 *     memory capacity
 *     node count
 *     topology size
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO universal finite capacity limits.
 *
 * It does not define limits for:
 *
 *     inference declarations
 *     inference calls
 *     inputs
 *     outputs
 *     arguments
 *     model parameters
 *     tensor rank
 *     tensor dimensions
 *     batch size
 *     sequence length
 *     pipeline depth
 *     workers
 *     devices
 *     accelerators
 *     nodes
 *     qubits
 *     memory
 *     network size
 *
 * Repetition is represented by:
 *
 *     *
 *     +
 *     ?
 *
 * rather than fixed cardinality.
 *
 * "Scale to infinity given available resources" means that this grammar
 * introduces no artificial finite machine-size ceiling.
 *
 * Physical feasibility remains determined by:
 *
 *     available resources
 *     target capabilities
 *     policies
 *     compiler implementation
 *     runtime
 *     operating environment
 *
 *
 * ============================================================================
 * OPEN-WORLD MODEL
 * ============================================================================
 *
 * Model names, input names, output names, preprocessing operations,
 * postprocessing operations, decoding operations, execution policies,
 * numerical policies, quality policies, context identifiers, distributed
 * policies, and extension names remain open-world identifiers.
 *
 * This grammar MUST NOT enumerate:
 *
 *     neural network architectures
 *     ML algorithms
 *     model vendors
 *     model formats
 *     activation functions
 *     optimizers
 *     datasets
 *     computer-vision operations
 *     natural-language operations
 *     robotics operations
 *     sentiment categories
 *     application domains
 *
 * Such concepts belong to:
 *
 *     libraries
 *     models
 *     dialects
 *     semantic registries
 *     capabilities
 *     runtime providers
 *
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
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     resource availability
 *     network state
 *     filesystem state
 *     runtime state
 *     scheduler state
 *     randomness
 *     model execution
 *
 * Stochastic inference is a semantic/runtime concern, not a parser concern.
 *
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no target-specific actions;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no runtime execution.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and use safe Rust.
 *
 *
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 */

/**
 * Complete inference-domain construct.
 *
 * Declaration:
 *
 *     infer classifier {
 *         ...
 *     }
 *
 * Invocation:
 *
 *     infer classifier(input);
 *
 * Annotation-compatible invocation:
 *
 *     @infer classifier(input);
 *
 * Annotation-compatible declaration:
 *
 *     @infer classifier {
 *         ...
 *     }
 *
 * The `@infer` form is supported because `INFER` is already a canonical
 * lexical keyword and therefore remains unambiguous.
 */
inferenceConstruct
    : inferenceDeclaration
    | inferenceInvocation
    ;


/*
 * ============================================================================
 * 2. DECLARATION
 * ============================================================================
 */

/**
 * Inference declaration.
 *
 * Supported forms:
 *
 *     infer classifier {
 *     }
 *
 *     infer classifier: Result {
 *     }
 *
 *     infer classifier = model {
 *     }
 *
 *     @infer classifier {
 *     }
 *
 *     @infer classifier: Result {
 *     }
 *
 *     @infer classifier = model {
 *     }
 *
 * No fixed input/output count is imposed.
 */
inferenceDeclaration
    : inferenceDeclarationIntroducer
      identifier
      inferenceDeclarationType?
      inferenceInitializer?
      inferenceBody
    ;


inferenceDeclarationIntroducer
    : INFER
    | AT INFER
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


/*
 * ============================================================================
 * 3. INVOCATION
 * ============================================================================
 */

/**
 * Invocation forms:
 *
 *     infer classifier(input);
 *
 *     infer classifier(input, context);
 *
 *     @infer classifier(input);
 *
 *     infer model;
 *
 * The target and all arguments remain expressions.
 *
 * This means model providers, model handles, dynamically selected models,
 * future execution providers, and domain-specific model references do not
 * require new grammar rules.
 */
inferenceInvocation
    : inferenceInvocationIntroducer
      inferenceInvocationTarget
      (
          LPAREN
          argumentList?
          RPAREN
      )?
      SEMICOLON?
    ;


inferenceInvocationIntroducer
    : INFER
    | AT INFER
    ;


inferenceInvocationTarget
    : expression
    ;


/*
 * ============================================================================
 * 4. INFERENCE MEMBERS
 * ============================================================================
 */

/**
 * An inference body is not a second programming language.
 *
 * It is a compositional region containing:
 *
 *     inference directives
 *     ordinary Zamani statements
 *
 * Resource statements, contracts, assertions, control flow, declarations,
 * concurrency, quantum statements, hardware intent, and other universal
 * constructs therefore enter through `statement`.
 */
inferenceMember
    : inferenceDirective
    | statement
    ;


/*
 * ============================================================================
 * 5. GENERIC INFERENCE DIRECTIVE
 * ============================================================================
 */

/**
 * Extensible inference-local directive.
 *
 * Examples:
 *
 *     @model classifier;
 *
 *     @input features;
 *
 *     @input features: Tensor<Float>;
 *
 *     @output prediction;
 *
 *     @output prediction: Tensor<Float>;
 *
 *     @preprocess normalize;
 *
 *     @postprocess decode;
 *
 *     @decode decoder;
 *
 *     @context context;
 *
 *     @configuration configuration;
 *
 *     @deterministic true;
 *
 *     @stream input;
 *
 *     @batch batch_size;
 *
 *     @quality quality_goal;
 *
 *     @fallback fallback_strategy;
 *
 *     @custom_extension(value);
 *
 * Directive names are deliberately identifiers rather than a finite keyword
 * inventory.
 *
 * Semantic analysis determines which directives are legal.
 */
inferenceDirective
    : AT
      inferenceDirectiveName
      inferenceDirectivePayload?
      SEMICOLON?
    ;


inferenceDirectiveName
    : identifier
    ;


/*
 * ============================================================================
 * 6. DIRECTIVE PAYLOAD
 * ============================================================================
 */

/**
 * The payload grammar is deliberately structural.
 *
 * It supports:
 *
 *     calls
 *     named bindings
 *     typed bindings
 *     assignments
 *     expressions
 *     nested regions
 *
 * without enumerating every possible future inference directive.
 */
inferenceDirectivePayload
    : inferenceDirectiveCall
    | inferenceDirectiveTypedBinding
    | inferenceDirectiveAssignment
    | inferenceDirectiveExpression
    | inferenceDirectiveRegion
    | inferenceDirectiveNamedBinding
    ;


/*
 * ============================================================================
 * 7. DIRECTIVE CALL
 * ============================================================================
 */

inferenceDirectiveCall
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 8. TYPED BINDING
 * ============================================================================
 */

inferenceDirectiveTypedBinding
    : identifier
      COLON
      typeExpression
      inferenceDirectiveCall?
    ;


/*
 * ============================================================================
 * 9. ASSIGNMENT
 * ============================================================================
 */

inferenceDirectiveAssignment
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 10. EXPRESSION PAYLOAD
 * ============================================================================
 */

/**
 * Expression payloads are intentionally generic.
 *
 * Examples:
 *
 *     @batch batch_size;
 *
 *     @context runtime_context;
 *
 *     @quality confidence >= required_confidence;
 *
 *     @configuration config;
 *
 *     @fallback strategy;
 *
 * The expression grammar owns all actual expression semantics.
 */
inferenceDirectiveExpression
    : expression
    ;


/*
 * ============================================================================
 * 11. NAMED BINDING
 * ============================================================================
 */

/**
 * Named binding without a type.
 *
 * Examples:
 *
 *     @model classifier;
 *
 *     @input features;
 *
 *     @output result;
 *
 *     @preprocess normalize;
 *
 *     @decode decoder;
 */
inferenceDirectiveNamedBinding
    : identifier
      inferenceDirectiveCall?
    ;


/*
 * ============================================================================
 * 12. NESTED DIRECTIVE REGION
 * ============================================================================
 */

/**
 * Nested directive region.
 *
 * Example:
 *
 *     @configuration {
 *         @precision precision;
 *         @deterministic true;
 *     }
 *
 * Nested regions remain inference regions rather than introducing another
 * general-purpose block language.
 */
inferenceDirectiveRegion
    : LBRACE
      inferenceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. EXPLICIT INPUT BINDING
 * ============================================================================
 */

/**
 * Stable tooling/AST anchor for an input directive.
 *
 * This is deliberately implemented in terms of the generic directive
 * structure rather than introducing another lexical vocabulary.
 *
 * The rule is reusable by semantic tooling and documentation generators.
 */
inferenceInputBinding
    : AT
      inferenceInputName
      inferenceInputPayload?
      SEMICOLON?
    ;


inferenceInputName
    : identifier
    ;


inferenceInputPayload
    : inferenceDirectiveCall
    | inferenceDirectiveTypedBinding
    | inferenceDirectiveAssignment
    | inferenceDirectiveExpression
    | inferenceDirectiveRegion
    ;


/*
 * ============================================================================
 * 14. EXPLICIT OUTPUT BINDING
 * ============================================================================
 */

inferenceOutputBinding
    : AT
      inferenceOutputName
      inferenceOutputPayload?
      SEMICOLON?
    ;


inferenceOutputName
    : identifier
    ;


inferenceOutputPayload
    : inferenceDirectiveCall
    | inferenceDirectiveTypedBinding
    | inferenceDirectiveAssignment
    | inferenceDirectiveExpression
    | inferenceDirectiveRegion
    ;


/*
 * ============================================================================
 * 15. EXPLICIT MODEL REFERENCE
 * ============================================================================
 */

/**
 * Stable parser anchor for tooling.
 *
 * Model semantics remain owned by grammar/ai/models.g4 and the semantic model.
 */
inferenceModelReference
    : AT
      inferenceModelName
      inferenceModelPayload?
      SEMICOLON?
    ;


inferenceModelName
    : identifier
    ;


inferenceModelPayload
    : inferenceDirectiveCall
    | inferenceDirectiveTypedBinding
    | inferenceDirectiveAssignment
    | inferenceDirectiveExpression
    | inferenceDirectiveRegion
    ;


/*
 * ============================================================================
 * 16. EXTENSION BOUNDARY
 * ============================================================================
 */

/**
 * Generic extension directive.
 *
 * No finite catalogue is encoded here.
 *
 * A dialect may register:
 *
 *     @vendor_extension(...)
 *
 *     @future_inference_feature(...)
 *
 *     @provider_specific(...)
 *
 * without changing this grammar.
 *
 * The semantic/dialect layer is responsible for deciding whether the
 * extension is available and what it means.
 */
inferenceExtension
    : AT
      inferenceExtensionName
      inferenceExtensionPayload?
      SEMICOLON?
    ;


inferenceExtensionName
    : identifier
    ;


inferenceExtensionPayload
    : inferenceDirectiveCall
    | inferenceDirectiveTypedBinding
    | inferenceDirectiveAssignment
    | inferenceDirectiveExpression
    | inferenceDirectiveRegion
    ;


/*
 * ============================================================================
 * 17. SEMANTIC DIRECTIVE CATEGORIES
 * ============================================================================
 *
 * The following conceptual categories are intentionally DOCUMENTATION
 * categories, not separate parser vocabularies.
 *
 * They include:
 *
 *     model
 *     input
 *     output
 *     preprocess
 *     postprocess
 *     decode
 *     batch
 *     stream
 *     context
 *     configuration
 *     execution
 *     numerical
 *     quality
 *     deterministic
 *     stochastic
 *     adaptive
 *     distributed
 *     interoperability
 *     fallback
 *     checkpoint
 *     reproducibility
 *
 * None is given a dedicated parser keyword.
 *
 * This is deliberate.
 *
 * New inference functionality should normally be introduced through semantic
 * registration rather than grammar modification.
 *
 *
 * ============================================================================
 * 18. RESOURCE AND CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Inference does NOT define:
 *
 *     inferenceRequires
 *     inferenceCapability
 *     inferenceConstraint
 *     inferencePreference
 *     inferenceHint
 *
 * because these would duplicate the universal resource architecture.
 *
 * Instead:
 *
 *     inferenceMember
 *          |
 *          v
 *       statement
 *          |
 *          v
 *     resourceStatement
 *          |
 *          v
 *     grammar/resources/
 *
 * This means all inference code automatically participates in the same
 * resource semantics as:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     data
 *     security
 *     execution
 *     compilation
 *
 *
 * ============================================================================
 * 19. CONTRACT BOUNDARY
 * ============================================================================
 *
 * Inference does not duplicate:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * Contract syntax is consumed through the canonical statement/validation
 * architecture.
 *
 * This permits inference to participate in:
 *
 *     preconditions
 *     postconditions
 *     invariants
 *     assumptions
 *     guarantees
 *     verification
 *
 * without creating AI-specific contract syntax.
 *
 *
 * ============================================================================
 * 20. POLICY BOUNDARY
 * ============================================================================
 *
 * Policies are semantic governance, not inference syntax.
 *
 * Inference may therefore be constrained by policies governing:
 *
 *     model use
 *     data use
 *     adaptation
 *     randomness
 *     privacy
 *     security
 *     resource use
 *     execution
 *     deployment
 *     provenance
 *     reproducibility
 *
 * Policy syntax remains owned by the policy subsystem.
 *
 *
 * ============================================================================
 * 21. EFFECT BOUNDARY
 * ============================================================================
 *
 * Inference may produce or require effects such as:
 *
 *     model access
 *     randomness
 *     external data
 *     network
 *     foreign execution
 *     measurement
 *     distributed execution
 *     learning
 *     adaptation
 *
 * These are semantic effects.
 *
 * The parser does not infer or authorize them.
 *
 *
 * ============================================================================
 * 22. PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Semantic inference analysis may record:
 *
 *     model identity
 *     model version
 *     input provenance
 *     output provenance
 *     transformation provenance
 *     configuration provenance
 *     evidence
 *     decision provenance
 *     execution provenance
 *     reproducibility information
 *
 * This grammar merely preserves enough source structure for those systems.
 *
 *
 * ============================================================================
 * 23. QUANTUM-CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Inference is deliberately composable with quantum computation.
 *
 * Examples:
 *
 *     infer classifier(measurement);
 *
 *     infer classifier(quantum_result);
 *
 *     infer decision(measurement_result, classical_context);
 *
 *     infer model(classical_value, quantum_value);
 *
 * No quantum-specific inference rule is necessary.
 *
 * The semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *     inference          quantum semantics
 *       |                    |
 *       |                    v
 *       |                quantum::ir
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *        canonical semantic model
 *
 *
 * ============================================================================
 * 24. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Inference may consume values produced by hardware or HDL computations.
 *
 * No hardware-specific syntax is introduced.
 *
 * Hardware realization remains downstream:
 *
 *     hardware intent
 *          |
 *          v
 *     capabilities/resources
 *          |
 *          v
 *     semantic realization
 *          |
 *          v
 *     lowering/synthesis
 *
 *
 * ============================================================================
 * 25. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed inference may be represented using:
 *
 *     ordinary statements
 *     concurrency constructs
 *     resource requirements
 *     capabilities
 *     policies
 *     execution directives
 *
 * The grammar does not impose:
 *
 *     worker count
 *     replica count
 *     node count
 *     device count
 *     cluster size
 *
 * Those values are source data or semantic requirements rather than grammar
 * limits.
 *
 *
 * ============================================================================
 * 26. ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Inference may be adaptive.
 *
 * For example:
 *
 *     @adaptive strategy;
 *
 *     @fallback alternate_model;
 *
 *     @context runtime_context;
 *
 * These are syntactic directives only.
 *
 * Semantic analysis must enforce:
 *
 *     authorization
 *     policy
 *     capabilities
 *     effects
 *     resources
 *     contracts
 *     provenance
 *
 * Adaptation MUST NOT mean unrestricted self-modifying program text.
 *
 *
 * ============================================================================
 * 27. DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * Source syntax may express deterministic or reproducible intent through
 * ordinary directives:
 *
 *     @deterministic true;
 *
 *     @reproducible true;
 *
 *     @seed seed_value;
 *
 * The grammar does not define a fixed seed width or finite randomness model.
 *
 * Runtime reproducibility is a semantic/runtime concern.
 *
 *
 * ============================================================================
 * 28. NUMERICAL AND QUALITY INTENT
 * ============================================================================
 *
 * Numerical policies remain open-ended:
 *
 *     @precision precision;
 *
 *     @tolerance tolerance;
 *
 *     @stability stability_goal;
 *
 * Quality policies may express:
 *
 *     @quality quality_goal;
 *
 *     @confidence required_confidence;
 *
 *     @acceptance acceptance_policy;
 *
 * No particular numerical representation is mandated by this grammar.
 *
 *
 * ============================================================================
 * 29. EXTENSIBILITY
 * ============================================================================
 *
 * Future inference capabilities should preferably be introduced by:
 *
 *     semantic registration
 *     library
 *     dialect
 *     capability
 *     provider
 *     policy
 *
 * rather than modifying this grammar.
 *
 * Examples of concepts that remain open-world:
 *
 *     future model formats
 *     future inference algorithms
 *     future accelerator types
 *     future quantum-classical algorithms
 *     future probabilistic systems
 *     future distributed systems
 *     future hardware
 *
 *
 * ============================================================================
 * 30. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level errors include:
 *
 *     missing inference target
 *     malformed declaration
 *     missing declaration body
 *     malformed type annotation
 *     malformed initializer
 *     malformed invocation argument list
 *     malformed directive
 *     missing delimiter
 *     missing closing brace
 *     malformed nested directive
 *
 * Semantic errors include:
 *
 *     unknown model
 *     unknown input
 *     unknown output
 *     incompatible input type
 *     incompatible output type
 *     invalid directive
 *     deprecated directive
 *     unsupported capability
 *     unsatisfied requirement
 *     conflicting policy
 *     unavailable resource
 *     unsupported target
 *     invalid effect
 *     non-reproducible execution under a reproducibility requirement
 *
 * Parser and semantic diagnostics MUST remain distinct.
 *
 *
 * ============================================================================
 * 31. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser MUST reject malformed forms such as:
 *
 *     infer;
 *
 *     infer ();
 *
 *     infer model(;
 *
 *     infer model);
 *
 *     infer model(input;
 *
 *     infer model(input,);
 *
 *     infer model: {
 *
 *     infer model = {
 *
 *     infer model {
 *
 *     @infer;
 *
 *     @infer();
 *
 *     @infer model(;
 *
 *     @infer model(input;
 *
 *     @input : Type;
 *
 *     @output : Type;
 *
 *     @configuration = ;
 *
 *
 * ============================================================================
 * 32. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser MUST accept structures equivalent to:
 *
 *     infer classifier(input);
 *
 *     infer classifier(input, context);
 *
 *     @infer classifier(input);
 *
 *     infer classifier {
 *         @model classifier;
 *         @input features: Tensor<Float>;
 *         @output prediction: Tensor<Float>;
 *     }
 *
 *     @infer classifier {
 *         @model classifier;
 *         @input features;
 *         @output prediction;
 *     }
 *
 *     infer classifier: Result {
 *         @input features: Input;
 *         @output prediction: Result;
 *     }
 *
 *     infer classifier = model {
 *         @input features;
 *         @output prediction;
 *     }
 *
 *     infer classifier {
 *         requires capability("tensor.compute");
 *         requires memory >= required_memory;
 *         constraint latency <= latency_budget;
 *         prefer throughput >= desired_throughput;
 *         hint execution::low_latency;
 *     }
 *
 *     infer classifier {
 *         @preprocess normalize;
 *         @postprocess decode;
 *         @decode decoder;
 *         @context context;
 *         @configuration configuration;
 *         @deterministic true;
 *     }
 *
 *     infer classifier {
 *         @adaptive strategy;
 *         @fallback alternate_model;
 *         @distributed execution_policy;
 *     }
 *
 *     infer classifier {
 *         quantum_value = measure_result;
 *         infer nested_model(quantum_value);
 *     }
 *
 *
 * ============================================================================
 * 33. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test progressively:
 *
 *     many inference declarations
 *     many inference invocations
 *     many inputs
 *     many outputs
 *     many arguments
 *     many directives
 *     deeply nested directive regions
 *     deeply nested inference regions
 *     deeply qualified model names
 *     large symbolic expressions
 *     large resource expressions
 *     large cross-domain inference bodies
 *     Unicode identifiers supported by the canonical lexer
 *
 * No chosen test size becomes a language-level maximum.
 *
 *
 * ============================================================================
 * 34. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability testing MUST vary:
 *
 *     declaration count
 *     invocation count
 *     directive count
 *     argument count
 *     nesting depth
 *     expression size
 *     type complexity
 *     resource-expression complexity
 *     cross-domain composition
 *
 * The grammar must not contain:
 *
 *     MAX_INFERENCES
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_ARGUMENTS
 *     MAX_DIRECTIVES
 *     MAX_MODELS
 *     MAX_TENSORS
 *     MAX_BATCH_SIZE
 *     MAX_SEQUENCE_LENGTH
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *
 * Practical limits are implementation/resource characteristics only.
 *
 *
 * ============================================================================
 * 35. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Inference must compose with:
 *
 *     classical
 *     numerical
 *     tensor
 *     data
 *     probabilistic
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     concurrency
 *     distributed
 *     networking
 *     security
 *     simulation
 *     FFI
 *     ABI
 *     metaprogramming
 *     future domains
 *
 * without creating an inference-specific version of each domain.
 *
 *
 * ============================================================================
 * 36. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical inference entry points remain:
 *
 *     inferenceConstruct
 *     inferenceDeclaration
 *     inferenceInvocation
 *
 * Existing consumers should continue to import:
 *
 *     Inference
 *
 * and consume:
 *
 *     inferenceConstruct
 *
 * The grammar identity MUST remain:
 *
 *     Inference
 *
 * This avoids unnecessary changes to:
 *
 *     grammar/ai/ai.g4
 *
 *
 * ============================================================================
 * 37. AI COMPOSITION INTEGRATION
 * ============================================================================
 *
 * grammar/ai/ai.g4 already owns:
 *
 *     aiConstruct
 *
 * and composes:
 *
 *     inferenceConstruct
 *
 * Therefore this file MUST NOT define:
 *
 *     aiConstruct
 *
 * and MUST NOT import:
 *
 *     AI
 *
 * This preserves the dependency direction:
 *
 *     AI
 *       |
 *       +--> Inference
 *
 * rather than:
 *
 *     Inference
 *       |
 *       +--> AI
 *
 *
 * ============================================================================
 * 38. MODEL INTEGRATION
 * ============================================================================
 *
 * Model declarations remain owned by:
 *
 *     grammar/ai/models.g4
 *
 * Inference references models through:
 *
 *     expression
 *
 * and/or:
 *
 *     @model ...
 *
 * The inference grammar does not import `AIModels`.
 *
 * This is intentional.
 *
 * It prevents:
 *
 *     Inference -> AIModels -> Inference
 *
 * dependency cycles.
 *
 *
 * ============================================================================
 * 39. TRAINING INTEGRATION
 * ============================================================================
 *
 * Training remains owned by:
 *
 *     grammar/ai/training.g4
 *
 * An inference may consume a trained model as an ordinary expression/value.
 *
 * This grammar does not define training algorithms or training syntax.
 *
 *
 * ============================================================================
 * 40. PROBABILISTIC INTEGRATION
 * ============================================================================
 *
 * Probabilistic inference is not duplicated here.
 *
 * Probabilistic computation remains owned by:
 *
 *     grammar/ai/probabilistic.g4
 *
 * A probabilistic model or result may participate in ordinary inference
 * expressions.
 *
 * This avoids two competing inference languages.
 *
 *
 * ============================================================================
 * 41. KNOWLEDGE / REASONING INTEGRATION
 * ============================================================================
 *
 * Knowledge and reasoning are semantic inputs/operations that may participate
 * in inference.
 *
 * They remain owned by their respective universal expression/statement
 * grammars.
 *
 * For example:
 *
 *     infer classifier(knowledge query(...));
 *
 * or:
 *
 *     infer decision(reasoning_result);
 *
 * where those expressions are supported by the canonical expression system.
 *
 * This file does not redefine:
 *
 *     infer
 *     deduce
 *     reason
 *     query
 *     assert
 *     retract
 *
 * as competing AI syntax.
 *
 *
 * ============================================================================
 * 42. ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptation is a controlled semantic capability.
 *
 * Inference may carry adaptation intent through directives such as:
 *
 *     @adaptive strategy;
 *
 *     @fallback alternate;
 *
 * but adaptation authority remains controlled by:
 *
 *     effects
 *     capabilities
 *     policies
 *     resources
 *     validation
 *     provenance
 *
 * The grammar does not grant authority merely by parsing a directive.
 *
 *
 * ============================================================================
 * 43. FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Inference may consume foreign or ABI-backed model implementations through
 * ordinary expressions and statements.
 *
 * This grammar does not define:
 *
 *     calling conventions
 *     ABI layout
 *     foreign type representation
 *     binary format
 *
 * Those remain owned by:
 *
 *     grammar/interoperability/
 *
 *
 * ============================================================================
 * 44. NO PHYSICAL TARGET SELECTION
 * ============================================================================
 *
 * This grammar MUST NOT accept universal inference semantics such as:
 *
 *     GPU 0
 *     CPU 3
 *     QPU 2
 *     FPGA 4
 *     device 17
 *
 * as special grammar constructs.
 *
 * If such names occur as ordinary application data or identifiers, their
 * meaning remains semantic rather than being imposed by inference.g4.
 *
 *
 * ============================================================================
 * 45. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO fixed model count
 *     NO fixed input count
 *     NO fixed output count
 *     NO fixed argument count
 *     NO fixed directive count
 *     NO fixed tensor rank
 *     NO fixed tensor dimensions
 *     NO fixed batch size
 *     NO fixed sequence length
 *     NO fixed worker count
 *     NO fixed device count
 *     NO fixed accelerator count
 *     NO fixed CPU count
 *     NO fixed GPU count
 *     NO fixed FPGA count
 *     NO fixed ASIC count
 *     NO fixed QPU count
 *     NO fixed qubit count
 *     NO fixed node count
 *     NO fixed memory capacity
 *     NO fixed register width
 *     NO fixed network size
 *     NO vendor catalogue
 *     NO model catalogue
 *     NO algorithm catalogue
 *     NO quantum-gate catalogue
 *
 *
 * ============================================================================
 * 46. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] Grammar identity is `Inference`.
 *
 * [ ] `tokenVocab = ZamaniLexer`.
 *
 * [ ] Only canonical parser grammars are imported.
 *
 * [ ] `inferenceConstruct` is the sole public AI-inference boundary.
 *
 * [ ] `inferenceDeclaration` is unambiguous against model/agent/pipeline
 *     declarations.
 *
 * [ ] `INFER` is used as the canonical inference entry keyword.
 *
 * [ ] `SEMICOLON` is used instead of legacy `SEMI`.
 *
 * [ ] No lexer rules are defined here.
 *
 * [ ] No duplicate resource grammar exists here.
 *
 * [ ] No duplicate capability grammar exists here.
 *
 * [ ] No duplicate constraint grammar exists here.
 *
 * [ ] No duplicate preference grammar exists here.
 *
 * [ ] No duplicate hint grammar exists here.
 *
 * [ ] No duplicate contract grammar exists here.
 *
 * [ ] No duplicate expression hierarchy exists here.
 *
 * [ ] No duplicate type hierarchy exists here.
 *
 * [ ] Model references remain open-world.
 *
 * [ ] Directive names remain open-world.
 *
 * [ ] No application-specific keyword catalogue exists.
 *
 * [ ] Quantum values remain ordinary semantic values.
 *
 * [ ] Quantum realization remains owned by the quantum subsystem.
 *
 * [ ] Hardware realization remains downstream.
 *
 * [ ] Resource negotiation remains downstream.
 *
 * [ ] Policy evaluation remains downstream.
 *
 * [ ] Provenance remains downstream.
 *
 * [ ] No IR is created by this grammar.
 *
 * [ ] Quantum computation crosses the canonical `quantum::ir` boundary.
 *
 * [ ] No artificial scalability ceiling exists.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Rust 1.97+ generation succeeds.
 *
 * [ ] Generated compiler integration remains safe Rust.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Inference describes:
 *
 *     WHAT inference computation is requested.
 *
 * It does not prescribe:
 *
 *     WHERE it executes;
 *     WHICH hardware executes it;
 *     HOW resources are allocated;
 *     WHICH implementation is selected;
 *     HOW quantum operations are routed;
 *     HOW hardware is synthesized;
 *     HOW the runtime schedules work.
 *
 * Therefore:
 *
 *     inference source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic inference model
 *          |
 *          +--> types
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
 *          +--> classical realization
 *          +--> quantum::ir
 *          +--> HDL/hardware realization
 *          +--> distributed realization
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     routing/scheduling where required
 *          |
 *          v
 *     resilience/recovery where required
 *          |
 *          v
 *     target realization
 *
 * This separation is what allows one inference program to remain source-level
 * portable across systems ranging from extremely small execution environments
 * to arbitrarily large resource configurations without embedding machine-size
 * assumptions into the grammar.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */