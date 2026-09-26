/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/models.g4
 *
 * Grammar:
 *     AIModels
 *
 * Role:
 *     Canonical source-level grammar for AI/ML model declarations and
 *     model-local structure.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the STRUCTURAL SOURCE SYNTAX of AI/ML model
 * declarations.
 *
 * It deliberately describes model intent rather than:
 *
 *     - a machine-learning framework;
 *     - a numerical implementation;
 *     - a CPU;
 *     - a GPU;
 *     - a TPU;
 *     - an NPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a vendor;
 *     - a runtime;
 *     - a scheduler;
 *     - a physical device;
 *     - a fixed memory capacity;
 *     - a fixed tensor capacity;
 *     - a fixed model size.
 *
 * The model grammar is therefore compatible with:
 *
 *     classical computation
 *     quantum computation
 *     HDL/hardware co-design
 *     distributed computation
 *     accelerator computation
 *     future computing domains
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
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
 *     AIModels
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type analysis
 *          +--> name resolution
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> effect analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical lowering
 *          +--> quantum lowering
 *          +--> hardware/HDL lowering
 *          +--> distributed lowering
 *          +--> accelerator lowering
 *          |
 *          v
 *     canonical/domain IR
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC where applicable
 *          +--> ZQN where applicable
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime / deployment
 *
 * This file MUST NOT construct or define an IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - model declaration structure;
 *     - model generic declaration parameters;
 *     - model inheritance/interface references as source syntax;
 *     - model body structure;
 *     - model member boundaries;
 *     - model ports;
 *     - model parameters;
 *     - model state;
 *     - model components;
 *     - model layers;
 *     - model submodels;
 *     - model graph connections;
 *     - model bindings;
 *     - model configuration;
 *     - model metadata;
 *     - model resource/capability/constraint/preference annotations;
 *     - model-local operation declarations;
 *     - model-local extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifier syntax;
 *     - general type syntax;
 *     - general expression syntax;
 *     - tensor implementation;
 *     - tensor storage;
 *     - dataset implementation;
 *     - training algorithms;
 *     - inference algorithms;
 *     - differentiation;
 *     - optimizer algorithms;
 *     - accelerator selection;
 *     - hardware discovery;
 *     - resource allocation;
 *     - distributed placement;
 *     - routing;
 *     - scheduling;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - calibration.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General types are owned by:
 *
 *     grammar/types/types.g4
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * The model grammar MUST reuse those rules.
 *
 * It MUST NOT define a second:
 *
 *     typeExpression
 *     expression
 *     identifier
 *     generic type system
 *     operator hierarchy
 *     literal system
 *
 * ============================================================================
 * LEXER POLICY
 * ============================================================================
 *
 * AI vocabulary is intentionally NOT converted into a closed set of lexer
 * keywords.
 *
 * The canonical lexer supplies:
 *
 *     NANO_ANNOTATION
 *
 * which provides the source-level annotation boundary:
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
 * These spellings are semantic roles, not independent parser tokens.
 *
 * The parser therefore MUST NOT attempt to compare annotation text using
 * semantic predicates or target-specific actions.
 *
 * Semantic analysis is responsible for:
 *
 *     annotation text
 *         |
 *         v
 *     normalized annotation name
 *         |
 *         v
 *     registered AI model role
 *
 * This keeps the lexer extensible and avoids framework/vendor lock-in.
 *
 * ============================================================================
 * IMPORTANT CORRECTION
 * ============================================================================
 *
 * The previous implementation used constructs such as:
 *
 *     roleAnnotation(MODEL_INPUT_ROLE)
 *
 * even though `roleAnnotation` was not defined as a parameterized parser rule
 * and the role constants were not canonical lexer tokens.
 *
 * That design is removed.
 *
 * The production representation is:
 *
 *     NANO_ANNOTATION
 *
 * followed by a structurally distinct payload.
 *
 * Semantic analysis validates whether the annotation is legal in the current
 * model context.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Model source describes:
 *
 *     WHAT the model is;
 *     WHAT it consumes;
 *     WHAT it produces;
 *     WHAT state it owns;
 *     WHAT computation it represents;
 *     WHAT dependencies exist;
 *     WHAT capabilities are required;
 *     WHAT resources are required or preferred;
 *     WHAT constraints apply.
 *
 * Model source MUST NOT silently select:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     physical qubit 17
 *     fixed VRAM
 *     fixed RAM
 *     fixed core count
 *     fixed worker count
 *     fixed cluster size
 *     fixed network topology
 *     fixed tensor rank
 *     fixed tensor dimension
 *
 * Such decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No language-level maximum is imposed for:
 *
 *     model inputs
 *     model outputs
 *     parameters
 *     states
 *     components
 *     layers
 *     submodels
 *     connections
 *     annotations
 *     generic parameters
 *     model members
 *     model nesting
 *     graph edges
 *     model operations
 *
 * Repetition is structural.
 *
 * Practical limits may arise from:
 *
 *     available memory;
 *     parser implementation;
 *     compiler resources;
 *     runtime resources;
 *     target resources;
 *
 * but such implementation/resource limits MUST NOT become source-language
 * semantic ceilings.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST is domain-neutral.
 *
 * The model grammar should map structurally to an existing/general AST
 * representation rather than introducing a model-specific parallel AST.
 *
 * The resulting model declaration must preserve:
 *
 *     - source span;
 *     - declaration name;
 *     - generic parameters;
 *     - model references;
 *     - member order;
 *     - annotation text;
 *     - member names;
 *     - types;
 *     - expressions;
 *     - graph endpoints;
 *     - nested members;
 *     - source provenance.
 *
 * Semantic model classification happens after parsing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - validating @model;
 *     - validating model member annotation roles;
 *     - resolving names;
 *     - resolving types;
 *     - resolving expressions;
 *     - validating generic parameters;
 *     - validating inheritance/interface relationships;
 *     - checking duplicate declarations;
 *     - checking graph endpoints;
 *     - checking type compatibility;
 *     - checking resource requirements;
 *     - checking capabilities;
 *     - checking constraints;
 *     - checking portability;
 *     - checking framework/dialect registration;
 *     - checking model graph legality;
 *     - checking model operation semantics.
 *
 * The parser performs none of these semantic decisions.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Model resource declarations express intent.
 *
 * Examples:
 *
 *     @requires memory >= required_memory;
 *     @capability tensor.compute;
 *     @constraint latency <= latency_budget;
 *     @preference accelerator.tensor;
 *
 * These MUST NOT mean:
 *
 *     use_gpu_0
 *     use_cpu_3
 *     use_device_7
 *
 * Resource discovery, selection, placement and scheduling are downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * AI models may reference quantum computation through:
 *
 *     types;
 *     expressions;
 *     model components;
 *     model operations;
 *     capabilities;
 *     resource requirements.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId;
 *     physical qubits;
 *     quantum gates;
 *     quantum topology;
 *     QEC;
 *     ZQN;
 *     quantum scheduling.
 *
 * Quantum semantics ultimately cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second AI-specific quantum IR is permitted.
 *
 * ============================================================================
 * HARDWARE / HDL CONTRACT
 * ============================================================================
 *
 * AI model structure may refer to hardware-oriented capabilities or model
 * components, but physical realization belongs to:
 *
 *     hardware/
 *     hdl/
 *     resources/
 *     compile/
 *     execution/
 *
 * The model grammar does not encode:
 *
 *     register width limits;
 *     memory-bank counts;
 *     accelerator counts;
 *     FPGA resources;
 *     ASIC technology;
 *     physical wiring;
 *     device IDs.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no runtime callbacks;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware inspection;
 *     - no random behavior.
 *
 * Parsing is deterministic for a fixed token stream.
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
 * Public model-domain boundary.
 *
 * A model declaration is the only construct that this rule should be used to
 * recognize as a complete model definition.
 *
 * Model references and model invocations are ordinary expressions and MUST
 * remain owned by Expressions rather than being admitted here as arbitrary
 * identifiers.
 */
aiModelConstruct
    : aiModelDeclaration
    ;


/* ============================================================================
 * 2. MODEL DECLARATION
 * ========================================================================== */

/**
 * Canonical source shape:
 *
 *     @model MyModel {
 *         ...
 *     }
 *
 * The parser accepts NANO_ANNOTATION structurally.
 *
 * Semantic analysis MUST require the annotation's normalized spelling to be
 * the registered model declaration annotation for this rule.
 */
aiModelDeclaration
    : modelDeclarationAnnotation
      identifier
      modelGenericParameters?
      modelInheritanceClause?
      modelInterfaceClause?
      modelBody
    ;


/**
 * Model declaration annotation.
 *
 * Semantic meaning:
 *
 *     @model
 *
 * The parser intentionally does not compare token text.
 */
modelDeclarationAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 3. MODEL GENERIC PARAMETERS
 * ========================================================================== */

/**
 * Model declaration generics are declaration parameters.
 *
 * They are NOT type generic arguments.
 *
 * Type generic arguments remain owned by:
 *
 *     grammar/types/types.g4
 *
 * Example:
 *
 *     @model NeuralModel<T, Shape> {
 *         ...
 *     }
 *
 * Generic parameter count is unbounded by grammar.
 */
modelGenericParameters
    : LESS_THAN
      modelGenericParameterList
      GREATER_THAN
    ;


modelGenericParameterList
    : modelGenericParameter
      (COMMA modelGenericParameter)*
      COMMA?
    ;


modelGenericParameter
    : identifier
      modelGenericParameterBound?
    ;


modelGenericParameterBound
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 4. MODEL INHERITANCE / INTERFACES
 * ========================================================================== */

/**
 * These clauses remain source-level relationships.
 *
 * They do not perform type or trait resolution.
 *
 * The annotations/semantic registry may choose to use:
 *
 *     extends
 *     implements
 *
 * as registered model language concepts.
 *
 * The clause keywords are represented structurally as annotations so that the
 * lexer does not need to become an ever-growing AI keyword registry.
 *
 * Example conceptual forms:
 *
 *     @extends BaseModel
 *     @implements TrainableModel
 *
 * Semantic analysis determines whether the relationship is legal.
 */
modelInheritanceClause
    : modelRelationshipClause
    ;


modelInterfaceClause
    : modelRelationshipClause
    ;


modelRelationshipClause
    : NANO_ANNOTATION
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
 * 5. MODEL BODY
 * ========================================================================== */

/**
 * Empty models are structurally legal.
 *
 * Semantic analysis determines whether a particular model dialect requires
 * inputs, outputs or other members.
 */
modelBody
    : LBRACE
      modelMember*
      RBRACE
    ;


/* ============================================================================
 * 6. MODEL MEMBER
 * ========================================================================== */

/**
 * Every model member begins with a NANO_ANNOTATION.
 *
 * The annotation is semantically classified downstream.
 *
 * The payload is structurally classified by punctuation, not annotation text.
 *
 * This avoids:
 *
 *     - semantic predicates;
 *     - duplicated role rules;
 *     - fake lexer tokens;
 *     - parameterized parser rules;
 *     - arbitrary fixed AI keyword lists.
 */
modelMember
    : NANO_ANNOTATION
      modelMemberPayload
    ;


/**
 * Model member payloads are separated by their source structure.
 *
 * The alternatives intentionally use different structural signatures where
 * possible:
 *
 *     qualifiedName -> qualifiedName
 *     identifier : type
 *     identifier = expression
 *     identifier(...)
 *     identifier { ... }
 *     expression ;
 *
 * Semantic analysis maps the annotation to the actual model role.
 */
modelMemberPayload
    : modelConnectionPayload
    | modelTypedMemberPayload
    | modelAssignmentMemberPayload
    | modelInvocationMemberPayload
    | modelBlockMemberPayload
    | modelExpressionMemberPayload
    ;


/* ============================================================================
 * 7. MODEL CONNECTION
 * ========================================================================== */

/**
 * Example:
 *
 *     @connect encoder.output -> classifier.input;
 *
 * The connection is a semantic model-graph edge.
 *
 * It is NOT:
 *
 *     - a network connection;
 *     - a hardware wire;
 *     - a GPU interconnect;
 *     - a quantum coupling;
 *     - a physical route.
 */
modelConnectionPayload
    : modelEndpoint
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
 * 8. TYPED MODEL MEMBERS
 * ========================================================================== */

/**
 * Examples:
 *
 *     @input features: Tensor<Float>;
 *     @output prediction: Tensor<Float>;
 *     @parameter weights: Tensor<Float>;
 *     @state running_mean: Tensor<Float>;
 *     @component encoder: Encoder;
 *     @layer attention: Attention;
 *     @submodel encoder: EncoderModel;
 *
 * The annotation determines the semantic role.
 */
modelTypedMemberPayload
    : identifier
      COLON
      typeExpression
      modelMemberInitializer?
      SEMICOLON
    ;


modelMemberInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 9. ASSIGNMENT MODEL MEMBERS
 * ========================================================================== */

/**
 * Examples:
 *
 *     @bind activation = relu;
 *     @config precision = "bf16";
 *     @metadata family = "transformer";
 *     @operation forward = forward_function;
 *     @layer encoder = Encoder(...);
 *
 * The annotation determines the semantic role.
 */
modelAssignmentMemberPayload
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 10. INVOCATION MODEL MEMBERS
 * ========================================================================== */

/**
 * This provides a model-local extensibility boundary for annotated
 * operations.
 *
 * Example:
 *
 *     @operation forward(x);
 *
 * The invocation is syntax only.
 *
 * Algorithm selection and execution belong downstream.
 */
modelInvocationMemberPayload
    : identifier
      LPAREN
      modelArgumentList?
      RPAREN
      SEMICOLON?
    ;


modelArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 11. BLOCK MODEL MEMBERS
 * ========================================================================== */

/**
 * Examples:
 *
 *     @component encoder {
 *         ...
 *     }
 *
 *     @operation train {
 *         ...
 *     }
 *
 *     @pipeline body {
 *         ...
 *     }
 *
 * The annotation determines whether the block is legal.
 */
modelBlockMemberPayload
    : identifier
      modelBody
    ;


/* ============================================================================
 * 12. GENERAL MODEL EXPRESSION MEMBERS
 * ========================================================================== */

/**
 * This is the open semantic extension mechanism.
 *
 * Examples:
 *
 *     @requires memory >= required_memory;
 *     @capability tensor.compute;
 *     @constraint latency <= latency_budget;
 *     @preference accelerator.tensor;
 *
 * These are ordinary expressions after the annotation.
 *
 * Resource/capability semantics are validated downstream by the appropriate
 * resource and capability subsystems.
 */
modelExpressionMemberPayload
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. MODEL EXTENSION MEMBER
 * ========================================================================== */

/**
 * Explicit helper boundary for dialects and future AI features.
 *
 * Dialects MUST NOT modify the meaning of existing model members silently.
 *
 * A dialect may register additional annotation semantics and consume the
 * existing structural payload.
 */
modelExtensionMember
    : NANO_ANNOTATION
      modelExtensionPayload
    ;


modelExtensionPayload
    : modelTypedMemberPayload
    | modelAssignmentMemberPayload
    | modelInvocationMemberPayload
    | modelBlockMemberPayload
    | modelExpressionMemberPayload
    ;


/* ============================================================================
 * 14. MODEL PORT
 * ========================================================================== */

/**
 * Shared source-level port representation.
 *
 * The semantic role is supplied by the containing annotation:
 *
 *     @input
 *     @output
 */
modelPort
    : identifier
      COLON
      typeExpression
      modelMemberInitializer?
    ;


/* ============================================================================
 * 15. MODEL REFERENCE
 * ========================================================================== */

/**
 * Model references are deliberately NOT part of aiModelConstruct.
 *
 * They are ordinary expressions/name references and therefore belong to the
 * canonical expression grammar.
 *
 * This avoids the previous ambiguity where every arbitrary qualified name
 * could be interpreted as an AI model construct.
 */
modelReference
    : qualifiedModelReference
    ;


/* ============================================================================
 * 16. MODEL TYPE REFERENCE
 * ========================================================================== */

/**
 * Model type references reuse the canonical type system.
 */
modelTypeReference
    : typeExpression
    ;


/* ============================================================================
 * 17. MODEL EXPRESSION REFERENCE
 * ========================================================================== */

/**
 * Model expressions reuse the canonical expression hierarchy.
 */
modelExpression
    : expression
    ;


/* ============================================================================
 * 18. MODEL CONFIGURATION
 * ========================================================================== */

/**
 * Configuration values are ordinary Zamani expressions.
 *
 * No framework-specific configuration syntax is introduced here.
 */
modelConfigurationValue
    : expression
    ;


/* ============================================================================
 * 19. MODEL METADATA
 * ========================================================================== */

/**
 * Metadata is intentionally open-ended.
 *
 * Metadata names and values are interpreted semantically.
 */
modelMetadataValue
    : expression
    ;


/* ============================================================================
 * 20. RESOURCE REQUIREMENT BOUNDARY
 * ========================================================================== */

/**
 * Model requirements are expressions after their annotation.
 *
 * Example:
 *
 *     @requires memory >= required_memory;
 *
 * The resource subsystem owns the semantic interpretation.
 */
modelRequirement
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. CAPABILITY BOUNDARY
 * ========================================================================== */

/**
 * Example:
 *
 *     @capability tensor.compute;
 *     @capability quantum.measurement;
 *
 * The capability subsystem owns the semantic interpretation.
 */
modelCapability
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 22. CONSTRAINT BOUNDARY
 * ========================================================================== */

/**
 * Example:
 *
 *     @constraint latency <= latency_budget;
 *
 * Constraint semantics belong to the resource/semantic subsystem.
 */
modelConstraint
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 23. PREFERENCE BOUNDARY
 * ========================================================================== */

/**
 * Preferences are advisory.
 *
 * They MUST NOT become requirements merely because a backend cannot satisfy
 * them.
 */
modelPreference
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 24. MODEL OPERATION BOUNDARY
 * ========================================================================== */

/**
 * Model operations intentionally remain generic.
 *
 * Examples:
 *
 *     @operation forward = expression;
 *     @operation train = expression;
 *     @operation evaluate = expression;
 *
 * The grammar does not enumerate:
 *
 *     SGD
 *     Adam
 *     Transformer
 *     CNN
 *     RNN
 *     SVM
 *     quantum neural network
 *     vendor algorithm
 *
 * Those are semantic/library concepts.
 */
modelOperation
    : NANO_ANNOTATION
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 25. MODEL COMPOSITION
 * ========================================================================== */

/**
 * Model composition remains an annotated semantic operation rather than a
 * second composition language.
 *
 * Example:
 *
 *     @compose classifier = compose(encoder, head);
 *
 * The right-hand side is an ordinary Zamani expression.
 */
modelComposition
    : NANO_ANNOTATION
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 26. MODEL RESOURCE EXPRESSION
 * ========================================================================== */

/**
 * Resource expressions are ordinary expressions.
 *
 * This keeps quantity semantics and units under the canonical type/expression
 * systems and allows symbolic values to scale without grammar limits.
 */
modelResourceExpression
    : expression
    ;


/* ============================================================================
 * 27. MODEL CAPABILITY EXPRESSION
 * ========================================================================== */

modelCapabilityExpression
    : expression
    ;


/* ============================================================================
 * 28. MODEL CONSTRAINT EXPRESSION
 * ========================================================================== */

modelConstraintExpression
    : expression
    ;


/* ============================================================================
 * 29. MODEL PORTABILITY EXPRESSION
 * ========================================================================== */

/**
 * Portability remains an expression-level semantic contract.
 *
 * It does not select a concrete machine.
 */
modelPortabilityExpression
    : expression
    ;


/* ============================================================================
 * 30. MODEL VALIDATION BOUNDARY
 * ========================================================================== */

/**
 * A parser-valid model is not necessarily a semantically valid model.
 *
 * Downstream validation must check:
 *
 *     - @model annotation identity;
 *     - annotation legality;
 *     - duplicate names;
 *     - duplicate ports;
 *     - duplicate parameters;
 *     - duplicate state;
 *     - type validity;
 *     - expression validity;
 *     - generic parameter validity;
 *     - inheritance/interface validity;
 *     - graph endpoint validity;
 *     - graph directionality;
 *     - cycles where prohibited;
 *     - resource requirement validity;
 *     - capability validity;
 *     - constraint consistency;
 *     - portability requirements;
 *     - dialect registration;
 *     - source provenance.
 */
modelValidationInput
    : aiModelDeclaration
    ;


/* ============================================================================
 * 31. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve, at minimum:
 *
 *     model:
 *         source_span
 *         annotation
 *         name
 *         generic_parameters
 *         relationships
 *         ordered_members
 *
 *     member:
 *         source_span
 *         annotation
 *         structural_kind
 *         name where present
 *         type where present
 *         expression where present
 *         endpoints where present
 *         nested_members where present
 *
 * No model-specific second AST is permitted when the domain-neutral frontend
 * AST can represent the structure.
 *
 * ============================================================================
 * 32. SEMANTIC LOWERING CONTRACT
 * ============================================================================
 *
 * The required direction is:
 *
 *     models.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     model semantic analysis
 *          |
 *          +--> types
 *          +--> expressions
 *          +--> resources
 *          +--> capabilities
 *          +--> effects
 *          +--> portability
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical
 *          +--> quantum
 *          +--> HDL/hardware
 *          +--> distributed
 *          +--> accelerator
 *          |
 *          v
 *     canonical/domain IR
 *
 * models.g4 MUST NOT bypass the frontend AST.
 *
 * ============================================================================
 * 33. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If a model contains quantum computation:
 *
 *     AI model syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * There must be no:
 *
 *     AIQuantumIR
 *     ModelQuantumIR
 *     ModelGateIR
 *     PhysicalQubitIR
 *
 * introduced by this grammar.
 *
 * ============================================================================
 * 34. CLASSICAL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Classical operations lower through the canonical classical semantic/IR
 * pipeline.
 *
 * Hardware/HDL intent crosses into:
 *
 *     hardware/
 *     hdl/
 *     resources/
 *     compile/
 *     execution/
 *
 * This grammar does not select a physical realization.
 *
 * ============================================================================
 * 35. RESOURCE SEPARATION
 * ============================================================================
 *
 * The following distinctions MUST remain semantic:
 *
 *     requirement != capability
 *     capability != resource identity
 *     resource identity != physical device
 *     preference != requirement
 *     hint != requirement
 *     semantic resource != allocation
 *     allocation != scheduling
 *     scheduling != routing
 *
 * Example:
 *
 *     @requires memory >= required_memory;
 *
 * does NOT mean:
 *
 *     allocate RAM bank X
 *
 * Example:
 *
 *     @capability tensor.compute;
 *
 * does NOT mean:
 *
 *     use GPU 0
 *
 * Example:
 *
 *     @capability quantum.measurement;
 *
 * does NOT mean:
 *
 *     use physical qubit N
 *
 * ============================================================================
 * 36. SCALABILITY AUDIT
 * ============================================================================
 *
 * The grammar contains no explicit finite maximum for:
 *
 *     models
 *     members
 *     inputs
 *     outputs
 *     parameters
 *     layers
 *     components
 *     submodels
 *     connections
 *     graph depth
 *     graph width
 *     generic parameters
 *     tensor dimensions
 *     tensor rank
 *     devices
 *     workers
 *     nodes
 *     accelerators
 *     memory
 *     quantum resources
 *
 * There are intentionally no:
 *
 *     MAX_MODELS
 *     MAX_LAYERS
 *     MAX_PARAMETERS
 *     MAX_TENSORS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *
 * language constants in this grammar.
 *
 * ============================================================================
 * 37. HARD-CODING AUDIT
 * ============================================================================
 *
 * Any future modification to this file MUST be checked for:
 *
 *     [ ] fixed resource count
 *     [ ] fixed device count
 *     [ ] fixed model size
 *     [ ] fixed tensor rank
 *     [ ] fixed tensor dimensions
 *     [ ] fixed worker count
 *     [ ] fixed CPU count
 *     [ ] fixed GPU count
 *     [ ] fixed QPU count
 *     [ ] fixed memory capacity
 *     [ ] fixed topology
 *     [ ] fixed physical identifier
 *     [ ] vendor-specific syntax
 *     [ ] framework-specific syntax
 *
 * Program data such as:
 *
 *     1024
 *     4096
 *     N
 *     batch_size
 *
 * remains legal because values are program semantics.
 *
 * What is prohibited is turning such values into grammar-level universal
 * ceilings.
 *
 * ============================================================================
 * 38. ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors are structural.
 *
 * Examples:
 *
 *     missing ':'
 *     missing '}'
 *     malformed generic parameter
 *     malformed connection
 *     malformed expression
 *
 * Semantic errors are downstream.
 *
 * Examples:
 *
 *     unknown model
 *     duplicate parameter
 *     incompatible tensor type
 *     unavailable capability
 *     unsatisfied resource requirement
 *     invalid model graph
 *     incompatible portability contract
 *
 * These categories MUST remain distinct.
 *
 * ============================================================================
 * 39. SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     environment inspection
 *     hardware discovery
 *     code execution
 *
 * Model source is untrusted input.
 *
 * Parser integration must use safe Rust only.
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * 40. TOOLING
 * ============================================================================
 *
 * This grammar must remain usable by:
 *
 *     compiler frontend
 *     formatter
 *     syntax highlighter
 *     language server
 *     IDE tooling
 *     documentation generator
 *     AST inspector
 *     grammar validator
 *     compatibility validator
 *
 * Tooling must consume the canonical grammar and AST contracts rather than
 * creating a second model syntax.
 *
 * ============================================================================
 * 41. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     @model M {}
 *
 *     @model M<T> {
 *         @input x: Tensor<T>;
 *         @output y: Tensor<T>;
 *     }
 *
 *     @model M<T, Shape> {
 *         @parameter weights: Tensor<T>;
 *         @state state: Tensor<T>;
 *         @component encoder: Encoder;
 *         @layer attention: Attention;
 *         @submodel decoder: Decoder;
 *         @bind activation = relu;
 *         @config precision = "bf16";
 *         @metadata family = "transformer";
 *         @operation forward = forward_function;
 *         @connect encoder.output -> decoder.input;
 *     }
 *
 *     @model Hybrid<T> {
 *         @input x: Tensor<T>;
 *         @capability tensor.compute;
 *         @capability quantum.measurement;
 *         @requires memory >= required_memory;
 *         @constraint latency <= latency_budget;
 *         @preference accelerator.tensor;
 *     }
 *
 * Required negative tests:
 *
 *     malformed model annotation;
 *     missing model name;
 *     missing model body;
 *     malformed generic parameter;
 *     malformed typed member;
 *     malformed assignment;
 *     malformed connection;
 *     missing connection endpoint;
 *     missing semicolon where required;
 *     malformed expression.
 *
 * Required scalability tests:
 *
 *     large member counts;
 *     large generic parameter lists;
 *     deep model nesting;
 *     large connection sets;
 *     symbolic resource expressions;
 *     symbolic tensor dimensions.
 *
 * Required portability tests:
 *
 *     same model source with different target capability sets;
 *     CPU realization;
 *     GPU realization;
 *     accelerator realization;
 *     distributed realization;
 *     quantum-assisted realization;
 *     future/unknown capability names.
 *
 * The grammar itself must not encode any particular resource ceiling.
 *
 * ============================================================================
 * 42. COMPATIBILITY
 * ============================================================================
 *
 * Existing valid model syntax should remain valid unless an explicit language
 * compatibility change is approved.
 *
 * The migration from the previous models.g4 implementation is:
 *
 *     OLD:
 *         roleAnnotation(MODEL_INPUT_ROLE)
 *
 *     NEW:
 *         NANO_ANNOTATION
 *         followed by a structurally classified payload.
 *
 * The semantic annotation names remain:
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
 * They remain semantic annotation values, not parser-level token constants.
 *
 * ============================================================================
 * 43. INTEGRATION WITH grammar/ai/ai.g4
 * ============================================================================
 *
 * `ai.g4` is the AI composition boundary.
 *
 * The canonical integration is:
 *
 *     AI
 *       |
 *       +--> AIModels
 *               |
 *               +--> aiModelConstruct
 *                       |
 *                       +--> aiModelDeclaration
 *
 * Because NANO_ANNOTATION is intentionally lexically opaque, the canonical
 * composition layer must not attempt to distinguish:
 *
 *     @model
 *     @dataset
 *     @tensor
 *     @training
 *
 * through parser token alternatives alone when those alternatives have the
 * same structural shape.
 *
 * The recommended frontend sequence is:
 *
 *     NANO_ANNOTATION
 *          |
 *          v
 *     structural parse
 *          |
 *          v
 *     annotation registry / semantic classification
 *          |
 *          +--> model
 *          +--> dataset
 *          +--> tensor
 *          +--> training
 *          +--> inference
 *          +--> agent
 *          +--> pipeline
 *          +--> accelerator
 *
 * This avoids semantic predicates and preserves deterministic parsing.
 *
 * ============================================================================
 * 44. INTEGRATION WITH grammar/types/types.g4
 * ============================================================================
 *
 * All declared model types use:
 *
 *     typeExpression
 *
 * from Types.
 *
 * Therefore:
 *
 *     Tensor<T>
 *     Tensor<T>[N, M]
 *     Model<T>
 *     Qubit
 *     LogicalQubit
 *     Resource<T>
 *
 * are interpreted by the canonical type system rather than by models.g4.
 *
 * No AI-specific tensor type grammar is introduced here.
 *
 * ============================================================================
 * 45. INTEGRATION WITH grammar/expressions/expressions.g4
 * ============================================================================
 *
 * All values, initializers, bindings, configuration, metadata, requirements,
 * capabilities, constraints and operations reuse:
 *
 *     expression
 *
 * from Expressions.
 *
 * This gives model syntax access to:
 *
 *     arithmetic
 *     logical operations
 *     comparisons
 *     calls
 *     indexing
 *     member access
 *     ranges
 *     literals
 *     generic expression constructs
 *
 * without duplicating the expression hierarchy.
 *
 * ============================================================================
 * 46. INTEGRATION WITH grammar/resources/
 * ============================================================================
 *
 * Model resource annotations are syntactic boundaries only.
 *
 * Resource semantics belong to:
 *
 *     grammar/resources/
 *
 * In particular:
 *
 *     @requires ...
 *     @capability ...
 *     @constraint ...
 *     @preference ...
 *
 * must eventually lower to the canonical resource/capability semantic model.
 *
 * They MUST NOT allocate resources at parse time.
 *
 * ============================================================================
 * 47. INTEGRATION WITH grammar/classical/
 * ============================================================================
 *
 * Model operations that are classical lower through the normal classical
 * semantic pipeline.
 *
 * models.g4 does not define classical instructions.
 *
 * ============================================================================
 * 48. INTEGRATION WITH grammar/quantum/
 * ============================================================================
 *
 * A model may contain or reference quantum computation through ordinary
 * canonical types/expressions and registered semantic capabilities.
 *
 * Quantum syntax remains owned by grammar/quantum/.
 *
 * Quantum lowering remains:
 *
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * 49. INTEGRATION WITH grammar/hdl/ AND grammar/hardware/
 * ============================================================================
 *
 * Model declarations can express hardware-related intent only through
 * canonical resource/capability/constraint abstractions.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * 50. INTEGRATION WITH grammar/distributed/
 * ============================================================================
 *
 * A model may be distributed.
 *
 * Distribution semantics belong to distributed/resource/execution systems.
 *
 * The model grammar does not encode:
 *
 *     node 0
 *     node 1
 *     fixed worker count
 *     fixed topology
 *
 * ============================================================================
 * 51. INTEGRATION WITH grammar/ai/training.g4
 * ============================================================================
 *
 * Training may reference a model semantically.
 *
 * The training grammar should consume model references through canonical
 * expressions/types rather than importing and redefining model syntax.
 *
 * models.g4 therefore does not define:
 *
 *     training loops;
 *     optimizers;
 *     gradient algorithms;
 *     checkpoint algorithms.
 *
 * ============================================================================
 * 52. INTEGRATION WITH grammar/ai/inference.g4
 * ============================================================================
 *
 * Inference consumes model values/references through canonical expression and
 * type syntax.
 *
 * It does not need a second model-reference grammar.
 *
 * ============================================================================
 * 53. INTEGRATION WITH grammar/ai/pipelines.g4
 * ============================================================================
 *
 * Pipelines may use model declarations and model references as semantic
 * pipeline stages.
 *
 * Pipeline topology remains owned by pipelines.g4.
 *
 * ============================================================================
 * 54. INTEGRATION WITH grammar/ai/agents.g4
 * ============================================================================
 *
 * Agents may reference models through canonical expressions.
 *
 * models.g4 does not own:
 *
 *     goals;
 *     planning;
 *     agent memory;
 *     agent tools;
 *     agent scheduling.
 *
 * ============================================================================
 * 55. INTEGRATION WITH FRONTEND AST
 * ============================================================================
 *
 * Before models.g4 is considered complete, the frontend must have a documented
 * mapping for:
 *
 *     ModelDecl
 *     ModelGenericParameter
 *     ModelRelationship
 *     ModelMember
 *     ModelEndpoint
 *
 * or equivalent existing domain-neutral AST structures.
 *
 * If the frontend AST already has generic declaration/member structures,
 * models.g4 MUST map to those structures rather than introducing parallel
 * model-only AST types.
 *
 * ============================================================================
 * 56. INTEGRATION WITH CANONICAL IR
 * ============================================================================
 *
 * models.g4 has no direct IR dependency.
 *
 * The compiler performs:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     canonical/domain IR
 *
 * Any quantum content ultimately uses:
 *
 *     quantum::ir
 *
 * Any classical content uses the canonical classical representation.
 *
 * Any hardware content uses the hardware/HDL semantic boundary.
 *
 * ============================================================================
 * 57. RUST 1.97 / 1.97.1
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The surrounding Rust implementation MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Rust 2021.
 *
 * Safe Rust only.
 *
 * No unsafe blocks/functions are required for this grammar.
 *
 * ============================================================================
 * 58. DEFINITION OF DONE
 * ============================================================================
 *
 * models.g4 is complete when:
 *
 *     [x] one parser grammar identity exists;
 *     [x] canonical lexer vocabulary is reused;
 *     [x] no fake MODEL_* lexer tokens are required;
 *     [x] no parameterized roleAnnotation rules are required;
 *     [x] type syntax comes from Types;
 *     [x] expression syntax comes from Expressions;
 *     [x] no model-specific expression hierarchy exists;
 *     [x] no model-specific type system exists;
 *     [x] no fixed hardware capacity exists;
 *     [x] no fixed model capacity exists;
 *     [x] no vendor framework is required;
 *     [x] model references are not arbitrary public AI constructs;
 *     [x] resource semantics remain downstream;
 *     [x] capability semantics remain downstream;
 *     [x] quantum semantics remain downstream;
 *     [x] quantum::ir remains canonical;
 *     [x] no IR is created here;
 *     [x] no unsafe implementation is required;
 *     [x] deterministic parsing is preserved structurally;
 *     [x] extensibility is annotation-driven;
 *     [x] positive/negative/scalability tests are defined;
 *     [x] integration boundaries are explicitly documented.
 *
 * Implementation completion additionally requires:
 *
 *     [ ] ANTLR generation succeeds against the repository's canonical
 *         ZamaniLexer;
 *     [ ] the generated parser integrates with the actual Rust frontend;
 *     [ ] frontend AST mapping is implemented;
 *     [ ] semantic annotation registration is implemented;
 *     [ ] conformance fixtures pass;
 *     [ ] grammar/ai/ai.g4 composition is validated;
 *     [ ] grammar.md reflects the resulting accepted syntax.
 *
 * Those implementation checks cannot honestly be marked complete by changing
 * this grammar file alone.
 *
 * ============================================================================
 */