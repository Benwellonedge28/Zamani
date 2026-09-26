/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/differentiable.g4
 *
 * Grammar:
 *     AIDifferentiable
 *
 * Status:
 *     Production AI differentiability-contract parser grammar.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only; no `unsafe` is introduced by this grammar.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL DIFFERENTIABILITY CONTRACT SYNTAX.
 *
 * It answers:
 *
 *     "What source computation/value is declared or asserted to be
 *      differentiable, and under which source-level mathematical/semantic
 *      conditions?"
 *
 * It does NOT answer:
 *
 *     "How is its derivative computed?"
 *
 * Derivative requests and differentiation strategies remain owned by:
 *
 *     grammar/ai/differentiation.g4
 *
 * This separation is intentional:
 *
 *     differentiable.g4
 *         -> differentiability property / contract
 *
 *     differentiation.g4
 *         -> derivative computation request
 *
 * Neither grammar owns an automatic-differentiation implementation.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic differentiability model
 *       |
 *       +----------------------+-----------------------+
 *       |                      |                       |
 *       v                      v                       v
 *     classical             quantum                 hybrid
 *     semantics             semantics               semantics
 *       |                      |                       |
 *       +----------------------+-----------------------+
 *                              |
 *                              v
 *                     canonical semantic IR
 *                              |
 *                       optimization/lowering
 *                              |
 *                         scheduling/routing
 *                              |
 *                         target realization
 *
 * Quantum computations, when present, continue through the canonical
 * `quantum::ir` boundary.
 *
 * This file does not create:
 *
 *     - a quantum differentiation IR;
 *     - a quantum gate set;
 *     - a physical-qubit model;
 *     - a QEC model;
 *     - a ZQN model.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - differentiability declarations;
 *   - differentiability assertions;
 *   - differentiability regions;
 *   - differentiability targets;
 *   - differentiability contract clauses;
 *   - source-level differentiability metadata that is part of the contract;
 *   - open-ended contract values represented by ordinary expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer tokens;
 *   - identifiers;
 *   - general expressions;
 *   - general types;
 *   - general statements;
 *   - derivative evaluation;
 *   - automatic differentiation;
 *   - symbolic differentiation;
 *   - numerical differentiation;
 *   - finite differences;
 *   - forward/reverse mode implementation;
 *   - adjoint differentiation implementation;
 *   - parameter-shift implementation;
 *   - optimizer algorithms;
 *   - tensor storage or kernels;
 *   - model execution/training;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - HAL;
 *   - device discovery;
 *   - target selection;
 *   - runtime execution.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Annotation spelling is consumed through the existing:
 *
 *     NANO_ANNOTATION
 *
 * token.
 *
 * This file MUST NOT introduce a new lexer keyword/token solely for
 * `differentiable`.
 *
 * Semantic analysis validates that the annotation used at this boundary has
 * the registered differentiability meaning, normally:
 *
 *     @differentiable
 *
 * This keeps the lexer extensible and avoids a growing global keyword list.
 *
 * ============================================================================
 * COMMON GRAMMAR CONTRACT
 * ============================================================================
 *
 * General types and expressions remain canonical:
 *
 *     Types       -> typeExpression
 *     Expressions -> expression
 *
 * General statements remain canonical:
 *
 *     Statements  -> statement
 *
 * This file never redefines their syntax.
 *
 * ============================================================================
 * ANTLR DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     AIDifferentiable
 *         |
 *         +--> Types
 *         +--> Expressions
 *         +--> Statements
 *
 * It does not import:
 *
 *     AI
 *     AIDifferentiation
 *     Quantum
 *     Hardware
 *     Resources
 *     Runtime
 *
 * This prevents cycles and keeps this leaf independently completable.
 *
 * `differentiation.g4` may consume this grammar through an explicit
 * integration boundary, but this grammar never imports differentiation.g4.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * There is NO grammar-level maximum for:
 *
 *   - differentiation order;
 *   - number of variables;
 *   - number of outputs;
 *   - tensor rank;
 *   - tensor dimensions;
 *   - model size;
 *   - expression size;
 *   - contract clause count;
 *   - nested differentiability regions;
 *   - number of differentiable values;
 *   - number of programs;
 *   - number of functions;
 *   - number of models;
 *   - CPUs;
 *   - GPUs;
 *   - FPGAs;
 *   - QPUs;
 *   - nodes;
 *   - workers;
 *   - devices.
 *
 * Repetition is structural and therefore scales with the source program and
 * available implementation resources.
 *
 * Practical parser/compiler limits for hostile-input protection are runtime
 * or implementation policy. They MUST NOT become language-level semantics.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_PARAMETERS
 *     MAX_MODEL_SIZE
 *     MAX_DERIVATIVE_ORDER
 *     MAX_VARIABLES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *
 * It also MUST NOT encode:
 *
 *     gpu0
 *     cpu0
 *     qpu0
 *     physical_qubit(17)
 *     fixed SIMD width
 *     fixed accelerator count
 *     fixed memory capacity
 *
 * Resource and capability requirements belong to the resource/hardware
 * semantic layers.
 *
 * For example:
 *
 *     requires = capability("tensor.autodiff")
 *
 * is source-level intent. It does not select a physical device.
 *
 * ============================================================================
 * DIFFERENTIABILITY CONTRACT SEMANTICS
 * ============================================================================
 *
 * Contract clause names are deliberately identifiers rather than a closed
 * keyword list. This permits future differentiability properties without
 * modifying the lexer or this grammar for every new mathematical domain.
 *
 * Examples of semantic clause names include:
 *
 *     wrt
 *     order
 *     domain
 *     codomain
 *     inputs
 *     outputs
 *     parameters
 *     condition
 *     requires
 *     capability
 *     constraint
 *     preference
 *     precision
 *     regularity
 *     continuity
 *     extension
 *
 * Their legality and meaning are semantic concerns.
 *
 * IMPORTANT:
 *
 * `mode`, `strategy`, `forward`, `reverse`, `adjoint`,
 * `parameter_shift`, `finite_difference`, etc. are NOT implementation
 * choices defined by this grammar.
 *
 * Those belong to differentiation strategy requests and compiler lowering.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A differentiability contract may target an expression whose semantic value
 * contains quantum computation.
 *
 * This file does not define:
 *
 *   - quantum gates;
 *   - qubits;
 *   - physical qubits;
 *   - parameter-shift rules;
 *   - adjoint circuits;
 *   - measurement semantics;
 *   - QEC;
 *   - ZQN;
 *   - topology;
 *   - calibration.
 *
 * If the target is quantum, semantic lowering remains:
 *
 *     differentiability contract
 *       -> semantic analysis
 *       -> quantum semantics
 *       -> quantum::ir
 *       -> differentiation/lowering
 *       -> optimization
 *       -> routing/scheduling/resilience
 *       -> HAL
 *       -> target
 *
 * ============================================================================
 * CLASSICAL / AI / HDL / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The target expression may refer to values from any domain whose source
 * syntax is already accepted by the canonical expression/type/statement
 * layers.
 *
 * This file does not define separate:
 *
 *     AIExpression
 *     TensorExpression
 *     QuantumExpression
 *     HardwareExpression
 *
 * Instead it consumes the universal `expression` boundary.
 *
 * AI, classical, quantum, HDL, distributed, networking, and future domains
 * therefore share the same differentiability contract mechanism.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *   - annotation/source span;
 *   - target expression;
 *   - optional declared identifier;
 *   - optional target type;
 *   - initializer where present;
 *   - ordered contract clauses;
 *   - clause names and values;
 *   - region statements where a region form is used.
 *
 * The AST must remain domain-neutral.
 *
 * Semantic analysis may subsequently classify a target as:
 *
 *   - classical;
 *   - tensor;
 *   - AI;
 *   - quantum;
 *   - hybrid;
 *   - or another supported domain.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *   1. the annotation is the registered differentiability annotation;
 *   2. the target resolves to a valid semantic object;
 *   3. clause names are recognized or explicitly permitted by a dialect;
 *   4. clause values have valid types/meaning;
 *   5. `wrt` variables belong to the target where required;
 *   6. requested differentiability properties are mathematically meaningful;
 *   7. resource/capability clauses are delegated to their owning subsystem;
 *   8. implementation strategies are not mistaken for source-level guarantees.
 *
 * The parser performs none of these semantic decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Semantic analysis creates the canonical differentiability representation
 * used by later compiler stages.
 *
 * If quantum semantics are involved, the canonical quantum representation
 * remains:
 *
 *     quantum::ir
 *
 * No:
 *
 *     DifferentiableIR
 *     AutomaticDifferentiationIR
 *     QuantumGradientIR
 *     PhysicalGradientIR
 *
 * is introduced by this grammar.
 *
 * ============================================================================
 * DETERMINISM / SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *   - no embedded Rust actions;
 *   - no semantic predicates;
 *   - no I/O;
 *   - no filesystem access;
 *   - no network access;
 *   - no environment inspection;
 *   - no hardware discovery;
 *   - no randomness;
 *   - no runtime execution;
 *   - no unsafe Rust.
 *
 * Identical source/token streams under the same language version must produce
 * deterministic parse structures and source spans.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors are parser errors:
 *
 *   - missing target;
 *   - missing delimiter;
 *   - malformed clause;
 *   - malformed type;
 *   - malformed expression;
 *   - malformed region.
 *
 * Semantic errors remain downstream:
 *
 *   - unknown target;
 *   - target is not differentiable;
 *   - invalid differentiation variable;
 *   - unsupported mathematical property;
 *   - unavailable capability;
 *   - impossible resource requirement;
 *   - incompatible quantum/hardware realization.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Contract values are parsed as source expressions only.
 *
 * They are not evaluated by this grammar and cannot trigger:
 *
 *   - execution;
 *   - hardware discovery;
 *   - file access;
 *   - network access;
 *   - backend invocation.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] It has exactly one public differentiability construct boundary.
 *   [ ] It uses the canonical ZamaniLexer vocabulary.
 *   [ ] It reuses canonical expression/type/statement syntax.
 *   [ ] It introduces no lexer keyword solely for differentiability.
 *   [ ] It distinguishes differentiability contracts from differentiation
 *       operations.
 *   [ ] It allows arbitrary symbolic differentiation order.
 *   [ ] It allows an unbounded number of contract clauses.
 *   [ ] It allows arbitrary target expressions.
 *   [ ] It allows differentiability regions without fixed statement counts.
 *   [ ] It introduces no hardware-size limit.
 *   [ ] It introduces no tensor/model-size limit.
 *   [ ] It introduces no fixed differentiation strategy.
 *   [ ] It introduces no quantum IR.
 *   [ ] It introduces no backend selection.
 *   [ ] It contains no semantic actions or predicates.
 *   [ ] It is compatible with the Rust 1.97/1.97.1 safe implementation
 *       baseline.
 *   [ ] Its AST and semantic consumers are defined before integration.
 *   [ ] Positive, negative, boundary, scalability, determinism, and
 *       compatibility tests exist at the repository test layer.
 *
 * ============================================================================
 */

parser grammar AIDifferentiable;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical parser entry point for differentiability contracts.
 *
 * The annotation spelling is validated semantically. The grammar intentionally
 * consumes the generic annotation token rather than creating a new keyword.
 */
aiDifferentiableConstruct
    : aiDifferentiabilityDeclaration
    | aiDifferentiabilityAssertion
    | aiDifferentiabilityRegion
    ;


/* ============================================================================
 * DECLARATION
 * ========================================================================== */

/**
 * Declares a named differentiability contract/value.
 *
 * Examples:
 *
 *     @differentiable f: Tensor<float> = model;
 *
 *     @differentiable loss = objective;
 *
 * The semantic layer validates that the annotation is `@differentiable` and
 * that the declared target/value has the requested differentiability meaning.
 */
aiDifferentiabilityDeclaration
    : aiDifferentiabilityAnnotation
      identifier
      aiDifferentiabilityType?
      aiDifferentiabilityInitializer?
      SEMICOLON
    ;


aiDifferentiabilityType
    : COLON typeExpression
    ;


aiDifferentiabilityInitializer
    : ASSIGN expression
    ;


/* ============================================================================
 * ASSERTION / INLINE CONTRACT
 * ========================================================================== */

/**
 * Inline differentiability assertion.
 *
 * Examples:
 *
 *     @differentiable(f)
 *
 *     @differentiable(f, wrt = x, order = n)
 *
 *     @differentiable(model(x), domain = real, outputs = y)
 *
 * Clause count is unbounded by the grammar.
 */
aiDifferentiabilityAssertion
    : aiDifferentiabilityAnnotation
      LPAREN
      aiDifferentiabilityTarget
      aiDifferentiabilityClause*
      RPAREN
      SEMICOLON?
    ;


aiDifferentiabilityTarget
    : expression
    ;


/**
 * A clause is a semantic name/value pair.
 *
 * Keeping the clause name open-ended prevents a permanent parser keyword list
 * for every mathematical or future differentiability property.
 */
aiDifferentiabilityClause
    : COMMA
      identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * REGION
 * ========================================================================== */

/**
 * Applies a differentiability contract to a computation region.
 *
 * Examples:
 *
 *     @differentiable {
 *         loss = model(input);
 *         output = loss;
 *     }
 *
 *     @differentiable target {
 *         ...
 *     }
 *
 * Statement syntax remains owned by Statements.
 */
aiDifferentiabilityRegion
    : aiDifferentiabilityAnnotation
      aiDifferentiabilityRegionTarget?
      LBRACE
      statement*
      RBRACE
    ;


aiDifferentiabilityRegionTarget
    : expression
    ;


/* ============================================================================
 * ANNOTATION BOUNDARY
 * ========================================================================== */

/**
 * Generic annotation token.
 *
 * Semantic analysis MUST validate the annotation as the registered
 * differentiability contract annotation. The grammar deliberately does not
 * hard-code its spelling into the lexer.
 */
aiDifferentiabilityAnnotation
    : NANO_ANNOTATION
    ;