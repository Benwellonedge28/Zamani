/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/models.g4
 *
 * Status:
 *     CANONICAL AI MODEL SOURCE-SYNTAX GRAMMAR
 *
 * Grammar:
 *     AIModels
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - This grammar contains no embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific implementation.
 *     - No I/O.
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
 * This file is the single owner of SOURCE-LEVEL AI MODEL DECLARATION syntax.
 *
 * It describes portable model intent and structure.
 *
 * It does NOT describe:
 *
 *     - a particular ML framework;
 *     - a vendor;
 *     - a CPU;
 *     - a GPU;
 *     - a TPU;
 *     - an NPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a physical device;
 *     - a physical topology;
 *     - a runtime;
 *     - a scheduler;
 *     - a compiler backend;
 *     - a memory implementation;
 *     - a quantum backend;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical parser
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     canonical types           canonical expressions
 *          |                          |
 *          +------------+-------------+
 *                       |
 *                       v
 *                  AIModels
 *                       |
 *                       v
 *                 Frontend AST
 *                       |
 *                       v
 *              structural validation
 *                       |
 *                       v
 *               semantic analysis
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      AI semantics  resources   capabilities
 *          |            |             |
 *          +------------+-------------+
 *                       |
 *                       v
 *                 canonical IR
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      classical     quantum::ir   HDL/hardware
 *          |            |             |
 *          +------------+-------------+
 *                       |
 *                       v
 *                 optimization
 *                       |
 *              routing / scheduling
 *                       |
 *                 resilience / QEC
 *                       |
 *                       ZQN
 *                       |
 *                       HAL
 *                       |
 *                 target realization
 *
 * AIModels MUST NOT construct IR.
 *
 * AIModels MUST NOT create a second AI IR.
 *
 * AIModels MUST NOT create a second quantum IR.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI model declaration syntax;
 *     - model identity;
 *     - model generic parameter attachment;
 *     - model inheritance/interface references;
 *     - model body composition;
 *     - model annotated members;
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
 *     - model requirements;
 *     - model capabilities;
 *     - model constraints;
 *     - model preferences;
 *     - model operations;
 *     - model portability declarations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifier syntax;
 *     - general expression syntax;
 *     - general type syntax;
 *     - generic type APPLICATION syntax;
 *     - function generic declarations;
 *     - tensor semantics;
 *     - dataset semantics;
 *     - training semantics;
 *     - inference semantics;
 *     - differentiation semantics;
 *     - agent semantics;
 *     - pipeline semantics;
 *     - accelerator implementation;
 *     - resource discovery;
 *     - capability discovery;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - hardware discovery;
 *     - distributed execution;
 *     - quantum semantics;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime behavior.
 *
 * ============================================================================
 *
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file retains the repository's existing filename:
 *
 *     grammar/ai/models.g4
 *
 * Do NOT introduce:
 *
 *     grammar/ai/model.g4
 *
 * as a second canonical model grammar.
 *
 * If a compatibility filename is ever required, it must be a thin
 * compatibility façade and MUST NOT contain independent model productions.
 *
 * `models.g4` remains the sole source-level model grammar authority.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It consumes:
 *
 *     grammar/lexer/tokens.g4
 *
 * through the canonical lexical composition.
 *
 * Parser grammars therefore use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * ============================================================================
 *
 * AI KEYWORD POLICY
 * ============================================================================
 *
 * AI concepts deliberately remain extensible.
 *
 * The following semantic roles are NOT introduced as dedicated lexer
 * keywords:
 *
 *     model
 *     input
 *     output
 *     parameter
 *     state
 *     component
 *     layer
 *     submodel
 *     connect
 *     bind
 *     config
 *     metadata
 *     requires
 *     capability
 *     constraint
 *     preference
 *     operation
 *     compose
 *
 * Instead, the existing canonical annotation token is used:
 *
 *     NANO_ANNOTATION
 *
 * Examples:
 *
 *     @model
 *     @input
 *     @output
 *     @parameter
 *     @state
 *
 * The parser recognizes the annotation structurally.
 *
 * Semantic analysis owns the exact normalized annotation spelling and its
 * meaning.
 *
 * This avoids:
 *
 *     - lexer proliferation;
 *     - framework-specific keywords;
 *     - vendor-specific keywords;
 *     - parser/lexer disagreement;
 *     - future dialect incompatibility.
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR CORRECTION
 * ============================================================================
 *
 * The previous implementation attempted constructs such as:
 *
 *     roleAnnotation(MODEL_INPUT_ROLE)
 *
 * This is not valid for the intended grammar contract because the role
 * constants are not lexer tokens and the grammar was not defining an
 * appropriate semantic-parameter mechanism.
 *
 * This implementation deliberately uses:
 *
 *     NANO_ANNOTATION
 *
 * directly and leaves annotation-role interpretation to semantic analysis.
 *
 * That makes the grammar deterministic, target-independent, and extensible.
 *
 * ============================================================================
 *
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Canonical source type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * Model ports and model state therefore consume:
 *
 *     typeExpression
 *
 * from the canonical type grammar.
 *
 * This file MUST NOT redefine:
 *
 *     typeExpression
 *     named types
 *     generic type application
 *     tensors
 *     quantum types
 *     hardware types
 *     resource types.
 *
 * ============================================================================
 *
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Canonical expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file therefore consumes:
 *
 *     expression
 *     argumentList
 *
 * where appropriate.
 *
 * It MUST NOT redefine:
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
 * GENERIC DECLARATION INTEGRATION
 * ============================================================================
 *
 * Model generic parameters are DECLARATION parameters.
 *
 * The repository already has the canonical generic-declaration grammar:
 *
 *     grammar/functions/generics.g4
 *
 * whose public rule family is:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *     functionGenericParameterName
 *     functionGenericParameterBounds
 *     functionGenericParameterBound
 *
 * This file consumes that established representation through:
 *
 *     modelGenericParameters
 *
 * which is a model-domain adapter only.
 *
 * It does NOT redefine generic parameter semantics.
 *
 * Generic TYPE APPLICATION remains owned by the type grammar.
 *
 * Therefore:
 *
 *     @model Model<T> ...
 *
 * uses generic DECLARATION syntax.
 *
 * While:
 *
 *     Tensor<T>
 *
 * uses generic TYPE APPLICATION syntax.
 *
 * These two concepts must not be conflated.
 *
 * ============================================================================
 *
 * GENERIC SCALABILITY
 * ============================================================================
 *
 * No finite model generic arity is encoded.
 *
 * Valid conceptual forms include:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *     <T, U, V, ...>
 *
 * Subject only to implementation resource availability.
 *
 * There is no:
 *
 *     MAX_MODEL_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_MODEL_DEPTH
 *
 * in this grammar.
 *
 * ============================================================================
 *
 * MODEL SEMANTICS
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What model structure did the programmer write?"
 *
 * Semantic analysis answers:
 *
 *     - Does the model name resolve?
 *     - Are annotations valid?
 *     - Are names unique?
 *     - Are types valid?
 *     - Are graph endpoints valid?
 *     - Are connections type-compatible?
 *     - Are requirements satisfiable?
 *     - Are capabilities available?
 *     - Are constraints consistent?
 *     - Are preferences legal?
 *     - Is the model portable?
 *     - Can it be lowered?
 *
 * None of those semantic decisions occur here.
 *
 * ============================================================================
 *
 * MODEL GRAPH
 * ============================================================================
 *
 * A model connection expresses a semantic dependency:
 *
 *     @connect encoder.output -> classifier.input;
 *
 * It does NOT express:
 *
 *     - a physical network link;
 *     - a GPU interconnect;
 *     - a CPU topology;
 *     - a QPU coupling edge;
 *     - a hardware route;
 *     - a scheduler dependency.
 *
 * The compiler may realize the same model graph differently on different
 * targets.
 *
 * ============================================================================
 *
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * AI model declarations may express:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *
 * but these remain declarations of intent.
 *
 * They do not allocate hardware.
 *
 * They do not select:
 *
 *     GPU 0
 *     CPU 3
 *     QPU 7
 *     FPGA 2
 *     node 4
 *
 * and they do not impose physical machine limits.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * A model declaration must remain portable across:
 *
 *     tiny systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     NPUs;
 *     TPUs;
 *     FPGAs;
 *     ASICs;
 *     quantum processors;
 *     heterogeneous accelerators;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational targets.
 *
 * The grammar contains no universal machine-size limits.
 *
 * In particular it contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_MODEL_SIZE
 *     MAX_LAYER_COUNT
 *     MAX_PARAMETER_COUNT
 *     MAX_INPUT_COUNT
 *     MAX_OUTPUT_COUNT
 *
 * A source-level number is program data, not a compiler-imposed maximum.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * A model may reference values and types associated with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     data
 *     networking
 *     security
 *     resources
 *     effects
 *     future domains
 *
 * This grammar does not import their implementations merely to recognize a
 * model.
 *
 * Semantic analysis resolves cross-domain meaning.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * AI model syntax may reference quantum types, operations, or model values
 * through canonical type/expression boundaries.
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     physical qubit
 *     gate enumeration
 *     quantum topology
 *     QEC
 *     ZQN
 *     calibration
 *     routing
 *     scheduling
 *
 * Quantum lowering remains:
 *
 *     source AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / routing / scheduling / resilience
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/AST layer must preserve, at minimum:
 *
 *     - complete model source span;
 *     - model declaration annotation;
 *     - model name;
 *     - generic parameters;
 *     - inheritance references;
 *     - implementation/interface references;
 *     - ordered model members;
 *     - member annotation;
 *     - member name;
 *     - member type;
 *     - initializer expression;
 *     - connection endpoints;
 *     - member expressions;
 *     - source provenance.
 *
 * This grammar does not define AST structs.
 *
 * The canonical frontend AST remains the only AST authority.
 *
 * ============================================================================
 *
 * SOURCE ORDER
 * ============================================================================
 *
 * Model members are represented as an ordered repetition:
 *
 *     modelMember*
 *
 * Source order MUST be preserved by the parser/AST layer.
 *
 * This is important for:
 *
 *     diagnostics;
 *     source maps;
 *     formatting;
 *     deterministic tooling;
 *     provenance;
 *     future semantic rules.
 *
 * The semantic layer may later establish whether particular members are
 * order-independent.
 *
 * ============================================================================
 *
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong here.
 *
 * Examples:
 *
 *     @model
 *     @model Name
 *     @model Name <
 *     @model Name {}
 *     @input
 *     @input x:
 *     @output y:
 *     @connect a ->
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     duplicate input name
 *     unknown output
 *     incompatible connection
 *     unknown capability
 *     impossible resource requirement
 *     unsatisfied generic bound
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no mutable global state;
 *     - no I/O;
 *     - no runtime calls;
 *     - no hardware inspection;
 *     - no environment inspection;
 *     - no randomness;
 *     - no time dependence.
 *
 * Identical token streams therefore produce structurally equivalent parse
 * trees.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * The grammar cannot:
 *
 *     - execute a model;
 *     - open a file;
 *     - access a network;
 *     - invoke a device;
 *     - access credentials;
 *     - allocate hardware;
 *     - access memory outside parser state.
 *
 * Untrusted-source protection belongs to explicit parser/compiler resource
 * policies and structural validation.
 *
 * ============================================================================
 *
 * RUST CONTRACT
 * ============================================================================
 *
 * The grammar contains no Rust implementation.
 *
 * The generated compiler/frontend integration MUST:
 *
 *     - compile on Rust 1.97;
 *     - compile on Rust 1.97.1;
 *     - use Rust 2021;
 *     - use safe Rust only;
 *     - contain no unsafe blocks;
 *     - contain no unsafe functions;
 *     - preserve source spans;
 *     - preserve source ordering;
 *     - remain target independent.
 *
 * ============================================================================
 *
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Existing models.g4 filename is retained.
 *     [x] models.g4 remains the sole model grammar authority.
 *     [x] Canonical ZamaniLexer vocabulary is consumed.
 *     [x] No lexer rules are duplicated.
 *     [x] No AI framework is encoded.
 *     [x] No vendor is encoded.
 *     [x] No hardware limit is encoded.
 *     [x] No machine identifier is encoded.
 *     [x] No fixed model cardinality is encoded.
 *     [x] Generic declaration syntax is separated from generic type application.
 *     [x] Canonical typeExpression is reused.
 *     [x] Canonical expression is reused.
 *     [x] Model graph syntax is source-level only.
 *     [x] Resource declarations remain declarative.
 *     [x] Capability declarations remain declarative.
 *     [x] Quantum semantics remain downstream.
 *     [x] quantum::ir remains canonical.
 *     [x] No second AI IR is introduced.
 *     [x] No Rust actions are embedded.
 *     [x] No unsafe Rust is required.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         v
 *     AIModels
 *
 *     grammar/types/types.g4
 *         |
 *         v
 *     typeExpression
 *
 *     grammar/expressions/expressions.g4
 *         |
 *         v
 *     expression / argumentList
 *
 *     grammar/functions/generics.g4
 *         |
 *         v
 *     functionGenericParameters
 *
 * DOWNSTREAM:
 *
 *     AIModels
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     structural validation
 *         |
 *         v
 *     semantic AI model analysis
 *         |
 *         +-------------------+
 *         |                   |
 *         v                   v
 *     resource/capability   type/effect analysis
 *         |                   |
 *         +---------+---------+
 *                   |
 *                   v
 *             canonical semantic model
 *                   |
 *                   v
 *              canonical IR
 *
 * CROSS-DOMAIN:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     data
 *     networking
 *     security
 *     future domains
 *
 * must consume this model syntax through semantic contracts rather than
 * modifying this grammar for target-specific implementations.
 *
 * ============================================================================
 */

parser grammar AIModels;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       FunctionGenerics;


/* ============================================================================
 * PUBLIC MODEL ENTRY POINT
 * ========================================================================== */

/**
 * Public AI model syntax boundary.
 *
 * Only an explicit @model annotation can enter this grammar.
 *
 * Ordinary expressions and ordinary identifiers are deliberately NOT model
 * constructs.
 *
 * Model references and model invocations remain ordinary expression/type
 * constructs and are therefore not duplicated here.
 */
aiModelConstruct
    : aiModelDeclaration
    ;


/* ============================================================================
 * MODEL DECLARATION
 * ========================================================================== */

/**
 * Canonical model declaration.
 *
 * Examples:
 *
 *     @model LinearModel {
 *         ...
 *     }
 *
 *     @model LinearModel<T> {
 *         ...
 *     }
 *
 *     @model Encoder<T extends Numeric> {
 *         ...
 *     }
 *
 * The semantic layer validates that the annotation is exactly `@model`.
 */
aiModelDeclaration
    : modelAnnotation
      identifier
      modelGenericParameters?
      modelExtendsClause?
      modelImplementsClause?
      modelBody
    ;


/**
 * Generic declaration adapter.
 *
 * `functionGenericParameters` is the existing repository-wide generic
 * declaration grammar. This adapter gives the model domain a stable semantic
 * name without creating another generic parameter grammar.
 */
modelGenericParameters
    : functionGenericParameters
    ;


/**
 * Model inheritance.
 *
 * Model identity remains a source-level name.
 */
modelExtendsClause
    : EXTENDS
      modelQualifiedNameList
    ;


/**
 * Model interface/contract implementation.
 */
modelImplementsClause
    : IMPLEMENTS
      modelQualifiedNameList
    ;


modelQualifiedNameList
    : modelQualifiedName
      (
          COMMA
          modelQualifiedName
      )*
    ;


modelQualifiedName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/* ============================================================================
 * MODEL ANNOTATION
 * ========================================================================== */

/**
 * The lexical token contains the complete annotation.
 *
 * Semantic analysis validates:
 *
 *     @model
 *
 * as the model declaration role.
 *
 * No parser-level text comparison is required.
 */
modelAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * MODEL BODY
 * ========================================================================== */

modelBody
    : LBRACE
      modelMember*
      RBRACE
    ;


/**
 * Every model member begins with an opaque annotation.
 *
 * The annotation determines semantic role:
 *
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
 * Semantic analysis owns this role table.
 */
modelMember
    : NANO_ANNOTATION
      modelMemberPayload
    ;


/**
 * The payload is deliberately generic enough for future AI model roles while
 * remaining structurally deterministic.
 */
modelMemberPayload
    : modelConnectionPayload
    | modelNamedMemberPayload
    ;


/* ============================================================================
 * CONNECTION MEMBER
 * ========================================================================== */

/**
 * Example:
 *
 *     @connect encoder.output -> classifier.input;
 *
 * This represents a semantic model graph edge.
 *
 * It is not a hardware/network topology declaration.
 */
modelConnectionPayload
    : modelEndpoint
      ARROW
      modelEndpoint
      SEMICOLON
    ;


modelEndpoint
    : identifier
      (
          DOT
          identifier
      )*
    ;


/* ============================================================================
 * NAMED MODEL MEMBER
 * ========================================================================== */

/**
 * Shared syntax for named model members.
 *
 * This intentionally avoids one parser alternative per annotation role.
 *
 * That prevents the previous ambiguity where all alternatives began with the
 * same NANO_ANNOTATION + identifier prefix.
 *
 * Semantic analysis maps the annotation to its role.
 */
modelNamedMemberPayload
    : identifier
      modelMemberValue
    ;


/**
 * Named member forms.
 *
 * Examples:
 *
 *     @input features: Tensor<Float>;
 *     @output prediction: Tensor<Float>;
 *     @parameter weights: Tensor<Float>;
 *     @state state: Tensor<Float>;
 *     @component encoder: Encoder;
 *     @layer block: TransformerLayer;
 *     @submodel encoder: Encoder;
 *
 *     @bind activation = relu;
 *     @config precision = "bf16";
 *     @metadata family = "transformer";
 *     @requires memory = required_memory;
 *     @capability tensor_compute = true;
 *     @constraint latency = maximum_latency;
 *     @preference throughput = preferred_throughput;
 *     @operation forward = expression;
 *
 * The annotation role supplies semantic meaning.
 */
modelMemberValue
    : modelTypedMemberValue
    | modelAssignedMemberValue
    | modelCallMemberValue
    | modelEmptyMemberValue
    ;


/* ============================================================================
 * TYPED MEMBER
 * ========================================================================== */

/**
 * Port/state/component/layer/submodel/parameter form.
 */
modelTypedMemberValue
    : COLON
      typeExpression
      modelOptionalInitializer?
      SEMICOLON
    ;


modelOptionalInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * ASSIGNED MEMBER
 * ========================================================================== */

/**
 * Generic expression-valued member.
 */
modelAssignedMemberValue
    : ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CALL-LIKE MEMBER
 * ========================================================================== */

/**
 * Extensible model role payload.
 *
 * Example:
 *
 *     @compose encoder(classifier);
 *
 *     @operation forward(input);
 *
 * The operation semantics are resolved downstream.
 */
modelCallMemberValue
    : LPAREN
      argumentList?
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * EMPTY MEMBER
 * ========================================================================== */

/**
 * Allows declaration-style annotations whose semantic payload is carried by
 * the annotation itself or by a later semantic phase.
 *
 * Example:
 *
 *     @trainable weight;
 *
 * The semantic layer determines whether a bare member is legal for that role.
 */
modelEmptyMemberValue
    : SEMICOLON
    ;


/* ============================================================================
 * MODEL PORT ADAPTER
 * ========================================================================== */

/**
 * Stable parser boundary for consumers that need a model port-shaped value.
 *
 * This is intentionally a parser alias over canonical type syntax.
 */
modelPortType
    : COLON
      typeExpression
    ;


/* ============================================================================
 * MODEL EXPRESSION ADAPTER
 * ========================================================================== */

/**
 * Stable model-domain expression adapter.
 *
 * It exists only as an integration boundary and does not define a second
 * expression grammar.
 */
modelExpression
    : expression
    ;


/* ============================================================================
 * MODEL TYPE ADAPTER
 * ========================================================================== */

/**
 * Stable model-domain type adapter.
 */
modelTypeReference
    : typeExpression
    ;


/* ============================================================================
 * MODEL RESOURCE EXPRESSION
 * ========================================================================== */

/**
 * Resource meaning is owned by the resource subsystem.
 *
 * This grammar merely preserves the source expression.
 */
modelResourceExpression
    : expression
    ;


/* ============================================================================
 * MODEL CAPABILITY EXPRESSION
 * ========================================================================== */

/**
 * Capability meaning is owned by capability/resource semantic analysis.
 */
modelCapabilityExpression
    : expression
    ;


/* ============================================================================
 * MODEL CONSTRAINT EXPRESSION
 * ========================================================================== */

/**
 * Constraint meaning is semantic, not syntactic.
 */
modelConstraintExpression
    : expression
    ;


/* ============================================================================
 * MODEL PORTABILITY EXPRESSION
 * ========================================================================== */

/**
 * Portability remains expression-based so future target classes do not require
 * grammar rewrites.
 */
modelPortabilityExpression
    : expression
    ;


/* ============================================================================
 * MODEL VALIDATION ENTRY
 * ========================================================================== */

/**
 * Parser/validation integration boundary.
 */
modelValidationInput
    : aiModelDeclaration
    ;


/* ============================================================================
 * MODEL AST CONTRACT
 * ============================================================================
 *
 * The AST lowering layer must preserve:
 *
 *     model:
 *         source span
 *         declaration annotation
 *         name
 *         generic parameters
 *         inheritance
 *         interfaces
 *         ordered members
 *
 *     member:
 *         source span
 *         annotation
 *         member name
 *         declared type
 *         initializer/expression
 *         connection endpoints
 *         source provenance
 *
 * No AI-specific semantic AST may be invented here.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. Normalize annotations.
 *     2. Validate @model.
 *     3. Validate member annotation roles.
 *     4. Resolve model names.
 *     5. Resolve generic parameters.
 *     6. Resolve types.
 *     7. Resolve expressions.
 *     8. Validate duplicate member names where applicable.
 *     9. Validate model graph endpoints.
 *    10. Validate type compatibility.
 *    11. Validate capability requirements.
 *    12. Validate resource requirements.
 *    13. Validate constraints.
 *    14. Validate preferences.
 *    15. Validate portability.
 *    16. Preserve source provenance.
 *    17. Produce deterministic diagnostics.
 *
 * None of those operations belong in this parser grammar.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT directly create:
 *
 *     AIModelIR
 *     TensorIR
 *     NeuralLayerIR
 *     QuantumGate
 *     Qubit
 *     PhysicalQubit
 *     HardwareDevice
 *     ResourceAllocation
 *     Schedule
 *     Backend
 *     Calibration
 *     QEC object
 *     ZQN object
 *
 * Correct path:
 *
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic AI model
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *   classical            quantum::ir
 *
 * Quantum computation remains subject to the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 *
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing public parser rule:
 *
 *     aiModelConstruct
 *
 * is retained.
 *
 * Existing principal declaration rule:
 *
 *     aiModelDeclaration
 *
 * is retained.
 *
 * Existing model grammar filename:
 *
 *     grammar/ai/models.g4
 *
 * is retained.
 *
 * The following obsolete constructs are intentionally removed:
 *
 *     roleAnnotation(MODEL_INPUT_ROLE)
 *     roleAnnotation(MODEL_OUTPUT_ROLE)
 *     roleAnnotation(MODEL_PARAMETER_ROLE)
 *     ...
 *
 * because those role constants were not part of the canonical lexical
 * vocabulary and the parameterized-rule approach was not a valid integration
 * contract for this design.
 *
 * Model references and model instantiation are also not owned here.
 *
 * They remain ordinary source expressions/type applications and therefore
 * belong to the canonical expression/type systems.
 *
 * ============================================================================
 *
 * NO SECOND MODEL LANGUAGE
 * ============================================================================
 *
 * This grammar does not create framework-specific syntax such as:
 *
 *     @pytorch
 *     @tensorflow
 *     @jax
 *     @onnx
 *     @cuda
 *     @rocm
 *     @tpu
 *     @npu
 *
 * Framework interoperability belongs to interoperability/dialect contracts.
 *
 * ============================================================================
 *
 * NO FIXED MODEL TAXONOMY
 * ============================================================================
 *
 * The grammar does not enumerate:
 *
 *     Dense
 *     CNN
 *     RNN
 *     LSTM
 *     GRU
 *     Transformer
 *     GNN
 *     GAN
 *     VAE
 *     diffusion
 *
 * as syntax-level model kinds.
 *
 * These can be represented by normal types, names, annotations, libraries, or
 * dialect semantics.
 *
 * Future model architectures therefore do not require changes to this file.
 *
 * ============================================================================
 *
 * NO FIXED RESOURCE MODEL
 * ============================================================================
 *
 * The grammar does not enumerate:
 *
 *     GPU count
 *     CPU count
 *     accelerator count
 *     memory capacity
 *     tensor-core count
 *     node count
 *     device count
 *     cluster size
 *     network width
 *
 * Resource semantics belong to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * ============================================================================
 *
 * DETERMINISTIC SOURCE ORDER
 * ============================================================================
 *
 * Model members use:
 *
 *     modelMember*
 *
 * The resulting parse tree preserves source ordering.
 *
 * The semantic layer may establish order-independent semantics where explicitly
 * specified, but parser ordering must remain deterministic.
 *
 * ============================================================================
 *
 * SCALABILITY INVARIANT
 * ============================================================================
 *
 * This grammar has no source-language cardinality ceilings for:
 *
 *     model declarations;
 *     generic parameters;
 *     members;
 *     inputs;
 *     outputs;
 *     parameters;
 *     states;
 *     components;
 *     layers;
 *     submodels;
 *     connections;
 *     requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     operations.
 *
 * Every collection is structurally repeated.
 *
 * Practical parser/compiler resource limits are implementation policy and must
 * never become language semantics.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS:
 *
 *     no MAX_QUBITS
 *     no MAX_CPUS
 *     no MAX_GPUS
 *     no MAX_FPGAS
 *     no MAX_NODES
 *     no MAX_MEMORY
 *     no MAX_THREADS
 *     no MAX_TENSOR_RANK
 *     no MAX_REGISTER_WIDTH
 *     no MAX_NETWORK_SIZE
 *     no MAX_DEVICE_COUNT
 *     no MAX_MODEL_SIZE
 *     no MAX_LAYER_COUNT
 *     no MAX_PARAMETER_COUNT
 *     no MAX_INPUT_COUNT
 *     no MAX_OUTPUT_COUNT
 *
 * No physical machine assumption is encoded.
 *
 * ============================================================================
 *
 * FUTURE-PROOFING
 * ============================================================================
 *
 * New AI concepts should normally be introduced through:
 *
 *     new annotation role
 *          +
 *     semantic registration
 *          +
 *     AST semantic interpretation
 *          +
 *     capability/resource contracts
 *          +
 *     canonical IR lowering
 *
 * rather than by modifying this grammar's fundamental model structure.
 *
 * This permits:
 *
 *     new architectures;
 *     new learning paradigms;
 *     new accelerators;
 *     new classical processors;
 *     new quantum processors;
 *     new distributed systems;
 *     new AI dialects;
 *
 * without making the source language depend on today's hardware.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive fixtures MUST include at least:
 *
 *     @model M {}
 *
 *     @model M {
 *         @input x: Tensor<Float>;
 *         @output y: Tensor<Float>;
 *     }
 *
 *     @model M<T> {
 *         @input x: Tensor<T>;
 *         @output y: Tensor<T>;
 *     }
 *
 *     @model M<T extends Numeric> {
 *         @parameter weights: Tensor<T>;
 *     }
 *
 *     @model M {
 *         @component encoder: Encoder;
 *         @layer block: TransformerBlock;
 *         @submodel head: Head;
 *     }
 *
 *     @model M {
 *         @connect encoder.output -> head.input;
 *     }
 *
 *     @model M {
 *         @requires memory = required_memory;
 *         @capability tensor_compute = true;
 *         @constraint latency = maximum_latency;
 *         @preference throughput = preferred_throughput;
 *     }
 *
 * Negative fixtures MUST include:
 *
 *     @model
 *     @model M
 *     @model M<
 *     @model M<T,>
 *         // accepted only when generic declaration grammar permits it
 *     @input
 *     @input x:
 *     @connect a ->
 *
 * Semantic negative tests must separately cover:
 *
 *     duplicate names;
 *     unknown endpoints;
 *     invalid types;
 *     invalid annotations;
 *     unsatisfied generic bounds;
 *     impossible resource requirements;
 *     unknown capabilities;
 *     contradictory constraints.
 *
 * ============================================================================
 *
 * FINAL INVARIANT
 * ============================================================================
 *
 * The AI model grammar is a SOURCE SYNTAX CONTRACT.
 *
 * It does not become:
 *
 *     AI runtime;
 *     model executor;
 *     tensor engine;
 *     accelerator selector;
 *     hardware mapper;
 *     distributed scheduler;
 *     quantum compiler;
 *     QEC engine;
 *     ZQN implementation;
 *     HAL implementation.
 *
 * The complete path remains:
 *
 *     Zamani source
 *          |
 *          v
 *     AIModels
 *          |
 *          v
 *     canonical frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     ZQN / HAL
 *          |
 *          v
 *     target realization
 *
 * This is the required separation for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */