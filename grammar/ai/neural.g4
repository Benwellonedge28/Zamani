/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/neural.g4
 *
 * Grammar:
 *     AINeural
 *
 * Status:
 *     Production neural-computation domain grammar.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL NEURAL COMPUTATION STRUCTURE.
 *
 * It provides a framework-neutral syntax boundary for describing neural
 * computation without embedding:
 *
 *     - a specific AI framework;
 *     - a specific accelerator;
 *     - a specific CPU/GPU/FPGA;
 *     - a specific quantum processor;
 *     - a specific tensor implementation;
 *     - a specific training algorithm;
 *     - a specific differentiation algorithm;
 *     - a specific optimizer;
 *     - a specific runtime;
 *     - a specific device;
 *     - a specific hardware topology.
 *
 * Neural syntax describes WHAT the neural computation is.
 *
 * Downstream semantic analysis determines HOW it is represented and executed.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     AINeural parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     neural semantic analysis
 *          |
 *          +---------------------+----------------------+
 *          |                     |                      |
 *          v                     v                      v
 *      classical              quantum                hybrid
 *      semantics              semantics              semantics
 *          |                     |                      |
 *          +---------------------+----------------------+
 *                                |
 *                                v
 *                       canonical semantic model
 *                                |
 *                                v
 *                         canonical IR / ZUIR
 *                                |
 *             +------------------+------------------+
 *             |                  |                  |
 *             v                  v                  v
 *        classical IR       quantum::ir       hardware/HDL
 *             |                  |                  |
 *             +------------------+------------------+
 *                                |
 *                                v
 *                           optimization
 *                                |
 *                       routing / scheduling
 *                                |
 *                    resilience / QEC / ZQN
 *                                |
 *                               HAL
 *                                |
 *                         target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar does NOT create a neural IR and does NOT create a quantum IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - neural computation declarations;
 *     - neural computation regions;
 *     - neural components;
 *     - neural layers as source-level semantic structures;
 *     - neural connections;
 *     - neural activations as semantic references;
 *     - neural architecture relationships;
 *     - neural forward-computation declarations;
 *     - neural configuration clauses;
 *     - neural-level resource/capability intent;
 *     - neural-level portability metadata;
 *     - neural source-level composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - general statements;
 *     - tensor type implementation;
 *     - tensor storage;
 *     - model declaration semantics;
 *     - dataset semantics;
 *     - training algorithms;
 *     - optimization algorithms;
 *     - automatic differentiation;
 *     - derivative strategies;
 *     - inference implementation;
 *     - probabilistic semantics;
 *     - agent semantics;
 *     - accelerator implementation;
 *     - hardware discovery;
 *     - device selection;
 *     - resource discovery;
 *     - scheduling;
 *     - routing;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - vendor APIs;
 *     - framework-specific syntax.
 *
 * ============================================================================
 * RELATIONSHIP TO OTHER AI GRAMMARS
 * ============================================================================
 *
 * Neural computation intersects several existing AI domains.
 *
 * Ownership remains separated:
 *
 *     models.g4
 *         -> model declarations and model-level structure
 *
 *     tensors.g4
 *         -> tensor-specific syntax
 *
 *     datasets.g4
 *         -> dataset syntax
 *
 *     training.g4
 *         -> training declarations and training intent
 *
 *     inference.g4
 *         -> inference intent
 *
 *     differentiation.g4
 *         -> derivative computation requests
 *
 *     differentiable.g4
 *         -> differentiability contracts
 *
 *     optimization.g4
 *         -> optimization intent
 *
 *     agents.g4
 *         -> agent semantics
 *
 *     neural.g4
 *         -> neural architecture/computation structure
 *
 * Neural syntax may reference concepts owned by those grammars through:
 *
 *     expression
 *     typeExpression
 *     identifier
 *
 * without duplicating their grammar.
 *
 * ============================================================================
 * IMPORTANT NON-DUPLICATION RULE
 * ============================================================================
 *
 * This grammar MUST NOT recreate:
 *
 *     modelDeclaration
 *     tensorDeclaration
 *     datasetDeclaration
 *     trainingDeclaration
 *     differentiationRequest
 *     differentiabilityContract
 *     inferenceDeclaration
 *     optimizationDeclaration
 *
 * If a neural construct needs one of these concepts, it references the
 * corresponding source-level value/name/expression and semantic analysis
 * establishes the relationship.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar contains NO lexer rules.
 *
 * Neural vocabulary is deliberately not added as a global keyword inventory.
 *
 * The existing:
 *
 *     NANO_ANNOTATION
 *
 * token provides the extensible annotation boundary.
 *
 * For example, semantic analysis may recognize:
 *
 *     @neural
 *     @network
 *     @layer
 *     @connection
 *     @activation
 *     @forward
 *     @input
 *     @output
 *     @state
 *     @parameter
 *     @requires
 *     @capability
 *     @constraint
 *     @prefer
 *
 * but these names are NOT enumerated by this grammar.
 *
 * The semantic registry owns their meaning and lifecycle.
 *
 * ============================================================================
 * COMMON GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * Those grammars remain the sole owners of:
 *
 *     typeExpression
 *     expression
 *     statement
 *     identifier
 *     argumentList
 *
 * No local replacements are introduced here.
 *
 * ============================================================================
 * ANTLR DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     AINeural
 *         |
 *         +--> Types
 *         +--> Expressions
 *         +--> Statements
 *
 * AINeural MUST NOT import:
 *
 *     AI
 *     Models
 *     Tensors
 *     Training
 *     Inference
 *     Differentiation
 *     AIDifferentiable
 *     Optimization
 *     Quantum
 *     Hardware
 *     Resources
 *
 * This keeps the file independently completable and prevents grammar cycles.
 *
 * ============================================================================
 * AI COMPOSITION INTEGRATION
 * ============================================================================
 *
 * This grammar exposes:
 *
 *     neuralConstruct
 *
 * as its leaf-domain entry point.
 *
 * The AI aggregate grammar MUST NOT blindly add:
 *
 *     | neuralConstruct
 *
 * beside other generic annotation-led alternatives if doing so creates
 * overlapping alternatives beginning with NANO_ANNOTATION.
 *
 * Instead, the canonical AI composition layer should establish one
 * deterministic neural-domain dispatch boundary.
 *
 * The semantic registry may also identify the annotation/domain association
 * after parsing.
 *
 * This preserves:
 *
 *     one AI composition boundary
 *
 * without creating:
 *
 *     multiple annotation-led parser branches with identical prefixes.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar maps into the existing domain-neutral frontend AST.
 *
 * The parser must not create framework-specific Rust structures.
 *
 * Conceptual mappings:
 *
 *     neuralConstruct
 *         -> NeuralConstructNode
 *
 *     neuralDeclaration
 *         -> NeuralDeclarationNode
 *
 *     neuralComponent
 *         -> NeuralComponentNode
 *
 *     neuralLayer
 *         -> NeuralLayerNode
 *
 *     neuralConnection
 *         -> NeuralConnectionNode
 *
 *     neuralActivation
 *         -> NeuralActivationNode
 *
 *     neuralForward
 *         -> NeuralForwardNode
 *
 *     neuralClause
 *         -> NeuralClauseNode
 *
 * Exact Rust AST naming belongs to the existing frontend AST contract.
 *
 * These names describe conceptual AST roles only.
 *
 * Every node must preserve:
 *
 *     - source span;
 *     - annotation;
 *     - source ordering;
 *     - identifier;
 *     - expressions;
 *     - type expressions;
 *     - nested structure;
 *     - clause ordering.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure.
 *
 * Semantic analysis establishes meaning.
 *
 * Semantic analysis is responsible for validating:
 *
 *     - annotation meaning;
 *     - neural component kinds;
 *     - duplicate names;
 *     - scope;
 *     - references;
 *     - input/output compatibility;
 *     - tensor compatibility;
 *     - shape compatibility;
 *     - activation validity;
 *     - connection legality;
 *     - graph validity;
 *     - cycle policy;
 *     - state semantics;
 *     - parameter semantics;
 *     - effect semantics;
 *     - resource requirements;
 *     - capability requirements;
 *     - differentiability requirements;
 *     - training compatibility;
 *     - inference compatibility;
 *     - quantum/classical interoperability;
 *     - hardware portability;
 *     - dialect/version compatibility.
 *
 * This grammar does not decide any of those semantic questions.
 *
 * ============================================================================
 * NEURAL GRAPH MODEL
 * ============================================================================
 *
 * A neural computation may be represented as a graph.
 *
 * The grammar therefore permits an unbounded sequence of components and
 * relationships.
 *
 * Conceptually:
 *
 *     neural network
 *          |
 *          +--> inputs
 *          +--> layers/components
 *          +--> connections
 *          +--> states
 *          +--> outputs
 *          +--> forward computation
 *
 * The grammar imposes no finite graph size.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO universal finite limits for:
 *
 *     - number of neural networks;
 *     - number of models;
 *     - number of layers;
 *     - number of neurons;
 *     - number of parameters;
 *     - number of inputs;
 *     - number of outputs;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - sequence length;
 *     - batch size;
 *     - graph nodes;
 *     - graph edges;
 *     - workers;
 *     - accelerators;
 *     - GPUs;
 *     - CPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - qubits;
 *     - nodes;
 *     - memory;
 *     - storage;
 *     - devices.
 *
 * Repetition is represented structurally using:
 *
 *     *
 *     +
 *
 * and not through finite enumerations.
 *
 * "Infinity" means that the language imposes no artificial finite capacity.
 *
 * Actual limits are determined by:
 *
 *     - source program semantics;
 *     - compiler resources;
 *     - runtime resources;
 *     - target capabilities;
 *     - resource requirements;
 *     - deployment policy.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_LAYERS
 *     MAX_NEURONS
 *     MAX_PARAMETERS
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_BATCH_SIZE
 *     MAX_SEQUENCE_LENGTH
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * It MUST also not select:
 *
 *     GPU 0
 *     CPU 0
 *     accelerator 1
 *     device 7
 *     fixed VRAM size
 *     fixed SIMD width
 *     fixed tensor-core count
 *     fixed cluster topology
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY MODEL
 * ============================================================================
 *
 * Neural programs may express semantic intent such as:
 *
 *     @requires capability("tensor.compute");
 *     @requires capability("neural.inference");
 *     @requires capability("neural.training");
 *     @requires capability("quantum.compute");
 *
 * or other expressions understood by the resource/capability semantic layer.
 *
 * This does NOT select a device.
 *
 * Distinction:
 *
 *     requirement
 *         -> what must be available
 *
 *     capability
 *         -> what a target can provide
 *
 *     preference
 *         -> what implementation is preferred
 *
 *     constraint
 *         -> what implementations are disallowed
 *
 *     realization
 *         -> what the compiler actually selects
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Neural computation may consume or produce:
 *
 *     classical values;
 *     tensors;
 *     data;
 *     distributed values;
 *     network values;
 *     hardware-backed values;
 *     quantum values;
 *     measurements;
 *     hybrid computations.
 *
 * These remain ordinary:
 *
 *     expression
 *     typeExpression
 *
 * constructs.
 *
 * The neural grammar does not define those domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A neural expression may reference a quantum computation through ordinary
 * source expressions or semantic references.
 *
 * Example:
 *
 *     @neural network Hybrid {
 *         @input x;
 *         @output y;
 *         @forward y = quantum_feature_map(x);
 *     }
 *
 * The neural grammar does not define:
 *
 *     Qubit
 *     gate sets
 *     physical qubits
 *     topology
 *     routing
 *     QEC
 *     ZQN
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Neural components may be associated semantically with hardware intent:
 *
 *     @requires capability("tensor.compute");
 *     @prefer capability("accelerator.compute");
 *
 * but neural.g4 does not define:
 *
 *     wires;
 *     registers;
 *     physical memory;
 *     device IDs;
 *     clock widths;
 *     FPGA resource counts;
 *     accelerator topology.
 *
 * Those belong to:
 *
 *     hdl/
 *     hardware/
 *     resources/
 *     compile/
 *     execution/
 *
 * ============================================================================
 * TRAINING INTEGRATION
 * ============================================================================
 *
 * Neural architecture and training are deliberately separate.
 *
 * Example:
 *
 *     @neural network N {
 *         ...
 *     }
 *
 * may later be referenced by:
 *
 *     training.g4
 *
 * through ordinary names/expressions.
 *
 * neural.g4 MUST NOT redefine:
 *
 *     optimizer syntax;
 *     loss syntax;
 *     training loops;
 *     learning-rate schedules;
 *     differentiation algorithms.
 *
 * ============================================================================
 * DIFFERENTIATION INTEGRATION
 * ============================================================================
 *
 * Neural computations frequently participate in differentiation.
 *
 * That relationship is semantic.
 *
 * neural.g4 does not define:
 *
 *     forward mode;
 *     reverse mode;
 *     adjoint mode;
 *     parameter shift;
 *     finite differences;
 *     automatic differentiation algorithms.
 *
 * Those remain owned by:
 *
 *     differentiation.g4
 *     differentiable.g4
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source tokens;
 *     - grammar version;
 *     - explicitly selected dialect configuration.
 *
 * It MUST NOT depend on:
 *
 *     - hardware availability;
 *     - network state;
 *     - filesystem state;
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - runtime state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no executable actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment access;
 *     - no secret access;
 *     - no process execution;
 *     - no hardware discovery.
 *
 * Security-sensitive validation belongs downstream.
 *
 * ============================================================================
 * RUST SAFETY
 * ============================================================================
 *
 * The grammar itself contains no Rust implementation.
 *
 * Code generated/consumed by the Zamani compiler must target:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *
 * and remain safe Rust.
 *
 * This grammar introduces no `unsafe` requirement.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The sole public entry point of this leaf grammar is:
 *
 *     neuralConstruct
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. PUBLIC NEURAL CONSTRUCT
 * ========================================================================== */

/**
 * Complete neural-domain source construct.
 *
 * The annotation is intentionally lexically opaque.
 *
 * Semantic analysis normally requires the annotation to represent the neural
 * domain, for example:
 *
 *     @neural
 *
 * The spelling is not hard-coded here.
 */
neuralConstruct
    : neuralDeclaration
    | neuralComponent
    | neuralConnection
    | neuralForward
    | neuralContract
    ;


/* ============================================================================
 * 2. NEURAL DECLARATION
 * ========================================================================== */

/**
 * Declares a neural computation/network/container.
 *
 * Canonical conceptual form:
 *
 *     @neural NetworkName {
 *         ...
 *     }
 *
 * The semantic layer determines whether the declaration represents:
 *
 *     network
 *     neural computation
 *     architecture
 *     subnetwork
 *     reusable neural component
 *     dialect-specific neural object
 */
neuralDeclaration
    : NANO_ANNOTATION
      identifier
      neuralDeclarationHeader*
      neuralBody
    ;


/**
 * Header clauses are intentionally expression-based.
 *
 * This allows future neural properties without creating a closed keyword list.
 */
neuralDeclarationHeader
    : COMMA
      neuralClause
    ;


/* ============================================================================
 * 3. NEURAL BODY
 * ========================================================================== */

/**
 * Neural bodies contain an unbounded sequence of neural members.
 */
neuralBody
    : LBRACE
      neuralMember*
      RBRACE
    ;


/**
 * A neural member may be a neural-domain construct or an ordinary Zamani
 * statement.
 *
 * This enables neural/classical/hybrid composition without duplicating the
 * statement grammar.
 */
neuralMember
    : neuralComponent
    | neuralConnection
    | neuralForward
    | neuralContract
    | neuralNestedDeclaration
    | statement
    ;


/**
 * Nested neural declarations are permitted structurally.
 *
 * Semantic analysis determines whether nesting is legal.
 */
neuralNestedDeclaration
    : NANO_ANNOTATION
      identifier
      neuralDeclarationHeader*
      neuralBody
    ;


/* ============================================================================
 * 4. NEURAL COMPONENT
 * ========================================================================== */

/**
 * Generic neural component.
 *
 * A component may represent semantically:
 *
 *     layer
 *     block
 *     neuron group
 *     activation
 *     normalization
 *     recurrent component
 *     attention component
 *     convolution component
 *     embedding
 *     encoder
 *     decoder
 *     user-defined neural component
 *
 * The grammar deliberately does not enumerate framework-specific layer types.
 */
neuralComponent
    : NANO_ANNOTATION
      identifier
      neuralComponentTypeClause?
      neuralComponentInitializer?
      neuralComponentClause*
      neuralComponentBody?
      SEMICOLON?
    ;


/**
 * Optional component type.
 *
 * Example semantic forms:
 *
 *     @layer dense : Dense
 *     @layer encoder : Encoder
 *
 * `Dense`, `Encoder`, etc. remain ordinary type names.
 */
neuralComponentTypeClause
    : COLON
      typeExpression
    ;


/**
 * Optional component initializer.
 */
neuralComponentInitializer
    : ASSIGN
      expression
    ;


/**
 * Component clauses are generic and extensible.
 */
neuralComponentClause
    : COMMA
      neuralClause
    ;


/**
 * Optional component body.
 */
neuralComponentBody
    : neuralBody
    ;


/* ============================================================================
 * 5. NEURAL CONNECTION
 * ========================================================================== */

/**
 * Describes a relationship between named neural components.
 *
 * The relationship is represented using ordinary expressions rather than
 * hard-coded node identifiers or topology limits.
 *
 * Examples:
 *
 *     @connect a -> b;
 *     @connect encoder -> decoder;
 *
 * The semantic layer decides whether the relationship represents:
 *
 *     data flow
 *     residual flow
 *     skip connection
 *     recurrent dependency
 *     attention dependency
 *     control dependency
 *     another registered neural relation.
 */
neuralConnection
    : NANO_ANNOTATION
      expression
      neuralConnectionOperator
      expression
      neuralConnectionClause*
      SEMICOLON?
    ;


/**
 * Connection operators are deliberately restricted to canonical lexer
 * operators rather than introducing neural-specific lexical tokens.
 */
neuralConnectionOperator
    : ARROW
    ;


/**
 * Additional connection metadata.
 */
neuralConnectionClause
    : COMMA
      neuralClause
    ;


/* ============================================================================
 * 6. NEURAL FORWARD COMPUTATION
 * ========================================================================== */

/**
 * Declares source-level forward computation.
 *
 * Examples:
 *
 *     @forward y = network(x);
 *
 *     @forward output = layer(input);
 *
 * The computation remains an ordinary Zamani expression.
 */
neuralForward
    : NANO_ANNOTATION
      neuralForwardTarget
      ASSIGN
      expression
      neuralForwardClause*
      SEMICOLON?
    ;


/**
 * Forward target.
 *
 * A target may be a canonical assignment target represented by an identifier
 * or a qualified/member/indexed expression.
 *
 * The expression grammar remains the owner of full expression syntax.
 */
neuralForwardTarget
    : expression
    ;


/**
 * Forward metadata.
 */
neuralForwardClause
    : COMMA
      neuralClause
    ;


/* ============================================================================
 * 7. NEURAL CONTRACTS
 * ========================================================================== */

/**
 * Neural-level contracts include requirements, capabilities, constraints,
 * preferences, and other semantic properties.
 *
 * The annotation determines the contract category semantically.
 *
 * Examples:
 *
 *     @requires capability("neural.training");
 *
 *     @capability capability("tensor.compute");
 *
 *     @constraint expression;
 *
 *     @prefer expression;
 */
neuralContract
    : NANO_ANNOTATION
      neuralContractPayload
      SEMICOLON?
    ;


neuralContractPayload
    : expression
    ;


/* ============================================================================
 * 8. GENERIC NEURAL CLAUSE
 * ========================================================================== */

/**
 * Open-ended neural property.
 *
 * Clause names remain identifiers rather than lexer keywords.
 *
 * Examples:
 *
 *     activation = relu
 *     dimensions = shape
 *     inputs = x
 *     outputs = y
 *     parameters = weights
 *     state = hidden
 *     precision = p
 *     domain = real
 *     regularization = policy
 *
 * The semantic registry owns the validity and interpretation of each clause.
 */
neuralClause
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 9. NAMED NEURAL VALUE
 * ========================================================================== */

/**
 * Provides a stable parser-level bridge for neural names.
 *
 * This is not a second identifier grammar.
 */
neuralName
    : identifier
    ;


/* ============================================================================
 * 10. NEURAL EXPRESSION BRIDGE
 * ========================================================================== */

/**
 * Neural values are ordinary Zamani expressions.
 */
neuralExpression
    : expression
    ;


/* ============================================================================
 * 11. NEURAL TYPE BRIDGE
 * ========================================================================== */

/**
 * Neural types are ordinary Zamani types.
 *
 * Examples that may be semantically meaningful:
 *
 *     Tensor<T, shape>
 *     NeuralInput<T>
 *     NeuralOutput<T>
 *     Model<T>
 *
 * The type grammar owns their syntax.
 */
neuralType
    : typeExpression
    ;


/* ============================================================================
 * 12. NEURAL ARGUMENTS
 * ========================================================================== */

/**
 * Reuses the canonical expression argument-list grammar.
 */
neuralArguments
    : argumentList
    ;


/* ============================================================================
 * 13. INPUT DECLARATION
 * ========================================================================== */

/**
 * Neural inputs are semantic roles.
 *
 * The annotation remains generic.
 *
 * Example:
 *
 *     @input x : Tensor<float, shape>
 *
 * Exact input semantics are validated downstream.
 */
neuralInput
    : NANO_ANNOTATION
      identifier
      neuralTypeClause?
      neuralInitializerClause?
      neuralClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 14. OUTPUT DECLARATION
 * ========================================================================== */

/**
 * Neural outputs are semantic roles.
 */
neuralOutput
    : NANO_ANNOTATION
      identifier
      neuralTypeClause?
      neuralInitializerClause?
      neuralClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 15. STATE DECLARATION
 * ========================================================================== */

/**
 * Neural state may represent recurrent state, persistent computation state,
 * intermediate state, or another semantic state category.
 *
 * Actual state semantics are downstream.
 */
neuralState
    : NANO_ANNOTATION
      identifier
      neuralTypeClause?
      neuralInitializerClause?
      neuralClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 16. PARAMETER DECLARATION
 * ========================================================================== */

/**
 * Neural parameters are source-level semantic values.
 *
 * Their storage and realization remain downstream.
 */
neuralParameter
    : NANO_ANNOTATION
      identifier
      neuralTypeClause?
      neuralInitializerClause?
      neuralClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 17. TYPE / INITIALIZER HELPERS
 * ========================================================================== */

neuralTypeClause
    : COLON
      typeExpression
    ;


neuralInitializerClause
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 18. ACTIVATION REFERENCE
 * ========================================================================== */

/**
 * Activation functions are represented as ordinary expressions/references.
 *
 * This intentionally avoids:
 *
 *     activation
 *         : RELU
 *         | SIGMOID
 *         | TANH
 *         | ...
 *
 * because that would create a closed framework-independent activation list.
 *
 * Semantic analysis may recognize standard, user-defined, symbolic,
 * differentiable, quantum-compatible, or dialect-defined activations.
 */
neuralActivation
    : NANO_ANNOTATION
      identifier?
      ASSIGN?
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 19. ARCHITECTURE REFERENCE
 * ========================================================================== */

/**
 * References an architecture using ordinary Zamani expressions.
 *
 * This does not define architecture internals.
 */
neuralArchitectureReference
    : expression
    ;


/* ============================================================================
 * 20. LAYER REFERENCE
 * ========================================================================== */

/**
 * Layer references remain names/expressions.
 *
 * Layer implementation belongs to semantic/library infrastructure.
 */
neuralLayerReference
    : expression
    ;


/* ============================================================================
 * 21. CONNECTION REFERENCE
 * ========================================================================== */

/**
 * Generic connection reference.
 */
neuralConnectionReference
    : expression
    ;


/* ============================================================================
 * 22. NETWORK INPUT REFERENCE
 * ========================================================================== */

/**
 * Network input references are ordinary expressions.
 */
neuralInputReference
    : expression
    ;


/* ============================================================================
 * 23. NETWORK OUTPUT REFERENCE
 * ========================================================================== */

/**
 * Network output references are ordinary expressions.
 */
neuralOutputReference
    : expression
    ;


/* ============================================================================
 * 24. RESOURCE REQUIREMENT
 * ========================================================================== */

/**
 * Generic resource requirement bridge.
 *
 * The expression may represent:
 *
 *     capability("tensor.compute")
 *     memory(...)
 *     communication(...)
 *     throughput(...)
 *     latency(...)
 *
 * without encoding physical machine limits.
 */
neuralResourceRequirement
    : NANO_ANNOTATION
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 25. CAPABILITY REQUIREMENT
 * ========================================================================== */

/**
 * Capability requirements remain semantic expressions.
 */
neuralCapabilityRequirement
    : NANO_ANNOTATION
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 26. CONSTRAINT
 * ========================================================================== */

/**
 * Neural constraints do not select concrete hardware.
 */
neuralConstraint
    : NANO_ANNOTATION
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 27. PREFERENCE
 * ========================================================================== */

/**
 * Preferences are weaker than requirements and constraints.
 */
neuralPreference
    : NANO_ANNOTATION
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 28. PORTABILITY CONTRACT
 * ========================================================================== */

/**
 * Explicit portability bridge.
 *
 * Neural programs may state semantic requirements while remaining independent
 * of the actual target.
 */
neuralPortability
    : NANO_ANNOTATION
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 29. CLASSICAL BRIDGE
 * ========================================================================== */

/**
 * Neural/classical interoperability uses ordinary expressions.
 */
neuralClassicalExpression
    : expression
    ;


/* ============================================================================
 * 30. QUANTUM BRIDGE
 * ========================================================================== */

/**
 * Neural/quantum interoperability uses ordinary expressions.
 *
 * The semantic layer determines whether the referenced computation is
 * quantum and lowers it through the canonical quantum::ir boundary.
 */
neuralQuantumExpression
    : expression
    ;


/* ============================================================================
 * 31. HYBRID BRIDGE
 * ========================================================================== */

/**
 * Neural/hybrid interoperability.
 */
neuralHybridExpression
    : expression
    ;


/* ============================================================================
 * 32. DATA BRIDGE
 * ========================================================================== */

/**
 * Neural/data interoperability.
 */
neuralDataExpression
    : expression
    ;


/* ============================================================================
 * 33. DISTRIBUTED BRIDGE
 * ========================================================================== */

/**
 * Neural/distributed interoperability.
 */
neuralDistributedExpression
    : expression
    ;


/* ============================================================================
 * 34. HARDWARE BRIDGE
 * ========================================================================== */

/**
 * Neural/hardware interoperability.
 *
 * This is a source-level semantic reference only.
 *
 * No physical device is selected by this rule.
 */
neuralHardwareExpression
    : expression
    ;


/* ============================================================================
 * 35. NEURAL BLOCK
 * ========================================================================== */

/**
 * Explicit neural computation block.
 *
 * This provides a stable structural boundary for tooling without creating
 * another general block grammar.
 */
neuralBlock
    : LBRACE
      neuralMember*
      RBRACE
    ;


/* ============================================================================
 * 36. NEURAL COMPONENT LIST
 * ========================================================================== */

/**
 * Unbounded component sequence.
 */
neuralComponentList
    : neuralComponent*
    ;


/* ============================================================================
 * 37. NEURAL CONNECTION LIST
 * ========================================================================== */

/**
 * Unbounded connection sequence.
 */
neuralConnectionList
    : neuralConnection*
    ;


/* ============================================================================
 * 38. NEURAL CLAUSE LIST
 * ========================================================================== */

/**
 * Unbounded clause sequence.
 */
neuralClauseList
    : neuralClause*
    ;


/* ============================================================================
 * 39. NEURAL CONTRACT LIST
 * ========================================================================== */

/**
 * Unbounded contract sequence.
 */
neuralContractList
    : neuralContract*
    ;


/* ============================================================================
 * 40. NEURAL INPUT / OUTPUT MEMBERS
 * ========================================================================== */

/**
 * Explicit semantic helper for tooling and semantic adapters.
 *
 * These rules are not additional universal language constructs.
 */
neuralInterfaceMember
    : neuralInput
    | neuralOutput
    | neuralState
    | neuralParameter
    ;


/* ============================================================================
 * 41. NEURAL INTERFACE LIST
 * ========================================================================== */

/**
 * Unbounded interface member sequence.
 */
neuralInterfaceList
    : neuralInterfaceMember*
    ;


/* ============================================================================
 * 42. NEURAL DECLARATION REFERENCE
 * ========================================================================== */

/**
 * Reference to another neural declaration.
 */
neuralDeclarationReference
    : expression
    ;


/* ============================================================================
 * 43. NEURAL CALL
 * ========================================================================== */

/**
 * Generic neural operation call.
 *
 * Examples:
 *
 *     network(x)
 *     layer(input)
 *     custom_neural_operation(x, y)
 *
 * Call syntax remains owned by Expressions.
 */
neuralCall
    : expression
    ;


/* ============================================================================
 * 44. NEURAL CONFIGURATION
 * ========================================================================== */

/**
 * Generic configuration value.
 *
 * No framework-specific configuration schema is embedded in the grammar.
 */
neuralConfiguration
    : neuralClause
    ;


/* ============================================================================
 * 45. NEURAL CONFIGURATION LIST
 * ========================================================================== */

/**
 * Unbounded configuration sequence.
 */
neuralConfigurationList
    : neuralConfiguration*
    ;


/* ============================================================================
 * 46. NEURAL SEMANTIC ROLE
 * ========================================================================== */

/**
 * Semantic role names are ordinary identifiers.
 *
 * Examples:
 *
 *     input
 *     output
 *     state
 *     parameter
 *     layer
 *     activation
 *     connection
 *     block
 *     component
 *
 * The grammar does not reserve these globally.
 */
neuralRole
    : identifier
    ;


/* ============================================================================
 * 47. NEURAL ROLE CLAUSE
 * ========================================================================== */

/**
 * Associates a role with an expression.
 */
neuralRoleClause
    : neuralRole
      ASSIGN
      expression
    ;


/* ============================================================================
 * 48. NEURAL ROLE CLAUSE LIST
 * ========================================================================== */

/**
 * Unbounded role metadata.
 */
neuralRoleClauseList
    : neuralRoleClause*
    ;


/* ============================================================================
 * 49. NEURAL PROPERTY
 * ========================================================================== */

/**
 * Generic neural property.
 *
 * This intentionally remains open-ended.
 */
neuralProperty
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 50. NEURAL PROPERTY LIST
 * ========================================================================== */

/**
 * Unbounded property sequence.
 */
neuralPropertyList
    : neuralProperty*
    ;


/* ============================================================================
 * 51. NEURAL TARGET
 * ========================================================================== */

/**
 * Generic neural computation target.
 */
neuralTarget
    : expression
    ;


/* ============================================================================
 * 52. NEURAL SOURCE
 * ========================================================================== */

/**
 * Generic neural computation source.
 */
neuralSource
    : expression
    ;


/* ============================================================================
 * 53. NEURAL EDGE
 * ========================================================================== */

/**
 * A generic directed neural relationship.
 */
neuralEdge
    : neuralSource
      ARROW
      neuralTarget
    ;


/* ============================================================================
 * 54. NEURAL EDGE LIST
 * ========================================================================== */

/**
 * Unbounded edge sequence.
 */
neuralEdgeList
    : neuralEdge*
    ;


/* ============================================================================
 * 55. NEURAL REGION
 * ========================================================================== */

/**
 * A neural computation region may contain arbitrary Zamani statements.
 */
neuralRegion
    : LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 56. NEURAL ANNOTATION BRIDGE
 * ========================================================================== */

/**
 * Lexically generic annotation bridge.
 *
 * Semantic analysis determines whether the annotation belongs to the neural
 * domain.
 */
neuralAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 57. NEURAL VALUE BRIDGE
 * ========================================================================== */

/**
 * Explicit source-level value bridge.
 */
neuralValue
    : expression
    ;


/* ============================================================================
 * 58. NEURAL TYPE VALUE
 * ========================================================================== */

/**
 * Explicit source-level type bridge.
 */
neuralTypeValue
    : typeExpression
    ;


/* ============================================================================
 * 59. COMPLETENESS CONTRACT
 * ============================================================================
 *
 * This grammar is complete only when the following external contracts exist:
 *
 *     Lexer
 *         grammar/antlr/ZamaniLexer.g4
 *
 *     Types
 *         grammar/types/types.g4
 *
 *     Expressions
 *         grammar/expressions/expressions.g4
 *
 *     Statements
 *         grammar/statements/statements.g4
 *
 *     AI composition
 *         grammar/ai/ai.g4
 *
 *     Models
 *         grammar/ai/models.g4
 *
 *     Tensors
 *         grammar/ai/tensors.g4
 *
 *     Training
 *         grammar/ai/training.g4
 *
 *     Inference
 *         grammar/ai/inference.g4
 *
 *     Differentiation
 *         grammar/ai/differentiation.g4
 *
 *     Differentiability
 *         grammar/ai/differentiable.g4
 *
 *     Optimization
 *         grammar/ai/optimization.g4
 *
 *     AI semantic analysis
 *
 *     domain-neutral frontend AST
 *
 *     canonical semantic model / ZUIR
 *
 *     classical IR
 *
 *     quantum::ir
 *
 *     hardware/HDL semantic representation
 *
 *     resource/capability analysis
 *
 *     compiler lowering
 *
 *     runtime execution
 *
 * ============================================================================
 * REQUIRED TEST CLASSES
 * ============================================================================
 *
 * Positive:
 *
 *     @neural Network {
 *         @input x;
 *         @output y;
 *         @forward y = Network(x);
 *     }
 *
 *     @neural Network {
 *         @layer input = source;
 *         @layer hidden = transform;
 *         @layer output = projection;
 *         @connect input -> hidden;
 *         @connect hidden -> output;
 *     }
 *
 *     @neural Network {
 *         @layer custom = user_defined_layer;
 *         @forward output = custom(input);
 *     }
 *
 *     @neural Network {
 *         @requires capability("tensor.compute");
 *     }
 *
 * Negative:
 *
 *     @neural {
 *     }
 *
 *     @neural Network {
 *         @connect -> output;
 *     }
 *
 *     @neural Network {
 *         @forward = ;
 *     }
 *
 * Boundary:
 *
 *     zero components;
 *     one component;
 *     one connection;
 *     deeply nested neural regions;
 *     symbolic dimensions;
 *     symbolic parameters;
 *     empty configuration;
 *     user-defined operations;
 *     user-defined activation references.
 *
 * Scalability:
 *
 *     many components;
 *     many connections;
 *     many inputs;
 *     many outputs;
 *     arbitrary symbolic tensor shapes;
 *     arbitrarily large source-level graphs;
 *     distributed neural computation;
 *     accelerator-backed neural computation;
 *     quantum/classical hybrid neural computation.
 *
 * Determinism:
 *
 *     identical source + identical grammar version
 *         -> identical parse structure.
 *
 * Hard-coding audit:
 *
 *     no machine capacity;
 *     no neural graph capacity;
 *     no tensor-rank capacity;
 *     no layer capacity;
 *     no parameter capacity;
 *     no device capacity;
 *     no vendor dependency.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * neural.g4 describes neural computation as portable source-level intent.
 *
 * It does not determine:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     accelerator;
 *     memory bank;
 *     physical qubit;
 *     cluster node;
 *     network topology.
 *
 * Consequently a single neural program can participate in the POCO-REAF
 * compilation model:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Anywhere
 *          ->
 *     Forever
 *
 * subject only to semantic correctness, available resources, target
 * capabilities, compatibility, and implementation support.
 *
 * ============================================================================
 */