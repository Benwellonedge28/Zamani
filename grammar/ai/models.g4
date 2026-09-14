/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/models.g4
 *
 * Status:
 *     Production AI model-domain parser grammar.
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
 * This file owns the SOURCE-LEVEL SYNTAX for declarative AI/ML MODEL
 * definitions in Zamani.
 *
 * It deliberately describes model intent and structure rather than a
 * particular machine-learning framework, accelerator, compiler, device,
 * vendor, or runtime.
 *
 * The grammar supports model descriptions that can express:
 *
 *     - model identity;
 *     - generic model parameters;
 *     - model interfaces;
 *     - inputs;
 *     - outputs;
 *     - trainable parameters;
 *     - non-trainable state;
 *     - components;
 *     - layers;
 *     - submodels;
 *     - model composition;
 *     - data dependencies;
 *     - graph connections;
 *     - model bindings;
 *     - model configuration;
 *     - model metadata;
 *     - model requirements;
 *     - model capabilities;
 *     - model constraints;
 *     - model preferences;
 *     - model portability declarations;
 *     - model execution expressions;
 *     - model construction expressions.
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
 *     canonical parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Types / Expressions           AI model grammar
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
 *                AI Semantic Model
 *                        |
 *                        +----------------------+
 *                        |                      |
 *                        v                      v
 *                  Classical IR           Quantum IR
 *                        |                      |
 *                        +----------+-----------+
 *                                   |
 *                                   v
 *                              Canonical IR
 *                                   |
 *                                   v
 *                              Optimization
 *                                   |
 *                                   v
 *                               Scheduling
 *                                   |
 *                                   v
 *                           Target realization
 *                                   |
 *                                   v
 *                                Runtime
 *
 * models.g4 MUST NOT construct IR.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - model declaration syntax;
 *     - model member composition;
 *     - model interface syntax;
 *     - model graph/dependency syntax;
 *     - model component/layer/submodel syntax;
 *     - model parameter/state declarations;
 *     - model metadata declarations;
 *     - model requirement/capability/constraint syntax;
 *     - model portability declarations;
 *     - model-local configuration syntax;
 *     - model-local operation boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - tensor implementation;
 *     - tensor storage;
 *     - numerical kernels;
 *     - automatic differentiation;
 *     - optimizer algorithms;
 *     - dataset storage;
 *     - training algorithms;
 *     - inference algorithms;
 *     - neural-network execution;
 *     - accelerator implementation;
 *     - GPU selection;
 *     - CPU selection;
 *     - NPU/TPU selection;
 *     - hardware discovery;
 *     - hardware topology;
 *     - distributed placement;
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
 * KEYWORD / LEXER POLICY
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `model`, `input`, `output`, `parameter`, `state`, `layer`, `component`,
 * `submodel`, `connect`, `requires`, `capability`, and similar AI-domain
 * vocabulary MUST NOT be added as machine-specific or framework-specific
 * lexer keywords merely because this file needs them.
 *
 * The canonical Zamani lexer already provides generic NANO_ANNOTATION syntax.
 *
 * Therefore the model declaration boundary is:
 *
 *     @model ModelName { ... }
 *
 * and model member roles use:
 *
 *     @input
 *     @output
 *     @parameter
 *     @state
 *     @component
 *     @layer
 *     @submodel
 *     @resource
 *     @capability
 *     @constraint
 *     @preference
 *     @metadata
 *
 * Semantic analysis validates the annotation spelling and meaning.
 *
 * This preserves the repository's existing policy that AI library/domain
 * vocabulary remains extensible rather than being exhaustively encoded in the
 * lexer.
 *
 * ============================================================================
 *
 * WHY ANNOTATION-BASED MODEL DECLARATIONS
 * ============================================================================
 *
 * ANTLR parser grammars using `tokenVocab = ZamaniLexer` cannot safely invent
 * arbitrary string-literal keyword tokens that are not present in the
 * canonical lexer.
 *
 * The current lexer intentionally keeps most AI vocabulary as identifiers.
 *
 * Using NANO_ANNOTATION therefore provides:
 *
 *     - a stable lexical boundary;
 *     - no duplicate lexer;
 *     - no implicit parser-only keyword tokens;
 *     - no semantic predicates;
 *     - no target-specific actions;
 *     - no framework coupling;
 *     - future AI dialect compatibility.
 *
 * The semantic layer MUST validate:
 *
 *     @model
 *
 * as the model declaration annotation.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A model declaration expresses:
 *
 *     WHAT the model is;
 *     WHAT values it accepts;
 *     WHAT values it produces;
 *     WHAT components it contains;
 *     WHAT dependencies exist;
 *     WHAT capabilities it requires;
 *     WHAT constraints it declares;
 *     WHAT resources it prefers or requires.
 *
 * It MUST NOT encode:
 *
 *     - a fixed GPU;
 *     - a fixed CPU;
 *     - a fixed accelerator;
 *     - a fixed device ID;
 *     - a fixed memory capacity;
 *     - a fixed VRAM capacity;
 *     - a fixed number of devices;
 *     - a fixed number of cores;
 *     - a fixed number of threads;
 *     - a fixed SIMD width;
 *     - a fixed cluster size;
 *     - a fixed network topology;
 *     - a fixed accelerator topology.
 *
 * Such properties belong to resource analysis, target description, scheduling,
 * deployment, and runtime.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     - number of model inputs;
 *     - number of model outputs;
 *     - number of parameters;
 *     - number of states;
 *     - number of layers;
 *     - number of components;
 *     - number of submodels;
 *     - number of connections;
 *     - number of model requirements;
 *     - number of capabilities;
 *     - number of constraints;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - model size;
 *     - parameter count;
 *     - sequence length;
 *     - batch size;
 *     - deployment scale.
 *
 * Repetition is structural.
 *
 * Practical limits are controlled by parser/runtime resources, compiler
 * policies, resource availability, and target capabilities.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * models.g4 MUST NOT introduce another type system.
 *
 * Model ports, parameters, states, components and outputs therefore use:
 *
 *     typeExpression
 *
 * imported from the canonical type grammar.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * models.g4 MUST NOT redefine:
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
 * All model expressions therefore use:
 *
 *     expression
 *
 * from the canonical expression grammar.
 *
 * ============================================================================
 *
 * GENERIC MODEL CONTRACT
 * ============================================================================
 *
 * Model declarations may be generic.
 *
 * Genericity can represent:
 *
 *     - element type;
 *     - feature type;
 *     - label type;
 *     - precision;
 *     - shape parameters;
 *     - model configuration types;
 *     - backend-neutral semantic parameters.
 *
 * Generic parameters MUST NOT be interpreted as machine resource counts
 * unless semantic analysis explicitly establishes such meaning.
 *
 * ============================================================================
 *
 * MODEL SEMANTICS
 * ============================================================================
 *
 * A model is a semantic object.
 *
 * This grammar only describes its source representation.
 *
 * The semantic layer is responsible for validating:
 *
 *     - duplicate ports;
 *     - duplicate parameters;
 *     - duplicate states;
 *     - invalid references;
 *     - incompatible types;
 *     - cyclic dependency rules where prohibited;
 *     - invalid graph edges;
 *     - missing required outputs;
 *     - invalid component references;
 *     - invalid resource declarations;
 *     - capability conflicts;
 *     - constraint conflicts;
 *     - portability violations.
 *
 * The grammar MUST NOT attempt to perform those checks.
 *
 * ============================================================================
 *
 * MODEL GRAPH CONTRACT
 * ============================================================================
 *
 * Model connections represent semantic data/control dependencies.
 *
 * They do NOT represent:
 *
 *     - physical network links;
 *     - GPU interconnects;
 *     - CPU topology;
 *     - quantum topology;
 *     - hardware routing;
 *     - scheduling order.
 *
 * The same model graph may therefore be realized differently on different
 * machines.
 *
 * ============================================================================
 *
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource declarations describe requirements or preferences.
 *
 * They do not allocate resources.
 *
 * Examples of semantic concepts that may be represented include:
 *
 *     compute;
 *     memory;
 *     storage;
 *     bandwidth;
 *     latency;
 *     throughput;
 *     energy;
 *     reliability;
 *     accelerator capability;
 *     quantum capability;
 *     distributed execution capability.
 *
 * The grammar intentionally does not define numerical machine limits.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * A model may contain or reference computations implemented using:
 *
 *     classical computation;
 *     quantum computation;
 *     hardware/HDL;
 *     distributed computation;
 *     accelerators;
 *     future Zamani domains.
 *
 * This file does not own those domains.
 *
 * It only provides model-level composition boundaries.
 *
 * ============================================================================
 */

parser grammar AIModels;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Public entry point for AI model-domain syntax.
 *
 * The canonical AI grammar can expose this rule as part of `aiConstruct`.
 */
aiModelConstruct
    : aiModelDeclaration
    | aiModelReference
    | aiModelInstantiation
    | aiModelComposition
    ;


/* ============================================================================
 * 2. MODEL DECLARATION
 * ========================================================================== */

/**
 * Canonical model declaration.
 *
 * Example:
 *
 *     @model LinearModel<T> {
 *         @input features: Tensor<T>;
 *         @output prediction: Tensor<T>;
 *     }
 *
 * The exact meaning of `@model` is validated by semantic analysis.
 */
aiModelDeclaration
    : modelAnnotation
      identifier
      genericParameters?
      modelExtendsClause?
      modelImplementsClause?
      modelBody
    ;


/**
 * The lexical token contains the complete annotation text.
 *
 * Semantic analysis MUST require the normalized annotation text to be
 * exactly `@model` for this rule to represent a model declaration.
 */
modelAnnotation
    : NANO_ANNOTATION
    ;


modelExtendsClause
    : EXTENDS
      qualifiedModelNameList
    ;


modelImplementsClause
    : IMPLEMENTS
      qualifiedModelNameList
    ;


qualifiedModelNameList
    : qualifiedModelName
      (COMMA qualifiedModelName)*
    ;


qualifiedModelName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 3. MODEL BODY
 * ========================================================================== */

modelBody
    : LBRACE
      modelMember*
      RBRACE
    ;


modelMember
    : modelInput
    | modelOutput
    | modelParameter
    | modelState
    | modelComponent
    | modelLayer
    | modelSubmodel
    | modelConnection
    | modelBinding
    | modelConfiguration
    | modelMetadata
    | modelRequirement
    | modelCapability
    | modelConstraint
    | modelPreference
    | modelOperation
    ;


/* ============================================================================
 * 4. MODEL INPUTS
 * ========================================================================== */

/**
 * Example:
 *
 *     @input features: Tensor<Float>;
 */
modelInput
    : roleAnnotation(MODEL_INPUT_ROLE)
      identifier
      modelPortType
      modelInitializer?
      SEMICOLON
    ;


modelPortType
    : COLON
      typeExpression
    ;


modelInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 5. MODEL OUTPUTS
 * ========================================================================== */

/**
 * Example:
 *
 *     @output prediction: Tensor<Float>;
 */
modelOutput
    : roleAnnotation(MODEL_OUTPUT_ROLE)
      identifier
      modelPortType
      SEMICOLON
    ;


/* ============================================================================
 * 6. MODEL PARAMETERS
 * ========================================================================== */

/**
 * Trainable or semantic model parameters.
 *
 * Example:
 *
 *     @parameter weights: Tensor<Float>;
 *
 * The grammar does not impose a finite parameter count.
 */
modelParameter
    : roleAnnotation(MODEL_PARAMETER_ROLE)
      identifier
      modelPortType
      modelInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 7. MODEL STATE
 * ========================================================================== */

/**
 * Persistent or execution state.
 *
 * Example:
 *
 *     @state running_mean: Tensor<Float>;
 */
modelState
    : roleAnnotation(MODEL_STATE_ROLE)
      identifier
      modelPortType
      modelInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 8. MODEL COMPONENTS
 * ========================================================================== */

/**
 * A component is a named semantic unit inside the model.
 *
 * Example:
 *
 *     @component encoder: Encoder;
 */
modelComponent
    : roleAnnotation(MODEL_COMPONENT_ROLE)
      identifier
      modelPortType
      modelInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 9. MODEL LAYERS
 * ========================================================================== */

/**
 * A layer is intentionally represented by a type/value expression rather than
 * an exhaustive list of neural-network layer kinds.
 *
 * This permits:
 *
 *     Dense
 *     Convolution
 *     Transformer
 *     GraphLayer
 *     QuantumLayer
 *     custom user layer
 *     future layer kinds
 *
 * without grammar changes.
 */
modelLayer
    : roleAnnotation(MODEL_LAYER_ROLE)
      identifier
      modelPortType
      modelInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 10. SUBMODELS
 * ========================================================================== */

/**
 * Nested model composition.
 *
 * Example:
 *
 *     @submodel encoder: EncoderModel;
 */
modelSubmodel
    : roleAnnotation(MODEL_SUBMODEL_ROLE)
      identifier
      modelPortType
      modelInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 11. CONNECTIONS
 * ========================================================================== */

/**
 * Semantic model graph edge.
 *
 * Example:
 *
 *     @connect encoder.output -> classifier.input;
 *
 * This is a model dependency, NOT a physical hardware connection.
 */
modelConnection
    : roleAnnotation(MODEL_CONNECTION_ROLE)
      modelEndpoint
      ARROW
      modelEndpoint
      SEMICOLON
    ;


modelEndpoint
    : qualifiedModelReference
    ;


qualifiedModelReference
    : identifier
      (DOT identifier)*
    ;


/* ============================================================================
 * 12. MODEL BINDINGS
 * ========================================================================== */

/**
 * Binds a named model member to an expression.
 *
 * Example:
 *
 *     @bind activation = relu;
 */
modelBinding
    : roleAnnotation(MODEL_BINDING_ROLE)
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. MODEL CONFIGURATION
 * ========================================================================== */

/**
 * Configuration remains expression-based.
 *
 * Example:
 *
 *     @config precision = "bf16";
 *
 * The grammar does not enumerate precision names.
 */
modelConfiguration
    : roleAnnotation(MODEL_CONFIGURATION_ROLE)
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. MODEL METADATA
 * ========================================================================== */

/**
 * Metadata is intentionally open-ended.
 *
 * Example:
 *
 *     @metadata family = "transformer";
 *     @metadata version = "1";
 */
modelMetadata
    : roleAnnotation(MODEL_METADATA_ROLE)
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 15. RESOURCE REQUIREMENTS
 * ========================================================================== */

/**
 * A model requirement describes something the implementation needs.
 *
 * Example:
 *
 *     @requires memory = required_memory;
 *
 * It does NOT allocate or select a physical resource.
 */
modelRequirement
    : roleAnnotation(MODEL_REQUIREMENT_ROLE)
      identifier
      modelRequirementOperator
      expression
      SEMICOLON
    ;


modelRequirementOperator
    : ASSIGN
    | COLON
    ;


/* ============================================================================
 * 16. CAPABILITIES
 * ========================================================================== */

/**
 * A capability declaration expresses semantic capability requirements.
 *
 * Example:
 *
 *     @capability tensor_compute = true;
 *
 * It does not select a particular accelerator.
 */
modelCapability
    : roleAnnotation(MODEL_CAPABILITY_ROLE)
      identifier
      modelValueAssignment
      SEMICOLON
    ;


modelValueAssignment
    : ASSIGN
      expression
    | COLON
      expression
    ;


/* ============================================================================
 * 17. CONSTRAINTS
 * ========================================================================== */

/**
 * Constraints describe semantic restrictions.
 *
 * Example:
 *
 *     @constraint latency = maximum_latency;
 */
modelConstraint
    : roleAnnotation(MODEL_CONSTRAINT_ROLE)
      identifier
      modelValueAssignment
      SEMICOLON
    ;


/* ============================================================================
 * 18. PREFERENCES
 * ========================================================================== */

/**
 * Preferences are non-mandatory implementation guidance.
 *
 * They MUST NOT silently become hard requirements.
 */
modelPreference
    : roleAnnotation(MODEL_PREFERENCE_ROLE)
      identifier
      modelValueAssignment
      SEMICOLON
    ;


/* ============================================================================
 * 19. MODEL OPERATIONS
 * ========================================================================== */

/**
 * Model operations provide an extensible semantic operation boundary.
 *
 * Examples include:
 *
 *     @operation forward = expression;
 *     @operation loss = expression;
 *     @operation predict = expression;
 *     @operation train = expression;
 *     @operation evaluate = expression;
 *
 * The grammar intentionally does not enumerate algorithms.
 */
modelOperation
    : roleAnnotation(MODEL_OPERATION_ROLE)
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 20. MODEL REFERENCES
 * ========================================================================== */

/**
 * A model reference is a normal Zamani name.
 *
 * No fixed model registry is encoded here.
 */
aiModelReference
    : qualifiedModelReference
    ;


/* ============================================================================
 * 21. MODEL INSTANTIATION
 * ========================================================================== */

/**
 * Model construction is expression-driven.
 *
 * Example:
 *
 *     ModelName<T>(configuration)
 *
 * The expression grammar remains authoritative for calls and arguments.
 *
 * This rule intentionally accepts a qualified model name followed by a
 * parenthesized expression list.
 */
aiModelInstantiation
    : qualifiedModelReference
      genericArguments?
      LPAREN
      modelArgumentList?
      RPAREN
    ;


modelArgumentList
    : expression
      (COMMA expression)*
    ;


/* ============================================================================
 * 22. MODEL COMPOSITION
 * ========================================================================== */

/**
 * Composition is deliberately represented as a list of model references and
 * expressions rather than a framework-specific composition language.
 */
aiModelComposition
    : roleAnnotation(MODEL_COMPOSE_ROLE)
      identifier
      ASSIGN
      modelCompositionExpression
      SEMICOLON
    ;


modelCompositionExpression
    : expression
    ;


/* ============================================================================
 * 23. GENERIC ARGUMENTS
 * ========================================================================== */

/**
 * Model generic arguments.
 *
 * This grammar intentionally uses expressions so model configuration can be
 * value-parameterized without introducing machine-specific limits.
 */
genericArguments
    : LT
      expression
      (COMMA expression)*
      GT
    ;


/* ============================================================================
 * 24. ROLE ANNOTATION
 * ========================================================================== */

/*
 * These parser-level symbolic aliases document the semantic roles expected
 * from NANO_ANNOTATION.
 *
 * They are represented by the existing NANO_ANNOTATION token rather than new
 * lexer keywords.
 *
 * The parser cannot safely compare token text without target-specific
 * semantic predicates. Therefore semantic analysis MUST validate the
 * annotation text.
 *
 * The aliases below exist as grammar documentation and stable integration
 * boundaries.
 *
 * The `roleAnnotation(...)` parameterized form is intentionally NOT used as
 * an ANTLR semantic predicate. The actual parser representation is the
 * generic modelRoleAnnotation rule below.
 */

modelRoleAnnotation
    : NANO_ANNOTATION
    ;


roleAnnotation
    : modelRoleAnnotation
    ;


/* ============================================================================
 * 25. MODEL ROLE IDENTIFIERS
 * ========================================================================== */

/*
 * Canonical semantic role strings:
 *
 *     @model
 *     @input
 *     @output
 *     @parameter
 *     @state
 *     @component
 *     @layer
 *     @submodel
 *     @connect
 *     @bind
 *     @config
 *     @metadata
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *     @operation
 *     @compose
 *
 * These are NOT lexer keywords.
 *
 * Semantic validation owns the exact spelling and role interpretation.
 *
 * The following rules are kept as stable named boundaries for AST construction
 * and future parser integration.
 */

modelDeclarationAnnotation
    : modelRoleAnnotation
    ;


modelInputAnnotation
    : modelRoleAnnotation
    ;


modelOutputAnnotation
    : modelRoleAnnotation
    ;


modelParameterAnnotation
    : modelRoleAnnotation
    ;


modelStateAnnotation
    : modelRoleAnnotation
    ;


modelComponentAnnotation
    : modelRoleAnnotation
    ;


modelLayerAnnotation
    : modelRoleAnnotation
    ;


modelSubmodelAnnotation
    : modelRoleAnnotation
    ;


modelConnectionAnnotation
    : modelRoleAnnotation
    ;


modelBindingAnnotation
    : modelRoleAnnotation
    ;


modelConfigurationAnnotation
    : modelRoleAnnotation
    ;


modelMetadataAnnotation
    : modelRoleAnnotation
    ;


modelRequirementAnnotation
    : modelRoleAnnotation
    ;


modelCapabilityAnnotation
    : modelRoleAnnotation
    ;


modelConstraintAnnotation
    : modelRoleAnnotation
    ;


modelPreferenceAnnotation
    : modelRoleAnnotation
    ;


modelOperationAnnotation
    : modelRoleAnnotation
    ;


modelCompositionAnnotation
    : modelRoleAnnotation
    ;


/* ============================================================================
 * 26. IDENTIFIER ADAPTER
 * ========================================================================== */

/**
 * Existing canonical name grammar owns identifier syntax.
 *
 * This adapter keeps models.g4 independent of lexer-level identifier details.
 */
modelIdentifier
    : identifier
    ;


/* ============================================================================
 * 27. MODEL QUALIFIED NAME
 * ========================================================================== */

modelQualifiedName
    : modelIdentifier
      (DOUBLE_COLON modelIdentifier)*
    ;


/* ============================================================================
 * 28. MODEL TYPE REFERENCE
 * ========================================================================== */

modelTypeReference
    : typeExpression
    ;


/* ============================================================================
 * 29. MODEL EXPRESSION REFERENCE
 * ========================================================================== */

modelExpression
    : expression
    ;


/* ============================================================================
 * 30. MODEL PORT
 * ========================================================================== */

/**
 * Shared port abstraction used by input/output declarations.
 */
modelPort
    : modelIdentifier
      modelPortType
    ;


/* ============================================================================
 * 31. MODEL DECLARATION VALUE
 * ========================================================================== */

modelDeclarationValue
    : expression
    ;


/* ============================================================================
 * 32. MODEL SEMANTIC EXTENSION POINT
 * ========================================================================== */

/**
 * Future AI dialects can attach additional annotated members without changing
 * the core model representation.
 *
 * Dialect grammars should depend on this boundary rather than modifying the
 * canonical model graph.
 */
modelExtensionMember
    : modelRoleAnnotation
      identifier
      modelExtensionPayload?
      SEMICOLON
    ;


modelExtensionPayload
    : ASSIGN expression
    | COLON expression
    | LPAREN modelArgumentList? RPAREN
    ;


/* ============================================================================
 * 33. MODEL RESOURCE EXTENSION
 * ========================================================================== */

/**
 * Resource information remains declarative.
 *
 * No physical allocation occurs here.
 */
modelResourceExpression
    : expression
    ;


/* ============================================================================
 * 34. MODEL CAPABILITY EXTENSION
 * ========================================================================== */

modelCapabilityExpression
    : expression
    ;


/* ============================================================================
 * 35. MODEL CONSTRAINT EXTENSION
 * ========================================================================== */

modelConstraintExpression
    : expression
    ;


/* ============================================================================
 * 36. MODEL PORTABILITY EXTENSION
 * ========================================================================== */

/**
 * Portability requirements remain expressions so future target classes can be
 * introduced without grammar rewrites.
 */
modelPortabilityExpression
    : expression
    ;


/* ============================================================================
 * 37. MODEL VALIDATION BOUNDARY
 * ========================================================================== */

/**
 * Parser-level model structure.
 *
 * All semantic validation belongs downstream.
 */
modelValidationInput
    : aiModelDeclaration
    ;


/* ============================================================================
 * 38. MODEL AST CONTRACT
 * ========================================================================== */

/*
 * The frontend AST should preserve, at minimum:
 *
 *     - source span;
 *     - declaration name;
 *     - generic parameters;
 *     - inheritance references;
 *     - interface references;
 *     - ordered members;
 *     - member role;
 *     - member name;
 *     - declared type;
 *     - initializer expression;
 *     - graph endpoints;
 *     - resource/capability/constraint expressions;
 *     - annotations;
 *     - source provenance.
 *
 * models.g4 MUST NOT define the AST implementation.
 */


/* ============================================================================
 * 39. SEMANTIC BOUNDARY
 * ========================================================================== */

/*
 * Semantic analysis MUST:
 *
 *     1. Verify @model declaration annotations.
 *     2. Verify model member role annotations.
 *     3. Resolve model names.
 *     4. Resolve model types.
 *     5. Resolve expressions.
 *     6. Detect duplicate declarations.
 *     7. Validate graph endpoints.
 *     8. Validate type compatibility.
 *     9. Validate capability expressions.
 *    10. Validate resource requirements.
 *    11. Validate constraints.
 *    12. Preserve provenance.
 *    13. Reject backend-specific assumptions where prohibited.
 *    14. Produce semantic diagnostics.
 *
 * None of these operations belong in this parser grammar.
 */


/* ============================================================================
 * 40. IR BOUNDARY
 * ========================================================================== */

/*
 * models.g4 MUST NOT emit:
 *
 *     QuantumGate
 *     Qubit
 *     PhysicalQubit
 *     QuantumCircuit
 *     HardwareDevice
 *     Schedule
 *     ResourceAllocation
 *     Backend
 *     Calibration
 *     ZQN fault
 *     QEC object
 *     Classical instruction
 *
 * Instead:
 *
 *     parser
 *       ->
 *     AST
 *       ->
 *     semantic model
 *       ->
 *     canonical semantic IR
 *
 * AI-specific lowering may then produce the appropriate canonical IR forms.
 *
 * Quantum subcomputations MUST ultimately use `quantum::ir` rather than a
 * second quantum representation inside AI model syntax.
 */


/* ============================================================================
 * 41. RESOURCE / SCALABILITY GUARANTEE
 * ========================================================================== */

/*
 * No finite grammar constants exist in this file.
 *
 * In particular, this grammar contains no:
 *
 *     MAX_LAYERS
 *     MAX_PARAMETERS
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_COMPONENTS
 *     MAX_MODELS
 *     MAX_TENSORS
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_MEMORY
 *
 * Scale is therefore limited only by:
 *
 *     parser/runtime resources;
 *     compiler policy;
 *     available memory;
 *     available execution resources;
 *     target capabilities.
 */


/* ============================================================================
 * 42. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * This grammar:
 *
 *     - contains no actions;
 *     - contains no semantic predicates;
 *     - performs no I/O;
 *     - performs no runtime execution;
 *     - has no mutable global parser state;
 *     - does not inspect hardware;
 *     - does not inspect environment variables.
 *
 * Therefore identical token streams must produce equivalent parser structures.
 */


/* ============================================================================
 * 43. SECURITY CONTRACT
 * ========================================================================== */

/*
 * This grammar:
 *
 *     - cannot open files;
 *     - cannot access networks;
 *     - cannot execute processes;
 *     - cannot execute models;
 *     - cannot allocate hardware;
 *     - cannot invoke accelerators;
 *     - cannot access credentials;
 *     - cannot access device memory.
 *
 * All such operations belong downstream to explicitly authorized compiler or
 * runtime components.
 */


/* ============================================================================
 * 44. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Existing model implementations remain representable through:
 *
 *     model references;
 *     generic types;
 *     expressions;
 *     model members;
 *     annotations;
 *     composition.
 *
 * Framework-specific concepts should be represented by libraries/dialects
 * rather than permanently hard-coded into this grammar.
 *
 * Future model families therefore do not require changes to the core model
 * grammar merely because a new algorithm is invented.
 */


/* ============================================================================
 * 45. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when:
 *
 * [ ] It compiles as an ANTLR parser grammar against ZamaniLexer.
 *
 * [ ] Its imports resolve against the canonical Types and Expressions
 *     grammars.
 *
 * [ ] No lexer rules are duplicated here.
 *
 * [ ] No Rust actions are embedded.
 *
 * [ ] No semantic predicates are embedded.
 *
 * [ ] No unsafe implementation exists.
 *
 * [ ] Model declarations have a stable AST boundary.
 *
 * [ ] Model inputs are representable.
 *
 * [ ] Model outputs are representable.
 *
 * [ ] Parameters are representable.
 *
 * [ ] Model state is representable.
 *
 * [ ] Components are representable.
 *
 * [ ] Layers are representable.
 *
 * [ ] Submodels are representable.
 *
 * [ ] Graph connections are representable.
 *
 * [ ] Model bindings are representable.
 *
 * [ ] Metadata is representable.
 *
 * [ ] Requirements are representable.
 *
 * [ ] Capabilities are representable.
 *
 * [ ] Constraints are representable.
 *
 * [ ] Preferences are representable.
 *
 * [ ] Model operations are representable.
 *
 * [ ] Generic models are representable.
 *
 * [ ] Model instantiation is representable.
 *
 * [ ] Model composition is representable.
 *
 * [ ] No fixed model-size limit exists.
 *
 * [ ] No fixed tensor-size limit exists.
 *
 * [ ] No fixed accelerator limit exists.
 *
 * [ ] No fixed device count exists.
 *
 * [ ] No hardware-specific assumption exists.
 *
 * [ ] No duplicate type system exists.
 *
 * [ ] No duplicate expression system exists.
 *
 * [ ] No duplicate quantum IR exists.
 *
 * [ ] Cross-domain lowering remains downstream.
 *
 * [ ] Source provenance can be retained by the AST.
 *
 * [ ] Positive tests exist for every model member.
 *
 * [ ] Negative tests exist for malformed model declarations.
 *
 * [ ] Boundary tests exist for empty and very large model structures.
 *
 * [ ] Cross-domain tests cover classical, quantum, hardware and distributed
 *     model references.
 *
 * [ ] Round-trip tests preserve model structure.
 */