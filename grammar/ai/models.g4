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
 * Status:
 *     CANONICAL AI MODEL SOURCE GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE canonical source-level grammar for AI/ML MODEL
 * DECLARATIONS.
 *
 * It describes model structure and model intent.
 *
 * It does NOT implement:
 *
 *     - machine learning algorithms;
 *     - tensor execution;
 *     - automatic differentiation;
 *     - training;
 *     - inference;
 *     - scheduling;
 *     - placement;
 *     - resource allocation;
 *     - hardware discovery;
 *     - accelerator selection;
 *     - distributed execution;
 *     - quantum execution;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime behavior.
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
 *     AI
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
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical semantics
 *          +--> quantum semantics
 *          +--> HDL/hardware semantics
 *          +--> distributed semantics
 *          +--> accelerator semantics
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization
 *          |
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
 *     runtime
 *
 * This grammar MUST NOT create an IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - model declaration syntax;
 *     - model identity;
 *     - model generic declaration parameters;
 *     - model inheritance syntax;
 *     - model interface/contract syntax;
 *     - model body structure;
 *     - model member structure;
 *     - model graph connection syntax;
 *     - model typed members;
 *     - model expression members;
 *     - model nested regions;
 *     - model annotation boundaries;
 *     - model source-level extensibility.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - lexer rules;
 *     - operators;
 *     - literals;
 *     - expression precedence;
 *     - type syntax;
 *     - generic type application;
 *     - generic substitution;
 *     - type inference;
 *     - tensor semantics;
 *     - dataset semantics;
 *     - training semantics;
 *     - inference semantics;
 *     - agent semantics;
 *     - accelerator implementation;
 *     - hardware discovery;
 *     - resource discovery;
 *     - scheduling;
 *     - routing;
 *     - distributed placement;
 *     - quantum semantics;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * `grammar/ai/models.g4` is the sole canonical AI model grammar.
 *
 * `grammar/ai/model.g4` MUST NOT be imported into the production ANTLR
 * composition because it previously declared the same `AIModels` grammar
 * identity and therefore created a competing grammar authority.
 *
 * The repository should retain `model.g4` only as a compatibility/deprecation
 * artifact until it is removed by the repository's normal compatibility
 * policy. It MUST NOT contain independent canonical productions.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Parser vocabulary:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * AI concepts are intentionally not added as dedicated lexer keywords.
 *
 * The canonical annotation token:
 *
 *     NANO_ANNOTATION
 *
 * supplies the annotation boundary.
 *
 * Examples:
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
 * The parser treats the annotation structurally.
 *
 * Semantic analysis determines whether the normalized annotation is a
 * registered model role.
 *
 * No semantic predicate or target-language action is required.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Canonical type syntax belongs to:
 *
 *     grammar/types/types.g4
 *
 * This grammar therefore consumes:
 *
 *     typeExpression
 *
 * and never redefines it.
 *
 * Model types may consequently refer to future and existing domains through
 * the canonical type system, including:
 *
 *     classical types;
 *     tensor types;
 *     quantum types;
 *     hardware types;
 *     resource types;
 *     distributed types;
 *     user-defined types;
 *     future domain types.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Canonical expression syntax belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * This grammar consumes:
 *
 *     expression
 *     argumentList
 *
 * and never redefines:
 *
 *     arithmetic;
 *     calls;
 *     indexing;
 *     member access;
 *     assignment;
 *     literals;
 *     operators;
 *     lambdas;
 *     ranges;
 *     conditional expressions.
 *
 * ============================================================================
 * GENERIC CONTRACT
 * ============================================================================
 *
 * Generic declaration syntax belongs to:
 *
 *     grammar/functions/generics.g4
 *
 * This grammar therefore reuses:
 *
 *     functionGenericParameters
 *
 * Model generic declarations are declaration parameters.
 *
 * Generic type application remains owned by the canonical type grammar.
 *
 * Example:
 *
 *     @model Network<T extends Numeric> {
 *         ...
 *     }
 *
 * The `<T ...>` portion is a declaration generic list.
 *
 * Example:
 *
 *     @input x: Tensor<T>;
 *
 * The `Tensor<T>` portion is a type application.
 *
 * These concepts MUST remain distinct.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Model declarations describe portable computation.
 *
 * They MUST NOT encode universal hardware limits.
 *
 * This grammar contains no:
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
 * Nor does it enumerate:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     FPGA 0
 *     physical qubit identifiers
 *     fixed memory banks
 *     fixed accelerator counts.
 *
 * Source-level values such as:
 *
 *     1024
 *
 * remain valid program data.
 *
 * The distinction is:
 *
 *     program value
 *         !=
 *     compiler-imposed universal limit
 *
 * Practical limits belong to:
 *
 *     parser resource policy;
 *     compiler resources;
 *     runtime resources;
 *     target resources;
 *     deployment policy.
 *
 * ============================================================================
 * MODEL GRAPH CONTRACT
 * ============================================================================
 *
 * A connection:
 *
 *     @connect encoder.output -> classifier.input;
 *
 * represents a source-level model graph edge.
 *
 * It does NOT represent:
 *
 *     - a network link;
 *     - a hardware wire;
 *     - a GPU interconnect;
 *     - a QPU coupling edge;
 *     - a physical route;
 *     - a scheduler dependency.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * AI models may reference quantum semantics through:
 *
 *     canonical types;
 *     canonical expressions;
 *     model bindings;
 *     model operations;
 *     capabilities;
 *     resource requirements.
 *
 * This grammar MUST NOT define:
 *
 *     - quantum gates;
 *     - physical qubits;
 *     - QubitId;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN.
 *
 * Quantum semantics continue through:
 *
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Model syntax may refer to hardware-oriented types and capabilities through
 * canonical types, expressions and annotations.
 *
 * This grammar MUST NOT encode:
 *
 *     wire [31:0]
 *     fixed register widths
 *     fixed RAM capacity
 *     fixed VRAM capacity
 *     fixed FPGA resource counts
 *     fixed ASIC technology
 *     physical wiring
 *     device identifiers.
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Model declarations may express source intent such as:
 *
 *     @requires memory >= required_memory;
 *     @capability tensor.compute;
 *     @constraint latency <= latency_budget;
 *     @preference accelerator.tensor;
 *
 * These are declarative source contracts.
 *
 * They do not allocate hardware.
 *
 * They do not imply:
 *
 *     use_gpu_0
 *     use_cpu_3
 *     use_qpu_7
 *     use_device_2
 *
 * Resource discovery, capability matching, placement and scheduling remain
 * downstream responsibilities.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps to the existing domain-neutral frontend AST.
 *
 * It MUST NOT create an AI-specific parallel AST solely because this grammar
 * exists.
 *
 * The AST must preserve:
 *
 *     - complete source span;
 *     - declaration span;
 *     - annotation span;
 *     - declaration name;
 *     - generic parameter ordering;
 *     - generic parameter spans;
 *     - inheritance/interface references;
 *     - model member ordering;
 *     - member annotation;
 *     - member name where present;
 *     - member type where present;
 *     - initializer expression where present;
 *     - connection endpoints where present;
 *     - nested member structure;
 *     - source provenance.
 *
 * Semantic classification occurs after parsing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - recognition of @model;
 *     - model annotation validity;
 *     - member-role validation;
 *     - name resolution;
 *     - type resolution;
 *     - generic bound validation;
 *     - inheritance validation;
 *     - interface/contract validation;
 *     - duplicate-name diagnostics;
 *     - graph endpoint resolution;
 *     - graph type compatibility;
 *     - capability validation;
 *     - resource validation;
 *     - constraint validation;
 *     - preference validation;
 *     - portability analysis;
 *     - dialect validation;
 *     - model lowering.
 *
 * The parser performs none of these semantic operations.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source text;
 *     lexical configuration;
 *     grammar version;
 *     dialect configuration;
 *
 * the parser must produce structurally equivalent parse trees.
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no mutable global state;
 *     - no I/O;
 *     - no hardware inspection;
 *     - no environment inspection;
 *     - no randomness;
 *     - no runtime callbacks.
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
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Only an explicit model declaration crosses this boundary.
 *
 * Ordinary expressions are NOT model constructs.
 *
 * This is important for deterministic composition with the wider AI grammar.
 * ============================================================================
 */

aiModelConstruct
    : aiModelDeclaration
    ;


/* ============================================================================
 * MODEL DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     @model MyModel {
 *     }
 *
 *     @model MyModel<T> {
 *     }
 *
 *     @model MyModel<T extends Numeric> {
 *     }
 *
 *     @model MyModel<T> extends BaseModel {
 *     }
 *
 *     @model MyModel<T> implements Trainable {
 *     }
 *
 * The exact semantic interpretation of the annotation is owned downstream.
 * ============================================================================
 */

aiModelDeclaration
    : modelAnnotation
      identifier
      modelGenericParameters?
      modelExtendsClause?
      modelImplementsClause*
      modelBody
    ;


/* ============================================================================
 * MODEL ANNOTATION
 * ============================================================================
 *
 * NANO_ANNOTATION contains the annotation spelling.
 *
 * Semantic analysis must validate that this is the model declaration role.
 *
 * The parser deliberately does not inspect annotation text.
 * ============================================================================
 */

modelAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * MODEL GENERICS
 * ============================================================================
 *
 * Declaration generics are reused from the repository's canonical generic
 * grammar.
 * ============================================================================
 */

modelGenericParameters
    : functionGenericParameters
    ;


/* ============================================================================
 * MODEL INHERITANCE
 * ============================================================================
 */

modelExtendsClause
    : EXTENDS
      modelQualifiedNameList
    ;


/* ============================================================================
 * MODEL IMPLEMENTATION / INTERFACE CONTRACTS
 * ============================================================================
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
      COMMA?
    ;


modelQualifiedName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/* ============================================================================
 * MODEL BODY
 * ============================================================================
 *
 * The body is intentionally unbounded by language-level cardinality.
 * ============================================================================
 */

modelBody
    : LBRACE
      modelMember*
      RBRACE
    ;


/* ============================================================================
 * MODEL MEMBER
 * ============================================================================
 *
 * Every model-specific member begins with the generic annotation boundary.
 *
 * Structural payload determines parsing.
 * Annotation spelling determines semantic role.
 *
 * This gives Zamani future extensibility without adding an ever-growing list
 * of AI lexer keywords.
 * ============================================================================
 */

modelMember
    : NANO_ANNOTATION
      modelMemberPayload
    ;


/* ============================================================================
 * MODEL MEMBER PAYLOAD
 * ============================================================================
 *
 * The alternatives are structurally distinguished:
 *
 *     connection -> ARROW
 *     typed member -> COLON
 *     block member -> LBRACE
 *     general member -> expression
 *
 * Invocation syntax is intentionally handled by the canonical expression
 * grammar rather than duplicated here.
 *
 * For example:
 *
 *     @operation forward(x);
 *
 * is represented by the general expression payload.
 *
 * This avoids ambiguity between:
 *
 *     identifier(argument-list)
 *
 * and:
 *
 *     expression
 *
 * ============================================================================
 */

modelMemberPayload
    : modelConnectionPayload
    | modelTypedMemberPayload
    | modelBlockMemberPayload
    | modelExpressionMemberPayload
    ;


/* ============================================================================
 * CONNECTION
 * ============================================================================
 *
 * Example:
 *
 *     @connect encoder.output -> classifier.input;
 *
 * ============================================================================
 */

modelConnectionPayload
    : modelEndpoint
      ARROW
      modelEndpoint
      SEMICOLON
    ;


modelEndpoint
    : modelQualifiedReference
    ;


modelQualifiedReference
    : identifier
      (
          DOT
          identifier
      )*
    ;


/* ============================================================================
 * TYPED MEMBER
 * ============================================================================
 *
 * Examples:
 *
 *     @input features: Tensor<Float>;
 *     @output result: Tensor<Float>;
 *     @parameter weights: Tensor<Float>;
 *     @state running_mean: Tensor<Float>;
 *     @component encoder: Encoder;
 *     @layer attention: Attention;
 *     @submodel encoder: EncoderModel;
 *
 * The annotation determines the semantic role.
 * ============================================================================
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
 * BLOCK MEMBER
 * ============================================================================
 *
 * Examples:
 *
 *     @component encoder {
 *         ...
 *     }
 *
 *     @operation forward {
 *         ...
 *     }
 *
 *     @pipeline body {
 *         ...
 *     }
 *
 * Nested model members remain structurally recursive and are not bounded by
 * a language-level depth constant.
 * ============================================================================
 */

modelBlockMemberPayload
    : identifier
      modelBody
    ;


/* ============================================================================
 * GENERAL EXPRESSION MEMBER
 * ============================================================================
 *
 * This is the extensibility boundary.
 *
 * Examples:
 *
 *     @bind activation = relu;
 *     @config precision = "bf16";
 *     @metadata family = "transformer";
 *     @operation forward = forward_function;
 *     @compose classifier = compose(encoder, head);
 *     @requires memory >= required_memory;
 *     @capability tensor.compute;
 *     @constraint latency <= latency_budget;
 *     @preference accelerator.tensor;
 *
 * Semantic interpretation belongs downstream.
 * ============================================================================
 */

modelExpressionMemberPayload
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * MODEL PORT BRIDGE
 * ============================================================================
 *
 * A reusable model port representation.
 *
 * This rule is intentionally not a second port grammar.
 *
 * The surrounding annotation supplies the semantic role.
 * ============================================================================
 */

modelPort
    : identifier
      COLON
      typeExpression
      modelMemberInitializer?
    ;


/* ============================================================================
 * MODEL REFERENCES
 * ============================================================================
 *
 * Model references are ordinary source references.
 *
 * They are NOT public model constructs.
 * ============================================================================
 */

modelReference
    : modelQualifiedReference
    ;


modelTypeReference
    : typeExpression
    ;


modelExpression
    : expression
    ;


modelConfigurationValue
    : expression
    ;


modelMetadataValue
    : expression
    ;


/* ============================================================================
 * SEMANTIC CONTRACT BRIDGES
 * ============================================================================
 *
 * These rules provide stable names for semantic consumers without creating
 * duplicate resource/capability/constraint grammars.
 * ============================================================================
 */

modelRequirement
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


modelCapability
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


modelConstraint
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


modelPreference
    : NANO_ANNOTATION
      expression
      SEMICOLON
    ;


/* ============================================================================
 * MODEL OPERATION BRIDGE
 * ============================================================================
 *
 * The operation body/value is deliberately represented by the canonical
 * expression grammar.
 *
 * Algorithm names are semantic/library concepts rather than grammar keywords.
 * ============================================================================
 */

modelOperation
    : NANO_ANNOTATION
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * MODEL COMPOSITION BRIDGE
 * ============================================================================
 */

modelComposition
    : NANO_ANNOTATION
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * RESOURCE / CAPABILITY / CONSTRAINT / PORTABILITY EXPRESSION BRIDGES
 * ============================================================================
 */

modelResourceExpression
    : expression
    ;


modelCapabilityExpression
    : expression
    ;


modelConstraintExpression
    : expression
    ;


modelPortabilityExpression
    : expression
    ;


/* ============================================================================
 * EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future AI dialects may reuse the existing structural model-member forms.
 *
 * They MUST register semantic meaning separately.
 *
 * They MUST NOT silently change the meaning of an existing annotation.
 * ============================================================================
 */

modelExtensionMember
    : NANO_ANNOTATION
      modelExtensionPayload
    ;


modelExtensionPayload
    : modelTypedMemberPayload
    | modelBlockMemberPayload
    | modelExpressionMemberPayload
    | modelConnectionPayload
    ;


/* ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This grammar is complete when:
 *
 *     [x] models.g4 remains the canonical model grammar filename.
 *     [x] AIModels is the sole canonical model grammar identity.
 *     [x] ZamaniLexer is the parser-facing vocabulary.
 *     [x] Types is the canonical type authority.
 *     [x] Expressions is the canonical expression authority.
 *     [x] FunctionGenerics is the canonical declaration-generic authority.
 *     [x] No duplicate type grammar exists here.
 *     [x] No duplicate expression grammar exists here.
 *     [x] No duplicate lexer rules exist here.
 *     [x] Model declarations have an explicit syntactic boundary.
 *     [x] Model members have an explicit syntactic boundary.
 *     [x] Model graph edges are structurally represented.
 *     [x] Nested model regions are supported.
 *     [x] Model generic arity is not artificially bounded.
 *     [x] Model member count is not artificially bounded.
 *     [x] Model nesting is not artificially bounded.
 *     [x] Model graph size is not artificially bounded.
 *     [x] No hardware limits are encoded.
 *     [x] No device identifiers are encoded.
 *     [x] No vendor is encoded.
 *     [x] No AI framework is encoded.
 *     [x] No quantum gate list is encoded.
 *     [x] No physical quantum resources are encoded.
 *     [x] quantum::ir remains downstream and canonical.
 *     [x] No AI-specific parallel IR is introduced.
 *     [x] No Rust actions are embedded.
 *     [x] No unsafe Rust is required.
 *     [x] Source order can be preserved.
 *     [x] Source spans can be preserved.
 *     [x] Semantic interpretation remains downstream.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax:
 *
 *     @model Empty {}
 *
 *     @model Linear {
 *         @input x: Tensor<Float>;
 *         @output y: Tensor<Float>;
 *     }
 *
 *     @model Network<T extends Numeric> {
 *         @input x: Tensor<T>;
 *         @output y: Tensor<T>;
 *     }
 *
 *     @model Pipeline {
 *         @component encoder: Encoder;
 *         @component classifier: Classifier;
 *         @connect encoder.output -> classifier.input;
 *     }
 *
 *     @model Hybrid {
 *         @input state: QuantumState;
 *         @output result: Tensor<Float>;
 *         @requires memory >= required_memory;
 *         @capability quantum.measurement;
 *     }
 *
 *     @model Portable {
 *         @requires qubits >= required_qubits;
 *         @requires memory >= required_memory;
 *         @capability tensor.compute;
 *         @constraint latency <= latency_budget;
 *         @preference accelerator.tensor;
 *     }
 *
 *     @model Nested {
 *         @component outer {
 *             @component inner {
 *                 @input value: Tensor<Float>;
 *             }
 *         }
 *     }
 *
 * Negative syntax:
 *
 *     @model
 *
 *     @model Name
 *
 *     @model Name <
 *
 *     @model Name<T,> // accepted only if FunctionGenerics accepts trailing comma
 *                     // according to its canonical contract
 *
 *     @model Name {
 *         @input x:
 *     }
 *
 *     @model Name {
 *         @connect a ->
 *     }
 *
 *     @model Name {
 *         @input x Tensor<Float>;
 *     }
 *
 * Semantic-only failures, which MUST remain outside this grammar:
 *
 *     duplicate member names;
 *     unknown model references;
 *     unknown generic bounds;
 *     incompatible graph endpoints;
 *     unsatisfied resources;
 *     unavailable capabilities;
 *     invalid portability requirements;
 *     invalid framework/dialect registrations.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST demonstrate that the grammar imposes no artificial language
 * ceiling on:
 *
 *     - model members;
 *     - nested model members;
 *     - generic parameters;
 *     - generic bounds;
 *     - graph connections;
 *     - model declarations;
 *     - expression complexity.
 *
 * Tests may use progressively larger generated programs subject only to the
 * actual parser/compiler test harness resources.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *              |
 *              v
 *         tokenVocab
 *              |
 *              v
 *           AIModels
 *
 *     grammar/types/types.g4
 *              |
 *              v
 *       typeExpression
 *
 *     grammar/expressions/expressions.g4
 *              |
 *              v
 *          expression
 *
 *     grammar/functions/generics.g4
 *              |
 *              v
 *    functionGenericParameters
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
 *     semantic model analysis
 *         |
 *         +--> types
 *         +--> effects
 *         +--> resources
 *         +--> capabilities
 *         +--> portability
 *         +--> AI semantics
 *         |
 *         v
 *     canonical semantic model
 *         |
 *         +--> classical lowering
 *         +--> quantum lowering
 *         +--> HDL/hardware lowering
 *         +--> distributed lowering
 *         +--> accelerator lowering
 *         |
 *         v
 *     canonical IR
 *
 * QUANTUM:
 *
 *     AI model
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar does not introduce another quantum representation.
 *
 * HARDWARE:
 *
 *     AI model
 *          |
 *          v
 *     capability/resource semantics
 *          |
 *          v
 *     hardware/resource analysis
 *          |
 *          v
 *     target realization
 *
 * The model grammar remains target-independent.
 *
 * ============================================================================
 */