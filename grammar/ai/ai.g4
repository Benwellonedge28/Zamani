/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/ai.g4
 *
 * Status:
 *     Production AI / ML domain-composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     Parser grammar only.
 *
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No process execution.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No mutable global state.
 *     - No vendor-specific implementation.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical AI-domain COMPOSITION BOUNDARY.
 *
 * It does not implement model execution, tensor storage, training algorithms,
 * inference engines, automatic differentiation, accelerator selection,
 * scheduling, deployment, or runtime behavior.
 *
 * Its responsibility is to establish the stable parser-level boundary through
 * which AI-domain syntax enters the Zamani frontend.
 *
 * The AI subsystem is intentionally layered:
 *
 *     grammar/ai/ai.g4
 *          |
 *          +--> models.g4
 *          +--> datasets.g4
 *          +--> tensors.g4
 *          +--> training.g4
 *          +--> inference.g4
 *          +--> agents.g4
 *          +--> differentiation.g4
 *          +--> pipelines.g4
 *          +--> ai-accelerators.g4
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Semantic analysis
 *          |
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> portability analysis
 *          |
 *          v
 *     Canonical semantic IR
 *          |
 *          +--> classical lowering
 *          +--> quantum lowering
 *          +--> accelerator lowering
 *          +--> distributed lowering
 *          +--> hardware realization
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * IMPORTANT:
 *
 * This grammar MUST NOT become an AI-specific IR.
 *
 * ============================================================================
 *
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the AI parser-domain entry point;
 *     - AI-domain construct composition;
 *     - AI-domain computation-region syntax;
 *     - AI-domain declaration/binding/invocation boundaries;
 *     - AI-domain resource/capability/constraint syntax;
 *     - AI-domain interoperability boundaries;
 *     - stable extension points for AI dialects;
 *     - parser-level separation between AI syntax and ordinary expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier syntax;
 *     - general expressions;
 *     - general types;
 *     - ordinary statements;
 *     - functions;
 *     - modules;
 *     - memory;
 *     - concurrency;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - hardware discovery;
 *     - hardware calibration;
 *     - routing;
 *     - scheduling;
 *     - optimization passes;
 *     - tensor storage;
 *     - tensor kernels;
 *     - model execution;
 *     - model training algorithms;
 *     - inference algorithms;
 *     - automatic differentiation implementation;
 *     - optimizer implementation;
 *     - accelerator selection;
 *     - device selection;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical expression grammar:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical type grammar:
 *
 *     grammar/antlr/Types.g4
 *
 * Canonical statement grammar:
 *
 *     grammar/statements/statements.g4
 *
 * These layers own their respective syntax.
 *
 * AI.g4 MUST NOT redefine them.
 *
 * ============================================================================
 *
 * AI LEAF-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * Existing AI leaf grammars include:
 *
 *     grammar/ai/models.g4
 *     grammar/ai/datasets.g4
 *     grammar/ai/tensors.g4
 *     grammar/ai/training.g4
 *     grammar/ai/inference.g4
 *     grammar/ai/agents.g4
 *     grammar/ai/differentiation.g4
 *     grammar/ai/pipelines.g4
 *     grammar/ai/ai-accelerators.g4
 *
 * These files are deliberately NOT imported here merely to expose every
 * implementation detail through this aggregate grammar.
 *
 * Reason:
 *
 *     1. Each leaf grammar has its own parser grammar identity.
 *     2. Several leaf grammars reuse generic rule names such as `expression`,
 *        `identifier`, and domain-local construct names.
 *     3. Importing every leaf grammar into this file would unnecessarily
 *        increase rule-collision and ambiguity risk.
 *     4. The canonical parser composition layer can import the required leaf
 *        grammars directly.
 *     5. AI.g4 remains the stable common boundary.
 *
 * This is intentional dependency inversion:
 *
 *     canonical parser
 *          |
 *          +--> AI
 *          +--> AIModels
 *          +--> AIDatasets
 *          +--> AITensors
 *          +--> AITraining
 *          +--> AIInference
 *          +--> Agents
 *          +--> ...
 *
 * rather than:
 *
 *     AI aggregate
 *          |
 *          +--> every AI implementation
 *          |
 *          +--> every future AI dialect
 *
 * ============================================================================
 *
 * LEXICAL POLICY
 * ============================================================================
 *
 * AI vocabulary MUST NOT be added to the lexer merely because an AI concept
 * exists.
 *
 * The canonical lexer already provides NANO_ANNOTATION:
 *
 *     @model
 *     @dataset
 *     @tensor
 *     @training
 *     @inference
 *     @agent
 *     @pipeline
 *     @accelerator
 *
 * Semantic analysis determines the meaning of the annotation.
 *
 * This permits future concepts without continuously modifying the lexical
 * grammar.
 *
 * ============================================================================
 *
 * IMPORTANT CORRECTION TO THE PREVIOUS DESIGN
 * ============================================================================
 *
 * The old design contained alternatives such as:
 *
 *     aiExpression
 *     aiValue
 *     aiModelExpression
 *     aiDatasetExpression
 *     aiTrainingExpression
 *
 * where multiple alternatives ultimately reduced to:
 *
 *     expression
 *
 * This makes the public AI entry point structurally ambiguous and allows an
 * arbitrary ordinary expression to be classified as an AI construct.
 *
 * That design is NOT acceptable for a production grammar.
 *
 * The corrected design requires AI-domain syntax to cross an explicit
 * AI-domain boundary.
 *
 * Ordinary expressions remain ordinary expressions.
 *
 * ============================================================================
 *
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar contains no limits on:
 *
 *     - number of models;
 *     - number of tensors;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - parameter count;
 *     - layer count;
 *     - dataset size;
 *     - batch size;
 *     - sequence length;
 *     - number of training steps;
 *     - number of inference calls;
 *     - number of agents;
 *     - number of pipeline stages;
 *     - number of workers;
 *     - number of nodes;
 *     - number of accelerators;
 *     - number of devices;
 *     - number of quantum resources;
 *     - number of classical resources.
 *
 * Repetition is structural.
 *
 * Resource availability is evaluated downstream.
 *
 * The grammar therefore does not encode:
 *
 *     MAX_TENSORS
 *     MAX_LAYERS
 *     MAX_PARAMETERS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_WORKERS
 *     MAX_NODES
 *
 * or equivalent constants.
 *
 * ============================================================================
 *
 * MACHINE INDEPENDENCE
 * ============================================================================
 *
 * AI syntax MUST NOT require:
 *
 *     GPU 0
 *     CPU 0
 *     accelerator 1
 *     device 7
 *     fixed VRAM
 *     fixed memory
 *     fixed SIMD width
 *     fixed tensor-core count
 *     fixed cluster size
 *     fixed node topology
 *     fixed vendor
 *     fixed accelerator architecture
 *
 * Such information belongs to:
 *
 *     capability analysis;
 *     resource analysis;
 *     target description;
 *     scheduling;
 *     deployment;
 *     runtime discovery;
 *     execution policy.
 *
 * ============================================================================
 *
 * QUANTUM INTEROPERABILITY
 * ============================================================================
 *
 * AI syntax may reference quantum computations through ordinary expressions
 * and AI-domain bindings.
 *
 * This grammar MUST NOT define quantum semantics.
 *
 * Quantum semantics remain owned by the quantum grammar and ultimately by
 * the canonical `quantum::ir` semantic boundary.
 *
 * Therefore:
 *
 *     AI -> quantum expression/reference
 *
 * is permitted at the source level, while:
 *
 *     AI -> quantum::ir
 *
 * is a downstream semantic/lowering operation.
 *
 * AI.g4 MUST NOT:
 *
 *     - define QubitId;
 *     - define physical qubits;
 *     - define quantum gates;
 *     - define quantum topology;
 *     - define QEC;
 *     - define ZQN.
 *
 * ============================================================================
 *
 * HARDWARE INTEROPERABILITY
 * ============================================================================
 *
 * AI source may express:
 *
 *     requires capability(...);
 *     requires resource(...);
 *     prefers capability(...);
 *     constrains(...);
 *
 * but must not directly encode a concrete device realization.
 *
 * ============================================================================
 */

parser grammar AI;

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
 * Canonical AI-domain parser entry point.
 *
 * Every alternative crosses an explicit AI-domain boundary.
 *
 * Ordinary expressions are NOT accepted directly here.
 */
aiConstruct
    : aiAnnotatedDeclaration
    | aiAnnotatedBinding
    | aiAnnotatedInvocation
    | aiAnnotatedRegion
    | aiResourceContract
    | aiCapabilityContract
    | aiConstraintContract
    | aiPreferenceContract
    | aiInteroperabilityConstruct
    ;


/* ============================================================================
 * 2. AI ANNOTATED DECLARATIONS
 * ========================================================================== */

/**
 * Generic AI declaration boundary.
 *
 * Examples:
 *
 *     @model MyModel ...
 *     @dataset TrainingData ...
 *     @tensor Input ...
 *     @training Experiment ...
 *     @agent ResearchAgent ...
 *
 * The annotation is lexically opaque.
 *
 * Semantic analysis resolves the annotation to a registered AI domain,
 * dialect, library construct, or extension.
 */
aiAnnotatedDeclaration
    : NANO_ANNOTATION
      identifier
      aiDeclarationParameters?
      aiDeclarationType?
      aiDeclarationInitializer?
      aiDeclarationBody?
    ;


/**
 * Optional generic declaration parameters.
 *
 * Parameters are expressions rather than machine-specific constants.
 */
aiDeclarationParameters
    : LPAREN argumentList? RPAREN
    ;


/**
 * Optional declared type.
 *
 * Type ownership remains canonical.
 */
aiDeclarationType
    : COLON typeExpression
    ;


/**
 * Optional initializer.
 *
 * Expression ownership remains canonical.
 */
aiDeclarationInitializer
    : ASSIGN expression
    ;


/**
 * Optional declaration body.
 *
 * A declaration may contain a computation region.
 */
aiDeclarationBody
    : aiComputationRegion
    | aiDeclarationExpressionBody
    ;


/**
 * Expression-valued AI declaration body.
 */
aiDeclarationExpressionBody
    : expression
    ;


/* ============================================================================
 * 3. AI BINDINGS
 * ========================================================================== */

/**
 * Binds an AI-domain value to a source-level name.
 *
 * Examples:
 *
 *     @input x
 *     @output y
 *     @parameter weights
 *     @state hidden
 *
 * The semantic layer determines whether the annotation represents input,
 * output, parameter, state, component, layer, or another registered role.
 */
aiAnnotatedBinding
    : NANO_ANNOTATION
      identifier
      aiBindingType?
      aiBindingInitializer?
      SEMICOLON
    ;


aiBindingType
    : COLON typeExpression
    ;


aiBindingInitializer
    : ASSIGN expression
    ;


/* ============================================================================
 * 4. AI INVOCATIONS
 * ========================================================================== */

/**
 * AI-domain operation invocation.
 *
 * The operation name is an ordinary identifier.
 *
 * This permits:
 *
 *     user-defined models;
 *     library models;
 *     future algorithms;
 *     dialect operations;
 *     vendor-independent operations;
 *     extension operations.
 */
aiAnnotatedInvocation
    : NANO_ANNOTATION
      identifier
      LPAREN argumentList? RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 5. AI COMPUTATION REGIONS
 * ========================================================================== */

/**
 * AI computation region.
 *
 * Statement syntax remains owned by Statements.
 */
aiAnnotatedRegion
    : NANO_ANNOTATION
      identifier
      aiComputationRegion
    ;


aiComputationRegion
    : LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 6. RESOURCE CONTRACTS
 * ========================================================================== */

/**
 * AI resource requirements.
 *
 * The annotation identifies the semantic resource domain.
 *
 * Examples:
 *
 *     @requires resource(...)
 *     @requires memory(...)
 *     @requires accelerator(...)
 *
 * The exact meaning is resolved by semantic/resource analysis.
 */
aiResourceContract
    : NANO_ANNOTATION
      aiResourceExpression
      SEMICOLON?
    ;


aiResourceExpression
    : expression
    ;


/* ============================================================================
 * 7. CAPABILITY CONTRACTS
 * ========================================================================== */

/**
 * Capability requirements are distinct from concrete resource selection.
 */
aiCapabilityContract
    : NANO_ANNOTATION
      aiCapabilityExpression
      SEMICOLON?
    ;


aiCapabilityExpression
    : expression
    ;


/* ============================================================================
 * 8. CONSTRAINT CONTRACTS
 * ========================================================================== */

/**
 * A constraint expresses a semantic requirement or admissibility condition.
 *
 * It does not select a particular machine.
 */
aiConstraintContract
    : NANO_ANNOTATION
      aiConstraintExpression
      SEMICOLON?
    ;


aiConstraintExpression
    : expression
    ;


/* ============================================================================
 * 9. PREFERENCE CONTRACTS
 * ========================================================================== */

/**
 * Preferences are deliberately weaker than requirements.
 *
 * Example semantic meanings:
 *
 *     prefer low latency
 *     prefer energy efficiency
 *     prefer accelerator execution
 *     prefer local execution
 *
 * These remain semantic/resource policy concerns.
 */
aiPreferenceContract
    : NANO_ANNOTATION
      aiPreferenceExpression
      SEMICOLON?
    ;


aiPreferenceExpression
    : expression
    ;


/* ============================================================================
 * 10. INTEROPERABILITY
 * ========================================================================== */

/**
 * Generic AI interoperability boundary.
 *
 * AI can interact with:
 *
 *     classical values;
 *     quantum values;
 *     hardware-backed values;
 *     distributed values;
 *     data values;
 *     external callable interfaces.
 *
 * The grammar does not define the implementation of those domains.
 */
aiInteroperabilityConstruct
    : aiInteroperabilityBinding
    | aiInteroperabilityCall
    ;


aiInteroperabilityBinding
    : NANO_ANNOTATION
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


aiInteroperabilityCall
    : NANO_ANNOTATION
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 11. AI DOMAIN MARKERS
 * ========================================================================== */

/**
 * Semantic domain markers.
 *
 * These are parser-level symbolic categories, not lexer keywords.
 *
 * Semantic analysis may map annotations such as:
 *
 *     @model
 *     @dataset
 *     @tensor
 *     @training
 *     @inference
 *     @agent
 *     @pipeline
 *     @accelerator
 *     @differentiation
 *
 * to registered AI domain descriptors.
 *
 * The grammar deliberately does not enumerate those names.
 *
 * This allows future AI domains without modifying this grammar.
 */
aiDomainMarker
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 12. AI VALUE BRIDGE
 * ========================================================================== */

/**
 * A source-level bridge for an AI value.
 *
 * This rule is intentionally NOT part of aiConstruct.
 *
 * It exists so another parser composition layer can explicitly request an
 * AI value without making every ordinary expression an AI construct.
 */
aiValue
    : expression
    ;


/**
 * AI type bridge.
 *
 * Type semantics remain owned by the canonical type grammar.
 */
aiType
    : typeExpression
    ;


/**
 * AI expression bridge.
 *
 * Expression semantics remain owned by Expressions.
 */
aiExpression
    : expression
    ;


/* ============================================================================
 * 13. MODEL / DATASET / TENSOR / TRAINING / INFERENCE BRIDGES
 * ========================================================================== */

/**
 * These rules are stable semantic bridges.
 *
 * They are deliberately not public alternatives of aiConstruct.
 *
 * Leaf grammars own their concrete syntax:
 *
 *     models.g4
 *     datasets.g4
 *     tensors.g4
 *     training.g4
 *     inference.g4
 *     agents.g4
 *     differentiation.g4
 *     pipelines.g4
 *     ai-accelerators.g4
 *
 * This prevents this aggregate grammar from duplicating those grammars.
 */

aiModelReference
    : expression
    ;


aiDatasetReference
    : expression
    ;


aiTensorReference
    : expression
    ;


aiTrainingReference
    : expression
    ;


aiInferenceReference
    : expression
    ;


aiAgentReference
    : expression
    ;


aiPipelineReference
    : expression
    ;


aiAcceleratorReference
    : expression
    ;


aiDifferentiationReference
    : expression
    ;


/* ============================================================================
 * 14. AI STATEMENT SEQUENCE BRIDGE
 * ========================================================================== */

/**
 * Canonical statement sequence used by AI regions.
 *
 * No AI-specific statement hierarchy is introduced.
 */
aiStatementSequence
    : statement*
    ;


/* ============================================================================
 * 15. AI ARGUMENT BRIDGE
 * ========================================================================== */

/**
 * Canonical argument syntax is owned by Expressions.
 *
 * This rule exists only as a stable semantic naming point for AI tooling.
 */
aiArgumentList
    : argumentList
    ;


/* ============================================================================
 * 16. AI TYPE ARGUMENT BRIDGE
 * ========================================================================== */

/**
 * Generic AI type argument.
 */
aiTypeArgument
    : typeExpression
    ;


/* ============================================================================
 * 17. AI METADATA
 * ========================================================================== */

/**
 * AI metadata remains expression-valued.
 *
 * Metadata semantics belong to the semantic layer.
 */
aiMetadataEntry
    : NANO_ANNOTATION
      expression
      SEMICOLON?
    ;


aiMetadataList
    : aiMetadataEntry*
    ;


/* ============================================================================
 * 18. AI EXTENSION POINT
 * ========================================================================== */

/**
 * Generic future AI extension.
 *
 * An extension is explicitly annotation-led so it cannot silently capture
 * arbitrary ordinary Zamani expressions.
 */
aiExtension
    : NANO_ANNOTATION
      identifier
      aiExtensionArguments?
      aiExtensionBody?
    ;


aiExtensionArguments
    : LPAREN argumentList? RPAREN
    ;


aiExtensionBody
    : aiComputationRegion
    | expression
    ;


/* ============================================================================
 * 19. PORTABILITY CONTRACT
 * ========================================================================== */

/**
 * AI portability declaration.
 *
 * The grammar permits a semantic portability contract without prescribing a
 * particular deployment target.
 */
aiPortabilityContract
    : NANO_ANNOTATION
      aiPortabilityExpression
      SEMICOLON?
    ;


aiPortabilityExpression
    : expression
    ;


/* ============================================================================
 * 20. COMPLETION CONTRACT
 * ========================================================================== */

/*
 * This grammar is complete only when all of the following remain true:
 *
 * [ ] `AI` is a parser grammar.
 *
 * [ ] `ZamaniLexer` remains the sole lexical authority.
 *
 * [ ] `Types` remains the sole owner of general type syntax.
 *
 * [ ] `Expressions` remains the sole owner of general expression syntax.
 *
 * [ ] `Statements` remains the sole owner of general statement syntax.
 *
 * [ ] No AI rule defines a competing expression grammar.
 *
 * [ ] No AI rule defines a competing type grammar.
 *
 * [ ] No AI rule defines a competing identifier grammar.
 *
 * [ ] No AI rule defines a competing quantum representation.
 *
 * [ ] No AI rule defines QubitId, PhysicalQubitId, QEC, ZQN, or hardware
 *     topology.
 *
 * [ ] No fixed model count exists.
 *
 * [ ] No fixed tensor rank exists.
 *
 * [ ] No fixed tensor dimension exists.
 *
 * [ ] No fixed parameter count exists.
 *
 * [ ] No fixed layer count exists.
 *
 * [ ] No fixed dataset size exists.
 *
 * [ ] No fixed accelerator count exists.
 *
 * [ ] No fixed worker count exists.
 *
 * [ ] No fixed device count exists.
 *
 * [ ] No fixed machine size exists.
 *
 * [ ] No device ID is required by syntax.
 *
 * [ ] No vendor-specific AI keyword is required by syntax.
 *
 * [ ] AI constructs are distinguishable from ordinary expressions.
 *
 * [ ] AI leaf grammars remain independently composable.
 *
 * [ ] Future AI dialects can be added without changing the lexical grammar.
 *
 * [ ] Semantic analysis, not parsing, resolves annotation meaning.
 *
 * [ ] Resource analysis, not parsing, determines resource feasibility.
 *
 * [ ] Capability analysis, not parsing, determines target suitability.
 *
 * [ ] Scheduling, not parsing, determines execution order and timing.
 *
 * [ ] Runtime, not parsing, discovers actual available resources.
 *
 * [ ] The grammar imposes no artificial scalability ceiling.
 *
 * [ ] The Rust implementation using this grammar contains no unsafe code.
 *
 * [ ] Rust 1.97 / 1.97.1 remains supported.
 *
 * [ ] Generated parser code is treated as generated output rather than
 *     hand-maintained source.
 */