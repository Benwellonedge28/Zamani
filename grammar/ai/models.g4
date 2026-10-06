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
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the sole canonical parser grammar for source-level model
 * declarations in the AI domain.
 *
 * It defines the STRUCTURE of a model declaration.
 *
 * It does not define model execution, model algorithms, tensor execution,
 * training, inference, optimization, scheduling, placement, hardware
 * selection, resource allocation, device discovery, or runtime behavior.
 *
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *     - model declaration syntax;
 *     - model declaration identity;
 *     - model declaration generic parameters;
 *     - model inheritance syntax;
 *     - model interface/contract references;
 *     - model body structure;
 *     - model member structure;
 *     - model member annotation attachment;
 *     - model typed members;
 *     - model expression members;
 *     - model nested regions;
 *     - model graph connection syntax;
 *     - model source-level composition boundaries.
 *
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     - lexical rules;
 *     - annotation syntax;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - expression syntax;
 *     - type syntax;
 *     - generic type application;
 *     - generic substitution;
 *     - tensor syntax;
 *     - dataset syntax;
 *     - training syntax;
 *     - inference syntax;
 *     - agent syntax;
 *     - differentiation syntax;
 *     - pipeline syntax;
 *     - accelerator syntax;
 *     - deployment syntax;
 *     - reasoning syntax;
 *     - knowledge syntax;
 *     - uncertainty syntax;
 *     - learning semantics;
 *     - adaptation semantics;
 *     - effect semantics;
 *     - resource resolution;
 *     - capability resolution;
 *     - policy evaluation;
 *     - security authorization;
 *     - provenance semantics;
 *     - classical IR;
 *     - AI-specific IR;
 *     - quantum IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - routing;
 *     - scheduling;
 *     - resilience;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The required direction is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
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
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> generic analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          +--> portability
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical semantics
 *          +--> quantum semantics
 *          +--> HDL/hardware semantics
 *          +--> distributed semantics
 *          +--> accelerator semantics
 *          |
 *          +-------------------------------+
 *                                          |
 *                                          v
 *                               target-independent IR
 *                                          |
 *                          +---------------+---------------+
 *                          |               |               |
 *                          v               v               v
 *                      classical       quantum::ir     HDL/hardware
 *                          |               |               |
 *                          +---------------+---------------+
 *                                          |
 *                                          v
 *                                  optimization
 *                                          |
 *                                  specialization
 *                                          |
 *                                    lowering
 *                                          |
 *                              routing / scheduling
 *                                          |
 *                              resilience / recovery
 *                                          |
 *                                  ZQN / HAL
 *                                          |
 *                                  target realization
 *
 *
 * IMPORTANT
 * ---------
 *
 * This grammar creates no IR.
 *
 * Quantum computation remains subject to the canonical:
 *
 *     quantum::ir
 *
 * semantic boundary.
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * `grammar/ai/models.g4` is the canonical source grammar for model
 * declarations.
 *
 * The repository also contains:
 *
 *     grammar/ai/model.g4
 *
 * That file historically declares the same `AIModels` grammar identity.
 *
 * It MUST NOT participate in the canonical ANTLR composition.
 *
 * `grammar/ai/ai.g4` MUST import:
 *
 *     AIModels
 *
 * from this file only.
 *
 * `grammar/ai/model.g4` must remain a deprecated/compatibility artifact until
 * it is removed through the repository compatibility process.
 *
 * It MUST NOT become a second production grammar.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It defines no lexer rules.
 *
 * Annotation syntax is NOT implemented locally.
 *
 * The canonical annotation system is:
 *
 *     grammar/core/annotations.g4
 *
 * whose public rule is:
 *
 *     annotation
 *
 * and whose lexical marker is:
 *
 *     AT
 *
 * Therefore this file MUST NOT use:
 *
 *     NANO_ANNOTATION
 *
 * and MUST NOT define:
 *
 *     MODEL_ANNOTATION
 *     INPUT_ANNOTATION
 *     OUTPUT_ANNOTATION
 *     COMPONENT_ANNOTATION
 *
 * as lexer tokens.
 *
 * Annotation identity is semantic.
 *
 *
 * ============================================================================
 * ANNOTATION CONTRACT
 * ============================================================================
 *
 * Model declarations use the canonical annotation grammar.
 *
 * Examples:
 *
 *     @model Encoder {
 *     }
 *
 *     @model(name = "Encoder") Encoder {
 *     }
 *
 *     @model Encoder<T> {
 *     }
 *
 * Model member annotations use the same canonical annotation system:
 *
 *     @input
 *     @output
 *     @parameter
 *     @state
 *     @component
 *     @layer
 *     @submodel
 *     @operation
 *     @config
 *     @metadata
 *     @requires(...)
 *     @capability(...)
 *     @constraint(...)
 *     @preference(...)
 *
 * This grammar parses their structure.
 *
 * Semantic analysis determines whether an annotation is legal for the
 * particular model or member context.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The repository's canonical type grammar is:
 *
 *     grammar/types/types.g4
 *
 * Its actual parser grammar identity is:
 *
 *     Type
 *
 * and its public type rule is:
 *
 *     typeExpression
 *
 * Therefore this grammar imports:
 *
 *     Type
 *
 * and consumes:
 *
 *     typeExpression
 *
 * It does not redefine any type constructor.
 *
 * Model members can therefore use:
 *
 *     classical types
 *     tensor types
 *     quantum types
 *     hardware types
 *     resource types
 *     capability types
 *     distributed types
 *     user-defined types
 *     future domain types
 *
 * without creating an AI-specific type system.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Canonical expression syntax belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * Public expression rule:
 *
 *     expression
 *
 * This file consumes `expression` and does not redefine:
 *
 *     arithmetic
 *     calls
 *     indexing
 *     member access
 *     assignment
 *     literals
 *     lambdas
 *     closures
 *     ranges
 *     conditionals
 *     patterns
 *     guards
 *     queries
 *     reasoning
 *     uncertainty
 *     policy expressions
 *     quantum expressions
 *     compile-time expressions
 *
 *
 * ============================================================================
 * GENERIC CONTRACT
 * ============================================================================
 *
 * Generic declaration syntax belongs to:
 *
 *     grammar/functions/generics.g4
 *
 * Public rule:
 *
 *     functionGenericParameters
 *
 * This grammar reuses that rule for model declaration parameters.
 *
 * Generic type application remains owned by the canonical type grammar.
 *
 * Therefore:
 *
 *     @model Network<T extends Numeric> {
 *     }
 *
 * uses `functionGenericParameters` for declaration syntax, while:
 *
 *     Tensor<T>
 *
 * remains a type-expression concern.
 *
 * This distinction MUST remain intact.
 *
 *
 * ============================================================================
 * MODEL SEMANTIC IDENTITY
 * ============================================================================
 *
 * The parser does not hard-code the semantic spelling of the model annotation
 * into a lexer token.
 *
 * Instead:
 *
 *     annotation
 *
 * is parsed structurally.
 *
 * Semantic analysis recognizes the model declaration role.
 *
 * This permits the language to retain an extensible annotation system without
 * creating a closed keyword catalogue for every possible model feature.
 *
 *
 * ============================================================================
 * MODEL DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     @model Name {
 *         ...
 *     }
 *
 * Generic form:
 *
 *     @model Name<T> {
 *         ...
 *     }
 *
 * Inheritance form:
 *
 *     @model Name<T> extends Base {
 *         ...
 *     }
 *
 * Interface form:
 *
 *     @model Name implements Trainable {
 *         ...
 *     }
 *
 * Combined form:
 *
 *     @model Name<T>
 *         extends Base
 *         implements Trainable, Serializable
 *     {
 *         ...
 *     }
 *
 * The parser preserves source ordering.
 *
 * Semantic analysis determines whether inheritance/interface composition is
 * legal.
 *
 *
 * ============================================================================
 * MODEL GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic arity is not language-limited.
 *
 * Generic bounds are not language-limited.
 *
 * Examples:
 *
 *     <T>
 *
 *     <T, U>
 *
 *     <T extends Numeric>
 *
 *     <T extends Numeric + Ordered>
 *
 *     <T extends Numeric, U extends Serializable>
 *
 * The generic grammar owns the details.
 *
 * This file only establishes the model declaration boundary.
 *
 *
 * ============================================================================
 * INHERITANCE CONTRACT
 * ============================================================================
 *
 * Model inheritance is source-level composition.
 *
 * It does NOT imply:
 *
 *     hardware inheritance
 *     device inheritance
 *     accelerator inheritance
 *     physical topology inheritance
 *     quantum topology inheritance
 *     resource allocation
 *
 * Semantic analysis determines whether a referenced model is a valid parent.
 *
 *
 * ============================================================================
 * INTERFACE / CONTRACT CONTRACT
 * ============================================================================
 *
 * `implements` identifies source-level interfaces/contracts.
 *
 * It does not itself establish:
 *
 *     runtime authorization
 *     resource availability
 *     hardware compatibility
 *     target support
 *
 * Those properties are checked by semantic analysis.
 *
 *
 * ============================================================================
 * MODEL BODY CONTRACT
 * ============================================================================
 *
 * A model body contains zero or more model members.
 *
 * Repetition is intentionally unbounded by language-level constants.
 *
 * There is no:
 *
 *     MAX_MODEL_MEMBERS
 *     MAX_MODEL_DEPTH
 *     MAX_MODEL_COMPONENTS
 *     MAX_MODEL_PORTS
 *     MAX_MODEL_CONNECTIONS
 *
 * Parser/compiler resource exhaustion remains an implementation concern, not
 * a language semantic ceiling.
 *
 *
 * ============================================================================
 * MODEL MEMBER CONTRACT
 * ============================================================================
 *
 * Every model-specific member begins with one canonical annotation.
 *
 * Structural payload then determines its syntactic form.
 *
 * Examples:
 *
 *     @input x: Tensor<Float>;
 *
 *     @output y: Tensor<Float>;
 *
 *     @parameter weights: Tensor<Float>;
 *
 *     @state running: Tensor<Float>;
 *
 *     @component encoder: Encoder;
 *
 *     @layer attention: Attention;
 *
 *     @submodel encoder: Encoder;
 *
 *     @operation forward = forward_function;
 *
 *     @config precision = "portable";
 *
 *     @metadata family = "transformer";
 *
 *     @requires capability("tensor.compute");
 *
 *     @capability tensor.compute;
 *
 *     @constraint latency <= latency_budget;
 *
 *     @preference accelerator.tensor;
 *
 *     @connect encoder.output -> classifier.input;
 *
 * The annotation determines semantic role.
 *
 * This grammar does not maintain a closed enumeration of those roles.
 *
 *
 * ============================================================================
 * MEMBER STRUCTURAL FORMS
 * ============================================================================
 *
 * The payload forms are:
 *
 *     connection
 *     typed declaration
 *     nested block
 *     expression member
 *
 * They are structurally distinguishable by:
 *
 *     ->
 *     :
 *     {
 *     expression terminator
 *
 * respectively.
 *
 *
 * ============================================================================
 * TYPED MODEL MEMBERS
 * ============================================================================
 *
 * Canonical:
 *
 *     @input x: Tensor<Float>;
 *
 *     @output result: Tensor<Float>;
 *
 *     @parameter weight: Tensor<Float> = initialWeight;
 *
 *     @state state: QuantumState;
 *
 *     @component encoder: Encoder;
 *
 *     @layer attention: Attention;
 *
 * A member initializer is an ordinary canonical expression.
 *
 * No model-specific expression language is introduced.
 *
 *
 * ============================================================================
 * NESTED MODEL MEMBERS
 * ============================================================================
 *
 * Nested structural composition is supported:
 *
 *     @component outer {
 *         @component inner {
 *             @input value: Tensor<Float>;
 *         }
 *     }
 *
 * Nesting is not bounded by a language-level constant.
 *
 * Semantic analysis may impose implementation/resource policies where
 * necessary, but such policies MUST NOT become universal grammar constants.
 *
 *
 * ============================================================================
 * MODEL GRAPH CONTRACT
 * ============================================================================
 *
 * A model connection:
 *
 *     @connect encoder.output -> classifier.input;
 *
 * represents a SOURCE-LEVEL MODEL GRAPH EDGE.
 *
 * It does not represent:
 *
 *     network topology
 *     hardware wiring
 *     GPU interconnect
 *     QPU coupling
 *     physical route
 *     scheduler dependency
 *     memory-bank topology
 *
 * Physical realization belongs downstream.
 *
 *
 * ============================================================================
 * GRAPH ENDPOINT CONTRACT
 * ============================================================================
 *
 * An endpoint is a qualified source reference:
 *
 *     encoder.output
 *
 *     encoder.block.output
 *
 *     classifier.input
 *
 * The grammar does not resolve the endpoint.
 *
 * Semantic analysis determines:
 *
 *     whether the source exists;
 *     whether the destination exists;
 *     whether both belong to the model;
 *     whether the connection is legal;
 *     whether the source type is compatible with the destination type;
 *     whether effects/capabilities/resources are compatible;
 *     whether the connection is portable.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Model declarations may carry resource intent through ordinary model-member
 * annotations and expressions.
 *
 * Examples:
 *
 *     @requires memory >= required_memory;
 *
 *     @requires qubits >= required_qubits;
 *
 *     @requires capability("tensor.compute");
 *
 *     @requires capability("quantum.measurement");
 *
 *     @constraint latency <= latency_budget;
 *
 *     @preference accelerator.tensor;
 *
 * These constructs express SOURCE INTENT.
 *
 * They do not allocate resources.
 *
 * They do not select:
 *
 *     GPU 0
 *     CPU 3
 *     QPU 7
 *     device 2
 *
 * Capability resolution, resource analysis, placement and scheduling are
 * downstream responsibilities.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Model members may refer semantically to operations that have effects.
 *
 * Examples include:
 *
 *     learning
 *     adaptation
 *     measurement
 *     network
 *     foreign
 *     native
 *     distributed
 *     randomness
 *     mutation
 *     simulation
 *     reflection
 *
 * The grammar does not validate or grant those effects.
 *
 * The semantic effect subsystem remains authoritative.
 *
 *
 * ============================================================================
 * CONTRACT / VALIDATION INTEGRATION
 * ============================================================================
 *
 * Model members may carry annotations describing:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This grammar parses the annotation structurally.
 *
 * Contract semantics remain owned by:
 *
 *     grammar/validation/
 *
 * and its semantic consumers.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Model declarations may be subject to policies such as:
 *
 *     portability
 *     security
 *     execution
 *     resource
 *     adaptation
 *     deployment
 *     simulation
 *
 * Policy syntax is not duplicated here.
 *
 * Model annotations and expressions provide the structural bridge.
 *
 * Policy evaluation occurs downstream.
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     declaration span
 *     annotation span
 *     declaration name
 *     generic parameter order
 *     inheritance order
 *     interface order
 *     member order
 *     endpoint order
 *     source expression structure
 *
 * so that semantic provenance can record:
 *
 *     source
 *     interpretation
 *     derivation
 *     transformation
 *     decision
 *     verification
 *
 * Provenance semantics remain outside this grammar.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A model may contain or reference quantum values through canonical types,
 * expressions and semantic capabilities.
 *
 * Examples:
 *
 *     @input state: QuantumState;
 *
 *     @requires capability("quantum.measurement");
 *
 *     @operation measure = measure_state;
 *
 * This grammar does not define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     physical qubits
 *     QubitId
 *     coupling maps
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *
 * Quantum operations remain owned by the quantum subsystem.
 *
 * The semantic pipeline is:
 *
 *     model source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Model declarations may reference:
 *
 *     hardware types
 *     abstract resources
 *     capabilities
 *     execution requirements
 *     hardware intent
 *
 * This grammar does not encode:
 *
 *     fixed signal widths
 *     fixed memory sizes
 *     fixed register widths
 *     fixed FPGA resources
 *     fixed ASIC technology
 *     physical addresses
 *     physical device IDs
 *     physical wiring
 *
 * HDL/hardware realization remains downstream.
 *
 *
 * ============================================================================
 * DISTRIBUTED / CONCURRENT INTEGRATION
 * ============================================================================
 *
 * Model structure can participate semantically in:
 *
 *     tasks
 *     actors
 *     pipelines
 *     channels
 *     distributed computation
 *     collective operations
 *
 * Model syntax does not create another concurrency system.
 *
 * Existing concurrency/distributed grammars remain authoritative.
 *
 *
 * ============================================================================
 * AI DOMAIN INTEGRATION
 * ============================================================================
 *
 * `models.g4` intentionally does not own:
 *
 *     datasets
 *     tensors
 *     training
 *     inference
 *     agents
 *     differentiation
 *     pipelines
 *     accelerators
 *     deployment
 *
 * Those remain independent AI grammar components.
 *
 * Their source-level constructs can reference model declarations through
 * ordinary canonical names, types and expressions.
 *
 *
 * ============================================================================
 * DIALECT / FRAMEWORK / VENDOR CONTRACT
 * ============================================================================
 *
 * The core model grammar is framework-neutral.
 *
 * It does not reserve syntax for:
 *
 *     CUDA
 *     ROCm
 *     TensorFlow
 *     PyTorch
 *     JAX
 *     ONNX
 *     OpenVINO
 *     vendor-specific accelerators
 *     cloud platforms
 *     model-family names
 *     runtime-specific execution systems
 *
 * External integrations belong to:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * A framework or vendor extension must not silently redefine the core model
 * grammar.
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Model syntax is independent of machine scale.
 *
 * The grammar imposes NO universal maximum on:
 *
 *     model declarations
 *     generic parameters
 *     generic bounds
 *     model members
 *     nested members
 *     graph connections
 *     model depth
 *     model composition
 *     input count
 *     output count
 *     parameter count
 *     tensor rank
 *     tensor dimensions
 *     dataset size
 *     workers
 *     processors
 *     accelerators
 *     nodes
 *     devices
 *     qubits
 *     memory
 *     network size
 *
 * The following MUST NOT appear as language constants:
 *
 *     MAX_MODELS
 *     MAX_MODEL_MEMBERS
 *     MAX_MODEL_DEPTH
 *     MAX_MODEL_COMPONENTS
 *     MAX_MODEL_PARAMETERS
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * Practical parser/compiler/runtime limits are implementation/resource
 * constraints and must not become source-language ceilings.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST remains domain-neutral.
 *
 * A model AST representation must preserve at least:
 *
 *     declaration source span
 *     model annotation
 *     model name
 *     generic parameter ordering
 *     generic parameter spans
 *     inheritance ordering
 *     implementation ordering
 *     member ordering
 *     member annotation structure
 *     member names
 *     member types
 *     member initializers
 *     nested member structure
 *     connection endpoints
 *     source ordering
 *     source spans
 *
 * No AI-specific parallel AST is permitted merely because this grammar is in
 * the AI directory.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     annotation identity
 *     annotation validity
 *     name resolution
 *     model resolution
 *     generic resolution
 *     generic bound validation
 *     inheritance validation
 *     interface validation
 *     member-role validation
 *     duplicate member validation
 *     endpoint resolution
 *     graph validation
 *     type compatibility
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     portability analysis
 *     provenance
 *     dialect validation
 *     lowering
 *
 * This parser performs none of those semantic operations.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * parsing must produce equivalent parse structure.
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no runtime callbacks
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no environment inspection
 *     no randomness
 *     no scheduler interaction
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural failures such as:
 *
 *     missing model annotation
 *     missing model name
 *     malformed generic parameter list
 *     malformed inheritance clause
 *     malformed implementation clause
 *     missing model body
 *     malformed typed member
 *     malformed connection
 *     missing expression terminator
 *     malformed nested member
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unknown model
 *     duplicate model
 *     duplicate member
 *     unknown annotation
 *     invalid annotation context
 *     unresolved model reference
 *     invalid inheritance
 *     invalid implementation
 *     incompatible endpoint types
 *     unavailable capability
 *     unsatisfied resource requirement
 *     invalid policy
 *     invalid effect
 *     non-portable target requirement
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical model source forms remain structurally supported:
 *
 *     @model Name {
 *     }
 *
 *     @model Name<T> {
 *     }
 *
 *     @model Name<T extends Numeric> {
 *     }
 *
 *     @model Name extends Base {
 *     }
 *
 *     @model Name implements Interface {
 *     }
 *
 *     @model Name {
 *         @input value: Tensor<Float>;
 *     }
 *
 *     @model Name {
 *         @connect encoder.output -> classifier.input;
 *     }
 *
 * The transition away from the obsolete `NANO_ANNOTATION` representation is
 * a grammar-architecture correction, not a semantic expansion of the model
 * language.
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports only canonical authorities needed for model syntax:
 *
 *     Annotations
 *         -> canonical annotation structure
 *
 *     Type
 *         -> canonical typeExpression
 *
 *     Expressions
 *         -> canonical expression
 *
 *     FunctionGenerics
 *         -> canonical generic declaration syntax
 *
 * The dependency direction is:
 *
 *     AIModels
 *        |
 *        +--> Annotations
 *        +--> Type
 *        +--> Expressions
 *        +--> FunctionGenerics
 *
 * AIModels does not modify or redefine any imported authority.
 *
 *
 * ============================================================================
 * INTEGRATION WITH AI.G4
 * ============================================================================
 *
 * `grammar/ai/ai.g4` is the AI composition facade.
 *
 * It imports:
 *
 *     AIModels
 *
 * and dispatches:
 *
 *     aiModelConstruct
 *
 * through:
 *
 *     aiConstruct
 *
 * `ai.g4` MUST NOT duplicate:
 *
 *     aiModelDeclaration
 *     modelBody
 *     modelMember
 *     modelConnectionPayload
 *
 * or any other model production owned here.
 *
 *
 * ============================================================================
 * INTEGRATION WITH ZAMANI.G4
 * ============================================================================
 *
 * The root grammar must reach the AI domain through the canonical AI
 * composition boundary.
 *
 * The intended direction is:
 *
 *     Zamani.g4
 *          |
 *          v
 *     AI
 *          |
 *          v
 *     AIModels
 *
 * `Zamani.g4` must not duplicate model syntax.
 *
 *
 * ============================================================================
 * INTEGRATION WITH AST
 * ============================================================================
 *
 * The AST layer consumes the parser contexts produced by this grammar.
 *
 * The model AST builder must map:
 *
 *     aiModelDeclaration
 *         -> domain-neutral declaration/model node
 *
 *     modelMember
 *         -> domain-neutral member/attribute node
 *
 *     modelConnectionPayload
 *         -> domain-neutral graph/relationship node
 *
 * The AST builder must preserve source spans.
 *
 * No parser rule should require modification merely because a downstream
 * target changes.
 *
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTIC MODEL
 * ============================================================================
 *
 * Semantic analysis consumes the AST and resolves:
 *
 *     annotations
 *     names
 *     types
 *     generic bounds
 *     interfaces
 *     graph endpoints
 *     resources
 *     capabilities
 *     effects
 *     policies
 *     contracts
 *     provenance
 *
 * This grammar remains independent of those implementations.
 *
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Model declarations do not lower directly to a model-specific IR merely
 * because they were parsed by this grammar.
 *
 * The semantic model determines the required canonical operations.
 *
 * Lowering may eventually produce:
 *
 *     classical semantic operations
 *     tensor operations
 *     distributed operations
 *     hardware intent
 *     quantum operations
 *
 * When quantum computation is involved, the canonical boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT define another quantum IR.
 *
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust actions.
 *
 * It requires:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * in the consuming frontend/toolchain.
 *
 * Generated parser integration must use safe Rust only.
 *
 * This grammar requires no `unsafe`.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Recommended test location:
 *
 *     grammar/tests/ai/models/
 *
 *
 * POSITIVE TESTS
 * --------------
 *
 *     @model Empty {}
 *
 *     @model Linear {
 *         @input x: Tensor<Float>;
 *         @output y: Tensor<Float>;
 *     }
 *
 *     @model Generic<T> {
 *         @input x: Tensor<T>;
 *     }
 *
 *     @model Bounded<T extends Numeric> {
 *         @input x: Tensor<T>;
 *     }
 *
 *     @model MultiBound<T extends Numeric + Ordered> {
 *         @input x: Tensor<T>;
 *     }
 *
 *     @model Inherited<T>
 *         extends BaseModel
 *     {
 *         @input x: Tensor<T>;
 *     }
 *
 *     @model Contracted
 *         implements Trainable, Serializable
 *     {
 *         @input x: Tensor<Float>;
 *     }
 *
 *     @model Pipeline {
 *         @component encoder: Encoder;
 *         @component classifier: Classifier;
 *         @connect encoder.output -> classifier.input;
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
 *     @model Hybrid {
 *         @input state: QuantumState;
 *         @output result: Tensor<Float>;
 *         @requires(memory >= required_memory);
 *         @requires(capability("quantum.measurement"));
 *     }
 *
 *     @model Portable {
 *         @requires(memory >= required_memory);
 *         @requires(capability("tensor.compute"));
 *         @constraint(latency <= latency_budget);
 *         @preference(accelerator.tensor);
 *     }
 *
 *     @model Annotated(
 *         family = "portable",
 *         domain = classical::numeric,
 *     ) {
 *         @metadata(tags = ["portable", "deterministic"]);
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE SYNTAX TESTS
 * ============================================================================
 *
 *     @model
 *
 *     @model Name
 *
 *     @model Name <
 *
 *     @model Name {}
 *         trailing_invalid_member_syntax
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
 *     @model Name {
 *         @input x: ;
 *     }
 *
 *
 * ============================================================================
 * SEMANTIC-ONLY FAILURE TESTS
 * ============================================================================
 *
 * These must NOT be rejected merely by this grammar:
 *
 *     duplicate model names
 *     duplicate member names
 *     unknown model parents
 *     unknown interfaces
 *     invalid generic bounds
 *     incompatible graph endpoints
 *     unavailable capabilities
 *     unsatisfied resource requirements
 *     invalid policies
 *     invalid effects
 *     target-specific incompatibility
 *
 * These belong to semantic analysis.
 *
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test combinations of:
 *
 *     model + generic
 *     model + inheritance
 *     model + interfaces
 *     model + annotations
 *     model + nested members
 *     model + expressions
 *     model + resources
 *     model + capabilities
 *     model + contracts
 *     model + policies
 *     model + provenance
 *     model + classical types
 *     model + tensor types
 *     model + quantum types
 *     model + hardware types
 *     model + distributed types
 *     model + foreign/interoperability values
 *
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The test suite must demonstrate that the grammar has no artificial
 * language-level ceiling on:
 *
 *     model declarations
 *     member count
 *     generic parameter count
 *     generic bound count
 *     nesting
 *     graph connections
 *     qualified endpoint depth
 *     expression complexity
 *     annotation arguments
 *
 * Test sizes are constrained only by actual test-harness resources.
 *
 * Those test limits MUST NOT become grammar constants.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 *     MAX_MODEL_MEMBERS
 *     MAX_MODEL_DEPTH
 *     MAX_PARAMETER_COUNT
 *
 * It contains no:
 *
 *     device identifiers
 *     physical addresses
 *     physical qubit identifiers
 *     vendor identifiers
 *     accelerator identifiers
 *     fixed topology
 *     fixed memory banks
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] AIModels is the only canonical model grammar identity.
 *
 *     [ ] model.g4 is not part of canonical ANTLR composition.
 *
 *     [ ] Canonical ZamaniLexer is consumed.
 *
 *     [ ] Canonical annotation syntax is consumed.
 *
 *     [ ] NANO_ANNOTATION is not used.
 *
 *     [ ] Canonical typeExpression is consumed.
 *
 *     [ ] Canonical expression is consumed.
 *
 *     [ ] Canonical generic declaration syntax is consumed.
 *
 *     [ ] No type grammar is duplicated.
 *
 *     [ ] No expression grammar is duplicated.
 *
 *     [ ] No annotation grammar is duplicated.
 *
 *     [ ] No identifier grammar is duplicated.
 *
 *     [ ] Model declarations have an explicit syntactic boundary.
 *
 *     [ ] Model members have an explicit syntactic boundary.
 *
 *     [ ] Typed members are supported.
 *
 *     [ ] Initializers are supported.
 *
 *     [ ] Nested model members are supported.
 *
 *     [ ] Model graph edges are supported.
 *
 *     [ ] Inheritance is supported.
 *
 *     [ ] Interface/contract references are supported.
 *
 *     [ ] Generic declarations are supported.
 *
 *     [ ] Generic arity is not artificially bounded.
 *
 *     [ ] Model member count is not artificially bounded.
 *
 *     [ ] Model nesting is not artificially bounded.
 *
 *     [ ] Graph size is not artificially bounded.
 *
 *     [ ] Hardware capacity is not encoded.
 *
 *     [ ] Quantum physical topology is not encoded.
 *
 *     [ ] Vendor syntax is not encoded.
 *
 *     [ ] Framework syntax is not encoded.
 *
 *     [ ] No AI-specific IR is introduced.
 *
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *
 *     [ ] No Rust actions are embedded.
 *
 *     [ ] No semantic predicates are embedded.
 *
 *     [ ] No filesystem access exists.
 *
 *     [ ] No network access exists.
 *
 *     [ ] No hardware access exists.
 *
 *     [ ] Parsing is deterministic.
 *
 *     [ ] Source ordering is preserved.
 *
 *     [ ] Source spans can be preserved.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Semantic-negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Rust 1.97 / 1.97.1 integration passes.
 *
 *     [ ] Safe-Rust requirements remain satisfied.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file defines the syntax of MODEL DECLARATIONS.
 *
 * It does not define model implementation.
 *
 * It does not define AI algorithms.
 *
 * It does not define tensor execution.
 *
 * It does not define hardware.
 *
 * It does not define resources.
 *
 * It does not define capabilities.
 *
 * It does not define security authorization.
 *
 * It does not define policies.
 *
 * It does not define runtime behavior.
 *
 * It does not define quantum hardware.
 *
 * It does not define quantum topology.
 *
 * It does not define QEC.
 *
 * It does not define ZQN.
 *
 * It does not define HAL.
 *
 * It therefore remains compatible with:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     quantum simulators
 *     QPUs
 *     heterogeneous systems
 *     clusters
 *     supercomputers
 *     distributed systems
 *     cloud systems
 *     future computational targets
 *
 * without making today's hardware characteristics part of the language.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR IDENTITY
 * ============================================================================
 */

parser grammar AIModels;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL IMPORTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `Type` is the actual canonical parser grammar identity of
 * grammar/types/types.g4.
 *
 * Do not replace it with `Types`.
 */
import
    Annotations,
    Type,
    Expressions,
    FunctionGenerics
;


/*
 * ============================================================================
 * PUBLIC MODEL COMPOSITION ENTRY
 * ============================================================================
 *
 * This is the only public AI model-domain entry point.
 */
aiModelConstruct
    : aiModelDeclaration
    ;


/*
 * ============================================================================
 * MODEL DECLARATION
 * ============================================================================
 *
 * The annotation itself is canonical annotation syntax.
 *
 * Semantic analysis determines that the annotation identifies a model
 * declaration.
 */
aiModelDeclaration
    : modelAnnotation
      IDENTIFIER
      modelGenericParameters?
      modelExtendsClause?
      modelImplementsClause*
      modelBody
    ;


/*
 * ============================================================================
 * MODEL ANNOTATION
 * ============================================================================
 *
 * Canonical annotation syntax is owned by grammar/core/annotations.g4.
 *
 * The semantic layer must validate that the annotation is appropriate for a
 * model declaration.
 *
 * This intentionally permits annotation arguments without creating a second
 * annotation grammar.
 */
modelAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * MODEL GENERIC PARAMETERS
 * ============================================================================
 *
 * Declaration generic syntax is owned by FunctionGenerics.
 */
modelGenericParameters
    : functionGenericParameters
    ;


/*
 * ============================================================================
 * MODEL INHERITANCE
 * ============================================================================
 *
 * Inheritance uses the canonical language keyword and canonical qualified-name
 * structure supplied through the imported name/annotation grammar.
 */
modelExtendsClause
    : EXTENDS
      modelQualifiedNameList
    ;


/*
 * ============================================================================
 * MODEL IMPLEMENTATION / INTERFACE CONTRACTS
 * ============================================================================
 */

modelImplementsClause
    : IMPLEMENTS
      modelQualifiedNameList
    ;


modelQualifiedNameList
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * MODEL BODY
 * ============================================================================
 */

modelBody
    : LBRACE
      modelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MODEL MEMBER
 * ============================================================================
 *
 * Every model-specific member begins with exactly one canonical annotation.
 *
 * Example:
 *
 *     @input x: Tensor<Float>;
 *
 *     @component encoder {
 *         ...
 *     }
 */
modelMember
    : annotation
      modelMemberPayload
    ;


/*
 * ============================================================================
 * MODEL MEMBER PAYLOAD
 * ============================================================================
 *
 * Structural alternatives are deliberately kept small.
 *
 * Domain meaning comes from the annotation and semantic analysis.
 */
modelMemberPayload
    : modelConnectionPayload
    | modelTypedMemberPayload
    | modelBlockMemberPayload
    | modelExpressionMemberPayload
    ;


/*
 * ============================================================================
 * MODEL CONNECTION
 * ============================================================================
 *
 * Example:
 *
 *     @connect encoder.output -> classifier.input;
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
    : IDENTIFIER
      (
          DOT
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * MODEL TYPED MEMBER
 * ============================================================================
 *
 * Examples:
 *
 *     @input x: Tensor<Float>;
 *
 *     @output y: Tensor<Float>;
 *
 *     @component encoder: Encoder;
 *
 *     @state state: QuantumState;
 *
 *     @parameter weights: Tensor<Float> = initialWeights;
 */
modelTypedMemberPayload
    : IDENTIFIER
      COLON
      typeExpression
      modelMemberInitializer?
      SEMICOLON
    ;


modelMemberInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * MODEL BLOCK MEMBER
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
 * Nested model regions are structurally recursive and are not bounded by a
 * language-level nesting constant.
 */
modelBlockMemberPayload
    : IDENTIFIER
      modelBody
    ;


/*
 * ============================================================================
 * MODEL EXPRESSION MEMBER
 * ============================================================================
 *
 * Examples:
 *
 *     @operation forward = forward_function;
 *
 *     @bind activation = relu;
 *
 *     @config precision = "portable";
 *
 *     @metadata family = "transformer";
 *
 *     @requires capability("tensor.compute");
 *
 *     @constraint latency <= latency_budget;
 *
 *     @preference accelerator.tensor;
 *
 * The expression itself remains owned by Expressions.
 */
modelExpressionMemberPayload
    : expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * REUSABLE MODEL PORT
 * ============================================================================
 *
 * This rule is a structural bridge for downstream model-oriented grammars.
 *
 * It deliberately does not introduce a second port type system.
 */
modelPort
    : IDENTIFIER
      COLON
      typeExpression
      modelMemberInitializer?
    ;


/*
 * ============================================================================
 * MODEL REFERENCE
 * ============================================================================
 *
 * These are structural bridge rules for consumers that need an explicit model
 * reference boundary.
 *
 * They do not create another reference system.
 */
modelReference
    : qualifiedName
    ;


modelTypeReference
    : typeExpression
    ;


modelExpression
    : expression
    ;


/*
 * ============================================================================
 * MODEL CONFIGURATION / METADATA BRIDGES
 * ============================================================================
 *
 * These rules are intentionally ordinary expressions.
 *
 * They do not create an AI-specific value language.
 */
modelConfigurationValue
    : expression
    ;


modelMetadataValue
    : expression
    ;


/*
 * ============================================================================
 * MODEL RESOURCE / CAPABILITY / CONSTRAINT / PREFERENCE BRIDGES
 * ============================================================================
 *
 * These rules are semantic integration boundaries.
 *
 * They intentionally do not duplicate:
 *
 *     resources/*
 *     capabilities/*
 *     validation/*
 *     policies/*
 *
 * They merely provide stable model-oriented parser entry points for downstream
 * composition where required.
 */
modelRequirement
    : annotation
      expression
      SEMICOLON
    ;


modelCapability
    : annotation
      expression
      SEMICOLON
    ;


modelConstraint
    : annotation
      expression
      SEMICOLON
    ;


modelPreference
    : annotation
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * MODEL OPERATION BRIDGE
 * ============================================================================
 *
 * Operation implementation syntax remains an ordinary expression.
 *
 * Algorithm names remain semantic/library concepts rather than parser-level
 * keyword inventories.
 */
modelOperation
    : annotation
      IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * MODEL COMPOSITION BRIDGE
 * ============================================================================
 */

modelComposition
    : annotation
      IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY / PORTABILITY EXPRESSION BRIDGES
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


/*
 * ============================================================================
 * EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future model-domain extensions may reuse these structural forms.
 *
 * They must register their semantic meaning outside this grammar.
 */
modelExtensionMember
    : annotation
      modelExtensionPayload
    ;


modelExtensionPayload
    : modelTypedMemberPayload
    | modelBlockMemberPayload
    | modelExpressionMemberPayload
    | modelConnectionPayload
    ;


/*
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     tokenVocab = ZamaniLexer
 *          |
 *          v
 *     AIModels
 *
 *     grammar/core/annotations.g4
 *          |
 *          v
 *     annotation
 *
 *     grammar/types/types.g4
 *          |
 *          v
 *     Type
 *          |
 *          v
 *     typeExpression
 *
 *     grammar/expressions/expressions.g4
 *          |
 *          v
 *     expression
 *
 *     grammar/functions/generics.g4
 *          |
 *          v
 *     functionGenericParameters
 *
 *
 * DOWNSTREAM
 * ----------
 *
 *     AIModels
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic model
 *          |
 *          +--> type analysis
 *          +--> generic analysis
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          +--> portability
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> distributed
 *          +--> accelerator
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     routing / scheduling
 *          |
 *          v
 *     resilience / recovery
 *          |
 *          v
 *     ZQN / HAL
 *          |
 *          v
 *     target realization
 *
 *
 * ============================================================================
 * AI.G4 INTEGRATION
 * ============================================================================
 *
 * grammar/ai/ai.g4 must:
 *
 *     import AIModels
 *
 * and dispatch:
 *
 *     aiModelConstruct
 *
 * through:
 *
 *     aiConstruct
 *
 * It must not duplicate any model production.
 *
 *
 * ============================================================================
 * ROOT GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The canonical root composition must reach AI through AI.g4.
 *
 * No root grammar should independently reproduce:
 *
 *     aiModelDeclaration
 *     modelMember
 *     modelBody
 *     modelConnectionPayload
 *
 *
 * ============================================================================
 * LEGACY FILE INTEGRATION
 * ============================================================================
 *
 * grammar/ai/model.g4
 *
 * is NOT a canonical source.
 *
 * It must not be imported by:
 *
 *     ai.g4
 *     Zamani.g4
 *     any production AI composition root
 *
 * until it is converted into a distinct compatibility façade or removed.
 *
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The AST builder must consume the parser contexts from this grammar and map
 * them into the existing domain-neutral AST.
 *
 * No AI-specific universal AST is introduced.
 *
 * The AST must preserve:
 *
 *     source spans
 *     source ordering
 *     annotation structure
 *     model name
 *     generic parameters
 *     inheritance
 *     interfaces
 *     member structure
 *     endpoints
 *     types
 *     expressions
 *
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis must resolve:
 *
 *     model annotation identity
 *     names
 *     generic parameters
 *     generic bounds
 *     model inheritance
 *     interface contracts
 *     member roles
 *     member names
 *     graph endpoints
 *     graph type compatibility
 *     resources
 *     capabilities
 *     effects
 *     policies
 *     contracts
 *     provenance
 *     portability
 *
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * No AI-specific IR is defined here.
 *
 * A model can semantically lower to canonical operations for:
 *
 *     classical execution
 *     tensor execution
 *     accelerator execution
 *     distributed execution
 *     hardware intent
 *     quantum execution
 *
 * Quantum operations MUST continue through:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 *
 * ============================================================================
 * TEST INTEGRATION
 * ============================================================================
 *
 * Recommended location:
 *
 *     grammar/tests/ai/models/
 *
 * Required categories:
 *
 *     positive
 *     negative
 *     semantic-negative
 *     boundary
 *     scalability
 *     determinism
 *     compatibility
 *     cross-domain
 *     source-span preservation
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No machine-capacity constants occur in this grammar.
 *
 * No fixed:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     node count
 *     device count
 *     memory capacity
 *     register width
 *     network size
 *     tensor rank
 *     model depth
 *     parameter count
 *
 * are encoded.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `models.g4` is complete when:
 *
 *     [ ] AIModels is the sole canonical model grammar identity.
 *
 *     [ ] Canonical ZamaniLexer is consumed.
 *
 *     [ ] Canonical annotation grammar is consumed.
 *
 *     [ ] NANO_ANNOTATION is absent.
 *
 *     [ ] Canonical Type grammar is consumed.
 *
 *     [ ] Canonical expression grammar is consumed.
 *
 *     [ ] Canonical generic declaration grammar is consumed.
 *
 *     [ ] No annotation syntax is duplicated.
 *
 *     [ ] No type syntax is duplicated.
 *
 *     [ ] No expression syntax is duplicated.
 *
 *     [ ] No lexer rules are defined.
 *
 *     [ ] Model declaration syntax is explicit.
 *
 *     [ ] Model members are structurally extensible.
 *
 *     [ ] Typed members are supported.
 *
 *     [ ] Nested members are supported.
 *
 *     [ ] Graph connections are supported.
 *
 *     [ ] Generic parameters are supported.
 *
 *     [ ] Inheritance is supported.
 *
 *     [ ] Interface/contract references are supported.
 *
 *     [ ] Annotation arguments remain supported through the canonical
 *         annotation grammar.
 *
 *     [ ] Model graph size is not artificially bounded.
 *
 *     [ ] Model nesting is not artificially bounded.
 *
 *     [ ] Generic arity is not artificially bounded.
 *
 *     [ ] No hardware capacity is encoded.
 *
 *     [ ] No device identity is encoded.
 *
 *     [ ] No vendor is encoded.
 *
 *     [ ] No framework is encoded.
 *
 *     [ ] No AI-specific IR is created.
 *
 *     [ ] quantum::ir remains canonical.
 *
 *     [ ] No Rust actions exist.
 *
 *     [ ] No semantic predicates exist.
 *
 *     [ ] No filesystem/network/hardware access exists.
 *
 *     [ ] Rust 1.97 / 1.97.1 integration succeeds.
 *
 *     [ ] Safe-Rust-only policy remains satisfied.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Compatibility tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * A model declaration describes portable SOURCE INTENT.
 *
 * It does not describe a particular machine.
 *
 * It does not select a particular device.
 *
 * It does not allocate resources.
 *
 * It does not select a backend.
 *
 * It does not define quantum physical realization.
 *
 * It does not define HDL physical realization.
 *
 * It does not define runtime execution.
 *
 * Therefore the same model source can remain semantically stable while
 * downstream compilation determines an appropriate realization for the
 * available resources, capabilities and target characteristics.
 *
 * ============================================================================
 */